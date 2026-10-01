// GENERATED typed client over the generic taut Client.
use crate::api::*;
use crate::client::Client;
use crate::cbor::Cbor;

pub struct ProjectAuthorityClient<'a> { c: &'a Client }

impl<'a> ProjectAuthorityClient<'a> {
    pub fn new(c: &'a Client) -> Self { Self { c } }
    // authority.open(project_id) -> ProjectOpen
    // self.c.call("authority.open", &[..encode args..]).await -> ProjectOpen::from_cbor(..)
    // authority.acquire_writer(request) -> WriterLease
    // self.c.call("authority.acquire_writer", &[..encode args..]).await -> WriterLease::from_cbor(..)
}
