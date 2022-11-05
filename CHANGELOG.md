# TBD
### `nstd`
- The overflow behavior for the "release" profile has been set to panic.
- The panic behavior for the "release" profile has been set to abort.
### `nstd.core`
- `str[_mut]_to_*` functions now return `Optional`.
- Added `cstr[_mut]_[first|last][_const]`.
- Added `ops`.
- Added `cty_is_unicode`.
- Renamed `str[_mut]_get_char` to `str[_mut]_get`.
- Added `Optional`.
- Added `Result`.
- Added `panic`.
- Made `math_[clamp|div_ceil|div_floor]_*` safe.
### `nstd.cstring`
- Added `from_cstr`.
- Renamed `to_bytes` to `into_bytes`.
### `nstd.fs`
- Added `NSTDFileResult`.
### `nstd.os`
- Added `unix.alloc`.
- Added `[unix|windows].shared_lib`.
- Added `NSTDWindowsHeapResult`.
- Added `windows_alloc_heap_validate`.
- Added `windows_alloc_heap_size`.
- Added `NSTDWindowsAllocError`.
- Renamed `NSTDWindowsHeapHandle` to `NSTDWindowsHeap`.
### `nstd.string`
- Added `from_str`.
- Renamed `to_bytes` to `into_bytes`.
### `nstd.vec`
- Added `from_slice`.
# 0.2.0
### `nstd`
- Added `nstd.fs`.
- Added `nstd.math`.
- Added `asm` feature.
- Made `NSTDChar` a primitive.
- Updated [windows-sys](https://crates.io/crates/windows-sys) to version 0.42.
### `nstd.core`
- Made `str[_mut]_substr` unsafe.
- Made `str[_mut]_from_bytes` unsafe.
- Made `str[_mut]_from_cstr` unsafe.
- Renamed `cty`'s ASCII functions (`is_punctuation`, `is_graphic`, `to_lowercase`, `to_uppercase`).
- Added `mem_search`.
- Added `str[_mut]_from_raw_cstr[_with_null]`.
- Made `cstr[_mut]_get_null[_const]` unsafe.
- Made `cstr[_mut]_from_raw[_with_null]` unsafe.
- Renamed range types.
- `math_[clamp|div_ceil|div_floor]` are now unsafe.
- `cty_is_[graphic|punctuation]` now take `NSTDChar`.
- Added `math_div_floor_*`.
- Added `math_pow_*`.
- Added `math_abs_*`.
### `nstd.os`
- Added `NSTD_OS_[LINUX|MACOS|WINDOWS]` constants.
- Made `NSTDWindowsHeapHandle` a structure.

# 0.1.3
### `nstd`
- Added `extern "C"` to `NSTDAPI` (removes `NSTDCPP[START|END]`).
### `nstd.core`
- Removed `nstd_core_slice[_mut]_compare`.

# 0.1.2
### `nstd`
- Removed `Const` postfix from `NSTDAnyConst`.
### `nstd.core`
- Fixed `NSTDSlice[Mut]::as_slice[_mut]` returning a slice of incorrect length.
- Renamed `fty` functions.
- Added `nstd_core_str[_mut]_byte_len`.
- Removed `Const` postfix from `NSTD[Ptr|Slice|Str|CStr]Const`.
### `nstd.string`
- Added `nstd_string_byte_len`.

# 0.1.1
### `nstd`
- Renamed `NSTD[I|U]Size` to `NSTD[U]Int`.
### `nstd.core`
- Fixed `math.div_ceil` functions returning the incorrect value.

# 0.1.0
### `nstd`
- Added `nstd.alloc`.
- Added `nstd.core`.
- Added `nstd.cstring`.
- Added `nstd.heap_ptr`.
- Added `nstd.io`.
- Added `nstd.os`.
- Added `nstd.shared_lib`.
- Added `nstd.shared_ptr`.
- Added `nstd.string`.
- Added `nstd.vec`.
