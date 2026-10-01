#!/usr/bin/env python3
"""Regenerate GVid render-graph Taut bindings for Python, Rust, and TypeScript."""

from __future__ import annotations

import importlib.metadata
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SCHEMA = ROOT / "ir" / "gvid_render_graph.taut.py"
OUTPUT = ROOT / "generated" / "taut"
TAUT_PROTO_VERSION = "0.10.0"


def ensure_taut_proto() -> None:
    """Install the pinned generator into this Python environment if absent."""
    try:
        installed = importlib.metadata.version("taut-proto")
    except importlib.metadata.PackageNotFoundError:
        package = f"taut-proto=={TAUT_PROTO_VERSION}"
        print(f"Installing {package} with {sys.executable}", flush=True)
        subprocess.run(
            [sys.executable, "-m", "pip", "install", package],
            check=True,
        )
    else:
        if installed != TAUT_PROTO_VERSION:
            print(
                f"Warning: taut-proto {installed} is installed; this script "
                f"was checked with {TAUT_PROTO_VERSION}.",
                file=sys.stderr,
            )


def main() -> int:
    if not SCHEMA.is_file():
        print(f"Taut schema not found: {SCHEMA}", file=sys.stderr)
        return 2

    ensure_taut_proto()

    # This runs the same entry point as tautc, without relying on PATH.
    command = [
        sys.executable,
        "-m",
        "taut.cli",
        "gen",
        str(SCHEMA),
        "--out",
        str(OUTPUT),
        "--lang",
        "python,rust,typescript",
        "--with-runtime",
        "--forward-compat",
    ]
    # Taut's generated runtime includes Unicode and needs UTF-8 on Windows.
    environment = os.environ.copy()
    environment["PYTHONUTF8"] = "1"
    print(f"Generating Taut bindings from {SCHEMA} into {OUTPUT}", flush=True)
    result = subprocess.run(
        command, cwd=ROOT, env=environment, check=False
    ).returncode
    if result:
        return result

    # tautc writes CRLF on Windows; keep checked-in bindings platform neutral.
    for path in OUTPUT.rglob("*"):
        if path.suffix in {".py", ".rs", ".ts"}:
            data = path.read_bytes()
            normalized = data.replace(b"\r\n", b"\n").rstrip(b"\n") + b"\n"
            if normalized != data:
                path.write_bytes(normalized)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
