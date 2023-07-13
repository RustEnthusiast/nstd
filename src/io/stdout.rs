//! A handle to the standard output stream.
use crate::{
    alloc::CBox,
    core::{
        optional::{gen_optional, NSTDOptional},
        slice::NSTDSlice,
    },
    io::{NSTDIOError, NSTDIOResult},
};
use nstdapi::nstdapi;
use std::io::{Stdout, StdoutLock};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// A handle to the standard output stream.
#[nstdapi]
pub struct NSTDStdout {
    /// Rust's [Stdout].
    out: CBox<Stdout>,
}
gen_optional!(NSTDOptionalStdout, NSTDStdout);

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDOptionalStdout handle` - A handle to the standard output stream on success, or an
/// uninitialized "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_io_stdout() -> NSTDOptionalStdout {
    match CBox::new(std::io::stdout()) {
        Some(out) => NSTDOptional::Some(NSTDStdout { out }),
        _ => NSTDOptional::None,
    }
}

/// Writes some data to the standard output stream, returning how many bytes were written.
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
/// - `const NSTDSlice *bytes` - The data to be written to stdout.
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
pub unsafe fn nstd_io_stdout_write(handle: &mut NSTDStdout, bytes: &NSTDSlice) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::write(&mut *handle.out, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write(handle.out.lock().as_raw_fd(), bytes).into();
}

/// Writes an entire buffer to the standard output stream.
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
/// - `const NSTDSlice *bytes` - The data to be written to stdout.
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
pub unsafe fn nstd_io_stdout_write_all(handle: &mut NSTDStdout, bytes: &NSTDSlice) -> NSTDIOError {
    #[cfg(not(unix))]
    return crate::io::stdio::write_all(&mut *handle.out, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write_all(handle.out.lock().as_raw_fd(), bytes).into();
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
#[nstdapi]
pub fn nstd_io_stdout_flush(handle: &mut NSTDStdout) -> NSTDIOError {
    crate::io::stdio::flush(&mut *handle.out)
}

/// Frees an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout handle` - A handle to the standard output stream.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_io_stdout_free(handle: NSTDStdout) {}

/// A locked handle to the standard output stream.
#[nstdapi]
pub struct NSTDStdoutLock {
    /// Rust's [StdoutLock].
    out: CBox<StdoutLock<'static>>,
}
gen_optional!(NSTDOptionalStdoutLock, NSTDStdoutLock);

/// Constructs a new locked handle to the standard output stream.
///
/// # Returns
///
/// `NSTDOptionalStdoutLock handle` - A locked handle to the standard output stream on success, or
/// an uninitialized "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_io_stdout_lock() -> NSTDOptionalStdoutLock {
    match CBox::new(std::io::stdout().lock()) {
        Some(out) => NSTDOptional::Some(NSTDStdoutLock { out }),
        _ => NSTDOptional::None,
    }
}

/// Writes some data to the standard output stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdoutLock *handle` - A locked handle to stdout.
///
/// - `const NSTDSlice *bytes` - The data to be written to stdout.
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
pub unsafe fn nstd_io_stdout_lock_write(
    handle: &mut NSTDStdoutLock,
    bytes: &NSTDSlice,
) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::write(&mut *handle.out, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write(handle.out.as_raw_fd(), bytes).into();
}

/// Writes an entire buffer to the standard output stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdoutLock *handle` - A locked handle to stdout.
///
/// - `const NSTDSlice *bytes` - The data to be written to stdout.
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
pub unsafe fn nstd_io_stdout_lock_write_all(
    handle: &mut NSTDStdoutLock,
    bytes: &NSTDSlice,
) -> NSTDIOError {
    #[cfg(not(unix))]
    return crate::io::stdio::write_all(&mut *handle.out, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write_all(handle.out.as_raw_fd(), bytes).into();
}

/// Flushes the standard output stream.
///
/// # Parameters:
///
/// - `NSTDStdoutLock *handle` - A locked handle to stdout.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
#[inline]
#[nstdapi]
pub fn nstd_io_stdout_lock_flush(handle: &mut NSTDStdoutLock) -> NSTDIOError {
    crate::io::stdio::flush(&mut *handle.out)
}

/// Frees and unlocks an instance of `NSTDStdoutLock`.
///
/// # Parameters:
///
/// - `NSTDStdoutLock handle` - A locked handle to the standard output stream.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_io_stdout_unlock(handle: NSTDStdoutLock) {}
