// GENERATED server stubs: a handler interface + IR-driven registration.
import type * as api from "./api.ts";
import type { CancelExportJob, ExportEventsQuery, ExportJobRequest, ExportLookupQuery, ExportStatusQuery } from "./api.ts";

export interface ExportJobsHandlers {
  export_submit(request: ExportJobRequest): Promise<api.ExportJobAck>;
  export_lookup(request: ExportLookupQuery): Promise<api.ExportLookupResult>;
  export_events(request: ExportEventsQuery): unknown;  // Subscription (log)
  export_status(request: ExportStatusQuery): Promise<api.ExportStatusResult>;
  export_cancel(request: CancelExportJob): Promise<api.ExportCancelAck>;
}

// Register against the IR (the transport reads kind/params from the contract):
export function register(transport: any, schema: any, h: ExportJobsHandlers): void {
  const bind: Record<string, unknown> = {
    "export.submit": h.export_submit.bind(h),
    "export.lookup": h.export_lookup.bind(h),
    "export.events": h.export_events.bind(h),
    "export.status": h.export_status.bind(h),
    "export.cancel": h.export_cancel.bind(h),
  };
  for (const m of schema.services["ExportJobs"].methods) transport.registerMethod(m, bind[m.name]);
}
