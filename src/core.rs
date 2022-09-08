//! Provides core functionality for `nstd`.
//!
//! This library makes no use of Rust's [std] module, and is looking to be dependency free in the
//! near future, meaning that this will only make use of the [core] standard library module.
//!
//! However, `nstd.core` makes use of inline assembly, which can be incredibly unsafe, so we
//! provide a feature flag named `asm` to enable the usage of this feature to increase performance.
pub mod cstr;
pub mod cty;
pub mod def;
pub mod fty;
pub mod ity;
pub mod math;
pub mod mem;
pub mod ptr;
pub mod range;
pub mod slice;
pub mod str;
