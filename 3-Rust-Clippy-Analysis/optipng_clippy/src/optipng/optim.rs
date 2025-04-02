use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type opng_bitset_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_options {
    pub backup: libc::c_int,
    pub clobber: libc::c_int,
    pub debug: libc::c_int,
    pub fix: libc::c_int,
    pub force: libc::c_int,
    pub full: libc::c_int,
    pub preserve: libc::c_int,
    pub quiet: libc::c_int,
    pub simulate: libc::c_int,
    pub verbose: libc::c_int,
    pub out_name: *const libc::c_char,
    pub dir_name: *const libc::c_char,
    pub log_name: *const libc::c_char,
    pub interlace: libc::c_int,
    pub nb: libc::c_int,
    pub nc: libc::c_int,
    pub np: libc::c_int,
    pub nz: libc::c_int,
    pub optim_level: libc::c_int,
    pub compr_level_set: opng_bitset_t,
    pub mem_level_set: opng_bitset_t,
    pub strategy_set: opng_bitset_t,
    pub filter_set: opng_bitset_t,
    pub window_bits: libc::c_int,
    pub snip: libc::c_int,
    pub strip_all: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ui {
    pub printf_fn: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub print_cntrl_fn: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub progress_fn: Option::<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong) -> ()>,
    pub panic_fn: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_engine_struct {
    pub started: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_summary_struct {
    pub file_count: libc::c_uint,
    pub err_count: libc::c_uint,
    pub fix_count: libc::c_uint,
    pub snip_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_image_struct {
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub bit_depth: libc::c_int,
    pub color_type: libc::c_int,
    pub compression_type: libc::c_int,
    pub filter_type: libc::c_int,
    pub interlace_type: libc::c_int,
    pub row_pointers: png_bytepp,
    pub palette: png_colorp,
    pub num_palette: libc::c_int,
    pub background_ptr: png_color_16p,
    pub background: png_color_16,
    pub hist: png_uint_16p,
    pub sig_bit_ptr: png_color_8p,
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub num_trans: libc::c_int,
    pub trans_color_ptr: png_color_16p,
    pub trans_color: png_color_16,
    pub unknowns: png_unknown_chunkp,
    pub num_unknowns: libc::c_int,
}
pub type png_unknown_chunkp = *mut png_unknown_chunk;
pub type png_unknown_chunk = png_unknown_chunk_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_unknown_chunk_t {
    pub name: [png_byte; 5],
    pub data: *mut png_byte,
    pub size: size_t,
    pub location: png_byte,
}
pub type png_byte = libc::c_uchar;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_uint_16 = libc::c_ushort;
pub type png_color_16p = *mut png_color_16;
pub type png_bytep = *mut png_byte;
pub type png_color_8 = png_color_8_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_color_8p = *mut png_color_8;
pub type png_uint_16p = *mut png_uint_16;
pub type png_colorp = *mut png_color;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_bytepp = *mut *mut png_byte;
pub type png_uint_32 = libc::c_uint;
pub type opng_status_t = libc::c_int;
pub const OPNG_STATUS_ERR_OPTIPNG: opng_status_t = -2;
pub const OPNG_STATUS_ERR_LIBPNG: opng_status_t = -1;
pub const OPNG_STATUS_OK: opng_status_t = 0;
#[no_mangle]
pub static mut the_exception_context: libc::c_int = 0;
static mut s_engine: opng_engine_struct = opng_engine_struct { started: 0 };
static mut s_summary: opng_summary_struct = opng_summary_struct {
    file_count: 0,
    err_count: 0,
    fix_count: 0,
    snip_count: 0,
};
static mut s_image: opng_image_struct = opng_image_struct {
    width: 0,
    height: 0,
    bit_depth: 0,
    color_type: 0,
    compression_type: 0,
    filter_type: 0,
    interlace_type: 0,
    row_pointers: 0 as *const *mut png_byte as *mut *mut png_byte,
    palette: 0 as *const png_color as *mut png_color,
    num_palette: 0,
    background_ptr: 0 as *const png_color_16 as *mut png_color_16,
    background: png_color_16_struct {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    },
    hist: 0 as *const png_uint_16 as *mut png_uint_16,
    sig_bit_ptr: 0 as *const png_color_8 as *mut png_color_8,
    sig_bit: png_color_8_struct {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0,
    },
    trans_alpha: 0 as *const png_byte as *mut png_byte,
    num_trans: 0,
    trans_color_ptr: 0 as *const png_color_16 as *mut png_color_16,
    trans_color: png_color_16_struct {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    },
    unknowns: 0 as *const png_unknown_chunk as *mut png_unknown_chunk,
    num_unknowns: 0,
};
static mut s_options: opng_options = opng_options {
    backup: 0,
    clobber: 0,
    debug: 0,
    fix: 0,
    force: 0,
    full: 0,
    preserve: 0,
    quiet: 0,
    simulate: 0,
    verbose: 0,
    out_name: 0 as *const libc::c_char,
    dir_name: 0 as *const libc::c_char,
    log_name: 0 as *const libc::c_char,
    interlace: 0,
    nb: 0,
    nc: 0,
    np: 0,
    nz: 0,
    optim_level: 0,
    compr_level_set: 0,
    mem_level_set: 0,
    strategy_set: 0,
    filter_set: 0,
    window_bits: 0,
    snip: 0,
    strip_all: 0,
};
static mut usr_printf: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()> = None;
static mut usr_print_cntrl: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut usr_progress: Option::<
    unsafe extern "C" fn(libc::c_ulong, libc::c_ulong) -> (),
> = None;
static mut usr_panic: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()> = None;
unsafe extern "C" fn opng_clear_image_info() {
    memset(
        &mut s_image as *mut opng_image_struct as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<opng_image_struct>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn opng_initialize(
    mut init_options: *const opng_options,
    mut init_ui: *const opng_ui,
) -> libc::c_int {
    usr_printf = (*init_ui).printf_fn;
    usr_print_cntrl = (*init_ui).print_cntrl_fn;
    usr_progress = (*init_ui).progress_fn;
    usr_panic = (*init_ui).panic_fn;
    if usr_printf.is_none() || usr_print_cntrl.is_none() || usr_progress.is_none()
        || usr_panic.is_none()
    {
        return -(1 as libc::c_int);
    }
    s_options = *init_options;
    if s_options.optim_level == 0 as libc::c_int {
        s_options.np = 1 as libc::c_int;
        s_options.nc = s_options.np;
        s_options.nb = s_options.nc;
        s_options.nz = 1 as libc::c_int;
    }
    memset(
        &mut s_summary as *mut opng_summary_struct as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<opng_summary_struct>() as libc::c_ulong,
    );
    s_engine.started = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opng_optimize(
    mut infile_name: *const libc::c_char,
) -> libc::c_int {
    let mut status: opng_status_t = OPNG_STATUS_OK;
    let mut result: libc::c_int = 0;
    if s_engine.started == 0 {
        usr_panic
            .expect(
                "non-null function pointer",
            )(
            b"The OptiPNG engine is not running\0" as *const u8 as *const libc::c_char,
        );
    }
    usr_printf
        .expect(
            "non-null function pointer",
        )(b"** Processing: %s\n\0" as *const u8 as *const libc::c_char, infile_name);
    s_summary.file_count = (s_summary.file_count).wrapping_add(1);
    s_summary.file_count;
    opng_clear_image_info();
    usr_printf
        .expect(
            "non-null function pointer",
        )(b"\n\0" as *const u8 as *const libc::c_char);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_finalize() -> libc::c_int {
    if s_options.verbose != 0 || s_summary.snip_count > 0 as libc::c_int as libc::c_uint
        || s_summary.err_count > 0 as libc::c_int as libc::c_uint
    {
        usr_printf
            .expect(
                "non-null function pointer",
            )(b"** Status report\n\0" as *const u8 as *const libc::c_char);
        usr_printf
            .expect(
                "non-null function pointer",
            )(
            b"%u file(s) have been processed.\n\0" as *const u8 as *const libc::c_char,
            s_summary.file_count,
        );
        if s_summary.snip_count > 0 as libc::c_int as libc::c_uint {
            usr_printf
                .expect(
                    "non-null function pointer",
                )(
                b"%u multi-image file(s) have been snipped.\n\0" as *const u8
                    as *const libc::c_char,
                s_summary.snip_count,
            );
        }
        if s_summary.err_count > 0 as libc::c_int as libc::c_uint {
            usr_printf
                .expect(
                    "non-null function pointer",
                )(
                b"%u error(s) have been encountered.\n\0" as *const u8
                    as *const libc::c_char,
                s_summary.err_count,
            );
            if s_summary.fix_count > 0 as libc::c_int as libc::c_uint {
                usr_printf
                    .expect(
                        "non-null function pointer",
                    )(
                    b"%u erroneous file(s) have been fixed.\n\0" as *const u8
                        as *const libc::c_char,
                    s_summary.fix_count,
                );
            }
        }
    }
    s_engine.started = 0 as libc::c_int;
    return 0 as libc::c_int;
}
