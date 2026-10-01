// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait AssetBindingsHandlers {
    fn bindings_snapshot(&self, project_id: String, graph_id: String) -> BindingSnapshot;
    fn bindings_rebind(&self, request: RebindSlot) -> BindingAck;
    // bindings.changes: returns a subscription (log)
}
// register(): for m in schema.services["AssetBindings"].methods { transport.register_method(m, ..) }
