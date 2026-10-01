"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class AssetBindingsHandlers(Protocol):
    async def bindings_snapshot(self, project_id: str, graph_id: str) -> BindingSnapshot: ...
    async def bindings_rebind(self, request: RebindSlot) -> BindingAck: ...
    def bindings_changes(self, project_id: str, graph_id: str, authority_incarnation_id: str, after_binding_revision: int): ...  # -> Subscription (log)

def register(transport, schema, handlers: "AssetBindingsHandlers") -> None:
    bind = {
        "bindings.snapshot": handlers.bindings_snapshot,
        "bindings.rebind": handlers.bindings_rebind,
        "bindings.changes": handlers.bindings_changes,
    }
    for m in schema.services["AssetBindings"].methods:
        transport.register_method(m, bind[m.name])
