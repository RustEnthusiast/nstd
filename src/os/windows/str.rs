//! String slice extensions for Windows.
use crate::{
    alloc::NSTD_ALLOCATOR,
    core::{
        alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
        optional::NSTDOptional,
        str::{nstd_core_str_as_ptr, nstd_core_str_byte_len, NSTDStr},
    },
    vec::{
        nstd_vec_as_ptr_mut, nstd_vec_new, nstd_vec_new_with_cap, nstd_vec_push, nstd_vec_set_len,
        NSTDOptionalVec,
    },
    NSTDChar16, NSTDUInt,
};
use core::ptr::addr_of;
use nstdapi::nstdapi;
use windows_sys::Win32::Globalization::{u_strFromUTF8, U_BUFFER_OVERFLOW_ERROR, U_ZERO_ERROR};

/// Converts a UTF-8 string slice into a null-terminated UTF-16 encoded buffer.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDOptionalVec utf16` - The new UTF-16 encoded buffer on success, or an uninitialized "none"
/// variant on error.
///
/// # Safety
///
/// `str`'s data must be valid for reads, especially in terms of UTF-8 conformance.
#[nstdapi]
pub unsafe fn nstd_os_windows_str_to_utf16(str: &NSTDStr) -> NSTDOptionalVec<'_> {
    /// The size of a UTF-16 code point.
    const CHAR_SIZE: NSTDUInt = core::mem::size_of::<NSTDChar16>();
    /// The alignment of a UTF-16 code point.
    const CHAR_ALIGN: NSTDUInt = core::mem::align_of::<NSTDChar16>();
    // Make sure the string slice's length is greater than 0.
    if let Ok(len) = nstd_core_str_byte_len(str).try_into() {
        if len == 0 {
            let mut v = nstd_vec_new(&NSTD_ALLOCATOR, CHAR_SIZE, CHAR_ALIGN);
            let nul: NSTDChar16 = 0;
            return match nstd_vec_push(&mut v, addr_of!(nul).cast()) {
                NSTD_ALLOC_ERROR_NONE => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            };
        }
        // Precalculate the length of the UTF-16 buffer.
        let mut u16_len = 0;
        let str_ptr = nstd_core_str_as_ptr(str);
        let mut errc = U_ZERO_ERROR;
        u_strFromUTF8(
            core::ptr::null_mut(),
            0,
            &mut u16_len,
            str_ptr,
            len,
            &mut errc,
        );
        #[allow(clippy::arithmetic_side_effects, clippy::cast_sign_loss)]
        if errc == U_ZERO_ERROR || errc == U_BUFFER_OVERFLOW_ERROR {
            // Make sure there is space for the null-terminator.
            u16_len += 1;
            // Create the buffer.
            if let NSTDOptional::Some(mut buf) =
                nstd_vec_new_with_cap(&NSTD_ALLOCATOR, CHAR_SIZE, CHAR_ALIGN, u16_len as _)
            {
                // Fill the buffer.
                let buf_ptr = nstd_vec_as_ptr_mut(&mut buf).cast();
                errc = U_ZERO_ERROR;
                u_strFromUTF8(
                    buf_ptr,
                    u16_len,
                    core::ptr::null_mut(),
                    str_ptr,
                    len,
                    &mut errc,
                );
                if errc == U_ZERO_ERROR {
                    nstd_vec_set_len(&mut buf, u16_len as _);
                    return NSTDOptional::Some(buf);
                }
            }
        }
    }
    NSTDOptional::None
}
