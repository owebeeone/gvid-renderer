// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait ExportJobsHandlers {
    fn export_submit(&self, request: ExportJobRequest) -> ExportJobAck;
    fn export_lookup(&self, request: ExportLookupQuery) -> ExportLookupResult;
    // export.events: returns a subscription (log)
    fn export_status(&self, request: ExportStatusQuery) -> ExportStatusSnapshot;
    fn export_cancel(&self, request: CancelExportJob) -> ExportCancelAck;
}
// register(): for m in schema.services["ExportJobs"].methods { transport.register_method(m, ..) }
