//! A sized pointer to some arbitrary type.
use crate::{core::mem::nstd_core_mem_copy, NSTDAnyConst, NSTDAnyMut, NSTDUSize};

/// A sized immutable pointer to some arbitrary type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NSTDPtrConst {
    /// A raw pointer to the data.
    pub(crate) raw: NSTDAnyConst,
    /// The size of the object being pointed to.
    size: NSTDUSize,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_const_new(obj: NSTDAnyConst, size: NSTDUSize) -> NSTDPtrConst {
    NSTDPtrConst { raw: obj, size }
}

/// Returns the size of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtrConst *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the object pointed to by `ptr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_const_size(ptr: &NSTDPtrConst) -> NSTDUSize {
    ptr.size
}

/// Returns a raw immutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtrConst *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAnyConst raw` - A raw pointer to the object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_const_get(ptr: &NSTDPtrConst) -> NSTDAnyConst {
    ptr.raw
}

/// A sized pointer to some arbitrary type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NSTDPtrMut {
    /// A raw pointer to the data.
    pub(crate) raw: NSTDAnyMut,
    /// The size of the object being pointed to.
    size: NSTDUSize,
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_new(obj: NSTDAnyMut, size: NSTDUSize) -> NSTDPtrMut {
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
/// `NSTDPtrConst ptr_const` - The immutable copy of `ptr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_as_const(ptr: &NSTDPtrMut) -> NSTDPtrConst {
    nstd_core_ptr_const_new(ptr.raw, ptr.size)
}

/// Returns the size of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUSize size` - The size of the object pointed to by `ptr`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_size(ptr: &NSTDPtrMut) -> NSTDUSize {
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
/// `NSTDAnyConst raw` - A raw pointer to the object.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_mut_get_const(ptr: &NSTDPtrMut) -> NSTDAnyConst {
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
/// - `NSTDAnyConst obj` - A pointer to the object to write to `ptr`.
///
/// # Safety
///
/// This operation is highly unsafe because there is no way of knowing if `obj`'s data is valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_ptr_mut_write(ptr: &mut NSTDPtrMut, obj: NSTDAnyConst) {
    nstd_core_mem_copy(ptr.raw.cast(), obj.cast(), ptr.size);
}
