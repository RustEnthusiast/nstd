//! Provides access to the file system.
pub mod file;
use crate::{
    core::str::NSTDStr,
    io::NSTDIOError,
    string::{nstd_string_new, NSTDString},
};
use std::fs::File;

/// Creates a new file on the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - The name of the new file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_file(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = File::create(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Creates a new directory on the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - The name of the new directory.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_dir(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::create_dir(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Recursively creates new directories on the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - A path to the new directory.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_create_dirs(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::create_dir_all(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Removes a file from the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - The name of the file to delete.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_file(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::remove_file(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Removes a directory from the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - The name of the directory to delete.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_dir(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::remove_dir(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Recursively removes a directory on the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - A path to the directory to remove.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_remove_dirs(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::remove_dir_all(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Renames a file or directory, replacing the destination if it already exists.
///
/// # Parameters:
///
/// - `const NSTDStr *from` - The original name of the file/directory.
///
/// - `const NSTDStr *to` - The new name of the file/dir.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if either `to` or `from`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_rename(from: &NSTDStr, to: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::rename(from.as_str(), to.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Copies the contents and permissions of one file to another.
///
/// # Parameters:
///
/// - `const NSTDStr *from` - The source file.
///
/// - `const NSTDStr *to` - The destination file.
///
/// # Safety
///
/// This operation can cause undefined behavior if either `to` or `from`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_copy(from: &NSTDStr, to: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::copy(from.as_str(), to.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Returns the absolute path of a file system item.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A relative path to the file system item.
///
/// - `NSTDIOError *errc` - Returns as the I/O operation's error code.
///
/// # Returns
///
/// `NSTDString abs_path` - The absolute path of `path`.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_absolute(path: &NSTDStr, errc: &mut NSTDIOError) -> NSTDString {
    match std::fs::canonicalize(path.as_str()) {
        Ok(path) => match path.into_os_string().into_string() {
            Ok(path) => {
                *errc = NSTDIOError::NSTD_IO_ERROR_NONE;
                return NSTDString::from_str(&path);
            }
            _ => *errc = NSTDIOError::NSTD_IO_ERROR_INVALID_DATA,
        },
        Err(err) => *errc = NSTDIOError::from_err(err.kind()),
    }
    nstd_string_new()
}
