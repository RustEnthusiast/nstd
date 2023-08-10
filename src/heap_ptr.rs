//! A pointer type for single value heap allocation.
use crate::{
    alloc::NSTDAllocator,
    core::{mem::nstd_core_mem_copy, optional::NSTDOptional},
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_NULL,
};
use nstdapi::nstdapi;

/// A pointer type for single value heap allocation.
#[nstdapi]
pub struct NSTDHeapPtr<'a> {
    /// The memory allocator.
    allocator: &'a NSTDAllocator,
    /// A raw pointer to the value on the heap.
    ptr: NSTDAnyMut,
    /// The size of the object in bytes.
    size: NSTDUInt,
}
impl<'a> NSTDHeapPtr<'a> {
    /// Constructs a zero-sized [NSTDHeapPtr].
    #[inline]
    const fn zero_sized(allocator: &'a NSTDAllocator) -> Self {
        Self {
            allocator,
            ptr: NSTD_NULL,
            size: 0,
        }
    }
}
impl Drop for NSTDHeapPtr<'_> {
    /// [NSTDHeapPtr]'s destructor.
    #[inline]
    fn drop(&mut self) {
        if self.size > 0 {
            // SAFETY: The heap object's size is non-zero.
            unsafe { (self.allocator.deallocate)(self.allocator.state, &mut self.ptr, self.size) };
        }
    }
}
/// # Safety
///
/// The data that the heap pointer holds must be able to be safely sent between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Send for NSTDHeapPtr<'_> {}
/// # Safety
///
/// The data that the heap pointer holds must be able to be safely shared between threads.
// SAFETY: The user guarantees that the data is thread-safe.
unsafe impl Sync for NSTDHeapPtr<'_> {}

/// Represents an optional value of type `NSTDHeapPtr`.
pub type NSTDOptionalHeapPtr<'a> = NSTDOptional<NSTDHeapPtr<'a>>;

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
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::{alloc::NSTD_ALLOCATOR, heap_ptr::nstd_heap_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<char>();
///
/// let v = '🦀';
/// let hptr = unsafe { nstd_heap_ptr_new(&NSTD_ALLOCATOR, SIZE, addr_of!(v).cast()).unwrap() };
/// ```
#[nstdapi]
pub unsafe fn nstd_heap_ptr_new(
    allocator: &NSTDAllocator,
    element_size: NSTDUInt,
    init: NSTDAny,
) -> NSTDOptionalHeapPtr {
    if element_size == 0 {
        NSTDOptional::Some(NSTDHeapPtr::zero_sized(allocator))
    } else {
        let mem = (allocator.allocate)(allocator.state, element_size);
        if mem.is_null() {
            return NSTDOptional::None;
        }
        nstd_core_mem_copy(mem.cast(), init.cast(), element_size);
        NSTDOptional::Some(NSTDHeapPtr {
            allocator,
            ptr: mem,
            size: element_size,
        })
    }
}

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
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     heap_ptr::{nstd_heap_ptr_get, nstd_heap_ptr_new_zeroed},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     let hptr = nstd_heap_ptr_new_zeroed(&NSTD_ALLOCATOR, SIZE).unwrap();
///     assert!(*nstd_heap_ptr_get(&hptr).cast::<u64>() == 0);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_heap_ptr_new_zeroed(
    allocator: &NSTDAllocator,
    element_size: NSTDUInt,
) -> NSTDOptionalHeapPtr {
    if element_size == 0 {
        NSTDOptional::Some(NSTDHeapPtr::zero_sized(allocator))
    } else {
        // SAFETY: `element_size` is not 0.
        let mem = unsafe { (allocator.allocate_zeroed)(allocator.state, element_size) };
        if mem.is_null() {
            return NSTDOptional::None;
        }
        NSTDOptional::Some(NSTDHeapPtr {
            allocator,
            ptr: mem,
            size: element_size,
        })
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
/// `NSTDOptionalHeapPtr cloned` - A new clone of the original heap object, or an uninitialized
/// "none" variant if allocating fails.
#[nstdapi]
pub fn nstd_heap_ptr_clone<'a>(hptr: &NSTDHeapPtr<'a>) -> NSTDOptionalHeapPtr<'a> {
    let size = nstd_heap_ptr_size(hptr);
    if size == 0 {
        NSTDOptional::Some(NSTDHeapPtr::zero_sized(hptr.allocator))
    } else {
        // SAFETY: `size` is not 0.
        let mem = unsafe { (hptr.allocator.allocate)(hptr.allocator.state, size) };
        if mem.is_null() {
            return NSTDOptional::None;
        }
        // SAFETY: Both pointers are non-null.
        unsafe { nstd_core_mem_copy(mem.cast(), hptr.ptr.cast(), size) };
        NSTDOptional::Some(NSTDHeapPtr {
            allocator: hptr.allocator,
            ptr: mem,
            size,
        })
    }
}

/// Returns an immutable reference to a heap object's allocator.
///
/// # Parameters:
///
/// - `const NSTDHeapPtr *hptr` - The heap object.
///
/// # Returns
///
/// `const NSTDAllocator *allocator` - The heap object's allocator.
#[inline]
#[nstdapi]
pub fn nstd_heap_ptr_allocator<'a>(hptr: &NSTDHeapPtr<'a>) -> &'a NSTDAllocator {
    hptr.allocator
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     heap_ptr::{nstd_heap_ptr_new_zeroed, nstd_heap_ptr_size},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i32>();
///
/// unsafe {
///     let hptr = nstd_heap_ptr_new_zeroed(&NSTD_ALLOCATOR, SIZE).unwrap();
///     assert!(nstd_heap_ptr_size(&hptr) == SIZE);
/// }
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     heap_ptr::{nstd_heap_ptr_get, nstd_heap_ptr_new},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = -46923i128;
///     let hptr = nstd_heap_ptr_new(&NSTD_ALLOCATOR, SIZE, addr_of!(v).cast()).unwrap();
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     heap_ptr::{nstd_heap_ptr_get_mut, nstd_heap_ptr_new},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = 32964i128;
///     let mut hptr = nstd_heap_ptr_new(&NSTD_ALLOCATOR, SIZE, addr_of!(v).cast()).unwrap();
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
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_heap_ptr_free(hptr: NSTDHeapPtr) {}

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
#[inline]
#[nstdapi]
pub unsafe fn nstd_heap_ptr_drop(hptr: NSTDHeapPtr, callback: unsafe extern "C" fn(NSTDAnyMut)) {
    callback(hptr.ptr);
}
