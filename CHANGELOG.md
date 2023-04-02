# TBD
## Added
### `nstd.core`
- Added `nstd_core_slice[_mut]_empty`.
### `nstd.os`
- Added `NSTDUnixAllocError`.
## Changed
### `nstd`
- Updated `windows-sys` to version 0.48.
### `nstd.core`
- `nstd_core_unichar_is_digit` no longer panics.
- `nstd.core.time` functions now take `NSTDDuration` by value.
- `nstd_core_str[_mut]_from_raw_cstr[_with_null]` no longer panics if `cstr` is null.
- `nstd_core_slice_mut_copy` now panics.
### `nstd.cstring`
- `nstd_cstring_from_cstr[_unchecked]` now returns `NSTDOptionalCString`.
- `nstd_cstring_clone` now returns `NSTDOptionalCString`.
### `nstd.heap_ptr`
- `nstd_heap_ptr_[new[_zeroed]|clone]` now returns `NSTDOptionalHeapPtr`.
### `nstd.os`
- `nstd_os_unix_mutex_timed_lock` now takes `NSTDDuration` by value.
- `nstd.os.unix.time` functions now take `NSTDUnixTime` & `NSTDDuration` by value.
### `nstd.shared_ptr`
- `nstd_shared_ptr_new[_zeroed]` now returns `NSTDOptionalSharedPtr`.
### `nstd.string`
- `nstd_string_from_str` now returns `NSTDOptionalString`.
- `nstd_string_clone` now returns `NSTDOptionalString`.
### `nstd.thread`
- `nstd_thread_spawn[_with_desc]` now takes `NSTDOptionalHeapPtr`.
- `nstd_thread_sleep` now takes `NSTDDuration`.
### `nstd.time`
- `nstd.time` functions now take `NSTDTime` & `NSTDDuration` by value.
### `nstd.timed_mutex`
- `nstd_timed_mutex_timed_lock` now takes `NSTDDuration` by value.
### `nstd.vec`
- `nstd_vec_from_slice` now returns `NSTDOptionalVec`.
- `nstd_vec_clone` now returns `NSTDOptionalVec`.

# 0.6.0
### `nstd`
Fixed some C99 header incompatibilities.
### `nstd.core`
- Added `nstd.core.time`.
- `nstd.core.cty`'s functions now use `NSTDChar`.
- Added `nstd_core_unichar_is_[ascii|alphabetic|numeric|alphanumeric|lowercase|uppercase|whitespace|control|digit]`.
- Added `nstd_core_unichar_replacement`.
- `nstd_core_str[_mut]_substr` now returns `NSTDOptionalStr[Mut]`.
- `nstd_core_str[_mut]_from_[cstr|raw_cstr[_with_null]|bytes]` now returns `NSTDOptionalStr[Mut]`.
- `nstd_core_cstr[_mut]_new` now returns `NSTDOptionalCStr[Mut]`.
- `nstd_core_slice[_mut]_new` now returns `NSTDOptionalSlice[Mut]`.
- `nstd_core_ptr[_mut]_new` now returns `NSTDOptionalPtr[Mut]`.
### `nstd.fs`
- `nstd_fs_file_[read[_all|_to_string]|write]` now returns `NSTDIOResult`.
### `nstd.io`
- Added `NSTDStd[in|out|err]Lock`.
- `nstd_io_[stdout|stderr]_write` now returns `NSTDIOResult`.
- `nstd_io_stdin_read[_all|_to_string|_line]` now returns `NSTDIOResult`.
- Added `NSTDIOResult`.
### `nstd.os`
- Added `nstd.os.unix.time`.
- Added `NSTD_OS_[IOS|ANDROID|DRAGONFLY|FREEBSD|NETBSD|OPENBSD|BSD|HAIKU|NTO|SOLARIS]`.
- Added `NSTDUnixIOResult`.
- Added `nstd.os.unix.mutex`.

# 0.5.0
### `nstd`
- Renamed the `clib` feature to `capi`.
- Moved `NSTDUnichar` into `core`.
- Added `nstd.mutex`.
- Added `nstd.timed_mutex`.
- Added `nstd.time`.
- Updated `windows-sys` to version 0.45.
### `nstd.core`
- Added `nstd_core_[ptr|slice|cstr][_mut]_new_unchecked`.
- `nstd_core_ops_sh[l|r]_*` now take `NSTDUInt32` as their right operand.
- Removed floating-point operations from `nstd.core.ops`.
- `nstd_core_str[_mut]_get` now returns `NSTDOptionalUnichar`.
- `nstd_core_cty_is_unicode` now takes `NSTDChar32`.
- Added `nstd_core_panic_with_msg`.
### `nstd.cstring`
- Added `NSTDOptionalCString`.
### `nstd.fs`
- Added `nstd_fs_metadata`.
### `nstd.heap_ptr`
- Added `NSTDOptionalHeapPtr`.
### `nstd.os`
- Added `NSTD_UNIX_IO_ERROR_IS_DIR`.
- Added `nstd.os.unix.io`.
- `nstd_os_windows_shared_lib_load` now accepts Unicode.
- Added `nstd_os_windows_str_to_utf16`.
- Added `nstd_os_unix_shared_lib_handle`.
### `nstd.shared_lib`
- `nstd_shared_lib_load` now takes `NSTDStr`.
### `nstd.shared_ptr`
- Added `NSTDOptionalSharedPtr`.
### `nstd.string`
- Added `NSTDOptionalString`.
- `nstd_string_push` now returns `NSTDAllocError`.
- `nstd_string_pop` now returns `NSTDOptionalUnichar`.
### `nstd.vec`
- Added `NSTDOptionalVec`.
- Added `nstd_vec_reserved`.

