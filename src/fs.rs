//! Provides access to the file system.
pub mod file;
use crate::{
    core::{optional::NSTDOptional, result::NSTDResult, slice::NSTDSlice, str::NSTDStr},
    io::{NSTDIOBufferResult, NSTDIOError, NSTDIOStringResult},
    string::NSTDString,
    time::{NSTDOptionalTime, NSTDTime},
    vec::NSTDVec,
    NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;
use std::fs::File;

/// A bit flag describing a file with read access.
pub const NSTD_FILE_PERMISSION_READ: NSTDUInt8 = 1;

/// Describes the type of a file.
#[nstdapi]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum NSTDFileType {
    /// An unknown file type.
    NSTD_FILE_TYPE_UNKNOWN,
    /// A normal text/binary file.
    NSTD_FILE_TYPE_REGULAR,
    /// A directory/folder.
    NSTD_FILE_TYPE_DIRECTORY,
    /// A symbolic link.
    NSTD_FILE_TYPE_SYMLINK,
}

/// Represents file metadata.
#[nstdapi]
#[derive(Clone, Copy)]
pub struct NSTDFileMetadata {
    /// The size of the file in bytes.
    pub size: NSTDUInt64,
    /// The time that the file was created.
    pub created: NSTDOptionalTime,
    /// The time that the file was last accessed.
    pub accessed: NSTDOptionalTime,
    /// The time that the file was last modified.
    pub modified: NSTDOptionalTime,
    /// The file type.
    pub file_type: NSTDFileType,
    /// A bit mask representing the file's permissions.
    pub permissions: NSTDUInt8,
}

/// A result type returned from `nstd_fs_metadata`.
pub type NSTDFileMetadataResult = NSTDResult<NSTDFileMetadata, NSTDIOError>;

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
#[nstdapi]
pub unsafe fn nstd_fs_create_file(name: &NSTDStr) -> NSTDIOError {
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
#[nstdapi]
pub unsafe fn nstd_fs_create_dir(name: &NSTDStr) -> NSTDIOError {
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
#[nstdapi]
pub unsafe fn nstd_fs_create_dirs(name: &NSTDStr) -> NSTDIOError {
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
#[nstdapi]
pub unsafe fn nstd_fs_remove_file(name: &NSTDStr) -> NSTDIOError {
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
#[nstdapi]
pub unsafe fn nstd_fs_remove_dir(name: &NSTDStr) -> NSTDIOError {
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
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
#[inline]
#[nstdapi]
pub unsafe fn nstd_fs_remove_dirs(name: &NSTDStr) -> NSTDIOError {
    if let Err(err) = std::fs::remove_dir_all(name.as_str()) {
        return NSTDIOError::from_err(err.kind());
    }
    NSTDIOError::NSTD_IO_ERROR_NONE
}

/// Reads the contents of a file.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to read.
///
/// # Returns
///
/// `NSTDIOBufferResult contents` - The file's contents, or the I/O operation error code on failure.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
#[nstdapi]
pub unsafe fn nstd_fs_read(path: &NSTDStr) -> NSTDIOBufferResult<'_> {
    match std::fs::read(path.as_str()) {
        Ok(contents) => NSTDResult::Ok(NSTDVec::from_vec(contents)),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Reads the contents of a file into a UTF-8 string.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to read.
///
/// # Returns
///
/// `NSTDIOStringResult contents` - The file's contents, or the I/O operation error code on failure.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
#[nstdapi]
pub unsafe fn nstd_fs_read_to_string(path: &NSTDStr) -> NSTDIOStringResult<'_> {
    match std::fs::read_to_string(path.as_str()) {
        Ok(contents) => NSTDResult::Ok(NSTDString::from_string(contents)),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
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
/// # Safety
///
/// This operation can cause undefined behavior if either `path` or `content`'s data is invalid.
#[nstdapi]
pub unsafe fn nstd_fs_write(path: &NSTDStr, content: &NSTDSlice) -> NSTDIOError {
    content.as_slice().map_or(
        NSTDIOError::NSTD_IO_ERROR_INVALID_INPUT,
        |bytes| match std::fs::write(path.as_str(), bytes) {
            Ok(()) => NSTDIOError::NSTD_IO_ERROR_NONE,
            Err(err) => NSTDIOError::from_err(err.kind()),
        },
    )
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
#[nstdapi]
pub unsafe fn nstd_fs_rename(from: &NSTDStr, to: &NSTDStr) -> NSTDIOError {
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
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This operation can cause undefined behavior if either `to` or `from`'s data is invalid.
#[inline]
#[nstdapi]
pub unsafe fn nstd_fs_copy(from: &NSTDStr, to: &NSTDStr) -> NSTDIOError {
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
/// # Returns
///
/// `NSTDIOStringResult contents` - The absolute version of `path`, or the I/O operation error code
/// on failure.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
#[nstdapi]
pub unsafe fn nstd_fs_absolute(path: &NSTDStr) -> NSTDIOStringResult<'_> {
    match std::fs::canonicalize(path.as_str()) {
        Ok(path) => path.into_os_string().into_string().map_or(
            NSTDResult::Err(NSTDIOError::NSTD_IO_ERROR_INVALID_DATA),
            |path| NSTDResult::Ok(NSTDString::from_string(path)),
        ),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}

/// Retrieves metadata about a file pointed to by `path`.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to retrieve metadata for.
///
/// # Returns
///
/// `NSTDFileMetadataResult metadata` - Metadata describing the file.
///
/// # Safety
///
/// `path` must be valid for reads.
#[nstdapi]
pub unsafe fn nstd_fs_metadata(path: &NSTDStr) -> NSTDFileMetadataResult {
    match std::fs::metadata(path.as_str()) {
        Ok(metadata) => NSTDResult::Ok(NSTDFileMetadata {
            size: metadata.len(),
            created: metadata.created().map_or(NSTDOptional::None, |t| {
                NSTDOptional::Some(NSTDTime::from(t))
            }),
            accessed: metadata.accessed().map_or(NSTDOptional::None, |t| {
                NSTDOptional::Some(NSTDTime::from(t))
            }),
            modified: metadata.modified().map_or(NSTDOptional::None, |t| {
                NSTDOptional::Some(NSTDTime::from(t))
            }),
            file_type: {
                if metadata.is_file() {
                    NSTDFileType::NSTD_FILE_TYPE_REGULAR
                } else if metadata.is_dir() {
                    NSTDFileType::NSTD_FILE_TYPE_DIRECTORY
                } else if metadata.is_symlink() {
                    NSTDFileType::NSTD_FILE_TYPE_SYMLINK
                } else {
                    NSTDFileType::NSTD_FILE_TYPE_UNKNOWN
                }
            },
            permissions: metadata.permissions().readonly().into(),
        }),
        Err(err) => NSTDResult::Err(NSTDIOError::from_err(err.kind())),
    }
}
