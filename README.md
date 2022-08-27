# NSTD
A cross-platform, fast, and safe general purpose C library written in Rust.

The library is organized as a series of modules. The top level module `nstd` encompasses the entire
crate. Each module can have their own submodules (eg. `nstd.core.def` or `nstd::core::def` with
Rust syntax).

# Modules
- `nstd` - A cross-platform, fast, and safe general purpose C library written in Rust.
    - `alloc` - Low level memory allocation.
    - `app` - An application event loop.
        - `data` - Application data passed to each event.
        - `events` - Contains callback based events through function pointers.
        - `handle` - A handle to the application event loop.
    - `core` - Provides core functionality for `nstd`.
        - `cstr` - Unowned C string slices.
            - `raw` - Raw C string processing.
        - `cty` - Provides functions for examining and operating on character types.
        - `def` - Contains common types used throughout `nstd`.
        - `fty` - Provides functions for examining and operating on floating point types.
        - `ity` - Provides functions for examining and operating on integral types.
        - `math` - Low level math operations.
        - `mem` - Contains mostly unsafe functions for interacting with raw memory.
        - `ptr` - A sized pointer to some arbitrary type.
        - `range` - A half-open (low inclusive, high exclusive) numerical range.
        - `slice` - A view into a sequence of values in memory.
        - `str` - An unowned view into a UTF-8 encoded byte string.
    - `cstring` - A dynamically sized, null terminated, C string.
    - `heap_ptr` - A pointer type for single value heap allocation.
    - `image` - Multi-format image processing.
    - `io` - Provides functionality for interacting with the standard I/O streams.
        - `stderr` - A handle to the standard error stream.
        - `stdin` - A handle to the standard input stream.
        - `stdout` - A handle to the standard output stream.
    - `os` - Operating system specific functionality.
        - `windows` - OS support for Windows.
            - `alloc` - Low level memory allocation for Windows.
                - `heap` - Process heap management for Windows.
    - `shared_lib` - Access symbols from loaded shared libraries.
    - `shared_ptr` - A reference counting smart pointer.
    - `string` - Dynamically sized UTF-8 encoded byte string.
    - `vec` - A dynamically sized contiguous sequence of values.
    - `window` - An `nstd` application window.

# Platform support
`nstd.core` should support anything that rustc supports.

`nstd.os`'s child modules will only work on the operating system they target. For example,
`nstd.os.windows` will only work on Windows and `nstd.os.linux` will only work on Linux
distributions.

Other modules will work on most platforms, primarily targeting Windows, macOS,
Linux, Android, and iOS.

# Language support
This library can be accessed from any language that supports calling C code! As of now this will
need to be done manually as there are no official wrappers for the API, however whenever library
versioning occurs, the plan is to start adding official wrappers so developers from other languages
can easily use the API.

# Safety notes
`nstd` tries it's best to comply with Rust's safety. This means anything that can cause undefined
behavior is considered unsafe (with the exception of functions that take Rusty references, which
always assume a non-null argument). However `nstd` *is* a C library, and we do not have access to
the borrow checker in C, and making every function that borrows data mutably "unsafe" would not be
ideal. I am always looking for ways to make this API as safe as sanely possible, so please open an
issue if you have any ideas on how we can do so, it would be greatly appreciated.

# How to build
`nstd` let you decide what features you want to use. Any module that falls under the top level
module has a dedicated feature flag, for example `nstd.core` has the feature flag `nstd_core` and
`nstd.alloc` has the feature flag `nstd_alloc`. Each module can also have additional features, for
example `nstd.os` has the additional `nstd_os_windows_alloc` feature for memory allocation on
Windows, this allows other modules to use the low level memory allocation API for Windows without
enabling memory allocation support for other operating systems. To build `nstd` as a C library, use
the `clib` feature flag. The `std` feature flag enables Rust standard library support. `std` and
`nstd_core` are enabled by default.

For example:
```sh
cargo build --release --features "clib nstd_alloc nstd_vec"
```

To build with all features:
```sh
cargo build --release --all-features
```
