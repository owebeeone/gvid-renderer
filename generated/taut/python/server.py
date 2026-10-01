"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class RenderGraphHandlers(Protocol):
    async def snapshot(self, project_id: str, graph_id: str) -> GraphSnapshot: ...
    async def apply(self, batch: EditBatch) -> EditAck: ...
    def changes(self, project_id: str, graph_id: str): ...  # -> Subscription (log)

def register(transport, schema, handlers: "RenderGraphHandlers") -> None:
    bind = {
        "snapshot": handlers.snapshot,
        "apply": handlers.apply,
        "changes": handlers.changes,
    }
    for m in schema.services["RenderGraph"].methods:
        transport.register_method(m, bind[m.name])
