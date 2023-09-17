# TBD
## Removed
### `nstd.thread`
- Removed `NSTDOptionalThreadHandle`.
- Removed `NSTDOptionalThreadID`.

# 0.10.0
## Added
### `nstd`
- Added `NSTDRef[Mut]`.
- Added `NSTDAnyRef[Mut]`.
### `nstd.core`
- Added `NSTDOptionalRef[Mut]`.
- Added `NSTDOptionalAnyRef[Mut]`.
## Changed
### `nstd.core`
- `nstd_core_math_div_[ceil|floor]_*` functions now return optionals.
- `nstd_core_str[_mut]_from_bytes` functions no longer panic.
- Functions in `nstd.core.mem` no longer panic.
- Functions in `nstd.core.ops` now return optionals instead of panicking.
### `nstd.fs`
- `nstd_fs_write` no longer panics.
### `nstd.os`
- `nstd_os_windows_str_to_utf16` now returns `NSTDOptionalVec`.
### `nstd.proc`
- `nstd_proc_spawn` no longer panics.
### `nstd.shared_lib`
- `nstd_shared_lib_load` no longer panics.
### `nstd.string`
- `nstd_string_new_with_cap` now returns `NSTDOptionalString`.
### `nstd.vec`
- `nstd_vec_new_with_cap` now returns `NSTDOptionalVec`.
## Removed
### `nstd.core`
- Removed `nstd_core_abort_with_msg`.
- Removed `nstd_core_ops_[inc|dec]_*` functions.

# 0.9.0
## Added
### `nstd`
- Added an `nstd` feature to enable all modules.
- Added a `link` feature.
### `nstd.io`
- Added `NSTDOptionalStd[in|out|err][Lock]`.
### `nstd.mutex`
- Added `NSTDOptionalMutex`.
### `nstd.thread`
- Added `NSTDOptionalThreadHandle`.
- Added `NSTDOptionalThreadID`.
## Changed
### `nstd`
- Removed the `nstd_` prefix from all features.
### `nstd.env`
- `nstd_env_temp_dir` now returns `NSTDOptionalString`.
### `nstd.io`
- `nstd_io_std[in|out|err]` functions now return `NSTDOptionalStd[in|out|err]` respectively.
- `nstd_io_std[in|out|err]_lock` functions now return `NSTDOptionalStd[in|out|err]Lock`
respectively.
### `nstd.mutex`
- `nstd_mutex_new` now returns `NSTDOptionalMutex`.
- `nstd_mutex_lock` now returns `NSTDOptionalMutexLockResult`.
### `nstd.thread`
- `NSTDThreadResult` is now `NSTDOptionalHeapPtr`.
- `nstd_thread_spawn` now takes `const NSTDThreadDescriptor *desc`.
- `nstd_thread_[current|handle]` functions now return `NSTDOptionalThreadHandle`.
- `nstd_thread_id` now returns `NSTDOptionalThreadID`.
## Removed
### `nstd`
- Removed the `asm` feature.
### `nstd.thread`
- `nstd_thread_spawn_with_desc`.

# 0.8.0
## Added
### `nstd.alloc`
- Added `NSTDAllocator` & `NSTD_ALLOCATOR`.
### `nstd.cstring`
- Added `nstd_cstring_allocator`.
### `nstd.heap_ptr`
- Added `nstd_heap_ptr_allocator`.
### `nstd.shared_ptr`
- Added `nstd_shared_ptr_allocator`.
### `nstd.string`
- Added `nstd_string_allocator`.
### `nstd.vec`
- Added `nstd_vec_allocator`.
## Changed
### `nstd.cstring`
- `nstd_cstring_from_cstr[_unchecked]` functions now take an `NSTDAllocator`.
- `nstd_cstring_new[_with_cap]` functions now take an `NSTDAllocator`.
- `nstd_cstring_new_with_cap` no longer panics.
### `nstd.env`
- `nstd_env_var` no longer panics.
- `nstd_env_temp_dir` now returns `NSTDOptionalString`.
- `nstd_env_current_[dir|exe]` functions no longer panic.
### `nstd.fs`
- `nstd_fs_absolute` no longer panics.
- `nstd_fs_read[_to_string]` functions no longer panic.
### `nstd.heap_ptr`
- `nstd_heap_ptr_new[_zeroed]` functions now take an `NSTDAllocator`.
### `nstd.io`
- `nstd_io_read[_line]` functions no longer panic.
### `nstd.shared_ptr`
- `nstd_shared_ptr_new[_zeroed]` functions now take an `NSTDAllocator`.
### `nstd.string`
- `nstd_string_from_str` now takes an `NSTDAllocator`.
- `nstd_string_new[_with_cap]` functions now take an `NSTDAllocator`.
- `nstd_string_new_with_cap` no longer panics.
- `nstd_string_from_*` functions now return `NSTDOptionalString`.
### `nstd.vec`
- `nstd_vec_from_slice` now takes an `NSTDAllocator`.
- `nstd_vec_new[_with_cap]` functions now take an `NSTDAllocator`.
- `nstd_vec_new[_with_cap]` functions no longer panic.
- `nstd_vec_from_slice` no longer panics.
- `nstd_vec_set_len` no longer returns `NSTDErrorCode`.

