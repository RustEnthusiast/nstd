# NSTD
A cross-platform, fast, and safe general purpose C library written in Rust.

The library is organized as a series of modules. The top level module `nstd` encompasses the entire
crate. Each module can have their own submodules (eg. `nstd.core.def` or `nstd::core::def` with
Rust syntax).

# Modules
- `nstd` - A cross-platform, fast, and safe general purpose C library written in Rust.
    - `core` - The central and most important part of `nstd`.
        - `def` - Contains common types used throughout `nstd`.
        - `ptr` - A sized pointer to some arbitrary type.

# How to build
`nstd` let's you decide what features you want to use. Any module that falls under the top level
module has a dedicated feature flag, for example `nstd.core` has the feature flag `nstd_core`. To
build `nstd` as a C library, use the `clib` feature flag. The `std` feature flag enables Rust
standard library support, all modules other than `nstd.core` require this flag. `std` and
`nstd_core` are enabled by default.

For example:
```
cargo build --features "clib nstd_libx nstd_liby nstd_libz"
```
