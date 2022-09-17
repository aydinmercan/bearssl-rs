use core::ops::Drop;
use core::slice;

use bearssl_sys::br_rsa_public_key;

#[repr(transparent)]
pub struct PublicKey(pub(crate) br_rsa_public_key);

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for PublicKey {
    fn zeroize(&mut self) {
        unsafe {
            let n = slice::from_raw_parts_mut(self.0.n, self.0.nlen);
            n.zeroize();

            let e = slice::from_raw_parts_mut(self.0.e, self.0.elen);
            e.zeroize();
        }
    }
}
