#ifndef NSTD_CORE_PTR_H_INCLUDED
#define NSTD_CORE_PTR_H_INCLUDED
#include "../nstd.h"
#include "def.h"
NSTDCPPSTART

/// A sized pointer to some arbitrary type.
typedef struct {
    /// A raw pointer to the data.
    NSTDAny raw;
    /// The size of the object being pointed to.
    NSTDUSize size;
} NSTDPtr;

/// Creates a new instance of `NSTDPtr`.
///
/// # Parameters:
///
/// - `NSTDAny obj` - The object to point to.
///
/// - `NSTDUSize size` - The number of bytes that `obj`'s type occupies.
///
/// # Returns
/// `NSTDPtr ptr` - A new instance of `NSTDPtr` that points to `obj`.
NSTDAPI NSTDPtr nstd_core_ptr_new(NSTDAny obj, NSTDUSize size);

/// Returns a raw pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtr *ptr` - The higher level pointer.
///
/// # Returns
/// `NSTDAnyConst raw` - A raw pointer to the object.
///
/// # Safety
/// This operation is unsafe because there is no way of knowing if the object being pointed to is
/// still valid.
NSTDAPI NSTDAnyConst nstd_core_ptr_read(const NSTDPtr *ptr);

/// Returns a raw mutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `NSTDPtr *ptr` - The higher level pointer.
///
/// # Returns
/// `NSTDAny raw` - A raw pointer to the object.
///
/// # Safety
/// This operation is unsafe because there is no way of knowing if the object being pointed to is
/// still valid.
NSTDAPI NSTDAny nstd_core_ptr_read_mut(NSTDPtr *ptr);

/// Writes data from `obj` to `ptr`. The number of bytes written is determined by `ptr.size`.
///
/// # Parameters:
///
/// - `NSTDPtr *ptr` - The pointer to write to.
///
/// - `NSTDAnyConst obj` - A pointer to the object to write to `ptr`.
///
/// # Safety
/// This operation is highly unsafe because there is no way of knowing if either of the pointers
/// are valid.
NSTDAPI void nstd_core_ptr_write(NSTDPtr *ptr, NSTDAnyConst obj);

NSTDCPPEND
#endif
