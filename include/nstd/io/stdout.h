#ifndef NSTD_IO_STDOUT_H
#define NSTD_IO_STDOUT_H
#include "../core/slice.h"
#include "../nstd.h"
#include "io.h"

/// A handle to the standard output stream.
typedef struct {
    /// Rust's [Stdout].
    NSTDAnyMut out;
} NSTDStdout;

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdout handle` - A handle to the standard output stream.
NSTDAPI NSTDStdout nstd_io_stdout(void);

/// Writes some data to the standard output stream, returning how many bytes were written.
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
/// `NSTDIOResult written` - The number of bytes written to `handle` on success, or the I/O
/// operation error code on failure.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOResult nstd_io_stdout_write(NSTDStdout *handle, const NSTDSlice *bytes);

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
NSTDAPI NSTDIOError nstd_io_stdout_write_all(NSTDStdout *handle, const NSTDSlice *bytes);

/// Flushes the standard output stream.
///
/// # Parameters:
///
/// - `NSTDStdout *handle` - A handle to stdout.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stdout_flush(NSTDStdout *handle);

/// Frees an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout handle` - A handle to the standard output stream.
NSTDAPI void nstd_io_stdout_free(NSTDStdout handle);

/// A locked handle to the standard output stream.
typedef struct {
    /// Rust's [StdoutLock].
    NSTDAnyMut out;
} NSTDStdoutLock;

/// Constructs a new locked handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdoutLock handle` - A locked handle to the standard output stream.
NSTDAPI NSTDStdoutLock nstd_io_stdout_lock(void);

/// Writes some data to the standard output stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdoutLock *handle` - A locked handle to stdout.
///
/// - `const NSTDSlice *bytes` - The data to be written to stdout.
///
/// # Returns
///
/// `NSTDIOResult written` - The number of bytes written to `handle` on success, or the I/O
/// operation error code on failure.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOResult nstd_io_stdout_lock_write(NSTDStdoutLock *handle, const NSTDSlice *bytes);

/// Writes an entire buffer to the standard output stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdoutLock *handle` - A locked handle to stdout.
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
NSTDAPI NSTDIOError nstd_io_stdout_lock_write_all(NSTDStdoutLock *handle, const NSTDSlice *bytes);

/// Flushes the standard output stream.
///
/// # Parameters:
///
/// - `NSTDStdoutLock *handle` - A locked handle to stdout.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stdout_lock_flush(NSTDStdoutLock *handle);

/// Frees and unlocks an instance of `NSTDStdoutLock`.
///
/// # Parameters:
///
/// - `NSTDStdoutLock handle` - A locked handle to the standard output stream.
NSTDAPI void nstd_io_stdout_unlock(NSTDStdoutLock handle);

#endif
