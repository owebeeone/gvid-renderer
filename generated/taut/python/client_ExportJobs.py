"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class ExportJobsClient:
    def __init__(self, transport):
        self._t = transport

    async def export_submit(self, request: ExportJobRequest) -> ExportJobAck:
        return await self._t.call("export.submit", ExportJobAck, request=request)

    async def export_lookup(self, request: ExportLookupQuery) -> ExportLookupResult:
        return await self._t.call("export.lookup", ExportLookupResult, request=request)

    def export_events(self, project_id: str, job_id: str, after_event_sequence: int):  # log stream
        return self._t.subscribe("export.events", project_id=project_id, job_id=job_id, after_event_sequence=after_event_sequence)

    async def export_status(self, request: ExportStatusQuery) -> ExportStatusSnapshot:
        return await self._t.call("export.status", ExportStatusSnapshot, request=request)

    async def export_cancel(self, request: CancelExportJob) -> ExportCancelAck:
        return await self._t.call("export.cancel", ExportCancelAck, request=request)
