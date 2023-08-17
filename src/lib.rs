#![doc = include_str!("../README.md")]
#![warn(
    deprecated_in_future,
    ffi_unwind_calls,
    future_incompatible,
    let_underscore,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    non_ascii_idents,
    nonstandard_style,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences,
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::undocumented_unsafe_blocks
)]
#![allow(
    clippy::match_bool,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::significant_drop_in_scrutinee
)]
#![cfg_attr(feature = "link", allow(dead_code, unused_imports))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
pub mod alloc;
#[cfg(feature = "core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "core")))]
pub mod core;
#[cfg(feature = "cstring")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "cstring")))]
pub mod cstring;
#[cfg(feature = "env")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "env")))]
pub mod env;
#[cfg(feature = "fs")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fs")))]
pub mod fs;
#[cfg(feature = "heap_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "heap_ptr")))]
pub mod heap_ptr;
#[cfg(feature = "io")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "io")))]
pub mod io;
#[cfg(feature = "math")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "math")))]
pub mod math;
#[cfg(feature = "mutex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "mutex")))]
pub mod mutex;
#[cfg(feature = "os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "os")))]
pub mod os;
#[cfg(feature = "proc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proc")))]
pub mod proc;
#[cfg(feature = "shared_lib")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "shared_lib")))]
pub mod shared_lib;
#[cfg(feature = "shared_ptr")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "shared_ptr")))]
pub mod shared_ptr;
#[cfg(feature = "string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "string")))]
pub mod string;
#[cfg(test)]
pub(crate) mod test;
#[cfg(feature = "thread")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "thread")))]
pub mod thread;
#[cfg(feature = "time")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "time")))]
pub mod time;
#[cfg(feature = "timed_mutex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "timed_mutex")))]
pub mod timed_mutex;
#[cfg(feature = "vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "vec")))]
pub mod vec;
use ::core::{
    ffi::{c_char, c_void},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::{addr_of, addr_of_mut},
};

/// [`NSTDInt`]'s maximum value.
#[allow(dead_code)]
const NSTD_INT_MAX: NSTDInt = NSTDInt::MAX;

/// A null pointer value constant.
pub const NSTD_NULL: NSTDAnyMut = ::core::ptr::null_mut();

/// Boolean value false (0).
pub const NSTD_FALSE: NSTDBool = false;
/// Boolean value true (1).
pub const NSTD_TRUE: NSTDBool = true;

/// An integral type who's size matches the target architecture's pointer size.
pub type NSTDInt = isize;
/// An unsigned integral type who's size matches the target architecture's pointer size.
pub type NSTDUInt = usize;

/// An 8-bit signed integer type.
pub type NSTDInt8 = i8;
/// An 8-bit unsigned integer type.
pub type NSTDUInt8 = u8;
/// A 16-bit signed integer type.
pub type NSTDInt16 = i16;
/// A 16-bit unsigned integer type.
pub type NSTDUInt16 = u16;
/// A 32-bit signed integer type.
pub type NSTDInt32 = i32;
/// A 32-bit unsigned integer type.
pub type NSTDUInt32 = u32;
/// A 64-bit signed integer type.
pub type NSTDInt64 = i64;
/// A 64-bit unsigned integer type.
pub type NSTDUInt64 = u64;

/// A 32-bit floating point type.
pub type NSTDFloat32 = f32;
/// A 64-bit floating point type.
pub type NSTDFloat64 = f64;

/// Equivalent to C's `char` type.
pub type NSTDChar = c_char;
/// An 8-bit character type.
pub type NSTDChar8 = NSTDUInt8;
/// A 16-bit character type.
pub type NSTDChar16 = NSTDUInt16;
/// A 32-bit character type.
pub type NSTDChar32 = NSTDUInt32;

/// A boolean type, can either be `NSTD_TRUE` (1) or `NSTD_FALSE` (0).
pub type NSTDBool = bool;

/// An opaque pointer to some immutable data.
pub type NSTDAny = *const c_void;
/// An opaque pointer to some mutable data.
pub type NSTDAnyMut = *mut c_void;

