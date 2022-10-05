//! A reference counting smart pointer.
use crate::{
    alloc::{nstd_alloc_allocate, nstd_alloc_allocate_zeroed, nstd_alloc_deallocate},
    core::mem::nstd_core_mem_copy,
    NSTDAny, NSTDAnyMut, NSTDUInt,
};

/// The size (in bytes) of [usize].
const USIZE_SIZE: usize = core::mem::size_of::<usize>();

/// A reference counting smart pointer.
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct NSTDSharedPtr {
    /// A raw pointer to private data about the shared object.
    ptr: NSTDAnyMut,
    /// The size of the shared pointer's memory buffer.
    size: NSTDUInt,
}
impl NSTDSharedPtr {
    /// Returns a copy of the number of pointers sharing the object.
    #[inline]
    fn ptrs(&self) -> usize {
        // SAFETY:
        // - Shared pointers are always non-null.
        // - Shared pointers never allocate more than `isize::MAX` bytes for their value.
        unsafe { *self.ptr.add(nstd_shared_ptr_size(self)).cast() }
    }

    /// Returns a mutable pointer to the number of pointers sharing the object.
    #[inline]
    fn ptrs_mut(&self) -> *mut usize {
        // SAFETY:
        // - Shared pointers are always non-null.
        // - Shared pointers never allocate more than `isize::MAX` bytes for their value.
        unsafe { self.ptr.add(nstd_shared_ptr_size(self)).cast() }
    }
}
impl Drop for NSTDSharedPtr {
    /// [NSTDSharedPtr]'s destructor.
    ///
    /// # Panics
    ///
    /// This operation may panic if getting a handle to the heap fails.
    fn drop(&mut self) {
        // SAFETY: Shared pointers are always non-null.
        unsafe {
            // Update the pointer count.
            let ptrs = self.ptrs_mut();
            *ptrs -= 1;
            // If the pointer count is zero, free the data.
            if *ptrs == 0 {
                nstd_alloc_deallocate(&mut self.ptr, self.size);
            }
        }
    }
}

/// Creates a new initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size of the shared object.
///
/// - `NSTDAny init` - A pointer to the object to initialize the shared pointer with.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The new shared pointer.
///
/// # Panics
///
/// This operation will panic if either `element_size` is greater than `NSTDInt`'s max value or
/// allocating fails.
///
/// # Safety
///
/// `init` must be a pointer to a value that is valid for reads of `element_size` bytes.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_ptr_new(
    element_size: NSTDUInt,
    init: NSTDAny,
) -> NSTDSharedPtr {
    assert!(element_size <= isize::MAX as usize);
    // Allocate a region of memory for the object and the pointer count.
    let buffer_size = element_size + USIZE_SIZE;
    let raw = nstd_alloc_allocate(buffer_size);
    assert!(!raw.is_null());
    // Initialize the shared object.
    nstd_core_mem_copy(raw.cast(), init.cast(), element_size);
    // Set the pointer count to one.
    let ptrs = raw.add(element_size).cast::<usize>();
    *ptrs = 1;
    // Construct the pointer.
    NSTDSharedPtr {
        ptr: raw,
        size: buffer_size,
    }
}

/// Creates a new zero-initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size of the shared object.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The yet to be shared pointer.
///
/// # Panics
///
/// This operation will panic if either `element_size` is greater than `NSTDInt`'s max value or
/// allocating fails.
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_new_zeroed(element_size: NSTDUInt) -> NSTDSharedPtr {
    // SAFETY: The allocated memory is validated after allocation.
    unsafe {
        assert!(element_size <= isize::MAX as usize);
        // Allocate a region of memory for the object and the pointer count.
        let buffer_size = element_size + USIZE_SIZE;
        let raw = nstd_alloc_allocate_zeroed(buffer_size);
        assert!(!raw.is_null());
        // Set the pointer count to one.
        let ptrs = raw.add(element_size).cast::<usize>();
        *ptrs = 1;
        // Construct the pointer.
        NSTDSharedPtr {
            ptr: raw,
            size: buffer_size,
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
    // SAFETY: Shared pointers are always non-null.
    unsafe {
        // Update the pointer count.
        let ptrs = shared_ptr.ptrs_mut();
        *ptrs += 1;
        // Construct the new shared pointer instance.
        NSTDSharedPtr {
            ptr: shared_ptr.ptr,
            size: shared_ptr.size,
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
/// `NSTDUInt owners` - The number of pointers that share `shared_ptr`'s data.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_owners(shared_ptr: &NSTDSharedPtr) -> NSTDUInt {
    shared_ptr.ptrs()
}

/// Returns the size of the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDUInt size` - The size of the shared object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_size(shared_ptr: &NSTDSharedPtr) -> NSTDUInt {
    shared_ptr.size - USIZE_SIZE
}

/// Returns an immutable raw pointer to the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDAny ptr` - A raw pointer to the shared object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_shared_ptr_get(shared_ptr: &NSTDSharedPtr) -> NSTDAny {
    shared_ptr.ptr
}

/// Frees an instance of `NSTDSharedPtr`.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
///
/// # Panics
///
/// This operation may panic if getting a handle to the heap fails.
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_shared_ptr_free(shared_ptr: NSTDSharedPtr) {}
