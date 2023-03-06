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
pub mod unichar;
pub mod time;
use self::str::NSTDStr;
use nstdapi::nstdapi;

/// Invokes the runtime's panic handler.
///
/// This operation will never return.
///
/// # Panics
///
/// This function will always panic.
#[inline]
#[nstdapi]
pub const fn nstd_core_panic() -> ! {
    panic!();
}

/// Invokes the runtime's panic handler with a UTF-8 encoded payload.
///
/// This operation will never return.
///
/// # Parameters:
///
/// - `const NSTDStr *msg` - The message to panic with.
///
/// # Panics
///
/// This function will always panic.
///
/// # Safety
///
/// `msg`'s data must be valid for reads.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_panic_with_msg(msg: &NSTDStr) -> ! {
    panic!("{}", msg.as_str());
}
