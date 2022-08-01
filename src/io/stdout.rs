//! A handle to the standard output stream.
use std::io::Stdout;

/// A handle to the standard output stream.
pub type NSTDStdout = Box<Stdout>;

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdout stdout` - A locked handle to the standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdout() -> NSTDStdout {
    NSTDStdout::new(std::io::stdout())
}

/// Frees and unlocks an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout stdout` - A handle to the standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_io_stdout_free(stdout: NSTDStdout) {}
