#ifndef NSTD_SHARED_LIB_H
#define NSTD_SHARED_LIB_H
#include "core/def.h"
#include "core/str.h"
#include "nstd.h"
NSTDCPPSTART

/// A handle to a dynamically loaded library.
typedef NSTDAnyMut NSTDSharedLib;

/// Dynamically loads a shared library at runtime.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the shared library.
///
/// # Returns
///
/// `NSTDSharedLib lib` - A handle to the dynamically loaded library, or null on error.
///
/// # Safety
///
/// See <https://docs.rs/libloading/latest/libloading/struct.Library.html#method.new>.
NSTDAPI NSTDSharedLib nstd_shared_lib_load(const NSTDStr *path);

/// Gets a pointer to a function or static variable in a dynamically loaded library by symbol name.
///
/// # Parameters
///
/// - `const NSTDSharedLib *lib` - The loaded library.
///
/// - `const NSTDChar *symbol` - The name of the function or variable to get a pointer to.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the function or variable.
///
/// # Safety
///
/// Undefined behavior may occur if `symbol`'s data is invalid.
NSTDAPI NSTDAny nstd_shared_lib_get(const NSTDSharedLib *lib, const NSTDChar *symbol);

/// Gets a mutable pointer to a function or static variable in a dynamically loaded library by
/// symbol name.
///
/// # Parameters
///
/// - `NSTDSharedLib *lib` - The loaded library.
///
/// - `const NSTDChar *symbol` - The name of the function or variable to get a pointer to.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the function or variable.
///
/// # Safety
///
/// Undefined behavior may occur if `symbol`'s data is invalid.
NSTDAPI NSTDAnyMut nstd_shared_lib_get_mut(NSTDSharedLib *lib, const NSTDChar *symbol);

/// Unloads and frees the resources of a dynamically loaded library.
///
/// # Parameters:
///
/// - `NSTDSharedLib lib` - The library handle.
NSTDAPI void nstd_shared_lib_free(NSTDSharedLib lib);

NSTDCPPEND
#endif
