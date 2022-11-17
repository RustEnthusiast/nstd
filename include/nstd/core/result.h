#ifndef NSTD_CORE_RESULT_H
#define NSTD_CORE_RESULT_H

/// Describes an `NSTDResult` variant.
typedef enum {
    /// A successful variant.
    NSTD_RESULT_STATUS_OK,
    /// An error variant.
    NSTD_RESULT_STATUS_ERR
} NSTDResultStatus;

/// Defines a "result" type with success and error variants.
#define NSTDResult(T, E) typedef struct {\
    NSTDResultStatus status;\
    union { T ok; E err; };\
}

#endif
