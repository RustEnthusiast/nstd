//! Provides functionality for interacting with the standard I/O streams.
pub mod stderr;
pub mod stdin;
pub(crate) mod stdio;
pub mod stdout;
#[cfg(unix)]
use crate::os::unix::io::{
    NSTDUnixIOError::{
        self, NSTD_UNIX_IO_ERROR_BLOCKING, NSTD_UNIX_IO_ERROR_BROKEN_PIPE,
        NSTD_UNIX_IO_ERROR_CONNECTION_RESET, NSTD_UNIX_IO_ERROR_INTERRUPTED,
        NSTD_UNIX_IO_ERROR_INVALID_DATA, NSTD_UNIX_IO_ERROR_INVALID_INPUT, NSTD_UNIX_IO_ERROR_NONE,
        NSTD_UNIX_IO_ERROR_NOT_FOUND, NSTD_UNIX_IO_ERROR_NO_CONNECTION,
        NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY, NSTD_UNIX_IO_ERROR_PERMISSION_DENIED,
        NSTD_UNIX_IO_ERROR_TIMED_OUT, NSTD_UNIX_IO_ERROR_UNEXPECTED_EOF,
    },
    NSTDUnixIOResult,
};
use crate::{
    core::{result::NSTDResult, str::NSTDStr},
    string::{nstd_string_pop, NSTDString},
    vec::NSTDVec,
    NSTDUInt,
};
use nstdapi::nstdapi;
use std::io::{ErrorKind, Write};

/// An error type for I/O operations.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
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
    /// Creates a new instance of [`NSTDIOError`] from a Rust [`ErrorKind`].
    pub(crate) const fn from_err(err: ErrorKind) -> Self {
        match err {
            ErrorKind::NotFound => Self::NSTD_IO_ERROR_NOT_FOUND,
            ErrorKind::PermissionDenied => Self::NSTD_IO_ERROR_PERMISSION_DENIED,
            ErrorKind::ConnectionRefused => Self::NSTD_IO_ERROR_CONNECTION_REFUSED,
            ErrorKind::ConnectionReset => Self::NSTD_IO_ERROR_CONNECTION_RESET,
            ErrorKind::ConnectionAborted => Self::NSTD_IO_ERROR_CONNECTION_TERMINATED,
            ErrorKind::NotConnected => Self::NSTD_IO_ERROR_NO_CONNECTION,
            ErrorKind::AddrInUse => Self::NSTD_IO_ERROR_SOCKET_IN_USE,
            ErrorKind::AddrNotAvailable => Self::NSTD_IO_ERROR_ADDRESS_NOT_FOUND,
            ErrorKind::BrokenPipe => Self::NSTD_IO_ERROR_BROKEN_PIPE,
            ErrorKind::AlreadyExists => Self::NSTD_IO_ERROR_ALREADY_EXISTS,
            ErrorKind::WouldBlock => Self::NSTD_IO_ERROR_BLOCKING,
            ErrorKind::InvalidInput => Self::NSTD_IO_ERROR_INVALID_INPUT,
            ErrorKind::InvalidData => Self::NSTD_IO_ERROR_INVALID_DATA,
            ErrorKind::TimedOut => Self::NSTD_IO_ERROR_TIMED_OUT,
            ErrorKind::WriteZero => Self::NSTD_IO_ERROR_WRITE_ZERO,
            ErrorKind::Interrupted => Self::NSTD_IO_ERROR_INTERRUPTED,
            ErrorKind::Unsupported => Self::NSTD_IO_ERROR_UNSUPPORTED,
            ErrorKind::UnexpectedEof => Self::NSTD_IO_ERROR_UNEXPECTED_EOF,
            ErrorKind::OutOfMemory => Self::NSTD_IO_ERROR_OUT_OF_MEMORY,
            _ => Self::NSTD_IO_ERROR_UNKNOWN,
        }
    }
}
#[cfg(unix)]
impl From<NSTDUnixIOError> for NSTDIOError {
    /// Converts an [`NSTDUnixIOError`] into an [`NSTDIOError`].
    fn from(err: NSTDUnixIOError) -> Self {
        match err {
            NSTD_UNIX_IO_ERROR_NONE => Self::NSTD_IO_ERROR_NONE,
            NSTD_UNIX_IO_ERROR_NOT_FOUND => Self::NSTD_IO_ERROR_NOT_FOUND,
            NSTD_UNIX_IO_ERROR_PERMISSION_DENIED => Self::NSTD_IO_ERROR_PERMISSION_DENIED,
            NSTD_UNIX_IO_ERROR_CONNECTION_RESET => Self::NSTD_IO_ERROR_CONNECTION_RESET,
            NSTD_UNIX_IO_ERROR_NO_CONNECTION => Self::NSTD_IO_ERROR_NO_CONNECTION,
            NSTD_UNIX_IO_ERROR_BROKEN_PIPE => Self::NSTD_IO_ERROR_BROKEN_PIPE,
            NSTD_UNIX_IO_ERROR_BLOCKING => Self::NSTD_IO_ERROR_BLOCKING,
            NSTD_UNIX_IO_ERROR_INVALID_INPUT => Self::NSTD_IO_ERROR_INVALID_INPUT,
            NSTD_UNIX_IO_ERROR_INVALID_DATA => Self::NSTD_IO_ERROR_INVALID_DATA,
            NSTD_UNIX_IO_ERROR_TIMED_OUT => Self::NSTD_IO_ERROR_TIMED_OUT,
            NSTD_UNIX_IO_ERROR_INTERRUPTED => Self::NSTD_IO_ERROR_INTERRUPTED,
            NSTD_UNIX_IO_ERROR_UNEXPECTED_EOF => Self::NSTD_IO_ERROR_UNEXPECTED_EOF,
            NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY => Self::NSTD_IO_ERROR_OUT_OF_MEMORY,
            _ => Self::NSTD_IO_ERROR_UNKNOWN,
        }
    }
}

