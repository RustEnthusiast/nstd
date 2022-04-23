#ifndef NSTD_CORE_DEF_H_INCLUDED
#define NSTD_CORE_DEF_H_INCLUDED
#include "../nstd.h"
#include <stddef.h>
#include <stdint.h>

/// A void pointer (a pointer to some arbitrary type).
typedef void *NSTDAny;
/// A void pointer to some immutable data.
typedef const void *NSTDAnyConst;

/// An integral type who's size matches the target architecture's pointer size.
typedef ptrdiff_t NSTDISize;
/// An unsigned integral type who's size matches the target architecture's pointer size.
typedef size_t NSTDUSize;

/// An 8-bit signed integer type.
typedef int_least8_t NSTDInt8;
/// An 8-bit unsigned integer type.
typedef uint_least8_t NSTDUInt8;
/// A 16-bit signed integer type.
typedef int_least16_t NSTDInt16;
/// A 16-bit unsigned integer type.
typedef uint_least16_t NSTDUInt16;
/// A 32-bit signed integer type.
typedef int_least32_t NSTDInt32;
/// A 32-bit unsigned integer type.
typedef uint_least32_t NSTDUInt32;
/// A 64-bit signed integer type.
typedef int_least64_t NSTDInt64;
/// A 64-bit unsigned integer type.
typedef uint_least64_t NSTDUInt64;

/// A 32-bit floating point type.
typedef float NSTDFloat32;
/// A 64-bit floating point type.
typedef double NSTDFloat64;

/// Equivalent to C's `char` type.
typedef char NSTDChar;
/// An 8-bit character type.
typedef NSTDUInt8 NSTDChar8;
/// A 16-bit character type.
typedef NSTDUInt16 NSTDChar16;
/// A 32-bit character type.
typedef NSTDUInt32 NSTDChar32;
/// Represents a Unicode scalar value.
typedef NSTDChar32 NSTDUnichar;

/// The smallest addressable unit of memory.
typedef NSTDUInt8 NSTDByte;

/// An error code type to be returned from functions. An error code of 0 means success, while
/// anything else indicates failure.
typedef NSTDInt32 NSTDErrorCode;

/// A boolean type, can either be `NSTD_BOOL_TRUE` (1) or `NSTD_BOOL_FALSE` (0).
typedef enum {
    /// Boolean value false (0).
    NSTD_BOOL_FALSE,
    /// Boolean value true (1).
    NSTD_BOOL_TRUE
} NSTDBool;

#endif
