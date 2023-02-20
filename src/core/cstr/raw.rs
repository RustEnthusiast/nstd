//! Raw C string processing.
use crate::{NSTDBool, NSTDChar, NSTDUInt};
use cfg_if::cfg_if;
use nstdapi::nstdapi;

/// Gets the length of a raw null terminated C string, excluding the null-terminator.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string, excluding the null-terminator.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::raw::nstd_core_cstr_raw_len;
///
/// let cstr = b"Hello, world!\0";
/// assert!(unsafe { nstd_core_cstr_raw_len(cstr.as_ptr().cast()) } == 13);
/// ```
#[inline]
#[nstdapi]
#[allow(unused_mut)]
pub unsafe fn nstd_core_cstr_raw_len(mut cstr: *const NSTDChar) -> NSTDUInt {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "fuchsia",
                target_os = "solid_asp3",
                target_os = "vxworks"
            ),
            feature = "libc"
        ))] {
            libc::strlen(cstr)
        } else if #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))] {
            let len;
            core::arch::asm!(
                include_str!("raw/x86/len.asm"),
                len = out(reg) len,
                cstr = inout(reg) cstr => _
            );
            len
        } else if #[cfg(all(feature = "asm", target_arch = "arm"))] {
            let len;
            core::arch::asm!(
                include_str!("raw/arm/len.asm"),
                len = out(reg) len,
                cstr = inout(reg) cstr => _,
                byte = out(reg) _
            );
            len
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            let len;
            core::arch::asm!(
                include_str!("raw/arm64/len.asm"),
                len = out(reg) len,
                cstr = inout(reg) cstr => _,
                byte = out(reg) _
            );
            len
        } else {
            let mut i = 0;
            while *cstr != 0 {
                cstr = cstr.offset(1);
                i += 1;
            }
            i
        }
    }
}

/// Gets the length of a raw null terminated C string, including the null-terminator.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr` - The null terminated C string.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the C string, including the null-terminator.
///
/// # Safety
///
/// `cstr` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::raw::nstd_core_cstr_raw_len_with_null;
///
/// let cstr = b"Hello, world!\0";
/// assert!(unsafe { nstd_core_cstr_raw_len_with_null(cstr.as_ptr().cast()) } == 14);
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_cstr_raw_len_with_null(cstr: *const NSTDChar) -> NSTDUInt {
    nstd_core_cstr_raw_len(cstr) + 1
}

/// Compares two raw null-terminated C strings, returning `NSTD_TRUE` if they are lexicographically
/// equal.
///
/// # Parameters:
///
/// - `const NSTDChar *cstr1` - The first C string.
///
/// - `const NSTDChar *cstr2` - The second C string.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two C strings are lexicographically equal.
///
/// # Safety
///
/// Both `cstr1` and `cstr2` must point to character arrays that are valid for reads up until and
/// including their null-terminating bytes.
///
/// # Example
///
/// ```
/// use nstd_sys::{core::cstr::raw::nstd_core_cstr_raw_compare, NSTD_FALSE};
///
/// let cstr1 = b"Hello, world!\0".as_ptr().cast();
/// let cstr2 = b"Hello world!\0".as_ptr().cast();
///
/// assert!(unsafe { nstd_core_cstr_raw_compare(cstr1, cstr2) } == NSTD_FALSE);
/// ```
#[nstdapi]
#[allow(unused_mut)]
pub unsafe fn nstd_core_cstr_raw_compare(
    mut cstr1: *const NSTDChar,
    mut cstr2: *const NSTDChar,
) -> NSTDBool {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "fuchsia",
                target_os = "solid_asp3",
                target_os = "vxworks"
            ),
            feature = "libc"
        ))] {
            libc::strcmp(cstr1, cstr2) == 0
        } else if #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))] {
            use crate::core::def::NSTDByte;
            let is_eq: NSTDByte;
            core::arch::asm!(
                include_str!("raw/x86/compare.asm"),
                cstr1 = inout(reg) cstr1 => _,
                cstr2 = inout(reg) cstr2 => _,
                is_eq = out(reg_byte) is_eq,
                byte = out(reg_byte) _
            );
            is_eq != 0
        } else if #[cfg(all(feature = "asm", target_arch = "arm"))] {
            let is_eq: NSTDUInt;
            core::arch::asm!(
                include_str!("raw/arm/compare.asm"),
                cstr1 = inout(reg) cstr1 => _,
                cstr2 = inout(reg) cstr2 => _,
                is_eq = out(reg) is_eq,
                ch1 = out(reg) _,
                ch2 = out(reg) _
            );
            is_eq != 0
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            let is_eq: NSTDUInt;
            core::arch::asm!(
                include_str!("raw/arm64/compare.asm"),
                cstr1 = inout(reg) cstr1 => _,
                cstr2 = inout(reg) cstr2 => _,
                is_eq = out(reg) is_eq,
                ch1 = out(reg) _,
                ch2 = out(reg) _
            );
            is_eq != 0
        } else {
            use crate::{NSTD_FALSE, NSTD_TRUE};
            // If the C strings point to the same data return true.
            if cstr1 == cstr2 {
                return NSTD_TRUE;
            }
            // Otherwise compare them lexicographically.
            while *cstr1 == *cstr2 {
                if *cstr1 == 0 {
                    return NSTD_TRUE;
                }
                cstr1 = cstr1.offset(1);
                cstr2 = cstr2.offset(1);
            }
            NSTD_FALSE
        }
    }
}

