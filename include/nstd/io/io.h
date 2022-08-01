#ifndef NSTD_IO_IO_H_INCLUDED
#define NSTD_IO_IO_H_INCLUDED
#include "../core/cstr/cstr.h"
#include "../core/def.h"
#include "../nstd.h"
#include "../string.h"
NSTDCPPSTART

/// An error type for I/O operations.
typedef enum {
    /// No error occurred.
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
} NSTDIOError;

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
NSTDAPI NSTDErrorCode nstd_io_print(const NSTDCStrConst *output);

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
NSTDAPI NSTDErrorCode nstd_io_print_line(const NSTDCStrConst *output);

/// Reads a line of UTF-8 input from stdin and returns it, discarding the newline.
///
/// # Returns
///
/// `NSTDString input` - The input from stdin, or an empty string on error.
NSTDAPI NSTDString nstd_io_read();

/// Reads a line of UTF-8 input from stdin and returns it.
///
/// # Returns
///
/// `NSTDString input` - The input from stdin, or an empty string on error.
NSTDAPI NSTDString nstd_io_read_line();

NSTDCPPEND
#endif
