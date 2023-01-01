#ifndef NSTD_CORE_UNICHAR_H
#define NSTD_CORE_UNICHAR_H
#include "../nstd.h"
#include "optional.h"

/// Represents a unicode scalar value.
typedef struct {
    /// The 32-bit value.
    NSTDChar32 value;
} NSTDUnichar;


/// Represents an optional value of type `NSTDUnichar`.
NSTDOptional(NSTDUnichar) NSTDOptionalUnichar;

/// Creates a new `NSTDUnichar` from a 32-bit character value.
///
/// # Parameters:
///
/// - `NSTDChar32 value` - The 32-bit character to be converted into an `NSTDUnichar`.
///
/// # Returns
///
/// `NSTDOptionalUnichar unichar` - The new Unicode scalar value on success.
NSTDAPI NSTDOptionalUnichar nstd_core_unichar_new(NSTDChar32 value);

#endif
