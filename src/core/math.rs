//! Low level math operations.
use crate::{
    NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt,
    NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;

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
#[nstdapi]
pub fn nstd_core_math_deg_f32(rad: NSTDFloat32) -> NSTDFloat32 {
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
#[nstdapi]
pub fn nstd_core_math_deg_f64(rad: NSTDFloat64) -> NSTDFloat64 {
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
#[nstdapi]
pub fn nstd_core_math_rad_f32(deg: NSTDFloat32) -> NSTDFloat32 {
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
#[nstdapi]
pub fn nstd_core_math_rad_f64(deg: NSTDFloat64) -> NSTDFloat64 {
    deg.to_radians()
}

/// Generates the `abs` functions.
macro_rules! gen_abs {
    ($name: ident, $T: ty) => {
        /// Computes the absolute (positive) value of `x`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!("- `", stringify!($T), " x` - The value.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("`", stringify!($T), " abs` - The absolute value of `x`.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::math::", stringify!($name), ";")]
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(10) == 10);")]
        #[doc = concat!("assert!(", stringify!($name), "(-10) == 10);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T) -> $T {
            x.abs()
        }
    };
}
gen_abs!(nstd_core_math_abs_int, NSTDInt);
gen_abs!(nstd_core_math_abs_i8, NSTDInt8);
gen_abs!(nstd_core_math_abs_i16, NSTDInt16);
gen_abs!(nstd_core_math_abs_i32, NSTDInt32);
gen_abs!(nstd_core_math_abs_i64, NSTDInt64);

/// Generates the `pow` functions.
macro_rules! gen_pow {
    ($name: ident, $T: ty) => {
        /// Raises the value `x` to the power of `exp`
        ///
        /// # Parameters:
        ///
        #[doc = concat!("- `", stringify!($T), " x` - The value.")]
        ///
        /// - `NSTDUInt32 exp` - The exponent.
        ///
        /// # Returns
        ///
        #[doc = concat!("`", stringify!($T), " pow` - `x` raised to the power of `exp`.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::math::", stringify!($name), ";")]
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(2, 3) == 8);")]
        #[doc = concat!("assert!(", stringify!($name), "(2, 5) == 32);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, exp: NSTDUInt32) -> $T {
            x.pow(exp)
        }
    };
}
gen_pow!(nstd_core_math_pow_int, NSTDInt);
gen_pow!(nstd_core_math_pow_uint, NSTDUInt);
gen_pow!(nstd_core_math_pow_i8, NSTDInt8);
gen_pow!(nstd_core_math_pow_u8, NSTDUInt8);
gen_pow!(nstd_core_math_pow_i16, NSTDInt16);
gen_pow!(nstd_core_math_pow_u16, NSTDUInt16);
gen_pow!(nstd_core_math_pow_i32, NSTDInt32);
gen_pow!(nstd_core_math_pow_u32, NSTDUInt32);
gen_pow!(nstd_core_math_pow_i64, NSTDInt64);
gen_pow!(nstd_core_math_pow_u64, NSTDUInt64);

