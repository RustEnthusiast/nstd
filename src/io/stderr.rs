//! A handle to the standard error stream.
use crate::{core::slice::NSTDSlice, io::NSTDIOError, NSTDUInt};
use std::io::Stderr;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

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
/// - `const NSTDSlice *bytes` - The data to be written to stderr.
///
/// - `NSTDUInt *written` - Returns as the number of bytes written.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr_write(
    handle: &mut NSTDStderr,
    bytes: &NSTDSlice,
    written: &mut NSTDUInt,
) -> NSTDIOError {
    #[cfg(not(unix))]
    {
        let (err, w) = crate::io::stdio::write(handle, bytes);
        *written = w;
        err
    }
    #[cfg(unix)]
    {
        let (err, w) = crate::os::unix::io::stdio::write(handle.lock().as_raw_fd(), bytes);
        *written = w;
        err.into()
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
/// - `const NSTDSlice *bytes` - The data to be written to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stderr_write_all(
    handle: &mut NSTDStderr,
    bytes: &NSTDSlice,
) -> NSTDIOError {
    #[cfg(not(unix))]
    return crate::io::stdio::write_all(handle, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write_all(handle.lock().as_raw_fd(), bytes).into();
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
    crate::io::stdio::flush(handle)
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
