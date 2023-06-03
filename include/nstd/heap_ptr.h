#ifndef NSTD_HEAP_PTR_H
#define NSTD_HEAP_PTR_H
#include "alloc.h"
#include "core/optional.h"
#include "nstd.h"

/// A pointer type for single value heap allocation.
typedef struct {
    /// The memory allocator.
    const NSTDAllocator *allocator;
    /// A raw pointer to the value on the heap.
    NSTDAnyMut ptr;
    /// The size of the object in bytes.
    NSTDUInt size;
} NSTDHeapPtr;

/// Represents an optional value of type `NSTDHeapPtr`.
NSTDOptional(NSTDHeapPtr) NSTDOptionalHeapPtr;

/// Creates a new initialized heap allocated object.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt element_size` - The size (in bytes) of the heap object.
///
/// - `NSTDAny init` - A pointer to the object to initialize the heap object with.
///
/// # Returns
///
/// `NSTDOptionalHeapPtr hptr` - The new heap allocated object, or an uninitialized "none" variant
/// if allocating fails.
///
/// # Safety
///
/// `init` must be a pointer to a value that is valid for reads of `element_size` bytes.
NSTDAPI NSTDOptionalHeapPtr
nstd_heap_ptr_new(const NSTDAllocator *allocator, NSTDUInt element_size, NSTDAny init);

/// Creates a new zero-initialized heap allocated object.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt element_size` - The size (in bytes) of the heap object.
///
/// # Returns
///
/// `NSTDOptionalHeapPtr hptr` - The new heap allocated object, or an uninitialized "none" variant
/// if allocating fails.
///
/// # Safety
///
/// The data to be stored in the heap pointer must be safely representable by an all-zero byte
/// pattern.
NSTDAPI NSTDOptionalHeapPtr
nstd_heap_ptr_new_zeroed(const NSTDAllocator *allocator, NSTDUInt element_size);

/// Creates a clone of a heap allocated object.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDOptionalHeapPtr cloned` - A new clone of the original heap object, or an uninitialized
/// "none" variant if allocating fails.
NSTDAPI NSTDOptionalHeapPtr nstd_heap_ptr_clone(const NSTDHeapPtr *hptr);

/// Returns an immutable reference to a heap object's allocator.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap object.
///
/// # Returns
///
/// `const NSTDAllocator *allocator` - The heap object's allocator.
NSTDAPI const NSTDAllocator *nstd_heap_ptr_allocator(const NSTDHeapPtr *hptr);

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
/// # Note
///
/// This will always return null if the size of the object being stored on the heap is 0.
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
/// # Note
///
/// This will always return null if the size of the object being stored on the heap is 0.
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

/// Frees an instance of `NSTDHeapPtr` after invoking `callback` with the heap object's data.
///
/// # Parameters:
///
/// - `NSTDHeapPtr hptr` - A pointer to the heap object.
///
/// - `void (*callback)(NSTDAnyMut)` - The heap object's destructor.
///
/// # Safety
///
/// This operation makes a direct call on a C function pointer (`callback`).
NSTDAPI void nstd_heap_ptr_drop(NSTDHeapPtr hptr, void (*callback)(NSTDAnyMut));

#endif
