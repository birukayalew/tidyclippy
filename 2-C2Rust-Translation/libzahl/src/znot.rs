#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
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
#[no_mangle]
pub unsafe extern "C" fn znot(mut a: *mut zahl, mut b: *mut zahl) {
    let mut bits: size_t = 0;
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    bits = zbits(b);
    (*a).used = (*b).used;
    (*a).sign = -zsignum(b);
    let mut a__: *mut zahl_char_t = (*a).chars;
    let mut b__: *const zahl_char_t = (*b).chars;
    let mut i__: size_t = 0;
    let mut n__: size_t = (*a).used;
    i__ = 0 as libc::c_int as size_t;
    while i__ < n__ {
        *a__
            .offset(
                i__.wrapping_add(0 as libc::c_int as size_t) as isize,
            ) = !*b__.offset(i__.wrapping_add(0 as libc::c_int as size_t) as isize);
        *a__
            .offset(
                i__.wrapping_add(1 as libc::c_int as size_t) as isize,
            ) = !*b__.offset(i__.wrapping_add(1 as libc::c_int as size_t) as isize);
        *a__
            .offset(
                i__.wrapping_add(2 as libc::c_int as size_t) as isize,
            ) = !*b__.offset(i__.wrapping_add(2 as libc::c_int as size_t) as isize);
        *a__
            .offset(
                i__.wrapping_add(3 as libc::c_int as size_t) as isize,
            ) = !*b__.offset(i__.wrapping_add(3 as libc::c_int as size_t) as isize);
        i__ = i__.wrapping_add(4 as libc::c_int as size_t);
    }
    bits = bits & (64 as libc::c_int - 1 as libc::c_int) as size_t;
    if bits != 0 {
        *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as size_t) as isize)
            &= ((1 as libc::c_int as zahl_char_t) << bits)
                .wrapping_sub(1 as libc::c_int as zahl_char_t);
    }
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
