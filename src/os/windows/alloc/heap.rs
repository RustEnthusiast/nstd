//! Process heap management for Windows.
use crate::{alloc::NSTDAllocError, NSTDAnyMut, NSTDInt, NSTDUInt, NSTD_NULL};
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapCreate, HeapDestroy, HeapFree, HeapReAlloc, HEAP_ZERO_MEMORY,
};

/// A handle to a process heap.
#[repr(C)]
pub struct NSTDWindowsHeapHandle {
    /// The private handle.
    handle: NSTDInt,
}

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeapHandle heap` - A handle to the default heap.
///
/// # Panics
///
/// Panics if getting a handle to the heap fails.
///
/// # Safety
///
/// - See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_default,
///     },
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_default();
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 64);
///     assert!(!mem.is_null());
///
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_default() -> NSTDWindowsHeapHandle {
    let handle = GetProcessHeap();
    assert!(handle != 0);
    NSTDWindowsHeapHandle { handle }
}

/// Creates a new private heap for the process.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The initial size of the heap, in bytes. If this parameter is 0,
/// the heap gets one page.
///
/// # Returns
///
/// `NSTDWindowsHeapHandle heap` - A handle to the new private heap.
///
/// # Panics
///
/// Panics if creating a new heap fails.
///
/// # Safety
///
/// - See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate>.
///
/// - This operation can cause undefined behavior if it panics into non-Rust code.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_free, nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 128);
///     assert!(!mem.is_null());
///
///     let mut errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_free(heap);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_new(size: NSTDUInt) -> NSTDWindowsHeapHandle {
    let handle = HeapCreate(0, size, 0);
    assert!(handle != 0);
    NSTDWindowsHeapHandle { handle }
}

/// Allocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeapHandle *heap` - A handle to the heap to allocate on.
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the new block of memory on the heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     core::mem::nstd_core_mem_zero,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_free, nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<isize>();
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, SIZE);
///     assert!(!mem.is_null());
///
///     nstd_core_mem_zero(mem.cast(), SIZE);
///     assert!(*mem.cast::<isize>() == 0);
///
///     let mut errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_free(heap);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate(
    heap: &NSTDWindowsHeapHandle,
    size: NSTDUInt,
) -> NSTDAnyMut {
    HeapAlloc(heap.handle, 0, size)
}

/// Allocates a zero-initialized block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeapHandle *heap` - A handle to the heap to allocate on.
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the new block of memory on the heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate_zeroed, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_free, nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate_zeroed(&heap, SIZE);
///     assert!(!mem.is_null());
///     assert!(*mem.cast::<u64>() == 0);
///
///     let mut errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_free(heap);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate_zeroed(
    heap: &NSTDWindowsHeapHandle,
    size: NSTDUInt,
) -> NSTDAnyMut {
    HeapAlloc(heap.handle, HEAP_ZERO_MEMORY, size)
}

/// Reallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeapHandle *heap` - A handle to the heap to reallocate on.
///
/// - `NSTDAnyMut *ptr` - A pointer to the memory to reallocate.
///
/// - `NSTDUInt size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_free, nstd_os_windows_alloc_heap_new,
///         nstd_os_windows_alloc_heap_reallocate,
///     },
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 32);
///     assert!(!mem.is_null());
///
///     let mut errc = nstd_os_windows_alloc_heap_reallocate(&heap, &mut mem, 64);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_free(heap);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_reallocate(
    heap: &NSTDWindowsHeapHandle,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDAllocError {
    let new_mem = HeapReAlloc(heap.handle, 0, *ptr, size);
    if !new_mem.is_null() {
        *ptr = new_mem;
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY
}

/// Deallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeapHandle *heap` - A handle to the heap to deallocate memory from.
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_free, nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 300);
///     assert!(!mem.is_null());
///
///     let mut errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_free(heap);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_deallocate(
    heap: &NSTDWindowsHeapHandle,
    ptr: &mut NSTDAnyMut,
) -> NSTDAllocError {
    if HeapFree(heap.handle, 0, *ptr) != 0 {
        *ptr = NSTD_NULL;
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_MEMORY_NOT_FOUND
}

/// Destroys a private heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle heap` - The heap to destroy.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_free, nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 16);
///     assert!(!mem.is_null());
///
///     let mut errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_free(heap);
///     assert!(errc == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_free(
    heap: NSTDWindowsHeapHandle,
) -> NSTDAllocError {
    if HeapDestroy(heap.handle) != 0 {
        return NSTDAllocError::NSTD_ALLOC_ERROR_NONE;
    }
    NSTDAllocError::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND
}
