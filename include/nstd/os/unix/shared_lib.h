#ifndef NSTD_OS_UNIX_SHARED_LIB_H
#define NSTD_OS_UNIX_SHARED_LIB_H
#include "../../core/optional.h"
#include "../../nstd.h"

/// Represents an owned handle to a dynamically loaded library.
typedef struct {
    /// A raw handle to the shared library.
    NSTDAnyMut handle;
} NSTDUnixSharedLib;

/// Represents an optional `NSTDUnixSharedLib`.
NSTDOptional(NSTDUnixSharedLib) NSTDUnixOptionalSharedLib;

/// Loads a dynamically loaded shared library.
///
/// # Parameters:
///
/// - `const NSTDChar *path` - A path to the shared library to load.
///
/// # Returns
///
/// `NSTDUnixOptionalSharedLib lib` - A handle to the loaded library.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/dlopen.3.html>.
NSTDAPI NSTDUnixOptionalSharedLib nstd_os_unix_shared_lib_load(const NSTDChar *path);

/// Returns an immutable opaque pointer to a symbol in a loaded library.
///
/// # Parameters:
///
/// - `const NSTDUnixSharedLib *lib` - The shared library.
///
/// - `const NSTDChar *symbol` - The symbol to retrieve a pointer to.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the loaded symbol, null on error.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/dlsym.3.html>.
NSTDAPI NSTDAny nstd_os_unix_shared_lib_get(const NSTDUnixSharedLib *lib, const NSTDChar *symbol);

/// Returns a mutable opaque pointer to a symbol in a loaded library.
///
/// # Parameters:
///
/// - `NSTDUnixSharedLib *lib` - The shared library.
///
/// - `const NSTDChar *symbol` - The symbol to retrieve a pointer to.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the loaded symbol, null on error.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/dlsym.3.html>.
NSTDAPI NSTDAnyMut nstd_os_unix_shared_lib_get_mut(NSTDUnixSharedLib *lib, const NSTDChar *symbol);

/// Closes and frees a loaded shared library.
///
/// # Parameters:
///
/// - `NSTDUnixSharedLib lib` - A handle to the loaded library to unload.
NSTDAPI void nstd_os_unix_shared_lib_free(NSTDUnixSharedLib lib);

#endif
