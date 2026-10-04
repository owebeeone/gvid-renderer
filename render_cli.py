"""Offline CLI for the first GVid renderer subset.

Example: python render_cli.py graph.json bindings.json s1 0 6 output.mp4
"""

import argparse
from fractions import Fraction
import json
from pathlib import Path
import sys

from render_core import RenderError, RenderProfile, prepare, render
from render_core.core import FFmpegTool, LocalResolver


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("graph", type=Path)
    parser.add_argument("bindings", type=Path)
    parser.add_argument("sequence")
    parser.add_argument("start", help="seconds, decimal or fraction")
    parser.add_argument("end", help="seconds, decimal or fraction")
    parser.add_argument("output", type=Path)
    parser.add_argument("--width", type=int, default=640)
    parser.add_argument("--height", type=int, default=360)
    parser.add_argument("--fps", type=int, default=30)
    parser.add_argument("--ffmpeg", default="ffmpeg")
    parser.add_argument("--ffprobe", default="ffprobe")
    parser.add_argument("--plan-only", action="store_true", help="validate and print contributing slices without rendering")
    args = parser.parse_args(argv)
    try:
        graph = json.loads(args.graph.read_text(encoding="utf-8"))
        plan = prepare(graph, args.sequence, Fraction(args.start), Fraction(args.end),
                       RenderProfile(args.width, args.height, args.fps))
        if args.plan_only:
            print(json.dumps({"graph_id": plan.graph_id, "revision": plan.revision,
                              "sequence_id": plan.sequence_id,
                              "range": [str(plan.start), str(plan.end)],
                              "slices": [{"node_id": part.node_id, "slot_id": part.slot_id,
                                          "source_start": str(part.source_start),
                                          "timeline_start": str(part.timeline_start),
                                          "duration": str(part.duration)} for part in plan.slices]}, sort_keys=True))
            return 0
        bindings = json.loads(args.bindings.read_text(encoding="utf-8"))
        result = render(plan, LocalResolver(bindings, args.bindings.parent),
                        FFmpegTool(args.ffmpeg, args.ffprobe), args.output)
        print(json.dumps(result, sort_keys=True))
        return 0
    except (RenderError, OSError, ValueError, KeyError) as exc:
        print(f"render error: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
