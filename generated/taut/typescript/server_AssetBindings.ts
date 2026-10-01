// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";
import type { RebindSlot } from "./api.ts";

export interface AssetBindingsHandlers {
  bindings_snapshot(project_id: string, graph_id: string): Promise<api.BindingSnapshot>;
  bindings_rebind(request: RebindSlot): Promise<api.BindingAck>;
  bindings_changes(project_id: string, graph_id: string, authority_incarnation_id: string, after_binding_revision: bigint): unknown;  // Subscription (log)
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: AssetBindingsHandlers): void {
  const bind: Record<string, unknown> = {
    "bindings.snapshot": h.bindings_snapshot.bind(h),
    "bindings.rebind": h.bindings_rebind.bind(h),
    "bindings.changes": h.bindings_changes.bind(h),
  };
  for (const m of schema.services["AssetBindings"].methods) transport.registerMethod(m, bind[m.name]);
}
