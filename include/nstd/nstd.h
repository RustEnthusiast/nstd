#ifndef NSTD_NSTD_H
#define NSTD_NSTD_H
#include "os/os.h"
#if __STDC_VERSION__ < 202311L
#    include <stdbool.h>
#endif
#include <stddef.h>
#include <stdint.h>
#ifdef __cplusplus
#    define NSTDCPP __cplusplus
#endif
#ifdef NSTD_OS_WINDOWS
#    ifdef NSTDCPP
#        define NSTDAPI extern "C" __declspec(dllexport)
#    else
#        define NSTDAPI __declspec(dllexport)
#    endif
#else
#    ifdef NSTDCPP
#        define NSTDAPI extern "C"
#    else
#        define NSTDAPI
#    endif
#endif

/// A null pointer value constant.
#ifndef NSTDCPP
#    define NSTD_NULL NULL
#else
#    define NSTD_NULL nullptr
#endif

/// Boolean value false (0).
#define NSTD_FALSE false
/// Boolean value true (1).
#define NSTD_TRUE true

/// An integral type who's size matches the target architecture's pointer size.
typedef ptrdiff_t NSTDInt;
/// An unsigned integral type who's size matches the target architecture's pointer size.
typedef size_t NSTDUInt;

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

/// An opaque pointer to some immutable data.
///
/// # Safety
///
/// Accessing any data through this pointer type is unsafe. Raw pointers have no way of knowing if
/// the data being pointed to is or isn't valid.
typedef const void *NSTDAny;
/// An opaque pointer to some mutable data.
///
/// # Safety
///
/// Accessing or mutating any data through this pointer type is unsafe. Raw pointers have no way of
/// knowing if the data being pointed to is or isn't valid.
typedef void *NSTDAnyMut;

/// A boolean type, can either be `NSTD_TRUE` (1) or `NSTD_FALSE` (0).
typedef bool NSTDBool;

#endif
