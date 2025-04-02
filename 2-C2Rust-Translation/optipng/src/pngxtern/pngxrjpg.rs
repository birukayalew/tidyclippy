use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const libc::c_char;
pub type png_const_charpp = *mut *const libc::c_char;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
static mut jpeg_sig_jp2: [png_byte; 12] = [
    0 as libc::c_int as png_byte,
    0 as libc::c_int as png_byte,
    0 as libc::c_int as png_byte,
    0xc as libc::c_int as png_byte,
    0x6a as libc::c_int as png_byte,
    0x50 as libc::c_int as png_byte,
    0x20 as libc::c_int as png_byte,
    0x20 as libc::c_int as png_byte,
    0xd as libc::c_int as png_byte,
    0xa as libc::c_int as png_byte,
    0x87 as libc::c_int as png_byte,
    0xa as libc::c_int as png_byte,
];
static mut jpeg_sig_jpc: [png_byte; 4] = [
    0xff as libc::c_int as png_byte,
    0x4f as libc::c_int as png_byte,
    0xff as libc::c_int as png_byte,
    0x51 as libc::c_int as png_byte,
];
static mut jpeg_sig_jng: [png_byte; 8] = [
    0x8b as libc::c_int as png_byte,
    0x4a as libc::c_int as png_byte,
    0x4e as libc::c_int as png_byte,
    0x47 as libc::c_int as png_byte,
    0xd as libc::c_int as png_byte,
    0xa as libc::c_int as png_byte,
    0x1a as libc::c_int as png_byte,
    0xa as libc::c_int as png_byte,
];
static mut jpeg_sig_jng_jhdr: [png_byte; 8] = [
    0 as libc::c_int as png_byte,
    0 as libc::c_int as png_byte,
    0 as libc::c_int as png_byte,
    0x1a as libc::c_int as png_byte,
    0x4a as libc::c_int as png_byte,
    0x48 as libc::c_int as png_byte,
    0x44 as libc::c_int as png_byte,
    0x52 as libc::c_int as png_byte,
];
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_jpeg(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> libc::c_int {
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut marker: libc::c_uint = 0;
    let mut result: libc::c_int = 0;
    if sig_size < 12 as libc::c_int as size_t {
        return -(1 as libc::c_int);
    }
    if *sig.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int
        && *sig.offset(1 as libc::c_int as isize) as libc::c_int == 0xd8 as libc::c_int
        && *sig.offset(2 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int
    {
        marker = 0xff00 as libc::c_uint
            | *sig.offset(3 as libc::c_int as isize) as libc::c_uint;
        if marker >= 0xffc0 as libc::c_uint && marker <= 0xffcf as libc::c_uint
            || marker >= 0xffda as libc::c_uint && marker <= 0xfffe as libc::c_uint
        {
            fmt = b"JPEG\0" as *const u8 as *const libc::c_char;
            result = 1 as libc::c_int;
        } else {
            return 0 as libc::c_int
        }
    } else if memcmp(
        sig as *const libc::c_void,
        jpeg_sig_jp2.as_ptr() as *const libc::c_void,
        12 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || memcmp(
            sig as *const libc::c_void,
            jpeg_sig_jpc.as_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        fmt = b"JPEG-2000\0" as *const u8 as *const libc::c_char;
        result = 2 as libc::c_int;
    } else if memcmp(
        sig as *const libc::c_void,
        jpeg_sig_jng.as_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || memcmp(
            sig as *const libc::c_void,
            jpeg_sig_jng_jhdr.as_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        fmt = b"JNG\0" as *const u8 as *const libc::c_char;
        result = 3 as libc::c_int;
    } else {
        return 0 as libc::c_int
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = fmt;
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = fmt;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_jpeg(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut buf: [png_byte; 12] = [0; 12];
    let mut sig_code: libc::c_int = 0;
    if fread(
        buf.as_mut_ptr() as *mut libc::c_void,
        12 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    sig_code = pngx_sig_is_jpeg(
        buf.as_mut_ptr(),
        12 as libc::c_int as size_t,
        0 as png_const_charpp,
        0 as png_const_charpp,
    );
    match sig_code {
        1 => {
            png_error(
                png_ptr as *const png_struct,
                b"JPEG decoding is not supported\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            png_error(
                png_ptr as *const png_struct,
                b"JPEG-2000 decoding is not supported\0" as *const u8
                    as *const libc::c_char,
            );
        }
        3 => {
            png_error(
                png_ptr as *const png_struct,
                b"JNG (JPEG) decoding is not supported\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {}
    }
    if info_ptr.is_null() {
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