/// Copies the contents of one raw C string to another, excluding the source's null-terminator.
///
/// # Note
///
/// If you already know how many bytes should be copied, `nstd_core_mem_copy[_overlapped]` should
/// be used instead as it can minimize execution times.
///
/// # Parameters:
///
/// - `NSTDChar *dest` - The C string buffer to copy data to.
///
/// - `const NSTDChar *src` - The C string to copy data from.
///
/// # Safety
///
/// - `src` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `dest` must point to a character array that is valid for writes.
///
/// - `dest`'s buffer must be large enough to contain the contents of `src`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::raw::nstd_core_cstr_raw_copy;
///
/// let cstr = b"Hello, world!\0";
/// let mut buffer = [0u8; 14];
///
/// unsafe { nstd_core_cstr_raw_copy(buffer.as_mut_ptr().cast(), cstr.as_ptr().cast()) };
/// assert!(&buffer == cstr);
/// ```
#[inline]
#[nstdapi]
#[allow(unused_mut)]
pub unsafe fn nstd_core_cstr_raw_copy(mut dest: *mut NSTDChar, mut src: *const NSTDChar) {
    cfg_if! {
        if #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))] {
            core::arch::asm!(
                include_str!("raw/x86/copy.asm"),
                dest = inout(reg) dest => _,
                src = inout(reg) src => _,
                byte = out(reg_byte) _
            );
        }
        else if #[cfg(all(feature = "asm", target_arch = "arm"))] {
            core::arch::asm!(
                include_str!("raw/arm/copy.asm"),
                dest = inout(reg) dest => _,
                src = inout(reg) src => _,
                byte = out(reg) _
            );
        }
        else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            core::arch::asm!(
                include_str!("raw/arm64/copy.asm"),
                dest = inout(reg) dest => _,
                src = inout(reg) src => _,
                byte = out(reg) _
            );
        } else {
            while *src != 0 {
                *dest = *src;
                dest = dest.offset(1);
                src = src.offset(1);
            }
        }
    }
}

/// Copies the contents of one raw C string to another, including the source's null-terminator.
///
/// # Note
///
/// If you already know how many bytes should be copied, `nstd_core_mem_copy[_overlapped]` should
/// be used instead as it can minimize execution times.
///
/// # Parameters:
///
/// - `NSTDChar *dest` - The C string buffer to copy data to.
///
/// - `const NSTDChar *src` - The C string to copy data from.
///
/// # Safety
///
/// - `src` must point to a character array that is valid for reads up until and including it's
/// null-terminating byte.
///
/// - `dest` must point to a character array that is valid for writes.
///
/// - `dest`'s buffer must be large enough to contain the contents of `src`, including it's
/// null-terminating byte.
///
/// # Example
///
/// ```
/// use nstd_sys::core::cstr::raw::nstd_core_cstr_raw_copy_with_null;
///
/// let cstr = b"Hello, world!\0";
/// let mut buffer = [u8::MAX; 14];
///
/// let buf_ptr = buffer.as_mut_ptr().cast();
/// unsafe { nstd_core_cstr_raw_copy_with_null(buf_ptr, cstr.as_ptr().cast()) };
/// assert!(&buffer == cstr);
/// ```
#[inline]
#[nstdapi]
#[allow(unused_mut)]
pub unsafe fn nstd_core_cstr_raw_copy_with_null(mut dest: *mut NSTDChar, mut src: *const NSTDChar) {
    cfg_if! {
        if #[cfg(all(
            any(
                unix,
                windows,
                any(target_env = "wasi", target_os = "wasi"),
                target_os = "fuchsia",
                target_os = "solid_asp3",
                target_os = "vxworks"
            ),
            feature = "libc"
        ))] {
            libc::strcpy(dest, src);
        } else if #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))] {
            core::arch::asm!(
                include_str!("raw/x86/copy_with_null.asm"),
                dest = inout(reg) dest => _,
                src = inout(reg) src => _,
                byte = out(reg_byte) _
            );
        } else if #[cfg(all(feature = "asm", target_arch = "arm"))] {
            core::arch::asm!(
                include_str!("raw/arm/copy_with_null.asm"),
                dest = inout(reg) dest => _,
                src = inout(reg) src => _,
                byte = out(reg) _
            );
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            core::arch::asm!(
                include_str!("raw/arm64/copy_with_null.asm"),
                dest = inout(reg) dest => _,
                src = inout(reg) src => _,
                byte = out(reg) _
            );
        } else {
            while {
                *dest = *src;
                *src != 0
            } {
                dest = dest.offset(1);
                src = src.offset(1);
            }
        }
    }
}
