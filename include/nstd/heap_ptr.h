#ifndef NSTD_HEAP_PTR_H_INCLUDED
#define NSTD_HEAP_PTR_H_INCLUDED
#include "core/def.h"
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

/// Creates a new initialized heap allocated object.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size (in bytes) of the heap object.
///
/// - `NSTDAnyConst init` - A pointer to the object to initialize the heap object with.
///
/// # Returns
///
/// `NSTDHeapPtr hptr` - The new heap allocated object.
///
/// # Panics
///
/// This function will panic if either `element_size` is zero, or allocation fails.
///
/// # Safety
///
/// This operation is unsafe because passing `init` as a null pointer can cause undefined behavior.
NSTDAPI NSTDHeapPtr nstd_heap_ptr_new_initialized(NSTDUSize element_size, NSTDAnyConst init);

/// Returns a raw pointer to the object on the heap.
///
/// # Parameters:
///
/// - `NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDAny ptr` - A raw pointer to the object on the heap.
NSTDAPI NSTDAny nstd_heap_ptr_read(NSTDHeapPtr *hptr);

/// Frees an instance of `NSTDHeapPtr`.
///
/// # Parameters:
///
/// - `NSTDHeapPtr *hptr` - A pointer to the heap object.
NSTDAPI void nstd_heap_ptr_free(NSTDHeapPtr *hptr);

NSTDCPPEND
#endif
