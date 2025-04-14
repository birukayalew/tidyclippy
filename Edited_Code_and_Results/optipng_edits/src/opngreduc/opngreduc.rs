use ::libc;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_free_data(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        free_me: png_uint_32,
        num: libc::c_int,
    );
    fn png_warning(png_ptr: png_const_structrp, warning_message: png_const_charp);
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_get_rows(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_bytepp;
    fn png_get_channels(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_byte;
    fn png_get_image_width(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_image_height(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_bit_depth(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_byte;
    fn png_get_color_type(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_byte;
    fn png_get_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_get_hIST(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        hist: *mut png_uint_16p,
    ) -> png_uint_32;
    fn png_get_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        width: *mut png_uint_32,
        height: *mut png_uint_32,
        bit_depth: *mut libc::c_int,
        color_type: *mut libc::c_int,
        interlace_method: *mut libc::c_int,
        compression_method: *mut libc::c_int,
        filter_method: *mut libc::c_int,
    ) -> png_uint_32;
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
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut libc::c_int,
    ) -> png_uint_32;
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: libc::c_int,
    );
    fn png_get_sBIT(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        sig_bit: *mut png_color_8p,
    ) -> png_uint_32;
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut libc::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: libc::c_int,
        trans_color: png_const_color_16p,
    );
    fn png_set_invalid(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        mask: libc::c_int,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type png_byte = libc::c_uchar;
pub type png_uint_16 = libc::c_ushort;
pub type png_uint_32 = libc::c_uint;
pub type png_alloc_size_t = size_t;
pub type png_voidp = *mut libc::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
pub type png_uint_16p = *mut png_uint_16;
pub type png_const_charp = *const libc::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_color = png_color_struct;
pub type png_colorp = *mut png_color;
pub type png_const_colorp = *const png_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_color_16 = png_color_16_struct;
pub type png_color_16p = *mut png_color_16;
pub type png_const_color_16p = *const png_color_16;
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
pub type png_color_8p = *mut png_color_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_row_info_struct {
    pub width: png_uint_32,
    pub rowbytes: size_t,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
}
pub type png_row_info = png_row_info_struct;
pub type png_row_infop = *mut png_row_info;
#[no_mangle]
pub unsafe extern "C" fn opng_validate_image(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> libc::c_int {
    if png_get_bit_depth(png_ptr as *const png_struct, info_ptr as *const png_info)
        as libc::c_int == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if png_get_color_type(png_ptr as *const png_struct, info_ptr as *const png_info)
        as libc::c_int & 1 as libc::c_int != 0
    {
        if png_get_valid(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
            0x8 as libc::c_uint,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x8000 as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn opng_insert_palette_entry(
    mut palette: png_colorp,
    mut num_palette: *mut libc::c_int,
    mut trans_alpha: png_bytep,
    mut num_trans: *mut libc::c_int,
    mut max_tuples: libc::c_int,
    mut red: libc::c_uint,
    mut green: libc::c_uint,
    mut blue: libc::c_uint,
    mut alpha: libc::c_uint,
    mut index: *mut libc::c_int,
) -> libc::c_int {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if *num_palette >= 0 as libc::c_int && *num_palette <= max_tuples {} else {
        __assert_fail(
            b"*num_palette >= 0 && *num_palette <= max_tuples\0" as *const u8
                as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4762: {
        if *num_palette >= 0 as libc::c_int && *num_palette <= max_tuples {} else {
            __assert_fail(
                b"*num_palette >= 0 && *num_palette <= max_tuples\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 135],
                    &[libc::c_char; 135],
                >(
                    b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if *num_trans >= 0 as libc::c_int && *num_trans <= *num_palette {} else {
        __assert_fail(
            b"*num_trans >= 0 && *num_trans <= *num_palette\0" as *const u8
                as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4701: {
        if *num_trans >= 0 as libc::c_int && *num_trans <= *num_palette {} else {
            __assert_fail(
                b"*num_trans >= 0 && *num_trans <= *num_palette\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 135],
                    &[libc::c_char; 135],
                >(
                    b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if alpha < 255 as libc::c_int as libc::c_uint {
        low = 0 as libc::c_int;
        high = *num_trans - 1 as libc::c_int;
        while low <= high {
            mid = (low + high) / 2 as libc::c_int;
            cmp = if alpha as libc::c_int
                != *trans_alpha.offset(mid as isize) as libc::c_int
            {
                alpha as libc::c_int - *trans_alpha.offset(mid as isize) as libc::c_int
            } else if red as libc::c_int
                != (*palette.offset(mid as isize)).red as libc::c_int
            {
                red as libc::c_int - (*palette.offset(mid as isize)).red as libc::c_int
            } else if green as libc::c_int
                != (*palette.offset(mid as isize)).green as libc::c_int
            {
                green as libc::c_int
                    - (*palette.offset(mid as isize)).green as libc::c_int
            } else {
                blue as libc::c_int - (*palette.offset(mid as isize)).blue as libc::c_int
            };
            if cmp < 0 as libc::c_int {
                high = mid - 1 as libc::c_int;
            } else if cmp > 0 as libc::c_int {
                low = mid + 1 as libc::c_int;
            } else {
                *index = mid;
                return 0 as libc::c_int;
            }
        }
    } else {
        low = *num_trans;
        high = *num_palette - 1 as libc::c_int;
        while low <= high {
            mid = (low + high) / 2 as libc::c_int;
            cmp = if red as libc::c_int
                != (*palette.offset(mid as isize)).red as libc::c_int
            {
                red as libc::c_int - (*palette.offset(mid as isize)).red as libc::c_int
            } else if green as libc::c_int
                != (*palette.offset(mid as isize)).green as libc::c_int
            {
                green as libc::c_int
                    - (*palette.offset(mid as isize)).green as libc::c_int
            } else {
                blue as libc::c_int - (*palette.offset(mid as isize)).blue as libc::c_int
            };
            if cmp < 0 as libc::c_int {
                high = mid - 1 as libc::c_int;
            } else if cmp > 0 as libc::c_int {
                low = mid + 1 as libc::c_int;
            } else {
                *index = mid;
                return 0 as libc::c_int;
            }
        }
    }
    if alpha > 255 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < *num_trans {
            cmp = if red as libc::c_int
                != (*palette.offset(i as isize)).red as libc::c_int
            {
                red as libc::c_int - (*palette.offset(i as isize)).red as libc::c_int
            } else if green as libc::c_int
                != (*palette.offset(i as isize)).green as libc::c_int
            {
                green as libc::c_int - (*palette.offset(i as isize)).green as libc::c_int
            } else {
                blue as libc::c_int - (*palette.offset(i as isize)).blue as libc::c_int
            };
            if cmp == 0 as libc::c_int {
                *index = i;
                return 0 as libc::c_int;
            }
            i += 1;
            i;
        }
    }
    if *num_palette >= max_tuples {
        *index = -(1 as libc::c_int);
        *num_trans = *index;
        *num_palette = *num_trans;
        return -(1 as libc::c_int);
    }
    if low >= 0 as libc::c_int && low <= *num_palette {} else {
        __assert_fail(
            b"low >= 0 && low <= *num_palette\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[libc::c_char; 135],
            >(
                b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4166: {
        if low >= 0 as libc::c_int && low <= *num_palette {} else {
            __assert_fail(
                b"low >= 0 && low <= *num_palette\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 135],
                    &[libc::c_char; 135],
                >(
                    b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = *num_palette;
    while i > low {
        *palette.offset(i as isize) = *palette.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    (*palette.offset(low as isize)).red = red as png_byte;
    (*palette.offset(low as isize)).green = green as png_byte;
    (*palette.offset(low as isize)).blue = blue as png_byte;
    *num_palette += 1;
    *num_palette;
    if alpha < 255 as libc::c_int as libc::c_uint {
        if low <= *num_trans {} else {
            __assert_fail(
                b"low <= *num_trans\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                188 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 135],
                    &[libc::c_char; 135],
                >(
                    b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4054: {
            if low <= *num_trans {} else {
                __assert_fail(
                    b"low <= *num_trans\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    188 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 135],
                        &[libc::c_char; 135],
                    >(
                        b"int opng_insert_palette_entry(png_colorp, int *, png_bytep, int *, int, unsigned int, unsigned int, unsigned int, unsigned int, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = *num_trans;
        while i > low {
            *trans_alpha
                .offset(
                    i as isize,
                ) = *trans_alpha.offset((i - 1 as libc::c_int) as isize);
            i -= 1;
            i;
        }
        *trans_alpha.offset(low as isize) = alpha as png_byte;
        *num_trans += 1;
        *num_trans;
    }
    *index = low;
    return 1 as libc::c_int;
}
unsafe extern "C" fn opng_realloc_PLTE(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut num_palette: libc::c_int,
) {
    let mut buffer: [png_color; 256] = [png_color_struct {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut palette: png_colorp = 0 as *mut png_color;
    let mut src_num_palette: libc::c_int = 0;
    let mut buffer_ptr: png_bytep = 0 as *mut png_byte;
    let mut trans_alpha: png_bytep = 0 as *mut png_byte;
    let mut num_trans: libc::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    if num_palette > 0 as libc::c_int {} else {
        __assert_fail(
            b"num_palette > 0\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void opng_realloc_PLTE(png_structp, png_infop, int)\0"))
                .as_ptr(),
        );
    }
    'c_3638: {
        if num_palette > 0 as libc::c_int {} else {
            __assert_fail(
                b"num_palette > 0\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void opng_realloc_PLTE(png_structp, png_infop, int)\0"))
                    .as_ptr(),
            );
        }
    };
    src_num_palette = 0 as libc::c_int;
    png_get_PLTE(
        png_ptr as *const png_struct,
        info_ptr,
        &mut palette,
        &mut src_num_palette,
    );
    if num_palette == src_num_palette {
        return;
    }
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        palette as *const libc::c_void,
        (num_palette as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<png_color>() as libc::c_ulong),
    );
    if num_palette > src_num_palette {
        memset(
            buffer.as_mut_ptr().offset(src_num_palette as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((num_palette - src_num_palette) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<png_color>() as libc::c_ulong),
        );
    }
    png_set_PLTE(
        png_ptr,
        info_ptr,
        buffer.as_mut_ptr() as png_const_colorp,
        num_palette,
    );
    if num_palette < src_num_palette {
        if png_get_tRNS(
            png_ptr as *const png_struct,
            info_ptr,
            &mut trans_alpha,
            &mut num_trans,
            0 as *mut png_color_16p,
        ) != 0 && num_trans > num_palette
        {
            buffer_ptr = buffer.as_mut_ptr() as png_voidp as png_bytep;
            png_warning(
                png_ptr as *const png_struct,
                b"Too many alpha values in tRNS\0" as *const u8 as *const libc::c_char,
            );
            num_trans = num_palette;
            memcpy(
                buffer_ptr as *mut libc::c_void,
                trans_alpha as *const libc::c_void,
                num_trans as size_t,
            );
            png_set_tRNS(
                png_ptr,
                info_ptr,
                buffer_ptr as png_const_bytep,
                num_trans,
                0 as png_const_color_16p,
            );
        }
        if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background) != 0
            && (*background).index as libc::c_int >= num_palette
        {
            png_warning(
                png_ptr as *const png_struct,
                b"Invalid color index in bKGD\0" as *const u8 as *const libc::c_char,
            );
            png_set_invalid(
                png_ptr as *const png_struct,
                info_ptr,
                0x20 as libc::c_uint as libc::c_int,
            );
        }
    }
}
unsafe extern "C" fn opng_realloc_tRNS(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut num_trans: libc::c_int,
) {
    let mut buffer: [png_byte; 256] = [0; 256];
    let mut trans_alpha: png_bytep = 0 as *mut png_byte;
    let mut src_num_trans: libc::c_int = 0;
    if num_trans > 0 as libc::c_int {} else {
        __assert_fail(
            b"num_trans > 0\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void opng_realloc_tRNS(png_structp, png_infop, int)\0"))
                .as_ptr(),
        );
    }
    'c_7077: {
        if num_trans > 0 as libc::c_int {} else {
            __assert_fail(
                b"num_trans > 0\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                270 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void opng_realloc_tRNS(png_structp, png_infop, int)\0"))
                    .as_ptr(),
            );
        }
    };
    src_num_trans = 0 as libc::c_int;
    png_get_tRNS(
        png_ptr as *const png_struct,
        info_ptr,
        &mut trans_alpha,
        &mut src_num_trans,
        0 as *mut png_color_16p,
    );
    if num_trans == src_num_trans {
        return;
    }
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        trans_alpha as *const libc::c_void,
        num_trans as size_t,
    );
    if num_trans > src_num_trans {
        memset(
            buffer.as_mut_ptr().offset(src_num_trans as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (num_trans - src_num_trans) as libc::c_ulong,
        );
    }
    png_set_tRNS(
        png_ptr,
        info_ptr,
        buffer.as_mut_ptr() as png_const_bytep,
        num_trans,
        0 as png_const_color_16p,
    );
}
unsafe extern "C" fn opng_get_alpha_row(
    mut row_info_ptr: png_row_infop,
    mut trans_color: png_color_16p,
    mut row: png_bytep,
    mut alpha_row: png_bytep,
) {
    let mut sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut color_type: libc::c_int = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut channels: libc::c_int = 0;
    let mut trans_red: png_byte = 0;
    let mut trans_green: png_byte = 0;
    let mut trans_blue: png_byte = 0;
    let mut trans_gray: png_byte = 0;
    let mut i: png_uint_32 = 0;
    width = (*row_info_ptr).width;
    color_type = (*row_info_ptr).color_type as libc::c_int;
    bit_depth = (*row_info_ptr).bit_depth as libc::c_int;
    channels = (*row_info_ptr).channels as libc::c_int;
    if color_type & 1 as libc::c_int == 0 {} else {
        __assert_fail(
            b"!(color_type & 1)\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5512: {
        if color_type & 1 as libc::c_int == 0 {} else {
            __assert_fail(
                b"!(color_type & 1)\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                299 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if bit_depth == 8 as libc::c_int {} else {
        __assert_fail(
            b"bit_depth == 8\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5474: {
        if bit_depth == 8 as libc::c_int {} else {
            __assert_fail(
                b"bit_depth == 8\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                300 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if color_type & 4 as libc::c_int == 0 {
        if trans_color.is_null() {
            memset(alpha_row as *mut libc::c_void, 255 as libc::c_int, width as size_t);
            return;
        }
        if color_type == 2 as libc::c_int {
            if channels == 3 as libc::c_int {} else {
                __assert_fail(
                    b"channels == 3\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    312 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 76],
                        &[libc::c_char; 76],
                    >(
                        b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5407: {
                if channels == 3 as libc::c_int {} else {
                    __assert_fail(
                        b"channels == 3\0" as *const u8 as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        312 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 76],
                            &[libc::c_char; 76],
                        >(
                            b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            trans_red = (*trans_color).red as png_byte;
            trans_green = (*trans_color).green as png_byte;
            trans_blue = (*trans_color).blue as png_byte;
            sample_ptr = row;
            i = 0 as libc::c_int as png_uint_32;
            while i < width {
                *alpha_row
                    .offset(
                        i as isize,
                    ) = (if *sample_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    == trans_red as libc::c_int
                    && *sample_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == trans_green as libc::c_int
                    && *sample_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                        == trans_blue as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    255 as libc::c_int
                }) as png_byte;
                i = i.wrapping_add(1);
                i;
                sample_ptr = sample_ptr.offset(3 as libc::c_int as isize);
            }
        } else {
            if color_type == 0 as libc::c_int {} else {
                __assert_fail(
                    b"color_type == 0\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    325 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 76],
                        &[libc::c_char; 76],
                    >(
                        b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5275: {
                if color_type == 0 as libc::c_int {} else {
                    __assert_fail(
                        b"color_type == 0\0" as *const u8 as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        325 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 76],
                            &[libc::c_char; 76],
                        >(
                            b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if channels == 1 as libc::c_int {} else {
                __assert_fail(
                    b"channels == 1\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    326 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 76],
                        &[libc::c_char; 76],
                    >(
                        b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5238: {
                if channels == 1 as libc::c_int {} else {
                    __assert_fail(
                        b"channels == 1\0" as *const u8 as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        326 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 76],
                            &[libc::c_char; 76],
                        >(
                            b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            trans_gray = (*trans_color).gray as png_byte;
            i = 0 as libc::c_int as png_uint_32;
            while i < width {
                *alpha_row
                    .offset(
                        i as isize,
                    ) = (if *row.offset(i as isize) as libc::c_int
                    == trans_gray as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    255 as libc::c_int
                }) as png_byte;
                i = i.wrapping_add(1);
                i;
            }
        }
        return;
    }
    if channels > 1 as libc::c_int {} else {
        __assert_fail(
            b"channels > 1\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5144: {
        if channels > 1 as libc::c_int {} else {
            __assert_fail(
                b"channels > 1\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                335 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sample_ptr = row.offset((channels - 1 as libc::c_int) as isize);
    i = 0 as libc::c_int as png_uint_32;
    while i < width {
        *alpha_row = *sample_ptr;
        i = i.wrapping_add(1);
        i;
        sample_ptr = sample_ptr.offset(channels as isize);
        alpha_row = alpha_row.offset(1);
        alpha_row;
    }
}
unsafe extern "C" fn opng_analyze_bits(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut component_ptr: png_bytep = 0 as *mut png_byte;
    let mut height: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut byte_depth: libc::c_int = 0;
    let mut channels: libc::c_int = 0;
    let mut sample_size: libc::c_int = 0;
    let mut offset_alpha: libc::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut bit_depth,
        &mut color_type,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    );
    if bit_depth < 8 as libc::c_int {
        return 0 as libc::c_int as png_uint_32;
    }
    if color_type & 1 as libc::c_int != 0 {
        return 0 as libc::c_int as png_uint_32;
    }
    byte_depth = bit_depth / 8 as libc::c_int;
    channels = png_get_channels(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    ) as libc::c_int;
    sample_size = channels * byte_depth;
    offset_alpha = (channels - 1 as libc::c_int) * byte_depth;
    reductions
        &= (0x1 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int) as png_uint_32;
    if bit_depth <= 8 as libc::c_int {
        reductions &= !(0x1 as libc::c_int) as png_uint_32;
    }
    if color_type & 2 as libc::c_int == 0 {
        reductions &= !(0x4 as libc::c_int) as png_uint_32;
    }
    if color_type & 4 as libc::c_int == 0 {
        reductions &= !(0x8 as libc::c_int) as png_uint_32;
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background) != 0 {
        if reductions & 0x1 as libc::c_int as png_uint_32 != 0 {
            if (*background).red as libc::c_int % 257 as libc::c_int != 0 as libc::c_int
                || (*background).green as libc::c_int % 257 as libc::c_int
                    != 0 as libc::c_int
                || (*background).blue as libc::c_int % 257 as libc::c_int
                    != 0 as libc::c_int
                || (*background).gray as libc::c_int % 257 as libc::c_int
                    != 0 as libc::c_int
            {
                reductions &= !(0x1 as libc::c_int) as png_uint_32;
            }
        }
        if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
            if (*background).red as libc::c_int != (*background).green as libc::c_int
                || (*background).red as libc::c_int != (*background).blue as libc::c_int
            {
                reductions &= !(0x4 as libc::c_int) as png_uint_32;
            }
        }
    }
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    i = 0 as libc::c_int as png_uint_32;
    while i < height {
        if reductions == 0 as libc::c_int as png_uint_32 {
            return 0 as libc::c_int as png_uint_32;
        }
        if reductions & 0x1 as libc::c_int as png_uint_32 != 0 {
            component_ptr = *row_ptr;
            j = 0 as libc::c_int as png_uint_32;
            while j < channels as png_uint_32 * width {
                if *component_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    != *component_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                {
                    reductions &= !(0x1 as libc::c_int) as png_uint_32;
                    break;
                } else {
                    j = j.wrapping_add(1);
                    j;
                    component_ptr = component_ptr.offset(2 as libc::c_int as isize);
                }
            }
        }
        if bit_depth == 8 as libc::c_int {
            if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
                component_ptr = *row_ptr;
                j = 0 as libc::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != *component_ptr.offset(1 as libc::c_int as isize)
                            as libc::c_int
                        || *component_ptr.offset(0 as libc::c_int as isize)
                            as libc::c_int
                            != *component_ptr.offset(2 as libc::c_int as isize)
                                as libc::c_int
                    {
                        reductions &= !(0x4 as libc::c_int) as png_uint_32;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        j;
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
            if reductions & 0x8 as libc::c_int as png_uint_32 != 0 {
                component_ptr = (*row_ptr).offset(offset_alpha as isize);
                j = 0 as libc::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != 255 as libc::c_int
                    {
                        reductions &= !(0x8 as libc::c_int) as png_uint_32;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        j;
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
        } else {
            if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
                component_ptr = *row_ptr;
                j = 0 as libc::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != *component_ptr.offset(2 as libc::c_int as isize)
                            as libc::c_int
                        || *component_ptr.offset(0 as libc::c_int as isize)
                            as libc::c_int
                            != *component_ptr.offset(4 as libc::c_int as isize)
                                as libc::c_int
                        || *component_ptr.offset(1 as libc::c_int as isize)
                            as libc::c_int
                            != *component_ptr.offset(3 as libc::c_int as isize)
                                as libc::c_int
                        || *component_ptr.offset(1 as libc::c_int as isize)
                            as libc::c_int
                            != *component_ptr.offset(5 as libc::c_int as isize)
                                as libc::c_int
                    {
                        reductions &= !(0x4 as libc::c_int) as png_uint_32;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        j;
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
            if reductions & 0x8 as libc::c_int as png_uint_32 != 0 {
                component_ptr = (*row_ptr).offset(offset_alpha as isize);
                j = 0 as libc::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != 255 as libc::c_int
                        || *component_ptr.offset(1 as libc::c_int as isize)
                            as libc::c_int != 255 as libc::c_int
                    {
                        reductions &= !(0x8 as libc::c_int) as png_uint_32;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        j;
                        component_ptr = component_ptr.offset(sample_size as isize);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
        row_ptr = row_ptr.offset(1);
        row_ptr;
    }
    return reductions;
}
unsafe extern "C" fn opng_reduce_bits(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut src_ptr: png_bytep = 0 as *mut png_byte;
    let mut dest_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut interlace_type: libc::c_int = 0;
    let mut compression_type: libc::c_int = 0;
    let mut filter_type: libc::c_int = 0;
    let mut src_bit_depth: libc::c_int = 0;
    let mut dest_bit_depth: libc::c_int = 0;
    let mut src_byte_depth: libc::c_int = 0;
    let mut dest_byte_depth: libc::c_int = 0;
    let mut src_color_type: libc::c_int = 0;
    let mut dest_color_type: libc::c_int = 0;
    let mut src_channels: libc::c_int = 0;
    let mut dest_channels: libc::c_int = 0;
    let mut src_sample_size: libc::c_int = 0;
    let mut dest_sample_size: libc::c_int = 0;
    let mut tran_tbl: [libc::c_int; 8] = [0; 8];
    let mut trans_color: png_color_16p = 0 as *mut png_color_16;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut sig_bits: png_color_8p = 0 as *mut png_color_8;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    let mut k: libc::c_int = 0;
    reductions = opng_analyze_bits(png_ptr, info_ptr, reductions);
    if reductions == 0 as libc::c_int as png_uint_32 {
        return 0 as libc::c_int as png_uint_32;
    }
    png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut src_bit_depth,
        &mut src_color_type,
        &mut interlace_type,
        &mut compression_type,
        &mut filter_type,
    );
    if src_bit_depth >= 8 as libc::c_int {} else {
        __assert_fail(
            b"src_bit_depth >= 8\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            538 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0"))
                .as_ptr(),
        );
    }
    'c_9088: {
        if src_bit_depth >= 8 as libc::c_int {} else {
            __assert_fail(
                b"src_bit_depth >= 8\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                538 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if reductions & 0x1 as libc::c_int as png_uint_32 != 0 {
        if src_bit_depth == 16 as libc::c_int {} else {
            __assert_fail(
                b"src_bit_depth == 16\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                541 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9047: {
            if src_bit_depth == 16 as libc::c_int {} else {
                __assert_fail(
                    b"src_bit_depth == 16\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    541 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        dest_bit_depth = 8 as libc::c_int;
    } else {
        dest_bit_depth = src_bit_depth;
    }
    src_byte_depth = src_bit_depth / 8 as libc::c_int;
    dest_byte_depth = dest_bit_depth / 8 as libc::c_int;
    dest_color_type = src_color_type;
    if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
        if src_color_type & 2 as libc::c_int != 0 {} else {
            __assert_fail(
                b"src_color_type & 2\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                553 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8979: {
            if src_color_type & 2 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"src_color_type & 2\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    553 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        dest_color_type &= !(2 as libc::c_int);
    }
    if reductions & 0x8 as libc::c_int as png_uint_32 != 0 {
        if src_color_type & 4 as libc::c_int != 0 {} else {
            __assert_fail(
                b"src_color_type & 4\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                558 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8928: {
            if src_color_type & 4 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"src_color_type & 4\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    558 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        dest_color_type &= !(4 as libc::c_int);
    }
    src_channels = png_get_channels(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    ) as libc::c_int;
    dest_channels = (if dest_color_type & 2 as libc::c_int != 0 {
        3 as libc::c_int
    } else {
        1 as libc::c_int
    })
        + (if dest_color_type & 4 as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        });
    src_sample_size = src_channels * src_byte_depth;
    dest_sample_size = dest_channels * dest_byte_depth;
    k = 0 as libc::c_int;
    while k < 4 as libc::c_int * dest_byte_depth {
        tran_tbl[k as usize] = k * src_bit_depth / dest_bit_depth;
        k += 1;
        k;
    }
    if reductions & 0x4 as libc::c_int as png_uint_32 != 0
        && dest_color_type & 4 as libc::c_int != 0
    {
        tran_tbl[dest_byte_depth
            as usize] = tran_tbl[(3 as libc::c_int * dest_byte_depth) as usize];
        if dest_byte_depth == 2 as libc::c_int {
            tran_tbl[(dest_byte_depth + 1 as libc::c_int)
                as usize] = tran_tbl[(3 as libc::c_int * dest_byte_depth
                + 1 as libc::c_int) as usize];
        }
    }
    if src_sample_size > dest_sample_size {} else {
        __assert_fail(
            b"src_sample_size > dest_sample_size\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            583 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0"))
                .as_ptr(),
        );
    }
    'c_8749: {
        if src_sample_size > dest_sample_size {} else {
            __assert_fail(
                b"src_sample_size > dest_sample_size\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                583 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    i = 0 as libc::c_int as png_uint_32;
    while i < height {
        dest_ptr = *row_ptr;
        src_ptr = dest_ptr;
        j = 0 as libc::c_int as png_uint_32;
        while j < width {
            k = 0 as libc::c_int;
            while k < dest_sample_size {
                *dest_ptr
                    .offset(k as isize) = *src_ptr.offset(tran_tbl[k as usize] as isize);
                k += 1;
                k;
            }
            src_ptr = src_ptr.offset(src_sample_size as isize);
            dest_ptr = dest_ptr.offset(dest_sample_size as isize);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
        row_ptr = row_ptr.offset(1);
        row_ptr;
    }
    if png_get_tRNS(
        png_ptr as *const png_struct,
        info_ptr,
        0 as *mut png_bytep,
        0 as *mut libc::c_int,
        &mut trans_color,
    ) != 0
    {
        if reductions & 0x1 as libc::c_int as png_uint_32 != 0 {
            if (*trans_color).red as libc::c_int % 257 as libc::c_int == 0 as libc::c_int
                && (*trans_color).green as libc::c_int % 257 as libc::c_int
                    == 0 as libc::c_int
                && (*trans_color).blue as libc::c_int % 257 as libc::c_int
                    == 0 as libc::c_int
                && (*trans_color).gray as libc::c_int % 257 as libc::c_int
                    == 0 as libc::c_int
            {
                (*trans_color)
                    .red = ((*trans_color).red as libc::c_int & 255 as libc::c_int)
                    as png_uint_16;
                (*trans_color)
                    .green = ((*trans_color).green as libc::c_int & 255 as libc::c_int)
                    as png_uint_16;
                (*trans_color)
                    .blue = ((*trans_color).blue as libc::c_int & 255 as libc::c_int)
                    as png_uint_16;
                (*trans_color)
                    .gray = ((*trans_color).gray as libc::c_int & 255 as libc::c_int)
                    as png_uint_16;
            } else {
                png_free_data(
                    png_ptr as *const png_struct,
                    info_ptr,
                    0x2000 as libc::c_uint,
                    -(1 as libc::c_int),
                );
                png_set_invalid(
                    png_ptr as *const png_struct,
                    info_ptr,
                    0x10 as libc::c_uint as libc::c_int,
                );
            }
        }
        if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
            if (*trans_color).red as libc::c_int == (*trans_color).green as libc::c_int
                || (*trans_color).red as libc::c_int
                    == (*trans_color).blue as libc::c_int
            {
                (*trans_color).gray = (*trans_color).red;
            } else {
                png_free_data(
                    png_ptr as *const png_struct,
                    info_ptr,
                    0x2000 as libc::c_uint,
                    -(1 as libc::c_int),
                );
                png_set_invalid(
                    png_ptr as *const png_struct,
                    info_ptr,
                    0x10 as libc::c_uint as libc::c_int,
                );
            }
        }
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background) != 0 {
        if reductions & 0x1 as libc::c_int as png_uint_32 != 0 {
            (*background)
                .red = ((*background).red as libc::c_int & 255 as libc::c_int)
                as png_uint_16;
            (*background)
                .green = ((*background).green as libc::c_int & 255 as libc::c_int)
                as png_uint_16;
            (*background)
                .blue = ((*background).blue as libc::c_int & 255 as libc::c_int)
                as png_uint_16;
            (*background)
                .gray = ((*background).gray as libc::c_int & 255 as libc::c_int)
                as png_uint_16;
        }
        if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
            (*background).gray = (*background).red;
        }
    }
    if png_get_sBIT(png_ptr as *const png_struct, info_ptr, &mut sig_bits) != 0 {
        if reductions & 0x1 as libc::c_int as png_uint_32 != 0 {
            if (*sig_bits).red as libc::c_int > 8 as libc::c_int {
                (*sig_bits).red = 8 as libc::c_int as png_byte;
            }
            if (*sig_bits).green as libc::c_int > 8 as libc::c_int {
                (*sig_bits).green = 8 as libc::c_int as png_byte;
            }
            if (*sig_bits).blue as libc::c_int > 8 as libc::c_int {
                (*sig_bits).blue = 8 as libc::c_int as png_byte;
            }
            if (*sig_bits).gray as libc::c_int > 8 as libc::c_int {
                (*sig_bits).gray = 8 as libc::c_int as png_byte;
            }
            if (*sig_bits).alpha as libc::c_int > 8 as libc::c_int {
                (*sig_bits).alpha = 8 as libc::c_int as png_byte;
            }
        }
        if reductions & 0x4 as libc::c_int as png_uint_32 != 0 {
            let mut max_sig_bits: png_byte = (*sig_bits).red;
            if (max_sig_bits as libc::c_int) < (*sig_bits).green as libc::c_int {
                max_sig_bits = (*sig_bits).green;
            }
            if (max_sig_bits as libc::c_int) < (*sig_bits).blue as libc::c_int {
                max_sig_bits = (*sig_bits).blue;
            }
            (*sig_bits).gray = max_sig_bits;
        }
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        dest_bit_depth,
        dest_color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
    return reductions;
}
unsafe extern "C" fn opng_reduce_palette_bits(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut src_sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut dest_sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut color_type: libc::c_int = 0;
    let mut interlace_type: libc::c_int = 0;
    let mut compression_type: libc::c_int = 0;
    let mut filter_type: libc::c_int = 0;
    let mut src_bit_depth: libc::c_int = 0;
    let mut dest_bit_depth: libc::c_int = 0;
    let mut src_mask_init: libc::c_uint = 0;
    let mut src_mask: libc::c_uint = 0;
    let mut src_shift: libc::c_uint = 0;
    let mut dest_shift: libc::c_uint = 0;
    let mut sample: libc::c_uint = 0;
    let mut dest_buf: libc::c_uint = 0;
    let mut palette: png_colorp = 0 as *mut png_color;
    let mut num_palette: libc::c_int = 0;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    if reductions & 0x2 as libc::c_int as png_uint_32 == 0 {
        return 0 as libc::c_int as png_uint_32;
    }
    png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut src_bit_depth,
        &mut color_type,
        &mut interlace_type,
        &mut compression_type,
        &mut filter_type,
    );
    if color_type != 2 as libc::c_int | 1 as libc::c_int {
        return 0 as libc::c_int as png_uint_32;
    }
    if png_get_PLTE(
        png_ptr as *const png_struct,
        info_ptr,
        &mut palette,
        &mut num_palette,
    ) == 0
    {
        num_palette = 0 as libc::c_int;
    }
    result = 0 as libc::c_int as png_uint_32;
    if num_palette > (1 as libc::c_int) << src_bit_depth {
        png_warning(
            png_ptr as *const png_struct,
            b"Too many colors in PLTE\0" as *const u8 as *const libc::c_char,
        );
        num_palette = (1 as libc::c_int) << src_bit_depth;
        opng_realloc_PLTE(png_ptr, info_ptr, num_palette);
        png_get_PLTE(
            png_ptr as *const png_struct,
            info_ptr,
            &mut palette,
            &mut num_palette,
        );
        if num_palette == (1 as libc::c_int) << src_bit_depth {} else {
            __assert_fail(
                b"num_palette == (1 << src_bit_depth)\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                725 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3322: {
            if num_palette == (1 as libc::c_int) << src_bit_depth {} else {
                __assert_fail(
                    b"num_palette == (1 << src_bit_depth)\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    725 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        result |= 0x2000 as libc::c_int as png_uint_32;
    }
    if num_palette > 16 as libc::c_int {
        return result
    } else if num_palette > 4 as libc::c_int {
        dest_bit_depth = 4 as libc::c_int;
    } else if num_palette > 2 as libc::c_int {
        dest_bit_depth = 2 as libc::c_int;
    } else {
        if num_palette > 0 as libc::c_int {} else {
            __assert_fail(
                b"num_palette > 0\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                738 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3254: {
            if num_palette > 0 as libc::c_int {} else {
                __assert_fail(
                    b"num_palette > 0\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    738 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        dest_bit_depth = 1 as libc::c_int;
    }
    if src_bit_depth <= dest_bit_depth {
        if src_bit_depth == dest_bit_depth {} else {
            __assert_fail(
                b"src_bit_depth == dest_bit_depth\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                744 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3187: {
            if src_bit_depth == dest_bit_depth {} else {
                __assert_fail(
                    b"src_bit_depth == dest_bit_depth\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    744 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[libc::c_char; 74],
                    >(
                        b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return result;
    }
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    if src_bit_depth == 8 as libc::c_int {
        i = 0 as libc::c_int as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as libc::c_int as libc::c_uint;
            dest_buf = 0 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int as png_uint_32;
            while j < width {
                dest_shift = dest_shift.wrapping_sub(dest_bit_depth as libc::c_uint);
                if dest_shift > 0 as libc::c_int as libc::c_uint {
                    dest_buf
                        |= ((*src_sample_ptr as libc::c_int) << dest_shift)
                            as libc::c_uint;
                } else {
                    let fresh0 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh0 = (dest_buf | *src_sample_ptr as libc::c_uint) as png_byte;
                    dest_shift = 8 as libc::c_int as libc::c_uint;
                    dest_buf = 0 as libc::c_int as libc::c_uint;
                }
                src_sample_ptr = src_sample_ptr.offset(1);
                src_sample_ptr;
                j = j.wrapping_add(1);
                j;
            }
            if dest_shift != 0 as libc::c_int as libc::c_uint {
                *dest_sample_ptr = dest_buf as png_byte;
            }
            i = i.wrapping_add(1);
            i;
            row_ptr = row_ptr.offset(1);
            row_ptr;
        }
    } else {
        src_mask_init = (((1 as libc::c_int) << 8 as libc::c_int + src_bit_depth)
            - ((1 as libc::c_int) << 8 as libc::c_int)) as libc::c_uint;
        i = 0 as libc::c_int as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as libc::c_int as libc::c_uint;
            src_shift = dest_shift;
            src_mask = src_mask_init;
            dest_buf = 0 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int as png_uint_32;
            while j < width {
                src_shift = src_shift.wrapping_sub(src_bit_depth as libc::c_uint);
                src_mask >>= src_bit_depth;
                sample = (*src_sample_ptr as libc::c_uint & src_mask) >> src_shift;
                dest_shift = dest_shift.wrapping_sub(dest_bit_depth as libc::c_uint);
                if dest_shift > 0 as libc::c_int as libc::c_uint {
                    dest_buf |= sample << dest_shift;
                } else {
                    let fresh1 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh1 = (dest_buf | sample) as png_byte;
                    dest_shift = 8 as libc::c_int as libc::c_uint;
                    dest_buf = 0 as libc::c_int as libc::c_uint;
                }
                if src_shift == 0 as libc::c_int as libc::c_uint {
                    src_shift = 8 as libc::c_int as libc::c_uint;
                    src_mask = src_mask_init;
                    src_sample_ptr = src_sample_ptr.offset(1);
                    src_sample_ptr;
                }
                j = j.wrapping_add(1);
                j;
            }
            if dest_shift != 0 as libc::c_int as libc::c_uint {
                *dest_sample_ptr = dest_buf as png_byte;
            }
            i = i.wrapping_add(1);
            i;
            row_ptr = row_ptr.offset(1);
            row_ptr;
        }
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        dest_bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
    result |= 0x2 as libc::c_int as png_uint_32;
    return result;
}
unsafe extern "C" fn opng_reduce_to_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut row_info: png_row_info = png_row_info_struct {
        width: 0,
        rowbytes: 0,
        color_type: 0,
        bit_depth: 0,
        channels: 0,
        pixel_depth: 0,
    };
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut alpha_row: png_bytep = 0 as *mut png_byte;
    let mut height: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut color_type: libc::c_int = 0;
    let mut interlace_type: libc::c_int = 0;
    let mut compression_type: libc::c_int = 0;
    let mut filter_type: libc::c_int = 0;
    let mut src_bit_depth: libc::c_int = 0;
    let mut dest_bit_depth: libc::c_int = 0;
    let mut channels: libc::c_int = 0;
    let mut palette: [png_color; 256] = [png_color_struct {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut trans_alpha: [png_byte; 256] = [0; 256];
    let mut trans_color: png_color_16p = 0 as *mut png_color_16;
    let mut num_palette: libc::c_int = 0;
    let mut num_trans: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut gray: libc::c_uint = 0;
    let mut red: libc::c_uint = 0;
    let mut green: libc::c_uint = 0;
    let mut blue: libc::c_uint = 0;
    let mut alpha: libc::c_uint = 0;
    let mut prev_gray: libc::c_uint = 0;
    let mut prev_red: libc::c_uint = 0;
    let mut prev_green: libc::c_uint = 0;
    let mut prev_blue: libc::c_uint = 0;
    let mut prev_alpha: libc::c_uint = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut src_bit_depth,
        &mut color_type,
        &mut interlace_type,
        &mut compression_type,
        &mut filter_type,
    );
    if src_bit_depth != 8 as libc::c_int {
        return 0 as libc::c_int as png_uint_32;
    }
    if color_type & 1 as libc::c_int == 0 {} else {
        __assert_fail(
            b"!(color_type & 1)\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            850 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6417: {
        if color_type & 1 as libc::c_int == 0 {} else {
            __assert_fail(
                b"!(color_type & 1)\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                850 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    channels = png_get_channels(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    ) as libc::c_int;
    alpha_row = png_malloc(png_ptr as *const png_struct, width as png_alloc_size_t)
        as png_bytep;
    row_info.width = width;
    row_info.rowbytes = 0 as libc::c_int as size_t;
    row_info.color_type = color_type as png_byte;
    row_info.bit_depth = src_bit_depth as png_byte;
    row_info.channels = channels as png_byte;
    row_info.pixel_depth = 0 as libc::c_int as png_byte;
    num_trans = 0 as libc::c_int;
    num_palette = num_trans;
    trans_color = 0 as png_color_16p;
    png_get_tRNS(
        png_ptr as *const png_struct,
        info_ptr,
        0 as *mut png_bytep,
        0 as *mut libc::c_int,
        &mut trans_color,
    );
    prev_alpha = -(1 as libc::c_int) as libc::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    prev_gray = prev_red;
    i = 0 as libc::c_int as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr;
        opng_get_alpha_row(&mut row_info, trans_color, *row_ptr, alpha_row);
        if color_type & 2 as libc::c_int != 0 {
            j = 0 as libc::c_int as png_uint_32;
            while j < width {
                red = *sample_ptr.offset(0 as libc::c_int as isize) as libc::c_uint;
                green = *sample_ptr.offset(1 as libc::c_int as isize) as libc::c_uint;
                blue = *sample_ptr.offset(2 as libc::c_int as isize) as libc::c_uint;
                alpha = *alpha_row.offset(j as isize) as libc::c_uint;
                if red != prev_red || green != prev_green || blue != prev_blue
                    || alpha != prev_alpha
                {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        palette.as_mut_ptr(),
                        &mut num_palette,
                        trans_alpha.as_mut_ptr(),
                        &mut num_trans,
                        256 as libc::c_int,
                        red,
                        green,
                        blue,
                        alpha,
                        &mut index,
                    ) < 0 as libc::c_int
                    {
                        if num_palette < 0 as libc::c_int {} else {
                            __assert_fail(
                                b"num_palette < 0\0" as *const u8 as *const libc::c_char,
                                b"opngreduc/opngreduc.c\0" as *const u8
                                    as *const libc::c_char,
                                893 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[libc::c_char; 72],
                                >(
                                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_6126: {
                            if num_palette < 0 as libc::c_int {} else {
                                __assert_fail(
                                    b"num_palette < 0\0" as *const u8 as *const libc::c_char,
                                    b"opngreduc/opngreduc.c\0" as *const u8
                                        as *const libc::c_char,
                                    893 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 72],
                                        &[libc::c_char; 72],
                                    >(
                                        b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        i = height;
                        break;
                    }
                }
                j = j.wrapping_add(1);
                j;
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        } else {
            j = 0 as libc::c_int as png_uint_32;
            while j < width {
                gray = *sample_ptr.offset(0 as libc::c_int as isize) as libc::c_uint;
                alpha = *alpha_row.offset(j as isize) as libc::c_uint;
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        palette.as_mut_ptr(),
                        &mut num_palette,
                        trans_alpha.as_mut_ptr(),
                        &mut num_trans,
                        256 as libc::c_int,
                        gray,
                        gray,
                        gray,
                        alpha,
                        &mut index,
                    ) < 0 as libc::c_int
                    {
                        if num_palette < 0 as libc::c_int {} else {
                            __assert_fail(
                                b"num_palette < 0\0" as *const u8 as *const libc::c_char,
                                b"opngreduc/opngreduc.c\0" as *const u8
                                    as *const libc::c_char,
                                915 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[libc::c_char; 72],
                                >(
                                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_5992: {
                            if num_palette < 0 as libc::c_int {} else {
                                __assert_fail(
                                    b"num_palette < 0\0" as *const u8 as *const libc::c_char,
                                    b"opngreduc/opngreduc.c\0" as *const u8
                                        as *const libc::c_char,
                                    915 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 72],
                                        &[libc::c_char; 72],
                                    >(
                                        b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        i = height;
                        break;
                    }
                }
                j = j.wrapping_add(1);
                j;
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
        row_ptr = row_ptr.offset(1);
        row_ptr;
    }
    if num_palette >= 0 as libc::c_int
        && png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background) != 0
    {
        if color_type & 2 as libc::c_int != 0 {
            red = (*background).red as libc::c_uint;
            green = (*background).green as libc::c_uint;
            blue = (*background).blue as libc::c_uint;
        } else {
            blue = (*background).gray as libc::c_uint;
            green = blue;
            red = green;
        }
        if red > 255 as libc::c_int as libc::c_uint
            || green > 255 as libc::c_int as libc::c_uint
            || blue > 255 as libc::c_int as libc::c_uint
        {
            png_warning(
                png_ptr as *const png_struct,
                b"Invalid colors in bKGD\0" as *const u8 as *const libc::c_char,
            );
            png_set_invalid(
                png_ptr as *const png_struct,
                info_ptr,
                0x20 as libc::c_uint as libc::c_int,
            );
        } else {
            opng_insert_palette_entry(
                palette.as_mut_ptr(),
                &mut num_palette,
                trans_alpha.as_mut_ptr(),
                &mut num_trans,
                256 as libc::c_int,
                red,
                green,
                blue,
                -(1 as libc::c_int) as libc::c_uint,
                &mut index,
            );
            if index >= 0 as libc::c_int {
                (*background).index = index as png_byte;
            }
        }
    }
    if num_palette >= 0 as libc::c_int {
        if num_palette > 0 as libc::c_int && num_palette <= 256 as libc::c_int {} else {
            __assert_fail(
                b"num_palette > 0 && num_palette <= 256\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                962 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5771: {
            if num_palette > 0 as libc::c_int && num_palette <= 256 as libc::c_int
            {} else {
                __assert_fail(
                    b"num_palette > 0 && num_palette <= 256\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    962 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if num_trans >= 0 as libc::c_int && num_trans <= num_palette {} else {
            __assert_fail(
                b"num_trans >= 0 && num_trans <= num_palette\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                963 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5722: {
            if num_trans >= 0 as libc::c_int && num_trans <= num_palette {} else {
                __assert_fail(
                    b"num_trans >= 0 && num_trans <= num_palette\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    963 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[libc::c_char; 72],
                    >(
                        b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if num_palette <= 2 as libc::c_int {
            dest_bit_depth = 1 as libc::c_int;
        } else if num_palette <= 4 as libc::c_int {
            dest_bit_depth = 2 as libc::c_int;
        } else if num_palette <= 16 as libc::c_int {
            dest_bit_depth = 4 as libc::c_int;
        } else {
            dest_bit_depth = 8 as libc::c_int;
        }
        if channels * 8 as libc::c_int == dest_bit_depth
            || ((3 as libc::c_int * num_palette + num_trans) * 8 as libc::c_int
                / (channels * 8 as libc::c_int - dest_bit_depth)) as png_uint_32 / width
                / height >= 1 as libc::c_int as png_uint_32
        {
            num_palette = -(1 as libc::c_int);
        }
    }
    if num_palette < 0 as libc::c_int {
        png_free(png_ptr as *const png_struct, alpha_row as png_voidp);
        return 0 as libc::c_int as png_uint_32;
    }
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    index = -(1 as libc::c_int);
    prev_alpha = -(1 as libc::c_int) as libc::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    i = 0 as libc::c_int as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr;
        opng_get_alpha_row(&mut row_info, trans_color, *row_ptr, alpha_row);
        if color_type & 2 as libc::c_int != 0 {
            j = 0 as libc::c_int as png_uint_32;
            while j < width {
                red = *sample_ptr.offset(0 as libc::c_int as isize) as libc::c_uint;
                green = *sample_ptr.offset(1 as libc::c_int as isize) as libc::c_uint;
                blue = *sample_ptr.offset(2 as libc::c_int as isize) as libc::c_uint;
                alpha = *alpha_row.offset(j as isize) as libc::c_uint;
                if red != prev_red || green != prev_green || blue != prev_blue
                    || alpha != prev_alpha
                {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        palette.as_mut_ptr(),
                        &mut num_palette,
                        trans_alpha.as_mut_ptr(),
                        &mut num_trans,
                        256 as libc::c_int,
                        red,
                        green,
                        blue,
                        alpha,
                        &mut index,
                    ) != 0 as libc::c_int
                    {
                        index = -(1 as libc::c_int);
                    }
                }
                if index >= 0 as libc::c_int {} else {
                    __assert_fail(
                        b"index >= 0\0" as *const u8 as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        1014 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[libc::c_char; 72],
                        >(
                            b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_4896: {
                    if index >= 0 as libc::c_int {} else {
                        __assert_fail(
                            b"index >= 0\0" as *const u8 as *const libc::c_char,
                            b"opngreduc/opngreduc.c\0" as *const u8
                                as *const libc::c_char,
                            1014 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                j;
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        } else {
            j = 0 as libc::c_int as png_uint_32;
            while j < width {
                gray = *sample_ptr.offset(0 as libc::c_int as isize) as libc::c_uint;
                alpha = *alpha_row.offset(j as isize) as libc::c_uint;
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(
                        palette.as_mut_ptr(),
                        &mut num_palette,
                        trans_alpha.as_mut_ptr(),
                        &mut num_trans,
                        256 as libc::c_int,
                        gray,
                        gray,
                        gray,
                        alpha,
                        &mut index,
                    ) != 0 as libc::c_int
                    {
                        index = -(1 as libc::c_int);
                    }
                }
                if index >= 0 as libc::c_int {} else {
                    __assert_fail(
                        b"index >= 0\0" as *const u8 as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        1034 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[libc::c_char; 72],
                        >(
                            b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3892: {
                    if index >= 0 as libc::c_int {} else {
                        __assert_fail(
                            b"index >= 0\0" as *const u8 as *const libc::c_char,
                            b"opngreduc/opngreduc.c\0" as *const u8
                                as *const libc::c_char,
                            1034 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[libc::c_char; 72],
                            >(
                                b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                j;
                sample_ptr = sample_ptr.offset(channels as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
        row_ptr = row_ptr.offset(1);
        row_ptr;
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        8 as libc::c_int,
        2 as libc::c_int | 1 as libc::c_int,
        interlace_type,
        compression_type,
        filter_type,
    );
    png_set_PLTE(
        png_ptr,
        info_ptr,
        palette.as_mut_ptr() as png_const_colorp,
        num_palette,
    );
    if num_trans > 0 as libc::c_int {
        png_set_tRNS(
            png_ptr,
            info_ptr,
            trans_alpha.as_mut_ptr() as png_const_bytep,
            num_trans,
            0 as png_const_color_16p,
        );
    }
    png_free(png_ptr as *const png_struct, alpha_row as png_voidp);
    result = 0x10 as libc::c_int as png_uint_32;
    if reductions & 0x2 as libc::c_int as png_uint_32 != 0 {
        result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions);
    }
    return result;
}
unsafe extern "C" fn opng_analyze_sample_usage(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut usage_map: png_bytep,
) {
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut init_shift: libc::c_int = 0;
    let mut init_mask: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    height = png_get_image_height(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    );
    width = png_get_image_width(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    );
    bit_depth = png_get_bit_depth(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    ) as libc::c_int;
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    memset(
        usage_map as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    if bit_depth == 8 as libc::c_int {
        i = 0 as libc::c_int as png_uint_32;
        while i < height {
            j = 0 as libc::c_int as png_uint_32;
            sample_ptr = *row_ptr;
            while j < width {
                *usage_map.offset(*sample_ptr as isize) = 1 as libc::c_int as png_byte;
                j = j.wrapping_add(1);
                j;
                sample_ptr = sample_ptr.offset(1);
                sample_ptr;
            }
            i = i.wrapping_add(1);
            i;
            row_ptr = row_ptr.offset(1);
            row_ptr;
        }
    } else {
        if bit_depth < 8 as libc::c_int {} else {
            __assert_fail(
                b"bit_depth < 8\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                1096 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"void opng_analyze_sample_usage(png_structp, png_infop, png_bytep)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7859: {
            if bit_depth < 8 as libc::c_int {} else {
                __assert_fail(
                    b"bit_depth < 8\0" as *const u8 as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    1096 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"void opng_analyze_sample_usage(png_structp, png_infop, png_bytep)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        init_shift = 8 as libc::c_int - bit_depth;
        init_mask = ((1 as libc::c_int) << 8 as libc::c_int)
            - ((1 as libc::c_int) << init_shift);
        i = 0 as libc::c_int as png_uint_32;
        while i < height {
            j = 0 as libc::c_int as png_uint_32;
            sample_ptr = *row_ptr;
            while j < width {
                mask = init_mask;
                shift = init_shift;
                loop {
                    *usage_map
                        .offset(
                            ((*sample_ptr as libc::c_int & mask) >> shift) as isize,
                        ) = 1 as libc::c_int as png_byte;
                    mask >>= bit_depth;
                    shift -= bit_depth;
                    j = j.wrapping_add(1);
                    j;
                    if !(mask > 0 as libc::c_int && j < width) {
                        break;
                    }
                }
                sample_ptr = sample_ptr.offset(1);
                sample_ptr;
            }
            i = i.wrapping_add(1);
            i;
            row_ptr = row_ptr.offset(1);
            row_ptr;
        }
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background) != 0 {
        *usage_map.offset((*background).index as isize) = 1 as libc::c_int as png_byte;
    }
}
unsafe extern "C" fn opng_reduce_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut palette: png_colorp = 0 as *mut png_color;
    let mut trans_alpha: png_bytep = 0 as *mut png_byte;
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut interlace_type: libc::c_int = 0;
    let mut compression_type: libc::c_int = 0;
    let mut filter_type: libc::c_int = 0;
    let mut num_palette: libc::c_int = 0;
    let mut num_trans: libc::c_int = 0;
    let mut last_color_index: libc::c_int = 0;
    let mut last_trans_index: libc::c_int = 0;
    let mut crt_trans_value: png_byte = 0;
    let mut last_trans_value: png_byte = 0;
    let mut is_used: [png_byte; 256] = [0; 256];
    let mut gray_trans: png_color_16 = png_color_16_struct {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    };
    let mut is_gray: libc::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut hist: png_uint_16p = 0 as *mut png_uint_16;
    let mut sig_bits: png_color_8p = 0 as *mut png_color_8;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    let mut k: libc::c_int = 0;
    result = 0 as libc::c_int as png_uint_32;
    png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut bit_depth,
        &mut color_type,
        &mut interlace_type,
        &mut compression_type,
        &mut filter_type,
    );
    row_ptr = png_get_rows(png_ptr as *const png_struct, info_ptr as *const png_info);
    if png_get_PLTE(
        png_ptr as *const png_struct,
        info_ptr,
        &mut palette,
        &mut num_palette,
    ) == 0
    {
        palette = 0 as png_colorp;
        num_palette = 0 as libc::c_int;
    }
    if png_get_tRNS(
        png_ptr as *const png_struct,
        info_ptr,
        &mut trans_alpha,
        &mut num_trans,
        0 as *mut png_color_16p,
    ) == 0
    {
        trans_alpha = 0 as png_bytep;
        num_trans = 0 as libc::c_int;
    } else {
        if !trans_alpha.is_null() && num_trans > 0 as libc::c_int {} else {
            __assert_fail(
                b"trans_alpha != ((void*)0) && num_trans > 0\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                1174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8011: {
            if !trans_alpha.is_null() && num_trans > 0 as libc::c_int {} else {
                __assert_fail(
                    b"trans_alpha != ((void*)0) && num_trans > 0\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    1174 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    opng_analyze_sample_usage(png_ptr, info_ptr, is_used.as_mut_ptr());
    is_gray = (reductions & 0x80 as libc::c_int as png_uint_32 != 0
        && bit_depth == 8 as libc::c_int) as libc::c_int;
    last_trans_index = -(1 as libc::c_int);
    last_color_index = last_trans_index;
    k = 0 as libc::c_int;
    while k < 256 as libc::c_int {
        if !(is_used[k as usize] == 0) {
            last_color_index = k;
            if k < num_trans
                && (*trans_alpha.offset(k as isize) as libc::c_int) < 255 as libc::c_int
            {
                last_trans_index = k;
            }
            if is_gray != 0 {
                if (*palette.offset(k as isize)).red as libc::c_int
                    != (*palette.offset(k as isize)).green as libc::c_int
                    || (*palette.offset(k as isize)).red as libc::c_int
                        != (*palette.offset(k as isize)).blue as libc::c_int
                {
                    is_gray = 0 as libc::c_int;
                }
            }
        }
        k += 1;
        k;
    }
    if last_color_index >= 0 as libc::c_int {} else {
        __assert_fail(
            b"last_color_index >= 0\0" as *const u8 as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            1192 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"))
                .as_ptr(),
        );
    }
    'c_7535: {
        if last_color_index >= 0 as libc::c_int {} else {
            __assert_fail(
                b"last_color_index >= 0\0" as *const u8 as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                1192 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if last_color_index >= last_trans_index {} else {
        __assert_fail(
            b"last_color_index >= last_trans_index\0" as *const u8
                as *const libc::c_char,
            b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
            1193 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0"))
                .as_ptr(),
        );
    }
    'c_7496: {
        if last_color_index >= last_trans_index {} else {
            __assert_fail(
                b"last_color_index >= last_trans_index\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                1193 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if last_color_index >= num_palette {
        png_warning(
            png_ptr as *const png_struct,
            b"Too few colors in PLTE\0" as *const u8 as *const libc::c_char,
        );
        opng_realloc_PLTE(png_ptr, info_ptr, last_color_index + 1 as libc::c_int);
        png_get_PLTE(
            png_ptr as *const png_struct,
            info_ptr,
            &mut palette,
            &mut num_palette,
        );
        if num_palette == last_color_index + 1 as libc::c_int {} else {
            __assert_fail(
                b"num_palette == last_color_index + 1\0" as *const u8
                    as *const libc::c_char,
                b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                1202 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7419: {
            if num_palette == last_color_index + 1 as libc::c_int {} else {
                __assert_fail(
                    b"num_palette == last_color_index + 1\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    1202 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        result |= 0x2000 as libc::c_int as png_uint_32;
    }
    if num_trans > num_palette {
        png_warning(
            png_ptr as *const png_struct,
            b"Too many alpha values in tRNS\0" as *const u8 as *const libc::c_char,
        );
        result |= 0x2000 as libc::c_int as png_uint_32;
    }
    if is_gray != 0 && last_trans_index >= 0 as libc::c_int {
        gray_trans
            .gray = (*palette.offset(last_trans_index as isize)).red as png_uint_16;
        last_trans_value = *trans_alpha.offset(last_trans_index as isize);
        k = 0 as libc::c_int;
        while k <= last_color_index {
            if !(is_used[k as usize] == 0) {
                if k <= last_trans_index {
                    crt_trans_value = *trans_alpha.offset(k as isize);
                    if (crt_trans_value as libc::c_int) < 255 as libc::c_int
                        && (*palette.offset(k as isize)).red as libc::c_int
                            != gray_trans.gray as libc::c_int
                    {
                        is_gray = 0 as libc::c_int;
                        break;
                    }
                } else {
                    crt_trans_value = 255 as libc::c_int as png_byte;
                }
                if (*palette.offset(k as isize)).red as libc::c_int
                    == gray_trans.gray as libc::c_int
                    && crt_trans_value as libc::c_int != last_trans_value as libc::c_int
                {
                    is_gray = 0 as libc::c_int;
                    break;
                }
            }
            k += 1;
            k;
        }
    }
    if num_trans > 0 as libc::c_int && last_trans_index < 0 as libc::c_int {
        num_trans = 0 as libc::c_int;
        png_free_data(
            png_ptr as *const png_struct,
            info_ptr,
            0x2000 as libc::c_uint,
            -(1 as libc::c_int),
        );
        png_set_invalid(
            png_ptr as *const png_struct,
            info_ptr,
            0x10 as libc::c_uint as libc::c_int,
        );
        result |= 0x200 as libc::c_int as png_uint_32;
    }
    if reductions & 0x200 as libc::c_int as png_uint_32 != 0 {
        if num_palette != last_color_index + 1 as libc::c_int {
            opng_realloc_PLTE(png_ptr, info_ptr, last_color_index + 1 as libc::c_int);
            png_get_PLTE(
                png_ptr as *const png_struct,
                info_ptr,
                &mut palette,
                &mut num_palette,
            );
            if num_palette == last_color_index + 1 as libc::c_int {} else {
                __assert_fail(
                    b"num_palette == last_color_index + 1\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    1260 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7137: {
                if num_palette == last_color_index + 1 as libc::c_int {} else {
                    __assert_fail(
                        b"num_palette == last_color_index + 1\0" as *const u8
                            as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        1260 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 69],
                            &[libc::c_char; 69],
                        >(
                            b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            result |= 0x200 as libc::c_int as png_uint_32;
        }
        if num_trans > 0 as libc::c_int
            && num_trans != last_trans_index + 1 as libc::c_int
        {
            opng_realloc_tRNS(png_ptr, info_ptr, last_trans_index + 1 as libc::c_int);
            png_get_tRNS(
                png_ptr as *const png_struct,
                info_ptr,
                &mut trans_alpha,
                &mut num_trans,
                0 as *mut png_color_16p,
            );
            if num_trans == last_trans_index + 1 as libc::c_int {} else {
                __assert_fail(
                    b"num_trans == last_trans_index + 1\0" as *const u8
                        as *const libc::c_char,
                    b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                    1269 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 69],
                        &[libc::c_char; 69],
                    >(
                        b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_6910: {
                if num_trans == last_trans_index + 1 as libc::c_int {} else {
                    __assert_fail(
                        b"num_trans == last_trans_index + 1\0" as *const u8
                            as *const libc::c_char,
                        b"opngreduc/opngreduc.c\0" as *const u8 as *const libc::c_char,
                        1269 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 69],
                            &[libc::c_char; 69],
                        >(
                            b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            result |= 0x200 as libc::c_int as png_uint_32;
        }
    }
    if reductions & 0x2 as libc::c_int as png_uint_32 != 0 {
        result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions);
        bit_depth = png_get_bit_depth(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
        ) as libc::c_int;
    }
    if bit_depth < 8 as libc::c_int || is_gray == 0 {
        return result;
    }
    i = 0 as libc::c_int as png_uint_32;
    while i < height {
        j = 0 as libc::c_int as png_uint_32;
        while j < width {
            *(*row_ptr.offset(i as isize))
                .offset(
                    j as isize,
                ) = (*palette
                .offset(*(*row_ptr.offset(i as isize)).offset(j as isize) as isize))
                .red;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if num_trans > 0 as libc::c_int {
        png_set_tRNS(
            png_ptr,
            info_ptr,
            0 as png_const_bytep,
            0 as libc::c_int,
            &mut gray_trans as *mut png_color_16 as png_const_color_16p,
        );
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background) != 0 {
        (*background)
            .gray = (*palette.offset((*background).index as isize)).red as png_uint_16;
    }
    if png_get_hIST(png_ptr as *const png_struct, info_ptr, &mut hist) != 0 {
        png_free_data(
            png_ptr as *const png_struct,
            info_ptr,
            0x8 as libc::c_uint,
            -(1 as libc::c_int),
        );
        png_set_invalid(
            png_ptr as *const png_struct,
            info_ptr,
            0x40 as libc::c_uint as libc::c_int,
        );
    }
    if png_get_sBIT(png_ptr as *const png_struct, info_ptr, &mut sig_bits) != 0 {
        let mut max_sig_bits: png_byte = (*sig_bits).red;
        if (max_sig_bits as libc::c_int) < (*sig_bits).green as libc::c_int {
            max_sig_bits = (*sig_bits).green;
        }
        if (max_sig_bits as libc::c_int) < (*sig_bits).blue as libc::c_int {
            max_sig_bits = (*sig_bits).blue;
        }
        (*sig_bits).gray = max_sig_bits;
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        bit_depth,
        0 as libc::c_int,
        interlace_type,
        compression_type,
        filter_type,
    );
    png_free_data(
        png_ptr as *const png_struct,
        info_ptr,
        0x1000 as libc::c_uint,
        -(1 as libc::c_int),
    );
    png_set_invalid(
        png_ptr as *const png_struct,
        info_ptr,
        0x8 as libc::c_uint as libc::c_int,
    );
    return 0x80 as libc::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn opng_reduce_image(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut reductions: png_uint_32,
) -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut color_type: libc::c_int = 0;
    if opng_validate_image(png_ptr, info_ptr) == 0 {
        png_warning(
            png_ptr as *const png_struct,
            b"Image reduction requires the presence of all critical information\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as png_uint_32;
    }
    color_type = png_get_color_type(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    ) as libc::c_int;
    result = opng_reduce_bits(png_ptr, info_ptr, reductions);
    if color_type == 2 as libc::c_int | 1 as libc::c_int
        && reductions
            & (0x80 as libc::c_int | 0x200 as libc::c_int | 0x2 as libc::c_int)
                as png_uint_32 != 0
    {
        result |= opng_reduce_palette(png_ptr, info_ptr, reductions);
    }
    if color_type & !(4 as libc::c_int) == 0 as libc::c_int
        && reductions & 0x40 as libc::c_int as png_uint_32 != 0
        || color_type & !(4 as libc::c_int) == 2 as libc::c_int
            && reductions & 0x10 as libc::c_int as png_uint_32 != 0
    {
        if result & 0x80 as libc::c_int as png_uint_32 == 0 {
            result |= opng_reduce_to_palette(png_ptr, info_ptr, reductions);
        }
    }
    return result;
}
