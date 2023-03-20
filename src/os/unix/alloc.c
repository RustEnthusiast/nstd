#include <nstd/nstd.h>
#include <nstd/os/unix/alloc.h>
#include <stdlib.h>

/// Allocates a block of memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate for the new block of memory.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/malloc.3.html>.
NSTDAPI NSTDAnyMut nstd_os_unix_alloc_allocate(const NSTDUInt size) {
    return malloc(size);
}

/// Allocates a block of zero initialized memory on the heap, returning a pointer to it.
///
/// # Parameters:
///
/// - `NSTDUInt size` - The number of bytes to allocate for the new block of memory.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the newly allocated block of memory, or null on error.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/calloc.3p.html>.
NSTDAPI NSTDAnyMut nstd_os_unix_alloc_allocate_zeroed(const NSTDUInt size) {
    return calloc(size, 1);
}

/// Reallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the block of memory to reallocate.
///
/// - `NSTDUInt new_size` - The new size of the memory block.
///
/// # Returns
///
/// `NSTDUnixAllocError errc` - The allocation operation error code.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/realloc.3p.html>.
NSTDAPI NSTDUnixAllocError
nstd_os_unix_alloc_reallocate(NSTDAnyMut *const ptr, const NSTDUInt new_size) {
    const NSTDAnyMut new_mem = realloc(*ptr, new_size);
    if (new_mem) {
        *ptr = new_mem;
        return NSTD_UNIX_ALLOC_ERROR_NONE;
    }
    return NSTD_UNIX_ALLOC_ERROR_OUT_OF_MEMORY;
}

/// Deallocates a block of memory previously allocated by `nstd_os_unix_alloc_allocate[_zeroed]`.
///
/// # Parameters:
///
/// - `NSTDAnyMut *ptr` - A pointer to the block of memory to free.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/free.3p.html>.
NSTDAPI void nstd_os_unix_alloc_deallocate(NSTDAnyMut *const ptr) {
    free(*ptr);
    *ptr = NSTD_NULL;
}
