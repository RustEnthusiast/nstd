//! A view into a sequence of values in memory.
use crate::core::{
    def::{NSTDAny, NSTDAnyConst, NSTDUSize},
    ptr::{nstd_core_ptr_new, NSTDPtr},
    NSTD_CORE_NULL,
};

/// A view into a sequence of values in memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDSlice {
    /// A pointer to the first element in the slice.
    pub ptr: NSTDPtr,
    /// The number of elements in the slice.
    pub len: NSTDUSize,
}

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUSize element_size` - The number of bytes each element occupies.
///
/// - `NSTDUSize len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_new(
    ptr: NSTDAny,
    element_size: NSTDUSize,
    len: NSTDUSize,
) -> NSTDSlice {
    NSTDSlice {
        ptr: nstd_core_ptr_new(ptr, element_size),
        len,
    }
}

/// Returns a pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `NSTDSlice *slice` - The slice to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_CORE_NULL` if `pos` is out of
/// the slice's boundaries.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_get(slice: &mut NSTDSlice, pos: NSTDUSize) -> NSTDAny {
    nstd_core_slice_get_const(slice, pos) as NSTDAny
}

/// Returns an immutable pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_CORE_NULL` if `pos` is out
/// of the slice's boundaries.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_get_const(
    slice: &NSTDSlice,
    pos: NSTDUSize,
) -> NSTDAnyConst {
    match pos < slice.len {
        true => slice.ptr.raw.add(pos * slice.ptr.size),
        false => NSTD_CORE_NULL,
    }
}

/// Returns a pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `NSTDSlice *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the first element in `slice` or `NSTD_CORE_NULL` if the slice
/// is empty.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_first(slice: &mut NSTDSlice) -> NSTDAny {
    nstd_core_slice_first_const(slice) as NSTDAny
}

/// Returns an immutable pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the first element in `slice` or `NSTD_CORE_NULL` if the
/// slice is empty.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_first_const(slice: &NSTDSlice) -> NSTDAnyConst {
    match slice.len > 0 {
        true => slice.ptr.raw,
        false => NSTD_CORE_NULL,
    }
}

/// Returns a pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `NSTDSlice *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the last element in `slice` or `NSTD_CORE_NULL` if the slice
/// is empty.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_last(slice: &mut NSTDSlice) -> NSTDAny {
    nstd_core_slice_last_const(slice) as NSTDAny
}

/// Returns an immutable pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the last element in `slice` or `NSTD_CORE_NULL` if the
/// slice is empty.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_last_const(slice: &NSTDSlice) -> NSTDAnyConst {
    match slice.len > 0 {
        true => nstd_core_slice_get_const(slice, slice.len - 1),
        false => NSTD_CORE_NULL,
    }
}
