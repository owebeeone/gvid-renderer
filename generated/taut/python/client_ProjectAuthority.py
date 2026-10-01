"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class ProjectAuthorityClient:
    def __init__(self, transport):
        self._t = transport

    async def authority_open(self, project_id: str) -> ProjectOpen:
        return await self._t.call("authority.open", ProjectOpen, project_id=project_id)

    async def authority_acquire_writer(self, request: AcquireWriter) -> WriterLease:
        return await self._t.call("authority.acquire_writer", WriterLease, request=request)
