#ifndef NSTD_CORE_MEM_H_INCLUDED
#define NSTD_CORE_MEM_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

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

NSTDCPPEND
#endif
