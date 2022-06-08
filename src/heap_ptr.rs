//! A pointer type for single value heap allocation.
use crate::{
    alloc::{nstd_alloc_allocate_zeroed, nstd_alloc_deallocate},
    core::ptr::{nstd_core_ptr_new, NSTDPtr},
    NSTDUSize,
};

/// A pointer type for single value heap allocation.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NSTDHeapPtr {
    /// A pointer to the value on the heap.
    ptr: NSTDPtr,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_heap_ptr_new(element_size: NSTDUSize) -> NSTDHeapPtr {
    assert!(element_size != 0);
    let mem = unsafe { nstd_alloc_allocate_zeroed(element_size) };
    assert!(!mem.is_null());
    NSTDHeapPtr {
        ptr: nstd_core_ptr_new(mem, element_size),
    }
}

/// Frees an instance of `NSTDHeapPtr`.
///
/// # Parameters:
///
/// - `NSTDHeapPtr *hptr` - A pointer to the heap object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_heap_ptr_free(hptr: &mut NSTDHeapPtr) {
    if !hptr.ptr.raw.is_null() {
        unsafe { nstd_alloc_deallocate(&mut hptr.ptr.raw, hptr.ptr.size) };
    }
}
