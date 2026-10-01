"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class PreviewClient:
    def __init__(self, transport):
        self._t = transport

    async def preview_source_frame(self, request: SourcePreviewRequest) -> SourcePreviewResult:
        return await self._t.call("preview.source_frame", SourcePreviewResult, request=request)

    async def preview_sequence_frame(self, request: SequencePreviewRequest) -> SequencePreviewResult:
        return await self._t.call("preview.sequence_frame", SequencePreviewResult, request=request)

    async def preview_cancel(self, request: CancelPreview) -> ResourceActionAck:
        return await self._t.call("preview.cancel", ResourceActionAck, request=request)

    async def preview_release(self, request: ReleaseResource) -> ResourceActionAck:
        return await self._t.call("preview.release", ResourceActionAck, request=request)
