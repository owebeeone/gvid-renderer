// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct ExportJobsClient<'a> { c: &'a Client }

impl<'a> ExportJobsClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // export.submit(request) -> ExportJobAck
    // self.c.call("export.submit", &[..encode args..]).await -> ExportJobAck::from_cbor(..)
    // export.lookup(request) -> ExportLookupResult
    // self.c.call("export.lookup", &[..encode args..]).await -> ExportLookupResult::from_cbor(..)
    // export.events: subscribe ("log") -> stream of ['append']
    // export.status(request) -> ExportStatusSnapshot
    // self.c.call("export.status", &[..encode args..]).await -> ExportStatusSnapshot::from_cbor(..)
    // export.cancel(request) -> ExportCancelAck
    // self.c.call("export.cancel", &[..encode args..]).await -> ExportCancelAck::from_cbor(..)
}
