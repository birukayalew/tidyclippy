use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _trio_string_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn mblen(__s: *const libc::c_char, __n: size_t) -> libc::c_int;
    fn trio_nan() -> libc::c_double;
    fn trio_pinf() -> libc::c_double;
    fn trio_ninf() -> libc::c_double;
    fn trio_isinf(number: libc::c_double) -> libc::c_int;
    fn trio_fpclassify_and_signbit(
        number: libc::c_double,
        is_negative: *mut libc::c_int,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn trio_copy_max(
        target: *mut libc::c_char,
        max: size_t,
        source: *const libc::c_char,
    ) -> libc::c_int;
    fn trio_destroy(string: *mut libc::c_char);
    fn trio_duplicate(source: *const libc::c_char) -> *mut libc::c_char;
    fn trio_equal(
        first: *const libc::c_char,
        second: *const libc::c_char,
    ) -> libc::c_int;
    fn trio_equal_case(
        first: *const libc::c_char,
        second: *const libc::c_char,
    ) -> libc::c_int;
    fn trio_equal_locale(
        first: *const libc::c_char,
        second: *const libc::c_char,
    ) -> libc::c_int;
    fn trio_equal_max(
        first: *const libc::c_char,
        max: size_t,
        second: *const libc::c_char,
    ) -> libc::c_int;
    fn trio_error(_: libc::c_int) -> *const libc::c_char;
    fn trio_length(string: *const libc::c_char) -> size_t;
    fn trio_to_double(
        source: *const libc::c_char,
        endp: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn trio_to_long(
        source: *const libc::c_char,
        endp: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_long;
    fn trio_to_long_double(
        source: *const libc::c_char,
        endp: *mut *mut libc::c_char,
    ) -> trio_long_double_t;
    fn trio_to_upper(source: libc::c_int) -> libc::c_int;
    fn trio_string_destroy(self_0: *mut trio_string_t);
    fn trio_string_extract(self_0: *mut trio_string_t) -> *mut libc::c_char;
    fn trio_string_terminate(self_0: *mut trio_string_t);
    fn trio_xstring_append_char(
        self_0: *mut trio_string_t,
        character: libc::c_char,
    ) -> libc::c_int;
    fn trio_xstring_duplicate(other: *const libc::c_char) -> *mut trio_string_t;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn powl(_: f128::f128, _: f128::f128) -> f128::f128;
    fn floorl(_: f128::f128) -> f128::f128;
    fn fmodl(_: f128::f128, _: f128::f128) -> f128::f128;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn localeconv() -> *mut lconv;
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
pub type trio_long_double_t = f128::f128;
pub type trio_pointer_t = *mut libc::c_void;
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type va_list = __gnuc_va_list;
pub type ssize_t = __ssize_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const TRIO_ECUSTOM: C2RustUnnamed = 9;
pub const TRIO_ERRNO: C2RustUnnamed = 8;
pub const TRIO_ERANGE: C2RustUnnamed = 7;
pub const TRIO_ENOMEM: C2RustUnnamed = 6;
pub const TRIO_EGAP: C2RustUnnamed = 5;
pub const TRIO_EDBLREF: C2RustUnnamed = 4;
pub const TRIO_ETOOMANY: C2RustUnnamed = 3;
pub const TRIO_EINVAL: C2RustUnnamed = 2;
pub const TRIO_EOF: C2RustUnnamed = 1;
pub type trio_outstream_t = Option::<
    unsafe extern "C" fn(trio_pointer_t, libc::c_int) -> libc::c_int,
>;
pub type trio_instream_t = Option::<unsafe extern "C" fn(trio_pointer_t) -> libc::c_int>;
pub type trio_class_t = _trio_class_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trio_class_t {
    pub OutStream: Option::<unsafe extern "C" fn(*mut _trio_class_t, libc::c_int) -> ()>,
    pub InStream: Option::<
        unsafe extern "C" fn(*mut _trio_class_t, *mut libc::c_int) -> (),
    >,
    pub location: trio_pointer_t,
    pub current: libc::c_int,
    pub processed: libc::c_int,
    pub committed: libc::c_int,
    pub max: libc::c_int,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trio_parameter_t {
    pub type_0: libc::c_int,
    pub flags: trio_flags_t,
    pub width: libc::c_int,
    pub precision: libc::c_int,
    pub base: libc::c_int,
    pub varsize: libc::c_int,
    pub indexAfterSpecifier: libc::c_int,
    pub data: C2RustUnnamed_0,
    pub user_name: [libc::c_char; 64],
    pub user_data: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: *mut libc::c_char,
    pub pointer: trio_pointer_t,
    pub number: C2RustUnnamed_1,
    pub doubleNumber: libc::c_double,
    pub doublePointer: *mut libc::c_double,
    pub longdoubleNumber: trio_long_double_t,
    pub longdoublePointer: *mut trio_long_double_t,
    pub errorNumber: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub as_signed: trio_intmax_t,
    pub as_unsigned: trio_uintmax_t,
}
pub type trio_uintmax_t = uintmax_t;
pub type uintmax_t = __uintmax_t;
pub type trio_intmax_t = intmax_t;
pub type intmax_t = __intmax_t;
pub type trio_flags_t = libc::c_ulong;
pub type trio_reference_t = _trio_reference_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trio_reference_t {
    pub data: *mut trio_class_t,
    pub parameter: *mut trio_parameter_t,
}
pub type trio_callback_t = Option::<unsafe extern "C" fn(trio_pointer_t) -> libc::c_int>;
pub type trio_userdef_t = _trio_userdef_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trio_userdef_t {
    pub next: *mut _trio_userdef_t,
    pub callback: trio_callback_t,
    pub name: *mut libc::c_char,
}
pub const BASE_DECIMAL: C2RustUnnamed_5 = 10;
pub const FLAGS_LEFTADJUST: C2RustUnnamed_5 = 8;
pub const NO_PRECISION: C2RustUnnamed_5 = -1;
pub const FLAGS_NILPADDING: C2RustUnnamed_5 = 8192;
pub const FLAGS_UPPER: C2RustUnnamed_5 = 32768;
pub const BASE_HEX: C2RustUnnamed_5 = 16;
pub const BASE_OCTAL: C2RustUnnamed_5 = 8;
pub const BASE_BINARY: C2RustUnnamed_5 = 2;
pub const FLAGS_ALTERNATIVE: C2RustUnnamed_5 = 16;
pub const FLAGS_SPACE: C2RustUnnamed_5 = 2;
pub const FLAGS_SHOWSIGN: C2RustUnnamed_5 = 4;
pub const FLAGS_QUOTE: C2RustUnnamed_5 = 16777216;
pub const FLAGS_LONG: C2RustUnnamed_5 = 128;
pub type trio_ulonglong_t = libc::c_ulonglong;
pub const FLAGS_QUAD: C2RustUnnamed_5 = 256;
pub const FLAGS_UNSIGNED: C2RustUnnamed_5 = 16384;
pub const NO_BASE: C2RustUnnamed_5 = -1;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const FLAGS_SHORT: C2RustUnnamed_5 = 32;
pub const FLAGS_INTMAX_T: C2RustUnnamed_5 = 4096;
pub type ptrdiff_t = libc::c_long;
pub const FLAGS_PTRDIFF_T: C2RustUnnamed_5 = 2048;
pub const FLAGS_SIZE_T: C2RustUnnamed_5 = 1024;
pub const FLAGS_FLOAT_E: C2RustUnnamed_5 = 4194304;
pub const FLAGS_ROUNDING: C2RustUnnamed_5 = 4096;
pub const FLAGS_FLOAT_G: C2RustUnnamed_5 = 8388608;
pub const FLAGS_LONGDOUBLE: C2RustUnnamed_5 = 512;
pub const TRIO_FP_INFINITE: C2RustUnnamed_3 = 0;
pub const TRIO_FP_NAN: C2RustUnnamed_3 = 1;
pub const FLAGS_BASE_PARAMETER: C2RustUnnamed_5 = 2097152;
pub const FLAGS_PRECISION_PARAMETER: C2RustUnnamed_5 = 524288;
pub const FLAGS_PRECISION: C2RustUnnamed_5 = 262144;
pub const FLAGS_WIDTH_PARAMETER: C2RustUnnamed_5 = 131072;
pub const TYPE_PRINT: C2RustUnnamed_5 = 1;
pub const TYPE_SCAN: C2RustUnnamed_5 = 2;
pub const FLAGS_USER_DEFINED: C2RustUnnamed_5 = 134217728;
pub type trio_longlong_t = libc::c_longlong;
pub const FLAGS_ALL_VARSIZES: C2RustUnnamed_5 = 7552;
pub const FLAGS_VARSIZE_PARAMETER: C2RustUnnamed_5 = 536870912;
pub const FLAGS_FIXED_SIZE: C2RustUnnamed_5 = 1073741824;
pub const FLAGS_IGNORE: C2RustUnnamed_5 = 134217728;
pub const NO_WIDTH: C2RustUnnamed_5 = 0;
pub const FLAGS_STICKY: C2RustUnnamed_5 = 1;
pub const MAX_USER_NAME: C2RustUnnamed_5 = 64;
pub const MAX_USER_DATA: C2RustUnnamed_5 = 256;
pub const FLAGS_WIDECHAR: C2RustUnnamed_5 = 33554432;
pub type trio_int8_t = int8_t;
pub type trio_int16_t = int16_t;
pub type trio_int32_t = int32_t;
pub type trio_int64_t = int64_t;
pub const FLAGS_ALL_SIZES: C2RustUnnamed_5 = 7648;
pub const FLAGS_SHORTSHORT: C2RustUnnamed_5 = 64;
pub const FLAGS_WIDTH: C2RustUnnamed_5 = 65536;
pub const NO_POSITION: C2RustUnnamed_5 = -1;
pub const MAX_BASE: C2RustUnnamed_5 = 36;
pub const FLAGS_BASE: C2RustUnnamed_5 = 1048576;
pub const NO_SIZE: C2RustUnnamed_5 = -1;
pub const MAX_PARAMETERS: C2RustUnnamed_5 = 64;
pub const FLAGS_NEW: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trio_custom_t {
    pub stream: C2RustUnnamed_2,
    pub closure: trio_pointer_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub out: trio_outstream_t,
    pub in_0: trio_instream_t,
}
pub type trio_string_t = _trio_string_t;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub const FLAGS_EXCLUDE: C2RustUnnamed_5 = 32;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const MAX_CHARACTER_CLASS: C2RustUnnamed_5 = 256;
pub const _ISupper: C2RustUnnamed_4 = 256;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TRIO_FP_ZERO: C2RustUnnamed_3 = 4;
pub const TRIO_FP_SUBNORMAL: C2RustUnnamed_3 = 3;
pub const TRIO_FP_NORMAL: C2RustUnnamed_3 = 2;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_int;
pub const DYNAMIC_START_SIZE: C2RustUnnamed_5 = 32;
pub const MAX_LOCALE_GROUPS: C2RustUnnamed_5 = 64;
pub const MAX_LOCALE_SEPARATOR_LENGTH: C2RustUnnamed_5 = 16;
pub const MIN_BASE: C2RustUnnamed_5 = 2;
pub const FLAGS_LAST: C2RustUnnamed_5 = 1073741824;
pub const FLAGS_IGNORE_PARAMETER: C2RustUnnamed_5 = 268435456;
pub const FLAGS_ALLOC: C2RustUnnamed_5 = 67108864;
static mut internalNullString: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"(nil)\0")
};
static mut internalLocaleValues: *mut lconv = 0 as *const lconv as *mut lconv;
static mut internalDecimalPointLength: libc::c_int = 1 as libc::c_int;
static mut internalThousandSeparatorLength: libc::c_int = 1 as libc::c_int;
static mut internalDecimalPoint: libc::c_char = '.' as i32 as libc::c_char;
static mut internalDecimalPointString: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b".\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
};
static mut internalThousandSeparator: [libc::c_char; 17] = unsafe {
    *::core::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b",\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
};
static mut internalGrouping: [libc::c_char; 64] = [
    127 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut internalDigitsLower: [libc::c_char; 37] = unsafe {
    *::core::mem::transmute::<
        &[u8; 37],
        &[libc::c_char; 37],
    >(b"0123456789abcdefghijklmnopqrstuvwxyz\0")
};
static mut internalDigitsUpper: [libc::c_char; 37] = unsafe {
    *::core::mem::transmute::<
        &[u8; 37],
        &[libc::c_char; 37],
    >(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\0")
};
static mut internalDigitsUnconverted: libc::c_int = !(1 as libc::c_int
    == 0 as libc::c_int) as libc::c_int;
static mut internalDigitArray: [libc::c_int; 128] = [0; 128];
static mut internalCollationUnconverted: libc::c_int = !(1 as libc::c_int
    == 0 as libc::c_int) as libc::c_int;
static mut internalCollationArray: [[libc::c_char; 256]; 256] = [[0; 256]; 256];
static mut internalEnterCriticalRegion: trio_callback_t = None;
static mut internalLeaveCriticalRegion: trio_callback_t = None;
static mut internalUserDef: *mut trio_userdef_t = 0 as *const trio_userdef_t
    as *mut trio_userdef_t;
unsafe extern "C" fn TrioIsQualifier(character: libc::c_char) -> libc::c_int {
    match character as libc::c_int {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 43 | 45 | 32 | 46 | 42 | 35
        | 104 | 108 | 76 | 94 | 122 | 116 | 106 | 113 | 90 | 119 | 39 | 33 | 38 | 64 | 73
        | 82 => return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int,
        _ => return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int,
    };
}
unsafe extern "C" fn TrioSetLocale() {
    internalLocaleValues = localeconv();
    if !internalLocaleValues.is_null() {
        if !((*internalLocaleValues).decimal_point).is_null()
            && *((*internalLocaleValues).decimal_point).offset(0 as libc::c_int as isize)
                as libc::c_int != 0 as libc::c_int as libc::c_char as libc::c_int
        {
            internalDecimalPointLength = trio_length(
                (*internalLocaleValues).decimal_point,
            ) as libc::c_int;
            if internalDecimalPointLength == 1 as libc::c_int {
                internalDecimalPoint = *((*internalLocaleValues).decimal_point)
                    .offset(0 as libc::c_int as isize);
            } else {
                internalDecimalPoint = 0 as libc::c_int as libc::c_char;
                trio_copy_max(
                    internalDecimalPointString.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
                    (*internalLocaleValues).decimal_point,
                );
            }
        }
        if !((*internalLocaleValues).thousands_sep).is_null()
            && *((*internalLocaleValues).thousands_sep).offset(0 as libc::c_int as isize)
                as libc::c_int != 0 as libc::c_int as libc::c_char as libc::c_int
        {
            trio_copy_max(
                internalThousandSeparator.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
                (*internalLocaleValues).thousands_sep,
            );
            internalThousandSeparatorLength = trio_length(
                internalThousandSeparator.as_mut_ptr(),
            ) as libc::c_int;
        }
        if !((*internalLocaleValues).grouping).is_null()
            && *((*internalLocaleValues).grouping).offset(0 as libc::c_int as isize)
                as libc::c_int != 0 as libc::c_int as libc::c_char as libc::c_int
        {
            trio_copy_max(
                internalGrouping.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                (*internalLocaleValues).grouping,
            );
        }
    }
}
unsafe extern "C" fn TrioCalcThousandSeparatorLength(
    mut digits: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut step: libc::c_int = 127 as libc::c_int;
    let mut groupingPointer: *mut libc::c_char = internalGrouping.as_mut_ptr();
    while digits > 0 as libc::c_int {
        if *groupingPointer as libc::c_int == 127 as libc::c_int {
            break;
        } else {
            if *groupingPointer as libc::c_int == 0 as libc::c_int {
                if step == 127 as libc::c_int {
                    break;
                }
            } else {
                let fresh0 = groupingPointer;
                groupingPointer = groupingPointer.offset(1);
                step = *fresh0 as libc::c_int;
            }
            if digits > step {
                count += internalThousandSeparatorLength;
            }
            digits -= step;
        }
    }
    return count;
}
unsafe extern "C" fn TrioFollowedBySeparator(mut position: libc::c_int) -> libc::c_int {
    let mut step: libc::c_int = 0 as libc::c_int;
    let mut groupingPointer: *mut libc::c_char = internalGrouping.as_mut_ptr();
    position -= 1;
    position;
    if position == 0 as libc::c_int {
        return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    }
    while position > 0 as libc::c_int {
        if *groupingPointer as libc::c_int == 127 as libc::c_int {
            break;
        } else {
            if *groupingPointer as libc::c_int != 0 as libc::c_int {
                let fresh1 = groupingPointer;
                groupingPointer = groupingPointer.offset(1);
                step = *fresh1 as libc::c_int;
            }
            if step == 0 as libc::c_int {
                break;
            }
            position -= step;
        }
    }
    return (position == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioGetPosition(
    mut format: *const libc::c_char,
    mut indexPointer: *mut libc::c_int,
) -> libc::c_int {
    let mut tmpformat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number: libc::c_int = 0 as libc::c_int;
    let mut index: libc::c_int = *indexPointer;
    number = trio_to_long(
        &*format.offset(index as isize),
        &mut tmpformat,
        BASE_DECIMAL as libc::c_int,
    ) as libc::c_int;
    index = tmpformat.offset_from(format) as libc::c_long as libc::c_int;
    if number != 0 as libc::c_int
        && {
            let fresh2 = index;
            index = index + 1;
            '$' as i32 == *format.offset(fresh2 as isize) as libc::c_int
        }
    {
        *indexPointer = index;
        return number - 1 as libc::c_int;
    }
    return NO_POSITION as libc::c_int;
}
unsafe extern "C" fn TrioFindNamespace(
    mut name: *const libc::c_char,
    mut prev: *mut *mut trio_userdef_t,
) -> *mut trio_userdef_t {
    let mut def: *mut trio_userdef_t = 0 as *mut trio_userdef_t;
    if internalEnterCriticalRegion.is_some() {
        internalEnterCriticalRegion
            .expect("non-null function pointer")(0 as *mut libc::c_void);
    }
    def = internalUserDef;
    while !def.is_null() {
        if trio_equal_case((*def).name, name) != 0 {
            break;
        }
        if !prev.is_null() {
            *prev = def;
        }
        def = (*def).next;
    }
    if internalLeaveCriticalRegion.is_some() {
        internalLeaveCriticalRegion
            .expect("non-null function pointer")(0 as *mut libc::c_void);
    }
    return def;
}
unsafe extern "C" fn TrioPower(
    mut number: libc::c_int,
    mut exponent: libc::c_int,
) -> trio_long_double_t {
    let mut result: trio_long_double_t = 0.;
    if number == 10 as libc::c_int {
        match exponent {
            0 => {
                result = f128::f128::new(number) * f128::f128::new(1E-1);
            }
            1 => {
                result = f128::f128::new(number) * f128::f128::new(1E+0);
            }
            2 => {
                result = f128::f128::new(number) * f128::f128::new(1E+1);
            }
            3 => {
                result = f128::f128::new(number) * f128::f128::new(1E+2);
            }
            4 => {
                result = f128::f128::new(number) * f128::f128::new(1E+3);
            }
            5 => {
                result = f128::f128::new(number) * f128::f128::new(1E+4);
            }
            6 => {
                result = f128::f128::new(number) * f128::f128::new(1E+5);
            }
            7 => {
                result = f128::f128::new(number) * f128::f128::new(1E+6);
            }
            8 => {
                result = f128::f128::new(number) * f128::f128::new(1E+7);
            }
            9 => {
                result = f128::f128::new(number) * f128::f128::new(1E+8);
            }
            _ => {
                result = powl(f128::f128::new(number), f128::f128::new(exponent));
            }
        }
    } else {
        return powl(f128::f128::new(number), f128::f128::new(exponent))
    }
    return result;
}
unsafe extern "C" fn TrioLogarithm(
    mut number: libc::c_double,
    mut base: libc::c_int,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    if number <= 0.0f64 {
        result = if number == 0.0f64 { trio_ninf() } else { trio_nan() };
    } else if base == 10 as libc::c_int {
        result = log10(number);
    } else {
        result = log10(number) / log10(base as libc::c_double);
    }
    return result;
}
unsafe extern "C" fn TrioLogarithmBase(mut base: libc::c_int) -> libc::c_double {
    match base {
        2 => return 1.0f64,
        8 => return 3.0f64,
        10 => return 3.321928094887362345f64,
        16 => return 4.0f64,
        _ => return TrioLogarithm(base as libc::c_double, 2 as libc::c_int),
    };
}
unsafe extern "C" fn TrioWriteNumber(
    mut self_0: *mut trio_class_t,
    mut number: trio_uintmax_t,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
    mut precision: libc::c_int,
    mut base: libc::c_int,
) {
    let mut isNegative: libc::c_int = 0;
    let mut isNumberZero: libc::c_int = 0;
    let mut isPrecisionZero: libc::c_int = 0;
    let mut ignoreNumber: libc::c_int = 0;
    let mut buffer: [libc::c_char; 1089] = [0; 1089];
    let mut bufferend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pointer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut digits: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    digits = if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
        internalDigitsUpper.as_ptr()
    } else {
        internalDigitsLower.as_ptr()
    };
    if base == NO_BASE as libc::c_int {
        base = BASE_DECIMAL as libc::c_int;
    }
    isNumberZero = (number == 0 as libc::c_int as trio_uintmax_t) as libc::c_int;
    isPrecisionZero = (precision == 0 as libc::c_int) as libc::c_int;
    ignoreNumber = (isNumberZero != 0 && isPrecisionZero != 0
        && !(flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0
            && base == BASE_OCTAL as libc::c_int)) as libc::c_int;
    if flags & FLAGS_UNSIGNED as libc::c_int as trio_flags_t != 0 {
        isNegative = (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        flags &= !(FLAGS_SHOWSIGN as libc::c_int) as trio_flags_t;
    } else {
        isNegative = ((number as trio_intmax_t) < 0 as libc::c_int as trio_intmax_t)
            as libc::c_int;
        if isNegative != 0 {
            number = -(number as trio_intmax_t) as trio_uintmax_t;
        }
    }
    if flags & FLAGS_QUAD as libc::c_int as trio_flags_t != 0 {
        number = (number as trio_ulonglong_t & -(1 as libc::c_int) as trio_ulonglong_t)
            as trio_uintmax_t;
    } else if flags & FLAGS_LONG as libc::c_int as trio_flags_t != 0 {
        number &= -(1 as libc::c_int) as libc::c_ulong;
    } else {
        number &= -(1 as libc::c_int) as libc::c_uint as trio_uintmax_t;
    }
    bufferend = &mut *buffer
        .as_mut_ptr()
        .offset(
            (::core::mem::size_of::<[libc::c_char; 1089]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut libc::c_char;
    pointer = bufferend;
    let fresh3 = pointer;
    pointer = pointer.offset(-1);
    *fresh3 = 0 as libc::c_int as libc::c_char;
    i = 1 as libc::c_int;
    while i
        < ::core::mem::size_of::<[libc::c_char; 1089]>() as libc::c_ulong as libc::c_int
    {
        let fresh4 = pointer;
        pointer = pointer.offset(-1);
        *fresh4 = *digits.offset((number % base as trio_uintmax_t) as isize);
        number = number / base as trio_uintmax_t;
        if number == 0 as libc::c_int as trio_uintmax_t {
            break;
        }
        if flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0
            && TrioFollowedBySeparator(i + 1 as libc::c_int) != 0
        {
            length = internalThousandSeparatorLength;
            if pointer.offset_from(buffer.as_mut_ptr()) as libc::c_long as libc::c_int
                - length > 0 as libc::c_int
            {
                p = &mut *internalThousandSeparator
                    .as_mut_ptr()
                    .offset((length - 1 as libc::c_int) as isize) as *mut libc::c_char;
                loop {
                    let fresh5 = length;
                    length = length - 1;
                    if !(fresh5 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh6 = p;
                    p = p.offset(-1);
                    let fresh7 = pointer;
                    pointer = pointer.offset(-1);
                    *fresh7 = *fresh6;
                }
            }
        }
        i += 1;
        i;
    }
    if ignoreNumber == 0 {
        width = (width as libc::c_long
            - (bufferend.offset_from(pointer) as libc::c_long
                - 1 as libc::c_int as libc::c_long)) as libc::c_int;
    }
    if NO_PRECISION as libc::c_int != precision {
        precision = (precision as libc::c_long
            - (bufferend.offset_from(pointer) as libc::c_long
                - 1 as libc::c_int as libc::c_long)) as libc::c_int;
        if precision < 0 as libc::c_int {
            precision = 0 as libc::c_int;
        }
        flags |= FLAGS_NILPADDING as libc::c_int as trio_flags_t;
    }
    count = if !(flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0
        || precision == NO_PRECISION as libc::c_int)
    {
        precision
    } else {
        0 as libc::c_int
    };
    if isNegative != 0 || flags & FLAGS_SHOWSIGN as libc::c_int as trio_flags_t != 0
        || flags & FLAGS_SPACE as libc::c_int as trio_flags_t != 0
    {
        width -= 1;
        width;
    }
    if flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0 && isNumberZero == 0
    {
        match base {
            2 | 16 => {
                width -= 2 as libc::c_int;
            }
            8 => {
                if flags & FLAGS_NILPADDING as libc::c_int as trio_flags_t == 0
                    || count == 0 as libc::c_int
                {
                    width -= 1;
                    width;
                }
            }
            _ => {}
        }
    }
    if !(flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0
        || flags & FLAGS_NILPADDING as libc::c_int as trio_flags_t != 0
            && precision == NO_PRECISION as libc::c_int)
    {
        loop {
            let fresh8 = width;
            width = width - 1;
            if !(fresh8 > count) {
                break;
            }
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
        }
    }
    if isNegative != 0 {
        ((*self_0).OutStream).expect("non-null function pointer")(self_0, '-' as i32);
    } else if flags & FLAGS_SHOWSIGN as libc::c_int as trio_flags_t != 0 {
        ((*self_0).OutStream).expect("non-null function pointer")(self_0, '+' as i32);
    } else if flags & FLAGS_SPACE as libc::c_int as trio_flags_t != 0 {
        ((*self_0).OutStream).expect("non-null function pointer")(self_0, ' ' as i32);
    }
    if flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0 && isNumberZero == 0
    {
        match base {
            2 => {
                ((*self_0).OutStream)
                    .expect("non-null function pointer")(self_0, '0' as i32);
                ((*self_0).OutStream)
                    .expect(
                        "non-null function pointer",
                    )(
                    self_0,
                    if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                        'B' as i32
                    } else {
                        'b' as i32
                    },
                );
            }
            8 => {
                if flags & FLAGS_NILPADDING as libc::c_int as trio_flags_t == 0
                    || count == 0 as libc::c_int
                {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, '0' as i32);
                }
            }
            16 => {
                ((*self_0).OutStream)
                    .expect("non-null function pointer")(self_0, '0' as i32);
                ((*self_0).OutStream)
                    .expect(
                        "non-null function pointer",
                    )(
                    self_0,
                    if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                        'X' as i32
                    } else {
                        'x' as i32
                    },
                );
            }
            _ => {}
        }
    }
    if flags & FLAGS_NILPADDING as libc::c_int as trio_flags_t != 0 {
        if precision == NO_PRECISION as libc::c_int {
            precision = width;
        }
        loop {
            let fresh9 = precision;
            precision = precision - 1;
            if !(fresh9 > 0 as libc::c_int) {
                break;
            }
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '0' as i32);
            width -= 1;
            width;
        }
    }
    if ignoreNumber == 0 {
        loop {
            pointer = pointer.offset(1);
            if !(*pointer != 0) {
                break;
            }
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, *pointer as libc::c_int);
        }
    }
    if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0 {
        loop {
            let fresh10 = width;
            width = width - 1;
            if !(fresh10 > 0 as libc::c_int) {
                break;
            }
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
        }
    }
}
unsafe extern "C" fn TrioWriteStringCharacter(
    mut self_0: *mut trio_class_t,
    mut ch: libc::c_int,
    mut flags: trio_flags_t,
) {
    if flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0 {
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '\\' as i32);
            match ch {
                7 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'a' as i32);
                }
                8 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'b' as i32);
                }
                12 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'f' as i32);
                }
                10 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'n' as i32);
                }
                13 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'r' as i32);
                }
                9 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 't' as i32);
                }
                11 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'v' as i32);
                }
                92 => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, '\\' as i32);
                }
                _ => {
                    ((*self_0).OutStream)
                        .expect("non-null function pointer")(self_0, 'x' as i32);
                    TrioWriteNumber(
                        self_0,
                        ch as trio_uintmax_t,
                        (FLAGS_UNSIGNED as libc::c_int | FLAGS_NILPADDING as libc::c_int)
                            as trio_flags_t,
                        2 as libc::c_int,
                        2 as libc::c_int,
                        BASE_HEX as libc::c_int,
                    );
                }
            }
        } else if ch == '\\' as i32 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '\\' as i32);
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '\\' as i32);
        } else {
            ((*self_0).OutStream).expect("non-null function pointer")(self_0, ch);
        }
    } else {
        ((*self_0).OutStream).expect("non-null function pointer")(self_0, ch);
    };
}
unsafe extern "C" fn TrioWriteString(
    mut self_0: *mut trio_class_t,
    mut string: *const libc::c_char,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
    mut precision: libc::c_int,
) {
    let mut length: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    if string.is_null() {
        string = internalNullString.as_ptr();
        length = (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        flags &= !(FLAGS_QUOTE as libc::c_int) as trio_flags_t;
        width = 0 as libc::c_int;
    } else {
        length = trio_length(string) as libc::c_int;
    }
    if NO_PRECISION as libc::c_int != precision && precision < length {
        length = precision;
    }
    width -= length;
    if flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0 {
        ((*self_0).OutStream).expect("non-null function pointer")(self_0, '"' as i32);
    }
    if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t == 0 {
        loop {
            let fresh11 = width;
            width = width - 1;
            if !(fresh11 > 0 as libc::c_int) {
                break;
            }
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
        }
    }
    loop {
        let fresh12 = length;
        length = length - 1;
        if !(fresh12 > 0 as libc::c_int) {
            break;
        }
        let fresh13 = string;
        string = string.offset(1);
        ch = *fresh13 as libc::c_uchar as libc::c_int;
        TrioWriteStringCharacter(self_0, ch, flags);
    }
    if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0 {
        loop {
            let fresh14 = width;
            width = width - 1;
            if !(fresh14 > 0 as libc::c_int) {
                break;
            }
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
        }
    }
    if flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0 {
        ((*self_0).OutStream).expect("non-null function pointer")(self_0, '"' as i32);
    }
}
unsafe extern "C" fn TrioWriteDouble(
    mut self_0: *mut trio_class_t,
    mut number: trio_long_double_t,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
    mut precision: libc::c_int,
    mut base: libc::c_int,
) {
    let mut integerNumber: trio_long_double_t = 0.;
    let mut fractionNumber: trio_long_double_t = 0.;
    let mut workNumber: trio_long_double_t = 0.;
    let mut integerDigits: libc::c_int = 0;
    let mut fractionDigits: libc::c_int = 0;
    let mut exponentDigits: libc::c_int = 0;
    let mut baseDigits: libc::c_int = 0;
    let mut integerThreshold: libc::c_int = 0;
    let mut fractionThreshold: libc::c_int = 0;
    let mut expectedWidth: libc::c_int = 0;
    let mut exponent: libc::c_int = 0 as libc::c_int;
    let mut uExponent: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut exponentBase: libc::c_int = 0;
    let mut dblBase: trio_long_double_t = 0.;
    let mut dblIntegerBase: trio_long_double_t = 0.;
    let mut dblFractionBase: trio_long_double_t = 0.;
    let mut integerAdjust: trio_long_double_t = 0.;
    let mut fractionAdjust: trio_long_double_t = 0.;
    let mut isNegative: libc::c_int = 0;
    let mut isExponentNegative: libc::c_int = (1 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
    let mut requireTwoDigitExponent: libc::c_int = 0;
    let mut isHex: libc::c_int = 0;
    let mut digits: *const libc::c_char = 0 as *const libc::c_char;
    let mut groupingPointer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut hasOnlyZeroes: libc::c_int = 0;
    let mut zeroes: libc::c_int = 0 as libc::c_int;
    let mut trailingZeroes: libc::c_int = 0;
    let mut keepTrailingZeroes: libc::c_int = 0;
    let mut keepDecimalPoint: libc::c_int = 0;
    let mut epsilon: trio_long_double_t = 0.;
    match trio_fpclassify_and_signbit(number as libc::c_double, &mut isNegative) {
        1 => {
            TrioWriteString(
                self_0,
                if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                    b"NAN\0" as *const u8 as *const libc::c_char
                } else {
                    b"nan\0" as *const u8 as *const libc::c_char
                },
                flags,
                width,
                precision,
            );
            return;
        }
        0 => {
            if isNegative != 0 {
                TrioWriteString(
                    self_0,
                    if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                        b"-INF\0" as *const u8 as *const libc::c_char
                    } else {
                        b"-inf\0" as *const u8 as *const libc::c_char
                    },
                    flags,
                    width,
                    precision,
                );
                return;
            } else {
                TrioWriteString(
                    self_0,
                    if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                        b"INF\0" as *const u8 as *const libc::c_char
                    } else {
                        b"inf\0" as *const u8 as *const libc::c_char
                    },
                    flags,
                    width,
                    precision,
                );
                return;
            }
        }
        _ => {}
    }
    if flags & FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t != 0 {
        baseDigits = if base == 10 as libc::c_int {
            18 as libc::c_int
        } else {
            floor(64 as libc::c_int as libc::c_double / TrioLogarithmBase(base))
                as libc::c_int
        };
        epsilon = f128::f128::new(1.08420217248550443401e-19);
    } else if flags & FLAGS_SHORT as libc::c_int as trio_flags_t != 0 {
        baseDigits = if base == BASE_DECIMAL as libc::c_int {
            6 as libc::c_int
        } else {
            floor(24 as libc::c_int as libc::c_double / TrioLogarithmBase(base))
                as libc::c_int
        };
        epsilon = f128::f128::new(1.19209290e-7f32);
    } else {
        baseDigits = if base == BASE_DECIMAL as libc::c_int {
            15 as libc::c_int
        } else {
            floor(53 as libc::c_int as libc::c_double / TrioLogarithmBase(base))
                as libc::c_int
        };
        epsilon = f128::f128::new(2.2204460492503131e-16f64);
    }
    digits = if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
        internalDigitsUpper.as_ptr()
    } else {
        internalDigitsLower.as_ptr()
    };
    isHex = (base == BASE_HEX as libc::c_int) as libc::c_int;
    if base == NO_BASE as libc::c_int {
        base = BASE_DECIMAL as libc::c_int;
    }
    dblBase = f128::f128::new(base);
    keepTrailingZeroes = !(flags & FLAGS_ROUNDING as libc::c_int as trio_flags_t != 0
        || flags & FLAGS_FLOAT_G as libc::c_int as trio_flags_t != 0
            && flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t == 0)
        as libc::c_int;
    if flags & FLAGS_ROUNDING as libc::c_int as trio_flags_t != 0 {
        precision = baseDigits;
    }
    if precision == NO_PRECISION as libc::c_int {
        if isHex != 0 {
            keepTrailingZeroes = (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
            precision = 24 as libc::c_int;
        } else {
            precision = 6 as libc::c_int;
        }
    }
    if isNegative != 0 {
        number = -number;
    }
    if isHex != 0 {
        flags |= FLAGS_FLOAT_E as libc::c_int as trio_flags_t;
    }
    if flags & FLAGS_FLOAT_G as libc::c_int as trio_flags_t != 0 {
        if precision == 0 as libc::c_int {
            precision = 1 as libc::c_int;
        }
        if number < f128::f128::new(1.0E-4f64)
            || number > powl(f128::f128::new(base), f128::f128::new(precision))
        {
            flags |= FLAGS_FLOAT_E as libc::c_int as trio_flags_t;
        } else if number < f128::f128::new(1.0f64) {
            workNumber = f128::f128::new(TrioLogarithm(number as libc::c_double, base));
            workNumber = if workNumber < f128::f128::new(0.0f64) {
                -workNumber
            } else {
                workNumber
            };
            if workNumber - floorl(workNumber) < f128::f128::new(0.001f64) {
                workNumber -= f128::f128::new(1.);
                workNumber;
            }
            zeroes = (floorl(workNumber)).to_i32().unwrap();
        }
    }
    if flags & FLAGS_FLOAT_E as libc::c_int as trio_flags_t != 0 {
        workNumber = f128::f128::new(TrioLogarithm(number as libc::c_double, base));
        if trio_isinf(workNumber as libc::c_double) == -(1 as libc::c_int) {
            exponent = 0 as libc::c_int;
            if flags & FLAGS_FLOAT_G as libc::c_int as trio_flags_t != 0 {
                flags &= !(FLAGS_FLOAT_E as libc::c_int) as trio_flags_t;
            }
        } else {
            exponent = (floorl(workNumber)).to_i32().unwrap();
            number /= powl(dblBase, f128::f128::new(exponent));
            isExponentNegative = (exponent < 0 as libc::c_int) as libc::c_int;
            uExponent = (if isExponentNegative != 0 { -exponent } else { exponent })
                as libc::c_uint;
            if isHex != 0 {
                uExponent = uExponent.wrapping_mul(4 as libc::c_int as libc::c_uint);
            }
            flags &= !(FLAGS_QUOTE as libc::c_int) as trio_flags_t;
        }
    }
    integerNumber = floorl(number);
    fractionNumber = number - integerNumber;
    integerDigits = if integerNumber > epsilon {
        1 as libc::c_int
            + TrioLogarithm(integerNumber as libc::c_double, base) as libc::c_int
    } else {
        1 as libc::c_int
    };
    fractionDigits = if flags & FLAGS_FLOAT_G as libc::c_int as trio_flags_t != 0
        && zeroes == 0 as libc::c_int
    {
        precision - integerDigits
    } else {
        zeroes + precision
    };
    dblFractionBase = TrioPower(base, fractionDigits);
    workNumber = number + f128::f128::new(0.5f64) / dblFractionBase;
    if floorl(number) != floorl(workNumber) {
        if flags & FLAGS_FLOAT_E as libc::c_int as trio_flags_t != 0 {
            exponent += 1;
            exponent;
            isExponentNegative = (exponent < 0 as libc::c_int) as libc::c_int;
            uExponent = (if isExponentNegative != 0 { -exponent } else { exponent })
                as libc::c_uint;
            if isHex != 0 {
                uExponent = uExponent.wrapping_mul(4 as libc::c_int as libc::c_uint);
            }
            workNumber = (number + f128::f128::new(0.5f64) / dblFractionBase) / dblBase;
            integerNumber = floorl(workNumber);
            fractionNumber = workNumber - integerNumber;
        } else {
            integerNumber = floorl(number + f128::f128::new(0.5f64));
            fractionNumber = f128::f128::new(0.0f64);
            integerDigits = if integerNumber > epsilon {
                1 as libc::c_int
                    + TrioLogarithm(integerNumber as libc::c_double, base) as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
    }
    fractionAdjust = f128::f128::new(0.5f64);
    integerAdjust = fractionAdjust;
    if flags & FLAGS_ROUNDING as libc::c_int as trio_flags_t != 0 {
        if integerDigits > baseDigits {
            integerThreshold = baseDigits;
            fractionDigits = 0 as libc::c_int;
            dblFractionBase = f128::f128::new(1.0f64);
            fractionThreshold = 0 as libc::c_int;
            precision = 0 as libc::c_int;
            integerAdjust = TrioPower(
                base,
                integerDigits - integerThreshold - 1 as libc::c_int,
            );
            fractionAdjust = f128::f128::new(0.0f64);
        } else {
            integerThreshold = integerDigits;
            fractionThreshold = fractionDigits - integerThreshold;
            fractionAdjust = f128::f128::new(1.0f64);
        }
    } else {
        integerThreshold = 2147483647 as libc::c_int;
        fractionThreshold = 2147483647 as libc::c_int;
    }
    fractionAdjust /= dblFractionBase;
    hasOnlyZeroes = (floorl((fractionNumber + fractionAdjust) * dblFractionBase)
        < epsilon) as libc::c_int;
    keepDecimalPoint = (flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0
        || !(precision == 0 as libc::c_int
            || keepTrailingZeroes == 0 && hasOnlyZeroes != 0)) as libc::c_int;
    if flags & FLAGS_FLOAT_E as libc::c_int as trio_flags_t != 0 {
        exponentDigits = if uExponent == 0 as libc::c_int as libc::c_uint {
            1 as libc::c_int
        } else {
            ceil(
                TrioLogarithm(
                    uExponent.wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_double,
                    (if isHex != 0 { 10.0f64 } else { base as libc::c_double })
                        as libc::c_int,
                ),
            ) as libc::c_int
        };
    } else {
        exponentDigits = 0 as libc::c_int;
    }
    requireTwoDigitExponent = (base == BASE_DECIMAL as libc::c_int
        && exponentDigits == 1 as libc::c_int) as libc::c_int;
    expectedWidth = integerDigits + fractionDigits
        + (if keepDecimalPoint != 0 {
            internalDecimalPointLength
        } else {
            0 as libc::c_int
        })
        + (if flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0 {
            TrioCalcThousandSeparatorLength(integerDigits)
        } else {
            0 as libc::c_int
        });
    if isNegative != 0 || flags & FLAGS_SHOWSIGN as libc::c_int as trio_flags_t != 0
        || flags & FLAGS_SPACE as libc::c_int as trio_flags_t != 0
    {
        expectedWidth = (expectedWidth as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
    }
    if exponentDigits > 0 as libc::c_int {
        expectedWidth = (expectedWidth as libc::c_ulong)
            .wrapping_add(
                (exponentDigits as libc::c_ulong)
                    .wrapping_add(
                        (if requireTwoDigitExponent != 0 {
                            ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                        } else {
                            ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as libc::c_int as libc::c_int;
    }
    if isHex != 0 {
        expectedWidth = (expectedWidth as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
    }
    if flags & FLAGS_NILPADDING as libc::c_int as trio_flags_t != 0 {
        if isNegative != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '-' as i32);
        } else if flags & FLAGS_SHOWSIGN as libc::c_int as trio_flags_t != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '+' as i32);
        } else if flags & FLAGS_SPACE as libc::c_int as trio_flags_t != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
        }
        if isHex != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '0' as i32);
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                    'X' as i32
                } else {
                    'x' as i32
                },
            );
        }
        if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t == 0 {
            i = expectedWidth;
            while i < width {
                ((*self_0).OutStream)
                    .expect("non-null function pointer")(self_0, '0' as i32);
                i += 1;
                i;
            }
        }
    } else {
        if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t == 0 {
            i = expectedWidth;
            while i < width {
                ((*self_0).OutStream)
                    .expect("non-null function pointer")(self_0, ' ' as i32);
                i += 1;
                i;
            }
        }
        if isNegative != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '-' as i32);
        } else if flags & FLAGS_SHOWSIGN as libc::c_int as trio_flags_t != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '+' as i32);
        } else if flags & FLAGS_SPACE as libc::c_int as trio_flags_t != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
        }
        if isHex != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '0' as i32);
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                    'X' as i32
                } else {
                    'x' as i32
                },
            );
        }
    }
    dblIntegerBase = f128::f128::new(1.0f64)
        / TrioPower(base, integerDigits - 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < integerDigits {
        workNumber = floorl((integerNumber + integerAdjust) * dblIntegerBase);
        if i > integerThreshold {
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(self_0, *digits.offset(0 as libc::c_int as isize) as libc::c_int);
        } else {
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                *digits.offset((fmodl(workNumber, dblBase)).to_i32().unwrap() as isize)
                    as libc::c_int,
            );
        }
        dblIntegerBase *= dblBase;
        if flags
            & (FLAGS_FLOAT_E as libc::c_int | FLAGS_QUOTE as libc::c_int) as trio_flags_t
            == FLAGS_QUOTE as libc::c_int as trio_flags_t
            && TrioFollowedBySeparator(integerDigits - i) != 0
        {
            groupingPointer = internalThousandSeparator.as_mut_ptr();
            while *groupingPointer as libc::c_int
                != 0 as libc::c_int as libc::c_char as libc::c_int
            {
                ((*self_0).OutStream)
                    .expect(
                        "non-null function pointer",
                    )(self_0, *groupingPointer as libc::c_int);
                groupingPointer = groupingPointer.offset(1);
                groupingPointer;
            }
        }
        i += 1;
        i;
    }
    trailingZeroes = 0 as libc::c_int;
    if keepDecimalPoint != 0 {
        if internalDecimalPoint != 0 {
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(self_0, internalDecimalPoint as libc::c_int);
        } else {
            i = 0 as libc::c_int;
            while i < internalDecimalPointLength {
                ((*self_0).OutStream)
                    .expect(
                        "non-null function pointer",
                    )(self_0, internalDecimalPointString[i as usize] as libc::c_int);
                i += 1;
                i;
            }
        }
    }
    i = 0 as libc::c_int;
    while i < fractionDigits {
        if integerDigits > integerThreshold || i > fractionThreshold {
            trailingZeroes += 1;
            trailingZeroes;
        } else {
            fractionNumber *= dblBase;
            fractionAdjust *= dblBase;
            workNumber = floorl(fractionNumber + fractionAdjust);
            fractionNumber -= workNumber;
            index = (fmodl(workNumber, dblBase)).to_i32().unwrap();
            if index == 0 as libc::c_int {
                trailingZeroes += 1;
                trailingZeroes;
            } else {
                while trailingZeroes > 0 as libc::c_int {
                    ((*self_0).OutStream)
                        .expect(
                            "non-null function pointer",
                        )(
                        self_0,
                        *digits.offset(0 as libc::c_int as isize) as libc::c_int,
                    );
                    trailingZeroes -= 1;
                    trailingZeroes;
                }
                ((*self_0).OutStream)
                    .expect(
                        "non-null function pointer",
                    )(self_0, *digits.offset(index as isize) as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    if keepTrailingZeroes != 0 {
        while trailingZeroes > 0 as libc::c_int {
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(self_0, *digits.offset(0 as libc::c_int as isize) as libc::c_int);
            trailingZeroes -= 1;
            trailingZeroes;
        }
    }
    if exponentDigits > 0 as libc::c_int {
        ((*self_0).OutStream)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            if isHex != 0 {
                if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                    'P' as i32
                } else {
                    'p' as i32
                }
            } else if flags & FLAGS_UPPER as libc::c_int as trio_flags_t != 0 {
                'E' as i32
            } else {
                'e' as i32
            },
        );
        ((*self_0).OutStream)
            .expect(
                "non-null function pointer",
            )(self_0, if isExponentNegative != 0 { '-' as i32 } else { '+' as i32 });
        if requireTwoDigitExponent != 0 {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, '0' as i32);
        }
        if isHex != 0 {
            base = 10.0f64 as libc::c_int;
        }
        exponentBase = TrioPower(base, exponentDigits - 1 as libc::c_int) as libc::c_int;
        i = 0 as libc::c_int;
        while i < exponentDigits {
            ((*self_0).OutStream)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                *digits
                    .offset(
                        uExponent
                            .wrapping_div(exponentBase as libc::c_uint)
                            .wrapping_rem(base as libc::c_uint) as isize,
                    ) as libc::c_int,
            );
            exponentBase /= base;
            i += 1;
            i;
        }
    }
    if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0 {
        i = expectedWidth;
        while i < width {
            ((*self_0).OutStream)
                .expect("non-null function pointer")(self_0, ' ' as i32);
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn TrioFormatProcess(
    mut data: *mut trio_class_t,
    mut format: *const libc::c_char,
    mut parameters: *mut trio_parameter_t,
) -> libc::c_int {
    let mut charlen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut string: *const libc::c_char = 0 as *const libc::c_char;
    let mut pointer: trio_pointer_t = 0 as *mut libc::c_void;
    let mut flags: trio_flags_t = 0;
    let mut width: libc::c_int = 0;
    let mut precision: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    index = 0 as libc::c_int;
    i = 0 as libc::c_int;
    mblen(0 as *const libc::c_char, 0 as libc::c_int as size_t);
    while *format.offset(index as isize) != 0 {
        if !(*format.offset(index as isize) as libc::c_int & !(0x7f as libc::c_int)
            == 0 as libc::c_int)
        {
            charlen = mblen(
                &*format.offset(index as isize),
                16 as libc::c_int as size_t,
            );
            if charlen != -(1 as libc::c_int) {
                loop {
                    let fresh15 = charlen;
                    charlen = charlen - 1;
                    if !(fresh15 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh16 = index;
                    index = index + 1;
                    ((*data).OutStream)
                        .expect(
                            "non-null function pointer",
                        )(data, *format.offset(fresh16 as isize) as libc::c_int);
                }
                continue;
            }
        }
        if '%' as i32 == *format.offset(index as isize) as libc::c_int {
            if '%' as i32
                == *format.offset((index + 1 as libc::c_int) as isize) as libc::c_int
            {
                ((*data).OutStream)
                    .expect("non-null function pointer")(data, '%' as i32);
                index += 2 as libc::c_int;
            } else {
                while (*parameters.offset(i as isize)).type_0 == 7 as libc::c_int {
                    i += 1;
                    i;
                }
                flags = (*parameters.offset(i as isize)).flags;
                width = (*parameters.offset(i as isize)).width;
                if flags & FLAGS_WIDTH_PARAMETER as libc::c_int as trio_flags_t != 0 {
                    width = (*parameters.offset(width as isize)).data.number.as_signed
                        as libc::c_int;
                    if width < 0 as libc::c_int {
                        flags |= FLAGS_LEFTADJUST as libc::c_int as trio_flags_t;
                        flags &= !(FLAGS_NILPADDING as libc::c_int) as trio_flags_t;
                        width = -width;
                    }
                }
                if flags & FLAGS_PRECISION as libc::c_int as trio_flags_t != 0 {
                    precision = (*parameters.offset(i as isize)).precision;
                    if flags & FLAGS_PRECISION_PARAMETER as libc::c_int as trio_flags_t
                        != 0
                    {
                        precision = (*parameters.offset(precision as isize))
                            .data
                            .number
                            .as_signed as libc::c_int;
                        if precision < 0 as libc::c_int {
                            precision = NO_PRECISION as libc::c_int;
                        }
                    }
                } else {
                    precision = NO_PRECISION as libc::c_int;
                }
                base = (*parameters.offset(i as isize)).base;
                if flags & FLAGS_BASE_PARAMETER as libc::c_int as trio_flags_t != 0 {
                    base = (*parameters.offset(base as isize)).data.number.as_signed
                        as libc::c_int;
                }
                match (*parameters.offset(i as isize)).type_0 {
                    3 => {
                        if flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0 {
                            ((*data).OutStream)
                                .expect("non-null function pointer")(data, '"' as i32);
                        }
                        if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t == 0 {
                            loop {
                                width -= 1;
                                if !(width > 0 as libc::c_int) {
                                    break;
                                }
                                ((*data).OutStream)
                                    .expect("non-null function pointer")(data, ' ' as i32);
                            }
                        }
                        TrioWriteStringCharacter(
                            data,
                            (*parameters.offset(i as isize)).data.number.as_signed
                                as libc::c_int,
                            flags,
                        );
                        if flags & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0 {
                            loop {
                                width -= 1;
                                if !(width > 0 as libc::c_int) {
                                    break;
                                }
                                ((*data).OutStream)
                                    .expect("non-null function pointer")(data, ' ' as i32);
                            }
                        }
                        if flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0 {
                            ((*data).OutStream)
                                .expect("non-null function pointer")(data, '"' as i32);
                        }
                    }
                    1 => {
                        TrioWriteNumber(
                            data,
                            (*parameters.offset(i as isize)).data.number.as_unsigned,
                            flags,
                            width,
                            precision,
                            base,
                        );
                    }
                    2 => {
                        TrioWriteDouble(
                            data,
                            (*parameters.offset(i as isize)).data.longdoubleNumber,
                            flags,
                            width,
                            precision,
                            base,
                        );
                    }
                    4 => {
                        TrioWriteString(
                            data,
                            (*parameters.offset(i as isize)).data.string,
                            flags,
                            width,
                            precision,
                        );
                    }
                    5 => {
                        let mut reference: trio_reference_t = _trio_reference_t {
                            data: 0 as *mut trio_class_t,
                            parameter: 0 as *mut trio_parameter_t,
                        };
                        reference.data = data;
                        reference
                            .parameter = &mut *parameters.offset(i as isize)
                            as *mut trio_parameter_t;
                        trio_print_pointer(
                            &mut reference as *mut trio_reference_t as trio_pointer_t,
                            (*parameters.offset(i as isize)).data.pointer,
                        );
                    }
                    6 => {
                        pointer = (*parameters.offset(i as isize)).data.pointer;
                        if !pointer.is_null() {
                            if flags & FLAGS_SIZE_T as libc::c_int as trio_flags_t != 0 {
                                *(pointer as *mut size_t) = (*data).committed as size_t;
                            } else if flags
                                & FLAGS_PTRDIFF_T as libc::c_int as trio_flags_t != 0
                            {
                                *(pointer
                                    as *mut ptrdiff_t) = (*data).committed as ptrdiff_t;
                            } else if flags
                                & FLAGS_INTMAX_T as libc::c_int as trio_flags_t != 0
                            {
                                *(pointer
                                    as *mut trio_intmax_t) = (*data).committed as trio_intmax_t;
                            } else if flags & FLAGS_QUAD as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer
                                    as *mut trio_ulonglong_t) = (*data).committed
                                    as trio_ulonglong_t;
                            } else if flags & FLAGS_LONG as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer
                                    as *mut libc::c_long) = (*data).committed as libc::c_long;
                            } else if flags & FLAGS_SHORT as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer
                                    as *mut libc::c_short) = (*data).committed as libc::c_short;
                            } else {
                                *(pointer as *mut libc::c_int) = (*data).committed;
                            }
                        }
                    }
                    9 => {
                        string = trio_error(
                            (*parameters.offset(i as isize)).data.errorNumber,
                        );
                        if !string.is_null() {
                            TrioWriteString(data, string, flags, width, precision);
                        } else {
                            ((*data).OutStream)
                                .expect("non-null function pointer")(data, '#' as i32);
                            TrioWriteNumber(
                                data,
                                (*parameters.offset(i as isize)).data.errorNumber
                                    as trio_uintmax_t,
                                flags,
                                width,
                                precision,
                                BASE_DECIMAL as libc::c_int,
                            );
                        }
                    }
                    10 => {
                        let mut reference_0: trio_reference_t = _trio_reference_t {
                            data: 0 as *mut trio_class_t,
                            parameter: 0 as *mut trio_parameter_t,
                        };
                        let mut def: *mut trio_userdef_t = 0 as *mut trio_userdef_t;
                        if (*parameters.offset(i as isize))
                            .user_name[0 as libc::c_int as usize] as libc::c_int
                            == 0 as libc::c_int as libc::c_char as libc::c_int
                        {
                            if i > 0 as libc::c_int
                                || (*parameters.offset((i - 1 as libc::c_int) as isize))
                                    .type_0 == 7 as libc::c_int
                            {
                                def = (*parameters.offset((i - 1 as libc::c_int) as isize))
                                    .data
                                    .pointer as *mut trio_userdef_t;
                            }
                        } else {
                            def = TrioFindNamespace(
                                ((*parameters.offset(i as isize)).user_name).as_mut_ptr(),
                                0 as *mut *mut trio_userdef_t,
                            );
                        }
                        if !def.is_null() {
                            reference_0.data = data;
                            reference_0
                                .parameter = &mut *parameters.offset(i as isize)
                                as *mut trio_parameter_t;
                            ((*def).callback)
                                .expect(
                                    "non-null function pointer",
                                )(
                                &mut reference_0 as *mut trio_reference_t as trio_pointer_t,
                            );
                        }
                    }
                    7 | _ => {}
                }
                index = (*parameters.offset(i as isize)).indexAfterSpecifier;
                i += 1;
                i;
            }
        } else {
            let fresh17 = index;
            index = index + 1;
            ((*data).OutStream)
                .expect(
                    "non-null function pointer",
                )(data, *format.offset(fresh17 as isize) as libc::c_int);
        }
    }
    return (*data).processed;
}
unsafe extern "C" fn TrioFormatRef(
    mut reference: *mut trio_reference_t,
    mut format: *const libc::c_char,
    mut arglist: *mut ::core::ffi::VaList,
    mut argarray: *mut trio_pointer_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut parameters: [trio_parameter_t; 64] = [trio_parameter_t {
        type_0: 0,
        flags: 0,
        width: 0,
        precision: 0,
        base: 0,
        varsize: 0,
        indexAfterSpecifier: 0,
        data: C2RustUnnamed_0 {
            string: 0 as *mut libc::c_char,
        },
        user_name: [0; 64],
        user_data: [0; 256],
    }; 64];
    status = TrioParse(
        TYPE_PRINT as libc::c_int,
        format,
        parameters.as_mut_ptr(),
        arglist,
        argarray,
    );
    if status < 0 as libc::c_int {
        return status;
    }
    status = TrioFormatProcess((*reference).data, format, parameters.as_mut_ptr());
    if (*(*reference).data).error != 0 as libc::c_int {
        status = (*(*reference).data).error;
    }
    return status;
}
unsafe extern "C" fn TrioFormat(
    mut destination: trio_pointer_t,
    mut destinationSize: size_t,
    mut OutStream: Option::<unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> ()>,
    mut format: *const libc::c_char,
    mut arglist: *mut ::core::ffi::VaList,
    mut argarray: *mut trio_pointer_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut data: trio_class_t = _trio_class_t {
        OutStream: None,
        InStream: None,
        location: 0 as *mut libc::c_void,
        current: 0,
        processed: 0,
        committed: 0,
        max: 0,
        error: 0,
    };
    let mut parameters: [trio_parameter_t; 64] = [trio_parameter_t {
        type_0: 0,
        flags: 0,
        width: 0,
        precision: 0,
        base: 0,
        varsize: 0,
        indexAfterSpecifier: 0,
        data: C2RustUnnamed_0 {
            string: 0 as *mut libc::c_char,
        },
        user_name: [0; 64],
        user_data: [0; 256],
    }; 64];
    memset(
        &mut data as *mut trio_class_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<trio_class_t>() as libc::c_ulong,
    );
    data.OutStream = OutStream;
    data.location = destination;
    data.max = destinationSize as libc::c_int;
    data.error = 0 as libc::c_int;
    if internalLocaleValues.is_null() {
        TrioSetLocale();
    }
    status = TrioParse(
        TYPE_PRINT as libc::c_int,
        format,
        parameters.as_mut_ptr(),
        arglist,
        argarray,
    );
    if status < 0 as libc::c_int {
        return status;
    }
    status = TrioFormatProcess(&mut data, format, parameters.as_mut_ptr());
    if data.error != 0 as libc::c_int {
        status = data.error;
    }
    return status;
}
unsafe extern "C" fn TrioOutStreamFile(
    mut self_0: *mut trio_class_t,
    mut output: libc::c_int,
) {
    let mut file: *mut FILE = 0 as *mut FILE;
    file = (*self_0).location as *mut FILE;
    (*self_0).processed += 1;
    (*self_0).processed;
    if fputc(output, file) == -(1 as libc::c_int) {
        (*self_0)
            .error = -(TRIO_EOF as libc::c_int
            + ((0 as libc::c_int) << 8 as libc::c_int));
    } else {
        (*self_0).committed += 1;
        (*self_0).committed;
    };
}
unsafe extern "C" fn TrioOutStreamFileDescriptor(
    mut self_0: *mut trio_class_t,
    mut output: libc::c_int,
) {
    let mut fd: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    fd = *((*self_0).location as *mut libc::c_int);
    ch = output as libc::c_char;
    (*self_0).processed += 1;
    (*self_0).processed;
    if write(
        fd,
        &mut ch as *mut libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) == -(1 as libc::c_int) as ssize_t
    {
        (*self_0)
            .error = -(TRIO_ERRNO as libc::c_int
            + ((0 as libc::c_int) << 8 as libc::c_int));
    } else {
        (*self_0).committed += 1;
        (*self_0).committed;
    };
}
unsafe extern "C" fn TrioOutStreamCustom(
    mut self_0: *mut trio_class_t,
    mut output: libc::c_int,
) {
    let mut status: libc::c_int = 0;
    let mut data: *mut trio_custom_t = 0 as *mut trio_custom_t;
    data = (*self_0).location as *mut trio_custom_t;
    if ((*data).stream.out).is_some() {
        status = ((*data).stream.out)
            .expect("non-null function pointer")((*data).closure, output);
        if status >= 0 as libc::c_int {
            (*self_0).committed += 1;
            (*self_0).committed;
        } else if (*self_0).error == 0 as libc::c_int {
            (*self_0)
                .error = -(TRIO_ECUSTOM as libc::c_int + (-status << 8 as libc::c_int));
        }
    }
    (*self_0).processed += 1;
    (*self_0).processed;
}
unsafe extern "C" fn TrioOutStreamString(
    mut self_0: *mut trio_class_t,
    mut output: libc::c_int,
) {
    let mut buffer: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    buffer = (*self_0).location as *mut *mut libc::c_char;
    **buffer = output as libc::c_char;
    *buffer = (*buffer).offset(1);
    *buffer;
    (*self_0).processed += 1;
    (*self_0).processed;
    (*self_0).committed += 1;
    (*self_0).committed;
}
unsafe extern "C" fn TrioOutStreamStringMax(
    mut self_0: *mut trio_class_t,
    mut output: libc::c_int,
) {
    let mut buffer: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    buffer = (*self_0).location as *mut *mut libc::c_char;
    if (*self_0).processed < (*self_0).max {
        **buffer = output as libc::c_char;
        *buffer = (*buffer).offset(1);
        *buffer;
        (*self_0).committed += 1;
        (*self_0).committed;
    }
    (*self_0).processed += 1;
    (*self_0).processed;
}
unsafe extern "C" fn TrioOutStreamStringDynamic(
    mut self_0: *mut trio_class_t,
    mut output: libc::c_int,
) {
    if (*self_0).error == 0 as libc::c_int {
        trio_xstring_append_char(
            (*self_0).location as *mut trio_string_t,
            output as libc::c_char,
        );
        (*self_0).committed += 1;
        (*self_0).committed;
    }
    (*self_0).processed += 1;
    (*self_0).processed;
}
#[no_mangle]
pub unsafe extern "C" fn trio_printf(
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioFormat(
        stdout as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vprintf(
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioFormat(
        stdout as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_printfv(
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioFormat(
        stdout as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_fprintf(
    mut file: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioFormat(
        file as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vfprintf(
    mut file: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioFormat(
        file as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_fprintfv(
    mut file: *mut FILE,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioFormat(
        file as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_dprintf(
    mut fd: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioFormat(
        &mut fd as *mut libc::c_int as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFileDescriptor
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vdprintf(
    mut fd: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioFormat(
        &mut fd as *mut libc::c_int as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFileDescriptor
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_dprintfv(
    mut fd: libc::c_int,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioFormat(
        &mut fd as *mut libc::c_int as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamFileDescriptor
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_cprintf(
    mut stream: trio_outstream_t,
    mut closure: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut data: trio_custom_t = trio_custom_t {
        stream: C2RustUnnamed_2 { out: None },
        closure: 0 as *mut libc::c_void,
    };
    args_0 = args.clone();
    data.stream.out = stream;
    data.closure = closure;
    status = TrioFormat(
        &mut data as *mut trio_custom_t as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamCustom
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vcprintf(
    mut stream: trio_outstream_t,
    mut closure: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut data: trio_custom_t = trio_custom_t {
        stream: C2RustUnnamed_2 { out: None },
        closure: 0 as *mut libc::c_void,
    };
    data.stream.out = stream;
    data.closure = closure;
    return TrioFormat(
        &mut data as *mut trio_custom_t as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamCustom
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_cprintfv(
    mut stream: trio_outstream_t,
    mut closure: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut data: trio_custom_t = trio_custom_t {
        stream: C2RustUnnamed_2 { out: None },
        closure: 0 as *mut libc::c_void,
    };
    data.stream.out = stream;
    data.closure = closure;
    return TrioFormat(
        &mut data as *mut trio_custom_t as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamCustom
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_sprintf(
    mut buffer: *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamString
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    *buffer = 0 as libc::c_int as libc::c_char;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vsprintf(
    mut buffer: *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamString
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
    *buffer = 0 as libc::c_int as libc::c_char;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_sprintfv(
    mut buffer: *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioOutStreamString
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
    *buffer = 0 as libc::c_int as libc::c_char;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_snprintf(
    mut buffer: *mut libc::c_char,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        if max > 0 as libc::c_int as size_t {
            max.wrapping_sub(1 as libc::c_int as size_t)
        } else {
            0 as libc::c_int as size_t
        },
        Some(
            TrioOutStreamStringMax
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    if max > 0 as libc::c_int as size_t {
        *buffer = 0 as libc::c_int as libc::c_char;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vsnprintf(
    mut buffer: *mut libc::c_char,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        if max > 0 as libc::c_int as size_t {
            max.wrapping_sub(1 as libc::c_int as size_t)
        } else {
            0 as libc::c_int as size_t
        },
        Some(
            TrioOutStreamStringMax
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
    if max > 0 as libc::c_int as size_t {
        *buffer = 0 as libc::c_int as libc::c_char;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_snprintfv(
    mut buffer: *mut libc::c_char,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        if max > 0 as libc::c_int as size_t {
            max.wrapping_sub(1 as libc::c_int as size_t)
        } else {
            0 as libc::c_int as size_t
        },
        Some(
            TrioOutStreamStringMax
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
    if max > 0 as libc::c_int as size_t {
        *buffer = 0 as libc::c_int as libc::c_char;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_snprintfcat(
    mut buffer: *mut libc::c_char,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut buf_len: size_t = 0;
    args_0 = args.clone();
    buf_len = trio_length(buffer);
    buffer = &mut *buffer.offset(buf_len as isize) as *mut libc::c_char;
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        max.wrapping_sub(1 as libc::c_int as size_t).wrapping_sub(buf_len),
        Some(
            TrioOutStreamStringMax
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    *buffer = 0 as libc::c_int as libc::c_char;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vsnprintfcat(
    mut buffer: *mut libc::c_char,
    mut max: size_t,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut buf_len: size_t = 0;
    buf_len = trio_length(buffer);
    buffer = &mut *buffer.offset(buf_len as isize) as *mut libc::c_char;
    status = TrioFormat(
        &mut buffer as *mut *mut libc::c_char as trio_pointer_t,
        max.wrapping_sub(1 as libc::c_int as size_t).wrapping_sub(buf_len),
        Some(
            TrioOutStreamStringMax
                as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
    *buffer = 0 as libc::c_int as libc::c_char;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_aprintf(
    mut format: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut info: *mut trio_string_t = 0 as *mut trio_string_t;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    info = trio_xstring_duplicate(b"\0" as *const u8 as *const libc::c_char);
    if !info.is_null() {
        args_0 = args.clone();
        TrioFormat(
            info as trio_pointer_t,
            0 as libc::c_int as size_t,
            Some(
                TrioOutStreamStringDynamic
                    as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
            ),
            format,
            &mut args_0.as_va_list(),
            0 as *mut trio_pointer_t,
        );
        trio_string_terminate(info);
        result = trio_string_extract(info);
        trio_string_destroy(info);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vaprintf(
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut info: *mut trio_string_t = 0 as *mut trio_string_t;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    info = trio_xstring_duplicate(b"\0" as *const u8 as *const libc::c_char);
    if !info.is_null() {
        TrioFormat(
            info as trio_pointer_t,
            0 as libc::c_int as size_t,
            Some(
                TrioOutStreamStringDynamic
                    as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
            ),
            format,
            &mut args.as_va_list() as *mut ::core::ffi::VaList
                as *mut ::core::ffi::VaList,
            0 as *mut trio_pointer_t,
        );
        trio_string_terminate(info);
        result = trio_string_extract(info);
        trio_string_destroy(info);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_asprintf(
    mut result: *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut status: libc::c_int = 0;
    let mut info: *mut trio_string_t = 0 as *mut trio_string_t;
    *result = 0 as *mut libc::c_char;
    info = trio_xstring_duplicate(b"\0" as *const u8 as *const libc::c_char);
    if info.is_null() {
        status = -(TRIO_ENOMEM as libc::c_int
            + ((0 as libc::c_int) << 8 as libc::c_int));
    } else {
        args_0 = args.clone();
        status = TrioFormat(
            info as trio_pointer_t,
            0 as libc::c_int as size_t,
            Some(
                TrioOutStreamStringDynamic
                    as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
            ),
            format,
            &mut args_0.as_va_list(),
            0 as *mut trio_pointer_t,
        );
        if status >= 0 as libc::c_int {
            trio_string_terminate(info);
            *result = trio_string_extract(info);
        }
        trio_string_destroy(info);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vasprintf(
    mut result: *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut info: *mut trio_string_t = 0 as *mut trio_string_t;
    *result = 0 as *mut libc::c_char;
    info = trio_xstring_duplicate(b"\0" as *const u8 as *const libc::c_char);
    if info.is_null() {
        status = -(TRIO_ENOMEM as libc::c_int
            + ((0 as libc::c_int) << 8 as libc::c_int));
    } else {
        status = TrioFormat(
            info as trio_pointer_t,
            0 as libc::c_int as size_t,
            Some(
                TrioOutStreamStringDynamic
                    as unsafe extern "C" fn(*mut trio_class_t, libc::c_int) -> (),
            ),
            format,
            &mut args.as_va_list() as *mut ::core::ffi::VaList
                as *mut ::core::ffi::VaList,
            0 as *mut trio_pointer_t,
        );
        if status >= 0 as libc::c_int {
            trio_string_terminate(info);
            *result = trio_string_extract(info);
        }
        trio_string_destroy(info);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_register(
    mut callback: trio_callback_t,
    mut name: *const libc::c_char,
) -> trio_pointer_t {
    let mut def: *mut trio_userdef_t = 0 as *mut trio_userdef_t;
    let mut prev: *mut trio_userdef_t = 0 as *mut trio_userdef_t;
    if callback.is_none() {
        return 0 as *mut libc::c_void;
    }
    if !name.is_null() {
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if trio_equal(name, b":enter\0" as *const u8 as *const libc::c_char) != 0 {
                ::core::ptr::write_volatile(
                    &mut internalEnterCriticalRegion as *mut trio_callback_t,
                    callback,
                );
            } else if trio_equal(name, b":leave\0" as *const u8 as *const libc::c_char)
                != 0
            {
                ::core::ptr::write_volatile(
                    &mut internalLeaveCriticalRegion as *mut trio_callback_t,
                    callback,
                );
            }
            return 0 as *mut libc::c_void;
        }
        if trio_length(name) >= MAX_USER_NAME as libc::c_int as size_t {
            return 0 as *mut libc::c_void;
        }
        def = TrioFindNamespace(name, &mut prev);
        if !def.is_null() {
            return 0 as *mut libc::c_void;
        }
    }
    def = malloc(::core::mem::size_of::<trio_userdef_t>() as libc::c_ulong)
        as *mut trio_userdef_t;
    if !def.is_null() {
        if internalEnterCriticalRegion.is_some() {
            internalEnterCriticalRegion
                .expect("non-null function pointer")(0 as *mut libc::c_void);
        }
        if !name.is_null() {
            if prev.is_null() {
                internalUserDef = def;
            } else {
                (*prev).next = def;
            }
        }
        (*def).callback = callback;
        (*def)
            .name = if name.is_null() {
            0 as *mut libc::c_char
        } else {
            trio_duplicate(name)
        };
        (*def).next = 0 as *mut _trio_userdef_t;
        if internalLeaveCriticalRegion.is_some() {
            internalLeaveCriticalRegion
                .expect("non-null function pointer")(0 as *mut libc::c_void);
        }
    }
    return def as trio_pointer_t;
}
#[no_mangle]
pub unsafe extern "C" fn trio_unregister(mut handle: trio_pointer_t) {
    let mut self_0: *mut trio_userdef_t = handle as *mut trio_userdef_t;
    let mut def: *mut trio_userdef_t = 0 as *mut trio_userdef_t;
    let mut prev: *mut trio_userdef_t = 0 as *mut trio_userdef_t;
    if !((*self_0).name).is_null() {
        def = TrioFindNamespace((*self_0).name, &mut prev);
        if !def.is_null() {
            if internalEnterCriticalRegion.is_some() {
                internalEnterCriticalRegion
                    .expect("non-null function pointer")(0 as *mut libc::c_void);
            }
            if prev.is_null() {
                internalUserDef = 0 as *mut trio_userdef_t;
            } else {
                (*prev).next = (*def).next;
            }
            if internalLeaveCriticalRegion.is_some() {
                internalLeaveCriticalRegion
                    .expect("non-null function pointer")(0 as *mut libc::c_void);
            }
        }
        trio_destroy((*self_0).name);
    }
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_format(
    mut ref_0: trio_pointer_t,
) -> *const libc::c_char {
    return ((*(*(ref_0 as *mut trio_reference_t)).parameter).user_data).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_argument(mut ref_0: trio_pointer_t) -> trio_pointer_t {
    return (*(*(ref_0 as *mut trio_reference_t)).parameter).data.pointer;
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_width(mut ref_0: trio_pointer_t) -> libc::c_int {
    return (*(*(ref_0 as *mut trio_reference_t)).parameter).width;
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_width(
    mut ref_0: trio_pointer_t,
    mut width: libc::c_int,
) {
    (*(*(ref_0 as *mut trio_reference_t)).parameter).width = width;
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_precision(mut ref_0: trio_pointer_t) -> libc::c_int {
    return (*(*(ref_0 as *mut trio_reference_t)).parameter).precision;
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_precision(
    mut ref_0: trio_pointer_t,
    mut precision: libc::c_int,
) {
    (*(*(ref_0 as *mut trio_reference_t)).parameter).precision = precision;
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_base(mut ref_0: trio_pointer_t) -> libc::c_int {
    return (*(*(ref_0 as *mut trio_reference_t)).parameter).base;
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_base(
    mut ref_0: trio_pointer_t,
    mut base: libc::c_int,
) {
    (*(*(ref_0 as *mut trio_reference_t)).parameter).base = base;
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_long(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_LONG as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_long(
    mut ref_0: trio_pointer_t,
    mut is_long: libc::c_int,
) {
    if is_long != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_LONG as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_LONG as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_longlong(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_QUAD as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_longlong(
    mut ref_0: trio_pointer_t,
    mut is_longlong: libc::c_int,
) {
    if is_longlong != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_QUAD as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_QUAD as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_longdouble(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_longdouble(
    mut ref_0: trio_pointer_t,
    mut is_longdouble: libc::c_int,
) {
    if is_longdouble != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_LONGDOUBLE as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_short(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_SHORT as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_short(
    mut ref_0: trio_pointer_t,
    mut is_short: libc::c_int,
) {
    if is_short != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_SHORT as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_SHORT as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_shortshort(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_SHORTSHORT as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_shortshort(
    mut ref_0: trio_pointer_t,
    mut is_shortshort: libc::c_int,
) {
    if is_shortshort != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_SHORTSHORT as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_SHORTSHORT as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_alternative(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_alternative(
    mut ref_0: trio_pointer_t,
    mut is_alternative: libc::c_int,
) {
    if is_alternative != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_ALTERNATIVE as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_alignment(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_LEFTADJUST as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_alignment(
    mut ref_0: trio_pointer_t,
    mut is_leftaligned: libc::c_int,
) {
    if is_leftaligned != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_LEFTADJUST as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_LEFTADJUST as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_spacing(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_SPACE as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_spacing(
    mut ref_0: trio_pointer_t,
    mut is_space: libc::c_int,
) {
    if is_space != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_SPACE as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_SPACE as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_sign(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_SHOWSIGN as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_sign(
    mut ref_0: trio_pointer_t,
    mut is_sign: libc::c_int,
) {
    if is_sign != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_SHOWSIGN as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_SHOWSIGN as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_padding(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_NILPADDING as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_padding(
    mut ref_0: trio_pointer_t,
    mut is_padding: libc::c_int,
) {
    if is_padding != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_NILPADDING as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_NILPADDING as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_quote(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_quote(
    mut ref_0: trio_pointer_t,
    mut is_quote: libc::c_int,
) {
    if is_quote != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_QUOTE as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_QUOTE as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_upper(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_UPPER as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_upper(
    mut ref_0: trio_pointer_t,
    mut is_upper: libc::c_int,
) {
    if is_upper != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_UPPER as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_UPPER as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_largest(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_INTMAX_T as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_largest(
    mut ref_0: trio_pointer_t,
    mut is_largest: libc::c_int,
) {
    if is_largest != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_INTMAX_T as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_INTMAX_T as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_ptrdiff(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_PTRDIFF_T as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_ptrdiff(
    mut ref_0: trio_pointer_t,
    mut is_ptrdiff: libc::c_int,
) {
    if is_ptrdiff != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_PTRDIFF_T as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_PTRDIFF_T as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_get_size(mut ref_0: trio_pointer_t) -> libc::c_int {
    return if (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
        & FLAGS_SIZE_T as libc::c_int as trio_flags_t != 0
    {
        !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_set_size(
    mut ref_0: trio_pointer_t,
    mut is_size: libc::c_int,
) {
    if is_size != 0 {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            |= FLAGS_SIZE_T as libc::c_int as trio_flags_t;
    } else {
        (*(*(ref_0 as *mut trio_reference_t)).parameter).flags
            &= !(FLAGS_SIZE_T as libc::c_int) as trio_flags_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_print_int(
    mut ref_0: trio_pointer_t,
    mut number: libc::c_int,
) {
    let mut self_0: *mut trio_reference_t = ref_0 as *mut trio_reference_t;
    TrioWriteNumber(
        (*self_0).data,
        number as trio_uintmax_t,
        (*(*self_0).parameter).flags,
        (*(*self_0).parameter).width,
        (*(*self_0).parameter).precision,
        (*(*self_0).parameter).base,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_print_uint(
    mut ref_0: trio_pointer_t,
    mut number: libc::c_uint,
) {
    let mut self_0: *mut trio_reference_t = ref_0 as *mut trio_reference_t;
    TrioWriteNumber(
        (*self_0).data,
        number as trio_uintmax_t,
        (*(*self_0).parameter).flags | FLAGS_UNSIGNED as libc::c_int as trio_flags_t,
        (*(*self_0).parameter).width,
        (*(*self_0).parameter).precision,
        (*(*self_0).parameter).base,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_print_double(
    mut ref_0: trio_pointer_t,
    mut number: libc::c_double,
) {
    let mut self_0: *mut trio_reference_t = ref_0 as *mut trio_reference_t;
    TrioWriteDouble(
        (*self_0).data,
        f128::f128::new(number),
        (*(*self_0).parameter).flags,
        (*(*self_0).parameter).width,
        (*(*self_0).parameter).precision,
        (*(*self_0).parameter).base,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_print_string(
    mut ref_0: trio_pointer_t,
    mut string: *mut libc::c_char,
) {
    let mut self_0: *mut trio_reference_t = ref_0 as *mut trio_reference_t;
    TrioWriteString(
        (*self_0).data,
        string,
        (*(*self_0).parameter).flags,
        (*(*self_0).parameter).width,
        (*(*self_0).parameter).precision,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_print_ref(
    mut ref_0: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut arglist: ::core::ffi::VaListImpl;
    arglist = args.clone();
    status = TrioFormatRef(
        ref_0 as *mut trio_reference_t,
        format,
        &mut arglist.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vprint_ref(
    mut ref_0: trio_pointer_t,
    mut format: *const libc::c_char,
    mut arglist: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioFormatRef(
        ref_0 as *mut trio_reference_t,
        format,
        &mut arglist.as_va_list() as *mut ::core::ffi::VaList
            as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_printv_ref(
    mut ref_0: trio_pointer_t,
    mut format: *const libc::c_char,
    mut argarray: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioFormatRef(
        ref_0 as *mut trio_reference_t,
        format,
        0 as *mut ::core::ffi::VaList,
        argarray,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_print_pointer(
    mut ref_0: trio_pointer_t,
    mut pointer: trio_pointer_t,
) {
    let mut self_0: *mut trio_reference_t = ref_0 as *mut trio_reference_t;
    let mut flags: trio_flags_t = 0;
    let mut number: trio_uintmax_t = 0;
    if pointer.is_null() {
        let mut string: *const libc::c_char = internalNullString.as_ptr();
        while *string != 0 {
            let fresh18 = string;
            string = string.offset(1);
            ((*(*self_0).data).OutStream)
                .expect(
                    "non-null function pointer",
                )((*self_0).data, *fresh18 as libc::c_int);
        }
    } else {
        number = (pointer as *mut libc::c_char).offset_from(0 as *mut libc::c_char)
            as libc::c_long as trio_uintmax_t;
        number &= -(1 as libc::c_int) as trio_uintmax_t;
        flags = (*(*self_0).parameter).flags;
        flags
            |= (FLAGS_UNSIGNED as libc::c_int | FLAGS_ALTERNATIVE as libc::c_int
                | FLAGS_NILPADDING as libc::c_int) as trio_flags_t;
        TrioWriteNumber(
            (*self_0).data,
            number,
            flags,
            (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<trio_pointer_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(4 as libc::c_int as libc::c_ulong),
                ) as libc::c_int,
            NO_PRECISION as libc::c_int,
            BASE_HEX as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_locale_set_decimal_point(
    mut decimalPoint: *mut libc::c_char,
) {
    if internalLocaleValues.is_null() {
        TrioSetLocale();
    }
    internalDecimalPointLength = trio_length(decimalPoint) as libc::c_int;
    if internalDecimalPointLength == 1 as libc::c_int {
        internalDecimalPoint = *decimalPoint;
    } else {
        internalDecimalPoint = 0 as libc::c_int as libc::c_char;
        trio_copy_max(
            internalDecimalPointString.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
            decimalPoint,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_locale_set_thousand_separator(
    mut thousandSeparator: *mut libc::c_char,
) {
    if internalLocaleValues.is_null() {
        TrioSetLocale();
    }
    trio_copy_max(
        internalThousandSeparator.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
        thousandSeparator,
    );
    internalThousandSeparatorLength = trio_length(internalThousandSeparator.as_mut_ptr())
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trio_locale_set_grouping(mut grouping: *mut libc::c_char) {
    if internalLocaleValues.is_null() {
        TrioSetLocale();
    }
    trio_copy_max(
        internalGrouping.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        grouping,
    );
}
unsafe extern "C" fn TrioSkipWhitespaces(mut self_0: *mut trio_class_t) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    ch = (*self_0).current;
    while *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
    }
    return ch;
}
unsafe extern "C" fn TrioGetCollation() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut first: [libc::c_char; 2] = [0; 2];
    let mut second: [libc::c_char; 2] = [0; 2];
    first[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    second[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < MAX_CHARACTER_CLASS as libc::c_int {
        k = 0 as libc::c_int;
        first[0 as libc::c_int as usize] = i as libc::c_char;
        j = 0 as libc::c_int;
        while j < MAX_CHARACTER_CLASS as libc::c_int {
            second[0 as libc::c_int as usize] = j as libc::c_char;
            if trio_equal_locale(first.as_mut_ptr(), second.as_mut_ptr()) != 0 {
                let fresh19 = k;
                k = k + 1;
                internalCollationArray[i as usize][fresh19 as usize] = j as libc::c_char;
            }
            j += 1;
            j;
        }
        internalCollationArray[i
            as usize][k as usize] = 0 as libc::c_int as libc::c_char;
        i += 1;
        i;
    }
}
unsafe extern "C" fn TrioGetCharacterClass(
    mut format: *const libc::c_char,
    mut indexPointer: *mut libc::c_int,
    mut flagsPointer: *mut trio_flags_t,
    mut characterclass: *mut libc::c_int,
) -> libc::c_int {
    let mut index: libc::c_int = *indexPointer;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    let mut range_begin: libc::c_char = 0;
    let mut range_end: libc::c_char = 0;
    *flagsPointer &= !(FLAGS_EXCLUDE as libc::c_int) as trio_flags_t;
    if *format.offset(index as isize) as libc::c_int == '^' as i32 {
        *flagsPointer |= FLAGS_EXCLUDE as libc::c_int as trio_flags_t;
        index += 1;
        index;
    }
    if *format.offset(index as isize) as libc::c_int == ']' as i32 {
        let ref mut fresh20 = *characterclass.offset(']' as i32 as isize);
        *fresh20 += 1;
        *fresh20;
        index += 1;
        index;
    }
    if *format.offset(index as isize) as libc::c_int == '-' as i32 {
        let ref mut fresh21 = *characterclass.offset('-' as i32 as isize);
        *fresh21 += 1;
        *fresh21;
        index += 1;
        index;
    }
    ch = *format.offset(index as isize);
    while ch as libc::c_int != ']' as i32
        && ch as libc::c_int != 0 as libc::c_int as libc::c_char as libc::c_int
    {
        match ch as libc::c_int {
            45 => {
                range_begin = *format.offset((index - 1 as libc::c_int) as isize);
                index += 1;
                range_end = *format.offset(index as isize);
                if range_end as libc::c_int == ']' as i32 {
                    let ref mut fresh22 = *characterclass
                        .offset(ch as libc::c_int as isize);
                    *fresh22 += 1;
                    *fresh22;
                    ch = range_end;
                } else {
                    if range_end as libc::c_int
                        == 0 as libc::c_int as libc::c_char as libc::c_int
                    {
                        return -(TRIO_EINVAL as libc::c_int
                            + (index << 8 as libc::c_int));
                    }
                    if range_begin as libc::c_int > range_end as libc::c_int {
                        return -(TRIO_ERANGE as libc::c_int
                            + (index << 8 as libc::c_int));
                    }
                    i = range_begin as libc::c_int;
                    while i <= range_end as libc::c_int {
                        let ref mut fresh23 = *characterclass.offset(i as isize);
                        *fresh23 += 1;
                        *fresh23;
                        i += 1;
                        i;
                    }
                    ch = range_end;
                }
            }
            91 => {
                match *format.offset((index + 1 as libc::c_int) as isize) as libc::c_int
                {
                    46 => {
                        i = index + 2 as libc::c_int;
                        loop {
                            if *format.offset(i as isize) as libc::c_int
                                == 0 as libc::c_int as libc::c_char as libc::c_int
                            {
                                return -(1 as libc::c_int)
                            } else {
                                if *format.offset(i as isize) as libc::c_int == '.' as i32 {
                                    break;
                                }
                                i += 1;
                                i;
                            }
                        }
                        i += 1;
                        if *format.offset(i as isize) as libc::c_int != ']' as i32 {
                            return -(1 as libc::c_int);
                        }
                        index = i;
                    }
                    61 => {
                        let mut j: libc::c_uint = 0;
                        let mut k: libc::c_uint = 0;
                        if internalCollationUnconverted != 0 {
                            TrioGetCollation();
                            internalCollationUnconverted = (1 as libc::c_int
                                == 0 as libc::c_int) as libc::c_int;
                        }
                        i = index + 2 as libc::c_int;
                        loop {
                            if *format.offset(i as isize) as libc::c_int
                                == 0 as libc::c_int as libc::c_char as libc::c_int
                            {
                                return -(1 as libc::c_int)
                            } else {
                                if *format.offset(i as isize) as libc::c_int == '=' as i32 {
                                    break;
                                }
                                k = *format.offset(i as isize) as libc::c_uint;
                                j = 0 as libc::c_int as libc::c_uint;
                                while internalCollationArray[k as usize][j as usize]
                                    as libc::c_int
                                    != 0 as libc::c_int as libc::c_char as libc::c_int
                                {
                                    let ref mut fresh24 = *characterclass
                                        .offset(
                                            internalCollationArray[k as usize][j as usize]
                                                as libc::c_int as isize,
                                        );
                                    *fresh24 += 1;
                                    *fresh24;
                                    j = j.wrapping_add(1);
                                    j;
                                }
                                i += 1;
                                i;
                            }
                        }
                        i += 1;
                        if *format.offset(i as isize) as libc::c_int != ']' as i32 {
                            return -(1 as libc::c_int);
                        }
                        index = i;
                    }
                    58 => {
                        if trio_equal_max(
                            b"[:alnum:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh25 = *characterclass.offset(i as isize);
                                    *fresh25 += 1;
                                    *fresh25;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:alpha:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh26 = *characterclass.offset(i as isize);
                                    *fresh26 += 1;
                                    *fresh26;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:cntrl:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh27 = *characterclass.offset(i as isize);
                                    *fresh27 += 1;
                                    *fresh27;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:digit:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh28 = *characterclass.offset(i as isize);
                                    *fresh28 += 1;
                                    *fresh28;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:graph:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh29 = *characterclass.offset(i as isize);
                                    *fresh29 += 1;
                                    *fresh29;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:lower:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISlower as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh30 = *characterclass.offset(i as isize);
                                    *fresh30 += 1;
                                    *fresh30;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:print:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh31 = *characterclass.offset(i as isize);
                                    *fresh31 += 1;
                                    *fresh31;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:punct:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh32 = *characterclass.offset(i as isize);
                                    *fresh32 += 1;
                                    *fresh32;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:space:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh33 = *characterclass.offset(i as isize);
                                    *fresh33 += 1;
                                    *fresh33;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:upper:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 10]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh34 = *characterclass.offset(i as isize);
                                    *fresh34 += 1;
                                    *fresh34;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 10]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else if trio_equal_max(
                            b"[:xdigit:]\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 11]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            &*format.offset(index as isize),
                        ) != 0
                        {
                            i = 0 as libc::c_int;
                            while i < MAX_CHARACTER_CLASS as libc::c_int {
                                if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                                {
                                    let ref mut fresh35 = *characterclass.offset(i as isize);
                                    *fresh35 += 1;
                                    *fresh35;
                                }
                                i += 1;
                                i;
                            }
                            index = (index as libc::c_ulong)
                                .wrapping_add(
                                    (::core::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        } else {
                            let ref mut fresh36 = *characterclass
                                .offset(ch as libc::c_int as isize);
                            *fresh36 += 1;
                            *fresh36;
                        }
                    }
                    _ => {
                        let ref mut fresh37 = *characterclass
                            .offset(ch as libc::c_int as isize);
                        *fresh37 += 1;
                        *fresh37;
                    }
                }
            }
            _ => {
                let ref mut fresh38 = *characterclass.offset(ch as libc::c_int as isize);
                *fresh38 += 1;
                *fresh38;
            }
        }
        index += 1;
        ch = *format.offset(index as isize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn TrioReadNumber(
    mut self_0: *mut trio_class_t,
    mut target: *mut trio_uintmax_t,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
    mut base: libc::c_int,
) -> libc::c_int {
    let mut number: trio_uintmax_t = 0 as libc::c_int as trio_uintmax_t;
    let mut digit: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut isNegative: libc::c_int = (1 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
    let mut gotNumber: libc::c_int = (1 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
    let mut j: libc::c_int = 0;
    if internalDigitsUnconverted != 0 {
        memset(
            internalDigitArray.as_mut_ptr() as *mut libc::c_void,
            -(1 as libc::c_int),
            ::core::mem::size_of::<[libc::c_int; 128]>() as libc::c_ulong,
        );
        j = 0 as libc::c_int;
        while j
            < ::core::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong
                as libc::c_int - 1 as libc::c_int
        {
            internalDigitArray[internalDigitsLower[j as usize] as libc::c_int
                as usize] = j;
            internalDigitArray[internalDigitsUpper[j as usize] as libc::c_int
                as usize] = j;
            j += 1;
            j;
        }
        internalDigitsUnconverted = (1 as libc::c_int == 0 as libc::c_int)
            as libc::c_int;
    }
    TrioSkipWhitespaces(self_0);
    if flags & FLAGS_UNSIGNED as libc::c_int as trio_flags_t == 0 {
        if (*self_0).current == '+' as i32 {
            ((*self_0).InStream)
                .expect("non-null function pointer")(self_0, 0 as *mut libc::c_int);
        } else if (*self_0).current == '-' as i32 {
            ((*self_0).InStream)
                .expect("non-null function pointer")(self_0, 0 as *mut libc::c_int);
            isNegative = !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
    }
    count = (*self_0).processed;
    if flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0 {
        match base {
            -1 | 8 | 16 | 2 => {
                if (*self_0).current == '0' as i32 {
                    ((*self_0).InStream)
                        .expect(
                            "non-null function pointer",
                        )(self_0, 0 as *mut libc::c_int);
                    if (*self_0).current != 0 {
                        if base == BASE_HEX as libc::c_int
                            && trio_to_upper((*self_0).current) == 'X' as i32
                        {
                            ((*self_0).InStream)
                                .expect(
                                    "non-null function pointer",
                                )(self_0, 0 as *mut libc::c_int);
                        } else if base == BASE_BINARY as libc::c_int
                            && trio_to_upper((*self_0).current) == 'B' as i32
                        {
                            ((*self_0).InStream)
                                .expect(
                                    "non-null function pointer",
                                )(self_0, 0 as *mut libc::c_int);
                        }
                    }
                } else {
                    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int
                }
            }
            _ => {}
        }
    }
    while (width == NO_WIDTH as libc::c_int || (*self_0).processed - count < width)
        && !((*self_0).current == -(1 as libc::c_int)
            || *(*__ctype_b_loc()).offset((*self_0).current as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        if (*self_0).current & !(0x7f as libc::c_int) == 0 as libc::c_int {
            digit = internalDigitArray[(*self_0).current as usize];
            if digit == -(1 as libc::c_int) || digit >= base {
                break;
            }
            number = number * base as trio_uintmax_t;
            number = number.wrapping_add(digit as trio_uintmax_t);
            gotNumber = !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
            ((*self_0).InStream)
                .expect("non-null function pointer")(self_0, 0 as *mut libc::c_int);
        } else {
            if !(flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0) {
                break;
            }
            j = 0 as libc::c_int;
            while internalThousandSeparator[j as usize] as libc::c_int != 0
                && (*self_0).current != 0
            {
                if internalThousandSeparator[j as usize] as libc::c_int
                    != (*self_0).current
                {
                    break;
                }
                ((*self_0).InStream)
                    .expect("non-null function pointer")(self_0, 0 as *mut libc::c_int);
                j += 1;
                j;
            }
            if internalThousandSeparator[j as usize] != 0 {
                break;
            }
        }
    }
    if gotNumber == 0 {
        return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    }
    if !target.is_null() {
        *target = if isNegative != 0 {
            -(number as trio_intmax_t) as trio_uintmax_t
        } else {
            number
        };
    }
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioReadChar(
    mut self_0: *mut trio_class_t,
    mut target: *mut libc::c_char,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    let mut number: trio_uintmax_t = 0;
    i = 0 as libc::c_int;
    while (*self_0).current != -(1 as libc::c_int) && i < width {
        ch = (*self_0).current as libc::c_char;
        ((*self_0).InStream)
            .expect("non-null function pointer")(self_0, 0 as *mut libc::c_int);
        if flags & FLAGS_ALTERNATIVE as libc::c_int as trio_flags_t != 0
            && ch as libc::c_int == '\\' as i32
        {
            match (*self_0).current {
                92 => {
                    ch = '\\' as i32 as libc::c_char;
                }
                97 => {
                    ch = '\u{7}' as i32 as libc::c_char;
                }
                98 => {
                    ch = '\u{8}' as i32 as libc::c_char;
                }
                102 => {
                    ch = '\u{c}' as i32 as libc::c_char;
                }
                110 => {
                    ch = '\n' as i32 as libc::c_char;
                }
                114 => {
                    ch = '\r' as i32 as libc::c_char;
                }
                116 => {
                    ch = '\t' as i32 as libc::c_char;
                }
                118 => {
                    ch = '\u{b}' as i32 as libc::c_char;
                }
                _ => {
                    if *(*__ctype_b_loc()).offset((*self_0).current as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        if TrioReadNumber(
                            self_0,
                            &mut number,
                            0 as libc::c_int as trio_flags_t,
                            3 as libc::c_int,
                            BASE_OCTAL as libc::c_int,
                        ) == 0
                        {
                            return 0 as libc::c_int;
                        }
                        ch = number as libc::c_char;
                    } else if trio_to_upper((*self_0).current) == 'X' as i32 {
                        ((*self_0).InStream)
                            .expect(
                                "non-null function pointer",
                            )(self_0, 0 as *mut libc::c_int);
                        if TrioReadNumber(
                            self_0,
                            &mut number,
                            0 as libc::c_int as trio_flags_t,
                            2 as libc::c_int,
                            BASE_HEX as libc::c_int,
                        ) == 0
                        {
                            return 0 as libc::c_int;
                        }
                        ch = number as libc::c_char;
                    } else {
                        ch = (*self_0).current as libc::c_char;
                    }
                }
            }
        }
        if !target.is_null() {
            *target.offset(i as isize) = ch;
        }
        i += 1;
        i;
    }
    return i + 1 as libc::c_int;
}
unsafe extern "C" fn TrioReadString(
    mut self_0: *mut trio_class_t,
    mut target: *mut libc::c_char,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    TrioSkipWhitespaces(self_0);
    i = 0 as libc::c_int;
    while (width == NO_WIDTH as libc::c_int || i < width)
        && !((*self_0).current == -(1 as libc::c_int)
            || *(*__ctype_b_loc()).offset((*self_0).current as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        if TrioReadChar(
            self_0,
            (if !target.is_null() {
                &mut *target.offset(i as isize)
            } else {
                0 as *mut libc::c_char
            }),
            flags,
            1 as libc::c_int,
        ) == 0 as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    if !target.is_null() {
        *target.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    }
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioReadGroup(
    mut self_0: *mut trio_class_t,
    mut target: *mut libc::c_char,
    mut characterclass: *mut libc::c_int,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    ch = (*self_0).current;
    i = 0 as libc::c_int;
    while (width == NO_WIDTH as libc::c_int || i < width)
        && !(ch == -(1 as libc::c_int)
            || (flags & FLAGS_EXCLUDE as libc::c_int as trio_flags_t
                != 0 as libc::c_int as trio_flags_t) as libc::c_int
                ^ (*characterclass.offset(ch as isize) == 0 as libc::c_int)
                    as libc::c_int != 0)
    {
        if !target.is_null() {
            *target.offset(i as isize) = ch as libc::c_char;
        }
        ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
        i += 1;
        i;
    }
    if !target.is_null() {
        *target.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    }
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioReadDouble(
    mut self_0: *mut trio_class_t,
    mut target: trio_pointer_t,
    mut flags: trio_flags_t,
    mut width: libc::c_int,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut doubleString: [libc::c_char; 512] = [0; 512];
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut isHex: libc::c_int = (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    doubleString[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if width == NO_WIDTH as libc::c_int
        || width
            > ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                as libc::c_int - 1 as libc::c_int
    {
        width = (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    TrioSkipWhitespaces(self_0);
    ch = (*self_0).current;
    if ch == '+' as i32 || ch == '-' as i32 {
        let fresh39 = index;
        index = index + 1;
        doubleString[fresh39 as usize] = ch as libc::c_char;
        ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
        width -= 1;
        width;
    }
    start = index;
    let mut current_block_47: u64;
    match ch {
        110 | 78 => {
            if index != 0 as libc::c_int {
                current_block_47 = 15512526488502093901;
            } else {
                current_block_47 = 1269939354136474429;
            }
        }
        105 | 73 => {
            current_block_47 = 1269939354136474429;
        }
        48 => {
            let fresh41 = index;
            index = index + 1;
            doubleString[fresh41 as usize] = ch as libc::c_char;
            ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
            if trio_to_upper(ch) == 'X' as i32 {
                isHex = !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
                let fresh42 = index;
                index = index + 1;
                doubleString[fresh42 as usize] = ch as libc::c_char;
                ((*self_0).InStream)
                    .expect("non-null function pointer")(self_0, &mut ch);
            }
            current_block_47 = 15512526488502093901;
        }
        _ => {
            current_block_47 = 15512526488502093901;
        }
    }
    match current_block_47 {
        15512526488502093901 => {}
        _ => {
            while *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                && index - start < width
            {
                let fresh40 = index;
                index = index + 1;
                doubleString[fresh40 as usize] = ch as libc::c_char;
                ((*self_0).InStream)
                    .expect("non-null function pointer")(self_0, &mut ch);
            }
            doubleString[index as usize] = 0 as libc::c_int as libc::c_char;
            if trio_equal(
                &mut *doubleString.as_mut_ptr().offset(start as isize),
                b"INF\0" as *const u8 as *const libc::c_char,
            ) != 0
                || trio_equal(
                    &mut *doubleString.as_mut_ptr().offset(start as isize),
                    b"INFINITE\0" as *const u8 as *const libc::c_char,
                ) != 0
            {
                if flags & FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t != 0 {
                    if start == 1 as libc::c_int
                        && doubleString[0 as libc::c_int as usize] as libc::c_int
                            == '-' as i32
                    {
                        *(target
                            as *mut trio_long_double_t) = f128::f128::new(trio_ninf());
                    } else {
                        *(target
                            as *mut trio_long_double_t) = f128::f128::new(trio_pinf());
                    }
                } else if start == 1 as libc::c_int
                    && doubleString[0 as libc::c_int as usize] as libc::c_int
                        == '-' as i32
                {
                    *(target as *mut libc::c_double) = trio_ninf();
                } else {
                    *(target as *mut libc::c_double) = trio_pinf();
                }
                return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
            }
            if trio_equal(
                doubleString.as_mut_ptr(),
                b"NAN\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                if flags & FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t != 0 {
                    *(target as *mut trio_long_double_t) = f128::f128::new(trio_nan());
                } else {
                    *(target as *mut libc::c_double) = trio_nan();
                }
                return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
            }
            return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
    }
    while ch != -(1 as libc::c_int) && index - start < width {
        if if isHex != 0 {
            *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
        } else {
            *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        } != 0
        {
            let fresh43 = index;
            index = index + 1;
            doubleString[fresh43 as usize] = ch as libc::c_char;
            ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
        } else {
            if !(flags & FLAGS_QUOTE as libc::c_int as trio_flags_t != 0) {
                break;
            }
            j = 0 as libc::c_int;
            while internalThousandSeparator[j as usize] as libc::c_int != 0
                && (*self_0).current != 0
            {
                if internalThousandSeparator[j as usize] as libc::c_int
                    != (*self_0).current
                {
                    break;
                }
                ((*self_0).InStream)
                    .expect("non-null function pointer")(self_0, &mut ch);
                j += 1;
                j;
            }
            if internalThousandSeparator[j as usize] != 0 {
                break;
            }
        }
    }
    if ch == '.' as i32 {
        let fresh44 = index;
        index = index + 1;
        doubleString[fresh44 as usize] = ch as libc::c_char;
        ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
        while (if isHex != 0 {
            *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
        } else {
            *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        }) != 0 && index - start < width
        {
            let fresh45 = index;
            index = index + 1;
            doubleString[fresh45 as usize] = ch as libc::c_char;
            ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
        }
        if if isHex != 0 {
            (trio_to_upper(ch) == 'P' as i32) as libc::c_int
        } else {
            (trio_to_upper(ch) == 'E' as i32) as libc::c_int
        } != 0
        {
            let fresh46 = index;
            index = index + 1;
            doubleString[fresh46 as usize] = ch as libc::c_char;
            ((*self_0).InStream).expect("non-null function pointer")(self_0, &mut ch);
            if ch == '+' as i32 || ch == '-' as i32 {
                let fresh47 = index;
                index = index + 1;
                doubleString[fresh47 as usize] = ch as libc::c_char;
                ((*self_0).InStream)
                    .expect("non-null function pointer")(self_0, &mut ch);
            }
            while *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                && index - start < width
            {
                let fresh48 = index;
                index = index + 1;
                doubleString[fresh48 as usize] = ch as libc::c_char;
                ((*self_0).InStream)
                    .expect("non-null function pointer")(self_0, &mut ch);
            }
        }
    }
    if index == start
        || *doubleString.as_mut_ptr() as libc::c_int
            == 0 as libc::c_int as libc::c_char as libc::c_int
    {
        return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    }
    doubleString[index as usize] = 0 as libc::c_int as libc::c_char;
    if flags & FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t != 0 {
        *(target
            as *mut trio_long_double_t) = trio_to_long_double(
            doubleString.as_mut_ptr(),
            0 as *mut *mut libc::c_char,
        );
    } else {
        *(target
            as *mut libc::c_double) = trio_to_double(
            doubleString.as_mut_ptr(),
            0 as *mut *mut libc::c_char,
        );
    }
    return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioReadPointer(
    mut self_0: *mut trio_class_t,
    mut target: *mut trio_pointer_t,
    mut flags: trio_flags_t,
) -> libc::c_int {
    let mut number: trio_uintmax_t = 0;
    let mut buffer: [libc::c_char; 6] = [0; 6];
    flags
        |= (FLAGS_UNSIGNED as libc::c_int | FLAGS_ALTERNATIVE as libc::c_int
            | FLAGS_NILPADDING as libc::c_int) as trio_flags_t;
    if TrioReadNumber(
        self_0,
        &mut number,
        flags,
        (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<trio_pointer_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(4 as libc::c_int as libc::c_ulong),
            ) as libc::c_int,
        BASE_HEX as libc::c_int,
    ) != 0
    {
        if !target.is_null() {
            *target = (0 as *mut libc::c_char).offset(number as isize) as trio_pointer_t;
        }
        return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    } else if TrioReadString(
        self_0,
        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t != 0 {
            0 as *mut libc::c_char
        } else {
            buffer.as_mut_ptr()
        },
        0 as libc::c_int as trio_flags_t,
        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) != 0
    {
        if trio_equal_case(buffer.as_mut_ptr(), internalNullString.as_ptr()) != 0 {
            if !target.is_null() {
                *target = 0 as *mut libc::c_void;
            }
            return !(1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
        }
    }
    return (1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TrioScanProcess(
    mut data: *mut trio_class_t,
    mut format: *const libc::c_char,
    mut parameters: *mut trio_parameter_t,
) -> libc::c_int {
    let mut charlen: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut assignment: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut flags: trio_flags_t = 0;
    let mut width: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut pointer: trio_pointer_t = 0 as *mut libc::c_void;
    assignment = 0 as libc::c_int;
    i = 0 as libc::c_int;
    index = 0 as libc::c_int;
    ((*data).InStream).expect("non-null function pointer")(data, &mut ch);
    mblen(0 as *const libc::c_char, 0 as libc::c_int as size_t);
    while *format.offset(index as isize) != 0 {
        if !(*format.offset(index as isize) as libc::c_int & !(0x7f as libc::c_int)
            == 0 as libc::c_int)
        {
            charlen = mblen(
                &*format.offset(index as isize),
                16 as libc::c_int as size_t,
            );
            if charlen != -(1 as libc::c_int) {
                cnt = 0 as libc::c_int;
                while cnt < charlen - 1 as libc::c_int {
                    if ch != *format.offset((index + cnt) as isize) as libc::c_int {
                        return -(TRIO_EINVAL as libc::c_int
                            + (index << 8 as libc::c_int));
                    }
                    ((*data).InStream)
                        .expect("non-null function pointer")(data, &mut ch);
                    cnt += 1;
                    cnt;
                }
                continue;
            }
        }
        if -(1 as libc::c_int) == ch
            && (*parameters.offset(i as isize)).type_0 != 6 as libc::c_int
        {
            return if assignment > 0 as libc::c_int {
                assignment
            } else {
                -(1 as libc::c_int)
            };
        }
        if '%' as i32 == *format.offset(index as isize) as libc::c_int {
            if '%' as i32
                == *format.offset((index + 1 as libc::c_int) as isize) as libc::c_int
            {
                if '%' as i32 == ch {
                    ((*data).InStream)
                        .expect("non-null function pointer")(data, &mut ch);
                    index += 2 as libc::c_int;
                } else {
                    return -(TRIO_EINVAL as libc::c_int + (index << 8 as libc::c_int))
                }
            } else {
                while (*parameters.offset(i as isize)).type_0 == 7 as libc::c_int {
                    i += 1;
                    i;
                }
                flags = (*parameters.offset(i as isize)).flags;
                width = (*parameters.offset(i as isize)).width;
                if flags & FLAGS_WIDTH_PARAMETER as libc::c_int as trio_flags_t != 0 {
                    width = (*parameters.offset(width as isize)).data.number.as_signed
                        as libc::c_int;
                }
                base = (*parameters.offset(i as isize)).base;
                if flags & FLAGS_BASE_PARAMETER as libc::c_int as trio_flags_t != 0 {
                    base = (*parameters.offset(base as isize)).data.number.as_signed
                        as libc::c_int;
                }
                match (*parameters.offset(i as isize)).type_0 {
                    1 => {
                        let mut number: trio_uintmax_t = 0;
                        if 0 as libc::c_int == base {
                            base = BASE_DECIMAL as libc::c_int;
                        }
                        if TrioReadNumber(data, &mut number, flags, width, base) == 0 {
                            return assignment;
                        }
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t == 0 {
                            assignment += 1;
                            assignment;
                            pointer = (*parameters.offset(i as isize)).data.pointer;
                            if flags & FLAGS_SIZE_T as libc::c_int as trio_flags_t != 0 {
                                *(pointer as *mut size_t) = number;
                            } else if flags
                                & FLAGS_PTRDIFF_T as libc::c_int as trio_flags_t != 0
                            {
                                *(pointer as *mut ptrdiff_t) = number as ptrdiff_t;
                            } else if flags
                                & FLAGS_INTMAX_T as libc::c_int as trio_flags_t != 0
                            {
                                *(pointer as *mut trio_intmax_t) = number as trio_intmax_t;
                            } else if flags & FLAGS_QUAD as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer
                                    as *mut trio_ulonglong_t) = number as trio_ulonglong_t;
                            } else if flags & FLAGS_LONG as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer as *mut libc::c_long) = number as libc::c_long;
                            } else if flags & FLAGS_SHORT as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer as *mut libc::c_short) = number as libc::c_short;
                            } else {
                                *(pointer as *mut libc::c_int) = number as libc::c_int;
                            }
                        }
                    }
                    4 => {
                        if TrioReadString(
                            data,
                            if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t != 0 {
                                0 as *mut libc::c_char
                            } else {
                                (*parameters.offset(i as isize)).data.string
                            },
                            flags,
                            width,
                        ) == 0
                        {
                            return assignment;
                        }
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t == 0 {
                            assignment += 1;
                            assignment;
                        }
                    }
                    2 => {
                        let mut pointer_0: trio_pointer_t = 0 as *mut libc::c_void;
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t != 0 {
                            pointer_0 = 0 as *mut libc::c_void;
                        } else {
                            pointer_0 = if flags
                                & FLAGS_LONGDOUBLE as libc::c_int as trio_flags_t != 0
                            {
                                (*parameters.offset(i as isize)).data.longdoublePointer
                                    as trio_pointer_t
                            } else {
                                (*parameters.offset(i as isize)).data.doublePointer
                                    as trio_pointer_t
                            };
                        }
                        if TrioReadDouble(data, pointer_0, flags, width) == 0 {
                            return assignment;
                        }
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t == 0 {
                            assignment += 1;
                            assignment;
                        }
                    }
                    8 => {
                        let mut characterclass: [libc::c_int; 257] = [0; 257];
                        let mut rc: libc::c_int = 0;
                        while *format.offset(index as isize) as libc::c_int != '[' as i32
                        {
                            index += 1;
                            index;
                        }
                        index += 1;
                        index;
                        memset(
                            characterclass.as_mut_ptr() as *mut libc::c_void,
                            0 as libc::c_int,
                            ::core::mem::size_of::<[libc::c_int; 257]>() as libc::c_ulong,
                        );
                        rc = TrioGetCharacterClass(
                            format,
                            &mut index,
                            &mut flags,
                            characterclass.as_mut_ptr(),
                        );
                        if rc < 0 as libc::c_int {
                            return rc;
                        }
                        if TrioReadGroup(
                            data,
                            if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t != 0 {
                                0 as *mut libc::c_char
                            } else {
                                (*parameters.offset(i as isize)).data.string
                            },
                            characterclass.as_mut_ptr(),
                            flags,
                            (*parameters.offset(i as isize)).width,
                        ) == 0
                        {
                            return assignment;
                        }
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t == 0 {
                            assignment += 1;
                            assignment;
                        }
                    }
                    6 => {
                        pointer = (*parameters.offset(i as isize)).data.pointer;
                        if !pointer.is_null() {
                            let mut count: libc::c_int = (*data).committed;
                            if ch != -(1 as libc::c_int) {
                                count -= 1;
                                count;
                            }
                            if flags & FLAGS_SIZE_T as libc::c_int as trio_flags_t != 0 {
                                *(pointer as *mut size_t) = count as size_t;
                            } else if flags
                                & FLAGS_PTRDIFF_T as libc::c_int as trio_flags_t != 0
                            {
                                *(pointer as *mut ptrdiff_t) = count as ptrdiff_t;
                            } else if flags
                                & FLAGS_INTMAX_T as libc::c_int as trio_flags_t != 0
                            {
                                *(pointer as *mut trio_intmax_t) = count as trio_intmax_t;
                            } else if flags & FLAGS_QUAD as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer
                                    as *mut trio_ulonglong_t) = count as trio_ulonglong_t;
                            } else if flags & FLAGS_LONG as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer as *mut libc::c_long) = count as libc::c_long;
                            } else if flags & FLAGS_SHORT as libc::c_int as trio_flags_t
                                != 0
                            {
                                *(pointer as *mut libc::c_short) = count as libc::c_short;
                            } else {
                                *(pointer as *mut libc::c_int) = count;
                            }
                        }
                    }
                    3 => {
                        if TrioReadChar(
                            data,
                            (if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t != 0
                            {
                                0 as *mut libc::c_char
                            } else {
                                (*parameters.offset(i as isize)).data.string
                            }),
                            flags,
                            (if width == NO_WIDTH as libc::c_int {
                                1 as libc::c_int
                            } else {
                                width
                            }),
                        ) == 0 as libc::c_int
                        {
                            return assignment;
                        }
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t == 0 {
                            assignment += 1;
                            assignment;
                        }
                    }
                    5 => {
                        if TrioReadPointer(
                            data,
                            if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t != 0 {
                                0 as *mut trio_pointer_t
                            } else {
                                (*parameters.offset(i as isize)).data.pointer
                                    as *mut trio_pointer_t
                            },
                            flags,
                        ) == 0
                        {
                            return assignment;
                        }
                        if flags & FLAGS_IGNORE as libc::c_int as trio_flags_t == 0 {
                            assignment += 1;
                            assignment;
                        }
                    }
                    7 => {}
                    _ => {
                        return -(TRIO_EINVAL as libc::c_int
                            + (index << 8 as libc::c_int));
                    }
                }
                ch = (*data).current;
                index = (*parameters.offset(i as isize)).indexAfterSpecifier;
                i += 1;
                i;
            }
        } else {
            if *(*__ctype_b_loc())
                .offset(*format.offset(index as isize) as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                ch = TrioSkipWhitespaces(data);
            } else if ch == *format.offset(index as isize) as libc::c_int {
                ((*data).InStream).expect("non-null function pointer")(data, &mut ch);
            } else {
                return assignment
            }
            index += 1;
            index;
        }
    }
    return assignment;
}
unsafe extern "C" fn TrioScan(
    mut source: trio_pointer_t,
    mut sourceSize: size_t,
    mut InStream: Option::<
        unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
    >,
    mut format: *const libc::c_char,
    mut arglist: *mut ::core::ffi::VaList,
    mut argarray: *mut trio_pointer_t,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut parameters: [trio_parameter_t; 64] = [trio_parameter_t {
        type_0: 0,
        flags: 0,
        width: 0,
        precision: 0,
        base: 0,
        varsize: 0,
        indexAfterSpecifier: 0,
        data: C2RustUnnamed_0 {
            string: 0 as *mut libc::c_char,
        },
        user_name: [0; 64],
        user_data: [0; 256],
    }; 64];
    let mut data: trio_class_t = _trio_class_t {
        OutStream: None,
        InStream: None,
        location: 0 as *mut libc::c_void,
        current: 0,
        processed: 0,
        committed: 0,
        max: 0,
        error: 0,
    };
    memset(
        &mut data as *mut trio_class_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<trio_class_t>() as libc::c_ulong,
    );
    data.InStream = InStream;
    data.location = source;
    data.max = sourceSize as libc::c_int;
    data.error = 0 as libc::c_int;
    if internalLocaleValues.is_null() {
        TrioSetLocale();
    }
    status = TrioParse(
        TYPE_SCAN as libc::c_int,
        format,
        parameters.as_mut_ptr(),
        arglist,
        argarray,
    );
    if status < 0 as libc::c_int {
        return status;
    }
    status = TrioScanProcess(&mut data, format, parameters.as_mut_ptr());
    if data.error != 0 as libc::c_int {
        status = data.error;
    }
    return status;
}
unsafe extern "C" fn TrioInStreamFile(
    mut self_0: *mut trio_class_t,
    mut intPointer: *mut libc::c_int,
) {
    let mut file: *mut FILE = 0 as *mut FILE;
    file = (*self_0).location as *mut FILE;
    (*self_0).current = fgetc(file);
    if (*self_0).current == -(1 as libc::c_int) {
        (*self_0)
            .error = if ferror(file) != 0 {
            -(TRIO_ERRNO as libc::c_int + ((0 as libc::c_int) << 8 as libc::c_int))
        } else {
            -(TRIO_EOF as libc::c_int + ((0 as libc::c_int) << 8 as libc::c_int))
        };
    } else {
        (*self_0).processed += 1;
        (*self_0).processed;
        (*self_0).committed += 1;
        (*self_0).committed;
    }
    if !intPointer.is_null() {
        *intPointer = (*self_0).current;
    }
}
unsafe extern "C" fn TrioInStreamFileDescriptor(
    mut self_0: *mut trio_class_t,
    mut intPointer: *mut libc::c_int,
) {
    let mut fd: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut input: libc::c_uchar = 0;
    fd = *((*self_0).location as *mut libc::c_int);
    size = read(
        fd,
        &mut input as *mut libc::c_uchar as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as libc::c_int;
    if size == -(1 as libc::c_int) {
        (*self_0)
            .error = -(TRIO_ERRNO as libc::c_int
            + ((0 as libc::c_int) << 8 as libc::c_int));
        (*self_0).current = -(1 as libc::c_int);
    } else {
        (*self_0)
            .current = if size == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            input as libc::c_int
        };
    }
    if (*self_0).current != -(1 as libc::c_int) {
        (*self_0).committed += 1;
        (*self_0).committed;
        (*self_0).processed += 1;
        (*self_0).processed;
    }
    if !intPointer.is_null() {
        *intPointer = (*self_0).current;
    }
}
unsafe extern "C" fn TrioInStreamCustom(
    mut self_0: *mut trio_class_t,
    mut intPointer: *mut libc::c_int,
) {
    let mut data: *mut trio_custom_t = 0 as *mut trio_custom_t;
    data = (*self_0).location as *mut trio_custom_t;
    (*self_0)
        .current = if ((*data).stream.in_0).is_none() {
        0 as libc::c_int as libc::c_char as libc::c_int
    } else {
        ((*data).stream.in_0).expect("non-null function pointer")((*data).closure)
    };
    if (*self_0).current == 0 as libc::c_int as libc::c_char as libc::c_int {
        (*self_0).current = -(1 as libc::c_int);
    } else {
        (*self_0).processed += 1;
        (*self_0).processed;
        (*self_0).committed += 1;
        (*self_0).committed;
    }
    if !intPointer.is_null() {
        *intPointer = (*self_0).current;
    }
}
unsafe extern "C" fn TrioInStreamString(
    mut self_0: *mut trio_class_t,
    mut intPointer: *mut libc::c_int,
) {
    let mut buffer: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    buffer = (*self_0).location as *mut *mut libc::c_uchar;
    (*self_0).current = *(*buffer).offset(0 as libc::c_int as isize) as libc::c_int;
    if (*self_0).current == 0 as libc::c_int as libc::c_char as libc::c_int {
        (*self_0).current = -(1 as libc::c_int);
    } else {
        *buffer = (*buffer).offset(1);
        *buffer;
        (*self_0).processed += 1;
        (*self_0).processed;
        (*self_0).committed += 1;
        (*self_0).committed;
    }
    if !intPointer.is_null() {
        *intPointer = (*self_0).current;
    }
}
#[no_mangle]
pub unsafe extern "C" fn trio_scanf(
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioScan(
        stdin as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vscanf(
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioScan(
        stdin as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_scanfv(
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioScan(
        stdin as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_fscanf(
    mut file: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioScan(
        file as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vfscanf(
    mut file: *mut FILE,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioScan(
        file as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_fscanfv(
    mut file: *mut FILE,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioScan(
        file as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFile
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_dscanf(
    mut fd: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioScan(
        &mut fd as *mut libc::c_int as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFileDescriptor
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vdscanf(
    mut fd: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioScan(
        &mut fd as *mut libc::c_int as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFileDescriptor
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_dscanfv(
    mut fd: libc::c_int,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioScan(
        &mut fd as *mut libc::c_int as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamFileDescriptor
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_cscanf(
    mut stream: trio_instream_t,
    mut closure: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut data: trio_custom_t = trio_custom_t {
        stream: C2RustUnnamed_2 { out: None },
        closure: 0 as *mut libc::c_void,
    };
    args_0 = args.clone();
    data.stream.in_0 = stream;
    data.closure = closure;
    status = TrioScan(
        &mut data as *mut trio_custom_t as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamCustom
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vcscanf(
    mut stream: trio_instream_t,
    mut closure: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut data: trio_custom_t = trio_custom_t {
        stream: C2RustUnnamed_2 { out: None },
        closure: 0 as *mut libc::c_void,
    };
    data.stream.in_0 = stream;
    data.closure = closure;
    return TrioScan(
        &mut data as *mut trio_custom_t as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamCustom
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_cscanfv(
    mut stream: trio_instream_t,
    mut closure: trio_pointer_t,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    let mut data: trio_custom_t = trio_custom_t {
        stream: C2RustUnnamed_2 { out: None },
        closure: 0 as *mut libc::c_void,
    };
    data.stream.in_0 = stream;
    data.closure = closure;
    return TrioScan(
        &mut data as *mut trio_custom_t as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamCustom
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_sscanf(
    mut buffer: *const libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    status = TrioScan(
        &mut buffer as *mut *const libc::c_char as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamString
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args_0.as_va_list(),
        0 as *mut trio_pointer_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn trio_vsscanf(
    mut buffer: *const libc::c_char,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return TrioScan(
        &mut buffer as *mut *const libc::c_char as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamString
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        &mut args.as_va_list() as *mut ::core::ffi::VaList as *mut ::core::ffi::VaList,
        0 as *mut trio_pointer_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_sscanfv(
    mut buffer: *const libc::c_char,
    mut format: *const libc::c_char,
    mut args: *mut trio_pointer_t,
) -> libc::c_int {
    return TrioScan(
        &mut buffer as *mut *const libc::c_char as trio_pointer_t,
        0 as libc::c_int as size_t,
        Some(
            TrioInStreamString
                as unsafe extern "C" fn(*mut trio_class_t, *mut libc::c_int) -> (),
        ),
        format,
        0 as *mut ::core::ffi::VaList,
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn trio_strerror(
    mut errorcode: libc::c_int,
) -> *const libc::c_char {
    match -errorcode & 0xff as libc::c_int {
        1 => return b"End of file\0" as *const u8 as *const libc::c_char,
        2 => return b"Invalid argument\0" as *const u8 as *const libc::c_char,
        3 => return b"Too many arguments\0" as *const u8 as *const libc::c_char,
        4 => return b"Double reference\0" as *const u8 as *const libc::c_char,
        5 => return b"Reference gap\0" as *const u8 as *const libc::c_char,
        6 => return b"Out of memory\0" as *const u8 as *const libc::c_char,
        7 => return b"Invalid range\0" as *const u8 as *const libc::c_char,
        9 => return b"Custom error\0" as *const u8 as *const libc::c_char,
        _ => return b"Unknown\0" as *const u8 as *const libc::c_char,
    };
}
