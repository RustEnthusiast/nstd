#ifndef NSTD_HEAP_PTR_H_INCLUDED
#define NSTD_HEAP_PTR_H_INCLUDED
#include "core/ptr.h"
#include "nstd.h"
NSTDCPPSTART

/// A pointer type for single value heap allocation.
typedef struct {
    /// A pointer to the value on the heap.
    NSTDPtr ptr;
} NSTDHeapPtr;

/// Creates a new zero-initialized heap allocated object.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size (in bytes) of the heap object.
///
/// # Returns
///
/// `NSTDHeapPtr hptr` - The new heap allocated object.
///
/// # Panics
///
/// This function will panic if either `element_size` is zero, or allocation fails.
NSTDAPI NSTDHeapPtr nstd_heap_ptr_new(NSTDUSize element_size);

/// Frees an instance of `NSTDHeapPtr`.
///
/// # Parameters:
///
/// - `NSTDHeapPtr *hptr` - A pointer to the heap object.
NSTDAPI void nstd_heap_ptr_free(NSTDHeapPtr *hptr);

NSTDCPPEND
#endif
