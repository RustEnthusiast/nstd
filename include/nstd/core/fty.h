#ifndef NSTD_CORE_FTY_H
#define NSTD_CORE_FTY_H
#include "../nstd.h"

/// Returns the smallest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 min` - The smallest finite value representable by `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_min_f32(void);
/// Returns the largest finite value representable by `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 max` - The largest finite value representable by `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_max_f32(void);
/// Returns the smallest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 min` - The smallest finite value representable by `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_min_f64(void);
/// Returns the largest finite value representable by `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 max` - The largest finite value representable by `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_max_f64(void);

/// Returns NaN represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 nan` - NaN represented as `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_nan_f32(void);
/// Returns NaN represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 nan` - NaN represented as `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_nan_f64(void);

/// Returns infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 inf` - Infinity represented as `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_inf_f32(void);
/// Returns infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 inf` - Infinity represented as `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_inf_f64(void);

/// Returns negative infinity represented as `NSTDFloat32`.
///
/// # Returns
///
/// `NSTDFloat32 neg_inf` - Negative infinity represented as `NSTDFloat32`.
NSTDAPI NSTDFloat32 nstd_core_fty_neg_inf_f32(void);
/// Returns negative infinity represented as `NSTDFloat64`.
///
/// # Returns
///
/// `NSTDFloat64 neg_inf` - Negative infinity represented as `NSTDFloat64`.
NSTDAPI NSTDFloat64 nstd_core_fty_neg_inf_f64(void);

#endif
