use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    fn opng_strparse_rangeset_to_bitset(
        out_set: *mut opng_bitset_t,
        rangeset_str: *const libc::c_char,
        mask_set: opng_bitset_t,
    ) -> libc::c_int;
    fn opng_initialize(
        options_0: *const opng_options,
        ui: *const opng_ui,
    ) -> libc::c_int;
    fn opng_optimize(infile_name: *const libc::c_char) -> libc::c_int;
    fn opng_finalize() -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn png_get_libpng_ver(png_ptr: png_const_structrp) -> png_const_charp;
    fn zlibVersion() -> *const libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
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
pub type png_const_charp = *const libc::c_char;
pub type png_struct = png_struct_def;
pub type png_const_structrp = *const png_struct;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const OP_SHOW_VERSION: C2RustUnnamed_0 = 2;
pub const OP_SHOW_HELP: C2RustUnnamed_0 = 1;
pub const OP_RUN: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub help: libc::c_int,
    pub version: libc::c_int,
}
static mut msg_intro: *const libc::c_char = b"OptiPNG version 0.7.8\nCopyright (C) 2001-2023 Cosmin Truta and the Contributing Authors.\n\0"
    as *const u8 as *const libc::c_char;
static mut msg_license: *const libc::c_char = b"This program is open-source software. See LICENSE for more details.\n\nPortions of this software are based in part on the work of:\n  Jean-loup Gailly and Mark Adler (zlib)\n  Glenn Randers-Pehrson and the PNG Development Group (libpng)\n  Miyasaka Masaru (BMP support)\n  David Koblas (GIF support)\n\0"
    as *const u8 as *const libc::c_char;
static mut msg_help_synopsis: *const libc::c_char = b"Synopsis:\n    optipng [options] files ...\nFiles:\n    Image files of type: PNG, BMP, GIF, PNM or TIFF\n\0"
    as *const u8 as *const libc::c_char;
static mut msg_help_basic_options: *const libc::c_char = b"Basic options:\n    -?, -h, -help\tshow the extended help\n    -o <level>\t\toptimization level (0-7)\t\t[default: 2]\n    -v\t\t\trun in verbose mode / show copyright and version info\n\0"
    as *const u8 as *const libc::c_char;
static mut msg_help_options: *const libc::c_char = b"Basic options:\n    -?, -h, -help\tshow this help\n    -o <level>\t\toptimization level (0-7)\t\t[default: 2]\n    -v\t\t\trun in verbose mode / show copyright and version info\nGeneral options:\n    -backup, -keep\tkeep a backup of the modified files\n    -clobber\t\toverwrite existing files\n    -fix\t\tenable error recovery\n    -force\t\tenforce writing of a new output file\n    -preserve\t\tpreserve file attributes if possible\n    -quiet, -silent\trun in quiet mode\n    -simulate\t\trun in simulation mode\n    -out <file>\t\twrite output file to <file>\n    -dir <directory>\twrite output file(s) to <directory>\n    -log <file>\t\tlog messages to <file>\n    --\t\t\tstop option switch parsing\nOptimization options:\n    -f <filters>\tPNG delta filters (0-5)\t\t\t[default: 0,5]\n    -i <type>\t\tPNG interlace type (0-1)\n    -zc <levels>\tzlib compression levels (1-9)\t\t[default: 9]\n    -zm <levels>\tzlib memory levels (1-9)\t\t[default: 8]\n    -zs <strategies>\tzlib compression strategies (0-3)\t[default: 0-3]\n    -zw <size>\t\tzlib window size (256,512,1k,2k,4k,8k,16k,32k)\n    -full\t\tproduce a full report on IDAT (might reduce speed)\n    -nb\t\t\tno bit depth reduction\n    -nc\t\t\tno color type reduction\n    -np\t\t\tno palette reduction\n    -nx\t\t\tno reductions\n    -nz\t\t\tno IDAT recoding\nEditing options:\n    -snip\t\tcut one image out of multi-image or animation files\n    -strip <objects>\tstrip metadata objects (e.g. \"all\")\nOptimization levels:\n    -o0\t\t<=>\t-o1 -nx -nz\t\t\t\t(0 or 1 trials)\n    -o1\t\t<=>\t-zc9 -zm8 -zs0 -f0\t\t\t(1 trial)\n    \t\t(or...)\t-zc9 -zm8 -zs1 -f5\t\t\t(1 trial)\n    -o2\t\t<=>\t-zc9 -zm8 -zs0-3 -f0,5\t\t\t(8 trials)\n    -o3\t\t<=>\t-zc9 -zm8-9 -zs0-3 -f0,5\t\t(16 trials)\n    -o4\t\t<=>\t-zc9 -zm8 -zs0-3 -f0-5\t\t\t(24 trials)\n    -o5\t\t<=>\t-zc9 -zm8-9 -zs0-3 -f0-5\t\t(48 trials)\n    -o6\t\t<=>\t-zc1-9 -zm8 -zs0-3 -f0-5\t\t(120 trials)\n    -o7\t\t<=>\t-zc1-9 -zm8-9 -zs0-3 -f0-5\t\t(240 trials)\n    -o7 -zm1-9\t<=>\t-zc1-9 -zm1-9 -zs0-3 -f0-5\t\t(1080 trials)\nNotes:\n    The combination for -o1 is chosen heuristically.\n    Exhaustive combinations such as \"-o7 -zm1-9\" are not generally recommended.\n\0"
    as *const u8 as *const libc::c_char;