/// A result type that yields an [`NSTDUInt`] representing the number of bytes read or written by an
/// I/O operation on success and an I/O operation error code on failure.
pub type NSTDIOResult = NSTDResult<NSTDUInt, NSTDIOError>;
#[cfg(unix)]
impl From<NSTDUnixIOResult> for NSTDIOResult {
    /// Converts an [`NSTDUnixIOResult`] into an [`NSTDIOResult`].
    #[inline]
    fn from(value: NSTDUnixIOResult) -> Self {
        match value {
            NSTDResult::Ok(value) => Self::Ok(value),
            NSTDResult::Err(err) => Self::Err(err.into()),
        }
    }
}

/// A result type that yields an [`NSTDVec`] on success and an I/O operation error code on failure.
pub type NSTDIOBufferResult<'a> = NSTDResult<NSTDVec<'a>, NSTDIOError>;

/// A result type that yields a UTF-8 string on success and an I/O operation error code on failure.
pub type NSTDIOStringResult<'a> = NSTDResult<NSTDString<'a>, NSTDIOError>;

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
/// # Safety
///
/// The provided string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[nstdapi]
pub unsafe fn nstd_io_print(output: &NSTDStr) -> NSTDIOError {
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
/// # Safety
///
/// The provided string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
#[nstdapi]
pub unsafe fn nstd_io_print_line(output: &NSTDStr) -> NSTDIOError {
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

/// Reads a line of UTF-8 input from stdin, discarding the newline character.
///
/// # Returns
///
/// `NSTDIOStringResult input` - The UTF-8 input from stdin on success and the I/O operation error
/// code on failure.
#[nstdapi]
pub fn nstd_io_read() -> NSTDIOStringResult<'static> {
    let mut res = nstd_io_read_line();
    if let NSTDResult::Ok(input) = &mut res {
        nstd_string_pop(input);
    }
    res
}

/// Reads a line of UTF-8 input from stdin.
///
/// # Returns
///
/// `NSTDIOStringResult input` - The UTF-8 input from stdin on success and the I/O operation error
/// code on failure.
#[nstdapi]
pub fn nstd_io_read_line() -> NSTDIOStringResult<'static> {
    // Attempt to read a line from stdin.
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => NSTDResult::Ok(NSTDString::from_string(input)),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}
