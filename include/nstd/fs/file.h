#ifndef NSTD_FS_FILE_H
#define NSTD_FS_FILE_H
#include "../core/result.h"
#include "../core/slice.h"
#include "../core/str.h"
#include "../io/io.h"
#include "../nstd.h"
#include "../string.h"
#include "../vec.h"

/// Creates the file upon opening if it does not already exist.
///
/// Either of the `NSTD_FILE_WRITE` or `NSTD_FILE_APPEND` options must also be toggled for the file
/// to be created.
#define NSTD_FILE_CREATE 1

/// Open a file in read mode.
#define NSTD_FILE_READ (1 << 1)

/// Open a file in write mode.
#define NSTD_FILE_WRITE (1 << 2)

/// Open a file in writing mode without overwriting saved data.
#define NSTD_FILE_APPEND (1 << 3)

/// Open a file in truncate mode, this will set the file's length to 0 upon opening.
#define NSTD_FILE_TRUNC (1 << 4)

/// A handle to an opened file.
typedef struct {
    /// The inner [File].
    NSTDAnyMut f;
} NSTDFile;

/// A result type yielding an `NSTDFile` on success.
NSTDResult(NSTDFile, NSTDIOError) NSTDFileResult;

/// Opens file on the filesystem and returns a handle to it.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - The name of the file to create.
///
/// - `NSTDUInt8 mask` - A bit mask for toggling the file's different open options.
///
/// # Returns
///
/// `NSTDFileResult file` - A handle to the opened file, or the IO error on failure.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
NSTDAPI NSTDFileResult nstd_fs_file_open(const NSTDStr *name, NSTDUInt8 mask);

/// Writes some data to a file & returns how many bytes were written.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to an open file.
///
/// - `const NSTDSlice *bytes` - The data to write to the file.
///
/// # Returns
///
/// `NSTDIOResult written` - The number of bytes written to `handle` on success, or the I/O
/// operation error code on failure.
///
/// # Safety
///
/// This function's caller must guarantee validity of `bytes`.
NSTDAPI NSTDIOResult nstd_fs_file_write(NSTDFile *file, const NSTDSlice *bytes);

/// Writes a whole buffer to a file.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to an open file.
///
/// - `const NSTDSlice *bytes` - The data to write to the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function's caller must guarantee validity of `bytes`.
NSTDAPI NSTDIOError nstd_fs_file_write_all(NSTDFile *file, const NSTDSlice *bytes);

/// Flushes a file stream.
///
/// # Parameters:
///
/// - `NSTDFile *file` - The file stream.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_fs_file_flush(NSTDFile *file);

/// Reads some data from an open file into a buffer.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to the opened file.
///
/// - `NSTDSliceMut *buffer` - The buffer to start filling with data from the file.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
NSTDAPI NSTDIOResult nstd_fs_file_read(NSTDFile *file, NSTDSliceMut *buffer);

/// Continuously reads data from `file` into a buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `file` in this case.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to the file.
///
/// - `NSTDVec *buffer` - The buffer to be extended with data from the file.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
NSTDAPI NSTDIOResult nstd_fs_file_read_all(NSTDFile *file, NSTDVec *buffer);

/// Continuously reads UTF-8 data from `file` into a string buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean there were no bytes read from `file` in this case.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to the file.
///
/// - `NSTDString *buffer` - The buffer to be extended with data from the file.
///
/// # Returns
///
/// `NSTDIOResult read` - The number of bytes read from `handle` on success, or the I/O operation
/// error code on failure.
NSTDAPI NSTDIOResult nstd_fs_file_read_to_string(NSTDFile *file, NSTDString *buffer);

/// Reads enough data from `file` to fill the entirety of `buffer`.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to the file.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer` must be valid for writes.
NSTDAPI NSTDIOError nstd_fs_file_read_exact(NSTDFile *file, NSTDSliceMut *buffer);

/// Closes a file handle.
///
/// # Parameters:
///
/// - `NSTDFile file` - The file handle to close.
NSTDAPI void nstd_fs_file_close(NSTDFile file);

#endif
