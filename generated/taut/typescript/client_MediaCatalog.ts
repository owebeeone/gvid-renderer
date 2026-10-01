// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { FrameIndexQuery } from "./api.ts";

export class MediaCatalogClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  catalog_snapshot(project_id: string, authority_incarnation_id: string): Promise<api.CatalogSnapshot> {
    return this.c.call("catalog.snapshot", { project_id, authority_incarnation_id }) as Promise<api.CatalogSnapshot>;
  }
  catalog_changes(project_id: string, authority_incarnation_id: string, after_catalog_revision: bigint, onEvent: (event: string, value: unknown) => void): () => void {  // log
    return this.c.subscribe("catalog.changes", { project_id, authority_incarnation_id, after_catalog_revision }, onEvent);
  }
  catalog_frame_at_index(request: FrameIndexQuery): Promise<api.FrameIndexResult> {
    return this.c.call("catalog.frame_at_index", { request }) as Promise<api.FrameIndexResult>;
  }
}
