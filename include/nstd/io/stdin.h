#ifndef NSTD_IO_STDIN_H
#define NSTD_IO_STDIN_H
#include "../core/slice.h"
#include "../nstd.h"
#include "../string.h"
#include "../vec.h"
#include "io.h"

/// A handle to the standard input stream.
typedef struct {
    /// Rust's [Stdin].
    NSTDAnyMut in;
} NSTDStdin;

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI NSTDStdin nstd_io_stdin(void);

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
NSTDAPI NSTDIOResult nstd_io_stdin_read(NSTDStdin *handle, NSTDSliceMut *buffer);

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
NSTDAPI NSTDIOResult nstd_io_stdin_read_all(NSTDStdin *handle, NSTDVec *buffer);

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
NSTDAPI NSTDIOResult nstd_io_stdin_read_to_string(NSTDStdin *handle, NSTDString *buffer);

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
NSTDAPI NSTDIOError nstd_io_stdin_read_exact(NSTDStdin *handle, NSTDSliceMut *buffer);

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
NSTDAPI NSTDIOResult nstd_io_stdin_read_line(NSTDStdin *handle, NSTDString *buffer);

/// Frees an instance of `NSTDStdin`.
///
/// # Parameters:
///
/// - `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI void nstd_io_stdin_free(NSTDStdin handle);

/// A locked handle to the standard input stream.
typedef struct {
    /// Rust's [StdinLock].
    NSTDAnyMut in;
} NSTDStdinLock;

/// Constructs a new locked handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdinLock handle` - A locked handle to the standard input stream.
NSTDAPI NSTDStdinLock nstd_io_stdin_lock(void);

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
NSTDAPI NSTDIOResult nstd_io_stdin_lock_read(NSTDStdinLock *handle, NSTDSliceMut *buffer);

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
NSTDAPI NSTDIOResult nstd_io_stdin_lock_read_all(NSTDStdinLock *handle, NSTDVec *buffer);

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
NSTDAPI NSTDIOResult nstd_io_stdin_lock_read_to_string(NSTDStdinLock *handle, NSTDString *buffer);

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
NSTDAPI NSTDIOError nstd_io_stdin_lock_read_exact(NSTDStdinLock *handle, NSTDSliceMut *buffer);

/// Frees and unlocks an instance of `NSTDStdinLock`.
///
/// # Parameters:
///
/// - `NSTDStdinLock handle` - A locked handle to the standard input stream.
NSTDAPI void nstd_io_stdin_unlock(NSTDStdinLock handle);

#endif
