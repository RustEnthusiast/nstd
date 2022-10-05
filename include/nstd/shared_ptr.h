#ifndef NSTD_SHARED_PTR_H
#define NSTD_SHARED_PTR_H
#include "nstd.h"

/// A reference counting smart pointer.
typedef struct {
    /// A raw pointer to private data about the shared object.
    NSTDAnyMut ptr;
    /// The size of the shared pointer's memory buffer.
    NSTDUInt size;
} NSTDSharedPtr;

/// Creates a new initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size of the shared object.
///
/// - `NSTDAny init` - A pointer to the object to initialize the shared pointer with.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The new shared pointer.
///
/// # Panics
///
/// This operation will panic if either `element_size` is greater than `NSTDInt`'s max value or
/// allocating fails.
///
/// # Safety
///
/// `init` must be a pointer to a value that is valid for reads of `element_size` bytes.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<i16>();
///
/// let v = i16::MIN;
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<i16>() == v);
/// }
/// ```
NSTDAPI NSTDSharedPtr nstd_shared_ptr_new(NSTDUInt element_size, NSTDAny init);

/// Creates a new zero-initialized instance of a shared pointer.
///
/// # Parameters:
///
/// - `NSTDUInt element_size` - The size of the shared object.
///
/// # Returns
///
/// `NSTDSharedPtr shared_ptr` - The yet to be shared pointer.
///
/// # Panics
///
/// This operation will panic if either `element_size` is greater than `NSTDInt`'s max value or
/// allocating fails.
///
/// # Example
///
/// ```
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new_zeroed};
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new_zeroed(SIZE);
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == 0);
/// }
/// ```
NSTDAPI NSTDSharedPtr nstd_shared_ptr_new_zeroed(NSTDUInt element_size);

/// Shares `shared_ptr`.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared object to share.
///
/// # Returns
///
/// `NSTDSharedPtr shared` - A new pointer pointing to the shared data.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_share};
///
/// const SIZE: usize = core::mem::size_of::<u64>();
///
/// unsafe {
///     let v = 39u64;
///     let share;
///     {
///         let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///         share = nstd_shared_ptr_share(&shared_ptr);
///     }
///     assert!(*nstd_shared_ptr_get(&share).cast::<u64>() == v);
/// }
/// ```
NSTDAPI NSTDSharedPtr nstd_shared_ptr_share(const NSTDSharedPtr *shared_ptr);

/// Returns the number of pointers that share `shared_ptr`'s data.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - An instance of a shared pointer.
///
/// # Returns
///
/// `NSTDUInt owners` - The number of pointers that share `shared_ptr`'s data.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{
///     nstd_shared_ptr_get, nstd_shared_ptr_new, nstd_shared_ptr_owners, nstd_shared_ptr_share,
/// };
///
/// const SIZE: usize = core::mem::size_of::<i128>();
///
/// unsafe {
///     let v = i128::MIN;
///     let share;
///     {
///         let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///         assert!(nstd_shared_ptr_owners(&shared_ptr) == 1);
///
///         share = nstd_shared_ptr_share(&shared_ptr);
///         assert!(nstd_shared_ptr_owners(&shared_ptr) == 2);
///
///         let temp = nstd_shared_ptr_share(&shared_ptr);
///         assert!(nstd_shared_ptr_owners(&temp) == 3);
///     }
///     assert!(nstd_shared_ptr_owners(&share) == 1);
///     assert!(*nstd_shared_ptr_get(&share).cast::<i128>() == v);
/// }
/// ```
NSTDAPI NSTDUInt nstd_shared_ptr_owners(const NSTDSharedPtr *shared_ptr);

/// Returns the size of the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDUInt size` - The size of the shared object.
///
/// # Example
///
/// ```
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_new_zeroed, nstd_shared_ptr_size};
///
/// const SIZE: usize = core::mem::size_of::<f64>();
///
/// let shared_ptr = nstd_shared_ptr_new_zeroed(SIZE);
/// assert!(nstd_shared_ptr_size(&shared_ptr) == SIZE);
/// ```
NSTDAPI NSTDUInt nstd_shared_ptr_size(const NSTDSharedPtr *shared_ptr);

/// Returns an immutable raw pointer to the shared object.
///
/// # Parameters:
///
/// - `const NSTDSharedPtr *shared_ptr` - The shared pointer.
///
/// # Returns
///
/// `NSTDAny ptr` - A raw pointer to the shared object.
///
/// # Example
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::shared_ptr::{nstd_shared_ptr_get, nstd_shared_ptr_new};
///
/// const SIZE: usize = core::mem::size_of::<u128>();
///
/// let v = u128::MAX;
/// unsafe {
///     let shared_ptr = nstd_shared_ptr_new(SIZE, addr_of!(v).cast());
///     assert!(*nstd_shared_ptr_get(&shared_ptr).cast::<u128>() == v);
/// }
/// ```
NSTDAPI NSTDAny nstd_shared_ptr_get(const NSTDSharedPtr *shared_ptr);

/// Frees an instance of `NSTDSharedPtr`.
///
/// # Parameters:
///
/// - `NSTDSharedPtr shared_ptr` - The shared object to free.
///
/// # Panics
///
/// This operation may panic if getting a handle to the heap fails.
NSTDAPI void nstd_shared_ptr_free(NSTDSharedPtr shared_ptr);

#endif
