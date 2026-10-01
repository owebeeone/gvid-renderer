"""Semantic shape checks for the draft ExportJobs owner-scope wire.

This reference validator cannot authenticate a caller or prove a current grant.
Host and CLI adapters must apply equivalent checks before privileged lookup.
"""

from __future__ import annotations

from typing import Any


def _nonempty(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def validate_export_job_context(context: dict[str, Any]) -> None:
    """Require exactly the governed owner or standalone import provenance."""
    if not isinstance(context, dict):
        raise ValueError("export context must be a record")
    provenance = context.get("provenance")
    owner_scope = context.get("owner_scope_id")
    incarnation = context.get("authority_incarnation_id")
    local_import = context.get("local_import_id")
    if provenance == "governed_host":
        if not _nonempty(owner_scope) or not _nonempty(incarnation) or local_import is not None:
            raise ValueError("governed export requires owner scope and host incarnation")
    elif provenance == "standalone_import":
        if not _nonempty(local_import) or owner_scope is not None or incarnation is not None:
            raise ValueError("standalone export requires import without host owner scope")
    else:
        raise ValueError("unknown export provenance")


def validate_export_lookup_query(query: dict[str, Any]) -> None:
    """Require one complete lookup authority branch before index access."""
    if not isinstance(query, dict):
        raise ValueError("export lookup query must be a record")
    owner_scope = query.get("owner_scope_id")
    incarnation = query.get("caller_incarnation_id")
    local_import = query.get("local_import_id")
    if _nonempty(incarnation) and local_import is None:
        if not _nonempty(owner_scope):
            raise ValueError("governed lookup requires owner scope")
    elif _nonempty(local_import) and incarnation is None:
        if owner_scope is not None:
            raise ValueError("standalone lookup forbids owner scope")
    else:
        raise ValueError("export lookup has invalid authority branch")


def validate_export_lookup_echo(query: dict[str, Any], result: dict[str, Any]) -> None:
    """Check the response echoes only the requested namespace and identity."""
    validate_export_lookup_query(query)
    if not isinstance(result, dict) or any(
        result.get(field) != query.get(field)
        for field in ("project_id", "request_id", "owner_scope_id")
    ):
        raise ValueError("export lookup response does not echo query identity")


def validate_export_submit_denial(request: dict[str, Any], ack: dict[str, Any]) -> None:
    """Bind an authorization denial to its request without accepted-job hints."""
    if not isinstance(request, dict) or not isinstance(request.get("context"), dict):
        raise ValueError("export denial requires the originating request")
    if not isinstance(ack, dict) or ack.get("status") not in {"unauthorized", "stale_context"}:
        raise ValueError("expected export authorization denial")
    if ack.get("contract_version") != request.get("contract_version") or ack.get("context") != request["context"]:
        raise ValueError("export authorization denial does not echo request")
    if ack.get("job_id") is not None or ack.get("diagnostic_id") is not None or ack.get("replayed") is not False:
        raise ValueError("export authorization denial discloses job data")
