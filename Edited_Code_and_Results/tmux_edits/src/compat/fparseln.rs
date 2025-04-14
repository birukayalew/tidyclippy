use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fgetln(_: *mut FILE, _: *mut size_t) -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn isescaped(
    mut sp: *const libc::c_char,
    mut p: *const libc::c_char,
    mut esc: libc::c_int,
) -> libc::c_int {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut ne: size_t = 0;
    if esc == '\0' as i32 {
        return 1 as libc::c_int;
    }
    ne = 0 as libc::c_int as size_t;
    cp = p;
    loop {
        cp = cp.offset(-1);
        if !(cp >= sp && *cp as libc::c_int == esc) {
            break;
        }
        ne = ne.wrapping_add(1);
        ne;
    }
    return (ne & 1 as libc::c_int as size_t != 0 as libc::c_int as size_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fparseln(
    mut fp: *mut FILE,
    mut size: *mut size_t,
    mut lineno: *mut size_t,
    mut str: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    static mut dstr: [libc::c_char; 3] = [
        '\\' as i32 as libc::c_char,
        '\\' as i32 as libc::c_char,
        '#' as i32 as libc::c_char,
    ];
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut esc: libc::c_char = 0;
    let mut con: libc::c_char = 0;
    let mut nl: libc::c_char = 0;
    let mut com: libc::c_char = 0;
    let mut s: size_t = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut cnt: libc::c_int = 1 as libc::c_int;
    if str.is_null() {
        str = dstr.as_ptr();
    }
    esc = *str.offset(0 as libc::c_int as isize);
    con = *str.offset(1 as libc::c_int as isize);
    com = *str.offset(2 as libc::c_int as isize);
    nl = '\n' as i32 as libc::c_char;
    while cnt != 0 {
        cnt = 0 as libc::c_int;
        if !lineno.is_null() {
            *lineno = (*lineno).wrapping_add(1);
            *lineno;
        }
        ptr = fgetln(fp, &mut s);
        if ptr.is_null() {
            break;
        }
        if s != 0 && com as libc::c_int != 0 {
            cp = ptr;
            while cp < ptr.offset(s as isize) {
                if *cp as libc::c_int == com as libc::c_int
                    && isescaped(ptr, cp, esc as libc::c_int) == 0
                {
                    s = cp.offset_from(ptr) as libc::c_long as size_t;
                    cnt = (s == 0 as libc::c_int as size_t && buf.is_null())
                        as libc::c_int;
                    break;
                } else {
                    cp = cp.offset(1);
                    cp;
                }
            }
        }
        if s != 0 && nl as libc::c_int != 0 {
            cp = &mut *ptr.offset(s.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as *mut libc::c_char;
            if *cp as libc::c_int == nl as libc::c_int {
                s = s.wrapping_sub(1);
                s;
            }
        }
        if s != 0 && con as libc::c_int != 0 {
            cp = &mut *ptr.offset(s.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as *mut libc::c_char;
            if *cp as libc::c_int == con as libc::c_int
                && isescaped(ptr, cp, esc as libc::c_int) == 0
            {
                s = s.wrapping_sub(1);
                s;
                cnt = 1 as libc::c_int;
            }
        }
        if s == 0 as libc::c_int as size_t && !buf.is_null() {
            continue;
        }
        cp = realloc(
            buf as *mut libc::c_void,
            len.wrapping_add(s).wrapping_add(1 as libc::c_int as size_t),
        ) as *mut libc::c_char;
        if cp.is_null() {
            free(buf as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        buf = cp;
        memcpy(
            buf.offset(len as isize) as *mut libc::c_void,
            ptr as *const libc::c_void,
            s,
        );
        len = len.wrapping_add(s);
        *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    if flags & 0xf as libc::c_int != 0 as libc::c_int && esc as libc::c_int != 0
        && !buf.is_null() && !(strchr(buf, esc as libc::c_int)).is_null()
    {
        cp = buf;
        ptr = cp;
        while *cp.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            let mut skipesc: libc::c_int = 0;
            while *cp.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
                && *cp.offset(0 as libc::c_int as isize) as libc::c_int
                    != esc as libc::c_int
            {
                let fresh0 = cp;
                cp = cp.offset(1);
                let fresh1 = ptr;
                ptr = ptr.offset(1);
                *fresh1 = *fresh0;
            }
            if *cp.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                || *cp.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                break;
            }
            skipesc = 0 as libc::c_int;
            if *cp.offset(1 as libc::c_int as isize) as libc::c_int == com as libc::c_int
            {
                skipesc += flags & 0x4 as libc::c_int;
            }
            if *cp.offset(1 as libc::c_int as isize) as libc::c_int == con as libc::c_int
            {
                skipesc += flags & 0x2 as libc::c_int;
            }
            if *cp.offset(1 as libc::c_int as isize) as libc::c_int == esc as libc::c_int
            {
                skipesc += flags & 0x1 as libc::c_int;
            }
            if *cp.offset(1 as libc::c_int as isize) as libc::c_int != com as libc::c_int
                && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                    != con as libc::c_int
                && *cp.offset(1 as libc::c_int as isize) as libc::c_int
                    != esc as libc::c_int
            {
                skipesc = flags & 0x8 as libc::c_int;
            }
            if skipesc != 0 {
                cp = cp.offset(1);
                cp;
            } else {
                let fresh2 = cp;
                cp = cp.offset(1);
                let fresh3 = ptr;
                ptr = ptr.offset(1);
                *fresh3 = *fresh2;
            }
            let fresh4 = cp;
            cp = cp.offset(1);
            let fresh5 = ptr;
            ptr = ptr.offset(1);
            *fresh5 = *fresh4;
        }
        *ptr = '\0' as i32 as libc::c_char;
        len = strlen(buf);
    }
    if !size.is_null() {
        *size = len;
    }
    return buf;
}
