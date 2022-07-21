#ifndef NSTD_IO_H_INCLUDED
#define NSTD_IO_H_INCLUDED
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Writes a raw null-terminated C string to stdout.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The raw null-terminated C string.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - Writing `cstr`'s bytes to stdout failed.
///
/// - `2` - Flushing stdout failed.
///
/// # Safety
///
/// The provided C string must be null terminated, else this function can cause garbage bytes to be
/// written to stdout.
NSTDAPI NSTDErrorCode nstd_io_print(const NSTDChar *cstr);

NSTDCPPEND
#endif
