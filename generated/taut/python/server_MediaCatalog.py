"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class MediaCatalogHandlers(Protocol):
    async def catalog_snapshot(self, project_id: str, authority_incarnation_id: str) -> CatalogSnapshot: ...
    def catalog_changes(self, project_id: str, authority_incarnation_id: str, after_catalog_revision: int): ...  # -> Subscription (log)
    async def catalog_frame_at_index(self, request: FrameIndexQuery) -> FrameIndexResult: ...

def register(transport, schema, handlers: "MediaCatalogHandlers") -> None:
    bind = {
        "catalog.snapshot": handlers.catalog_snapshot,
        "catalog.changes": handlers.catalog_changes,
        "catalog.frame_at_index": handlers.catalog_frame_at_index,
    }
    for m in schema.services["MediaCatalog"].methods:
        transport.register_method(m, bind[m.name])
