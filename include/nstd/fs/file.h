#ifndef NSTD_FS_FILE_H
#define NSTD_FS_FILE_H
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
#define NSTD_FILE_CREATE 0b00000001

/// Open a file in read mode.
#define NSTD_FILE_READ 0b00000010

/// Open a file in write mode.
#define NSTD_FILE_WRITE 0b00000100

/// Open a file in writing mode without overwriting saved data.
#define NSTD_FILE_APPEND 0b00001000

/// Open a file in truncate mode, this will set the file's length to 0 upon opening.
#define NSTD_FILE_TRUNC 0b00010000

/// A handle to an opened file.
typedef NSTDAnyMut NSTDFile;

/// Opens file on the filesystem and returns a handle to it.
///
/// # Parameters:
///
/// - `const NSTDStr *name` - The name of the file to create.
///
/// - `NSTDUInt8 mask` - A bit mask for toggling the file's different open options.
///
/// - `NSTDIOError *errc` - Returns as the I/O operation error code.
///
/// # Returns
///
/// `NSTDFile file` - A handle to the opened file, or null if an error occurs.
///
/// # Panics
///
/// Panics if `name`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// # Safety
///
/// This operation can cause undefined behavior if `name`'s data is invalid.
NSTDAPI NSTDFile nstd_fs_file_open(const NSTDStr *name, NSTDUInt8 mask, NSTDIOError *errc);

/// Writes some data to a file & returns how many bytes were written.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to an open file.
///
/// - `const NSTDSlice *bytes` - The data to write to the file.
///
/// - `NSTDUInt *written` - Returns as the number of bytes written to the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// This function's caller must guarantee validity of `bytes`.
NSTDAPI NSTDIOError nstd_fs_file_write(NSTDFile *file, const NSTDSlice *bytes, NSTDUInt *written);

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
/// - `NSTDUInt *read` - Returns as the number of bytes read from the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
NSTDAPI NSTDIOError nstd_fs_file_read(NSTDFile *file, NSTDSliceMut *buffer, NSTDUInt *read);

/// Continuously reads data from `file` into a buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean `read` will return as 0 in this case.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to the file.
///
/// - `NSTDVec *buffer` - The buffer to be extended with data from the file.
///
/// - `NSTDUInt *read` - Returns as the number of bytes read from the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// Panics if getting a handle to the heap fails.
NSTDAPI NSTDIOError nstd_fs_file_read_all(NSTDFile *file, NSTDVec *buffer, NSTDUInt *read);

/// Continuously reads UTF-8 data from `file` into a string buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean `read` will return as 0 in this case.
///
/// # Parameters:
///
/// - `NSTDFile *file` - A handle to the file.
///
/// - `NSTDString *buffer` - The buffer to be extended with data from the file.
///
/// - `NSTDUInt *read` - Returns as the number of bytes read from the file.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Panics
///
/// This function will panic in the following situations:
///
/// - `buffer`'s length in bytes exceeds `NSTDInt`'s max value.
///
/// - Getting a handle to the heap fails.
NSTDAPI NSTDIOError nstd_fs_file_read_to_string(NSTDFile *file, NSTDString *buffer, NSTDUInt *read);

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
