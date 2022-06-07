#ifndef NSTD_HEAP_PTR_H_INCLUDED
#define NSTD_HEAP_PTR_H_INCLUDED
#include "core/ptr.h"
#include "nstd.h"

/// A pointer type for single value heap allocation.
typedef struct {
    /// A pointer to the value on the heap.
    NSTDPtr ptr;
} NSTDHeapPtr;

#endif
