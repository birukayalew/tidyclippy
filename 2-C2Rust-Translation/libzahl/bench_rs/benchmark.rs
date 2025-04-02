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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    static mut libzahl_tmp_div: [zahl; 1];
    static mut libzahl_tmp_mod: [zahl; 1];
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zsetup(_: *mut __jmp_buf_tag);
    fn zunsetup();
    fn zfree(_: *mut zahl);
    fn zload(_: *mut zahl, _: *const libc::c_void) -> size_t;
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmodmul(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zdivmod(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmodsqr(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zpow(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmodpow(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zpowu(_: *mut zahl, _: *mut zahl, _: libc::c_ulonglong);
    fn zmodpowu(_: *mut zahl, _: *mut zahl, _: libc::c_ulonglong, _: *mut zahl);
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zand(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zor(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zxor(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn znot(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn ztrunc(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zgcd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zptest(_: *mut zahl, _: *mut zahl, _: libc::c_int) -> zprimality;
    fn zrand(_: *mut zahl, _: zranddev, _: zranddist, _: *mut zahl);
    fn zstr(_: *mut zahl, _: *mut libc::c_char, _: size_t) -> *mut libc::c_char;
    fn zsets(_: *mut zahl, _: *const libc::c_char) -> libc::c_int;
    fn zstr_length(_: *mut zahl, _: libc::c_ulonglong) -> size_t;
    fn zperror(_: *const libc::c_char);
    fn zbset_ll_set(_: *mut zahl, _: size_t);
    fn zbset_ll_clear(_: *mut zahl, _: size_t);
    fn zbset_ll_flip(_: *mut zahl, _: size_t);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsqr_ll(_: *mut zahl, _: *mut zahl);
    static mut timebuf: [libc::c_char; 512];
    static mut freq: libc::c_ulonglong;
    fn benchmark_init();
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub type zprimality = libc::c_uint;
pub const PRIME: zprimality = 2;
pub const PROBABLY_PRIME: zprimality = 1;
pub const NONPRIME: zprimality = 0;
pub type zranddev = libc::c_uint;
pub const LIBC_RAND48_RANDOM: zranddev = 6;
pub const LIBC_RANDOM_RANDOM: zranddev = 5;
pub const LIBC_RAND_RANDOM: zranddev = 4;
pub const FASTEST_RANDOM: zranddev = 3;
pub const DEFAULT_RANDOM: zranddev = 2;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = libc::c_uint;
pub const MODUNIFORM: zranddist = 2;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
pub type rdtsc_t = libc::c_ulonglong;
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
            current_block_42 = 1593741079485388122;
        }
        19 => {
            current_block_42 = 1593741079485388122;
        }
        18 => {
            current_block_42 = 3835136233266689143;
        }
        17 => {
            current_block_42 = 5014865779978224114;
        }
        16 => {
            current_block_42 = 7524173820000460458;
        }
        15 => {
            current_block_42 = 5918751339751240251;
        }
        14 => {
            current_block_42 = 8104518946956323795;
        }
        13 => {
            current_block_42 = 3505445475423316423;
        }
        12 => {
            current_block_42 = 9386991487288681626;
        }
        11 => {
            current_block_42 = 4888706221226997782;
        }
        10 => {
            current_block_42 = 13404399150746970173;
        }
        9 => {
            current_block_42 = 11016356878856320980;
        }
        8 => {
            current_block_42 = 9462696770877435158;
        }
        7 => {
            current_block_42 = 5111859029198025704;
        }
        6 => {
            current_block_42 = 1462253450244455034;
        }
        5 => {
            current_block_42 = 14281147861434119243;
        }
        4 => {
            current_block_42 = 2670656178806173362;
        }
        3 => {
            current_block_42 = 14850480777090065785;
        }
        2 => {
            current_block_42 = 766553780809049687;
        }
        1 => {
            current_block_42 = 1653818194578356337;
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
        1593741079485388122 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3835136233266689143;
        }
        _ => {}
    }
    match current_block_42 {
        3835136233266689143 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5014865779978224114;
        }
        _ => {}
    }
    match current_block_42 {
        5014865779978224114 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7524173820000460458;
        }
        _ => {}
    }
    match current_block_42 {
        7524173820000460458 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5918751339751240251;
        }
        _ => {}
    }
    match current_block_42 {
        5918751339751240251 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8104518946956323795;
        }
        _ => {}
    }
    match current_block_42 {
        8104518946956323795 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3505445475423316423;
        }
        _ => {}
    }
    match current_block_42 {
        3505445475423316423 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 9386991487288681626;
        }
        _ => {}
    }
    match current_block_42 {
        9386991487288681626 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4888706221226997782;
        }
        _ => {}
    }
    match current_block_42 {
        4888706221226997782 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13404399150746970173;
        }
        _ => {}
    }
    match current_block_42 {
        13404399150746970173 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11016356878856320980;
        }
        _ => {}
    }
    match current_block_42 {
        11016356878856320980 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 9462696770877435158;
        }
        _ => {}
    }
    match current_block_42 {
        9462696770877435158 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5111859029198025704;
        }
        _ => {}
    }
    match current_block_42 {
        5111859029198025704 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1462253450244455034;
        }
        _ => {}
    }
    match current_block_42 {
        1462253450244455034 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14281147861434119243;
        }
        _ => {}
    }
    match current_block_42 {
        14281147861434119243 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 2670656178806173362;
        }
        _ => {}
    }
    match current_block_42 {
        2670656178806173362 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14850480777090065785;
        }
        _ => {}
    }
    match current_block_42 {
        14850480777090065785 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 766553780809049687;
        }
        _ => {}
    }
    match current_block_42 {
        766553780809049687 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1653818194578356337;
        }
        _ => {}
    }
    match current_block_42 {
        1653818194578356337 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zinit(mut a: *mut zahl) {
    (*a).alloced = 0 as libc::c_int as size_t;
    (*a).chars = 0 as *mut zahl_char_t;
}
#[inline]
unsafe extern "C" fn zeven(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0
        || !*((*a).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as zahl_char_t != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zodd(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign != 0
        && *((*a).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as zahl_char_t != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zeven_nonzero(mut a: *mut zahl) -> libc::c_int {
    return (!*((*a).chars).offset(0 as libc::c_int as isize)
        & 1 as libc::c_int as zahl_char_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zodd_nonzero(mut a: *mut zahl) -> libc::c_int {
    return (*((*a).chars).offset(0 as libc::c_int as isize)
        & 1 as libc::c_int as zahl_char_t) as libc::c_int;
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
unsafe extern "C" fn zswap(mut a_: *mut zahl, mut b_: *mut zahl) {
    let mut t: libc::c_long = 0;
    let mut a: *mut libc::c_long = a_ as *mut libc::c_long;
    let mut b: *mut libc::c_long = b_ as *mut libc::c_long;
    t = *a.offset(0 as libc::c_int as isize);
    *a.offset(0 as libc::c_int as isize) = *b.offset(0 as libc::c_int as isize);
    *b.offset(0 as libc::c_int as isize) = t;
    t = *b.offset(1 as libc::c_int as isize);
    *b.offset(1 as libc::c_int as isize) = *a.offset(1 as libc::c_int as isize);
    *a.offset(1 as libc::c_int as isize) = t;
    t = *a.offset(2 as libc::c_int as isize);
    *a.offset(2 as libc::c_int as isize) = *b.offset(2 as libc::c_int as isize);
    *b.offset(2 as libc::c_int as isize) = t;
    t = *b.offset(3 as libc::c_int as isize);
    *b.offset(3 as libc::c_int as isize) = *a.offset(3 as libc::c_int as isize);
    *a.offset(3 as libc::c_int as isize) = t;
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
unsafe extern "C" fn zseti(mut a: *mut zahl, mut b: int64_t) {
    if (b >= 0 as libc::c_int as int64_t) as libc::c_int as libc::c_long != 0 {
        zsetu(a, b as uint64_t);
        return;
    }
    if (*a).alloced < 1 as libc::c_int as size_t {
        libzahl_realloc(a, 1 as libc::c_int as size_t);
    }
    (*a).sign = -(1 as libc::c_int);
    *((*a).chars).offset(0 as libc::c_int as isize) = -b as zahl_char_t;
    (*a).used = 1 as libc::c_int as size_t;
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
unsafe extern "C" fn zlsb(mut a: *mut zahl) -> size_t {
    let mut i: size_t = 0 as libc::c_int as size_t;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return 18446744073709551615 as libc::c_ulong;
    }
    while *((*a).chars).offset(i as isize) == 0 {
        i = i.wrapping_add(1);
        i;
    }
    i = (i as libc::c_ulong)
        .wrapping_mul(
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        ) as size_t as size_t;
    i = i
        .wrapping_add(
            (*((*a).chars).offset(i as isize) as libc::c_ulonglong).trailing_zeros()
                as i32 as size_t,
        );
    return i;
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
unsafe extern "C" fn zcmp(mut a: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    if zsignum(a) != zsignum(b) {
        return if zsignum(a) < zsignum(b) {
            -(1 as libc::c_int)
        } else {
            (zsignum(a) > zsignum(b)) as libc::c_int
        };
    }
    return zsignum(a) * zcmpmag(a, b);
}
#[inline]
unsafe extern "C" fn zcmpu(mut a: *mut zahl, mut b: uint64_t) -> libc::c_int {
    if (b == 0) as libc::c_int as libc::c_long != 0 {
        return zsignum(a);
    }
    if (zsignum(a) <= 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    while *((*a).chars)
        .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    if (*a).used > 1 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    return if *((*a).chars).offset(0 as libc::c_int as isize) < b {
        -(1 as libc::c_int)
    } else {
        (*((*a).chars).offset(0 as libc::c_int as isize) > b) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn zcmpi(mut a: *mut zahl, mut b: int64_t) -> libc::c_int {
    if (b == 0) as libc::c_int as libc::c_long != 0 {
        return zsignum(a);
    }
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return if (b < 0 as libc::c_int as int64_t) as libc::c_int as libc::c_long != 0 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if (b < 0 as libc::c_int as int64_t) as libc::c_int as libc::c_long != 0 {
        if zsignum(a) > 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        while *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
        {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        }
        if (*a).used > 1 as libc::c_int as size_t {
            return -(1 as libc::c_int);
        }
        return if *((*a).chars).offset(0 as libc::c_int as isize) > -b as zahl_char_t {
            -(1 as libc::c_int)
        } else {
            (*((*a).chars).offset(0 as libc::c_int as isize) < -b as zahl_char_t)
                as libc::c_int
        };
    } else {
        if zsignum(a) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        while *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
        {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        }
        if (*a).used > 1 as libc::c_int as size_t {
            return 1 as libc::c_int;
        }
        return if *((*a).chars).offset(0 as libc::c_int as isize) < b as zahl_char_t {
            -(1 as libc::c_int)
        } else {
            (*((*a).chars).offset(0 as libc::c_int as isize) > b as zahl_char_t)
                as libc::c_int
        };
    };
}
#[inline]
unsafe extern "C" fn zbset(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut bit: size_t,
    mut action: libc::c_int,
) {
    if (a != b) as libc::c_int as libc::c_long != 0 {
        zset(a, b);
    }
    if 0 != 0 && 0 != 0 {
        let mut mask: zahl_char_t = 1 as libc::c_int as zahl_char_t;
        if zzero(a) != 0 || bit >> 6 as libc::c_int >= (*a).used {
            if action == 0 {
                return;
            }
        } else {
            mask <<= bit & (64 as libc::c_int - 1 as libc::c_int) as size_t;
            if action > 0 as libc::c_int {
                *((*a).chars).offset((bit >> 6 as libc::c_int) as isize) |= mask;
                return;
            } else if action < 0 as libc::c_int {
                *((*a).chars).offset((bit >> 6 as libc::c_int) as isize) ^= mask;
            } else {
                *((*a).chars).offset((bit >> 6 as libc::c_int) as isize) &= !mask;
            }
            while (*a).used != 0
                && *((*a).chars)
                    .offset(
                        ((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) == 0
            {
                (*a).used = ((*a).used).wrapping_sub(1);
                (*a).used;
            }
            if (*a).used == 0 {
                (*a).sign = 0 as libc::c_int;
            }
            return;
        }
    }
    if action > 0 as libc::c_int {
        zbset_ll_set(a, bit);
    } else if action < 0 as libc::c_int {
        zbset_ll_flip(a, bit);
    } else {
        zbset_ll_clear(a, bit);
    };
}
#[inline]
unsafe extern "C" fn zbtest(mut a: *mut zahl, mut bit: size_t) -> libc::c_int {
    let mut chars: size_t = 0;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int;
    }
    chars = bit >> 6 as libc::c_int;
    if (chars >= (*a).used) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int;
    }
    bit &= bit & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    return (*((*a).chars).offset(chars as isize) >> bit
        & 1 as libc::c_int as zahl_char_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsplit(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut delim: size_t,
) {
    if (high == a) as libc::c_int as libc::c_long != 0 {
        ztrunc(low, a, delim);
        zrsh(high, a, delim);
    } else {
        zrsh(high, a, delim);
        ztrunc(low, a, delim);
    };
}
#[inline]
unsafe extern "C" fn zsave(mut a: *mut zahl, mut buffer: *mut libc::c_void) -> size_t {
    if !buffer.is_null() as libc::c_int as libc::c_long != 0 {
        let mut buf: *mut libc::c_char = buffer as *mut libc::c_char;
        *(buf as *mut libc::c_long) = (*a).sign as libc::c_long;
        buf = buf
            .offset(::core::mem::size_of::<libc::c_long>() as libc::c_ulong as isize);
        *(buf as *mut size_t) = (*a).used;
        buf = buf.offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
        if (zzero(a) == 0) as libc::c_int as libc::c_long != 0 {
            *((*a).chars)
                .offset(
                    ((*a).used).wrapping_add(2 as libc::c_int as size_t) as isize,
                ) = 0 as libc::c_int as zahl_char_t;
            *((*a).chars)
                .offset(
                    ((*a).used).wrapping_add(1 as libc::c_int as size_t) as isize,
                ) = 0 as libc::c_int as zahl_char_t;
            *((*a).chars)
                .offset(
                    ((*a).used).wrapping_add(0 as libc::c_int as size_t) as isize,
                ) = 0 as libc::c_int as zahl_char_t;
            libzahl_memcpy(buf as *mut zahl_char_t, (*a).chars, (*a).used);
        }
    }
    return (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(
            (if zzero(a) != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (((*a).used).wrapping_add(3 as libc::c_int as size_t)
                    & !(3 as libc::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong)
            }),
        );
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: libc::c_int = 0;
    let mut c_sign: libc::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
}
#[inline]
unsafe extern "C" fn zsqr(mut a: *mut zahl, mut b: *mut zahl) {
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        zsqr_ll(a, b);
        (*a).sign = 1 as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn zdiv(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    zdivmod(a, libzahl_tmp_div.as_mut_ptr(), b, c);
}
#[inline]
unsafe extern "C" fn zmod(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    zdivmod(libzahl_tmp_mod.as_mut_ptr(), a, b, c);
}
static mut dur: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
static mut start_high: libc::c_uint = 0;
static mut start_low: libc::c_uint = 0;
static mut end_high: libc::c_uint = 0;
static mut end_low: libc::c_uint = 0;
#[inline]
unsafe extern "C" fn rdtsc(mut low: *mut libc::c_uint, mut high: *mut libc::c_uint) {
    asm!(
        "rdtsc", lateout("ax") * low, lateout("dx") * high, options(preserves_flags,
        att_syntax)
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 2000] = [0; 2000];
    let mut a: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    let mut b: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    let mut c: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    let mut d: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    let mut tiny: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    let mut jmp: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut i: size_t = 0;
    benchmark_init();
    if _setjmp(jmp.as_mut_ptr()) != 0 {
        zperror(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    zsetup(jmp.as_mut_ptr());
    zinit(a.as_mut_ptr());
    zinit(b.as_mut_ptr());
    zinit(c.as_mut_ptr());
    zinit(d.as_mut_ptr());
    zinit(tiny.as_mut_ptr());
    zsets(
        a.as_mut_ptr(),
        b"5495468234592964023447280368442884381000481887\0" as *const u8
            as *const libc::c_char,
    );
    zsets(
        b.as_mut_ptr(),
        b"4781084818570683458641843084358135840548636081\0" as *const u8
            as *const libc::c_char,
    );
    zsets(tiny.as_mut_ptr(), b"5\0" as *const u8 as *const libc::c_char);
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        zset(c.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles: rdtsc_t = 0;
    let mut dur_seconds: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles = dur_cycles
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds = dur_cycles as libc::c_double;
    dur_seconds /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds as libc::c_int as __time_t;
    dur_seconds -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zset(c, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        zseti(c.as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
    }
    let mut dur_cycles_0: rdtsc_t = 0;
    let mut dur_seconds_0: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_0 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_0 = dur_cycles_0
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_0 = dur_cycles_0 as libc::c_double;
    dur_seconds_0 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_0 as libc::c_int as __time_t;
    dur_seconds_0 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_0 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zseti(c, 1000000000LL)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh2 = i;
        i = i.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        zsetu(c.as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
    }
    let mut dur_cycles_1: rdtsc_t = 0;
    let mut dur_seconds_1: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_1 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_1 = dur_cycles_1
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_1 = dur_cycles_1 as libc::c_double;
    dur_seconds_1 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_1 as libc::c_int as __time_t;
    dur_seconds_1 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_1 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsetu(c, 1000000000ULL)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh3 = i;
        i = i.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        zneg(c.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_2: rdtsc_t = 0;
    let mut dur_seconds_2: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_2 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_2 = dur_cycles_2
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_2 = dur_cycles_2 as libc::c_double;
    dur_seconds_2 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_2 as libc::c_int as __time_t;
    dur_seconds_2 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_2 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zneg(c, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh4 = i;
        i = i.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        zneg(a.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_3: rdtsc_t = 0;
    let mut dur_seconds_3: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_3 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_3 = dur_cycles_3
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_3 = dur_cycles_3 as libc::c_double;
    dur_seconds_3 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_3 as libc::c_int as __time_t;
    dur_seconds_3 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_3 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zneg(a, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh5 = i;
        i = i.wrapping_sub(1);
        if !(fresh5 != 0) {
            break;
        }
        zabs(c.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_4: rdtsc_t = 0;
    let mut dur_seconds_4: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_4 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_4 = dur_cycles_4
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_4 = dur_cycles_4 as libc::c_double;
    dur_seconds_4 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_4 as libc::c_int as __time_t;
    dur_seconds_4 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_4 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zabs(c, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh6 = i;
        i = i.wrapping_sub(1);
        if !(fresh6 != 0) {
            break;
        }
        zabs(a.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_5: rdtsc_t = 0;
    let mut dur_seconds_5: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_5 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_5 = dur_cycles_5
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_5 = dur_cycles_5 as libc::c_double;
    dur_seconds_5 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_5 as libc::c_int as __time_t;
    dur_seconds_5 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_5 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zabs(a, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh7 = i;
        i = i.wrapping_sub(1);
        if !(fresh7 != 0) {
            break;
        }
        zadd_unsigned(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_6: rdtsc_t = 0;
    let mut dur_seconds_6: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_6 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_6 = dur_cycles_6
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_6 = dur_cycles_6 as libc::c_double;
    dur_seconds_6 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_6 as libc::c_int as __time_t;
    dur_seconds_6 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_6 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zadd_unsigned(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh8 = i;
        i = i.wrapping_sub(1);
        if !(fresh8 != 0) {
            break;
        }
        zsub_unsigned(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_7: rdtsc_t = 0;
    let mut dur_seconds_7: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_7 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_7 = dur_cycles_7
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_7 = dur_cycles_7 as libc::c_double;
    dur_seconds_7 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_7 as libc::c_int as __time_t;
    dur_seconds_7 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_7 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsub_unsigned(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh9 = i;
        i = i.wrapping_sub(1);
        if !(fresh9 != 0) {
            break;
        }
        zadd(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_8: rdtsc_t = 0;
    let mut dur_seconds_8: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_8 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_8 = dur_cycles_8
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_8 = dur_cycles_8 as libc::c_double;
    dur_seconds_8 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_8 as libc::c_int as __time_t;
    dur_seconds_8 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_8 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zadd(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh10 = i;
        i = i.wrapping_sub(1);
        if !(fresh10 != 0) {
            break;
        }
        zsub(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_9: rdtsc_t = 0;
    let mut dur_seconds_9: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_9 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_9 = dur_cycles_9
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_9 = dur_cycles_9 as libc::c_double;
    dur_seconds_9 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_9 as libc::c_int as __time_t;
    dur_seconds_9 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_9 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsub(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh11 = i;
        i = i.wrapping_sub(1);
        if !(fresh11 != 0) {
            break;
        }
        zand(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_10: rdtsc_t = 0;
    let mut dur_seconds_10: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_10 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_10 = dur_cycles_10
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_10 = dur_cycles_10 as libc::c_double;
    dur_seconds_10 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_10 as libc::c_int as __time_t;
    dur_seconds_10 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_10 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zand(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh12 = i;
        i = i.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        zor(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_11: rdtsc_t = 0;
    let mut dur_seconds_11: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_11 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_11 = dur_cycles_11
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_11 = dur_cycles_11 as libc::c_double;
    dur_seconds_11 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_11 as libc::c_int as __time_t;
    dur_seconds_11 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_11 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zor(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh13 = i;
        i = i.wrapping_sub(1);
        if !(fresh13 != 0) {
            break;
        }
        zxor(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_12: rdtsc_t = 0;
    let mut dur_seconds_12: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_12 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_12 = dur_cycles_12
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_12 = dur_cycles_12 as libc::c_double;
    dur_seconds_12 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_12 as libc::c_int as __time_t;
    dur_seconds_12 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_12 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zxor(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh14 = i;
        i = i.wrapping_sub(1);
        if !(fresh14 != 0) {
            break;
        }
        znot(c.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_13: rdtsc_t = 0;
    let mut dur_seconds_13: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_13 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_13 = dur_cycles_13
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_13 = dur_cycles_13 as libc::c_double;
    dur_seconds_13 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_13 as libc::c_int as __time_t;
    dur_seconds_13 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_13 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"znot(c, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh15 = i;
        i = i.wrapping_sub(1);
        if !(fresh15 != 0) {
            break;
        }
        zeven(a.as_mut_ptr());
    }
    let mut dur_cycles_14: rdtsc_t = 0;
    let mut dur_seconds_14: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_14 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_14 = dur_cycles_14
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_14 = dur_cycles_14 as libc::c_double;
    dur_seconds_14 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_14 as libc::c_int as __time_t;
    dur_seconds_14 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_14 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zeven(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh16 = i;
        i = i.wrapping_sub(1);
        if !(fresh16 != 0) {
            break;
        }
        zodd(a.as_mut_ptr());
    }
    let mut dur_cycles_15: rdtsc_t = 0;
    let mut dur_seconds_15: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_15 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_15 = dur_cycles_15
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_15 = dur_cycles_15 as libc::c_double;
    dur_seconds_15 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_15 as libc::c_int as __time_t;
    dur_seconds_15 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_15 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zodd(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh17 = i;
        i = i.wrapping_sub(1);
        if !(fresh17 != 0) {
            break;
        }
        zeven_nonzero(a.as_mut_ptr());
    }
    let mut dur_cycles_16: rdtsc_t = 0;
    let mut dur_seconds_16: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_16 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_16 = dur_cycles_16
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_16 = dur_cycles_16 as libc::c_double;
    dur_seconds_16 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_16 as libc::c_int as __time_t;
    dur_seconds_16 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_16 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zeven_nonzero(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh18 = i;
        i = i.wrapping_sub(1);
        if !(fresh18 != 0) {
            break;
        }
        zodd_nonzero(a.as_mut_ptr());
    }
    let mut dur_cycles_17: rdtsc_t = 0;
    let mut dur_seconds_17: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_17 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_17 = dur_cycles_17
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_17 = dur_cycles_17 as libc::c_double;
    dur_seconds_17 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_17 as libc::c_int as __time_t;
    dur_seconds_17 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_17 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zodd_nonzero(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh19 = i;
        i = i.wrapping_sub(1);
        if !(fresh19 != 0) {
            break;
        }
        zzero(a.as_mut_ptr());
    }
    let mut dur_cycles_18: rdtsc_t = 0;
    let mut dur_seconds_18: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_18 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_18 = dur_cycles_18
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_18 = dur_cycles_18 as libc::c_double;
    dur_seconds_18 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_18 as libc::c_int as __time_t;
    dur_seconds_18 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_18 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zzero(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh20 = i;
        i = i.wrapping_sub(1);
        if !(fresh20 != 0) {
            break;
        }
        zsignum(a.as_mut_ptr());
    }
    let mut dur_cycles_19: rdtsc_t = 0;
    let mut dur_seconds_19: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_19 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_19 = dur_cycles_19
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_19 = dur_cycles_19 as libc::c_double;
    dur_seconds_19 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_19 as libc::c_int as __time_t;
    dur_seconds_19 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_19 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsignum(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh21 = i;
        i = i.wrapping_sub(1);
        if !(fresh21 != 0) {
            break;
        }
        zbits(a.as_mut_ptr());
    }
    let mut dur_cycles_20: rdtsc_t = 0;
    let mut dur_seconds_20: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_20 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_20 = dur_cycles_20
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_20 = dur_cycles_20 as libc::c_double;
    dur_seconds_20 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_20 as libc::c_int as __time_t;
    dur_seconds_20 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_20 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbits(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh22 = i;
        i = i.wrapping_sub(1);
        if !(fresh22 != 0) {
            break;
        }
        zlsb(a.as_mut_ptr());
    }
    let mut dur_cycles_21: rdtsc_t = 0;
    let mut dur_seconds_21: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_21 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_21 = dur_cycles_21
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_21 = dur_cycles_21 as libc::c_double;
    dur_seconds_21 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_21 as libc::c_int as __time_t;
    dur_seconds_21 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_21 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zlsb(a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh23 = i;
        i = i.wrapping_sub(1);
        if !(fresh23 != 0) {
            break;
        }
        zswap(a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_22: rdtsc_t = 0;
    let mut dur_seconds_22: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_22 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_22 = dur_cycles_22
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_22 = dur_cycles_22 as libc::c_double;
    dur_seconds_22 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_22 as libc::c_int as __time_t;
    dur_seconds_22 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_22 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zswap(a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh24 = i;
        i = i.wrapping_sub(1);
        if !(fresh24 != 0) {
            break;
        }
        zlsh(c.as_mut_ptr(), a.as_mut_ptr(), 76 as libc::c_int as size_t);
    }
    let mut dur_cycles_23: rdtsc_t = 0;
    let mut dur_seconds_23: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_23 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_23 = dur_cycles_23
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_23 = dur_cycles_23 as libc::c_double;
    dur_seconds_23 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_23 as libc::c_int as __time_t;
    dur_seconds_23 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_23 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zlsh(c, a, 76)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh25 = i;
        i = i.wrapping_sub(1);
        if !(fresh25 != 0) {
            break;
        }
        zrsh(c.as_mut_ptr(), a.as_mut_ptr(), 76 as libc::c_int as size_t);
    }
    let mut dur_cycles_24: rdtsc_t = 0;
    let mut dur_seconds_24: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_24 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_24 = dur_cycles_24
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_24 = dur_cycles_24 as libc::c_double;
    dur_seconds_24 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_24 as libc::c_int as __time_t;
    dur_seconds_24 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_24 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zrsh(c, a, 76)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh26 = i;
        i = i.wrapping_sub(1);
        if !(fresh26 != 0) {
            break;
        }
        ztrunc(c.as_mut_ptr(), a.as_mut_ptr(), 76 as libc::c_int as size_t);
    }
    let mut dur_cycles_25: rdtsc_t = 0;
    let mut dur_seconds_25: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_25 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_25 = dur_cycles_25
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_25 = dur_cycles_25 as libc::c_double;
    dur_seconds_25 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_25 as libc::c_int as __time_t;
    dur_seconds_25 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_25 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"ztrunc(c, a, 76)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh27 = i;
        i = i.wrapping_sub(1);
        if !(fresh27 != 0) {
            break;
        }
        ztrunc(c.as_mut_ptr(), c.as_mut_ptr(), 76 as libc::c_int as size_t);
    }
    let mut dur_cycles_26: rdtsc_t = 0;
    let mut dur_seconds_26: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_26 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_26 = dur_cycles_26
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_26 = dur_cycles_26 as libc::c_double;
    dur_seconds_26 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_26 as libc::c_int as __time_t;
    dur_seconds_26 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_26 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"ztrunc(c, c, 76)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh28 = i;
        i = i.wrapping_sub(1);
        if !(fresh28 != 0) {
            break;
        }
        zsplit(
            c.as_mut_ptr(),
            d.as_mut_ptr(),
            a.as_mut_ptr(),
            76 as libc::c_int as size_t,
        );
    }
    let mut dur_cycles_27: rdtsc_t = 0;
    let mut dur_seconds_27: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_27 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_27 = dur_cycles_27
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_27 = dur_cycles_27 as libc::c_double;
    dur_seconds_27 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_27 as libc::c_int as __time_t;
    dur_seconds_27 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_27 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsplit(c, d, a, 76)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh29 = i;
        i = i.wrapping_sub(1);
        if !(fresh29 != 0) {
            break;
        }
        zcmpmag(a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_28: rdtsc_t = 0;
    let mut dur_seconds_28: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_28 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_28 = dur_cycles_28
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_28 = dur_cycles_28 as libc::c_double;
    dur_seconds_28 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_28 as libc::c_int as __time_t;
    dur_seconds_28 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_28 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zcmpmag(a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh30 = i;
        i = i.wrapping_sub(1);
        if !(fresh30 != 0) {
            break;
        }
        zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_29: rdtsc_t = 0;
    let mut dur_seconds_29: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_29 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_29 = dur_cycles_29
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_29 = dur_cycles_29 as libc::c_double;
    dur_seconds_29 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_29 as libc::c_int as __time_t;
    dur_seconds_29 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_29 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh31 = i;
        i = i.wrapping_sub(1);
        if !(fresh31 != 0) {
            break;
        }
        zcmpi(a.as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
    }
    let mut dur_cycles_30: rdtsc_t = 0;
    let mut dur_seconds_30: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_30 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_30 = dur_cycles_30
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_30 = dur_cycles_30 as libc::c_double;
    dur_seconds_30 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_30 as libc::c_int as __time_t;
    dur_seconds_30 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_30 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zcmpi(a, 1000000000LL)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh32 = i;
        i = i.wrapping_sub(1);
        if !(fresh32 != 0) {
            break;
        }
        zcmpi(a.as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
    }
    let mut dur_cycles_31: rdtsc_t = 0;
    let mut dur_seconds_31: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_31 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_31 = dur_cycles_31
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_31 = dur_cycles_31 as libc::c_double;
    dur_seconds_31 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_31 as libc::c_int as __time_t;
    dur_seconds_31 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_31 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zcmpi(a, -1000000000LL)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh33 = i;
        i = i.wrapping_sub(1);
        if !(fresh33 != 0) {
            break;
        }
        zcmpu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
    }
    let mut dur_cycles_32: rdtsc_t = 0;
    let mut dur_seconds_32: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_32 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_32 = dur_cycles_32
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_32 = dur_cycles_32 as libc::c_double;
    dur_seconds_32 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_32 as libc::c_int as __time_t;
    dur_seconds_32 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_32 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zcmpu(a, 1000000000ULL)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh34 = i;
        i = i.wrapping_sub(1);
        if !(fresh34 != 0) {
            break;
        }
        zbset(
            c.as_mut_ptr(),
            a.as_mut_ptr(),
            76 as libc::c_int as size_t,
            1 as libc::c_int,
        );
    }
    let mut dur_cycles_33: rdtsc_t = 0;
    let mut dur_seconds_33: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_33 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_33 = dur_cycles_33
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_33 = dur_cycles_33 as libc::c_double;
    dur_seconds_33 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_33 as libc::c_int as __time_t;
    dur_seconds_33 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_33 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbset(c, a, 76, 1)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh35 = i;
        i = i.wrapping_sub(1);
        if !(fresh35 != 0) {
            break;
        }
        zbset(
            a.as_mut_ptr(),
            a.as_mut_ptr(),
            76 as libc::c_int as size_t,
            1 as libc::c_int,
        );
    }
    let mut dur_cycles_34: rdtsc_t = 0;
    let mut dur_seconds_34: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_34 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_34 = dur_cycles_34
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_34 = dur_cycles_34 as libc::c_double;
    dur_seconds_34 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_34 as libc::c_int as __time_t;
    dur_seconds_34 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_34 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbset(a, a, 76, 1)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh36 = i;
        i = i.wrapping_sub(1);
        if !(fresh36 != 0) {
            break;
        }
        zbset(
            c.as_mut_ptr(),
            a.as_mut_ptr(),
            76 as libc::c_int as size_t,
            0 as libc::c_int,
        );
    }
    let mut dur_cycles_35: rdtsc_t = 0;
    let mut dur_seconds_35: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_35 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_35 = dur_cycles_35
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_35 = dur_cycles_35 as libc::c_double;
    dur_seconds_35 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_35 as libc::c_int as __time_t;
    dur_seconds_35 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_35 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbset(c, a, 76, 0)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh37 = i;
        i = i.wrapping_sub(1);
        if !(fresh37 != 0) {
            break;
        }
        zbset(
            c.as_mut_ptr(),
            c.as_mut_ptr(),
            76 as libc::c_int as size_t,
            0 as libc::c_int,
        );
    }
    let mut dur_cycles_36: rdtsc_t = 0;
    let mut dur_seconds_36: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_36 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_36 = dur_cycles_36
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_36 = dur_cycles_36 as libc::c_double;
    dur_seconds_36 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_36 as libc::c_int as __time_t;
    dur_seconds_36 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_36 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbset(c, c, 76, 0)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh38 = i;
        i = i.wrapping_sub(1);
        if !(fresh38 != 0) {
            break;
        }
        zbset(
            c.as_mut_ptr(),
            a.as_mut_ptr(),
            76 as libc::c_int as size_t,
            -(1 as libc::c_int),
        );
    }
    let mut dur_cycles_37: rdtsc_t = 0;
    let mut dur_seconds_37: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_37 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_37 = dur_cycles_37
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_37 = dur_cycles_37 as libc::c_double;
    dur_seconds_37 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_37 as libc::c_int as __time_t;
    dur_seconds_37 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_37 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbset(c, a, 76, -1)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh39 = i;
        i = i.wrapping_sub(1);
        if !(fresh39 != 0) {
            break;
        }
        zbset(
            a.as_mut_ptr(),
            a.as_mut_ptr(),
            76 as libc::c_int as size_t,
            -(1 as libc::c_int),
        );
    }
    let mut dur_cycles_38: rdtsc_t = 0;
    let mut dur_seconds_38: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_38 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_38 = dur_cycles_38
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_38 = dur_cycles_38 as libc::c_double;
    dur_seconds_38 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_38 as libc::c_int as __time_t;
    dur_seconds_38 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_38 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbset(a, a, 76, -1)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh40 = i;
        i = i.wrapping_sub(1);
        if !(fresh40 != 0) {
            break;
        }
        zbtest(a.as_mut_ptr(), 76 as libc::c_int as size_t);
    }
    let mut dur_cycles_39: rdtsc_t = 0;
    let mut dur_seconds_39: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_39 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_39 = dur_cycles_39
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_39 = dur_cycles_39 as libc::c_double;
    dur_seconds_39 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_39 as libc::c_int as __time_t;
    dur_seconds_39 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_39 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zbtest(a, 76)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh41 = i;
        i = i.wrapping_sub(1);
        if !(fresh41 != 0) {
            break;
        }
        zgcd(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_40: rdtsc_t = 0;
    let mut dur_seconds_40: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_40 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_40 = dur_cycles_40
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_40 = dur_cycles_40 as libc::c_double;
    dur_seconds_40 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_40 as libc::c_int as __time_t;
    dur_seconds_40 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_40 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zgcd(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh42 = i;
        i = i.wrapping_sub(1);
        if !(fresh42 != 0) {
            break;
        }
        zmul(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_41: rdtsc_t = 0;
    let mut dur_seconds_41: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_41 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_41 = dur_cycles_41
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_41 = dur_cycles_41 as libc::c_double;
    dur_seconds_41 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_41 as libc::c_int as __time_t;
    dur_seconds_41 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_41 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmul(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh43 = i;
        i = i.wrapping_sub(1);
        if !(fresh43 != 0) {
            break;
        }
        zmul(c.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_42: rdtsc_t = 0;
    let mut dur_seconds_42: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_42 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_42 = dur_cycles_42
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_42 = dur_cycles_42 as libc::c_double;
    dur_seconds_42 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_42 as libc::c_int as __time_t;
    dur_seconds_42 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_42 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmul(c, a, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh44 = i;
        i = i.wrapping_sub(1);
        if !(fresh44 != 0) {
            break;
        }
        zsqr(c.as_mut_ptr(), a.as_mut_ptr());
    }
    let mut dur_cycles_43: rdtsc_t = 0;
    let mut dur_seconds_43: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_43 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_43 = dur_cycles_43
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_43 = dur_cycles_43 as libc::c_double;
    dur_seconds_43 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_43 as libc::c_int as __time_t;
    dur_seconds_43 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_43 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsqr(c, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    zsets(
        d.as_mut_ptr(),
        b"1484298084218938358480511181388394862858002249\0" as *const u8
            as *const libc::c_char,
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh45 = i;
        i = i.wrapping_sub(1);
        if !(fresh45 != 0) {
            break;
        }
        zmodmul(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr(), d.as_mut_ptr());
    }
    let mut dur_cycles_44: rdtsc_t = 0;
    let mut dur_seconds_44: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_44 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_44 = dur_cycles_44
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_44 = dur_cycles_44 as libc::c_double;
    dur_seconds_44 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_44 as libc::c_int as __time_t;
    dur_seconds_44 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_44 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodmul(c, a, b, d)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh46 = i;
        i = i.wrapping_sub(1);
        if !(fresh46 != 0) {
            break;
        }
        zmodmul(c.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr(), d.as_mut_ptr());
    }
    let mut dur_cycles_45: rdtsc_t = 0;
    let mut dur_seconds_45: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_45 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_45 = dur_cycles_45
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_45 = dur_cycles_45 as libc::c_double;
    dur_seconds_45 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_45 as libc::c_int as __time_t;
    dur_seconds_45 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_45 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodmul(c, a, a, d)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh47 = i;
        i = i.wrapping_sub(1);
        if !(fresh47 != 0) {
            break;
        }
        zmodsqr(c.as_mut_ptr(), a.as_mut_ptr(), d.as_mut_ptr());
    }
    let mut dur_cycles_46: rdtsc_t = 0;
    let mut dur_seconds_46: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_46 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_46 = dur_cycles_46
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_46 = dur_cycles_46 as libc::c_double;
    dur_seconds_46 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_46 as libc::c_int as __time_t;
    dur_seconds_46 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_46 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodsqr(c, a, d)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh48 = i;
        i = i.wrapping_sub(1);
        if !(fresh48 != 0) {
            break;
        }
        zmodmul(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr(), tiny.as_mut_ptr());
    }
    let mut dur_cycles_47: rdtsc_t = 0;
    let mut dur_seconds_47: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_47 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_47 = dur_cycles_47
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_47 = dur_cycles_47 as libc::c_double;
    dur_seconds_47 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_47 as libc::c_int as __time_t;
    dur_seconds_47 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_47 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodmul(c, a, b, tiny)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh49 = i;
        i = i.wrapping_sub(1);
        if !(fresh49 != 0) {
            break;
        }
        zmodmul(c.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr(), tiny.as_mut_ptr());
    }
    let mut dur_cycles_48: rdtsc_t = 0;
    let mut dur_seconds_48: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_48 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_48 = dur_cycles_48
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_48 = dur_cycles_48 as libc::c_double;
    dur_seconds_48 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_48 as libc::c_int as __time_t;
    dur_seconds_48 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_48 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodmul(c, a, a, tiny)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh50 = i;
        i = i.wrapping_sub(1);
        if !(fresh50 != 0) {
            break;
        }
        zmodsqr(c.as_mut_ptr(), a.as_mut_ptr(), tiny.as_mut_ptr());
    }
    let mut dur_cycles_49: rdtsc_t = 0;
    let mut dur_seconds_49: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_49 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_49 = dur_cycles_49
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_49 = dur_cycles_49 as libc::c_double;
    dur_seconds_49 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_49 as libc::c_int as __time_t;
    dur_seconds_49 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_49 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodsqr(c, a, tiny)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    zsets(d.as_mut_ptr(), b"12\0" as *const u8 as *const libc::c_char);
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh51 = i;
        i = i.wrapping_sub(1);
        if !(fresh51 != 0) {
            break;
        }
        zpow(c.as_mut_ptr(), a.as_mut_ptr(), d.as_mut_ptr());
    }
    let mut dur_cycles_50: rdtsc_t = 0;
    let mut dur_seconds_50: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_50 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_50 = dur_cycles_50
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_50 = dur_cycles_50 as libc::c_double;
    dur_seconds_50 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_50 as libc::c_int as __time_t;
    dur_seconds_50 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_50 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zpow(c, a, d)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh52 = i;
        i = i.wrapping_sub(1);
        if !(fresh52 != 0) {
            break;
        }
        zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 12 as libc::c_int as libc::c_ulonglong);
    }
    let mut dur_cycles_51: rdtsc_t = 0;
    let mut dur_seconds_51: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_51 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_51 = dur_cycles_51
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_51 = dur_cycles_51 as libc::c_double;
    dur_seconds_51 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_51 as libc::c_int as __time_t;
    dur_seconds_51 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_51 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zpowu(c, a, 12)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh53 = i;
        i = i.wrapping_sub(1);
        if !(fresh53 != 0) {
            break;
        }
        zmodpow(c.as_mut_ptr(), a.as_mut_ptr(), d.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_52: rdtsc_t = 0;
    let mut dur_seconds_52: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_52 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_52 = dur_cycles_52
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_52 = dur_cycles_52 as libc::c_double;
    dur_seconds_52 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_52 as libc::c_int as __time_t;
    dur_seconds_52 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_52 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodpow(c, a, d, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh54 = i;
        i = i.wrapping_sub(1);
        if !(fresh54 != 0) {
            break;
        }
        zmodpowu(
            c.as_mut_ptr(),
            a.as_mut_ptr(),
            12 as libc::c_int as libc::c_ulonglong,
            b.as_mut_ptr(),
        );
    }
    let mut dur_cycles_53: rdtsc_t = 0;
    let mut dur_seconds_53: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_53 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_53 = dur_cycles_53
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_53 = dur_cycles_53 as libc::c_double;
    dur_seconds_53 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_53 as libc::c_int as __time_t;
    dur_seconds_53 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_53 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmodpowu(c, a, 12, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh55 = i;
        i = i.wrapping_sub(1);
        if !(fresh55 != 0) {
            break;
        }
        zsets(
            c.as_mut_ptr(),
            b"5495468234592964023447280368442884381000481887\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut dur_cycles_54: rdtsc_t = 0;
    let mut dur_seconds_54: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_54 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_54 = dur_cycles_54
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_54 = dur_cycles_54 as libc::c_double;
    dur_seconds_54 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_54 as libc::c_int as __time_t;
    dur_seconds_54 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_54 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsets(c, \"5495468234592964023447280368442884381000481887\")\0" as *const u8
            as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh56 = i;
        i = i.wrapping_sub(1);
        if !(fresh56 != 0) {
            break;
        }
        zstr_length(a.as_mut_ptr(), 10 as libc::c_int as libc::c_ulonglong);
    }
    let mut dur_cycles_55: rdtsc_t = 0;
    let mut dur_seconds_55: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_55 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_55 = dur_cycles_55
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_55 = dur_cycles_55 as libc::c_double;
    dur_seconds_55 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_55 as libc::c_int as __time_t;
    dur_seconds_55 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_55 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zstr_length(a, 10)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh57 = i;
        i = i.wrapping_sub(1);
        if !(fresh57 != 0) {
            break;
        }
        zstr(a.as_mut_ptr(), buf.as_mut_ptr(), 0 as libc::c_int as size_t);
    }
    let mut dur_cycles_56: rdtsc_t = 0;
    let mut dur_seconds_56: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_56 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_56 = dur_cycles_56
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_56 = dur_cycles_56 as libc::c_double;
    dur_seconds_56 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_56 as libc::c_int as __time_t;
    dur_seconds_56 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_56 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zstr(a, buf, 0)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh58 = i;
        i = i.wrapping_sub(1);
        if !(fresh58 != 0) {
            break;
        }
        zstr(
            a.as_mut_ptr(),
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    let mut dur_cycles_57: rdtsc_t = 0;
    let mut dur_seconds_57: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_57 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_57 = dur_cycles_57
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_57 = dur_cycles_57 as libc::c_double;
    dur_seconds_57 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_57 as libc::c_int as __time_t;
    dur_seconds_57 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_57 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zstr(a, buf, sizeof(buf) - 1)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh59 = i;
        i = i.wrapping_sub(1);
        if !(fresh59 != 0) {
            break;
        }
        zrand(c.as_mut_ptr(), DEFAULT_RANDOM, QUASIUNIFORM, a.as_mut_ptr());
    }
    let mut dur_cycles_58: rdtsc_t = 0;
    let mut dur_seconds_58: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_58 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_58 = dur_cycles_58
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_58 = dur_cycles_58 as libc::c_double;
    dur_seconds_58 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_58 as libc::c_int as __time_t;
    dur_seconds_58 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_58 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zrand(c, DEFAULT_RANDOM, QUASIUNIFORM, a)\0" as *const u8
            as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh60 = i;
        i = i.wrapping_sub(1);
        if !(fresh60 != 0) {
            break;
        }
        zrand(c.as_mut_ptr(), DEFAULT_RANDOM, UNIFORM, a.as_mut_ptr());
    }
    let mut dur_cycles_59: rdtsc_t = 0;
    let mut dur_seconds_59: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_59 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_59 = dur_cycles_59
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_59 = dur_cycles_59 as libc::c_double;
    dur_seconds_59 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_59 as libc::c_int as __time_t;
    dur_seconds_59 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_59 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zrand(c, DEFAULT_RANDOM, UNIFORM, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh61 = i;
        i = i.wrapping_sub(1);
        if !(fresh61 != 0) {
            break;
        }
        zrand(c.as_mut_ptr(), DEFAULT_RANDOM, MODUNIFORM, a.as_mut_ptr());
    }
    let mut dur_cycles_60: rdtsc_t = 0;
    let mut dur_seconds_60: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_60 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_60 = dur_cycles_60
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_60 = dur_cycles_60 as libc::c_double;
    dur_seconds_60 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_60 as libc::c_int as __time_t;
    dur_seconds_60 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_60 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zrand(c, DEFAULT_RANDOM, MODUNIFORM, a)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh62 = i;
        i = i.wrapping_sub(1);
        if !(fresh62 != 0) {
            break;
        }
        zptest(d.as_mut_ptr(), a.as_mut_ptr(), 5 as libc::c_int);
    }
    let mut dur_cycles_61: rdtsc_t = 0;
    let mut dur_seconds_61: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_61 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_61 = dur_cycles_61
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_61 = dur_cycles_61 as libc::c_double;
    dur_seconds_61 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_61 as libc::c_int as __time_t;
    dur_seconds_61 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_61 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zptest(d, a, 5)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh63 = i;
        i = i.wrapping_sub(1);
        if !(fresh63 != 0) {
            break;
        }
        zsave(a.as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
    }
    let mut dur_cycles_62: rdtsc_t = 0;
    let mut dur_seconds_62: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_62 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_62 = dur_cycles_62
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_62 = dur_cycles_62 as libc::c_double;
    dur_seconds_62 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_62 as libc::c_int as __time_t;
    dur_seconds_62 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_62 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zsave(a, buf)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh64 = i;
        i = i.wrapping_sub(1);
        if !(fresh64 != 0) {
            break;
        }
        zload(a.as_mut_ptr(), buf.as_mut_ptr() as *const libc::c_void);
    }
    let mut dur_cycles_63: rdtsc_t = 0;
    let mut dur_seconds_63: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_63 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_63 = dur_cycles_63
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_63 = dur_cycles_63 as libc::c_double;
    dur_seconds_63 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_63 as libc::c_int as __time_t;
    dur_seconds_63 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_63 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zload(a, buf)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh65 = i;
        i = i.wrapping_sub(1);
        if !(fresh65 != 0) {
            break;
        }
        zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_64: rdtsc_t = 0;
    let mut dur_seconds_64: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_64 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_64 = dur_cycles_64
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_64 = dur_cycles_64 as libc::c_double;
    dur_seconds_64 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_64 as libc::c_int as __time_t;
    dur_seconds_64 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_64 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zdiv(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh66 = i;
        i = i.wrapping_sub(1);
        if !(fresh66 != 0) {
            break;
        }
        zmod(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_65: rdtsc_t = 0;
    let mut dur_seconds_65: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_65 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_65 = dur_cycles_65
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_65 = dur_cycles_65 as libc::c_double;
    dur_seconds_65 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_65 as libc::c_int as __time_t;
    dur_seconds_65 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_65 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmod(c, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 1 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh67 = i;
        i = i.wrapping_sub(1);
        if !(fresh67 != 0) {
            break;
        }
        zdivmod(c.as_mut_ptr(), d.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    }
    let mut dur_cycles_66: rdtsc_t = 0;
    let mut dur_seconds_66: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_66 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_66 = dur_cycles_66
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_66 = dur_cycles_66 as libc::c_double;
    dur_seconds_66 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_66 as libc::c_int as __time_t;
    dur_seconds_66 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_66 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zdivmod(c, d, a, b)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 1 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh68 = i;
        i = i.wrapping_sub(1);
        if !(fresh68 != 0) {
            break;
        }
        zdiv(c.as_mut_ptr(), a.as_mut_ptr(), tiny.as_mut_ptr());
    }
    let mut dur_cycles_67: rdtsc_t = 0;
    let mut dur_seconds_67: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_67 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_67 = dur_cycles_67
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_67 = dur_cycles_67 as libc::c_double;
    dur_seconds_67 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_67 as libc::c_int as __time_t;
    dur_seconds_67 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_67 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zdiv(c, a, tiny)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh69 = i;
        i = i.wrapping_sub(1);
        if !(fresh69 != 0) {
            break;
        }
        zmod(c.as_mut_ptr(), a.as_mut_ptr(), tiny.as_mut_ptr());
    }
    let mut dur_cycles_68: rdtsc_t = 0;
    let mut dur_seconds_68: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_68 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_68 = dur_cycles_68
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_68 = dur_cycles_68 as libc::c_double;
    dur_seconds_68 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_68 as libc::c_int as __time_t;
    dur_seconds_68 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_68 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zmod(c, a, tiny)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    i = (if 0 as libc::c_int != 0 {
        1000000 as libc::c_long
    } else {
        1000 as libc::c_long
    }) as size_t;
    rdtsc(&mut start_low, &mut start_high);
    loop {
        let fresh70 = i;
        i = i.wrapping_sub(1);
        if !(fresh70 != 0) {
            break;
        }
        zdivmod(c.as_mut_ptr(), d.as_mut_ptr(), a.as_mut_ptr(), tiny.as_mut_ptr());
    }
    let mut dur_cycles_69: rdtsc_t = 0;
    let mut dur_seconds_69: libc::c_double = 0.;
    rdtsc(&mut end_low, &mut end_high);
    dur_cycles_69 = end_low as rdtsc_t | (end_high as rdtsc_t) << 32 as libc::c_int;
    dur_cycles_69 = dur_cycles_69
        .wrapping_sub(
            start_low as rdtsc_t | (start_high as rdtsc_t) << 32 as libc::c_int,
        );
    dur_seconds_69 = dur_cycles_69 as libc::c_double;
    dur_seconds_69 /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
    dur.tv_sec = dur_seconds_69 as libc::c_int as __time_t;
    dur_seconds_69 -= dur.tv_sec as libc::c_double;
    dur
        .tv_nsec = (dur_seconds_69 * 1000000000 as libc::c_long as libc::c_double)
        as libc::c_long;
    sprintf(
        timebuf.as_mut_ptr(),
        b"%lli.%09li\0" as *const u8 as *const libc::c_char,
        dur.tv_sec as libc::c_longlong,
        dur.tv_nsec,
    );
    printf(
        b"%s: %s %s (152 bits)\n\0" as *const u8 as *const libc::c_char,
        b"zdivmod(c, d, a, tiny)\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    zfree(a.as_mut_ptr());
    zfree(b.as_mut_ptr());
    zfree(c.as_mut_ptr());
    zfree(d.as_mut_ptr());
    zfree(tiny.as_mut_ptr());
    zunsetup();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
