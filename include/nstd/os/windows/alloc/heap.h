#ifndef NSTD_OS_WINDOWS_ALLOC_HEAP_H
#define NSTD_OS_WINDOWS_ALLOC_HEAP_H
#include "../../../alloc.h"
#include "../../../nstd.h"

/// A handle to a process heap.
typedef NSTDInt NSTDWindowsHeapHandle;

/// Returns a handle to the default heap of the current process.
///
/// # Returns
///
/// `NSTDWindowsHeapHandle heap` - A handle to the default heap, null on error.
///
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>.
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
NSTDAPI NSTDWindowsHeapHandle nstd_os_windows_alloc_heap_default();

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
/// # Safety
///
/// See <https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate>.
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
NSTDAPI NSTDWindowsHeapHandle nstd_os_windows_alloc_heap_new(NSTDUInt size);

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
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_heap_allocate(const NSTDWindowsHeapHandle *heap,
NSTDUInt size);

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
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_heap_allocate_zeroed(const NSTDWindowsHeapHandle *heap,
NSTDUInt size);

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
NSTDAPI NSTDAllocError nstd_os_windows_alloc_heap_reallocate(const NSTDWindowsHeapHandle *heap,
NSTDAnyMut *ptr, NSTDUInt size);

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
NSTDAPI NSTDAllocError nstd_os_windows_alloc_heap_deallocate(const NSTDWindowsHeapHandle *heap,
NSTDAnyMut *ptr);

/// Destroys a private heap.
///
/// # Parameters:
///
/// - `NSTDWindowsHeapHandle *heap` - The heap to destroy.
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
NSTDAPI NSTDAllocError nstd_os_windows_alloc_heap_free(NSTDWindowsHeapHandle *heap);

#endif
