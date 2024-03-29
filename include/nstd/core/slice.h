#ifndef NSTD_CORE_SLICE_H
#define NSTD_CORE_SLICE_H
#include "../nstd.h"
#include "optional.h"

/// An immutable view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDAny ptr;
    /// The number of elements in the slice.
    NSTDUInt len;
    /// The slice's stride.
    NSTDUInt stride;
    /// The slice's align.
    NSTDUInt align;
} NSTDSlice;

/// Represents an optional value of type `NSTDSlice`.
NSTDOptional(NSTDSlice) NSTDOptionalSlice;

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
NSTDAPI NSTDOptionalSlice
nstd_core_slice_new(NSTDAny ptr, NSTDUInt stride, NSTDUInt align, NSTDUInt len);

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
NSTDAPI NSTDSlice
nstd_core_slice_new_unchecked(NSTDAny ptr, NSTDUInt stride, NSTDUInt align, NSTDUInt len);

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
NSTDAPI NSTDSlice nstd_core_slice_empty(NSTDUInt stride, NSTDUInt align);

/// Returns a raw pointer to the slice's memory.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `AnyConst ptr` - A raw pointer to the slice's memory.
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
NSTDAPI NSTDUInt nstd_core_slice_stride(const NSTDSlice *slice);

/// Returns the alignment of each value in a slice.
///
/// # Parameters:
///
/// - `const NSTDSlice *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt align` - The alignment of each value in the slice.
NSTDAPI NSTDUInt nstd_core_slice_align(const NSTDSlice *slice);

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
NSTDAPI NSTDAny nstd_core_slice_last(const NSTDSlice *slice);

/// A view into a sequence of values in memory.
typedef struct {
    /// A pointer to the first element in the slice.
    NSTDAnyMut ptr;
    /// The number of elements in the slice.
    NSTDUInt len;
    /// The slice's stride.
    NSTDUInt stride;
    /// The slice's align.
    NSTDUInt align;
} NSTDSliceMut;

/// Represents an optional value of type `NSTDSliceMut`.
NSTDOptional(NSTDSliceMut) NSTDOptionalSliceMut;

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
NSTDAPI NSTDOptionalSliceMut
nstd_core_slice_mut_new(NSTDAnyMut ptr, NSTDUInt stride, NSTDUInt align, NSTDUInt len);

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
NSTDAPI NSTDSliceMut
nstd_core_slice_mut_new_unchecked(NSTDAnyMut ptr, NSTDUInt stride, NSTDUInt align, NSTDUInt len);

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
NSTDAPI NSTDSliceMut nstd_core_slice_mut_empty(NSTDUInt stride, NSTDUInt align);

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

/// Returns the alignment of each value in a slice.
///
/// # Parameters:
///
/// - `const NSTDSliceMut *slice` - The slice.
///
/// # Returns
///
/// `NSTDUInt align` - The alignment of each value in the slice.
NSTDAPI NSTDUInt nstd_core_slice_mut_align(const NSTDSliceMut *slice);

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
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the slice's boundaries.
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
NSTDAPI NSTDAny nstd_core_slice_mut_last_const(const NSTDSliceMut *slice);

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
NSTDAPI void nstd_core_slice_mut_copy(NSTDSliceMut *dest, const NSTDSlice *src);

#endif
