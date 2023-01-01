#ifndef NSTD_CORE_OPS_H
#define NSTD_CORE_OPS_H
#include "../nstd.h"

/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_int(NSTDInt *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_uint(NSTDUInt *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt8 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_i8(NSTDInt8 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt8 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_u8(NSTDUInt8 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt16 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_i16(NSTDInt16 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt16 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_u16(NSTDUInt16 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt32 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_i32(NSTDInt32 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt32 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_u32(NSTDUInt32 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt64 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_i64(NSTDInt64 *x);
/// Increments `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt64 *x` - The value to increment.
///
/// # Panics
///
/// This will panic if the increment operation results in an overflow.
NSTDAPI void nstd_core_ops_inc_u64(NSTDUInt64 *x);

/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_int(NSTDInt *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_uint(NSTDUInt *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt8 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_i8(NSTDInt8 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt8 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_u8(NSTDUInt8 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt16 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_i16(NSTDInt16 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt16 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_u16(NSTDUInt16 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt32 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_i32(NSTDInt32 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt32 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_u32(NSTDUInt32 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDInt64 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_i64(NSTDInt64 *x);
/// Decrements `x` by 1.
///
/// # Parameters:
///
/// - `NSTDUInt64 *x` - The value to decrement.
///
/// # Panics
///
/// This will panic if the decrement operation results in an overflow.
NSTDAPI void nstd_core_ops_dec_u64(NSTDUInt64 *x);

/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt x` - The value to negate.
///
/// # Returns
///
/// `NSTDInt v` - The negative value of `x`.
///
/// # Panics
///
/// This will panic if the negate operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_neg_int(NSTDInt x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt8 x` - The value to negate.
///
/// # Returns
///
/// `NSTDInt8 v` - The negative value of `x`.
///
/// # Panics
///
/// This will panic if the negate operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_neg_i8(NSTDInt8 x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt16 x` - The value to negate.
///
/// # Returns
///
/// `NSTDInt16 v` - The negative value of `x`.
///
/// # Panics
///
/// This will panic if the negate operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_neg_i16(NSTDInt16 x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt32 x` - The value to negate.
///
/// # Returns
///
/// `NSTDInt32 v` - The negative value of `x`.
///
/// # Panics
///
/// This will panic if the negate operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_neg_i32(NSTDInt32 x);
/// Returns the negative value of `x`.
///
/// # Parameters:
///
/// - `NSTDInt64 x` - The value to negate.
///
/// # Returns
///
/// `NSTDInt64 v` - The negative value of `x`.
///
/// # Panics
///
/// This will panic if the negate operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_neg_i64(NSTDInt64 x);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_add_int(NSTDInt x, NSTDInt y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_add_uint(NSTDUInt x, NSTDUInt y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_add_i8(NSTDInt8 x, NSTDInt8 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_add_u8(NSTDUInt8 x, NSTDUInt8 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_add_i16(NSTDInt16 x, NSTDInt16 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_add_u16(NSTDUInt16 x, NSTDUInt16 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_add_i32(NSTDInt32 x, NSTDInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_add_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_add_i64(NSTDInt64 x, NSTDInt64 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the addition operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_add_u64(NSTDUInt64 x, NSTDUInt64 y);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_sub_int(NSTDInt x, NSTDInt y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_sub_uint(NSTDUInt x, NSTDUInt y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_sub_i8(NSTDInt8 x, NSTDInt8 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_sub_u8(NSTDUInt8 x, NSTDUInt8 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_sub_i16(NSTDInt16 x, NSTDInt16 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_sub_u16(NSTDUInt16 x, NSTDUInt16 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_sub_i32(NSTDInt32 x, NSTDInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_sub_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_sub_i64(NSTDInt64 x, NSTDInt64 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the subtraction operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_sub_u64(NSTDUInt64 x, NSTDUInt64 y);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_mul_int(NSTDInt x, NSTDInt y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_mul_uint(NSTDUInt x, NSTDUInt y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_mul_i8(NSTDInt8 x, NSTDInt8 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_mul_u8(NSTDUInt8 x, NSTDUInt8 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_mul_i16(NSTDInt16 x, NSTDInt16 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_mul_u16(NSTDUInt16 x, NSTDUInt16 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_mul_i32(NSTDInt32 x, NSTDInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_mul_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_mul_i64(NSTDInt64 x, NSTDInt64 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the multiplication operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_mul_u64(NSTDUInt64 x, NSTDUInt64 y);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_div_int(NSTDInt x, NSTDInt y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_div_uint(NSTDUInt x, NSTDUInt y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_div_i8(NSTDInt8 x, NSTDInt8 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_div_u8(NSTDUInt8 x, NSTDUInt8 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_div_i16(NSTDInt16 x, NSTDInt16 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_div_u16(NSTDUInt16 x, NSTDUInt16 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_div_i32(NSTDInt32 x, NSTDInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_div_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_div_i64(NSTDInt64 x, NSTDInt64 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the division operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_div_u64(NSTDUInt64 x, NSTDUInt64 y);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_rem_int(NSTDInt x, NSTDInt y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_rem_uint(NSTDUInt x, NSTDUInt y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_rem_i8(NSTDInt8 x, NSTDInt8 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_rem_u8(NSTDUInt8 x, NSTDUInt8 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_rem_i16(NSTDInt16 x, NSTDInt16 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_rem_u16(NSTDUInt16 x, NSTDUInt16 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_rem_i32(NSTDInt32 x, NSTDInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_rem_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_rem_i64(NSTDInt64 x, NSTDInt64 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if `y` is 0 or the remainder operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_rem_u64(NSTDUInt64 x, NSTDUInt64 y);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_shl_int(NSTDInt x, NSTDUInt32 y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_shl_uint(NSTDUInt x, NSTDUInt32 y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_shl_i8(NSTDInt8 x, NSTDUInt32 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_shl_u8(NSTDUInt8 x, NSTDUInt32 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_shl_i16(NSTDInt16 x, NSTDUInt32 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_shl_u16(NSTDUInt16 x, NSTDUInt32 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_shl_i32(NSTDInt32 x, NSTDUInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_shl_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_shl_i64(NSTDInt64 x, NSTDUInt32 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the left shift operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_shl_u64(NSTDUInt64 x, NSTDUInt32 y);

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
/// `NSTDInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDInt nstd_core_ops_shr_int(NSTDInt x, NSTDUInt32 y);
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
/// `NSTDUInt z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDUInt nstd_core_ops_shr_uint(NSTDUInt x, NSTDUInt32 y);
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
/// `NSTDInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDInt8 nstd_core_ops_shr_i8(NSTDInt8 x, NSTDUInt32 y);
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
/// `NSTDUInt8 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDUInt8 nstd_core_ops_shr_u8(NSTDUInt8 x, NSTDUInt32 y);
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
/// `NSTDInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDInt16 nstd_core_ops_shr_i16(NSTDInt16 x, NSTDUInt32 y);
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
/// `NSTDUInt16 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDUInt16 nstd_core_ops_shr_u16(NSTDUInt16 x, NSTDUInt32 y);
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
/// `NSTDInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDInt32 nstd_core_ops_shr_i32(NSTDInt32 x, NSTDUInt32 y);
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
/// `NSTDUInt32 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDUInt32 nstd_core_ops_shr_u32(NSTDUInt32 x, NSTDUInt32 y);
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
/// `NSTDInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDInt64 nstd_core_ops_shr_i64(NSTDInt64 x, NSTDUInt32 y);
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
/// `NSTDUInt64 z` - The result of the operation.
///
/// # Panics
///
/// This will panic if the right shift operation results in an overflow.
NSTDAPI NSTDUInt64 nstd_core_ops_shr_u64(NSTDUInt64 x, NSTDUInt32 y);

#endif
