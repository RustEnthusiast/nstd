//! Provides functions for examining and operating on floating point types.
use crate::{NSTDFloat32, NSTDFloat64};
use nstdapi::nstdapi;

/// Returns the smallest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 min` - The smallest finite value representable by `NSTDFloat32`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_min_f32;
///
/// # unsafe {
/// assert!(nstd_core_fty_min_f32() == f32::MIN);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_min_f32() -> NSTDFloat32 {
    NSTDFloat32::MIN
}
/// Returns the largest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 max` - The largest finite value representable by `NSTDFloat32`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_max_f32;
///
/// # unsafe {
/// assert!(nstd_core_fty_max_f32() == f32::MAX);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_max_f32() -> NSTDFloat32 {
    NSTDFloat32::MAX
}
/// Returns the smallest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 min` - The smallest finite value representable by `NSTDFloat64`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_min_f64;
///
/// # unsafe {
/// assert!(nstd_core_fty_min_f64() == f64::MIN);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_min_f64() -> NSTDFloat64 {
    NSTDFloat64::MIN
}
/// Returns the largest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 max` - The largest finite value representable by `NSTDFloat64`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_max_f64;
///
/// # unsafe {
/// assert!(nstd_core_fty_max_f64() == f64::MAX);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_max_f64() -> NSTDFloat64 {
    NSTDFloat64::MAX
}

/// Returns NaN represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 nan` - NaN represented as `NSTDFloat32`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_nan_f32;
///
/// # unsafe {
/// assert!(nstd_core_fty_nan_f32().is_nan());
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_nan_f32() -> NSTDFloat32 {
    NSTDFloat32::NAN
}
/// Returns NaN represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 nan` - NaN represented as `NSTDFloat64`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_nan_f64;
///
/// # unsafe {
/// assert!(nstd_core_fty_nan_f64().is_nan());
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_nan_f64() -> NSTDFloat64 {
    NSTDFloat64::NAN
}

/// Returns infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 inf` - Infinity represented as `NSTDFloat32`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_inf_f32;
///
/// # unsafe {
/// assert!(nstd_core_fty_inf_f32() == f32::INFINITY);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_inf_f32() -> NSTDFloat32 {
    NSTDFloat32::INFINITY
}
/// Returns infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 inf` - Infinity represented as `NSTDFloat64`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_inf_f64;
///
/// # unsafe {
/// assert!(nstd_core_fty_inf_f64() == f64::INFINITY);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_inf_f64() -> NSTDFloat64 {
    NSTDFloat64::INFINITY
}

/// Returns negative infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 neg_inf` - Negative infinity represented as `NSTDFloat32`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_neg_inf_f32;
///
/// # unsafe {
/// assert!(nstd_core_fty_neg_inf_f32() == f32::NEG_INFINITY);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_neg_inf_f32() -> NSTDFloat32 {
    NSTDFloat32::NEG_INFINITY
}
/// Returns negative infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 neg_inf` - Negative infinity represented as `NSTDFloat64`.
///
/// # Example
///
/// ```
/// use nstd_sys::core::fty::nstd_core_fty_neg_inf_f64;
///
/// # unsafe {
/// assert!(nstd_core_fty_neg_inf_f64() == f64::NEG_INFINITY);
/// # }
/// ```
#[inline]
#[nstdapi]
pub const fn nstd_core_fty_neg_inf_f64() -> NSTDFloat64 {
    NSTDFloat64::NEG_INFINITY
}
