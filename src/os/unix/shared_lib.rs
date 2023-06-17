//! Provides shared library access for Unix like systems.
use crate::{core::optional::NSTDOptional, NSTDAny, NSTDAnyMut, NSTDChar, NSTD_NULL};
use libc::{dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
use nstdapi::nstdapi;

/// Represents an owned handle to a dynamically loaded library.
#[nstdapi]
pub struct NSTDUnixSharedLib {
    /// A raw handle to the shared library.
    handle: NSTDAnyMut,
}
impl Drop for NSTDUnixSharedLib {
    /// [NSTDUnixSharedLib]'s destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `self.handle` is valid.
        unsafe { dlclose(self.handle) };
    }
}
// SAFETY: `NSTDUnixSharedLib` owns a handle to the dynamically loaded library.
unsafe impl Send for NSTDUnixSharedLib {}
// SAFETY: `NSTDUnixSharedLib` does not undergo interior mutability.
unsafe impl Sync for NSTDUnixSharedLib {}

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
#[nstdapi]
pub unsafe fn nstd_os_unix_shared_lib_load(path: *const NSTDChar) -> NSTDUnixOptionalSharedLib {
    match dlopen(path, RTLD_LAZY | RTLD_LOCAL) {
        NSTD_NULL => NSTDOptional::None,
        handle => NSTDOptional::Some(NSTDUnixSharedLib { handle }),
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
#[inline]
#[nstdapi]
pub fn nstd_os_unix_shared_lib_handle(lib: &NSTDUnixSharedLib) -> NSTDAnyMut {
    lib.handle
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
#[nstdapi]
pub unsafe fn nstd_os_unix_shared_lib_get(
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
#[nstdapi]
pub unsafe fn nstd_os_unix_shared_lib_get_mut(
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
#[nstdapi]
#[allow(unused_variables)]
pub fn nstd_os_unix_shared_lib_free(lib: NSTDUnixSharedLib) {}
