use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
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
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: libc::c_int,
    ) -> png_bytepp;
    fn minitiff_init_info(info_ptr: *mut minitiff_info);
    fn minitiff_validate_info(info_ptr: *const minitiff_info);
    fn minitiff_destroy_info(info_ptr: *mut minitiff_info);
    fn minitiff_read_info(info_ptr: *mut minitiff_info, stream: *mut FILE);
    fn minitiff_read_row(
        info_ptr: *mut minitiff_info,
        row_ptr: *mut libc::c_uchar,
        row_index: size_t,
        stream: *mut FILE,
    );
    static minitiff_sig_m: [libc::c_char; 4];
    static minitiff_sig_i: [libc::c_char; 4];
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
pub struct minitiff_info {
    pub error_handler: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub warning_handler: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    pub byte_order: libc::c_int,
    pub width: size_t,
    pub height: size_t,
    pub bits_per_sample: libc::c_uint,
    pub compression: libc::c_uint,
    pub photometric: libc::c_uint,
    pub strip_offsets_count: size_t,
    pub strip_offsets: *mut libc::c_ulong,
    pub orientation: libc::c_uint,
    pub samples_per_pixel: libc::c_uint,
    pub rows_per_strip: size_t,
}
static mut tiff_fmt_name: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"TIFF\0")
};
static mut tiff_fmt_long_name: [libc::c_char; 25] = unsafe {
    *::core::mem::transmute::<
        &[u8; 25],
        &[libc::c_char; 25],
    >(b"Tagged Image File Format\0")
};
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_tiff(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> libc::c_int {
    if sig_size < 8 as libc::c_int as size_t {
        return -(1 as libc::c_int);
    }
    if memcmp(
        sig as *const libc::c_void,
        minitiff_sig_m.as_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && memcmp(
            sig as *const libc::c_void,
            minitiff_sig_i.as_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = tiff_fmt_name.as_ptr();
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = tiff_fmt_long_name.as_ptr();
    }
    return 1 as libc::c_int;
}
static mut err_png_ptr: png_structp = 0 as *const png_struct as png_structp;
static mut num_extra_images: libc::c_uint = 0;
unsafe extern "C" fn pngx_tiff_error(mut msg: *const libc::c_char) {
    png_error(err_png_ptr as *const png_struct, msg);
}
unsafe extern "C" fn pngx_tiff_warning(mut msg: *const libc::c_char) {
    if !(strstr(msg, b"multi-image\0" as *const u8 as *const libc::c_char)).is_null() {
        num_extra_images = num_extra_images.wrapping_add(1);
        num_extra_images;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_tiff(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut tiff_info: minitiff_info = minitiff_info {
        error_handler: None,
        warning_handler: None,
        byte_order: 0,
        width: 0,
        height: 0,
        bits_per_sample: 0,
        compression: 0,
        photometric: 0,
        strip_offsets_count: 0,
        strip_offsets: 0 as *mut libc::c_ulong,
        orientation: 0,
        samples_per_pixel: 0,
        rows_per_strip: 0,
    };
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut pixel_size: libc::c_uint = 0;
    let mut sample_depth: libc::c_uint = 0;
    let mut sample_max: libc::c_uint = 0;
    let mut color_type: libc::c_int = 0;
    let mut sample_overflow: libc::c_int = 0;
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut row: png_bytep = 0 as *mut png_byte;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    err_png_ptr = png_ptr;
    num_extra_images = 0 as libc::c_int as libc::c_uint;
    minitiff_init_info(&mut tiff_info);
    tiff_info
        .error_handler = Some(
        pngx_tiff_error as unsafe extern "C" fn(*const libc::c_char) -> (),
    );
    tiff_info
        .warning_handler = Some(
        pngx_tiff_warning as unsafe extern "C" fn(*const libc::c_char) -> (),
    );
    minitiff_read_info(&mut tiff_info, stream);
    minitiff_validate_info(&mut tiff_info);
    width = tiff_info.width as libc::c_uint;
    height = tiff_info.height as libc::c_uint;
    pixel_size = tiff_info.samples_per_pixel;
    sample_depth = tiff_info.bits_per_sample;
    match pixel_size {
        1 => {
            color_type = 0 as libc::c_int;
        }
        2 => {
            color_type = 4 as libc::c_int;
        }
        3 => {
            color_type = 2 as libc::c_int;
        }
        4 => {
            color_type = 2 as libc::c_int | 4 as libc::c_int;
        }
        _ => {
            png_error(
                png_ptr as *const png_struct,
                b"Unsupported TIFF color space\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    if sample_depth > 16 as libc::c_int as libc::c_uint {
        png_error(
            png_ptr as *const png_struct,
            b"Unsupported TIFF sample depth\0" as *const u8 as *const libc::c_char,
        );
    }
    sample_max = (((1 as libc::c_int) << sample_depth) - 1 as libc::c_int)
        as libc::c_uint;
    sample_overflow = 0 as libc::c_int;
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        if sample_depth <= 8 as libc::c_int as libc::c_uint {
            8 as libc::c_int
        } else {
            16 as libc::c_int
        },
        color_type,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, 0 as libc::c_int);
    if sample_depth <= 8 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as libc::c_uint;
        while i < height {
            row = *row_pointers.offset(i as isize);
            minitiff_read_row(&mut tiff_info, row, i as size_t, stream);
            if sample_depth < 8 as libc::c_int as libc::c_uint {
                j = 0 as libc::c_int as libc::c_uint;
                while j < pixel_size.wrapping_mul(width) {
                    let mut b: libc::c_uint = *row.offset(j as isize) as libc::c_uint;
                    if b > sample_max {
                        b = sample_max;
                        sample_overflow = 1 as libc::c_int;
                    }
                    *row
                        .offset(
                            j as isize,
                        ) = b
                        .wrapping_mul(255 as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            sample_max.wrapping_div(2 as libc::c_int as libc::c_uint),
                        )
                        .wrapping_div(sample_max) as png_byte;
                    j = j.wrapping_add(1);
                    j;
                }
            }
            if tiff_info.photometric == 0 as libc::c_int as libc::c_uint {
                j = 0 as libc::c_int as libc::c_uint;
                while j < pixel_size.wrapping_mul(width) {
                    *row
                        .offset(
                            j as isize,
                        ) = (255 as libc::c_int - *row.offset(j as isize) as libc::c_int)
                        as png_byte;
                    j = j.wrapping_add(1);
                    j;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < height {
            row = *row_pointers.offset(i as isize);
            minitiff_read_row(&mut tiff_info, row, i as size_t, stream);
            if tiff_info.byte_order == 'I' as i32 {
                k = 0 as libc::c_int as libc::c_uint;
                j = k;
                while j < pixel_size.wrapping_mul(width) {
                    let mut b_0: png_byte = *row.offset(k as isize);
                    *row
                        .offset(
                            k as isize,
                        ) = *row
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        );
                    *row
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) = b_0;
                    j = j.wrapping_add(1);
                    j;
                    k = k.wrapping_add(2 as libc::c_int as libc::c_uint);
                }
            }
            if sample_depth < 16 as libc::c_int as libc::c_uint {
                k = 0 as libc::c_int as libc::c_uint;
                j = k;
                while k < pixel_size.wrapping_mul(width) {
                    let mut b_1: libc::c_uint = (((*row.offset(k as isize)
                        as libc::c_int) << 8 as libc::c_int)
                        + *row
                            .offset(
                                k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int) as libc::c_uint;
                    if b_1 > sample_max {
                        b_1 = sample_max;
                        sample_overflow = 1 as libc::c_int;
                    }
                    b_1 = b_1
                        .wrapping_mul(65535 as libc::c_uint)
                        .wrapping_add(
                            sample_max.wrapping_div(2 as libc::c_int as libc::c_uint),
                        )
                        .wrapping_div(sample_max);
                    *row.offset(k as isize) = (b_1 >> 8 as libc::c_int) as png_byte;
                    *row
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) = (b_1 & 255 as libc::c_int as libc::c_uint) as png_byte;
                    j = j.wrapping_add(1);
                    j;
                    k = k.wrapping_add(2 as libc::c_int as libc::c_uint);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if sample_overflow != 0 {
        png_warning(
            png_ptr as *const png_struct,
            b"Overflow in TIFF samples\0" as *const u8 as *const libc::c_char,
        );
    }
    minitiff_destroy_info(&mut tiff_info);
    return (1 as libc::c_int as libc::c_uint).wrapping_add(num_extra_images)
        as libc::c_int;
}
