#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, extern_types)]
use core::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zsetup(_: *mut __jmp_buf_tag);
    fn zunsetup();
    fn zfree(_: *mut zahl);
    fn zload(_: *mut zahl, _: *const libc::c_void) -> size_t;
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zand(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zor(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zxor(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn znot(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn ztrunc(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zptest(_: *mut zahl, _: *mut zahl, _: libc::c_int) -> zprimality;
    fn zstr(_: *mut zahl, _: *mut libc::c_char, _: size_t) -> *mut libc::c_char;
    fn zsets(_: *mut zahl, _: *const libc::c_char) -> libc::c_int;
    fn zstr_length(_: *mut zahl, _: libc::c_ulonglong) -> size_t;
    fn zperror(_: *const libc::c_char);
    fn zbset_ll_set(_: *mut zahl, _: size_t);
    fn zbset_ll_clear(_: *mut zahl, _: size_t);
    fn zbset_ll_flip(_: *mut zahl, _: size_t);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsqr_ll(_: *mut zahl, _: *mut zahl);
    static mut freq: libc::c_ulonglong;
    fn benchmark_init();
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type rdtsc_t = libc::c_ulonglong;
pub type C2RustUnnamed = libc::c_uint;
pub const FULL: C2RustUnnamed = 3;
pub const HALF: C2RustUnnamed = 2;
pub const HIGH_AND_LOW: C2RustUnnamed = 1;
pub const HIGH_ONLY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct function {
    pub name: *const libc::c_char,
    pub f: Option::<unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> ()>,
    pub a_start: size_t,
    pub a_end: size_t,
    pub a_step: size_t,
    pub b_start: size_t,
    pub b_end: size_t,
    pub b_step: size_t,
    pub a_mode: libc::c_int,
    pub b_mode: libc::c_int,
    pub runs: size_t,
    pub measurements: size_t,
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
        let mut buf_0: *mut libc::c_char = buffer as *mut libc::c_char;
        *(buf_0 as *mut libc::c_long) = (*a).sign as libc::c_long;
        buf_0 = buf_0
            .offset(::core::mem::size_of::<libc::c_long>() as libc::c_ulong as isize);
        *(buf_0 as *mut size_t) = (*a).used;
        buf_0 = buf_0.offset(::core::mem::size_of::<size_t>() as libc::c_ulong as isize);
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
            libzahl_memcpy(buf_0 as *mut zahl_char_t, (*a).chars, (*a).used);
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
static mut buf: [libc::c_char; 2000] = [0; 2000];
static mut temp: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
static mut temp2: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
static mut measurements: [libc::c_ulonglong; 200] = [0; 200];
unsafe extern "C" fn measurementpcmp(
    mut ap_: *const libc::c_void,
    mut bp_: *const libc::c_void,
) -> libc::c_int {
    let mut ap: *const libc::c_ulonglong = ap_ as *const libc::c_ulonglong;
    let mut bp: *const libc::c_ulonglong = bp_ as *const libc::c_ulonglong;
    return if *ap < *bp { -(1 as libc::c_int) } else { (*ap > *bp) as libc::c_int };
}
unsafe extern "C" fn gettime(mut m: size_t) -> libc::c_ulonglong {
    let mut i: size_t = m * 2 as libc::c_int as size_t / 7 as libc::c_int as size_t;
    let mut n: size_t = m
        .wrapping_sub(m * 2 as libc::c_int as size_t / 7 as libc::c_int as size_t);
    let mut tot: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    qsort(
        measurements.as_mut_ptr() as *mut libc::c_void,
        m,
        ::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong,
        Some(
            measurementpcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    while i < n {
        tot = tot.wrapping_add(measurements[i as usize]);
        i = i.wrapping_add(1);
        i;
    }
    return tot
        .wrapping_div(
            n.wrapping_sub(m * 2 as libc::c_int as size_t / 7 as libc::c_int as size_t)
                as libc::c_ulonglong,
        );
}
unsafe extern "C" fn bench_zsetu(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = (*f).measurements;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        zsetu(temp.as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
        zsetu(temp.as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
        j = (*f).runs;
        rdtsc(&mut start_low, &mut start_high);
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            zsetu(temp.as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
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
        measurements[i
            as usize] = (dur.tv_sec as libc::c_ulonglong)
            .wrapping_mul(1000000000 as libc::c_ulonglong)
            .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
    }
    printf(b"%llu\n\0" as *const u8 as *const libc::c_char, gettime((*f).measurements));
}
unsafe extern "C" fn bench_pos_zseti(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = (*f).measurements;
    loop {
        let fresh2 = i;
        i = i.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        zseti(temp.as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
        zseti(temp.as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
        j = (*f).runs;
        rdtsc(&mut start_low, &mut start_high);
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            zseti(temp.as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
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
        measurements[i
            as usize] = (dur.tv_sec as libc::c_ulonglong)
            .wrapping_mul(1000000000 as libc::c_ulonglong)
            .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
    }
    printf(b"%llu\n\0" as *const u8 as *const libc::c_char, gettime((*f).measurements));
}
unsafe extern "C" fn bench_zseti(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = (*f).measurements;
    loop {
        let fresh4 = i;
        i = i.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        zseti(temp.as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
        zseti(temp.as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
        j = (*f).runs;
        rdtsc(&mut start_low, &mut start_high);
        loop {
            let fresh5 = j;
            j = j.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            zseti(temp.as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
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
        measurements[i
            as usize] = (dur.tv_sec as libc::c_ulonglong)
            .wrapping_mul(1000000000 as libc::c_ulonglong)
            .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
    }
    printf(b"%llu\n\0" as *const u8 as *const libc::c_char, gettime((*f).measurements));
}
unsafe extern "C" fn bench_zsets(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        zstr(
            (*a).as_mut_ptr(),
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        k = (*f).measurements;
        loop {
            let fresh6 = k;
            k = k.wrapping_sub(1);
            if !(fresh6 != 0) {
                break;
            }
            zsets(temp.as_mut_ptr(), buf.as_mut_ptr());
            zsets(temp.as_mut_ptr(), buf.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh7 = j;
                j = j.wrapping_sub(1);
                if !(fresh7 != 0) {
                    break;
                }
                zsets(temp.as_mut_ptr(), buf.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zsub_unsigned(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh8 = k;
            k = k.wrapping_sub(1);
            if !(fresh8 != 0) {
                break;
            }
            zsub_unsigned(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zsub_unsigned(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh9 = j;
                j = j.wrapping_sub(1);
                if !(fresh9 != 0) {
                    break;
                }
                zsub_unsigned(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zadd(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh10 = k;
            k = k.wrapping_sub(1);
            if !(fresh10 != 0) {
                break;
            }
            zadd(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zadd(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh11 = j;
                j = j.wrapping_sub(1);
                if !(fresh11 != 0) {
                    break;
                }
                zadd(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zsub(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh12 = k;
            k = k.wrapping_sub(1);
            if !(fresh12 != 0) {
                break;
            }
            zsub(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zsub(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh13 = j;
                j = j.wrapping_sub(1);
                if !(fresh13 != 0) {
                    break;
                }
                zsub(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zand(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh14 = k;
            k = k.wrapping_sub(1);
            if !(fresh14 != 0) {
                break;
            }
            zand(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zand(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh15 = j;
                j = j.wrapping_sub(1);
                if !(fresh15 != 0) {
                    break;
                }
                zand(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zor(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh16 = k;
            k = k.wrapping_sub(1);
            if !(fresh16 != 0) {
                break;
            }
            zor(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zor(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh17 = j;
                j = j.wrapping_sub(1);
                if !(fresh17 != 0) {
                    break;
                }
                zor(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zxor(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh18 = k;
            k = k.wrapping_sub(1);
            if !(fresh18 != 0) {
                break;
            }
            zxor(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zxor(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh19 = j;
                j = j.wrapping_sub(1);
                if !(fresh19 != 0) {
                    break;
                }
                zxor(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_znot(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh20 = k;
            k = k.wrapping_sub(1);
            if !(fresh20 != 0) {
                break;
            }
            znot(temp.as_mut_ptr(), (*a).as_mut_ptr());
            znot(temp.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh21 = j;
                j = j.wrapping_sub(1);
                if !(fresh21 != 0) {
                    break;
                }
                znot(temp.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zeven(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh22 = k;
            k = k.wrapping_sub(1);
            if !(fresh22 != 0) {
                break;
            }
            zeven((*a).as_mut_ptr());
            zeven((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh23 = j;
                j = j.wrapping_sub(1);
                if !(fresh23 != 0) {
                    break;
                }
                zeven((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zodd(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh24 = k;
            k = k.wrapping_sub(1);
            if !(fresh24 != 0) {
                break;
            }
            zodd((*a).as_mut_ptr());
            zodd((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh25 = j;
                j = j.wrapping_sub(1);
                if !(fresh25 != 0) {
                    break;
                }
                zodd((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zeven_nonzero(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh26 = k;
            k = k.wrapping_sub(1);
            if !(fresh26 != 0) {
                break;
            }
            zeven_nonzero((*a).as_mut_ptr());
            zeven_nonzero((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh27 = j;
                j = j.wrapping_sub(1);
                if !(fresh27 != 0) {
                    break;
                }
                zeven_nonzero((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zodd_nonzero(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh28 = k;
            k = k.wrapping_sub(1);
            if !(fresh28 != 0) {
                break;
            }
            zodd_nonzero((*a).as_mut_ptr());
            zodd_nonzero((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh29 = j;
                j = j.wrapping_sub(1);
                if !(fresh29 != 0) {
                    break;
                }
                zodd_nonzero((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zzero(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh30 = k;
            k = k.wrapping_sub(1);
            if !(fresh30 != 0) {
                break;
            }
            zzero((*a).as_mut_ptr());
            zzero((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh31 = j;
                j = j.wrapping_sub(1);
                if !(fresh31 != 0) {
                    break;
                }
                zzero((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zsignum(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh32 = k;
            k = k.wrapping_sub(1);
            if !(fresh32 != 0) {
                break;
            }
            zsignum((*a).as_mut_ptr());
            zsignum((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh33 = j;
                j = j.wrapping_sub(1);
                if !(fresh33 != 0) {
                    break;
                }
                zsignum((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zbits(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh34 = k;
            k = k.wrapping_sub(1);
            if !(fresh34 != 0) {
                break;
            }
            zbits((*a).as_mut_ptr());
            zbits((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh35 = j;
                j = j.wrapping_sub(1);
                if !(fresh35 != 0) {
                    break;
                }
                zbits((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zlsb(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh36 = k;
            k = k.wrapping_sub(1);
            if !(fresh36 != 0) {
                break;
            }
            zlsb((*a).as_mut_ptr());
            zlsb((*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh37 = j;
                j = j.wrapping_sub(1);
                if !(fresh37 != 0) {
                    break;
                }
                zlsb((*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zswap(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh38 = k;
            k = k.wrapping_sub(1);
            if !(fresh38 != 0) {
                break;
            }
            zswap(temp.as_mut_ptr(), (*a).as_mut_ptr());
            zswap(temp.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh39 = j;
                j = j.wrapping_sub(1);
                if !(fresh39 != 0) {
                    break;
                }
                zswap(temp.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zcmpmag(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh40 = k;
            k = k.wrapping_sub(1);
            if !(fresh40 != 0) {
                break;
            }
            zcmpmag(temp2.as_mut_ptr(), (*a).as_mut_ptr());
            zcmpmag(temp2.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh41 = j;
                j = j.wrapping_sub(1);
                if !(fresh41 != 0) {
                    break;
                }
                zcmpmag(temp2.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zcmp(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh42 = k;
            k = k.wrapping_sub(1);
            if !(fresh42 != 0) {
                break;
            }
            zcmp(temp2.as_mut_ptr(), (*a).as_mut_ptr());
            zcmp(temp2.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh43 = j;
                j = j.wrapping_sub(1);
                if !(fresh43 != 0) {
                    break;
                }
                zcmp(temp2.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_pos_zcmpi(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh44 = k;
            k = k.wrapping_sub(1);
            if !(fresh44 != 0) {
                break;
            }
            zcmpi((*a).as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
            zcmpi((*a).as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh45 = j;
                j = j.wrapping_sub(1);
                if !(fresh45 != 0) {
                    break;
                }
                zcmpi((*a).as_mut_ptr(), 1000000000 as libc::c_longlong as int64_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zcmpi(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh46 = k;
            k = k.wrapping_sub(1);
            if !(fresh46 != 0) {
                break;
            }
            zcmpi((*a).as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
            zcmpi((*a).as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh47 = j;
                j = j.wrapping_sub(1);
                if !(fresh47 != 0) {
                    break;
                }
                zcmpi((*a).as_mut_ptr(), -(1000000000 as libc::c_longlong) as int64_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zcmpu(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh48 = k;
            k = k.wrapping_sub(1);
            if !(fresh48 != 0) {
                break;
            }
            zcmpu((*a).as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
            zcmpu((*a).as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh49 = j;
                j = j.wrapping_sub(1);
                if !(fresh49 != 0) {
                    break;
                }
                zcmpu((*a).as_mut_ptr(), 1000000000 as libc::c_ulonglong as uint64_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_sqr_zmul(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh50 = k;
            k = k.wrapping_sub(1);
            if !(fresh50 != 0) {
                break;
            }
            zmul(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zmul(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh51 = j;
                j = j.wrapping_sub(1);
                if !(fresh51 != 0) {
                    break;
                }
                zmul(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zsqr(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh52 = k;
            k = k.wrapping_sub(1);
            if !(fresh52 != 0) {
                break;
            }
            zsqr(temp.as_mut_ptr(), (*a).as_mut_ptr());
            zsqr(temp.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh53 = j;
                j = j.wrapping_sub(1);
                if !(fresh53 != 0) {
                    break;
                }
                zsqr(temp.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zadd_unsigned(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh54 = k;
            k = k.wrapping_sub(1);
            if !(fresh54 != 0) {
                break;
            }
            zadd_unsigned(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            zadd_unsigned(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh55 = j;
                j = j.wrapping_sub(1);
                if !(fresh55 != 0) {
                    break;
                }
                zadd_unsigned(temp.as_mut_ptr(), (*a).as_mut_ptr(), temp2.as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zstr(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh56 = k;
            k = k.wrapping_sub(1);
            if !(fresh56 != 0) {
                break;
            }
            zstr(
                (*a).as_mut_ptr(),
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            zstr(
                (*a).as_mut_ptr(),
                buf.as_mut_ptr(),
                (::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh57 = j;
                j = j.wrapping_sub(1);
                if !(fresh57 != 0) {
                    break;
                }
                zstr(
                    (*a).as_mut_ptr(),
                    buf.as_mut_ptr(),
                    (::core::mem::size_of::<[libc::c_char; 2000]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_auto_zstr(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh58 = k;
            k = k.wrapping_sub(1);
            if !(fresh58 != 0) {
                break;
            }
            zstr((*a).as_mut_ptr(), buf.as_mut_ptr(), 0 as libc::c_int as size_t);
            zstr((*a).as_mut_ptr(), buf.as_mut_ptr(), 0 as libc::c_int as size_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh59 = j;
                j = j.wrapping_sub(1);
                if !(fresh59 != 0) {
                    break;
                }
                zstr((*a).as_mut_ptr(), buf.as_mut_ptr(), 0 as libc::c_int as size_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zsave(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh60 = k;
            k = k.wrapping_sub(1);
            if !(fresh60 != 0) {
                break;
            }
            zsave((*a).as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
            zsave((*a).as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh61 = j;
                j = j.wrapping_sub(1);
                if !(fresh61 != 0) {
                    break;
                }
                zsave((*a).as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zload(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        zsave((*a).as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
        k = (*f).measurements;
        loop {
            let fresh62 = k;
            k = k.wrapping_sub(1);
            if !(fresh62 != 0) {
                break;
            }
            zload(temp.as_mut_ptr(), buf.as_mut_ptr() as *const libc::c_void);
            zload(temp.as_mut_ptr(), buf.as_mut_ptr() as *const libc::c_void);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh63 = j;
                j = j.wrapping_sub(1);
                if !(fresh63 != 0) {
                    break;
                }
                zload(temp.as_mut_ptr(), buf.as_mut_ptr() as *const libc::c_void);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zbset_set(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh64 = k;
            k = k.wrapping_sub(1);
            if !(fresh64 != 0) {
                break;
            }
            zbset(
                temp.as_mut_ptr(),
                (*a).as_mut_ptr(),
                2 as libc::c_int as size_t,
                1 as libc::c_int,
            );
            zbset(
                temp.as_mut_ptr(),
                (*a).as_mut_ptr(),
                2 as libc::c_int as size_t,
                1 as libc::c_int,
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh65 = j;
                j = j.wrapping_sub(1);
                if !(fresh65 != 0) {
                    break;
                }
                zbset(
                    temp.as_mut_ptr(),
                    (*a).as_mut_ptr(),
                    2 as libc::c_int as size_t,
                    1 as libc::c_int,
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zbset_clear(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh66 = k;
            k = k.wrapping_sub(1);
            if !(fresh66 != 0) {
                break;
            }
            zbset(
                temp.as_mut_ptr(),
                (*a).as_mut_ptr(),
                2 as libc::c_int as size_t,
                0 as libc::c_int,
            );
            zbset(
                temp.as_mut_ptr(),
                (*a).as_mut_ptr(),
                2 as libc::c_int as size_t,
                0 as libc::c_int,
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh67 = j;
                j = j.wrapping_sub(1);
                if !(fresh67 != 0) {
                    break;
                }
                zbset(
                    temp.as_mut_ptr(),
                    (*a).as_mut_ptr(),
                    2 as libc::c_int as size_t,
                    0 as libc::c_int,
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zbset_flip(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh68 = k;
            k = k.wrapping_sub(1);
            if !(fresh68 != 0) {
                break;
            }
            zbset(
                temp.as_mut_ptr(),
                (*a).as_mut_ptr(),
                2 as libc::c_int as size_t,
                -(1 as libc::c_int),
            );
            zbset(
                temp.as_mut_ptr(),
                (*a).as_mut_ptr(),
                2 as libc::c_int as size_t,
                -(1 as libc::c_int),
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh69 = j;
                j = j.wrapping_sub(1);
                if !(fresh69 != 0) {
                    break;
                }
                zbset(
                    temp.as_mut_ptr(),
                    (*a).as_mut_ptr(),
                    2 as libc::c_int as size_t,
                    -(1 as libc::c_int),
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_self_zbset_set(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh70 = k;
            k = k.wrapping_sub(1);
            if !(fresh70 != 0) {
                break;
            }
            zbset(
                temp2.as_mut_ptr(),
                temp2.as_mut_ptr(),
                2 as libc::c_int as size_t,
                1 as libc::c_int,
            );
            zbset(
                temp2.as_mut_ptr(),
                temp2.as_mut_ptr(),
                2 as libc::c_int as size_t,
                1 as libc::c_int,
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh71 = j;
                j = j.wrapping_sub(1);
                if !(fresh71 != 0) {
                    break;
                }
                zbset(
                    temp2.as_mut_ptr(),
                    temp2.as_mut_ptr(),
                    2 as libc::c_int as size_t,
                    1 as libc::c_int,
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_self_zbset_clear(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh72 = k;
            k = k.wrapping_sub(1);
            if !(fresh72 != 0) {
                break;
            }
            zbset(
                temp2.as_mut_ptr(),
                temp2.as_mut_ptr(),
                2 as libc::c_int as size_t,
                0 as libc::c_int,
            );
            zbset(
                temp2.as_mut_ptr(),
                temp2.as_mut_ptr(),
                2 as libc::c_int as size_t,
                0 as libc::c_int,
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh73 = j;
                j = j.wrapping_sub(1);
                if !(fresh73 != 0) {
                    break;
                }
                zbset(
                    temp2.as_mut_ptr(),
                    temp2.as_mut_ptr(),
                    2 as libc::c_int as size_t,
                    0 as libc::c_int,
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_self_zbset_flip(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh74 = k;
            k = k.wrapping_sub(1);
            if !(fresh74 != 0) {
                break;
            }
            zbset(
                temp2.as_mut_ptr(),
                temp2.as_mut_ptr(),
                2 as libc::c_int as size_t,
                -(1 as libc::c_int),
            );
            zbset(
                temp2.as_mut_ptr(),
                temp2.as_mut_ptr(),
                2 as libc::c_int as size_t,
                -(1 as libc::c_int),
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh75 = j;
                j = j.wrapping_sub(1);
                if !(fresh75 != 0) {
                    break;
                }
                zbset(
                    temp2.as_mut_ptr(),
                    temp2.as_mut_ptr(),
                    2 as libc::c_int as size_t,
                    -(1 as libc::c_int),
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zbtest(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh76 = k;
            k = k.wrapping_sub(1);
            if !(fresh76 != 0) {
                break;
            }
            zbtest((*a).as_mut_ptr(), 2 as libc::c_int as size_t);
            zbtest((*a).as_mut_ptr(), 2 as libc::c_int as size_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh77 = j;
                j = j.wrapping_sub(1);
                if !(fresh77 != 0) {
                    break;
                }
                zbtest((*a).as_mut_ptr(), 2 as libc::c_int as size_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zptest(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh78 = k;
            k = k.wrapping_sub(1);
            if !(fresh78 != 0) {
                break;
            }
            zptest(temp.as_mut_ptr(), (*a).as_mut_ptr(), 5 as libc::c_int);
            zptest(temp.as_mut_ptr(), (*a).as_mut_ptr(), 5 as libc::c_int);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh79 = j;
                j = j.wrapping_sub(1);
                if !(fresh79 != 0) {
                    break;
                }
                zptest(temp.as_mut_ptr(), (*a).as_mut_ptr(), 5 as libc::c_int);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zset(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh80 = k;
            k = k.wrapping_sub(1);
            if !(fresh80 != 0) {
                break;
            }
            zset(temp.as_mut_ptr(), (*a).as_mut_ptr());
            zset(temp.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh81 = j;
                j = j.wrapping_sub(1);
                if !(fresh81 != 0) {
                    break;
                }
                zset(temp.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zlsh(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh82 = k;
            k = k.wrapping_sub(1);
            if !(fresh82 != 0) {
                break;
            }
            zlsh(temp.as_mut_ptr(), (*a).as_mut_ptr(), 1 as libc::c_int as size_t);
            zlsh(temp.as_mut_ptr(), (*a).as_mut_ptr(), 1 as libc::c_int as size_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh83 = j;
                j = j.wrapping_sub(1);
                if !(fresh83 != 0) {
                    break;
                }
                zlsh(temp.as_mut_ptr(), (*a).as_mut_ptr(), 1 as libc::c_int as size_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zrsh(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh84 = k;
            k = k.wrapping_sub(1);
            if !(fresh84 != 0) {
                break;
            }
            zrsh(temp.as_mut_ptr(), (*a).as_mut_ptr(), 1 as libc::c_int as size_t);
            zrsh(temp.as_mut_ptr(), (*a).as_mut_ptr(), 1 as libc::c_int as size_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh85 = j;
                j = j.wrapping_sub(1);
                if !(fresh85 != 0) {
                    break;
                }
                zrsh(temp.as_mut_ptr(), (*a).as_mut_ptr(), 1 as libc::c_int as size_t);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_ztrunc(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh86 = k;
            k = k.wrapping_sub(1);
            if !(fresh86 != 0) {
                break;
            }
            ztrunc(temp.as_mut_ptr(), (*a).as_mut_ptr(), i / 2 as libc::c_int as size_t);
            ztrunc(temp.as_mut_ptr(), (*a).as_mut_ptr(), i / 2 as libc::c_int as size_t);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh87 = j;
                j = j.wrapping_sub(1);
                if !(fresh87 != 0) {
                    break;
                }
                ztrunc(
                    temp.as_mut_ptr(),
                    (*a).as_mut_ptr(),
                    i / 2 as libc::c_int as size_t,
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_self_ztrunc(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh88 = k;
            k = k.wrapping_sub(1);
            if !(fresh88 != 0) {
                break;
            }
            ztrunc((*a).as_mut_ptr(), (*a).as_mut_ptr(), i);
            ztrunc((*a).as_mut_ptr(), (*a).as_mut_ptr(), i);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh89 = j;
                j = j.wrapping_sub(1);
                if !(fresh89 != 0) {
                    break;
                }
                ztrunc((*a).as_mut_ptr(), (*a).as_mut_ptr(), i);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zsplit(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh90 = k;
            k = k.wrapping_sub(1);
            if !(fresh90 != 0) {
                break;
            }
            zsplit(
                temp.as_mut_ptr(),
                temp2.as_mut_ptr(),
                (*a).as_mut_ptr(),
                i / 2 as libc::c_int as size_t,
            );
            zsplit(
                temp.as_mut_ptr(),
                temp2.as_mut_ptr(),
                (*a).as_mut_ptr(),
                i / 2 as libc::c_int as size_t,
            );
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh91 = j;
                j = j.wrapping_sub(1);
                if !(fresh91 != 0) {
                    break;
                }
                zsplit(
                    temp.as_mut_ptr(),
                    temp2.as_mut_ptr(),
                    (*a).as_mut_ptr(),
                    i / 2 as libc::c_int as size_t,
                );
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zneg(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh92 = k;
            k = k.wrapping_sub(1);
            if !(fresh92 != 0) {
                break;
            }
            zneg(temp.as_mut_ptr(), (*a).as_mut_ptr());
            zneg(temp.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh93 = j;
                j = j.wrapping_sub(1);
                if !(fresh93 != 0) {
                    break;
                }
                zneg(temp.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zabs(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh94 = k;
            k = k.wrapping_sub(1);
            if !(fresh94 != 0) {
                break;
            }
            zabs(temp.as_mut_ptr(), (*a).as_mut_ptr());
            zabs(temp.as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh95 = j;
                j = j.wrapping_sub(1);
                if !(fresh95 != 0) {
                    break;
                }
                zabs(temp.as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_self_zneg(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh96 = k;
            k = k.wrapping_sub(1);
            if !(fresh96 != 0) {
                break;
            }
            zneg((*a).as_mut_ptr(), (*a).as_mut_ptr());
            zneg((*a).as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh97 = j;
                j = j.wrapping_sub(1);
                if !(fresh97 != 0) {
                    break;
                }
                zneg((*a).as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_self_zabs(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh98 = k;
            k = k.wrapping_sub(1);
            if !(fresh98 != 0) {
                break;
            }
            zabs((*a).as_mut_ptr(), (*a).as_mut_ptr());
            zabs((*a).as_mut_ptr(), (*a).as_mut_ptr());
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh99 = j;
                j = j.wrapping_sub(1);
                if !(fresh99 != 0) {
                    break;
                }
                zabs((*a).as_mut_ptr(), (*a).as_mut_ptr());
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
unsafe extern "C" fn bench_zstr_length(
    mut as_0: *mut z_t,
    mut bs: *mut z_t,
    mut f: *mut function,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n: size_t = ((*f).a_end)
        .wrapping_sub((*f).a_start)
        .wrapping_add(1 as libc::c_int as size_t);
    let mut a: *mut z_t = 0 as *mut z_t;
    zmul(
        temp.as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
        (*as_0.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize)).as_mut_ptr(),
    );
    zadd(temp.as_mut_ptr(), temp.as_mut_ptr(), temp.as_mut_ptr());
    i = 0 as libc::c_int as size_t;
    while i < n {
        a = as_0.offset(i as isize);
        zset(temp2.as_mut_ptr(), (*a).as_mut_ptr());
        k = (*f).measurements;
        loop {
            let fresh100 = k;
            k = k.wrapping_sub(1);
            if !(fresh100 != 0) {
                break;
            }
            zstr_length((*a).as_mut_ptr(), 10 as libc::c_int as libc::c_ulonglong);
            zstr_length((*a).as_mut_ptr(), 10 as libc::c_int as libc::c_ulonglong);
            j = (*f).runs;
            rdtsc(&mut start_low, &mut start_high);
            loop {
                let fresh101 = j;
                j = j.wrapping_sub(1);
                if !(fresh101 != 0) {
                    break;
                }
                zstr_length((*a).as_mut_ptr(), 10 as libc::c_int as libc::c_ulonglong);
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
            dur_seconds
                /= 1000 as libc::c_int as libc::c_double * freq as libc::c_double;
            dur.tv_sec = dur_seconds as libc::c_int as __time_t;
            dur_seconds -= dur.tv_sec as libc::c_double;
            dur
                .tv_nsec = (dur_seconds * 1000000000 as libc::c_long as libc::c_double)
                as libc::c_long;
            measurements[k
                as usize] = (dur.tv_sec as libc::c_ulonglong)
                .wrapping_mul(1000000000 as libc::c_ulonglong)
                .wrapping_add(dur.tv_nsec as libc::c_ulonglong);
        }
        printf(
            b"%llu\n\0" as *const u8 as *const libc::c_char,
            gettime((*f).measurements),
        );
        a = a.offset(1);
        a;
        i = i.wrapping_add((*f).a_step);
    }
}
#[no_mangle]
pub static mut functions: [function; 52] = unsafe {
    [
        {
            let mut init = function {
                name: b"pos_zseti\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_pos_zseti
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 0 as libc::c_int as size_t,
                a_end: 0 as libc::c_int as size_t,
                a_step: 0 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: 0 as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zseti\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zseti
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 0 as libc::c_int as size_t,
                a_end: 0 as libc::c_int as size_t,
                a_step: 0 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: 0 as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsetu\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsetu
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 0 as libc::c_int as size_t,
                a_end: 0 as libc::c_int as size_t,
                a_step: 0 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: 0 as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zset\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zset
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zneg\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zneg
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zabs\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zabs
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"self_zneg\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_self_zneg
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"self_zabs\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_self_zabs
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zadd_unsigned\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zadd_unsigned
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsub_unsigned\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsub_unsigned
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zadd\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zadd
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsub\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsub
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zand\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zand
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zor\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zor
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zxor\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zxor
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"znot\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_znot
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zeven\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zeven
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zodd\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zodd
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zeven_nonzero\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zeven_nonzero
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zodd_nonzero\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zodd_nonzero
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zzero\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zzero
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsignum\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsignum
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zbits\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zbits
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zlsb\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zlsb
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: HIGH_ONLY as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zswap\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zswap
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zcmpmag\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zcmpmag
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zcmp\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zcmp
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"pos_zcmpi\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_pos_zcmpi
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zcmpi\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zcmpi
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zcmpu\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zcmpu
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"sqr_zmul\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_sqr_zmul
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 10 as libc::c_int as size_t,
                measurements: 20 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsqr\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsqr
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 10 as libc::c_int as size_t,
                measurements: 20 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zstr_length\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zstr_length
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 10 as libc::c_int as size_t,
                measurements: 20 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zstr\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zstr
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 10 as libc::c_int as size_t,
                measurements: 20 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"auto_zstr\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_auto_zstr
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 10 as libc::c_int as size_t,
                measurements: 20 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsave\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsave
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zload\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zload
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zbset_set\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zbset_set
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zbset_clear\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zbset_clear
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zbset_flip\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zbset_flip
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"self_zbset_set\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_self_zbset_set
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"self_zbset_clear\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_self_zbset_clear
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"self_zbset_flip\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_self_zbset_flip
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zbtest\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zbtest
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zptest\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zptest
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsets\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsets
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zlsh\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zlsh
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zrsh\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zrsh
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"ztrunc\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_ztrunc
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"self_ztrunc\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_self_ztrunc
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: b"zsplit\0" as *const u8 as *const libc::c_char,
                f: Some(
                    bench_zsplit
                        as unsafe extern "C" fn(*mut z_t, *mut z_t, *mut function) -> (),
                ),
                a_start: 1 as libc::c_int as size_t,
                a_end: 4097 as libc::c_int as size_t,
                a_step: 64 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: FULL as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 1000 as libc::c_int as size_t,
                measurements: 200 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = function {
                name: 0 as *const libc::c_char,
                f: None,
                a_start: 0 as libc::c_int as size_t,
                a_end: 0 as libc::c_int as size_t,
                a_step: 0 as libc::c_int as size_t,
                b_start: 0 as libc::c_int as size_t,
                b_end: 0 as libc::c_int as size_t,
                b_step: 0 as libc::c_int as size_t,
                a_mode: 0 as libc::c_int,
                b_mode: 0 as libc::c_int,
                runs: 0 as libc::c_int as size_t,
                measurements: 0 as libc::c_int as size_t,
            };
            init
        },
    ]
};
unsafe extern "C" fn create_ints(
    mut start: size_t,
    mut end: size_t,
    mut mode: libc::c_int,
) -> *mut z_t {
    end = end.wrapping_add(1);
    let mut array: *mut z_t = malloc(
        end
            .wrapping_sub(start)
            .wrapping_mul(::core::mem::size_of::<z_t>() as libc::c_ulong),
    ) as *mut z_t;
    let mut rc: *mut z_t = array;
    let mut n: ssize_t = 0;
    while start < end {
        zinit((*array).as_mut_ptr());
        match mode {
            0 => {
                zsetu(temp.as_mut_ptr(), 1 as libc::c_int as uint64_t);
                zlsh(
                    (*array).as_mut_ptr(),
                    temp.as_mut_ptr(),
                    start.wrapping_sub(1 as libc::c_int as size_t),
                );
            }
            1 => {
                zsetu(temp.as_mut_ptr(), 1 as libc::c_int as uint64_t);
                zlsh(
                    (*array).as_mut_ptr(),
                    temp.as_mut_ptr(),
                    start.wrapping_sub(1 as libc::c_int as size_t),
                );
                if start > 1 as libc::c_int as size_t {
                    zadd(
                        (*array).as_mut_ptr(),
                        (*array).as_mut_ptr(),
                        temp.as_mut_ptr(),
                    );
                }
            }
            2 => {
                n = start as ssize_t;
                zsetu(
                    temp.as_mut_ptr(),
                    ((1 as libc::c_int) << (!start & 1 as libc::c_int as size_t))
                        as uint64_t,
                );
                zsetu((*array).as_mut_ptr(), 0 as libc::c_int as uint64_t);
                while n > 0 as libc::c_int as ssize_t {
                    zlsh(
                        (*array).as_mut_ptr(),
                        (*array).as_mut_ptr(),
                        2 as libc::c_int as size_t,
                    );
                    zadd(
                        (*array).as_mut_ptr(),
                        (*array).as_mut_ptr(),
                        temp.as_mut_ptr(),
                    );
                    n -= 2 as libc::c_int as ssize_t;
                }
            }
            3 => {
                zsetu(temp.as_mut_ptr(), 1 as libc::c_int as uint64_t);
                zlsh((*array).as_mut_ptr(), temp.as_mut_ptr(), start);
                zsub((*array).as_mut_ptr(), (*array).as_mut_ptr(), temp.as_mut_ptr());
            }
            _ => {
                abort();
            }
        }
        start = start.wrapping_add(1);
        start;
        array = array.offset(1);
        array;
    }
    return rc;
}
unsafe extern "C" fn destroy_ints(
    mut array: *mut z_t,
    mut start: size_t,
    mut end: size_t,
) {
    let mut array_: *mut z_t = array;
    while start <= end {
        let fresh102 = array;
        array = array.offset(1);
        zfree((*fresh102).as_mut_ptr());
        start = start.wrapping_add(1);
        start;
    }
    free(array_ as *mut libc::c_void);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    static mut fs: *mut function = unsafe { functions.as_ptr() as *mut _ };
    static mut as_0: *mut z_t = 0 as *const z_t as *mut z_t;
    static mut bs: *mut z_t = 0 as *const z_t as *mut z_t;
    let mut jmp: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    if argc != 2 as libc::c_int {
        fprintf(
            stderr,
            b"usage: %s function\n\0" as *const u8 as *const libc::c_char,
            *argv,
        );
        return 2 as libc::c_int;
    }
    benchmark_init();
    if _setjmp(jmp.as_mut_ptr()) != 0 {
        zperror(*argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    zsetup(jmp.as_mut_ptr());
    printf(
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        b"libzahl\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    zinit(temp.as_mut_ptr());
    zinit(temp2.as_mut_ptr());
    while !((*fs).name).is_null()
        && strcmp((*fs).name, *argv.offset(1 as libc::c_int as isize)) != 0
    {
        fs = fs.offset(1);
        fs;
    }
    if ((*fs).name).is_null() {
        fprintf(
            stderr,
            b"%s: function not recognised: %s\n\0" as *const u8 as *const libc::c_char,
            *argv,
            *argv.offset(1 as libc::c_int as isize),
        );
        return 2 as libc::c_int;
    }
    if (*fs).b_end != 0 {
        as_0 = create_ints((*fs).a_start, (*fs).a_end, (*fs).a_mode);
        bs = create_ints((*fs).b_start, (*fs).b_end, (*fs).b_mode);
        printf(
            b"3\n%zu %zu %zu\n%zu %zu %zu\n\0" as *const u8 as *const libc::c_char,
            (*fs).a_start,
            (*fs).a_end,
            (*fs).a_step,
            (*fs).b_start,
            (*fs).b_end,
            (*fs).b_step,
        );
    } else if (*fs).a_end != 0 {
        as_0 = create_ints((*fs).a_start, (*fs).a_end, (*fs).a_mode);
        printf(
            b"2\n%zu %zu %zu\n\0" as *const u8 as *const libc::c_char,
            (*fs).a_start,
            (*fs).a_end,
            (*fs).a_step,
        );
    } else {
        printf(b"1\n\0" as *const u8 as *const libc::c_char);
    }
    ((*fs).f).expect("non-null function pointer")(as_0, bs, fs);
    if !as_0.is_null() {
        destroy_ints(as_0, (*fs).a_start, (*fs).a_end);
    }
    if !bs.is_null() {
        destroy_ints(bs, (*fs).b_start, (*fs).b_end);
    }
    zfree(temp.as_mut_ptr());
    zfree(temp2.as_mut_ptr());
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
