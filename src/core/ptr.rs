//! A sized pointer to some arbitrary type.
use crate::{
    core::{
        mem::{nstd_core_mem_copy, nstd_core_mem_is_aligned},
        optional::{gen_optional, NSTDOptional},
    },
    NSTDAny, NSTDAnyMut, NSTDUInt, NSTD_INT_MAX,
};
use nstdapi::nstdapi;

/// A sized immutable pointer to some arbitrary type.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDPtr {
    /// A raw pointer to the data.
    raw: NSTDAny,
    /// The size of the object being pointed to.
    size: NSTDUInt,
    /// The alignment of the object being pointed to.
    align: NSTDUInt,
}
gen_optional!(NSTDOptionalPtr, NSTDPtr);

/// Creates a new instance of `NSTDPtr`.
///
/// # Parameters:
///
/// - `NSTDAny obj` - The object to point to.
///
/// - `NSTDUInt size` - The number of bytes that `obj`'s type occupies.
///
/// - `NSTDUInt align` - The alignment of the object that `obj` points to.
///
/// # Returns
///
/// `NSTDOptionalPtr ptr` - A new instance of `NSTDPtr` that points to `obj` on success, or
/// an uninitialized "none" variant if `obj` is null or unaligned or if `size` is greater than
/// `NSTDInt`'s max value.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two.
#[inline]
#[nstdapi]
pub fn nstd_core_ptr_new(obj: NSTDAny, size: NSTDUInt, align: NSTDUInt) -> NSTDOptionalPtr {
    match !obj.is_null() && nstd_core_mem_is_aligned(obj, align) && size <= NSTD_INT_MAX {
        true => NSTDOptional::Some(NSTDPtr {
            raw: obj,
            size,
            align,
        }),
        false => NSTDOptional::None,
    }
}

/// Creates a new instance of `NSTDPtr` without checking if `obj` is null.
///
/// # Parameters:
///
/// - `NSTDAny obj` - The object to point to.
///
/// - `NSTDUInt size` - The number of bytes that `obj`'s type occupies.
///
/// - `NSTDUInt align` - The alignment of the object that `obj` points to.
///
/// # Returns
///
/// `NSTDPtr ptr` - A new instance of `NSTDPtr` that points to `obj`.
///
/// # Safety
///
/// - `obj` must be non-null.
///
/// - `obj` must be aligned to `align`.
///
/// - `align` must be a nonzero power of two.
///
/// - `size` must not be greater than `NSTDInt`'s max value.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_ptr_new_unchecked(
    obj: NSTDAny,
    size: NSTDUInt,
    align: NSTDUInt,
) -> NSTDPtr {
    NSTDPtr {
        raw: obj,
        size,
        align,
    }
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
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<isize>();
///     const ALIGN: usize = core::mem::align_of::<isize>();
///     let x = 33isize;
///     let ptr = nstd_core_ptr_new(addr_of!(x).cast(), SIZE, ALIGN).unwrap();
///     assert!(nstd_core_ptr_size(&ptr) == SIZE);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_size(ptr: &NSTDPtr) -> NSTDUInt {
    ptr.size
}

/// Returns the alignment of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtr *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUInt align` - The alignment of the object pointed to by `ptr`.
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of;
/// use nstd_sys::core::ptr::{nstd_core_ptr_align, nstd_core_ptr_new};
///
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<[u8; 32]>();
///     const ALIGN: usize = core::mem::align_of::<[u8; 32]>();
///     let x = [33u8; 32];
///     let ptr = nstd_core_ptr_new(addr_of!(x).cast(), SIZE, ALIGN).unwrap();
///     assert!(nstd_core_ptr_align(&ptr) == ALIGN);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_align(ptr: &NSTDPtr) -> NSTDUInt {
    ptr.align
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
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<u32>();
///     const ALIGN: usize = core::mem::align_of::<u32>();
///     let x = 45u32;
///     let ptr = nstd_core_ptr_new(addr_of!(x).cast(), SIZE, ALIGN).unwrap();
///     assert!(*nstd_core_ptr_get(&ptr).cast::<u32>() == x);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_get(ptr: &NSTDPtr) -> NSTDAny {
    ptr.raw
}

/// A sized pointer to some arbitrary type.
#[nstdapi]
pub struct NSTDPtrMut {
    /// A raw pointer to the data.
    raw: NSTDAnyMut,
    /// The size of the object being pointed to.
    size: NSTDUInt,
    /// The alignment of the object being pointed to.
    align: NSTDUInt,
}
gen_optional!(NSTDOptionalPtrMut, NSTDPtrMut);

