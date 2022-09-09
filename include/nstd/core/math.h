#ifndef NSTD_CORE_MATH_H
#define NSTD_CORE_MATH_H
#include "../nstd.h"

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

/// Computes the absolute (positive) value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The value.
///
/// # Returns
///
/// `NSTDInt abs` - The absolute value of `x`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_abs_int;
/// assert!(nstd_core_math_abs_int(10) == 10);
/// assert!(nstd_core_math_abs_int(-10) == 10);
/// ```
NSTDAPI NSTDInt nstd_core_math_abs_int(NSTDInt x);
/// Computes the absolute (positive) value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value.
///
/// # Returns
///
/// `NSTDInt8 abs` - The absolute value of `x`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_abs_i8;
/// assert!(nstd_core_math_abs_i8(10) == 10);
/// assert!(nstd_core_math_abs_i8(-10) == 10);
/// ```
NSTDAPI NSTDInt8 nstd_core_math_abs_i8(NSTDInt8 x);
/// Computes the absolute (positive) value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value.
///
/// # Returns
///
/// `NSTDInt16 abs` - The absolute value of `x`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_abs_i16;
/// assert!(nstd_core_math_abs_i16(10) == 10);
/// assert!(nstd_core_math_abs_i16(-10) == 10);
/// ```
NSTDAPI NSTDInt16 nstd_core_math_abs_i16(NSTDInt16 x);
/// Computes the absolute (positive) value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value.
///
/// # Returns
///
/// `NSTDInt32 abs` - The absolute value of `x`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_abs_i32;
/// assert!(nstd_core_math_abs_i32(10) == 10);
/// assert!(nstd_core_math_abs_i32(-10) == 10);
/// ```
NSTDAPI NSTDInt32 nstd_core_math_abs_i32(NSTDInt32 x);
/// Computes the absolute (positive) value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value.
///
/// # Returns
///
/// `NSTDInt64 abs` - The absolute value of `x`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_abs_i64;
/// assert!(nstd_core_math_abs_i64(10) == 10);
/// assert!(nstd_core_math_abs_i64(-10) == 10);
/// ```
NSTDAPI NSTDInt64 nstd_core_math_abs_i64(NSTDInt64 x);

