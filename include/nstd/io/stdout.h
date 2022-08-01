#ifndef NSTD_IO_STDOUT_H_INCLUDED
#define NSTD_IO_STDOUT_H_INCLUDED
#include "../core/slice.h"
#include "../nstd.h"
#include "io.h"
NSTDCPPSTART

/// A handle to the standard output stream.
typedef NSTDAnyMut NSTDStdout;

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdout stdout` - A locked handle to the standard output stream.
NSTDAPI NSTDStdout nstd_io_stdout();

/// Writes some data to the standard output stream, setting `written` to the number of bytes
/// written.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the slice's element
/// size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdout *stdout` - A handle to stdout.
///
/// - `const NSTDSliceConst *bytes` - The data to be written to stdout.
///
/// - `NSTDUSize *written` - Returns as the number of bytes written.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function can cause undefined behavior if `bytes`'s data is invalid.
NSTDAPI NSTDIOError nstd_io_stdout_write(NSTDStdout *stdout, const NSTDSliceConst *bytes,
NSTDUSize *written);

/// Frees and unlocks an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout stdout` - A handle to the standard output stream.
NSTDAPI void nstd_io_stdout_free(NSTDStdout stdout);

NSTDCPPEND
#endif
