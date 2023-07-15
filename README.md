# NSTD
A cross-platform, fast, and safe general purpose C library written in Rust.

The library is organized as a series of modules. The top level module `nstd` encompasses the entire
crate. Each module can have their own submodules (eg. `nstd.io.stdout` or `nstd::io::stdout` with
Rust syntax).

# Example using C
```c
// Build nstd with features set to "capi nstd_core nstd_io".
#include <assert.h>
#include <nstd.h>

/// Main entry point of the program.
int main(void) {
    const NSTDOptionalStr output_opt = nstd_core_str_from_raw_cstr("Hello, 🌎!");
    assert(output_opt.status);
    const NSTDStr output = output_opt.value.some;
    assert(nstd_io_print_line(&output) == NSTD_IO_ERROR_NONE);
    return 0;
}
```

# Library modules
- `nstd` - A cross-platform, fast, and safe general purpose C library written in Rust.
    - `alloc` - Low level memory allocation.
    - `app` - An application event loop.
        - `display` - Provides access to physical displays.
        - `events` - Contains callback based events through function pointers.
        - `gamepad` - Gamepad access.
    - `core` - Provides core functionality for `nstd`.
        - `cstr` - Unowned C string slices.
            - `raw` - Raw C string processing.
        - `cty` - Provides functions for examining and operating on character types.
        - `def` - Contains common types used throughout `nstd`.
        - `fty` - Provides functions for examining and operating on floating point types.
        - `ity` - Provides functions for examining and operating on integral types.
        - `math` - Low level math operations.
        - `mem` - Contains mostly unsafe functions for interacting with raw memory.
        - `ops` - Operator overloading for types and operators that may cause overflow.
        - `optional` - Represents an optional (possibly uninitialized) value.
        - `ptr` - A sized pointer to some arbitrary type.
            - `raw` - Provides useful utilities for working with raw pointers.
        - `range` - A numerical range.
        - `result` - Defines a "result" type with success and error variants.
        - `slice` - A view into a sequence of values in memory.
        - `str` - An unowned view into a UTF-8 encoded byte string.
        - `time` - Low level time utilities.
        - `unichar` - A Unicode scalar value.
    - `cstring` - A dynamically sized, null terminated, C string.
    - `env` - Process environment management.
    - `fs` - Provides access to the file system.
        - `file` - A handle to an opened file.
    - `heap_ptr` - A pointer type for single value heap allocation.
    - `io` - Provides functionality for interacting with the standard I/O streams.
        - `stderr` - A handle to the standard error stream.
        - `stdin` - A handle to the standard input stream.
        - `stdout` - A handle to the standard output stream.
    - `math` - High level math operations.
    - `mutex` - A mutual exclusion primitive useful for protecting shared data.
    - `os` - Operating system specific functionality.
        - `unix` - Low level Unix-like operating system support.
            - `alloc` - Memory allocation for Unix-like systems.
            - `io` - Provides functionality for working with input & output on Unix platforms.
            - `mutex` - A mutual exclusion primitive useful for protecting shared data.
            - `shared_lib` - Provides shared library access for Unix-like systems.
            - `time` - Unix time utilities.
        - `windows` - OS support for Windows.
            - `alloc` - Low level memory allocation for Windows.
                - `heap` - Process heap management for Windows.
            - `shared_lib` - Shared library/module access for Windows.
            - `str` - String slice extensions for Windows.
    - `proc` - Calling/Child process management.
    - `shared_lib` - Access symbols from loaded shared libraries.
    - `shared_ptr` - A reference counting smart pointer.
    - `string` - Dynamically sized UTF-8 encoded byte string.
    - `thread` - Thread spawning, joining, and detaching.
    - `time` - Time utilities.
    - `timed_mutex` - A mutual exclusion primitive with a timed locking mechanism.
    - `vec` - A dynamically sized contiguous sequence of values.

# Platform support
`nstd.core` should support anything that rustc supports.

`nstd.os`'s child modules will only work on the operating system they target. For example,
`nstd.os.windows` will only work on Windows and `nstd.os.unix` will only work on Unix-like systems.

