//! A pointer type for single value heap allocation.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_allocate_zeroed, nstd_alloc_deallocate},
    core::{
        def::{NSTDAny, NSTDAnyConst},
        mem::nstd_core_mem_copy,
        ptr::{nstd_core_ptr_new, NSTDPtr},
    },
    NSTDUSize,
};

/// A pointer type for single value heap allocation.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NSTDHeapPtr {
    /// A pointer to the value on the heap.
    ptr: NSTDPtr,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_heap_ptr_new(
    element_size: NSTDUSize,
    init: NSTDAnyConst,
) -> NSTDHeapPtr {
    assert!(element_size != 0);
    let mem = nstd_alloc_allocate(element_size);
    assert!(!mem.is_null());
    nstd_core_mem_copy(mem.cast(), init.cast(), element_size);
    NSTDHeapPtr {
        ptr: nstd_core_ptr_new(mem, element_size),
    }
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
pub extern "C" fn nstd_heap_ptr_new_zeroed(element_size: NSTDUSize) -> NSTDHeapPtr {
    assert!(element_size != 0);
    let mem = unsafe { nstd_alloc_allocate_zeroed(element_size) };
    assert!(!mem.is_null());
    NSTDHeapPtr {
        ptr: nstd_core_ptr_new(mem, element_size),
    }
}

/// Returns the size of the heap allocated object.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the heap allocated object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_heap_ptr_size(hptr: &NSTDHeapPtr) -> NSTDUSize {
    hptr.ptr.size
}

/// Returns a raw pointer to the object on the heap.
///
/// # Parameters:
///
/// - `NSTDHeapPtr *hptr` - The heap pointer.
///
/// # Returns
///
/// `NSTDAny ptr` - A raw pointer to the object on the heap.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_heap_ptr_get(hptr: &mut NSTDHeapPtr) -> NSTDAny {
    hptr.ptr.raw
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
