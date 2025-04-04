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
    fn png_set_PLTE(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        palette: png_const_colorp,
        num_palette: libc::c_int,
    );
    fn png_set_tRNS(
        png_ptr: png_structrp,
        info_ptr: png_inforp,
        trans_alpha: png_const_bytep,
        num_trans: libc::c_int,
        trans_color: png_const_color_16p,
    );
    fn pngx_set_interlace_type(
        png_ptr: png_structp,
        info_ptr: png_infop,
        interlace_type: libc::c_int,
    );
    fn pngx_malloc_rows(
        png_ptr: png_structp,
        info_ptr: png_infop,
        filler: libc::c_int,
    ) -> png_bytepp;
    fn GIFReadScreen(screen: *mut GIFScreen, stream: *mut FILE);
    fn GIFInitImage(
        image: *mut GIFImage,
        screen: *mut GIFScreen,
        rows: *mut *mut libc::c_uchar,
    );
    fn GIFDestroyImage(image: *mut GIFImage);
    fn GIFReadNextBlock(
        image: *mut GIFImage,
        ext: *mut GIFExtension,
        stream: *mut FILE,
    ) -> libc::c_int;
    fn GIFGetColorTable(
        colors: *mut *mut libc::c_uchar,
        numColors: *mut libc::c_uint,
        image: *mut GIFImage,
    );
    fn GIFInitExtension(
        ext: *mut GIFExtension,
        screen: *mut GIFScreen,
        initBufferSize: libc::c_uint,
    );
    fn GIFDestroyExtension(ext: *mut GIFExtension);
    fn GIFGetGraphicCtl(graphicExt: *mut GIFGraphicCtlExt, ext: *mut GIFExtension);
    static mut GIFError: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>;
    static mut GIFWarning: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>;
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
pub type png_uint_16 = libc::c_ushort;
pub type png_uint_32 = libc::c_uint;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
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
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_color_16 = png_color_16_struct;
pub type png_const_color_16p = *const png_color_16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFScreen {
    pub Width: libc::c_uint,
    pub Height: libc::c_uint,
    pub GlobalColorFlag: libc::c_uint,
    pub ColorResolution: libc::c_uint,
    pub SortFlag: libc::c_uint,
    pub GlobalNumColors: libc::c_uint,
    pub Background: libc::c_uint,
    pub PixelAspectRatio: libc::c_uint,
    pub GlobalColorTable: [libc::c_uchar; 768],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFImage {
    pub Screen: *mut GIFScreen,
    pub LeftPos: libc::c_uint,
    pub TopPos: libc::c_uint,
    pub Width: libc::c_uint,
    pub Height: libc::c_uint,
    pub LocalColorFlag: libc::c_uint,
    pub InterlaceFlag: libc::c_uint,
    pub SortFlag: libc::c_uint,
    pub LocalNumColors: libc::c_uint,
    pub LocalColorTable: [libc::c_uchar; 768],
    pub Rows: *mut *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFExtension {
    pub Screen: *mut GIFScreen,
    pub Buffer: *mut libc::c_uchar,
    pub BufferSize: libc::c_uint,
    pub Label: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFGraphicCtlExt {
    pub DisposalMethod: libc::c_uint,
    pub InputFlag: libc::c_uint,
    pub TransparentFlag: libc::c_uint,
    pub DelayTime: libc::c_uint,
    pub Transparent: libc::c_uint,
}
static mut gif_fmt_name: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"GIF\0")
};
static mut gif_fmt_long_name: [libc::c_char; 28] = unsafe {
    *::core::mem::transmute::<
        &[u8; 28],
        &[libc::c_char; 28],
    >(b"Graphics Interchange Format\0")
};
static mut gif_sig_gif87a: [png_byte; 6] = [
    0x47 as libc::c_int as png_byte,
    0x49 as libc::c_int as png_byte,
    0x46 as libc::c_int as png_byte,
    0x38 as libc::c_int as png_byte,
    0x37 as libc::c_int as png_byte,
    0x61 as libc::c_int as png_byte,
];
static mut gif_sig_gif89a: [png_byte; 6] = [
    0x47 as libc::c_int as png_byte,
    0x49 as libc::c_int as png_byte,
    0x46 as libc::c_int as png_byte,
    0x38 as libc::c_int as png_byte,
    0x39 as libc::c_int as png_byte,
    0x61 as libc::c_int as png_byte,
];
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_gif(
    mut sig: png_bytep,
    mut sig_size: size_t,
    mut fmt_name_ptr: png_const_charpp,
    mut fmt_long_name_ptr: png_const_charpp,
) -> libc::c_int {
    if sig_size < (6 as libc::c_int + 7 as libc::c_int) as size_t {
        return -(1 as libc::c_int);
    }
    if memcmp(
        sig as *const libc::c_void,
        gif_sig_gif87a.as_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && memcmp(
            sig as *const libc::c_void,
            gif_sig_gif89a.as_ptr() as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !fmt_name_ptr.is_null() {
        *fmt_name_ptr = gif_fmt_name.as_ptr();
    }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = gif_fmt_long_name.as_ptr();
    }
    return 1 as libc::c_int;
}
static mut err_png_ptr: png_structp = 0 as *const png_struct as *mut png_struct;
static mut err_gif_image_ptr: *mut GIFImage = 0 as *const GIFImage as *mut GIFImage;
static mut err_gif_ext_ptr: *mut GIFExtension = 0 as *const GIFExtension
    as *mut GIFExtension;
unsafe extern "C" fn pngx_gif_error(mut msg: *const libc::c_char) {
    if !err_gif_image_ptr.is_null() {
        GIFDestroyImage(err_gif_image_ptr);
    }
    if !err_gif_ext_ptr.is_null() {
        GIFDestroyExtension(err_gif_ext_ptr);
    }
    png_error(err_png_ptr as *const png_struct, msg);
}
unsafe extern "C" fn pngx_gif_warning(mut msg: *const libc::c_char) {
    png_warning(err_png_ptr as *const png_struct, msg);
}
unsafe extern "C" fn pngx_set_gif_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut color_table: *mut libc::c_uchar,
    mut num_colors: libc::c_uint,
) {
    let mut palette: [png_color; 256] = [png_color_struct {
        red: 0,
        green: 0,
        blue: 0,
    }; 256];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_colors {
        palette[i as usize]
            .red = *color_table
            .offset((3 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize);
        palette[i as usize]
            .green = *color_table
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            );
        palette[i as usize]
            .blue = *color_table
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    png_set_PLTE(
        png_ptr,
        info_ptr,
        palette.as_mut_ptr() as png_const_colorp,
        num_colors as libc::c_int,
    );
}
unsafe extern "C" fn pngx_set_gif_transparent(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut transparent: libc::c_uint,
) {
    let mut trans: [png_byte; 256] = [0; 256];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < transparent {
        trans[i as usize] = 255 as libc::c_int as png_byte;
        i = i.wrapping_add(1);
        i;
    }
    trans[transparent as usize] = 0 as libc::c_int as png_byte;
    png_set_tRNS(
        png_ptr,
        info_ptr,
        trans.as_mut_ptr() as png_const_bytep,
        transparent as libc::c_int + 1 as libc::c_int,
        0 as png_const_color_16p,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_gif(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut screen: GIFScreen = GIFScreen {
        Width: 0,
        Height: 0,
        GlobalColorFlag: 0,
        ColorResolution: 0,
        SortFlag: 0,
        GlobalNumColors: 0,
        Background: 0,
        PixelAspectRatio: 0,
        GlobalColorTable: [0; 768],
    };
    let mut image: GIFImage = GIFImage {
        Screen: 0 as *mut GIFScreen,
        LeftPos: 0,
        TopPos: 0,
        Width: 0,
        Height: 0,
        LocalColorFlag: 0,
        InterlaceFlag: 0,
        SortFlag: 0,
        LocalNumColors: 0,
        LocalColorTable: [0; 768],
        Rows: 0 as *mut *mut libc::c_uchar,
    };
    let mut ext: GIFExtension = GIFExtension {
        Screen: 0 as *mut GIFScreen,
        Buffer: 0 as *mut libc::c_uchar,
        BufferSize: 0,
        Label: 0,
    };
    let mut graphicExt: GIFGraphicCtlExt = GIFGraphicCtlExt {
        DisposalMethod: 0,
        InputFlag: 0,
        TransparentFlag: 0,
        DelayTime: 0,
        Transparent: 0,
    };
    let mut blockCode: libc::c_int = 0;
    let mut colorTable: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut numColors: libc::c_uint = 0;
    let mut transparent: libc::c_uint = 0;
    let mut numImages: libc::c_uint = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    GIFError = Some(pngx_gif_error as unsafe extern "C" fn(*const libc::c_char) -> ());
    GIFWarning = Some(
        pngx_gif_warning as unsafe extern "C" fn(*const libc::c_char) -> (),
    );
    err_png_ptr = png_ptr;
    err_gif_image_ptr = 0 as *mut GIFImage;
    err_gif_ext_ptr = 0 as *mut GIFExtension;
    GIFReadScreen(&mut screen, stream);
    width = screen.Width;
    height = screen.Height;
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        8 as libc::c_int,
        2 as libc::c_int | 1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, screen.Background as libc::c_int);
    GIFInitImage(&mut image, &mut screen, row_pointers);
    err_gif_image_ptr = &mut image;
    GIFInitExtension(&mut ext, &mut screen, 256 as libc::c_int as libc::c_uint);
    err_gif_ext_ptr = &mut ext;
    numImages = 0 as libc::c_int as libc::c_uint;
    transparent = -(1 as libc::c_int) as libc::c_uint;
    loop {
        blockCode = GIFReadNextBlock(&mut image, &mut ext, stream);
        if blockCode == 0x2c as libc::c_int {
            if !(image.Rows).is_null() {
                if image.InterlaceFlag != 0 {
                    pngx_set_interlace_type(png_ptr, info_ptr, 1 as libc::c_int);
                }
                GIFGetColorTable(&mut colorTable, &mut numColors, &mut image);
                pngx_set_gif_palette(png_ptr, info_ptr, colorTable, numColors);
                if transparent < 256 as libc::c_int as libc::c_uint {
                    pngx_set_gif_transparent(png_ptr, info_ptr, transparent);
                }
                image.Rows = 0 as *mut *mut libc::c_uchar;
            }
            numImages = numImages.wrapping_add(1);
            numImages;
        } else if blockCode == 0x21 as libc::c_int {
            if ext.Label as libc::c_int == 0xf9 as libc::c_int {
                GIFGetGraphicCtl(&mut graphicExt, &mut ext);
                if !(image.Rows).is_null() && graphicExt.TransparentFlag != 0 {
                    if transparent >= 256 as libc::c_int as libc::c_uint {
                        transparent = graphicExt.Transparent;
                    }
                }
            }
        } else if blockCode == 0x3b as libc::c_int {
            break;
        }
    }
    if !(image.Rows).is_null() {
        png_error(
            png_ptr as *const png_struct,
            b"No image in GIF file\0" as *const u8 as *const libc::c_char,
        );
    }
    GIFDestroyImage(&mut image);
    GIFDestroyExtension(&mut ext);
    return numImages as libc::c_int;
}
