#ifndef NSTD_FS_FILE_H
#define NSTD_FS_FILE_H
#include "../core/slice.h"
#include "../core/str.h"
#include "../io/io.h"
#include "../nstd.h"
NSTDCPPSTART

/// Creates the file upon opening if it does not already exist.
///
/// Either of the `NSTD_FILE_WRITE` or `NSTD_FILE_APPEND` options must also be toggled for the file
/// to be created.
#define NSTD_FILE_CREATE 0b00000001

/// Open a file in read mode.
#define NSTD_FILE_READ 0b00000010

/// Open a file in write mode.
#define NSTD_FILE_WRITE 0b00000100

/// Open a file in writing mode without overwriting saved data.
#define NSTD_FILE_APPEND 0b00001000

/// Open a file in truncate mode, this will set the file's length to 0 upon openning.
#define NSTD_FILE_TRUNC 0b00010000

/// A handle to an opened file.
typedef NSTDAnyMut NSTDFile;

/// Opens file on the filesystem and returns a handle to it.
///
/// # Parameters:
///
/// - `const NSTDStrConst *name` - The name of the file to create.
///
/// - `NSTDUInt8 mask` - A bit mask for toggling the file's different open options.
///
/// - `NSTDIOError *errc` - Returns as the I/O operation error code.
///
/// # Returns
///
/// `NSTDFile file` - A handle to the opened file, or null if an error occurrs.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
NSTDAPI NSTDFile nstd_fs_file_open(const NSTDStrConst *name, NSTDUInt8 mask, NSTDIOError *errc);

/// Writes some data to a file & returns how many bytes were written.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to an open file.
///
/// - `const NSTDSliceConst *bytes` - The data to write to the file.
///
/// - `NSTDUSize *written` - Returns as the number of bytes written to the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function's caller must guarantee validity of `bytes`.
NSTDAPI NSTDIOError nstd_fs_file_write(NSTDFile *file, const NSTDSliceConst *bytes,
NSTDUSize *written);

/// Writes a whole buffer to a file.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to an open file.
///
/// - `const NSTDSliceConst *bytes` - The data to write to the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function's caller must guarantee validity of `bytes`.
NSTDAPI NSTDIOError nstd_fs_file_write_all(NSTDFile *file, const NSTDSliceConst *bytes);

/// Closes a file handle.
///
/// # Parameters:
///
/// - `NSTDFile file` - The file handle to close.
NSTDAPI void nstd_fs_file_close(NSTDFile file);

NSTDCPPEND
#endif
