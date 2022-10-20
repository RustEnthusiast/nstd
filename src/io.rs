//! Provides functionality for interacting with the standard I/O streams.
pub mod stderr;
pub mod stdin;
pub(crate) mod stdio;
pub mod stdout;
use crate::{
    alloc::NSTDAllocError,
    core::{
        slice::nstd_core_slice_new,
        str::{nstd_core_str_from_bytes_unchecked, NSTDStr},
    },
    string::{nstd_string_pop, nstd_string_push_str, NSTDString},
};
use std::io::{ErrorKind, Write};

/// An error type for I/O operations.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDIOError {
    /// No error occurred.
    NSTD_IO_ERROR_NONE,
    /// An unknown/other error occurred.
    NSTD_IO_ERROR_UNKNOWN,
    /// An entity, such as a file, was not found.
    NSTD_IO_ERROR_NOT_FOUND,
    /// Permission was denied.
    NSTD_IO_ERROR_PERMISSION_DENIED,
    /// The connection was refused by a remote server.
    NSTD_IO_ERROR_CONNECTION_REFUSED,
    /// The connection was reset by a remote server.
    NSTD_IO_ERROR_CONNECTION_RESET,
    /// The connection was terminated by a remote server.
    NSTD_IO_ERROR_CONNECTION_TERMINATED,
    /// There is no connection.
    NSTD_IO_ERROR_NO_CONNECTION,
    /// A socket address could not be used.
    NSTD_IO_ERROR_SOCKET_IN_USE,
    /// An address could not be found.
    NSTD_IO_ERROR_ADDRESS_NOT_FOUND,
    /// The operation failed because a pipe was closed.
    NSTD_IO_ERROR_BROKEN_PIPE,
    /// An entity, such as a file, already exists.
    NSTD_IO_ERROR_ALREADY_EXISTS,
    /// The operation needs to block to complete.
    NSTD_IO_ERROR_BLOCKING,
    /// Some input parameter was incorrect.
    NSTD_IO_ERROR_INVALID_INPUT,
    /// Some input data was incorrect.
    NSTD_IO_ERROR_INVALID_DATA,
    /// The I/O operation's timeout expired, causing it to be canceled.
    NSTD_IO_ERROR_TIMED_OUT,
    /// Zero bytes were written to an output stream.
    NSTD_IO_ERROR_WRITE_ZERO,
    /// The operation was interrupted.
    NSTD_IO_ERROR_INTERRUPTED,
    /// The operation is unsupported on the current platform.
    NSTD_IO_ERROR_UNSUPPORTED,
    /// A reader unexpectedly reached the end of a file.
    NSTD_IO_ERROR_UNEXPECTED_EOF,
    /// An operation could not be completed, because it failed to allocate enough memory.
    NSTD_IO_ERROR_OUT_OF_MEMORY,
}
impl NSTDIOError {
    /// Creates a new instance of [NSTDIOError] from a Rust [ErrorKind].
    pub(crate) fn from_err(err: ErrorKind) -> Self {
        match err {
            ErrorKind::NotFound => NSTDIOError::NSTD_IO_ERROR_NOT_FOUND,
            ErrorKind::PermissionDenied => NSTDIOError::NSTD_IO_ERROR_PERMISSION_DENIED,
            ErrorKind::ConnectionRefused => NSTDIOError::NSTD_IO_ERROR_CONNECTION_REFUSED,
            ErrorKind::ConnectionReset => NSTDIOError::NSTD_IO_ERROR_CONNECTION_RESET,
            ErrorKind::ConnectionAborted => NSTDIOError::NSTD_IO_ERROR_CONNECTION_TERMINATED,
            ErrorKind::NotConnected => NSTDIOError::NSTD_IO_ERROR_NO_CONNECTION,
            ErrorKind::AddrInUse => NSTDIOError::NSTD_IO_ERROR_SOCKET_IN_USE,
            ErrorKind::AddrNotAvailable => NSTDIOError::NSTD_IO_ERROR_ADDRESS_NOT_FOUND,
            ErrorKind::BrokenPipe => NSTDIOError::NSTD_IO_ERROR_BROKEN_PIPE,
            ErrorKind::AlreadyExists => NSTDIOError::NSTD_IO_ERROR_ALREADY_EXISTS,
            ErrorKind::WouldBlock => NSTDIOError::NSTD_IO_ERROR_BLOCKING,
            ErrorKind::InvalidInput => NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT,
            ErrorKind::InvalidData => NSTDIOError::NSTD_IO_ERROR_INVALID_DATA,
            ErrorKind::TimedOut => NSTDIOError::NSTD_IO_ERROR_TIMED_OUT,
            ErrorKind::WriteZero => NSTDIOError::NSTD_IO_ERROR_WRITE_ZERO,
            ErrorKind::Interrupted => NSTDIOError::NSTD_IO_ERROR_INTERRUPTED,
            ErrorKind::Unsupported => NSTDIOError::NSTD_IO_ERROR_UNSUPPORTED,
            ErrorKind::UnexpectedEof => NSTDIOError::NSTD_IO_ERROR_UNEXPECTED_EOF,
            ErrorKind::OutOfMemory => NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY,
            _ => NSTDIOError::NSTD_IO_ERROR_UNKNOWN,
        }
    }
}

/// Writes a string slice to stdout.
///
/// # Parameters:
///
/// - `const NSTDStr *output` - The string slice to write to stdout.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// Panics if `output`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// The provided string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print(output: &NSTDStr) -> NSTDIOError {
    let mut stdout = std::io::stdout();
    if let Err(err) = stdout.write_all(output.as_str().as_bytes()) {
        return NSTDIOError::from_err(err.kind());
    } else if let Err(err) = stdout.flush() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Writes a string slice to stdout followed by a new line.
///
/// # Parameters:
///
/// - `const NSTDStr *output` - The string slice to write to stdout.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// Panics if `output`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// The provided string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print_line(output: &NSTDStr) -> NSTDIOError {
    let mut stdout = std::io::stdout();
    if let Err(err) = stdout.write_all(output.as_str().as_bytes()) {
        return NSTDIOError::from_err(err.kind());
    } else if let Err(err) = stdout.write_all(b"\n") {
        return NSTDIOError::from_err(err.kind());
    } else if let Err(err) = stdout.flush() {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Reads a line of UTF-8 input from stdin and pushes it onto `buffer` without the newline.
///
/// # Parameters:
///
/// - `NSTDString *buffer` - The string buffer to be extended with input from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// Panics if `buffer`'s length in bytes exceeds `NSTDInt`'s max value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_read(buffer: &mut NSTDString) -> NSTDIOError {
    let errc = nstd_io_read_line(buffer);
    nstd_string_pop(buffer);
    errc
}

/// Reads a line of UTF-8 input from stdin and pushes it onto `buffer`.
///
/// # Parameters:
///
/// - `NSTDString *buffer` - The string buffer to be extended with input from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// Panics if `buffer`'s length in bytes exceeds `NSTDInt`'s max value.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_read_line(buffer: &mut NSTDString) -> NSTDIOError {
    // Attempt to read a line from stdin.
    let mut input = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut input) {
        return NSTDIOError::from_err(err.kind());
    }
    // SAFETY: Rust strings are UTF-8 encoded.
    unsafe {
        // Extend the string buffer with the input from stdin.
        let bytes = nstd_core_slice_new(input.as_ptr().cast(), 1, input.len());
        let str = nstd_core_str_from_bytes_unchecked(&bytes);
        match nstd_string_push_str(buffer, &str) {
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDIOError::NSTD_IO_ERROR_NONE,
            _ => NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY,
        }
    }
}
