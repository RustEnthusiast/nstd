//! A reference counting smart pointer.
use crate::{
    alloc::{
        nstd_alloc_allocate, nstd_alloc_allocate_zeroed, nstd_alloc_deallocate,
        NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
    },
    core::{
        mem::nstd_core_mem_copy,
        optional::{gen_optional, NSTDOptional},
    },
    NSTDAny, NSTDAnyMut, NSTDUInt,
};
use nstdapi::nstdapi;

/// The size (in bytes) of [usize].
const USIZE_SIZE: usize = core::mem::size_of::<usize>();

/// A reference counting smart pointer.
#[nstdapi]
#[derive(Debug)]
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
        unsafe { core::ptr::read_unaligned(self.ptr.add(nstd_shared_ptr_size(self)).cast()) }
    }

    /// Returns a mutable pointer to the number of pointers sharing the object.
    ///
    /// # Note
    ///
    /// The returned pointer may be unaligned, so reading/writing must be done with
    /// [core::ptr::read_unaligned] and [core::ptr::write_unaligned].
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
    /// Panics if deallocating fails.
    fn drop(&mut self) {
        // SAFETY: Shared pointers are always non-null.
        unsafe {
            // Update the pointer count.
            let ptrs = self.ptrs_mut();
            let new_size = self.ptrs() - 1;
            core::ptr::write_unaligned(ptrs, new_size);
            // If the pointer count is zero, free the data.
            if new_size == 0 {
                assert!(nstd_alloc_deallocate(&mut self.ptr, self.size) == NSTD_ALLOC_ERROR_NONE);
            }
        }
    }
}
gen_optional!(NSTDOptionalSharedPtr, NSTDSharedPtr);

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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<i16>();
///
/// let v = i16::MIN;
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<i16>() == v);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_shared_ptr_new(element_size: NSTDUInt, init: NSTDAny) -> NSTDSharedPtr {
    assert!(element_size <= isize::MAX as usize);
    // Allocate a region of memory for the object and the pointer count.
    let buffer_size = element_size + USIZE_SIZE;
    let raw = nstd_alloc_allocate(buffer_size);
    assert!(!raw.is_null());
    // Initialize the shared object.
    nstd_core_mem_copy(raw.cast(), init.cast(), element_size);
    // Set the pointer count to one.
    let ptrs = raw.add(element_size).cast::<usize>();
    core::ptr::write_unaligned(ptrs, 1);
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
///
/// # Safety
///
/// The data to be stored in the shared pointer must be safely representable by an all-zero byte
/// pattern.
///
/// # Example
///
/// ```
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new_zeroed};
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new_zeroed(SIZE);
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == 0);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_shared_ptr_new_zeroed(element_size: NSTDUInt) -> NSTDSharedPtr {
    // SAFETY: The allocated memory is validated after allocation.
    unsafe {
        assert!(element_size <= isize::MAX as usize);
        // Allocate a region of memory for the object and the pointer count.
        let buffer_size = element_size + USIZE_SIZE;
        let raw = nstd_alloc_allocate_zeroed(buffer_size);
        assert!(!raw.is_null());
        // Set the pointer count to one.
        let ptrs = raw.add(element_size).cast::<usize>();
        core::ptr::write_unaligned(ptrs, 1);
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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_share};
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     let v = 39u64;
///     let share;
///     {
///         let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///         share = nstd_shared_ptr_share(&shared_ptr);
///     }
///     assert!(*nstd_shared_ptr_get(&share).cast::<u64>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_share(shared_ptr: &NSTDSharedPtr) -> NSTDSharedPtr {
    // SAFETY: Shared pointers are always non-null.
    unsafe {
        // Update the pointer count.
        let ptrs = shared_ptr.ptrs_mut();
        core::ptr::write_unaligned(ptrs, shared_ptr.ptrs() + 1);
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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{
///     nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_owners, nstd_shared_ptr_share,
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = i128::MIN;
///     let share;
///     {
///         let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///         assert!(nstd_shared_ptr_owners(&shared_ptr) == 1);
///
///         share = nstd_shared_ptr_share(&shared_ptr);
///         assert!(nstd_shared_ptr_owners(&shared_ptr) == 2);
///
///         let temp = nstd_shared_ptr_share(&shared_ptr);
///         assert!(nstd_shared_ptr_owners(&temp) == 3);
///     }
///     assert!(nstd_shared_ptr_owners(&share) == 1);
///     assert!(*nstd_shared_ptr_get(&share).cast::<i128>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_owners(shared_ptr: &NSTDSharedPtr) -> NSTDUInt {
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
///
/// # Example
///
/// ```
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_new_zeroed, nstd_shared_ptr_size};
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// let shared_ptr = unsafe { nstd_shared_ptr_new_zeroed(SIZE) };
/// assert!(nstd_shared_ptr_size(&shared_ptr) == SIZE);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_size(shared_ptr: &NSTDSharedPtr) -> NSTDUInt {
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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// let v = u128::MAX;
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_get(shared_ptr: &NSTDSharedPtr) -> NSTDAny {
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
/// Panics if there are no more shared pointers referencing the shared data and freeing the heap
/// memory fails.
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_shared_ptr_free(shared_ptr: NSTDSharedPtr) {}
