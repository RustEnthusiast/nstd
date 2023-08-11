//! String slice extensions for Windows.
use crate::{
    alloc::NSTD_ALLOCATOR,
    core::str::{nstd_core_str_as_ptr, nstd_core_str_byte_len, NSTDStr},
    vec::{
        nstd_vec_as_ptr_mut, nstd_vec_cap, nstd_vec_new, nstd_vec_new_with_cap, nstd_vec_set_len,
        NSTDVec,
    },
    NSTDChar16, NSTDUInt,
};
use nstdapi::nstdapi;
use windows_sys::Win32::Globalization::{u_strFromUTF8, U_BUFFER_OVERFLOW_ERROR, U_ZERO_ERROR};

/// Converts a UTF-8 string slice into a null-terminated UTF-16 encoded buffer.
///
/// If the string is empty, this will return a buffer that has yet to allocate.
///
/// # Parameters:
///
/// - `const NSTDStr *str` - The UTF-8 encoded string slice.
///
/// # Returns
///
/// `NSTDVec utf16` - The new UTF-16 encoded buffer.
///
/// # Panics
///
/// This operation will panic if either conversion or allocation fails.
///
/// # Safety
///
/// `str`'s data must be valid for reads, especially in terms of UTF-8 conformance.
#[nstdapi]
pub unsafe fn nstd_os_windows_str_to_utf16(str: &NSTDStr) -> NSTDVec<'_> {
    // The size of a UTF-16 code point.
    const CHAR_SIZE: NSTDUInt = core::mem::size_of::<NSTDChar16>();
    // Make sure the string slice's length is greater than 0.
    let len = nstd_core_str_byte_len(str) as _;
    if len == 0 {
        return nstd_vec_new(&NSTD_ALLOCATOR, CHAR_SIZE);
    }
    // Precalculate the length of the UTF-16 buffer.
    let mut u16_len = 0;
    let ptr = nstd_core_str_as_ptr(str);
    let mut errc = U_ZERO_ERROR;
    u_strFromUTF8(core::ptr::null_mut(), 0, &mut u16_len, ptr, len, &mut errc);
    assert!(errc == U_ZERO_ERROR || errc == U_BUFFER_OVERFLOW_ERROR);
    errc = U_ZERO_ERROR;
    // Make sure there is space for the null-terminator.
    u16_len += 1;
    // Create the buffer.
    let mut buf = nstd_vec_new_with_cap(&NSTD_ALLOCATOR, CHAR_SIZE, u16_len as _);
    assert!(nstd_vec_cap(&buf) == u16_len as _);
    // Fill the buffer.
    let buf_ptr = nstd_vec_as_ptr_mut(&mut buf) as _;
    u_strFromUTF8(buf_ptr, u16_len, core::ptr::null_mut(), ptr, len, &mut errc);
    assert!(errc == U_ZERO_ERROR);
    nstd_vec_set_len(&mut buf, u16_len as _);
    buf
}
