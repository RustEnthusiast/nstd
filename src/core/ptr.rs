//! A sized pointer to some arbitrary type.
use crate::core::{
    def::{NSTDAny, NSTDAnyConst, NSTDUSize},
    mem::nstd_core_mem_copy,
};

/// A sized pointer to some arbitrary type.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NSTDPtr {
    /// A raw pointer to the data.
    pub raw: NSTDAny,
    /// The size of the object being pointed to.
    pub size: NSTDUSize,
}

/// Creates a new instance of `NSTDPtr`.
///
/// # Parameters:
///
/// - `NSTDAny obj` - The object to point to.
///
/// - `NSTDUSize size` - The number of bytes that `obj`'s type occupies.
///
/// # Returns
///
/// `NSTDPtr ptr` - A new instance of `NSTDPtr` that points to `obj`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_ptr_new(obj: NSTDAny, size: NSTDUSize) -> NSTDPtr {
    NSTDPtr { raw: obj, size }
}

/// Returns a raw pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `NSTDPtr *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAny raw` - A raw pointer to the object.
///
/// # Safety
///
/// This operation is unsafe because there is no way of knowing if the object being pointed to is
/// still valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_ptr_read(ptr: &mut NSTDPtr) -> NSTDAny {
    ptr.raw
}

/// Returns a raw immutable pointer to the object pointed to by `ptr`.
///
/// # Parameters:
///
/// - `const NSTDPtr *ptr` - The higher level pointer.
///
/// # Returns
///
/// `NSTDAnyConst raw` - A raw pointer to the object.
///
/// # Safety
///
/// This operation is unsafe because there is no way of knowing if the object being pointed to is
/// still valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_ptr_read_const(ptr: &NSTDPtr) -> NSTDAnyConst {
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
/// - `NSTDPtr *ptr` - The pointer to write to.
///
/// - `NSTDAnyConst obj` - A pointer to the object to write to `ptr`.
///
/// # Safety
///
/// This operation is highly unsafe because there is no way of knowing if either of the pointers
/// are valid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_core_ptr_write(ptr: &mut NSTDPtr, obj: NSTDAnyConst) {
    nstd_core_mem_copy(ptr.raw.cast(), obj.cast(), ptr.size);
}
