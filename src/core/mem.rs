//! Contains mostly unsafe functions for interacting with raw memory.
use crate::{core::def::NSTDByte, NSTDAny, NSTDAnyMut, NSTDBool, NSTDUInt};
use cfg_if::cfg_if;
use nstdapi::nstdapi;

/// The default alignment suitable for any scalar type.
///
/// Corresponds to `alignof(max_align_t)`.
/// The C/C++ standards specify that this value should be at least 8 or 16, I'm going with 16 for
/// safety but of course this is platform dependent so if you (the reader) know of a platform that
/// this value is smaller (or larger for that matter) on, please submit an issue/pull request.
pub(crate) const MAX_ALIGN: usize = 16;

/// Checks if `align` is a power of 2.
#[inline]
#[allow(clippy::arithmetic_side_effects)]
pub(crate) const fn is_power_of_two(align: NSTDUInt) -> NSTDBool {
    (align != 0) && ((align & (align - 1)) == 0)
}

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
#[allow(unused_mut)]
pub unsafe fn nstd_core_mem_compare(
    mut buf1: *const NSTDByte,
    mut buf2: *const NSTDByte,
    num: NSTDUInt,
) -> NSTDBool {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3",
                target_os = "teeos"
            ),
            feature = "libc"
        ))] {
            libc::memcmp(buf1.cast(), buf2.cast(), num) == 0
        } else {
            use crate::{NSTD_FALSE, NSTD_TRUE};
            if buf1 == buf2 || num == 0 {
                return NSTD_TRUE;
            }
            let mut i = 0;
            #[allow(clippy::arithmetic_side_effects)]
            while i < num {
                if *buf1 != *buf2 {
                    return NSTD_FALSE;
                }
                buf1 = buf1.add(1);
                buf2 = buf2.add(1);
                i += 1;
            }
            NSTD_TRUE
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
#[allow(unused_mut, clippy::missing_const_for_fn)]
pub unsafe fn nstd_core_mem_search(
    mut buf: *const NSTDByte,
    size: NSTDUInt,
    delim: NSTDByte,
) -> *const NSTDByte {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3",
                target_os = "teeos"
            ),
            feature = "libc"
        ))] {
            libc::memchr(buf.cast(), delim.into(), size) as _
        } else {
            let mut i = 0;
            #[allow(clippy::arithmetic_side_effects)]
            while i < size {
                if *buf == delim {
                    return buf;
                }
                buf = buf.add(1);
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
#[allow(unused_mut, clippy::missing_const_for_fn)]
pub unsafe fn nstd_core_mem_zero(mut buf: *mut NSTDByte, size: NSTDUInt) {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3",
                target_os = "teeos"
            ),
            feature = "libc"
        ))] {
            libc::memset(buf.cast(), 0, size);
        } else {
            let mut i = 0;
            #[allow(clippy::arithmetic_side_effects)]
            while i < size {
                *buf = 0;
                buf = buf.add(1);
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
#[allow(unused_mut, clippy::missing_const_for_fn)]
pub unsafe fn nstd_core_mem_fill(mut buf: *mut NSTDByte, size: NSTDUInt, fill: NSTDByte) {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "solid_asp3",
                target_os = "teeos"
            ),
            feature = "libc"
        ))] {
            libc::memset(buf.cast(), fill.into(), size);
        } else {
            let mut i = 0;
            #[allow(clippy::arithmetic_side_effects)]
            while i < size {
                *buf = fill;
                buf = buf.add(1);
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
#[allow(clippy::missing_const_for_fn)]
pub unsafe fn nstd_core_mem_copy(dest: *mut NSTDByte, src: *const NSTDByte, num: NSTDUInt) {
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
#[allow(clippy::missing_const_for_fn)]
pub unsafe fn nstd_core_mem_copy_overlapping(
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

/// Creates a new dangling pointer to some immutable memory. The pointer is guaranteed to have valid
/// alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAny dangling` - The new dangling raw pointer.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{mem::nstd_core_mem_dangling, slice::nstd_core_slice_new};
///
/// let slice = unsafe { nstd_core_slice_new(nstd_core_mem_dangling(), 1, 0).unwrap() };
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_mem_dangling() -> NSTDAny {
    MAX_ALIGN as NSTDAny
}

/// Creates a new dangling pointer to some mutable memory. The pointer is guaranteed to have valid
/// alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAnyMut dangling` - The new dangling raw pointer.
///
/// # Example
///
/// ```
/// use nstd_sys::core::{mem::nstd_core_mem_dangling_mut, slice::nstd_core_slice_mut_new};
///
/// let slice = unsafe { nstd_core_slice_mut_new(nstd_core_mem_dangling_mut(), 1, 0).unwrap() };
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_mem_dangling_mut() -> NSTDAnyMut {
    MAX_ALIGN as NSTDAnyMut
}

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
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::{nstd_core_mem_align, nstd_core_mem_is_aligned};
///
/// unsafe {
///     let ptr = 2 as _;
///     let aligned = nstd_core_mem_align(ptr, 16);
///     assert!(nstd_core_mem_is_aligned(aligned, 16));
/// }
/// ```
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub unsafe fn nstd_core_mem_align(ptr: NSTDAny, align: NSTDUInt) -> NSTDAny {
    assert!(is_power_of_two(align));
    ((ptr as NSTDUInt)
        .checked_add(align - 1)
        .expect("pointer arithmetic should not overflow")
        & !(align - 1)) as NSTDAny
}

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
///
/// # Example
///
/// ```
/// use nstd_sys::core::mem::{nstd_core_mem_align_mut, nstd_core_mem_is_aligned};
///
/// unsafe {
///     let ptr = 2 as _;
///     let aligned = nstd_core_mem_align_mut(ptr, 16);
///     assert!(nstd_core_mem_is_aligned(aligned, 16));
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_mem_align_mut(ptr: NSTDAnyMut, align: NSTDUInt) -> NSTDAnyMut {
    nstd_core_mem_align(ptr, align).cast_mut()
}

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
///
/// # Example
///
/// ```
/// use nstd_sys::{
///     core::mem::{nstd_core_mem_align, nstd_core_mem_is_aligned},
///     NSTDAny,
/// };
///
/// unsafe {
///     let mut a = 1usize as NSTDAny;
///     a = nstd_core_mem_align(a, 8);
///     assert!(!nstd_core_mem_is_aligned(a, 16));
///     a = nstd_core_mem_align(a, 16);
///     assert!(nstd_core_mem_is_aligned(a, 16));
/// }
/// ```
#[inline]
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub fn nstd_core_mem_is_aligned(ptr: NSTDAny, align: NSTDUInt) -> NSTDBool {
    assert!(is_power_of_two(align));
    ptr as NSTDUInt & (align - 1) == 0
}
