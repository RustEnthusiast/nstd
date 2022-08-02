//! A handle to the standard input stream.
use std::io::{BufReader, Stdin};

/// A handle to the standard input stream.
///
/// This stream is buffered by default.
pub type NSTDStdin = Box<BufReader<Stdin>>;

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdin handle` - A handle to the standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdin() -> NSTDStdin {
    NSTDStdin::new(BufReader::new(std::io::stdin()))
}

/// Frees an instance of `NSTDStdin`.
///
/// # Parameters:
///
/// - `NSTDStdin handle` - A handle to the standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_io_stdin_free(handle: NSTDStdin) {}
