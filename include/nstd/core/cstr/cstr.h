#ifndef NSTD_CORE_CSTR_CSTR_H_INCLUDED
#define NSTD_CORE_CSTR_CSTR_H_INCLUDED
#include "../../nstd.h"
#include "../def.h"
#include "../slice.h"
NSTDCPPSTART

/// A mutable slice of a C string.
typedef struct {
    /// A pointer to the first character in the C string.
    NSTDChar *ptr;
    /// The length of the C string, excluding the null byte.
    NSTDUSize len;
} NSTDCStr;

/// Creates a new C string slice from a raw pointer and a size.
///
/// # Parameters:
///
/// - `NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUSize len` - The length of the C string, excluding the null byte.
///
/// # Returns
///
/// `NSTDCStr cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw`'s data must remain valid while the returned C string slice is in use.
NSTDAPI NSTDCStr nstd_core_cstr_new(NSTDChar *raw, NSTDUSize len);

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice of the C string slice's data.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned byte slice is in use.
NSTDAPI NSTDSliceConst nstd_core_cstr_as_bytes(const NSTDCStr *cstr);

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `NSTDCStr *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDChar *nstd_core_cstr_get(NSTDCStr *cstr, NSTDUSize pos);

/// Return an immutable pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStr *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
NSTDAPI const NSTDChar *nstd_core_cstr_get_const(const NSTDCStr *cstr, NSTDUSize pos);

/// An immutable slice of a C string.
typedef struct {
    /// A pointer to the first character in the C string.
    const NSTDChar *ptr;
    /// The length of the C string, excluding the null byte.
    NSTDUSize len;
} NSTDCStrConst;

/// Creates a new C string slice from a raw pointer and a size.
///
/// # Parameters:
///
/// - `const NSTDChar *raw` - A pointer to the first character to be in the C string slice.
///
/// - `NSTDUSize len` - The length of the C string, excluding the null byte.
///
/// # Returns
///
/// `NSTDCStrConst cstr` - The new C string slice, referencing `raw`'s data.
///
/// # Safety
///
/// `raw`'s data must remain valid while the returned C string slice is in use.
NSTDAPI NSTDCStrConst nstd_core_cstr_const_new(const NSTDChar *raw, NSTDUSize len);

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice of the C string slice's data.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned byte slice is in use.
NSTDAPI NSTDSliceConst nstd_core_cstr_const_as_bytes(const NSTDCStrConst *cstr);

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
///
/// # Safety
///
/// `cstr`'s data must remain valid while the returned pointer is in use.
NSTDAPI const NSTDChar *nstd_core_cstr_const_get(const NSTDCStrConst *cstr, NSTDUSize pos);

NSTDCPPEND
#endif