/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDInt x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDInt pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_int;
/// assert!(nstd_core_math_pow_int(2, 3) == 8);
/// assert!(nstd_core_math_pow_int(4, 5) == 1024);
/// ```
NSTDAPI NSTDInt nstd_core_math_pow_int(NSTDInt x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDUInt x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDUInt pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_uint;
/// assert!(nstd_core_math_pow_uint(2, 3) == 8);
/// assert!(nstd_core_math_pow_uint(4, 5) == 1024);
/// ```
NSTDAPI NSTDUInt nstd_core_math_pow_uint(NSTDUInt x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDInt8 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_i8;
/// assert!(nstd_core_math_pow_i8(2, 3) == 8);
/// assert!(nstd_core_math_pow_i8(2, 5) == 32);
/// ```
NSTDAPI NSTDInt8 nstd_core_math_pow_i8(NSTDInt8 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDUInt8 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_u8;
/// assert!(nstd_core_math_pow_u8(2, 3) == 8);
/// assert!(nstd_core_math_pow_u8(2, 5) == 32);
/// ```
NSTDAPI NSTDUInt8 nstd_core_math_pow_u8(NSTDUInt8 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDInt16 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_i16;
/// assert!(nstd_core_math_pow_i16(2, 3) == 8);
/// assert!(nstd_core_math_pow_i16(2, 5) == 32);
/// ```
NSTDAPI NSTDInt16 nstd_core_math_pow_i16(NSTDInt16 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDUInt16 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_u16;
/// assert!(nstd_core_math_pow_u16(2, 3) == 8);
/// assert!(nstd_core_math_pow_u16(2, 5) == 32);
/// ```
NSTDAPI NSTDUInt16 nstd_core_math_pow_u16(NSTDUInt16 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDInt32 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_i32;
/// assert!(nstd_core_math_pow_i32(2, 3) == 8);
/// assert!(nstd_core_math_pow_i32(2, 5) == 32);
/// ```
NSTDAPI NSTDInt32 nstd_core_math_pow_i32(NSTDInt32 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDUInt32 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_u32;
/// assert!(nstd_core_math_pow_u32(2, 3) == 8);
/// assert!(nstd_core_math_pow_u32(2, 5) == 32);
/// ```
NSTDAPI NSTDUInt32 nstd_core_math_pow_u32(NSTDUInt32 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDInt64 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_i64;
/// assert!(nstd_core_math_pow_i64(2, 3) == 8);
/// assert!(nstd_core_math_pow_i64(2, 5) == 32);
/// ```
NSTDAPI NSTDInt64 nstd_core_math_pow_i64(NSTDInt64 x, NSTDUInt32 exp);
/// Raises the value `x` to the power of `exp`
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The value.
///
/// - `NSTDUInt32 exp` - The exponent.
///
/// # Returns
///
/// `NSTDUInt64 pow` - `x` raised to the power of `exp`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::math::nstd_core_math_pow_u64;
/// assert!(nstd_core_math_pow_u64(2, 3) == 8);
/// assert!(nstd_core_math_pow_u64(2, 5) == 32);
/// ```
NSTDAPI NSTDUInt64 nstd_core_math_pow_u64(NSTDUInt64 x, NSTDUInt32 exp);

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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDFloat64 nstd_core_math_clamp_f64(NSTDFloat64 x, NSTDFloat64 min, NSTDFloat64 max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The value to clamp.
///
/// - `NSTDInt min` - The minimum clamp value.
///
/// - `NSTDInt max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDInt v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt nstd_core_math_clamp_int(NSTDInt x, NSTDInt min, NSTDInt max);
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The value to clamp.
///
/// - `NSTDUInt min` - The minimum clamp value.
///
/// - `NSTDUInt max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDUInt v` - The clamped value.
///
/// # Panics
///
/// Panics if `min` > `max`.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt nstd_core_math_clamp_uint(NSTDUInt x, NSTDUInt min, NSTDUInt max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt8 nstd_core_math_clamp_i8(NSTDInt8 x, NSTDInt8 min, NSTDInt8 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt8 nstd_core_math_clamp_u8(NSTDUInt8 x, NSTDUInt8 min, NSTDUInt8 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt16 nstd_core_math_clamp_i16(NSTDInt16 x, NSTDInt16 min, NSTDInt16 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt16 nstd_core_math_clamp_u16(NSTDUInt16 x, NSTDUInt16 min, NSTDUInt16 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt32 nstd_core_math_clamp_i32(NSTDInt32 x, NSTDInt32 min, NSTDInt32 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt32 nstd_core_math_clamp_u32(NSTDUInt32 x, NSTDUInt32 min, NSTDUInt32 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt64 nstd_core_math_clamp_i64(NSTDInt64 x, NSTDInt64 min, NSTDInt64 max);
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
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt64 nstd_core_math_clamp_u64(NSTDUInt64 x, NSTDUInt64 min, NSTDUInt64 max);

/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt x` - The first value.
///
/// - `NSTDInt y` - The second value.
///
/// # Returns
///
/// `NSTDInt v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt nstd_core_math_div_ceil_int(NSTDInt x, NSTDInt y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The first value.
///
/// - `NSTDUInt y` - The second value.
///
/// # Returns
///
/// `NSTDUInt v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt nstd_core_math_div_ceil_uint(NSTDUInt x, NSTDUInt y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The first value.
///
/// - `NSTDInt8 y` - The second value.
///
/// # Returns
///
/// `NSTDInt8 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt8 nstd_core_math_div_ceil_i8(NSTDInt8 x, NSTDInt8 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The first value.
///
/// - `NSTDUInt8 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt8 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt8 nstd_core_math_div_ceil_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The first value.
///
/// - `NSTDInt16 y` - The second value.
///
/// # Returns
///
/// `NSTDInt16 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt16 nstd_core_math_div_ceil_i16(NSTDInt16 x, NSTDInt16 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The first value.
///
/// - `NSTDUInt16 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt16 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt16 nstd_core_math_div_ceil_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The first value.
///
/// - `NSTDInt32 y` - The second value.
///
/// # Returns
///
/// `NSTDInt32 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt32 nstd_core_math_div_ceil_i32(NSTDInt32 x, NSTDInt32 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The first value.
///
/// - `NSTDUInt32 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt32 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt32 nstd_core_math_div_ceil_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The first value.
///
/// - `NSTDInt64 y` - The second value.
///
/// # Returns
///
/// `NSTDInt64 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt64 nstd_core_math_div_ceil_i64(NSTDInt64 x, NSTDInt64 y);
/// Divides two numbers and rounds the result up to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The first value.
///
/// - `NSTDUInt64 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt64 v` - The divided value, rounded up.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt64 nstd_core_math_div_ceil_u64(NSTDUInt64 x, NSTDUInt64 y);

/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt x` - The first value.
///
/// - `NSTDInt y` - The second value.
///
/// # Returns
///
/// `NSTDInt v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt nstd_core_math_div_floor_int(NSTDInt x, NSTDInt y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The first value.
///
/// - `NSTDUInt y` - The second value.
///
/// # Returns
///
/// `NSTDUInt v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt nstd_core_math_div_floor_uint(NSTDUInt x, NSTDUInt y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The first value.
///
/// - `NSTDInt8 y` - The second value.
///
/// # Returns
///
/// `NSTDInt8 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt8 nstd_core_math_div_floor_i8(NSTDInt8 x, NSTDInt8 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The first value.
///
/// - `NSTDUInt8 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt8 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt8 nstd_core_math_div_floor_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The first value.
///
/// - `NSTDInt16 y` - The second value.
///
/// # Returns
///
/// `NSTDInt16 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt16 nstd_core_math_div_floor_i16(NSTDInt16 x, NSTDInt16 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The first value.
///
/// - `NSTDUInt16 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt16 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt16 nstd_core_math_div_floor_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The first value.
///
/// - `NSTDInt32 y` - The second value.
///
/// # Returns
///
/// `NSTDInt32 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt32 nstd_core_math_div_floor_i32(NSTDInt32 x, NSTDInt32 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The first value.
///
/// - `NSTDUInt32 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt32 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt32 nstd_core_math_div_floor_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The first value.
///
/// - `NSTDInt64 y` - The second value.
///
/// # Returns
///
/// `NSTDInt64 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDInt64 nstd_core_math_div_floor_i64(NSTDInt64 x, NSTDInt64 y);
/// Divides two numbers and rounds the result down to the next integer.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The first value.
///
/// - `NSTDUInt64 y` - The second value.
///
/// # Returns
///
/// `NSTDUInt64 v` - The divided value, rounded down.
///
/// # Panics
///
/// This operation will panic if `y` is 0.
///
/// # Safety
///
/// This operation can cause undefined behavior if it panics into non-Rust code.
NSTDAPI NSTDUInt64 nstd_core_math_div_floor_u64(NSTDUInt64 x, NSTDUInt64 y);

#endif
