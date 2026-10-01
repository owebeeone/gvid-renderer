// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";

export class RenderGraphClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  snapshot(project_id: string, graph_id: string): Promise<api.GraphSnapshot> {
    return this.c.call("snapshot", { project_id, graph_id }) as Promise<api.GraphSnapshot>;
  }
  apply(batch: EditBatch): Promise<api.EditAck> {
    return this.c.call("apply", { batch }) as Promise<api.EditAck>;
  }
  changes(project_id: string, graph_id: string, onEvent: (event: string, value: unknown) => void): () => void {  // log
    return this.c.subscribe("changes", { project_id, graph_id }, onEvent);
  }
}
