use ::libc;
extern "C" {
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
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn attributes_tostring(
    mut attr: libc::c_int,
) -> *const libc::c_char {
    static mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = 0;
    if attr == 0 as libc::c_int {
        return b"none\0" as *const u8 as *const libc::c_char;
    }
    len = xsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%s%s%s%s%s%s%s%s\0" as *const u8 as *const libc::c_char,
        if attr & 0x1 as libc::c_int != 0 {
            b"bright,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x2 as libc::c_int != 0 {
            b"dim,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x4 as libc::c_int != 0 {
            b"underscore,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x8 as libc::c_int != 0 {
            b"blink,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x10 as libc::c_int != 0 {
            b"reverse,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x20 as libc::c_int != 0 {
            b"hidden,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x40 as libc::c_int != 0 {
            b"italics,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if attr & 0x100 as libc::c_int != 0 {
            b"strikethrough,\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    ) as size_t;
    if len > 0 as libc::c_int as size_t {
        buf[len.wrapping_sub(1 as libc::c_int as size_t)
            as usize] = '\0' as i32 as libc::c_char;
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn attributes_fromstring(
    mut str: *const libc::c_char,
) -> libc::c_int {
    let delimiters: [libc::c_char; 4] = *::core::mem::transmute::<
        &[u8; 4],
        &[libc::c_char; 4],
    >(b" ,|\0");
    let mut attr: libc::c_int = 0;
    let mut end: size_t = 0;
    if *str as libc::c_int == '\0' as i32
        || strcspn(str, delimiters.as_ptr()) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    if !(strchr(
        delimiters.as_ptr(),
        *str
            .offset(
                (strlen(str)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int,
    ))
        .is_null()
    {
        return -(1 as libc::c_int);
    }
    if strcasecmp(str, b"default\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcasecmp(str, b"none\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    attr = 0 as libc::c_int;
    loop {
        end = strcspn(str, delimiters.as_ptr());
        if end == 6 as libc::c_int as size_t
            && strncasecmp(str, b"bright\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
            || end == 4 as libc::c_int as size_t
                && strncasecmp(str, b"bold\0" as *const u8 as *const libc::c_char, end)
                    == 0 as libc::c_int
        {
            attr |= 0x1 as libc::c_int;
        } else if end == 3 as libc::c_int as size_t
            && strncasecmp(str, b"dim\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
        {
            attr |= 0x2 as libc::c_int;
        } else if end == 10 as libc::c_int as size_t
            && strncasecmp(str, b"underscore\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
        {
            attr |= 0x4 as libc::c_int;
        } else if end == 5 as libc::c_int as size_t
            && strncasecmp(str, b"blink\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
        {
            attr |= 0x8 as libc::c_int;
        } else if end == 7 as libc::c_int as size_t
            && strncasecmp(str, b"reverse\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
        {
            attr |= 0x10 as libc::c_int;
        } else if end == 6 as libc::c_int as size_t
            && strncasecmp(str, b"hidden\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
        {
            attr |= 0x20 as libc::c_int;
        } else if end == 7 as libc::c_int as size_t
            && strncasecmp(str, b"italics\0" as *const u8 as *const libc::c_char, end)
                == 0 as libc::c_int
        {
            attr |= 0x40 as libc::c_int;
        } else if end == 13 as libc::c_int as size_t
            && strncasecmp(
                str,
                b"strikethrough\0" as *const u8 as *const libc::c_char,
                end,
            ) == 0 as libc::c_int
        {
            attr |= 0x100 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
        str = str
            .offset(
                end.wrapping_add(strspn(str.offset(end as isize), delimiters.as_ptr()))
                    as isize,
            );
        if !(*str as libc::c_int != '\0' as i32) {
            break;
        }
    }
    return attr;
}
