// GENERATED typed client over the generic tautClient (call/subscribe).
import type { tautClient } from "./taut_client.ts";
import type * as api from "./api.ts";
import type { CancelExportJob, ExportEventsQuery, ExportJobRequest, ExportLookupQuery, ExportStatusQuery } from "./api.ts";

export class ExportJobsClient {
  private c: tautClient;
  constructor(c: tautClient) { this.c = c; }
  export_submit(request: ExportJobRequest): Promise<api.ExportJobAck> {
    return this.c.call("export.submit", { request }) as Promise<api.ExportJobAck>;
  }
  export_lookup(request: ExportLookupQuery): Promise<api.ExportLookupResult> {
    return this.c.call("export.lookup", { request }) as Promise<api.ExportLookupResult>;
  }
  export_events(request: ExportEventsQuery, onEvent: (event: string, value: unknown) => void): () => void {  // log
    return this.c.subscribe("export.events", { request }, onEvent);
  }
  export_status(request: ExportStatusQuery): Promise<api.ExportStatusResult> {
    return this.c.call("export.status", { request }) as Promise<api.ExportStatusResult>;
  }
  export_cancel(request: CancelExportJob): Promise<api.ExportCancelAck> {
    return this.c.call("export.cancel", { request }) as Promise<api.ExportCancelAck>;
  }
}
