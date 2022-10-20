//! Process heap management for Windows.
use crate::{
    os::windows::alloc::NSTDWindowsAllocError, NSTDAny, NSTDAnyMut, NSTDInt, NSTDUInt, NSTD_NULL,
};
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapCreate, HeapDestroy, HeapFree, HeapReAlloc, HeapSize,
    HeapValidate, HEAP_ZERO_MEMORY,
};

/// A handle to a process heap.
#[repr(C)]
pub struct NSTDWindowsHeap {
    /// The private handle.
    handle: NSTDInt,
}
impl Drop for NSTDWindowsHeap {
    /// [NSTDWindowsHeap] destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `self.handle` may have been acquired through `GetProcessHeap`.
        unsafe {
            if self.handle != GetProcessHeap() {
                HeapDestroy(self.handle);
            }
        }
    }
}

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeap heap` - A handle to the default heap.
///
/// # Panics
///
/// Panics if getting a handle to the heap fails.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_default,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_default();
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 64);
///     assert!(!mem.is_null());
///
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_default() -> NSTDWindowsHeap {
    let handle = GetProcessHeap();
    assert!(handle != 0);
    NSTDWindowsHeap { handle }
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
/// `NSTDWindowsHeap heap` - A handle to the new private heap.
///
/// # Panics
///
/// Panics if creating a new heap fails.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 128);
///     assert!(!mem.is_null());
///
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_new(size: NSTDUInt) -> NSTDWindowsHeap {
    let handle = HeapCreate(0, size, 0);
    assert!(handle != 0);
    NSTDWindowsHeap { handle }
}

/// Returns the size of a memory block previously allocated by an `NSTDWindowsHeap`.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap.
///
/// - `NSTDAny ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDUInt size` - The number of bytes allocated at the memory block pointed to by `ptr`.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsize>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_default, nstd_os_windows_alloc_heap_size,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_default();
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 32);
///     assert!(nstd_os_windows_alloc_heap_size(&heap, mem) == 32);
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_size(
    heap: &NSTDWindowsHeap,
    ptr: NSTDAny,
) -> NSTDUInt {
    HeapSize(heap.handle, 0, ptr)
}

/// Validates a heap or memory block allocated on a heap.
///
/// If `ptr` is null, the function will attempt to validate the entire heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to validate.
///
/// - `NSTDAny ptr` - A pointer to the block of memory to validate. Pass null to validate the
/// entire heap.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapvalidate>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     os::windows::alloc::{
///         heap::{
///             nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///             nstd_os_windows_alloc_heap_default, nstd_os_windows_alloc_heap_validate,
///         },
///         NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
///     },
///     NSTD_NULL,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_default();
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 64);
///     let mut errc = nstd_os_windows_alloc_heap_validate(&heap, NSTD_NULL);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
///     errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_validate(
    heap: &NSTDWindowsHeap,
    ptr: NSTDAny,
) -> NSTDWindowsAllocError {
    match HeapValidate(heap.handle, 0, ptr) {
        0 => NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_INVALID_HEAP,
        _ => NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
    }
}

/// Allocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to allocate on.
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
///     core::mem::nstd_core_mem_zero,
///     os::windows::alloc::{
///         heap::{
///             nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///             nstd_os_windows_alloc_heap_new,
///         },
///         NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
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
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate(
    heap: &NSTDWindowsHeap,
    size: NSTDUInt,
) -> NSTDAnyMut {
    HeapAlloc(heap.handle, 0, size)
}

/// Allocates a zero-initialized block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to allocate on.
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
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate_zeroed, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
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
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_allocate_zeroed(
    heap: &NSTDWindowsHeap,
    size: NSTDUInt,
) -> NSTDAnyMut {
    HeapAlloc(heap.handle, HEAP_ZERO_MEMORY, size)
}

/// Reallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to reallocate on.
///
/// - `NSTDAnyMut *ptr` - A pointer to the memory to reallocate.
///
/// - `NSTDUInt size` - The number of bytes to reallocate.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new, nstd_os_windows_alloc_heap_reallocate,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 32);
///     assert!(!mem.is_null());
///
///     let mut errc = nstd_os_windows_alloc_heap_reallocate(&heap, &mut mem, 64);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
///
///     errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_reallocate(
    heap: &NSTDWindowsHeap,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDWindowsAllocError {
    let new_mem = HeapReAlloc(heap.handle, 0, *ptr, size);
    if !new_mem.is_null() {
        *ptr = new_mem;
        return NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE;
    }
    NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_OUT_OF_MEMORY
}

/// Deallocates a block of memory on a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - A handle to the heap to deallocate memory from.
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// # Returns
///
/// `NSTDWindowsAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 300);
///     assert!(!mem.is_null());
///
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_deallocate(
    heap: &NSTDWindowsHeap,
    ptr: &mut NSTDAnyMut,
) -> NSTDWindowsAllocError {
    if HeapFree(heap.handle, 0, *ptr) != 0 {
        *ptr = NSTD_NULL;
        return NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE;
    }
    NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_MEMORY_NOT_FOUND
}

/// Destroys a private heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeap heap` - The heap to destroy.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy>.
///
/// # Example
///
/// ```
/// use nstd_sys::os::windows::alloc::{
///     heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
///     NSTDWindowsAllocError::NSTD_WINDOWS_ALLOC_ERROR_NONE,
/// };
///
/// unsafe {
///     let heap = nstd_os_windows_alloc_heap_new(0);
///
///     let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 16);
///     assert!(!mem.is_null());
///
///     let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///     assert!(errc == NSTD_WINDOWS_ALLOC_ERROR_NONE);
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub unsafe extern "C" fn nstd_os_windows_alloc_heap_free(heap: NSTDWindowsHeap) {}
