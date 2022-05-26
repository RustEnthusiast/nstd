#ifndef NSTD_VEC_H_INCLUDED
#define NSTD_VEC_H_INCLUDED
#include "core/def.h"
#include "core/slice.h"
#include "nstd.h"
NSTDCPPSTART

/// A dynamically sized contiguous sequence of values.
typedef struct {
    /// The underlying memory buffer.
    NSTDSlice buffer;
    /// The number of active elements in the vector.
    NSTDUSize len;
} NSTDVec;

/// Creates a new vector without allocating any resources.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size in bytes of each value in the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if `element_size` is zero.
NSTDAPI NSTDVec nstd_vec_new(NSTDUSize element_size);

/// Creates a new vector initialized with the given capacity.
///
/// # Note
///
/// This will return a "null vector" (a vector that has not allocated yet) on error.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size in bytes of each value in the vector.
///
/// - `NSTDUSize cap` - The initial capacity for the vector.
///
/// # Returns
///
/// `NSTDVec vec` - The new vector.
///
/// # Panics
///
/// This function will panic if either `element_size` or `cap` are zero.
NSTDAPI NSTDVec nstd_vec_new_with_cap(NSTDUSize element_size, NSTDUSize cap);

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

/// Returns a slice containing all of a vector's active elements.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDSlice slice` - A *mutable* view into the vector.
NSTDAPI NSTDSlice nstd_vec_as_slice(NSTDVec *vec);

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
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAny element` - A pointer to the element at `pos` or `NSTD_CORE_NULL` if `pos` is out of
/// the vector's boundaries.
NSTDAPI NSTDAny nstd_vec_get(NSTDVec *vec, NSTDUSize pos);

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
/// - `NSTDUSize pos` - The position of the element to get, starting at 0.
///
/// # Returns
///
/// `NSTDAnyConst element` - A pointer to the element at `pos` or `NSTD_CORE_NULL` if `pos` is out
/// of the vector's boundaries.
NSTDAPI NSTDAnyConst nstd_vec_get_const(const NSTDVec *vec, NSTDUSize pos);

/// Pushes a value onto a vector by copying bytes to the end of the vector's buffer. The number of
/// bytes to push is determined by `vec.buffer.ptr.size`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAnyConst value` - A pointer to the value to push onto the vector.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Safety
///
/// This operation is unsafe because undefined behaviour can occur if the size of the value being
/// pushed onto the vector is not equal to `vec.buffer.ptr.size`.
NSTDAPI NSTDErrorCode nstd_vec_push(NSTDVec *vec, NSTDAnyConst value);

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
/// - `NSTDAnyConst value` - A pointer to the value that was popped off the stack, or null if the
/// vector is empty.
NSTDAPI NSTDAnyConst nstd_vec_pop(NSTDVec *vec);

/// Attempts to insert a value into a vector at `index`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDAnyConst value` - A pointer to the value to insert into the vector.
///
/// - `NSTDUSize index` - The index at which to insert the value.
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
/// # Safety
///
/// This operation is unsafe because undefined behaviour can occur if the size of the value being
/// inserted into the vector is not equal to `vec.buffer.ptr.size`.
NSTDAPI NSTDErrorCode nstd_vec_insert(NSTDVec *vec, NSTDAnyConst value, NSTDUSize index);

/// Removes the element at `index` in a vector.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// - `NSTDUSize index` - The index of the element to remove.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero if `index` is invalid.
NSTDAPI NSTDErrorCode nstd_vec_remove(NSTDVec *vec, NSTDUSize index);

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
/// `NSTDErrorCode errc` - Nonzero if reserving memory for the extension fails.
///
/// # Panics
///
/// This operation will panic if the element sizes for `vec` and `values` do not match.
///
/// # Safety
///
/// This operation is unsafe because `values`'s data is never guaranteed to be valid.
NSTDAPI NSTDErrorCode nstd_vec_extend(NSTDVec *vec, const NSTDSlice *values);

/// Reserves some space on the heap for at least `size` more elements to be pushed onto a vector
/// without making more allocations.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to reserve space for.
///
/// - `NSTDUSize size` - The number of additional elements to allocate for.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
///
/// # Panics
///
/// This operation will panic if `size` is zero.
NSTDAPI NSTDErrorCode nstd_vec_reserve(NSTDVec *vec, NSTDUSize size);

/// Decreases a vector's capacity to match it's length.
///
/// # Note
///
/// This will return an error code of `0` if the vector is "null".
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector.
///
/// # Returns
///
/// `NSTDErrorCode errc` - Nonzero on error.
NSTDAPI NSTDErrorCode nstd_vec_shrink(NSTDVec *vec);

/// Frees an instance of `NSTDVec`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to free.
NSTDAPI void nstd_vec_free(NSTDVec *vec);

NSTDCPPEND
#endif
