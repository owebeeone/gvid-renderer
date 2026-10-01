"""Wire and semantic fixtures for the draft GVid Taut contract."""

from __future__ import annotations

import copy
import hashlib
import json
import unittest
from pathlib import Path

from taut.ir.load import load_schema
from taut.ir.validate import validate_or_raise
from taut.wire import codec

from ir.validate_baseline_graph import validate_baseline_graph

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures"
GOLDEN_SHA256 = {
    "two_clip_graph": ("GraphSnapshot", "5146ad3427bbcfb487ac60e54b8701a5f1eccb9e10adb67f7510fe33d254e643"),
    "ripple_insert_graph": ("GraphSnapshot", "aca35e4ef88f9ecfd62a957413182e3254ff760c47e6d49a4afc95be83ead01a"),
    "ripple_insert_batch": ("EditBatch", "39ef319a92accd070e5f29f163ab8678ccf2a4020dfaf9a85fd78dc47cb913f0"),
    "insert_source_span": ("InsertSourceSpan", "4cff483f8ee8f66a6ef3777d72f04c6791c25cf12c5a31898a1b942c24b32e25"),
    "insert_new_asset": ("InsertSourceSpan", "cd73757f7e0681ccc4e7665adb90b635e099f07034e48846441a7d6c13364bf4"),
    "export_job_request": ("ExportJobRequest", "eb2e9d01be0c6212f12fcedb9fd69177da3e309149af7bfe1fd46a21b4d16c2a"),
    "export_job_event": ("ExportJobEvent", "ed4b8743f6bfaa40910c3b11bc1ac79497e4cf00ba636048bd8895942f12b985"),
    "cancel_export_job": ("CancelExportJob", "f5628b82f135ad562d0c361bb0bad9c48692be422b3dca61c2083208b720db9e"),
    "export_lookup_query": ("ExportLookupQuery", "6e0859ac33824e32ec7f1db297f77510e5dfd43f22bea258173212c9d8458565"),
    "export_lookup_result": ("ExportLookupResult", "c573a0b96c91a5b73b9650206eee87208239f30496c4ebc9598376c998c29c53"),
    "export_job_request_standalone": ("ExportJobRequest", "d26bd70b5f7488dd8626a4ebb243b7f45653ade049e62ad6e33fd32226de3dfe"),
    "export_job_ack_retired": ("ExportJobAck", "a4eb254d6062b85609025f03b592baca4ed9ef0986486dac90fb66ef5acb696b"),
    "export_lookup_query_standalone": ("ExportLookupQuery", "4bdfde9d79c7604be6048eeaefb880ab9195fcad71d3a583349eff014e3a855e"),
    "export_lookup_retired": ("ExportLookupResult", "d8f1192749613482cf88a1a68c3af833d83f0d31424e9cca039f44b5df95aca0"),
    "export_events_query_governed": ("ExportEventsQuery", "d04f18a5e214b45882b834240c52c9458fad9ad0cdcb4cc9c80fd51320ef4e3b"),
    "export_events_query_standalone": ("ExportEventsQuery", "0ed4a238080e0e58954f45b223fb6ebdff2b98c9331f29ae531c9b0b6fe13f14"),
    "export_event_delivery_event": ("ExportEventDelivery", "aa337d33eb1d0a47d0ad65ff8cc6664b5150225a052050ae7af180bdf6a000a1"),
    "export_event_delivery_unavailable": ("ExportEventDelivery", "6f782986058c7fc227d3e0946a53e684c2d5de06801a999b74de92a92703be00"),
    "export_status_found": ("ExportStatusResult", "f1b7824fb28506510d721cba26b94373b732ffe3979ee58b8b4c6154983090e7"),
    "export_status_unavailable": ("ExportStatusResult", "6f782986058c7fc227d3e0946a53e684c2d5de06801a999b74de92a92703be00"),
    "export_status_stale": ("ExportStatusResult", "61b4a5801a5e0285d14118d6632083fe9417375d3872857d37b47eb85a8cd8c6"),
    "export_cancel_unavailable": ("ExportCancelAck", "381a30d40eecc1729ef3b0c0f386cd864b428cce4405709453dc6a6b49b5aabf"),
}


def fixture(name: str) -> dict:
    return json.loads((FIXTURES / f"{name}.json").read_text(encoding="utf-8"))


class TautContractTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.schema = load_schema(ROOT / "ir" / "gvid_render_graph.taut.py")
        validate_or_raise(cls.schema)

    def round_trip(self, name: str, value: dict) -> None:
        encoded = codec.encode(self.schema, name, value)
        self.assertEqual(codec.decode(self.schema, name, encoded), value)

    def test_golden_wire_fixtures(self) -> None:
        for filename, (message, digest) in GOLDEN_SHA256.items():
            with self.subTest(filename=filename):
                value = fixture(filename)
                encoded = codec.encode(self.schema, message, value)
                self.assertEqual(codec.decode(self.schema, message, encoded), value)
                self.assertEqual(hashlib.sha256(encoded).hexdigest(), digest)

    def test_two_clip_ripple_batch_is_atomic_and_playable(self) -> None:
        before = fixture("two_clip_graph")
        expected = fixture("ripple_insert_graph")
        batch = fixture("ripple_insert_batch")
        validate_baseline_graph(before)
        validate_baseline_graph(expected)
        self.assertEqual(batch["expected_revision"], before["revision"])
        candidate = copy.deepcopy(before)
        record_maps = {
            "node": "nodes", "edge": "edges", "track": "tracks",
            "sequence": "sequences", "slot": "slots",
        }
        for operation in batch["operations"]:
            verb, kind = operation["kind"].split("_", 1)
            records = candidate[record_maps[kind]]
            if verb == "put":
                records[operation["target_id"]] = operation[kind]
            else:
                del records[operation["target_id"]]
        candidate["revision"] += 1
        validate_baseline_graph(candidate)
        self.assertEqual(candidate, expected)
        self.assertEqual(before, fixture("two_clip_graph"))

    def test_invalid_overlap_and_port_reject(self) -> None:
        graph = fixture("two_clip_graph")
        overlap = copy.deepcopy(graph)
        overlap["nodes"]["clip-b"]["timeline_range"]["start"]["numerator"] = 1
        overlap["nodes"]["clip-b"]["timeline_range"]["end"]["numerator"] = 3
        with self.assertRaisesRegex(ValueError, "overlap"):
            validate_baseline_graph(overlap)

        wrong_port = copy.deepcopy(graph)
        wrong_port["edges"]["edge-b"]["to_port"] = "bogus"
        with self.assertRaisesRegex(ValueError, "port"):
            validate_baseline_graph(wrong_port)

        cycle = copy.deepcopy(graph)
        cycle["edges"]["edge-track"]["from_node"] = "out-v"
        with self.assertRaises(ValueError):
            validate_baseline_graph(cycle)
        validate_baseline_graph(graph)

    def test_insert_selection_modes(self) -> None:
        existing = fixture("insert_source_span")
        new_asset = fixture("insert_new_asset")
        self.assertIsNotNone(existing["slot_id"])
        self.assertIsNone(existing["registered_asset_id"])
        self.assertIsNone(existing["asset_version_id"])
        self.assertIsNone(new_asset["slot_id"])
        self.assertIsNotNone(new_asset["registered_asset_id"])
        self.assertIsNotNone(new_asset["asset_version_id"])

    def test_correlated_delivery_and_binding_preview_messages(self) -> None:
        graph = fixture("two_clip_graph")
        batch = fixture("ripple_insert_batch")
        footprint = {
            "intervals": [{"sequence_id": "s1", "range": graph["sequences"]["s1"]["range"]}],
            "node_ids": ["clip-a", "clip-b"], "slot_ids": [], "full_invalidation": False,
        }
        ack = {
            "command_id": "cmd-insert-1", "status": "accepted", "revision": 9,
            "semantic_digest": "sha256:fixture", "reason": None,
            "footprint": footprint, "project_id": "p1", "graph_id": "g1",
            "authority_incarnation_id": "open-1", "replayed": False,
        }
        self.round_trip("EditAck", ack)
        self.round_trip("ApplyGraphBatch", {
            "batch": batch, "authority_incarnation_id": "open-1",
            "writer_capability": "fixture-only-capability",
        })
        self.round_trip("EditAck", {**ack, "replayed": True})
        for status in ("stale", "invalid", "unauthorized"):
            rejected = {**ack, "status": status, "footprint": None,
                        "semantic_digest": None, "reason": status}
            self.round_trip("EditAck", rejected)
        change = {
            "graph_id": "g1", "command_id": "cmd-insert-1",
            "from_revision": 8, "to_revision": 9,
            "operations": batch["operations"], "footprint": footprint,
            "project_id": "p1",
        }
        self.round_trip("AcceptedChange", change)
        self.round_trip("GraphSnapshotDelivery", {
            "snapshot": graph, "authority_incarnation_id": "open-1",
        })
        for kind, event_change in (("ready", None), ("change", change),
                                   ("recovery_required", None)):
            self.round_trip("GraphChangeEvent", {
                "kind": kind, "project_id": "p1", "graph_id": "g1",
                "authority_incarnation_id": "open-1", "revision": 9,
                "first_available_revision": 1, "change": event_change,
                "reason": "expired" if kind == "recovery_required" else None,
            })
        self.round_trip("EditorAck", {
            "contract_version": 1, "project_id": "p1", "graph_id": "g1",
            "sequence_id": "s1", "authority_incarnation_id": "open-1",
            "command_id": "cmd-insert-1", "status": "accepted",
            "replayed": False, "revision": 9, "binding_set_id": "bind-2",
            "footprint": footprint, "diagnostic_id": None,
            "binding_revision": 2, "created_slot_id": "camera-c",
        })
        self.round_trip("HistoryState", {
            "contract_version": 1, "project_id": "p1", "graph_id": "g1",
            "sequence_id": "s1", "authority_incarnation_id": "open-1",
            "revision": 9, "undo_groups": [{
                "group_id": "gesture-1", "sequence_id": "s1",
                "label": "Insert source", "command_id": "cmd-insert-1",
            }], "redo_groups": [], "can_undo": True, "can_redo": False,
        })
        catalog_asset = {
            "id": "asset-2", "version_id": "asset-2-v1",
            "content_fingerprint": "sha256:different-bytes",
            "display_name": "Camera B", "availability": "online",
            "diagnostic_id": None, "streams": {"video-0": {
                "id": "video-0", "media": "video",
                "duration": {"numerator": 2, "denominator": 1},
                "pts_origin": {"numerator": 0, "denominator": 1},
                "frame_count": 48, "frame_index_id": "index-2",
                "index_status": "ready",
            }},
        }
        self.round_trip("CatalogSnapshot", {
            "contract_version": 1, "project_id": "p1",
            "authority_incarnation_id": "open-1", "catalog_revision": 1,
            "assets": {"asset-2": catalog_asset},
        })
        self.round_trip("CatalogChangeEvent", {
            "kind": "change", "project_id": "p1",
            "authority_incarnation_id": "open-1",
            "catalog_revision": 2, "first_available_revision": 1,
            "change": {"project_id": "p1", "from_catalog_revision": 1,
                       "to_catalog_revision": 2,
                       "upserts": {"asset-2": {**catalog_asset, "availability": "missing",
                                              "diagnostic_id": "missing-asset"}},
                       "removed_asset_ids": []},
            "reason": None,
        })
        self.round_trip("FrameIndexResult", {
            "contract_version": 1, "project_id": "p1",
            "authority_incarnation_id": "open-1",
            "registered_asset_id": "asset-2", "asset_version_id": "asset-2-v1",
            "stream_id": "video-0", "frame_index": 24, "status": "ready",
            "pts": {"numerator": 1001, "denominator": 1000},
            "frame_count": 48, "content_fingerprint": "sha256:different-bytes",
            "diagnostic_id": None,
        })
        binding = {
            "contract_version": 1, "project_id": "p1", "graph_id": "g1",
            "authority_incarnation_id": "open-1", "binding_set_id": "bind-2",
            "binding_revision": 2,
            "bindings": {"camera-a": {
                "slot_id": "camera-a", "registered_asset_id": "asset-2",
                "asset_version_id": "asset-2-v1", "stream_id": "video-0",
                "content_fingerprint": "sha256:different-bytes",
                "media": "video", "duration": {"numerator": 2, "denominator": 1},
            }},
        }
        self.round_trip("BindingSnapshot", binding)
        self.round_trip("BindingChange", {
            "project_id": "p1", "graph_id": "g1",
            "from_binding_revision": 1, "to_binding_revision": 2,
            "from_binding_set_id": "bind-1", "to_binding_set_id": "bind-2",
            "changed_slot_ids": ["camera-a"], "footprint": {
                **footprint, "slot_ids": ["camera-a"],
            },
        })
        resource = {
            "resource_id": "frame-1", "lease_id": "lease-1",
            "mime_type": "image/png", "width": 1280, "height": 720,
            "byte_length": 1234, "expires_at_unix_ms": 1770000000000,
        }
        sequence_request = {
            "contract_version": 1, "project_id": "p1", "graph_id": "g1",
            "sequence_id": "s1", "authority_incarnation_id": "open-1",
            "accepted_revision": 9, "binding_set_id": "bind-2",
            "binding_revision": 2, "request_id": "seek-1",
            "viewer_id": "viewer-1", "cancel_group_id": "scrub-1",
            "at": {"numerator": 5, "denominator": 1},
            "fidelity": "exact", "max_edge_px": 1280,
        }
        self.round_trip("SequencePreviewRequest", sequence_request)
        self.round_trip("SequencePreviewResult", {
            **{k: v for k, v in sequence_request.items() if k not in ("at", "max_edge_px")},
            "status": "ready", "actual_time": sequence_request["at"],
            "plan_id": "plan-1", "resource": resource,
            "error_code": None, "diagnostic_id": None,
        })
        source_request = {
            "contract_version": 1, "project_id": "p1",
            "authority_incarnation_id": "open-1",
            "registered_asset_id": "asset-2", "asset_version_id": "asset-2-v1",
            "stream_id": "video-0", "request_id": "source-1",
            "viewer_id": "viewer-2", "cancel_group_id": "source-scrub-1",
            "at": {"numerator": 1, "denominator": 1},
            "fidelity": "exact", "max_edge_px": 1280,
            "expected_content_fingerprint": "sha256:different-bytes",
        }
        self.assertNotIn("graph_id", source_request)
        self.round_trip("SourcePreviewRequest", source_request)
        self.round_trip("SourcePreviewResult", {
            **{k: v for k, v in source_request.items()
               if k not in ("at", "max_edge_px", "expected_content_fingerprint")},
            "status": "ready", "actual_time": source_request["at"],
            "resource": resource, "error_code": None, "diagnostic_id": None,
            "content_fingerprint": "sha256:different-bytes",
        })

    def test_export_job_wire_context_and_recovery_shapes(self) -> None:
        host_context = {
            "request_id": "export-a", "provenance": "governed_host",
            "project_id": "p1", "graph_id": "g1", "sequence_id": "s1",
            "authority_incarnation_id": "open-1", "local_import_id": None,
            "accepted_revision": 9, "binding_set_id": "bind-2",
            "binding_revision": 2, "profile_id": "web-h264",
            "profile_version": 3, "profile_digest": "sha256:profile-v3",
        }
        other_context = {
            **host_context, "request_id": "export-b", "accepted_revision": 8,
            "binding_set_id": "bind-1", "binding_revision": 1,
        }
        local_context = {
            **host_context, "request_id": "offline-a",
            "provenance": "standalone_import",
            "authority_incarnation_id": None, "local_import_id": "import-1",
        }
        for context in (host_context, other_context, local_context):
            self.round_trip("ExportJobRequest", {
                "contract_version": 1, "context": context,
                "range": {"start": {"numerator": 0, "denominator": 1},
                          "end": {"numerator": 2, "denominator": 1}},
                "destination_ref": "opaque-destination-1",
                "allow_software_fallback": True,
                "destination_policy": "replace_existing",
            })
        standalone_request = fixture("export_job_request_standalone")
        self.assertEqual(standalone_request["context"]["provenance"],
                         "standalone_import")
        self.assertIsNone(standalone_request["context"]["authority_incarnation_id"])
        self.assertEqual(standalone_request["context"]["local_import_id"],
                         "import-1")
        self.round_trip("ExportJobRequest", standalone_request)
        self.round_trip("ExportJobAck", {
            "contract_version": 1, "context": host_context,
            "status": "accepted", "job_id": "job-a",
            "replayed": False, "diagnostic_id": None,
        })
        self.round_trip("ExportJobAck", {
            "contract_version": 1, "context": host_context,
            "status": "accepted", "job_id": "job-a",
            "replayed": True, "diagnostic_id": None,
        })
        for status in ("destination_busy", "idempotency_conflict"):
            self.round_trip("ExportJobAck", {
                "contract_version": 1, "context": other_context,
                "status": status, "job_id": None,
                "replayed": False, "diagnostic_id": "rejected",
            })
        result = {
            "export_record_id": "record-a", "output_artifact_id": "artifact-a",
            "probe_summary": {
                "video_stream_count": 1, "audio_stream_count": 1,
                "duration": {"numerator": 2, "denominator": 1},
                "width": 1280, "height": 720, "audio_sample_rate": 48000,
                "audio_channel_layout": "stereo",
            },
            "output_probe_digest": "sha256:probe-a",
            "destination_generation_id": "generation-a",
        }
        for sequence, kind, state, progress, event_result in (
            (0, "ready", "queued", None, None),
            (1, "progress", "running", {"numerator": 1, "denominator": 2}, None),
            (2, "state_change", "succeeded", None, result),
        ):
            self.round_trip("ExportJobEvent", {
                "contract_version": 1, "context": host_context,
                "job_id": "job-a", "event_sequence": sequence,
                "first_available_sequence": 0, "kind": kind,
                "state": state, "progress": progress, "result": event_result,
                "diagnostic_id": None,
            })
        for name in ("export_events_query_governed",
                     "export_events_query_standalone"):
            query = fixture(name)
            self.assertEqual(query["caller_incarnation_id"] is None,
                             query["local_import_id"] is not None)
            self.round_trip("ExportEventsQuery", query)
        for name in ("export_event_delivery_event",
                     "export_event_delivery_unavailable"):
            delivery = fixture(name)
            self.assertEqual(delivery["event"] is not None,
                             delivery["status"] == "event")
            self.round_trip("ExportEventDelivery", delivery)
        self.round_trip("ExportStatusQuery", {
            "contract_version": 1, "project_id": "p1", "job_id": "job-a",
            "caller_incarnation_id": "open-2", "local_import_id": None,
        })
        status_snapshot = {
            "contract_version": 1, "context": host_context,
            "job_id": "job-a", "state": "succeeded",
            "last_event_sequence": 2, "result": result,
            "diagnostic_id": None,
        }
        self.round_trip("ExportStatusSnapshot", status_snapshot)
        for name in ("export_status_found", "export_status_unavailable",
                     "export_status_stale"):
            status_result = fixture(name)
            self.assertEqual(status_result["snapshot"] is not None,
                             status_result["status"] == "found")
            self.round_trip("ExportStatusResult", status_result)
        self.round_trip("ExportLookupQuery", {
            "contract_version": 1, "project_id": "p1",
            "request_id": "export-a", "caller_incarnation_id": "open-2",
            "local_import_id": None,
        })
        self.round_trip("ExportLookupResult", {
            "contract_version": 1, "project_id": "p1",
            "request_id": "export-a", "status": "found",
            "snapshot": status_snapshot, "diagnostic_id": None,
        })
        self.round_trip("ExportLookupResult", {
            "contract_version": 1, "project_id": "p1",
            "request_id": "not-visible", "status": "unavailable",
            "snapshot": None, "diagnostic_id": None,
        })
        retired_ack = fixture("export_job_ack_retired")
        self.assertEqual(retired_ack["status"], "retired_request")
        self.assertIsNone(retired_ack["job_id"])
        self.assertFalse(retired_ack["replayed"])
        self.round_trip("ExportJobAck", retired_ack)
        standalone_lookup = fixture("export_lookup_query_standalone")
        self.assertIsNone(standalone_lookup["caller_incarnation_id"])
        self.assertEqual(standalone_lookup["local_import_id"], "import-1")
        self.round_trip("ExportLookupQuery", standalone_lookup)
        retired_lookup = fixture("export_lookup_retired")
        self.assertEqual(retired_lookup["status"], "retired_request")
        self.assertIsNone(retired_lookup["snapshot"])
        self.round_trip("ExportLookupResult", retired_lookup)
        self.round_trip("CancelExportJob", {
            "contract_version": 1, "project_id": "p1", "job_id": "job-b",
            "command_id": "cancel-b", "caller_incarnation_id": "open-1",
            "local_import_id": None,
        })
        self.round_trip("ExportCancelAck", {
            "contract_version": 1, "project_id": "p1", "job_id": "job-b",
            "command_id": "cancel-b", "status": "accepted",
            "terminal_state": None,
        })
        unavailable_cancel = fixture("export_cancel_unavailable")
        self.assertEqual(unavailable_cancel["status"], "unavailable")
        self.assertIsNone(unavailable_cancel["terminal_state"])
        self.round_trip("ExportCancelAck", unavailable_cancel)


if __name__ == "__main__":
    unittest.main()
