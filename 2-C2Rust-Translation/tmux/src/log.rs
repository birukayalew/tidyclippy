use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn event_set_log_callback(cb: event_log_cb);
    fn stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn log_debug(_: *const libc::c_char, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type event_log_cb = Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_char) -> (),
>;
static mut log_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut log_level: libc::c_int = 0;
unsafe extern "C" fn log_event_cb(
    mut severity: libc::c_int,
    mut msg: *const libc::c_char,
) {
    log_debug(b"%s\0" as *const u8 as *const libc::c_char, msg);
}
#[no_mangle]
pub unsafe extern "C" fn log_add_level() {
    log_level += 1;
    log_level;
}
#[no_mangle]
pub unsafe extern "C" fn log_get_level() -> libc::c_int {
    return log_level;
}
#[no_mangle]
pub unsafe extern "C" fn log_open(mut name: *const libc::c_char) {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    if log_level == 0 as libc::c_int {
        return;
    }
    log_close();
    xasprintf(
        &mut path as *mut *mut libc::c_char,
        b"tmux-%s-%ld.log\0" as *const u8 as *const libc::c_char,
        name,
        getpid() as libc::c_long,
    );
    log_file = fopen(path, b"a\0" as *const u8 as *const libc::c_char);
    free(path as *mut libc::c_void);
    if log_file.is_null() {
        return;
    }
    setvbuf(
        log_file,
        0 as *mut libc::c_char,
        1 as libc::c_int,
        0 as libc::c_int as size_t,
    );
    event_set_log_callback(
        Some(
            log_event_cb as unsafe extern "C" fn(libc::c_int, *const libc::c_char) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn log_toggle(mut name: *const libc::c_char) {
    if log_level == 0 as libc::c_int {
        log_level = 1 as libc::c_int;
        log_open(name);
        log_debug(b"log opened\0" as *const u8 as *const libc::c_char);
    } else {
        log_debug(b"log closed\0" as *const u8 as *const libc::c_char);
        log_level = 0 as libc::c_int;
        log_close();
    };
}
#[no_mangle]
pub unsafe extern "C" fn log_close() {
    if !log_file.is_null() {
        fclose(log_file);
    }
    log_file = 0 as *mut FILE;
    event_set_log_callback(None);
}
#[no_mangle]
pub unsafe extern "C" fn log_vwrite(
    mut msg: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if log_file.is_null() {
        return;
    }
    if vasprintf(&mut fmt, msg, ap.as_va_list()) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    if stravis(
        &mut out,
        fmt,
        0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        exit(1 as libc::c_int);
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    if fprintf(
        log_file,
        b"%lld.%06d %s\n\0" as *const u8 as *const libc::c_char,
        tv.tv_sec as libc::c_longlong,
        tv.tv_usec as libc::c_int,
        out,
    ) == -(1 as libc::c_int)
    {
        exit(1 as libc::c_int);
    }
    fflush(log_file);
    free(out as *mut libc::c_void);
    free(fmt as *mut libc::c_void);
}
