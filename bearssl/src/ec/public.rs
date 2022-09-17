use core::ops::Drop;
use core::slice;

use bearssl_sys::br_ec_public_key;

#[repr(transparent)]
pub struct PublicKey(pub(crate) br_ec_public_key);

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for PublicKey {
    fn zeroize(&mut self) {
        unsafe {
            let q = slice::from_raw_parts_mut(self.0.q, self.0.qlen);
            q.zeroize();
        }
    }
}
