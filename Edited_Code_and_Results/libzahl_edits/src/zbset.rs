#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
unsafe extern "C" fn libzahl_memset(
    mut a: *mut zahl_char_t,
    mut v: zahl_char_t,
    mut n: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        *a.offset(i.wrapping_add(0 as libc::c_int as size_t) as isize) = v;
        *a.offset(i.wrapping_add(1 as libc::c_int as size_t) as isize) = v;
        *a.offset(i.wrapping_add(2 as libc::c_int as size_t) as isize) = v;
        *a.offset(i.wrapping_add(3 as libc::c_int as size_t) as isize) = v;
        i = i.wrapping_add(4 as libc::c_int as size_t);
    }
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_set(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = bit >> 6 as libc::c_int;
    if zzero(a) != 0 {
        (*a).used = 0 as libc::c_int as size_t;
        (*a).sign = 1 as libc::c_int;
    }
    if (chars >= (*a).used) as libc::c_int as libc::c_long != 0 {
        if (*a).alloced < chars.wrapping_add(1 as libc::c_int as size_t) {
            libzahl_realloc(a, chars.wrapping_add(1 as libc::c_int as size_t));
        }
        libzahl_memset(
            ((*a).chars).offset((*a).used as isize),
            0 as libc::c_int as zahl_char_t,
            chars.wrapping_add(1 as libc::c_int as size_t).wrapping_sub((*a).used),
        );
        (*a).used = chars.wrapping_add(1 as libc::c_int as size_t);
    }
    bit = bit & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    mask <<= bit;
    *((*a).chars).offset(chars as isize) |= mask;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_clear(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = bit >> 6 as libc::c_int;
    if (chars >= (*a).used) as libc::c_int as libc::c_long != 0 {
        return;
    }
    bit = bit & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    mask <<= bit;
    *((*a).chars).offset(chars as isize) &= !mask;
    while (*a).used != 0
        && *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    if (*a).used == 0 {
        (*a).sign = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_flip(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = bit >> 6 as libc::c_int;
    if zzero(a) != 0 {
        (*a).used = 0 as libc::c_int as size_t;
        (*a).sign = 1 as libc::c_int;
    }
    if (chars >= (*a).used) as libc::c_int as libc::c_long != 0 {
        if (*a).alloced < chars.wrapping_add(1 as libc::c_int as size_t) {
            libzahl_realloc(a, chars.wrapping_add(1 as libc::c_int as size_t));
        }
        libzahl_memset(
            ((*a).chars).offset((*a).used as isize),
            0 as libc::c_int as zahl_char_t,
            chars.wrapping_add(1 as libc::c_int as size_t).wrapping_sub((*a).used),
        );
        (*a).used = chars.wrapping_add(1 as libc::c_int as size_t);
    }
    bit = bit & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    mask <<= bit;
    *((*a).chars).offset(chars as isize) ^= mask;
    while (*a).used != 0
        && *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    if (*a).used == 0 {
        (*a).sign = 0 as libc::c_int;
    }
}
