//! High level math operations.
//!
//! This library provides access to math functions that require the use of the "std" feature.
use crate::{NSTDFloat32, NSTDFloat64, NSTDInt32};

/// Rounds the value `x` down to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 value` - The value rounded down to the nearest integral value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_floor_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.floor()
}
/// Rounds the value `x` down to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 value` - The value rounded down to the nearest integral value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_floor_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.floor()
}

/// Rounds the value `x` up to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 value` - The value rounded up to the nearest integral value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_ceil_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.ceil()
}
/// Rounds the value `x` up to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 value` - The value rounded up to the nearest integral value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_ceil_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.ceil()
}

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

/// Computes the sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 sin` - The sine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_sin_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.sin()
}
/// Computes the sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 sin` - The sine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_sin_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.sin()
}

/// Computes the cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 cos` - The cosine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_cos_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.cos()
}
/// Computes the cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 cos` - The cosine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_cos_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.cos()
}

/// Computes the tangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 tan` - The tangent value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_tan_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.tan()
}
/// Computes the tangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 tan` - The tangent value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_tan_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.tan()
}

/// Computes the arc sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 asin` - The arc sine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_asin_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.asin()
}
/// Computes the arc sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 asin` - The arc sine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_asin_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.asin()
}

/// Computes the arc cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 acos` - The arc cosine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_acos_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.acos()
}
/// Computes the arc cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 acos` - The arc cosine value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_acos_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.acos()
}

/// Computes the arctangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 atan` - The arctangent value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_atan_f32(x: NSTDFloat32) -> NSTDFloat32 {
    x.atan()
}
/// Computes the arctangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 atan` - The arctangent value of `x`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_atan_f64(x: NSTDFloat64) -> NSTDFloat64 {
    x.atan()
}

/// Computes the four quadrant arctangent of `x` & `y`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// - `NSTDFloat32 y` - The second value.
///
/// # Returns
///
/// `NSTDFloat32 atan2` - The four quadrant arctangent of `x` & `y`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_atan2_f32(x: NSTDFloat32, y: NSTDFloat32) -> NSTDFloat32 {
    x.atan2(y)
}
/// Computes the four quadrant arctangent of `x` & `y`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// - `NSTDFloat64 y` - The second value.
///
/// # Returns
///
/// `NSTDFloat64 atan2` - The four quadrant arctangent of `x` & `y`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_math_atan2_f64(x: NSTDFloat64, y: NSTDFloat64) -> NSTDFloat64 {
    x.atan2(y)
}
