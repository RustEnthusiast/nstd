//! Low level memory allocation.
#[cfg(not(target_os = "windows"))]
extern crate alloc;
use crate::{core::def::NSTDErrorCode, NSTDAny, NSTDUSize};

/// Allocates a block of memory on the heap.
/// The number of bytes to be allocated is specified by `size`.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `size` is zero.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate(size: NSTDUSize) -> NSTDAny {
    #[cfg(not(target_os = "windows"))]
    {
        use alloc::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc::alloc::alloc(layout).cast()
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_allocate(size)
    }
}

/// Allocates a block of zero-initialized memory on the heap.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate on the heap.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the allocated memory, null on error.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `size` is zero.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_alloc_allocate_zeroed(size: NSTDUSize) -> NSTDAny {
    #[cfg(not(target_os = "windows"))]
    {
        use alloc::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc::alloc::alloc_zeroed(layout).cast()
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_allocate_zeroed(size)
    }
}

/// Reallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and 0 will be
/// returned. If this is not the case and allocation fails, the pointer will remain untouched and a
/// value of nonzero is returned.
///
/// # Parameters:
///
/// - `NSTDAny *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUSize size` - The number of bytes currently allocated.
///
/// - `NSTDUSize new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `ptr` is not a value returned by
/// `nstd_alloc_allocate[_zeroed]`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(target_os = "windows", allow(unused_variables))]
pub unsafe extern "C" fn nstd_alloc_reallocate(
    ptr: &mut NSTDAny,
    size: NSTDUSize,
    new_size: NSTDUSize,
) -> NSTDErrorCode {
    #[cfg(not(target_os = "windows"))]
    {
        use alloc::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        let new_mem = alloc::alloc::realloc((*ptr).cast(), layout, new_size);
        if !new_mem.is_null() {
            *ptr = new_mem.cast();
            return 0;
        }
        1
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_reallocate(ptr, new_size)
    }
}

/// Deallocates a block of memory previously allocated by `nstd_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAny *ptr` - A pointer to the allocated memory, once freed the pointer is set to null.
///
/// - `NSTDUSize size` - The number of bytes to free.
///
/// # Safety
///
/// This operation is unsafe because the behaviour is undefined if `ptr` is not a value returned by
/// `nstd_alloc_allocate[_zeroed]`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[cfg_attr(target_os = "windows", allow(unused_variables))]
pub unsafe extern "C" fn nstd_alloc_deallocate(ptr: &mut NSTDAny, size: NSTDUSize) {
    #[cfg(not(target_os = "windows"))]
    {
        use crate::NSTD_NULL;
        use alloc::alloc::Layout;
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc::alloc::dealloc((*ptr).cast(), layout);
        *ptr = NSTD_NULL;
    }
    #[cfg(target_os = "windows")]
    {
        crate::os::windows::alloc::nstd_os_windows_alloc_deallocate(ptr);
    }
}
