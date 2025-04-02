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
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zfree(_: *mut zahl);
    fn zadd_unsigned_assign(_: *mut zahl, _: *mut zahl);
    fn zsub_nonnegative_assign(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn ztrunc(_: *mut zahl, _: *mut zahl, _: size_t);
    fn __errno_location() -> *mut libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack_end: *mut *mut zahl;
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
#[inline]
unsafe extern "C" fn zinit(mut a: *mut zahl) {
    (*a).alloced = 0 as libc::c_int as size_t;
    (*a).chars = 0 as *mut zahl_char_t;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
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
unsafe extern "C" fn zzero1(mut a: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    return (zzero(a) != 0 || zzero(b) != 0) as libc::c_int;
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
unsafe extern "C" fn libzahl_memfailure() {
    if *__errno_location() == 0 {
        *__errno_location() = 2 as libc::c_int;
    }
    libzahl_failure(*__errno_location());
}
#[inline]
unsafe extern "C" fn zsplit_pz(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut delim: size_t,
) {
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        (*high).sign = 0 as libc::c_int;
        (*low).sign = 0 as libc::c_int;
    } else {
        zsplit(high, low, a, delim);
    };
}
#[inline]
unsafe extern "C" fn zinit_temp(mut a: *mut zahl) {
    zinit(a);
    if (libzahl_temp_stack_head == libzahl_temp_stack_end) as libc::c_int as libc::c_long
        != 0
    {
        let mut n: size_t = libzahl_temp_stack_end.offset_from(libzahl_temp_stack)
            as libc::c_long as size_t;
        let mut old: *mut libc::c_void = libzahl_temp_stack as *mut libc::c_void;
        libzahl_temp_stack = realloc(
            old,
            (2 as libc::c_int as size_t * n)
                .wrapping_mul(::core::mem::size_of::<*mut zahl>() as libc::c_ulong),
        ) as *mut *mut zahl;
        if libzahl_temp_stack.is_null() as libc::c_int as libc::c_long != 0 {
            libzahl_temp_stack = old as *mut *mut zahl;
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack.offset(n as isize);
        libzahl_temp_stack_end = libzahl_temp_stack_head.offset(n as isize);
    }
    let fresh0 = libzahl_temp_stack_head;
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(1);
    *fresh0 = a;
}
#[inline]
unsafe extern "C" fn zfree_temp(mut a: *mut zahl) {
    zfree(a);
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
    libzahl_temp_stack_head;
}
#[inline]
unsafe extern "C" fn zmul_ll_single_char(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    if (*a).alloced < 1 as libc::c_int as size_t {
        libzahl_realloc(a, 1 as libc::c_int as size_t);
    }
    (*a).used = 1 as libc::c_int as size_t;
    *((*a).chars)
        .offset(
            0 as libc::c_int as isize,
        ) = *((*b).chars).offset(0 as libc::c_int as isize)
        * *((*c).chars).offset(0 as libc::c_int as isize);
    (*a).sign = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmul_ll(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut m: size_t = 0;
    let mut m2: size_t = 0;
    let mut b_high: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_low: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut c_high: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut c_low: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    if (zzero1(b, c) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    m = zbits(b);
    m2 = if b == c { m } else { zbits(c) };
    if m.wrapping_add(m2) <= 64 as libc::c_int as size_t {
        zmul_ll_single_char(a, b, c);
        return;
    }
    m = if m > m2 { m } else { m2 };
    m2 = m >> 1 as libc::c_int;
    zinit_temp(b_high.as_mut_ptr());
    zinit_temp(b_low.as_mut_ptr());
    zinit_temp(c_high.as_mut_ptr());
    zinit_temp(c_low.as_mut_ptr());
    zsplit_pz(b_high.as_mut_ptr(), b_low.as_mut_ptr(), b, m2);
    zsplit_pz(c_high.as_mut_ptr(), c_low.as_mut_ptr(), c, m2);
    zmul_ll(a, b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zadd_unsigned_assign(b_low.as_mut_ptr(), b_high.as_mut_ptr());
    zadd_unsigned_assign(c_low.as_mut_ptr(), c_high.as_mut_ptr());
    zmul_ll(b_low.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zmul_ll(c_low.as_mut_ptr(), b_high.as_mut_ptr(), c_high.as_mut_ptr());
    zsub_nonnegative_assign(b_low.as_mut_ptr(), a);
    zsub_nonnegative_assign(b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zlsh(b_low.as_mut_ptr(), b_low.as_mut_ptr(), m2);
    m2 <<= 1 as libc::c_int;
    zlsh(c_low.as_mut_ptr(), c_low.as_mut_ptr(), m2);
    zadd_unsigned_assign(a, b_low.as_mut_ptr());
    zadd_unsigned_assign(a, c_low.as_mut_ptr());
    zfree_temp(c_low.as_mut_ptr());
    zfree_temp(c_high.as_mut_ptr());
    zfree_temp(b_low.as_mut_ptr());
    zfree_temp(b_high.as_mut_ptr());
}
