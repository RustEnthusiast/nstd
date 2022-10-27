//! Provides core functionality for `nstd`.
//!
//! The entire `nstd.core` module is dependency free and makes no use of Rust's [std] library,
//! making it fit for resource constrained/embedded environments.
pub mod cstr;
pub mod cty;
pub mod def;
pub mod fty;
pub mod ity;
pub mod math;
pub mod mem;
pub mod ops;
pub mod optional;
pub mod ptr;
pub mod range;
pub mod result;
pub mod slice;
pub mod str;

/// Invokes the runtime's panic handler.
///
/// This operation will never return.
///
/// # Panics
///
/// This function will always panic.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_panic() -> ! {
    panic!();
}
