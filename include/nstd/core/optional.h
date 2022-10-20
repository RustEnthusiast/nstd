#ifndef NSTD_CORE_OPTIONAL_H
#define NSTD_CORE_OPTIONAL_H

/// Represents an optional (possibly uninitialized) value.
#define NSTDOptional(T) typedef struct {\
    enum { NSTD_OPTIONAL_SOME, NSTD_OPTIONAL_NONE } opt;\
    union { T some; } value;\
}

#endif
