//! A handle to the standard output stream.
use crate::{core::slice::NSTDSlice, io::NSTDIOError, NSTDUInt};
use std::io::Stdout;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// A handle to the standard output stream.
pub type NSTDStdout = Box<Stdout>;

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdout handle` - A handle to the standard output stream.
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
/// - `const NSTDSlice *bytes` - The data to be written to stdout.
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
pub unsafe extern "C" fn nstd_io_stdout_write(
    handle: &mut NSTDStdout,
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
        let (err, w) = crate::os::unix::io::stdio::write(handle.as_raw_fd(), bytes);
        *written = w;
        err.into()
    }
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_stdout_write_all(
    handle: &mut NSTDStdout,
    bytes: &NSTDSlice,
) -> NSTDIOError {
    #[cfg(not(unix))]
    return crate::io::stdio::write_all(handle, bytes);
    #[cfg(unix)]
    return crate::os::unix::io::stdio::write_all(handle.as_raw_fd(), bytes).into();
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
    crate::io::stdio::flush(handle)
}

/// Frees an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout handle` - A handle to the standard output stream.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_io_stdout_free(handle: NSTDStdout) {}
