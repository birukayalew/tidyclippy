use ::libc;
pub type __u_char = libc::c_uchar;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn unvis(
    mut cp: *mut libc::c_char,
    mut c: libc::c_char,
    mut astate: *mut libc::c_int,
    mut flag: libc::c_int,
) -> libc::c_int {
    if flag & 1 as libc::c_int != 0 {
        if *astate == 5 as libc::c_int || *astate == 6 as libc::c_int {
            *astate = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        return if *astate == 0 as libc::c_int {
            3 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    match *astate {
        0 => {
            *cp = 0 as libc::c_int as libc::c_char;
            if c as libc::c_int == '\\' as i32 {
                *astate = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            *cp = c;
            return 1 as libc::c_int;
        }
        1 => {
            match c as libc::c_int {
                92 => {
                    *cp = c;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    *cp = (c as libc::c_int - '0' as i32) as libc::c_char;
                    *astate = 5 as libc::c_int;
                    return 0 as libc::c_int;
                }
                77 => {
                    *cp = 0o200 as libc::c_int as libc::c_char;
                    *astate = 2 as libc::c_int;
                    return 0 as libc::c_int;
                }
                94 => {
                    *astate = 4 as libc::c_int;
                    return 0 as libc::c_int;
                }
                110 => {
                    *cp = '\n' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                114 => {
                    *cp = '\r' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                98 => {
                    *cp = '\u{8}' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                97 => {
                    *cp = '\u{7}' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                118 => {
                    *cp = '\u{b}' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                116 => {
                    *cp = '\t' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                102 => {
                    *cp = '\u{c}' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                115 => {
                    *cp = ' ' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                69 => {
                    *cp = '\u{1b}' as i32 as libc::c_char;
                    *astate = 0 as libc::c_int;
                    return 1 as libc::c_int;
                }
                10 => {
                    *astate = 0 as libc::c_int;
                    return 3 as libc::c_int;
                }
                36 => {
                    *astate = 0 as libc::c_int;
                    return 3 as libc::c_int;
                }
                _ => {}
            }
            *astate = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
        2 => {
            if c as libc::c_int == '-' as i32 {
                *astate = 3 as libc::c_int;
            } else if c as libc::c_int == '^' as i32 {
                *astate = 4 as libc::c_int;
            } else {
                *astate = 0 as libc::c_int;
                return -(1 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        3 => {
            *astate = 0 as libc::c_int;
            *cp = (*cp as libc::c_int | c as libc::c_int) as libc::c_char;
            return 1 as libc::c_int;
        }
        4 => {
            if c as libc::c_int == '?' as i32 {
                *cp = (*cp as libc::c_int | 0o177 as libc::c_int) as libc::c_char;
            } else {
                *cp = (*cp as libc::c_int | c as libc::c_int & 0o37 as libc::c_int)
                    as libc::c_char;
            }
            *astate = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        5 => {
            if c as u_char as libc::c_int >= '0' as i32
                && c as u_char as libc::c_int <= '7' as i32
            {
                *cp = (((*cp as libc::c_int) << 3 as libc::c_int)
                    + (c as libc::c_int - '0' as i32)) as libc::c_char;
                *astate = 6 as libc::c_int;
                return 0 as libc::c_int;
            }
            *astate = 0 as libc::c_int;
            return 2 as libc::c_int;
        }
        6 => {
            *astate = 0 as libc::c_int;
            if c as u_char as libc::c_int >= '0' as i32
                && c as u_char as libc::c_int <= '7' as i32
            {
                *cp = (((*cp as libc::c_int) << 3 as libc::c_int)
                    + (c as libc::c_int - '0' as i32)) as libc::c_char;
                return 1 as libc::c_int;
            }
            return 2 as libc::c_int;
        }
        _ => {
            *astate = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn strunvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = dst;
    let mut state: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = src;
        src = src.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        loop {
            match unvis(dst, c, &mut state, 0 as libc::c_int) {
                1 => {
                    dst = dst.offset(1);
                    dst;
                    break;
                }
                2 => {
                    dst = dst.offset(1);
                    dst;
                }
                0 | 3 => {
                    break;
                }
                _ => {
                    *dst = '\0' as i32 as libc::c_char;
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    if unvis(dst, c, &mut state, 1 as libc::c_int) == 1 as libc::c_int {
        dst = dst.offset(1);
        dst;
    }
    *dst = '\0' as i32 as libc::c_char;
    return dst.offset_from(start) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strnunvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut sz: size_t,
) -> ssize_t {
    let mut c: libc::c_char = 0;
    let mut p: libc::c_char = 0;
    let mut start: *mut libc::c_char = dst;
    let mut end: *mut libc::c_char = dst
        .offset(sz as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut state: libc::c_int = 0 as libc::c_int;
    if sz > 0 as libc::c_int as size_t {
        *end = '\0' as i32 as libc::c_char;
    }
    loop {
        let fresh1 = src;
        src = src.offset(1);
        c = *fresh1;
        if !(c != 0) {
            break;
        }
        loop {
            match unvis(&mut p, c, &mut state, 0 as libc::c_int) {
                1 => {
                    if dst < end {
                        *dst = p;
                    }
                    dst = dst.offset(1);
                    dst;
                    break;
                }
                2 => {
                    if dst < end {
                        *dst = p;
                    }
                    dst = dst.offset(1);
                    dst;
                }
                0 | 3 => {
                    break;
                }
                _ => {
                    if dst <= end {
                        *dst = '\0' as i32 as libc::c_char;
                    }
                    return -(1 as libc::c_int) as ssize_t;
                }
            }
        }
    }
    if unvis(&mut p, c, &mut state, 1 as libc::c_int) == 1 as libc::c_int {
        if dst < end {
            *dst = p;
        }
        dst = dst.offset(1);
        dst;
    }
    if dst <= end {
        *dst = '\0' as i32 as libc::c_char;
    }
    return dst.offset_from(start) as libc::c_long;
}
