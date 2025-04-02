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
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    static mut libzahl_tmp_sub: z_t;
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
unsafe extern "C" fn zneg(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign = -(*a).sign;
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
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return -((zzero(b) == 0) as libc::c_int);
    }
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int;
    }
    i = ((*a).used).wrapping_sub(1 as libc::c_int as size_t);
    j = ((*b).used).wrapping_sub(1 as libc::c_int as size_t);
    while i > j {
        if *((*a).chars).offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
        i = i.wrapping_sub(1);
        i;
    }
    while j > i {
        if *((*b).chars).offset(j as isize) != 0 {
            return -(1 as libc::c_int);
        }
        (*b).used = ((*b).used).wrapping_sub(1);
        (*b).used;
        j = j.wrapping_sub(1);
        j;
    }
    while i != 0 && *((*a).chars).offset(i as isize) == *((*b).chars).offset(i as isize)
    {
        i = i.wrapping_sub(1);
        i;
    }
    return if *((*a).chars).offset(i as isize) < *((*b).chars).offset(i as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*a).chars).offset(i as isize) > *((*b).chars).offset(i as isize))
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn zsub_impl(mut a: *mut zahl, mut b: *mut zahl, mut n: size_t) {
    let mut carry: zahl_char_t = 0 as libc::c_int as zahl_char_t;
    let mut tcarry: zahl_char_t = 0;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        tcarry = (if carry != 0 {
            (*((*a).chars).offset(i as isize) <= *((*b).chars).offset(i as isize))
                as libc::c_int
        } else {
            (*((*a).chars).offset(i as isize) < *((*b).chars).offset(i as isize))
                as libc::c_int
        }) as zahl_char_t;
        let ref mut fresh0 = *((*a).chars).offset(i as isize);
        *fresh0 = (*fresh0).wrapping_sub(*((*b).chars).offset(i as isize));
        let ref mut fresh1 = *((*a).chars).offset(i as isize);
        *fresh1 = (*fresh1).wrapping_sub(carry);
        carry = tcarry;
        i = i.wrapping_add(1);
        i;
    }
    if carry != 0 {
        while *((*a).chars).offset(i as isize) == 0 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *((*a).chars)
                .offset(fresh2 as isize) = 18446744073709551615 as libc::c_ulong;
        }
        if *((*a).chars).offset(i as isize) == 1 as libc::c_int as zahl_char_t {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        } else {
            let ref mut fresh3 = *((*a).chars).offset(i as isize);
            *fresh3 = (*fresh3).wrapping_sub(1 as libc::c_int as zahl_char_t);
        }
    }
}
#[inline]
unsafe extern "C" fn libzahl_zsub_unsigned(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    let mut magcmp: libc::c_int = 0;
    let mut n: size_t = 0;
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        zabs(a, c);
        zneg(a, a);
        return;
    } else if (zzero(c) != 0) as libc::c_int as libc::c_long != 0 {
        zabs(a, b);
        return;
    }
    magcmp = zcmpmag(b, c);
    if (magcmp <= 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if (magcmp == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            (*a).sign = 0 as libc::c_int;
            return;
        }
        n = (*b).used;
        if a == b {
            zset(libzahl_tmp_sub.as_mut_ptr(), b);
            if a != c {
                zset(a, c);
            }
            zsub_impl(a, libzahl_tmp_sub.as_mut_ptr(), n);
        } else {
            if a != c {
                zset(a, c);
            }
            zsub_impl(a, b, n);
        }
    } else {
        n = (*c).used;
        if (a == c) as libc::c_int as libc::c_long != 0 {
            zset(libzahl_tmp_sub.as_mut_ptr(), c);
            if a != b {
                zset(a, b);
            }
            zsub_impl(a, libzahl_tmp_sub.as_mut_ptr(), n);
        } else {
            if a != b {
                zset(a, b);
            }
            zsub_impl(a, c, n);
        }
    }
    (*a).sign = magcmp;
}
#[no_mangle]
pub unsafe extern "C" fn zsub_unsigned(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    libzahl_zsub_unsigned(a, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn zsub_nonnegative_assign(mut a: *mut zahl, mut b: *mut zahl) {
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        zabs(a, a);
    } else if (zcmpmag(a, b) == 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        zsub_impl(a, b, (*b).used);
    };
}
#[no_mangle]
pub unsafe extern "C" fn zsub_positive_assign(mut a: *mut zahl, mut b: *mut zahl) {
    zsub_impl(a, b, (*b).used);
}
#[no_mangle]
pub unsafe extern "C" fn zsub(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        zneg(a, c);
    } else if (zzero(c) != 0) as libc::c_int as libc::c_long != 0 {
        if a != b {
            zset(a, b);
        }
    } else if (zsignum(b) < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if zsignum(c) < 0 as libc::c_int {
            libzahl_zsub_unsigned(a, c, b);
        } else {
            zadd_unsigned(a, b, c);
            (*a).sign = -zsignum(a);
        }
    } else if zsignum(c) < 0 as libc::c_int {
        zadd_unsigned(a, b, c);
    } else {
        libzahl_zsub_unsigned(a, b, c);
    };
}
