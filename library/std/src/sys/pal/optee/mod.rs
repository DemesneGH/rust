//! System bindings for the Teeos platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for Teeos.
// #![deny(unsafe_op_in_unsafe_fn)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::os::raw::c_char;

pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unsupported/env.rs"]
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod net;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/stdio.rs"]
pub mod stdio;
#[path = "../unsupported/thread.rs"]
pub mod thread;
pub mod thread_local_dtor;
pub mod thread_local_key;
#[allow(non_upper_case_globals)]
pub mod time;

pub unsafe fn init(argc: isize, argv: *const *const u8, sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Other,
                          "operation not supported on optee yet")
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Other
}

#[inline]
pub(crate) fn is_interrupted(errno: i32) -> bool {
    errno == crate::io::ErrorKind::Interrupted as i32
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut _s: *const c_char) -> usize {
    0
}

pub fn abort_internal() -> ! {
    loop { }
}

// same as the one in unsupported/common.rs
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}