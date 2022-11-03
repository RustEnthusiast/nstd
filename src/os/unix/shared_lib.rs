//! Provides shared library access for Unix like systems.
use crate::{core::optional::NSTDOptional, NSTDAny, NSTDAnyMut, NSTDChar};
use libc::{dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};

/// Represents an owned handle to a dynamically loaded library.
#[repr(C)]
pub struct NSTDUnixSharedLib {
    /// A raw handle to the shared library.
    handle: NSTDAnyMut,
}
impl Drop for NSTDUnixSharedLib {
    /// [NSTDUnixSharedLib]'s destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `self.handle` is never null.
        unsafe { dlclose(self.handle) };
    }
}

/// Represents an optional `NSTDUnixSharedLib`.
pub type NSTDUnixOptionalSharedLib = NSTDOptional<NSTDUnixSharedLib>;

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_unix_shared_lib_load(
    path: *const NSTDChar,
) -> NSTDUnixOptionalSharedLib {
    let handle = dlopen(path, RTLD_LAZY | RTLD_LOCAL);
    if handle.is_null() {
        return NSTDOptional::None;
    }
    NSTDOptional::Some(NSTDUnixSharedLib { handle })
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_unix_shared_lib_get(
    lib: &NSTDUnixSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAny {
    dlsym(lib.handle, symbol)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_unix_shared_lib_get_mut(
    lib: &mut NSTDUnixSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAnyMut {
    dlsym(lib.handle, symbol)
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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub unsafe extern "C" fn nstd_os_unix_shared_lib_free(lib: NSTDUnixSharedLib) {}
