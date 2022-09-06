use core::cmp::min;
use core::mem::MaybeUninit;
use core::slice;

use bearssl_sys::*;

pub enum Error {
    /// A session resumption has been attempted with a distinct version or cipher suite.
    SessionResumptionMismatch,

    /// Other side has sent an invalid signature.
    BadSignature,

    /// The error code is unknown or not wrapped around yet.
    Unknown,
}

/// TLS engine used by both server and client connections.
#[repr(transparent)]
pub struct TlsEngine {
    pub(crate) context: br_ssl_engine_context,
}

impl TlsEngine {
    /// Push some plaintext bytes into engine.
    pub fn push_write(&mut self, src: &[u8]) -> Result<usize, Error> {
        let buf = unsafe {
            let mut l = MaybeUninit::<usize>::uninit();

            let b = br_ssl_engine_sendapp_buf(&self.context, l.as_mut_ptr());

            if b.is_null() {
                return Err(Error::Unknown);
            }

            slice::from_raw_parts_mut(b, l.assume_init_read())
        };

        let len = match min(buf.len(), src.len()) {
            0 => return Err(Error::Unknown),
            l => l,
        };

        buf[..len].copy_from_slice(&src[..len]);

        unsafe {
            br_ssl_engine_sendapp_ack(&mut self.context, len);
        }

        Ok(len)
    }

    /// Get the TLS record that has wrapped the pushed data. Returns an error or how many bytes of TLS record were written to `dst`.
    pub fn pull_write(&mut self, dst: &mut [u8]) -> Result<usize, Error> {
        let buf = unsafe {
            let mut l = MaybeUninit::<usize>::uninit();

            let b = br_ssl_engine_sendrec_buf(&mut self.context, l.as_mut_ptr());

            if b.is_null() {
                return Err(Error::Unknown);
            }

            slice::from_raw_parts(b, l.assume_init_read())
        };

        let len = match min(buf.len(), dst.len()) {
            0 => return Err(Error::Unknown),
            l => l,
        };

        dst[..len].copy_from_slice(&buf[..len]);

        unsafe {
            br_ssl_engine_sendrec_ack(&mut self.context, len);
        }

        Ok(len)
    }

    /// Push incoming TLS record to be decrypted.
    pub fn push_read(&mut self, src: &[u8]) -> Result<usize, Error> {
        let buf = unsafe {
            let mut l = MaybeUninit::<usize>::uninit();

            let b = br_ssl_engine_recvrec_buf(&mut self.context, l.as_mut_ptr());

            if b.is_null() {
                return Err(Error::Unknown);
            }

            slice::from_raw_parts_mut(b, l.assume_init_read())
        };

        let len = match min(buf.len(), src.len()) {
            0 => return Err(Error::Unknown),
            l => l,
        };

        buf[..len].copy_from_slice(&src[..len]);

        unsafe {
            br_ssl_engine_recvrec_ack(&mut self.context, len);
        }

        Ok(len)
    }

    /// Get the application data pushed before.
    pub fn pull_read(&mut self, dst: &mut [u8]) -> Result<usize, Error> {
        let buf = unsafe {
            let mut l = MaybeUninit::<usize>::uninit();

            let b = br_ssl_engine_recvapp_buf(&mut self.context, l.as_mut_ptr());

            if b.is_null() {
                return Err(Error::Unknown);
            }

            slice::from_raw_parts(b, l.assume_init_read())
        };

        let len = match min(buf.len(), dst.len()) {
            0 => return Err(Error::Unknown),
            l => l,
        };

        dst[..len].copy_from_slice(&buf[..len]);

        unsafe {
            br_ssl_engine_recvapp_ack(&mut self.context, len);
        }

        Ok(len)
    }

    pub fn shutdown_recieved(&self) -> bool {
        self.context.shutdown_recv != 0
    }
}
