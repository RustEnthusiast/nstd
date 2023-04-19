#include <nstd/core/optional.h>
#include <nstd/nstd.h>
#include <nstd/os/unix/shared_lib.h>
#include <dlfcn.h>

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
NSTDAPI NSTDUnixOptionalSharedLib nstd_os_unix_shared_lib_load(const NSTDChar *const path) {
    const NSTDAnyMut handle = dlopen(path, RTLD_LAZY | RTLD_LOCAL);
    if (handle) {
        const NSTDUnixOptionalSharedLib lib = {NSTD_OPTIONAL_STATUS_SOME, {{handle}}};
        return lib;
    } else {
        const NSTDUnixOptionalSharedLib lib = {NSTD_OPTIONAL_STATUS_NONE};
        return lib;
    }
}

/// Returns a raw handle to a dynamically loaded library.
///
/// # Parameters:
///
/// - `const NSTDUnixSharedLib *lib` - The shared library.
///
/// # Returns
///
/// `NSTDAnyMut handle` - A raw handle to the dynamically loaded library.
NSTDAPI NSTDAnyMut nstd_os_unix_shared_lib_handle(const NSTDUnixSharedLib *const lib) {
    return lib->handle;
}

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
NSTDAPI NSTDAny
nstd_os_unix_shared_lib_get(const NSTDUnixSharedLib *const lib, const NSTDChar *const symbol) {
    return dlsym(lib->handle, symbol);
}

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
NSTDAPI NSTDAnyMut
nstd_os_unix_shared_lib_get_mut(NSTDUnixSharedLib *const lib, const NSTDChar *const symbol) {
    return dlsym(lib->handle, symbol);
}

/// Closes and frees a loaded shared library.
///
/// # Parameters:
///
/// - `NSTDUnixSharedLib lib` - A handle to the loaded library to unload.
///
/// # Safety
///
/// See <https://man7.org/linux/man-pages/man3/dlclose.3p.html>.
NSTDAPI void nstd_os_unix_shared_lib_free(const NSTDUnixSharedLib lib) {
    dlclose(lib.handle);
}
