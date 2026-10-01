"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class PreviewHandlers(Protocol):
    async def preview_source_frame(self, request: SourcePreviewRequest) -> SourcePreviewResult: ...
    async def preview_sequence_frame(self, request: SequencePreviewRequest) -> SequencePreviewResult: ...
    async def preview_cancel(self, request: CancelPreview) -> ResourceActionAck: ...
    async def preview_release(self, request: ReleaseResource) -> ResourceActionAck: ...

def register(transport, schema, handlers: "PreviewHandlers") -> None:
    bind = {
        "preview.source_frame": handlers.preview_source_frame,
        "preview.sequence_frame": handlers.preview_sequence_frame,
        "preview.cancel": handlers.preview_cancel,
        "preview.release": handlers.preview_release,
    }
    for m in schema.services["Preview"].methods:
        transport.register_method(m, bind[m.name])
