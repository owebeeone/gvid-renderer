// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct AssetBindingsClient<'a> { c: &'a Client }

impl<'a> AssetBindingsClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // bindings.snapshot(project_id, graph_id) -> BindingSnapshot
    // self.c.call("bindings.snapshot", &[..encode args..]).await -> BindingSnapshot::from_cbor(..)
    // bindings.rebind(request) -> BindingAck
    // self.c.call("bindings.rebind", &[..encode args..]).await -> BindingAck::from_cbor(..)
    // bindings.changes: subscribe ("log") -> stream of ['append']
}
