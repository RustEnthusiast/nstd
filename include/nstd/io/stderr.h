#ifndef NSTD_IO_STDERR_H_INCLUDED
#define NSTD_IO_STDERR_H_INCLUDED
#include "../nstd.h"
NSTDCPPSTART

/// A handle to the standard error stream.
typedef NSTDAnyMut NSTDStderr;

/// Constructs a new handle to the standard error stream.
///
/// # Returns
///
/// `NSTDStderr handle` - A handle to the standard error stream.
NSTDAPI NSTDStderr nstd_io_stderr();

/// Frees an instance of `NSTDStderr`.
///
/// # Parameters:
///
/// - `NSTDStderr handle` - A handle to the standard error stream.
NSTDAPI void nstd_io_stderr_free(NSTDStderr handle);

NSTDCPPEND
#endif
