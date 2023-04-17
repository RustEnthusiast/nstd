//! A view into a sequence of values in memory.
use super::NSTD_INT_MAX;
use crate::{
    core::{
        mem::nstd_core_mem_copy,
        optional::{gen_optional, NSTDOptional},
        ptr::{
            nstd_core_ptr_get, nstd_core_ptr_mut_get, nstd_core_ptr_mut_get_const,
            nstd_core_ptr_mut_new, nstd_core_ptr_mut_new_unchecked, nstd_core_ptr_mut_size,
            nstd_core_ptr_new, nstd_core_ptr_new_unchecked, nstd_core_ptr_size,
            raw::{nstd_core_ptr_raw_dangling, nstd_core_ptr_raw_dangling_mut},
            NSTDPtr, NSTDPtrMut,
        },
    },
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_NULL,
};
use nstdapi::nstdapi;

/// An immutable view into a sequence of values in memory.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid and unmodified
/// while an instance of this structure is in use.
#[nstdapi]
#[derive(Clone, Copy, Debug)]
pub struct NSTDSlice {
    /// A pointer to the first element in the slice.
    ptr: NSTDPtr,
    /// The number of elements in the slice.
    len: NSTDUInt,
}
impl NSTDSlice {
    /// Creates a new [NSTDSlice] from a Rust slice.
    ///
    /// # Panics
    ///
    /// This operation will panic if `sizeof(T)` is greater than `NSTDInt`'s max value.
    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn from_slice<T>(s: &[T]) -> Self {
        // SAFETY: Rust references are never null.
        unsafe {
            let size = core::mem::size_of::<T>();
            assert!(size <= NSTD_INT_MAX as _);
            let ptr = nstd_core_ptr_new_unchecked(s.as_ptr() as _, size);
            Self { ptr, len: s.len() }
        }
    }

