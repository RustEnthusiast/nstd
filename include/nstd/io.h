#ifndef NSTD_IO_H_INCLUDED
#define NSTD_IO_H_INCLUDED
#include "core/cstr/cstr.h"
#include "core/def.h"
#include "nstd.h"
#include "string.h"
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

/// Reads a line of UTF-8 input from stdin and returns it, discarding the newline.
///
/// # Returns
///
/// `NSTDString input` - The input from stdin, or an empty string on error.
NSTDAPI NSTDString nstd_io_read();

NSTDCPPEND
#endif
