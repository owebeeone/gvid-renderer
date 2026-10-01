// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";
import type { FrameIndexQuery } from "./api.ts";

export interface MediaCatalogHandlers {
  catalog_snapshot(project_id: string, authority_incarnation_id: string): Promise<api.CatalogSnapshot>;
  catalog_changes(project_id: string, authority_incarnation_id: string, after_catalog_revision: bigint): unknown;  // Subscription (log)
  catalog_frame_at_index(request: FrameIndexQuery): Promise<api.FrameIndexResult>;
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: MediaCatalogHandlers): void {
  const bind: Record<string, unknown> = {
    "catalog.snapshot": h.catalog_snapshot.bind(h),
    "catalog.changes": h.catalog_changes.bind(h),
    "catalog.frame_at_index": h.catalog_frame_at_index.bind(h),
  };
  for (const m of schema.services["MediaCatalog"].methods) transport.registerMethod(m, bind[m.name]);
}