# 0.4.1
### `nstd`
- Fixed missing panic handler compiler error for embedded builds.

# 0.4.0
### `nstd`
- Added `nstd.env`.
### `nstd.core`
- Added `nstd_core_ptr_raw_is_aligned`.
- Added `nstd_core_ptr_raw_align[_mut]`.
- Added `NSTDOptional[Ptr|Slice|CStr|Str][Mut]`.
- Added `nstd_core_ptr_raw_dangling[_mut]`.
- Added `nstd_core_str[_mut]_as_cstr`.
### `nstd.cstring`
- Added `nstd_cstring_from_bytes`.
- Added `nstd_cstring_from_cstr_unchecked`.
- Removed `nstd_cstring_as_cstr_mut`.
### `nstd.fs`
- `nstd_fs_[absolute|read_to_string]` now returns `NSTDIOStringResult`.
- `nstd_fs_read` now returns `NSTDIOBufferResult`.
### `nstd.heap_ptr`
- `nstd_heap_ptr_new_zeroed` is now unsafe.
### `nstd.io`
- `nstd_io_read[_line]` now returns `NSTDIOStringResult`.
- Added `NSTDIOBufferResult` & `NSTDIOStringResult`.
### `nstd.os`
- Added `NSTDWindowsHandle`.
- Removed `NSTDWindowsSharedLibHandle`.
- Removed `NSTDWindowsHeapHandle`.
- Removed `NSTDUnixSharedLibHandle`.
- [`unix.alloc`] Fixed linker error when using multiple versions of the crate.
### `nstd.shared_lib`
- `nstd_shared_lib_load` now takes `NSTDCStr`.
### `nstd.shared_ptr`
- `nstd_shared_ptr_new_zeroed` is now unsafe.
### `nstd.string`
- Added `nstd_string_from_bytes`.
### `nstd.thread`
- Added `NSTDThreadResult`.
- `nstd_thread_name` now returns `NSTDOptionalStr`.
- `NSTDThreadDescriptor::name` is now `NSTDOptionalStr`.
- Added `nstd_thread_current`.
### `nstd.vec`
- Added `nstd_vec_end[_mut]`.
- Added `nstd_vec_set_len`.
- Renamed `nstd_vec_as_mut_ptr` to `nstd_vec_as_ptr_mut`.

# 0.3.3
### `nstd`
- Internal safety improvements.
- Added `proc`.
- Added `thread`.
- Removed dependency for `libloading`.
### `nstd.os`
- Added `NSTDUnixSharedLibHandle`.
- Added `NSTD_OS_UNIX`.

# 0.3.2
### `nstd`
- Second attempt to fix docs.rs build for non-x86_64 Unix systems.

# 0.3.1
### `nstd`
- Attempted to fix docs.rs build for non-x86_64 Unix systems.

# 0.3.0
### `nstd`
- The overflow behavior for the "release" profile has been set to panic.
- The panic behavior for the "release" profile has been set to abort.
### `nstd.core`
- `str[_mut]_to_*` functions now return `NSTDOptional`.
- Added `cstr[_mut]_[first|last][_const]`.
- Added `ops`.
- Added `cty_is_unicode`.
- Renamed `str[_mut]_get_char` to `str[_mut]_get`.
- Added `NSTDOptional`.
- Added `NSTDResult`.
- Added `panic`.
- Made `math_[clamp|div_ceil|div_floor]_*` safe.
### `nstd.cstring`
- Added `clear`.
- Added `from_cstr`.
- Renamed `to_bytes` to `into_bytes`.
### `nstd.fs`
- Added `NSTDFileResult`.
### `nstd.os`
- Added `NSTDWindowsHeapHandle`.
- Added `NSTDWindowsSharedLibHandle`.
- Added `unix.alloc`.
- Added `[unix|windows].shared_lib`.
- Added `NSTDWindowsHeapResult`.
- Added `windows_alloc_heap_validate`.
- Added `windows_alloc_heap_size`.
- Added `NSTDWindowsAllocError`.
- Renamed `NSTDWindowsHeapHandle` to `NSTDWindowsHeap`.
### `nstd.shared_lib`
- Added `NSTDOptionalSharedLib`.
### `nstd.string`
- Added `clear`.
- Added `from_str`.
- Renamed `to_bytes` to `into_bytes`.
### `nstd.vec`
- Added `clear`.
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
