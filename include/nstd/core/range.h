#ifndef NSTD_CORE_RANGE_H
#define NSTD_CORE_RANGE_H
#include "../nstd.h"

/// A 32-bit floating point numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDFloat32 start;
    /// The higher bound of the range.
    NSTDFloat32 end;
} NSTDRangeF32;
/// A 64-bit floating point numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDFloat64 start;
    /// The higher bound of the range.
    NSTDFloat64 end;
} NSTDRangeF64;

/// An arch-bit signed numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDInt start;
    /// The higher bound of the range.
    NSTDInt end;
} NSTDRange;
/// An arch-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDUInt start;
    /// The higher bound of the range.
    NSTDUInt end;
} NSTDURange;

/// An 8-bit signed numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDInt8 start;
    /// The higher bound of the range.
    NSTDInt8 end;
} NSTDRangeI8;
/// An 8-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDUInt8 start;
    /// The higher bound of the range.
    NSTDUInt8 end;
} NSTDRangeU8;

/// A 16-bit signed numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDInt16 start;
    /// The higher bound of the range.
    NSTDInt16 end;
} NSTDRangeI16;
/// A 16-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDUInt16 start;
    /// The higher bound of the range.
    NSTDUInt16 end;
} NSTDRangeU16;

/// A 32-bit signed numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDInt32 start;
    /// The higher bound of the range.
    NSTDInt32 end;
} NSTDRangeI32;
/// A 32-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDUInt32 start;
    /// The higher bound of the range.
    NSTDUInt32 end;
} NSTDRangeU32;

/// A 64-bit signed numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDInt64 start;
    /// The higher bound of the range.
    NSTDInt64 end;
} NSTDRangeI64;
/// A 64-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range.
    NSTDUInt64 start;
    /// The higher bound of the range.
    NSTDUInt64 end;
} NSTDRangeU64;

#endif
