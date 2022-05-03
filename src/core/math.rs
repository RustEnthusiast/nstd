//! Low level math operations.
use crate::core::def::{NSTDFloat32, NSTDFloat64};

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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_deg_f32(rad: NSTDFloat32) -> NSTDFloat32 {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_deg_f64(rad: NSTDFloat64) -> NSTDFloat64 {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_rad_f32(deg: NSTDFloat32) -> NSTDFloat32 {
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
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_rad_f64(deg: NSTDFloat64) -> NSTDFloat64 {
    deg.to_radians()
}

/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDFloat32 x` - The value to clamp.
///
/// - `NSTDFloat32 min` - The minimum clamp value.
///
/// - `NSTDFloat32 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDFloat32 v` - The clamped value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_clamp_f32(
    x: NSTDFloat32,
    min: NSTDFloat32,
    max: NSTDFloat32,
) -> NSTDFloat32 {
    x.clamp(min, max)
}
/// Clamps the value `x` to the bounds `min` and `max`.
///
/// # Parameters:
///
/// - `NSTDFloat64 x` - The value to clamp.
///
/// - `NSTDFloat64 min` - The minimum clamp value.
///
/// - `NSTDFloat64 max` - The maximum clamp value.
///
/// # Returns
///
/// `NSTDFloat64 v` - The clamped value.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub extern "C" fn nstd_core_math_clamp_f64(
    x: NSTDFloat64,
    min: NSTDFloat64,
    max: NSTDFloat64,
) -> NSTDFloat64 {
    x.clamp(min, max)
}
