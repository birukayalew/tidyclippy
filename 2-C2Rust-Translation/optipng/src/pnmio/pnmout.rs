use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn pnm_fput_header(
    mut pnm_ptr: *const pnm_struct,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut format: libc::c_uint = (*pnm_ptr).format;
    let mut depth: libc::c_uint = (*pnm_ptr).depth;
    let mut width: libc::c_uint = (*pnm_ptr).width;
    let mut height: libc::c_uint = (*pnm_ptr).height;
    let mut maxval: libc::c_uint = (*pnm_ptr).maxval;
    let mut result: libc::c_int = 0;
    if pnm_is_valid(pnm_ptr) == 0 {
        return 0 as libc::c_int;
    }
    match format {
        1 | 4 => {
            result = fprintf(
                stream,
                b"P%c\n%u %u\n\0" as *const u8 as *const libc::c_char,
                format.wrapping_add('0' as i32 as libc::c_uint),
                width,
                height,
            );
        }
        2 | 3 | 5 | 6 => {
            result = fprintf(
                stream,
                b"P%c\n%u %u\n%u\n\0" as *const u8 as *const libc::c_char,
                format.wrapping_add('0' as i32 as libc::c_uint),
                width,
                height,
                maxval,
            );
        }
        7 => {
            result = fprintf(
                stream,
                b"P7\nDEPTH %u\nWIDTH %u\nHEIGHT %u\nMAXVAL %u\nENDHDR\n\0" as *const u8
                    as *const libc::c_char,
                depth,
                width,
                height,
                maxval,
            );
        }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    return if result > 0 as libc::c_int {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fput_values(
    mut pnm_ptr: *const pnm_struct,
    mut sample_values: *const libc::c_uint,
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
    let mut mask: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    match format {
        1 => {
            j = 0 as libc::c_int as size_t;
            i = j;
            while i < num_samples {
                if putc(
                    (if *sample_values.offset(i as isize)
                        != 0 as libc::c_int as libc::c_uint
                    {
                        '0' as i32
                    } else {
                        '1' as i32
                    }),
                    stream,
                ) == -(1 as libc::c_int)
                {
                    break;
                }
                j = j.wrapping_add(1);
                if j == row_length {
                    j = 0 as libc::c_int as size_t;
                    if putc('\n' as i32, stream) == -(1 as libc::c_int) {
                        break;
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        2 | 3 => {
            j = 0 as libc::c_int as size_t;
            i = j;
            while i < num_samples {
                j = j.wrapping_add(1);
                if j == row_length {
                    j = 0 as libc::c_int as size_t;
                }
                if fprintf(
                    stream,
                    (if j == 0 as libc::c_int as size_t {
                        b"%u\n\0" as *const u8 as *const libc::c_char
                    } else {
                        b"%u \0" as *const u8 as *const libc::c_char
                    }),
                    *sample_values.offset(i as isize),
                ) <= 0 as libc::c_int
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
                ch = 0 as libc::c_int;
                mask = 0x80 as libc::c_int;
                while mask != 0 as libc::c_int {
                    let fresh0 = i;
                    i = i.wrapping_add(1);
                    if *sample_values.offset(fresh0 as isize)
                        == 0 as libc::c_int as libc::c_uint
                    {
                        ch |= mask;
                    }
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as libc::c_int as size_t;
                        break;
                    } else {
                        mask >>= 1 as libc::c_int;
                    }
                }
                if putc(ch, stream) == -(1 as libc::c_int) {
                    break;
                }
            }
        }
        5 | 6 | 7 => {
            if maxval <= 0xff as libc::c_uint {
                i = 0 as libc::c_int as size_t;
                while i < num_samples {
                    if putc(
                        (*sample_values.offset(i as isize)
                            & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                        stream,
                    ) == -(1 as libc::c_int)
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            } else if maxval <= 0xffff as libc::c_uint {
                i = 0 as libc::c_int as size_t;
                while i < num_samples {
                    if putc(
                        (*sample_values.offset(i as isize) >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                        stream,
                    ) == -(1 as libc::c_int)
                        || putc(
                            (*sample_values.offset(i as isize)
                                & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                            stream,
                        ) == -(1 as libc::c_int)
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            } else if maxval <= 0xffffffff as libc::c_uint {
                i = 0 as libc::c_int as size_t;
                while i < num_samples {
                    if maxval > 0xffffff as libc::c_uint {
                        if putc(
                            (*sample_values.offset(i as isize) >> 24 as libc::c_int
                                & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                            stream,
                        ) == -(1 as libc::c_int)
                        {
                            break;
                        }
                    }
                    if putc(
                        (*sample_values.offset(i as isize) >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                        stream,
                    ) == -(1 as libc::c_int)
                        || putc(
                            (*sample_values.offset(i as isize) >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                            stream,
                        ) == -(1 as libc::c_int)
                        || putc(
                            (*sample_values.offset(i as isize)
                                & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
                            stream,
                        ) == -(1 as libc::c_int)
                    {
                        break;
                    }
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
    return if i == num_samples { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fput_bytes(
    mut pnm_ptr: *const pnm_struct,
    mut sample_bytes: *const libc::c_uchar,
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
                ch = 0 as libc::c_int;
                mask = 0x80 as libc::c_int;
                while mask != 0 as libc::c_int {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    if *sample_bytes.offset(fresh1 as isize) as libc::c_int
                        == 0 as libc::c_int
                    {
                        ch |= mask;
                    }
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as libc::c_int as size_t;
                        break;
                    } else {
                        mask >>= 1 as libc::c_int;
                    }
                }
                if putc(ch, stream) == -(1 as libc::c_int) {
                    break;
                }
            }
        }
        5 | 6 | 7 => {
            i = fwrite(
                sample_bytes as *const libc::c_void,
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
    return if i == num_samples { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
