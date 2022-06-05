//! Provides functions for examining and operating on floating point types.
use crate::{NSTDFloat32, NSTDFloat64};

/// Returns the smallest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 min` - The smallest finite value representable by `NSTDFloat32`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f32_min() -> NSTDFloat32 {
    NSTDFloat32::MIN
}
/// Returns the largest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 max` - The largest finite value representable by `NSTDFloat32`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f32_max() -> NSTDFloat32 {
    NSTDFloat32::MAX
}
/// Returns the smallest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 min` - The smallest finite value representable by `NSTDFloat64`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f64_min() -> NSTDFloat64 {
    NSTDFloat64::MIN
}
/// Returns the largest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 max` - The largest finite value representable by `NSTDFloat64`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f64_max() -> NSTDFloat64 {
    NSTDFloat64::MAX
}

/// Returns NaN represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 nan` - NaN represented as `NSTDFloat32`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f32_nan() -> NSTDFloat32 {
    NSTDFloat32::NAN
}
/// Returns NaN represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 nan` - NaN represented as `NSTDFloat64`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f64_nan() -> NSTDFloat64 {
    NSTDFloat64::NAN
}

/// Returns infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 inf` - Infinity represented as `NSTDFloat32`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f32_inf() -> NSTDFloat32 {
    NSTDFloat32::INFINITY
}
/// Returns infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 inf` - Infinity represented as `NSTDFloat64`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f64_inf() -> NSTDFloat64 {
    NSTDFloat64::INFINITY
}

/// Returns negative infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 neg_inf` - Negative infinity represented as `NSTDFloat32`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f32_neg_inf() -> NSTDFloat32 {
    NSTDFloat32::NEG_INFINITY
}
/// Returns negative infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 neg_inf` - Negative infinity represented as `NSTDFloat64`.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_fty_f64_neg_inf() -> NSTDFloat64 {
    NSTDFloat64::NEG_INFINITY
}
