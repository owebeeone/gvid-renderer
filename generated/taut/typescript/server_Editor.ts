// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";
import type { HistoryIntent, InsertSourceSpan } from "./api.ts";

export interface EditorHandlers {
  editor_insert_source_span(request: InsertSourceSpan): Promise<api.EditorAck>;
  editor_undo(request: HistoryIntent): Promise<api.EditorAck>;
  editor_redo(request: HistoryIntent): Promise<api.EditorAck>;
  editor_history(project_id: string, graph_id: string, sequence_id: string, authority_incarnation_id: string): Promise<api.HistoryState>;
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: EditorHandlers): void {
  const bind: Record<string, unknown> = {
    "editor.insert_source_span": h.editor_insert_source_span.bind(h),
    "editor.undo": h.editor_undo.bind(h),
    "editor.redo": h.editor_redo.bind(h),
    "editor.history": h.editor_history.bind(h),
  };
  for (const m of schema.services["Editor"].methods) transport.registerMethod(m, bind[m.name]);
}
