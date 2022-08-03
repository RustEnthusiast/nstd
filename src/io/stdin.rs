//! A handle to the standard input stream.
use crate::{core::slice::NSTDSliceMut, io::NSTDIOError, NSTDUSize};
use std::io::Stdin;

/// A handle to the standard input stream.
pub type NSTDStdin = Box<Stdin>;

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdin handle` - A handle to the standard input stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdin() -> NSTDStdin {
    NSTDStdin::new(std::io::stdin())
}

/// Reads some data from stdin into a byte slice buffer.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// - `NSTDUSize *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOErrorCode errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_read(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDSliceMut,
    read: &mut NSTDUSize,
) -> NSTDIOError {
    crate::io::stdio::read(handle, buffer, read)
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
