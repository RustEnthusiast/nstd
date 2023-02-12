//! A handle to the standard error stream.
use crate::{
    core::slice::NSTDSlice,
    io::{NSTDIOError, NSTDIOResult},
};
use nstdapi::nstdapi;
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
#[nstdapi]
pub fn nstd_io_stderr() -> NSTDStderr {
    NSTDStderr::new(std::io::stderr())
}

/// Writes some data to the standard error stream, returning how many bytes were written.
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
/// `NSTDIOResult written` - The number of bytes written to `handle` on success, or the I/O
/// operation error code on failure.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
#[inline]
#[nstdapi]
pub unsafe fn nstd_io_stderr_write(handle: &mut NSTDStderr, bytes: &NSTDSlice) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::write(handle, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write(handle.lock().as_raw_fd(), bytes).into();
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
#[nstdapi]
pub unsafe fn nstd_io_stderr_write_all(handle: &mut NSTDStderr, bytes: &NSTDSlice) -> NSTDIOError {
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
#[nstdapi]
pub fn nstd_io_stderr_flush(handle: &mut NSTDStderr) -> NSTDIOError {
    crate::io::stdio::flush(handle)
}

/// Frees an instance of `NSTDStderr`.
///
/// # Parameters:
///
/// - `NSTDStderr handle` - A handle to the standard error stream.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_io_stderr_free(handle: NSTDStderr) {}
