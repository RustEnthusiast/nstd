#ifndef NSTD_CORE_PTR_H
#define NSTD_CORE_PTR_H
#include "../nstd.h"

/// A sized immutable pointer to some arbitrary type.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid and unmodified
/// while an instance of this structure is in use.
typedef struct {
    /// A raw pointer to the data.
    NSTDAny raw;
    /// The size of the object being pointed to.
    NSTDUInt size;
} NSTDPtr;

/// Creates a new instance of `NSTDPtr`.
///
/// # Parameters:
///
/// - `NSTDAny obj` - The object to point to.
///
/// - `NSTDUInt size` - The number of bytes that `obj`'s type occupies.
///
/// # Returns
///
/// `NSTDPtr ptr` - A new instance of `NSTDPtr` that points to `obj`.
///
/// # Panics
///
/// Panics if `obj` is null.
NSTDAPI NSTDPtr nstd_core_ptr_new(NSTDAny obj, NSTDUInt size);

/// Returns the size of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtr *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUInt size` - The size of the object pointed to by `ptr`.
NSTDAPI NSTDUInt nstd_core_ptr_size(const NSTDPtr *ptr);

/// Returns a raw immutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtr *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAny raw` - A raw pointer to the object.
NSTDAPI NSTDAny nstd_core_ptr_get(const NSTDPtr *ptr);

/// A sized pointer to some arbitrary type.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid, unmodified, and
/// unreferenced in any other code while an instance of this structure is in use, else data races
/// may occur.
typedef struct {
    /// A raw pointer to the data.
    NSTDAnyMut raw;
    /// The size of the object being pointed to.
    NSTDUInt size;
} NSTDPtrMut;

/// Creates a new instance of `NSTDPtrMut`.
///
/// # Parameters:
///
/// - `NSTDAnyMut obj` - The object to point to.
///
/// - `NSTDUInt size` - The number of bytes that `obj`'s type occupies.
///
/// # Returns
///
/// `NSTDPtrMut ptr` - A new instance of `NSTDPtrMut` that points to `obj`.
///
/// # Panics
///
/// Panics if `obj` is null.
NSTDAPI NSTDPtrMut nstd_core_ptr_mut_new(NSTDAnyMut obj, NSTDUInt size);

/// Creates an immutable version of a mutable pointer.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The mutable pointer.
///
/// # Returns
///
/// `NSTDPtr ptr_const` - The immutable copy of `ptr`.
NSTDAPI NSTDPtr nstd_core_ptr_mut_as_const(const NSTDPtrMut *ptr);

/// Returns the size of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUInt size` - The size of the object pointed to by `ptr`.
NSTDAPI NSTDUInt nstd_core_ptr_mut_size(const NSTDPtrMut *ptr);

/// Returns a raw pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `NSTDPtrMut *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAnyMut raw` - A raw pointer to the object.
NSTDAPI NSTDAnyMut nstd_core_ptr_mut_get(NSTDPtrMut *ptr);

/// Returns a raw immutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAny raw` - A raw pointer to the object.
NSTDAPI NSTDAny nstd_core_ptr_mut_get_const(const NSTDPtrMut *ptr);

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
/// - `NSTDAny obj` - A pointer to the object to write to `ptr`.
///
/// # Safety
///
/// This operation is highly unsafe because there is no way of knowing if `obj`'s data is valid.
NSTDAPI void nstd_core_ptr_mut_write(NSTDPtrMut *ptr, NSTDAny obj);

#endif
