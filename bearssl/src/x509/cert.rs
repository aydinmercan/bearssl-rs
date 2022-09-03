use core::marker::PhantomData;

use bearssl_sys::*;

#[repr(transparent)]
pub struct X509Certificate<'a> {
    pub(crate) inner: br_x509_certificate,
    _marker: PhantomData<&'a mut u8>,
}

impl<'a> X509Certificate<'a> {
    pub fn from_unchecked_der(raw: &'a mut [u8]) -> X509Certificate<'a> {
        let inner = br_x509_certificate {
            data: raw.as_mut_ptr(),
            data_len: raw.len(),
        };

        X509Certificate {
            inner,
            _marker: PhantomData,
        }
    }
}
