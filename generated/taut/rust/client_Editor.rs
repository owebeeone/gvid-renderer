// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct EditorClient<'a> { c: &'a Client }

impl<'a> EditorClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // editor.insert_source_span(request) -> EditorAck
    // self.c.call("editor.insert_source_span", &[..encode args..]).await -> EditorAck::from_cbor(..)
    // editor.undo(request) -> EditorAck
    // self.c.call("editor.undo", &[..encode args..]).await -> EditorAck::from_cbor(..)
    // editor.redo(request) -> EditorAck
    // self.c.call("editor.redo", &[..encode args..]).await -> EditorAck::from_cbor(..)
    // editor.history(project_id, graph_id, sequence_id, authority_incarnation_id) -> HistoryState
    // self.c.call("editor.history", &[..encode args..]).await -> HistoryState::from_cbor(..)
}
