#ifndef NSTD_CORE_ITY_H
#define NSTD_CORE_ITY_H
#include "../nstd.h"

/// Returns the smallest value representable by `NSTDInt`.
///
/// # Returns
///
/// `NSTDInt min` - The smallest value representable by `NSTDInt`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_int;
///
/// assert!(nstd_core_ity_min_int() == isize::MIN);
/// ```
NSTDAPI NSTDInt nstd_core_ity_min_int();
/// Returns the largest value representable by `NSTDInt`.
///
/// # Returns
///
/// `NSTDInt max` - The largest value representable by `NSTDInt`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_int;
///
/// assert!(nstd_core_ity_max_int() == isize::MAX);
/// ```
NSTDAPI NSTDInt nstd_core_ity_max_int();
/// Returns the smallest value representable by `NSTDUInt`.
///
/// # Returns
///
/// `NSTDUInt min` - The smallest value representable by `NSTDUInt`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_uint;
///
/// assert!(nstd_core_ity_min_uint() == usize::MIN);
/// ```
NSTDAPI NSTDUInt nstd_core_ity_min_uint();
/// Returns the largest value representable by `NSTDUInt`.
///
/// # Returns
///
/// `NSTDUInt max` - The largest value representable by `NSTDUInt`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_uint;
///
/// assert!(nstd_core_ity_max_uint() == usize::MAX);
/// ```
NSTDAPI NSTDUInt nstd_core_ity_max_uint();
/// Returns the smallest value representable by `NSTDInt8`.
///
/// # Returns
///
/// `NSTDInt8 min` - The smallest value representable by `NSTDInt8`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_i8;
///
/// assert!(nstd_core_ity_min_i8() == i8::MIN);
/// ```
NSTDAPI NSTDInt8 nstd_core_ity_min_i8();
/// Returns the largest value representable by `NSTDInt8`.
///
/// # Returns
///
/// `NSTDInt8 max` - The largest value representable by `NSTDInt8`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_i8;
///
/// assert!(nstd_core_ity_max_i8() == i8::MAX);
/// ```
NSTDAPI NSTDInt8 nstd_core_ity_max_i8();
/// Returns the smallest value representable by `NSTDUInt8`.
///
/// # Returns
///
/// `NSTDUInt8 min` - The smallest value representable by `NSTDUInt8`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_u8;
///
/// assert!(nstd_core_ity_min_u8() == u8::MIN);
/// ```
NSTDAPI NSTDUInt8 nstd_core_ity_min_u8();
/// Returns the largest value representable by `NSTDUInt8`.
///
/// # Returns
///
/// `NSTDUInt8 max` - The largest value representable by `NSTDUInt8`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_u8;
///
/// assert!(nstd_core_ity_max_u8() == u8::MAX);
/// ```
NSTDAPI NSTDUInt8 nstd_core_ity_max_u8();
/// Returns the smallest value representable by `NSTDInt16`.
///
/// # Returns
///
/// `NSTDInt16 min` - The smallest value representable by `NSTDInt16`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_i16;
///
/// assert!(nstd_core_ity_min_i16() == i16::MIN);
/// ```
NSTDAPI NSTDInt16 nstd_core_ity_min_i16();
/// Returns the largest value representable by `NSTDInt16`.
///
/// # Returns
///
/// `NSTDInt16 max` - The largest value representable by `NSTDInt16`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_i16;
///
/// assert!(nstd_core_ity_max_i16() == i16::MAX);
/// ```
NSTDAPI NSTDInt16 nstd_core_ity_max_i16();
/// Returns the smallest value representable by `NSTDUInt16`.
///
/// # Returns
///
/// `NSTDUInt16 min` - The smallest value representable by `NSTDUInt16`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_u16;
///
/// assert!(nstd_core_ity_min_u16() == u16::MIN);
/// ```
NSTDAPI NSTDUInt16 nstd_core_ity_min_u16();
/// Returns the largest value representable by `NSTDUInt16`.
///
/// # Returns
///
/// `NSTDUInt16 max` - The largest value representable by `NSTDUInt16`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_u16;
///
/// assert!(nstd_core_ity_max_u16() == u16::MAX);
/// ```
NSTDAPI NSTDUInt16 nstd_core_ity_max_u16();
/// Returns the smallest value representable by `NSTDInt32`.
///
/// # Returns
///
/// `NSTDInt32 min` - The smallest value representable by `NSTDInt32`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_i32;
///
/// assert!(nstd_core_ity_min_i32() == i32::MIN);
/// ```
NSTDAPI NSTDInt32 nstd_core_ity_min_i32();
/// Returns the largest value representable by `NSTDInt32`.
///
/// # Returns
///
/// `NSTDInt32 max` - The largest value representable by `NSTDInt32`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_i32;
///
/// assert!(nstd_core_ity_max_i32() == i32::MAX);
/// ```
NSTDAPI NSTDInt32 nstd_core_ity_max_i32();
/// Returns the smallest value representable by `NSTDUInt32`.
///
/// # Returns
///
/// `NSTDUInt32 min` - The smallest value representable by `NSTDUInt32`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_u32;
///
/// assert!(nstd_core_ity_min_u32() == u32::MIN);
/// ```
NSTDAPI NSTDUInt32 nstd_core_ity_min_u32();
/// Returns the largest value representable by `NSTDUInt32`.
///
/// # Returns
///
/// `NSTDUInt32 max` - The largest value representable by `NSTDUInt32`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_u32;
///
/// assert!(nstd_core_ity_max_u32() == u32::MAX);
/// ```
NSTDAPI NSTDUInt32 nstd_core_ity_max_u32();
/// Returns the smallest value representable by `NSTDInt64`.
///
/// # Returns
///
/// `NSTDInt64 min` - The smallest value representable by `NSTDInt64`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_i64;
///
/// assert!(nstd_core_ity_min_i64() == i64::MIN);
/// ```
NSTDAPI NSTDInt64 nstd_core_ity_min_i64();
/// Returns the largest value representable by `NSTDInt64`.
///
/// # Returns
///
/// `NSTDInt64 max` - The largest value representable by `NSTDInt64`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_i64;
///
/// assert!(nstd_core_ity_max_i64() == i64::MAX);
/// ```
NSTDAPI NSTDInt64 nstd_core_ity_max_i64();
/// Returns the smallest value representable by `NSTDUInt64`.
///
/// # Returns
///
/// `NSTDUInt64 min` - The smallest value representable by `NSTDUInt64`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_min_u64;
///
/// assert!(nstd_core_ity_min_u64() == u64::MIN);
/// ```
NSTDAPI NSTDUInt64 nstd_core_ity_min_u64();
/// Returns the largest value representable by `NSTDUInt64`.
///
/// # Returns
///
/// `NSTDUInt64 max` - The largest value representable by `NSTDUInt64`.
///
/// # Examples
///
/// ```
/// use nstd_sys::core::ity::nstd_core_ity_max_u64;
///
/// assert!(nstd_core_ity_max_u64() == u64::MAX);
/// ```
NSTDAPI NSTDUInt64 nstd_core_ity_max_u64();

#endif
