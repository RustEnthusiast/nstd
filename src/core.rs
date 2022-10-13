//! Provides core functionality for `nstd`.
//!
//! This library makes no use of Rust's [std] module, and is looking to be dependency free in the
//! near future, meaning that this will only make use of the [core] standard library module.
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
