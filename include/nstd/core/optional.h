#ifndef NSTD_CORE_OPTIONAL_H
#define NSTD_CORE_OPTIONAL_H

/// Describes an `NSTDOptional` variant.
typedef enum {
    /// No value.
    NSTD_OPTIONAL_STATUS_NONE,
    /// "Some" initialized value.
    NSTD_OPTIONAL_STATUS_SOME
} NSTDOptionalStatus;

/// Represents an optional (possibly uninitialized) value.
#define NSTDOptional(T) typedef struct {\
    NSTDOptionalStatus status;\
    union { T some; };\
}

#endif
