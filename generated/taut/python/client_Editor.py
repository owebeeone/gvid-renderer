"""GENERATED typed client over a generic taut transport (call/subscribe)."""
from __future__ import annotations
from .api import *  # noqa: F401,F403

class EditorClient:
    def __init__(self, transport):
        self._t = transport

    async def editor_insert_source_span(self, request: InsertSourceSpan) -> EditorAck:
        return await self._t.call("editor.insert_source_span", EditorAck, request=request)

    async def editor_undo(self, request: HistoryIntent) -> EditorAck:
        return await self._t.call("editor.undo", EditorAck, request=request)

    async def editor_redo(self, request: HistoryIntent) -> EditorAck:
        return await self._t.call("editor.redo", EditorAck, request=request)

    async def editor_history(self, project_id: str, graph_id: str, sequence_id: str, authority_incarnation_id: str) -> HistoryState:
        return await self._t.call("editor.history", HistoryState, project_id=project_id, graph_id=graph_id, sequence_id=sequence_id, authority_incarnation_id=authority_incarnation_id)