Other modules will work on most platforms, primarily targeting Windows, macOS,
Linux, Android, and iOS.

# Language support
This library can be accessed from any language that supports calling C code. As of now this will
need to be done manually as there are no official wrappers for the API, however somewhere around
version 0.11, the plan is to start adding official wrappers so developers from other languages
can easily use the API.

# Safety
*Please note that these safety notes (as well as the framework as a whole) are a work in progress.*

## User safety notes

- Raw pointers are unsafe to access.

- References are assumed to be valid (aligned, non-null, and non-dangling), and are safe to access.
Users can refer to the [docs](https://docs.rs/nstd-sys/latest/nstd_sys/) to see which APIs expect
or return valid references.

- Input reference data is assumed to remain unaltered by other code/threads.

- C function pointers are assumed to be non-null unless wrapped in an `Option`.

- Private (non-`pub`) structure members must not be directly accessed by the user.

- Structured enum variants must be checked before they're accessed (eg. `NSTDOptional` or
`NSTDResult` types).

- Data is *moved* when using the value-copy semantic on a type that does not implement `Copy`.

- Data must not be moved while being referenced by another object.

- Types that do not implement the `Send` trait must not be sent between threads.

- Types that do not implement the `Sync` trait must not be shared between threads.

## Contributor safety notes

- Any operation that may cause
[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) must
be marked unsafe.

- All C function pointers taken as input by the API must be marked unsafe.

- The panic behavior is set to abort by default, as it is undefined behavior to unwind from Rust
code into foreign code (though this is
[subject to change](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html)).

# How to build
Building `nstd` as a C library requires you to specify the "crate-type" manually. To do this you
must pass a `--crate-type` of either `cdylib` or `staticlib` to rustc. Rust allows you to use this
flag multiple times in case you need both.

`nstd` lets you decide what features you want to use.

Any module that falls under the top level module has a dedicated feature flag, for example
`nstd.core` has the feature flag `nstd_core` and `nstd.alloc` has the feature flag `nstd_alloc`.

Each module may have additional features, for example `nstd.os` has the additional
`nstd_os_windows_alloc` feature for memory allocation on Windows, this allows other modules to use
the low level memory allocation API for Windows without enabling memory allocation for other
operating systems.

The `std` feature flag links the Rust standard library into the binary.

The `capi` feature flag is used to build `nstd` as a C library.

The `link` feature flag will link to an existing `nstd` library on the system. This feature is
encouraged but not required to be enabled for Rust crates that use `nstd`'s `capi` feature.

`std` and `nstd_core` are enabled by default.

Example:
```sh
cargo rustc --release --crate-type cdylib --crate-type staticlib --features "capi nstd_alloc"
```

To build with all features:
```sh
cargo rustc --release --crate-type cdylib --crate-type staticlib --all-features
```

# Installing with `cargo-c`
`nstd` also allows you to use `cargo-c` to build or install the library.

Install `cargo-c`:
```sh
cargo install cargo-c
```

Here is an example of how to build the library for a Unix machine with all features enabled:
```sh
cargo cinstall --release --all-features --destdir=./install --prefix=/usr --libdir=/usr/lib
```
This will create a new `install` directory with the installation contents.

You can now copy the contents to the root folder using `cp`:
```sh
sudo cp -a ./install/* /
```

More information can be found in the [cargo-c](https://github.com/lu-zero/cargo-c) repo and in this
[blog post](https://dev.to/luzero/building-crates-so-they-look-like-c-abi-libraries-1ibn) by Luca
Barbato.

# Releases
`nstd` versions follow the Semantic Versioning rules. Each release is given a major, minor, and
patch number that makes up that version of the library (major.minor.patch).

There have not yet been any major releases for the framework as it is not yet stable.

A new minor version is released every 6 weeks, exactly 1 week after a new minor Rust release.

Patch releases are released every so often with minor fixes and additions.

See [semver.org](https://semver.org/) to learn more about Semantic Versioning.
