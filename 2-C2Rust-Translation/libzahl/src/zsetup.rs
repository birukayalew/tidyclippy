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
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[no_mangle]
pub static mut libzahl_tmp_divmod_d: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_mod: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_b: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modmul: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_sub: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_v: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_u: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_rem: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_div: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_mag: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_num: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n4: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n1: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_d: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_a: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_x: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_div: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_d: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_b: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_a: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modsqr: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_c: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_4: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1e19: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_2: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1: z_t = [zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_ds: [z_t; 64] = [[zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1]; 64];
#[no_mangle]
pub static mut libzahl_jmp_buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut libzahl_set_up: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut libzahl_error: libc::c_int = 0;
#[no_mangle]
pub static mut libzahl_pool: [*mut *mut zahl_char_t; 64] = [0 as *const *mut zahl_char_t
    as *mut *mut zahl_char_t; 64];
#[no_mangle]
pub static mut libzahl_pool_n: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_pool_alloc: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_temp_stack: *mut *mut zahl = 0 as *const *mut zahl
    as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_stack_head: *mut *mut zahl = 0 as *const *mut zahl
    as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_stack_end: *mut *mut zahl = 0 as *const *mut zahl
    as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_allocation: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut constant_chars: [zahl_char_t; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn zsetup(mut env: *mut __jmp_buf_tag) {
    let mut i: size_t = 0;
    *libzahl_jmp_buf.as_mut_ptr() = *env;
    if (libzahl_set_up == 0) as libc::c_int as libc::c_long != 0 {
        libzahl_set_up = 1 as libc::c_int;
        memset(
            libzahl_pool.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as libc::c_ulong,
        );
        memset(
            libzahl_pool_n.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[size_t; 64]>() as libc::c_ulong,
        );
        memset(
            libzahl_pool_alloc.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[size_t; 64]>() as libc::c_ulong,
        );
        zinit(libzahl_tmp_div.as_mut_ptr());
        zinit(libzahl_tmp_mod.as_mut_ptr());
        zinit(libzahl_tmp_str_num.as_mut_ptr());
        zinit(libzahl_tmp_str_mag.as_mut_ptr());
        zinit(libzahl_tmp_str_div.as_mut_ptr());
        zinit(libzahl_tmp_str_rem.as_mut_ptr());
        zinit(libzahl_tmp_gcd_u.as_mut_ptr());
        zinit(libzahl_tmp_gcd_v.as_mut_ptr());
        zinit(libzahl_tmp_sub.as_mut_ptr());
        zinit(libzahl_tmp_modmul.as_mut_ptr());
        zinit(libzahl_tmp_pow_b.as_mut_ptr());
        zinit(libzahl_tmp_pow_c.as_mut_ptr());
        zinit(libzahl_tmp_pow_d.as_mut_ptr());
        zinit(libzahl_tmp_modsqr.as_mut_ptr());
        zinit(libzahl_tmp_divmod_a.as_mut_ptr());
        zinit(libzahl_tmp_divmod_b.as_mut_ptr());
        zinit(libzahl_tmp_divmod_d.as_mut_ptr());
        zinit(libzahl_tmp_ptest_x.as_mut_ptr());
        zinit(libzahl_tmp_ptest_a.as_mut_ptr());
        zinit(libzahl_tmp_ptest_d.as_mut_ptr());
        zinit(libzahl_tmp_ptest_n1.as_mut_ptr());
        zinit(libzahl_tmp_ptest_n4.as_mut_ptr());
        (*libzahl_const_1e19.as_mut_ptr()).alloced = 1 as libc::c_int as size_t;
        let ref mut fresh0 = (*libzahl_const_1e19.as_mut_ptr()).chars;
        *fresh0 = constant_chars.as_mut_ptr().offset(0 as libc::c_int as isize);
        zsetu(
            libzahl_const_1e19.as_mut_ptr(),
            10000000000000000000 as libc::c_ulonglong as uint64_t,
        );
        (*libzahl_const_1.as_mut_ptr()).alloced = 1 as libc::c_int as size_t;
        let ref mut fresh1 = (*libzahl_const_1.as_mut_ptr()).chars;
        *fresh1 = constant_chars.as_mut_ptr().offset(1 as libc::c_int as isize);
        zsetu(libzahl_const_1.as_mut_ptr(), 1 as libc::c_int as uint64_t);
        (*libzahl_const_2.as_mut_ptr()).alloced = 1 as libc::c_int as size_t;
        let ref mut fresh2 = (*libzahl_const_2.as_mut_ptr()).chars;
        *fresh2 = constant_chars.as_mut_ptr().offset(2 as libc::c_int as isize);
        zsetu(libzahl_const_2.as_mut_ptr(), 2 as libc::c_int as uint64_t);
        (*libzahl_const_4.as_mut_ptr()).alloced = 1 as libc::c_int as size_t;
        let ref mut fresh3 = (*libzahl_const_4.as_mut_ptr()).chars;
        *fresh3 = constant_chars.as_mut_ptr().offset(3 as libc::c_int as isize);
        zsetu(libzahl_const_4.as_mut_ptr(), 4 as libc::c_int as uint64_t);
        i = 64 as libc::c_int as size_t;
        loop {
            let fresh4 = i;
            i = i.wrapping_sub(1);
            if !(fresh4 != 0) {
                break;
            }
            zinit((libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr());
        }
        libzahl_temp_stack = malloc(
            (256 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut zahl>() as libc::c_ulong),
        ) as *mut *mut zahl;
        if libzahl_temp_stack.is_null() as libc::c_int as libc::c_long != 0 {
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack;
        libzahl_temp_stack_end = libzahl_temp_stack.offset(256 as libc::c_int as isize);
    }
}
