//! A view into a sequence of values in memory.
use crate::{
    core::{
        mem::{nstd_core_mem_copy, nstd_core_mem_is_aligned},
        optional::{gen_optional, NSTDOptional},
    },
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_INT_MAX, NSTD_NULL,
};
use nstdapi::nstdapi;

/// An immutable view into a sequence of values in memory.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDSlice {
    /// A pointer to the first element in the slice.
    ptr: NSTDAny,
    /// The number of elements in the slice.
    len: NSTDUInt,
    /// The slice's stride.
    stride: NSTDUInt,
    /// The slice's align.
    align: NSTDUInt,
}
impl NSTDSlice {
    /// Creates a new [`NSTDSlice`] from a Rust slice.
    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn from_slice<T>(s: &[T]) -> Self {
        Self {
            ptr: s.as_ptr().cast(),
            len: s.len(),
            stride: core::mem::size_of::<T>(),
            align: core::mem::align_of::<T>(),
        }
    }

    /// Returns the number of bytes that this slice covers.
    #[inline]
    #[allow(clippy::arithmetic_side_effects)]
    pub(crate) const fn byte_len(&self) -> usize {
        self.len * self.stride
    }

    /// Creates a Rust slice from this `NSTDSlice`.
    ///
    /// Returns [`None`] if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// - The `NSTDSlice`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    pub(crate) const unsafe fn as_slice<T>(&self) -> Option<&[T]> {
        match self.stride == core::mem::size_of::<T>() {
            true => Some(core::slice::from_raw_parts(self.ptr.cast(), self.len)),
            false => None,
        }
    }
}
gen_optional!(NSTDOptionalSlice, NSTDSlice);

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt stride` - The number of bytes each element occupies.
///
/// - `NSTDUInt align` - The alignment of each element in the slice.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDOptionalSlice slice` - The new slice on success, or an uninitialized "none" variant if
/// `ptr` is null, `align` is not a power of two, `stride` is not a multiple of `align`, `ptr` is
/// not a multiple of `align`, or the slice's length in bytes would exceed `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_core_slice_new(
    ptr: NSTDAny,
    stride: NSTDUInt,
    align: NSTDUInt,
    len: NSTDUInt,
) -> NSTDOptionalSlice {
    if let Some(size) = len.checked_mul(stride) {
        #[allow(clippy::arithmetic_side_effects)]
        if size <= NSTD_INT_MAX
            && crate::core::mem::is_power_of_two(align)
            && stride % align == 0
            && !ptr.is_null()
            && nstd_core_mem_is_aligned(ptr, align)
        {
            return NSTDOptional::Some(NSTDSlice {
                ptr,
                len,
                stride,
                align,
            });
        }
    }
    NSTDOptional::None
}

/// Creates a new slice from raw data without checking if `ptr` is null.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt stride` - The number of bytes each element occupies.
///
/// - `NSTDUInt align` - The alignment of each element in the slice.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSlice slice` - The new slice.
///
/// # Safety
///
/// - `ptr` must be non-null.
///
/// - `align` must be a nonzero power of two.
///
/// - `stride` must be a multiple of `align`.
///
/// - `ptr` must be a multiple of `align`.
///
/// - The slice's total length in bytes will not exceed `NSTDInt`'s max value.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_slice_new_unchecked(
    ptr: NSTDAny,
    stride: NSTDUInt,
    align: NSTDUInt,
    len: NSTDUInt,
) -> NSTDSlice {
    NSTDSlice {
        ptr,
        len,
        stride,
        align,
    }
}

