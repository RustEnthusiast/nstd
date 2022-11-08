//! Access symbols from loaded shared libraries.
//!
//! # Platform support
//!
//! This module is only functional on Windows and Unix systems.
#[cfg(target_os = "windows")]
use crate::{
    alloc::NSTDAllocError::NSTD_ALLOC_ERROR_NONE,
    os::windows::shared_lib::{
        nstd_os_windows_shared_lib_get, nstd_os_windows_shared_lib_get_mut,
        nstd_os_windows_shared_lib_load, NSTDWindowsSharedLib,
    },
    string::{nstd_string_from_str, nstd_string_into_bytes},
    vec::{nstd_vec_as_ptr, nstd_vec_push},
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
        use core::ptr::addr_of;
        let path = nstd_string_from_str(path);
        let mut bytes = nstd_string_into_bytes(path);
        let null = 0u8;
        assert!(nstd_vec_push(&mut bytes, addr_of!(null).cast()) == NSTD_ALLOC_ERROR_NONE);
        nstd_os_windows_shared_lib_load(nstd_vec_as_ptr(&bytes).cast())
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
