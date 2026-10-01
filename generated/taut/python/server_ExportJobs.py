"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class ExportJobsHandlers(Protocol):
    async def export_submit(self, request: ExportJobRequest) -> ExportJobAck: ...
    async def export_lookup(self, request: ExportLookupQuery) -> ExportLookupResult: ...
    def export_events(self, project_id: str, job_id: str, after_event_sequence: int): ...  # -> Subscription (log)
    async def export_status(self, request: ExportStatusQuery) -> ExportStatusSnapshot: ...
    async def export_cancel(self, request: CancelExportJob) -> ExportCancelAck: ...

def register(transport, schema, handlers: "ExportJobsHandlers") -> None:
    bind = {
        "export.submit": handlers.export_submit,
        "export.lookup": handlers.export_lookup,
        "export.events": handlers.export_events,
        "export.status": handlers.export_status,
        "export.cancel": handlers.export_cancel,
    }
    for m in schema.services["ExportJobs"].methods:
        transport.register_method(m, bind[m.name])
