//! Operator overloading for types and operators that may cause overflow.
//!
//! The types of overflows that this module attempts to catch can be found
//! [here](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow).
//!
//! Also see [this](https://github.com/RustEnthusiast/nstd/issues/22) issue for more information.
use crate::{
    NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt, NSTDUInt16, NSTDUInt32,
    NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;

/// Generates the negate (-) operator implementation, this is not to be confused with the
/// subtraction operator.
macro_rules! gen_neg {
    ($name: ident, $T: ty) => {
        /// Returns the negative value of `x`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The value to negate.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " v` - The negative value of `x`.")]
        ///
        /// # Panics
        ///
        /// This will panic if the negate operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(69) == -69);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T) -> $T {
            x.checked_neg().expect("attempt to negate with overflow")
        }
    };
}
gen_neg!(nstd_core_ops_neg_int, NSTDInt);
gen_neg!(nstd_core_ops_neg_i8, NSTDInt8);
gen_neg!(nstd_core_ops_neg_i16, NSTDInt16);
gen_neg!(nstd_core_ops_neg_i32, NSTDInt32);
gen_neg!(nstd_core_ops_neg_i64, NSTDInt64);

/// Generates the addition (+) operator implementations.
macro_rules! gen_add {
    ($name: ident, $T: ty) => {
        /// Computes the addition operation of `x` + `y`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The left operand.")]
        ///
        #[doc = concat!(" - `", stringify!($T), " y` - The right operand.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if the addition operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(4, 5) == 9);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: $T) -> $T {
            x.checked_add(y).expect("attempt to add with overflow")
        }
    };
}
gen_add!(nstd_core_ops_add_int, NSTDInt);
gen_add!(nstd_core_ops_add_uint, NSTDUInt);
gen_add!(nstd_core_ops_add_i8, NSTDInt8);
gen_add!(nstd_core_ops_add_u8, NSTDUInt8);
gen_add!(nstd_core_ops_add_i16, NSTDInt16);
gen_add!(nstd_core_ops_add_u16, NSTDUInt16);
gen_add!(nstd_core_ops_add_i32, NSTDInt32);
gen_add!(nstd_core_ops_add_u32, NSTDUInt32);
gen_add!(nstd_core_ops_add_i64, NSTDInt64);
gen_add!(nstd_core_ops_add_u64, NSTDUInt64);

/// Generates the subtraction (-) operator implementations.
macro_rules! gen_sub {
    ($name: ident, $T: ty) => {
        /// Computes the subtraction operation of `x` - `y`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The left operand.")]
        ///
        #[doc = concat!(" - `", stringify!($T), " y` - The right operand.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if the subtraction operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(9, 5) == 4);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: $T) -> $T {
            x.checked_sub(y).expect("attempt to subtract with overflow")
        }
    };
}
gen_sub!(nstd_core_ops_sub_int, NSTDInt);
gen_sub!(nstd_core_ops_sub_uint, NSTDUInt);
gen_sub!(nstd_core_ops_sub_i8, NSTDInt8);
gen_sub!(nstd_core_ops_sub_u8, NSTDUInt8);
gen_sub!(nstd_core_ops_sub_i16, NSTDInt16);
gen_sub!(nstd_core_ops_sub_u16, NSTDUInt16);
gen_sub!(nstd_core_ops_sub_i32, NSTDInt32);
gen_sub!(nstd_core_ops_sub_u32, NSTDUInt32);
gen_sub!(nstd_core_ops_sub_i64, NSTDInt64);
gen_sub!(nstd_core_ops_sub_u64, NSTDUInt64);

/// Generates the multiplication (*) operator implementations.
macro_rules! gen_mul {
    ($name: ident, $T: ty) => {
        /// Computes the multiplication operation of `x` * `y`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The left operand.")]
        ///
        #[doc = concat!(" - `", stringify!($T), " y` - The right operand.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if the multiplication operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(3, 4) == 12);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: $T) -> $T {
            x.checked_mul(y).expect("attempt to multiply with overflow")
        }
    };
}
gen_mul!(nstd_core_ops_mul_int, NSTDInt);
gen_mul!(nstd_core_ops_mul_uint, NSTDUInt);
gen_mul!(nstd_core_ops_mul_i8, NSTDInt8);
gen_mul!(nstd_core_ops_mul_u8, NSTDUInt8);
gen_mul!(nstd_core_ops_mul_i16, NSTDInt16);
gen_mul!(nstd_core_ops_mul_u16, NSTDUInt16);
gen_mul!(nstd_core_ops_mul_i32, NSTDInt32);
gen_mul!(nstd_core_ops_mul_u32, NSTDUInt32);
gen_mul!(nstd_core_ops_mul_i64, NSTDInt64);
gen_mul!(nstd_core_ops_mul_u64, NSTDUInt64);

