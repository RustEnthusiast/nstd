#ifndef NSTD_OS_WINDOWS_STR_H
#define NSTD_OS_WINDOWS_STR_H
#include "../../core/str.h"
#include "../../nstd.h"
#include "../../vec.h"

/// Converts a UTF-8 string slice into a null-terminated UTF-16 encoded buffer.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDOptionalVec utf16` - The new UTF-16 encoded buffer on success, or an uninitialized "none"
/// variant on error.
///
/// # Safety
///
/// `str`'s data must be valid for reads, especially in terms of UTF-8 conformance.
NSTDAPI NSTDOptionalVec nstd_os_windows_str_to_utf16(const NSTDStr *str);

#endif
