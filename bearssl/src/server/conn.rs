use core::marker::PhantomData;
use core::mem::transmute;

use bearssl_sys::*;

use crate::engine::tls;
use crate::x509::cert::X509Certificate;

/// Represents a connected client.
#[repr(transparent)]
pub struct ServerConnection<'a> {
    pub(crate) context: br_ssl_server_context,
    pub(crate) chain: PhantomData<&'a X509Certificate<'a>>,
}

impl<'a> ServerConnection<'a> {
    pub fn push_write(&mut self, src: &[u8]) -> Result<usize, tls::Error> {
        let engine: &mut tls::TlsEngine = unsafe { transmute(&mut self.context.eng) };

        engine.push_write(src)
    }

    pub fn pull_write(&mut self, dst: &mut [u8]) -> Result<usize, tls::Error> {
        let engine: &mut tls::TlsEngine = unsafe { transmute(&mut self.context.eng) };

        engine.pull_write(dst)
    }

    pub fn push_read(&mut self, src: &[u8]) -> Result<usize, tls::Error> {
        let engine: &mut tls::TlsEngine = unsafe { transmute(&mut self.context.eng) };

        engine.push_read(src)
    }

    pub fn pull_read(&mut self, dst: &mut [u8]) -> Result<usize, tls::Error> {
        let engine: &mut tls::TlsEngine = unsafe { transmute(&mut self.context.eng) };

        engine.pull_read(dst)
    }
}
