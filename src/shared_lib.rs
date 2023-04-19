//! Access symbols from loaded shared libraries.
//!
//! # Platform support
//!
//! This module is only functional on Windows and Unix systems.
#![cfg(any(unix, windows))]
use crate::{
    core::{optional::NSTDOptional, str::NSTDStr},
    NSTDAny, NSTDAnyMut, NSTDChar,
};
use cfg_if::cfg_if;
use nstdapi::nstdapi;

cfg_if! {
    if #[cfg(unix)] {
        use crate::{
            core::{
                cstr::{nstd_core_cstr_as_ptr, nstd_core_cstr_get_null},
                str::nstd_core_str_as_cstr,
            },
            cstring::{nstd_cstring_as_ptr, nstd_cstring_from_cstr_unchecked},
        };
        use libc::{dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};

        /// A handle to a dynamically loaded library.
        #[nstdapi]
        pub struct NSTDSharedLib {
            /// A raw handle to the shared library.
            handle: NSTDAnyMut,
        }
        impl Drop for NSTDSharedLib {
            /// [NSTDSharedLib]s destructor.
            #[inline]
            fn drop(&mut self) {
                // SAFETY: `self.handle` is valid.
                unsafe { dlclose(self.handle) };
            }
        }
        // SAFETY: `NSTDSharedLib` owns a handle to the dynamically loaded library.
        unsafe impl Send for NSTDSharedLib {}
        // SAFETY: `NSTDSharedLib` does not undergo interior mutability.
        unsafe impl Sync for NSTDSharedLib {}
    } else if #[cfg(windows)] {
        use crate::{
            os::windows::{
                shared_lib::{
                    nstd_os_windows_shared_lib_get, nstd_os_windows_shared_lib_get_mut,
                    nstd_os_windows_shared_lib_load, NSTDWindowsSharedLib,
                },
                str::nstd_os_windows_str_to_utf16,
            },
            vec::nstd_vec_as_ptr,
        };

        /// A handle to a dynamically loaded library.
        pub type NSTDSharedLib = NSTDWindowsSharedLib;
    }
}

/// An optional handle to a shared library.
///
/// This type is returned from `nstd_shared_lib_load`.
pub type NSTDOptionalSharedLib = NSTDOptional<NSTDSharedLib>;

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
/// # Panics
///
/// This operation may panic in the following situations:
///
/// - Allocating memory fails.
///
/// - Conversion from UTF-8 to UTF-16 fails on Windows.
///
/// # Safety
///
/// - `path`'s data must be valid for reads.
///
/// - The loaded library may have platform-specific initialization routines ran when it is loaded.
#[nstdapi]
pub unsafe fn nstd_shared_lib_load(path: &NSTDStr) -> NSTDOptionalSharedLib {
    #[cfg(unix)]
    {
        // Check if `path` is already null terminated.
        let path = nstd_core_str_as_cstr(path);
        if nstd_core_cstr_get_null(&path).is_null() {
            // Allocate a null byte for `path`.
            if let NSTDOptional::Some(path) = nstd_cstring_from_cstr_unchecked(&path) {
                let handle = dlopen(nstd_cstring_as_ptr(&path), RTLD_LAZY | RTLD_LOCAL);
                if !handle.is_null() {
                    return NSTDOptional::Some(NSTDSharedLib { handle });
                }
            }
            NSTDOptional::None
        } else {
            // Use the already null terminated `path`.
            let handle = dlopen(nstd_core_cstr_as_ptr(&path), RTLD_LAZY | RTLD_LOCAL);
            match !handle.is_null() {
                true => NSTDOptional::Some(NSTDSharedLib { handle }),
                false => NSTDOptional::None,
            }
        }
    }
    #[cfg(windows)]
    {
        let utf16 = nstd_os_windows_str_to_utf16(path);
        nstd_os_windows_shared_lib_load(nstd_vec_as_ptr(&utf16) as _)
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
#[nstdapi]
pub unsafe fn nstd_shared_lib_get(lib: &NSTDSharedLib, symbol: *const NSTDChar) -> NSTDAny {
    #[cfg(unix)]
    return dlsym(lib.handle, symbol);
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
#[nstdapi]
pub unsafe fn nstd_shared_lib_get_mut(
    lib: &mut NSTDSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAnyMut {
    #[cfg(unix)]
    return dlsym(lib.handle, symbol);
    #[cfg(windows)]
    return nstd_os_windows_shared_lib_get_mut(lib, symbol);
}

/// Unloads and frees the resources of a dynamically loaded library.
///
/// # Parameters:
///
/// - `NSTDSharedLib lib` - The library handle.
#[inline]
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_shared_lib_free(lib: NSTDSharedLib) {}
