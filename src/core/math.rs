//! Low level math operations.
use crate::{
    NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt,
    NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8,
};

/// Converts radians to degrees.
///
/// # Parameters:
///
/// - `NSTDFloat32 rad` - The radians value.
///
/// # Returns
///
/// `NSTDFloat32 deg` - The radians value converted to degrees.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_deg_f32(rad: NSTDFloat32) -> NSTDFloat32 {
    rad.to_degrees()
}
/// Converts radians to degrees.
///
/// # Parameters:
///
/// - `NSTDFloat64 rad` - The radians value.
///
/// # Returns
///
/// `NSTDFloat64 deg` - The radians value converted to degrees.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_deg_f64(rad: NSTDFloat64) -> NSTDFloat64 {
    rad.to_degrees()
}

/// Converts degrees to radians.
///
/// # Parameters:
///
/// - `NSTDFloat32 deg` - The degrees value.
///
/// # Returns
///
/// `NSTDFloat32 rad` - The degrees value converted to radians.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_rad_f32(deg: NSTDFloat32) -> NSTDFloat32 {
    deg.to_radians()
}
/// Converts degrees to radians.
///
/// # Parameters:
///
/// - `NSTDFloat64 deg` - The degrees value.
///
/// # Returns
///
/// `NSTDFloat64 rad` - The degrees value converted to radians.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_rad_f64(deg: NSTDFloat64) -> NSTDFloat64 {
    deg.to_radians()
}

/// Generates the `abs` functions.
macro_rules! gen_abs {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        $(#[$meta])*
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $name(x: $T) -> $T {
            x.abs()
        }
    };
}
gen_abs!(
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
    nstd_core_math_abs_int,
    NSTDInt
);
gen_abs!(
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
    nstd_core_math_abs_i8,
    NSTDInt8
);
gen_abs!(
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
    nstd_core_math_abs_i16,
    NSTDInt16
);
gen_abs!(
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
    nstd_core_math_abs_i32,
    NSTDInt32
);
gen_abs!(
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
    nstd_core_math_abs_i64,
    NSTDInt64
);

/// Generates the `pow` functions.
macro_rules! gen_pow {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        $(#[$meta])*
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $name(x: $T, exp: NSTDUInt32) -> $T {
            x.pow(exp)
        }
    };
}
gen_pow!(
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
    nstd_core_math_pow_int,
    NSTDInt
);
gen_pow!(
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
    nstd_core_math_pow_uint,
    NSTDUInt
);
gen_pow!(
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
    nstd_core_math_pow_i8,
    NSTDInt8
);
gen_pow!(
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
    nstd_core_math_pow_u8,
    NSTDUInt8
);
gen_pow!(
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
    /// assert!(nstd_core_math_pow_i16(4, 5) == 1024);
    /// ```
    nstd_core_math_pow_i16,
    NSTDInt16
);
gen_pow!(
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
    /// assert!(nstd_core_math_pow_u16(4, 5) == 1024);
    /// ```
    nstd_core_math_pow_u16,
    NSTDUInt16
);
gen_pow!(
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
    /// assert!(nstd_core_math_pow_i32(4, 5) == 1024);
    /// ```
    nstd_core_math_pow_i32,
    NSTDInt32
);
gen_pow!(
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
    /// assert!(nstd_core_math_pow_u32(4, 5) == 1024);
    /// ```
    nstd_core_math_pow_u32,
    NSTDUInt32
);
gen_pow!(
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
    /// assert!(nstd_core_math_pow_i64(4, 5) == 1024);
    /// ```
    nstd_core_math_pow_i64,
    NSTDInt64
);
gen_pow!(
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
    /// assert!(nstd_core_math_pow_u64(4, 5) == 1024);
    /// ```
    nstd_core_math_pow_u64,
    NSTDUInt64
);

/// Generates the `clamp` functions.
macro_rules! gen_clamp {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        $(#[$meta])*
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $name(x: $T, min: $T, max: $T) -> $T {
            x.clamp(min, max)
        }
    };
}
gen_clamp!(
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
    nstd_core_math_clamp_f32,
    NSTDFloat32
);
gen_clamp!(
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
    nstd_core_math_clamp_f64,
    NSTDFloat64
);
gen_clamp!(
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
    nstd_core_math_clamp_int,
    NSTDInt
);
gen_clamp!(
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
    nstd_core_math_clamp_uint,
    NSTDUInt
);
gen_clamp!(
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
    nstd_core_math_clamp_i8,
    NSTDInt8
);
gen_clamp!(
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
    nstd_core_math_clamp_u8,
    NSTDUInt8
);
gen_clamp!(
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
    nstd_core_math_clamp_i16,
    NSTDInt16
);
gen_clamp!(
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
    nstd_core_math_clamp_u16,
    NSTDUInt16
);
gen_clamp!(
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
    nstd_core_math_clamp_i32,
    NSTDInt32
);
gen_clamp!(
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
    nstd_core_math_clamp_u32,
    NSTDUInt32
);
gen_clamp!(
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
    nstd_core_math_clamp_i64,
    NSTDInt64
);
gen_clamp!(
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
    nstd_core_math_clamp_u64,
    NSTDUInt64
);

/// Generates the `div_ceil` functions.
macro_rules! gen_div_ceil {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        $(#[$meta])*
        /// # Panics
        ///
        /// This operation will panic if `y` is 0.
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        #[allow(unused_comparisons)]
        pub extern "C" fn $name(x: $T, y: $T) -> $T {
            let d = x / y;
            let r = x % y;
            if (r > 0 && y > 0) || (r < 0 && y < 0) {
                d + 1
            } else {
                d
            }
        }
    };
}
gen_div_ceil!(
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
    nstd_core_math_div_ceil_int,
    NSTDInt
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_uint,
    NSTDUInt
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_i8,
    NSTDInt8
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_u8,
    NSTDUInt8
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_i16,
    NSTDInt16
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_u16,
    NSTDUInt16
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_i32,
    NSTDInt32
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_u32,
    NSTDUInt32
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_i64,
    NSTDInt64
);
gen_div_ceil!(
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
    nstd_core_math_div_ceil_u64,
    NSTDUInt64
);