/// Creates a new empty slice with a given `stride`.
///
/// # Parameters:
///
/// - `NSTDUInt stride` - The number of bytes each element occupies.
///
/// - `NSTDUInt align` - The alignment of each element in the slice.
///
/// # Returns
///
/// `NSTDSlice slice` - The new empty slice.
///
/// # Panics
///
/// This operation will panic if either `align` is not a power of two or `stride` is not a multiple
/// of `align`.
#[inline]
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub const fn nstd_core_slice_empty(stride: NSTDUInt, align: NSTDUInt) -> NSTDSlice {
    assert!(crate::core::mem::is_power_of_two(align) && stride % align == 0);
    NSTDSlice {
        ptr: align as NSTDAny,
        len: 0,
        stride,
        align,
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_as_ptr, nstd_core_slice_new};
///
/// unsafe {
///     let bytes = "Hello, world!".as_bytes();
///     let bytes_ptr = bytes.as_ptr().cast();
///     let slice = nstd_core_slice_new(bytes_ptr, 1, 1, bytes.len()).unwrap();
///     assert!(nstd_core_slice_as_ptr(&slice) == bytes_ptr);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_as_ptr(slice: &NSTDSlice) -> NSTDAny {
    slice.ptr
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_len, nstd_core_slice_new};
///
/// unsafe {
///     let bytes = "Goodbye, world!".as_bytes();
///     let len = bytes.len();
///     let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, 1, len).unwrap();
///     assert!(nstd_core_slice_len(&slice) == len);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_len(slice: &NSTDSlice) -> NSTDUInt {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_stride, nstd_core_slice_new};
///
/// unsafe {
///     let bytes = "Hello, world!".as_bytes();
///     let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, 1, bytes.len()).unwrap();
///     assert!(nstd_core_slice_stride(&slice) == 1);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_stride(slice: &NSTDSlice) -> NSTDUInt {
    slice.stride
}

/// Returns the alignment of each value in a slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt align` - The alignment of each value in the slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_align, nstd_core_slice_new};
///
/// unsafe {
///     let bytes = "Hello, world!".as_bytes();
///     let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, 1, bytes.len()).unwrap();
///     assert!(nstd_core_slice_align(&slice) == 1);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_align(slice: &NSTDSlice) -> NSTDUInt {
    slice.align
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_get, nstd_core_slice_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
/// const ALIGN: usize = core::mem::align_of::<i32>();
///
/// unsafe {
///     let numbers: [i32; 3] = [33, 103, 45];
///     let slice =
///         nstd_core_slice_new(numbers.as_ptr().cast(), STRIDE, ALIGN, numbers.len()).unwrap();
///
///     assert!(*nstd_core_slice_get(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_get(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_get(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_get(&slice, 3).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_get(slice: &NSTDSlice, mut pos: NSTDUInt) -> NSTDAny {
    #[allow(clippy::arithmetic_side_effects)]
    if pos < slice.len {
        pos *= slice.stride;
        // SAFETY: We've checked `pos`.
        return unsafe { slice.ptr.add(pos) };
    }
    NSTD_NULL
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_first, nstd_core_slice_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
/// const ALIGN: usize = core::mem::align_of::<u64>();
///
/// unsafe {
///     let numbers: [u64; 3] = [707, 23043, 8008];
///     let numbers_ptr = numbers.as_ptr().cast();
///     let slice = nstd_core_slice_new(numbers_ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///     let empty = nstd_core_slice_new(numbers_ptr, STRIDE, ALIGN, 0).unwrap();
///
///     assert!(nstd_core_slice_first(&slice) == numbers_ptr);
///     assert!(*nstd_core_slice_first(&slice).cast::<u64>() == 707);
///     assert!(nstd_core_slice_first(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_first(slice: &NSTDSlice) -> NSTDAny {
    match slice.len > 0 {
        true => slice.ptr,
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_last, nstd_core_slice_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
/// const ALIGN: usize = core::mem::align_of::<u64>();
///
/// unsafe {
///     let numbers: [u64; 3] = [717, 421, 4317];
///     let numbers_ptr = numbers.as_ptr().cast();
///     let slice = nstd_core_slice_new(numbers_ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///     let empty = nstd_core_slice_new(numbers_ptr, STRIDE, ALIGN, 0).unwrap();
///
///     assert!(*nstd_core_slice_last(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_last(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_last(slice: &NSTDSlice) -> NSTDAny {
    match slice.len > 0 {
        #[allow(clippy::arithmetic_side_effects)]
        true => nstd_core_slice_get(slice, slice.len - 1),
        false => NSTD_NULL,
    }
}