static mut msg_help_examples: *const libc::c_char = b"Examples:\n    optipng file.png\t\t\t\t\t\t(default speed)\n    optipng -o5 file.png\t\t\t\t\t(slow)\n    optipng -o7 file.png\t\t\t\t\t(very slow)\n\0"
    as *const u8 as *const libc::c_char;
static mut msg_help_more: *const libc::c_char = b"Type \"optipng -h\" for extended help.\n\0"
    as *const u8 as *const libc::c_char;
static mut operation: C2RustUnnamed_0 = OP_RUN;
static mut local_options: C2RustUnnamed_1 = C2RustUnnamed_1 {
    help: 0,
    version: 0,
};
static mut options: opng_options = opng_options {
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
static mut con_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut log_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut start_of_line: libc::c_int = 0;
unsafe extern "C" fn error(mut fmt: *const libc::c_char, mut args: ...) {
    let mut arg_ptr: ::core::ffi::VaListImpl;
    fprintf(stderr, b"** Error: \0" as *const u8 as *const libc::c_char);
    arg_ptr = args.clone();
    vfprintf(stderr, fmt, arg_ptr.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn panic(mut msg: *const libc::c_char) {
    fprintf(
        stderr,
        b"\n** INTERNAL ERROR: %s\n\0" as *const u8 as *const libc::c_char,
        msg,
    );
    fprintf(
        stderr,
        b"Please submit a defect report.\nhttp://optipng.sourceforge.net/\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    fflush(stderr);
    if options.debug != 0 {
        abort();
    } else {
        fprintf(
            stderr,
            b"The execution of this program has been terminated abnormally.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(70 as libc::c_int);
    };
}
unsafe extern "C" fn opng_strcasecmp(
    mut str1: *const libc::c_char,
    mut str2: *const libc::c_char,
) -> libc::c_int {
    let mut ch1: libc::c_int = 0;
    let mut ch2: libc::c_int = 0;
    loop {
        let fresh0 = str1;
        str1 = str1.offset(1);
        ch1 = tolower(*fresh0 as libc::c_int);
        let fresh1 = str2;
        str2 = str2.offset(1);
        ch2 = tolower(*fresh1 as libc::c_int);
        if ch1 != ch2 {
            return ch1 - ch2;
        }
        if ch1 == '\0' as i32 {
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn opng_strltrim(mut str: *const libc::c_char) -> *mut libc::c_char {
    while *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        str = str.offset(1);
        str;
    }
    return str as *mut libc::c_char;
}
unsafe extern "C" fn opng_strtail(
    mut str: *const libc::c_char,
    mut num: size_t,
) -> *mut libc::c_char {
    let mut len: size_t = 0;
    len = strlen(str);
    if len <= num {
        return str as *mut libc::c_char;
    }
    return (str as *mut libc::c_char).offset(len as isize).offset(-(num as isize));
}
unsafe extern "C" fn opng_strpbrk_digit(
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    loop {
        if *str as libc::c_int == '\0' as i32 {
            return 0 as *mut libc::c_char;
        }
        if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            return str as *mut libc::c_char;
        }
        str = str.offset(1);
        str;
    };
}
unsafe extern "C" fn opng_str2ulong(
    mut out_val: *mut libc::c_ulong,
    mut in_str: *const libc::c_char,
    mut allow_multiplier: libc::c_int,
) -> libc::c_int {
    let mut begin_ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut end_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut multiplier: libc::c_ulong = 0;
    end_ptr = opng_strltrim(in_str);
    begin_ptr = end_ptr;
    if *begin_ptr as libc::c_int >= '0' as i32 && *begin_ptr as libc::c_int <= '9' as i32
    {
        *out_val = strtoul(begin_ptr, &mut end_ptr, 10 as libc::c_int);
    }
    if begin_ptr == end_ptr as *const libc::c_char {
        *__errno_location() = 22 as libc::c_int;
        *out_val = 0 as libc::c_int as libc::c_ulong;
        return -(1 as libc::c_int);
    }
    if allow_multiplier != 0 {
        if *end_ptr as libc::c_int == 'k' as i32 || *end_ptr as libc::c_int == 'K' as i32
        {
            end_ptr = end_ptr.offset(1);
            end_ptr;
            multiplier = 1024 as libc::c_ulong;
        } else if *end_ptr as libc::c_int == 'M' as i32 {
            end_ptr = end_ptr.offset(1);
            end_ptr;
            multiplier = (1024 as libc::c_ulong).wrapping_mul(1024 as libc::c_ulong);
        } else if *end_ptr as libc::c_int == 'G' as i32 {
            end_ptr = end_ptr.offset(1);
            end_ptr;
            multiplier = (1024 as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_ulong);
        } else {
            multiplier = 1 as libc::c_int as libc::c_ulong;
        }
        if multiplier > 1 as libc::c_int as libc::c_ulong {
            if *out_val
                > (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_ulong)
                    .wrapping_add(1 as libc::c_ulong)
                    .wrapping_div(multiplier)
            {
                *__errno_location() = 34 as libc::c_int;
                *out_val = (9223372036854775807 as libc::c_long as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_ulong)
                    .wrapping_add(1 as libc::c_ulong);
            } else {
                *out_val = (*out_val).wrapping_mul(multiplier);
            }
        }
    }
    if *opng_strltrim(end_ptr) as libc::c_int != '\0' as i32 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn err_option_arg(
    mut opt: *const libc::c_char,
    mut opt_arg: *const libc::c_char,
) {
    if opt_arg.is_null() || *opng_strltrim(opt_arg) as libc::c_int == '\0' as i32 {
        error(
            b"Missing argument for option %s\0" as *const u8 as *const libc::c_char,
            opt,
        );
    } else {
        error(
            b"Invalid argument for option %s: %s\0" as *const u8 as *const libc::c_char,
            opt,
            opt_arg,
        );
    };
}
unsafe extern "C" fn check_num_option(
    mut opt: *const libc::c_char,
    mut opt_arg: *const libc::c_char,
    mut lowest: libc::c_int,
    mut highest: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_ulong = 0;
    if opng_str2ulong(&mut value, opt_arg, 0 as libc::c_int) != 0 as libc::c_int
        || value > 2147483647 as libc::c_int as libc::c_ulong
        || (value as libc::c_int) < lowest || value as libc::c_int > highest
    {
        err_option_arg(opt, opt_arg);
    }
    return value as libc::c_int;
}
unsafe extern "C" fn check_power2_option(
    mut opt: *const libc::c_char,
    mut opt_arg: *const libc::c_char,
    mut lowest: libc::c_int,
    mut highest: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_ulong = 0;
    let mut result: libc::c_int = 0;
    if opng_str2ulong(&mut value, opt_arg, 1 as libc::c_int) == 0 as libc::c_int {
        if lowest < 0 as libc::c_int {
            lowest = 0 as libc::c_int;
        }
        if highest
            > (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int
        {
            highest = (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        result = lowest;
        while result <= highest {
            if (1 as libc::c_ulong) << result == value {
                return result;
            }
            result += 1;
            result;
        }
    }
    err_option_arg(opt, opt_arg);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn check_rangeset_option(
    mut opt: *const libc::c_char,
    mut opt_arg: *const libc::c_char,
    mut result_mask: opng_bitset_t,
) -> opng_bitset_t {
    let mut result: opng_bitset_t = 0;
    if opng_strparse_rangeset_to_bitset(&mut result, opt_arg, result_mask)
        != 0 as libc::c_int
    {
        result = 0 as libc::c_uint;
    }
    if result & result_mask != result {
        result = 0 as libc::c_uint;
    }
    if result == 0 as libc::c_uint {
        err_option_arg(opt, opt_arg);
    }
    return result;
}
unsafe extern "C" fn check_obj_option(
    mut opt: *const libc::c_char,
    mut opt_arg: *const libc::c_char,
) {
    let mut i: libc::c_uint = 0;
    if strcmp(b"all\0" as *const u8 as *const libc::c_char, opt_arg) == 0 as libc::c_int
    {
        return;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        if !(*opt_arg.offset(i as isize) as libc::c_int >= 'A' as i32
            && *opt_arg.offset(i as isize) as libc::c_int <= 'Z' as i32
            || *opt_arg.offset(i as isize) as libc::c_int >= 'a' as i32
                && *opt_arg.offset(i as isize) as libc::c_int <= 'z' as i32)
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == 4 as libc::c_int as libc::c_uint
        && *opt_arg.offset(i as isize) as libc::c_int == '\0' as i32
    {
        error(
            b"Manipulation of individual chunks is not implemented\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        err_option_arg(opt, opt_arg);
    };
}
unsafe extern "C" fn scan_option(
    mut str: *const libc::c_char,
    mut opt_buf: *mut libc::c_char,
    mut opt_buf_size: size_t,
    mut opt_arg_ptr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt_len: libc::c_uint = 0;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
        || *str.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    opt_len = 0 as libc::c_int as libc::c_uint;
    ptr = str.offset(1 as libc::c_int as isize);
    while *ptr as libc::c_int == '-' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr as libc::c_int == '\0' as i32 {
        ptr = ptr.offset(-1);
        ptr;
    }
    loop {
        if (opt_len as size_t) < opt_buf_size {
            *opt_buf
                .offset(opt_len as isize) = tolower(*ptr as libc::c_int) as libc::c_char;
        }
        opt_len = opt_len.wrapping_add(1);
        opt_len;
        ptr = ptr.offset(1);
        ptr;
        if *ptr as libc::c_int == '\0' as i32
            || *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                ptr = ptr.offset(1);
                ptr;
            }
            *opt_arg_ptr = if *ptr as libc::c_int != '\0' as i32 {
                ptr
            } else {
                0 as *const libc::c_char
            };
            break;
        } else {
            if !(*ptr as libc::c_int == '=' as i32) {
                continue;
            }
            ptr = ptr.offset(1);
            ptr;
            *opt_arg_ptr = ptr;
            break;
        }
    }
    if opt_buf_size > 0 as libc::c_int as size_t {
        if (opt_len as size_t) < opt_buf_size {
            *opt_buf.offset(opt_len as isize) = '\0' as i32 as libc::c_char;
        } else {
            *opt_buf
                .offset(
                    opt_buf_size.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opt: [libc::c_char; 16] = [0; 16];
    let mut opt_len: size_t = 0;
    let mut xopt: *const libc::c_char = 0 as *const libc::c_char;
    let mut simple_opt: libc::c_int = 0;
    let mut stop_switch: libc::c_int = 0;
    let mut set: opng_bitset_t = 0;
    let mut val: libc::c_int = 0;
    let mut file_count: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    memset(
        &mut options as *mut opng_options as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<opng_options>() as libc::c_ulong,
    );
    options.optim_level = -(1 as libc::c_int);
    options.interlace = -(1 as libc::c_int);
    file_count = 0 as libc::c_int as libc::c_uint;
    stop_switch = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        arg = *argv.offset(i as isize);
        if stop_switch != 0
            || scan_option(
                arg,
                opt.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                &mut xopt,
            ) < 1 as libc::c_int
        {
            file_count = file_count.wrapping_add(1);
            file_count;
        } else {
            opt_len = strlen(opt.as_mut_ptr());
            let ref mut fresh2 = *argv.offset(i as isize);
            *fresh2 = 0 as *mut libc::c_char;
            if !(strchr(
                b"fio\0" as *const u8 as *const libc::c_char,
                opt[0 as libc::c_int as usize] as libc::c_int,
            ))
                .is_null()
                && *(*__ctype_b_loc())
                    .offset(opt[1 as libc::c_int as usize] as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || opt[0 as libc::c_int as usize] as libc::c_int == 'z' as i32
                    && *(*__ctype_b_loc())
                        .offset(opt[1 as libc::c_int as usize] as libc::c_int as isize)
                        as libc::c_int
                        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(opt[2 as libc::c_int as usize] as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                opt_len = (opng_strpbrk_digit(opt.as_mut_ptr()))
                    .offset_from(opt.as_mut_ptr()) as libc::c_long as size_t;
                opt[opt_len as usize] = '\0' as i32 as libc::c_char;
                xopt = opng_strpbrk_digit(arg);
            }
            simple_opt = 1 as libc::c_int;
            if strcmp(b"-\0" as *const u8 as *const libc::c_char, opt.as_mut_ptr())
                == 0 as libc::c_int
            {
                stop_switch = 1 as libc::c_int;
            } else if strcmp(
                b"?\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
                || strncmp(
                    b"help\0" as *const u8 as *const libc::c_char,
                    opt.as_mut_ptr(),
                    opt_len,
                ) == 0 as libc::c_int
            {
                local_options.help = 1 as libc::c_int;
            } else if strncmp(
                b"backup\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int
                || strncmp(
                    b"keep\0" as *const u8 as *const libc::c_char,
                    opt.as_mut_ptr(),
                    opt_len,
                ) == 0 as libc::c_int
            {
                options.backup = 1 as libc::c_int;
            } else if strncmp(
                b"clobber\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int
            {
                options.clobber = 1 as libc::c_int;
            } else if strcmp(
                b"debug\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.debug = 1 as libc::c_int;
            } else if strncmp(
                b"fix\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 2 as libc::c_int as size_t
            {
                options.fix = 1 as libc::c_int;
            } else if strncmp(
                b"force\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 2 as libc::c_int as size_t
            {
                options.force = 1 as libc::c_int;
            } else if strncmp(
                b"full\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 2 as libc::c_int as size_t
            {
                options.full = 1 as libc::c_int;
            } else if strcmp(
                b"nb\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.nb = 1 as libc::c_int;
            } else if strcmp(
                b"nc\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.nc = 1 as libc::c_int;
            } else if strcmp(
                b"np\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.np = 1 as libc::c_int;
            } else if strcmp(
                b"nx\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.np = 1 as libc::c_int;
                options.nc = options.np;
                options.nb = options.nc;
            } else if strcmp(
                b"nz\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.nz = 1 as libc::c_int;
            } else if strncmp(
                b"preserve\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int
            {
                options.preserve = 1 as libc::c_int;
            } else if strncmp(
                b"quiet\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int
                || strncmp(
                    b"silent\0" as *const u8 as *const libc::c_char,
                    opt.as_mut_ptr(),
                    opt_len,
                ) == 0 as libc::c_int && opt_len >= 3 as libc::c_int as size_t
            {
                options.quiet = 1 as libc::c_int;
            } else if strncmp(
                b"simulate\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 3 as libc::c_int as size_t
            {
                options.simulate = 1 as libc::c_int;
            } else if strncmp(
                b"snip\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 2 as libc::c_int as size_t
            {
                options.snip = 1 as libc::c_int;
            } else if strcmp(
                b"v\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                options.verbose = 1 as libc::c_int;
                local_options.version = 1 as libc::c_int;
            } else if strncmp(
                b"verbose\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 4 as libc::c_int as size_t
            {
                options.verbose = 1 as libc::c_int;
            } else if strncmp(
                b"version\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 4 as libc::c_int as size_t
            {
                local_options.version = 1 as libc::c_int;
            } else {
                simple_opt = 0 as libc::c_int;
                if xopt.is_null() {
                    i += 1;
                    if i < argc {
                        xopt = *argv.offset(i as isize);
                        let ref mut fresh3 = *argv.offset(i as isize);
                        *fresh3 = 0 as *mut libc::c_char;
                    } else {
                        xopt = b"\0" as *const u8 as *const libc::c_char;
                    }
                }
            }
            if simple_opt != 0 {
                if !xopt.is_null() {
                    error(
                        b"No argument allowed for option: %s\0" as *const u8
                            as *const libc::c_char,
                        arg,
                    );
                }
            } else if strcmp(
                b"o\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                val = check_num_option(
                    b"-o\0" as *const u8 as *const libc::c_char,
                    xopt,
                    0 as libc::c_int,
                    2147483647 as libc::c_int,
                );
                if options.optim_level < 0 as libc::c_int {
                    options.optim_level = val;
                } else if options.optim_level != val {
                    error(
                        b"Multiple optimization levels are not permitted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if strcmp(
                b"i\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                val = check_num_option(
                    b"-i\0" as *const u8 as *const libc::c_char,
                    xopt,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                if options.interlace < 0 as libc::c_int {
                    options.interlace = val;
                } else if options.interlace != val {
                    error(
                        b"Multiple interlace types are not permitted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if strcmp(
                b"f\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                set = check_rangeset_option(
                    b"-f\0" as *const u8 as *const libc::c_char,
                    xopt,
                    (((1 as libc::c_int) << 5 as libc::c_int + 1 as libc::c_int)
                        - ((1 as libc::c_int) << 0 as libc::c_int)) as opng_bitset_t,
                );
                options.filter_set |= set;
            } else if strcmp(
                b"zc\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                set = check_rangeset_option(
                    b"-zc\0" as *const u8 as *const libc::c_char,
                    xopt,
                    (((1 as libc::c_int) << 9 as libc::c_int + 1 as libc::c_int)
                        - ((1 as libc::c_int) << 1 as libc::c_int)) as opng_bitset_t,
                );
                options.compr_level_set |= set;
            } else if strcmp(
                b"zm\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                set = check_rangeset_option(
                    b"-zm\0" as *const u8 as *const libc::c_char,
                    xopt,
                    (((1 as libc::c_int) << 9 as libc::c_int + 1 as libc::c_int)
                        - ((1 as libc::c_int) << 1 as libc::c_int)) as opng_bitset_t,
                );
                options.mem_level_set |= set;
            } else if strcmp(
                b"zs\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                set = check_rangeset_option(
                    b"-zs\0" as *const u8 as *const libc::c_char,
                    xopt,
                    (((1 as libc::c_int) << 3 as libc::c_int + 1 as libc::c_int)
                        - ((1 as libc::c_int) << 0 as libc::c_int)) as opng_bitset_t,
                );
                options.strategy_set |= set;
            } else if strcmp(
                b"zw\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
            ) == 0 as libc::c_int
            {
                val = check_power2_option(
                    b"-zw\0" as *const u8 as *const libc::c_char,
                    xopt,
                    8 as libc::c_int,
                    15 as libc::c_int,
                );
                if options.window_bits == 0 as libc::c_int {
                    options.window_bits = val;
                } else if options.window_bits != val {
                    error(
                        b"Multiple window sizes are not permitted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if strncmp(
                b"strip\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 2 as libc::c_int as size_t
            {
                check_obj_option(b"-strip\0" as *const u8 as *const libc::c_char, xopt);
                options.strip_all = 1 as libc::c_int;
            } else if strncmp(
                b"out\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int && opt_len >= 2 as libc::c_int as size_t
            {
                if !(options.out_name).is_null() {
                    error(
                        b"Multiple output file names are not permitted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if *xopt.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    err_option_arg(
                        b"-out\0" as *const u8 as *const libc::c_char,
                        0 as *const libc::c_char,
                    );
                }
                options.out_name = xopt;
            } else if strncmp(
                b"dir\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int
            {
                if !(options.dir_name).is_null() {
                    error(
                        b"Multiple output dir names are not permitted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if *xopt.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    err_option_arg(
                        b"-dir\0" as *const u8 as *const libc::c_char,
                        0 as *const libc::c_char,
                    );
                }
                options.dir_name = xopt;
            } else if strncmp(
                b"log\0" as *const u8 as *const libc::c_char,
                opt.as_mut_ptr(),
                opt_len,
            ) == 0 as libc::c_int
            {
                if !(options.log_name).is_null() {
                    error(
                        b"Multiple log file names are not permitted\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if *xopt.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    err_option_arg(
                        b"-log\0" as *const u8 as *const libc::c_char,
                        0 as *const libc::c_char,
                    );
                }
                options.log_name = xopt;
            } else {
                error(
                    b"Unrecognized option: %s\0" as *const u8 as *const libc::c_char,
                    arg,
                );
            }
        }
        i += 1;
        i;
    }
    if !(options.out_name).is_null() {
        if file_count > 1 as libc::c_int as libc::c_uint {
            error(
                b"The option -out requires one input file\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !(options.dir_name).is_null() {
            error(
                b"The options -out and -dir are mutually exclusive\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if !(options.log_name).is_null() {
        if opng_strcasecmp(
            b".log\0" as *const u8 as *const libc::c_char,
            opng_strtail(options.log_name, 4 as libc::c_int as size_t),
        ) != 0 as libc::c_int
        {
            error(
                b"To prevent accidental data corruption, the log file name must end with \".log\"\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if local_options.help != 0 {
        operation = OP_SHOW_HELP;
    } else if file_count != 0 as libc::c_int as libc::c_uint {
        operation = OP_RUN;
    } else if local_options.version != 0 {
        operation = OP_SHOW_VERSION;
    } else {
        operation = OP_SHOW_HELP;
    };
}
unsafe extern "C" fn app_printf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut arg_ptr: ::core::ffi::VaListImpl;
    if *fmt.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return;
    }
    start_of_line = if *fmt
        .offset((strlen(fmt)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\n' as i32
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if !con_file.is_null() {
        arg_ptr = args.clone();
        vfprintf(con_file, fmt, arg_ptr.as_va_list());
    }
    if !log_file.is_null() {
        arg_ptr = args.clone();
        vfprintf(log_file, fmt, arg_ptr.as_va_list());
    }
}
unsafe extern "C" fn app_print_cntrl(mut cntrl_code: libc::c_int) {
    let mut con_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut log_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if cntrl_code == '\r' as i32 {
        con_str = b"\r\0" as *const u8 as *const libc::c_char;
        log_str = b"\n\0" as *const u8 as *const libc::c_char;
        start_of_line = 1 as libc::c_int;
    } else if cntrl_code == '\u{b}' as i32 {
        if start_of_line == 0 {
            log_str = b"\n\0" as *const u8 as *const libc::c_char;
            con_str = log_str;
            start_of_line = 1 as libc::c_int;
        } else {
            log_str = b"\0" as *const u8 as *const libc::c_char;
            con_str = log_str;
        }
    } else if cntrl_code < 0 as libc::c_int && cntrl_code > -(80 as libc::c_int)
        && start_of_line != 0
    {
        if !con_file.is_null() {
            i = 0 as libc::c_int;
            while i > cntrl_code {
                fputc(' ' as i32, con_file);
                i -= 1;
                i;
            }
        }
        con_str = b"\r\0" as *const u8 as *const libc::c_char;
        log_str = b"\0" as *const u8 as *const libc::c_char;
    } else {
        log_str = b"<?>\0" as *const u8 as *const libc::c_char;
        con_str = log_str;
    }
    if !con_file.is_null() {
        fputs(con_str, con_file);
    }
    if !log_file.is_null() {
        fputs(log_str, log_file);
    }
}
unsafe extern "C" fn app_progress(
    mut current_step: libc::c_ulong,
    mut total_steps: libc::c_ulong,
) {
    if !con_file.is_null() {
        fflush(con_file);
    }
    if current_step != 0 && total_steps != 0 {
        return;
    }
}
unsafe extern "C" fn app_init() {
    start_of_line = 1 as libc::c_int;
    if operation as libc::c_uint == OP_SHOW_HELP as libc::c_int as libc::c_uint
        || operation as libc::c_uint == OP_SHOW_VERSION as libc::c_int as libc::c_uint
    {
        con_file = stdout;
    } else if options.quiet == 0 {
        con_file = stderr;
    } else {
        con_file = 0 as *mut FILE;
    }
    if !(options.log_name).is_null() {
        log_file = fopen(options.log_name, b"a\0" as *const u8 as *const libc::c_char);
        if log_file.is_null() {
            error(
                b"Can't open log file: %s\n\0" as *const u8 as *const libc::c_char,
                options.log_name,
            );
        }
        setvbuf(
            log_file,
            0 as *mut libc::c_char,
            1 as libc::c_int,
            8192 as libc::c_int as size_t,
        );
        app_printf(
            b"** Warning: %s\n\n\0" as *const u8 as *const libc::c_char,
            b"The option -log is deprecated; use shell redirection\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn app_finish() {
    if !log_file.is_null() {
        fclose(log_file);
    }
}
unsafe extern "C" fn process_files(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut ui: opng_ui = opng_ui {
        printf_fn: None,
        print_cntrl_fn: None,
        progress_fn: None,
        panic_fn: None,
    };
    let mut i: libc::c_int = 0;
    ui
        .printf_fn = Some(
        app_printf as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
    );
    ui.print_cntrl_fn = Some(app_print_cntrl as unsafe extern "C" fn(libc::c_int) -> ());
    ui
        .progress_fn = Some(
        app_progress as unsafe extern "C" fn(libc::c_ulong, libc::c_ulong) -> (),
    );
    ui.panic_fn = Some(panic as unsafe extern "C" fn(*const libc::c_char) -> ());
    if opng_initialize(&mut options, &mut ui) != 0 as libc::c_int {
        panic(
            b"Can't initialize optimization engine\0" as *const u8 as *const libc::c_char,
        );
    }
    result = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if !((*argv.offset(i as isize)).is_null()
            || *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32)
        {
            if opng_optimize(*argv.offset(i as isize)) != 0 as libc::c_int {
                result = 1 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    if opng_finalize() != 0 as libc::c_int {
        panic(
            b"Can't finalize optimization engine\0" as *const u8 as *const libc::c_char,
        );
    }
    return result;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    parse_args(argc, argv);
    app_init();
    result = 0 as libc::c_int;
    if local_options.version != 0 {
        app_printf(b"%s\n\0" as *const u8 as *const libc::c_char, msg_intro);
    }
    match operation as libc::c_uint {
        0 => {
            result = process_files(argc, argv);
        }
        1 => {
            if local_options.help != 0 {
                app_printf(
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    msg_help_synopsis,
                    msg_help_options,
                    msg_help_examples,
                );
            } else {
                app_printf(
                    b"%s%s%s%s\0" as *const u8 as *const libc::c_char,
                    msg_help_synopsis,
                    msg_help_basic_options,
                    msg_help_examples,
                    msg_help_more,
                );
            }
        }
        2 => {
            app_printf(b"%s\n\0" as *const u8 as *const libc::c_char, msg_license);
            app_printf(
                b"Using libpng version %s and zlib version %s\n\0" as *const u8
                    as *const libc::c_char,
                png_get_libpng_ver(0 as *const png_struct),
                zlibVersion(),
            );
        }
        _ => {
            result = -(1 as libc::c_int);
        }
    }
    app_finish();
    return result;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
