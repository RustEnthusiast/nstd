#ifndef NSTD_FS_FS_H
#define NSTD_FS_FS_H
#include "../core/slice.h"
#include "../core/str.h"
#include "../io/io.h"
#include "../nstd.h"
#include "../string.h"
#include "../vec.h"

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

/// Reads the contents of a file into a vector of bytes.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to read.
///
/// - `NSTDIOError *errc` - Returns as the I/O operation's error code.
///
/// # Returns
///
/// `NSTDVec contents` - The contents of the file, or empty on error.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
NSTDAPI NSTDVec nstd_fs_read(const NSTDStr *path, NSTDIOError *errc);

/// Reads the contents of a file into a string.
///
/// # Parameters:
///
/// - `const NSTDStr *path` - A path to the file to read.
///
/// - `NSTDIOError *errc` - Returns as the I/O operation's error code.
///
/// # Returns
///
/// `NSTDString contents` - The contents of the file, or empty on error.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
NSTDAPI NSTDString nstd_fs_read_to_string(const NSTDStr *path, NSTDIOError *errc);

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
/// - `NSTDIOError *errc` - Returns as the I/O operation's error code.
///
/// # Returns
///
/// `NSTDString abs_path` - The absolute path of `path`.
///
/// # Safety
///
/// This operation can cause undefined behavior if `path`'s data is invalid.
NSTDAPI NSTDString nstd_fs_absolute(const NSTDStr *path, NSTDIOError *errc);

#endif
