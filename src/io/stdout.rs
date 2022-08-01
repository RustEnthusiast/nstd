//! A handle to the standard output stream.
use crate::{core::slice::NSTDSliceConst, io::NSTDIOError, NSTDUSize};
use std::io::{Stdout, Write};

/// A handle to the standard output stream.
pub type NSTDStdout = Box<Stdout>;

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdout handle` - A locked handle to the standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdout() -> NSTDStdout {
    NSTDStdout::new(std::io::stdout())
}

/// Writes some data to the standard output stream, setting `written` to the number of bytes
/// written.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdout *handle` - A handle to stdout.
///
/// - `const NSTDSliceConst *bytes` - The data to be written to stdout.
///
/// - `NSTDUSize *written` - Returns as the number of bytes written.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout_write(
    handle: &mut NSTDStdout,
    bytes: &NSTDSliceConst,
    written: &mut NSTDUSize,
) -> NSTDIOError {
    // Make sure the slice's element size is 1.
    if bytes.ptr.size != 1 {
        return NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT;
    }
    // Attempt to write the bytes to stdout.
    match handle.write(bytes.as_slice()) {
        Ok(w) => {
            *written = w;
            NSTDIOError::NSTD_IO_ERROR_NONE
        }
        Err(err) => {
            *written = 0;
            NSTDIOError::from_err(err.kind())
        }
    }
}

/// Flushes the standard output stream.
///
/// # Parameters:
///
/// - `NSTDStdout *handle` - A handle to stdout.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdout_flush(handle: &mut NSTDStdout) -> NSTDIOError {
    if let Err(err) = handle.flush() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Frees and unlocks an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout handle` - A handle to the standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_io_stdout_free(handle: NSTDStdout) {}
