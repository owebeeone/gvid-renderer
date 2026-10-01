// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait MediaCatalogHandlers {
    fn catalog_snapshot(&self, project_id: String, authority_incarnation_id: String) -> CatalogSnapshot;
    // catalog.changes: returns a subscription (log)
    fn catalog_frame_at_index(&self, request: FrameIndexQuery) -> FrameIndexResult;
}
// register(): for m in schema.services["MediaCatalog"].methods { transport.register_method(m, ..) }
