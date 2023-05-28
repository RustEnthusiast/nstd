#ifndef NSTD_FS_FS_H
#define NSTD_FS_FS_H
#include "../core/result.h"
#include "../core/slice.h"
#include "../core/str.h"
#include "../io/io.h"
#include "../nstd.h"
#include "../time.h"

/// A bit flag describing a file with read access.
#define NSTD_FILE_PERMISSION_READ 1

/// Describes the type of a file.
typedef enum {
    /// An unknown file type.
    NSTD_FILE_TYPE_UNKNOWN,
    /// A normal text/binary file.
    NSTD_FILE_TYPE_REGULAR,
    /// A directory/folder.
    NSTD_FILE_TYPE_DIRECTORY,
    /// A symbolic link.
    NSTD_FILE_TYPE_SYMLINK
} NSTDFileType;

/// Represents file metadata.
typedef struct {
    /// The size of the file in bytes.
    NSTDUInt64 size;
    /// The time that the file was created.
    NSTDOptionalTime created;
    /// The time that the file was last accessed.
    NSTDOptionalTime accessed;
    /// The time that the file was last modified.
    NSTDOptionalTime modified;
    /// The file type.
    NSTDFileType file_type;
    /// A bit mask representing the file's permissions.
    NSTDUInt8 permissions;
} NSTDFileMetadata;

/// A result type returned from `nstd_fs_metadata`.
NSTDResult(NSTDFileMetadata, NSTDIOError) NSTDFileMetadataResult;

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
NSTDAPI NSTDIOError nstd_fs_create_file(const NSTDStr *name);

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
NSTDAPI NSTDIOError nstd_fs_create_dir(const NSTDStr *name);

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
NSTDAPI NSTDIOError nstd_fs_create_dirs(const NSTDStr *name);

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
NSTDAPI NSTDIOError nstd_fs_remove_file(const NSTDStr *name);

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
NSTDAPI NSTDIOError nstd_fs_remove_dir(const NSTDStr *name);

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
NSTDAPI NSTDIOError nstd_fs_remove_dirs(const NSTDStr *name);

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
NSTDAPI NSTDIOBufferResult nstd_fs_read(const NSTDStr *path);

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
NSTDAPI NSTDIOStringResult nstd_fs_read_to_string(const NSTDStr *path);

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
/// This operation will panic if `content`'s stride is not 1.
///
/// # Safety
///
/// This operation can cause undefined behavior if either `path` or `content`'s data is invalid.
NSTDAPI NSTDIOError nstd_fs_write(const NSTDStr *path, const NSTDSlice *content);

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
NSTDAPI NSTDIOError nstd_fs_rename(const NSTDStr *from, const NSTDStr *to);

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
NSTDAPI NSTDIOError nstd_fs_copy(const NSTDStr *from, const NSTDStr *to);

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
NSTDAPI NSTDIOStringResult nstd_fs_absolute(const NSTDStr *path);

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
NSTDAPI NSTDFileMetadataResult nstd_fs_metadata(const NSTDStr *path);

#endif
