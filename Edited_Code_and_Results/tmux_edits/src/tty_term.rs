use ::libc;
extern "C" {
    pub type options;
    pub type options_entry;
    fn tigetflag(_: *const libc::c_char) -> libc::c_int;
    fn tigetnum(_: *const libc::c_char) -> libc::c_int;
    fn tigetstr(_: *const libc::c_char) -> *mut libc::c_char;
    fn tparm(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut global_options: *mut options;
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_array_get(_: *mut options_entry, _: u_int) -> *const libc::c_char;
    fn options_array_size(_: *mut options_entry, _: *mut u_int) -> libc::c_int;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut cur_term: *mut TERMINAL;
    fn del_curterm(_: *mut TERMINAL) -> libc::c_int;
    fn setupterm(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn strnvis(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: size_t,
        _: libc::c_int,
    ) -> libc::c_int;
    fn strunvis(_: *mut libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termtype {
    pub term_names: *mut libc::c_char,
    pub str_table: *mut libc::c_char,
    pub Booleans: *mut libc::c_char,
    pub Numbers: *mut libc::c_short,
    pub Strings: *mut *mut libc::c_char,
    pub ext_str_table: *mut libc::c_char,
    pub ext_Names: *mut *mut libc::c_char,
    pub num_Booleans: libc::c_ushort,
    pub num_Numbers: libc::c_ushort,
    pub num_Strings: libc::c_ushort,
    pub ext_Booleans: libc::c_ushort,
    pub ext_Numbers: libc::c_ushort,
    pub ext_Strings: libc::c_ushort,
}
pub type TERMTYPE = termtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub type_0: TERMTYPE,
}
pub type TERMINAL = term;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_code {
    pub type_0: tty_code_type,
    pub value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: *mut libc::c_char,
    pub number: libc::c_int,
    pub flag: libc::c_int,
}
pub type tty_code_type = libc::c_uint;
pub const TTYCODE_FLAG: tty_code_type = 3;
pub const TTYCODE_NUMBER: tty_code_type = 2;
pub const TTYCODE_STRING: tty_code_type = 1;
pub const TTYCODE_NONE: tty_code_type = 0;
pub type tty_code_code = libc::c_uint;
pub const TTYC_XT: tty_code_code = 207;
pub const TTYC_XENL: tty_code_code = 206;
pub const TTYC_VPA: tty_code_code = 205;
pub const TTYC_U8: tty_code_code = 204;
pub const TTYC_TSL: tty_code_code = 203;
pub const TTYC_TC: tty_code_code = 202;
pub const TTYC_SS: tty_code_code = 201;
pub const TTYC_SMXX: tty_code_code = 200;
pub const TTYC_SMUL: tty_code_code = 199;
pub const TTYC_SMSO: tty_code_code = 198;
pub const TTYC_SMKX: tty_code_code = 197;
pub const TTYC_SMCUP: tty_code_code = 196;
pub const TTYC_SMACS: tty_code_code = 195;
pub const TTYC_SITM: tty_code_code = 194;
pub const TTYC_SGR0: tty_code_code = 193;
pub const TTYC_SETRGBF: tty_code_code = 192;
pub const TTYC_SETRGBB: tty_code_code = 191;
pub const TTYC_SETAF: tty_code_code = 190;
pub const TTYC_SETAB: tty_code_code = 189;
pub const TTYC_SE: tty_code_code = 188;
pub const TTYC_RMKX: tty_code_code = 187;
pub const TTYC_RMCUP: tty_code_code = 186;
pub const TTYC_RMACS: tty_code_code = 185;
pub const TTYC_RI: tty_code_code = 184;
pub const TTYC_RGB: tty_code_code = 183;
pub const TTYC_REV: tty_code_code = 182;
pub const TTYC_OP: tty_code_code = 181;
pub const TTYC_MS: tty_code_code = 180;
pub const TTYC_KUP7: tty_code_code = 179;
pub const TTYC_KUP6: tty_code_code = 178;
pub const TTYC_KUP5: tty_code_code = 177;
pub const TTYC_KUP4: tty_code_code = 176;
pub const TTYC_KUP3: tty_code_code = 175;
pub const TTYC_KUP2: tty_code_code = 174;
pub const TTYC_KRIT7: tty_code_code = 173;
pub const TTYC_KRIT6: tty_code_code = 172;
pub const TTYC_KRIT5: tty_code_code = 171;
pub const TTYC_KRIT4: tty_code_code = 170;
pub const TTYC_KRIT3: tty_code_code = 169;
pub const TTYC_KRIT2: tty_code_code = 168;
pub const TTYC_KRI: tty_code_code = 167;
pub const TTYC_KPRV7: tty_code_code = 166;
pub const TTYC_KPRV6: tty_code_code = 165;
pub const TTYC_KPRV5: tty_code_code = 164;
pub const TTYC_KPRV4: tty_code_code = 163;
pub const TTYC_KPRV3: tty_code_code = 162;
pub const TTYC_KPRV2: tty_code_code = 161;
pub const TTYC_KPP: tty_code_code = 160;
pub const TTYC_KNXT7: tty_code_code = 159;
pub const TTYC_KNXT6: tty_code_code = 158;
pub const TTYC_KNXT5: tty_code_code = 157;
pub const TTYC_KNXT4: tty_code_code = 156;
pub const TTYC_KNXT3: tty_code_code = 155;
pub const TTYC_KNXT2: tty_code_code = 154;
pub const TTYC_KNP: tty_code_code = 153;
pub const TTYC_KMOUS: tty_code_code = 152;
pub const TTYC_KLFT7: tty_code_code = 151;
pub const TTYC_KLFT6: tty_code_code = 150;
pub const TTYC_KLFT5: tty_code_code = 149;
pub const TTYC_KLFT4: tty_code_code = 148;
pub const TTYC_KLFT3: tty_code_code = 147;
pub const TTYC_KLFT2: tty_code_code = 146;
pub const TTYC_KIND: tty_code_code = 145;
pub const TTYC_KICH1: tty_code_code = 144;
pub const TTYC_KIC7: tty_code_code = 143;
pub const TTYC_KIC6: tty_code_code = 142;
pub const TTYC_KIC5: tty_code_code = 141;
pub const TTYC_KIC4: tty_code_code = 140;
pub const TTYC_KIC3: tty_code_code = 139;
pub const TTYC_KIC2: tty_code_code = 138;
pub const TTYC_KHOME: tty_code_code = 137;
pub const TTYC_KHOM7: tty_code_code = 136;
pub const TTYC_KHOM6: tty_code_code = 135;
pub const TTYC_KHOM5: tty_code_code = 134;
pub const TTYC_KHOM4: tty_code_code = 133;
pub const TTYC_KHOM3: tty_code_code = 132;
pub const TTYC_KHOM2: tty_code_code = 131;
pub const TTYC_KF9: tty_code_code = 130;
pub const TTYC_KF8: tty_code_code = 129;
pub const TTYC_KF7: tty_code_code = 128;
pub const TTYC_KF63: tty_code_code = 127;
pub const TTYC_KF62: tty_code_code = 126;
pub const TTYC_KF61: tty_code_code = 125;
pub const TTYC_KF60: tty_code_code = 124;
pub const TTYC_KF6: tty_code_code = 123;
pub const TTYC_KF59: tty_code_code = 122;
pub const TTYC_KF58: tty_code_code = 121;
pub const TTYC_KF57: tty_code_code = 120;
pub const TTYC_KF56: tty_code_code = 119;
pub const TTYC_KF55: tty_code_code = 118;
pub const TTYC_KF54: tty_code_code = 117;
pub const TTYC_KF53: tty_code_code = 116;
pub const TTYC_KF52: tty_code_code = 115;
pub const TTYC_KF51: tty_code_code = 114;
pub const TTYC_KF50: tty_code_code = 113;
pub const TTYC_KF5: tty_code_code = 112;
pub const TTYC_KF49: tty_code_code = 111;
pub const TTYC_KF48: tty_code_code = 110;
pub const TTYC_KF47: tty_code_code = 109;
pub const TTYC_KF46: tty_code_code = 108;
pub const TTYC_KF45: tty_code_code = 107;
pub const TTYC_KF44: tty_code_code = 106;
pub const TTYC_KF43: tty_code_code = 105;
pub const TTYC_KF42: tty_code_code = 104;
pub const TTYC_KF41: tty_code_code = 103;
pub const TTYC_KF40: tty_code_code = 102;
pub const TTYC_KF4: tty_code_code = 101;
pub const TTYC_KF39: tty_code_code = 100;
pub const TTYC_KF38: tty_code_code = 99;
pub const TTYC_KF37: tty_code_code = 98;
pub const TTYC_KF36: tty_code_code = 97;
pub const TTYC_KF35: tty_code_code = 96;
pub const TTYC_KF34: tty_code_code = 95;
pub const TTYC_KF33: tty_code_code = 94;
pub const TTYC_KF32: tty_code_code = 93;
pub const TTYC_KF31: tty_code_code = 92;
pub const TTYC_KF30: tty_code_code = 91;
pub const TTYC_KF3: tty_code_code = 90;
pub const TTYC_KF29: tty_code_code = 89;
pub const TTYC_KF28: tty_code_code = 88;
pub const TTYC_KF27: tty_code_code = 87;
pub const TTYC_KF26: tty_code_code = 86;
pub const TTYC_KF25: tty_code_code = 85;
pub const TTYC_KF24: tty_code_code = 84;
pub const TTYC_KF23: tty_code_code = 83;
pub const TTYC_KF22: tty_code_code = 82;
pub const TTYC_KF21: tty_code_code = 81;
pub const TTYC_KF20: tty_code_code = 80;
pub const TTYC_KF2: tty_code_code = 79;
pub const TTYC_KF19: tty_code_code = 78;
pub const TTYC_KF18: tty_code_code = 77;
pub const TTYC_KF17: tty_code_code = 76;
pub const TTYC_KF16: tty_code_code = 75;
pub const TTYC_KF15: tty_code_code = 74;
pub const TTYC_KF14: tty_code_code = 73;
pub const TTYC_KF13: tty_code_code = 72;
pub const TTYC_KF12: tty_code_code = 71;
pub const TTYC_KF11: tty_code_code = 70;
pub const TTYC_KF10: tty_code_code = 69;
pub const TTYC_KF1: tty_code_code = 68;
pub const TTYC_KEND7: tty_code_code = 67;
pub const TTYC_KEND6: tty_code_code = 66;
pub const TTYC_KEND5: tty_code_code = 65;
pub const TTYC_KEND4: tty_code_code = 64;
pub const TTYC_KEND3: tty_code_code = 63;
pub const TTYC_KEND2: tty_code_code = 62;
pub const TTYC_KEND: tty_code_code = 61;
pub const TTYC_KDN7: tty_code_code = 60;
pub const TTYC_KDN6: tty_code_code = 59;
pub const TTYC_KDN5: tty_code_code = 58;
pub const TTYC_KDN4: tty_code_code = 57;
pub const TTYC_KDN3: tty_code_code = 56;
pub const TTYC_KDN2: tty_code_code = 55;
pub const TTYC_KDCH1: tty_code_code = 54;
pub const TTYC_KDC7: tty_code_code = 53;
pub const TTYC_KDC6: tty_code_code = 52;
pub const TTYC_KDC5: tty_code_code = 51;
pub const TTYC_KDC4: tty_code_code = 50;
pub const TTYC_KDC3: tty_code_code = 49;
pub const TTYC_KDC2: tty_code_code = 48;
pub const TTYC_KCUU1: tty_code_code = 47;
pub const TTYC_KCUF1: tty_code_code = 46;
pub const TTYC_KCUD1: tty_code_code = 45;
pub const TTYC_KCUB1: tty_code_code = 44;
pub const TTYC_KCBT: tty_code_code = 43;
pub const TTYC_INVIS: tty_code_code = 42;
pub const TTYC_INDN: tty_code_code = 41;
pub const TTYC_IL1: tty_code_code = 40;
pub const TTYC_IL: tty_code_code = 39;
pub const TTYC_ICH1: tty_code_code = 38;
pub const TTYC_ICH: tty_code_code = 37;
pub const TTYC_HPA: tty_code_code = 36;
pub const TTYC_HOME: tty_code_code = 35;
pub const TTYC_FSL: tty_code_code = 34;
pub const TTYC_ENACS: tty_code_code = 33;
pub const TTYC_EL1: tty_code_code = 32;
pub const TTYC_EL: tty_code_code = 31;
pub const TTYC_ED: tty_code_code = 30;
pub const TTYC_ECH: tty_code_code = 29;
pub const TTYC_E3: tty_code_code = 28;
pub const TTYC_DL1: tty_code_code = 27;
pub const TTYC_DL: tty_code_code = 26;
pub const TTYC_DIM: tty_code_code = 25;
pub const TTYC_DCH1: tty_code_code = 24;
pub const TTYC_DCH: tty_code_code = 23;
pub const TTYC_CVVIS: tty_code_code = 22;
pub const TTYC_CUU1: tty_code_code = 21;
pub const TTYC_CUU: tty_code_code = 20;
pub const TTYC_CUP: tty_code_code = 19;
pub const TTYC_CUF1: tty_code_code = 18;
pub const TTYC_CUF: tty_code_code = 17;
pub const TTYC_CUD1: tty_code_code = 16;
pub const TTYC_CUD: tty_code_code = 15;
pub const TTYC_CUB1: tty_code_code = 14;
pub const TTYC_CUB: tty_code_code = 13;
pub const TTYC_CSR: tty_code_code = 12;
pub const TTYC_CS: tty_code_code = 11;
pub const TTYC_CR: tty_code_code = 10;
pub const TTYC_COLORS: tty_code_code = 9;
pub const TTYC_CNORM: tty_code_code = 8;
pub const TTYC_CLEAR: tty_code_code = 7;
pub const TTYC_CIVIS: tty_code_code = 6;
pub const TTYC_BOLD: tty_code_code = 5;
pub const TTYC_BLINK: tty_code_code = 4;
pub const TTYC_BEL: tty_code_code = 3;
pub const TTYC_BCE: tty_code_code = 2;
pub const TTYC_ACSC: tty_code_code = 1;
pub const TTYC_AX: tty_code_code = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_terms {
    pub lh_first: *mut tty_term,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term_code_entry {
    pub type_0: tty_code_type,
    pub name: *const libc::c_char,
}
#[no_mangle]
pub static mut tty_terms: tty_terms = {
    let mut init = tty_terms {
        lh_first: 0 as *const tty_term as *mut tty_term,
    };
    init
};
static mut tty_term_codes: [tty_term_code_entry; 208] = [
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"AX\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"acsc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"bce\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"bel\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"blink\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"bold\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"civis\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"clear\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cnorm\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_NUMBER,
            name: b"colors\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Cr\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Cs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"csr\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cub\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cub1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cud\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cud1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuf1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuu\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuu1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cvvis\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dch\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dch1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dim\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dl1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"E3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ech\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ed\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"el\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"el1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"enacs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"fsl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"home\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"hpa\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ich\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ich1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"il\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"il1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"indn\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"invis\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcbt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcub1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcud1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcuf1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcuu1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kdch1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kend\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf13\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf14\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf15\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf16\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf17\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf18\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf19\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf20\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf21\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf22\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf23\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf24\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf25\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf26\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf27\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf28\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf29\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf30\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf31\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf32\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf33\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf34\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf35\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf36\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf37\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf38\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf39\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf40\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf41\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf42\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf43\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf44\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf45\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf46\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf47\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf48\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf49\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf50\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf51\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf52\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf53\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf54\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf55\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf56\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf57\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf58\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf59\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf60\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf61\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf62\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf63\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"khome\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kich1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kind\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kmous\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"knp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kpp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kri\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Ms\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"op\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rev\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"RGB\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ri\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rmacs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rmcup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rmkx\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Se\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setab\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setaf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setrgbb\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setrgbf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"sgr0\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"sitm\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smacs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smcup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smkx\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smso\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smul\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smxx\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Ss\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"Tc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"tsl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_NUMBER,
            name: b"U8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"vpa\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"xenl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"XT\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn tty_term_ncodes() -> u_int {
    return (::core::mem::size_of::<[tty_term_code_entry; 208]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<tty_term_code_entry>() as libc::c_ulong)
        as u_int;
}
unsafe extern "C" fn tty_term_strip(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    static mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut len: size_t = 0;
    if (strchr(s, '$' as i32)).is_null() {
        return xstrdup(s);
    }
    len = 0 as libc::c_int as size_t;
    ptr = s;
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr as libc::c_int == '$' as i32
            && *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '<' as i32
        {
            while *ptr as libc::c_int != '\0' as i32 && *ptr as libc::c_int != '>' as i32
            {
                ptr = ptr.offset(1);
                ptr;
            }
            if *ptr as libc::c_int == '>' as i32 {
                ptr = ptr.offset(1);
                ptr;
            }
        }
        let fresh0 = len;
        len = len.wrapping_add(1);
        buf[fresh0 as usize] = *ptr;
        if len
            == (::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    buf[len as usize] = '\0' as i32 as libc::c_char;
    return xstrdup(buf.as_mut_ptr());
}
unsafe extern "C" fn tty_term_override(
    mut term: *mut tty_term,
    mut override_0: *const libc::c_char,
) {
    let mut ent: *const tty_term_code_entry = 0 as *const tty_term_code_entry;
    let mut code: *mut tty_code = 0 as *mut tty_code;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    let mut remove: libc::c_int = 0;
    next = xstrdup(override_0);
    copy = next;
    s = strsep(&mut next, b":\0" as *const u8 as *const libc::c_char);
    if s.is_null() || next.is_null()
        || fnmatch(s, (*term).name, 0 as libc::c_int) != 0 as libc::c_int
    {
        free(copy as *mut libc::c_void);
        return;
    }
    loop {
        s = strsep(&mut next, b":\0" as *const u8 as *const libc::c_char);
        if s.is_null() {
            break;
        }
        if *s as libc::c_int == '\0' as i32 {
            continue;
        }
        value = 0 as *mut libc::c_char;
        remove = 0 as libc::c_int;
        cp = strchr(s, '=' as i32);
        if !cp.is_null() {
            let fresh1 = cp;
            cp = cp.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
            value = xstrdup(cp);
            if strunvis(value, cp) == -(1 as libc::c_int) {
                free(value as *mut libc::c_void);
                value = xstrdup(cp);
            }
        } else if *s
            .offset((strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '@' as i32
        {
            *s
                .offset(
                    (strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            remove = 1 as libc::c_int;
        } else {
            value = xstrdup(b"\0" as *const u8 as *const libc::c_char);
        }
        if remove != 0 {
            log_debug(
                b"%s override: %s@\0" as *const u8 as *const libc::c_char,
                (*term).name,
                s,
            );
        } else {
            log_debug(
                b"%s override: %s=%s\0" as *const u8 as *const libc::c_char,
                (*term).name,
                s,
                value,
            );
        }
        i = 0 as libc::c_int as u_int;
        while i < tty_term_ncodes() {
            ent = &*tty_term_codes.as_ptr().offset(i as isize)
                as *const tty_term_code_entry;
            if !(strcmp(s, (*ent).name) != 0 as libc::c_int) {
                code = &mut *((*term).codes).offset(i as isize) as *mut tty_code;
                if remove != 0 {
                    (*code).type_0 = TTYCODE_NONE;
                } else {
                    match (*ent).type_0 as libc::c_uint {
                        1 => {
                            if (*code).type_0 as libc::c_uint
                                == TTYCODE_STRING as libc::c_int as libc::c_uint
                            {
                                free((*code).value.string as *mut libc::c_void);
                            }
                            (*code).value.string = xstrdup(value);
                            (*code).type_0 = (*ent).type_0;
                        }
                        2 => {
                            n = strtonum(
                                value,
                                0 as libc::c_int as libc::c_longlong,
                                2147483647 as libc::c_int as libc::c_longlong,
                                &mut errstr,
                            ) as libc::c_int;
                            if errstr.is_null() {
                                (*code).value.number = n;
                                (*code).type_0 = (*ent).type_0;
                            }
                        }
                        3 => {
                            (*code).value.flag = 1 as libc::c_int;
                            (*code).type_0 = (*ent).type_0;
                        }
                        0 | _ => {}
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        free(value as *mut libc::c_void);
    }
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_find(
    mut name: *mut libc::c_char,
    mut fd: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut tty_term {
    let mut term: *mut tty_term = 0 as *mut tty_term;
    let mut ent: *const tty_term_code_entry = 0 as *const tty_term_code_entry;
    let mut code: *mut tty_code = 0 as *mut tty_code;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut size: u_int = 0;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut acs: *const libc::c_char = 0 as *const libc::c_char;
    term = tty_terms.lh_first;
    while !term.is_null() {
        if strcmp((*term).name, name) == 0 as libc::c_int {
            (*term).references = ((*term).references).wrapping_add(1);
            (*term).references;
            return term;
        }
        term = (*term).entry.le_next;
    }
    log_debug(b"new term: %s\0" as *const u8 as *const libc::c_char, name);
    term = xmalloc(::core::mem::size_of::<tty_term>() as libc::c_ulong) as *mut tty_term;
    (*term).name = xstrdup(name);
    (*term).references = 1 as libc::c_int as u_int;
    (*term).flags = 0 as libc::c_int;
    (*term)
        .codes = xcalloc(
        tty_term_ncodes() as size_t,
        ::core::mem::size_of::<tty_code>() as libc::c_ulong,
    ) as *mut tty_code;
    (*term).entry.le_next = tty_terms.lh_first;
    if !((*term).entry.le_next).is_null() {
        (*tty_terms.lh_first).entry.le_prev = &mut (*term).entry.le_next;
    }
    tty_terms.lh_first = term;
    (*term).entry.le_prev = &mut tty_terms.lh_first;
    if setupterm(name, fd, &mut error) != 0 as libc::c_int {
        match error {
            1 => {
                xasprintf(
                    cause,
                    b"can't use hardcopy terminal: %s\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
            0 => {
                xasprintf(
                    cause,
                    b"missing or unsuitable terminal: %s\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
            -1 => {
                xasprintf(
                    cause,
                    b"can't find terminfo database\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                xasprintf(cause, b"unknown error\0" as *const u8 as *const libc::c_char);
            }
        }
    } else {
        i = 0 as libc::c_int as u_int;
        while i < tty_term_ncodes() {
            ent = &*tty_term_codes.as_ptr().offset(i as isize)
                as *const tty_term_code_entry;
            code = &mut *((*term).codes).offset(i as isize) as *mut tty_code;
            (*code).type_0 = TTYCODE_NONE;
            match (*ent).type_0 as libc::c_uint {
                1 => {
                    s = tigetstr((*ent).name as *mut libc::c_char);
                    if !(s.is_null()
                        || s
                            == -(1 as libc::c_int) as *mut libc::c_char
                                as *const libc::c_char)
                    {
                        (*code).type_0 = TTYCODE_STRING;
                        (*code).value.string = tty_term_strip(s);
                    }
                }
                2 => {
                    n = tigetnum((*ent).name as *mut libc::c_char);
                    if !(n == -(1 as libc::c_int) || n == -(2 as libc::c_int)) {
                        (*code).type_0 = TTYCODE_NUMBER;
                        (*code).value.number = n;
                    }
                }
                3 => {
                    n = tigetflag((*ent).name as *mut libc::c_char);
                    if !(n == -(1 as libc::c_int)) {
                        (*code).type_0 = TTYCODE_FLAG;
                        (*code).value.flag = n;
                    }
                }
                0 | _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        o = options_get_only(
            global_options,
            b"terminal-overrides\0" as *const u8 as *const libc::c_char,
        );
        if options_array_size(o, &mut size) != -(1 as libc::c_int) {
            i = 0 as libc::c_int as u_int;
            while i < size {
                s = options_array_get(o, i);
                if !s.is_null() {
                    tty_term_override(term, s);
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        del_curterm(cur_term);
        if tty_term_has(term, TTYC_CLEAR) == 0 {
            xasprintf(
                cause,
                b"terminal does not support clear\0" as *const u8 as *const libc::c_char,
            );
        } else if tty_term_has(term, TTYC_CUP) == 0 {
            xasprintf(
                cause,
                b"terminal does not support cup\0" as *const u8 as *const libc::c_char,
            );
        } else if tty_term_has(term, TTYC_CUD1) == 0 && tty_term_has(term, TTYC_CUD) == 0
        {
            xasprintf(
                cause,
                b"terminal does not support cud1 or cud\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            if tty_term_number(term, TTYC_COLORS) == 256 as libc::c_int {
                (*term).flags |= 0x1 as libc::c_int;
            }
            if tty_term_flag(term, TTYC_XENL) == 0 {
                (*term).flags |= 0x2 as libc::c_int;
            }
            memset(
                ((*term).acs).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[[libc::c_char; 2]; 256]>() as libc::c_ulong,
            );
            if tty_term_has(term, TTYC_ACSC) != 0 {
                acs = tty_term_string(term, TTYC_ACSC);
            } else {
                acs = b"a#j+k+l+m+n+o-p-q-r-s-t+u+v+w+x|y<z>~.\0" as *const u8
                    as *const libc::c_char;
            }
            while *acs.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
                && *acs.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
            {
                (*term)
                    .acs[*acs.offset(0 as libc::c_int as isize) as u_char
                    as usize][0 as libc::c_int
                    as usize] = *acs.offset(1 as libc::c_int as isize);
                acs = acs.offset(2 as libc::c_int as isize);
            }
            if tty_term_flag(term, TTYC_XT) != 0 && tty_term_has(term, TTYC_TSL) == 0
                && tty_term_has(term, TTYC_FSL) == 0
            {
                code = &mut *((*term).codes).offset(TTYC_TSL as libc::c_int as isize)
                    as *mut tty_code;
                (*code)
                    .value
                    .string = xstrdup(b"\x1B]0;\0" as *const u8 as *const libc::c_char);
                (*code).type_0 = TTYCODE_STRING;
                code = &mut *((*term).codes).offset(TTYC_FSL as libc::c_int as isize)
                    as *mut tty_code;
                (*code)
                    .value
                    .string = xstrdup(b"\x07\0" as *const u8 as *const libc::c_char);
                (*code).type_0 = TTYCODE_STRING;
            }
            if (tty_term_flag(term, TTYC_TC) != 0 || tty_term_flag(term, TTYC_RGB) != 0)
                && tty_term_has(term, TTYC_SETRGBF) == 0
                && tty_term_has(term, TTYC_SETRGBB) == 0
            {
                code = &mut *((*term).codes).offset(TTYC_SETRGBF as libc::c_int as isize)
                    as *mut tty_code;
                (*code)
                    .value
                    .string = xstrdup(
                    b"\x1B[38;2;%p1%d;%p2%d;%p3%dm\0" as *const u8 as *const libc::c_char,
                );
                (*code).type_0 = TTYCODE_STRING;
                code = &mut *((*term).codes).offset(TTYC_SETRGBB as libc::c_int as isize)
                    as *mut tty_code;
                (*code)
                    .value
                    .string = xstrdup(
                    b"\x1B[48;2;%p1%d;%p2%d;%p3%dm\0" as *const u8 as *const libc::c_char,
                );
                (*code).type_0 = TTYCODE_STRING;
            }
            i = 0 as libc::c_int as u_int;
            while i < tty_term_ncodes() {
                log_debug(
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    name,
                    tty_term_describe(term, i as tty_code_code),
                );
                i = i.wrapping_add(1);
                i;
            }
            return term;
        }
    }
    tty_term_free(term);
    return 0 as *mut tty_term;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_free(mut term: *mut tty_term) {
    let mut i: u_int = 0;
    (*term).references = ((*term).references).wrapping_sub(1);
    if (*term).references != 0 as libc::c_int as u_int {
        return;
    }
    if !((*term).entry.le_next).is_null() {
        (*(*term).entry.le_next).entry.le_prev = (*term).entry.le_prev;
    }
    *(*term).entry.le_prev = (*term).entry.le_next;
    i = 0 as libc::c_int as u_int;
    while i < tty_term_ncodes() {
        if (*((*term).codes).offset(i as isize)).type_0 as libc::c_uint
            == TTYCODE_STRING as libc::c_int as libc::c_uint
        {
            free(
                (*((*term).codes).offset(i as isize)).value.string as *mut libc::c_void,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    free((*term).codes as *mut libc::c_void);
    free((*term).name as *mut libc::c_void);
    free(term as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_has(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> libc::c_int {
    return ((*((*term).codes).offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_NONE as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> *const libc::c_char {
    if tty_term_has(term, code) == 0 {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    if (*((*term).codes).offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_STRING as libc::c_int as libc::c_uint
    {
        fatalx(
            b"not a string: %d\0" as *const u8 as *const libc::c_char,
            code as libc::c_uint,
        );
    }
    return (*((*term).codes).offset(code as isize)).value.string;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string1(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: libc::c_int,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string2(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        b,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string3(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        b,
        c,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_ptr1(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: *const libc::c_void,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_ptr2(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        b,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_number(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> libc::c_int {
    if tty_term_has(term, code) == 0 {
        return 0 as libc::c_int;
    }
    if (*((*term).codes).offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_NUMBER as libc::c_int as libc::c_uint
    {
        fatalx(
            b"not a number: %d\0" as *const u8 as *const libc::c_char,
            code as libc::c_uint,
        );
    }
    return (*((*term).codes).offset(code as isize)).value.number;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_flag(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> libc::c_int {
    if tty_term_has(term, code) == 0 {
        return 0 as libc::c_int;
    }
    if (*((*term).codes).offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_FLAG as libc::c_int as libc::c_uint
    {
        fatalx(
            b"not a flag: %d\0" as *const u8 as *const libc::c_char,
            code as libc::c_uint,
        );
    }
    return (*((*term).codes).offset(code as isize)).value.flag;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_describe(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = [0; 256];
    let mut out: [libc::c_char; 128] = [0; 128];
    match (*((*term).codes).offset(code as isize)).type_0 as libc::c_uint {
        0 => {
            xsnprintf(
                s.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: [missing]\0" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
            );
        }
        1 => {
            strnvis(
                out.as_mut_ptr(),
                (*((*term).codes).offset(code as isize)).value.string,
                ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int,
            );
            xsnprintf(
                s.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: (string) %s\0" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
                out.as_mut_ptr(),
            );
        }
        2 => {
            xsnprintf(
                s.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: (number) %d\0" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
                (*((*term).codes).offset(code as isize)).value.number,
            );
        }
        3 => {
            xsnprintf(
                s.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: (flag) %s\0" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
                if (*((*term).codes).offset(code as isize)).value.flag != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        _ => {}
    }
    return s.as_mut_ptr();
}
