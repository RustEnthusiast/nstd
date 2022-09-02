#ifndef NSTD_IO_STDERR_H
#define NSTD_IO_STDERR_H
#include "../core/slice.h"
#include "../nstd.h"
#include "io.h"

/// A handle to the standard error stream.
typedef NSTDAnyMut NSTDStderr;

/// Constructs a new handle to the standard error stream.
///
/// # Returns
///
/// `NSTDStderr handle` - A handle to the standard error stream.
NSTDAPI NSTDStderr nstd_io_stderr();

/// Writes some data to the standard error stream, setting `written` to the number of bytes
/// written.
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
/// - `NSTDUInt *written` - Returns as the number of bytes written.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOError nstd_io_stderr_write(NSTDStderr *handle, const NSTDSlice *bytes,
NSTDUInt *written);

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

#endif
