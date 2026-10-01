// GENERATED server handler trait + registration sketch.
use crate::api::*;

pub trait ProjectAuthorityHandlers {
    fn authority_open(&self, project_id: String) -> ProjectOpen;
    fn authority_acquire_writer(&self, request: AcquireWriter) -> WriterLease;
}
// register(): for m in schema.services["ProjectAuthority"].methods { transport.register_method(m, ..) }
