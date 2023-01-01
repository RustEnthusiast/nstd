//! A handle to the standard input stream.
use crate::{
    alloc::NSTDAllocError,
    core::{
        slice::{nstd_core_slice_new, NSTDSliceMut},
        str::nstd_core_str_from_bytes_unchecked,
    },
    io::NSTDIOError,
    string::{nstd_string_push_str, NSTDString},
    vec::NSTDVec,
    NSTDUInt,
};
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
/// - `NSTDUInt *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_read(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDSliceMut,
    read: &mut NSTDUInt,
) -> NSTDIOError {
    let (err, r) = crate::io::stdio::read(handle, buffer);
    *read = r;
    err
}

/// Continuously reads data from stdin into a buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean `read` will return as 0 in this case.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDVec *buffer` - The buffer to be extended with data from stdin.
///
/// - `NSTDUInt *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdin_read_all(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDVec,
    read: &mut NSTDUInt,
) -> NSTDIOError {
    let (err, r) = crate::io::stdio::read_all(handle, buffer);
    *read = r;
    err
}

/// Continuously reads UTF-8 data from stdin into a string buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean `read` will return as 0 in this case.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDString *buffer` - The buffer to be extended with data from stdin.
///
/// - `NSTDUInt *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This function will panic if `buffer`'s length in bytes exceeds `NSTDInt`'s max value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdin_read_to_string(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDString,
    read: &mut NSTDUInt,
) -> NSTDIOError {
    let (err, r) = crate::io::stdio::read_to_string(handle, buffer);
    *read = r;
    err
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdin_read_exact(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDSliceMut,
) -> NSTDIOError {
    crate::io::stdio::read_exact(handle, buffer)
}

/// Reads a line from stdin and appends it to `buffer`.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to stdin.
///
/// - `NSTDString *buffer` - The string buffer to extend with a line from stdin.
///
/// - `NSTDUInt *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This function will panic if `buffer`'s length in bytes exceeds `NSTDInt`'s max value.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_stdin_read_line(
    handle: &mut NSTDStdin,
    buffer: &mut NSTDString,
    read: &mut NSTDUInt,
) -> NSTDIOError {
    let mut buf = String::new();
    match handle.read_line(&mut buf) {
        Ok(r) => {
            *read = r;
            let bytes = nstd_core_slice_new(buf.as_ptr().cast(), 1, buf.len());
            // SAFETY: `bytes` refers to `buf`'s data, which is still valid UTF-8 here.
            unsafe {
                let str = nstd_core_str_from_bytes_unchecked(&bytes);
                match nstd_string_push_str(buffer, &str) {
                    NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDIOError::NSTD_IO_ERROR_NONE,
                    _ => NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY,
                }
            }
        }
        Err(err) => {
            *read = 0;
            NSTDIOError::from_err(err.kind())
        }
    }
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
