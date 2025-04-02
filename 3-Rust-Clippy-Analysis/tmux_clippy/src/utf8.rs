use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn free(_: *mut libc::c_void);
    fn mbtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> libc::c_int;
    fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn vis(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type ssize_t = __ssize_t;
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[no_mangle]
pub unsafe extern "C" fn utf8_set(mut ud: *mut utf8_data, mut ch: u_char) {
    static mut empty: utf8_data = {
        let mut init = utf8_data {
            data: [0 as libc::c_int as u_char, 0, 0, 0, 0, 0, 0, 0, 0],
            have: 1 as libc::c_int as u_char,
            size: 1 as libc::c_int as u_char,
            width: 1 as libc::c_int as u_char,
        };
        init
    };
    memcpy(
        ud as *mut libc::c_void,
        &empty as *const utf8_data as *const libc::c_void,
        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    *((*ud).data).as_mut_ptr() = ch;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_copy(mut to: *mut utf8_data, mut from: *const utf8_data) {
    let mut i: u_int = 0;
    memcpy(
        to as *mut libc::c_void,
        from as *const libc::c_void,
        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    i = (*to).size as u_int;
    while (i as libc::c_ulong) < ::core::mem::size_of::<[u_char; 9]>() as libc::c_ulong {
        (*to).data[i as usize] = '\0' as i32 as u_char;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn utf8_open(
    mut ud: *mut utf8_data,
    mut ch: u_char,
) -> utf8_state {
    memset(
        ud as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    if ch as libc::c_int >= 0xc2 as libc::c_int
        && ch as libc::c_int <= 0xdf as libc::c_int
    {
        (*ud).size = 2 as libc::c_int as u_char;
    } else if ch as libc::c_int >= 0xe0 as libc::c_int
        && ch as libc::c_int <= 0xef as libc::c_int
    {
        (*ud).size = 3 as libc::c_int as u_char;
    } else if ch as libc::c_int >= 0xf0 as libc::c_int
        && ch as libc::c_int <= 0xf4 as libc::c_int
    {
        (*ud).size = 4 as libc::c_int as u_char;
    } else {
        return UTF8_ERROR
    }
    utf8_append(ud, ch);
    return UTF8_MORE;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_append(
    mut ud: *mut utf8_data,
    mut ch: u_char,
) -> utf8_state {
    let mut wc: wchar_t = 0;
    let mut width: libc::c_int = 0;
    if (*ud).have as libc::c_int >= (*ud).size as libc::c_int {
        fatalx(b"UTF-8 character overflow\0" as *const u8 as *const libc::c_char);
    }
    if (*ud).size as libc::c_ulong
        > ::core::mem::size_of::<[u_char; 9]>() as libc::c_ulong
    {
        fatalx(b"UTF-8 character size too large\0" as *const u8 as *const libc::c_char);
    }
    if (*ud).have as libc::c_int != 0 as libc::c_int
        && ch as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int
    {
        (*ud).width = 0xff as libc::c_int as u_char;
    }
    let fresh0 = (*ud).have;
    (*ud).have = ((*ud).have).wrapping_add(1);
    (*ud).data[fresh0 as usize] = ch;
    if (*ud).have as libc::c_int != (*ud).size as libc::c_int {
        return UTF8_MORE;
    }
    if (*ud).width as libc::c_int == 0xff as libc::c_int {
        return UTF8_ERROR;
    }
    if utf8_combine(ud, &mut wc) as libc::c_uint
        != UTF8_DONE as libc::c_int as libc::c_uint
    {
        return UTF8_ERROR;
    }
    width = utf8_width(wc);
    if width < 0 as libc::c_int {
        return UTF8_ERROR;
    }
    (*ud).width = width as u_char;
    return UTF8_DONE;
}
unsafe extern "C" fn utf8_width(mut wc: wchar_t) -> libc::c_int {
    let mut width: libc::c_int = 0;
    width = wcwidth(wc);
    if width < 0 as libc::c_int || width > 0xff as libc::c_int {
        log_debug(
            b"Unicode %04lx, wcwidth() %d\0" as *const u8 as *const libc::c_char,
            wc as libc::c_long,
            width,
        );
        if width < 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_combine(
    mut ud: *const utf8_data,
    mut wc: *mut wchar_t,
) -> utf8_state {
    match mbtowc(
        wc,
        ((*ud).data).as_ptr() as *const libc::c_char,
        (*ud).size as size_t,
    ) {
        -1 => {
            log_debug(
                b"UTF-8 %.*s, mbtowc() %d\0" as *const u8 as *const libc::c_char,
                (*ud).size as libc::c_int,
                ((*ud).data).as_ptr(),
                *__errno_location(),
            );
            mbtowc(
                0 as *mut wchar_t,
                0 as *const libc::c_char,
                __ctype_get_mb_cur_max(),
            );
            return UTF8_ERROR;
        }
        0 => return UTF8_ERROR,
        _ => return UTF8_DONE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn utf8_split(
    mut wc: wchar_t,
    mut ud: *mut utf8_data,
) -> utf8_state {
    let mut s: [libc::c_char; 16] = [0; 16];
    let mut slen: libc::c_int = 0;
    slen = wctomb(s.as_mut_ptr(), wc);
    if slen <= 0 as libc::c_int
        || slen > ::core::mem::size_of::<[u_char; 9]>() as libc::c_ulong as libc::c_int
    {
        return UTF8_ERROR;
    }
    memcpy(
        ((*ud).data).as_mut_ptr() as *mut libc::c_void,
        s.as_mut_ptr() as *const libc::c_void,
        slen as libc::c_ulong,
    );
    (*ud).size = slen as u_char;
    (*ud).width = utf8_width(wc) as u_char;
    return UTF8_DONE;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_strvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut more: utf8_state = UTF8_MORE;
    let mut i: size_t = 0;
    start = dst;
    end = src.offset(len as isize);
    while src < end {
        more = utf8_open(&mut ud, *src as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                src = src.offset(1);
                if !(src < end
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut ud, *src as u_char);
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                i = 0 as libc::c_int as size_t;
                while i < ud.size as size_t {
                    let fresh1 = dst;
                    dst = dst.offset(1);
                    *fresh1 = ud.data[i as usize] as libc::c_char;
                    i = i.wrapping_add(1);
                    i;
                }
                continue;
            } else {
                src = src.offset(-(ud.have as libc::c_int as isize));
            }
        }
        if src < end.offset(-(1 as libc::c_int as isize)) {
            dst = vis(
                dst,
                *src.offset(0 as libc::c_int as isize) as libc::c_int,
                flag,
                *src.offset(1 as libc::c_int as isize) as libc::c_int,
            );
        } else if src < end {
            dst = vis(
                dst,
                *src.offset(0 as libc::c_int as isize) as libc::c_int,
                flag,
                '\0' as i32,
            );
        }
        src = src.offset(1);
        src;
    }
    *dst = '\0' as i32 as libc::c_char;
    return dst.offset_from(start) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_stravis(
    mut dst: *mut *mut libc::c_char,
    mut src: *const libc::c_char,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    buf = xreallocarray(
        0 as *mut libc::c_void,
        4 as libc::c_int as size_t,
        (strlen(src)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    len = utf8_strvis(buf, src, strlen(src), flag);
    *dst = xrealloc(buf as *mut libc::c_void, (len + 1 as libc::c_int) as size_t)
        as *mut libc::c_char;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_isvalid(mut s: *const libc::c_char) -> libc::c_int {
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut more: utf8_state = UTF8_MORE;
    end = s.offset(strlen(s) as isize);
    while s < end {
        more = utf8_open(&mut ud, *s as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                s = s.offset(1);
                if !(s < end
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut ud, *s as u_char);
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                continue;
            }
            return 0 as libc::c_int;
        } else {
            if (*s as libc::c_int) < 0x20 as libc::c_int
                || *s as libc::c_int > 0x7e as libc::c_int
            {
                return 0 as libc::c_int;
            }
            s = s.offset(1);
            s;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_sanitize(
    mut src: *const libc::c_char,
) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    let mut more: utf8_state = UTF8_MORE;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut i: u_int = 0;
    dst = 0 as *mut libc::c_char;
    n = 0 as libc::c_int as size_t;
    while *src as libc::c_int != '\0' as i32 {
        dst = xreallocarray(
            dst as *mut libc::c_void,
            n.wrapping_add(1 as libc::c_int as size_t),
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        more = utf8_open(&mut ud, *src as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                src = src.offset(1);
                if !(*src as libc::c_int != '\0' as i32
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut ud, *src as u_char);
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                dst = xreallocarray(
                    dst as *mut libc::c_void,
                    n.wrapping_add(ud.width as size_t),
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                ) as *mut libc::c_char;
                i = 0 as libc::c_int as u_int;
                while i < ud.width as u_int {
                    let fresh2 = n;
                    n = n.wrapping_add(1);
                    *dst.offset(fresh2 as isize) = '_' as i32 as libc::c_char;
                    i = i.wrapping_add(1);
                    i;
                }
                continue;
            } else {
                src = src.offset(-(ud.have as libc::c_int as isize));
            }
        }
        if *src as libc::c_int > 0x1f as libc::c_int
            && (*src as libc::c_int) < 0x7f as libc::c_int
        {
            let fresh3 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh3 as isize) = *src;
        } else {
            let fresh4 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh4 as isize) = '_' as i32 as libc::c_char;
        }
        src = src.offset(1);
        src;
    }
    dst = xreallocarray(
        dst as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as size_t),
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    *dst.offset(n as isize) = '\0' as i32 as libc::c_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_strlen(mut s: *const utf8_data) -> size_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while (*s.offset(i as isize)).size as libc::c_int != 0 as libc::c_int {
        i = i.wrapping_add(1);
        i;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_strwidth(
    mut s: *const utf8_data,
    mut n: ssize_t,
) -> u_int {
    let mut i: ssize_t = 0;
    let mut width: u_int = 0;
    width = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as ssize_t;
    while (*s.offset(i as isize)).size as libc::c_int != 0 as libc::c_int {
        if n != -(1 as libc::c_int) as ssize_t && n == i {
            break;
        }
        width = width.wrapping_add((*s.offset(i as isize)).width as u_int);
        i += 1;
        i;
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_fromcstr(mut src: *const libc::c_char) -> *mut utf8_data {
    let mut dst: *mut utf8_data = 0 as *mut utf8_data;
    let mut n: size_t = 0;
    let mut more: utf8_state = UTF8_MORE;
    dst = 0 as *mut utf8_data;
    n = 0 as libc::c_int as size_t;
    while *src as libc::c_int != '\0' as i32 {
        dst = xreallocarray(
            dst as *mut libc::c_void,
            n.wrapping_add(1 as libc::c_int as size_t),
            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
        ) as *mut utf8_data;
        more = utf8_open(&mut *dst.offset(n as isize), *src as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                src = src.offset(1);
                if !(*src as libc::c_int != '\0' as i32
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut *dst.offset(n as isize), *src as u_char);
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                n = n.wrapping_add(1);
                n;
                continue;
            } else {
                src = src
                    .offset(-((*dst.offset(n as isize)).have as libc::c_int as isize));
            }
        }
        utf8_set(&mut *dst.offset(n as isize), *src as u_char);
        n = n.wrapping_add(1);
        n;
        src = src.offset(1);
        src;
    }
    dst = xreallocarray(
        dst as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as size_t),
        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
    ) as *mut utf8_data;
    (*dst.offset(n as isize)).size = 0 as libc::c_int as u_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_tocstr(mut src: *mut utf8_data) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    dst = 0 as *mut libc::c_char;
    n = 0 as libc::c_int as size_t;
    while (*src).size as libc::c_int != 0 as libc::c_int {
        dst = xreallocarray(
            dst as *mut libc::c_void,
            n.wrapping_add((*src).size as size_t),
            1 as libc::c_int as size_t,
        ) as *mut libc::c_char;
        memcpy(
            dst.offset(n as isize) as *mut libc::c_void,
            ((*src).data).as_mut_ptr() as *const libc::c_void,
            (*src).size as libc::c_ulong,
        );
        n = n.wrapping_add((*src).size as size_t);
        src = src.offset(1);
        src;
    }
    dst = xreallocarray(
        dst as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as size_t),
        1 as libc::c_int as size_t,
    ) as *mut libc::c_char;
    *dst.offset(n as isize) = '\0' as i32 as libc::c_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_cstrwidth(mut s: *const libc::c_char) -> u_int {
    let mut tmp: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut width: u_int = 0;
    let mut more: utf8_state = UTF8_MORE;
    width = 0 as libc::c_int as u_int;
    while *s as libc::c_int != '\0' as i32 {
        more = utf8_open(&mut tmp, *s as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                s = s.offset(1);
                if !(*s as libc::c_int != '\0' as i32
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut tmp, *s as u_char);
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                width = width.wrapping_add(tmp.width as u_int);
                continue;
            } else {
                s = s.offset(-(tmp.have as libc::c_int as isize));
            }
        }
        if *s as libc::c_int > 0x1f as libc::c_int
            && *s as libc::c_int != 0x7f as libc::c_int
        {
            width = width.wrapping_add(1);
            width;
        }
        s = s.offset(1);
        s;
    }
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_trimcstr(
    mut s: *const libc::c_char,
    mut width: u_int,
) -> *mut libc::c_char {
    let mut tmp: *mut utf8_data = 0 as *mut utf8_data;
    let mut next: *mut utf8_data = 0 as *mut utf8_data;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut at: u_int = 0;
    tmp = utf8_fromcstr(s);
    at = 0 as libc::c_int as u_int;
    next = tmp;
    while (*next).size as libc::c_int != 0 as libc::c_int {
        if at.wrapping_add((*next).width as u_int) > width {
            (*next).size = 0 as libc::c_int as u_char;
            break;
        } else {
            at = at.wrapping_add((*next).width as u_int);
            next = next.offset(1);
            next;
        }
    }
    out = utf8_tocstr(tmp);
    free(tmp as *mut libc::c_void);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_rtrimcstr(
    mut s: *const libc::c_char,
    mut width: u_int,
) -> *mut libc::c_char {
    let mut tmp: *mut utf8_data = 0 as *mut utf8_data;
    let mut next: *mut utf8_data = 0 as *mut utf8_data;
    let mut end: *mut utf8_data = 0 as *mut utf8_data;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut at: u_int = 0;
    tmp = utf8_fromcstr(s);
    end = tmp;
    while (*end).size as libc::c_int != 0 as libc::c_int {
        end = end.offset(1);
        end;
    }
    if end == tmp {
        free(tmp as *mut libc::c_void);
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    next = end.offset(-(1 as libc::c_int as isize));
    at = 0 as libc::c_int as u_int;
    loop {
        if at.wrapping_add((*next).width as u_int) > width {
            next = next.offset(1);
            next;
            break;
        } else {
            at = at.wrapping_add((*next).width as u_int);
            if next == tmp {
                break;
            }
            next = next.offset(-1);
            next;
        }
    }
    out = utf8_tocstr(next);
    free(tmp as *mut libc::c_void);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_padcstr(
    mut s: *const libc::c_char,
    mut width: u_int,
) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    n = utf8_cstrwidth(s);
    if n >= width {
        return xstrdup(s);
    }
    slen = strlen(s);
    out = xmalloc(
        slen
            .wrapping_add(1 as libc::c_int as size_t)
            .wrapping_add(width.wrapping_sub(n) as size_t),
    ) as *mut libc::c_char;
    memcpy(out as *mut libc::c_void, s as *const libc::c_void, slen);
    i = n;
    while i < width {
        let fresh5 = slen;
        slen = slen.wrapping_add(1);
        *out.offset(fresh5 as isize) = ' ' as i32 as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    *out.offset(slen as isize) = '\0' as i32 as libc::c_char;
    return out;
}
