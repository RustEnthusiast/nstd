//! Access symbols from loaded shared libraries.
//!
//! # Platform support
//!
//! This module is only functional on Windows and Unix systems.
#![cfg(any(unix, windows))]
#[cfg(unix)]
use crate::os::unix::shared_lib::{
    nstd_os_unix_shared_lib_get, nstd_os_unix_shared_lib_get_mut, nstd_os_unix_shared_lib_load,
    NSTDUnixSharedLib,
};
#[cfg(windows)]
use crate::os::windows::shared_lib::{
    nstd_os_windows_shared_lib_get, nstd_os_windows_shared_lib_get_mut,
    nstd_os_windows_shared_lib_load, NSTDWindowsSharedLib,
};
use crate::{
    core::{
        cstr::{nstd_core_cstr_as_ptr, nstd_core_cstr_get_null, NSTDCStr},
        optional::NSTDOptional,
    },
    cstring::{nstd_cstring_as_ptr, nstd_cstring_from_cstr_unchecked},
    NSTDAny, NSTDAnyMut, NSTDChar,
};

/// A handle to a dynamically loaded library.
#[cfg(unix)]
pub type NSTDSharedLib = NSTDUnixSharedLib;
/// A handle to a dynamically loaded library.
#[cfg(windows)]
pub type NSTDSharedLib = NSTDWindowsSharedLib;

/// An optional handle to a shared library.
///
/// This type is returned from `nstd_shared_lib_load`.
pub type NSTDOptionalSharedLib = NSTDOptional<NSTDSharedLib>;

/// Dynamically loads a shared library at runtime.
///
/// # Parameters:
///
/// - `const NSTDCStr *path` - A path to the shared library.
///
/// # Returns
///
/// `NSTDOptionalSharedLib lib` - A handle to the dynamically loaded library, or none on error.
///
/// # Panics
///
/// Panics if `path`'s length in bytes exceeds `NSTDInt`'s max value or allocating fails.
///
/// # Safety
///
/// - `path`'s data must be valid for reads.
///
/// - The loaded library may have platform-specific initialization routines ran when it is loaded.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_lib_load(path: &NSTDCStr) -> NSTDOptionalSharedLib {
    // Check if `path` is already null terminated.
    if nstd_core_cstr_get_null(path).is_null() {
        // Allocate a null byte for `path`.
        let path = nstd_cstring_from_cstr_unchecked(path);
        #[cfg(unix)]
        return nstd_os_unix_shared_lib_load(nstd_cstring_as_ptr(&path));
        #[cfg(windows)]
        return nstd_os_windows_shared_lib_load(nstd_cstring_as_ptr(&path));
    } else {
        // Use the already null terminated `path`.
        #[cfg(unix)]
        return nstd_os_unix_shared_lib_load(nstd_core_cstr_as_ptr(path));
        #[cfg(windows)]
        return nstd_os_windows_shared_lib_load(nstd_core_cstr_as_ptr(path));
    }
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_lib_get(
    lib: &NSTDSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAny {
    #[cfg(unix)]
    return nstd_os_unix_shared_lib_get(lib, symbol);
    #[cfg(windows)]
    return nstd_os_windows_shared_lib_get(lib, symbol);
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_lib_get_mut(
    lib: &mut NSTDSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAnyMut {
    #[cfg(unix)]
    return nstd_os_unix_shared_lib_get_mut(lib, symbol);
    #[cfg(windows)]
    return nstd_os_windows_shared_lib_get_mut(lib, symbol);
}

/// Unloads and frees the resources of a dynamically loaded library.
///
/// # Parameters:
///
/// - `NSTDSharedLib lib` - The library handle.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn nstd_shared_lib_free(lib: NSTDSharedLib) {}
