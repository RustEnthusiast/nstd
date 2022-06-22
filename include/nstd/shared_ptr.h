#ifndef NSTD_SHARED_PTR_H_INCLUDED
#define NSTD_SHARED_PTR_H_INCLUDED
#include "heap_ptr.h"
#include "nstd.h"

/// A reference counting smart pointer.
typedef struct {
    /// A heap pointer to private data about the shared object.
    NSTDHeapPtr ptr;
} NSTDSharedPtr;

#endif
