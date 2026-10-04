import json
from fractions import Fraction
from pathlib import Path
import tempfile
import unittest

from render_core import RenderError, prepare
from render_core.core import LocalResolver


FIXTURE = Path(__file__).resolve().parents[1] / "fixtures" / "two_clip_graph.json"


class RenderCoreTests(unittest.TestCase):
    def setUp(self):
        self.graph = json.loads(FIXTURE.read_text(encoding="utf-8"))

    def test_late_seek_only_plans_contributing_source(self):
        plan = prepare(self.graph, "s1", Fraction(9, 2), Fraction(5))
        self.assertEqual([part.node_id for part in plan.slices], ["clip-b"])
        self.assertEqual(plan.slices[0].source_start, Fraction(1, 2))
        self.assertEqual(plan.duration, Fraction(1, 2))

    def test_full_export_keeps_gap_without_source_task(self):
        plan = prepare(self.graph, "s1", Fraction(0), Fraction(6))
        self.assertEqual([part.node_id for part in plan.slices], ["clip-a", "clip-b"])
        self.assertEqual([part.timeline_start for part in plan.slices], [Fraction(0), Fraction(4)])

    def test_rejects_invalid_and_unsupported_graph(self):
        self.graph["nodes"]["clip-b"]["source"]["slot_id"] = "missing"
        with self.assertRaisesRegex(RenderError, "invalid baseline graph"):
            prepare(self.graph, "s1", Fraction(0), Fraction(6))
        self.graph = json.loads(FIXTURE.read_text(encoding="utf-8"))
        with self.assertRaisesRegex(RenderError, "outside sequence"):
            prepare(self.graph, "s1", Fraction(0), Fraction(7))

    def test_resolver_detects_binding_change(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "source.bin").write_bytes(b"different")
            resolver = LocalResolver({"slots": {"camera-a": {
                "path": "source.bin", "sha256": "0" * 64}}}, root)
            with self.assertRaisesRegex(RenderError, "fingerprint mismatch"):
                resolver.resolve("camera-a")


if __name__ == "__main__":
    unittest.main()
