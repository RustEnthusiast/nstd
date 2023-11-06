#ifndef NSTD_CORE_MEM_H
#define NSTD_CORE_MEM_H
#include "../nstd.h"
#include "def.h"

/// Compares two memory buffers of `num` bytes.
///
/// # Parameters:
///
/// - `const NSTDByte *buf1` - A pointer to the first memory buffer.
///
/// - `const NSTDByte *buf2` - A pointer to the second memory buffer.
///
/// - `NSTDUInt num` - The number of bytes to compare.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the memory buffers carry the same data.
///
/// # Safety
///
/// - This function is highly unsafe as it does not know how large either of the memory buffers
/// actually are, which can lead to undefined behavior if either of the buffers' length are less
/// than `num`.
///
/// - `buf1` and `buf2` must be non-null.
NSTDAPI NSTDBool nstd_core_mem_compare(const NSTDByte *buf1, const NSTDByte *buf2, NSTDUInt num);

/// Iterates through each byte in a raw memory buffer until `delim` is reached, returning a pointer
/// to the delimiter byte if it is found.
///
/// # Parameters:
///
/// - `const NSTDByte *buf` - The memory buffer to search.
///
/// - `NSTDUInt size` - The number of bytes to search.
///
/// - `NSTDByte delim` - The delimiter byte.
///
/// # Returns
///
/// `const NSTDByte *delim_ptr` - A pointer to the delimiter byte, or null if it was not found.
///
/// # Safety
///
/// This operation makes access to raw pointer data, leading to undefined behavior if `buf`'s
/// data is invalid.
NSTDAPI const NSTDByte *nstd_core_mem_search(const NSTDByte *buf, NSTDUInt size, NSTDByte delim);

/// Zeros out a memory buffer.
///
/// # Parameters:
///
/// - `NSTDByte *buf` - A pointer to the first byte in the memory buffer.
///
/// - `NSTDUInt size` - The number of bytes to set to 0.
///
/// # Safety
///
/// The caller must ensure that `buf` is valid for reads of `size` contiguous bytes.
NSTDAPI void nstd_core_mem_zero(NSTDByte *buf, NSTDUInt size);

/// Fills the memory buffer `buf` with byte `fill`.
///
/// # Parameters:
///
/// - `NSTDByte *buf` - The memory buffer to fill.
///
/// - `NSTDUInt size` - The size of the memory buffer.
///
/// - `NSTDByte fill` - The byte value to fill the memory buffer with.
///
/// # Safety
///
/// This operation can cause undefined behavior if the caller does not ensure that the memory
/// buffer is at least `size` bytes in size.
NSTDAPI void nstd_core_mem_fill(NSTDByte *buf, NSTDUInt size, NSTDByte fill);

/// Copies `num` bytes from `src` to `dest`.
///
/// # Parameters:
///
/// - `NSTDByte *dest` - A pointer to the memory buffer to copy `src`'s bytes to.
///
/// - `const NSTDByte *src` - A pointer to the memory buffer to copy from.
///
/// - `NSTDUInt num` - The number of bytes to copy from `src` to `dest`.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers are,
/// quickly leading to undefined behavior if this function ends up reading or writing past the end
/// of a buffer.
NSTDAPI void nstd_core_mem_copy(NSTDByte *dest, const NSTDByte *src, NSTDUInt num);

/// Copies `num` bytes from `src` to `dest`. Unlike `nstd_core_mem_copy` this operation can be used
/// when the two memory buffers overlap.
///
/// # Parameters:
///
/// - `NSTDByte *dest` - A pointer to the memory buffer to copy `src`'s bytes to.
///
/// - `const NSTDByte *src` - A pointer to the memory buffer to copy from.
///
/// - `NSTDUInt num` - The number of bytes to copy from `src` to `dest`.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers are,
/// quickly leading to undefined behavior if this function ends up reading or writing past the end
/// of a buffer.
NSTDAPI void nstd_core_mem_copy_overlapping(NSTDByte *dest, const NSTDByte *src, NSTDUInt num);

/// Swaps `num` bytes between the memory buffers `x` and `y`.
///
/// # Parameters:
///
/// - `NSTDByte *x` - A pointer to the first memory buffer.
///
/// - `NSTDByte *y` - A pointer to the second memory buffer.
///
/// - `NSTDUInt num` - The number of bytes to swap.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers are,
/// quickly leading to undefined behavior if this function ends up reading or writing past the end
/// of a buffer.
NSTDAPI void nstd_core_mem_swap(NSTDByte *x, NSTDByte *y, NSTDUInt num);

/// Creates a new dangling pointer to some immutable memory. The pointer is guaranteed to have valid
/// alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAny dangling` - The new dangling raw pointer.
NSTDAPI NSTDAny nstd_core_mem_dangling(void);

/// Creates a new dangling pointer to some mutable memory. The pointer is guaranteed to have valid
/// alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAnyMut dangling` - The new dangling raw pointer.
NSTDAPI NSTDAnyMut nstd_core_mem_dangling_mut(void);

/// Returns a pointer that is properly aligned to `align` based on the offset `ptr`.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - The pointer to align.
///
/// - `NSTDUInt align` - The alignment requirement. This must be a power of two.
///
/// # Returns
///
/// `NSTDAny aligned` - The properly aligned pointer.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two or overflow occurs.
///
/// # Safety
///
/// Both `ptr` and the resulting pointer must be either in bounds or one byte past the end of the
/// same allocated object.
NSTDAPI NSTDAny nstd_core_mem_align(NSTDAny ptr, NSTDUInt align);

/// Returns a pointer that is properly aligned to `align` based on the offset `ptr`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - The pointer to align.
///
/// - `NSTDUInt align` - The alignment requirement. This must be a power of two.
///
/// # Returns
///
/// `NSTDAnyMut aligned` - The properly aligned pointer.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two or overflow occurs.
///
/// # Safety
///
/// Both `ptr` and the resulting pointer must be either in bounds or one byte past the end of the
/// same allocated object.
NSTDAPI NSTDAnyMut nstd_core_mem_align_mut(NSTDAnyMut ptr, NSTDUInt align);

/// Checks if `ptr` is aligned to `align`.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - The pointer to check.
///
/// - `NSTDUInt align` - The alignment to check for. This must be a power of two.
///
/// # Returns
///
/// `NSTDBool is_aligned` - `NSTD_TRUE` if the pointer is aligned to `align`.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two.
NSTDAPI NSTDBool nstd_core_mem_is_aligned(NSTDAny ptr, NSTDUInt align);

#endif
