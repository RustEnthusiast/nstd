//! Contains mostly unsafe functions for interacting with raw memory.
use crate::{core::def::NSTDByte, NSTDBool, NSTDUInt, NSTD_TRUE};

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_compare(
    buf1: *const NSTDByte,
    buf2: *const NSTDByte,
    num: NSTDUInt,
) -> NSTDBool {
    // If the two pointers point to the same buffer, or `num` is 0, return true.
    if buf1 == buf2 || num == 0 {
        return NSTD_TRUE;
    }
    // Otherwise compare them manually.
    let buf1 = core::slice::from_raw_parts(buf1, num);
    let buf2 = core::slice::from_raw_parts(buf2, num);
    (buf1 == buf2).into()
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_zero(buf: *mut NSTDByte, size: NSTDUInt) {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let mut i = 0;
        while i < size {
            *buf.add(i) = 0;
            i += 1;
        }
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        use core::arch::asm;
        let i = 0usize;
        asm!(include_str!("mem/zero.asm"), buf = in(reg) buf, size = in(reg) size, i = in(reg) i);
    }
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_fill(buf: *mut NSTDByte, size: NSTDUInt, fill: NSTDByte) {
    let mut i = 0;
    while i < size {
        *buf.add(i) = fill;
        i += 1;
    }
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_copy(
    dest: *mut NSTDByte,
    src: *const NSTDByte,
    num: NSTDUInt,
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
/// - `NSTDUInt num` - The number of bytes to copy from `src` to `dest`.
///
/// # Safety
///
/// This function is highly unsafe as it does not know how large either of the memory buffers are,
/// quickly leading to undefined behavior if this function ends up reading or writing past the end
/// of a buffer.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_copy_overlapping(
    dest: *mut NSTDByte,
    src: *const NSTDByte,
    num: NSTDUInt,
) {
    core::ptr::copy(src, dest, num);
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_swap(x: *mut NSTDByte, y: *mut NSTDByte, num: NSTDUInt) {
    core::ptr::swap_nonoverlapping(x, y, num);
}
