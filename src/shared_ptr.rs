//! A reference counting smart pointer.
use crate::{
    alloc::NSTDAllocator,
    core::{mem::nstd_core_mem_copy, optional::NSTDOptional},
    NSTDAny, NSTDAnyMut, NSTDUInt,
};
use nstdapi::nstdapi;

/// The size (in bytes) of [usize].
const USIZE_SIZE: usize = core::mem::size_of::<usize>();

/// A reference counting smart pointer.
#[nstdapi]
pub struct NSTDSharedPtr<'a> {
    /// The memory allocator.
    allocator: &'a NSTDAllocator,
    /// A raw pointer to private data about the shared object.
    ptr: NSTDAnyMut,
    /// The size of the shared pointer's memory buffer.
    size: NSTDUInt,
}
impl NSTDSharedPtr<'_> {
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
impl Drop for NSTDSharedPtr<'_> {
    /// [NSTDSharedPtr]'s destructor.
    fn drop(&mut self) {
        // SAFETY: Shared pointers are always non-null.
        unsafe {
            // Update the pointer count.
            let ptrs = self.ptrs_mut();
            let new_size = self.ptrs() - 1;
            core::ptr::write_unaligned(ptrs, new_size);
            // If the pointer count is zero, free the data.
            if new_size == 0 {
                (self.allocator.deallocate)(self.allocator.state, &mut self.ptr, self.size);
            }
        }
    }
}

/// Represents an optional value of type `NSTDSharedPtr`.
pub type NSTDOptionalSharedPtr<'a> = NSTDOptional<NSTDSharedPtr<'a>>;

/// Creates a new initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt element_size` - The size of the shared object.
///
/// - `NSTDAny init` - A pointer to the object to initialize the shared pointer with.
///
/// # Returns
///
/// `NSTDOptionalSharedPtr shared_ptr` - The new shared pointer, or an uninitialized "none" variant
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new},
/// };
///
/// const SIZE: usize = core::mem::size_of::<i16>();
///
/// let v = i16::MIN;
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, SIZE, addr_of!(v).cast()).unwrap();
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<i16>() == v);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_shared_ptr_new(
    allocator: &NSTDAllocator,
    element_size: NSTDUInt,
    init: NSTDAny,
) -> NSTDOptionalSharedPtr<'_> {
    // Allocate a region of memory for the object and the pointer count.
    let buffer_size = element_size + USIZE_SIZE;
    let raw = (allocator.allocate)(allocator.state, buffer_size);
    if raw.is_null() {
        return NSTDOptional::None;
    }
    // Initialize the shared object.
    nstd_core_mem_copy(raw.cast(), init.cast(), element_size);
    // Set the pointer count to one.
    let ptrs = raw.add(element_size).cast::<usize>();
    core::ptr::write_unaligned(ptrs, 1);
    // Construct the pointer.
    NSTDOptional::Some(NSTDSharedPtr {
        allocator,
        ptr: raw,
        size: buffer_size,
    })
}

/// Creates a new zero-initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDUInt element_size` - The size of the shared object.
///
/// # Returns
///
/// `NSTDOptionalSharedPtr shared_ptr` - The yet to be shared pointer, or an uninitialized "none"
/// variant if allocating fails.
///
/// # Safety
///
/// The data to be stored in the shared pointer must be safely representable by an all-zero byte
/// pattern.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new_zeroed},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new_zeroed(&NSTD_ALLOCATOR, SIZE).unwrap();
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == 0);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_shared_ptr_new_zeroed(
    allocator: &NSTDAllocator,
    element_size: NSTDUInt,
) -> NSTDOptionalSharedPtr<'_> {
    // SAFETY: The allocated memory is validated after allocation.
    unsafe {
        // Allocate a region of memory for the object and the pointer count.
        let buffer_size = element_size + USIZE_SIZE;
        let raw = (allocator.allocate_zeroed)(allocator.state, buffer_size);
        if raw.is_null() {
            return NSTDOptional::None;
        }
        // Set the pointer count to one.
        let ptrs = raw.add(element_size).cast::<usize>();
        core::ptr::write_unaligned(ptrs, 1);
        // Construct the pointer.
        NSTDOptional::Some(NSTDSharedPtr {
            allocator,
            ptr: raw,
            size: buffer_size,
        })
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_share},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     let v = 39u64;
///     let share;
///     {
///         let addr = addr_of!(v).cast();
///         let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, SIZE, addr).unwrap();
///         share = nstd_shared_ptr_share(&shared_ptr);
///     }
///     assert!(*nstd_shared_ptr_get(&share).cast::<u64>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_share<'a>(shared_ptr: &NSTDSharedPtr<'a>) -> NSTDSharedPtr<'a> {
    // SAFETY: Shared pointers are always non-null.
    unsafe {
        // Update the pointer count.
        let ptrs = shared_ptr.ptrs_mut();
        core::ptr::write_unaligned(ptrs, shared_ptr.ptrs() + 1);
        // Construct the new shared pointer instance.
        NSTDSharedPtr {
            allocator: shared_ptr.allocator,
            ptr: shared_ptr.ptr,
            size: shared_ptr.size,
        }
    }
}

/// Returns an immutable reference to a shared object's allocator.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared object.
///
/// # Returns
///
/// `const NSTDAllocator *allocator` - The shared object's allocator.
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_allocator<'a>(shared_ptr: &NSTDSharedPtr<'a>) -> &'a NSTDAllocator {
    shared_ptr.allocator
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     shared_ptr::{
///         nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_owners, nstd_shared_ptr_share,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = i128::MIN;
///     let share;
///     {
///         let addr = addr_of!(v).cast();
///         let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, SIZE, addr).unwrap();
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
pub fn nstd_shared_ptr_owners(shared_ptr: &NSTDSharedPtr<'_>) -> NSTDUInt {
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     shared_ptr::{nstd_shared_ptr_new_zeroed, nstd_shared_ptr_size},
/// };
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new_zeroed(&NSTD_ALLOCATOR, SIZE).unwrap();
///     assert!(nstd_shared_ptr_size(&shared_ptr) == SIZE);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_size(shared_ptr: &NSTDSharedPtr<'_>) -> NSTDUInt {
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
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new},
/// };
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// let v = u128::MAX;
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, SIZE, addr_of!(v).cast()).unwrap();
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_shared_ptr_get(shared_ptr: &NSTDSharedPtr<'_>) -> NSTDAny {
    shared_ptr.ptr
}

/// Frees an instance of `NSTDSharedPtr`.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_shared_ptr_free(shared_ptr: NSTDSharedPtr<'_>) {}

/// Frees an instance of `NSTDSharedPtr` after invoking `callback` with the shared object.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
///
/// - `void (*callback)(NSTDAnyMut)` - The shared object's destructor.
///
/// # Safety
///
/// This operation makes a direct call on a C function pointer (`callback`).
#[inline]
#[nstdapi]
pub unsafe fn nstd_shared_ptr_drop(
    shared_ptr: NSTDSharedPtr<'_>,
    callback: unsafe extern "C" fn(NSTDAnyMut),
) {
    callback(shared_ptr.ptr);
}
