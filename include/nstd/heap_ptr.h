#ifndef NSTD_HEAP_PTR_H
#define NSTD_HEAP_PTR_H
#include "nstd.h"
NSTDCPPSTART

/// A pointer type for single value heap allocation.
typedef struct {
    /// A raw pointer to the value on the heap.
    NSTDAnyMut ptr;
    /// The size of the object in bytes.
    NSTDUInt size;
} NSTDHeapPtr;

/// Creates a new initialized heap allocated object.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size (in bytes) of the heap object.
///
/// - `NSTDAny init` - A pointer to the object to initialize the heap object with.
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
NSTDAPI NSTDHeapPtr nstd_heap_ptr_new(NSTDUInt element_size, NSTDAny init);

/// Creates a new zero-initialized heap allocated object.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size (in bytes) of the heap object.
///
/// # Returns
///
/// `NSTDHeapPtr hptr` - The new heap allocated object.
///
/// # Panics
///
/// This function will panic if either `element_size` is zero, or allocation fails.
NSTDAPI NSTDHeapPtr nstd_heap_ptr_new_zeroed(NSTDUInt element_size);

/// Creates a clone of a heap allocated object.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDHeapPtr cloned` - A new clone of the original heap object.
///
/// # Panics
///
/// This function will panic if allocation fails.
NSTDAPI NSTDHeapPtr nstd_heap_ptr_clone(const NSTDHeapPtr *hptr);

/// Returns the size of the heap allocated object.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDUInt size` - The size of the heap allocated object.
NSTDAPI NSTDUInt nstd_heap_ptr_size(const NSTDHeapPtr *hptr);

/// Returns an immutable raw pointer to the object on the heap.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDAny ptr` - A raw pointer to the object on the heap.
NSTDAPI NSTDAny nstd_heap_ptr_get(const NSTDHeapPtr *hptr);

/// Returns a raw pointer to the object on the heap.
///
/// # Parameters:
///
/// - `NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A raw pointer to the object on the heap.
NSTDAPI NSTDAnyMut nstd_heap_ptr_get_mut(NSTDHeapPtr *hptr);

/// Frees an instance of `NSTDHeapPtr`.
///
/// # Parameters:
///
/// - `NSTDHeapPtr hptr` - A pointer to the heap object.
NSTDAPI void nstd_heap_ptr_free(NSTDHeapPtr hptr);

NSTDCPPEND
#endif
