#ifndef NSTD_IMAGE_H
#define NSTD_IMAGE_H
#include "core/slice.h"
#include "nstd.h"

/// An image of any format.
typedef NSTDAnyMut NSTDImage;

/// Loads an image from an in-memory buffer.
///
/// # Parameters:
///
/// - `const NSTDSlice *buffer` - The image buffer.
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
NSTDAPI NSTDImage nstd_image_load(const NSTDSlice *buffer);

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
