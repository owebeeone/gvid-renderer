// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { CancelExportJob, ExportJobRequest, ExportLookupQuery, ExportStatusQuery } from "./api.ts";

export class ExportJobsClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  export_submit(request: ExportJobRequest): Promise<api.ExportJobAck> {
    return this.c.call("export.submit", { request }) as Promise<api.ExportJobAck>;
  }
  export_lookup(request: ExportLookupQuery): Promise<api.ExportLookupResult> {
    return this.c.call("export.lookup", { request }) as Promise<api.ExportLookupResult>;
  }
  export_events(project_id: string, job_id: string, after_event_sequence: bigint, onEvent: (event: string, value: unknown) => void): () => void {  // log
    return this.c.subscribe("export.events", { project_id, job_id, after_event_sequence }, onEvent);
  }
  export_status(request: ExportStatusQuery): Promise<api.ExportStatusSnapshot> {
    return this.c.call("export.status", { request }) as Promise<api.ExportStatusSnapshot>;
  }
  export_cancel(request: CancelExportJob): Promise<api.ExportCancelAck> {
    return this.c.call("export.cancel", { request }) as Promise<api.ExportCancelAck>;
  }
}
