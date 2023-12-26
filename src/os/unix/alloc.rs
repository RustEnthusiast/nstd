//! Memory allocation for Unix-like systems.
use crate::{
    core::{
        alloc::{nstd_core_alloc_layout_align, nstd_core_alloc_layout_size, NSTDAllocLayout},
        mem::{nstd_core_mem_copy, nstd_core_mem_zero},
    },
    NSTDAnyMut, NSTD_NULL,
};
use libc::{free, posix_memalign};
use nstdapi::nstdapi;

/// Describes an error returned from an `nstd.os.unix.alloc` function.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDUnixAllocError {
    /// No error occurred.
    NSTD_UNIX_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_UNIX_ALLOC_ERROR_OUT_OF_MEMORY,
}

/// Allocates a block of memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// - Behavior is undefined if `layout`'s size is zero.
///
/// - The new memory buffer should be considered uninitialized.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::alloc::nstd_core_alloc_layout_new,
///     os::unix::alloc::{nstd_os_unix_alloc_allocate, nstd_os_unix_alloc_deallocate},
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<u128>();
///     let align = core::mem::align_of::<u128>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mem = nstd_os_unix_alloc_allocate(layout);
///     assert!(!mem.is_null());
///     nstd_os_unix_alloc_deallocate(mem);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_unix_alloc_allocate(layout: NSTDAllocLayout) -> NSTDAnyMut {
    let size = nstd_core_alloc_layout_size(layout);
    let align = nstd_core_alloc_layout_align(layout).max(core::mem::size_of::<NSTDAnyMut>());
    let mut ptr = NSTD_NULL;
    posix_memalign(&mut ptr, align, size);
    ptr
}

/// Allocates a block of zero initialized memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// Behavior is undefined if `layout`'s size is zero.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::alloc::nstd_core_alloc_layout_new,
///     os::unix::alloc::{nstd_os_unix_alloc_allocate_zeroed, nstd_os_unix_alloc_deallocate},
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<isize>();
///     let align = core::mem::align_of::<isize>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mem = nstd_os_unix_alloc_allocate_zeroed(layout);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<isize>() == 0);
///     nstd_os_unix_alloc_deallocate(mem);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_unix_alloc_allocate_zeroed(layout: NSTDAllocLayout) -> NSTDAnyMut {
    let ptr = nstd_os_unix_alloc_allocate(layout);
    if !ptr.is_null() {
        nstd_core_mem_zero(ptr.cast(), nstd_core_alloc_layout_size(layout));
    }
    ptr
}

/// Reallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the block of memory to reallocate.
///
/// - `NSTDAllocLayout old_layout` - Describes the previous memory layout.
///
/// - `NSTDAllocLayout new_layout` - Describes the new memory layout to allocate for.
///
/// # Returns
///
/// `NSTDUnixAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// - Behavior is undefined if `new_layout`'s size is zero.
///
/// - `ptr` must point to memory previously allocated with `old_layout`.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::alloc::nstd_core_alloc_layout_new,
///     os::unix::alloc::{
///         nstd_os_unix_alloc_allocate_zeroed, nstd_os_unix_alloc_deallocate,
///         nstd_os_unix_alloc_reallocate, NSTDUnixAllocError::NSTD_UNIX_ALLOC_ERROR_NONE,
///     },
/// };
///
///
/// unsafe {
///     let mut size = core::mem::size_of::<u64>();
///     let mut align = core::mem::align_of::<u64>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mut mem = nstd_os_unix_alloc_allocate_zeroed(layout);
///     assert!(!mem.is_null());
///     size = core::mem::size_of::<u32>();
///     align = core::mem::align_of::<u32>();
///     let new_layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let errc = nstd_os_unix_alloc_reallocate(&mut mem, layout, new_layout);
///     assert!(errc == NSTD_UNIX_ALLOC_ERROR_NONE);
///     assert!(*mem.cast::<u32>() == 0);
///     nstd_os_unix_alloc_deallocate(mem);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_unix_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    old_layout: NSTDAllocLayout,
    new_layout: NSTDAllocLayout,
) -> NSTDUnixAllocError {
    let new_mem = nstd_os_unix_alloc_allocate(new_layout);
    if new_mem.is_null() {
        return NSTDUnixAllocError::NSTD_UNIX_ALLOC_ERROR_OUT_OF_MEMORY;
    }
    let old_size = nstd_core_alloc_layout_size(old_layout);
    let new_size = nstd_core_alloc_layout_size(new_layout);
    nstd_core_mem_copy(new_mem.cast(), (*ptr).cast(), old_size.min(new_size));
    nstd_os_unix_alloc_deallocate(*ptr);
    *ptr = new_mem;
    NSTDUnixAllocError::NSTD_UNIX_ALLOC_ERROR_NONE
}

/// Deallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the block of memory to free.
///
/// # Safety
///
/// Behavior is undefined if `ptr` does not point to memory allocated by
/// `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::alloc::nstd_core_alloc_layout_new,
///     os::unix::alloc::{nstd_os_unix_alloc_allocate, nstd_os_unix_alloc_deallocate},
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<[i16; 8]>();
///     let align = core::mem::align_of::<[i16; 8]>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mem = nstd_os_unix_alloc_allocate(layout);
///     assert!(!mem.is_null());
///     nstd_os_unix_alloc_deallocate(mem);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_unix_alloc_deallocate(ptr: NSTDAnyMut) {
    free(ptr);
}
