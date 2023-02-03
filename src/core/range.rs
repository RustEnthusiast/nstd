//! A numerical range.
use crate::{
    NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt,
    NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;

/// Generates the `NSTD*Range*` structs.
macro_rules! gen_range_struct {
    (
        $(#[$meta:meta])*
        $name: ident, $T: ty
    ) => {
        $(#[$meta])*
        #[nstdapi]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name {
            /// The lower bound of the range.
            pub start: $T,
            /// The higher bound of the range.
            pub end: $T,
        }
    };
}
gen_range_struct!(
    /// A 32-bit floating point numerical range.
    NSTDRangeF32,
    NSTDFloat32
);
gen_range_struct!(
    /// A 64-bit floating point numerical range.
    NSTDRangeF64,
    NSTDFloat64
);
gen_range_struct!(
    /// An arch-bit signed numerical range.
    NSTDRange,
    NSTDInt
);
gen_range_struct!(
    /// An arch-bit unsigned numerical range.
    NSTDURange,
    NSTDUInt
);
gen_range_struct!(
    /// An 8-bit signed numerical range.
    NSTDRangeI8,
    NSTDInt8
);
gen_range_struct!(
    /// An 8-bit unsigned numerical range.
    NSTDRangeU8,
    NSTDUInt8
);
gen_range_struct!(
    /// A 16-bit signed numerical range.
    NSTDRangeI16,
    NSTDInt16
);
gen_range_struct!(
    /// A 16-bit unsigned numerical range.
    NSTDRangeU16,
    NSTDUInt16
);
gen_range_struct!(
    /// A 32-bit signed numerical range.
    NSTDRangeI32,
    NSTDInt32
);
gen_range_struct!(
    /// A 32-bit unsigned numerical range.
    NSTDRangeU32,
    NSTDUInt32
);
gen_range_struct!(
    /// A 64-bit signed numerical range.
    NSTDRangeI64,
    NSTDInt64
);
gen_range_struct!(
    /// A 64-bit unsigned numerical range.
    NSTDRangeU64,
    NSTDUInt64
);
