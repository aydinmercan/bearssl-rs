use core::ops::Drop;
use core::slice;

use bearssl_sys::br_rsa_private_key;

#[repr(transparent)]
pub struct PrivateKey(pub(crate) br_rsa_private_key);

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
            let p = slice::from_raw_parts_mut(self.0.p, self.0.plen);
            p.zeroize();

            let q = slice::from_raw_parts_mut(self.0.q, self.0.qlen);
            q.zeroize();

            let dp = slice::from_raw_parts_mut(self.0.dp, self.0.dplen);
            dp.zeroize();

            let dq = slice::from_raw_parts_mut(self.0.dq, self.0.dqlen);
            dq.zeroize();

            let iq = slice::from_raw_parts_mut(self.0.iq, self.0.iqlen);
            iq.zeroize();
        }
    }
}
