//! A half-open (low inclusive, high exclusive) numerical range.
use crate::{
    NSTDFloat32, NSTDFloat64, NSTDISize, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt16,
    NSTDUInt32, NSTDUInt64, NSTDUInt8, NSTDUSize,
};

/// Generates the `NSTD*Range*` structs.
macro_rules! gen_range_struct {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        $(#[$meta])*
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $name {
            /// The lower bound of the range (inclusive).
            pub start: $T,
            /// The higher bound of the range (exclusive).
            pub end: $T,
        }
    };
}
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) arch-bit unsigned numerical range.
    NSTDURange,
    NSTDUSize
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) arch-bit signed numerical range.
    NSTDIRange,
    NSTDISize
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 8-bit unsigned numerical range.
    NSTDURange8,
    NSTDUInt8
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 8-bit signed numerical range.
    NSTDIRange8,
    NSTDInt8
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 16-bit unsigned numerical range.
    NSTDURange16,
    NSTDUInt16
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 16-bit signed numerical range.
    NSTDIRange16,
    NSTDInt16
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 32-bit unsigned numerical range.
    NSTDURange32,
    NSTDUInt32
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 32-bit signed numerical range.
    NSTDIRange32,
    NSTDInt32
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 64-bit unsigned numerical range.
    NSTDURange64,
    NSTDUInt64
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 64-bit signed numerical range.
    NSTDIRange64,
    NSTDInt64
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 32-bit floating point numerical range.
    NSTDFRange32,
    NSTDFloat32
);
gen_range_struct!(
    /// A half-open (low inclusive, high exclusive) 64-bit floating point numerical range.
    NSTDFRange64,
    NSTDFloat64
);