    /// Returns the number of bytes that this slice covers.
    #[inline]
    pub(crate) const fn byte_len(&self) -> usize {
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
    /// - The `NSTDSlice`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    pub(crate) const unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_stride(self) == core::mem::size_of::<T>());
        let ptr = nstd_core_slice_as_ptr(self).cast();
        core::slice::from_raw_parts(ptr, self.len)
    }
}
gen_optional!(NSTDOptionalSlice, NSTDSlice);

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
/// `NSTDOptionalSlice slice` - The new slice on success, or an uninitialized "none" variant if
/// either `ptr` is null or the slice's length in bytes would exceed `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_core_slice_new(
    ptr: NSTDAny,
    element_size: NSTDUInt,
    len: NSTDUInt,
) -> NSTDOptionalSlice {
    if let NSTDOptional::Some(ptr) = nstd_core_ptr_new(ptr, element_size) {
        if len * element_size <= NSTD_INT_MAX as _ {
            return NSTDOptional::Some(NSTDSlice { ptr, len });
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
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSlice slice` - The new slice.
///
/// # Safety
///
/// The user of this function must ensure that `ptr` is non-null, `element_size` does not exceed
/// `NSTDInt`'s max value, and that the slice's total length in bytes will not exceed `NSTDInt`'s
/// max value.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_slice_new_unchecked(
    ptr: NSTDAny,
    element_size: NSTDUInt,
    len: NSTDUInt,
) -> NSTDSlice {
    NSTDSlice {
        ptr: nstd_core_ptr_new_unchecked(ptr, element_size),
        len,
    }
}

/// Creates a new empty slice with a given `element_size`.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// # Returns
///
/// `NSTDSlice slice` - The new empty slice.
///
/// # Panics
///
/// This operation will panic if `element_size` is greater than `NSTDInt`'s max value.
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_empty(element_size: NSTDUInt) -> NSTDSlice {
    assert!(element_size <= NSTD_INT_MAX as _);
    NSTDSlice {
        // SAFETY: The slice is given a non-null pointer and a length of 0.
        ptr: unsafe { nstd_core_ptr_new_unchecked(nstd_core_ptr_raw_dangling(), element_size) },
        len: 0,
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
/// let bytes = "Hello, world!".as_bytes();
/// let bytes_ptr = bytes.as_ptr().cast();
/// let slice = nstd_core_slice_new(bytes_ptr, 1, bytes.len()).unwrap();
/// assert!(nstd_core_slice_as_ptr(&slice) == bytes_ptr);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_as_ptr(slice: &NSTDSlice) -> NSTDAny {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_len, nstd_core_slice_new};
///
/// let bytes = "Goodbye, world!".as_bytes();
/// let len = bytes.len();
/// let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, len).unwrap();
/// assert!(nstd_core_slice_len(&slice) == len);
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
/// let bytes = "Hello, world!".as_bytes();
/// let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, bytes.len()).unwrap();
/// assert!(nstd_core_slice_stride(&slice) == 1);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_stride(slice: &NSTDSlice) -> NSTDUInt {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_get, nstd_core_slice_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
///
/// let numbers: [i32; 3] = [33, 103, 45];
/// let slice = nstd_core_slice_new(numbers.as_ptr().cast(), STRIDE, numbers.len()).unwrap();
///
/// unsafe {
///     assert!(*nstd_core_slice_get(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_get(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_get(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_get(&slice, 3).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_get(slice: &NSTDSlice, mut pos: NSTDUInt) -> NSTDAny {
    if pos < slice.len {
        pos *= nstd_core_slice_stride(slice);
        // SAFETY: We've checked `pos`.
        return unsafe { nstd_core_slice_as_ptr(slice).add(pos) };
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
///
/// let numbers: [u64; 3] = [707, 23043, 8008];
/// let numbers_ptr = numbers.as_ptr().cast();
/// let slice = nstd_core_slice_new(numbers_ptr, STRIDE, numbers.len()).unwrap();
/// let empty = nstd_core_slice_new(numbers_ptr, STRIDE, 0).unwrap();
///
/// unsafe {
///     assert!(nstd_core_slice_first(&slice) == numbers_ptr);
///     assert!(*nstd_core_slice_first(&slice).cast::<u64>() == 707);
///     assert!(nstd_core_slice_first(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_first(slice: &NSTDSlice) -> NSTDAny {
    match slice.len > 0 {
        true => nstd_core_slice_as_ptr(slice),
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
///
/// let numbers: [u64; 3] = [717, 421, 4317];
/// let numbers_ptr = numbers.as_ptr().cast();
/// let slice = nstd_core_slice_new(numbers_ptr, STRIDE, numbers.len()).unwrap();
/// let empty = nstd_core_slice_new(numbers_ptr, STRIDE, 0).unwrap();
///
/// unsafe {
///     assert!(*nstd_core_slice_last(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_last(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_last(slice: &NSTDSlice) -> NSTDAny {
    match slice.len > 0 {
        true => nstd_core_slice_get(slice, slice.len - 1),
        false => NSTD_NULL,
    }
}

/// A view into a sequence of values in memory.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid, unmodified, and
/// unreferenced in any other code while an instance of this structure is in use, else data races
/// may occur.
#[nstdapi]
#[derive(Debug)]
pub struct NSTDSliceMut {
    /// A pointer to the first element in the slice.
    ptr: NSTDPtrMut,
    /// The number of elements in the slice.
    len: NSTDUInt,
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
    /// - The `NSTDSliceMut`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    pub(crate) const unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_mut_stride(self) == core::mem::size_of::<T>());
        let ptr = nstd_core_slice_mut_as_ptr_const(self).cast();
        core::slice::from_raw_parts(ptr, self.len)
    }

    /// Creates a mutable Rust byte slice from this `NSTDSliceMut`.
    ///
    /// # Panics
    ///
    /// This operation will panic if `size_of::<T>()` does not match the slice's stride.
    ///
    /// # Safety
    ///
    /// - The `NSTDSliceMut`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn as_slice_mut<T>(&mut self) -> &mut [T] {
        assert!(nstd_core_slice_mut_stride(self) == core::mem::size_of::<T>());
        let ptr = nstd_core_slice_mut_as_ptr(self).cast();
        core::slice::from_raw_parts_mut(ptr, self.len)
    }
}
gen_optional!(NSTDOptionalSliceMut, NSTDSliceMut);

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
/// `NSTDOptionalSliceMut slice` - The new slice on success, or an uninitialized "none" variant if
/// either `ptr` is null or the slice's length in bytes would exceed `NSTDInt`'s max value.
#[nstdapi]
pub fn nstd_core_slice_mut_new(
    ptr: NSTDAnyMut,
    element_size: NSTDUInt,
    len: NSTDUInt,
) -> NSTDOptionalSliceMut {
    if let NSTDOptional::Some(ptr) = nstd_core_ptr_mut_new(ptr, element_size) {
        if len * element_size <= NSTD_INT_MAX as _ {
            return NSTDOptional::Some(NSTDSliceMut { ptr, len });
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
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSliceMut slice` - The new slice.
///
/// # Safety
///
/// The user of this function must ensure that `ptr` is non-null, `element_size` does not exceed
/// `NSTDInt`'s max value, and that the slice's total length in bytes will not exceed `NSTDInt`'s
/// max value.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_slice_mut_new_unchecked(
    ptr: NSTDAnyMut,
    element_size: NSTDUInt,
    len: NSTDUInt,
) -> NSTDSliceMut {
    NSTDSliceMut {
        ptr: nstd_core_ptr_mut_new_unchecked(ptr, element_size),
        len,
    }
}

/// Creates a new empty slice with a given `element_size`.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// # Returns
///
/// `NSTDSliceMut slice` - The new empty slice.
///
/// # Panics
///
/// This operation will panic if `element_size` is greater than `NSTDInt`'s max value.
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_empty(element_size: NSTDUInt) -> NSTDSliceMut {
    assert!(element_size <= NSTD_INT_MAX as _);
    NSTDSliceMut {
        // SAFETY: The slice is given a non-null pointer and a length of 0.
        ptr: unsafe {
            nstd_core_ptr_mut_new_unchecked(nstd_core_ptr_raw_dangling_mut(), element_size)
        },
        len: 0,
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
    unsafe { nstd_core_slice_new_unchecked(ptr, stride, slice.len) }
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
///
/// let mut buf: [u16; 3] = [3, 5, 7];
/// let mut slice = nstd_core_slice_mut_new(buf.as_mut_ptr().cast(), STRIDE, buf.len()).unwrap();
///
/// unsafe {
///     *nstd_core_slice_mut_as_ptr(&mut slice).cast::<u16>() = 1;
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<u16>() == 1);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_as_ptr(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_as_ptr_const, nstd_core_slice_mut_new};
///
/// let mut m33 = String::from("33marrow");
/// let raw_ptr = m33.as_mut_ptr().cast();
/// let slice = nstd_core_slice_mut_new(raw_ptr, 1, m33.len()).unwrap();
/// assert!(nstd_core_slice_mut_as_ptr_const(&slice) == raw_ptr);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_as_ptr_const(slice: &NSTDSliceMut) -> NSTDAny {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_len, nstd_core_slice_mut_new};
///
/// let mut bye = String::from("Goodbye, cruel world!");
/// let len = bye.len();
/// let slice = nstd_core_slice_mut_new(bye.as_mut_ptr().cast(), 1, len).unwrap();
/// assert!(nstd_core_slice_mut_len(&slice) == len);
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
/// let mut hw = String::from("Hello, world!");
/// let slice = nstd_core_slice_mut_new(hw.as_mut_ptr().cast(), 1, hw.len()).unwrap();
/// assert!(nstd_core_slice_mut_stride(&slice) == 1);
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_stride(slice: &NSTDSliceMut) -> NSTDUInt {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_get, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
///
/// let mut numbers = [0i32; 3];
/// let ptr = numbers.as_mut_ptr().cast();
/// let mut slice = nstd_core_slice_mut_new(ptr, STRIDE, numbers.len()).unwrap();
///
/// unsafe {
///     *nstd_core_slice_mut_get(&mut slice, 0).cast::<i32>() = 33;
///     *nstd_core_slice_mut_get(&mut slice, 1).cast::<i32>() = 103;
///     *nstd_core_slice_mut_get(&mut slice, 2).cast::<i32>() = 45;
///     assert!(numbers == [33, 103, 45]);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_get(slice: &mut NSTDSliceMut, pos: NSTDUInt) -> NSTDAnyMut {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_get_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
///
/// let mut numbers: [i32; 3] = [33, 103, 45];
/// let ptr = numbers.as_mut_ptr().cast();
/// let slice = nstd_core_slice_mut_new(ptr, STRIDE, numbers.len()).unwrap();
///
/// unsafe {
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_mut_get_const(&slice, 3).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_get_const(slice: &NSTDSliceMut, mut pos: NSTDUInt) -> NSTDAny {
    if pos < slice.len {
        pos *= nstd_core_slice_mut_stride(slice);
        // SAFETY: We've checked `pos`.
        return unsafe { nstd_core_slice_mut_as_ptr_const(slice).add(pos) };
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
///
/// let mut numbers: [u64; 3] = [707, 23043, 8008];
/// let ptr = numbers.as_mut_ptr().cast();
/// let mut slice = nstd_core_slice_mut_new(ptr, STRIDE, numbers.len()).unwrap();
///
/// unsafe { *nstd_core_slice_mut_first(&mut slice).cast::<u64>() = 101 };
/// assert!(numbers[0] == 101);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_first(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_first_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
///
/// let mut numbers: [u64; 3] = [707, 23043, 8008];
/// let numbers_ptr = numbers.as_mut_ptr().cast();
/// let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, numbers.len()).unwrap();
/// let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, 0).unwrap();
///
/// unsafe {
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
///
/// let mut numbers: [u64; 3] = [717, 421, 4317];
/// let ptr = numbers.as_mut_ptr().cast();
/// let mut slice = nstd_core_slice_mut_new(ptr, STRIDE, numbers.len()).unwrap();
///
/// unsafe { *nstd_core_slice_mut_last(&mut slice).cast::<u64>() = 1738 };
/// assert!(numbers[2] == 1738);
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_slice_mut_last(slice: &mut NSTDSliceMut) -> NSTDAnyMut {
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_last_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
///
/// let mut numbers: [u64; 3] = [717, 421, 4317];
/// let numbers_ptr = numbers.as_mut_ptr().cast();
/// let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, numbers.len()).unwrap();
/// let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, 0).unwrap();
///
/// unsafe {
///     assert!(*nstd_core_slice_mut_last_const(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_mut_last_const(&empty).is_null());
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_slice_mut_last_const(slice: &NSTDSliceMut) -> NSTDAny {
    match slice.len > 0 {
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
///
/// let mut dest_arr = [0u32; 5];
/// let src_arr: [u32; 5] = [7, 43, 32, 90, 15];
///
/// let ptr = dest_arr.as_mut_ptr().cast();
/// let mut dest = nstd_core_slice_mut_new(ptr, STRIDE, dest_arr.len()).unwrap();
/// let src = nstd_core_slice_new(src_arr.as_ptr().cast(), STRIDE, src_arr.len()).unwrap();
///
/// unsafe { nstd_core_slice_mut_copy(&mut dest, &src) };
/// assert!(dest_arr == src_arr);
/// ```
#[nstdapi]
pub unsafe fn nstd_core_slice_mut_copy(dest: &mut NSTDSliceMut, src: &NSTDSlice) {
    assert!(dest.len == src.len && nstd_core_slice_mut_stride(dest) == nstd_core_slice_stride(src));
    let len = src.byte_len();
    let dest = nstd_core_slice_mut_as_ptr(dest) as _;
    let src = nstd_core_slice_as_ptr(src) as _;
    nstd_core_mem_copy(dest, src, len);
}
