use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pnm_is_valid(pnm_ptr: *const pnm_struct) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: libc::c_uint,
    pub depth: libc::c_uint,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub maxval: libc::c_uint,
}
unsafe extern "C" fn pnm_fget_char(mut stream: *mut FILE) -> libc::c_int {
    let mut ch: libc::c_int = getc(stream);
    if ch == '#' as i32 {
        loop {
            ch = getc(stream);
            if !(ch != -(1 as libc::c_int) && ch != '\n' as i32 && ch != '\r' as i32) {
                break;
            }
        }
    }
    if ch == '\r' as i32 {
        ch = getc(stream);
        if ch != '\n' as i32 {
            ungetc(ch, stream);
            ch = '\n' as i32;
        }
    }
    return ch;
}
unsafe extern "C" fn pnm_fscan_uint(
    mut stream: *mut FILE,
    mut value: *mut libc::c_uint,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut tmp: libc::c_uint = 0;
    loop {
        ch = pnm_fget_char(stream);
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32
            || ch == '\r' as i32)
        {
            break;
        }
    }
    if ch == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !(ch >= '0' as i32 && ch <= '9' as i32) {
        ungetc(ch, stream);
        return 0 as libc::c_int;
    }
    *value = 0 as libc::c_int as libc::c_uint;
    loop {
        tmp = (*value)
            .wrapping_mul(10 as libc::c_int as libc::c_uint)
            .wrapping_add((ch - '0' as i32) as libc::c_uint);
        if tmp >= *value {
            *value = tmp;
        } else {
            *value = (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint);
            *__errno_location() = 34 as libc::c_int;
        }
        ch = getc(stream);
        if !(ch >= '0' as i32 && ch <= '9' as i32) {
            break;
        }
    }
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32)
    {
        ungetc(ch, stream);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_header(
    mut pnm_ptr: *mut pnm_struct,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut format: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    memset(
        pnm_ptr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<pnm_struct>() as libc::c_ulong,
    );
    ch = getc(stream);
    if ch == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if ch != 'P' as i32 {
        return -(1 as libc::c_int);
    }
    ch = getc(stream);
    if ch < '1' as i32 || ch > '9' as i32 {
        return -(1 as libc::c_int);
    }
    format = (ch - '0' as i32) as libc::c_uint;
    ch = pnm_fget_char(stream);
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32)
    {
        return -(1 as libc::c_int);
    }
    (*pnm_ptr).format = format;
    if format >= PNM_P1 as libc::c_int as libc::c_uint
        && format <= PNM_P6 as libc::c_int as libc::c_uint
    {
        (*pnm_ptr)
            .depth = (if format == PNM_P3 as libc::c_int as libc::c_uint
            || format == PNM_P6 as libc::c_int as libc::c_uint
        {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_uint;
        if pnm_fscan_uint(stream, &mut (*pnm_ptr).width) != 1 as libc::c_int
            || pnm_fscan_uint(stream, &mut (*pnm_ptr).height) != 1 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if format == PNM_P1 as libc::c_int as libc::c_uint
            || format == PNM_P4 as libc::c_int as libc::c_uint
        {
            (*pnm_ptr).maxval = 1 as libc::c_int as libc::c_uint;
        } else if pnm_fscan_uint(stream, &mut (*pnm_ptr).maxval) != 1 as libc::c_int {
            return -(1 as libc::c_int)
        }
        return if pnm_is_valid(pnm_ptr) != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_values(
    mut pnm_ptr: *const pnm_struct,
    mut sample_values: *mut libc::c_uint,
    mut num_rows: libc::c_uint,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut format: libc::c_uint = (*pnm_ptr).format;
    let mut depth: libc::c_uint = (*pnm_ptr).depth;
    let mut width: libc::c_uint = (*pnm_ptr).width;
    let mut maxval: libc::c_uint = (*pnm_ptr).maxval;
    let mut row_length: size_t = depth as size_t * width as size_t;
    let mut num_samples: size_t = num_rows as size_t * row_length;
    let mut ch: libc::c_int = 0;
    let mut ch8: libc::c_int = 0;
    let mut ch16: libc::c_int = 0;
    let mut ch24: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    match format {
        1 => {
            i = 0 as libc::c_int as size_t;
            while i < num_samples {
                loop {
                    ch = pnm_fget_char(stream);
                    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32
                        || ch == '\r' as i32)
                    {
                        break;
                    }
                }
                if ch != '0' as i32 && ch != '1' as i32 {
                    ungetc(ch, stream);
                    break;
                } else {
                    *sample_values
                        .offset(
                            i as isize,
                        ) = (if ch == '0' as i32 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint;
                    i = i.wrapping_add(1);
                    i;
                }
            }
        }
        2 | 3 => {
            i = 0 as libc::c_int as size_t;
            while i < num_samples {
                if pnm_fscan_uint(stream, &mut *sample_values.offset(i as isize))
                    != 1 as libc::c_int
                {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        4 => {
            j = 0 as libc::c_int as size_t;
            i = j;
            while i < num_samples {
                ch = getc(stream);
                if ch == -(1 as libc::c_int) {
                    break;
                }
                mask = 0x80 as libc::c_int;
                while mask != 0 as libc::c_int {
                    let fresh0 = i;
                    i = i.wrapping_add(1);
                    *sample_values
                        .offset(
                            fresh0 as isize,
                        ) = (if ch & mask != 0 {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::c_uint;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as libc::c_int as size_t;
                        break;
                    } else {
                        mask >>= 1 as libc::c_int;
                    }
                }
            }
        }
        5 | 6 | 7 => {
            if maxval <= 0xff as libc::c_uint {
                i = 0 as libc::c_int as size_t;
                while i < num_samples {
                    ch = getc(stream);
                    if ch == -(1 as libc::c_int) {
                        break;
                    }
                    *sample_values.offset(i as isize) = ch as libc::c_uint;
                    i = i.wrapping_add(1);
                    i;
                }
            } else if maxval <= 0xffff as libc::c_uint {
                i = 0 as libc::c_int as size_t;
                while i < num_samples {
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == -(1 as libc::c_int) {
                        break;
                    }
                    *sample_values
                        .offset(
                            i as isize,
                        ) = ((ch8 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_add(ch as libc::c_uint);
                    i = i.wrapping_add(1);
                    i;
                }
            } else if maxval <= 0xffffffff as libc::c_uint {
                ch24 = 0 as libc::c_int;
                i = 0 as libc::c_int as size_t;
                while i < num_samples {
                    if maxval > 0xffffff as libc::c_uint {
                        ch24 = getc(stream);
                    }
                    ch16 = getc(stream);
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == -(1 as libc::c_int) {
                        break;
                    }
                    *sample_values
                        .offset(
                            i as isize,
                        ) = ((ch24 as libc::c_uint) << 24 as libc::c_int)
                        .wrapping_add((ch16 as libc::c_uint) << 16 as libc::c_int)
                        .wrapping_add((ch8 as libc::c_uint) << 8 as libc::c_int)
                        .wrapping_add(ch as libc::c_uint);
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                *__errno_location() = 22 as libc::c_int;
                return 0 as libc::c_int;
            }
        }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    if i < num_samples {
        memset(
            sample_values.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            num_samples
                .wrapping_sub(i)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_bytes(
    mut pnm_ptr: *const pnm_struct,
    mut sample_bytes: *mut libc::c_uchar,
    mut sample_size: size_t,
    mut num_rows: libc::c_uint,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut format: libc::c_uint = (*pnm_ptr).format;
    let mut depth: libc::c_uint = (*pnm_ptr).depth;
    let mut width: libc::c_uint = (*pnm_ptr).width;
    let mut maxval: libc::c_uint = (*pnm_ptr).maxval;
    let mut row_length: size_t = depth as size_t * width as size_t;
    let mut num_samples: size_t = num_rows as size_t * row_length;
    let mut raw_sample_size: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if maxval <= 0xff as libc::c_uint {
        raw_sample_size = 1 as libc::c_int as size_t;
    } else if maxval <= 0xffff as libc::c_uint {
        raw_sample_size = 2 as libc::c_int as size_t;
    } else if maxval <= 0xffffff as libc::c_uint {
        raw_sample_size = 3 as libc::c_int as size_t;
    } else if maxval <= 0xffffffff as libc::c_uint {
        raw_sample_size = 4 as libc::c_int as size_t;
    } else {
        raw_sample_size = (sample_size == 0) as libc::c_int as size_t;
    }
    if raw_sample_size != sample_size {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    match format {
        4 => {
            j = 0 as libc::c_int as size_t;
            i = j;
            while i < num_samples {
                ch = getc(stream);
                if ch == -(1 as libc::c_int) {
                    break;
                }
                mask = 0x80 as libc::c_int;
                while mask != 0 as libc::c_int {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    *sample_bytes
                        .offset(
                            fresh1 as isize,
                        ) = (if ch & mask != 0 {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::c_uchar;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as libc::c_int as size_t;
                        break;
                    } else {
                        mask >>= 1 as libc::c_int;
                    }
                }
            }
        }
        5 | 6 | 7 => {
            i = fread(
                sample_bytes as *mut libc::c_void,
                sample_size,
                num_samples,
                stream,
            );
        }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    if i < num_samples {
        memset(
            sample_bytes.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (sample_size * num_samples).wrapping_sub(i),
        );
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
