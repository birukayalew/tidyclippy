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
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
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
#[inline]
unsafe extern "C" fn zmemcpy_range(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut i: size_t,
    mut n: size_t,
) {
    d = d.offset(i as isize);
    s = s.offset(i as isize);
    n = n.wrapping_sub(i);
    libzahl_memcpy(d, s, n);
}
#[no_mangle]
pub unsafe extern "C" fn zxor(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    let mut bn: size_t = 0;
    let mut cn: size_t = 0;
    let mut bc: *const zahl_char_t = 0 as *const zahl_char_t;
    let mut cc: *const zahl_char_t = 0 as *const zahl_char_t;
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        if a != c {
            zset(a, c);
        }
        return;
    } else if (zzero(c) != 0) as libc::c_int as libc::c_long != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    bn = (*b).used;
    bc = (*b).chars;
    cn = (*c).used;
    cc = (*c).chars;
    n = (if bn < cn { bn } else { cn });
    m = (if bn > cn { bn } else { cn });
    if (*a).alloced < m {
        libzahl_realloc(a, m);
    }
    if a == b {
        let mut a__: *mut zahl_char_t = (*a).chars;
        let mut b__: *const zahl_char_t = (*a).chars;
        let mut c__: *const zahl_char_t = cc;
        let mut i__: size_t = 0;
        let mut n__: size_t = n;
        if n__ <= 4 as libc::c_int as size_t {
            if n__ >= 1 as libc::c_int as size_t {
                *a__
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *b__.offset(0 as libc::c_int as isize)
                    ^ *c__.offset(0 as libc::c_int as isize);
            }
            if n__ >= 2 as libc::c_int as size_t {
                *a__
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *b__.offset(1 as libc::c_int as isize)
                    ^ *c__.offset(1 as libc::c_int as isize);
            }
            if n__ >= 3 as libc::c_int as size_t {
                *a__
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *b__.offset(2 as libc::c_int as isize)
                    ^ *c__.offset(2 as libc::c_int as isize);
            }
            if n__ >= 4 as libc::c_int as size_t {
                *a__
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *b__.offset(3 as libc::c_int as isize)
                    ^ *c__.offset(3 as libc::c_int as isize);
            }
        } else {
            i__ = 0 as libc::c_int as size_t;
            loop {
                i__ = i__.wrapping_add(4 as libc::c_int as size_t);
                if !(i__ < n__) {
                    break;
                }
                *a__
                    .offset(
                        i__.wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) = *b__
                    .offset(i__.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(1 as libc::c_int as size_t) as isize);
                *a__
                    .offset(
                        i__.wrapping_sub(2 as libc::c_int as size_t) as isize,
                    ) = *b__
                    .offset(i__.wrapping_sub(2 as libc::c_int as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(2 as libc::c_int as size_t) as isize);
                *a__
                    .offset(
                        i__.wrapping_sub(3 as libc::c_int as size_t) as isize,
                    ) = *b__
                    .offset(i__.wrapping_sub(3 as libc::c_int as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(3 as libc::c_int as size_t) as isize);
                *a__
                    .offset(
                        i__.wrapping_sub(4 as libc::c_int as size_t) as isize,
                    ) = *b__
                    .offset(i__.wrapping_sub(4 as libc::c_int as size_t) as isize)
                    ^ *c__.offset(i__.wrapping_sub(4 as libc::c_int as size_t) as isize);
            }
            if i__ > n__ {
                i__ = i__.wrapping_sub(4 as libc::c_int as size_t);
                while i__ < n__ {
                    *a__
                        .offset(
                            i__ as isize,
                        ) = *b__.offset(i__ as isize) ^ *c__.offset(i__ as isize);
                    i__ = i__.wrapping_add(1);
                    i__;
                }
            }
        }
        if (*a).used < cn {
            zmemcpy_range((*a).chars, cc, n, m);
        }
    } else if (a == c) as libc::c_int as libc::c_long != 0 {
        let mut a___0: *mut zahl_char_t = (*a).chars;
        let mut b___0: *const zahl_char_t = (*a).chars;
        let mut c___0: *const zahl_char_t = bc;
        let mut i___0: size_t = 0;
        let mut n___0: size_t = n;
        if n___0 <= 4 as libc::c_int as size_t {
            if n___0 >= 1 as libc::c_int as size_t {
                *a___0
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *b___0.offset(0 as libc::c_int as isize)
                    ^ *c___0.offset(0 as libc::c_int as isize);
            }
            if n___0 >= 2 as libc::c_int as size_t {
                *a___0
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *b___0.offset(1 as libc::c_int as isize)
                    ^ *c___0.offset(1 as libc::c_int as isize);
            }
            if n___0 >= 3 as libc::c_int as size_t {
                *a___0
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *b___0.offset(2 as libc::c_int as isize)
                    ^ *c___0.offset(2 as libc::c_int as isize);
            }
            if n___0 >= 4 as libc::c_int as size_t {
                *a___0
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *b___0.offset(3 as libc::c_int as isize)
                    ^ *c___0.offset(3 as libc::c_int as isize);
            }
        } else {
            i___0 = 0 as libc::c_int as size_t;
            loop {
                i___0 = i___0.wrapping_add(4 as libc::c_int as size_t);
                if !(i___0 < n___0) {
                    break;
                }
                *a___0
                    .offset(
                        i___0.wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) = *b___0
                    .offset(i___0.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    ^ *c___0
                        .offset(i___0.wrapping_sub(1 as libc::c_int as size_t) as isize);
                *a___0
                    .offset(
                        i___0.wrapping_sub(2 as libc::c_int as size_t) as isize,
                    ) = *b___0
                    .offset(i___0.wrapping_sub(2 as libc::c_int as size_t) as isize)
                    ^ *c___0
                        .offset(i___0.wrapping_sub(2 as libc::c_int as size_t) as isize);
                *a___0
                    .offset(
                        i___0.wrapping_sub(3 as libc::c_int as size_t) as isize,
                    ) = *b___0
                    .offset(i___0.wrapping_sub(3 as libc::c_int as size_t) as isize)
                    ^ *c___0
                        .offset(i___0.wrapping_sub(3 as libc::c_int as size_t) as isize);
                *a___0
                    .offset(
                        i___0.wrapping_sub(4 as libc::c_int as size_t) as isize,
                    ) = *b___0
                    .offset(i___0.wrapping_sub(4 as libc::c_int as size_t) as isize)
                    ^ *c___0
                        .offset(i___0.wrapping_sub(4 as libc::c_int as size_t) as isize);
            }
            if i___0 > n___0 {
                i___0 = i___0.wrapping_sub(4 as libc::c_int as size_t);
                while i___0 < n___0 {
                    *a___0
                        .offset(
                            i___0 as isize,
                        ) = *b___0.offset(i___0 as isize)
                        ^ *c___0.offset(i___0 as isize);
                    i___0 = i___0.wrapping_add(1);
                    i___0;
                }
            }
        }
        if (*a).used < bn {
            zmemcpy_range((*a).chars, bc, n, m);
        }
    } else if m == bn {
        let mut a___1: *mut zahl_char_t = (*a).chars;
        let mut b___1: *const zahl_char_t = (*c).chars;
        let mut c___1: *const zahl_char_t = (*b).chars;
        let mut i___1: size_t = 0;
        let mut n___1: size_t = n;
        i___1 = 0 as libc::c_int as size_t;
        while i___1 < n___1 {
            *a___1
                .offset(
                    i___1.wrapping_add(0 as libc::c_int as size_t) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(0 as libc::c_int as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(0 as libc::c_int as size_t) as isize);
            *a___1
                .offset(
                    i___1.wrapping_add(1 as libc::c_int as size_t) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(1 as libc::c_int as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(1 as libc::c_int as size_t) as isize);
            *a___1
                .offset(
                    i___1.wrapping_add(2 as libc::c_int as size_t) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(2 as libc::c_int as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(2 as libc::c_int as size_t) as isize);
            *a___1
                .offset(
                    i___1.wrapping_add(3 as libc::c_int as size_t) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(3 as libc::c_int as size_t) as isize)
                ^ *c___1.offset(i___1.wrapping_add(3 as libc::c_int as size_t) as isize);
            i___1 = i___1.wrapping_add(4 as libc::c_int as size_t);
        }
        zmemcpy_range((*a).chars, (*b).chars, n, m);
    } else {
        let mut a___2: *mut zahl_char_t = (*a).chars;
        let mut b___2: *const zahl_char_t = (*b).chars;
        let mut c___2: *const zahl_char_t = (*c).chars;
        let mut i___2: size_t = 0;
        let mut n___2: size_t = n;
        i___2 = 0 as libc::c_int as size_t;
        while i___2 < n___2 {
            *a___2
                .offset(
                    i___2.wrapping_add(0 as libc::c_int as size_t) as isize,
                ) = *b___2
                .offset(i___2.wrapping_add(0 as libc::c_int as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(0 as libc::c_int as size_t) as isize);
            *a___2
                .offset(
                    i___2.wrapping_add(1 as libc::c_int as size_t) as isize,
                ) = *b___2
                .offset(i___2.wrapping_add(1 as libc::c_int as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(1 as libc::c_int as size_t) as isize);
            *a___2
                .offset(
                    i___2.wrapping_add(2 as libc::c_int as size_t) as isize,
                ) = *b___2
                .offset(i___2.wrapping_add(2 as libc::c_int as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(2 as libc::c_int as size_t) as isize);
            *a___2
                .offset(
                    i___2.wrapping_add(3 as libc::c_int as size_t) as isize,
                ) = *b___2
                .offset(i___2.wrapping_add(3 as libc::c_int as size_t) as isize)
                ^ *c___2.offset(i___2.wrapping_add(3 as libc::c_int as size_t) as isize);
            i___2 = i___2.wrapping_add(4 as libc::c_int as size_t);
        }
        zmemcpy_range((*a).chars, (*c).chars, n, m);
    }
    (*a).used = m;
    while (*a).used != 0
        && *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    (*a)
        .sign = if (*a).used != 0 {
        1 as libc::c_int
            - 2 as libc::c_int
                * (zsignum(b) ^ zsignum(c) < 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
