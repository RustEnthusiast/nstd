#ifndef NSTD_CORE_FTY_H_INCLUDED
#define NSTD_CORE_FTY_H_INCLUDED
#include "../nstd.h"
#include "def.h"
NSTDCPPSTART

/// Returns the smallest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 min` - The smallest finite value representable by `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_f32_min();
/// Returns the largest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 max` - The largest finite value representable by `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_f32_max();
/// Returns the smallest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 min` - The smallest finite value representable by `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_f64_min();
/// Returns the largest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 max` - The largest finite value representable by `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_f64_max();

/// Returns NaN represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 nan` - NaN represented as `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_f32_nan();
/// Returns NaN represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 nan` - NaN represented as `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_f64_nan();

/// Returns infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 inf` - Infinity represented as `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_f32_inf();
/// Returns infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 inf` - Infinity represented as `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_f64_inf();

/// Returns negative infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 neg_inf` - Negative infinity represented as `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_f32_neg_inf();
/// Returns negative infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 neg_inf` - Negative infinity represented as `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_f64_neg_inf();

NSTDCPPEND
#endif
