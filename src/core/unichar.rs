//! A Unicode scalar value.
//!
//! This is a structure that wraps an `NSTDChar32` (Rust's `char` primitive is not FFI safe). This
//! is done so that an `NSTDUnichar` can be created once and used a number of times without
//! worrying about Unicode validity.
use crate::{
    core::optional::{gen_optional, NSTDOptional},
    NSTDChar32,
};

/// Represents a unicode scalar value.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NSTDUnichar {
    /// The 32-bit value.
    value: NSTDChar32,
}
impl From<char> for NSTDUnichar {
    /// Converts a Rust `char` into an `NSTDUnichar`.
    #[inline]
    fn from(value: char) -> Self {
        Self { value: value as _ }
    }
}
impl From<NSTDUnichar> for char {
    /// Converts an `NSTDUnichar` into a Rust `char`.
    #[inline]
    fn from(value: NSTDUnichar) -> Self {
        // SAFETY: `value` is always a valid Unicode scalar value.
        unsafe { char::from_u32_unchecked(value.value) }
    }
}
gen_optional!(NSTDOptionalUnichar, NSTDUnichar);

/// Creates a new `NSTDUnichar` from a 32-bit character value.
///
/// # Parameters:
///
/// - `NSTDChar32 value` - The 32-bit character to be converted into an `NSTDUnichar`.
///
/// # Returns
///
/// `NSTDOptionalUnichar unichar` - The new Unicode scalar value on success.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub const extern "C" fn nstd_core_unichar_new(value: NSTDChar32) -> NSTDOptionalUnichar {
    match char::from_u32(value) {
        Some(_) => NSTDOptional::Some(NSTDUnichar { value }),
        _ => NSTDOptional::None,
    }
}
