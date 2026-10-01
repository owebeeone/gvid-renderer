"""GENERATED server stubs: a handler Protocol + IR-driven registration."""
from __future__ import annotations
from typing import Protocol
from .api import *  # noqa: F401,F403

class EditorHandlers(Protocol):
    async def editor_insert_source_span(self, request: InsertSourceSpan) -> EditorAck: ...
    async def editor_undo(self, request: HistoryIntent) -> EditorAck: ...
    async def editor_redo(self, request: HistoryIntent) -> EditorAck: ...
    async def editor_history(self, project_id: str, graph_id: str, sequence_id: str, authority_incarnation_id: str) -> HistoryState: ...

def register(transport, schema, handlers: "EditorHandlers") -> None:
    bind = {
        "editor.insert_source_span": handlers.editor_insert_source_span,
        "editor.undo": handlers.editor_undo,
        "editor.redo": handlers.editor_redo,
        "editor.history": handlers.editor_history,
    }
    for m in schema.services["Editor"].methods:
        transport.register_method(m, bind[m.name])
