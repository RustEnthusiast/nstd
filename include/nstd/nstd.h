#ifndef NSTD_NSTD_H
#define NSTD_NSTD_H
#if defined(__WINDOWS__)\
    || defined(_WIN32)\
    || defined(_WIN64)\
    || defined(__WIN32__)\
    || defined(__TOS_WIN__)
#define NSTDAPI __declspec(dllexport)
#else
#define NSTDAPI
#endif
#ifdef __cplusplus
#define NSTDCPP __cplusplus
#define NSTDCPPSTART extern "C" {
#define NSTDCPPEND }
#else
#define NSTDCPPSTART
#define NSTDCPPEND
#endif
#include <stddef.h>
#include <stdint.h>

/// A null pointer value constant.
#ifndef NSTDCPP
#define NSTD_NULL NULL
#else
#define NSTD_NULL nullptr
#endif

/// Boolean value false (0).
#define NSTD_FALSE 0
/// Boolean value true (1).
#define NSTD_TRUE 1

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

/// An 8-bit character type.
typedef NSTDUInt8 NSTDChar8;
/// A 16-bit character type.
typedef NSTDUInt16 NSTDChar16;
/// A 32-bit character type.
typedef NSTDUInt32 NSTDChar32;
/// Represents a Unicode scalar value.
typedef NSTDChar32 NSTDUnichar;

/// A void pointer to some immutable data.
///
/// # Safety
///
/// Accessing any data through this pointer type is unsafe. Raw pointers have no way of knowing if
/// the data being pointed to is or isn't valid.
typedef const void *NSTDAnyConst;
/// A void pointer (a pointer to some arbitrary type).
///
/// # Safety
///
/// Accessing any data through this pointer type is unsafe. Raw pointers have no way of knowing if
/// the data being pointed to is or isn't valid.
typedef void *NSTDAnyMut;

/// A boolean type, can either be `NSTD_TRUE` (1) or `NSTD_FALSE` (0).
typedef NSTDUInt8 NSTDBool;

#endif
