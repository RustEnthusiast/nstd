#ifndef NSTD_CORE_RANGE_H_INCLUDED
#define NSTD_CORE_RANGE_H_INCLUDED
#include "../nstd.h"
#include "def.h"

/// A half-open (low inclusive, high exclusive) arch-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDUSize start;
    /// The higher bound of the range (exclusive).
    NSTDUSize end;
} NSTDURange;
/// A half-open (low inclusive, high exclusive) arch-bit signed numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDISize start;
    /// The higher bound of the range (exclusive).
    NSTDISize end;
} NSTDIRange;

/// A half-open (low inclusive, high exclusive) 8-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDUInt8 start;
    /// The higher bound of the range (exclusive).
    NSTDUInt8 end;
} NSTDURange8;
/// A half-open (low inclusive, high exclusive) 8-bit signed numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDInt8 start;
    /// The higher bound of the range (exclusive).
    NSTDInt8 end;
} NSTDIRange8;

/// A half-open (low inclusive, high exclusive) 16-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDUInt16 start;
    /// The higher bound of the range (exclusive).
    NSTDUInt16 end;
} NSTDURange16;
/// A half-open (low inclusive, high exclusive) 16-bit signed numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDInt16 start;
    /// The higher bound of the range (exclusive).
    NSTDInt16 end;
} NSTDIRange16;

/// A half-open (low inclusive, high exclusive) 32-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDUInt32 start;
    /// The higher bound of the range (exclusive).
    NSTDUInt32 end;
} NSTDURange32;
/// A half-open (low inclusive, high exclusive) 32-bit signed numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDInt32 start;
    /// The higher bound of the range (exclusive).
    NSTDInt32 end;
} NSTDIRange32;

/// A half-open (low inclusive, high exclusive) 64-bit unsigned numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDUInt64 start;
    /// The higher bound of the range (exclusive).
    NSTDUInt64 end;
} NSTDURange64;
/// A half-open (low inclusive, high exclusive) 64-bit signed numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDInt64 start;
    /// The higher bound of the range (exclusive).
    NSTDInt64 end;
} NSTDIRange64;

/// A half-open (low inclusive, high exclusive) 32-bit floating point numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDFloat32 start;
    /// The higher bound of the range (exclusive).
    NSTDFloat32 end;
} NSTDFRange32;
/// A half-open (low inclusive, high exclusive) 64-bit floating point numerical range.
typedef struct {
    /// The lower bound of the range (inclusive).
    NSTDFloat64 start;
    /// The higher bound of the range (exclusive).
    NSTDFloat64 end;
} NSTDFRange64;

#endif
