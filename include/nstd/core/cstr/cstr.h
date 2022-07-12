#ifndef NSTD_CORE_CSTR_CSTR_H_INCLUDED
#define NSTD_CORE_CSTR_CSTR_H_INCLUDED
#include "../../nstd.h"
#include "../def.h"
#include "../slice.h"
NSTDCPPSTART

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
NSTDAPI NSTDSliceConst nstd_core_cstr_const_as_bytes(const NSTDCStrConst *cstr);

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
NSTDAPI const NSTDChar *nstd_core_cstr_const_as_ptr(const NSTDCStrConst *cstr);

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrConst *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string slice.
NSTDAPI NSTDUSize nstd_core_cstr_const_len(const NSTDCStrConst *cstr);

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
NSTDAPI const NSTDChar *nstd_core_cstr_const_get(const NSTDCStrConst *cstr, NSTDUSize pos);

/// A mutable slice of a C string.
typedef struct {
    /// A pointer to the first character in the C string.
    NSTDChar *ptr;
    /// The length of the C string, excluding the null byte.
    NSTDUSize len;
} NSTDCStrMut;

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
/// `NSTDCStrMut cstr` - The new C string slice, referencing `raw`'s data.
NSTDAPI NSTDCStrMut nstd_core_cstr_mut_new(NSTDChar *raw, NSTDUSize len);

/// Returns a byte slice of a C string slice's data.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDSliceConst bytes` - An immutable byte slice of the C string slice's data.
NSTDAPI NSTDSliceConst nstd_core_cstr_mut_as_bytes(const NSTDCStrMut *cstr);

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDChar *ptr` - A pointer to the first character in the C string.
NSTDAPI NSTDChar *nstd_core_cstr_mut_as_ptr(NSTDCStrMut *cstr);

/// Returns a pointer to the first character in a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `const NSTDChar *ptr` - A pointer to the first character in the C string.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_as_ptr_const(const NSTDCStrMut *cstr);

/// Returns the length of a C string slice.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the C string slice.
NSTDAPI NSTDUSize nstd_core_cstr_mut_len(const NSTDCStrMut *cstr);

/// Return a pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `NSTDCStrMut *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
NSTDAPI NSTDChar *nstd_core_cstr_mut_get(NSTDCStrMut *cstr, NSTDUSize pos);

/// Return an immutable pointer the character at `pos` in `cstr`.
///
/// # Parameters:
///
/// - `const NSTDCStrMut *cstr` - The C string.
///
/// - `NSTDUSize pos` - The position of the character to get.
///
/// # Returns
///
/// `const NSTDChar *chr` - A pointer to the character at `pos`, or null on error.
NSTDAPI const NSTDChar *nstd_core_cstr_mut_get_const(const NSTDCStrMut *cstr, NSTDUSize pos);

NSTDCPPEND
#endif
