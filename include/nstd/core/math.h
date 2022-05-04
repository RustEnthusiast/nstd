#ifndef NSTD_CORE_MATH_H_INCLUDED
#define NSTD_CORE_MATH_H_INCLUDED
#include "../core/def.h"
#include "../nstd.h"
NSTDCPPSTART

/// Converts radians to degrees.
///
/// # Parameters:
///
/// - `NSTDFloat32 rad` - The radians value.
///
/// # Returns
///
/// `NSTDFloat32 deg` - The radians value converted to degrees.
NSTDAPI NSTDFloat32 nstd_core_math_deg_f32(NSTDFloat32 rad);
/// Converts radians to degrees.
///
/// # Parameters:
///
/// - `NSTDFloat64 rad` - The radians value.
///
/// # Returns
///
/// `NSTDFloat64 deg` - The radians value converted to degrees.
NSTDAPI NSTDFloat64 nstd_core_math_deg_f64(NSTDFloat64 rad);

/// Converts degrees to radians.
///
/// # Parameters:
///
/// - `NSTDFloat32 deg` - The degrees value.
///
/// # Returns
///
/// `NSTDFloat32 rad` - The degrees value converted to radians.
NSTDAPI NSTDFloat32 nstd_core_math_rad_f32(NSTDFloat32 deg);
/// Converts degrees to radians.
///
/// # Parameters:
///
/// - `NSTDFloat64 deg` - The degrees value.
///
/// # Returns
///
/// `NSTDFloat64 rad` - The degrees value converted to radians.
NSTDAPI NSTDFloat64 nstd_core_math_rad_f64(NSTDFloat64 deg);

/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value to clamp.
///
/// - `NSTDFloat32 min` - The minimum clamp value.
///
/// - `NSTDFloat32 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDFloat32 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`, `min` is NaN, or `max` is NaN.
NSTDAPI NSTDFloat32 nstd_core_math_clamp_f32(NSTDFloat32 x, NSTDFloat32 min, NSTDFloat32 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value to clamp.
///
/// - `NSTDFloat64 min` - The minimum clamp value.
///
/// - `NSTDFloat64 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDFloat64 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`, `min` is NaN, or `max` is NaN.
NSTDAPI NSTDFloat64 nstd_core_math_clamp_f64(NSTDFloat64 x, NSTDFloat64 min, NSTDFloat64 max);

NSTDCPPEND
#endif
