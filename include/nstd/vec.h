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

/// Frees an instance of `NSTDVec`.
///
/// # Parameters:
///
/// - `NSTDVec *vec` - The vector to free.
NSTDAPI void nstd_vec_free(NSTDVec *vec);

NSTDCPPEND
#endif
