//! A view into a sequence of values in memory.
use crate::{
    core::{
        mem::nstd_core_mem_copy,
        ptr::{nstd_core_ptr_const_new, nstd_core_ptr_new, NSTDPtr, NSTDPtrConst},
    },
    NSTDAny, NSTDAnyConst, NSTDBool, NSTDUSize, NSTD_NULL,
};

/// A view into a sequence of values in memory.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDSlice {
    /// A pointer to the first element in the slice.
    pub ptr: NSTDPtr,
    /// The number of elements in the slice.
    pub len: NSTDUSize,
}
impl NSTDSlice {
    /// Returns the number of bytes that this slice covers.
    #[inline]
    pub(crate) const fn byte_len(&self) -> usize {
        self.len * self.ptr.size
    }

    /// Creates a Rust byte slice from this `NSTDSlice`.
    #[inline]
    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr.raw.cast(), self.byte_len()) }
    }
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
///
/// # Safety
///
/// `ptr`'s data must remain valid while the returned slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_new(
    ptr: NSTDAny,
    element_size: NSTDUSize,
    len: NSTDUSize,
) -> NSTDSlice {
    NSTDSlice {
        ptr: nstd_core_ptr_new(ptr, element_size),
        len,
    }
}

/// Returns the number of elements in a slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_len(slice: &NSTDSlice) -> NSTDUSize {
    slice.len
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
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the slice's boundaries.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
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
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_get_const(
    slice: &NSTDSlice,
    pos: NSTDUSize,
) -> NSTDAnyConst {
    match pos < slice.len {
        true => slice.ptr.raw.add(pos * slice.ptr.size),
        false => NSTD_NULL,
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
/// `NSTDAny element` - A pointer to the first element in `slice` or `NSTD_NULL` if the slice
/// is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
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
/// `NSTDAnyConst element` - A pointer to the first element in `slice` or `NSTD_NULL` if the
/// slice is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_first_const(slice: &NSTDSlice) -> NSTDAnyConst {
    match slice.len > 0 {
        true => slice.ptr.raw,
        false => NSTD_NULL,
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
/// `NSTDAny element` - A pointer to the last element in `slice` or `NSTD_NULL` if the slice
/// is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
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
/// `NSTDAnyConst element` - A pointer to the last element in `slice` or `NSTD_NULL` if the
/// slice is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_last_const(slice: &NSTDSlice) -> NSTDAnyConst {
    match slice.len > 0 {
        true => nstd_core_slice_get_const(slice, slice.len - 1),
        false => NSTD_NULL,
    }
}

/// Compares two slices, returning true if the slices carry, or point to the same data.
///
/// # Parameters:
///
/// - `const NSTDSlice *s1` - The first slice to compare.
///
/// - `const NSTDSlice *s2` - The second slice to compare.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two slices compare equal.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_compare(s1: &NSTDSlice, s2: &NSTDSlice) -> NSTDBool {
    (s1.as_slice() == s2.as_slice()).into()
}

/// Copies data into `dest` from `src`. The number of bytes copied is determined by `src`.
///
/// # Parameters:
///
/// - `NSTDSlice *dest` - The slice to copy data to.
///
/// - `const NSTDSlice *src` - The slice to copy data from.
///
/// # Panics
///
/// This function panics if the byte length of `dest` is less than the byte length of `src`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_copy(dest: &mut NSTDSlice, src: &NSTDSlice) {
    assert!(dest.byte_len() >= src.byte_len());
    unsafe { nstd_core_mem_copy(dest.ptr.raw.cast(), src.ptr.raw.cast(), src.byte_len()) };
}

/// An immutable view into a sequence of values in memory.
#[repr(C)]
#[derive(Debug, Hash)]
pub struct NSTDSliceConst {
    /// A pointer to the first element in the slice.
    pub ptr: NSTDPtrConst,
    /// The number of elements in the slice.
    pub len: NSTDUSize,
}
impl NSTDSliceConst {
    /// Returns the number of bytes that this slice covers.
    #[inline]
    pub(crate) const fn byte_len(&self) -> usize {
        self.len * self.ptr.size
    }

    /// Creates a Rust byte slice from this `NSTDSliceConst`.
    #[inline]
    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr.raw.cast(), self.byte_len()) }
    }
}

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAnyConst ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUSize element_size` - The number of bytes each element occupies.
///
/// - `NSTDUSize len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSliceConst slice` - The new slice.
///
/// # Safety
///
/// `ptr`'s data must remain valid while the returned slice is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_const_new(
    ptr: NSTDAnyConst,
    element_size: NSTDUSize,
    len: NSTDUSize,
) -> NSTDSliceConst {
    NSTDSliceConst {
        ptr: nstd_core_ptr_const_new(ptr, element_size),
        len,
    }
}

/// Returns the number of elements in an immutable slice.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The immutable slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_len(slice: &NSTDSliceConst) -> NSTDUSize {
    slice.len
}

/// Returns an immutable pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_const_get(
    slice: &NSTDSliceConst,
    pos: NSTDUSize,
) -> NSTDAnyConst {
    match pos < slice.len {
        true => slice.ptr.raw.add(pos * slice.ptr.size),
        false => NSTD_NULL,
    }
}

/// Returns an immutable pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the first element in `slice` or `NSTD_NULL` if the
/// slice is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_const_first(slice: &NSTDSliceConst) -> NSTDAnyConst {
    match slice.len > 0 {
        true => slice.ptr.raw,
        false => NSTD_NULL,
    }
}

/// Returns an immutable pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the last element in `slice` or `NSTD_NULL` if the
/// slice is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_const_last(slice: &NSTDSliceConst) -> NSTDAnyConst {
    match slice.len > 0 {
        true => nstd_core_slice_const_get(slice, slice.len - 1),
        false => NSTD_NULL,
    }
}

/// Compares two slices, returning true if the slices carry, or point to the same data.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *s1` - The first slice to compare.
///
/// - `const NSTDSliceConst *s2` - The second slice to compare.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two slices compare equal.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_compare(
    s1: &NSTDSliceConst,
    s2: &NSTDSliceConst,
) -> NSTDBool {
    (s1.as_slice() == s2.as_slice()).into()
}
