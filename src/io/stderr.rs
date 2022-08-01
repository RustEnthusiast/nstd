//! A handle to the standard error stream.
use crate::{core::slice::NSTDSliceConst, io::NSTDIOError, NSTDUSize};
use std::io::{Stderr, Write};

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

/// Writes some data to the standard error stream, setting `written` to the number of bytes
/// written.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStderr *handle` - A handle to stderr.
///
/// - `const NSTDSliceConst *bytes` - The data to be written to stderr.
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
pub unsafe extern "C" fn nstd_io_stderr_write(
    handle: &mut NSTDStderr,
    bytes: &NSTDSliceConst,
    written: &mut NSTDUSize,
) -> NSTDIOError {
    // Make sure the slice's element size is 1.
    if bytes.ptr.size != 1 {
        return NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT;
    }
    // Attempt to write the bytes to stderr.
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

/// Writes an entire buffer to the standard error stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStderr *handle` - A handle to stderr.
///
/// - `const NSTDSliceConst *bytes` - The data to be written to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr_write_all(
    handle: &mut NSTDStderr,
    bytes: &NSTDSliceConst,
) -> NSTDIOError {
    // Make sure the slice's element size is 1.
    if bytes.ptr.size != 1 {
        return NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT;
    }
    // Attempt to write the bytes to stderr.
    if let Err(err) = handle.write_all(bytes.as_slice()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Flushes the standard error stream.
///
/// # Parameters:
///
/// - `NSTDStderr *handle` - A handle to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stderr_flush(handle: &mut NSTDStderr) -> NSTDIOError {
    if let Err(err) = handle.flush() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
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
