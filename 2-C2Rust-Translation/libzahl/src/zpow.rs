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
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsqr_ll(_: *mut zahl, _: *mut zahl);
    fn free(_: *mut libc::c_void);
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_pow_c: z_t;
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
pub type zerror = libc::c_uint;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
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
unsafe extern "C" fn zodd(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign != 0
        && *((*a).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as zahl_char_t != 0) as libc::c_int;
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
unsafe extern "C" fn zsetu(mut a: *mut zahl, mut b: uint64_t) {
    if (b == 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    if (*a).alloced < 1 as libc::c_int as size_t {
        libzahl_realloc(a, 1 as libc::c_int as size_t);
    }
    (*a).sign = 1 as libc::c_int;
    *((*a).chars).offset(0 as libc::c_int as isize) = b;
    (*a).used = 1 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int as size_t;
    }
    while *((*a).chars)
        .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    rc = ((*a).used * 8 as libc::c_int as size_t)
        .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong);
    rc = rc
        .wrapping_sub(
            (*((*a).chars)
                .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_ulonglong)
                .leading_zeros() as i32 as size_t,
        );
    return rc;
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
#[no_mangle]
pub unsafe extern "C" fn zpow(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0;
    let mut bits: size_t = 0;
    let mut x: zahl_char_t = 0;
    let mut neg: libc::c_int = 0;
    if (zsignum(c) <= 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if zzero(c) != 0 {
            if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
                libzahl_failure(-(ZERROR_0_POW_0 as libc::c_int));
            }
            zsetu(a, 1 as libc::c_int as uint64_t);
        } else if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
            libzahl_failure(-(ZERROR_DIV_0 as libc::c_int));
        } else {
            (*a).sign = 0 as libc::c_int;
        }
        return;
    } else if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    bits = zbits(c);
    n = bits >> 6 as libc::c_int;
    neg = (zsignum(b) < 0 as libc::c_int && zodd(c) != 0) as libc::c_int;
    zabs(libzahl_tmp_pow_b.as_mut_ptr(), b);
    zset(libzahl_tmp_pow_c.as_mut_ptr(), c);
    zsetu(a, 1 as libc::c_int as uint64_t);
    i = 0 as libc::c_int as size_t;
    while i < n {
        x = *((*libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
        j = 64 as libc::c_int as size_t;
        loop {
            let fresh0 = j;
            j = j.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            if x & 1 as libc::c_int as zahl_char_t != 0 {
                zmul_ll(a, a, libzahl_tmp_pow_b.as_mut_ptr());
            }
            zsqr_ll(libzahl_tmp_pow_b.as_mut_ptr(), libzahl_tmp_pow_b.as_mut_ptr());
            x >>= 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    x = *((*libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
    while x != 0 {
        if x & 1 as libc::c_int as zahl_char_t != 0 {
            zmul_ll(a, a, libzahl_tmp_pow_b.as_mut_ptr());
        }
        zsqr_ll(libzahl_tmp_pow_b.as_mut_ptr(), libzahl_tmp_pow_b.as_mut_ptr());
        x >>= 1 as libc::c_int;
    }
    if neg != 0 {
        zneg(a, a);
    }
}
