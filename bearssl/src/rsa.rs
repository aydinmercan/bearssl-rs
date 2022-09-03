use bearssl_sys::*;

pub struct PublicKey(pub(crate) br_rsa_public_key);
pub struct PrivateKey(pub(crate) br_rsa_private_key);
