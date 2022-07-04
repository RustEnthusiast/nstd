#ifndef NSTD_CORE_SLICE_H_INCLUDED
#define NSTD_CORE_SLICE_H_INCLUDED
#include "../nstd.h"
#include "ptr.h"
NSTDCPPSTART

/// A view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDPtr ptr;
    /// The number of elements in the slice.
    NSTDUSize len;
} NSTDSlice;

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
/// `NSTDSlice slice` - The new slice.
///
/// # Safety
///
/// `ptr`'s data must remain valid while the returned slice is in use.
NSTDAPI NSTDSlice nstd_core_slice_new(NSTDAnyMut ptr, NSTDUSize element_size, NSTDUSize len);

/// Returns the number of elements in a slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the slice.
NSTDAPI NSTDUSize nstd_core_slice_len(const NSTDSlice *slice);

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
/// `NSTDAnyMut element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the slice's boundaries.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDAnyMut nstd_core_slice_get_mut(NSTDSlice *slice, NSTDUSize pos);

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
NSTDAPI NSTDAnyConst nstd_core_slice_get_const(const NSTDSlice *slice, NSTDUSize pos);

/// Returns a pointer to the first element in the slice.
///
/// # Parameters:
///
/// - `NSTDSlice *slice` - The slice to get the first element of.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the first element in `slice` or `NSTD_NULL` if the slice
/// is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDAnyMut nstd_core_slice_first_mut(NSTDSlice *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_first_const(const NSTDSlice *slice);

/// Returns a pointer to the last element in the slice.
///
/// # Parameters:
///
/// - `NSTDSlice *slice` - The slice to get the last element of.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the last element in `slice` or `NSTD_NULL` if the slice
/// is empty.
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDAnyMut nstd_core_slice_last_mut(NSTDSlice *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_last_const(const NSTDSlice *slice);

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
NSTDAPI NSTDBool nstd_core_slice_compare(const NSTDSlice *s1, const NSTDSlice *s2);

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
NSTDAPI void nstd_core_slice_copy(NSTDSlice *dest, const NSTDSlice *src);

/// An immutable view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDPtrConst ptr;
    /// The number of elements in the slice.
    NSTDUSize len;
} NSTDSliceConst;

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
NSTDAPI NSTDSliceConst nstd_core_slice_const_new(NSTDAnyConst ptr, NSTDUSize element_size,
NSTDUSize len);

/// Returns the number of elements in an immutable slice.
///
/// # Parameters:
///
/// - `const NSTDSliceConst *slice` - The immutable slice.
///
/// # Returns
///
/// `NSTDUSize len` - The length of the slice.
NSTDAPI NSTDUSize nstd_core_slice_const_len(const NSTDSliceConst *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_const_get(const NSTDSliceConst *slice, NSTDUSize pos);

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
///
/// # Safety
///
/// `slice`'s data must remain valid while the returned pointer is in use.
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
NSTDAPI NSTDBool nstd_core_slice_const_compare(const NSTDSliceConst *s1, const NSTDSliceConst *s2);

NSTDCPPEND
#endif
