//! Shared library/module access for Windows.
use crate::{
    core::optional::NSTDOptional, os::windows::NSTDWindowsHandle, NSTDAny, NSTDAnyMut, NSTDChar,
    NSTDChar16,
};
use windows_sys::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryW};

/// A handle to a loaded library.
#[repr(C)]
pub struct NSTDWindowsSharedLib {
    /// A raw handle to the module.
    handle: NSTDWindowsHandle,
}
impl Drop for NSTDWindowsSharedLib {
    /// [NSTDWindowsSharedLib]'s destructor.
    ///
    /// # Panics
    ///
    /// Panics if unloading the library fails.
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `handle` is non-null.
        unsafe { assert!(FreeLibrary(self.handle) != 0) };
    }
}

/// An optional (possibly null) shared Windows library handle.
pub type NSTDWindowsOptionalSharedLib = NSTDOptional<NSTDWindowsSharedLib>;

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
#[inline]
#[cfg_attr(feature = "capi", no_mangle)]
pub unsafe extern "C" fn nstd_os_windows_shared_lib_load(
    name: *const NSTDChar16,
) -> NSTDWindowsOptionalSharedLib {
    match LoadLibraryW(name) {
        0 => NSTDOptional::None,
        handle => NSTDOptional::Some(NSTDWindowsSharedLib { handle }),
    }
}

/// Returns a raw handle to a dynamically loaded library.
///
/// # Parameters:
///
/// - `const NSTDWindowsSharedLib *lib` - The loaded library.
///
/// # Returns
///
/// `NSTDWindowsHandle handle` - A native handle to the dynamically loaded library.
#[inline]
#[cfg_attr(feature = "capi", no_mangle)]
pub extern "C" fn nstd_os_windows_shared_lib_handle(
    lib: &NSTDWindowsSharedLib,
) -> NSTDWindowsHandle {
    lib.handle
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
#[cfg_attr(feature = "capi", no_mangle)]
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
#[cfg_attr(feature = "capi", no_mangle)]
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
/// # Panics
///
/// Panics if unloading the library fails.
///
/// # Safety
///
/// See
/// <https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary>.
#[inline]
#[cfg_attr(feature = "capi", no_mangle)]
#[allow(unused_variables)]
pub unsafe extern "C" fn nstd_os_windows_shared_lib_free(lib: NSTDWindowsSharedLib) {}
