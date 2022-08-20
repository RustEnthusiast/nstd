#ifndef NSTD_CORE_SLICE_H
#define NSTD_CORE_SLICE_H
#include "../nstd.h"
#include "def.h"
#include "ptr.h"
NSTDCPPSTART

/// An immutable view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDPtrConst ptr;
    /// The number of elements in the slice.
    NSTDUInt len;
} NSTDSliceConst;

/// Creates a new slice from raw data.
///
/// # Parameters:
///
/// - `NSTDAnyConst ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUInt element_size` - The number of bytes each element occupies.
///
/// - `NSTDUInt len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSliceConst slice` - The new slice.
NSTDAPI NSTDSliceConst nstd_core_slice_const_new(NSTDAnyConst ptr, NSTDUInt element_size,
NSTDUInt len);

/// Returns a raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice.
///
/// # Returns
///
/// `AnyConst ptr` - A raw pointer to the slice's memory.
NSTDAPI NSTDAnyConst nstd_core_slice_const_as_ptr(const NSTDSliceConst *slice);

/// Returns the number of elements in an immutable slice.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The immutable slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the slice.
NSTDAPI NSTDUInt nstd_core_slice_const_len(const NSTDSliceConst *slice);

/// Returns the amount of bytes each value in a slice occupies.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt stride` - The size of each value in the slice.
NSTDAPI NSTDUInt nstd_core_slice_const_stride(const NSTDSliceConst *slice);

/// Returns an immutable pointer to the element at index `pos` in `slice`.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The slice to read an element from.
///
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
NSTDAPI NSTDAnyConst nstd_core_slice_const_get(const NSTDSliceConst *slice, NSTDUInt pos);

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
NSTDAPI NSTDAnyConst nstd_core_slice_const_first(const NSTDSliceConst *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_const_last(const NSTDSliceConst *slice);

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
NSTDAPI NSTDBool nstd_core_slice_const_compare(const NSTDSliceConst *s1, const NSTDSliceConst *s2);

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
/// `NSTDSliceConst slice_const` - The immutable copy of `slice`.
NSTDAPI NSTDSliceConst nstd_core_slice_mut_as_const(const NSTDSliceMut *slice);

/// Returns a raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A raw pointer to the slice's memory.
NSTDAPI NSTDAnyMut nstd_core_slice_mut_as_ptr(NSTDSliceMut *slice);

/// Returns an immutable raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDAnyConst ptr` - A raw pointer to the slice's memory.
NSTDAPI NSTDAnyConst nstd_core_slice_mut_as_ptr_const(const NSTDSliceMut *slice);

/// Returns the number of elements in a slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the slice.
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
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
NSTDAPI NSTDAnyConst nstd_core_slice_mut_get_const(const NSTDSliceMut *slice, NSTDUInt pos);

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
NSTDAPI NSTDAnyMut nstd_core_slice_mut_first(NSTDSliceMut *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_mut_first_const(const NSTDSliceMut *slice);

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
NSTDAPI NSTDAnyMut nstd_core_slice_mut_last(NSTDSliceMut *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_mut_last_const(const NSTDSliceMut *slice);

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
NSTDAPI NSTDBool nstd_core_slice_mut_compare(const NSTDSliceMut *s1, const NSTDSliceMut *s2);

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
NSTDAPI NSTDErrorCode nstd_core_slice_mut_copy(NSTDSliceMut *dest, const NSTDSliceConst *src);

NSTDCPPEND
#endif
