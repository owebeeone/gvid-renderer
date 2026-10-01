// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { RebindSlot } from "./api.ts";

export class AssetBindingsClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  bindings_snapshot(project_id: string, graph_id: string): Promise<api.BindingSnapshot> {
    return this.c.call("bindings.snapshot", { project_id, graph_id }) as Promise<api.BindingSnapshot>;
  }
  bindings_rebind(request: RebindSlot): Promise<api.BindingAck> {
    return this.c.call("bindings.rebind", { request }) as Promise<api.BindingAck>;
  }
  bindings_changes(project_id: string, graph_id: string, authority_incarnation_id: string, after_binding_revision: bigint, onEvent: (event: string, value: unknown) => void): () => void {  // log
    return this.c.subscribe("bindings.changes", { project_id, graph_id, authority_incarnation_id, after_binding_revision }, onEvent);
  }
}
