#ifndef NSTD_CORE_PTR_H_INCLUDED
#define NSTD_CORE_PTR_H_INCLUDED
#include "../nstd.h"
NSTDCPPSTART

/// A sized pointer to some arbitrary type.
typedef struct {
    /// A raw pointer to the data.
    NSTDAnyMut raw;
    /// The size of the object being pointed to.
    NSTDUSize size;
} NSTDPtrMut;

/// Creates a new instance of `NSTDPtrMut`.
///
/// # Parameters:
///
/// - `NSTDAnyMut obj` - The object to point to.
///
/// - `NSTDUSize size` - The number of bytes that `obj`'s type occupies.
///
/// # Returns
///
/// `NSTDPtrMut ptr` - A new instance of `NSTDPtrMut` that points to `obj`.
///
/// # Safety
///
/// `obj` must remain valid while the returned pointer is in use.
NSTDAPI NSTDPtrMut nstd_core_ptr_mut_new(NSTDAnyMut obj, NSTDUSize size);

/// Returns the size of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the object pointed to by `ptr`.
NSTDAPI NSTDUSize nstd_core_ptr_mut_size(const NSTDPtrMut *ptr);

/// Returns a raw pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `NSTDPtrMut *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAnyMut raw` - A raw pointer to the object.
///
/// # Safety
///
/// `ptr`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDAnyMut nstd_core_ptr_mut_get(NSTDPtrMut *ptr);

/// Returns a raw immutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAnyConst raw` - A raw pointer to the object.
///
/// # Safety
///
/// `ptr`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDAnyConst nstd_core_ptr_mut_get_const(const NSTDPtrMut *ptr);

/// Writes data from `obj` to `ptr`. The number of bytes written is determined by `ptr.size`.
///
/// # Note
///
/// It is up to the user of this function to ensure that `obj`'s memory buffer is at least
/// `ptr.size` bytes wide to avoid writing garbage data to this pointer.
///
/// # Parameters:
///
/// - `NSTDPtrMut *ptr` - The pointer to write to.
///
/// - `NSTDAnyConst obj` - A pointer to the object to write to `ptr`.
///
/// # Safety
///
/// This operation is highly unsafe because there is no way of knowing if `obj`'s data is valid.
NSTDAPI void nstd_core_ptr_mut_write(NSTDPtrMut *ptr, NSTDAnyConst obj);

/// A sized immutable pointer to some arbitrary type.
typedef struct {
    /// A raw pointer to the data.
    NSTDAnyConst raw;
    /// The size of the object being pointed to.
    NSTDUSize size;
} NSTDPtrConst;

/// Creates a new instance of `NSTDPtrConst`.
///
/// # Parameters:
///
/// - `NSTDAnyConst obj` - The object to point to.
///
/// - `NSTDUSize size` - The number of bytes that `obj`'s type occupies.
///
/// # Returns
///
/// `NSTDPtrConst ptr` - A new instance of `NSTDPtrConst` that points to `obj`.
///
/// # Safety
///
/// `obj` must remain valid while the returned pointer is in use.
NSTDAPI NSTDPtrConst nstd_core_ptr_const_new(NSTDAnyConst obj, NSTDUSize size);

/// Returns the size of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtrConst *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the object pointed to by `ptr`.
NSTDAPI NSTDUSize nstd_core_ptr_const_size(const NSTDPtrConst *ptr);

/// Returns a raw immutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtrConst *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAnyConst raw` - A raw pointer to the object.
///
/// # Safety
///
/// `ptr`'s data must remain valid while the returned pointer is in use.
NSTDAPI NSTDAnyConst nstd_core_ptr_const_get(const NSTDPtrConst *ptr);

NSTDCPPEND
#endif
