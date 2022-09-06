#![allow(non_camel_case_types)]

pub enum KeyType {
    Rsa,
    EllipticCurve,
}

/// The naming is based on [IANA cipher names](https://www.iana.org/assignments/tls-parameters/tls-parameters.xhtml#tls-parameters-4).
pub enum TlsProfile {
    TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
    TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
}
