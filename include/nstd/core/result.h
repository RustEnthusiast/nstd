#ifndef NSTD_CORE_RESULT_H
#define NSTD_CORE_RESULT_H
#include "../nstd.h"

/// Describes an erroneous `NSTDResult` value.
#define NSTD_RESULT_ERR 0
/// Describes a successful `NSTDResult` value.
#define NSTD_RESULT_OK 1

/// Defines a "result" type with success and error variants.
#define NSTDResult(T, E)  \
    typedef struct {      \
        NSTDUInt8 status; \
        union {           \
            E err;        \
            T ok;         \
        } value;          \
    }

#endif
