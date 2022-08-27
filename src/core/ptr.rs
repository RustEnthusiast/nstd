//! A sized pointer to some arbitrary type.
use crate::{core::mem::nstd_core_mem_copy, NSTDAny, NSTDAnyMut, NSTDUInt};

/// A sized immutable pointer to some arbitrary type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NSTDPtr {
    /// A raw pointer to the data.
    pub(crate) raw: NSTDAny,
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_new(obj: NSTDAny, size: NSTDUInt) -> NSTDPtr {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_get(ptr: &NSTDPtr) -> NSTDAny {
    ptr.raw
}

/// A sized pointer to some arbitrary type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NSTDPtrMut {
    /// A raw pointer to the data.
    pub(crate) raw: NSTDAnyMut,
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_new(obj: NSTDAnyMut, size: NSTDUInt) -> NSTDPtrMut {
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_ptr_mut_write(ptr: &mut NSTDPtrMut, obj: NSTDAny) {
    nstd_core_mem_copy(ptr.raw.cast(), obj.cast(), ptr.size);
}
