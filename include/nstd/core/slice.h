#ifndef NSTD_CORE_SLICE_H_INCLUDED
#define NSTD_CORE_SLICE_H_INCLUDED
#include "../nstd.h"
#include "def.h"
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
/// - `NSTDAny ptr` - A pointer to the first element in the sequence.
///
/// - `NSTDUSize element_size` - The number of bytes each element occupies.
///
/// - `NSTDUSize len` - The number of elements in the sequence.
///
/// # Returns
///
/// `NSTDSlice slice` - The new slice.
NSTDAPI NSTDSlice nstd_core_slice_new(NSTDAny ptr, NSTDUSize element_size, NSTDUSize len);

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
NSTDAPI NSTDAny nstd_core_slice_get(NSTDSlice *slice, NSTDUSize pos);

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
NSTDAPI NSTDAnyConst nstd_core_slice_get_const(const NSTDSlice *slice, NSTDUSize pos);

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
NSTDAPI NSTDAny nstd_core_slice_first(NSTDSlice *slice);

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
NSTDAPI NSTDAnyConst nstd_core_slice_first_const(const NSTDSlice *slice);

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
NSTDAPI NSTDAny nstd_core_slice_last(NSTDSlice *slice);

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
/// `NSTDBool is_eq` - `NSTD_BOOL_TRUE` if the two slices compare equal.
///
/// # Safety
///
/// This operation is unsafe because the underlying data is not guaranteed to be valid.
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
///
/// # Safety
///
/// This operation is unsafe because the underlying data for `dest` or `src` is not guaranteed to
/// be valid.
NSTDAPI void nstd_core_slice_copy(NSTDSlice *dest, const NSTDSlice *src);

NSTDCPPEND
#endif
