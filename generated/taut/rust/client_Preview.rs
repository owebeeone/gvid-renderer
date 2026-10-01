// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct PreviewClient<'a> { c: &'a Client }

impl<'a> PreviewClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // preview.source_frame(request) -> SourcePreviewResult
    // self.c.call("preview.source_frame", &[..encode args..]).await -> SourcePreviewResult::from_cbor(..)
    // preview.sequence_frame(request) -> SequencePreviewResult
    // self.c.call("preview.sequence_frame", &[..encode args..]).await -> SequencePreviewResult::from_cbor(..)
    // preview.cancel(request) -> ResourceActionAck
    // self.c.call("preview.cancel", &[..encode args..]).await -> ResourceActionAck::from_cbor(..)
    // preview.release(request) -> ResourceActionAck
    // self.c.call("preview.release", &[..encode args..]).await -> ResourceActionAck::from_cbor(..)
}
