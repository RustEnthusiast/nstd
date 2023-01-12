# Welcome
`nstd` not good enough? Let's make it better. This text is here to provide a formal introduction on
how to contribute to the `nstd` codebase. The code is currently hosted on GitHub at
https://github.com/RustEnthusiast/nstd. Make a clone or fork of the repository to get started.

Please note that you may need to know a bit of Rust and C before contributing.

# Sections
- [Adding a module](#adding-a-module)
- [Adding a submodule](#adding-a-submodule)
- [Adding a type](#adding-a-type)
- [Adding a function](#adding-a-function)

# Adding a module
Adding a top-level module to `nstd` is fairly straight forward, but requires a few tedious steps.

## Create a Cargo feature for the module
The first thing you'll need to do when creating a new module is create a feature for the module.
This will allow users of the framework to disable compilation of our module if their code doesn't
require it.

To do this, open the `Cargo.toml` file and head to the `[features]` section. The module features
are listed in alphabetical order following the `default`, `std`, `clib`, and `asm` features. Place
the new feature where it is appropriate.

The naming convention for a module feature is as follows: `nstd_{module_name}`.

The new feature should look something like this: `nstd_{module_name} = []`.

The square brackets contain any dependencies that our module requires.

## Add the module header
Each top-level module needs a C header file in the `include/nstd` directory in the repository
describing the module's API. The header file shall be named `{module_name}.h`. There should now be
a new file located at `include/nstd/{module_name}.h`.

Each header file needs a header guard. `nstd` does not use the `#pragma once` directive as it is not
strictly part of the C standard, so don't use this approach. Instead we will use a form of header
guards that are guaranteed to work with any compiler. Header guard definitions in `nstd` are just a
path to the file from within the `include` directory. The header guard should look like this:
```c
#ifndef NSTD_{MODULE_NAME}_H
#define NSTD_{MODULE_NAME}_H
#endif
```
The `include/nstd.h` header includes all top-level headers in alphabetical order, be sure to
include your header in this file.

## Add the source file
You will now need to add a Rust source file that defines your module's API. Create a new Rust
source file in the `src` directory named `{module_name}.rs`. Make sure to give a description of
your module with a `//!` comment at the top of the file. Since we are creating a top-level module
we will need to modify the `src/lib.rs` file by declaring our module in there. All other top-level
modules are listed here in alphabetical order. Every top-level module must be hidden behind a
feature gate, we will need to enforce that here by using Rust's `cfg` attribute. Place a
declaration of your module where it is appropriate. The module declaration should look like this:
```rs
#[cfg(feature = "nstd_{module_name}")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "nstd_{module_name}")))]
pub mod {module_name};
```

## Update the README
This step isn't that important but it is technically required. The repo's `README.md` markdown file
contains an alphabetically sorted list of `nstd`'s modules along with a short description of each
module. Place your module into this tree where it is appropriate.

# Adding a submodule
```rs
todo!();
```

# Adding a type
```rs
todo!();
```

# Adding a function
```rs
todo!();
```
