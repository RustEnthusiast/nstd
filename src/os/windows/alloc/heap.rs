//! Process heap management for Windows.
use crate::{
    core::{alloc::NSTDAllocError, result::NSTDResult},
    os::windows::NSTDWindowsHandle,
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_INT_MAX, NSTD_NULL,
};
use nstdapi::nstdapi;
use windows_sys::Win32::System::Memory::{
    GetProcessHeap, HeapAlloc, HeapCreate, HeapDestroy, HeapFree, HeapReAlloc, HeapSize,
    HeapValidate, HEAP_ZERO_MEMORY,
};

/// A handle to a process heap.
#[nstdapi]
pub struct NSTDWindowsHeap {
    /// The private handle.
    handle: NSTDWindowsHandle,
}
impl Drop for NSTDWindowsHeap {
    /// [`NSTDWindowsHeap`] destructor.
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

/// A result type that holds an `NSTDWindowsHeap` as the success variant.
pub type NSTDWindowsHeapResult = NSTDResult<NSTDWindowsHeap, NSTDAllocError>;

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeapResult heap` - A handle to the default heap.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_default,
///     },
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_default() {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 64);
///         assert!(!mem.is_null());
///
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_default() -> NSTDWindowsHeapResult {
    match GetProcessHeap() {
        0 => NSTDResult::Err(NSTDAllocError::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND),
        handle => NSTDResult::Ok(NSTDWindowsHeap { handle }),
    }
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
/// `NSTDWindowsHeapResult heap` - A handle to the new private heap.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_new(0) {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 128);
///         assert!(!mem.is_null());
///
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_new(size: NSTDUInt) -> NSTDWindowsHeapResult {
    match HeapCreate(0, size, 0) {
        0 => NSTDResult::Err(NSTDAllocError::NSTD_ALLOC_ERROR_HEAP_NOT_FOUND),
        handle => NSTDResult::Ok(NSTDWindowsHeap { handle }),
    }
}

/// Returns a raw handle to a heap.
///
/// # Parameters:
///
/// - `const NSTDWindowsHeap *heap` - The heap.
///
/// # Returns
///
/// `NSTDWindowsHandle handle` - A native handle to the heap.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::result::NSTDResult,
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_default, nstd_os_windows_alloc_heap_handle,
///     },
/// };
///
/// if let NSTDResult::Ok(heap) = unsafe { nstd_os_windows_alloc_heap_default() } {
///     let handle = nstd_os_windows_alloc_heap_handle(&heap);
///     assert!(handle != 0);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_os_windows_alloc_heap_handle(heap: &NSTDWindowsHeap) -> NSTDWindowsHandle {
    heap.handle
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
/// use nstd_sys::{
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_default, nstd_os_windows_alloc_heap_size,
///     },
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_default() {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 32);
///         assert!(nstd_os_windows_alloc_heap_size(&heap, mem) == 32);
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_size(heap: &NSTDWindowsHeap, ptr: NSTDAny) -> NSTDUInt {
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
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapvalidate>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_default, nstd_os_windows_alloc_heap_validate,
///     },
///     NSTD_NULL,
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_default() {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 64);
///         let mut errc = nstd_os_windows_alloc_heap_validate(&heap, NSTD_NULL);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///         errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_validate(
    heap: &NSTDWindowsHeap,
    ptr: NSTDAny,
) -> NSTDAllocError {
    match HeapValidate(heap.handle, 0, ptr) {
        0 => NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_HEAP,
        _ => NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
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
///     core::{
///         alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, mem::nstd_core_mem_zero,
///         result::NSTDResult,
///     },
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<isize>();
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_new(0) {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, SIZE);
///         assert!(!mem.is_null());
///
///         nstd_core_mem_zero(mem.cast(), SIZE);
///         assert!(*mem.cast::<isize>() == 0);
///
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_allocate(
    heap: &NSTDWindowsHeap,
    size: NSTDUInt,
) -> NSTDAnyMut {
    match size <= NSTD_INT_MAX {
        true => HeapAlloc(heap.handle, 0, size),
        false => NSTD_NULL,
    }
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
/// use nstd_sys::{
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate_zeroed, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_new(0) {
///         let mut mem = nstd_os_windows_alloc_heap_allocate_zeroed(&heap, SIZE);
///         assert!(!mem.is_null());
///         assert!(*mem.cast::<u64>() == 0);
///
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_allocate_zeroed(
    heap: &NSTDWindowsHeap,
    size: NSTDUInt,
) -> NSTDAnyMut {
    match size <= NSTD_INT_MAX {
        true => HeapAlloc(heap.handle, HEAP_ZERO_MEMORY, size),
        false => NSTD_NULL,
    }
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
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new, nstd_os_windows_alloc_heap_reallocate,
///     },
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_new(0) {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 32);
///         assert!(!mem.is_null());
///
///         let mut errc = nstd_os_windows_alloc_heap_reallocate(&heap, &mut mem, 64);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///
///         errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_reallocate(
    heap: &NSTDWindowsHeap,
    ptr: &mut NSTDAnyMut,
    size: NSTDUInt,
) -> NSTDAllocError {
    if size > NSTD_INT_MAX {
        return NSTDAllocError::NSTD_ALLOC_ERROR_INVALID_LAYOUT;
    }
    match HeapReAlloc(heap.handle, 0, *ptr, size) {
        NSTD_NULL => NSTDAllocError::NSTD_ALLOC_ERROR_OUT_OF_MEMORY,
        new_mem => {
            *ptr = new_mem;
            NSTDAllocError::NSTD_ALLOC_ERROR_NONE
        }
    }
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
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_new(0) {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 300);
///         assert!(!mem.is_null());
///
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_os_windows_alloc_heap_deallocate(
    heap: &NSTDWindowsHeap,
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
/// - `NSTDWindowsHeap heap` - The heap to destroy.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy>.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::{alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE, result::NSTDResult},
///     os::windows::alloc::heap::{
///         nstd_os_windows_alloc_heap_allocate, nstd_os_windows_alloc_heap_deallocate,
///         nstd_os_windows_alloc_heap_new,
///     },
/// };
///
/// unsafe {
///     if let NSTDResult::Ok(heap) = nstd_os_windows_alloc_heap_new(0) {
///         let mut mem = nstd_os_windows_alloc_heap_allocate(&heap, 16);
///         assert!(!mem.is_null());
///
///         let errc = nstd_os_windows_alloc_heap_deallocate(&heap, &mut mem);
///         assert!(errc == NSTD_ALLOC_ERROR_NONE);
///     }
/// }
/// ```
#[inline]
#[nstdapi]
#[allow(
    unused_variables,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value
)]
pub unsafe fn nstd_os_windows_alloc_heap_free(heap: NSTDWindowsHeap) {}
