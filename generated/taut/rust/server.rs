// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait RenderGraphHandlers {
    fn snapshot(&self, project_id: String, graph_id: String) -> GraphSnapshot;
    fn apply(&self, batch: EditBatch) -> EditAck;
    // changes: returns a subscription (log)
}
// register(): for m in schema.services["RenderGraph"].methods { transport.register_method(m, ..) }
