use core::marker::PhantomData;

use bearssl_sys::*;

#[repr(transparent)]
pub struct Certificate<'a> {
    context: br_x509_certificate,
    _marker: PhantomData<&'a u8>,
}

impl<'a> Certificate<'a> {
    /// Validity of the certificate is not checked.
    pub fn from_unchecked_der(slice: &'a mut [u8]) -> Certificate {
        let context = br_x509_certificate {
            data: slice.as_mut_ptr(),
            data_len: slice.len(),
        };

        Certificate {
            context,
            _marker: PhantomData,
        }
    }
}
