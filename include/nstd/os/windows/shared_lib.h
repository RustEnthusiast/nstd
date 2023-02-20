#ifndef NSTD_OS_WINDOWS_SHARED_LIB_H
#define NSTD_OS_WINDOWS_SHARED_LIB_H
#include "../../core/optional.h"
#include "../../nstd.h"
#include "windows.h"

/// A handle to a loaded library.
typedef struct {
    /// A raw handle to the module.
    NSTDWindowsHandle handle;
} NSTDWindowsSharedLib;

/// An optional (possibly null) shared Windows library handle.
NSTDOptional(NSTDWindowsSharedLib) NSTDWindowsOptionalSharedLib;

/// Loads a shared library/module by name.
///
/// # Parameters:
///
/// - `const NSTDChar16 *name` - The name of the module to load.
///
/// # Returns
///
/// `NSTDWindowsOptionalSharedLib lib` - A handle to the shared library.
///
/// # Safety
///
/// See
/// <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw>.
NSTDAPI NSTDWindowsOptionalSharedLib nstd_os_windows_shared_lib_load(const NSTDChar16 *name);

/// Returns a raw handle to a dynamically loaded library.
///
/// # Parameters:
///
/// - `const NSTDWindowsSharedLib *lib` - The loaded library.
///
/// # Returns
///
/// `NSTDWindowsHandle handle` - A native handle to the dynamically loaded library.
NSTDAPI NSTDWindowsHandle nstd_os_windows_shared_lib_handle(const NSTDWindowsSharedLib *lib);

/// Gets a pointer to a function or static variable in a dynamically loaded library by symbol name.
///
/// # Parameters
///
/// - `const NSTDWindowsSharedLib *lib` - The loaded library.
///
/// - `const NSTDChar *symbol` - The name of the function or variable to get a pointer to.
///
/// # Returns
///
/// `NSTDAny ptr` - A pointer to the function or variable.
///
/// # Safety
///
/// See
/// <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>.
NSTDAPI NSTDAny
nstd_os_windows_shared_lib_get(const NSTDWindowsSharedLib *lib, const NSTDChar *symbol);

/// Gets a mutable pointer to a function or static variable in a dynamically loaded library by
/// symbol name.
///
/// # Parameters
///
/// - `NSTDWindowsSharedLib *lib` - The loaded library.
///
/// - `const NSTDChar *symbol` - The name of the function or variable to get a pointer to.
///
/// # Returns
///
/// `NSTDAnyMut ptr` - A pointer to the function or variable.
///
/// # Safety
///
/// See
/// <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>.
NSTDAPI NSTDAnyMut
nstd_os_windows_shared_lib_get_mut(NSTDWindowsSharedLib *lib, const NSTDChar *symbol);

/// Unloads and frees a dynamically loaded shared library.
///
/// # Parameters:
///
/// - `NSTDWindowsSharedLib lib` - The library handle.
///
/// # Safety
///
/// See
/// <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary>.
NSTDAPI void nstd_os_windows_shared_lib_free(NSTDWindowsSharedLib lib);

#endif