# 0.7.0
## Added
### `nstd.core`
- Added `nstd_core_abort[_with_msg]`.
- Added `NSTDOptionalAny[Mut]`.
- Added `nstd_core_slice[_mut]_empty`.
### `nstd.heap_ptr`
- Added `nstd_heap_ptr_drop`.
### `nstd.mutex`
- Added `nstd_mutex_into_inner`.
- Added `nstd_mutex_drop`.
### `nstd.os`
- Added `NSTD_UNIX_ALLOC_ERROR_INVALID_LAYOUT`.
- Added `nstd_os_unix_mutex_into_inner`.
- Added `nstd_os_unix_mutex_drop`.
- Added `NSTDUnixAllocError`.
### `nstd.shared_ptr`
- Added `nstd_shared_ptr_drop`.
### `nstd.timed_mutex`
- Added `nstd_timed_mutex_into_inner`.
- Added `nstd_timed_mutex_drop`.
### `nstd.vec`
- Added `nstd_vec_drop`.
## Changed
### `nstd`
- Updated `windows-sys` to version 0.48.
### `nstd.core`
- `nstd_core_str[_mut]_[from_cstr|from_raw_cstr[_with_null]|len|get|to_*]` no longer panics.
- `nstd_core_cstr[_mut]_[is_null_terminated|get_null|get][_const]` no longer panics.
- `nstd_core_slice[_mut]_[get|last][_const]` no longer panics.
- `nstd_core_mem_compare` no longer panics.
- `NSTDOptional` & `NSTDResult` now use `NSTDUInt8` as a discriminant.
- `nstd_core_unichar_is_digit` no longer panics.
- `nstd.core.time` functions now take `NSTDDuration` by value.
- `nstd_core_slice_mut_copy` now panics.
### `nstd.cstring`
- `nstd_cstring_new[_with_cap]` now returns `NSTDOptionalCString`.
- `nstd_cstring_push` now returns `NSTDAllocError`.
- `nstd_cstring_[new|from_cstr[_unchecked]|push|pop]` no longer panics.
- `nstd_cstring_from_cstr[_unchecked]` now returns `NSTDOptionalCString`.
- `nstd_cstring_clone` now returns `NSTDOptionalCString`.
### `nstd.env`
- `nstd_env_set_current_dir` no longer panics.
### `nstd.fs`
- `nstd_fs_[[create|remove]_[file|dir|dirs]|rename|copy|metadata]` no longer panics.
- `nstd_fs_file_[open|read_all|read_to_string]` no longer panics.
### `nstd.heap_ptr`
- `nstd_heap_ptr_[new[_zeroed]|clone]` now returns `NSTDOptionalHeapPtr`.
### `nstd.io`
- `nstd_io_print[_line]` no longer panics.
- `nstd_io_stdin[_lock]_[read_all|read_to_string|read_line]` no longer panics.
### `nstd.os`
- `nstd_os_unix_mutex_timed_lock` now takes `NSTDDuration` by value.
- `nstd.os.unix.time` functions now take `NSTDUnixTime` & `NSTDDuration` by value.
### `nstd.shared_ptr`
- `nstd_shared_ptr_new[_zeroed]` now returns `NSTDOptionalSharedPtr`.
### `nstd.string`
- `nstd_string_[len|push[_str]|pop]` no longer panics.
- `nstd_string_from_[str|bytes]` now returns `NSTDOptionalString`.
- `nstd_string_clone` now returns `NSTDOptionalString`.
### `nstd.thread`
- `nstd_thread_spawn_with_desc` no longer panics.
- `nstd_thread_spawn[_with_desc]` now takes `NSTDOptionalHeapPtr`.
- `nstd_thread_sleep` now takes `NSTDDuration`.
### `nstd.time`
- `nstd_time_now` now returns `NSTDOptionalTime`.
- `nstd.time` functions now take `NSTDTime` & `NSTDDuration` by value.
### `nstd.timed_mutex`
- `nstd_timed_mutex_timed_lock` now takes `NSTDDuration` by value.
### `nstd.vec`
- `nstd_vec_[end|get|push|pop|insert|remove][_mut]` no longer panics.
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
