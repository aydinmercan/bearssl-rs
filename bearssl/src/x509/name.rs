use core::marker::PhantomData;

use bearssl_sys::*;

#[repr(transparent)]
pub struct DistinguishedName<'a> {
    context: br_x500_name,
    _marker: PhantomData<&'a u8>,
}

impl<'a> DistinguishedName<'a> {
    pub fn from_bytes(slice: &'a mut [u8]) -> DistinguishedName {
        let context = br_x500_name {
            data: slice.as_mut_ptr(),
            len: slice.len(),
        };

        DistinguishedName {
            context,
            _marker: PhantomData,
        }
    }
}
