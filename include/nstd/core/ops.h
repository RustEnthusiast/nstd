#ifndef NSTD_CORE_OPS_H
#define NSTD_CORE_OPS_H
#include "../nstd.h"
#include "optional.h"

/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The value to negate.
///
/// # Returns
///
/// `NSTDOptionalInt v` - The negative value of `x` on success, or an uninitialized "none" variant
/// on overflow.
NSTDAPI NSTDOptionalInt nstd_core_ops_neg_int(NSTDInt x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value to negate.
///
/// # Returns
///
/// `NSTDOptionalInt8 v` - The negative value of `x` on success, or an uninitialized "none" variant
/// on overflow.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_neg_i8(NSTDInt8 x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value to negate.
///
/// # Returns
///
/// `NSTDOptionalInt16 v` - The negative value of `x` on success, or an uninitialized "none" variant
/// on overflow.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_neg_i16(NSTDInt16 x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value to negate.
///
/// # Returns
///
/// `NSTDOptionalInt32 v` - The negative value of `x` on success, or an uninitialized "none" variant
/// on overflow.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_neg_i32(NSTDInt32 x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value to negate.
///
/// # Returns
///
/// `NSTDOptionalInt64 v` - The negative value of `x` on success, or an uninitialized "none" variant
/// on overflow.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_neg_i64(NSTDInt64 x);

/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The left operand.
///
/// - `NSTDInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt nstd_core_ops_add_int(NSTDInt x, NSTDInt y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The left operand.
///
/// - `NSTDUInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt nstd_core_ops_add_uint(NSTDUInt x, NSTDUInt y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The left operand.
///
/// - `NSTDInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_add_i8(NSTDInt8 x, NSTDInt8 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The left operand.
///
/// - `NSTDUInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_add_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The left operand.
///
/// - `NSTDInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_add_i16(NSTDInt16 x, NSTDInt16 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The left operand.
///
/// - `NSTDUInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_add_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The left operand.
///
/// - `NSTDInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_add_i32(NSTDInt32 x, NSTDInt32 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The left operand.
///
/// - `NSTDUInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_add_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The left operand.
///
/// - `NSTDInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_add_i64(NSTDInt64 x, NSTDInt64 y);
/// Computes the addition operation of `x` + `y`.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The left operand.
///
/// - `NSTDUInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_add_u64(NSTDUInt64 x, NSTDUInt64 y);

/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The left operand.
///
/// - `NSTDInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt nstd_core_ops_sub_int(NSTDInt x, NSTDInt y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The left operand.
///
/// - `NSTDUInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt nstd_core_ops_sub_uint(NSTDUInt x, NSTDUInt y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The left operand.
///
/// - `NSTDInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_sub_i8(NSTDInt8 x, NSTDInt8 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The left operand.
///
/// - `NSTDUInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_sub_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The left operand.
///
/// - `NSTDInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_sub_i16(NSTDInt16 x, NSTDInt16 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The left operand.
///
/// - `NSTDUInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_sub_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The left operand.
///
/// - `NSTDInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_sub_i32(NSTDInt32 x, NSTDInt32 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The left operand.
///
/// - `NSTDUInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_sub_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The left operand.
///
/// - `NSTDInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_sub_i64(NSTDInt64 x, NSTDInt64 y);
/// Computes the subtraction operation of `x` - `y`.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The left operand.
///
/// - `NSTDUInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_sub_u64(NSTDUInt64 x, NSTDUInt64 y);

/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The left operand.
///
/// - `NSTDInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt nstd_core_ops_mul_int(NSTDInt x, NSTDInt y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The left operand.
///
/// - `NSTDUInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt nstd_core_ops_mul_uint(NSTDUInt x, NSTDUInt y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The left operand.
///
/// - `NSTDInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_mul_i8(NSTDInt8 x, NSTDInt8 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The left operand.
///
/// - `NSTDUInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_mul_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The left operand.
///
/// - `NSTDInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_mul_i16(NSTDInt16 x, NSTDInt16 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The left operand.
///
/// - `NSTDUInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_mul_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The left operand.
///
/// - `NSTDInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_mul_i32(NSTDInt32 x, NSTDInt32 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The left operand.
///
/// - `NSTDUInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_mul_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The left operand.
///
/// - `NSTDInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_mul_i64(NSTDInt64 x, NSTDInt64 y);
/// Computes the multiplication operation of `x` * `y`.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The left operand.
///
/// - `NSTDUInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_mul_u64(NSTDUInt64 x, NSTDUInt64 y);

/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The left operand.
///
/// - `NSTDInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt nstd_core_ops_div_int(NSTDInt x, NSTDInt y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The left operand.
///
/// - `NSTDUInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt nstd_core_ops_div_uint(NSTDUInt x, NSTDUInt y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The left operand.
///
/// - `NSTDInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_div_i8(NSTDInt8 x, NSTDInt8 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The left operand.
///
/// - `NSTDUInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_div_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The left operand.
///
/// - `NSTDInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_div_i16(NSTDInt16 x, NSTDInt16 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The left operand.
///
/// - `NSTDUInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_div_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The left operand.
///
/// - `NSTDInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_div_i32(NSTDInt32 x, NSTDInt32 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The left operand.
///
/// - `NSTDUInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_div_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The left operand.
///
/// - `NSTDInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_div_i64(NSTDInt64 x, NSTDInt64 y);
/// Computes the division operation of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The left operand.
///
/// - `NSTDUInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_div_u64(NSTDUInt64 x, NSTDUInt64 y);

/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The left operand.
///
/// - `NSTDInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt nstd_core_ops_rem_int(NSTDInt x, NSTDInt y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The left operand.
///
/// - `NSTDUInt y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt nstd_core_ops_rem_uint(NSTDUInt x, NSTDUInt y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The left operand.
///
/// - `NSTDInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_rem_i8(NSTDInt8 x, NSTDInt8 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The left operand.
///
/// - `NSTDUInt8 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_rem_u8(NSTDUInt8 x, NSTDUInt8 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The left operand.
///
/// - `NSTDInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_rem_i16(NSTDInt16 x, NSTDInt16 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The left operand.
///
/// - `NSTDUInt16 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_rem_u16(NSTDUInt16 x, NSTDUInt16 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The left operand.
///
/// - `NSTDInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_rem_i32(NSTDInt32 x, NSTDInt32 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The left operand.
///
/// - `NSTDUInt32 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_rem_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The left operand.
///
/// - `NSTDInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_rem_i64(NSTDInt64 x, NSTDInt64 y);
/// Computes the remainder of `x` / `y`.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The left operand.
///
/// - `NSTDUInt64 y` - The right operand.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant if `y` is 0 or overflow occurs.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_rem_u64(NSTDUInt64 x, NSTDUInt64 y);

/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDInt x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt nstd_core_ops_shl_int(NSTDInt x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt nstd_core_ops_shl_uint(NSTDUInt x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_shl_i8(NSTDInt8 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_shl_u8(NSTDUInt8 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_shl_i16(NSTDInt16 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_shl_u16(NSTDUInt16 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_shl_i32(NSTDInt32 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_shl_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_shl_i64(NSTDInt64 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the left.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_shl_u64(NSTDUInt64 x, NSTDUInt32 y);

/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDInt x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt nstd_core_ops_shr_int(NSTDInt x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDUInt x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt nstd_core_ops_shr_uint(NSTDUInt x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt8 nstd_core_ops_shr_i8(NSTDInt8 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDUInt8 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt8 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt8 nstd_core_ops_shr_u8(NSTDUInt8 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt16 nstd_core_ops_shr_i16(NSTDInt16 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDUInt16 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt16 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt16 nstd_core_ops_shr_u16(NSTDUInt16 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt32 nstd_core_ops_shr_i32(NSTDInt32 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDUInt32 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt32 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt32 nstd_core_ops_shr_u32(NSTDUInt32 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalInt64 nstd_core_ops_shr_i64(NSTDInt64 x, NSTDUInt32 y);
/// Shifts value `x` `y` bits to the right.
///
/// # Parameters:
///
/// - `NSTDUInt64 x` - The value to shift.
///
/// - `NSTDUInt32 y` - The number of bits to shift.
///
/// # Returns
///
/// `NSTDOptionalUInt64 z` - The result of the operation on success, or an uninitialized "none"
/// variant on overflow.
NSTDAPI NSTDOptionalUInt64 nstd_core_ops_shr_u64(NSTDUInt64 x, NSTDUInt32 y);

#endif
