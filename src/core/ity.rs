//! Provides functions for examining and operating on integral types.
use crate::{
    NSTDISize, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt16, NSTDUInt32, NSTDUInt64,
    NSTDUInt8, NSTDUSize,
};

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
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $minname() -> $T {
            <$T>::MIN
        }

        $(#[$maxmeta])*
        #[inline]
        #[cfg_attr(feature = "clib", no_mangle)]
        pub extern "C" fn $maxname() -> $T {
            <$T>::MAX
        }
    };
}
gen_min_max!(
    /// Returns the smallest value representable by `NSTDUInt8`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt8 min` - The smallest value representable by `NSTDUInt8`.
    nstd_core_ity_min_u8,
    /// Returns the largest value representable by `NSTDUInt8`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt8 max` - The largest value representable by `NSTDUInt8`.
    nstd_core_ity_max_u8,
    NSTDUInt8
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDInt8`.
    ///
    /// # Returns
    ///
    /// `NSTDInt8 min` - The smallest value representable by `NSTDInt8`.
    nstd_core_ity_min_i8,
    /// Returns the largest value representable by `NSTDInt8`.
    ///
    /// # Returns
    ///
    /// `NSTDInt8 max` - The largest value representable by `NSTDInt8`.
    nstd_core_ity_max_i8,
    NSTDInt8
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDUInt16`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt16 min` - The smallest value representable by `NSTDUInt16`.
    nstd_core_ity_min_u16,
    /// Returns the largest value representable by `NSTDUInt16`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt16 max` - The largest value representable by `NSTDUInt16`.
    nstd_core_ity_max_u16,
    NSTDUInt16
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDInt16`.
    ///
    /// # Returns
    ///
    /// `NSTDInt16 min` - The smallest value representable by `NSTDInt16`.
    nstd_core_ity_min_i16,
    /// Returns the largest value representable by `NSTDInt16`.
    ///
    /// # Returns
    ///
    /// `NSTDInt16 max` - The largest value representable by `NSTDInt16`.
    nstd_core_ity_max_i16,
    NSTDInt16
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDUInt32`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt32 min` - The smallest value representable by `NSTDUInt32`.
    nstd_core_ity_min_u32,
    /// Returns the largest value representable by `NSTDUInt32`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt32 max` - The largest value representable by `NSTDUInt32`.
    nstd_core_ity_max_u32,
    NSTDUInt32
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDInt32`.
    ///
    /// # Returns
    ///
    /// `NSTDInt32 min` - The smallest value representable by `NSTDInt32`.
    nstd_core_ity_min_i32,
    /// Returns the largest value representable by `NSTDInt32`.
    ///
    /// # Returns
    ///
    /// `NSTDInt32 max` - The largest value representable by `NSTDInt32`.
    nstd_core_ity_max_i32,
    NSTDInt32
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDUInt64`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt64 min` - The smallest value representable by `NSTDUInt64`.
    nstd_core_ity_min_u64,
    /// Returns the largest value representable by `NSTDUInt64`.
    ///
    /// # Returns
    ///
    /// `NSTDUInt64 max` - The largest value representable by `NSTDUInt64`.
    nstd_core_ity_max_u64,
    NSTDUInt64
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDInt64`.
    ///
    /// # Returns
    ///
    /// `NSTDInt64 min` - The smallest value representable by `NSTDInt64`.
    nstd_core_ity_min_i64,
    /// Returns the largest value representable by `NSTDInt64`.
    ///
    /// # Returns
    ///
    /// `NSTDInt64 max` - The largest value representable by `NSTDInt64`.
    nstd_core_ity_max_i64,
    NSTDInt64
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDUSize`.
    ///
    /// # Returns
    ///
    /// `NSTDUSize min` - The smallest value representable by `NSTDUSize`.
    nstd_core_ity_min_usize,
    /// Returns the largest value representable by `NSTDUSize`.
    ///
    /// # Returns
    ///
    /// `NSTDUSize max` - The largest value representable by `NSTDUSize`.
    nstd_core_ity_max_usize,
    NSTDUSize
);
gen_min_max!(
    /// Returns the smallest value representable by `NSTDISize`.
    ///
    /// # Returns
    ///
    /// `NSTDISize min` - The smallest value representable by `NSTDISize`.
    nstd_core_ity_min_isize,
    /// Returns the largest value representable by `NSTDISize`.
    ///
    /// # Returns
    ///
    /// `NSTDISize max` - The largest value representable by `NSTDISize`.
    nstd_core_ity_max_isize,
    NSTDISize
);