/// Generates the `clamp` functions.
macro_rules! gen_clamp {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        /// Clamps the value `x` to the bounds `min` and `max`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!("- `", stringify!($T), " x` - The value to clamp.")]
        ///
        #[doc = concat!("- `", stringify!($T), " min` - The minimum clamp value.")]
        ///
        #[doc = concat!("- `", stringify!($T), " max` - The maximum clamp value.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("`", stringify!($T), " v` - The clamped value.")]
        $(#[$meta])*
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, min: $T, max: $T) -> $T {
            x.clamp(min, max)
        }
    };
}
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`, `min` is NaN, or `max` is NaN.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_f32;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_f32(2.5, 3.0, 5.0) == 3.0);
    /// assert!(nstd_core_math_clamp_f32(4.0, 3.0, 5.0) == 4.0);
    /// assert!(nstd_core_math_clamp_f32(7.5, 3.0, 5.0) == 5.0);
    /// # }
    /// ```
    nstd_core_math_clamp_f32,
    NSTDFloat32
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`, `min` is NaN, or `max` is NaN.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_f64;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_f64(2.5, 3.0, 5.0) == 3.0);
    /// assert!(nstd_core_math_clamp_f64(4.0, 3.0, 5.0) == 4.0);
    /// assert!(nstd_core_math_clamp_f64(7.5, 3.0, 5.0) == 5.0);
    /// # }
    /// ```
    nstd_core_math_clamp_f64,
    NSTDFloat64
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_int;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_int(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_int(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_int(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_int,
    NSTDInt
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_uint;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_uint(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_uint(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_uint(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_uint,
    NSTDUInt
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_i8;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_i8(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_i8(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_i8(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_i8,
    NSTDInt8
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_u8;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_u8(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_u8(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_u8(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_u8,
    NSTDUInt8
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_i16;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_i16(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_i16(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_i16(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_i16,
    NSTDInt16
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_u16;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_u16(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_u16(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_u16(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_u16,
    NSTDUInt16
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_i32;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_i32(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_i32(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_i32(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_i32,
    NSTDInt32
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_u32;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_u32(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_u32(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_u32(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_u32,
    NSTDUInt32
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_i64;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_i64(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_i64(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_i64(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_i64,
    NSTDInt64
);
gen_clamp!(
    ///
    /// # Panics
    ///
    /// Panics if `min` > `max`.
    ///
    /// # Example
    ///
    /// ```
    /// use nstd_sys::core::math::nstd_core_math_clamp_u64;
    ///
    /// # unsafe {
    /// assert!(nstd_core_math_clamp_u64(2, 5, 10) == 5);
    /// assert!(nstd_core_math_clamp_u64(8, 5, 10) == 8);
    /// assert!(nstd_core_math_clamp_u64(14, 5, 10) == 10);
    /// # }
    /// ```
    nstd_core_math_clamp_u64,
    NSTDUInt64
);

/// Generates the `div_ceil` functions.
macro_rules! gen_div_ceil {
    ($name: ident, $T: ty) => {
        /// Divides two numbers and rounds the result up to the next integer.
        ///
        /// # Parameters:
        ///
        #[doc = concat!("- `", stringify!($T), " x` - The first value.")]
        ///
        #[doc = concat!("- `", stringify!($T), " y` - The second value.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("`", stringify!($T), " v` - The divided value, rounded up.")]
        ///
        /// # Panics
        ///
        /// This operation will panic if `y` is 0.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::math::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(8, 5) == 2);")]
        #[doc = concat!("assert!(", stringify!($name), "(8, 3) == 3);")]
        #[doc = concat!("assert!(", stringify!($name), "(8, 2) == 4);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        #[allow(unused_comparisons)]
        pub const fn $name(x: $T, y: $T) -> $T {
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
gen_div_ceil!(nstd_core_math_div_ceil_int, NSTDInt);
gen_div_ceil!(nstd_core_math_div_ceil_uint, NSTDUInt);
gen_div_ceil!(nstd_core_math_div_ceil_i8, NSTDInt8);
gen_div_ceil!(nstd_core_math_div_ceil_u8, NSTDUInt8);
gen_div_ceil!(nstd_core_math_div_ceil_i16, NSTDInt16);
gen_div_ceil!(nstd_core_math_div_ceil_u16, NSTDUInt16);
gen_div_ceil!(nstd_core_math_div_ceil_i32, NSTDInt32);
gen_div_ceil!(nstd_core_math_div_ceil_u32, NSTDUInt32);
gen_div_ceil!(nstd_core_math_div_ceil_i64, NSTDInt64);
gen_div_ceil!(nstd_core_math_div_ceil_u64, NSTDUInt64);

/// Generates the `div_floor` functions.
macro_rules! gen_div_floor {
    ($name: ident, $T: ty) => {
        /// Divides two numbers and rounds the result down to the next integer.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The first value.")]
        ///
        #[doc = concat!(" - `", stringify!($T), " y` - The second value.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " v` - The divided value, rounded down.")]
        ///
        /// # Panics
        ///
        /// This operation will panic if `y` is 0.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::math::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(5, 2) == 2);")]
        #[doc = concat!("assert!(", stringify!($name), "(13, 4) == 3);")]
        #[doc = concat!("assert!(", stringify!($name), "(23, 5) == 4);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        #[allow(unused_comparisons)]
        pub const fn $name(x: $T, y: $T) -> $T {
            let d = x / y;
            let r = x % y;
            if (r > 0 && y < 0) || (r < 0 && y > 0) {
                d - 1
            } else {
                d
            }
        }
    };
}
gen_div_floor!(nstd_core_math_div_floor_int, NSTDInt);
gen_div_floor!(nstd_core_math_div_floor_uint, NSTDUInt);
gen_div_floor!(nstd_core_math_div_floor_i8, NSTDInt8);
gen_div_floor!(nstd_core_math_div_floor_u8, NSTDUInt8);
gen_div_floor!(nstd_core_math_div_floor_i16, NSTDInt16);
gen_div_floor!(nstd_core_math_div_floor_u16, NSTDUInt16);
gen_div_floor!(nstd_core_math_div_floor_i32, NSTDInt32);
gen_div_floor!(nstd_core_math_div_floor_u32, NSTDUInt32);
gen_div_floor!(nstd_core_math_div_floor_i64, NSTDInt64);
gen_div_floor!(nstd_core_math_div_floor_u64, NSTDUInt64);
