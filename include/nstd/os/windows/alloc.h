#ifndef NSTD_OS_WINDOWS_ALLOC_H_INCLUDED
#define NSTD_OS_WINDOWS_ALLOC_H_INCLUDED
#include "../../core/def.h"
#include "../../nstd.h"
NSTDCPPSTART

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
/// See https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc.
NSTDAPI NSTDAny nstd_os_windows_alloc_allocate(NSTDUSize size);

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
/// See https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc.
NSTDAPI NSTDAny nstd_os_windows_alloc_allocate_zeroed(NSTDUSize size);

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
/// See https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_reallocate(NSTDAny *ptr, NSTDUSize new_size);

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
/// See https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree.
NSTDAPI NSTDErrorCode nstd_os_windows_alloc_deallocate(NSTDAny *ptr);

NSTDCPPEND
#endif
