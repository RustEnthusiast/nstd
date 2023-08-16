//! Contains mostly unsafe functions for interacting with raw memory.
use crate::{core::def::NSTDByte, NSTDBool, NSTDUInt};
use cfg_if::cfg_if;
use nstdapi::nstdapi;

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
/// # Panics
///
/// This operation may panic if `num` is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// - This function is highly unsafe as it does not know how large either of the memory buffers
/// actually are, which can lead to undefined behavior if either of the buffers' length are less
/// than `num`.
///
/// - `buf1` and `buf2` must be non-null.
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::mem::{nstd_core_mem_compare, nstd_core_mem_copy},
///     NSTD_TRUE,
/// };
///
/// let buf1 = [0u32; 12];
/// let mut buf2 = [u32::MAX; 12];
///
/// let num = core::mem::size_of::<[u32; 12]>();
/// let ptr1 = buf1.as_ptr().cast();
/// let ptr2 = buf2.as_mut_ptr().cast();
///
/// unsafe {
///     nstd_core_mem_copy(ptr2, ptr1, num);
///     assert!(nstd_core_mem_compare(ptr1, ptr2, num) == NSTD_TRUE);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_mem_compare(
    buf1: *const NSTDByte,
    buf2: *const NSTDByte,
    num: NSTDUInt,
) -> NSTDBool {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3"
            ),
            feature = "libc"
        ))] {
            libc::memcmp(buf1.cast(), buf2.cast(), num) == 0
        } else {
            use crate::{NSTD_INT_MAX, NSTD_TRUE};
            // If the two pointers point to the same buffer, or `num` is 0, return true.
            if buf1 == buf2 || num == 0 {
                return NSTD_TRUE;
            }
            // Check if `num` exceeds `isize::MAX`.
            assert!(num <= NSTD_INT_MAX as _);
            // Otherwise compare them manually.
            let buf1 = core::slice::from_raw_parts(buf1, num);
            let buf2 = core::slice::from_raw_parts(buf2, num);
            buf1 == buf2
        }
    }
}

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
/// # Panics
///
/// This operation may panic if `size` is greater than `NSTDInt`'s max value.
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
#[nstdapi]
pub unsafe fn nstd_core_mem_search(
    buf: *const NSTDByte,
    size: NSTDUInt,
    delim: NSTDByte,
) -> *const NSTDByte {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3"
            ),
            feature = "libc"
        ))] {
            libc::memchr(buf.cast(), delim.into(), size) as _
        } else {
            use crate::NSTD_INT_MAX;
            assert!(size <= NSTD_INT_MAX as _);
            let mut i = 0;
            while i < size {
                if *buf.add(i) == delim {
                    return buf.add(i);
                }
                i += 1;
            }
            core::ptr::null()
        }
    }
}

/// Zeros out a memory buffer.
///
/// # Parameters:
///
/// - `NSTDByte *buf` - A pointer to the first byte in the memory buffer.
///
/// - `NSTDUInt size` - The number of bytes to set to 0.
///
/// # Panics
///
/// This operation will panic if `size` is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// The caller must ensure that `buf` is valid for reads of `size` contiguous bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::nstd_core_mem_zero;
///
/// unsafe {
///     let mut buf = [i32::MAX; 10];
///     nstd_core_mem_zero(buf.as_mut_ptr().cast(), core::mem::size_of::<i32>() * 10);
///     assert!(buf == [0i32; 10]);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_mem_zero(buf: *mut NSTDByte, size: NSTDUInt) {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3"
            ),
            feature = "libc"
        ))] {
            libc::memset(buf.cast(), 0, size);
        } else {
            use crate::NSTD_INT_MAX;
            assert!(size <= NSTD_INT_MAX as _);
            let mut i = 0;
            while i < size {
                *buf.add(i) = 0;
                i += 1;
            }
        }
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
/// # Panics
///
/// This operation will panic if `size` is greater than `NSTDInt`'s max value.
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_mem_fill(buf: *mut NSTDByte, size: NSTDUInt, fill: NSTDByte) {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3"
            ),
            feature = "libc"
        ))] {
            libc::memset(buf.cast(), fill.into(), size);
        } else {
            use crate::NSTD_INT_MAX;
            assert!(size <= NSTD_INT_MAX as _);
            let mut i = 0;
            while i < size {
                *buf.add(i) = fill;
                i += 1;
            }
        }
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
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_mem_copy(dest: *mut NSTDByte, src: *const NSTDByte, num: NSTDUInt) {
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
#[nstdapi]
pub const unsafe fn nstd_core_mem_copy_overlapping(
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
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_mem_swap(x: *mut NSTDByte, y: *mut NSTDByte, num: NSTDUInt) {
    core::ptr::swap_nonoverlapping(x, y, num);
}
