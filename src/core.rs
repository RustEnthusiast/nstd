//! `nstd.core` is the central part of nstd. It makes no use of the Rust `std` crate, so it
//! supports a wide range of systems, including embedded.
pub mod cstr;
pub mod cty;
pub mod def;
pub mod fty;
pub mod ity;
pub mod math;
pub mod mem;
pub mod ptr;
pub mod range;
pub mod slice;
pub mod str;
use self::def::NSTDAny;

/// A null pointer value constant.
pub const NSTD_CORE_NULL: NSTDAny = core::ptr::null_mut();
