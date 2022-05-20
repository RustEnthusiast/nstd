#ifndef NSTD_CORE_MATH_H_INCLUDED
#define NSTD_CORE_MATH_H_INCLUDED
#include "../nstd.h"
#include "def.h"
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
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The value to clamp.
///
/// - `NSTDUInt8 min` - The minimum clamp value.
///
/// - `NSTDUInt8 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDUInt8 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDUInt8 nstd_core_math_clamp_u8(NSTDUInt8 x, NSTDUInt8 min, NSTDUInt8 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value to clamp.
///
/// - `NSTDInt8 min` - The minimum clamp value.
///
/// - `NSTDInt8 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDInt8 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDInt8 nstd_core_math_clamp_i8(NSTDInt8 x, NSTDInt8 min, NSTDInt8 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The value to clamp.
///
/// - `NSTDUInt16 min` - The minimum clamp value.
///
/// - `NSTDUInt16 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDUInt16 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDUInt16 nstd_core_math_clamp_u16(NSTDUInt16 x, NSTDUInt16 min, NSTDUInt16 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value to clamp.
///
/// - `NSTDInt16 min` - The minimum clamp value.
///
/// - `NSTDInt16 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDInt16 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDInt16 nstd_core_math_clamp_i16(NSTDInt16 x, NSTDInt16 min, NSTDInt16 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The value to clamp.
///
/// - `NSTDUInt32 min` - The minimum clamp value.
///
/// - `NSTDUInt32 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDUInt32 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDUInt32 nstd_core_math_clamp_u32(NSTDUInt32 x, NSTDUInt32 min, NSTDUInt32 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value to clamp.
///
/// - `NSTDInt32 min` - The minimum clamp value.
///
/// - `NSTDInt32 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDInt32 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDInt32 nstd_core_math_clamp_i32(NSTDInt32 x, NSTDInt32 min, NSTDInt32 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The value to clamp.
///
/// - `NSTDUInt64 min` - The minimum clamp value.
///
/// - `NSTDUInt64 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDUInt64 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDUInt64 nstd_core_math_clamp_u64(NSTDUInt64 x, NSTDUInt64 min, NSTDUInt64 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value to clamp.
///
/// - `NSTDInt64 min` - The minimum clamp value.
///
/// - `NSTDInt64 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDInt64 v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDInt64 nstd_core_math_clamp_i64(NSTDInt64 x, NSTDInt64 min, NSTDInt64 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDUSize x` - The value to clamp.
///
/// - `NSTDUSize min` - The minimum clamp value.
///
/// - `NSTDUSize max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDUSize v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDUSize nstd_core_math_clamp_usize(NSTDUSize x, NSTDUSize min, NSTDUSize max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDISize x` - The value to clamp.
///
/// - `NSTDISize min` - The minimum clamp value.
///
/// - `NSTDISize max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDISize v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
NSTDAPI NSTDISize nstd_core_math_clamp_isize(NSTDISize x, NSTDISize min, NSTDISize max);

NSTDCPPEND
#endif
