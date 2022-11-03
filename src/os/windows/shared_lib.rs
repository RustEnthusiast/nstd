//! Shared library/module access for Windows.
use crate::{core::optional::NSTDOptional, NSTDAny, NSTDAnyMut, NSTDChar, NSTDInt};
use windows_sys::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA};

/// A handle to a loaded library.
#[repr(C)]
pub struct NSTDWindowsSharedLib {
    /// A raw handle to the module.
    handle: NSTDInt,
}
impl Drop for NSTDWindowsSharedLib {
    /// [NSTDWindowsSharedLib]'s destructor.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `handle` is non-null.
        unsafe { FreeLibrary(self.handle) };
    }
}

/// An optional (possibly null) shared Windows library handle.
pub type NSTDWindowsOptionalSharedLib = NSTDOptional<NSTDWindowsSharedLib>;

/// Loads a shared library/module by name.
///
/// # Parameters:
///
/// - `const NSTDChar *name` - The name of the module to load.
///
/// # Returns
///
/// `NSTDWindowsOptionalSharedLib lib` - A handle to the shared library.
///
/// # Safety
///
/// See
/// <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_shared_lib_load(
    name: *const NSTDChar,
) -> NSTDWindowsOptionalSharedLib {
    match LoadLibraryA(name.cast()) {
        0 => NSTDOptional::None,
        handle => NSTDOptional::Some(NSTDWindowsSharedLib { handle }),
    }
}

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
/// See <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_shared_lib_get(
    lib: &NSTDWindowsSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAny {
    core::mem::transmute(GetProcAddress(lib.handle, symbol.cast()))
}

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
/// See <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_shared_lib_get_mut(
    lib: &mut NSTDWindowsSharedLib,
    symbol: *const NSTDChar,
) -> NSTDAnyMut {
    core::mem::transmute(GetProcAddress(lib.handle, symbol.cast()))
}

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
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
#[allow(unused_variables)]
pub unsafe extern "C" fn nstd_os_windows_shared_lib_free(lib: NSTDWindowsSharedLib) {}
