// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait PreviewHandlers {
    fn preview_source_frame(&self, request: SourcePreviewRequest) -> SourcePreviewResult;
    fn preview_sequence_frame(&self, request: SequencePreviewRequest) -> SequencePreviewResult;
    fn preview_cancel(&self, request: CancelPreview) -> ResourceActionAck;
    fn preview_release(&self, request: ReleaseResource) -> ResourceActionAck;
}
// register(): for m in schema.services["Preview"].methods { transport.register_method(m, ..) }
