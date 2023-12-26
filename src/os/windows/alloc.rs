//! Low level memory allocation for Windows.
pub mod heap;
use crate::{
    core::{
        alloc::{nstd_core_alloc_layout_align, nstd_core_alloc_layout_size, NSTDAllocLayout},
        mem::{nstd_core_mem_copy, nstd_core_mem_zero},
    },
    NSTDAnyMut,
};
use libc::{aligned_free, aligned_malloc};
use nstdapi::nstdapi;

/// Describes an error returned from allocation functions for Windows.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDWindowsAllocError {
    /// No error occurred.
    NSTD_WINDOWS_ALLOC_ERROR_NONE,
    /// Allocating or reallocating failed.
    NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY,
    /// Deallocating memory failed.
    NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND,
    /// Getting a handle to a heap failed.
    NSTD_WINDOWS_ALLOC_ERROR_HEAP_NOT_FOUND,
    /// A heap is invalid.
    NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP,
    /// An allocation function received input parameters that resulted in an invalid memory layout.
    NSTD_WINDOWS_ALLOC_ERROR_INVALID_LAYOUT,
}

/// Allocates a new block of memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
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
///     core::{alloc::nstd_core_alloc_layout_new, mem::nstd_core_mem_zero},
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_deallocate,
///         NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
///     },
/// };
///
///
/// unsafe {
///     let size = core::mem::size_of::<[u32; 5]>();
///     let align = core::mem::align_of::<[u32; 5]>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let buf = nstd_os_windows_alloc_allocate(layout);
///     assert!(!buf.is_null());
///
///     nstd_core_mem_zero(buf.cast(), size);
///     assert!(*buf.cast::<[u32; 5]>() == [0u32; 5]);
///
///     nstd_os_windows_alloc_deallocate(buf);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_allocate(layout: NSTDAllocLayout) -> NSTDAnyMut {
    let size = nstd_core_alloc_layout_size(layout);
    let align = nstd_core_alloc_layout_align(layout);
    aligned_malloc(size, align)
}

/// Allocates a new block of zero-initialized memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDAllocLayout layout` - Describes the memory layout to allocate for.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
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
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate_zeroed, nstd_os_windows_alloc_deallocate,
///         NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
///     },
/// };
///
///
/// unsafe {
///     let size = core::mem::size_of::<[i64; 5]>();
///     let align = core::mem::align_of::<[i64; 5]>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let buf = nstd_os_windows_alloc_allocate_zeroed(layout);
///     assert!(!buf.is_null());
///     assert!(*buf.cast::<[i64; 5]>() == [0i64; 5]);
///     nstd_os_windows_alloc_deallocate(buf);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_allocate_zeroed(layout: NSTDAllocLayout) -> NSTDAnyMut {
    let ptr = nstd_os_windows_alloc_allocate(layout);
    if !ptr.is_null() {
        nstd_core_mem_zero(ptr.cast(), nstd_core_alloc_layout_size(layout));
    }
    ptr
}

/// Reallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDAllocLayout old_layout` - Describes the previous memory layout.
///
/// - `NSTDAllocLayout new_layout` - Describes the new memory layout to allocate for.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
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
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate_zeroed, nstd_os_windows_alloc_deallocate,
///         nstd_os_windows_alloc_reallocate, NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
///     },
/// };
///
///
/// unsafe {
///     let mut size = core::mem::size_of::<i128>();
///     let mut align = core::mem::align_of::<i128>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let mut mem = nstd_os_windows_alloc_allocate_zeroed(layout);
///     assert!(!mem.is_null());
///     size = core::mem::size_of::<i64>();
///     align = core::mem::align_of::<i64>();
///     let new_layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let errc = nstd_os_windows_alloc_reallocate(&mut mem, layout, new_layout);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
///     assert!(*mem.cast::<i64>() == 0);
///     nstd_os_windows_alloc_deallocate(mem);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAnyMut,
    old_layout: NSTDAllocLayout,
    new_layout: NSTDAllocLayout,
) -> NSTDWindowsAllocError {
    let new_mem = nstd_os_windows_alloc_allocate(new_layout);
    if new_mem.is_null() {
        return NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY;
    }
    let old_size = nstd_core_alloc_layout_size(old_layout);
    let new_size = nstd_core_alloc_layout_size(new_layout);
    nstd_core_mem_copy(new_mem.cast(), (*ptr).cast(), old_size.min(new_size));
    nstd_os_windows_alloc_deallocate(*ptr);
    *ptr = new_mem;
    NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE
}

/// Deallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the allocated memory.
///
/// # Safety
///
/// Behavior is undefined if `ptr` does not point to memory allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::alloc::nstd_core_alloc_layout_new,
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_deallocate,
///         NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
///     },
/// };
///
/// unsafe {
///     let size = core::mem::size_of::<[f64; 4]>();
///     let align = core::mem::align_of::<[f64; 4]>();
///     let layout = nstd_core_alloc_layout_new(size, align).unwrap();
///     let buf = nstd_os_windows_alloc_allocate(layout);
///     assert!(!buf.is_null());
///     nstd_os_windows_alloc_deallocate(buf);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_deallocate(ptr: NSTDAnyMut) {
    aligned_free(ptr);
}
