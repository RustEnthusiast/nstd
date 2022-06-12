#ifndef NSTD_CORE_DEF_H_INCLUDED
#define NSTD_CORE_DEF_H_INCLUDED
#include "../nstd.h"

/// Equivalent to C's `char` type.
typedef char NSTDChar;

/// The smallest addressable unit of memory.
typedef NSTDUInt8 NSTDByte;

/// An error code type to be returned from functions. An error code of 0 means success, while
/// anything else indicates failure.
typedef NSTDInt32 NSTDErrorCode;

#endif
