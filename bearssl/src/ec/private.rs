use core::ops::Drop;
use core::slice;

use bearssl_sys::br_ec_private_key;

#[repr(transparent)]
pub struct PrivateKey(pub(crate) br_ec_private_key);

#[cfg(feature = "zeroize")]
impl Drop for PrivateKey {
    fn drop(&mut self) {
        use zeroize::Zeroize;

        self.zeroize();
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::ZeroizeOnDrop for PrivateKey {}

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for PrivateKey {
    fn zeroize(&mut self) {
        // Safety: Slice constructions should be safe as long as lengths have not been modified
        // outside what BearSSL set.
        unsafe {
            let x = slice::from_raw_parts_mut(self.0.x, self.0.xlen);
            x.zeroize();
        }
    }
}
