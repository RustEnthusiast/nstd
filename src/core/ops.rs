//! Operator overloading for types and operators that may cause overflow.
//!
//! The types of overflows that this module attempts to catch can be found
//! [here](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow).
use crate::{
    core::optional::{
        NSTDOptional, NSTDOptionalInt, NSTDOptionalInt16, NSTDOptionalInt32, NSTDOptionalInt64,
        NSTDOptionalInt8, NSTDOptionalUInt, NSTDOptionalUInt16, NSTDOptionalUInt32,
        NSTDOptionalUInt64, NSTDOptionalUInt8,
    },
    NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt, NSTDUInt16, NSTDUInt32,
    NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;

/// Generates the negate (-) operator implementation, this is not to be confused with the
/// subtraction operator.
macro_rules! gen_neg {
    ($name: ident, $T: ty, $Opt: ty) => {
        /// Returns the negative value of `x`.
        ///
        /// # Parameters:
        ///
        #[doc = concat!(" - `", stringify!($T), " x` - The value to negate.")]
        ///
        /// # Returns
        ///
        #[doc = concat!(" `", stringify!($Opt), " v` - The negative value of `x` on success, or an uninitialized \"none\" variant on overflow.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(69) == NSTDOptional::Some(-69));")]
        #[doc = concat!("assert!(", stringify!($name), "(", stringify!($T), "::MIN) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T) -> $Opt {
            match x.checked_neg() {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_neg!(nstd_core_ops_neg_int, NSTDInt, NSTDOptionalInt);
gen_neg!(nstd_core_ops_neg_i8, NSTDInt8, NSTDOptionalInt8);
gen_neg!(nstd_core_ops_neg_i16, NSTDInt16, NSTDOptionalInt16);
gen_neg!(nstd_core_ops_neg_i32, NSTDInt32, NSTDOptionalInt32);
gen_neg!(nstd_core_ops_neg_i64, NSTDInt64, NSTDOptionalInt64);

/// Generates the addition (+) operator implementations.
macro_rules! gen_add {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant on overflow.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(4, 5) == NSTDOptional::Some(9));")]
        #[doc = concat!("assert!(", stringify!($name), "(", stringify!($T), "::MAX, 1) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: $T) -> $Opt {
            match x.checked_add(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_add!(nstd_core_ops_add_int, NSTDInt, NSTDOptionalInt);
gen_add!(nstd_core_ops_add_uint, NSTDUInt, NSTDOptionalUInt);
gen_add!(nstd_core_ops_add_i8, NSTDInt8, NSTDOptionalInt8);
gen_add!(nstd_core_ops_add_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_add!(nstd_core_ops_add_i16, NSTDInt16, NSTDOptionalInt16);
gen_add!(nstd_core_ops_add_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_add!(nstd_core_ops_add_i32, NSTDInt32, NSTDOptionalInt32);
gen_add!(nstd_core_ops_add_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_add!(nstd_core_ops_add_i64, NSTDInt64, NSTDOptionalInt64);
gen_add!(nstd_core_ops_add_u64, NSTDUInt64, NSTDOptionalUInt64);

/// Generates the subtraction (-) operator implementations.
macro_rules! gen_sub {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant on overflow.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(9, 5) == NSTDOptional::Some(4));")]
        #[doc = concat!("assert!(", stringify!($name), "(", stringify!($T), "::MIN, 1) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: $T) -> $Opt {
            match x.checked_sub(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_sub!(nstd_core_ops_sub_int, NSTDInt, NSTDOptionalInt);
gen_sub!(nstd_core_ops_sub_uint, NSTDUInt, NSTDOptionalUInt);
gen_sub!(nstd_core_ops_sub_i8, NSTDInt8, NSTDOptionalInt8);
gen_sub!(nstd_core_ops_sub_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_sub!(nstd_core_ops_sub_i16, NSTDInt16, NSTDOptionalInt16);
gen_sub!(nstd_core_ops_sub_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_sub!(nstd_core_ops_sub_i32, NSTDInt32, NSTDOptionalInt32);
gen_sub!(nstd_core_ops_sub_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_sub!(nstd_core_ops_sub_i64, NSTDInt64, NSTDOptionalInt64);
gen_sub!(nstd_core_ops_sub_u64, NSTDUInt64, NSTDOptionalUInt64);

/// Generates the multiplication (*) operator implementations.
macro_rules! gen_mul {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant on overflow.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(3, 4) == NSTDOptional::Some(12));")]
        #[doc = concat!("assert!(", stringify!($name), "(", stringify!($T), "::MAX, 2) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: $T) -> $Opt {
            match x.checked_mul(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_mul!(nstd_core_ops_mul_int, NSTDInt, NSTDOptionalInt);
gen_mul!(nstd_core_ops_mul_uint, NSTDUInt, NSTDOptionalUInt);
gen_mul!(nstd_core_ops_mul_i8, NSTDInt8, NSTDOptionalInt8);
gen_mul!(nstd_core_ops_mul_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_mul!(nstd_core_ops_mul_i16, NSTDInt16, NSTDOptionalInt16);
gen_mul!(nstd_core_ops_mul_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_mul!(nstd_core_ops_mul_i32, NSTDInt32, NSTDOptionalInt32);
gen_mul!(nstd_core_ops_mul_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_mul!(nstd_core_ops_mul_i64, NSTDInt64, NSTDOptionalInt64);
gen_mul!(nstd_core_ops_mul_u64, NSTDUInt64, NSTDOptionalUInt64);

/// Generates the division (/) operator implementations.
macro_rules! gen_div {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant if `y` is 0 or overflow occurs.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(15, 3) == NSTDOptional::Some(5));")]
        #[doc = concat!("assert!(", stringify!($name), "(15, 0) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: $T) -> $Opt {
            match x.checked_div(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_div!(nstd_core_ops_div_int, NSTDInt, NSTDOptionalInt);
gen_div!(nstd_core_ops_div_uint, NSTDUInt, NSTDOptionalUInt);
gen_div!(nstd_core_ops_div_i8, NSTDInt8, NSTDOptionalInt8);
gen_div!(nstd_core_ops_div_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_div!(nstd_core_ops_div_i16, NSTDInt16, NSTDOptionalInt16);
gen_div!(nstd_core_ops_div_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_div!(nstd_core_ops_div_i32, NSTDInt32, NSTDOptionalInt32);
gen_div!(nstd_core_ops_div_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_div!(nstd_core_ops_div_i64, NSTDInt64, NSTDOptionalInt64);
gen_div!(nstd_core_ops_div_u64, NSTDUInt64, NSTDOptionalUInt64);

/// Generates the remainder (%) operator implementations.
macro_rules! gen_rem {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant if `y` is 0 or overflow occurs.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(23, 5) == NSTDOptional::Some(3));")]
        #[doc = concat!("assert!(", stringify!($name), "(23, 0) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: $T) -> $Opt {
            match x.checked_rem(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_rem!(nstd_core_ops_rem_int, NSTDInt, NSTDOptionalInt);
gen_rem!(nstd_core_ops_rem_uint, NSTDUInt, NSTDOptionalUInt);
gen_rem!(nstd_core_ops_rem_i8, NSTDInt8, NSTDOptionalInt8);
gen_rem!(nstd_core_ops_rem_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_rem!(nstd_core_ops_rem_i16, NSTDInt16, NSTDOptionalInt16);
gen_rem!(nstd_core_ops_rem_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_rem!(nstd_core_ops_rem_i32, NSTDInt32, NSTDOptionalInt32);
gen_rem!(nstd_core_ops_rem_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_rem!(nstd_core_ops_rem_i64, NSTDInt64, NSTDOptionalInt64);
gen_rem!(nstd_core_ops_rem_u64, NSTDUInt64, NSTDOptionalUInt64);

/// Generates the shift left (<<) operator implementations.
macro_rules! gen_shl {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant on overflow.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::", stringify!($T), ";")]
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(1, 4) == NSTDOptional::Some(16));")]
        #[doc = concat!("assert!(", stringify!($name), "(1, 128) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: NSTDUInt32) -> $Opt {
            match x.checked_shl(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_shl!(nstd_core_ops_shl_int, NSTDInt, NSTDOptionalInt);
gen_shl!(nstd_core_ops_shl_uint, NSTDUInt, NSTDOptionalUInt);
gen_shl!(nstd_core_ops_shl_i8, NSTDInt8, NSTDOptionalInt8);
gen_shl!(nstd_core_ops_shl_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_shl!(nstd_core_ops_shl_i16, NSTDInt16, NSTDOptionalInt16);
gen_shl!(nstd_core_ops_shl_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_shl!(nstd_core_ops_shl_i32, NSTDInt32, NSTDOptionalInt32);
gen_shl!(nstd_core_ops_shl_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_shl!(nstd_core_ops_shl_i64, NSTDInt64, NSTDOptionalInt64);
gen_shl!(nstd_core_ops_shl_u64, NSTDUInt64, NSTDOptionalUInt64);

/// Generates the shift right (>>) operator implementations.
macro_rules! gen_shr {
    ($name: ident, $T: ty, $Opt: ty) => {
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
        #[doc = concat!(" `", stringify!($Opt), " z` - The result of the operation on success, or an uninitialized \"none\" variant on overflow.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!("use nstd_sys::core::{ops::", stringify!($name), ", optional::NSTDOptional};")]
        ///
        /// # unsafe {
        #[doc = concat!("assert!(", stringify!($name), "(16, 4) == NSTDOptional::Some(1));")]
        #[doc = concat!("assert!(", stringify!($name), "(16, 128) == NSTDOptional::None);")]
        /// # }
        /// ```
        #[inline]
        #[nstdapi]
        pub const fn $name(x: $T, y: NSTDUInt32) -> $Opt {
            match x.checked_shr(y) {
                Some(v) => NSTDOptional::Some(v),
                _ => NSTDOptional::None,
            }
        }
    };
}
gen_shr!(nstd_core_ops_shr_int, NSTDInt, NSTDOptionalInt);
gen_shr!(nstd_core_ops_shr_uint, NSTDUInt, NSTDOptionalUInt);
gen_shr!(nstd_core_ops_shr_i8, NSTDInt8, NSTDOptionalInt8);
gen_shr!(nstd_core_ops_shr_u8, NSTDUInt8, NSTDOptionalUInt8);
gen_shr!(nstd_core_ops_shr_i16, NSTDInt16, NSTDOptionalInt16);
gen_shr!(nstd_core_ops_shr_u16, NSTDUInt16, NSTDOptionalUInt16);
gen_shr!(nstd_core_ops_shr_i32, NSTDInt32, NSTDOptionalInt32);
gen_shr!(nstd_core_ops_shr_u32, NSTDUInt32, NSTDOptionalUInt32);
gen_shr!(nstd_core_ops_shr_i64, NSTDInt64, NSTDOptionalInt64);
gen_shr!(nstd_core_ops_shr_u64, NSTDUInt64, NSTDOptionalUInt64);