/// Generates the division (/) operator implementations.
macro_rules! gen_div {
    ($name: ident, $T: ty) => {
        /// Computes the division operation of `x` / `y`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The left operand.")]
        ///
        #[doc = concat!(" - `", stringify!($T), " y` - The right operand.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if `y` is 0 or the division operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(15, 3) == 5);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: $T) -> $T {
            if y == 0 {
                panic!("attempt to divide by zero");
            }
            x.checked_div(y).expect("attempt to divide with overflow")
        }
    };
}
gen_div!(nstd_core_ops_div_int, NSTDInt);
gen_div!(nstd_core_ops_div_uint, NSTDUInt);
gen_div!(nstd_core_ops_div_i8, NSTDInt8);
gen_div!(nstd_core_ops_div_u8, NSTDUInt8);
gen_div!(nstd_core_ops_div_i16, NSTDInt16);
gen_div!(nstd_core_ops_div_u16, NSTDUInt16);
gen_div!(nstd_core_ops_div_i32, NSTDInt32);
gen_div!(nstd_core_ops_div_u32, NSTDUInt32);
gen_div!(nstd_core_ops_div_i64, NSTDInt64);
gen_div!(nstd_core_ops_div_u64, NSTDUInt64);

/// Generates the remainder (%) operator implementations.
macro_rules! gen_rem {
    ($name: ident, $T: ty) => {
        /// Computes the remainder of `x` / `y`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The left operand.")]
        ///
        #[doc = concat!(" - `", stringify!($T), " y` - The right operand.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if `y` is 0 or the remainder operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(23, 5) == 3);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: $T) -> $T {
            if y == 0 {
                panic!("attempt to calculate the remainder with a divisor of zero");
            }
            x.checked_rem(y)
                .expect("attempt to calculate the remainder with overflow")
        }
    };
}
gen_rem!(nstd_core_ops_rem_int, NSTDInt);
gen_rem!(nstd_core_ops_rem_uint, NSTDUInt);
gen_rem!(nstd_core_ops_rem_i8, NSTDInt8);
gen_rem!(nstd_core_ops_rem_u8, NSTDUInt8);
gen_rem!(nstd_core_ops_rem_i16, NSTDInt16);
gen_rem!(nstd_core_ops_rem_u16, NSTDUInt16);
gen_rem!(nstd_core_ops_rem_i32, NSTDInt32);
gen_rem!(nstd_core_ops_rem_u32, NSTDUInt32);
gen_rem!(nstd_core_ops_rem_i64, NSTDInt64);
gen_rem!(nstd_core_ops_rem_u64, NSTDUInt64);

/// Generates the shift left (<<) operator implementations.
macro_rules! gen_shl {
    ($name: ident, $T: ty) => {
        /// Shifts value `x` `y` bits to the left.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The value to shift.")]
        ///
        /// - `NSTDUInt32 y` - The number of bits to shift.
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if the left shift operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(1, 4) == 16);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: NSTDUInt32) -> $T {
            x.checked_shl(y)
                .expect("attempt to shift left with overflow")
        }
    };
}
gen_shl!(nstd_core_ops_shl_int, NSTDInt);
gen_shl!(nstd_core_ops_shl_uint, NSTDUInt);
gen_shl!(nstd_core_ops_shl_i8, NSTDInt8);
gen_shl!(nstd_core_ops_shl_u8, NSTDUInt8);
gen_shl!(nstd_core_ops_shl_i16, NSTDInt16);
gen_shl!(nstd_core_ops_shl_u16, NSTDUInt16);
gen_shl!(nstd_core_ops_shl_i32, NSTDInt32);
gen_shl!(nstd_core_ops_shl_u32, NSTDUInt32);
gen_shl!(nstd_core_ops_shl_i64, NSTDInt64);
gen_shl!(nstd_core_ops_shl_u64, NSTDUInt64);

/// Generates the shift right (>>) operator implementations.
macro_rules! gen_shr {
    ($name: ident, $T: ty) => {
        /// Shifts value `x` `y` bits to the right.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The value to shift.")]
        ///
        /// - `NSTDUInt32 y` - The number of bits to shift.
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($T), " z` - The result of the operation.")]
        ///
        /// # Panics
        ///
        /// This will panic if the right shift operation results in an overflow.
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::ops::", stringify!($name), ";")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(16, 4) == 1);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub fn $name(x: $T, y: NSTDUInt32) -> $T {
            x.checked_shr(y)
                .expect("attempt to shift right with overflow")
        }
    };
}
gen_shr!(nstd_core_ops_shr_int, NSTDInt);
gen_shr!(nstd_core_ops_shr_uint, NSTDUInt);
gen_shr!(nstd_core_ops_shr_i8, NSTDInt8);
gen_shr!(nstd_core_ops_shr_u8, NSTDUInt8);
gen_shr!(nstd_core_ops_shr_i16, NSTDInt16);
gen_shr!(nstd_core_ops_shr_u16, NSTDUInt16);
gen_shr!(nstd_core_ops_shr_i32, NSTDInt32);
gen_shr!(nstd_core_ops_shr_u32, NSTDUInt32);
gen_shr!(nstd_core_ops_shr_i64, NSTDInt64);
gen_shr!(nstd_core_ops_shr_u64, NSTDUInt64);
