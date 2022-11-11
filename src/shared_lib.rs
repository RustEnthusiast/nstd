//! Access symbols from loaded shared libraries.
//!
//! # Platform support
//!
//! This module is only functional on Windows and Unix systems.
#[cfg(target_os = "windows")]
use crate::{
    core::{
        cstr::{nstd_core_cstr_get_null, nstd_core_cstr_new},
        str::{nstd_core_str_as_ptr, nstd_core_str_byte_len},
    },
    cstring::{nstd_cstring_as_ptr, nstd_cstring_from_cstr},
    os::windows::shared_lib::{
        nstd_os_windows_shared_lib_get, nstd_os_windows_shared_lib_get_mut,
        nstd_os_windows_shared_lib_load, NSTDWindowsSharedLib,
    },
};
use crate::{
    core::{optional::NSTDOptional, str::NSTDStr},
    NSTDAny, NSTDAnyMut, NSTDChar,
};
#[cfg(not(target_os = "windows"))]
use libloading::{Error, Library, Symbol};

/// A handle to a dynamically loaded library.
#[cfg(not(target_os = "windows"))]
pub type NSTDSharedLib = Box<Library>;
/// A handle to a dynamically loaded library.
#[cfg(target_os = "windows")]
pub type NSTDSharedLib = NSTDWindowsSharedLib;

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
/// Panics if `path`'s length in bytes exceeds `NSTDInt`'s max value or allocating fails.
///
/// # Safety
///
/// - `path`'s data must be valid for reads.
///
/// - See <https://docs.rs/libloading/latest/libloading/struct.Library.html#method.new>.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_lib_load(path: &NSTDStr) -> NSTDOptionalSharedLib {
    #[cfg(not(target_os = "windows"))]
    {
        match Library::new(path.as_str()) {
            Ok(lib) => NSTDOptional::Some(Box::new(lib)),
            _ => NSTDOptional::None,
        }
    }
    #[cfg(target_os = "windows")]
    {
        // Check if `path` is already null terminated.
        let path_ptr = nstd_core_str_as_ptr(path).cast();
        let path_len = nstd_core_str_byte_len(path);
        let c_path = nstd_core_cstr_new(path_ptr, path_len);
        // Allocate a null byte for `path`.
        if nstd_core_cstr_get_null(&c_path).is_null() {
            let path = nstd_cstring_from_cstr(&c_path);
            nstd_os_windows_shared_lib_load(nstd_cstring_as_ptr(&path))
        }
        // Use the already null terminated `path`.
        else {
            nstd_os_windows_shared_lib_load(path_ptr)
        }
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
/// # Panics
///
/// Panics if `symbol`'s length exceeds `NSTDInt`'s max value.
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
    #[cfg(not(target_os = "windows"))]
    {
        use crate::NSTD_NULL;
        match get(lib, symbol) {
            Ok(ptr) => *ptr,
            _ => NSTD_NULL,
        }
    }
    #[cfg(target_os = "windows")]
    {
        nstd_os_windows_shared_lib_get(lib, symbol)
    }
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
/// # Panics
///
/// Panics if `symbol`'s length exceeds `NSTDInt`'s max value.
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
    #[cfg(not(target_os = "windows"))]
    {
        use crate::NSTD_NULL;
        match get(lib, symbol) {
            Ok(ptr) => *ptr,
            _ => NSTD_NULL,
        }
    }
    #[cfg(target_os = "windows")]
    {
        nstd_os_windows_shared_lib_get_mut(lib, symbol)
    }
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

/// Gets a pointer to a function or static variable in a dynamically loaded library by symbol name.
///
/// # Panics
///
/// Panics if `symbol`'s length exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// Undefined behavior may occur if `symbol`'s data is invalid.
#[cfg(not(target_os = "windows"))]
unsafe fn get<T>(lib: &NSTDSharedLib, symbol: *const NSTDChar) -> Result<Symbol<T>, Error> {
    use crate::core::cstr::raw::nstd_core_cstr_raw_len_with_null;
    let symbol_len = nstd_core_cstr_raw_len_with_null(symbol);
    assert!(symbol_len <= isize::MAX as usize);
    let symbol_name = std::slice::from_raw_parts(symbol.cast(), symbol_len);
    lib.get(symbol_name)
}
