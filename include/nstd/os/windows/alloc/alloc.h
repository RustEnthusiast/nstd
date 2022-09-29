#ifndef NSTD_OS_WINDOWS_ALLOC_ALLOC_H
#define NSTD_OS_WINDOWS_ALLOC_ALLOC_H
#include "../../../alloc.h"
#include "../../../nstd.h"

/// Allocates a new block of memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
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
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_deallocate,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<[u32; 5]>();
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate(SIZE);
///     assert!(!buf.is_null());
///
///     nstd_core_mem_zero(buf.cast(), SIZE);
///     assert!(*buf.cast::<[u32; 5]>() == [0u32; 5]);
///
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_allocate(NSTDUInt size);

/// Allocates a new block of zero-initialized memory on the current process' heap.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the block of memory, null on error.
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
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate_zeroed, nstd_os_windows_alloc_deallocate,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<[i64; 5]>();
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate_zeroed(SIZE);
///     assert!(!buf.is_null());
///     assert!(*buf.cast::<[i64; 5]>() == [0i64; 5]);
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
NSTDAPI NSTDAnyMut nstd_os_windows_alloc_allocate_zeroed(NSTDUInt size);

/// Reallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// If everything goes right, the pointer will point to the new memory location and 0 will be
/// returned. If this is not the case and allocation fails, the pointer will remain untouched and a
/// value of nonzero is returned.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the allocated memory.
///
/// - `NSTDUInt new_size` - The number of bytes to reallocate.
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
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate_zeroed, nstd_os_windows_alloc_deallocate,
///         nstd_os_windows_alloc_reallocate,
///     },
/// };
///
/// const SIZE: usize = core::mem::size_of::<[i16; 10]>();
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate_zeroed(SIZE);
///     assert!(!buf.is_null());
///     assert!(*buf.cast::<[i16; 10]>() == [0i16; 10]);
///
///     assert!(nstd_os_windows_alloc_reallocate(&mut buf, SIZE / 2) == NSTD_ALLOC_ERROR_NONE);
///     assert!(*buf.cast::<[i16; 5]>() == [0i16; 5]);
///
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
NSTDAPI NSTDAllocError nstd_os_windows_alloc_reallocate(NSTDAnyMut *ptr, NSTDUInt new_size);

/// Deallocates a block of memory previously allocated by
/// `nstd_os_windows_alloc_allocate[_zeroed]`.
///
/// # Parameters:
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
///     os::windows::alloc::{
///         nstd_os_windows_alloc_allocate, nstd_os_windows_alloc_deallocate,
///     },
/// };
///
/// unsafe {
///     let mut buf = nstd_os_windows_alloc_allocate(128);
///     assert!(!buf.is_null());
///     assert!(nstd_os_windows_alloc_deallocate(&mut buf) == NSTD_ALLOC_ERROR_NONE);
/// }
/// ```
NSTDAPI NSTDAllocError nstd_os_windows_alloc_deallocate(NSTDAnyMut *ptr);

#endif
