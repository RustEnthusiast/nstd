#ifndef NSTD_IO_STDIN_H_INCLUDED
#define NSTD_IO_STDIN_H_INCLUDED
#include "../core/slice.h"
#include "../nstd.h"
#include "../string.h"
#include "../vec.h"
#include "io.h"
NSTDCPPSTART

/// A handle to the standard input stream.
typedef NSTDAnyMut NSTDStdin;

/// Constructs a new handle to the standard input stream.
///
/// # Returns
///
/// `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI NSTDStdin nstd_io_stdin();

/// Reads some data from stdin into a byte slice buffer.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// - `NSTDUSize *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer`'s data must be valid for writes.
NSTDAPI NSTDIOError nstd_io_stdin_read(NSTDStdin *handle, NSTDSliceMut *buffer, NSTDUSize *read);

/// Continuously reads data from stdin into a buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean `read` will return as 0 in this case.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDVec *buffer` - The buffer to be extended with data from stdin.
///
/// - `NSTDUSize *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stdin_read_all(NSTDStdin *handle, NSTDVec *buffer, NSTDUSize *read);

/// Continuously reads UTF-8 data from stdin into a string buffer until EOF is reached.
///
/// # Note
///
/// If extending the buffer fails, an error code of `NSTD_IO_ERROR_OUT_OF_MEMORY` will be returned.
/// This does not mean `read` will return as 0 in this case.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDString *buffer` - The buffer to be extended with data from stdin.
///
/// - `NSTDUSize *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stdin_read_to_string(NSTDStdin *handle, NSTDString *buffer,
NSTDUSize *read);

/// Reads enough data from stdin to fill the entirety of `buffer`.
///
/// # Note
///
/// This function will return an error code of `NSTD_IO_ERROR_INVALID_INPUT` if the buffer's
/// element size is not 1.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to the standard input stream.
///
/// - `NSTDSliceMut *buffer` - The buffer to fill with data from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
///
/// # Safety
///
/// `buffer` must be valid for writes.
NSTDAPI NSTDIOError nstd_io_stdin_read_exact(NSTDStdin *handle, NSTDSliceMut *buffer);

/// Reads a line from stdin and appends it to `buffer`.
///
/// # Parameters:
///
/// - `NSTDStdin *handle` - A handle to stdin.
///
/// - `NSTDString *buffer` - The string buffer to extend with a line from stdin.
///
/// - `NSTDUSize *read` - Returns as the number of bytes read from stdin.
///
/// # Returns
///
/// `NSTDIOError errc` - The I/O operation error code.
NSTDAPI NSTDIOError nstd_io_stdin_read_line(NSTDStdin *handle, NSTDString *buffer,
NSTDUSize *read);

/// Frees an instance of `NSTDStdin`.
///
/// # Parameters:
///
/// - `NSTDStdin handle` - A handle to the standard input stream.
NSTDAPI void nstd_io_stdin_free(NSTDStdin handle);

NSTDCPPEND
#endif