/// Creates a new instance of `NSTDPtrMut`.
///
/// # Parameters:
///
/// - `NSTDAnyMut obj` - The object to point to.
///
/// - `NSTDUInt size` - The number of bytes that `obj`'s type occupies.
///
/// - `NSTDUInt align` - The alignment of the object that `obj` points to.
///
/// # Returns
///
/// `NSTDOptionalPtrMut ptr` - A new instance of `NSTDPtrMut` that points to `obj` on success, or
/// an uninitialized "none" variant if `obj` is null or unaligned or if `size` is greater than
/// `NSTDInt`'s max value.
///
/// # Panics
///
/// This operation will panic if `align` is not a power of two.
#[inline]
#[nstdapi]
pub fn nstd_core_ptr_mut_new(
    obj: NSTDAnyMut,
    size: NSTDUInt,
    align: NSTDUInt,
) -> NSTDOptionalPtrMut {
    match !obj.is_null() && nstd_core_mem_is_aligned(obj, align) && size <= NSTD_INT_MAX {
        true => NSTDOptional::Some(NSTDPtrMut {
            raw: obj,
            size,
            align,
        }),
        false => NSTDOptional::None,
    }
}

/// Creates a new instance of `NSTDPtrMut` without checking if `obj` is null.
///
/// # Parameters:
///
/// - `NSTDAnyMut obj` - The object to point to.
///
/// - `NSTDUInt size` - The number of bytes that `obj`'s type occupies.
///
/// - `NSTDUInt align` - The alignment of the object that `obj` points to.
///
/// # Returns
///
/// `NSTDPtrMut ptr` - A new instance of `NSTDPtrMut` that points to `obj`.
///
/// # Safety
///
/// - `obj` must be non-null.
///
/// - `obj` must be aligned to `align`.
///
/// - `align` must be a nonzero power of two.
///
/// - `size` must not be greater than `NSTDInt`'s max value.
#[inline]
#[nstdapi]
pub const unsafe fn nstd_core_ptr_mut_new_unchecked(
    obj: NSTDAnyMut,
    size: NSTDUInt,
    align: NSTDUInt,
) -> NSTDPtrMut {
    NSTDPtrMut {
        raw: obj,
        size,
        align,
    }
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
#[nstdapi]
pub const fn nstd_core_ptr_mut_as_const(ptr: &NSTDPtrMut) -> NSTDPtr {
    // SAFETY: `ptr.raw` is never null, `ptr.size` is never greater than `NSTDInt`'s max value.
    unsafe { nstd_core_ptr_new_unchecked(ptr.raw, ptr.size, ptr.align) }
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
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<isize>();
///     const ALIGN: usize = core::mem::align_of::<isize>();
///     let mut x = 33isize;
///     let ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), SIZE, ALIGN).unwrap();
///     assert!(nstd_core_ptr_mut_size(&ptr) == SIZE);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_mut_size(ptr: &NSTDPtrMut) -> NSTDUInt {
    ptr.size
}

/// Returns the alignment of the object being pointed to.
///
/// # Parameters:
///
/// - `const NSTDPtrMut *ptr` - The pointer.
///
/// # Returns
///
/// `NSTDUInt align` - The alignment of the object pointed to by `ptr`.
///
/// # Examples
///
/// ```
/// use core::ptr::addr_of_mut;
/// use nstd_sys::core::ptr::{nstd_core_ptr_mut_align, nstd_core_ptr_mut_new};
///
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<[u8; 32]>();
///     const ALIGN: usize = core::mem::align_of::<[u8; 32]>();
///     let mut x = [33u8; 32];
///     let ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), SIZE, ALIGN).unwrap();
///     assert!(nstd_core_ptr_mut_align(&ptr) == ALIGN);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_mut_align(ptr: &NSTDPtrMut) -> NSTDUInt {
    ptr.align
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
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<u32>();
///     const ALIGN: usize = core::mem::align_of::<u32>();
///     let mut x = 8u32;
///     let mut ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), SIZE, ALIGN).unwrap();
///     let x_ptr = nstd_core_ptr_mut_get(&mut ptr).cast();
///     *x_ptr *= 2;
///     assert!(x == *x_ptr);
/// }
/// ```
#[inline]
#[nstdapi]
pub fn nstd_core_ptr_mut_get(ptr: &mut NSTDPtrMut) -> NSTDAnyMut {
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
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<u32>();
///     const ALIGN: usize = core::mem::align_of::<u32>();
///     let mut x = 45u32;
///     let ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), SIZE, ALIGN).unwrap();
///     assert!(*nstd_core_ptr_mut_get_const(&ptr).cast::<u32>() == x);
/// }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_ptr_mut_get_const(ptr: &NSTDPtrMut) -> NSTDAny {
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
/// unsafe {
///     const SIZE: usize = core::mem::size_of::<i64>();
///     const ALIGN: usize = core::mem::align_of::<i64>();
///     let mut x = -69i64;
///     let mut ptr = nstd_core_ptr_mut_new(addr_of_mut!(x).cast(), SIZE, ALIGN).unwrap();
///     let y = 420i64;
///     nstd_core_ptr_mut_write(&mut ptr, addr_of!(y).cast());
///     assert!(x == y);
/// }
/// ```
#[inline]
#[nstdapi]
pub unsafe fn nstd_core_ptr_mut_write(ptr: &mut NSTDPtrMut, obj: NSTDAny) {
    nstd_core_mem_copy(ptr.raw.cast(), obj.cast(), ptr.size);
}
