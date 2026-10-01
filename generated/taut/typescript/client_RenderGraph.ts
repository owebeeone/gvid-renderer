// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { ApplyGraphBatch } from "./api.ts";

export class RenderGraphClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  snapshot(project_id: string, graph_id: string): Promise<api.GraphSnapshotDelivery> {
    return this.c.call("snapshot", { project_id, graph_id }) as Promise<api.GraphSnapshotDelivery>;
  }
  apply(request: ApplyGraphBatch): Promise<api.EditAck> {
    return this.c.call("apply", { request }) as Promise<api.EditAck>;
  }
  changes(project_id: string, graph_id: string, authority_incarnation_id: string, after_revision: bigint, onEvent: (event: string, value: unknown) => void): () => void {  // log
    return this.c.subscribe("changes", { project_id, graph_id, authority_incarnation_id, after_revision }, onEvent);
  }
}
