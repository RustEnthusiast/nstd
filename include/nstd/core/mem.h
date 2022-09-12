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
/// This function is highly unsafe as it does not know how large either of the memory buffers
/// actually are, which can lead to undefined behavior if either of the buffers' length are less
/// than `num`.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::mem::nstd_core_mem_compare, NSTD_TRUE};
///
/// unsafe {
///     let buf1 = [0u8; 12];
///     let mut buf2 = [u8::MAX; 12];
///     buf2.copy_from_slice(&buf1);
///     assert!(nstd_core_mem_compare(buf1.as_ptr(), buf2.as_ptr(), 12) == NSTD_TRUE);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::nstd_core_mem_search;
///
/// let buffer = b"Hello, world!\0";
/// let ptr = buffer.as_ptr().cast();
/// unsafe {
///     assert!(nstd_core_mem_search(ptr, buffer.len(), b'H') == ptr);
///     assert!(nstd_core_mem_search(ptr, buffer.len(), b' ') == ptr.add(6));
///     assert!(nstd_core_mem_search(ptr, buffer.len(), 0) == ptr.add(13));
/// }
/// ```
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
/// This operation can cause undefined behavior if the caller does not ensure that the memory
/// buffer is at least `size` bytes in size.
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::nstd_core_mem_zero;
///
/// unsafe {
///     let mut buf = [u8::MAX; 10];
///     nstd_core_mem_zero(buf.as_mut_ptr(), 10);
///     assert!(buf == [0u8; 10]);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::nstd_core_mem_fill;
///
/// unsafe {
///     let mut buf = [u8::MAX; 10];
///     nstd_core_mem_fill(buf.as_mut_ptr(), 10, 0);
///     assert!(buf == [0u8; 10]);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::nstd_core_mem_copy;
///
/// unsafe {
///     let buf1 = [0u8; 25];
///     let mut buf2 = [u8::MAX; 25];
///     nstd_core_mem_copy(buf2.as_mut_ptr(), buf1.as_ptr(), 25);
///     assert!(buf1 == buf2);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::nstd_core_mem_swap;
///
/// unsafe {
///     let mut buf1 = [0u8; 25];
///     let mut buf2 = [u8::MAX; 25];
///     nstd_core_mem_swap(buf1.as_mut_ptr(), buf2.as_mut_ptr(), 25);
///     assert!(buf1 == [u8::MAX; 25] && buf2 == [0u8; 25]);
/// }
/// ```
NSTDAPI void nstd_core_mem_swap(NSTDByte *x, NSTDByte *y, NSTDUInt num);

#endif
