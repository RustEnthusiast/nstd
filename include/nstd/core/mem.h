#ifndef NSTD_CORE_MEM_H_INCLUDED
#define NSTD_CORE_MEM_H_INCLUDED
#include "../nstd.h"
#include "def.h"
NSTDCPPSTART

/// Compares two memory buffers of `num` bytes.
///
/// # Parameters:
///
/// - `const NSTDByte *buf1` - A pointer to the first memory buffer.
///
/// - `const NSTDByte *buf2` - A pointer to the second memory buffer.
///
/// - `NSTDUSize num` - The number of bytes to compare.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the memory buffers carry the same data.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers
/// actually are, which can lead to undefined behaviour if either of the buffers' length are less
/// than `num`.
NSTDAPI NSTDBool nstd_core_mem_compare(const NSTDByte *buf1, const NSTDByte *buf2, NSTDUSize num);

/// Copies `num` bytes from `src` to `dest`.
///
/// # Parameters:
///
/// - `NSTDByte *dest` - A pointer to the memory buffer to copy `src`'s bytes to.
///
/// - `const NSTDByte *src` - A pointer to the memory buffer to copy from.
///
/// - `NSTDUSize num` - The number of bytes to copy from `src` to `dest`.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers are,
/// quickly leading to undefined behaviour if this function ends up reading or writing past the end
/// of a buffer.
NSTDAPI void nstd_core_mem_copy(NSTDByte *dest, const NSTDByte *src, NSTDUSize num);

/// Copies `num` bytes from `src` to `dest`. Unlike `nstd_core_mem_copy` this operation can be used
/// when the two memory buffers overlap.
///
/// # Parameters:
///
/// - `NSTDByte *dest` - A pointer to the memory buffer to copy `src`'s bytes to.
///
/// - `const NSTDByte *src` - A pointer to the memory buffer to copy from.
///
/// - `NSTDUSize num` - The number of bytes to copy from `src` to `dest`.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers are,
/// quickly leading to undefined behaviour if this function ends up reading or writing past the end
/// of a buffer.
NSTDAPI void nstd_core_mem_copy_overlapping(NSTDByte *dest, const NSTDByte *src, NSTDUSize num);

NSTDCPPEND
#endif
