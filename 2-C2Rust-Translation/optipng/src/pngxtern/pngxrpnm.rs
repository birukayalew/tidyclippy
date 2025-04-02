use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: libc::c_int,
        color_type: libc::c_int,
        interlace_method: libc::c_int,
        compression_method: libc::c_int,
        filter_method: libc::c_int,
    );
    fn png_set_sBIT(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        sig_bit: png_const_color_8p,
    );
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: libc::c_int,
    ) -> png_bytepp;
    fn pnm_fget_header(pnm_ptr: *mut pnm_struct, stream: *mut FILE) -> libc::c_int;
    fn pnm_fget_values(
        pnm_ptr: *const pnm_struct,
        sample_values: *mut libc::c_uint,
        num_rows: libc::c_uint,
        stream: *mut FILE,
    ) -> libc::c_int;
    fn pnm_fget_bytes(
        pnm_ptr: *const pnm_struct,
        sample_bytes: *mut libc::c_uchar,
        sample_size: size_t,
        num_rows: libc::c_uint,
        stream: *mut FILE,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub type png_byte = libc::c_uchar;
pub type png_uint_32 = libc::c_uint;
pub type png_alloc_size_t = size_t;
pub type png_voidp = *mut libc::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const libc::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_const_charpp = *mut *const libc::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_color_8 = png_color_8_struct;
pub type png_const_color_8p = *const png_color_8;
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
static mut pbm_fmt_name: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"PBM\0")
};
static mut pgm_fmt_name: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"PGM\0")
};
static mut ppm_fmt_name: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"PPM\0")
};
static mut pam_fmt_name: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"PAM\0")
};
static mut pbm_fmt_long_name: [libc::c_char; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"Portable Bitmap\0")
};
static mut pgm_fmt_long_name: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"Portable Graymap\0")
};
static mut ppm_fmt_long_name: [libc::c_char; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"Portable Pixmap\0")
};
static mut pam_fmt_long_name: [libc::c_char; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"Portable Anymap\0")
};
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_pnm(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> libc::c_int {
    static mut fmt_names: [*const libc::c_char; 7] = unsafe {
        [
            pbm_fmt_name.as_ptr(),
            pgm_fmt_name.as_ptr(),
            ppm_fmt_name.as_ptr(),
            pbm_fmt_name.as_ptr(),
            pgm_fmt_name.as_ptr(),
            ppm_fmt_name.as_ptr(),
            pam_fmt_name.as_ptr(),
        ]
    };
    static mut fmt_long_names: [*const libc::c_char; 7] = unsafe {
        [
            pbm_fmt_long_name.as_ptr(),
            pgm_fmt_long_name.as_ptr(),
            ppm_fmt_long_name.as_ptr(),
            pbm_fmt_long_name.as_ptr(),
            pgm_fmt_long_name.as_ptr(),
            ppm_fmt_long_name.as_ptr(),
            pam_fmt_long_name.as_ptr(),
        ]
    };
    if sig_size < 4 as libc::c_int as size_t {
        return -(1 as libc::c_int);
    }
    if *sig.offset(0 as libc::c_int as isize) as libc::c_int != 'P' as i32
        || (*sig.offset(1 as libc::c_int as isize) as libc::c_int) < '1' as i32
        || *sig.offset(1 as libc::c_int as isize) as libc::c_int > '7' as i32
    {
        return 0 as libc::c_int;
    }
    if *sig.offset(2 as libc::c_int as isize) as libc::c_int != ' ' as i32
        && *sig.offset(2 as libc::c_int as isize) as libc::c_int != '\t' as i32
        && *sig.offset(2 as libc::c_int as isize) as libc::c_int != '\n' as i32
        && *sig.offset(2 as libc::c_int as isize) as libc::c_int != '\r' as i32
        && *sig.offset(2 as libc::c_int as isize) as libc::c_int != '#' as i32
    {
        return 0 as libc::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = fmt_names[(*sig.offset(1 as libc::c_int as isize) as libc::c_int
            - '1' as i32) as usize];
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = fmt_long_names[(*sig.offset(1 as libc::c_int as isize)
            as libc::c_int - '1' as i32) as usize];
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pnm_fpeek_eof(
    mut pnm_ptr: *mut pnm_struct,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    if (*pnm_ptr).format >= PNM_P1 as libc::c_int as libc::c_uint
        && (*pnm_ptr).format <= PNM_P3 as libc::c_int as libc::c_uint
    {
        loop {
            ch = getc(stream);
            if ch == '#' as i32 {
                loop {
                    ch = getc(stream);
                    if !(ch != -(1 as libc::c_int) && ch != '\n' as i32
                        && ch != '\r' as i32)
                    {
                        break;
                    }
                }
            }
            if ch == -(1 as libc::c_int) {
                return 1 as libc::c_int;
            }
            if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32
                || ch == '\r' as i32)
            {
                break;
            }
        }
    } else {
        ch = getc(stream);
        if ch == -(1 as libc::c_int) {
            return 1 as libc::c_int;
        }
    }
    ungetc(ch, stream);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_pnm(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut pnminfo: pnm_struct = pnm_struct {
        format: 0,
        depth: 0,
        width: 0,
        height: 0,
        maxval: 0,
    };
    let mut format: libc::c_uint = 0;
    let mut depth: libc::c_uint = 0;
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut maxval: libc::c_uint = 0;
    let mut max_width: libc::c_uint = 0;
    let mut num_samples: libc::c_uint = 0;
    let mut sample_size: libc::c_uint = 0;
    let mut pnmrow: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut row_size: size_t = 0;
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut sig_bit: png_color_8 = png_color_8_struct {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    };
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut failed: libc::c_int = 0;
    let mut overflow: libc::c_int = 0;
    if pnm_fget_header(&mut pnminfo, stream) != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    format = pnminfo.format;
    depth = pnminfo.depth;
    width = pnminfo.width;
    height = pnminfo.height;
    maxval = pnminfo.maxval;
    if format > PNM_P6 as libc::c_int as libc::c_uint {
        png_error(
            png_ptr as *const png_struct,
            b"Can't handle PNM formats newer than PPM (\"P6\")\0" as *const u8
                as *const libc::c_char,
        );
    }
    max_width = (if ::core::mem::size_of::<size_t>() as libc::c_ulong
        <= ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
    {
        ((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_div(depth as libc::c_ulong)
    } else {
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
    }) as libc::c_uint;
    if max_width > 0x7fffffff as libc::c_uint {
        max_width = 0x7fffffff as libc::c_uint;
    }
    if width > max_width {
        png_error(
            png_ptr as *const png_struct,
            b"Can't handle exceedingly large PNM dimensions\0" as *const u8
                as *const libc::c_char,
        );
    }
    sample_size = 1 as libc::c_int as libc::c_uint;
    num_samples = depth.wrapping_mul(width);
    row_size = num_samples as size_t;
    if maxval > 65535 as libc::c_int as libc::c_uint {
        png_error(
            png_ptr as *const png_struct,
            b"Can't handle PNM samples larger than 16 bits\0" as *const u8
                as *const libc::c_char,
        );
    } else if maxval > 255 as libc::c_int as libc::c_uint {
        sample_size = 2 as libc::c_int as libc::c_uint;
        row_size = row_size * 2 as libc::c_int as size_t;
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        if maxval <= 255 as libc::c_int as libc::c_uint {
            8 as libc::c_int
        } else {
            16 as libc::c_int
        },
        if depth == 1 as libc::c_int as libc::c_uint {
            0 as libc::c_int
        } else {
            2 as libc::c_int
        },
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    i = 1 as libc::c_int as libc::c_uint;
    j = 2 as libc::c_int as libc::c_uint;
    while j.wrapping_sub(1 as libc::c_int as libc::c_uint) < maxval {
        i = i.wrapping_add(1);
        i;
        j <<= 1 as libc::c_int;
    }
    if j.wrapping_sub(1 as libc::c_int as libc::c_uint) != maxval {
        png_warning(
            png_ptr as *const png_struct,
            b"Possibly inexact sample conversion from PNM to PNG\0" as *const u8
                as *const libc::c_char,
        );
    } else if i.wrapping_rem(8 as libc::c_int as libc::c_uint)
        != 0 as libc::c_int as libc::c_uint
        && (depth > 1 as libc::c_int as libc::c_uint
            || (8 as libc::c_int as libc::c_uint).wrapping_rem(i)
                != 0 as libc::c_int as libc::c_uint)
    {
        sig_bit.gray = i as png_byte;
        sig_bit.blue = sig_bit.gray;
        sig_bit.green = sig_bit.blue;
        sig_bit.red = sig_bit.green;
        sig_bit.alpha = 0 as libc::c_int as png_byte;
        png_set_sBIT(
            png_ptr as *const png_struct,
            info_ptr,
            &mut sig_bit as *mut png_color_8 as png_const_color_8p,
        );
    }
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, -(1 as libc::c_int));
    if format >= PNM_P4 as libc::c_int as libc::c_uint
        && (maxval == 255 as libc::c_int as libc::c_uint
            || maxval == 65535 as libc::c_int as libc::c_uint)
    {
        pnmrow = 0 as *mut libc::c_uint;
    } else {
        pnmrow = png_malloc(
            png_ptr as *const png_struct,
            (num_samples as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint;
    }
    failed = 0 as libc::c_int;
    overflow = 0 as libc::c_int;
    if !pnmrow.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < height {
            if pnm_fget_values(
                &mut pnminfo,
                pnmrow,
                1 as libc::c_int as libc::c_uint,
                stream,
            ) <= 0 as libc::c_int
            {
                failed = 1 as libc::c_int;
            }
            if maxval <= 255 as libc::c_int as libc::c_uint {
                j = 0 as libc::c_int as libc::c_uint;
                while j < num_samples {
                    let mut val: libc::c_uint = *pnmrow.offset(j as isize);
                    if val > maxval {
                        val = 255 as libc::c_int as libc::c_uint;
                        overflow = 1 as libc::c_int;
                    } else if maxval != 255 as libc::c_int as libc::c_uint {
                        val = val
                            .wrapping_mul(255 as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                maxval.wrapping_div(2 as libc::c_int as libc::c_uint),
                            )
                            .wrapping_div(maxval);
                    }
                    *(*row_pointers.offset(i as isize))
                        .offset(j as isize) = val as png_byte;
                    j = j.wrapping_add(1);
                    j;
                }
            } else {
                j = 0 as libc::c_int as libc::c_uint;
                while j < num_samples {
                    let mut val_0: png_uint_32 = *pnmrow.offset(j as isize);
                    if val_0 > maxval {
                        val_0 = 65535 as libc::c_int as png_uint_32;
                        overflow = 1 as libc::c_int;
                    } else if maxval != 65535 as libc::c_int as libc::c_uint {
                        val_0 = (val_0 * 65535 as libc::c_int as png_uint_32)
                            .wrapping_add(
                                maxval.wrapping_div(2 as libc::c_int as libc::c_uint),
                            )
                            .wrapping_div(maxval);
                    }
                    *(*row_pointers.offset(i as isize))
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_mul(j) as isize,
                        ) = (val_0 >> 8 as libc::c_int) as png_byte;
                    *(*row_pointers.offset(i as isize))
                        .offset(
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_mul(j)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) = (val_0 & 0xff as libc::c_int as png_uint_32) as png_byte;
                    j = j.wrapping_add(1);
                    j;
                }
            }
            if failed != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < height {
            if pnm_fget_bytes(
                &mut pnminfo,
                *row_pointers.offset(i as isize),
                sample_size as size_t,
                1 as libc::c_int as libc::c_uint,
                stream,
            ) <= 0 as libc::c_int
            {
                failed = 1 as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    while i < height {
        memset(
            *row_pointers.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            row_size,
        );
        i = i.wrapping_add(1);
        i;
    }
    if !pnmrow.is_null() {
        png_free(png_ptr as *const png_struct, pnmrow as png_voidp);
    }
    if overflow != 0 {
        png_warning(
            png_ptr as *const png_struct,
            b"Overflow in PNM samples\0" as *const u8 as *const libc::c_char,
        );
    }
    if failed != 0 {
        png_error(
            png_ptr as *const png_struct,
            b"Error in PNM image file\0" as *const u8 as *const libc::c_char,
        );
    } else if pnm_fpeek_eof(&mut pnminfo, stream) == 0 {
        png_warning(
            png_ptr as *const png_struct,
            b"Extraneous data found after PNM image\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
