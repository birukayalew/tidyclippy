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
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zsetup(_: *mut __jmp_buf_tag);
    fn zunsetup();
    fn zfree(_: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrand(_: *mut zahl, _: zranddev, _: zranddist, _: *mut zahl);
    fn zperror(_: *const libc::c_char);
    static mut timebuf: [libc::c_char; 512];
    static mut freq: libc::c_ulonglong;
    fn benchmark_init();
}
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
    let mut r: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut n: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
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
    zinit(r.as_mut_ptr());
    zinit(n.as_mut_ptr());
    zsetu(n.as_mut_ptr(), 1 as libc::c_int as uint64_t);
    zlsh(
        n.as_mut_ptr(),
        n.as_mut_ptr(),
        (64000 as libc::c_long - 1 as libc::c_long) as size_t,
    );
    zset(r.as_mut_ptr(), n.as_mut_ptr());
    i = (if 0 as libc::c_int != 0 {
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
        zrand(r.as_mut_ptr(), FAST_RANDOM, MODUNIFORM, n.as_mut_ptr());
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
        b"%s: %s %s\n\0" as *const u8 as *const libc::c_char,
        b"zrand(r, FAST_RANDOM, MODUNIFORM, n)\0" as *const u8 as *const libc::c_char,
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
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        zrand(r.as_mut_ptr(), LIBC_RAND_RANDOM, MODUNIFORM, n.as_mut_ptr());
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
        b"%s: %s %s\n\0" as *const u8 as *const libc::c_char,
        b"zrand(r, LIBC_RAND_RANDOM, MODUNIFORM, n)\0" as *const u8
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
        let fresh2 = i;
        i = i.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        zrand(r.as_mut_ptr(), LIBC_RANDOM_RANDOM, MODUNIFORM, n.as_mut_ptr());
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
        b"%s: %s %s\n\0" as *const u8 as *const libc::c_char,
        b"zrand(r, LIBC_RANDOM_RANDOM, MODUNIFORM, n)\0" as *const u8
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
        let fresh3 = i;
        i = i.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        zrand(r.as_mut_ptr(), LIBC_RAND48_RANDOM, MODUNIFORM, n.as_mut_ptr());
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
        b"%s: %s %s\n\0" as *const u8 as *const libc::c_char,
        b"zrand(r, LIBC_RAND48_RANDOM, MODUNIFORM, n)\0" as *const u8
            as *const libc::c_char,
        timebuf.as_mut_ptr(),
        if 0 as libc::c_int != 0 {
            b"\xC2\xB5s\0" as *const u8 as *const libc::c_char
        } else {
            b"ms\0" as *const u8 as *const libc::c_char
        },
    );
    zfree(r.as_mut_ptr());
    zfree(n.as_mut_ptr());
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
