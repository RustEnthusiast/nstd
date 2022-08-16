#ifndef NSTD_FS_FS_H
#define NSTD_FS_FS_H
#include "../core/str.h"
#include "../io/io.h"
#include "../nstd.h"
NSTDCPPSTART

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
NSTDAPI NSTDIOError nstd_fs_create_file(const NSTDStrConst *name);

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
NSTDAPI NSTDIOError nstd_fs_create_dir(const NSTDStrConst *name);

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
NSTDAPI NSTDIOError nstd_fs_create_dirs(const NSTDStrConst *name);

NSTDCPPEND
#endif
