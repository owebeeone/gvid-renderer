"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class MediaCatalogClient:
    def __init__(self, transport):
        self._t = transport

    async def catalog_snapshot(self, project_id: str, authority_incarnation_id: str) -> CatalogSnapshot:
        return await self._t.call("catalog.snapshot", CatalogSnapshot, project_id=project_id, authority_incarnation_id=authority_incarnation_id)

    def catalog_changes(self, project_id: str, authority_incarnation_id: str, after_catalog_revision: int):  # log stream
        return self._t.subscribe("catalog.changes", project_id=project_id, authority_incarnation_id=authority_incarnation_id, after_catalog_revision=after_catalog_revision)

    async def catalog_frame_at_index(self, request: FrameIndexQuery) -> FrameIndexResult:
        return await self._t.call("catalog.frame_at_index", FrameIndexResult, request=request)
