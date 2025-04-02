#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm)]
use core::arch::asm;
extern "C" {
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zdivmod(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_allocation: *mut libc::c_void;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: libc::c_int,
    pub padding__: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [zahl; 1];
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17123592612869879796;
        }
        19 => {
            current_block_42 = 17123592612869879796;
        }
        18 => {
            current_block_42 = 7295796347073542504;
        }
        17 => {
            current_block_42 = 1678841039753865640;
        }
        16 => {
            current_block_42 = 4428433140869529006;
        }
        15 => {
            current_block_42 = 7060192243506437934;
        }
        14 => {
            current_block_42 = 10319869968961460719;
        }
        13 => {
            current_block_42 = 1232922633126777150;
        }
        12 => {
            current_block_42 = 16818959577490016211;
        }
        11 => {
            current_block_42 = 11360923742995600144;
        }
        10 => {
            current_block_42 = 12310883806328202904;
        }
        9 => {
            current_block_42 = 6807571309547269490;
        }
        8 => {
            current_block_42 = 2540506661003364611;
        }
        7 => {
            current_block_42 = 13253931237392700050;
        }
        6 => {
            current_block_42 = 6589414636452177323;
        }
        5 => {
            current_block_42 = 1198327748524965992;
        }
        4 => {
            current_block_42 = 7339321152356695781;
        }
        3 => {
            current_block_42 = 8085337086024616189;
        }
        2 => {
            current_block_42 = 8830848799650308304;
        }
        1 => {
            current_block_42 = 11295390336478990698;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n    shlq $3, {3}\n    addq {1}, {3}\n 1:\n    movq 0({2}), {0}\n    movq {0}, 0({1})\n    movq 8({2}), {0}\n    movq {0}, 8({1})\n    movq 16({2}), {0}\n    movq {0}, 16({1})\n    movq 24({2}), {0}\n    movq {0}, 24({1})\n    addq $32, {2}\n    addq $32, {1}\n    cmpq {3}, {1}\n    jl 1b",
                lateout(reg) t, inlateout(reg) d, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17123592612869879796 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7295796347073542504;
        }
        _ => {}
    }
    match current_block_42 {
        7295796347073542504 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1678841039753865640;
        }
        _ => {}
    }
    match current_block_42 {
        1678841039753865640 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4428433140869529006;
        }
        _ => {}
    }
    match current_block_42 {
        4428433140869529006 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7060192243506437934;
        }
        _ => {}
    }
    match current_block_42 {
        7060192243506437934 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 10319869968961460719;
        }
        _ => {}
    }
    match current_block_42 {
        10319869968961460719 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1232922633126777150;
        }
        _ => {}
    }
    match current_block_42 {
        1232922633126777150 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16818959577490016211;
        }
        _ => {}
    }
    match current_block_42 {
        16818959577490016211 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11360923742995600144;
        }
        _ => {}
    }
    match current_block_42 {
        11360923742995600144 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12310883806328202904;
        }
        _ => {}
    }
    match current_block_42 {
        12310883806328202904 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 6807571309547269490;
        }
        _ => {}
    }
    match current_block_42 {
        6807571309547269490 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 2540506661003364611;
        }
        _ => {}
    }
    match current_block_42 {
        2540506661003364611 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13253931237392700050;
        }
        _ => {}
    }
    match current_block_42 {
        13253931237392700050 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 6589414636452177323;
        }
        _ => {}
    }
    match current_block_42 {
        6589414636452177323 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1198327748524965992;
        }
        _ => {}
    }
    match current_block_42 {
        1198327748524965992 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7339321152356695781;
        }
        _ => {}
    }
    match current_block_42 {
        7339321152356695781 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8085337086024616189;
        }
        _ => {}
    }
    match current_block_42 {
        8085337086024616189 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8830848799650308304;
        }
        _ => {}
    }
    match current_block_42 {
        8830848799650308304 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11295390336478990698;
        }
        _ => {}
    }
    match current_block_42 {
        11295390336478990698 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign &= 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
unsafe extern "C" fn libzahl_failure(mut error: libc::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn libzahl_memfailure() {
    if *__errno_location() == 0 {
        *__errno_location() = 2 as libc::c_int;
    }
    libzahl_failure(*__errno_location());
}
#[inline]
unsafe extern "C" fn sprintint_fix(mut buf: *mut libc::c_char, mut v: zahl_char_t) {
    let mut partials: *const libc::c_char = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0"
        as *const u8 as *const libc::c_char;
    let mut buffer: *mut uint16_t = buf.offset(1 as libc::c_int as isize)
        as *mut uint16_t;
    *buffer
        .offset(
            8 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            7 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            6 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            5 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            4 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            3 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            2 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            1 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buffer
        .offset(
            0 as libc::c_int as isize,
        ) = *(partials
        .offset(
            (2 as libc::c_int as zahl_char_t * (v % 100 as libc::c_int as zahl_char_t))
                as isize,
        ) as *const uint16_t);
    v = v / 100 as libc::c_int as zahl_char_t;
    *buf = ('0' as i32 as zahl_char_t).wrapping_add(v) as libc::c_char;
    *buf.offset(19 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
}
#[inline]
unsafe extern "C" fn cmemmove(
    mut d: *mut libc::c_char,
    mut s: *const libc::c_char,
    mut n: libc::c_long,
) {
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        let fresh2 = d;
        d = d.offset(1);
        *fresh2 = *fresh1;
    };
}
#[inline]
unsafe extern "C" fn sprintint_min(
    mut buf: *mut libc::c_char,
    mut v: zahl_char_t,
) -> size_t {
    let mut i: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut j: libc::c_long = 0;
    sprintint_fix(buf, v);
    while *buf.offset(i as isize) as libc::c_int == '0' as i32 {
        i += 1;
        i;
    }
    j = 19 as libc::c_int as libc::c_long - i;
    cmemmove(buf, buf.offset(i as isize), j);
    *buf.offset(j as isize) = 0 as libc::c_int as libc::c_char;
    return j as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn zstr(
    mut a: *mut zahl,
    mut b: *mut libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut len: size_t = 0;
    let mut neg: size_t = 0;
    let mut last: size_t = 0;
    let mut tot: size_t = 0 as libc::c_int as size_t;
    let mut overridden: libc::c_char = 0 as libc::c_int as libc::c_char;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        if b.is_null() as libc::c_int as libc::c_long != 0
            && {
                b = malloc(2 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
                b.is_null() as libc::c_int as libc::c_long != 0
            }
        {
            libzahl_memfailure();
        }
        *b.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *b.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return b;
    }
    if n == 0 {
        n = (20 as libc::c_int * 64 as libc::c_int / 64 as libc::c_int
            + (64 as libc::c_int == 8 as libc::c_int) as libc::c_int) as size_t
            * (*a).used;
    }
    if b.is_null() as libc::c_int as libc::c_long != 0
        && {
            libzahl_temp_allocation = malloc(n.wrapping_add(1 as libc::c_int as size_t));
            b = libzahl_temp_allocation as *mut libc::c_char;
            b.is_null() as libc::c_int as libc::c_long != 0
        }
    {
        libzahl_memfailure();
    }
    neg = (zsignum(a) < 0 as libc::c_int) as libc::c_int as size_t;
    zabs(libzahl_tmp_str_num.as_mut_ptr(), a);
    *b.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    b = b.offset(neg as isize);
    n = n.wrapping_sub(neg);
    last = n;
    n = if last > 19 as libc::c_int as size_t {
        n.wrapping_sub(19 as libc::c_int as size_t)
    } else {
        0 as libc::c_int as size_t
    };
    loop {
        zdivmod(
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_rem.as_mut_ptr(),
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_const_1e19.as_mut_ptr(),
        );
        if (zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0) as libc::c_int as libc::c_long
            != 0
        {
            sprintint_fix(
                b.offset(n as isize),
                if zzero(libzahl_tmp_str_rem.as_mut_ptr()) != 0 {
                    0 as libc::c_int as zahl_char_t
                } else {
                    *((*libzahl_tmp_str_rem.as_mut_ptr()).chars)
                        .offset(0 as libc::c_int as isize)
                },
            );
            *b.offset(n.wrapping_add(19 as libc::c_int as size_t) as isize) = overridden;
            overridden = *b.offset(n as isize);
            last = n;
            n = if last > 19 as libc::c_int as size_t {
                n.wrapping_sub(19 as libc::c_int as size_t)
            } else {
                0 as libc::c_int as size_t
            };
            tot = tot.wrapping_add(19 as libc::c_int as size_t);
        } else {
            len = sprintint_min(
                buf.as_mut_ptr(),
                *((*libzahl_tmp_str_rem.as_mut_ptr()).chars)
                    .offset(0 as libc::c_int as isize),
            );
            if tot != 0 {
                memcpy(
                    b as *mut libc::c_void,
                    buf.as_mut_ptr() as *const libc::c_void,
                    len,
                );
                memmove(
                    b.offset(len as isize) as *mut libc::c_void,
                    b.offset(last as isize) as *const libc::c_void,
                    tot.wrapping_add(1 as libc::c_int as size_t),
                );
            } else {
                memcpy(
                    b as *mut libc::c_void,
                    buf.as_mut_ptr() as *const libc::c_void,
                    len.wrapping_add(1 as libc::c_int as size_t),
                );
            }
            break;
        }
    }
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    return b.offset(-(neg as isize));
}