/// A view into a sequence of values in memory.
#[nstdapi]
pub struct NSTDSliceMut {
    /// A pointer to the first element in the slice.
    ptr: NSTDAnyMut,
    /// The number of elements in the slice.
    len: NSTDUInt,
    /// The slice's stride.
    stride: NSTDUInt,
    /// The slice's align.
    align: NSTDUInt,
}
impl NSTDSliceMut {
    /// Creates a mutable Rust slice from this `NSTDSliceMut`.
    ///
    /// Returns [`None`] if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// - The `NSTDSliceMut`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_slice_mut<T>(&mut self) -> Option<&mut [T]> {
        match self.stride == core::mem::size_of::<T>() {
            true => Some(core::slice::from_raw_parts_mut(self.ptr.cast(), self.len)),
            false => None,
        }
    }
}
gen_optional!(NSTDOptionalSliceMut, NSTDSliceMut);

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt stride` - The number of bytes each element occupies.
///
/// - `NSTDUInt align` - The alignment of each element in the slice.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDOptionalSliceMut slice` - The new slice on success, or an uninitialized "none" variant if
/// `ptr` is null, `align` is not a power of two, `stride` is not a multiple of `align`, `ptr` is
/// not a multiple of `align`, or the slice's length in bytes would exceed `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_core_slice_mut_new(
    ptr: NSTDAnyMut,
    stride: NSTDUInt,
    align: NSTDUInt,
    len: NSTDUInt,
) -> NSTDOptionalSliceMut {
    if let Some(size) = len.checked_mul(stride) {
        #[allow(clippy::arithmetic_side_effects)]
        if size <= NSTD_INT_MAX
            && crate::core::mem::is_power_of_two(align)
            && stride % align == 0
            && !ptr.is_null()
            && nstd_core_mem_is_aligned(ptr, align)
        {
            return NSTDOptional::Some(NSTDSliceMut {
                ptr,
                len,
                stride,
                align,
            });
        }
    }
    NSTDOptional::None
}

/// Creates a new slice from raw data without checking if `ptr` is null.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt stride` - The number of bytes each element occupies.
///
/// - `NSTDUInt align` - The alignment of each element in the slice.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSliceMut slice` - The new slice.
///
/// # Safety
///
/// - `ptr` must be non-null.
///
/// - `align` must be a nonzero power of two.
///
/// - `stride` must be a multiple of `align`.
///
/// - `ptr` must be a multiple of `align`.
///
/// - The slice's total length in bytes will not exceed `NSTDInt`'s max value.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_slice_mut_new_unchecked(
    ptr: NSTDAnyMut,
    stride: NSTDUInt,
    align: NSTDUInt,
    len: NSTDUInt,
) -> NSTDSliceMut {
    NSTDSliceMut {
        ptr,
        len,
        stride,
        align,
    }
}

