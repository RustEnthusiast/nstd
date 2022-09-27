#ifndef NSTD_CORE_DEF_H
#define NSTD_CORE_DEF_H
#include "../nstd.h"

/// The smallest addressable unit of memory.
typedef NSTDUInt8 NSTDByte;

/// An error code type to be returned from functions. An error code of 0 means success, while
/// anything else indicates failure.
typedef NSTDInt32 NSTDErrorCode;

#endif
