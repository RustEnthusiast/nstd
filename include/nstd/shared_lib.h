#ifndef NSTD_SHARED_LIB_H
#define NSTD_SHARED_LIB_H
#include "core/optional.h"
#include "core/str.h"
#include "nstd.h"
#include "os/os.h"
#if NSTD_OS_WINDOWS
#    include "os/windows/shared_lib.h"
#endif

/// A handle to a dynamically loaded library.
#if NSTD_OS_UNIX
typedef struct {
    /// A raw handle to the shared library.
    NSTDAnyMut handle;
} NSTDSharedLib;
#elif NSTD_OS_WINDOWS
typedef NSTDWindowsSharedLib NSTDSharedLib;
#else
typedef NSTDAnyMut NSTDSharedLib;
#endif

/// An optional handle to a shared library.
///
/// This type is returned from `nstd_shared_lib_load`.
NSTDOptional(NSTDSharedLib) NSTDOptionalSharedLib;

/// Dynamically loads a shared library at runtime.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the shared library.
///
/// # Returns
///
/// `NSTDOptionalSharedLib lib` - A handle to the dynamically loaded library, or none on error.
///
/// # Safety
///
/// - `path`'s data must be valid for reads.
///
/// - The loaded library may have platform-specific initialization routines ran when it is loaded.
NSTDAPI NSTDOptionalSharedLib nstd_shared_lib_load(const NSTDStr *path);

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

#endif
