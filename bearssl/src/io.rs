use core::ffi::c_void;

use bearssl_sys::*;
use libc::{c_int, c_uchar};

pub trait HazardReadCallback {
    /// Callback made by the SSL I/O context.
    extern "C" fn read(ctx: *mut c_void, data: *mut c_uchar, len: usize) -> c_int;

    /// Get the context to be used in the callback.
    unsafe fn get_context(&self) -> *mut c_void;
}

pub trait HazardWriteCallback {
    /// Callback made by the SSL I/O context.
    extern "C" fn write(ctx: *mut c_void, data: *const c_uchar, len: usize) -> c_int;

    /// Get the context to be used in the callback.
    unsafe fn get_context(&self) -> *mut c_void;
}

#[repr(transparent)]
pub struct TlsIo {
    context: br_sslio_context,
}
