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
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_NULL,
};

/// An immutable view into a sequence of values in memory.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid and unmodified
/// while an instance of this structure is in use.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NSTDSlice {
    /// A pointer to the first element in the slice.
    ptr: NSTDPtr,
    /// The number of elements in the slice.
    len: NSTDUInt,
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
    /// This operation will panic in the following situations:
    ///
    /// - `size_of::<T>()` does not match the slice's stride.
    ///
    /// - The number of bytes the slice contains is greater than `NSTDInt`'s maximum value.
    ///
    /// # Safety
    ///
    /// - The `NSTDSlice`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_stride(self) == core::mem::size_of::<T>());
        assert!(self.byte_len() <= isize::MAX as usize);
        let ptr = nstd_core_slice_as_ptr(self).cast();
        core::slice::from_raw_parts(ptr, self.len)
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_as_ptr, nstd_core_slice_new};
///
/// let bytes = "Hello, world!".as_bytes();
/// let bytes_ptr = bytes.as_ptr().cast();
/// let slice = nstd_core_slice_new(bytes_ptr, 1, bytes.len());
/// assert!(nstd_core_slice_as_ptr(&slice) == bytes_ptr);
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_len, nstd_core_slice_new};
///
/// let bytes = "Goodbye, world!".as_bytes();
/// let len = bytes.len();
/// let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, len);
/// assert!(nstd_core_slice_len(&slice) == len);
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_stride, nstd_core_slice_new};
///
/// let bytes = "Hello, world!".as_bytes();
/// let slice = nstd_core_slice_new(bytes.as_ptr().cast(), 1, bytes.len());
/// assert!(nstd_core_slice_stride(&slice) == 1);
/// ```
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
///
/// # Panics
///
/// Panics if the slice's current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_get, nstd_core_slice_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
///
/// let numbers: [i32; 3] = [33, 103, 45];
/// let slice = nstd_core_slice_new(numbers.as_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe {
///     assert!(*nstd_core_slice_get(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_get(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_get(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_get(&slice, 3).is_null());
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_get(slice: &NSTDSlice, mut pos: NSTDUInt) -> NSTDAny {
    if pos < slice.len {
        pos *= nstd_core_slice_stride(slice);
        assert!(pos <= isize::MAX as usize);
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
/// let slice = nstd_core_slice_new(numbers_ptr, STRIDE, numbers.len());
/// let empty = nstd_core_slice_new(numbers_ptr, STRIDE, 0);
///
/// unsafe {
///     assert!(nstd_core_slice_first(&slice) == numbers_ptr);
///     assert!(*nstd_core_slice_first(&slice).cast::<u64>() == 707);
///     assert!(nstd_core_slice_first(&empty).is_null());
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_first(slice: &NSTDSlice) -> NSTDAny {
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
/// # Panics
///
/// Panics if the slice's current length in bytes exceeds `NSTDInt`'s max value.
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
/// let slice = nstd_core_slice_new(numbers_ptr, STRIDE, numbers.len());
/// let empty = nstd_core_slice_new(numbers_ptr, STRIDE, 0);
///
/// unsafe {
///     assert!(*nstd_core_slice_last(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_last(&empty).is_null());
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_last(slice: &NSTDSlice) -> NSTDAny {
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
#[repr(C)]
#[derive(Debug)]
pub struct NSTDSliceMut {
    /// A pointer to the first element in the slice.
    ptr: NSTDPtrMut,
    /// The number of elements in the slice.
    len: NSTDUInt,
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
    /// This operation will panic in the following situations:
    ///
    /// - `size_of::<T>()` does not match the slice's stride.
    ///
    /// - The number of bytes the slice contains is greater than `NSTDInt`'s maximum value.
    ///
    /// # Safety
    ///
    /// - The `NSTDSliceMut`'s data must remain valid and unmodified while the returned slice is in
    /// use.
    ///
    /// - The slice's data must be properly aligned.
    #[inline]
    pub(crate) unsafe fn as_slice<T>(&self) -> &[T] {
        assert!(nstd_core_slice_mut_stride(self) == core::mem::size_of::<T>());
        assert!(self.byte_len() <= isize::MAX as usize);
        let ptr = nstd_core_slice_mut_as_ptr_const(self).cast();
        core::slice::from_raw_parts(ptr, self.len)
    }

    /// Creates a mutable Rust byte slice from this `NSTDSliceMut`.
    ///
    /// # Panics
    ///
    /// This operation will panic in the following situations:
    ///
    /// - `size_of::<T>()` does not match the slice's stride.
    ///
    /// - The number of bytes the slice contains is greater than `NSTDInt`'s maximum value.
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
        assert!(self.byte_len() <= isize::MAX as usize);
        let ptr = nstd_core_slice_mut_as_ptr(self).cast();
        core::slice::from_raw_parts_mut(ptr, self.len)
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
/// let mut slice = nstd_core_slice_mut_new(buf.as_mut_ptr().cast(), STRIDE, buf.len());
///
/// unsafe {
///     *nstd_core_slice_mut_as_ptr(&mut slice).cast::<u16>() = 1;
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<u16>() == 1);
/// }
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_as_ptr_const, nstd_core_slice_mut_new};
///
/// let mut m33 = String::from("33marrow");
/// let raw_ptr = m33.as_mut_ptr().cast();
/// let slice = nstd_core_slice_mut_new(raw_ptr, 1, m33.len());
/// assert!(nstd_core_slice_mut_as_ptr_const(&slice) == raw_ptr);
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_len, nstd_core_slice_mut_new};
///
/// let mut bye = String::from("Goodbye, cruel world!");
/// let len = bye.len();
/// let slice = nstd_core_slice_mut_new(bye.as_mut_ptr().cast(), 1, len);
/// assert!(nstd_core_slice_mut_len(&slice) == len);
/// ```
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
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_stride, nstd_core_slice_mut_new};
///
/// let mut hw = String::from("Hello, world!");
/// let slice = nstd_core_slice_mut_new(hw.as_mut_ptr().cast(), 1, hw.len());
/// assert!(nstd_core_slice_mut_stride(&slice) == 1);
/// ```
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
///
/// # Panics
///
/// Panics if the slice's current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_get, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
///
/// let mut numbers = [0i32; 3];
/// let mut slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe {
///     *nstd_core_slice_mut_get(&mut slice, 0).cast::<i32>() = 33;
///     *nstd_core_slice_mut_get(&mut slice, 1).cast::<i32>() = 103;
///     *nstd_core_slice_mut_get(&mut slice, 2).cast::<i32>() = 45;
///     assert!(numbers == [33, 103, 45]);
/// }
/// ```
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
///
/// # Panics
///
/// Panics if the slice's current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_get_const, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<i32>();
///
/// let mut numbers: [i32; 3] = [33, 103, 45];
/// let slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe {
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_mut_get_const(&slice, 3).is_null());
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_get_const(
    slice: &NSTDSliceMut,
    mut pos: NSTDUInt,
) -> NSTDAny {
    if pos < slice.len {
        pos *= nstd_core_slice_mut_stride(slice);
        assert!(pos <= isize::MAX as usize);
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
/// let mut slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe { *nstd_core_slice_mut_first(&mut slice).cast::<u64>() = 101 };
/// assert!(numbers[0] == 101);
/// ```
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
/// let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, numbers.len());
/// let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, 0);
///
/// unsafe {
///     assert!(nstd_core_slice_mut_first_const(&slice) == numbers_ptr);
///     assert!(*nstd_core_slice_mut_first_const(&slice).cast::<u64>() == 707);
///     assert!(nstd_core_slice_mut_first_const(&empty).is_null());
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_first_const(slice: &NSTDSliceMut) -> NSTDAny {
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
/// # Panics
///
/// Panics if the slice's current length in bytes exceeds `NSTDInt`'s max value.
///
/// # Example
///
/// ```
/// use nstd_sys::core::slice::{nstd_core_slice_mut_last, nstd_core_slice_mut_new};
///
/// const STRIDE: usize = core::mem::size_of::<u64>();
///
/// let mut numbers: [u64; 3] = [717, 421, 4317];
/// let mut slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe { *nstd_core_slice_mut_last(&mut slice).cast::<u64>() = 1738 };
/// assert!(numbers[2] == 1738);
/// ```
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
///
/// # Panics
///
/// Panics if the slice's current length in bytes exceeds `NSTDInt`'s max value.
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
/// let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, numbers.len());
/// let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, 0);
///
/// unsafe {
///     assert!(*nstd_core_slice_mut_last_const(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_mut_last_const(&empty).is_null());
/// }
/// ```
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_slice_mut_last_const(slice: &NSTDSliceMut) -> NSTDAny {
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
/// let mut dest = nstd_core_slice_mut_new(dest_arr.as_mut_ptr().cast(), STRIDE, dest_arr.len());
/// let src = nstd_core_slice_new(src_arr.as_ptr().cast(), STRIDE, src_arr.len());
///
/// unsafe { nstd_core_slice_mut_copy(&mut dest, &src) };
/// assert!(dest_arr == src_arr);
/// ```
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
        let len = src.byte_len();
        let dest = nstd_core_slice_mut_as_ptr(dest).cast();
        let src = nstd_core_slice_as_ptr(src).cast();
        nstd_core_mem_copy(dest, src, len);
        0
    }
}
