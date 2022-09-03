use core::marker::PhantomData;

use bearssl_sys::*;

use crate::x509::cert::X509Certificate;

/// Represents a connected client.
#[repr(transparent)]
pub struct ServerConnection<'a> {
    pub(crate) context: br_ssl_server_context,
    pub(crate) chain: PhantomData<&'a X509Certificate<'a>>,
}

impl<'a> ServerConnection<'a> {
    pub fn push_bytes(&self, data: &[u8]) {}
}
