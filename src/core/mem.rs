//! Contains mostly unsafe functions for interacting with raw memory.
use crate::core::def::{NSTDByte, NSTDUSize};

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_copy(
    dest: *mut NSTDByte,
    src: *const NSTDByte,
    num: NSTDUSize,
) {
    core::ptr::copy_nonoverlapping(src, dest, num);
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_copy_overlapping(
    dest: *mut NSTDByte,
    src: *const NSTDByte,
    num: NSTDUSize,
) {
    core::ptr::copy(src, dest, num);
}
