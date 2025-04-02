use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type u_char = __u_char;
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn colour_dist_sq(
    mut R: libc::c_int,
    mut G: libc::c_int,
    mut B: libc::c_int,
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return (R - r) * (R - r) + (G - g) * (G - g) + (B - b) * (B - b);
}
unsafe extern "C" fn colour_to_6cube(mut v: libc::c_int) -> libc::c_int {
    if v < 48 as libc::c_int {
        return 0 as libc::c_int;
    }
    if v < 114 as libc::c_int {
        return 1 as libc::c_int;
    }
    return (v - 35 as libc::c_int) / 40 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn colour_find_rgb(
    mut r: u_char,
    mut g: u_char,
    mut b: u_char,
) -> libc::c_int {
    static mut q2c: [libc::c_int; 6] = [
        0 as libc::c_int,
        0x5f as libc::c_int,
        0x87 as libc::c_int,
        0xaf as libc::c_int,
        0xd7 as libc::c_int,
        0xff as libc::c_int,
    ];
    let mut qr: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    let mut qb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut cg: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut grey_avg: libc::c_int = 0;
    let mut grey_idx: libc::c_int = 0;
    let mut grey: libc::c_int = 0;
    qr = colour_to_6cube(r as libc::c_int);
    cr = q2c[qr as usize];
    qg = colour_to_6cube(g as libc::c_int);
    cg = q2c[qg as usize];
    qb = colour_to_6cube(b as libc::c_int);
    cb = q2c[qb as usize];
    if cr == r as libc::c_int && cg == g as libc::c_int && cb == b as libc::c_int {
        return 16 as libc::c_int + 36 as libc::c_int * qr + 6 as libc::c_int * qg + qb
            | 0x1000000 as libc::c_int;
    }
    grey_avg = (r as libc::c_int + g as libc::c_int + b as libc::c_int)
        / 3 as libc::c_int;
    if grey_avg > 238 as libc::c_int {
        grey_idx = 23 as libc::c_int;
    } else {
        grey_idx = (grey_avg - 3 as libc::c_int) / 10 as libc::c_int;
    }
    grey = 8 as libc::c_int + 10 as libc::c_int * grey_idx;
    d = colour_dist_sq(cr, cg, cb, r as libc::c_int, g as libc::c_int, b as libc::c_int);
    if colour_dist_sq(
        grey,
        grey,
        grey,
        r as libc::c_int,
        g as libc::c_int,
        b as libc::c_int,
    ) < d
    {
        idx = 232 as libc::c_int + grey_idx;
    } else {
        idx = 16 as libc::c_int + 36 as libc::c_int * qr + 6 as libc::c_int * qg + qb;
    }
    return idx | 0x1000000 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn colour_join_rgb(
    mut r: u_char,
    mut g: u_char,
    mut b: u_char,
) -> libc::c_int {
    return (r as libc::c_int & 0xff as libc::c_int) << 16 as libc::c_int
        | (g as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
        | b as libc::c_int & 0xff as libc::c_int | 0x2000000 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn colour_split_rgb(
    mut c: libc::c_int,
    mut r: *mut u_char,
    mut g: *mut u_char,
    mut b: *mut u_char,
) {
    *r = (c >> 16 as libc::c_int & 0xff as libc::c_int) as u_char;
    *g = (c >> 8 as libc::c_int & 0xff as libc::c_int) as u_char;
    *b = (c & 0xff as libc::c_int) as u_char;
}
#[no_mangle]
pub unsafe extern "C" fn colour_tostring(mut c: libc::c_int) -> *const libc::c_char {
    static mut s: [libc::c_char; 32] = [0; 32];
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if c & 0x2000000 as libc::c_int != 0 {
        colour_split_rgb(c, &mut r, &mut g, &mut b);
        xsnprintf(
            s.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
            r as libc::c_int,
            g as libc::c_int,
            b as libc::c_int,
        );
        return s.as_mut_ptr();
    }
    if c & 0x1000000 as libc::c_int != 0 {
        xsnprintf(
            s.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"colour%u\0" as *const u8 as *const libc::c_char,
            c & 0xff as libc::c_int,
        );
        return s.as_mut_ptr();
    }
    match c {
        0 => return b"black\0" as *const u8 as *const libc::c_char,
        1 => return b"red\0" as *const u8 as *const libc::c_char,
        2 => return b"green\0" as *const u8 as *const libc::c_char,
        3 => return b"yellow\0" as *const u8 as *const libc::c_char,
        4 => return b"blue\0" as *const u8 as *const libc::c_char,
        5 => return b"magenta\0" as *const u8 as *const libc::c_char,
        6 => return b"cyan\0" as *const u8 as *const libc::c_char,
        7 => return b"white\0" as *const u8 as *const libc::c_char,
        8 => return b"default\0" as *const u8 as *const libc::c_char,
        90 => return b"brightblack\0" as *const u8 as *const libc::c_char,
        91 => return b"brightred\0" as *const u8 as *const libc::c_char,
        92 => return b"brightgreen\0" as *const u8 as *const libc::c_char,
        93 => return b"brightyellow\0" as *const u8 as *const libc::c_char,
        94 => return b"brightblue\0" as *const u8 as *const libc::c_char,
        95 => return b"brightmagenta\0" as *const u8 as *const libc::c_char,
        96 => return b"brightcyan\0" as *const u8 as *const libc::c_char,
        97 => return b"brightwhite\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn colour_fromstring(mut s: *const libc::c_char) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if *s as libc::c_int == '#' as i32 && strlen(s) == 7 as libc::c_int as libc::c_ulong
    {
        cp = s.offset(1 as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*cp as u_char as libc::c_int as isize)
            as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            cp = cp.offset(1);
            cp;
        }
        if *cp as libc::c_int != '\0' as i32 {
            return -(1 as libc::c_int);
        }
        n = sscanf(
            s.offset(1 as libc::c_int as isize),
            b"%2hhx%2hhx%2hhx\0" as *const u8 as *const libc::c_char,
            &mut r as *mut u_char,
            &mut g as *mut u_char,
            &mut b as *mut u_char,
        );
        if n != 3 as libc::c_int {
            return -(1 as libc::c_int);
        }
        return colour_join_rgb(r, g, b);
    }
    if strncasecmp(
        s,
        b"colour\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        n = strtonum(
            s
                .offset(
                    ::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)),
            0 as libc::c_int as libc::c_longlong,
            255 as libc::c_int as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        if !errstr.is_null() {
            return -(1 as libc::c_int);
        }
        return n | 0x1000000 as libc::c_int;
    }
    if strcasecmp(s, b"black\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"0\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if strcasecmp(s, b"red\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"1\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strcasecmp(s, b"green\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"2\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if strcasecmp(s, b"yellow\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"3\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 3 as libc::c_int;
    }
    if strcasecmp(s, b"blue\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"4\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 4 as libc::c_int;
    }
    if strcasecmp(s, b"magenta\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"5\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 5 as libc::c_int;
    }
    if strcasecmp(s, b"cyan\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"6\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 6 as libc::c_int;
    }
    if strcasecmp(s, b"white\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(s, b"7\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 7 as libc::c_int;
    }
    if strcasecmp(s, b"default\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"8\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 8 as libc::c_int;
    }
    if strcasecmp(s, b"brightblack\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"90\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 90 as libc::c_int;
    }
    if strcasecmp(s, b"brightred\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"91\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 91 as libc::c_int;
    }
    if strcasecmp(s, b"brightgreen\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"92\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 92 as libc::c_int;
    }
    if strcasecmp(s, b"brightyellow\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"93\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 93 as libc::c_int;
    }
    if strcasecmp(s, b"brightblue\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"94\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 94 as libc::c_int;
    }
    if strcasecmp(s, b"brightmagenta\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"95\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 95 as libc::c_int;
    }
    if strcasecmp(s, b"brightcyan\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"96\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 96 as libc::c_int;
    }
    if strcasecmp(s, b"brightwhite\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(s, b"97\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 97 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn colour_256to16(mut c: u_char) -> u_char {
    static mut table: [u_char; 256] = [
        0 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        4 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        2 as libc::c_int as u_char,
        6 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        1 as libc::c_int as u_char,
        5 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        3 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        12 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        10 as libc::c_int as u_char,
        14 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        9 as libc::c_int as u_char,
        13 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        11 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        0 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        8 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        7 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
        15 as libc::c_int as u_char,
    ];
    return table[c as usize];
}
