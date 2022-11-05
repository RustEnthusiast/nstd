//! Represents an optional (possibly uninitialized) value.
use crate::{
    NSTDBool, NSTDChar, NSTDChar16, NSTDChar32, NSTDChar8, NSTDFloat32, NSTDFloat64, NSTDInt,
    NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8, NSTDUInt, NSTDUInt16, NSTDUInt32, NSTDUInt64,
    NSTDUInt8, NSTDUnichar,
};

/// Describes an `NSTDOptional` variant.
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum NSTDOptionalStatus {
    /// No value.
    NSTD_OPTIONAL_STATUS_NONE,
    /// "Some" initialized value.
    NSTD_OPTIONAL_STATUS_SOME,
}

/// Represents an optional (possibly uninitialized) value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDOptional<T> {
    /// The uninitialized variant.
    #[default]
    None,
    /// The initialized variant.
    Some(T),
}

/// Generates optional primitive structures.
macro_rules! gen_optional {
    ($name: ident, $T: ty) => {
        #[doc = concat!("Represents an optional value of type `", stringify!($T), "`.")]
        pub type $name = NSTDOptional<$T>;
    };
}
gen_optional!(NSTDOptionalBool, NSTDBool);
gen_optional!(NSTDOptionalChar, NSTDChar);
gen_optional!(NSTDOptionalChar8, NSTDChar8);
gen_optional!(NSTDOptionalChar16, NSTDChar16);
gen_optional!(NSTDOptionalChar32, NSTDChar32);
gen_optional!(NSTDOptionalUnichar, NSTDUnichar);
gen_optional!(NSTDOptionalFloat32, NSTDFloat32);
gen_optional!(NSTDOptionalFloat64, NSTDFloat64);
gen_optional!(NSTDOptionalInt, NSTDInt);
gen_optional!(NSTDOptionalUInt, NSTDUInt);
gen_optional!(NSTDOptionalInt8, NSTDInt8);
gen_optional!(NSTDOptionalUInt8, NSTDUInt8);
gen_optional!(NSTDOptionalInt16, NSTDInt16);
gen_optional!(NSTDOptionalUInt16, NSTDUInt16);
gen_optional!(NSTDOptionalInt32, NSTDInt32);
gen_optional!(NSTDOptionalUInt32, NSTDUInt32);
gen_optional!(NSTDOptionalInt64, NSTDInt64);
gen_optional!(NSTDOptionalUInt64, NSTDUInt64);
