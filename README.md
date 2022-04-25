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
