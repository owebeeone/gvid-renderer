"""Semantic privacy checks for draft ExportJobs response envelopes.

Taut validates field types, but does not express the status-dependent
presence rules. The host and CLI adapters must apply equivalent validation
after decoding and before sending an ExportJobs response.
"""

from __future__ import annotations

from typing import Any

_BRANCHES = {
    "ExportLookupResult": ("found", "snapshot",
                           {"unavailable", "stale_context", "retired_request"}),
    "ExportStatusResult": ("found", "snapshot",
                           {"unavailable", "stale_context"}),
    "ExportEventDelivery": ("event", "event",
                            {"unavailable", "stale_context"}),
}


def validate_export_response(message: str, value: dict[str, Any]) -> None:
    """Reject a negative response that carries job data or a diagnostic ID."""
    if message not in _BRANCHES or not isinstance(value, dict):
        raise ValueError("unsupported export response")
    positive, payload_field, negative = _BRANCHES[message]
    status = value.get("status")
    payload = value.get(payload_field)
    if status == positive:
        if not isinstance(payload, dict):
            raise ValueError("positive export response requires job payload")
        return
    if status not in negative:
        raise ValueError("invalid export response status")
    if payload is not None or value.get("diagnostic_id") is not None:
        raise ValueError("negative export response discloses job data")
