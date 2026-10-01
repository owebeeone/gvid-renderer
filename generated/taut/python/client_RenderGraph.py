"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class RenderGraphClient:
    def __init__(self, transport):
        self._t = transport

    async def snapshot(self, project_id: str, graph_id: str) -> GraphSnapshotDelivery:
        return await self._t.call("snapshot", GraphSnapshotDelivery, project_id=project_id, graph_id=graph_id)

    async def apply(self, request: ApplyGraphBatch) -> EditAck:
        return await self._t.call("apply", EditAck, request=request)

    def changes(self, project_id: str, graph_id: str, authority_incarnation_id: str, after_revision: int):  # log stream
        return self._t.subscribe("changes", project_id=project_id, graph_id=graph_id, authority_incarnation_id=authority_incarnation_id, after_revision=after_revision)
