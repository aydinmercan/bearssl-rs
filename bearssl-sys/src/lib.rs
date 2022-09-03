#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "dont-assume-size_t-equals-uintptr_t")]
use libc::size_t;

#[cfg(not(feature = "dont-assume-size_t-equals-uintptr_t"))]
type size_t = usize;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
