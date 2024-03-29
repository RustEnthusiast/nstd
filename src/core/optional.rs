//! Represents an optional (possibly uninitialized) value.
use crate::{
    NSTDAny, NSTDAnyMut, NSTDAnyRef, NSTDAnyRefMut, NSTDBool, NSTDChar, NSTDChar16, NSTDChar32,
    NSTDChar8, NSTDFloat32, NSTDFloat64, NSTDInt, NSTDInt16, NSTDInt32, NSTDInt64, NSTDInt8,
    NSTDRef, NSTDRefMut, NSTDUInt, NSTDUInt16, NSTDUInt32, NSTDUInt64, NSTDUInt8,
};
use nstdapi::nstdapi;

/// Describes an `NSTDOptional` with no value.
pub const NSTD_OPTIONAL_NONE: NSTDUInt8 = 0;
/// Describes an `NSTDOptional` with "some" initialized value.
pub const NSTD_OPTIONAL_SOME: NSTDUInt8 = 1;

/// Represents an optional (possibly uninitialized) value.
#[nstdapi]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum NSTDOptional<T> {
    /// The uninitialized variant.
    #[default]
    None,
    /// The initialized variant.
    Some(T),
}
impl<T> NSTDOptional<T> {
    /// Attempts to return the contained `Some` value in an `NSTDOptional`.
    ///
    /// This operation is only useful for testing code, it's use in production should be
    /// discouraged.
    ///
    /// # Panics
    ///
    /// Panics if `self` is a `None` value.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => panic!("called `NSTDOptional::unwrap()` on a `None` value"),
        }
    }

    /// Attempts to return the contained `Some` value in an `NSTDOptional`.
    ///
    /// # Panics
    ///
    /// Panics with `msg` if `self` is a `None` value.
    #[inline]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => panic!("{msg}"),
        }
    }
}

/// Generates optional data structures.
///
/// `NSTDOptional` must be in scope.
///
/// # Example
///
/// ```ignore
/// use nstd_sys::{
///     core::optional::{gen_optional, NSTDOptional},
///     string::NSTDString,
/// };
///
/// gen_optional!(NSTDOptionalString, NSTDString);
/// ```
macro_rules! gen_optional {
    ($name: ident, $T: ty) => {
        #[doc = concat!("Represents an optional value of type `", stringify!($T), "`.")]
        pub type $name = NSTDOptional<$T>;
    };
}
pub(crate) use gen_optional;
gen_optional!(NSTDOptionalAny, NSTDAny);
gen_optional!(NSTDOptionalAnyMut, NSTDAnyMut);
gen_optional!(NSTDOptionalBool, NSTDBool);
gen_optional!(NSTDOptionalChar, NSTDChar);
gen_optional!(NSTDOptionalChar8, NSTDChar8);
gen_optional!(NSTDOptionalChar16, NSTDChar16);
gen_optional!(NSTDOptionalChar32, NSTDChar32);
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

/// Represents an optional value of type `NSTDRef`.
pub type NSTDOptionalRef<'a, T> = NSTDOptional<NSTDRef<'a, T>>;
/// Represents an optional value of type `NSTDRefMut`.
pub type NSTDOptionalRefMut<'a, T> = NSTDOptional<NSTDRefMut<'a, T>>;

/// Represents an optional value of type `NSTDAnyRef`.
pub type NSTDOptionalAnyRef<'a> = NSTDOptional<NSTDAnyRef<'a>>;
/// Represents an optional value of type `NSTDAnyRefMut`.
pub type NSTDOptionalAnyRefMut<'a> = NSTDOptional<NSTDAnyRefMut<'a>>;
