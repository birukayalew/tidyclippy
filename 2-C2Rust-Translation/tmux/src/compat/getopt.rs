use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getprogname() -> *const libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
#[no_mangle]
pub static mut BSDopterr: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut BSDoptind: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut BSDoptopt: libc::c_int = 0;
#[no_mangle]
pub static mut BSDoptreset: libc::c_int = 0;
#[no_mangle]
pub static mut BSDoptarg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn BSDgetopt(
    mut nargc: libc::c_int,
    mut nargv: *const *mut libc::c_char,
    mut ostr: *const libc::c_char,
) -> libc::c_int {
    static mut place: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut oli: *mut libc::c_char = 0 as *mut libc::c_char;
    if ostr.is_null() {
        return -(1 as libc::c_int);
    }
    if BSDoptreset != 0 || *place == 0 {
        BSDoptreset = 0 as libc::c_int;
        if BSDoptind >= nargc
            || {
                place = *nargv.offset(BSDoptind as isize);
                *place as libc::c_int != '-' as i32
            }
        {
            place = b"\0" as *const u8 as *const libc::c_char;
            return -(1 as libc::c_int);
        }
        if *place.offset(1 as libc::c_int as isize) as libc::c_int != 0
            && {
                place = place.offset(1);
                *place as libc::c_int == '-' as i32
            }
        {
            if *place.offset(1 as libc::c_int as isize) != 0 {
                return '?' as i32;
            }
            BSDoptind += 1;
            BSDoptind;
            place = b"\0" as *const u8 as *const libc::c_char;
            return -(1 as libc::c_int);
        }
    }
    let fresh0 = place;
    place = place.offset(1);
    BSDoptopt = *fresh0 as libc::c_int;
    if BSDoptopt == ':' as i32
        || {
            oli = strchr(ostr, BSDoptopt);
            oli.is_null()
        }
    {
        if BSDoptopt == '-' as i32 {
            return -(1 as libc::c_int);
        }
        if *place == 0 {
            BSDoptind += 1;
            BSDoptind;
        }
        if BSDopterr != 0 && *ostr as libc::c_int != ':' as i32 {
            fprintf(
                stderr,
                b"%s: unknown option -- %c\n\0" as *const u8 as *const libc::c_char,
                getprogname(),
                BSDoptopt,
            );
        }
        return '?' as i32;
    }
    oli = oli.offset(1);
    if *oli as libc::c_int != ':' as i32 {
        BSDoptarg = 0 as *mut libc::c_char;
        if *place == 0 {
            BSDoptind += 1;
            BSDoptind;
        }
    } else {
        if *place != 0 {
            BSDoptarg = place as *mut libc::c_char;
        } else {
            BSDoptind += 1;
            if nargc <= BSDoptind {
                place = b"\0" as *const u8 as *const libc::c_char;
                if *ostr as libc::c_int == ':' as i32 {
                    return ':' as i32;
                }
                if BSDopterr != 0 {
                    fprintf(
                        stderr,
                        b"%s: option requires an argument -- %c\n\0" as *const u8
                            as *const libc::c_char,
                        getprogname(),
                        BSDoptopt,
                    );
                }
                return '?' as i32;
            } else {
                BSDoptarg = *nargv.offset(BSDoptind as isize);
            }
        }
        place = b"\0" as *const u8 as *const libc::c_char;
        BSDoptind += 1;
        BSDoptind;
    }
    return BSDoptopt;
}
