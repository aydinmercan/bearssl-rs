#![doc = include_str!("../README.md")]
#![no_std]
#![allow(clippy::missing_safety_doc, non_camel_case_types, non_snake_case, non_upper_case_globals)]
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

#[cfg(feature = "dont-assume-size_t-equals-uintptr_t")]
use libc::size_t;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
