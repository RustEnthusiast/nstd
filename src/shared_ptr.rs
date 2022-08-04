//! A reference counting smart pointer.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_allocate_zeroed, nstd_alloc_deallocate},
    core::{
        mem::nstd_core_mem_copy,
        ptr::{nstd_core_ptr_mut_new, nstd_core_ptr_mut_size, NSTDPtrMut},
    },
    NSTDAnyConst, NSTDUSize,
};

/// The size (in bytes) of [usize].
const USIZE_SIZE: usize = core::mem::size_of::<usize>();

/// A reference counting smart pointer.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NSTDSharedPtr {
    /// A pointer to private data about the shared object.
    ptr: NSTDPtrMut,
}
impl NSTDSharedPtr {
    /// Returns the number of pointers sharing the object.
    #[inline]
    fn ptrs(&self) -> *mut usize {
        unsafe { self.ptr.raw.add(nstd_shared_ptr_size(self)).cast() }
    }
}
impl Drop for NSTDSharedPtr {
    /// [NSTDSharedPtr]'s destructor.
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // Update the pointer count.
            let ptrs = self.ptrs();
            *ptrs -= 1;
            // If the pointer count is zero, free the data.
            if *ptrs == 0 {
                let size = nstd_shared_ptr_size(self);
                nstd_alloc_deallocate(&mut self.ptr.raw, size);
            }
        }
    }
}

/// Creates a new initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size of the shared object.
///
/// - `NSTDAnyConst init` - A pointer to the object to initialize the shared pointer with.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The new shared pointer.
///
/// # Panics
///
/// This operation will panic if allocating fails.
///
/// # Safety
///
/// This operation is unsafe because passing `init` as a null pointer can cause undefined behavior.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_ptr_new(
    element_size: NSTDUSize,
    init: NSTDAnyConst,
) -> NSTDSharedPtr {
    // Allocate a region of memory for the object and the pointer count.
    let buffer_size = element_size + USIZE_SIZE;
    let raw = nstd_alloc_allocate(buffer_size);
    assert!(!raw.is_null());
    // Initialize the shared object.
    nstd_core_mem_copy(raw.cast(), init.cast(), element_size);
    // Set the pointer count to one.
    let ptrs = raw.add(element_size).cast::<usize>();
    *ptrs = 1;
    // Construct the pointer with `element_size`, this does not include the size of the pointer
    // count (a `usize`).
    NSTDSharedPtr {
        ptr: nstd_core_ptr_mut_new(raw, buffer_size),
    }
}

/// Creates a new zero-initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size of the shared object.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The yet to be shared pointer.
///
/// # Panics
///
/// This operation will panic if allocating fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_new_zeroed(element_size: NSTDUSize) -> NSTDSharedPtr {
    unsafe {
        // Allocate a region of memory for the object and the pointer count.
        let buffer_size = element_size + USIZE_SIZE;
        let raw = nstd_alloc_allocate_zeroed(buffer_size);
        assert!(!raw.is_null());
        // Set the pointer count to one.
        let ptrs = raw.add(element_size).cast::<usize>();
        *ptrs = 1;
        // Construct the pointer with `element_size`, this does not include the size of the pointer
        // count (a `usize`).
        NSTDSharedPtr {
            ptr: nstd_core_ptr_mut_new(raw, buffer_size),
        }
    }
}

/// Shares `shared_ptr`.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared object to share.
///
/// # Returns
///
/// `NSTDSharedPtr shared` - A new pointer pointing to the shared data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_share(shared_ptr: &NSTDSharedPtr) -> NSTDSharedPtr {
    unsafe {
        // Update the pointer count.
        let ptrs = shared_ptr.ptrs();
        *ptrs += 1;
        // Construct the new shared pointer instance.
        NSTDSharedPtr {
            ptr: nstd_core_ptr_mut_new(shared_ptr.ptr.raw, nstd_shared_ptr_size(shared_ptr)),
        }
    }
}

/// Returns the number of pointers that share `shared_ptr`'s data.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - An instance of a shared pointer.
///
/// # Returns
///
/// `NSTDUSize owners` - The number of pointers that share `shared_ptr`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_owners(shared_ptr: &NSTDSharedPtr) -> NSTDUSize {
    unsafe { *shared_ptr.ptrs() }
}

/// Returns the size of the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the shared object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_size(shared_ptr: &NSTDSharedPtr) -> NSTDUSize {
    nstd_core_ptr_mut_size(&shared_ptr.ptr) - USIZE_SIZE
}

/// Returns an immutable raw pointer to the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDAnyConst ptr` - A raw pointer to the shared object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_get(shared_ptr: &NSTDSharedPtr) -> NSTDAnyConst {
    shared_ptr.ptr.raw
}

/// Frees an instance of `NSTDSharedPtr`.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_shared_ptr_free(shared_ptr: NSTDSharedPtr) {}
