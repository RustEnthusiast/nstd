//! Low level memory allocation for Windows.
use crate::{
    core::{
        def::{NSTDAny, NSTDErrorCode},
        NSTD_CORE_NULL,
    },
    NSTDUSize,
};
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc, HEAP_FLAGS, HEAP_ZERO_MEMORY,
};

/// Allocates a new block of memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the block of memory, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate(size: NSTDUSize) -> NSTDAny {
    HeapAlloc(GetProcessHeap(), HEAP_FLAGS::default(), size)
}

/// Allocates a new block of zero-initialized memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUSize size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the block of memory, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_allocate_zeroed(size: NSTDUSize) -> NSTDAny {
    HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, size)
}

/// Reallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and 0 will be
/// returned. If this is not the case and allocation fails, the pointer will remain untouched and a
/// value of nonzero is returned.
///
/// # Parameters:
///
/// - `NSTDAny *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUSize new_size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_reallocate(
    ptr: &mut NSTDAny,
    new_size: NSTDUSize,
) -> NSTDErrorCode {
    let new_mem = HeapReAlloc(GetProcessHeap(), HEAP_FLAGS::default(), *ptr, new_size);
    if !new_mem.is_null() {
        *ptr = new_mem;
        return 0;
    }
    1
}

/// Deallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAny *ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_deallocate(ptr: &mut NSTDAny) -> NSTDErrorCode {
    let memptr = *ptr;
    *ptr = NSTD_CORE_NULL;
    (HeapFree(GetProcessHeap(), HEAP_FLAGS::default(), memptr) == 0) as NSTDErrorCode
}
