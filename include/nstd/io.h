#ifndef NSTD_IO_H_INCLUDED
#define NSTD_IO_H_INCLUDED
#include "core/cstr/cstr.h"
#include "core/def.h"
#include "nstd.h"
NSTDCPPSTART

/// Writes a C string slice to stdout.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice to write to stdout.
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
/// The provided C string slice's data must be valid, else this function can cause garbage bytes to
/// be written to stdout.
NSTDAPI NSTDErrorCode nstd_io_print(const NSTDCStrConst *cstr);

NSTDCPPEND
#endif
