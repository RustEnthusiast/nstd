//! A handle to the standard input stream.
use crate::{core::slice::NSTDSliceMut, io::NSTDIOError, NSTDUSize};
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
    let rd = crate::io::stdio::read(handle, buffer);
    *read = rd.0;
    rd.1
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
