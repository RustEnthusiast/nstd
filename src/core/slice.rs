//! A view into a sequence of values in memory.
use crate::{
    core::{
        def::NSTDErrorCode,
        mem::nstd_core_mem_copy,
        ptr::{
            nstd_core_ptr_const_get, nstd_core_ptr_const_new, nstd_core_ptr_const_size,
            nstd_core_ptr_mut_get, nstd_core_ptr_mut_get_const, nstd_core_ptr_mut_new,
            nstd_core_ptr_mut_size, NSTDPtrConst, NSTDPtrMut,
        },
    },
    NSTDAnyConst, NSTDAnyMut, NSTDBool, NSTDUSize, NSTD_NULL,
};

/// An immutable view into a sequence of values in memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDSliceConst {
    /// A pointer to the first element in the slice.
    pub(crate) ptr: NSTDPtrConst,
    /// The number of elements in the slice.
    pub(crate) len: NSTDUSize,
}
impl NSTDSliceConst {
    /// Returns the number of bytes that this slice covers.
    #[inline]
    pub(crate) fn byte_len(&self) -> usize {
        self.len * nstd_core_slice_const_stride(self)
    }

    /// Creates a Rust byte slice from this `NSTDSliceConst`.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// The `NSTDSliceConst`'s data must remain valid while the returned slice is in use.
    #[inline]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_const_stride(self) == core::mem::size_of::<T>());
        core::slice::from_raw_parts(self.ptr.raw.cast(), self.byte_len())
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_new(
    ptr: NSTDAnyConst,
    element_size: NSTDUSize,
    len: NSTDUSize,
) -> NSTDSliceConst {
    NSTDSliceConst {
        ptr: nstd_core_ptr_const_new(ptr, element_size),
        len,
    }
}

/// Returns a raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice.
///
/// # Returns
///
/// `AnyConst ptr` - A raw pointer to the slice's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_as_ptr(slice: &NSTDSliceConst) -> NSTDAnyConst {
    nstd_core_ptr_const_get(&slice.ptr)
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

/// Returns the amount of bytes each value in a slice occupies.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice.
///
/// # Returns
///
/// `NSTDUSize stride` - The size of each value in the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_stride(slice: &NSTDSliceConst) -> NSTDUSize {
    nstd_core_ptr_const_size(&slice.ptr)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_get(
    slice: &NSTDSliceConst,
    pos: NSTDUSize,
) -> NSTDAnyConst {
    match pos < slice.len {
        // SAFETY: We've checked `pos`, and the returned pointer is already unsafe to access.
        true => unsafe {
            let stride = nstd_core_slice_const_stride(slice);
            slice.ptr.raw.add(pos * stride)
        },
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_first(slice: &NSTDSliceConst) -> NSTDAnyConst {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_const_last(slice: &NSTDSliceConst) -> NSTDAnyConst {
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
///
/// # Safety
///
/// This function can cause undefined behavior if either `s1` or `s2`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_const_compare(
    s1: &NSTDSliceConst,
    s2: &NSTDSliceConst,
) -> NSTDBool {
    (s1.as_slice::<u8>() == s2.as_slice::<u8>()).into()
}

/// A view into a sequence of values in memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDSliceMut {
    /// A pointer to the first element in the slice.
    pub(crate) ptr: NSTDPtrMut,
    /// The number of elements in the slice.
    pub(crate) len: NSTDUSize,
}
impl NSTDSliceMut {
    /// Returns the number of bytes that this slice covers.
    #[inline]
    pub(crate) fn byte_len(&self) -> usize {
        self.len * nstd_core_slice_mut_stride(self)
    }

    /// Creates a Rust byte slice from this `NSTDSliceMut`.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// The `NSTDSliceMut`'s data must remain valid while the returned slice is in use.
    #[inline]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_mut_stride(self) == core::mem::size_of::<T>());
        core::slice::from_raw_parts(self.ptr.raw.cast(), self.byte_len())
    }

    /// Creates a mutable Rust byte slice from this `NSTDSliceMut`.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// The `NSTDSliceMut`'s data must remain valid while the returned slice is in use.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_slice_mut<T>(&mut self) -> &mut [T] {
        assert!(nstd_core_slice_mut_stride(self) == core::mem::size_of::<T>());
        core::slice::from_raw_parts_mut(self.ptr.raw.cast(), self.byte_len())
    }
}

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUSize element_size` - The number of bytes each element occupies.
///
/// - `NSTDUSize len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSliceMut slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_new(
    ptr: NSTDAnyMut,
    element_size: NSTDUSize,
    len: NSTDUSize,
) -> NSTDSliceMut {
    NSTDSliceMut {
        ptr: nstd_core_ptr_mut_new(ptr, element_size),
        len,
    }
}

/// Returns a raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A raw pointer to the slice's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_as_ptr(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
    nstd_core_ptr_mut_get(&mut slice.ptr)
}

