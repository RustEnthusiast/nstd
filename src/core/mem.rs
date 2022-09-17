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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_search(
    mut buf: *const NSTDByte,
    size: NSTDUInt,
    delim: NSTDByte,
) -> *const NSTDByte {
    #[cfg(not(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        let mut i = 0;
        while i < size {
            if *buf == delim {
                return buf;
            }
            buf = buf.offset(1);
            i += 1;
        }
        core::ptr::null()
    }
    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        use core::arch::asm;
        asm!(
            include_str!("mem/search.asm"),
            buf = inout(reg) buf,
            size = in(reg) size,
            delim = in(reg_byte) delim,
            end = out(reg) _
        );
        buf
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_zero(buf: *mut NSTDByte, size: NSTDUInt) {
    #[cfg(not(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        let mut i = 0;
        while i < size {
            *buf.add(i) = 0;
            i += 1;
        }
    }
    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        use core::arch::asm;
        asm!(include_str!("mem/zero.asm"), buf = in(reg) buf, size = in(reg) size, i = out(reg) _);
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_fill(buf: *mut NSTDByte, size: NSTDUInt, fill: NSTDByte) {
    #[cfg(not(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        let mut i = 0;
        while i < size {
            *buf.add(i) = fill;
            i += 1;
        }
    }
    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        use core::arch::asm;
        asm!(
            include_str!("mem/fill.asm"),
            buf = in(reg) buf,
            size = in(reg) size,
            fill = in(reg_byte) fill,
            i = out(reg) _
        );
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
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_mem_swap(x: *mut NSTDByte, y: *mut NSTDByte, num: NSTDUInt) {
    core::ptr::swap_nonoverlapping(x, y, num);
}
