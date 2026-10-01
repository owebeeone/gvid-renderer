// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct RenderGraphClient<'a> { c: &'a Client }

impl<'a> RenderGraphClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // snapshot(project_id, graph_id) -> GraphSnapshotDelivery
    // self.c.call("snapshot", &[..encode args..]).await -> GraphSnapshotDelivery::from_cbor(..)
    // apply(request) -> EditAck
    // self.c.call("apply", &[..encode args..]).await -> EditAck::from_cbor(..)
    // changes: subscribe ("log") -> stream of ['append']
}