/// Returns an immutable raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDAnyConst ptr` - A raw pointer to the slice's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_as_ptr_const(slice: &NSTDSliceMut) -> NSTDAnyConst {
    nstd_core_ptr_mut_get_const(&slice.ptr)
}

/// Returns the number of elements in a slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_len(slice: &NSTDSliceMut) -> NSTDUSize {
    slice.len
}

/// Returns the amount of bytes each value in a slice occupies.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDUSize stride` - The size of each value in the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_stride(slice: &NSTDSliceMut) -> NSTDUSize {
    nstd_core_ptr_mut_size(&slice.ptr)
}

/// Returns a pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `NSTDSliceMut *slice` - The slice to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the slice's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_get(slice: &mut NSTDSliceMut, pos: NSTDUSize) -> NSTDAnyMut {
    nstd_core_slice_mut_get_const(slice, pos) as NSTDAnyMut
}

/// Returns an immutable pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice to read an element from.
///
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_get_const(
    slice: &NSTDSliceMut,
    pos: NSTDUSize,
) -> NSTDAnyConst {
    match pos < slice.len {
        // SAFETY: We've checked `pos`, and the returned pointer is already unsafe to access.
        true => unsafe {
            let stride = nstd_core_slice_mut_stride(slice);
            slice.ptr.raw.add(pos * stride)
        },
        false => NSTD_NULL,
    }
}

/// Returns a pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `NSTDSliceMut *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the first element in `slice` or `NSTD_NULL` if the slice
/// is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_first(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
    nstd_core_slice_mut_first_const(slice) as NSTDAnyMut
}

/// Returns an immutable pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the first element in `slice` or `NSTD_NULL` if the
/// slice is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_first_const(slice: &NSTDSliceMut) -> NSTDAnyConst {
    match slice.len > 0 {
        true => slice.ptr.raw,
        false => NSTD_NULL,
    }
}

/// Returns a pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `NSTDSliceMut *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the last element in `slice` or `NSTD_NULL` if the slice
/// is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_last(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
    nstd_core_slice_mut_last_const(slice) as NSTDAnyMut
}

/// Returns an immutable pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the last element in `slice` or `NSTD_NULL` if the
/// slice is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_last_const(slice: &NSTDSliceMut) -> NSTDAnyConst {
    match slice.len > 0 {
        true => nstd_core_slice_mut_get_const(slice, slice.len - 1),
        false => NSTD_NULL,
    }
}

/// Compares two slices, returning true if the slices carry, or point to the same data.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *s1` - The first slice to compare.
///
/// - `const NSTDSliceMut *s2` - The second slice to compare.
///
/// # Returns
///
/// `NSTDBool is_eq` - `NSTD_TRUE` if the two slices compare equal.
///
/// # Safety
///
/// This function can cause undefined behavior if either `s1` or `s2`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_mut_compare(
    s1: &NSTDSliceMut,
    s2: &NSTDSliceMut,
) -> NSTDBool {
    (s1.as_slice::<u8>() == s2.as_slice::<u8>()).into()
}

/// Copies data into `dest` from `src`. The number of bytes copied is determined by `src`.
///
/// # Parameters:
///
/// - `NSTDSliceMut *dest` - The slice to copy data to.
///
/// - `const NSTDSliceConst *src` - The slice to copy data from.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - The two buffer's lengths do not match.
///
/// - `2` - The two buffer's strides do not match.
///
/// # Safety
///
/// This function can cause undefined behavior if either `dest` or `src`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_mut_copy(
    dest: &mut NSTDSliceMut,
    src: &NSTDSliceConst,
) -> NSTDErrorCode {
    if dest.len != src.len {
        1
    } else if nstd_core_slice_mut_stride(dest) != nstd_core_slice_const_stride(src) {
        2
    } else {
        nstd_core_mem_copy(dest.ptr.raw.cast(), src.ptr.raw.cast(), src.byte_len());
        0
    }
}
