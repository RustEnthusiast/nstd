#ifndef NSTD_CORE_RESULT_H
#define NSTD_CORE_RESULT_H

/// Defines a "result" type with success and error variants.
#define NSTDResult(T, E) typedef struct {\
    enum { NSTD_RESULT_OK, NSTD_RESULT_ERR } errc;\
    union { T ok; E err; } value;\
}

#endif
