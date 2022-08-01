//! Provides functionality for interacting with the standard I/O streams.
pub mod stderr;
pub mod stdout;
use crate::{
    core::{cstr::NSTDCStrConst, def::NSTDErrorCode},
    string::{nstd_string_new, nstd_string_pop, NSTDString},
};
use std::io::{prelude::*, BufReader, ErrorKind};

/// An error type for I/O operations.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum NSTDIOError {
    /// No error ocurred.
    NSTD_IO_ERROR_NONE,
    /// An unknown/other error ocurred.
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
    #[allow(dead_code)]
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

/// Writes a C string slice to stdout.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *output` - The C string slice to write to stdout.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - Writing `output`'s bytes to stdout failed.
///
/// - `2` - Flushing stdout failed.
///
/// # Safety
///
/// The provided C string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print(output: &NSTDCStrConst) -> NSTDErrorCode {
    let mut stdout = std::io::stdout();
    if let Err(_) = stdout.write_all(output.as_bytes()) {
        return 1;
    } else if let Err(_) = stdout.flush() {
        return 2;
    }
    0
}

/// Writes a C string slice to stdout followed by a new line.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *output` - The C string slice to write to stdout.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - Writing bytes to stdout failed.
///
/// - `2` - Flushing stdout failed.
///
/// # Safety
///
/// The provided C string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_io_print_line(output: &NSTDCStrConst) -> NSTDErrorCode {
    let mut stdout = std::io::stdout();
    if let Err(_) = stdout.write_all(output.as_bytes()) {
        return 1;
    } else if let Err(_) = stdout.write_all(b"\n") {
        return 1;
    } else if let Err(_) = stdout.flush() {
        return 2;
    }
    0
}

/// Reads a line of UTF-8 input from stdin and returns it, discarding the newline.
///
/// # Returns
///
/// `NSTDString input` - The input from stdin, or an empty string on error.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_read() -> NSTDString {
    let mut input = nstd_io_read_line();
    nstd_string_pop(&mut input);
    input
}

/// Reads a line of UTF-8 input from stdin and returns it.
///
/// # Returns
///
/// `NSTDString input` - The input from stdin, or an empty string on error.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_io_read_line() -> NSTDString {
    let mut buffer = String::new();
    if let Err(_) = BufReader::new(std::io::stdin()).read_line(&mut buffer) {
        return nstd_string_new();
    }
    NSTDString::from(buffer.as_str())
}
