//! `nstd.core` is the central part of nstd. It makes no use of the Rust `std` crate, so it
//! supports a wide range of systems, including embedded.
pub mod def;
pub mod ptr;
pub mod slice;
use self::def::NSTDAny;

/// A null pointer value constant.
pub const NSTD_CORE_NULL: NSTDAny = core::ptr::null_mut();
