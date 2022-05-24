#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "nstd_alloc")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_alloc")))]
pub mod alloc;
#[cfg(feature = "nstd_core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_core")))]
pub mod core;
#[cfg(feature = "nstd_os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_os")))]
pub mod os;
#[cfg(feature = "nstd_string")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_string")))]
pub mod string;
#[cfg(test)]
pub(crate) mod test;
#[cfg(feature = "nstd_vec")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_vec")))]
pub mod vec;
