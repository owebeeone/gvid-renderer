"""Small, offline reference path for the GVid v0.2 graph subset.

The graph is immutable input. A plan contains only nodes that contribute to the
requested time range. FFmpeg is an injected tool; transport and UI are absent.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
from typing import Protocol


class RenderError(Exception):
    pass


def rational(value: dict) -> Fraction:
    return Fraction(value["numerator"], value["denominator"])


def interval(value: dict) -> tuple[Fraction, Fraction]:
    return rational(value["start"]), rational(value["end"])


def stamp(value: Fraction) -> str:
    return f"{float(value):.9f}"


def file_digest(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


@dataclass(frozen=True)
class RenderProfile:
    width: int = 640
    height: int = 360
    fps: int = 30
    video_codec: str = "libx264"
    pixel_format: str = "yuv420p"

    def validate(self) -> None:
        if min(self.width, self.height, self.fps) <= 0 or max(self.width, self.height) > 8192 or self.fps > 120:
            raise RenderError("profile dimensions and frame rate must be positive")
        if self.video_codec != "libx264" or self.pixel_format != "yuv420p":
            raise RenderError("this reference provider supports libx264/yuv420p only")


@dataclass(frozen=True)
class SourceSlice:
    node_id: str
    slot_id: str
    source_start: Fraction
    timeline_start: Fraction
    duration: Fraction
    track_order: str


@dataclass(frozen=True)
class RenderPlan:
    graph_id: str
    revision: int
    sequence_id: str
    start: Fraction
    end: Fraction
    slices: tuple[SourceSlice, ...]
    profile: RenderProfile

    @property
    def duration(self) -> Fraction:
        return self.end - self.start


class AssetResolver(Protocol):
    def resolve(self, slot_id: str) -> Path: ...


class MediaTool(Protocol):
    def run(self, plan: RenderPlan, resolver: AssetResolver, destination: Path) -> dict: ...


class LocalResolver:
    def __init__(self, bindings: dict, package_root: Path):
        self.bindings = bindings
        self.package_root = package_root.resolve()

    def resolve(self, slot_id: str) -> Path:
        record = self.bindings.get("slots", {}).get(slot_id)
        if not isinstance(record, dict) or not isinstance(record.get("path"), str):
            raise RenderError(f"missing local asset binding for slot {slot_id}")
        path = Path(record["path"])
        if not path.is_absolute():
            path = self.package_root / path
        path = path.resolve(strict=True)
        if not path.is_file():
            raise RenderError(f"asset binding is not a file: {slot_id}")
        expected = record.get("sha256")
        if expected and file_digest(path) != expected:
            raise RenderError(f"asset fingerprint mismatch: {slot_id}")
        return path


def prepare(graph: dict, sequence_id: str, start: Fraction, end: Fraction,
            profile: RenderProfile = RenderProfile()) -> RenderPlan:
    # Reuse the repository's semantic validator before reading any graph edges.
    from ir.validate_baseline_graph import validate_baseline_graph
    try:
        validate_baseline_graph(graph)
    except (ValueError, KeyError, TypeError) as exc:
        raise RenderError(f"invalid baseline graph: {exc}") from exc
    profile.validate()
    seq = graph["sequences"].get(sequence_id)
    if seq is None:
        raise RenderError(f"unknown sequence: {sequence_id}")
    sequence_start, sequence_end = interval(seq["range"])
    if start < sequence_start or end > sequence_end or start >= end:
        raise RenderError("requested range is outside sequence")
    if seq.get("audio_root") is not None:
        raise RenderError("audio graphs are not yet supported by this provider")
    incoming: dict[str, list[dict]] = {node: [] for node in graph["nodes"]}
    for edge in graph["edges"].values():
        incoming[edge["to_node"]].append(edge)
    reachable: set[str] = set()
    def walk(node_id: str) -> None:
        if node_id in reachable:
            return
        reachable.add(node_id)
        for edge in incoming[node_id]:
            walk(edge["from_node"])
    walk(seq["video_root"])
    slices = []
    for node_id in reachable:
        node = graph["nodes"][node_id]
        if node["kind"] != "source":
            continue
        left, right = interval(node["timeline_range"])
        visible_start, visible_end = max(start, left), min(end, right)
        if visible_start >= visible_end:
            continue
        speed = rational(node["source"]["speed"])
        source_left, _ = interval(node["source"]["source_range"])
        slices.append(SourceSlice(node_id, node["source"]["slot_id"],
                                  source_left + (visible_start - left) * speed,
                                  visible_start - start, (visible_end - visible_start) * speed,
                                  graph["tracks"][node["track_id"]]["order_key"]))
        if speed != 1:
            raise RenderError(f"source speed other than 1 is not yet supported: {node_id}")
    slices.sort(key=lambda item: (item.track_order, item.timeline_start, item.node_id))
    return RenderPlan(graph["graph_id"], graph["revision"], sequence_id,
                      start, end, tuple(slices), profile)


class FFmpegTool:
    def __init__(self, ffmpeg: str = "ffmpeg", ffprobe: str = "ffprobe"):
        self.ffmpeg, self.ffprobe = ffmpeg, ffprobe

    def run(self, plan: RenderPlan, resolver: AssetResolver, destination: Path) -> dict:
        destination = destination.resolve()
        destination.parent.mkdir(parents=True, exist_ok=True)
        paths = [resolver.resolve(item.slot_id) for item in plan.slices]
        source_digests = [file_digest(path) for path in paths]
        fd, candidate_name = tempfile.mkstemp(prefix=".gvid-", suffix=".mp4", dir=destination.parent)
        os.close(fd)
        candidate = Path(candidate_name)
        try:
            command = [self.ffmpeg, "-hide_banner", "-loglevel", "error", "-y"]
            for item, path in zip(plan.slices, paths):
                command += ["-ss", stamp(item.source_start), "-t", stamp(item.duration), "-i", str(path)]
            p = plan.profile
            filters = [f"color=c=black:s={p.width}x{p.height}:r={p.fps}:d={stamp(plan.duration)}[base]"]
            previous = "base"
            for index, item in enumerate(plan.slices):
                filters.append(f"[{index}:v]fps={p.fps},scale={p.width}:{p.height},setsar=1,setpts=PTS-STARTPTS+{stamp(item.timeline_start)}/TB[v{index}]")
                output = f"o{index}"
                filters.append(f"[{previous}][v{index}]overlay=eof_action=pass:shortest=0:enable='gte(t,{stamp(item.timeline_start)})*lt(t,{stamp(item.timeline_start + item.duration)})'[{output}]")
                previous = output
            command += ["-filter_complex", ";".join(filters), "-map", f"[{previous}]",
                        "-an", "-t", stamp(plan.duration), "-r", str(p.fps),
                        "-c:v", p.video_codec, "-pix_fmt", p.pixel_format,
                        "-movflags", "+faststart", str(candidate)]
            completed = subprocess.run(command, capture_output=True, text=True, timeout=300)
            if completed.returncode:
                raise RenderError(f"FFmpeg failed: {completed.stderr[-2000:]}")
            probed = subprocess.run([self.ffprobe, "-v", "error", "-select_streams", "v:0",
                                     "-show_entries", "stream=width,height,codec_name:format=duration",
                                     "-of", "json", str(candidate)], capture_output=True, text=True, timeout=30)
            if probed.returncode:
                raise RenderError(f"FFprobe failed: {probed.stderr[-2000:]}")
            metadata = json.loads(probed.stdout)
            streams = metadata.get("streams", [])
            if len(streams) != 1 or streams[0].get("width") != p.width or streams[0].get("height") != p.height:
                raise RenderError("output probe did not match pinned profile")
            observed = float(metadata.get("format", {}).get("duration", 0))
            if abs(observed - float(plan.duration)) > 2 / p.fps:
                raise RenderError("output duration did not match requested range")
            if [file_digest(path) for path in paths] != source_digests:
                raise RenderError("source bytes changed during render")
            digest = file_digest(candidate)
            os.replace(candidate, destination)
            return {"path": str(destination), "sha256": digest, "duration": observed,
                    "width": p.width, "height": p.height, "graph_id": plan.graph_id,
                    "graph_revision": plan.revision, "sequence_id": plan.sequence_id,
                    "range": [stamp(plan.start), stamp(plan.end)], "provider": "ffmpeg"}
        finally:
            candidate.unlink(missing_ok=True)


def render(plan: RenderPlan, resolver: AssetResolver, tool: MediaTool,
           destination: Path) -> dict:
    return tool.run(plan, resolver, destination)
