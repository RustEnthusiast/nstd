//! Provides core functionality for `nstd`.
//!
//! The entire `nstd.core` module is dependency free and makes no use of Rust's [std] library,
//! making it fit for resource constrained/embedded environments.
pub mod alloc;
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
pub mod time;
pub mod unichar;
use self::str::NSTDStr;
use nstdapi::nstdapi;

/// Takes advantage of Rust's unwinding behavior by panicking while a thread is already unwinding
/// from a panic, resulting in program abortion.
struct Abort;
impl Drop for Abort {
    /// Panics if Rust's panic strategy is set to unwind.
    #[inline]
    fn drop(&mut self) {
        #[cfg(panic = "unwind")]
        panic!();
    }
}

/// Terminates the program immediately in an abnormal fashion.
///
/// This operation will never return.
///
/// # Panics
///
/// This operation will always panic.
#[inline]
#[nstdapi]
pub const fn nstd_core_abort() -> ! {
    #[allow(unused_variables)]
    let abort = Abort;
    panic!();
}

/// Invokes the runtime's panic handler.
///
/// This operation will never return.
///
/// In contrast to `nstd_core_abort`, which will terminate the program immediately, this method of
/// abortion will begin unwinding the stack (when panic = "unwind"). This can be useful for Rust
/// programs that don't unwind through call frames from foreign languages.
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
/// In contrast to `nstd_core_abort`, which will terminate the program immediately, this method of
/// abortion will begin unwinding the stack (when panic = "unwind"). This can be useful for Rust
/// programs that don't unwind through call frames from foreign languages.
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
