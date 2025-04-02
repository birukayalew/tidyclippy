use ::libc;
use ::f128;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_float;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type trio_long_double_t = f128::f128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TRIO_HASH_TWOSIGNED: C2RustUnnamed_0 = 2;
pub const TRIO_HASH_PLAIN: C2RustUnnamed_0 = 1;
pub const TRIO_HASH_NONE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trio_string_t {
    pub content: *mut libc::c_char,
    pub length: size_t,
    pub allocated: size_t,
}
pub type trio_string_t = _trio_string_t;
#[no_mangle]
pub unsafe extern "C" fn trio_create(mut size: size_t) -> *mut libc::c_char {
    return malloc(size) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn trio_destroy(mut string: *mut libc::c_char) {
    if !string.is_null() {
        free(string as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn trio_length(mut string: *const libc::c_char) -> size_t {
    return strlen(string);
}
#[no_mangle]
pub unsafe extern "C" fn trio_append(
    mut target: *mut libc::c_char,
    mut source: *const libc::c_char,
) -> libc::c_int {
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int trio_append(char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4668: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                191 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int trio_append(char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            192 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int trio_append(char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4635: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                192 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int trio_append(char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (strcat(target, source) != 0 as *mut libc::c_void as *mut libc::c_char)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_append_max(
    mut target: *mut libc::c_char,
    mut max: size_t,
    mut source: *const libc::c_char,
) -> libc::c_int {
    let mut length: size_t = 0;
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            223 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int trio_append_max(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4780: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                223 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int trio_append_max(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int trio_append_max(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4748: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int trio_append_max(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    length = trio_length(target);
    if max > length {
        strncat(
            target,
            source,
            max.wrapping_sub(length).wrapping_sub(1 as libc::c_int as size_t),
        );
    }
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_contains(
    mut string: *const libc::c_char,
    mut substring: *const libc::c_char,
) -> libc::c_int {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            251 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int trio_contains(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4864: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                251 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int trio_contains(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !substring.is_null() {} else {
        __assert_fail(
            b"substring\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            252 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int trio_contains(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4831: {
        if !substring.is_null() {} else {
            __assert_fail(
                b"substring\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                252 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int trio_contains(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (0 as *mut libc::c_char != strstr(string, substring)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_copy(
    mut target: *mut libc::c_char,
    mut source: *const libc::c_char,
) -> libc::c_int {
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            279 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int trio_copy(char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4951: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int trio_copy(char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            280 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int trio_copy(char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4918: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                280 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int trio_copy(char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    strcpy(target, source);
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_copy_max(
    mut target: *mut libc::c_char,
    mut max: size_t,
    mut source: *const libc::c_char,
) -> libc::c_int {
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_copy_max(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3219: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                309 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_copy_max(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            310 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_copy_max(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3186: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                310 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_copy_max(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if max > 0 as libc::c_int as size_t {} else {
        __assert_fail(
            b"max > 0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            311 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_copy_max(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3143: {
        if max > 0 as libc::c_int as size_t {} else {
            __assert_fail(
                b"max > 0\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                311 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_copy_max(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    strncpy(target, source, max.wrapping_sub(1 as libc::c_int as size_t));
    *target
        .offset(
            max.wrapping_sub(1 as libc::c_int as size_t) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioDuplicateMax(
    mut source: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"char *TrioDuplicateMax(const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_3345: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                330 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"char *TrioDuplicateMax(const char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    size = size.wrapping_add(1);
    size;
    target = trio_create(size);
    if !target.is_null() {
        trio_copy_max(target, size, source);
    }
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn trio_duplicate(
    mut source: *const libc::c_char,
) -> *mut libc::c_char {
    return TrioDuplicateMax(source, trio_length(source));
}
#[no_mangle]
pub unsafe extern "C" fn trio_duplicate_max(
    mut source: *const libc::c_char,
    mut max: size_t,
) -> *mut libc::c_char {
    let mut length: size_t = 0;
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            377 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"char *trio_duplicate_max(const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5058: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                377 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"char *trio_duplicate_max(const char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if max > 0 as libc::c_int as size_t {} else {
        __assert_fail(
            b"max > 0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            378 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"char *trio_duplicate_max(const char *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5019: {
        if max > 0 as libc::c_int as size_t {} else {
            __assert_fail(
                b"max > 0\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                378 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"char *trio_duplicate_max(const char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    length = trio_length(source);
    if length > max {
        length = max;
    }
    return TrioDuplicateMax(source, length);
}
#[no_mangle]
pub unsafe extern "C" fn trio_equal(
    mut first: *const libc::c_char,
    mut second: *const libc::c_char,
) -> libc::c_int {
    if !first.is_null() {} else {
        __assert_fail(
            b"first\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            405 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int trio_equal(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3454: {
        if !first.is_null() {} else {
            __assert_fail(
                b"first\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                405 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"int trio_equal(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !second.is_null() {} else {
        __assert_fail(
            b"second\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            406 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int trio_equal(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3421: {
        if !second.is_null() {} else {
            __assert_fail(
                b"second\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                406 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"int trio_equal(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !first.is_null() && !second.is_null() {
        return (0 as libc::c_int == strcasecmp(first, second)) as libc::c_int;
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_equal_case(
    mut first: *const libc::c_char,
    mut second: *const libc::c_char,
) -> libc::c_int {
    if !first.is_null() {} else {
        __assert_fail(
            b"first\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            444 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_equal_case(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3560: {
        if !first.is_null() {} else {
            __assert_fail(
                b"first\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                444 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_equal_case(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !second.is_null() {} else {
        __assert_fail(
            b"second\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            445 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_equal_case(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3528: {
        if !second.is_null() {} else {
            __assert_fail(
                b"second\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                445 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_equal_case(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !first.is_null() && !second.is_null() {
        return (0 as libc::c_int == strcmp(first, second)) as libc::c_int;
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_equal_case_max(
    mut first: *const libc::c_char,
    mut max: size_t,
    mut second: *const libc::c_char,
) -> libc::c_int {
    if !first.is_null() {} else {
        __assert_fail(
            b"first\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            473 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"int trio_equal_case_max(const char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5167: {
        if !first.is_null() {} else {
            __assert_fail(
                b"first\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                473 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"int trio_equal_case_max(const char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !second.is_null() {} else {
        __assert_fail(
            b"second\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            474 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"int trio_equal_case_max(const char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5134: {
        if !second.is_null() {} else {
            __assert_fail(
                b"second\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                474 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"int trio_equal_case_max(const char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !first.is_null() && !second.is_null() {
        return (0 as libc::c_int == strncmp(first, second, max)) as libc::c_int;
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_equal_locale(
    mut first: *const libc::c_char,
    mut second: *const libc::c_char,
) -> libc::c_int {
    if !first.is_null() {} else {
        __assert_fail(
            b"first\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            500 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int trio_equal_locale(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3639: {
        if !first.is_null() {} else {
            __assert_fail(
                b"first\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                500 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int trio_equal_locale(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !second.is_null() {} else {
        __assert_fail(
            b"second\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            501 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int trio_equal_locale(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3606: {
        if !second.is_null() {} else {
            __assert_fail(
                b"second\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                501 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int trio_equal_locale(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_equal(first, second);
}
#[no_mangle]
pub unsafe extern "C" fn trio_equal_max(
    mut first: *const libc::c_char,
    mut max: size_t,
    mut second: *const libc::c_char,
) -> libc::c_int {
    if !first.is_null() {} else {
        __assert_fail(
            b"first\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            528 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int trio_equal_max(const char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3750: {
        if !first.is_null() {} else {
            __assert_fail(
                b"first\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                528 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int trio_equal_max(const char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !second.is_null() {} else {
        __assert_fail(
            b"second\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            529 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int trio_equal_max(const char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3717: {
        if !second.is_null() {} else {
            __assert_fail(
                b"second\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                529 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int trio_equal_max(const char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !first.is_null() && !second.is_null() {
        return (0 as libc::c_int == strncasecmp(first, second, max)) as libc::c_int;
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_error(
    mut error_number: libc::c_int,
) -> *const libc::c_char {
    return strerror(error_number);
}
#[no_mangle]
pub unsafe extern "C" fn trio_format_date_max(
    mut target: *mut libc::c_char,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut datetime: *const tm,
) -> size_t {
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            608 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5327: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                608 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !format.is_null() {} else {
        __assert_fail(
            b"format\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            609 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5295: {
        if !format.is_null() {} else {
            __assert_fail(
                b"format\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                609 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !datetime.is_null() {} else {
        __assert_fail(
            b"datetime\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            610 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5262: {
        if !datetime.is_null() {} else {
            __assert_fail(
                b"datetime\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                610 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if max > 0 as libc::c_int as size_t {} else {
        __assert_fail(
            b"max > 0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            611 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5223: {
        if max > 0 as libc::c_int as size_t {} else {
            __assert_fail(
                b"max > 0\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                611 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"size_t trio_format_date_max(char *, size_t, const char *, const struct tm *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return strftime(target, max, format, datetime);
}
#[no_mangle]
pub unsafe extern "C" fn trio_hash(
    mut string: *const libc::c_char,
    mut type_0: libc::c_int,
) -> libc::c_ulong {
    let mut value: libc::c_ulong = 0 as libc::c_long as libc::c_ulong;
    let mut ch: libc::c_char = 0;
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            638 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"unsigned long trio_hash(const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5448: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                638 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"unsigned long trio_hash(const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    match type_0 {
        1 => {
            loop {
                let fresh0 = string;
                string = string.offset(1);
                ch = *fresh0;
                if !(ch as libc::c_int
                    != 0 as libc::c_int as libc::c_char as libc::c_int)
                {
                    break;
                }
                value = value.wrapping_mul(31 as libc::c_int as libc::c_ulong);
                value = value.wrapping_add(ch as libc::c_ulong);
            }
        }
        _ => {
            if 1 as libc::c_int == 0 as libc::c_int {} else {
                __assert_fail(
                    b"FALSE\0" as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                        as *const u8 as *const libc::c_char,
                    650 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"unsigned long trio_hash(const char *, int)\0"))
                        .as_ptr(),
                );
            }
            'c_5381: {
                if 1 as libc::c_int == 0 as libc::c_int {} else {
                    __assert_fail(
                        b"FALSE\0" as *const u8 as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                            as *const u8 as *const libc::c_char,
                        650 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 43],
                            &[libc::c_char; 43],
                        >(b"unsigned long trio_hash(const char *, int)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn trio_index(
    mut string: *const libc::c_char,
    mut character: libc::c_int,
) -> *mut libc::c_char {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            672 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"char *trio_index(const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5494: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                672 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *trio_index(const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return strchr(string, character);
}
#[no_mangle]
pub unsafe extern "C" fn trio_index_last(
    mut string: *const libc::c_char,
    mut character: libc::c_int,
) -> *mut libc::c_char {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            693 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"char *trio_index_last(const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5539: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                693 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"char *trio_index_last(const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return strchr(string, character);
}
#[no_mangle]
pub unsafe extern "C" fn trio_lower(mut target: *mut libc::c_char) -> libc::c_int {
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            712 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int trio_lower(char *)\0"))
                .as_ptr(),
        );
    }
    'c_5742: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                712 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int trio_lower(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_span_function(
        target,
        target,
        Some(trio_to_lower as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_match(
    mut string: *const libc::c_char,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            739 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int trio_match(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5928: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                739 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"int trio_match(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pattern.is_null() {} else {
        __assert_fail(
            b"pattern\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            740 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int trio_match(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5896: {
        if !pattern.is_null() {} else {
            __assert_fail(
                b"pattern\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                740 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"int trio_match(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    while '*' as i32 != *pattern as libc::c_int {
        if 0 as libc::c_int as libc::c_char as libc::c_int == *string as libc::c_int {
            return (0 as libc::c_int as libc::c_char as libc::c_int
                == *pattern as libc::c_int) as libc::c_int;
        }
        if trio_to_upper(*string as libc::c_int)
            != trio_to_upper(*pattern as libc::c_int)
            && '?' as i32 != *pattern as libc::c_int
        {
            return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
        pattern = pattern.offset(1);
        pattern;
        string = string.offset(1);
        string;
    }
    while '*' as i32 == *pattern.offset(1 as libc::c_int as isize) as libc::c_int {
        pattern = pattern.offset(1);
        pattern;
    }
    loop {
        if trio_match(string, &*pattern.offset(1 as libc::c_int as isize)) != 0 {
            return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
        let fresh1 = string;
        string = string.offset(1);
        if !(*fresh1 != 0) {
            break;
        }
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_match_case(
    mut string: *const libc::c_char,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            792 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_match_case(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6107: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                792 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_match_case(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pattern.is_null() {} else {
        __assert_fail(
            b"pattern\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            793 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"int trio_match_case(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6075: {
        if !pattern.is_null() {} else {
            __assert_fail(
                b"pattern\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                793 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"int trio_match_case(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    while '*' as i32 != *pattern as libc::c_int {
        if 0 as libc::c_int as libc::c_char as libc::c_int == *string as libc::c_int {
            return (0 as libc::c_int as libc::c_char as libc::c_int
                == *pattern as libc::c_int) as libc::c_int;
        }
        if *string as libc::c_int != *pattern as libc::c_int
            && '?' as i32 != *pattern as libc::c_int
        {
            return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
        pattern = pattern.offset(1);
        pattern;
        string = string.offset(1);
        string;
    }
    while '*' as i32 == *pattern.offset(1 as libc::c_int as isize) as libc::c_int {
        pattern = pattern.offset(1);
        pattern;
    }
    loop {
        if trio_match_case(string, &*pattern.offset(1 as libc::c_int as isize)) != 0 {
            return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
        let fresh2 = string;
        string = string.offset(1);
        if !(*fresh2 != 0) {
            break;
        }
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_span_function(
    mut target: *mut libc::c_char,
    mut source: *const libc::c_char,
    mut Function: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) -> size_t {
    let mut count: size_t = 0 as libc::c_int as size_t;
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            843 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"size_t trio_span_function(char *, const char *, int (*)(int))\0"))
                .as_ptr(),
        );
    }
    'c_5710: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                843 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"size_t trio_span_function(char *, const char *, int (*)(int))\0"))
                    .as_ptr(),
            );
        }
    };
    if !source.is_null() {} else {
        __assert_fail(
            b"source\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            844 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"size_t trio_span_function(char *, const char *, int (*)(int))\0"))
                .as_ptr(),
        );
    }
    'c_5678: {
        if !source.is_null() {} else {
            __assert_fail(
                b"source\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                844 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"size_t trio_span_function(char *, const char *, int (*)(int))\0"))
                    .as_ptr(),
            );
        }
    };
    if Function.is_some() {} else {
        __assert_fail(
            b"Function\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            845 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"size_t trio_span_function(char *, const char *, int (*)(int))\0"))
                .as_ptr(),
        );
    }
    'c_5645: {
        if Function.is_some() {} else {
            __assert_fail(
                b"Function\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                845 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"size_t trio_span_function(char *, const char *, int (*)(int))\0"))
                    .as_ptr(),
            );
        }
    };
    while *source as libc::c_int != 0 as libc::c_int as libc::c_char as libc::c_int {
        let fresh3 = source;
        source = source.offset(1);
        let fresh4 = target;
        target = target.offset(1);
        *fresh4 = Function.expect("non-null function pointer")(*fresh3 as libc::c_int)
            as libc::c_char;
        count = count.wrapping_add(1);
        count;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn trio_substring(
    mut string: *const libc::c_char,
    mut substring: *const libc::c_char,
) -> *mut libc::c_char {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            872 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"char *trio_substring(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6186: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                872 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"char *trio_substring(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !substring.is_null() {} else {
        __assert_fail(
            b"substring\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            873 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"char *trio_substring(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6153: {
        if !substring.is_null() {} else {
            __assert_fail(
                b"substring\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                873 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"char *trio_substring(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return strstr(string, substring);
}
#[no_mangle]
pub unsafe extern "C" fn trio_substring_max(
    mut string: *const libc::c_char,
    mut max: size_t,
    mut substring: *const libc::c_char,
) -> *mut libc::c_char {
    let mut count: size_t = 0;
    let mut size: size_t = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            901 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"char *trio_substring_max(const char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6331: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                901 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"char *trio_substring_max(const char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !substring.is_null() {} else {
        __assert_fail(
            b"substring\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            902 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"char *trio_substring_max(const char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6298: {
        if !substring.is_null() {} else {
            __assert_fail(
                b"substring\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                902 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"char *trio_substring_max(const char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    size = trio_length(substring);
    if size <= max {
        count = 0 as libc::c_int as size_t;
        while count <= max.wrapping_sub(size) {
            if trio_equal_max(substring, size, &*string.offset(count as isize)) != 0 {
                result = &*string.offset(count as isize) as *const libc::c_char
                    as *mut libc::c_char;
                break;
            } else {
                count = count.wrapping_add(1);
                count;
            }
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_tokenize(
    mut string: *mut libc::c_char,
    mut delimiters: *const libc::c_char,
) -> *mut libc::c_char {
    if !delimiters.is_null() {} else {
        __assert_fail(
            b"delimiters\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            937 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"char *trio_tokenize(char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6490: {
        if !delimiters.is_null() {} else {
            __assert_fail(
                b"delimiters\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                937 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"char *trio_tokenize(char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return strtok(string, delimiters);
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_long_double(
    mut source: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
) -> trio_long_double_t {
    let mut isNegative: libc::c_int = (1 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
    let mut isExponentNegative: libc::c_int = (1 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
    let mut integer: trio_long_double_t = f128::f128::new(0.0f64);
    let mut fraction: trio_long_double_t = f128::f128::new(0.0f64);
    let mut exponent: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut base: trio_long_double_t = 0.;
    let mut fracdiv: trio_long_double_t = f128::f128::new(1.0f64);
    let mut value: trio_long_double_t = f128::f128::new(0.0f64);
    if *source.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && (*source.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *source.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
    {
        base = f128::f128::new(16.0f64);
        source = source.offset(2 as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*source as libc::c_int as isize) as libc::c_int
            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            integer *= base;
            integer
                += f128::f128::new(
                    if *(*__ctype_b_loc()).offset(*source as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        *source as libc::c_int - '0' as i32
                    } else {
                        10 as libc::c_int
                            + (trio_to_upper(*source as libc::c_int) - 'A' as i32)
                    },
                );
            source = source.offset(1);
            source;
        }
        if *source as libc::c_int == '.' as i32 {
            source = source.offset(1);
            source;
            while *(*__ctype_b_loc()).offset(*source as libc::c_int as isize)
                as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                fracdiv /= base;
                fraction
                    += fracdiv
                        * f128::f128::new(
                            (if *(*__ctype_b_loc())
                                .offset(*source as libc::c_int as isize) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                *source as libc::c_int - '0' as i32
                            } else {
                                10 as libc::c_int
                                    + (trio_to_upper(*source as libc::c_int) - 'A' as i32)
                            }),
                        );
                source = source.offset(1);
                source;
            }
            if *source as libc::c_int == 'p' as i32
                || *source as libc::c_int == 'P' as i32
            {
                source = source.offset(1);
                source;
                if *source as libc::c_int == '+' as i32
                    || *source as libc::c_int == '-' as i32
                {
                    isExponentNegative = (*source as libc::c_int == '-' as i32)
                        as libc::c_int;
                    source = source.offset(1);
                    source;
                }
                while *(*__ctype_b_loc()).offset(*source as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    exponent = exponent.wrapping_mul(10 as libc::c_int as libc::c_ulong);
                    exponent = exponent
                        .wrapping_add(
                            (*source as libc::c_int - '0' as i32) as libc::c_ulong,
                        );
                    source = source.offset(1);
                    source;
                }
            }
        }
        base = f128::f128::new(2.0f64);
    } else {
        base = f128::f128::new(10.0f64);
        isNegative = (*source as libc::c_int == '-' as i32) as libc::c_int;
        if *source as libc::c_int == '+' as i32 || *source as libc::c_int == '-' as i32 {
            source = source.offset(1);
            source;
        }
        while *(*__ctype_b_loc()).offset(*source as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            integer *= base;
            integer += f128::f128::new(*source as libc::c_int - '0' as i32);
            source = source.offset(1);
            source;
        }
        if *source as libc::c_int == '.' as i32 {
            source = source.offset(1);
            source;
            while *(*__ctype_b_loc()).offset(*source as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                fracdiv /= base;
                fraction
                    += f128::f128::new(*source as libc::c_int - '0' as i32) * fracdiv;
                source = source.offset(1);
                source;
            }
        }
        if *source as libc::c_int == 'e' as i32 || *source as libc::c_int == 'E' as i32
            || *source as libc::c_int == 'd' as i32
            || *source as libc::c_int == 'D' as i32
        {
            source = source.offset(1);
            source;
            isExponentNegative = (*source as libc::c_int == '-' as i32) as libc::c_int;
            if *source as libc::c_int == '+' as i32
                || *source as libc::c_int == '-' as i32
            {
                source = source.offset(1);
                source;
            }
            while *(*__ctype_b_loc()).offset(*source as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                exponent = exponent.wrapping_mul(base as libc::c_int as libc::c_ulong);
                exponent = exponent
                    .wrapping_add(
                        (*source as libc::c_int - '0' as i32) as libc::c_ulong,
                    );
                source = source.offset(1);
                source;
            }
        }
    }
    value = integer + fraction;
    if exponent != 0 as libc::c_int as libc::c_ulong {
        if isExponentNegative != 0 {
            value
                /= f128::f128::new(
                    pow(base as libc::c_double, exponent as libc::c_double),
                );
        } else {
            value
                *= f128::f128::new(
                    pow(base as libc::c_double, exponent as libc::c_double),
                );
        }
    }
    if isNegative != 0 {
        value = -value;
    }
    if !endp.is_null() {
        *endp = source as *mut libc::c_char;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_double(
    mut source: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
) -> libc::c_double {
    return strtod(source, endp);
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_float(
    mut source: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
) -> libc::c_float {
    return strtof(source, endp);
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_long(
    mut string: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_long {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1153 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"long trio_to_long(const char *, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_3877: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1153 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"long trio_to_long(const char *, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if base >= 2 as libc::c_int && base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"(base >= 2) && (base <= 36)\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"long trio_to_long(const char *, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_3825: {
        if base >= 2 as libc::c_int && base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"(base >= 2) && (base <= 36)\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1154 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"long trio_to_long(const char *, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return strtol(string, endp, base);
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_lower(mut source: libc::c_int) -> libc::c_int {
    return tolower(source);
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_unsigned_long(
    mut string: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_ulong {
    if !string.is_null() {} else {
        __assert_fail(
            b"string\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1202 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"unsigned long trio_to_unsigned_long(const char *, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_6445: {
        if !string.is_null() {} else {
            __assert_fail(
                b"string\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1202 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"unsigned long trio_to_unsigned_long(const char *, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if base >= 2 as libc::c_int && base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"(base >= 2) && (base <= 36)\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1203 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"unsigned long trio_to_unsigned_long(const char *, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_6394: {
        if base >= 2 as libc::c_int && base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"(base >= 2) && (base <= 36)\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1203 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"unsigned long trio_to_unsigned_long(const char *, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return strtoul(string, endp, base);
}
#[no_mangle]
pub unsafe extern "C" fn trio_to_upper(mut source: libc::c_int) -> libc::c_int {
    return toupper(source);
}
#[no_mangle]
pub unsafe extern "C" fn trio_upper(mut target: *mut libc::c_char) -> libc::c_int {
    if !target.is_null() {} else {
        __assert_fail(
            b"target\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1247 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int trio_upper(char *)\0"))
                .as_ptr(),
        );
    }
    'c_6540: {
        if !target.is_null() {} else {
            __assert_fail(
                b"target\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1247 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int trio_upper(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_span_function(
        target,
        target,
        Some(trio_to_upper as unsafe extern "C" fn(libc::c_int) -> libc::c_int),
    ) as libc::c_int;
}
unsafe extern "C" fn TrioStringAlloc() -> *mut trio_string_t {
    let mut self_0: *mut trio_string_t = 0 as *mut trio_string_t;
    self_0 = malloc(::core::mem::size_of::<trio_string_t>() as libc::c_ulong)
        as *mut trio_string_t;
    if !self_0.is_null() {
        (*self_0).content = 0 as *mut libc::c_char;
        (*self_0).length = 0 as libc::c_int as size_t;
        (*self_0).allocated = 0 as libc::c_int as size_t;
    }
    return self_0;
}
unsafe extern "C" fn TrioStringGrow(
    mut self_0: *mut trio_string_t,
    mut delta: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    let mut new_content: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_size: size_t = 0;
    new_size = if delta == 0 as libc::c_int as size_t {
        if (*self_0).allocated == 0 as libc::c_int as size_t {
            1 as libc::c_int as size_t
        } else {
            (*self_0).allocated * 2 as libc::c_int as size_t
        }
    } else {
        ((*self_0).allocated).wrapping_add(delta)
    };
    new_content = realloc((*self_0).content as *mut libc::c_void, new_size)
        as *mut libc::c_char;
    if !new_content.is_null() {
        (*self_0).content = new_content;
        (*self_0).allocated = new_size;
        status = !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    }
    return status;
}
unsafe extern "C" fn TrioStringGrowTo(
    mut self_0: *mut trio_string_t,
    mut length: size_t,
) -> libc::c_int {
    length = length.wrapping_add(1);
    length;
    return if (*self_0).allocated < length {
        TrioStringGrow(self_0, length.wrapping_sub((*self_0).allocated))
    } else {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_create(
    mut initial_size: libc::c_int,
) -> *mut trio_string_t {
    let mut self_0: *mut trio_string_t = 0 as *mut trio_string_t;
    self_0 = TrioStringAlloc();
    if !self_0.is_null() {
        if TrioStringGrow(
            self_0,
            (if initial_size > 0 as libc::c_int {
                initial_size
            } else {
                1 as libc::c_int
            }) as size_t,
        ) != 0
        {
            *((*self_0).content)
                .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            (*self_0).allocated = initial_size as size_t;
        } else {
            trio_string_destroy(self_0);
            self_0 = 0 as *mut trio_string_t;
        }
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_destroy(mut self_0: *mut trio_string_t) {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1384 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void trio_string_destroy(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6607: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1384 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void trio_string_destroy(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !self_0.is_null() {
        trio_destroy((*self_0).content);
        free(self_0 as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_get(
    mut self_0: *mut trio_string_t,
    mut offset: libc::c_int,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1417 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"char *trio_string_get(trio_string_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_7346: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1417 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"char *trio_string_get(trio_string_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*self_0).content).is_null() {
        if (*self_0).length == 0 as libc::c_int as size_t {
            trio_string_length(self_0);
        }
        if offset >= 0 as libc::c_int {
            if offset > (*self_0).length as libc::c_int {
                offset = (*self_0).length as libc::c_int;
            }
        } else {
            offset = (offset as size_t)
                .wrapping_add(
                    ((*self_0).length).wrapping_add(1 as libc::c_int as size_t),
                ) as libc::c_int as libc::c_int;
            if offset < 0 as libc::c_int {
                offset = 0 as libc::c_int;
            }
        }
        result = &mut *((*self_0).content).offset(offset as isize) as *mut libc::c_char;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_extract(
    mut self_0: *mut trio_string_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1463 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"char *trio_string_extract(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6677: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1463 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"char *trio_string_extract(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    result = (*self_0).content;
    (*self_0).content = 0 as *mut libc::c_char;
    (*self_0).allocated = 0 as libc::c_int as size_t;
    (*self_0).length = (*self_0).allocated;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_set(
    mut self_0: *mut trio_string_t,
    mut buffer: *mut libc::c_char,
) {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1493 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void trio_xstring_set(trio_string_t *, char *)\0"))
                .as_ptr(),
        );
    }
    'c_7405: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1493 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void trio_xstring_set(trio_string_t *, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    trio_destroy((*self_0).content);
    (*self_0).content = trio_duplicate(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_size(
    mut self_0: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1509 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int trio_string_size(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6720: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1509 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int trio_string_size(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (*self_0).allocated as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_terminate(mut self_0: *mut trio_string_t) {
    trio_xstring_append_char(self_0, 0 as libc::c_int as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_append(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    let mut length: size_t = 0;
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1543 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int trio_string_append(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7576: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1543 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int trio_string_append(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1544 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int trio_string_append(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7543: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1544 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int trio_string_append(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    length = ((*self_0).length).wrapping_add((*other).length);
    if TrioStringGrowTo(self_0, length) == 0 {
        return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        trio_copy(
            &mut *((*self_0).content).offset((*self_0).length as isize),
            (*other).content,
        );
        (*self_0).length = length;
        return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_append(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    let mut length: size_t = 0;
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1571 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int trio_xstring_append(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_8829: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1571 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int trio_xstring_append(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1572 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int trio_xstring_append(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_8797: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1572 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int trio_xstring_append(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    length = ((*self_0).length).wrapping_add(trio_length(other));
    if TrioStringGrowTo(self_0, length) == 0 {
        return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        trio_copy(&mut *((*self_0).content).offset((*self_0).length as isize), other);
        (*self_0).length = length;
        return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_append_char(
    mut self_0: *mut trio_string_t,
    mut character: libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1596 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"int trio_xstring_append_char(trio_string_t *, char)\0"))
                .as_ptr(),
        );
    }
    'c_6925: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1596 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"int trio_xstring_append_char(trio_string_t *, char)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*self_0).length as libc::c_int >= trio_string_size(self_0) {
        if TrioStringGrow(self_0, 0 as libc::c_int as size_t) == 0 {
            return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
    }
    *((*self_0).content).offset((*self_0).length as isize) = character;
    (*self_0).length = ((*self_0).length).wrapping_add(1);
    (*self_0).length;
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_contains(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1626 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int trio_string_contains(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7661: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1626 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int trio_string_contains(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1627 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int trio_string_contains(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7628: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1627 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int trio_string_contains(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_contains((*self_0).content, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_contains(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1644 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int trio_xstring_contains(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_8910: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1644 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int trio_xstring_contains(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1645 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"int trio_xstring_contains(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_8878: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1645 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int trio_xstring_contains(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_contains((*self_0).content, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_copy(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1662 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int trio_string_copy(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7747: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1662 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int trio_string_copy(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1663 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int trio_string_copy(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7715: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1663 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int trio_string_copy(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*self_0).length = 0 as libc::c_int as size_t;
    return trio_string_append(self_0, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_copy(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1681 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int trio_xstring_copy(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_8997: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1681 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"int trio_xstring_copy(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1682 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"int trio_xstring_copy(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_8964: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1682 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"int trio_xstring_copy(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*self_0).length = 0 as libc::c_int as size_t;
    return trio_xstring_append(self_0, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_duplicate(
    mut other: *mut trio_string_t,
) -> *mut trio_string_t {
    let mut self_0: *mut trio_string_t = 0 as *mut trio_string_t;
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1701 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"trio_string_t *trio_string_duplicate(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7855: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1701 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"trio_string_t *trio_string_duplicate(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    self_0 = TrioStringAlloc();
    if !self_0.is_null() {
        (*self_0).content = TrioDuplicateMax((*other).content, (*other).length);
        if !((*self_0).content).is_null() {
            (*self_0).length = (*other).length;
            (*self_0)
                .allocated = ((*self_0).length).wrapping_add(1 as libc::c_int as size_t);
        } else {
            (*self_0).allocated = 0 as libc::c_int as size_t;
            (*self_0).length = (*self_0).allocated;
        }
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_duplicate(
    mut other: *const libc::c_char,
) -> *mut trio_string_t {
    let mut self_0: *mut trio_string_t = 0 as *mut trio_string_t;
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1732 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"trio_string_t *trio_xstring_duplicate(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_7080: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1732 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"trio_string_t *trio_xstring_duplicate(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    self_0 = TrioStringAlloc();
    if !self_0.is_null() {
        (*self_0).content = TrioDuplicateMax(other, trio_length(other));
        if !((*self_0).content).is_null() {
            (*self_0).length = trio_length((*self_0).content);
            (*self_0)
                .allocated = ((*self_0).length).wrapping_add(1 as libc::c_int as size_t);
        } else {
            (*self_0).allocated = 0 as libc::c_int as size_t;
            (*self_0).length = (*self_0).allocated;
        }
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_equal(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1762 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int trio_string_equal(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7941: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1762 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int trio_string_equal(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1763 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int trio_string_equal(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7908: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1763 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int trio_string_equal(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_equal((*self_0).content, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_equal(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1780 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int trio_xstring_equal(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9078: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1780 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int trio_xstring_equal(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1781 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int trio_xstring_equal(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9046: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1781 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int trio_xstring_equal(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_equal((*self_0).content, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_equal_max(
    mut self_0: *mut trio_string_t,
    mut max: size_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1799 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int trio_string_equal_max(trio_string_t *, size_t, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8030: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1799 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"int trio_string_equal_max(trio_string_t *, size_t, trio_string_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1800 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"int trio_string_equal_max(trio_string_t *, size_t, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7997: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1800 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"int trio_string_equal_max(trio_string_t *, size_t, trio_string_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return trio_equal_max((*self_0).content, max, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_equal_max(
    mut self_0: *mut trio_string_t,
    mut max: size_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1818 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"int trio_xstring_equal_max(trio_string_t *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9164: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1818 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"int trio_xstring_equal_max(trio_string_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1819 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"int trio_xstring_equal_max(trio_string_t *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9131: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1819 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"int trio_xstring_equal_max(trio_string_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return trio_equal_max((*self_0).content, max, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_equal_case(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1836 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"int trio_string_equal_case(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8114: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1836 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int trio_string_equal_case(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1837 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"int trio_string_equal_case(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8082: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1837 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int trio_string_equal_case(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_equal_case((*self_0).content, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_equal_case(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1854 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int trio_xstring_equal_case(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9245: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1854 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int trio_xstring_equal_case(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1855 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int trio_xstring_equal_case(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9213: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1855 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int trio_xstring_equal_case(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_equal_case((*self_0).content, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_equal_case_max(
    mut self_0: *mut trio_string_t,
    mut max: size_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1873 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int trio_string_equal_case_max(trio_string_t *, size_t, trio_string_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8202: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1873 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"int trio_string_equal_case_max(trio_string_t *, size_t, trio_string_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1874 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int trio_string_equal_case_max(trio_string_t *, size_t, trio_string_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8169: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1874 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"int trio_string_equal_case_max(trio_string_t *, size_t, trio_string_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return trio_equal_case_max((*self_0).content, max, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_equal_case_max(
    mut self_0: *mut trio_string_t,
    mut max: size_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1892 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int trio_xstring_equal_case_max(trio_string_t *, size_t, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9330: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1892 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"int trio_xstring_equal_case_max(trio_string_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1893 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int trio_xstring_equal_case_max(trio_string_t *, size_t, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9297: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1893 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"int trio_xstring_equal_case_max(trio_string_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return trio_equal_case_max((*self_0).content, max, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_format_date_max(
    mut self_0: *mut trio_string_t,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut datetime: *const tm,
) -> size_t {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1912 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 93],
                &[libc::c_char; 93],
            >(
                b"size_t trio_string_format_date_max(trio_string_t *, size_t, const char *, const struct tm *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8257: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1912 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 93],
                    &[libc::c_char; 93],
                >(
                    b"size_t trio_string_format_date_max(trio_string_t *, size_t, const char *, const struct tm *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return trio_format_date_max((*self_0).content, max, format, datetime);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_index(
    mut self_0: *mut trio_string_t,
    mut character: libc::c_int,
) -> *mut libc::c_char {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1929 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"char *trio_string_index(trio_string_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_8305: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1929 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"char *trio_string_index(trio_string_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_index((*self_0).content, character);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_index_last(
    mut self_0: *mut trio_string_t,
    mut character: libc::c_int,
) -> *mut libc::c_char {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1946 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"char *trio_string_index_last(trio_string_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_8353: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1946 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"char *trio_string_index_last(trio_string_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_index_last((*self_0).content, character);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_length(
    mut self_0: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1962 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"int trio_string_length(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7299: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1962 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"int trio_string_length(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*self_0).length == 0 as libc::c_int as size_t {
        (*self_0).length = trio_length((*self_0).content);
    }
    return (*self_0).length as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_lower(
    mut self_0: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1982 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int trio_string_lower(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8399: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1982 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"int trio_string_lower(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_lower((*self_0).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_match(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            1999 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int trio_string_match(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8484: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                1999 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int trio_string_match(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2000 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int trio_string_match(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8452: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2000 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int trio_string_match(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_match((*self_0).content, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_match(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2017 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int trio_xstring_match(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9411: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2017 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int trio_xstring_match(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2018 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int trio_xstring_match(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9379: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2018 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int trio_xstring_match(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_match((*self_0).content, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_match_case(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2035 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"int trio_string_match_case(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8568: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2035 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int trio_string_match_case(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2036 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"int trio_string_match_case(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8536: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2036 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int trio_string_match_case(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_match_case((*self_0).content, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_match_case(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2053 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int trio_xstring_match_case(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9492: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2053 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int trio_xstring_match_case(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2054 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int trio_xstring_match_case(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9460: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2054 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int trio_xstring_match_case(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_match_case((*self_0).content, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_substring(
    mut self_0: *mut trio_string_t,
    mut other: *mut trio_string_t,
) -> *mut libc::c_char {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2071 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"char *trio_string_substring(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8653: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2071 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"char *trio_string_substring(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2072 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"char *trio_string_substring(trio_string_t *, trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8621: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2072 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"char *trio_string_substring(trio_string_t *, trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_substring((*self_0).content, (*other).content);
}
#[no_mangle]
pub unsafe extern "C" fn trio_xstring_substring(
    mut self_0: *mut trio_string_t,
    mut other: *const libc::c_char,
) -> *mut libc::c_char {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2089 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"char *trio_xstring_substring(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9574: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2089 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"char *trio_xstring_substring(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !other.is_null() {} else {
        __assert_fail(
            b"other\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2090 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"char *trio_xstring_substring(trio_string_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_9542: {
        if !other.is_null() {} else {
            __assert_fail(
                b"other\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2090 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"char *trio_xstring_substring(trio_string_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_substring((*self_0).content, other);
}
#[no_mangle]
pub unsafe extern "C" fn trio_string_upper(
    mut self_0: *mut trio_string_t,
) -> libc::c_int {
    if !self_0.is_null() {} else {
        __assert_fail(
            b"self\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                as *const u8 as *const libc::c_char,
            2106 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int trio_string_upper(trio_string_t *)\0"))
                .as_ptr(),
        );
    }
    'c_8697: {
        if !self_0.is_null() {} else {
            __assert_fail(
                b"self\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/c2rust/examples/libxml2/repo/triostr.c\0"
                    as *const u8 as *const libc::c_char,
                2106 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"int trio_string_upper(trio_string_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return trio_upper((*self_0).content);
}
