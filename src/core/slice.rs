//! A view into a sequence of values in memory.
use crate::{
    core::{
        def::NSTDErrorCode,
        mem::nstd_core_mem_copy,
        ptr::{
            nstd_core_ptr_get, nstd_core_ptr_mut_get, nstd_core_ptr_mut_get_const,
            nstd_core_ptr_mut_new, nstd_core_ptr_mut_size, nstd_core_ptr_new, nstd_core_ptr_size,
            NSTDPtr, NSTDPtrMut,
        },
    },
    NSTDAny, NSTDAnyMut, NSTDBool, NSTDUInt, NSTD_NULL,
};

/// An immutable view into a sequence of values in memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDSlice {
    /// A pointer to the first element in the slice.
    pub(crate) ptr: NSTDPtr,
    /// The number of elements in the slice.
    pub(crate) len: NSTDUInt,
}
impl NSTDSlice {
    /// Returns the number of bytes that this slice covers.
    #[inline]
    pub(crate) fn byte_len(&self) -> usize {
        self.len * nstd_core_slice_stride(self)
    }

    /// Creates a Rust byte slice from this `NSTDSlice`.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// The `NSTDSlice`'s data must remain valid while the returned slice is in use.
    #[inline]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_stride(self) == core::mem::size_of::<T>());
        core::slice::from_raw_parts(self.ptr.raw.cast(), nstd_core_slice_len(self))
    }
}

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSlice slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_new(
    ptr: NSTDAny,
    element_size: NSTDUInt,
    len: NSTDUInt,
) -> NSTDSlice {
    NSTDSlice {
        ptr: nstd_core_ptr_new(ptr, element_size),
        len,
    }
}

/// Returns a raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `AnyConst ptr` - A raw pointer to the slice's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_as_ptr(slice: &NSTDSlice) -> NSTDAny {
    nstd_core_ptr_get(&slice.ptr)
}

/// Returns the number of elements in an immutable slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The immutable slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_len(slice: &NSTDSlice) -> NSTDUInt {
    slice.len
}

/// Returns the amount of bytes each value in a slice occupies.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt stride` - The size of each value in the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_stride(slice: &NSTDSlice) -> NSTDUInt {
    nstd_core_ptr_size(&slice.ptr)
}

/// Returns an immutable pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to read an element from.
///
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_get(slice: &NSTDSlice, pos: NSTDUInt) -> NSTDAny {
    match pos < slice.len {
        // SAFETY: We've checked `pos`, and the returned pointer is already unsafe to access.
        true => unsafe {
            let stride = nstd_core_slice_stride(slice);
            slice.ptr.raw.add(pos * stride)
        },
        false => NSTD_NULL,
    }
}

/// Returns an immutable pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the first element in `slice` or `NSTD_NULL` if the
/// slice is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_first(slice: &NSTDSlice) -> NSTDAny {
    match slice.len > 0 {
        true => slice.ptr.raw,
        false => NSTD_NULL,
    }
}

/// Returns an immutable pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the last element in `slice` or `NSTD_NULL` if the
/// slice is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_last(slice: &NSTDSlice) -> NSTDAny {
    match slice.len > 0 {
        true => nstd_core_slice_get(slice, slice.len - 1),
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
///
/// # Safety
///
/// This function can cause undefined behavior if either `s1` or `s2`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_slice_compare(s1: &NSTDSlice, s2: &NSTDSlice) -> NSTDBool {
    (s1.as_slice::<u8>() == s2.as_slice::<u8>()).into()
}

/// A view into a sequence of values in memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct NSTDSliceMut {
    /// A pointer to the first element in the slice.
    pub(crate) ptr: NSTDPtrMut,
    /// The number of elements in the slice.
    pub(crate) len: NSTDUInt,
}
impl NSTDSliceMut {
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
        core::slice::from_raw_parts(self.ptr.raw.cast(), nstd_core_slice_mut_len(self))
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
    pub(crate) unsafe fn as_slice_mut<T>(&mut self) -> &mut [T] {
        assert!(nstd_core_slice_mut_stride(self) == core::mem::size_of::<T>());
        core::slice::from_raw_parts_mut(self.ptr.raw.cast(), nstd_core_slice_mut_len(self))
    }
}

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSliceMut slice` - The new slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_new(
    ptr: NSTDAnyMut,
    element_size: NSTDUInt,
    len: NSTDUInt,
) -> NSTDSliceMut {
    NSTDSliceMut {
        ptr: nstd_core_ptr_mut_new(ptr, element_size),
        len,
    }
}

/// Creates an immutable version of a mutable slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The mutable slice.
///
/// # Returns
///
/// `NSTDSlice slice_const` - The immutable copy of `slice`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_as_const(slice: &NSTDSliceMut) -> NSTDSlice {
    let ptr = nstd_core_slice_mut_as_ptr_const(slice);
    let stride = nstd_core_slice_mut_stride(slice);
    nstd_core_slice_new(ptr, stride, slice.len)
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
/// `NSTDAny ptr` - A raw pointer to the slice's memory.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_as_ptr_const(slice: &NSTDSliceMut) -> NSTDAny {
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
/// `NSTDUInt len` - The length of the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_len(slice: &NSTDSliceMut) -> NSTDUInt {
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
/// `NSTDUInt stride` - The size of each value in the slice.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_stride(slice: &NSTDSliceMut) -> NSTDUInt {
    nstd_core_ptr_mut_size(&slice.ptr)
}

/// Returns a pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `NSTDSliceMut *slice` - The slice to read an element from.
///
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the slice's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_get(slice: &mut NSTDSliceMut, pos: NSTDUInt) -> NSTDAnyMut {
    nstd_core_slice_mut_get_const(slice, pos) as NSTDAnyMut
}

/// Returns an immutable pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice to read an element from.
///
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_get_const(slice: &NSTDSliceMut, pos: NSTDUInt) -> NSTDAny {
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
/// `NSTDAny element` - A pointer to the first element in `slice` or `NSTD_NULL` if the
/// slice is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_first_const(slice: &NSTDSliceMut) -> NSTDAny {
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
/// `NSTDAny element` - A pointer to the last element in `slice` or `NSTD_NULL` if the
/// slice is empty.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_last_const(slice: &NSTDSliceMut) -> NSTDAny {
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
/// - `const NSTDSlice *src` - The slice to copy data from.
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
    src: &NSTDSlice,
) -> NSTDErrorCode {
    if dest.len != src.len {
        1
    } else if nstd_core_slice_mut_stride(dest) != nstd_core_slice_stride(src) {
        2
    } else {
        nstd_core_mem_copy(dest.ptr.raw.cast(), src.ptr.raw.cast(), src.byte_len());
        0
    }
}
