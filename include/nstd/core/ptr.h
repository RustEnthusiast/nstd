#ifndef NSTD_CORE_PTR_H
#define NSTD_CORE_PTR_H
#include "../nstd.h"

/// A sized immutable pointer to some arbitrary type.
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
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::core::ptr::{nstd_core_ptr_new, nstd_core_ptr_size};
///
/// const VALUE_SIZE: usize = core::mem::size_of::<isize>();
/// let x = 33isize;
/// let ptr = nstd_core_ptr_new(addr_of!(x).cast(), VALUE_SIZE);
/// assert!(nstd_core_ptr_size(&ptr) == VALUE_SIZE);
/// ```
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
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::core::ptr::{nstd_core_ptr_get, nstd_core_ptr_new};
///
/// const VALUE_SIZE: usize = core::mem::size_of::<u32>();
/// let x = 45u32;
/// let ptr = nstd_core_ptr_new(addr_of!(x).cast(), VALUE_SIZE);
/// unsafe {
///     assert!(*nstd_core_ptr_get(&ptr).cast::<u32>() == x);
/// }
/// ```
NSTDAPI NSTDAny nstd_core_ptr_get(const NSTDPtr *ptr);

/// A sized pointer to some arbitrary type.
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
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of_mut;
/// use nstd_sys::core::ptr::{nstd_core_ptr_mut_new, nstd_core_ptr_mut_size};
///
/// const VALUE_SIZE: usize = core::mem::size_of::<isize>();
/// let mut x = 33isize;
/// let ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), VALUE_SIZE);
/// assert!(nstd_core_ptr_mut_size(&ptr) == VALUE_SIZE);
/// ```
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
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of_mut;
/// use nstd_sys::core::ptr::{nstd_core_ptr_mut_get, nstd_core_ptr_mut_new};
///
/// const VALUE_SIZE: usize = core::mem::size_of::<u32>();
/// let mut x = 8u32;
/// let mut ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), VALUE_SIZE);
/// unsafe {
///     let x_ptr = nstd_core_ptr_mut_get(&mut ptr).cast();
///     *x_ptr *= 2;
///     assert!(x == *x_ptr);
/// }
/// ```
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
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of_mut;
/// use nstd_sys::core::ptr::{nstd_core_ptr_mut_get_const, nstd_core_ptr_mut_new};
///
/// const VALUE_SIZE: usize = core::mem::size_of::<u32>();
/// let mut x = 45u32;
/// let ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), VALUE_SIZE);
/// unsafe {
///     assert!(*nstd_core_ptr_mut_get_const(&ptr).cast::<u32>() == x);
/// }
/// ```
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
///
/// # Examples
///
/// ```
/// use core::ptr::{addr_of, addr_of_mut};
/// use nstd_sys::core::ptr::{
///     nstd_core_ptr_mut_get_const, nstd_core_ptr_mut_new, nstd_core_ptr_mut_write,
/// };
///
/// const VALUE_SIZE: usize = core::mem::size_of::<i64>();
/// let mut x = -69i64;
/// let mut ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), VALUE_SIZE);
/// unsafe {
///     let y = 420i64;
///     nstd_core_ptr_mut_write(&mut ptr, addr_of!(y).cast());
///     assert!(x == y);
/// }
/// ```
NSTDAPI void nstd_core_ptr_mut_write(NSTDPtrMut *ptr, NSTDAny obj);

#endif