/// Creates a new empty slice with a given `stride`.
///
/// # Parameters:
///
/// - `NSTDUInt stride` - The number of bytes each element occupies.
///
/// - `NSTDUInt align` - The alignment of each element in the slice.
///
/// # Returns
///
/// `NSTDSliceMut slice` - The new empty slice.
///
/// # Panics
///
/// This operation will panic if either `align` is not a power of two or `stride` is not a multiple
/// of `align`.
#[inline]
#[nstdapi]
#[allow(clippy::arithmetic_side_effects)]
pub const fn nstd_core_slice_mut_empty(stride: NSTDUInt, align: NSTDUInt) -> NSTDSliceMut {
    assert!(crate::core::mem::is_power_of_two(align) && stride % align == 0);
    NSTDSliceMut {
        ptr: align as NSTDAnyMut,
        len: 0,
        stride,
        align,
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
#[nstdapi]
pub const fn nstd_core_slice_mut_as_const(slice: &NSTDSliceMut) -> NSTDSlice {
    let ptr = nstd_core_slice_mut_as_ptr_const(slice);
    let stride = nstd_core_slice_mut_stride(slice);
    // SAFETY: `ptr` is never null.
    unsafe { nstd_core_slice_new_unchecked(ptr, stride, slice.align, slice.len) }
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{
///     nstd_core_slice_mut_as_ptr, nstd_core_slice_mut_get_const, nstd_core_slice_mut_new,
/// };
///
/// const STRIDE: usize = core::mem::size_of::<u16>();
/// const ALIGN: usize = core::mem::align_of::<u16>();
///
/// unsafe {
///     let mut buf: [u16; 3] = [3, 5, 7];
///     let ptr = buf.as_mut_ptr().cast();
///     let mut slice = nstd_core_slice_mut_new(ptr, STRIDE, ALIGN, buf.len()).unwrap();
///
///     *nstd_core_slice_mut_as_ptr(&mut slice).cast::<u16>() = 1;
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<u16>() == 1);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_as_ptr(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
    slice.ptr
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_as_ptr_const, nstd_core_slice_mut_new};
///
/// unsafe {
///     let mut m33 = String::from("33marrow");
///     let raw_ptr = m33.as_mut_ptr().cast();
///     let slice = nstd_core_slice_mut_new(raw_ptr, 1, 1, m33.len()).unwrap();
///     assert!(nstd_core_slice_mut_as_ptr_const(&slice) == raw_ptr);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_as_ptr_const(slice: &NSTDSliceMut) -> NSTDAny {
    slice.ptr
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_len, nstd_core_slice_mut_new};
///
/// unsafe {
///     let mut bye = String::from("Goodbye, cruel world!");
///     let len = bye.len();
///     let slice = nstd_core_slice_mut_new(bye.as_mut_ptr().cast(), 1, 1, len).unwrap();
///     assert!(nstd_core_slice_mut_len(&slice) == len);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_len(slice: &NSTDSliceMut) -> NSTDUInt {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_stride, nstd_core_slice_mut_new};
///
/// unsafe {
///     let mut hw = String::from("Hello, world!");
///     let slice = nstd_core_slice_mut_new(hw.as_mut_ptr().cast(), 1, 1, hw.len()).unwrap();
///     assert!(nstd_core_slice_mut_stride(&slice) == 1);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_stride(slice: &NSTDSliceMut) -> NSTDUInt {
    slice.stride
}

/// Returns the alignment of each value in a slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt align` - The alignment of each value in the slice.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_align, nstd_core_slice_mut_new};
///
/// unsafe {
///     let mut bytes = b"Hello, world!".to_vec();
///     let slice = nstd_core_slice_mut_new(bytes.as_mut_ptr().cast(), 1, 1, bytes.len()).unwrap();
///     assert!(nstd_core_slice_mut_align(&slice) == 1);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_align(slice: &NSTDSliceMut) -> NSTDUInt {
    slice.align
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_get, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
/// const ALIGN: usize = core::mem::align_of::<i32>();
///
/// unsafe {
///     let mut numbers = [0i32; 3];
///     let ptr = numbers.as_mut_ptr().cast();
///     let mut slice = nstd_core_slice_mut_new(ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///
///     *nstd_core_slice_mut_get(&mut slice, 0).cast::<i32>() = 33;
///     *nstd_core_slice_mut_get(&mut slice, 1).cast::<i32>() = 103;
///     *nstd_core_slice_mut_get(&mut slice, 2).cast::<i32>() = 45;
///     assert!(numbers == [33, 103, 45]);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_get(slice: &mut NSTDSliceMut, pos: NSTDUInt) -> NSTDAnyMut {
    nstd_core_slice_mut_get_const(slice, pos).cast_mut()
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_get_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
/// const ALIGN: usize = core::mem::align_of::<i32>();
///
/// unsafe {
///     let mut numbers: [i32; 3] = [33, 103, 45];
///     let ptr = numbers.as_mut_ptr().cast();
///     let slice = nstd_core_slice_mut_new(ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_mut_get_const(&slice, 3).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_get_const(slice: &NSTDSliceMut, mut pos: NSTDUInt) -> NSTDAny {
    #[allow(clippy::arithmetic_side_effects)]
    if pos < slice.len {
        pos *= slice.stride;
        // SAFETY: We've checked `pos`.
        return unsafe { slice.ptr.add(pos) };
    }
    NSTD_NULL
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_first, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
/// const ALIGN: usize = core::mem::align_of::<u64>();
///
/// let mut numbers: [u64; 3] = [707, 23043, 8008];
/// let ptr = numbers.as_mut_ptr().cast();
/// let mut slice = unsafe { nstd_core_slice_mut_new(ptr, STRIDE, ALIGN, numbers.len()).unwrap() };
///
/// unsafe { *nstd_core_slice_mut_first(&mut slice).cast::<u64>() = 101 };
/// assert!(numbers[0] == 101);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_first(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
    nstd_core_slice_mut_first_const(slice).cast_mut()
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_first_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
/// const ALIGN: usize = core::mem::align_of::<u64>();
///
/// unsafe {
///     let mut numbers: [u64; 3] = [707, 23043, 8008];
///     let numbers_ptr = numbers.as_mut_ptr().cast();
///     let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///     let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, ALIGN, 0).unwrap();
///
///     assert!(nstd_core_slice_mut_first_const(&slice) == numbers_ptr);
///     assert!(*nstd_core_slice_mut_first_const(&slice).cast::<u64>() == 707);
///     assert!(nstd_core_slice_mut_first_const(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_first_const(slice: &NSTDSliceMut) -> NSTDAny {
    match slice.len > 0 {
        true => nstd_core_slice_mut_as_ptr_const(slice),
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_last, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
/// const ALIGN: usize = core::mem::align_of::<u64>();
///
/// unsafe {
///     let mut numbers: [u64; 3] = [717, 421, 4317];
///     let ptr = numbers.as_mut_ptr().cast();
///     let mut slice = nstd_core_slice_mut_new(ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///
///     *nstd_core_slice_mut_last(&mut slice).cast::<u64>() = 1738;
///     assert!(numbers[2] == 1738);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_last(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
    nstd_core_slice_mut_last_const(slice).cast_mut()
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_last_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
/// const ALIGN: usize = core::mem::align_of::<u64>();
///
/// unsafe {
///     let mut numbers: [u64; 3] = [717, 421, 4317];
///     let numbers_ptr = numbers.as_mut_ptr().cast();
///     let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, ALIGN, numbers.len()).unwrap();
///     let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, ALIGN, 0).unwrap();
///
///     assert!(*nstd_core_slice_mut_last_const(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_mut_last_const(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_last_const(slice: &NSTDSliceMut) -> NSTDAny {
    match slice.len > 0 {
        #[allow(clippy::arithmetic_side_effects)]
        true => nstd_core_slice_mut_get_const(slice, slice.len - 1),
        false => NSTD_NULL,
    }
}

/// Copies data into `dest` from `src`. The number of bytes copied is determined by `src`.
///
/// # Parameters:
///
/// - `NSTDSliceMut *dest` - The slice to copy data to.
///
/// - `const NSTDSlice *src` - The slice to copy data from.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - The two buffer's lengths do not match.
///
/// - The two buffer's strides do not match.
///
/// # Safety
///
/// This function can cause undefined behavior if either `dest` or `src`'s data is invalid.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{
///     nstd_core_slice_mut_copy, nstd_core_slice_mut_new, nstd_core_slice_new,
/// };
///
/// const STRIDE: usize = core::mem::size_of::<u32>();
/// const ALIGN: usize = core::mem::align_of::<u32>();
///
/// let mut dest_arr = [0u32; 5];
/// let src_arr: [u32; 5] = [7, 43, 32, 90, 15];
///
/// unsafe {
///     let ptr = dest_arr.as_mut_ptr().cast();
///     let mut dest = nstd_core_slice_mut_new(ptr, STRIDE, ALIGN, dest_arr.len()).unwrap();
///     let src =
///         nstd_core_slice_new(src_arr.as_ptr().cast(), STRIDE, ALIGN, src_arr.len()).unwrap();
///
///     nstd_core_slice_mut_copy(&mut dest, &src);
///     assert!(dest_arr == src_arr);
/// }
/// ```
#[nstdapi]
pub unsafe fn nstd_core_slice_mut_copy(dest: &mut NSTDSliceMut, src: &NSTDSlice) {
    assert!(dest.len == src.len && dest.stride == src.stride);
    let len = src.byte_len();
    let dest = nstd_core_slice_mut_as_ptr(dest).cast();
    let src = nstd_core_slice_as_ptr(src).cast();
    nstd_core_mem_copy(dest, src, len);
}
