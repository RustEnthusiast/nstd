//! A sized pointer to some arbitrary type.
pub mod raw;
use crate::{core::mem::nstd_core_mem_copy, NSTDAny, NSTDAnyMut, NSTDUInt};

/// A sized immutable pointer to some arbitrary type.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid and unmodified
/// while an instance of this structure is in use.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NSTDPtr {
    /// A raw pointer to the data.
    raw: NSTDAny,
    /// The size of the object being pointed to.
    size: NSTDUInt,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_new(obj: NSTDAny, size: NSTDUInt) -> NSTDPtr {
    assert!(!obj.is_null());
    NSTDPtr { raw: obj, size }
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_size(ptr: &NSTDPtr) -> NSTDUInt {
    ptr.size
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_get(ptr: &NSTDPtr) -> NSTDAny {
    ptr.raw
}

/// A sized pointer to some arbitrary type.
///
/// # Safety
///
/// The user of this structure must ensure that the pointed-to data remains valid, unmodified, and
/// unreferenced in any other code while an instance of this structure is in use, else data races
/// may occur.
#[repr(C)]
#[derive(Debug)]
pub struct NSTDPtrMut {
    /// A raw pointer to the data.
    raw: NSTDAnyMut,
    /// The size of the object being pointed to.
    size: NSTDUInt,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_new(obj: NSTDAnyMut, size: NSTDUInt) -> NSTDPtrMut {
    assert!(!obj.is_null());
    NSTDPtrMut { raw: obj, size }
}

/// Creates an immutable version of a mutable pointer.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The mutable pointer.
///
/// # Returns
///
/// `NSTDPtr ptr_const` - The immutable copy of `ptr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_as_const(ptr: &NSTDPtrMut) -> NSTDPtr {
    nstd_core_ptr_new(ptr.raw, ptr.size)
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_size(ptr: &NSTDPtrMut) -> NSTDUInt {
    ptr.size
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_get(ptr: &mut NSTDPtrMut) -> NSTDAnyMut {
    ptr.raw
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_get_const(ptr: &NSTDPtrMut) -> NSTDAny {
    ptr.raw
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_ptr_mut_write(ptr: &mut NSTDPtrMut, obj: NSTDAny) {
    nstd_core_mem_copy(ptr.raw.cast(), obj.cast(), ptr.size);
}
