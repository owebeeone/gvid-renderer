// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct MediaCatalogClient<'a> { c: &'a Client }

impl<'a> MediaCatalogClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // catalog.snapshot(project_id, authority_incarnation_id) -> CatalogSnapshot
    // self.c.call("catalog.snapshot", &[..encode args..]).await -> CatalogSnapshot::from_cbor(..)
    // catalog.changes: subscribe ("log") -> stream of ['append']
    // catalog.frame_at_index(request) -> FrameIndexResult
    // self.c.call("catalog.frame_at_index", &[..encode args..]).await -> FrameIndexResult::from_cbor(..)
}
