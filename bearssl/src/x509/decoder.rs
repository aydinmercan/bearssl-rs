use core::default::Default;
use core::mem::MaybeUninit;
use core::ptr;

use bearssl_sys::*;

#[non_exhaustive]
pub enum Error {
    Unknown,
    DecodingNotFinished,

    EmptyCertificateChain,
    Overflow,
}

// TODO: Combine similar error types and add meaningful information (expected/actual etc.)
#[repr(i32)]
pub enum RawError {
    InvalidValue = BR_ERR_X509_INVALID_VALUE,
    TruncatedCertificate = BR_ERR_X509_TRUNCATED,
    InnerElementExtendsOuterSize = BR_ERR_X509_INNER_TRUNC,
    BadTagClass = BR_ERR_X509_BAD_TAG_CLASS,
    BadTagValue = BR_ERR_X509_BAD_TAG_VALUE,
    IndefiniteLength = BR_ERR_X509_INDEFINITE_LENGTH,
    ExtraneousElement = BR_ERR_X509_EXTRA_ELEMENT,
    UnexpectedElement = BR_ERR_X509_UNEXPECTED,
    ElementNotConstructed = BR_ERR_X509_NOT_CONSTRUCTED,
    ElementNotPrimitive = BR_ERR_X509_NOT_PRIMITIVE,
    PartialByte = BR_ERR_X509_PARTIAL_BYTE,
    BadBoolean = BR_ERR_X509_BAD_BOOLEAN,
    BadDN = BR_ERR_X509_BAD_DN,
    BadTime = BR_ERR_X509_BAD_TIME,
    UnsupportedUnignorableExtension = BR_ERR_X509_UNSUPPORTED,
    LimitExceeded = BR_ERR_X509_LIMIT_EXCEEDED,
    WrongKeyType = BR_ERR_X509_WRONG_KEY_TYPE,
    BadSignature = BR_ERR_X509_BAD_SIGNATURE,
    TimeUnknown = BR_ERR_X509_TIME_UNKNOWN,
    CertificateExpired = BR_ERR_X509_EXPIRED,
    DNMismatch = BR_ERR_X509_DN_MISMATCH,
    BadServerName = BR_ERR_X509_BAD_SERVER_NAME,
    UnknownCriticalExtension = BR_ERR_X509_CRITICAL_EXTENSION,
    IsNotCA = BR_ERR_X509_NOT_CA,
    ForbiddenKeyUsage = BR_ERR_X509_FORBIDDEN_KEY_USAGE,
    WeakPublicKey = BR_ERR_X509_WEAK_PUBLIC_KEY,
    NotTrusted = BR_ERR_X509_NOT_TRUSTED,
}

pub struct X509Decoder(br_x509_decoder_context);

impl X509Decoder {
    pub fn new() -> X509Decoder {
        let context = unsafe {
            let mut ctx = MaybeUninit::<br_x509_decoder_context>::uninit();

            br_x509_decoder_init(ctx.as_mut_ptr(), None, ptr::null_mut());

            ctx.assume_init_read()
        };

        X509Decoder(context)
    }

    pub fn is_ca(&self) -> bool {
        self.0.isCA != 0
    }

    pub fn raw_publickey(&'_ self) -> Result<&'_ br_x509_pkey, Error> {
        if self.0.decoded != 0 {
            return Err(Error::DecodingNotFinished);
        }

        if self.0.err != 0 {
            return Err(self.context_error_code());
        }

        Ok(&self.0.pkey)
    }

    #[inline]
    fn context_error_code(&self) -> Error {
        match self.0.err {
            BR_ERR_X509_OVERFLOW => Error::Overflow,
            BR_ERR_X509_EMPTY_CHAIN => Error::EmptyCertificateChain,
            0 => unreachable!("0 should have been checked beforehand"),
            _ => Error::Unknown,
        }
    }
}

impl Default for X509Decoder {
    fn default() -> Self {
        X509Decoder::new()
    }
}
