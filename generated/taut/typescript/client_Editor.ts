// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { HistoryIntent, InsertSourceSpan } from "./api.ts";

export class EditorClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  editor_insert_source_span(request: InsertSourceSpan): Promise<api.EditorAck> {
    return this.c.call("editor.insert_source_span", { request }) as Promise<api.EditorAck>;
  }
  editor_undo(request: HistoryIntent): Promise<api.EditorAck> {
    return this.c.call("editor.undo", { request }) as Promise<api.EditorAck>;
  }
  editor_redo(request: HistoryIntent): Promise<api.EditorAck> {
    return this.c.call("editor.redo", { request }) as Promise<api.EditorAck>;
  }
  editor_history(project_id: string, graph_id: string, sequence_id: string, authority_incarnation_id: string): Promise<api.HistoryState> {
    return this.c.call("editor.history", { project_id, graph_id, sequence_id, authority_incarnation_id }) as Promise<api.HistoryState>;
  }
}
