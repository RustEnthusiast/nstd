#ifndef NSTD_MATH_H
#define NSTD_MATH_H
#include "nstd.h"
NSTDCPPSTART

/// Returns the absolute value of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 abs` - The absolute value of `x`.
NSTDAPI NSTDFloat32 nstd_math_abs_f32(NSTDFloat32 x);
/// Returns the absolute value of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 abs` - The absolute value of `x`.
NSTDAPI NSTDFloat64 nstd_math_abs_f64(NSTDFloat64 x);

/// Rounds the value `x` down to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 value` - The value rounded down to the nearest integral value.
NSTDAPI NSTDFloat32 nstd_math_floor_f32(NSTDFloat32 x);
/// Rounds the value `x` down to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 value` - The value rounded down to the nearest integral value.
NSTDAPI NSTDFloat64 nstd_math_floor_f64(NSTDFloat64 x);

/// Rounds the value `x` up to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 value` - The value rounded up to the nearest integral value.
NSTDAPI NSTDFloat32 nstd_math_ceil_f32(NSTDFloat32 x);
/// Rounds the value `x` up to the closest integral value.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 value` - The value rounded up to the nearest integral value.
NSTDAPI NSTDFloat64 nstd_math_ceil_f64(NSTDFloat64 x);

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
NSTDAPI NSTDFloat32 nstd_math_pow_f32(NSTDFloat32 x, NSTDInt32 exp);
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
NSTDAPI NSTDFloat64 nstd_math_pow_f64(NSTDFloat64 x, NSTDInt32 exp);

/// Computes the square root of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 sqrt` - The square root of `x`.
NSTDAPI NSTDFloat32 nstd_math_sqrt_f32(NSTDFloat32 x);
/// Computes the square root of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 sqrt` - The square root of `x`.
NSTDAPI NSTDFloat64 nstd_math_sqrt_f64(NSTDFloat64 x);

/// Computes the sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 sin` - The sine value of `x`.
NSTDAPI NSTDFloat32 nstd_math_sin_f32(NSTDFloat32 x);
/// Computes the sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 sin` - The sine value of `x`.
NSTDAPI NSTDFloat64 nstd_math_sin_f64(NSTDFloat64 x);

/// Computes the cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 cos` - The cosine value of `x`.
NSTDAPI NSTDFloat32 nstd_math_cos_f32(NSTDFloat32 x);
/// Computes the cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 cos` - The cosine value of `x`.
NSTDAPI NSTDFloat64 nstd_math_cos_f64(NSTDFloat64 x);

/// Computes the tangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 tan` - The tangent value of `x`.
NSTDAPI NSTDFloat32 nstd_math_tan_f32(NSTDFloat32 x);
/// Computes the tangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 tan` - The tangent value of `x`.
NSTDAPI NSTDFloat64 nstd_math_tan_f64(NSTDFloat64 x);

/// Computes the arcsine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 asin` - The arcsine value of `x`.
NSTDAPI NSTDFloat32 nstd_math_asin_f32(NSTDFloat32 x);
/// Computes the arcsine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 asin` - The arcsine value of `x`.
NSTDAPI NSTDFloat64 nstd_math_asin_f64(NSTDFloat64 x);

/// Computes the arccosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 acos` - The arccosine value of `x`.
NSTDAPI NSTDFloat32 nstd_math_acos_f32(NSTDFloat32 x);
/// Computes the arccosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 acos` - The arccosine value of `x`.
NSTDAPI NSTDFloat64 nstd_math_acos_f64(NSTDFloat64 x);

/// Computes the arctangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 atan` - The arctangent value of `x`.
NSTDAPI NSTDFloat32 nstd_math_atan_f32(NSTDFloat32 x);
/// Computes the arctangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 atan` - The arctangent value of `x`.
NSTDAPI NSTDFloat64 nstd_math_atan_f64(NSTDFloat64 x);

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
NSTDAPI NSTDFloat32 nstd_math_atan2_f32(NSTDFloat32 x, NSTDFloat32 y);
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
NSTDAPI NSTDFloat64 nstd_math_atan2_f64(NSTDFloat64 x, NSTDFloat64 y);

/// Computes the hyperbolic sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 sinh` - The hyperbolic sine of `x`.
NSTDAPI NSTDFloat32 nstd_math_sinh_f32(NSTDFloat32 x);
/// Computes the hyperbolic sine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 sinh` - The hyperbolic sine of `x`.
NSTDAPI NSTDFloat64 nstd_math_sinh_f64(NSTDFloat64 x);

/// Computes the hyperbolic cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 cosh` - The hyperbolic cosine of `x`.
NSTDAPI NSTDFloat32 nstd_math_cosh_f32(NSTDFloat32 x);
/// Computes the hyperbolic cosine of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 cosh` - The hyperbolic cosine of `x`.
NSTDAPI NSTDFloat64 nstd_math_cosh_f64(NSTDFloat64 x);

/// Computes the hyperbolic tangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value.
///
/// # Returns
///
/// `NSTDFloat32 tanh` - The hyperbolic tangent of `x`.
NSTDAPI NSTDFloat32 nstd_math_tanh_f32(NSTDFloat32 x);
/// Computes the hyperbolic tangent of `x`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value.
///
/// # Returns
///
/// `NSTDFloat64 tanh` - The hyperbolic tangent of `x`.
NSTDAPI NSTDFloat64 nstd_math_tanh_f64(NSTDFloat64 x);

NSTDCPPEND
#endif
