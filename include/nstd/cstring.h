#ifndef NSTD_CSTRING_H_INCLUDED
#define NSTD_CSTRING_H_INCLUDED
#include "nstd.h"
#include "vec.h"

/// A dynamically sized, null terminated, C string.
typedef struct {
    /// The underlying vector of `NSTDChar`s.
    NSTDVec bytes;
} NSTDCString;

#endif
