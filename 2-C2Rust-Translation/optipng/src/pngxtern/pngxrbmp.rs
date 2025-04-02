use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    pub type png_info_def;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn png_set_sig_bytes(png_ptr: png_structrp, num_bytes: libc::c_int);
    fn png_set_read_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        read_data_fn: png_rw_ptr,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
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
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: libc::c_int,
    );
    fn png_set_sBIT(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        sig_bit: png_const_color_8p,
    );
    fn png_read_png(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        transforms: libc::c_int,
        params: png_voidp,
    );
    fn pngx_malloc_rows_extended(
        png_ptr: png_structp,
        info_ptr: png_infop,
        min_row_size: pngx_alloc_size_t,
        filler: libc::c_int,
    ) -> png_bytepp;
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
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_color = png_color_struct;
pub type png_const_colorp = *const png_color;
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
pub type png_rw_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_bytep, size_t) -> (),
>;
pub type pngx_alloc_size_t = png_alloc_size_t;
unsafe extern "C" fn bmp_get_word(mut ptr: png_bytep) -> libc::c_uint {
    return (*ptr.offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_add(
            (*ptr.offset(1 as libc::c_int as isize) as libc::c_uint) << 8 as libc::c_int,
        );
}
unsafe extern "C" fn bmp_get_dword(mut ptr: png_bytep) -> png_uint_32 {
    return (*ptr.offset(0 as libc::c_int as isize) as png_uint_32)
        .wrapping_add(
            (*ptr.offset(1 as libc::c_int as isize) as png_uint_32) << 8 as libc::c_int,
        )
        .wrapping_add(
            (*ptr.offset(2 as libc::c_int as isize) as png_uint_32) << 16 as libc::c_int,
        )
        .wrapping_add(
            (*ptr.offset(3 as libc::c_int as isize) as png_uint_32) << 24 as libc::c_int,
        );
}
unsafe extern "C" fn bmp_memset_bytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut ch: libc::c_int,
    mut len: size_t,
) {
    memset(ptr.offset(offset as isize) as *mut libc::c_void, ch, len);
}
unsafe extern "C" fn bmp_memset_halfbytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut ch: libc::c_int,
    mut len: size_t,
) {
    if len == 0 as libc::c_int as size_t {
        return;
    }
    ptr = ptr.offset((offset / 2 as libc::c_int as size_t) as isize);
    if offset & 1 as libc::c_int as size_t != 0 {
        *ptr = (*ptr as libc::c_int & 0xf0 as libc::c_int | ch & 0xf as libc::c_int)
            as png_byte;
        ch = (ch & 0xf0 as libc::c_int) >> 4 as libc::c_int
            | (ch & 0xf as libc::c_int) << 4 as libc::c_int;
        ptr = ptr.offset(1);
        ptr;
        len = len.wrapping_sub(1);
        len;
    }
    memset(ptr as *mut libc::c_void, ch, len / 2 as libc::c_int as size_t);
    if len & 1 as libc::c_int as size_t != 0 {
        *ptr
            .offset(
                (len / 2 as libc::c_int as size_t) as isize,
            ) = (ch & 0xf0 as libc::c_int) as png_byte;
    }
}
unsafe extern "C" fn bmp_fread_bytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut len: size_t,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: size_t = 0;
    result = fread(
        ptr.offset(offset as isize) as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len,
        stream,
    );
    if len & 1 as libc::c_int as size_t != 0 {
        getc(stream);
    }
    return result;
}
unsafe extern "C" fn bmp_fread_halfbytes(
    mut ptr: png_bytep,
    mut offset: size_t,
    mut len: size_t,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: size_t = 0;
    let mut ch: libc::c_int = 0;
    if len == 0 as libc::c_int as size_t {
        return 0 as libc::c_int as size_t;
    }
    ptr = ptr.offset((offset / 2 as libc::c_int as size_t) as isize);
    if offset & 1 as libc::c_int as size_t != 0 {
        result = 0 as libc::c_int as size_t;
        while result < len.wrapping_sub(1 as libc::c_int as size_t) {
            ch = getc(stream);
            if ch == -(1 as libc::c_int) {
                break;
            }
            *ptr = (*ptr as libc::c_int & 0xf0 as libc::c_int
                | (ch & 0xf0 as libc::c_int) >> 4 as libc::c_int) as png_byte;
            ptr = ptr.offset(1);
            ptr;
            *ptr = ((ch & 0xf as libc::c_int) << 4 as libc::c_int) as png_byte;
            result = result.wrapping_add(2 as libc::c_int as size_t);
        }
    } else {
        result = (fread(
            ptr as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len.wrapping_add(1 as libc::c_int as size_t) / 2 as libc::c_int as size_t,
            stream,
        ))
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    }
    if len & 2 as libc::c_int as size_t != 0 {
        getc(stream);
    }
    return if result <= len { result } else { len };
}
unsafe extern "C" fn bmp_process_mask(
    mut bmp_mask: png_uint_32,
    mut sig_bit: png_bytep,
    mut shift_bit: png_bytep,
) {
    *shift_bit = 0 as libc::c_int as png_byte;
    *sig_bit = *shift_bit;
    if bmp_mask == 0 as libc::c_int as png_uint_32 {
        return;
    }
    while bmp_mask & 1 as libc::c_int as png_uint_32 == 0 as libc::c_int as png_uint_32 {
        bmp_mask >>= 1 as libc::c_int;
        *shift_bit = (*shift_bit).wrapping_add(1);
        *shift_bit;
    }
    while bmp_mask != 0 as libc::c_int as png_uint_32 {
        if bmp_mask & 1 as libc::c_int as png_uint_32 == 0 as libc::c_int as png_uint_32
            || *sig_bit as libc::c_int >= 8 as libc::c_int
        {
            *sig_bit = 0 as libc::c_int as png_byte;
            return;
        }
        bmp_mask >>= 1 as libc::c_int;
        *sig_bit = (*sig_bit).wrapping_add(1);
        *sig_bit;
    }
}
unsafe extern "C" fn bmp_read_rows(
    mut begin_row: png_bytepp,
    mut end_row: png_bytepp,
    mut row_size: size_t,
    mut compression: libc::c_uint,
    mut stream: *mut FILE,
) -> size_t {
    let mut result: size_t = 0;
    let mut crt_row: png_bytepp = 0 as *mut *mut png_byte;
    let mut inc: libc::c_int = 0;
    let mut crtn: size_t = 0;
    let mut dcrtn: size_t = 0;
    let mut endn: size_t = 0;
    let mut len: libc::c_uint = 0;
    let mut b1: libc::c_uint = 0;
    let mut b2: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut bmp_memset_fn: Option::<
        unsafe extern "C" fn(png_bytep, size_t, libc::c_int, size_t) -> (),
    > = None;
    let mut bmp_fread_fn: Option::<
        unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t,
    > = None;
    if row_size == 0 as libc::c_int as size_t {
        return 0 as libc::c_int as size_t;
    }
    inc = if begin_row <= end_row { 1 as libc::c_int } else { -(1 as libc::c_int) };
    crtn = 0 as libc::c_int as size_t;
    result = 0 as libc::c_int as size_t;
    if compression == 2 as libc::c_int as libc::c_uint {
        endn = row_size * 2 as libc::c_int as size_t;
        if endn <= row_size {
            return 0 as libc::c_int as size_t;
        }
        bmp_memset_fn = Some(
            bmp_memset_halfbytes
                as unsafe extern "C" fn(png_bytep, size_t, libc::c_int, size_t) -> (),
        );
        bmp_fread_fn = Some(
            bmp_fread_halfbytes
                as unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t,
        );
    } else {
        endn = row_size;
        bmp_memset_fn = Some(
            bmp_memset_bytes
                as unsafe extern "C" fn(png_bytep, size_t, libc::c_int, size_t) -> (),
        );
        bmp_fread_fn = Some(
            bmp_fread_bytes
                as unsafe extern "C" fn(png_bytep, size_t, size_t, *mut FILE) -> size_t,
        );
    }
    if compression == 0 as libc::c_int as libc::c_uint
        || compression == 3 as libc::c_int as libc::c_uint
    {
        crt_row = begin_row;
        while crt_row != end_row {
            crtn = bmp_fread_fn
                .expect(
                    "non-null function pointer",
                )(*crt_row, 0 as libc::c_int as size_t, endn, stream);
            if crtn != endn {
                break;
            }
            result = result.wrapping_add(1);
            result;
            crt_row = crt_row.offset(inc as isize);
        }
    } else if compression == 1 as libc::c_int as libc::c_uint
        || compression == 2 as libc::c_int as libc::c_uint
    {
        if compression == 1 as libc::c_int as libc::c_uint {
            endn = row_size;
        } else {
            endn = row_size * 2 as libc::c_int as size_t;
            if endn <= row_size {
                return 0 as libc::c_int as size_t;
            }
        }
        crt_row = begin_row;
        while crt_row != end_row {
            ch = getc(stream);
            b1 = ch as libc::c_uint;
            ch = getc(stream);
            b2 = ch as libc::c_uint;
            if ch == -(1 as libc::c_int) {
                break;
            }
            if b1 == 0 as libc::c_int as libc::c_uint {
                if b2 == 0 as libc::c_int as libc::c_uint {
                    bmp_memset_fn
                        .expect(
                            "non-null function pointer",
                        )(*crt_row, crtn, 0 as libc::c_int, endn.wrapping_sub(crtn));
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as libc::c_int as size_t;
                    result = result.wrapping_add(1);
                    result;
                    if !(crt_row == end_row) {
                        continue;
                    }
                    ch = getc(stream);
                    if ch != -(1 as libc::c_int) && ch != 0 as libc::c_int {
                        ungetc(ch, stream);
                        break;
                    } else {
                        getc(stream);
                        break;
                    }
                } else if b2 == 1 as libc::c_int as libc::c_uint {
                    bmp_memset_fn
                        .expect(
                            "non-null function pointer",
                        )(*crt_row, crtn, 0 as libc::c_int, endn.wrapping_sub(crtn));
                    crt_row = crt_row.offset(inc as isize);
                    crtn = 0 as libc::c_int as size_t;
                    result = (if begin_row <= end_row {
                        end_row.offset_from(begin_row) as libc::c_long
                    } else {
                        begin_row.offset_from(end_row) as libc::c_long
                    }) as size_t;
                    break;
                } else if b2 == 2 as libc::c_int as libc::c_uint {
                    ch = getc(stream);
                    b1 = ch as libc::c_uint;
                    ch = getc(stream);
                    b2 = ch as libc::c_uint;
                    if ch == -(1 as libc::c_int) {
                        break;
                    }
                    dcrtn = if (b1 as size_t) < endn.wrapping_sub(crtn) {
                        crtn.wrapping_add(b1 as size_t)
                    } else {
                        endn
                    };
                    while b2 > 0 as libc::c_int as libc::c_uint {
                        bmp_memset_fn
                            .expect(
                                "non-null function pointer",
                            )(*crt_row, crtn, 0 as libc::c_int, endn.wrapping_sub(crtn));
                        crt_row = crt_row.offset(inc as isize);
                        crtn = 0 as libc::c_int as size_t;
                        result = result.wrapping_add(1);
                        result;
                        if crt_row == end_row {
                            break;
                        }
                        b2 = b2.wrapping_sub(1);
                        b2;
                    }
                    if crt_row != end_row {
                        bmp_memset_fn
                            .expect(
                                "non-null function pointer",
                            )(
                            *crt_row,
                            crtn,
                            0 as libc::c_int,
                            dcrtn.wrapping_sub(crtn),
                        );
                    }
                } else {
                    len = if b2 as size_t <= endn.wrapping_sub(crtn) {
                        b2
                    } else {
                        endn.wrapping_sub(crtn) as libc::c_uint
                    };
                    if bmp_fread_fn
                        .expect(
                            "non-null function pointer",
                        )(*crt_row, crtn, len as size_t, stream) != len as size_t
                    {
                        break;
                    }
                    crtn = crtn.wrapping_add(len as size_t);
                }
            } else {
                len = if b1 as size_t <= endn.wrapping_sub(crtn) {
                    b1
                } else {
                    endn.wrapping_sub(crtn) as libc::c_uint
                };
                bmp_memset_fn
                    .expect(
                        "non-null function pointer",
                    )(*crt_row, crtn, b2 as libc::c_int, len as size_t);
                crtn = crtn.wrapping_add(len as size_t);
            }
        }
    } else {
        return 0 as libc::c_int as size_t
    }
    while crt_row != end_row {
        bmp_memset_fn
            .expect(
                "non-null function pointer",
            )(*crt_row, crtn, 0 as libc::c_int, endn.wrapping_sub(crtn));
        crtn = 0 as libc::c_int as size_t;
        crt_row = crt_row.offset(inc as isize);
    }
    return result;
}
unsafe extern "C" fn bmp_to_png_rows(
    mut row_pointers: png_bytepp,
    mut width: png_uint_32,
    mut height: png_uint_32,
    mut pixdepth: libc::c_uint,
    mut rgba_sig: png_bytep,
    mut rgba_shift: png_bytep,
) {
    let mut src_ptr: png_bytep = 0 as *mut png_byte;
    let mut dest_ptr: png_bytep = 0 as *mut png_byte;
    let mut rgba_mask: [libc::c_uint; 4] = [0; 4];
    let mut num_samples: libc::c_uint = 0;
    let mut sample: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    let mut wpix: libc::c_uint = 0;
    let mut dwpix: png_uint_32 = 0;
    let mut x: png_uint_32 = 0;
    let mut y: png_uint_32 = 0;
    let mut i: libc::c_uint = 0;
    if pixdepth == 24 as libc::c_int as libc::c_uint {
        y = 0 as libc::c_int as png_uint_32;
        while y < height {
            src_ptr = *row_pointers.offset(y as isize);
            x = 0 as libc::c_int as png_uint_32;
            while x < width {
                let mut tmp: png_byte = *src_ptr.offset(0 as libc::c_int as isize);
                *src_ptr
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *src_ptr.offset(2 as libc::c_int as isize);
                *src_ptr.offset(2 as libc::c_int as isize) = tmp;
                x = x.wrapping_add(1);
                x;
                src_ptr = src_ptr.offset(3 as libc::c_int as isize);
            }
            y = y.wrapping_add(1);
            y;
        }
        return;
    }
    num_samples = (if *rgba_sig.offset(3 as libc::c_int as isize) as libc::c_int
        != 0 as libc::c_int
    {
        4 as libc::c_int
    } else {
        3 as libc::c_int
    }) as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_samples {
        rgba_mask[i
            as usize] = ((1 as libc::c_uint)
            << *rgba_sig.offset(i as isize) as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        i;
    }
    if pixdepth == 16 as libc::c_int as libc::c_uint {
        y = 0 as libc::c_int as png_uint_32;
        while y < height {
            src_ptr = (*row_pointers.offset(y as isize))
                .offset(
                    (width.wrapping_sub(1 as libc::c_int as png_uint_32)
                        * 2 as libc::c_int as png_uint_32) as isize,
                );
            dest_ptr = (*row_pointers.offset(y as isize))
                .offset(
                    width
                        .wrapping_sub(1 as libc::c_int as png_uint_32)
                        .wrapping_mul(num_samples) as isize,
                );
            x = 0 as libc::c_int as png_uint_32;
            while x < width {
                wpix = (*src_ptr.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*src_ptr.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                i = 0 as libc::c_int as libc::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample = wpix >> *rgba_shift.offset(i as isize) as libc::c_int
                        & mask;
                    *dest_ptr
                        .offset(
                            i as isize,
                        ) = sample
                        .wrapping_mul(255 as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            mask.wrapping_div(2 as libc::c_int as libc::c_uint),
                        )
                        .wrapping_div(mask) as png_byte;
                    i = i.wrapping_add(1);
                    i;
                }
                x = x.wrapping_add(1);
                x;
                src_ptr = src_ptr.offset(-(2 as libc::c_int as isize));
                dest_ptr = dest_ptr.offset(-(num_samples as isize));
            }
            y = y.wrapping_add(1);
            y;
        }
    } else if pixdepth == 32 as libc::c_int as libc::c_uint {
        y = 0 as libc::c_int as png_uint_32;
        while y < height {
            dest_ptr = *row_pointers.offset(y as isize);
            src_ptr = dest_ptr;
            x = 0 as libc::c_int as png_uint_32;
            while x < width {
                dwpix = (*src_ptr.offset(0 as libc::c_int as isize) as png_uint_32)
                    .wrapping_add(
                        (*src_ptr.offset(1 as libc::c_int as isize) as png_uint_32)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*src_ptr.offset(2 as libc::c_int as isize) as png_uint_32)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*src_ptr.offset(3 as libc::c_int as isize) as png_uint_32)
                            << 24 as libc::c_int,
                    );
                i = 0 as libc::c_int as libc::c_uint;
                while i < num_samples {
                    mask = rgba_mask[i as usize];
                    sample = dwpix >> *rgba_shift.offset(i as isize) as libc::c_int
                        & mask;
                    *dest_ptr
                        .offset(
                            i as isize,
                        ) = sample
                        .wrapping_mul(255 as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            mask.wrapping_div(2 as libc::c_int as libc::c_uint),
                        )
                        .wrapping_div(mask) as png_byte;
                    i = i.wrapping_add(1);
                    i;
                }
                x = x.wrapping_add(1);
                x;
                src_ptr = src_ptr.offset(4 as libc::c_int as isize);
                dest_ptr = dest_ptr.offset(num_samples as isize);
            }
            y = y.wrapping_add(1);
            y;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_bmp(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> libc::c_int {
    static mut bmp_fmt_name: [libc::c_char; 4] = unsafe {
        *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"BMP\0")
    };
    static mut os2bmp_fmt_long_name: [libc::c_char; 12] = unsafe {
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"OS/2 Bitmap\0")
    };
    static mut winbmp_fmt_long_name: [libc::c_char; 15] = unsafe {
        *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"Windows Bitmap\0")
    };
    let mut bihsize: png_uint_32 = 0;
    if sig_size < (14 as libc::c_int + 4 as libc::c_int) as size_t {
        return -(1 as libc::c_int);
    }
    if bmp_get_word(sig) != 0x4d42 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    bihsize = bmp_get_dword(sig.offset(14 as libc::c_int as isize));
    if bihsize > 0x7fffffff as libc::c_long as png_uint_32
        || bihsize != 12 as libc::c_int as png_uint_32
            && bihsize < 40 as libc::c_int as png_uint_32
    {
        return 0 as libc::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = bmp_fmt_name.as_ptr();
    }
    if !fmt_long_name_ptr.is_null() {
        if bihsize == 12 as libc::c_int as png_uint_32 {
            *fmt_long_name_ptr = os2bmp_fmt_long_name.as_ptr();
        } else {
            *fmt_long_name_ptr = winbmp_fmt_long_name.as_ptr();
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_bmp(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut bfh: [png_byte; 138] = [0; 138];
    let bih: png_bytep = bfh.as_mut_ptr().offset(14 as libc::c_int as isize);
    let mut rgbq: [png_byte; 4] = [0; 4];
    let mut offbits: png_uint_32 = 0;
    let mut bihsize: png_uint_32 = 0;
    let mut skip: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut rowsize: png_uint_32 = 0;
    let mut topdown: libc::c_int = 0;
    let mut pixdepth: libc::c_uint = 0;
    let mut compression: png_uint_32 = 0;
    let mut palsize: libc::c_uint = 0;
    let mut palnum: libc::c_uint = 0;
    let mut rgba_mask: [png_uint_32; 4] = [0; 4];
    let mut rgba_sig: [png_byte; 4] = [0; 4];
    let mut rgba_shift: [png_byte; 4] = [0; 4];
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut palette: [png_color; 256] = [png_color_struct {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut sig_bit: png_color_8 = png_color_8_struct {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    };
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut begin_row: png_bytepp = 0 as *mut *mut png_byte;
    let mut end_row: png_bytepp = 0 as *mut *mut png_byte;
    let mut i: libc::c_uint = 0;
    let mut y: size_t = 0;
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        if fread(
            bfh.as_mut_ptr() as *mut libc::c_void,
            (14 as libc::c_int + 4 as libc::c_int) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            stream,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            i = i.wrapping_add(1);
            i;
        } else if bmp_get_word(bfh.as_mut_ptr().offset(0 as libc::c_int as isize))
            == 0x4d42 as libc::c_int as libc::c_uint
        {
            break;
        }
        if fread(
            bfh.as_mut_ptr() as *mut libc::c_void,
            (128 as libc::c_int - 14 as libc::c_int - 4 as libc::c_int) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            stream,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            i = i.wrapping_add(1);
            i;
        }
        if i > 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    offbits = bmp_get_dword(bfh.as_mut_ptr().offset(10 as libc::c_int as isize));
    bihsize = bmp_get_dword(bfh.as_mut_ptr().offset(14 as libc::c_int as isize));
    if offbits > 0x7fffffff as libc::c_long as png_uint_32
        || bihsize > 0x7fffffff as libc::c_long as png_uint_32
        || offbits < bihsize.wrapping_add(14 as libc::c_int as png_uint_32)
        || bihsize != 12 as libc::c_int as png_uint_32
            && bihsize < 40 as libc::c_int as png_uint_32
    {
        return 0 as libc::c_int;
    }
    if bihsize > 124 as libc::c_int as png_uint_32 {
        skip = bihsize.wrapping_sub(124 as libc::c_int as png_uint_32);
        bihsize = 124 as libc::c_int as png_uint_32;
    } else {
        skip = 0 as libc::c_int as png_uint_32;
    }
    if fread(
        bih.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        bihsize.wrapping_sub(4 as libc::c_int as png_uint_32) as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if skip > 0 as libc::c_int as png_uint_32 {
        if fseek(stream, skip as libc::c_long, 1 as libc::c_int) != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    skip = offbits.wrapping_sub(bihsize).wrapping_sub(14 as libc::c_int as png_uint_32);
    topdown = 0 as libc::c_int;
    if bihsize < 40 as libc::c_int as png_uint_32 {
        width = bmp_get_word(bih.offset(4 as libc::c_int as isize));
        height = bmp_get_word(bih.offset(6 as libc::c_int as isize));
        pixdepth = bmp_get_word(bih.offset(10 as libc::c_int as isize));
        compression = 0 as libc::c_int as png_uint_32;
        palsize = 3 as libc::c_int as libc::c_uint;
    } else {
        width = bmp_get_dword(bih.offset(4 as libc::c_int as isize));
        height = bmp_get_dword(bih.offset(8 as libc::c_int as isize));
        pixdepth = bmp_get_word(bih.offset(14 as libc::c_int as isize));
        compression = bmp_get_dword(bih.offset(16 as libc::c_int as isize));
        palsize = 4 as libc::c_int as libc::c_uint;
        if height > 0x7fffffff as libc::c_long as png_uint_32 {
            height = (-(1 as libc::c_int) as png_uint_32)
                .wrapping_sub(height)
                .wrapping_add(1 as libc::c_int as png_uint_32);
            topdown = 1 as libc::c_int;
        }
        if bihsize == 40 as libc::c_int as png_uint_32
            && compression == 3 as libc::c_int as png_uint_32
        {
            i = if skip <= 16 as libc::c_int as png_uint_32 {
                skip
            } else {
                16 as libc::c_int as libc::c_uint
            };
            if fread(
                bih.offset(40 as libc::c_int as isize) as *mut libc::c_void,
                i as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                stream,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                return 0 as libc::c_int;
            }
            bihsize = (bihsize as libc::c_uint).wrapping_add(i) as png_uint_32
                as png_uint_32;
            skip = (skip as libc::c_uint).wrapping_sub(i) as png_uint_32 as png_uint_32;
        }
    }
    memset(
        rgba_mask.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[png_uint_32; 4]>() as libc::c_ulong,
    );
    if pixdepth > 8 as libc::c_int as libc::c_uint {
        if compression == 0 as libc::c_int as png_uint_32 {
            if pixdepth == 16 as libc::c_int as libc::c_uint {
                compression = 3 as libc::c_int as png_uint_32;
                rgba_mask[0 as libc::c_int
                    as usize] = 0x7c00 as libc::c_int as png_uint_32;
                rgba_mask[1 as libc::c_int
                    as usize] = 0x3e0 as libc::c_int as png_uint_32;
                rgba_mask[2 as libc::c_int
                    as usize] = 0x1f as libc::c_int as png_uint_32;
            } else {
                rgba_mask[0 as libc::c_int
                    as usize] = 0xff0000 as libc::c_long as png_uint_32;
                rgba_mask[1 as libc::c_int
                    as usize] = 0xff00 as libc::c_long as png_uint_32;
                rgba_mask[2 as libc::c_int
                    as usize] = 0xff as libc::c_long as png_uint_32;
            }
        } else if compression == 3 as libc::c_int as png_uint_32 {
            if bihsize >= (40 as libc::c_int + 12 as libc::c_int) as png_uint_32 {
                rgba_mask[0 as libc::c_int
                    as usize] = bmp_get_dword(bih.offset(40 as libc::c_int as isize));
                rgba_mask[1 as libc::c_int
                    as usize] = bmp_get_dword(bih.offset(44 as libc::c_int as isize));
                rgba_mask[2 as libc::c_int
                    as usize] = bmp_get_dword(bih.offset(48 as libc::c_int as isize));
            } else {
                png_error(
                    png_ptr as *const png_struct,
                    b"Missing color mask in BMP file\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        if bihsize >= (40 as libc::c_int + 16 as libc::c_int) as png_uint_32 {
            rgba_mask[3 as libc::c_int
                as usize] = bmp_get_dword(bih.offset(52 as libc::c_int as isize));
        }
    }
    match compression {
        0 => {
            if pixdepth > 0 as libc::c_int as libc::c_uint
                && (32 as libc::c_int as libc::c_uint).wrapping_rem(pixdepth)
                    != 0 as libc::c_int as libc::c_uint
                && pixdepth != 24 as libc::c_int as libc::c_uint
            {
                pixdepth = 0 as libc::c_int as libc::c_uint;
            }
        }
        1 => {
            if pixdepth != 8 as libc::c_int as libc::c_uint {
                pixdepth = 0 as libc::c_int as libc::c_uint;
            }
        }
        2 => {
            if pixdepth != 4 as libc::c_int as libc::c_uint {
                pixdepth = 0 as libc::c_int as libc::c_uint;
            }
        }
        3 => {
            if pixdepth != 16 as libc::c_int as libc::c_uint
                && pixdepth != 32 as libc::c_int as libc::c_uint
            {
                pixdepth = 0 as libc::c_int as libc::c_uint;
            }
        }
        4 => {
            png_error(
                png_ptr as *const png_struct,
                b"JPEG-compressed BMP files are not supported\0" as *const u8
                    as *const libc::c_char,
            );
        }
        5 => {
            if ungetc(getc(stream), stream) == 0 as libc::c_int {
                png_set_sig_bytes(png_ptr, 8 as libc::c_int);
            }
            png_set_read_fn(png_ptr, stream as png_voidp, None);
            png_read_png(png_ptr, info_ptr, 0 as libc::c_int, 0 as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        _ => {
            png_error(
                png_ptr as *const png_struct,
                b"Unsupported compression method in BMP file\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if width == 0 as libc::c_int as png_uint_32
        || width > 0x7fffffff as libc::c_long as png_uint_32
        || height == 0 as libc::c_int as png_uint_32
    {
        png_error(
            png_ptr as *const png_struct,
            b"Invalid image dimensions in BMP file\0" as *const u8 as *const libc::c_char,
        );
    }
    if pixdepth == 0 as libc::c_int as libc::c_uint {
        png_error(
            png_ptr as *const png_struct,
            b"Invalid pixel depth in BMP file\0" as *const u8 as *const libc::c_char,
        );
    }
    if pixdepth <= 8 as libc::c_int as libc::c_uint {
        palnum = skip.wrapping_div(palsize);
        if palnum > 256 as libc::c_int as libc::c_uint {
            palnum = 256 as libc::c_int as libc::c_uint;
        }
        skip = (skip as libc::c_uint).wrapping_sub(palsize.wrapping_mul(palnum))
            as png_uint_32 as png_uint_32;
        rowsize = width
            .wrapping_add((32 as libc::c_int as libc::c_uint).wrapping_div(pixdepth))
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div((32 as libc::c_int as libc::c_uint).wrapping_div(pixdepth))
            .wrapping_mul(4 as libc::c_int as libc::c_uint);
        bit_depth = pixdepth as libc::c_int;
        color_type = if palnum > 0 as libc::c_int as libc::c_uint {
            2 as libc::c_int | 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        palnum = 0 as libc::c_int as libc::c_uint;
        bit_depth = 8 as libc::c_int;
        match pixdepth {
            16 => {
                rowsize = (width * 2 as libc::c_int as png_uint_32)
                    .wrapping_add(3 as libc::c_int as png_uint_32)
                    & !(3 as libc::c_int) as png_uint_32;
            }
            24 => {
                rowsize = (width * 3 as libc::c_int as png_uint_32)
                    .wrapping_add(3 as libc::c_int as png_uint_32)
                    & !(3 as libc::c_int) as png_uint_32;
            }
            32 => {
                rowsize = width * 4 as libc::c_int as png_uint_32;
            }
            _ => {
                bit_depth = 0 as libc::c_int;
                rowsize = 0 as libc::c_int as png_uint_32;
            }
        }
        if rowsize / width < pixdepth.wrapping_div(8 as libc::c_int as libc::c_uint) {
            rowsize = 0 as libc::c_int as png_uint_32;
        }
        color_type = if rgba_mask[3 as libc::c_int as usize]
            != 0 as libc::c_int as png_uint_32
        {
            2 as libc::c_int | 4 as libc::c_int
        } else {
            2 as libc::c_int
        };
    }
    if rowsize == 0 as libc::c_int as png_uint_32 {
        png_error(
            png_ptr as *const png_struct,
            b"Can't handle exceedingly large BMP dimensions\0" as *const u8
                as *const libc::c_char,
        );
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        bit_depth,
        color_type,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if pixdepth > 8 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as libc::c_uint;
        while i < 4 as libc::c_int as libc::c_uint {
            bmp_process_mask(
                rgba_mask[i as usize],
                &mut *rgba_sig.as_mut_ptr().offset(i as isize),
                &mut *rgba_shift.as_mut_ptr().offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        if rgba_sig[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int
            || rgba_sig[1 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int
            || rgba_sig[2 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int
        {
            png_error(
                png_ptr as *const png_struct,
                b"Invalid color mask in BMP file\0" as *const u8 as *const libc::c_char,
            );
        }
        if rgba_sig[0 as libc::c_int as usize] as libc::c_int != 8 as libc::c_int
            || rgba_sig[1 as libc::c_int as usize] as libc::c_int != 8 as libc::c_int
            || rgba_sig[2 as libc::c_int as usize] as libc::c_int != 8 as libc::c_int
            || rgba_sig[3 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
                && rgba_sig[3 as libc::c_int as usize] as libc::c_int != 8 as libc::c_int
        {
            sig_bit.red = rgba_sig[0 as libc::c_int as usize];
            sig_bit.green = rgba_sig[1 as libc::c_int as usize];
            sig_bit.blue = rgba_sig[2 as libc::c_int as usize];
            sig_bit.alpha = rgba_sig[3 as libc::c_int as usize];
            png_set_sBIT(
                png_ptr as *const png_struct,
                info_ptr,
                &mut sig_bit as *mut png_color_8 as png_const_color_8p,
            );
        }
    }
    if palnum > 0 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as libc::c_uint;
        while i < palnum {
            if fread(
                rgbq.as_mut_ptr() as *mut libc::c_void,
                palsize as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                stream,
            ) != 1 as libc::c_int as libc::c_ulong
            {
                break;
            }
            palette[i as usize].red = rgbq[2 as libc::c_int as usize];
            palette[i as usize].green = rgbq[1 as libc::c_int as usize];
            palette[i as usize].blue = rgbq[0 as libc::c_int as usize];
            i = i.wrapping_add(1);
            i;
        }
        png_set_PLTE(
            png_ptr,
            info_ptr,
            palette.as_mut_ptr() as png_const_colorp,
            i as libc::c_int,
        );
        if i != palnum {
            png_error(
                png_ptr as *const png_struct,
                b"Error reading color palette in BMP file\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    row_pointers = pngx_malloc_rows_extended(
        png_ptr,
        info_ptr,
        rowsize as pngx_alloc_size_t,
        -(1 as libc::c_int),
    );
    if topdown != 0 {
        begin_row = row_pointers;
        end_row = row_pointers.offset(height as isize);
    } else {
        begin_row = row_pointers
            .offset(height as isize)
            .offset(-(1 as libc::c_int as isize));
        end_row = row_pointers.offset(-(1 as libc::c_int as isize));
    }
    if skip > 0 as libc::c_int as png_uint_32 {
        fseek(stream, skip as libc::c_long, 1 as libc::c_int);
    }
    y = bmp_read_rows(begin_row, end_row, rowsize as size_t, compression, stream);
    if pixdepth > 8 as libc::c_int as libc::c_uint {
        bmp_to_png_rows(
            row_pointers,
            width,
            height,
            pixdepth,
            rgba_sig.as_mut_ptr(),
            rgba_shift.as_mut_ptr(),
        );
    }
    if y != height as size_t {
        png_error(
            png_ptr as *const png_struct,
            b"Error reading BMP file\0" as *const u8 as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
