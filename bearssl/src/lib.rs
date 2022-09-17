#![doc = include_str!("../README.md")]
#![no_std]
#![deny(
    clippy::expect_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::zero_ptr,
    unused_lifetimes,
    unused_qualifications
)]

#[cfg(feature = "std")]
extern crate std;

pub mod ec;
pub mod engine;
pub mod profile;
pub mod rsa;
pub mod server;
pub mod x509;
