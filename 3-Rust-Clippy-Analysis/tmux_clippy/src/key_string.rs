use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_combine(_: *const utf8_data, _: *mut wchar_t) -> utf8_state;
    fn utf8_split(_: wchar_t, _: *mut utf8_data) -> utf8_state;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type key_code = libc::c_ulonglong;
pub type C2RustUnnamed = libc::c_uint;
pub const KEYC_KP_PERIOD: C2RustUnnamed = 268435564;
pub const KEYC_KP_ZERO: C2RustUnnamed = 268435563;
pub const KEYC_KP_ENTER: C2RustUnnamed = 268435562;
pub const KEYC_KP_THREE: C2RustUnnamed = 268435561;
pub const KEYC_KP_TWO: C2RustUnnamed = 268435560;
pub const KEYC_KP_ONE: C2RustUnnamed = 268435559;
pub const KEYC_KP_SIX: C2RustUnnamed = 268435558;
pub const KEYC_KP_FIVE: C2RustUnnamed = 268435557;
pub const KEYC_KP_FOUR: C2RustUnnamed = 268435556;
pub const KEYC_KP_PLUS: C2RustUnnamed = 268435555;
pub const KEYC_KP_NINE: C2RustUnnamed = 268435554;
pub const KEYC_KP_EIGHT: C2RustUnnamed = 268435553;
pub const KEYC_KP_SEVEN: C2RustUnnamed = 268435552;
pub const KEYC_KP_MINUS: C2RustUnnamed = 268435551;
pub const KEYC_KP_STAR: C2RustUnnamed = 268435550;
pub const KEYC_KP_SLASH: C2RustUnnamed = 268435549;
pub const KEYC_RIGHT: C2RustUnnamed = 268435548;
pub const KEYC_LEFT: C2RustUnnamed = 268435547;
pub const KEYC_DOWN: C2RustUnnamed = 268435546;
pub const KEYC_UP: C2RustUnnamed = 268435545;
pub const KEYC_BTAB: C2RustUnnamed = 268435544;
pub const KEYC_PPAGE: C2RustUnnamed = 268435543;
pub const KEYC_NPAGE: C2RustUnnamed = 268435542;
pub const KEYC_END: C2RustUnnamed = 268435541;
pub const KEYC_HOME: C2RustUnnamed = 268435540;
pub const KEYC_DC: C2RustUnnamed = 268435539;
pub const KEYC_IC: C2RustUnnamed = 268435538;
pub const KEYC_F12: C2RustUnnamed = 268435537;
pub const KEYC_F11: C2RustUnnamed = 268435536;
pub const KEYC_F10: C2RustUnnamed = 268435535;
pub const KEYC_F9: C2RustUnnamed = 268435534;
pub const KEYC_F8: C2RustUnnamed = 268435533;
pub const KEYC_F7: C2RustUnnamed = 268435532;
pub const KEYC_F6: C2RustUnnamed = 268435531;
pub const KEYC_F5: C2RustUnnamed = 268435530;
pub const KEYC_F4: C2RustUnnamed = 268435529;
pub const KEYC_F3: C2RustUnnamed = 268435528;
pub const KEYC_F2: C2RustUnnamed = 268435527;
pub const KEYC_F1: C2RustUnnamed = 268435526;
pub const KEYC_BSPACE: C2RustUnnamed = 268435525;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed = 268435524;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed = 268435523;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed = 268435522;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed = 268435521;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed = 268435520;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed = 268435519;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed = 268435518;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed = 268435517;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed = 268435516;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed = 268435515;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed = 268435514;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed = 268435513;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed = 268435512;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed = 268435511;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed = 268435510;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed = 268435509;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed = 268435508;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed = 268435507;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed = 268435506;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed = 268435505;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed = 268435504;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed = 268435503;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed = 268435502;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed = 268435501;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed = 268435500;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed = 268435499;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed = 268435498;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed = 268435497;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed = 268435496;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed = 268435495;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed = 268435494;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed = 268435493;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed = 268435492;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed = 268435491;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed = 268435490;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed = 268435489;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed = 268435488;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed = 268435487;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed = 268435486;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed = 268435485;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed = 268435484;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed = 268435483;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed = 268435482;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed = 268435481;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed = 268435480;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed = 268435479;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed = 268435478;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed = 268435477;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed = 268435476;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed = 268435475;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed = 268435474;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed = 268435473;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed = 268435472;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed = 268435471;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed = 268435470;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed = 268435469;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed = 268435468;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed = 268435467;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed = 268435466;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed = 268435465;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed = 268435464;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed = 268435463;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed = 268435462;
pub const KEYC_DRAGGING: C2RustUnnamed = 268435461;
pub const KEYC_MOUSE: C2RustUnnamed = 268435460;
pub const KEYC_PASTE_END: C2RustUnnamed = 268435459;
pub const KEYC_PASTE_START: C2RustUnnamed = 268435458;
pub const KEYC_FOCUS_OUT: C2RustUnnamed = 268435457;
pub const KEYC_FOCUS_IN: C2RustUnnamed = 268435456;
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub string: *const libc::c_char,
    pub key: key_code,
}
static mut key_string_table: [C2RustUnnamed_0; 108] = [
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F1\0" as *const u8 as *const libc::c_char,
            key: KEYC_F1 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F2\0" as *const u8 as *const libc::c_char,
            key: KEYC_F2 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F3\0" as *const u8 as *const libc::c_char,
            key: KEYC_F3 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F4\0" as *const u8 as *const libc::c_char,
            key: KEYC_F4 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F5\0" as *const u8 as *const libc::c_char,
            key: KEYC_F5 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F6\0" as *const u8 as *const libc::c_char,
            key: KEYC_F6 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F7\0" as *const u8 as *const libc::c_char,
            key: KEYC_F7 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F8\0" as *const u8 as *const libc::c_char,
            key: KEYC_F8 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F9\0" as *const u8 as *const libc::c_char,
            key: KEYC_F9 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F10\0" as *const u8 as *const libc::c_char,
            key: KEYC_F10 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F11\0" as *const u8 as *const libc::c_char,
            key: KEYC_F11 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F12\0" as *const u8 as *const libc::c_char,
            key: KEYC_F12 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"IC\0" as *const u8 as *const libc::c_char,
            key: KEYC_IC as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DC\0" as *const u8 as *const libc::c_char,
            key: KEYC_DC as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Home\0" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"End\0" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"NPage\0" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PageDown\0" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PgDn\0" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PPage\0" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PageUp\0" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PgUp\0" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Tab\0" as *const u8 as *const libc::c_char,
            key: '\t' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"BTab\0" as *const u8 as *const libc::c_char,
            key: KEYC_BTAB as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Space\0" as *const u8 as *const libc::c_char,
            key: ' ' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"BSpace\0" as *const u8 as *const libc::c_char,
            key: KEYC_BSPACE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Enter\0" as *const u8 as *const libc::c_char,
            key: '\r' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Escape\0" as *const u8 as *const libc::c_char,
            key: '\u{1b}' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Up\0" as *const u8 as *const libc::c_char,
            key: KEYC_UP as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Down\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOWN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Left\0" as *const u8 as *const libc::c_char,
            key: KEYC_LEFT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Right\0" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP/\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SLASH as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP*\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_STAR as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP-\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_MINUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP7\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SEVEN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP8\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_EIGHT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP9\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_NINE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP+\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_PLUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP4\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_FOUR as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP5\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_FIVE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP6\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SIX as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP1\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ONE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP2\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_TWO as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP3\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_THREE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KPEnter\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ENTER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP0\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ZERO as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP.\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_PERIOD as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpPane\0" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatus\0" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpBorder\0" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownPane\0" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatus\0" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownBorder\0" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_BORDER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Pane\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_PANE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Status\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_STATUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Border\0" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_BORDER as libc::c_int as key_code,
        };
        init
    },
];
unsafe extern "C" fn key_string_search_table(
    mut string: *const libc::c_char,
) -> key_code {
    let mut i: u_int = 0;
    let mut user: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 108]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        if strcasecmp(string, key_string_table[i as usize].string) == 0 as libc::c_int {
            return key_string_table[i as usize].key;
        }
        i = i.wrapping_add(1);
        i;
    }
    if sscanf(
        string,
        b"User%u\0" as *const u8 as *const libc::c_char,
        &mut user as *mut u_int,
    ) == 1 as libc::c_int && user < 1000 as libc::c_int as u_int
    {
        return (0x20000000 as libc::c_ulonglong).wrapping_add(user as libc::c_ulonglong);
    }
    return 0xfffe00000000 as libc::c_ulonglong;
}
unsafe extern "C" fn key_string_get_modifiers(
    mut string: *mut *const libc::c_char,
) -> key_code {
    let mut modifiers: key_code = 0;
    modifiers = 0 as libc::c_int as key_code;
    while *(*string).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        && *(*string).offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        match *(*string).offset(0 as libc::c_int as isize) as libc::c_int {
            67 | 99 => {
                modifiers |= 0x400000000000 as libc::c_ulonglong;
            }
            77 | 109 => {
                modifiers |= 0x200000000000 as libc::c_ulonglong;
            }
            83 | 115 => {
                modifiers |= 0x800000000000 as libc::c_ulonglong;
            }
            _ => {
                *string = 0 as *const libc::c_char;
                return 0 as libc::c_int as key_code;
            }
        }
        *string = (*string).offset(2 as libc::c_int as isize);
    }
    return modifiers;
}
#[no_mangle]
pub unsafe extern "C" fn key_string_lookup_string(
    mut string: *const libc::c_char,
) -> key_code {
    static mut other: *const libc::c_char = b"!#()+,-.0123456789:;<=>?'\r\t\0"
        as *const u8 as *const libc::c_char;
    let mut key: key_code = 0;
    let mut u: u_int = 0;
    let mut modifiers: key_code = 0;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut i: u_int = 0;
    let mut more: utf8_state = UTF8_MORE;
    let mut wc: wchar_t = 0;
    if strcasecmp(string, b"None\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0xffff00000000 as libc::c_ulonglong;
    }
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && *string.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
    {
        if sscanf(
            string.offset(2 as libc::c_int as isize),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut u as *mut u_int,
        ) != 1 as libc::c_int
        {
            return 0xfffe00000000 as libc::c_ulonglong;
        }
        if u > 0x1fffff as libc::c_int as u_int {
            return 0xfffe00000000 as libc::c_ulonglong;
        }
        return u as key_code;
    }
    modifiers = 0 as libc::c_int as key_code;
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32
        && *string.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        modifiers |= 0x400000000000 as libc::c_ulonglong;
        string = string.offset(1);
        string;
    }
    modifiers |= key_string_get_modifiers(&mut string);
    if string.is_null()
        || *string.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0xfffe00000000 as libc::c_ulonglong;
    }
    if *string.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        && *string.offset(0 as libc::c_int as isize) as u_char as libc::c_int
            <= 127 as libc::c_int
    {
        key = *string.offset(0 as libc::c_int as isize) as u_char as key_code;
        if key < 32 as libc::c_int as key_code || key == 127 as libc::c_int as key_code {
            return 0xfffe00000000 as libc::c_ulonglong;
        }
    } else {
        more = utf8_open(&mut ud, *string as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            if strlen(string) != ud.size as libc::c_ulong {
                return 0xfffe00000000 as libc::c_ulonglong;
            }
            i = 1 as libc::c_int as u_int;
            while i < ud.size as u_int {
                more = utf8_append(&mut ud, *string.offset(i as isize) as u_char);
                i = i.wrapping_add(1);
                i;
            }
            if more as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
                return 0xfffe00000000 as libc::c_ulonglong;
            }
            if utf8_combine(&mut ud, &mut wc) as libc::c_uint
                != UTF8_DONE as libc::c_int as libc::c_uint
            {
                return 0xfffe00000000 as libc::c_ulonglong;
            }
            return wc as key_code | modifiers;
        }
        key = key_string_search_table(string);
        if key == 0xfffe00000000 as libc::c_ulonglong {
            return 0xfffe00000000 as libc::c_ulonglong;
        }
    }
    if key < 0x10000000 as libc::c_ulonglong
        && modifiers & 0x400000000000 as libc::c_ulonglong != 0
        && (strchr(other, key as libc::c_int)).is_null()
    {
        if key >= 97 as libc::c_int as key_code && key <= 122 as libc::c_int as key_code
        {
            key = key.wrapping_sub(96 as libc::c_int as key_code);
        } else if key >= 64 as libc::c_int as key_code
            && key <= 95 as libc::c_int as key_code
        {
            key = key.wrapping_sub(64 as libc::c_int as key_code);
        } else if key == 32 as libc::c_int as key_code {
            key = 0 as libc::c_int as key_code;
        } else if key == 63 as libc::c_int as key_code {
            key = KEYC_BSPACE as libc::c_int as key_code;
        } else {
            return 0xfffe00000000 as libc::c_ulonglong
        }
        modifiers &= !(0x400000000000 as libc::c_ulonglong);
    }
    return key | modifiers;
}
#[no_mangle]
pub unsafe extern "C" fn key_string_lookup_key(
    mut key: key_code,
) -> *const libc::c_char {
    static mut out: [libc::c_char; 32] = [0; 32];
    let mut tmp: [libc::c_char; 8] = [0; 8];
    let mut i: u_int = 0;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut off: size_t = 0;
    *out.as_mut_ptr() = '\0' as i32 as libc::c_char;
    if key == 0xffff00000000 as libc::c_ulonglong {
        return b"None\0" as *const u8 as *const libc::c_char;
    }
    if key == 0xfffe00000000 as libc::c_ulonglong {
        return b"Unknown\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_FOCUS_IN as libc::c_int as key_code {
        return b"FocusIn\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_FOCUS_OUT as libc::c_int as key_code {
        return b"FocusOut\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_PASTE_START as libc::c_int as key_code {
        return b"PasteStart\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_PASTE_END as libc::c_int as key_code {
        return b"PasteEnd\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_MOUSE as libc::c_int as key_code {
        return b"Mouse\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_DRAGGING as libc::c_int as key_code {
        return b"Dragging\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_MOUSEMOVE_PANE as libc::c_int as key_code {
        return b"MouseMovePane\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_MOUSEMOVE_STATUS as libc::c_int as key_code {
        return b"MouseMoveStatus\0" as *const u8 as *const libc::c_char;
    }
    if key == KEYC_MOUSEMOVE_BORDER as libc::c_int as key_code {
        return b"MouseMoveBorder\0" as *const u8 as *const libc::c_char;
    }
    if key >= 0x20000000 as libc::c_ulonglong
        && key
            < (0x20000000 as libc::c_ulonglong)
                .wrapping_add(1000 as libc::c_int as libc::c_ulonglong)
    {
        snprintf(
            out.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"User%u\0" as *const u8 as *const libc::c_char,
            key.wrapping_sub(0x20000000 as libc::c_ulonglong) as u_int,
        );
        return out.as_mut_ptr();
    }
    if key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        == 0 as libc::c_int as libc::c_ulonglong
    {
        key = ' ' as i32 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | key
                & (0x200000000000 as libc::c_ulonglong
                    | 0x400000000000 as libc::c_ulonglong
                    | 0x800000000000 as libc::c_ulonglong
                    | 0x1000000000000 as libc::c_ulonglong);
    }
    if key & 0x400000000000 as libc::c_ulonglong != 0 {
        strlcat(
            out.as_mut_ptr(),
            b"C-\0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    if key & 0x200000000000 as libc::c_ulonglong != 0 {
        strlcat(
            out.as_mut_ptr(),
            b"M-\0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    if key & 0x800000000000 as libc::c_ulonglong != 0 {
        strlcat(
            out.as_mut_ptr(),
            b"S-\0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    key
        &= !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong
            | 0x1000000000000 as libc::c_ulonglong);
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 108]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        if key == key_string_table[i as usize].key {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i as libc::c_ulong
        != (::core::mem::size_of::<[C2RustUnnamed_0; 108]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        strlcat(
            out.as_mut_ptr(),
            key_string_table[i as usize].string,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
        return out.as_mut_ptr();
    }
    if key > 127 as libc::c_int as key_code && key < 0x10000000 as libc::c_ulonglong {
        if utf8_split(key as wchar_t, &mut ud) as libc::c_uint
            == UTF8_DONE as libc::c_int as libc::c_uint
        {
            off = strlen(out.as_mut_ptr());
            memcpy(
                out.as_mut_ptr().offset(off as isize) as *mut libc::c_void,
                (ud.data).as_mut_ptr() as *const libc::c_void,
                ud.size as libc::c_ulong,
            );
            out[off.wrapping_add(ud.size as size_t)
                as usize] = '\0' as i32 as libc::c_char;
            return out.as_mut_ptr();
        }
    }
    if key == 127 as libc::c_int as key_code || key > 255 as libc::c_int as key_code {
        snprintf(
            out.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"Invalid#%llx\0" as *const u8 as *const libc::c_char,
            key,
        );
        return out.as_mut_ptr();
    }
    if key <= 32 as libc::c_int as key_code {
        if key == 0 as libc::c_int as key_code || key > 26 as libc::c_int as key_code {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                b"C-%c\0" as *const u8 as *const libc::c_char,
                (64 as libc::c_int as key_code).wrapping_add(key) as libc::c_int,
            );
        } else {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                b"C-%c\0" as *const u8 as *const libc::c_char,
                (96 as libc::c_int as key_code).wrapping_add(key) as libc::c_int,
            );
        }
    } else if key >= 32 as libc::c_int as key_code
        && key <= 126 as libc::c_int as key_code
    {
        tmp[0 as libc::c_int as usize] = key as libc::c_char;
        tmp[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else if key >= 128 as libc::c_int as key_code {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            b"\\%llo\0" as *const u8 as *const libc::c_char,
            key,
        );
    }
    strlcat(
        out.as_mut_ptr(),
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    return out.as_mut_ptr();
}
