// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";

export interface RenderGraphHandlers {
  snapshot(project_id: string, graph_id: string): Promise<api.GraphSnapshot>;
  apply(batch: EditBatch): Promise<api.EditAck>;
  changes(project_id: string, graph_id: string): unknown;  // Subscription (log)
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: RenderGraphHandlers): void {
  const bind: Record<string, unknown> = {
    "snapshot": h.snapshot.bind(h),
    "apply": h.apply.bind(h),
    "changes": h.changes.bind(h),
  };
  for (const m of schema.services["RenderGraph"].methods) transport.registerMethod(m, bind[m.name]);
}
