#ifndef NSTD_IO_STDIN_H_INCLUDED
#define NSTD_IO_STDIN_H_INCLUDED
#include "../nstd.h"
NSTDCPPSTART

/// A handle to the standard input stream.
///
/// This stream is buffered by default.
typedef NSTDAnyMut NSTDStdin;

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI NSTDStdin nstd_io_stdin();

/// Frees an instance of `NSTDStdin`.
///
/// # Parameters:
///
/// - `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI void nstd_io_stdin_free(NSTDStdin handle);

NSTDCPPEND
#endif
