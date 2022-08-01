//! A handle to the standard error stream.
use std::io::Stderr;

/// A handle to the standard error stream.
pub type NSTDStderr = Box<Stderr>;

/// Constructs a new handle to the standard error stream.
///
/// # Returns
///
/// `NSTDStderr handle` - A handle to the standard error stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stderr() -> NSTDStderr {
    NSTDStderr::new(std::io::stderr())
}

/// Frees an instance of `NSTDStderr`.
///
/// # Parameters:
///
/// - `NSTDStderr handle` - A handle to the standard error stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_io_stderr_free(handle: NSTDStderr) {}
