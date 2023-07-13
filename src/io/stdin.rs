//! A handle to the standard input stream.
use crate::{
    alloc::{CBox, NSTDAllocError},
    core::{
        optional::{gen_optional, NSTDOptional},
        result::NSTDResult,
        slice::{NSTDSlice, NSTDSliceMut},
        str::nstd_core_str_from_bytes_unchecked,
    },
    io::{NSTDIOError, NSTDIOResult},
    string::{nstd_string_push_str, NSTDString},
    vec::NSTDVec,
};
use nstdapi::nstdapi;
use std::io::{Stdin, StdinLock};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// A handle to the standard input stream.
#[nstdapi]
pub struct NSTDStdin {
    /// Rust's [Stdin].
    r#in: CBox<Stdin>,
}
gen_optional!(NSTDOptionalStdin, NSTDStdin);

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDOptionalStdin handle` - A handle to the standard input stream on success, or an
/// uninitialized "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_io_stdin() -> NSTDOptionalStdin {
    match CBox::new(std::io::stdin()) {
        Some(r#in) => NSTDOptional::Some(NSTDStdin { r#in }),
        _ => NSTDOptional::None,
    }
}

/// Reads some data from stdin into a byte slice buffer.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
#[inline]
#[nstdapi]
pub unsafe fn nstd_io_stdin_read(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDSliceMut,
) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::read(&mut handle.r#in, buffer);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::read(handle.r#in.lock().as_raw_fd(), buffer).into();
}

/// Continuously reads data from stdin into a buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `handle` in this case.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDVec *buffer` - The buffer to be extended with data from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
#[inline]
#[nstdapi]
pub fn nstd_io_stdin_read_all(handle: &mut NSTDStdin, buffer: &mut NSTDVec) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::read_all(&mut handle.r#in, buffer);
    #[cfg(unix)]
    // SAFETY: `handle` owns the file descriptor.
    unsafe {
        crate::os::unix::io::stdio::read_all(handle.r#in.lock().as_raw_fd(), buffer).into()
    }
}

/// Continuously reads UTF-8 data from stdin into a string buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `handle` in this case.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDString *buffer` - The buffer to be extended with data from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
#[inline]
#[nstdapi]
pub fn nstd_io_stdin_read_to_string(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDString,
) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::read_to_string(&mut handle.r#in, buffer);
    #[cfg(unix)]
    // SAFETY: `handle` owns the file descriptor.
    unsafe {
        crate::os::unix::io::stdio::read_to_string(handle.r#in.lock().as_raw_fd(), buffer).into()
    }
}

/// Reads enough data from stdin to fill the entirety of `buffer`.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer` must be valid for writes.
#[inline]
#[nstdapi]
pub unsafe fn nstd_io_stdin_read_exact(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDSliceMut,
) -> NSTDIOError {
    #[cfg(not(unix))]
    return crate::io::stdio::read_exact(&mut handle.r#in, buffer);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::read_exact(handle.r#in.lock().as_raw_fd(), buffer).into();
}

/// Reads a line from stdin and appends it to `buffer`.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to stdin.
///
/// - `NSTDString *buffer` - The string buffer to extend with a line from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
#[nstdapi]
pub fn nstd_io_stdin_read_line(handle: &mut NSTDStdin, buffer: &mut NSTDString) -> NSTDIOResult {
    let mut buf = String::new();
    match handle.r#in.read_line(&mut buf) {
        Ok(r) => {
            let bytes = NSTDSlice::from_slice(buf.as_bytes());
            // SAFETY: `bytes` refers to `buf`'s data, which is still valid UTF-8 here.
            unsafe {
                let str = nstd_core_str_from_bytes_unchecked(&bytes);
                match nstd_string_push_str(buffer, &str) {
                    NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDResult::Ok(r),
                    _ => NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY),
                }
            }
        }
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Frees an instance of `NSTDStdin`.
///
/// # Parameters:
///
/// - `NSTDStdin handle` - A handle to the standard input stream.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_io_stdin_free(handle: NSTDStdin) {}

/// A locked handle to the standard input stream.
#[nstdapi]
pub struct NSTDStdinLock {
    /// Rust's [StdinLock].
    r#in: CBox<StdinLock<'static>>,
}
gen_optional!(NSTDOptionalStdinLock, NSTDStdinLock);

/// Constructs a new locked handle to the standard input stream.
///
/// # Returns
///
/// `NSTDOptionalStdinLock handle` - A locked handle to the standard input stream on success, or an
/// uninitialized "none" variant on error.
#[inline]
#[nstdapi]
pub fn nstd_io_stdin_lock() -> NSTDOptionalStdinLock {
    match CBox::new(std::io::stdin().lock()) {
        Some(r#in) => NSTDOptional::Some(NSTDStdinLock { r#in }),
        _ => NSTDOptional::None,
    }
}

/// Reads some data from stdin into a byte slice buffer.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdinLock *handle` - A locked handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
#[inline]
#[nstdapi]
pub unsafe fn nstd_io_stdin_lock_read(
    handle: &mut NSTDStdinLock,
    buffer: &mut NSTDSliceMut,
) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::read(&mut handle.r#in, buffer);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::read(handle.r#in.as_raw_fd(), buffer).into();
}

/// Continuously reads data from stdin into a buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `handle` in this case.
///
/// # Parameters:
///
/// - `NSTDStdinLock *handle` - A locked handle to the standard input stream.
///
/// - `NSTDVec *buffer` - The buffer to be extended with data from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
#[inline]
#[nstdapi]
pub fn nstd_io_stdin_lock_read_all(
    handle: &mut NSTDStdinLock,
    buffer: &mut NSTDVec,
) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::read_all(&mut handle.r#in, buffer);
    #[cfg(unix)]
    // SAFETY: `handle` owns the file descriptor.
    unsafe {
        crate::os::unix::io::stdio::read_all(handle.r#in.as_raw_fd(), buffer).into()
    }
}

/// Continuously reads UTF-8 data from stdin into a string buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `handle` in this case.
///
/// # Parameters:
///
/// - `NSTDStdinLock *handle` - A locked handle to the standard input stream.
///
/// - `NSTDString *buffer` - The buffer to be extended with data from stdin.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
#[inline]
#[nstdapi]
pub fn nstd_io_stdin_lock_read_to_string(
    handle: &mut NSTDStdinLock,
    buffer: &mut NSTDString,
) -> NSTDIOResult {
    #[cfg(not(unix))]
    return crate::io::stdio::read_to_string(&mut handle.r#in, buffer);
    #[cfg(unix)]
    // SAFETY: `handle` owns the file descriptor.
    unsafe {
        crate::os::unix::io::stdio::read_to_string(handle.r#in.as_raw_fd(), buffer).into()
    }
}

/// Reads enough data from stdin to fill the entirety of `buffer`.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdinLock *handle` - A locked handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer` must be valid for writes.
#[inline]
#[nstdapi]
pub unsafe fn nstd_io_stdin_lock_read_exact(
    handle: &mut NSTDStdinLock,
    buffer: &mut NSTDSliceMut,
) -> NSTDIOError {
    #[cfg(not(unix))]
    return crate::io::stdio::read_exact(&mut handle.r#in, buffer);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::read_exact(handle.r#in.as_raw_fd(), buffer).into();
}

/// Frees and unlocks an instance of `NSTDStdinLock`.
///
/// # Parameters:
///
/// - `NSTDStdinLock handle` - A locked handle to the standard input stream.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_io_stdin_unlock(handle: NSTDStdinLock) {}
