#ifndef NSTD_IO_IO_H
#define NSTD_IO_IO_H
#include "../core/str.h"
#include "../nstd.h"
#include "../string.h"

/// An error type for I/O operations.
typedef enum {
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
} NSTDIOError;

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
NSTDAPI NSTDIOError nstd_io_print(const NSTDStr *output);

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
NSTDAPI NSTDIOError nstd_io_print_line(const NSTDStr *output);

/// Reads a line of UTF-8 input from stdin and pushes it onto `buffer` without the newline.
///
/// # Parameters:
///
/// - `NSTDString *buffer` - The string buffer to be extended with input from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_read(NSTDString *buffer);

/// Reads a line of UTF-8 input from stdin and pushes it onto `buffer`.
///
/// # Parameters:
///
/// - `NSTDString *buffer` - The string buffer to be extended with input from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_read_line(NSTDString *buffer);

#endif
