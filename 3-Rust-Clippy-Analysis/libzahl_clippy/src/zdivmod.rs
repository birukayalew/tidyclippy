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
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zbset_ll_set(_: *mut zahl, _: size_t);
    fn zbset_ll_clear(_: *mut zahl, _: size_t);
    fn zbset_ll_flip(_: *mut zahl, _: size_t);
    fn free(_: *mut libc::c_void);
    static mut libzahl_tmp_divmod_d: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_ds: [z_t; 64];
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
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
unsafe extern "C" fn zrsh_taint(mut a: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if (bits == 0) as libc::c_int as libc::c_long != 0 {
        return;
    }
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return;
    }
    chars = bits >> 6 as libc::c_int;
    if (chars >= (*a).used || zbits(a) <= bits) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    bits = bits & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    cbits = (64 as libc::c_int as size_t).wrapping_sub(bits);
    if (chars != 0) as libc::c_int as libc::c_long != 0 {
        (*a).used = ((*a).used).wrapping_sub(chars);
        (*a).chars = ((*a).chars).offset(chars as isize);
    }
    if (bits != 0) as libc::c_int as libc::c_long != 0 {
        *((*a).chars).offset(0 as libc::c_int as isize) >>= bits;
        i = 1 as libc::c_int as size_t;
        while i < (*a).used {
            *((*a).chars).offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize)
                |= *((*a).chars).offset(i as isize) << cbits;
            *((*a).chars).offset(i as isize) >>= bits;
            i = i.wrapping_add(1);
            i;
        }
        while *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
        {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        }
    }
}
#[inline]
unsafe extern "C" fn zdivmod_impl(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
    mut d: *mut zahl,
) {
    let mut c_bits: size_t = 0;
    let mut d_bits: size_t = 0;
    let mut bit: size_t = 0;
    let mut i: size_t = 0;
    static mut tds: [z_t; 64] = [[zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1]; 64];
    c_bits = zbits(c);
    d_bits = zbits(d);
    bit = c_bits.wrapping_sub(d_bits);
    zlsh(libzahl_tmp_divmod_d.as_mut_ptr(), d, bit);
    (*libzahl_tmp_divmod_d.as_mut_ptr()).sign = 1 as libc::c_int;
    if zcmpmag(libzahl_tmp_divmod_d.as_mut_ptr(), c) > 0 as libc::c_int {
        zrsh(
            libzahl_tmp_divmod_d.as_mut_ptr(),
            libzahl_tmp_divmod_d.as_mut_ptr(),
            1 as libc::c_int as size_t,
        );
        bit = bit.wrapping_sub(1 as libc::c_int as size_t);
    }
    (*libzahl_tmp_divmod_a.as_mut_ptr()).sign = 0 as libc::c_int;
    zabs(libzahl_tmp_divmod_b.as_mut_ptr(), c);
    if (bit <= 64 as libc::c_int as size_t) as libc::c_int as libc::c_long != 0 {
        loop {
            if zcmpmag(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_b.as_mut_ptr(),
            ) <= 0 as libc::c_int
            {
                zsub_unsigned(
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                );
                zbset(
                    libzahl_tmp_divmod_a.as_mut_ptr(),
                    libzahl_tmp_divmod_a.as_mut_ptr(),
                    bit,
                    1 as libc::c_int,
                );
            }
            let fresh0 = bit;
            bit = bit.wrapping_sub(1);
            if fresh0 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                break;
            }
            zrsh(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                1 as libc::c_int as size_t,
            );
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < 64 as libc::c_int as size_t {
            zrsh(
                (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                i,
            );
            (*(tds[i as usize]).as_mut_ptr())
                .used = (*(libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr()).used;
            (*(tds[i as usize]).as_mut_ptr())
                .sign = (*(libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr()).sign;
            let ref mut fresh1 = (*(tds[i as usize]).as_mut_ptr()).chars;
            *fresh1 = (*(libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr()).chars;
            i = i.wrapping_add(1);
            i;
        }
        's_114: loop {
            i = 0 as libc::c_int as size_t;
            while i < 64 as libc::c_int as size_t {
                if zcmpmag(
                    (tds[i as usize]).as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                ) <= 0 as libc::c_int
                {
                    zsub_unsigned(
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        (tds[i as usize]).as_mut_ptr(),
                    );
                    zbset(
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        bit,
                        1 as libc::c_int,
                    );
                }
                let fresh2 = bit;
                bit = bit.wrapping_sub(1);
                if fresh2 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                    break 's_114;
                }
                i = i.wrapping_add(1);
                i;
            }
            i = (if bit < (64 as libc::c_int - 1 as libc::c_int) as size_t {
                bit
            } else {
                (64 as libc::c_int - 1 as libc::c_int) as size_t
            })
                .wrapping_add(1 as libc::c_int as size_t);
            loop {
                let fresh3 = i;
                i = i.wrapping_sub(1);
                if !(fresh3 != 0) {
                    break;
                }
                zrsh_taint((tds[i as usize]).as_mut_ptr(), 64 as libc::c_int as size_t);
            }
        }
    }
    zswap(a, libzahl_tmp_divmod_a.as_mut_ptr());
    zswap(b, libzahl_tmp_divmod_b.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn zdivmod(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
    mut d: *mut zahl,
) {
    let mut c_sign: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut cmpmag: libc::c_int = 0;
    c_sign = zsignum(c);
    sign = c_sign * zsignum(d);
    if (sign == 0) as libc::c_int as libc::c_long != 0 {
        if (zzero(c) == 0) as libc::c_int as libc::c_long != 0 {
            libzahl_failure(-(ZERROR_DIV_0 as libc::c_int));
        } else if (zzero(d) != 0) as libc::c_int as libc::c_long != 0 {
            libzahl_failure(-(ZERROR_0_DIV_0 as libc::c_int));
        } else {
            (*a).sign = 0 as libc::c_int;
            (*b).sign = 0 as libc::c_int;
        }
        return;
    } else {
        cmpmag = zcmpmag(c, d);
        if (cmpmag <= 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            if (cmpmag == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                zseti(a, sign as int64_t);
                (*b).sign = 0 as libc::c_int;
            } else {
                if b != c {
                    zset(b, c);
                }
                (*a).sign = 0 as libc::c_int;
            }
            return;
        }
    }
    zdivmod_impl(a, b, c, d);
    (*a).sign = sign;
    if zsignum(b) > 0 as libc::c_int {
        (*b).sign = c_sign;
    }
}
