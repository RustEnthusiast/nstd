#ifndef NSTD_MATH_H
#define NSTD_MATH_H
#include "nstd.h"
NSTDCPPSTART

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

NSTDCPPEND
#endif
