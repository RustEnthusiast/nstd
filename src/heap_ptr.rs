//! A pointer type for single value heap allocation.
use crate::{
    alloc::{
        nstd_alloc_allocate, nstd_alloc_allocate_zeroed, nstd_alloc_deallocate,
        NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
    },
    core::{
        mem::nstd_core_mem_copy,
        optional::{gen_optional, NSTDOptional},
    },
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_NULL,
};
use nstdapi::nstdapi;

/// A pointer type for single value heap allocation.
#[nstdapi]
#[derive(Debug)]
pub struct NSTDHeapPtr {
    /// A raw pointer to the value on the heap.
    ptr: NSTDAnyMut,
    /// The size of the object in bytes.
    size: NSTDUInt,
}
impl NSTDHeapPtr {
    /// Constructs a zero-sized [NSTDHeapPtr].
    #[inline]
    const fn zero_sized() -> Self {
        Self {
            ptr: NSTD_NULL,
            size: 0,
        }
    }
}
impl Drop for NSTDHeapPtr {
    /// [NSTDHeapPtr]'s destructor.
    ///
    /// # Panics
    ///
    /// Panics if deallocating fails.
    #[inline]
    fn drop(&mut self) {
        if self.size > 0 {
            // SAFETY: The heap object's size is non-zero.
            unsafe {
                assert!(nstd_alloc_deallocate(&mut self.ptr, self.size) == NSTD_ALLOC_ERROR_NONE);
            }
        }
    }
}
/// # Safety
///
/// The data that the heap pointer holds must be able to be safely sent between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Send for NSTDHeapPtr {}
/// # Safety
///
/// The data that the heap pointer holds must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDHeapPtr {}
gen_optional!(NSTDOptionalHeapPtr, NSTDHeapPtr);

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
/// This function will panic if allocation fails.
///
/// # Safety
///
/// `init` must be a pointer to a value that is valid for reads of `element_size` bytes.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::heap_ptr::nstd_heap_ptr_new;
///
/// const SIZE: usize = core::mem::size_of::<char>();
///
/// let v = '🦀';
/// let hptr = unsafe { nstd_heap_ptr_new(SIZE, addr_of!(v).cast()) };
/// ```
#[nstdapi]
pub unsafe fn nstd_heap_ptr_new(element_size: NSTDUInt, init: NSTDAny) -> NSTDHeapPtr {
    if element_size == 0 {
        NSTDHeapPtr::zero_sized()
    } else {
        let mem = nstd_alloc_allocate(element_size);
        assert!(!mem.is_null());
        nstd_core_mem_copy(mem.cast(), init.cast(), element_size);
        NSTDHeapPtr {
            ptr: mem,
            size: element_size,
        }
    }
}

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
/// This function will panic if allocation fails.
///
/// # Safety
///
/// The data to be stored in the heap pointer must be safely representable by an all-zero byte
/// pattern.
///
/// # Example
///
/// ```
/// use nstd_sys::heap_ptr::{nstd_heap_ptr_get, nstd_heap_ptr_new_zeroed};
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     let hptr = nstd_heap_ptr_new_zeroed(SIZE);
///     assert!(*nstd_heap_ptr_get(&hptr).cast::<u64>() == 0);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_heap_ptr_new_zeroed(element_size: NSTDUInt) -> NSTDHeapPtr {
    if element_size == 0 {
        NSTDHeapPtr::zero_sized()
    } else {
        // SAFETY: `element_size` is not 0.
        let mem = unsafe { nstd_alloc_allocate_zeroed(element_size) };
        assert!(!mem.is_null());
        NSTDHeapPtr {
            ptr: mem,
            size: element_size,
        }
    }
}

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
#[nstdapi]
pub fn nstd_heap_ptr_clone(hptr: &NSTDHeapPtr) -> NSTDHeapPtr {
    let size = nstd_heap_ptr_size(hptr);
    if size == 0 {
        NSTDHeapPtr::zero_sized()
    } else {
        // SAFETY: `size` is not 0.
        let mem = unsafe { nstd_alloc_allocate(size) };
        assert!(!mem.is_null());
        // SAFETY: Both pointers are non-null.
        unsafe { nstd_core_mem_copy(mem.cast(), hptr.ptr.cast(), size) };
        NSTDHeapPtr { ptr: mem, size }
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
/// `NSTDUInt size` - The size of the heap allocated object.
///
/// # Example
///
/// ```
/// use nstd_sys::heap_ptr::{nstd_heap_ptr_new_zeroed, nstd_heap_ptr_size};
///
/// const SIZE: usize = core::mem::size_of::<i32>();
///
/// let hptr = unsafe { nstd_heap_ptr_new_zeroed(SIZE) };
/// assert!(nstd_heap_ptr_size(&hptr) == SIZE);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_heap_ptr_size(hptr: &NSTDHeapPtr) -> NSTDUInt {
    hptr.size
}

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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::heap_ptr::{nstd_heap_ptr_get, nstd_heap_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = -46923i128;
///     let hptr = nstd_heap_ptr_new(SIZE, addr_of!(v).cast());
///     assert!(*nstd_heap_ptr_get(&hptr).cast::<i128>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_heap_ptr_get(hptr: &NSTDHeapPtr) -> NSTDAny {
    hptr.ptr
}

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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::heap_ptr::{nstd_heap_ptr_get_mut, nstd_heap_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = 32964i128;
///     let mut hptr = nstd_heap_ptr_new(SIZE, addr_of!(v).cast());
///     let hv = nstd_heap_ptr_get_mut(&mut hptr).cast::<i128>();
///     assert!(*hv == v);
///     *hv = -46923;
///     assert!(*hv != v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_heap_ptr_get_mut(hptr: &mut NSTDHeapPtr) -> NSTDAnyMut {
    hptr.ptr
}

/// Frees an instance of `NSTDHeapPtr`.
///
/// # Parameters:
///
/// - `NSTDHeapPtr hptr` - A pointer to the heap object.
///
/// # Panics
///
/// Panics if freeing the heap memory fails.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_heap_ptr_free(hptr: NSTDHeapPtr) {}
