//! High level math operations.
//!
//! This library provides access to math functions that require the use of the "std" feature.
use crate::{NSTDFloat32, NSTDFloat64, NSTDInt32};

/// Raises `x` to an integral power.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// - `NSTDInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDFloat32 pow` - `x` raised to the power of `exp`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_pow_f32(x: NSTDFloat32, exp: NSTDInt32) -> NSTDFloat32 {
    x.powi(exp)
}
/// Raises `x` to an integral power.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// - `NSTDInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDFloat64 pow` - `x` raised to the power of `exp`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_pow_f64(x: NSTDFloat64, exp: NSTDInt32) -> NSTDFloat64 {
    x.powi(exp)
}

/// Computes the square root of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 sqrt` - The square root of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_sqrt_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.sqrt()
}
/// Computes the square root of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 sqrt` - The square root of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_sqrt_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.sqrt()
}
