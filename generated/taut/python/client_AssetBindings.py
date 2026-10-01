"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class AssetBindingsClient:
    def __init__(self, transport):
        self._t = transport

    async def bindings_snapshot(self, project_id: str, graph_id: str) -> BindingSnapshot:
        return await self._t.call("bindings.snapshot", BindingSnapshot, project_id=project_id, graph_id=graph_id)

    async def bindings_rebind(self, request: RebindSlot) -> BindingAck:
        return await self._t.call("bindings.rebind", BindingAck, request=request)

    def bindings_changes(self, project_id: str, graph_id: str, authority_incarnation_id: str, after_binding_revision: int):  # log stream
        return self._t.subscribe("bindings.changes", project_id=project_id, graph_id=graph_id, authority_incarnation_id=authority_incarnation_id, after_binding_revision=after_binding_revision)
