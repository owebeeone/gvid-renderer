"""Reference semantic validator for the v0.2 built-in playable graph subset.

The project host still needs authorization, persistence, resource and full-registry
validation. This module deliberately rejects effects, transitions and titles until
their port contracts are defined.
"""

from __future__ import annotations

from fractions import Fraction
from math import gcd

MAX_EXACT = 2**53 - 1
MAX_DENOMINATOR = 10**9


def _fail(reason: str) -> None:
    raise ValueError(reason)


def _rational(value: dict) -> Fraction:
    if not isinstance(value, dict):
        _fail("invalid rational")
    numerator = value.get("numerator")
    denominator = value.get("denominator")
    if (type(numerator) is not int or type(denominator) is not int
            or denominator <= 0 or denominator > MAX_DENOMINATOR
            or abs(numerator) > MAX_EXACT or denominator > MAX_EXACT
            or gcd(abs(numerator), denominator) != 1):
        _fail("invalid rational")
    return Fraction(numerator, denominator)


def _range(value: dict) -> tuple[Fraction, Fraction]:
    if not isinstance(value, dict):
        _fail("invalid range")
    start = _rational(value.get("start"))
    end = _rational(value.get("end"))
    if start >= end:
        _fail("invalid range")
    return start, end


def validate_baseline_graph(graph: dict) -> None:
    """Reject a graph outside the first deterministic source/selector/root subset."""
    if graph.get("schema_major") != 0 or graph.get("schema_minor") != 2:
        _fail("unsupported graph version")
    if (not graph.get("project_id") or not graph.get("graph_id")
            or type(graph.get("revision")) is not int
            or graph["revision"] < 0):
        _fail("invalid graph identity or revision")

    entities = ("sequences", "tracks", "nodes", "edges", "slots")
    maps = {}
    all_ids = set()
    for name in entities:
        values = graph.get(name)
        if not isinstance(values, dict):
            _fail(f"missing {name} map")
        for key, value in values.items():
            if (not isinstance(key, str) or not key
                    or not isinstance(value, dict) or value.get("id") != key):
                _fail(f"invalid {name} ID")
            if key in all_ids:
                _fail("duplicate entity ID")
            all_ids.add(key)
        maps[name] = values

    sequences = maps["sequences"]
    tracks = maps["tracks"]
    nodes = maps["nodes"]
    edges = maps["edges"]
    slots = maps["slots"]
    sequence_ranges = {key: _range(value["range"])
                       for key, value in sequences.items()}
    if not sequences:
        _fail("no sequence")
    declared_roots = set()
    for sequence in sequences.values():
        chosen = [sequence.get(name) for name in ("video_root", "audio_root")
                  if sequence.get(name) is not None]
        if not chosen:
            _fail("sequence without root")
        declared_roots.update(chosen)

    track_orders = set()
    for track in tracks.values():
        seq_id = track.get("sequence_id")
        media = track.get("media")
        if seq_id not in sequences or media not in ("video", "audio"):
            _fail("invalid track")
        order = (seq_id, media, track.get("order_key"))
        if not order[2] or order in track_orders:
            _fail("duplicate track order")
        track_orders.add(order)
        if not track.get("enabled"):
            _fail("disabled tracks are outside baseline subset")

    selectors = {}
    roots = {}
    source_intervals = {}
    for node in nodes.values():
        seq_id = node.get("sequence_id")
        media = node.get("media")
        if seq_id not in sequences or media not in ("video", "audio"):
            _fail("invalid node context")
        seq_range = sequence_ranges[seq_id]
        kind = node.get("kind")
        track_id = node.get("track_id")
        payload_names = ("source", "effect", "composite", "transition",
                         "title", "output")
        present = [name for name in payload_names if node.get(name) is not None]
        if present != [kind]:
            _fail("node payload mismatch")
        if kind == "source":
            track = tracks.get(track_id)
            payload = node.get("source")
            if (track is None or track["sequence_id"] != seq_id
                    or track["media"] != media or not isinstance(payload, dict)):
                _fail("invalid source track or payload")
            timeline = _range(node.get("timeline_range"))
            source_range = _range(payload.get("source_range"))
            if timeline[0] < seq_range[0] or timeline[1] > seq_range[1]:
                _fail("source outside sequence")
            if payload.get("slot_id") not in slots:
                _fail("dangling slot")
            if slots[payload["slot_id"]]["expected_media"] != media:
                _fail("slot media mismatch")
            speed = _rational(payload.get("speed"))
            if speed <= 0 or (source_range[1] - source_range[0]) / speed != timeline[1] - timeline[0]:
                _fail("source speed/range mismatch")
            source_intervals.setdefault(track_id, []).append((timeline, node["id"]))
        elif kind == "composite":
            payload = node.get("composite")
            if not isinstance(payload, dict) or _range(node.get("timeline_range")) != seq_range:
                _fail("invalid composite")
            blend = payload.get("blend_id")
            if blend == "timeline.select.v1":
                track = tracks.get(track_id)
                if (track is None or track["sequence_id"] != seq_id
                        or track["media"] != media or track_id in selectors):
                    _fail("invalid track selector")
                selectors[track_id] = node["id"]
            elif blend == "layer.stack.v1":
                if track_id is not None:
                    _fail("layer stack must not belong to a track")
            else:
                _fail("unsupported composite")
        elif kind == "output":
            if track_id is not None or node.get("timeline_range") is not None:
                _fail("invalid output placement")
            if node.get("output", {}).get("output_media") != media:
                _fail("output media mismatch")
            roots[node["id"]] = (seq_id, media)
        else:
            _fail("unsupported node kind")

    if set(roots) != declared_roots:
        _fail("undeclared output root")
    for track_id, spans in source_intervals.items():
        spans.sort(key=lambda item: (item[0][0], item[1]))
        for prior, later in zip(spans, spans[1:]):
            if prior[0][1] > later[0][0]:
                _fail("same-track overlap")
    if set(selectors) != set(tracks):
        _fail("each enabled track needs one selector")

    incoming = {key: [] for key in nodes}
    outgoing = {key: [] for key in nodes}
    for edge in edges.values():
        src = nodes.get(edge.get("from_node"))
        dst = nodes.get(edge.get("to_node"))
        if src is None or dst is None:
            _fail("dangling edge")
        if src["sequence_id"] != dst["sequence_id"]:
            _fail("cross-sequence edge")
        src_port = src["media"] if src["kind"] == "source" else "output"
        if src["kind"] == "output" or edge.get("from_port") != src_port:
            _fail("invalid source port")
        dest_port = edge.get("to_port")
        if dst["kind"] == "composite":
            blend = dst["composite"]["blend_id"]
            if blend == "timeline.select.v1":
                if (dest_port != "item" or src["kind"] != "source"
                        or src["track_id"] != dst["track_id"]
                        or src["media"] != dst["media"]
                        or edge.get("order_key") is not None):
                    _fail("invalid selector port")
            elif blend == "layer.stack.v1":
                track = tracks.get(src.get("track_id"))
                if (dest_port != "layer" or src["kind"] != "composite"
                        or src["composite"]["blend_id"] != "timeline.select.v1"
                        or track is None or edge.get("order_key") != track["order_key"]
                        or src["media"] != dst["media"]):
                    _fail("invalid layer port")
        elif dst["kind"] == "output":
            if dest_port != "input" or src["kind"] != "composite" or src["media"] != dst["media"] or edge.get("order_key") is not None:
                _fail("invalid output port")
        else:
            _fail("source nodes have no input port")
        incoming[dst["id"]].append(edge)
        outgoing[src["id"]].append(edge)

    for node in nodes.values():
        if node["kind"] == "source" and (incoming[node["id"]] or len(outgoing[node["id"]]) != 1):
            _fail("source routing")
        if node["kind"] == "composite" and node["composite"]["blend_id"] == "timeline.select.v1":
            linked = {edge["from_node"] for edge in incoming[node["id"]]}
            expected = {src["id"] for src in nodes.values()
                        if src["kind"] == "source" and src["track_id"] == node["track_id"]}
            if linked != expected:
                _fail("selector missing source")

    for seq_id, sequence in sequences.items():
        for media, field in (("video", "video_root"), ("audio", "audio_root")):
            root_id = sequence.get(field)
            active = [t for t in tracks.values()
                      if t["sequence_id"] == seq_id and t["media"] == media]
            if root_id is None:
                if active:
                    _fail("track without root")
                continue
            if not active:
                _fail("root without track")
            if roots.get(root_id) != (seq_id, media):
                _fail("invalid sequence root")
            root_edges = incoming[root_id]
            if len(root_edges) != 1:
                _fail("output root needs one input")
            feeder = nodes[root_edges[0]["from_node"]]
            if len(active) == 1:
                if feeder["id"] != selectors[active[0]["id"]]:
                    _fail("single track must feed root")
            else:
                if (feeder["kind"] != "composite"
                        or feeder["composite"]["blend_id"] != "layer.stack.v1"):
                    _fail("multi-track root needs layer stack")
                linked = {nodes[e["from_node"]]["track_id"] for e in incoming[feeder["id"]]}
                if (linked != {track["id"] for track in active}
                        or len(incoming[feeder["id"]]) != len(active)):
                    _fail("layer stack missing or duplicate track")

    state = {}
    def visit(node_id: str) -> None:
        if state.get(node_id) == 1:
            _fail("cycle")
        if state.get(node_id) == 2:
            return
        state[node_id] = 1
        for edge in outgoing[node_id]:
            visit(edge["to_node"])
        state[node_id] = 2

    for node_id in nodes:
        visit(node_id)

    reachable = set()
    def walk_back(node_id: str) -> None:
        if node_id in reachable:
            return
        reachable.add(node_id)
        for edge in incoming[node_id]:
            walk_back(edge["from_node"])

    for root_id in roots:
        walk_back(root_id)
    if reachable != set(nodes):
        _fail("unreachable node")
