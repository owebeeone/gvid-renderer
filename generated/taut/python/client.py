"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class RenderGraphClient:
    def __init__(self, transport):
        self._t = transport

    async def snapshot(self, project_id: str, graph_id: str) -> GraphSnapshot:
        return await self._t.call("snapshot", GraphSnapshot, project_id=project_id, graph_id=graph_id)

    async def apply(self, batch: EditBatch) -> EditAck:
        return await self._t.call("apply", EditAck, batch=batch)

    def changes(self, project_id: str, graph_id: str):  # log stream
        return self._t.subscribe("changes", project_id=project_id, graph_id=graph_id)
