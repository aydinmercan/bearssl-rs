use core::marker::PhantomData;
use core::mem::MaybeUninit;

use bearssl_sys::*;

use crate::profile::{KeyType, TlsProfile};
use crate::server::conn::ServerConnection;
use crate::x509::cert::X509Certificate;

pub enum Error {
    PrivateKeyType { expected: KeyType, actual: KeyType },
    UnsupportedProfile,
}

pub struct ServerBuilder<'a> {
    chain: &'a [X509Certificate<'a>],
    rsa: Option<br_rsa_private_key>,
}

impl<'a> ServerBuilder<'a> {
    pub fn build(&self, profile: TlsProfile) -> Result<ServerConnection, Error> {
        match profile {
            TlsProfile::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 => {}
            _ => return Err(Error::UnsupportedProfile),
        }

        let context = unsafe {
            let mut ctx = MaybeUninit::<br_ssl_server_context>::uninit();

            br_ssl_server_init_mine2c(
                ctx.as_mut_ptr(),
                self.chain.as_ptr() as *const _,
                self.chain.len(),
                core::ptr::null(),
            );

            ctx.assume_init_read()
        };

        Ok(ServerConnection {
            context,
            chain: PhantomData,
        })
    }
}
