use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[no_mangle]
pub unsafe extern "C" fn fgetln(
    mut fp: *mut FILE,
    mut len: *mut size_t,
) -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: size_t = 0 as libc::c_int as size_t;
    let mut r: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    if fp.is_null() || len.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    if buf.is_null() {
        buf = calloc(
            1 as libc::c_int as libc::c_ulong,
            8192 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        if buf.is_null() {
            return 0 as *mut libc::c_char;
        }
        bufsz = 8192 as libc::c_int as size_t;
    }
    loop {
        c = getc(fp);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let fresh0 = r;
        r = r.wrapping_add(1);
        *buf.offset(fresh0 as isize) = c as libc::c_char;
        if r == bufsz {
            p = reallocarray(buf as *mut libc::c_void, 2 as libc::c_int as size_t, bufsz)
                as *mut libc::c_char;
            if p.is_null() {
                e = *__errno_location();
                free(buf as *mut libc::c_void);
                *__errno_location() = e;
                buf = 0 as *mut libc::c_char;
                bufsz = 0 as libc::c_int as size_t;
                return 0 as *mut libc::c_char;
            }
            buf = p;
            bufsz = 2 as libc::c_int as size_t * bufsz;
        }
        if c == '\n' as i32 {
            break;
        }
    }
    *len = r;
    return if *len != 0 { buf } else { 0 as *mut libc::c_char };
}
