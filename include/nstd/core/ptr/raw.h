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

#endif
