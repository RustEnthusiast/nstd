#ifndef NSTD_IO_STDOUT_H_INCLUDED
#define NSTD_IO_STDOUT_H_INCLUDED
#include "../nstd.h"
NSTDCPPSTART

/// A handle to the standard output stream.
typedef NSTDAnyMut NSTDStdout;

/// Constructs a new handle to the standard output stream.
///
/// # Returns
///
/// `NSTDStdout stdout` - A locked handle to the standard output stream.
NSTDAPI NSTDStdout nstd_io_stdout();

/// Frees and unlocks an instance of `NSTDStdout`.
///
/// # Parameters:
///
/// - `NSTDStdout stdout` - A handle to the standard output stream.
NSTDAPI void nstd_io_stdout_free(NSTDStdout stdout);

NSTDCPPEND
#endif
