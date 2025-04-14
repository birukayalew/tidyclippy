use ::libc;
extern "C" {
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __u_int = libc::c_uint;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xterm_keys_entry {
    pub key: key_code,
    pub template: *const libc::c_char,
}
static mut xterm_keys_table: [xterm_keys_entry; 57] = [
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F1 as libc::c_int as key_code,
            template: b"\x1B[1;_P\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F1 as libc::c_int as key_code,
            template: b"\x1BO1;_P\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F1 as libc::c_int as key_code,
            template: b"\x1BO_P\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F2 as libc::c_int as key_code,
            template: b"\x1B[1;_Q\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F2 as libc::c_int as key_code,
            template: b"\x1BO1;_Q\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F2 as libc::c_int as key_code,
            template: b"\x1BO_Q\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F3 as libc::c_int as key_code,
            template: b"\x1B[1;_R\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F3 as libc::c_int as key_code,
            template: b"\x1BO1;_R\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F3 as libc::c_int as key_code,
            template: b"\x1BO_R\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F4 as libc::c_int as key_code,
            template: b"\x1B[1;_S\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F4 as libc::c_int as key_code,
            template: b"\x1BO1;_S\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F4 as libc::c_int as key_code,
            template: b"\x1BO_S\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F5 as libc::c_int as key_code,
            template: b"\x1B[15;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F6 as libc::c_int as key_code,
            template: b"\x1B[17;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F7 as libc::c_int as key_code,
            template: b"\x1B[18;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F8 as libc::c_int as key_code,
            template: b"\x1B[19;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F9 as libc::c_int as key_code,
            template: b"\x1B[20;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F10 as libc::c_int as key_code,
            template: b"\x1B[21;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F11 as libc::c_int as key_code,
            template: b"\x1B[23;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_F12 as libc::c_int as key_code,
            template: b"\x1B[24;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_UP as libc::c_int as key_code,
            template: b"\x1B[1;_A\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_DOWN as libc::c_int as key_code,
            template: b"\x1B[1;_B\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_RIGHT as libc::c_int as key_code,
            template: b"\x1B[1;_C\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_LEFT as libc::c_int as key_code,
            template: b"\x1B[1;_D\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_HOME as libc::c_int as key_code,
            template: b"\x1B[1;_H\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_END as libc::c_int as key_code,
            template: b"\x1B[1;_F\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_PPAGE as libc::c_int as key_code,
            template: b"\x1B[5;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_NPAGE as libc::c_int as key_code,
            template: b"\x1B[6;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_IC as libc::c_int as key_code,
            template: b"\x1B[2;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: KEYC_DC as libc::c_int as key_code,
            template: b"\x1B[3;_~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '!' as i32 as key_code,
            template: b"\x1B[27;_;33~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '#' as i32 as key_code,
            template: b"\x1B[27;_;35~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '(' as i32 as key_code,
            template: b"\x1B[27;_;40~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: ')' as i32 as key_code,
            template: b"\x1B[27;_;41~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '+' as i32 as key_code,
            template: b"\x1B[27;_;43~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: ',' as i32 as key_code,
            template: b"\x1B[27;_;44~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '-' as i32 as key_code,
            template: b"\x1B[27;_;45~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '.' as i32 as key_code,
            template: b"\x1B[27;_;46~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '0' as i32 as key_code,
            template: b"\x1B[27;_;48~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '1' as i32 as key_code,
            template: b"\x1B[27;_;49~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '2' as i32 as key_code,
            template: b"\x1B[27;_;50~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '3' as i32 as key_code,
            template: b"\x1B[27;_;51~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '4' as i32 as key_code,
            template: b"\x1B[27;_;52~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '5' as i32 as key_code,
            template: b"\x1B[27;_;53~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '6' as i32 as key_code,
            template: b"\x1B[27;_;54~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '7' as i32 as key_code,
            template: b"\x1B[27;_;55~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '8' as i32 as key_code,
            template: b"\x1B[27;_;56~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '9' as i32 as key_code,
            template: b"\x1B[27;_;57~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: ':' as i32 as key_code,
            template: b"\x1B[27;_;58~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: ';' as i32 as key_code,
            template: b"\x1B[27;_;59~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '<' as i32 as key_code,
            template: b"\x1B[27;_;60~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '=' as i32 as key_code,
            template: b"\x1B[27;_;61~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '>' as i32 as key_code,
            template: b"\x1B[27;_;62~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '?' as i32 as key_code,
            template: b"\x1B[27;_;63~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '\'' as i32 as key_code,
            template: b"\x1B[27;_;39~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '\r' as i32 as key_code,
            template: b"\x1B[27;_;13~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = xterm_keys_entry {
            key: '\t' as i32 as key_code,
            template: b"\x1B[27;_;9~\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn xterm_keys_match(
    mut template: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut size: *mut size_t,
    mut modifiers: *mut key_code,
) -> libc::c_int {
    let mut pos: size_t = 0;
    let mut retval: libc::c_int = 0;
    *modifiers = 0 as libc::c_int as key_code;
    if len == 0 as libc::c_int as size_t {
        return 0 as libc::c_int;
    }
    pos = 0 as libc::c_int as size_t;
    loop {
        if *template as libc::c_int == '_' as i32 {
            retval = xterm_keys_modifiers(buf, len, &mut pos, modifiers);
            if retval != 0 as libc::c_int {
                return retval;
            }
        } else {
            if *buf.offset(pos as isize) as libc::c_int != *template as libc::c_int {
                return -(1 as libc::c_int);
            }
            pos = pos.wrapping_add(1);
            pos;
        }
        template = template.offset(1);
        if !(*template as libc::c_int != '\0' as i32 && pos != len) {
            break;
        }
    }
    if *template as libc::c_int != '\0' as i32 {
        return 1 as libc::c_int;
    }
    *size = pos;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xterm_keys_modifiers(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut pos: *mut size_t,
    mut modifiers: *mut key_code,
) -> libc::c_int {
    let mut flags: u_int = 0;
    if len.wrapping_sub(*pos) < 2 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    if (*buf.offset(*pos as isize) as libc::c_int) < '0' as i32
        || *buf.offset(*pos as isize) as libc::c_int > '9' as i32
    {
        return -(1 as libc::c_int);
    }
    let fresh0 = *pos;
    *pos = (*pos).wrapping_add(1);
    flags = (*buf.offset(fresh0 as isize) as libc::c_int - '0' as i32) as u_int;
    if *buf.offset(*pos as isize) as libc::c_int >= '0' as i32
        && *buf.offset(*pos as isize) as libc::c_int <= '9' as i32
    {
        let fresh1 = *pos;
        *pos = (*pos).wrapping_add(1);
        flags = (flags * 10 as libc::c_int as u_int)
            .wrapping_add(
                (*buf.offset(fresh1 as isize) as libc::c_int - '0' as i32) as u_int,
            );
    }
    flags = flags.wrapping_sub(1 as libc::c_int as u_int);
    *modifiers = 0 as libc::c_int as key_code;
    if flags & 1 as libc::c_int as u_int != 0 {
        *modifiers |= 0x800000000000 as libc::c_ulonglong;
    }
    if flags & 2 as libc::c_int as u_int != 0 {
        *modifiers |= 0x200000000000 as libc::c_ulonglong;
    }
    if flags & 4 as libc::c_int as u_int != 0 {
        *modifiers |= 0x400000000000 as libc::c_ulonglong;
    }
    if flags & 8 as libc::c_int as u_int != 0 {
        *modifiers |= 0x200000000000 as libc::c_ulonglong;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xterm_keys_find(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut size: *mut size_t,
    mut key: *mut key_code,
) -> libc::c_int {
    let mut entry: *const xterm_keys_entry = 0 as *const xterm_keys_entry;
    let mut i: u_int = 0;
    let mut matched: libc::c_int = 0;
    let mut modifiers: key_code = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[xterm_keys_entry; 57]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<xterm_keys_entry>() as libc::c_ulong)
    {
        entry = &*xterm_keys_table.as_ptr().offset(i as isize)
            as *const xterm_keys_entry;
        matched = xterm_keys_match((*entry).template, buf, len, size, &mut modifiers);
        if matched == -(1 as libc::c_int) {
            i = i.wrapping_add(1);
            i;
        } else {
            if matched == 0 as libc::c_int {
                *key = (*entry).key | modifiers | 0x1000000000000 as libc::c_ulonglong;
            }
            return matched;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xterm_keys_lookup(mut key: key_code) -> *mut libc::c_char {
    let mut entry: *const xterm_keys_entry = 0 as *const xterm_keys_entry;
    let mut i: u_int = 0;
    let mut modifiers: key_code = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    modifiers = 1 as libc::c_int as key_code;
    if key & 0x800000000000 as libc::c_ulonglong != 0 {
        modifiers = modifiers.wrapping_add(1 as libc::c_int as key_code);
    }
    if key & 0x200000000000 as libc::c_ulonglong != 0 {
        modifiers = modifiers.wrapping_add(2 as libc::c_int as key_code);
    }
    if key & 0x400000000000 as libc::c_ulonglong != 0 {
        modifiers = modifiers.wrapping_add(4 as libc::c_int as key_code);
    }
    if modifiers == 1 as libc::c_int as key_code {
        return 0 as *mut libc::c_char;
    }
    if key & (0x200000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        == 0x200000000000 as libc::c_ulonglong
    {
        return 0 as *mut libc::c_char;
    }
    key
        &= !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong
            | 0x1000000000000 as libc::c_ulonglong);
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[xterm_keys_entry; 57]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<xterm_keys_entry>() as libc::c_ulong)
    {
        entry = &*xterm_keys_table.as_ptr().offset(i as isize)
            as *const xterm_keys_entry;
        if key == (*entry).key {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i as libc::c_ulong
        == (::core::mem::size_of::<[xterm_keys_entry; 57]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<xterm_keys_entry>() as libc::c_ulong)
    {
        return 0 as *mut libc::c_char;
    }
    out = xstrdup((*entry).template);
    *out
        .offset(
            strcspn(out, b"_\0" as *const u8 as *const libc::c_char) as isize,
        ) = ('0' as i32 as key_code).wrapping_add(modifiers) as libc::c_char;
    return out;
}
