use ::libc;
extern "C" {
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type environ;
    pub type options;
    pub type hooks;
    pub type options_entry;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn tzset();
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    static mut environ: *mut *mut libc::c_char;
    fn getuid() -> __uid_t;
    fn err(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn errx(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn getprogname() -> *const libc::c_char;
    fn getptmfd() -> libc::c_int;
    static mut BSDoptind: libc::c_int;
    static mut BSDoptarg: *mut libc::c_char;
    fn BSDgetopt(
        _: libc::c_int,
        _: *const *mut libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn set_cfg_file(_: *const libc::c_char);
    fn hooks_create(_: *mut hooks) -> *mut hooks;
    fn options_create(_: *mut options) -> *mut options;
    fn options_default(
        _: *mut options,
        _: *const options_table_entry,
    ) -> *mut options_entry;
    fn options_set_string(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> *mut options_entry;
    fn options_set_number(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut options_entry;
    static options_table: [options_table_entry; 0];
    fn environ_create() -> *mut environ;
    fn environ_set(
        _: *mut environ,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    fn environ_put(_: *mut environ, _: *const libc::c_char);
    fn client_main(
        _: *mut event_base,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn osdep_event_init() -> *mut event_base;
    fn log_add_level();
}
pub type __u_int = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type u_int = __u_int;
pub type uid_t = __uid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type nl_item = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed = 327684;
pub const __NOSTR: C2RustUnnamed = 327683;
pub const __YESSTR: C2RustUnnamed = 327682;
pub const __NOEXPR: C2RustUnnamed = 327681;
pub const __YESEXPR: C2RustUnnamed = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed = 65539;
pub const __GROUPING: C2RustUnnamed = 65538;
pub const THOUSEP: C2RustUnnamed = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed = 65537;
pub const RADIXCHAR: C2RustUnnamed = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed = 262149;
pub const __MON_GROUPING: C2RustUnnamed = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed = 15;
pub const CODESET: C2RustUnnamed = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed = 131195;
pub const __ALTMON_12: C2RustUnnamed = 131194;
pub const __ALTMON_11: C2RustUnnamed = 131193;
pub const __ALTMON_10: C2RustUnnamed = 131192;
pub const __ALTMON_9: C2RustUnnamed = 131191;
pub const __ALTMON_8: C2RustUnnamed = 131190;
pub const __ALTMON_7: C2RustUnnamed = 131189;
pub const __ALTMON_6: C2RustUnnamed = 131188;
pub const __ALTMON_5: C2RustUnnamed = 131187;
pub const __ALTMON_4: C2RustUnnamed = 131186;
pub const __ALTMON_3: C2RustUnnamed = 131185;
pub const __ALTMON_2: C2RustUnnamed = 131184;
pub const __ALTMON_1: C2RustUnnamed = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed = 131181;
pub const _DATE_FMT: C2RustUnnamed = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed = 131167;
pub const _NL_WT_FMT: C2RustUnnamed = 131166;
pub const _NL_WD_FMT: C2RustUnnamed = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed = 131164;
pub const _NL_WPM_STR: C2RustUnnamed = 131163;
pub const _NL_WAM_STR: C2RustUnnamed = 131162;
pub const _NL_WMON_12: C2RustUnnamed = 131161;
pub const _NL_WMON_11: C2RustUnnamed = 131160;
pub const _NL_WMON_10: C2RustUnnamed = 131159;
pub const _NL_WMON_9: C2RustUnnamed = 131158;
pub const _NL_WMON_8: C2RustUnnamed = 131157;
pub const _NL_WMON_7: C2RustUnnamed = 131156;
pub const _NL_WMON_6: C2RustUnnamed = 131155;
pub const _NL_WMON_5: C2RustUnnamed = 131154;
pub const _NL_WMON_4: C2RustUnnamed = 131153;
pub const _NL_WMON_3: C2RustUnnamed = 131152;
pub const _NL_WMON_2: C2RustUnnamed = 131151;
pub const _NL_WMON_1: C2RustUnnamed = 131150;
pub const _NL_WABMON_12: C2RustUnnamed = 131149;
pub const _NL_WABMON_11: C2RustUnnamed = 131148;
pub const _NL_WABMON_10: C2RustUnnamed = 131147;
pub const _NL_WABMON_9: C2RustUnnamed = 131146;
pub const _NL_WABMON_8: C2RustUnnamed = 131145;
pub const _NL_WABMON_7: C2RustUnnamed = 131144;
pub const _NL_WABMON_6: C2RustUnnamed = 131143;
pub const _NL_WABMON_5: C2RustUnnamed = 131142;
pub const _NL_WABMON_4: C2RustUnnamed = 131141;
pub const _NL_WABMON_3: C2RustUnnamed = 131140;
pub const _NL_WABMON_2: C2RustUnnamed = 131139;
pub const _NL_WABMON_1: C2RustUnnamed = 131138;
pub const _NL_WDAY_7: C2RustUnnamed = 131137;
pub const _NL_WDAY_6: C2RustUnnamed = 131136;
pub const _NL_WDAY_5: C2RustUnnamed = 131135;
pub const _NL_WDAY_4: C2RustUnnamed = 131134;
pub const _NL_WDAY_3: C2RustUnnamed = 131133;
pub const _NL_WDAY_2: C2RustUnnamed = 131132;
pub const _NL_WDAY_1: C2RustUnnamed = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed = 131122;
pub const ERA_T_FMT: C2RustUnnamed = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed = 131120;
pub const ALT_DIGITS: C2RustUnnamed = 131119;
pub const ERA_D_FMT: C2RustUnnamed = 131118;
pub const __ERA_YEAR: C2RustUnnamed = 131117;
pub const ERA: C2RustUnnamed = 131116;
pub const T_FMT_AMPM: C2RustUnnamed = 131115;
pub const T_FMT: C2RustUnnamed = 131114;
pub const D_FMT: C2RustUnnamed = 131113;
pub const D_T_FMT: C2RustUnnamed = 131112;
pub const PM_STR: C2RustUnnamed = 131111;
pub const AM_STR: C2RustUnnamed = 131110;
pub const MON_12: C2RustUnnamed = 131109;
pub const MON_11: C2RustUnnamed = 131108;
pub const MON_10: C2RustUnnamed = 131107;
pub const MON_9: C2RustUnnamed = 131106;
pub const MON_8: C2RustUnnamed = 131105;
pub const MON_7: C2RustUnnamed = 131104;
pub const MON_6: C2RustUnnamed = 131103;
pub const MON_5: C2RustUnnamed = 131102;
pub const MON_4: C2RustUnnamed = 131101;
pub const MON_3: C2RustUnnamed = 131100;
pub const MON_2: C2RustUnnamed = 131099;
pub const MON_1: C2RustUnnamed = 131098;
pub const ABMON_12: C2RustUnnamed = 131097;
pub const ABMON_11: C2RustUnnamed = 131096;
pub const ABMON_10: C2RustUnnamed = 131095;
pub const ABMON_9: C2RustUnnamed = 131094;
pub const ABMON_8: C2RustUnnamed = 131093;
pub const ABMON_7: C2RustUnnamed = 131092;
pub const ABMON_6: C2RustUnnamed = 131091;
pub const ABMON_5: C2RustUnnamed = 131090;
pub const ABMON_4: C2RustUnnamed = 131089;
pub const ABMON_3: C2RustUnnamed = 131088;
pub const ABMON_2: C2RustUnnamed = 131087;
pub const ABMON_1: C2RustUnnamed = 131086;
pub const DAY_7: C2RustUnnamed = 131085;
pub const DAY_6: C2RustUnnamed = 131084;
pub const DAY_5: C2RustUnnamed = 131083;
pub const DAY_4: C2RustUnnamed = 131082;
pub const DAY_3: C2RustUnnamed = 131081;
pub const DAY_2: C2RustUnnamed = 131080;
pub const DAY_1: C2RustUnnamed = 131079;
pub const ABDAY_7: C2RustUnnamed = 131078;
pub const ABDAY_6: C2RustUnnamed = 131077;
pub const ABDAY_5: C2RustUnnamed = 131076;
pub const ABDAY_4: C2RustUnnamed = 131075;
pub const ABDAY_3: C2RustUnnamed = 131074;
pub const ABDAY_2: C2RustUnnamed = 131073;
pub const ABDAY_1: C2RustUnnamed = 131072;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: options_table_scope,
    pub minimum: u_int,
    pub maximum: u_int,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub separator: *const libc::c_char,
    pub style: *const libc::c_char,
}
#[no_mangle]
pub static mut global_options: *mut options = 0 as *const options as *mut options;
#[no_mangle]
pub static mut global_s_options: *mut options = 0 as *const options as *mut options;
#[no_mangle]
pub static mut global_w_options: *mut options = 0 as *const options as *mut options;
#[no_mangle]
pub static mut global_environ: *mut environ = 0 as *const environ as *mut environ;
#[no_mangle]
pub static mut global_hooks: *mut hooks = 0 as *const hooks as *mut hooks;
#[no_mangle]
pub static mut start_time: timeval = timeval { tv_sec: 0, tv_usec: 0 };
#[no_mangle]
pub static mut socket_path: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut ptm_fd: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut shell_command: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn usage() -> ! {
    fprintf(
        stderr,
        b"usage: %s [-2CluvV] [-c shell-command] [-f file] [-L socket-name]\n            [-S socket-path] [command [flags]]\n\0"
            as *const u8 as *const libc::c_char,
        getprogname(),
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn getshell() -> *const libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    shell = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
    if checkshell(shell) != 0 {
        return shell;
    }
    pw = getpwuid(getuid());
    if !pw.is_null() && checkshell((*pw).pw_shell) != 0 {
        return (*pw).pw_shell;
    }
    return b"/bin/sh\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn checkshell(mut shell: *const libc::c_char) -> libc::c_int {
    if shell.is_null() || *shell as libc::c_int != '/' as i32 {
        return 0 as libc::c_int;
    }
    if areshell(shell) != 0 {
        return 0 as libc::c_int;
    }
    if access(shell, 1 as libc::c_int) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn areshell(mut shell: *const libc::c_char) -> libc::c_int {
    let mut progname: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ptr = strrchr(shell, '/' as i32);
    if !ptr.is_null() {
        ptr = ptr.offset(1);
        ptr;
    } else {
        ptr = shell;
    }
    progname = getprogname();
    if *progname as libc::c_int == '-' as i32 {
        progname = progname.offset(1);
        progname;
    }
    if strcmp(ptr, progname) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_label(
    mut label: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut uid: uid_t = 0;
    *cause = 0 as *mut libc::c_char;
    if label.is_null() {
        label = b"default\0" as *const u8 as *const libc::c_char;
    }
    uid = getuid();
    s = getenv(b"TMUX_TMPDIR\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && *s as libc::c_int != '\0' as i32 {
        xasprintf(
            &mut base as *mut *mut libc::c_char,
            b"%s/tmux-%ld\0" as *const u8 as *const libc::c_char,
            s,
            uid as libc::c_long,
        );
    } else {
        xasprintf(
            &mut base as *mut *mut libc::c_char,
            b"%s/tmux-%ld\0" as *const u8 as *const libc::c_char,
            b"/tmp/\0" as *const u8 as *const libc::c_char,
            uid as libc::c_long,
        );
    }
    if (realpath(base, resolved.as_mut_ptr())).is_null()
        && strlcpy(
            resolved.as_mut_ptr(),
            base,
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        ) >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
    {
        *__errno_location() = 34 as libc::c_int;
        free(base as *mut libc::c_void);
    } else if !(mkdir(
        resolved.as_mut_ptr(),
        (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int) as __mode_t,
    ) != 0 as libc::c_int && *__errno_location() != 17 as libc::c_int)
    {
        if !(lstat(resolved.as_mut_ptr(), &mut sb) != 0 as libc::c_int) {
            if !(sb.st_mode & 0o170000 as libc::c_int as __mode_t
                == 0o40000 as libc::c_int as __mode_t)
            {
                *__errno_location() = 20 as libc::c_int;
            } else if sb.st_uid != uid
                || sb.st_mode
                    & ((0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                        as __mode_t != 0 as libc::c_int as __mode_t
            {
                *__errno_location() = 13 as libc::c_int;
            } else {
                xasprintf(
                    &mut path as *mut *mut libc::c_char,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    resolved.as_mut_ptr(),
                    label,
                );
                return path;
            }
        }
    }
    xasprintf(
        cause,
        b"error creating %s (%s)\0" as *const u8 as *const libc::c_char,
        resolved.as_mut_ptr(),
        strerror(*__errno_location()),
    );
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn setblocking(mut fd: libc::c_int, mut state: libc::c_int) {
    let mut mode: libc::c_int = 0;
    mode = fcntl(fd, 3 as libc::c_int);
    if mode != -(1 as libc::c_int) {
        if state == 0 {
            mode |= 0o4000 as libc::c_int;
        } else {
            mode &= !(0o4000 as libc::c_int);
        }
        fcntl(fd, 4 as libc::c_int, mode);
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_home() -> *const libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    static mut home: *const libc::c_char = 0 as *const libc::c_char;
    if !home.is_null() {
        return home;
    }
    home = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if home.is_null() || *home as libc::c_int == '\0' as i32 {
        pw = getpwuid(getuid());
        if !pw.is_null() {
            home = (*pw).pw_dir;
        } else {
            home = 0 as *const libc::c_char;
        }
    }
    return home;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut var: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut keys: libc::c_int = 0;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    if (setlocale(
        0 as libc::c_int,
        b"en_US.UTF-8\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
        && (setlocale(
            0 as libc::c_int,
            b"C.UTF-8\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
    {
        if (setlocale(0 as libc::c_int, b"\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            errx(
                1 as libc::c_int,
                b"invalid LC_ALL, LC_CTYPE or LANG\0" as *const u8 as *const libc::c_char,
            );
        }
        s = nl_langinfo(CODESET as libc::c_int);
        if strcasecmp(s, b"UTF-8\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
            && strcasecmp(s, b"UTF8\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
        {
            errx(
                1 as libc::c_int,
                b"need UTF-8 locale (LC_CTYPE) but have %s\0" as *const u8
                    as *const libc::c_char,
                s,
            );
        }
    }
    setlocale(2 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    tzset();
    if **argv as libc::c_int == '-' as i32 {
        flags = 0x2 as libc::c_int;
    } else {
        flags = 0 as libc::c_int;
    }
    path = 0 as *mut libc::c_char;
    label = path;
    loop {
        opt = BSDgetopt(
            argc,
            argv,
            b"2c:Cdf:lL:qS:uUVv\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            50 => {
                flags |= 0x20000 as libc::c_int;
            }
            99 => {
                shell_command = BSDoptarg;
            }
            67 => {
                if flags & 0x2000 as libc::c_int != 0 {
                    flags |= 0x4000 as libc::c_int;
                } else {
                    flags |= 0x2000 as libc::c_int;
                }
            }
            86 => {
                printf(
                    b"%s %s\n\0" as *const u8 as *const libc::c_char,
                    getprogname(),
                    b"master\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            102 => {
                set_cfg_file(BSDoptarg);
            }
            108 => {
                flags |= 0x2 as libc::c_int;
            }
            76 => {
                free(label as *mut libc::c_void);
                label = xstrdup(BSDoptarg);
            }
            113 => {}
            83 => {
                free(path as *mut libc::c_void);
                path = xstrdup(BSDoptarg);
            }
            117 => {
                flags |= 0x10000 as libc::c_int;
            }
            118 => {
                log_add_level();
            }
            _ => {
                usage();
            }
        }
    }
    argc -= BSDoptind;
    argv = argv.offset(BSDoptind as isize);
    if !shell_command.is_null() && argc != 0 as libc::c_int {
        usage();
    }
    ptm_fd = getptmfd();
    if ptm_fd == -(1 as libc::c_int) {
        err(1 as libc::c_int, b"getptmfd\0" as *const u8 as *const libc::c_char);
    }
    if 0 as libc::c_int != 0 as libc::c_int {
        err(1 as libc::c_int, b"pledge\0" as *const u8 as *const libc::c_char);
    }
    if !(getenv(b"TMUX\0" as *const u8 as *const libc::c_char)).is_null() {
        flags |= 0x10000 as libc::c_int;
    } else {
        s = getenv(b"LC_ALL\0" as *const u8 as *const libc::c_char);
        if s.is_null() || *s as libc::c_int == '\0' as i32 {
            s = getenv(b"LC_CTYPE\0" as *const u8 as *const libc::c_char);
        }
        if s.is_null() || *s as libc::c_int == '\0' as i32 {
            s = getenv(b"LANG\0" as *const u8 as *const libc::c_char);
        }
        if s.is_null() || *s as libc::c_int == '\0' as i32 {
            s = b"\0" as *const u8 as *const libc::c_char;
        }
        if !(strcasestr(s, b"UTF-8\0" as *const u8 as *const libc::c_char)).is_null()
            || !(strcasestr(s, b"UTF8\0" as *const u8 as *const libc::c_char)).is_null()
        {
            flags |= 0x10000 as libc::c_int;
        }
    }
    global_hooks = hooks_create(0 as *mut hooks);
    global_environ = environ_create();
    var = environ;
    while !(*var).is_null() {
        environ_put(global_environ, *var);
        var = var.offset(1);
        var;
    }
    cwd = getenv(b"PWD\0" as *const u8 as *const libc::c_char);
    if cwd.is_null()
        && {
            cwd = getcwd(
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            );
            !cwd.is_null()
        }
    {
        environ_set(
            global_environ,
            b"PWD\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            cwd,
        );
    }
    global_options = options_create(0 as *mut options);
    global_s_options = options_create(0 as *mut options);
    global_w_options = options_create(0 as *mut options);
    oe = options_table.as_ptr();
    while !((*oe).name).is_null() {
        if (*oe).scope as libc::c_uint
            == OPTIONS_TABLE_SERVER as libc::c_int as libc::c_uint
        {
            options_default(global_options, oe);
        }
        if (*oe).scope as libc::c_uint
            == OPTIONS_TABLE_SESSION as libc::c_int as libc::c_uint
        {
            options_default(global_s_options, oe);
        }
        if (*oe).scope as libc::c_uint
            == OPTIONS_TABLE_WINDOW as libc::c_int as libc::c_uint
        {
            options_default(global_w_options, oe);
        }
        oe = oe.offset(1);
        oe;
    }
    shell = getshell();
    options_set_string(
        global_s_options,
        b"default-shell\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        shell,
    );
    s = getenv(b"VISUAL\0" as *const u8 as *const libc::c_char);
    if !s.is_null()
        || {
            s = getenv(b"EDITOR\0" as *const u8 as *const libc::c_char);
            !s.is_null()
        }
    {
        if !(strrchr(s, '/' as i32)).is_null() {
            s = (strrchr(s, '/' as i32)).offset(1 as libc::c_int as isize);
        }
        if !(strstr(s, b"vi\0" as *const u8 as *const libc::c_char)).is_null() {
            keys = 1 as libc::c_int;
        } else {
            keys = 0 as libc::c_int;
        }
        options_set_number(
            global_s_options,
            b"status-keys\0" as *const u8 as *const libc::c_char,
            keys as libc::c_longlong,
        );
        options_set_number(
            global_w_options,
            b"mode-keys\0" as *const u8 as *const libc::c_char,
            keys as libc::c_longlong,
        );
    }
    if path.is_null() && label.is_null() {
        s = getenv(b"TMUX\0" as *const u8 as *const libc::c_char);
        if !s.is_null() && *s as libc::c_int != '\0' as i32
            && *s as libc::c_int != ',' as i32
        {
            path = xstrdup(s);
            *path
                .offset(
                    strcspn(path, b",\0" as *const u8 as *const libc::c_char) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
    }
    if path.is_null()
        && {
            path = make_label(label, &mut cause);
            path.is_null()
        }
    {
        if !cause.is_null() {
            fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, cause);
            free(cause as *mut libc::c_void);
        }
        exit(1 as libc::c_int);
    }
    socket_path = path;
    free(label as *mut libc::c_void);
    exit(client_main(osdep_event_init(), argc, argv, flags));
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
