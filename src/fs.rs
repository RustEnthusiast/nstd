//! Provides access to the file system.
pub mod file;
use crate::{
    alloc::NSTDAllocError,
    core::{
        slice::{nstd_core_slice_new, NSTDSlice},
        str::{nstd_core_str_from_bytes_unchecked, NSTDStr},
    },
    io::NSTDIOError,
    string::{nstd_string_new, nstd_string_push_str, NSTDString},
    vec::{nstd_vec_extend, NSTDVec},
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
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
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
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
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
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
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
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
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
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
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

/// Recursively removes directories on the file system.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - A path to the directory to remove.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
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

/// Extends a vector with the contents of a file.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to read.
///
/// - `NSTDVec *buffer` - The buffer to extend.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `path`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// - `buffer`'s length exceeds `NSTDInt`'s max value.
///
/// - `buffer`'s stride is not 1.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_read(path: &NSTDStr, buffer: &mut NSTDVec) -> NSTDIOError {
    match std::fs::read(path.as_str()) {
        Ok(contents) => {
            let contents = nstd_core_slice_new(contents.as_ptr().cast(), 1, contents.len());
            match nstd_vec_extend(buffer, &contents) {
                NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDIOError::NSTD_IO_ERROR_NONE,
                _ => NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY,
            }
        }
        Err(err) => NSTDIOError::from_err(err.kind()),
    }
}

/// Extends a string with the contents of a file.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to read.
///
/// - `NSTDString *buffer` - The buffer to extend.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `path`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// - `buffer`'s length exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_read_to_string(
    path: &NSTDStr,
    buffer: &mut NSTDString,
) -> NSTDIOError {
    match std::fs::read_to_string(path.as_str()) {
        Ok(contents) => {
            let bytes = nstd_core_slice_new(contents.as_ptr().cast(), 1, contents.len());
            let contents = nstd_core_str_from_bytes_unchecked(&bytes);
            match nstd_string_push_str(buffer, &contents) {
                NSTDAllocError::NSTD_ALLOC_ERROR_NONE => NSTDIOError::NSTD_IO_ERROR_NONE,
                _ => NSTDIOError::NSTD_IO_ERROR_OUT_OF_MEMORY,
            }
        }
        Err(err) => NSTDIOError::from_err(err.kind()),
    }
}

/// Overwrites the contents of a file.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to write to.
///
/// - `const NSTDSlice *content` - The new content to write to the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `path`'s length in bytes is greater than `NSTDInt`'s max value.
///
/// - `content`'s stride is not 1.
///
/// - `content`'s length is greater than `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation can cause undefined behavior if either `path` or `content`'s data is invalid.
#[inline]
#[cfg_attr(feature = "clib", no_mangle)]
pub unsafe extern "C" fn nstd_fs_write(path: &NSTDStr, content: &NSTDSlice) -> NSTDIOError {
    if let Err(err) = std::fs::write(path.as_str(), content.as_slice()) {
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
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `from`'s length in bytes is greater than `NSTDInt`'s max value.
///
/// - `to`'s length in bytes is greater than `NSTDInt`'s max value.
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
/// # Panics
///
/// This operation will panic in the following situations:
///
/// - `from`'s length in bytes is greater than `NSTDInt`'s max value.
///
/// - `to`'s length in bytes is greater than `NSTDInt`'s max value.
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
/// # Panics
///
/// Panics if `path`'s length in bytes exceeds `NSTDInt`'s max value or allocating fails.
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
