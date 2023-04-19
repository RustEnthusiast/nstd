#ifndef NSTD_CORE_OPTIONAL_H
#define NSTD_CORE_OPTIONAL_H
#include "../nstd.h"

/// Describes an `NSTDOptional` with no value.
#define NSTD_OPTIONAL_NONE 0
/// Describes an `NSTDOptional` with "some" initialized value.
#define NSTD_OPTIONAL_SOME 1

/// Represents an optional (possibly uninitialized) value.
#define NSTDOptional(T)   \
    typedef struct {      \
        NSTDUInt8 status; \
        union {           \
            T some;       \
        } value;          \
    }

/// Represents an optional value of type `NSTDBool`.
NSTDOptional(NSTDBool) NSTDOptionalBool;
/// Represents an optional value of type `NSTDChar`.
NSTDOptional(NSTDChar) NSTDOptionalChar;
/// Represents an optional value of type `NSTDChar8`.
NSTDOptional(NSTDChar8) NSTDOptionalChar8;
/// Represents an optional value of type `NSTDChar16`.
NSTDOptional(NSTDChar16) NSTDOptionalChar16;
/// Represents an optional value of type `NSTDChar32`.
NSTDOptional(NSTDChar32) NSTDOptionalChar32;
/// Represents an optional value of type `NSTDFloat32`.
NSTDOptional(NSTDFloat32) NSTDOptionalFloat32;
/// Represents an optional value of type `NSTDFloat64`.
NSTDOptional(NSTDFloat64) NSTDOptionalFloat64;
/// Represents an optional value of type `NSTDInt`.
NSTDOptional(NSTDInt) NSTDOptionalInt;
/// Represents an optional value of type `NSTDUInt`.
NSTDOptional(NSTDUInt) NSTDOptionalUInt;
/// Represents an optional value of type `NSTDInt8`.
NSTDOptional(NSTDInt8) NSTDOptionalInt8;
/// Represents an optional value of type `NSTDUInt8`.
NSTDOptional(NSTDUInt8) NSTDOptionalUInt8;
/// Represents an optional value of type `NSTDInt16`.
NSTDOptional(NSTDInt16) NSTDOptionalInt16;
/// Represents an optional value of type `NSTDUInt16`.
NSTDOptional(NSTDUInt16) NSTDOptionalUInt16;
/// Represents an optional value of type `NSTDInt32`.
NSTDOptional(NSTDInt32) NSTDOptionalInt32;
/// Represents an optional value of type `NSTDUInt32`.
NSTDOptional(NSTDUInt32) NSTDOptionalUInt32;
/// Represents an optional value of type `NSTDInt64`.
NSTDOptional(NSTDInt64) NSTDOptionalInt64;
/// Represents an optional value of type `NSTDUInt64`.
NSTDOptional(NSTDUInt64) NSTDOptionalUInt64;

#endif
