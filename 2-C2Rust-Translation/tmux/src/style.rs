use ::libc;
extern "C" {
    pub type options;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn options_get_style(_: *mut options, _: *const libc::c_char) -> *const grid_cell;
    fn colour_tostring(_: libc::c_int) -> *const libc::c_char;
    fn colour_fromstring(s: *const libc::c_char) -> libc::c_int;
    fn attributes_tostring(_: libc::c_int) -> *const libc::c_char;
    fn attributes_fromstring(_: *const libc::c_char) -> libc::c_int;
    static grid_default_cell: grid_cell;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[no_mangle]
pub unsafe extern "C" fn style_parse(
    mut defgc: *const grid_cell,
    mut gc: *mut grid_cell,
    mut in_0: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut savedgc: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let delimiters: [libc::c_char; 3] = *::core::mem::transmute::<
        &[u8; 3],
        &[libc::c_char; 3],
    >(b" ,\0");
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut val: libc::c_int = 0;
    let mut fg: libc::c_int = 0;
    let mut bg: libc::c_int = 0;
    let mut attr: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut end: size_t = 0;
    if *in_0 as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    if !(strchr(
        delimiters.as_ptr(),
        *in_0
            .offset(
                (strlen(in_0)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int,
    ))
        .is_null()
    {
        return -(1 as libc::c_int);
    }
    memcpy(
        &mut savedgc as *mut grid_cell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    fg = (*gc).fg;
    bg = (*gc).bg;
    attr = (*gc).attr as libc::c_int;
    flags = (*gc).flags as libc::c_int;
    loop {
        end = strcspn(in_0, delimiters.as_ptr());
        if end
            > (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            current_block = 16677006778750603458;
            break;
        }
        memcpy(tmp.as_mut_ptr() as *mut libc::c_void, in_0 as *const libc::c_void, end);
        tmp[end as usize] = '\0' as i32 as libc::c_char;
        if strcasecmp(tmp.as_mut_ptr(), b"default\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            fg = (*defgc).fg;
            bg = (*defgc).bg;
            attr = (*defgc).attr as libc::c_int;
            flags = (*defgc).flags as libc::c_int;
        } else if end > 3 as libc::c_int as size_t
            && strncasecmp(
                tmp.as_mut_ptr().offset(1 as libc::c_int as isize),
                b"g=\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            val = colour_fromstring(tmp.as_mut_ptr().offset(3 as libc::c_int as isize));
            if val == -(1 as libc::c_int) {
                current_block = 16677006778750603458;
                break;
            }
            if *in_0 as libc::c_int == 'f' as i32 || *in_0 as libc::c_int == 'F' as i32 {
                if val != 8 as libc::c_int {
                    fg = val;
                } else {
                    fg = (*defgc).fg;
                }
            } else {
                if !(*in_0 as libc::c_int == 'b' as i32
                    || *in_0 as libc::c_int == 'B' as i32)
                {
                    current_block = 16677006778750603458;
                    break;
                }
                if val != 8 as libc::c_int {
                    bg = val;
                } else {
                    bg = (*defgc).bg;
                }
            }
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"none\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            attr = 0 as libc::c_int;
        } else if end > 2 as libc::c_int as size_t
            && strncasecmp(
                tmp.as_mut_ptr(),
                b"no\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            val = attributes_fromstring(
                tmp.as_mut_ptr().offset(2 as libc::c_int as isize),
            );
            if val == -(1 as libc::c_int) {
                current_block = 16677006778750603458;
                break;
            }
            attr &= !val;
        } else {
            val = attributes_fromstring(tmp.as_mut_ptr());
            if val == -(1 as libc::c_int) {
                current_block = 16677006778750603458;
                break;
            }
            attr |= val;
        }
        in_0 = in_0
            .offset(
                end.wrapping_add(strspn(in_0.offset(end as isize), delimiters.as_ptr()))
                    as isize,
            );
        if !(*in_0 as libc::c_int != '\0' as i32) {
            current_block = 7333393191927787629;
            break;
        }
    }
    match current_block {
        16677006778750603458 => {
            memcpy(
                gc as *mut libc::c_void,
                &mut savedgc as *mut grid_cell as *const libc::c_void,
                ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
            );
            return -(1 as libc::c_int);
        }
        _ => {
            (*gc).fg = fg;
            (*gc).bg = bg;
            (*gc).attr = attr as u_short;
            (*gc).flags = flags as u_char;
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn style_tostring(mut gc: *mut grid_cell) -> *const libc::c_char {
    let mut off: libc::c_int = 0 as libc::c_int;
    let mut comma: libc::c_int = 0 as libc::c_int;
    static mut s: [libc::c_char; 256] = [0; 256];
    *s.as_mut_ptr() = '\0' as i32 as libc::c_char;
    if (*gc).fg != 8 as libc::c_int {
        off
            += xsnprintf(
                s.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"fg=%s\0" as *const u8 as *const libc::c_char,
                colour_tostring((*gc).fg),
            );
        comma = 1 as libc::c_int;
    }
    if (*gc).bg != 8 as libc::c_int {
        off
            += xsnprintf(
                s.as_mut_ptr().offset(off as isize),
                (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(off as libc::c_ulong),
                b"%sbg=%s\0" as *const u8 as *const libc::c_char,
                if comma != 0 {
                    b",\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                colour_tostring((*gc).bg),
            );
        comma = 1 as libc::c_int;
    }
    if (*gc).attr as libc::c_int != 0 as libc::c_int
        && (*gc).attr as libc::c_int != 0x80 as libc::c_int
    {
        xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            if comma != 0 {
                b",\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            attributes_tostring((*gc).attr as libc::c_int),
        );
    }
    if *s.as_mut_ptr() as libc::c_int == '\0' as i32 {
        return b"default\0" as *const u8 as *const libc::c_char;
    }
    return s.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn style_apply(
    mut gc: *mut grid_cell,
    mut oo: *mut options,
    mut name: *const libc::c_char,
) {
    let mut gcp: *const grid_cell = 0 as *const grid_cell;
    memcpy(
        gc as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gcp = options_get_style(oo, name);
    (*gc).fg = (*gcp).fg;
    (*gc).bg = (*gcp).bg;
    (*gc).attr = ((*gc).attr as libc::c_int | (*gcp).attr as libc::c_int) as u_short;
}
#[no_mangle]
pub unsafe extern "C" fn style_apply_update(
    mut gc: *mut grid_cell,
    mut oo: *mut options,
    mut name: *const libc::c_char,
) {
    let mut gcp: *const grid_cell = 0 as *const grid_cell;
    gcp = options_get_style(oo, name);
    if (*gcp).fg != 8 as libc::c_int {
        (*gc).fg = (*gcp).fg;
    }
    if (*gcp).bg != 8 as libc::c_int {
        (*gc).bg = (*gcp).bg;
    }
    if (*gcp).attr as libc::c_int != 0 as libc::c_int {
        (*gc).attr = ((*gc).attr as libc::c_int | (*gcp).attr as libc::c_int) as u_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn style_equal(
    mut gc1: *const grid_cell,
    mut gc2: *const grid_cell,
) -> libc::c_int {
    return ((*gc1).fg == (*gc2).fg && (*gc1).bg == (*gc2).bg
        && (*gc1).flags as libc::c_int & !(0x4 as libc::c_int)
            == (*gc2).flags as libc::c_int & !(0x4 as libc::c_int)
        && (*gc1).attr as libc::c_int & !(0x80 as libc::c_int)
            == (*gc2).attr as libc::c_int & !(0x80 as libc::c_int)) as libc::c_int;
}
