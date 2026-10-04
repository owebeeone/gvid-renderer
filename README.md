# GVid renderer

The first runnable slice is a headless Python library (`render_core`) and an
offline CLI (`render_cli.py`). It consumes a validated v0.2 flat graph and a
separate local binding manifest. It plans only source spans contributing to
the requested half-open range, then uses an injected FFmpeg/FFprobe provider
to create and verify an MP4 before replacing the destination.

This is an early reference provider, not the complete accepted renderer
contract. It supports video source nodes, timeline selectors, layer stacks,
gaps, and output roots, with speed 1. It currently rejects audio graphs,
effects, transitions, titles, retiming, and nonbaseline graph semantics. The
CLI takes JSON snapshots for local use; this does not replace the Taut wire
contract for Glade communication. It has no Glade adapter, job journal,
concurrent scheduler, cancellation or cache yet.

## Local use

Install or provide FFmpeg and FFprobe. The executable paths may be passed
with `--ffmpeg` and `--ffprobe`. A binding manifest has this shape:

```json
{
  "slots": {
    "camera-a": {"path": "media/a.mp4", "sha256": "optional-64-character-digest"},
    "camera-b": {"path": "media/b.mp4"}
  }
}
```

Relative paths resolve from the binding manifest's directory. To inspect a
late seek without media tools:

```text
python render_cli.py fixtures/two_clip_graph.json bindings.json s1 9/2 5 output.mp4 --plan-only
```

To render the full sequence:

```text
python render_cli.py fixtures/two_clip_graph.json bindings.json s1 0 6 output.mp4
```

Run tests from this directory with the local Taut source on `PYTHONPATH`:

```text
python -m unittest discover -s tests -v
```

The four core tests do not need FFmpeg. A real media acceptance run still
requires FFmpeg, generated synthetic source videos, pixel/audio oracles, and
the runtime verification scenarios in `dev-docs/render-runtime-verification-design.md`.
