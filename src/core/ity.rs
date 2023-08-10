//! Provides functions for examining and operating on integral types.
use crate::{
    NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt, NSTDUInt16, NSTDUInt32,
    NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;

/// Generates the `min` and `max` functions.
macro_rules! gen_min_max {
    (
        $(#[$minmeta:meta])*
        $minname: ident,
        $(#[$maxmeta:meta])*
        $maxname: ident,
        $T: ty
    ) => {
        $(#[$minmeta])*
        #[inline]
        #[nstdapi]
        pub const fn $minname() -> $T {
            <$T>::MIN
        }

        $(#[$maxmeta])*
        #[inline]
        #[nstdapi]
        pub const fn $maxname() -> $T {
            <$T>::MAX
        }
    };
}
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_int() == isize::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_int,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_int() == isize::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_int,
    NSTDInt
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_uint() == usize::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_uint,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_uint() == usize::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_uint,
    NSTDUInt
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_i8() == i8::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_i8,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_i8() == i8::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_i8,
    NSTDInt8
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_u8() == u8::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_u8,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_u8() == u8::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_u8,
    NSTDUInt8
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_i16() == i16::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_i16,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_i16() == i16::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_i16,
    NSTDInt16
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_u16() == u16::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_u16,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_u16() == u16::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_u16,
    NSTDUInt16
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_i32() == i32::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_i32,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_i32() == i32::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_i32,
    NSTDInt32
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_u32() == u32::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_u32,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_u32() == u32::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_u32,
    NSTDUInt32
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_i64() == i64::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_i64,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_i64() == i64::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_i64,
    NSTDInt64
);
gen_min_max!(
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
    /// # unsafe {
    /// assert!(nstd_core_ity_min_u64() == u64::MIN);
    /// # }
    /// ```
    nstd_core_ity_min_u64,
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
    /// # unsafe {
    /// assert!(nstd_core_ity_max_u64() == u64::MAX);
    /// # }
    /// ```
    nstd_core_ity_max_u64,
    NSTDUInt64
);
