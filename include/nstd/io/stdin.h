#ifndef NSTD_IO_STDIN_H_INCLUDED
#define NSTD_IO_STDIN_H_INCLUDED
#include "../core/slice.h"
#include "../nstd.h"
#include "io.h"
NSTDCPPSTART

/// A handle to the standard input stream.
typedef NSTDAnyMut NSTDStdin;

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI NSTDStdin nstd_io_stdin();

/// Reads some data from stdin into a byte slice buffer.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// - `NSTDUSize *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOErrorCode errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
NSTDAPI NSTDIOError nstd_io_stdin_read(NSTDStdin *handle, NSTDSliceMut *buffer, NSTDUSize *read);

/// Frees an instance of `NSTDStdin`.
///
/// # Parameters:
///
/// - `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI void nstd_io_stdin_free(NSTDStdin handle);

NSTDCPPEND
#endif
