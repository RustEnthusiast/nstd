#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#[cfg(feature = "nstd_core")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_core")))]
pub mod core;
