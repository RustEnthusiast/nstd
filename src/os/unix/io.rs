//! Provides functionality for working with input & output on Unix platforms.
pub(crate) mod stdio;
use crate::{core::result::NSTDResult, NSTDUInt};
use core::ffi::c_int;
use libc::{
    EACCES, EAGAIN, EBADF, ECONNRESET, EINTR, EINVAL, EISDIR, ENETDOWN, ENETUNREACH, ENOMEM,
    ENOTCONN, EPIPE, ESPIPE, ETIMEDOUT, EWOULDBLOCK,
};
use nstdapi::nstdapi;

/// An error type for Unix I/O operations.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDUnixIOError {
    /// No error occurred.
    NSTD_UNIX_IO_ERROR_NONE,
    /// An unknown error occurred.
    NSTD_UNIX_IO_ERROR_UNKNOWN,
    /// An entity, such as a file, was not found.
    NSTD_UNIX_IO_ERROR_NOT_FOUND,
    /// Permission was denied.
    NSTD_UNIX_IO_ERROR_PERMISSION_DENIED,
    /// The connection was reset by a remote server.
    NSTD_UNIX_IO_ERROR_CONNECTION_RESET,
    /// There is no connection.
    NSTD_UNIX_IO_ERROR_NO_CONNECTION,
    /// A seek operation failed because the file descriptor provided refers to a pipe, FIFO, or
    /// socket object.
    NSTD_UNIX_IO_ERROR_INVALID_SEEK,
    /// The operation failed because a pipe was closed.
    NSTD_UNIX_IO_ERROR_BROKEN_PIPE,
    /// The operation needs to block to complete.
    NSTD_UNIX_IO_ERROR_BLOCKING,
    /// A pathname was expected to refer to a regular file, but a directory was found.
    NSTD_UNIX_IO_ERROR_IS_DIR,
    /// Some input parameter was incorrect.
    NSTD_UNIX_IO_ERROR_INVALID_INPUT,
    /// Some input/output data had an incorrect format.
    NSTD_UNIX_IO_ERROR_INVALID_DATA,
    /// The I/O operation's timeout expired, causing it to be canceled.
    NSTD_UNIX_IO_ERROR_TIMED_OUT,
    /// The operation was interrupted.
    NSTD_UNIX_IO_ERROR_INTERRUPTED,
    /// A reader unexpectedly reached the end of a file.
    NSTD_UNIX_IO_ERROR_UNEXPECTED_EOF,
    /// An operation could not be completed, because it failed to allocate enough memory.
    NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY,
}
impl NSTDUnixIOError {
    /// Retrieves the last system error and turns it into an `NSTDUnixIOError`.
    #[allow(unused)]
    fn last() -> Self {
        #[allow(trivial_numeric_casts, unreachable_patterns)]
        match errno::errno().0 as c_int {
            0 => Self::NSTD_UNIX_IO_ERROR_NONE,
            EBADF => Self::NSTD_UNIX_IO_ERROR_NOT_FOUND,
            EACCES => Self::NSTD_UNIX_IO_ERROR_PERMISSION_DENIED,
            ECONNRESET => Self::NSTD_UNIX_IO_ERROR_CONNECTION_RESET,
            ENETDOWN | ENETUNREACH | ENOTCONN => Self::NSTD_UNIX_IO_ERROR_NO_CONNECTION,
            ESPIPE => Self::NSTD_UNIX_IO_ERROR_INVALID_SEEK,
            EPIPE => Self::NSTD_UNIX_IO_ERROR_BROKEN_PIPE,
            EAGAIN | EWOULDBLOCK => Self::NSTD_UNIX_IO_ERROR_BLOCKING,
            EISDIR => Self::NSTD_UNIX_IO_ERROR_IS_DIR,
            EINVAL => Self::NSTD_UNIX_IO_ERROR_INVALID_INPUT,
            ETIMEDOUT => Self::NSTD_UNIX_IO_ERROR_TIMED_OUT,
            EINTR => Self::NSTD_UNIX_IO_ERROR_INTERRUPTED,
            ENOMEM => Self::NSTD_UNIX_IO_ERROR_OUT_OF_MEMORY,
            _ => Self::NSTD_UNIX_IO_ERROR_UNKNOWN,
        }
    }
}

/// A result type that yields an [`NSTDUInt`] representing the number of bytes read or written by a
/// Unix I/O operation on success and a Unix I/O operation error code on failure.
pub type NSTDUnixIOResult = NSTDResult<NSTDUInt, NSTDUnixIOError>;

/// Represents a raw Unix file descriptor.
pub type NSTDUnixFileDescriptor = c_int;
