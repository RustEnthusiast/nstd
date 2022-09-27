#ifndef NSTD_CORE_SLICE_H
#define NSTD_CORE_SLICE_H
#include "../nstd.h"
#include "def.h"
#include "ptr.h"

/// An immutable view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDPtr ptr;
    /// The number of elements in the slice.
    NSTDUInt len;
} NSTDSlice;

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
NSTDAPI NSTDSlice nstd_core_slice_new(NSTDAny ptr, NSTDUInt element_size,
NSTDUInt len);

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
NSTDAPI NSTDAny nstd_core_slice_as_ptr(const NSTDSlice *slice);

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
NSTDAPI NSTDUInt nstd_core_slice_len(const NSTDSlice *slice);

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
NSTDAPI NSTDUInt nstd_core_slice_stride(const NSTDSlice *slice);

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
/// let slice = nstd_core_slice_new(numbers.as_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe {
///     assert!(*nstd_core_slice_get(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_get(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_get(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_get(&slice, 3).is_null());
/// }
/// ```
NSTDAPI NSTDAny nstd_core_slice_get(const NSTDSlice *slice, NSTDUInt pos);

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
NSTDAPI NSTDAny nstd_core_slice_first(const NSTDSlice *slice);

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
/// let slice = nstd_core_slice_new(numbers_ptr, STRIDE, numbers.len());
/// let empty = nstd_core_slice_new(numbers_ptr, STRIDE, 0);
///
/// unsafe {
///     assert!(*nstd_core_slice_last(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_last(&empty).is_null());
/// }
/// ```
NSTDAPI NSTDAny nstd_core_slice_last(const NSTDSlice *slice);

/// A view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDPtrMut ptr;
    /// The number of elements in the slice.
    NSTDUInt len;
} NSTDSliceMut;

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
NSTDAPI NSTDSliceMut nstd_core_slice_mut_new(NSTDAnyMut ptr, NSTDUInt element_size, NSTDUInt len);

/// Creates an immutable version of a mutable slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The mutable slice.
///
/// # Returns
///
/// `NSTDSlice slice_const` - The immutable copy of `slice`.
NSTDAPI NSTDSlice nstd_core_slice_mut_as_const(const NSTDSliceMut *slice);

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
NSTDAPI NSTDAnyMut nstd_core_slice_mut_as_ptr(NSTDSliceMut *slice);

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
NSTDAPI NSTDAny nstd_core_slice_mut_as_ptr_const(const NSTDSliceMut *slice);

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
NSTDAPI NSTDUInt nstd_core_slice_mut_len(const NSTDSliceMut *slice);

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
NSTDAPI NSTDUInt nstd_core_slice_mut_stride(const NSTDSliceMut *slice);

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
/// let mut slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe {
///     *nstd_core_slice_mut_get(&mut slice, 0).cast::<i32>() = 33;
///     *nstd_core_slice_mut_get(&mut slice, 1).cast::<i32>() = 103;
///     *nstd_core_slice_mut_get(&mut slice, 2).cast::<i32>() = 45;
///     assert!(numbers == [33, 103, 45]);
/// }
/// ```
NSTDAPI NSTDAnyMut nstd_core_slice_mut_get(NSTDSliceMut *slice, NSTDUInt pos);

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
/// let slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe {
///     assert!(*nstd_core_slice_mut_get_const(&slice, 0).cast::<i32>() == 33);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 1).cast::<i32>() == 103);
///     assert!(*nstd_core_slice_mut_get_const(&slice, 2).cast::<i32>() == 45);
///     assert!(nstd_core_slice_mut_get_const(&slice, 3).is_null());
/// }
/// ```
NSTDAPI NSTDAny nstd_core_slice_mut_get_const(const NSTDSliceMut *slice, NSTDUInt pos);

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
NSTDAPI NSTDAnyMut nstd_core_slice_mut_first(NSTDSliceMut *slice);

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
NSTDAPI NSTDAny nstd_core_slice_mut_first_const(const NSTDSliceMut *slice);

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
/// let mut slice = nstd_core_slice_mut_new(numbers.as_mut_ptr().cast(), STRIDE, numbers.len());
///
/// unsafe { *nstd_core_slice_mut_last(&mut slice).cast::<u64>() = 1738 };
/// assert!(numbers[2] == 1738);
/// ```
NSTDAPI NSTDAnyMut nstd_core_slice_mut_last(NSTDSliceMut *slice);

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
/// let slice = nstd_core_slice_mut_new(numbers_ptr, STRIDE, numbers.len());
/// let empty = nstd_core_slice_mut_new(numbers_ptr, STRIDE, 0);
///
/// unsafe {
///     assert!(*nstd_core_slice_mut_last_const(&slice).cast::<u64>() == 4317);
///     assert!(nstd_core_slice_mut_last_const(&empty).is_null());
/// }
/// ```
NSTDAPI NSTDAny nstd_core_slice_mut_last_const(const NSTDSliceMut *slice);

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
NSTDAPI NSTDErrorCode nstd_core_slice_mut_copy(NSTDSliceMut *dest, const NSTDSlice *src);

#endif
