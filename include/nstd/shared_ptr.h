#ifndef NSTD_SHARED_PTR_H_INCLUDED
#define NSTD_SHARED_PTR_H_INCLUDED
#include "core/ptr.h"
#include "nstd.h"
NSTDCPPSTART

/// A reference counting smart pointer.
typedef struct {
    /// A pointer to private data about the shared object.
    NSTDPtrMut ptr;
} NSTDSharedPtr;

/// Creates a new initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size of the shared object.
///
/// - `NSTDAnyConst init` - A pointer to the object to initialize the shared pointer with.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The new shared pointer.
///
/// # Panics
///
/// This operation will panic if allocating fails.
///
/// # Safety
///
/// This operation is unsafe because passing `init` as a null pointer can cause undefined behavior.
NSTDAPI NSTDSharedPtr nstd_shared_ptr_new(NSTDUSize element_size, NSTDAnyConst init);

/// Creates a new zero-initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUSize element_size` - The size of the shared object.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The yet to be shared pointer.
///
/// # Panics
///
/// This operation will panic if allocating fails.
NSTDAPI NSTDSharedPtr nstd_shared_ptr_new_zeroed(NSTDUSize element_size);

/// Shares `shared_ptr`.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared object to share.
///
/// # Returns
///
/// `NSTDSharedPtr shared` - A new pointer pointing to the shared data.
NSTDAPI NSTDSharedPtr nstd_shared_ptr_share(const NSTDSharedPtr *shared_ptr);

/// Returns the number pointers that share `shared_ptr`'s data.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - An instance of a shared pointer.
///
/// # Returns
///
/// `NSTDUSize owners` - The number pointers that share `shared_ptr`'s data.
NSTDAPI NSTDUSize nstd_shared_ptr_owners(const NSTDSharedPtr *shared_ptr);

/// Returns the size of the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the shared object.
NSTDAPI NSTDUSize nstd_shared_ptr_size(const NSTDSharedPtr *shared_ptr);

/// Returns an immutable raw pointer to the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDAnyConst ptr` - A raw pointer to the shared object.
NSTDAPI NSTDAnyConst nstd_shared_ptr_get(const NSTDSharedPtr *shared_ptr);

/// Frees an instance of `NSTDSharedPtr`.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
NSTDAPI void nstd_shared_ptr_free(NSTDSharedPtr shared_ptr);

NSTDCPPEND
#endif
