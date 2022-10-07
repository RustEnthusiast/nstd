#ifndef NSTD_VEC_H
#define NSTD_VEC_H
#include "alloc.h"
#include "core/def.h"
#include "core/slice.h"
#include "nstd.h"

/// A dynamically sized contiguous sequence of values.
typedef struct {
    /// A raw pointer to the vector's memory buffer.
    NSTDAnyMut ptr;
    /// The number of bytes each value in the vector takes up.
    NSTDUInt stride;
    /// The number of active elements in the vector.
    NSTDUInt len;
    /// The number of values allocated in the memory buffer.
    NSTDUInt cap;
} NSTDVec;

/// Creates a new vector without allocating any resources.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size in bytes of each value in the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if `element_size` is zero.
NSTDAPI NSTDVec nstd_vec_new(NSTDUInt element_size);

/// Creates a new vector initialized with the given capacity.
///
/// # Note
///
/// This will return a "null vector" (a vector that has not allocated yet) on error.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size in bytes of each value in the vector.
///
/// - `NSTDUInt cap` - The initial capacity for the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function may panic in the following situations:
///
/// - Either `element_size` or `cap` are zero.
///
/// - Getting a handle to the heap fails.
NSTDAPI NSTDVec nstd_vec_new_with_cap(NSTDUInt element_size, NSTDUInt cap);

/// Creates a new deep copy of `vec`.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector to create a new deep copy of.
///
/// # Returns
///
/// `NSTDVec cloned` - The new deep copy of `vec`.
///
/// # Panics
///
/// This operation will panic if allocating for the new vector fails.
NSTDAPI NSTDVec nstd_vec_clone(const NSTDVec *vec);

/// Returns the length of a vector.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUInt len` - The length of the vector.
NSTDAPI NSTDUInt nstd_vec_len(const NSTDVec *vec);

/// Returns a vector's capacity.
///
/// This is the max number of values the vector can contain without reallocating.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUInt cap` - The vector's capacity.
NSTDAPI NSTDUInt nstd_vec_cap(const NSTDVec *vec);

/// Returns the amount of bytes each value in a vector occupies.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDUInt stride` - The size of each value in the vector.
NSTDAPI NSTDUInt nstd_vec_stride(const NSTDVec *vec);

/// Returns an immutable slice containing all of a vector's active elements.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDSlice slice` - An *immutable* view into the vector.
NSTDAPI NSTDSlice nstd_vec_as_slice(const NSTDVec *vec);

/// Returns a slice containing all of a vector's active elements.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDSliceMut slice` - A *mutable* view into the vector.
NSTDAPI NSTDSliceMut nstd_vec_as_slice_mut(NSTDVec *vec);

/// Returns a pointer to a vector's raw data.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the vector's raw data.
NSTDAPI NSTDAny nstd_vec_as_ptr(const NSTDVec *vec);

/// Returns a pointer to a vector's raw data.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the vector's raw data.
NSTDAPI NSTDAnyMut nstd_vec_as_mut_ptr(NSTDVec *vec);

/// Returns an immutable pointer to the element at index `pos` in `vec`.
///
/// # Note
///
/// It is highly advised to copy the return value onto the stack because the pointer can easily
/// become invalid if the vector is mutated.
///
/// # Parameters:
///
/// - `const NSTDVec *vec` - The vector to read an element from.
///
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out
/// of the vector's boundaries.
NSTDAPI NSTDAny nstd_vec_get(const NSTDVec *vec, NSTDUInt pos);

/// Returns a pointer to the element at index `pos` in `vec`.
///
/// # Note
///
/// It is highly advised to copy the return value onto the stack because the pointer can easily
/// become invalid if the vector is mutated.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to read an element from.
///
/// - `NSTDUInt pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyMut element` - A pointer to the element at `pos` or `NSTD_NULL` if `pos` is out of
/// the vector's boundaries.
NSTDAPI NSTDAnyMut nstd_vec_get_mut(NSTDVec *vec, NSTDUInt pos);

/// Pushes a value onto a vector by copying bytes to the end of the vector's buffer. The number of
/// bytes to push is determined by `vec`'s stride.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAny value` - A pointer to the value to push onto the vector.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation may panic if getting a handle to the heap fails.
///
/// # Safety
///
/// This operation is unsafe because undefined behavior can occur if the size of the value being
/// pushed onto the vector is not equal to `vec`'s stride.
NSTDAPI NSTDAllocError nstd_vec_push(NSTDVec *vec, NSTDAny value);

/// Removes the last value of a vector and returns a pointer to it.
///
/// # Note
///
/// It is highly advised to copy the return value onto the stack because the pointer can easily
/// become invalid if the vector is mutated.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// - `NSTDAny value` - A pointer to the value that was popped off the stack, or null if the
/// vector is empty.
///
/// # Panics
///
/// Panics if `vec`'s new length (in bytes) exceeds `NSTDInt`'s max value.
NSTDAPI NSTDAny nstd_vec_pop(NSTDVec *vec);

/// Attempts to insert a value into a vector at `index`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAny value` - A pointer to the value to insert into the vector.
///
/// - `NSTDUInt index` - The index at which to insert the value.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Possible errors
///
/// - `1` - `index` is greater than the vector's length.
///
/// - `2` - Reserving space for the vector failed.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `index` multiplied by `vec`'s stride exceeds `NSTDInt`'s max value.
///
/// - Getting a handle to the heap fails.
///
/// # Safety
///
/// This operation is unsafe because undefined behavior can occur if the size of the value being
/// inserted into the vector is not equal to `vec`'s stride.
NSTDAPI NSTDErrorCode nstd_vec_insert(NSTDVec *vec, NSTDAny value, NSTDUInt index);

/// Removes the element at `index` in a vector.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDUInt index` - The index of the element to remove.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if `index` is invalid.
NSTDAPI NSTDErrorCode nstd_vec_remove(NSTDVec *vec, NSTDUInt index);

/// Pushes a series of values onto a vector.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to extend.
///
/// - `const NSTDSlice *values` - A slice of values to push onto the vector.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation will panic if the element sizes for `vec` and `values` do not match.
///
/// # Safety
///
/// This operation can cause undefined behavior if `values`'s data is invalid.
NSTDAPI NSTDAllocError nstd_vec_extend(NSTDVec *vec, const NSTDSlice *values);

/// Shortens a vector, keeping the first `len` elements.
///
/// # Note
///
/// This function does nothing if `vec.len` is less than or equal to `len`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to truncate.
///
/// - `NSTDUInt len` - The number of elements to keep.
NSTDAPI void nstd_vec_truncate(NSTDVec *vec, NSTDUInt len);

/// Reserves some space on the heap for at least `size` more elements to be pushed onto a vector
/// without making more allocations.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to reserve space for.
///
/// - `NSTDUInt size` - The number of additional elements to allocate for.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
///
/// # Panics
///
/// This operation may panic if either `size` is zero or getting a handle to the heap fails.
NSTDAPI NSTDAllocError nstd_vec_reserve(NSTDVec *vec, NSTDUInt size);

/// Decreases a vector's capacity to match it's length.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDAllocError errc` - The allocation operation error code.
NSTDAPI NSTDAllocError nstd_vec_shrink(NSTDVec *vec);

/// Frees an instance of `NSTDVec`.
///
/// # Parameters:
///
/// - `NSTDVec vec` - The vector to free.
///
/// # Panics
///
/// This operation may panic if getting a handle to the heap fails.
NSTDAPI void nstd_vec_free(NSTDVec vec);

#endif
