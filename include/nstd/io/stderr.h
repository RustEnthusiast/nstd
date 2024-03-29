#ifndef NSTD_IO_STDERR_H
#define NSTD_IO_STDERR_H
#include "../core/optional.h"
#include "../core/slice.h"
#include "../nstd.h"
#include "io.h"

/// A handle to the standard error stream.
typedef struct {
    /// Rust's [Stderr].
    NSTDAnyMut err;
} NSTDStderr;

/// Represents an optional value of type `NSTDStderr`.
NSTDOptional(NSTDStderr) NSTDOptionalStderr;

/// Constructs a new handle to the standard error stream.
///
/// # Returns
///
/// `NSTDOptionalStderr handle` - A handle to the standard error stream, or an uninitialized "none"
/// variant on error.
NSTDAPI NSTDOptionalStderr nstd_io_stderr(void);

/// Writes some data to the standard error stream, returning how many bytes were written.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStderr *handle` - A handle to stderr.
///
/// - `const NSTDSlice *bytes` - The data to be written to stderr.
///
/// # Returns
///
/// `NSTDIOResult written` - The number of bytes written to `handle` on success, or the I/O
/// operation error code on failure.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOResult nstd_io_stderr_write(NSTDStderr *handle, const NSTDSlice *bytes);

/// Writes an entire buffer to the standard error stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStderr *handle` - A handle to stderr.
///
/// - `const NSTDSlice *bytes` - The data to be written to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOError nstd_io_stderr_write_all(NSTDStderr *handle, const NSTDSlice *bytes);

/// Flushes the standard error stream.
///
/// # Parameters:
///
/// - `NSTDStderr *handle` - A handle to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stderr_flush(NSTDStderr *handle);

/// Frees an instance of `NSTDStderr`.
///
/// # Parameters:
///
/// - `NSTDStderr handle` - A handle to the standard error stream.
NSTDAPI void nstd_io_stderr_free(NSTDStderr handle);

/// A locked handle to the standard error stream.
typedef struct {
    /// Rust's [StderrLock].
    NSTDAnyMut err;
} NSTDStderrLock;

/// Represents an optional value of type `NSTDStderrLock`.
NSTDOptional(NSTDStderrLock) NSTDOptionalStderrLock;

/// Constructs a new locked handle to the standard error stream.
///
/// # Returns
///
/// `NSTDOptionalStderrLock handle` - A locked handle to the standard error stream on success, or
/// an uninitialized "none" variant on error.
NSTDAPI NSTDOptionalStderrLock nstd_io_stderr_lock(void);

/// Writes some data to the standard error stream, returning how many bytes were written.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStderrLock *handle` - A locked handle to stderr.
///
/// - `const NSTDSlice *bytes` - The data to be written to stderr.
///
/// # Returns
///
/// `NSTDIOResult written` - The number of bytes written to `handle` on success, or the I/O
/// operation error code on failure.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOResult nstd_io_stderr_lock_write(NSTDStderrLock *handle, const NSTDSlice *bytes);

/// Writes an entire buffer to the standard error stream.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStderrLock *handle` - A locked handle to stderr.
///
/// - `const NSTDSlice *bytes` - The data to be written to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOError nstd_io_stderr_lock_write_all(NSTDStderrLock *handle, const NSTDSlice *bytes);

/// Flushes the standard error stream.
///
/// # Parameters:
///
/// - `NSTDStderrLock *handle` - A locked handle to stderr.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stderr_lock_flush(NSTDStderrLock *handle);

/// Frees and unlocks an instance of `NSTDStderrLock`.
///
/// # Parameters:
///
/// - `NSTDStderrLock handle` - A locked handle to the standard error stream.
NSTDAPI void nstd_io_stderr_unlock(NSTDStderrLock handle);

#endif
