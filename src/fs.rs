//! Provides access to the file system.
pub mod file;
use crate::{core::str::NSTDStrConst, io::NSTDIOError};
use std::fs::File;

/// Creates a new file on the file system.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - The name of the new file.
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
pub unsafe extern "C" fn nstd_fs_create_file(name: &NSTDStrConst) -> NSTDIOError {
    if let Err(err) = File::create(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Creates a new directory on the file system.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - The name of the new directory.
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
pub unsafe extern "C" fn nstd_fs_create_dir(name: &NSTDStrConst) -> NSTDIOError {
    if let Err(err) = std::fs::create_dir(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Recursively creates new directories on the file system.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - A path to the new directory.
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
pub unsafe extern "C" fn nstd_fs_create_dirs(name: &NSTDStrConst) -> NSTDIOError {
    if let Err(err) = std::fs::create_dir_all(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Removes a file from the file system.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - The name of the file to delete.
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
pub unsafe extern "C" fn nstd_fs_remove_file(name: &NSTDStrConst) -> NSTDIOError {
    if let Err(err) = std::fs::remove_file(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Removes a directory from the file system.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - The name of the directory to delete.
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
pub unsafe extern "C" fn nstd_fs_remove_dir(name: &NSTDStrConst) -> NSTDIOError {
    if let Err(err) = std::fs::remove_dir(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Recursively removes a directory on the file system.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - A path to the directory to remove.
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
pub unsafe extern "C" fn nstd_fs_remove_dirs(name: &NSTDStrConst) -> NSTDIOError {
    if let Err(err) = std::fs::remove_dir_all(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}
