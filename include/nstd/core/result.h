#ifndef NSTD_CORE_RESULT_H
#define NSTD_CORE_RESULT_H

/// Describes an `NSTDResult` variant.
typedef enum {
    /// An error variant.
    NSTD_RESULT_STATUS_ERR,
    /// A successful variant.
    NSTD_RESULT_STATUS_OK
} NSTDResultStatus;

/// Defines a "result" type with success and error variants.
#define NSTDResult(T, E)         \
    typedef struct {             \
        NSTDResultStatus status; \
        union {                  \
            E err;               \
            T ok;                \
        } value;                 \
    }

#endif
