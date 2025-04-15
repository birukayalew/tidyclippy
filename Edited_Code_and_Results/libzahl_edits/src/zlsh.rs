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
pub type __ssize_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
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
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn libzahl_memset_precise(
    mut a: *mut zahl_char_t,
    mut v: zahl_char_t,
    mut n: size_t,
) {
    let mut i: size_t = 0;
    if n <= 4 as libc::c_int as size_t {
        if n >= 1 as libc::c_int as size_t {
            *a.offset(0 as libc::c_int as isize) = v;
        }
        if n >= 2 as libc::c_int as size_t {
            *a.offset(1 as libc::c_int as isize) = v;
        }
        if n >= 3 as libc::c_int as size_t {
            *a.offset(2 as libc::c_int as isize) = v;
        }
        if n >= 4 as libc::c_int as size_t {
            *a.offset(3 as libc::c_int as isize) = v;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        loop {
            i = i.wrapping_add(4 as libc::c_int as size_t);
            if !(i <= n) {
                break;
            }
            *a.offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize) = v;
            *a.offset(i.wrapping_sub(2 as libc::c_int as size_t) as isize) = v;
            *a.offset(i.wrapping_sub(3 as libc::c_int as size_t) as isize) = v;
            *a.offset(i.wrapping_sub(4 as libc::c_int as size_t) as isize) = v;
        }
        if i > n {
            i = i.wrapping_sub(4 as libc::c_int as size_t);
            while i < n {
                *a.offset(i as isize) = v;
                i = i.wrapping_add(1);
                i;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmoveb(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut i: ssize_t = 0;
    let mut current_block_47: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 6587057311241304070;
        }
        19 => {
            current_block_47 = 6587057311241304070;
        }
        18 => {
            current_block_47 = 8270668366830825409;
        }
        17 => {
            current_block_47 = 6382675588328852115;
        }
        16 => {
            current_block_47 = 17687827032931720921;
        }
        15 => {
            current_block_47 = 5446065646654037394;
        }
        14 => {
            current_block_47 = 5561858650283740197;
        }
        13 => {
            current_block_47 = 3744832089702322480;
        }
        12 => {
            current_block_47 = 4597665499950114931;
        }
        11 => {
            current_block_47 = 4997459678070010420;
        }
        10 => {
            current_block_47 = 17917876647522425601;
        }
        9 => {
            current_block_47 = 186392437941418340;
        }
        8 => {
            current_block_47 = 11535604127128287556;
        }
        7 => {
            current_block_47 = 18432424578507615774;
        }
        6 => {
            current_block_47 = 12289033752759627105;
        }
        5 => {
            current_block_47 = 8724207398491971587;
        }
        4 => {
            current_block_47 = 18224123963623233877;
        }
        3 => {
            current_block_47 = 13255373871257710942;
        }
        2 => {
            current_block_47 = 1803792003068296022;
        }
        1 => {
            current_block_47 = 525324511667026418;
        }
        0 => {
            current_block_47 = 7226443171521532240;
        }
        _ => {
            i = n as ssize_t + 3 as libc::c_int as ssize_t
                & !(3 as libc::c_int) as ssize_t;
            loop {
                i -= 4 as libc::c_int as ssize_t;
                if !(i >= 0 as libc::c_int as ssize_t) {
                    break;
                }
                *d
                    .offset(
                        (i + 3 as libc::c_int as ssize_t) as isize,
                    ) = *s.offset((i + 3 as libc::c_int as ssize_t) as isize);
                *d
                    .offset(
                        (i + 2 as libc::c_int as ssize_t) as isize,
                    ) = *s.offset((i + 2 as libc::c_int as ssize_t) as isize);
                *d
                    .offset(
                        (i + 1 as libc::c_int as ssize_t) as isize,
                    ) = *s.offset((i + 1 as libc::c_int as ssize_t) as isize);
                *d
                    .offset(
                        (i + 0 as libc::c_int as ssize_t) as isize,
                    ) = *s.offset((i + 0 as libc::c_int as ssize_t) as isize);
            }
            current_block_47 = 7226443171521532240;
        }
    }
    match current_block_47 {
        6587057311241304070 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 8270668366830825409;
        }
        _ => {}
    }
    match current_block_47 {
        8270668366830825409 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 6382675588328852115;
        }
        _ => {}
    }
    match current_block_47 {
        6382675588328852115 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 17687827032931720921;
        }
        _ => {}
    }
    match current_block_47 {
        17687827032931720921 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 5446065646654037394;
        }
        _ => {}
    }
    match current_block_47 {
        5446065646654037394 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 5561858650283740197;
        }
        _ => {}
    }
    match current_block_47 {
        5561858650283740197 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 3744832089702322480;
        }
        _ => {}
    }
    match current_block_47 {
        3744832089702322480 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 4597665499950114931;
        }
        _ => {}
    }
    match current_block_47 {
        4597665499950114931 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 4997459678070010420;
        }
        _ => {}
    }
    match current_block_47 {
        4997459678070010420 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 17917876647522425601;
        }
        _ => {}
    }
    match current_block_47 {
        17917876647522425601 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 186392437941418340;
        }
        _ => {}
    }
    match current_block_47 {
        186392437941418340 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 11535604127128287556;
        }
        _ => {}
    }
    match current_block_47 {
        11535604127128287556 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 18432424578507615774;
        }
        _ => {}
    }
    match current_block_47 {
        18432424578507615774 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 12289033752759627105;
        }
        _ => {}
    }
    match current_block_47 {
        12289033752759627105 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 8724207398491971587;
        }
        _ => {}
    }
    match current_block_47 {
        8724207398491971587 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 18224123963623233877;
        }
        _ => {}
    }
    match current_block_47 {
        18224123963623233877 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 13255373871257710942;
        }
        _ => {}
    }
    match current_block_47 {
        13255373871257710942 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 1803792003068296022;
        }
        _ => {}
    }
    match current_block_47 {
        1803792003068296022 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 525324511667026418;
        }
        _ => {}
    }
    match current_block_47 {
        525324511667026418 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
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
#[no_mangle]
pub unsafe extern "C" fn zlsh(mut a: *mut zahl, mut b: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    let mut carry: zahl_char_t = 0 as libc::c_int as zahl_char_t;
    let mut tcarry: zahl_char_t = 0;
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    chars = bits >> 6 as libc::c_int;
    bits = bits & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    cbits = (64 as libc::c_int as size_t).wrapping_sub(bits);
    if (*a).alloced
        < ((*b).used).wrapping_add(chars).wrapping_add(1 as libc::c_int as size_t)
    {
        libzahl_realloc(
            a,
            ((*b).used).wrapping_add(chars).wrapping_add(1 as libc::c_int as size_t),
        );
    }
    if (a == b) as libc::c_int as libc::c_long != 0 {
        libzahl_memmoveb(((*a).chars).offset(chars as isize), (*b).chars, (*b).used);
    } else {
        libzahl_memcpy(((*a).chars).offset(chars as isize), (*b).chars, (*b).used);
    }
    libzahl_memset_precise((*a).chars, 0 as libc::c_int as zahl_char_t, chars);
    (*a).used = ((*b).used).wrapping_add(chars);
    if (bits != 0) as libc::c_int as libc::c_long != 0 {
        i = chars;
        while i < (*a).used {
            tcarry = *((*a).chars).offset(i as isize) >> cbits;
            *((*a).chars).offset(i as isize) <<= bits;
            *((*a).chars).offset(i as isize) |= carry;
            carry = tcarry;
            i = i.wrapping_add(1);
            i;
        }
        if carry != 0 {
            let fresh0 = (*a).used;
            (*a).used = ((*a).used).wrapping_add(1);
            *((*a).chars).offset(fresh0 as isize) = carry;
        }
    }
    (*a).sign = zsignum(b);
}
