//! Access symbols from loaded shared libraries.
//!
//! # Platform support
//!
//! This module is only functional on Windows and Unix systems.
use crate::{
    core::{cstr::raw::nstd_core_cstr_raw_len_with_null, str::NSTDStr},
    NSTDAny, NSTDAnyMut, NSTDChar, NSTD_NULL,
};
use libloading::{Error, Library, Symbol};

/// A handle to a dynamically loaded library.
pub type NSTDSharedLib = Box<Library>;

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
/// # Panics
///
/// Panics if `path`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// See <https://docs.rs/libloading/latest/libloading/struct.Library.html#method.new>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_shared_lib_load(path: &NSTDStr) -> Option<NSTDSharedLib> {
    match Library::new(path.as_str()) {
        Ok(lib) => Some(Box::new(lib)),
        _ => None,
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
    match get(lib, symbol) {
        Ok(ptr) => *ptr,
        _ => NSTD_NULL,
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
    match get(lib, symbol) {
        Ok(ptr) => *ptr,
        _ => NSTD_NULL,
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
unsafe fn get<T>(lib: &NSTDSharedLib, symbol: *const NSTDChar) -> Result<Symbol<T>, Error> {
    let symbol_len = nstd_core_cstr_raw_len_with_null(symbol);
    assert!(symbol_len <= isize::MAX as usize);
    let symbol_name = std::slice::from_raw_parts(symbol.cast(), symbol_len);
    lib.get(symbol_name)
}
