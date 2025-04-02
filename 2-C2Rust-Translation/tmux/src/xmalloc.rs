use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as size_t {
        fatalx(b"xmalloc: zero size\0" as *const u8 as *const libc::c_char);
    }
    ptr = malloc(size);
    if ptr.is_null() {
        fatalx(
            b"xmalloc: allocating %zu bytes: %s\0" as *const u8 as *const libc::c_char,
            size,
            strerror(*__errno_location()),
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as size_t || nmemb == 0 as libc::c_int as size_t {
        fatalx(b"xcalloc: zero size\0" as *const u8 as *const libc::c_char);
    }
    ptr = calloc(nmemb, size);
    if ptr.is_null() {
        fatalx(
            b"xcalloc: allocating %zu * %zu bytes: %s\0" as *const u8
                as *const libc::c_char,
            nmemb,
            size,
            strerror(*__errno_location()),
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return xreallocarray(ptr, 1 as libc::c_int as size_t, size);
}
#[no_mangle]
pub unsafe extern "C" fn xreallocarray(
    mut ptr: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if nmemb == 0 as libc::c_int as size_t || size == 0 as libc::c_int as size_t {
        fatalx(b"xreallocarray: zero size\0" as *const u8 as *const libc::c_char);
    }
    new_ptr = reallocarray(ptr, nmemb, size);
    if new_ptr.is_null() {
        fatalx(
            b"xreallocarray: allocating %zu * %zu bytes: %s\0" as *const u8
                as *const libc::c_char,
            nmemb,
            size,
            strerror(*__errno_location()),
        );
    }
    return new_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strdup(str);
    if cp.is_null() {
        fatalx(
            b"xstrdup: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn xstrndup(
    mut str: *const libc::c_char,
    mut maxlen: size_t,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strndup(str, maxlen);
    if cp.is_null() {
        fatalx(
            b"xstrndup: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn xvasprintf(
    mut ret: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = vasprintf(ret, fmt, ap.as_va_list());
    if i < 0 as libc::c_int || (*ret).is_null() {
        fatalx(
            b"xasprintf: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn xvsnprintf(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if len > 2147483647 as libc::c_int as size_t {
        fatalx(b"xsnprintf: len > INT_MAX\0" as *const u8 as *const libc::c_char);
    }
    i = vsnprintf(str, len, fmt, ap.as_va_list());
    if i < 0 as libc::c_int || i >= len as libc::c_int {
        fatalx(b"xsnprintf: overflow\0" as *const u8 as *const libc::c_char);
    }
    return i;
}
