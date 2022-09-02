#ifndef NSTD_IMAGE_H
#define NSTD_IMAGE_H
#include "core/slice.h"
#include "io/io.h"
#include "nstd.h"

/// An image of any format.
typedef NSTDAnyMut NSTDImage;

/// An enumeration that describes each type of `nstd.image` error.
typedef enum {
    /// An unknown image operation error.
    NSTD_IMAGE_ERROR_TYPE_UNKNOWN,
    /// An I/O operation failed.
    NSTD_IMAGE_ERROR_TYPE_IO,
} NSTDImageErrorType;

/// A union of each possible error from `nstd.image`.
typedef union {
    /// I/O operation error data.
    NSTDIOError io_data;
} NSTDImageErrorData;

/// An error returned from an `nstd.image` function.
typedef struct {
    /// Indicates the type of error that has occurred.
    NSTDImageErrorType errc;
    /// A union of each possible error.
    NSTDImageErrorData err;
} NSTDImageError;

/// Loads an image from an in-memory buffer.
///
/// # Parameters:
///
/// - `const NSTDSlice *buffer` - The image buffer.
///
/// - `NSTDImageError *errc` - Returns as the image operation's error code.
///
/// # Returns
///
/// `NSTDImage img` - The new image, or null on error.
///
/// # Panics
///
/// This operation will panic if `buffer`'s stride is not 1.
///
/// # Safety
///
/// This operation can cause undefined behavior if `buffer`'s data is invalid.
NSTDAPI NSTDImage nstd_image_load(const NSTDSlice *buffer, NSTDImageError *errc);

/// Returns an image's raw pixel data as a byte slice.
///
/// # Parameters:
///
/// - `const NSTDImage *img` - The image.
///
/// # Returns
///
/// `NSTDSlice bytes` - The image's raw pixel data.
NSTDAPI NSTDSlice nstd_image_as_bytes(const NSTDImage *img);

/// Returns the width of an image.
///
/// # Parameters:
///
/// - `const NSTDImage *img` - The image.
///
/// # Returns
///
/// `NSTDUInt32 width` - The width of the image.
NSTDAPI NSTDUInt32 nstd_image_width(const NSTDImage *img);

/// Returns the height of an image.
///
/// # Parameters:
///
/// - `const NSTDImage *img` - The image.
///
/// # Returns
///
/// `NSTDUInt32 height` - The height of the image.
NSTDAPI NSTDUInt32 nstd_image_height(const NSTDImage *img);

/// Frees image data.
///
/// # Parameters:
///
/// - `NSTDImage img` - The image.
NSTDAPI void nstd_image_free(NSTDImage img);

#endif
