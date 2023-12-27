//! A reference counting smart pointer.
use crate::{
    core::{
        alloc::{
            nstd_core_alloc_layout_align, nstd_core_alloc_layout_new, nstd_core_alloc_layout_size,
            NSTDAllocLayout, NSTDAllocator,
        },
        mem::nstd_core_mem_copy,
        optional::NSTDOptional,
    },
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
    /// The shared object's memory layout.
    layout: NSTDAllocLayout,
}
impl NSTDSharedPtr<'_> {
    /// Returns a copy of the number of pointers sharing the object.
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
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
    /// [`core::ptr::read_unaligned`] and [`core::ptr::write_unaligned`].
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    fn ptrs_mut(&self) -> *mut usize {
        // SAFETY:
        // - Shared pointers are always non-null.
        // - Shared pointers never allocate more than `isize::MAX` bytes for their value.
        unsafe { self.ptr.add(nstd_shared_ptr_size(self)).cast() }
    }
}
impl Drop for NSTDSharedPtr<'_> {
    /// [`NSTDSharedPtr`]'s destructor.
    fn drop(&mut self) {
        // SAFETY: Shared pointers are always non-null.
        unsafe {
            // Update the pointer count.
            let ptrs = self.ptrs_mut();
            #[allow(clippy::arithmetic_side_effects)]
            let new_size = self.ptrs() - 1;
            core::ptr::write_unaligned(ptrs, new_size);
            // If the pointer count is zero, free the data.
            if new_size == 0 {
                (self.allocator.deallocate)(self.allocator.state, self.ptr, self.layout);
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
/// - `NSTDAllocLayout layout` - The shared object's memory layout.
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
/// `init` must be a pointer to a value that is valid for reads based on `layout`.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::{
///     alloc::NSTD_ALLOCATOR,
///     core::alloc::nstd_core_alloc_layout_new,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new},
/// };
///
/// unsafe {
///     let v = i16::MIN;
///     let size = core::mem::size_of::<i16>();
///     let align = core::mem::align_of::<i16>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, layout, addr_of!(v).cast()).unwrap();
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<i16>() == v);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_shared_ptr_new(
    allocator: &NSTDAllocator,
    layout: NSTDAllocLayout,
    init: NSTDAny,
) -> NSTDOptionalSharedPtr<'_> {
    // Allocate a region of memory for the object and the pointer count.
    let size = nstd_core_alloc_layout_size(layout);
    if let Some(buffer_size) = size.checked_add(USIZE_SIZE) {
        let align = nstd_core_alloc_layout_align(layout);
        if let NSTDOptional::Some(layout) = nstd_core_alloc_layout_new(buffer_size, align) {
            let ptr = (allocator.allocate)(allocator.state, layout);
            if !ptr.is_null() {
                // Initialize the shared object.
                nstd_core_mem_copy(ptr.cast(), init.cast(), size);
                // Set the pointer count to one.
                let ptrs = ptr.add(size).cast::<usize>();
                core::ptr::write_unaligned(ptrs, 1);
                // Construct the pointer.
                return NSTDOptional::Some(NSTDSharedPtr {
                    allocator,
                    ptr,
                    layout,
                });
            }
        }
    }
    NSTDOptional::None
}

/// Creates a new zero-initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `const NSTDAllocator *allocator` - The memory allocator.
///
/// - `NSTDAllocLayout layout` - The shared object's memory layout.
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
///     core::alloc::nstd_core_alloc_layout_new,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new_zeroed},
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<u128>();
///     let align = core::mem::align_of::<u128>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let shared_ptr = nstd_shared_ptr_new_zeroed(&NSTD_ALLOCATOR, layout).unwrap();
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == 0);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_shared_ptr_new_zeroed(
    allocator: &NSTDAllocator,
    layout: NSTDAllocLayout,
) -> NSTDOptionalSharedPtr<'_> {
    // Allocate a region of memory for the object and the pointer count.
    let size = nstd_core_alloc_layout_size(layout);
    if let Some(buffer_size) = size.checked_add(USIZE_SIZE) {
        let align = nstd_core_alloc_layout_align(layout);
        if let NSTDOptional::Some(layout) = nstd_core_alloc_layout_new(buffer_size, align) {
            let ptr = (allocator.allocate_zeroed)(allocator.state, layout);
            if !ptr.is_null() {
                // Set the pointer count to one.
                let ptrs = ptr.add(size).cast::<usize>();
                core::ptr::write_unaligned(ptrs, 1);
                // Construct the pointer.
                return NSTDOptional::Some(NSTDSharedPtr {
                    allocator,
                    ptr,
                    layout,
                });
            }
        }
    }
    NSTDOptional::None
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
///     core::alloc::nstd_core_alloc_layout_new,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_share},
/// };
///
/// unsafe {
///     let v = 39u64;
///     let share;
///     {
///         let size = core::mem::size_of::<u64>();
///         let align = core::mem::align_of::<u64>();
///         let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///         let addr = addr_of!(v).cast();
///         let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, layout, addr).unwrap();
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
        #[allow(clippy::arithmetic_side_effects)]
        core::ptr::write_unaligned(ptrs, *ptrs + 1);
        // Construct the new shared pointer instance.
        NSTDSharedPtr {
            allocator: shared_ptr.allocator,
            ptr: shared_ptr.ptr,
            layout: shared_ptr.layout,
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
pub const fn nstd_shared_ptr_allocator<'a>(shared_ptr: &NSTDSharedPtr<'a>) -> &'a NSTDAllocator {
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
///     core::alloc::nstd_core_alloc_layout_new,
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
///         let size = core::mem::size_of::<i128>();
///         let align = core::mem::align_of::<i128>();
///         let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///         let addr = addr_of!(v).cast();
///         let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, layout, addr).unwrap();
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
///     core::alloc::nstd_core_alloc_layout_new,
///     shared_ptr::{nstd_shared_ptr_new_zeroed, nstd_shared_ptr_size},
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<f64>();
///     let align = core::mem::align_of::<f64>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let shared_ptr = nstd_shared_ptr_new_zeroed(&NSTD_ALLOCATOR, layout).unwrap();
///     assert!(nstd_shared_ptr_size(&shared_ptr) == size);
/// }
/// ```
#[inline]
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub const fn nstd_shared_ptr_size(shared_ptr: &NSTDSharedPtr<'_>) -> NSTDUInt {
    nstd_core_alloc_layout_size(shared_ptr.layout) - USIZE_SIZE
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
///     core::alloc::nstd_core_alloc_layout_new,
///     shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new},
/// };
///
/// unsafe {
///     let v = u128::MAX;
///     let size = core::mem::size_of::<u128>();
///     let align = core::mem::align_of::<u128>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let shared_ptr = nstd_shared_ptr_new(&NSTD_ALLOCATOR, layout, addr_of!(v).cast()).unwrap();
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == v);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_shared_ptr_get(shared_ptr: &NSTDSharedPtr<'_>) -> NSTDAny {
    shared_ptr.ptr
}

/// Frees an instance of `NSTDSharedPtr`.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
#[inline]
#[nstdapi]
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
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
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn nstd_shared_ptr_drop(
    shared_ptr: NSTDSharedPtr<'_>,
    callback: unsafe extern "C" fn(NSTDAnyMut),
) {
    callback(shared_ptr.ptr);
}