/// An FFI-safe reference to some immutable data.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct NSTDRef<'a, T>(&'a c_void, PhantomData<&'a T>);
impl<T> Deref for NSTDRef<'_, T> {
    /// [`NSTDRef`]'s dereference target.
    type Target = T;

    /// Gets the immutable reference.
    #[inline]
    fn deref(&self) -> &T {
        let ptr: *const c_void = self.0;
        // SAFETY: `self.0` is of type `&T`.
        unsafe { &*(ptr.cast()) }
    }
}
impl<'a, T> From<&'a T> for NSTDRef<'a, T> {
    /// Creates a new FFI-safe reference.
    #[inline]
    fn from(value: &'a T) -> Self {
        // SAFETY: Reference to reference transmute.
        Self(unsafe { &*(addr_of!(*value).cast()) }, PhantomData)
    }
}
/// An FFI-safe reference to some mutable data.
#[repr(transparent)]
pub struct NSTDRefMut<'a, T>(&'a mut c_void, PhantomData<&'a mut T>);
impl<T> Deref for NSTDRefMut<'_, T> {
    /// [`NSTDRefMut`]'s dereference target.
    type Target = T;

    /// Gets the reference.
    #[inline]
    fn deref(&self) -> &T {
        let ptr: *const c_void = self.0;
        // SAFETY: `self.0` is of type `&mut T`.
        unsafe { &*(ptr.cast()) }
    }
}
impl<T> DerefMut for NSTDRefMut<'_, T> {
    /// Gets the mutable reference.
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        let ptr: *mut c_void = self.0;
        // SAFETY: `self.0` is of type `&mut T`.
        unsafe { &mut *(ptr.cast()) }
    }
}
impl<'a, T> From<&'a mut T> for NSTDRefMut<'a, T> {
    /// Creates a new FFI-safe reference.
    #[inline]
    fn from(value: &'a mut T) -> Self {
        // SAFETY: Reference to reference transmute.
        Self(unsafe { &mut *addr_of_mut!(*value).cast() }, PhantomData)
    }
}

/// An FFI-safe reference to some immutable data, without type safety.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct NSTDAnyRef<'a>(&'a c_void);
impl NSTDAnyRef<'_> {
    /// Gets the immutable reference.
    ///
    /// # Safety
    ///
    /// This reference must be pointing to an object of type `T`.
    #[inline]
    pub const unsafe fn get<T>(&self) -> &T {
        let ptr: *const c_void = self.0;
        &*ptr.cast()
    }
}
impl<'a, T> From<&'a T> for NSTDAnyRef<'a> {
    /// Creates a new FFI-safe reference.
    #[inline]
    fn from(value: &'a T) -> Self {
        // SAFETY: Reference to reference transmute.
        Self(unsafe { &*(addr_of!(*value).cast()) })
    }
}
/// An FFI-safe reference to some mutable data, without type safety.
#[repr(transparent)]
pub struct NSTDAnyRefMut<'a>(&'a mut c_void);
impl NSTDAnyRefMut<'_> {
    /// Gets an immutable reference to the data.
    ///
    /// # Safety
    ///
    /// This reference must be pointing to an object of type `T`.
    #[inline]
    pub const unsafe fn get<T>(&self) -> &T {
        let ptr: *const c_void = self.0;
        &*ptr.cast()
    }

    /// Gets the mutable reference.
    ///
    /// # Safety
    ///
    /// This reference must be pointing to an object of type `T`.
    #[inline]
    pub unsafe fn get_mut<T>(&mut self) -> &mut T {
        let ptr: *mut c_void = self.0;
        &mut *ptr.cast()
    }
}
impl<'a, T> From<&'a mut T> for NSTDAnyRefMut<'a> {
    /// Creates a new FFI-safe reference.
    #[inline]
    fn from(value: &'a mut T) -> Self {
        // SAFETY: Reference to reference transmute.
        Self(unsafe { &mut *addr_of_mut!(*value).cast() })
    }
}
