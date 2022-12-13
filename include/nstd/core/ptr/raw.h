#ifndef NSTD_CORE_PTR_RAW_H
#define NSTD_CORE_PTR_RAW_H
#include "../../nstd.h"

/// Creates a new dangling immutable pointer with valid alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAny dangling` - The new dangling raw pointer.
NSTDAPI NSTDAny nstd_core_ptr_raw_dangling();

/// Creates a new dangling mutable pointer with valid alignment for any scalar type.
///
/// # Returns
///
/// `NSTDAnyMut dangling` - The new dangling raw pointer.
NSTDAPI NSTDAnyMut nstd_core_ptr_raw_dangling_mut();

/// Returns a pointer that is properly aligned to `align` based on the offset `ptr`.
///
/// # Parameters:
///
/// - `NSTDAny ptr` - The pointer to align.
///
/// - `NSTDUInt align` - The alignment requirement. This must be a power of two.
///
/// # Returns
///
/// `NSTDAny aligned` - The properly aligned pointer.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two or overflow occurs.
///
/// # Safety
///
/// Both `ptr` and the resulting pointer must be either in bounds or one byte past the end of the
/// same allocated object.
NSTDAPI NSTDAny nstd_core_ptr_raw_align(NSTDAny ptr, NSTDUInt align);

/// Returns a pointer that is properly aligned to `align` based on the offset `ptr`.
///
/// # Parameters:
///
/// - `NSTDAnyMut ptr` - The pointer to align.
///
/// - `NSTDUInt align` - The alignment requirement. This must be a power of two.
///
/// # Returns
///
/// `NSTDAnyMut aligned` - The properly aligned pointer.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two or overflow occurs.
///
/// # Safety
///
/// Both `ptr` and the resulting pointer must be either in bounds or one byte past the end of the
/// same allocated object.
NSTDAPI NSTDAnyMut nstd_core_ptr_raw_align_mut(NSTDAnyMut ptr, NSTDUInt align);

#endif
