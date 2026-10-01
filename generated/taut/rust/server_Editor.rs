// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait EditorHandlers {
    fn editor_insert_source_span(&self, request: InsertSourceSpan) -> EditorAck;
    fn editor_undo(&self, request: HistoryIntent) -> EditorAck;
    fn editor_redo(&self, request: HistoryIntent) -> EditorAck;
    fn editor_history(&self, project_id: String, graph_id: String, sequence_id: String, authority_incarnation_id: String) -> HistoryState;
}
// register(): for m in schema.services["Editor"].methods { transport.register_method(m, ..) }
