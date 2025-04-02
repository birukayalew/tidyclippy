use ::libc;
extern "C" {
    fn nan(_: *const libc::c_char) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRIO_FP_ZERO: C2RustUnnamed = 4;
pub const TRIO_FP_SUBNORMAL: C2RustUnnamed = 3;
pub const TRIO_FP_NORMAL: C2RustUnnamed = 2;
pub const TRIO_FP_NAN: C2RustUnnamed = 1;
pub const TRIO_FP_INFINITE: C2RustUnnamed = 0;
static mut internalEndianMagic: libc::c_double = 7.949928895127363e-275f64;
static mut ieee_754_negzero_array: [libc::c_uchar; 8] = [
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn trio_make_double(
    mut values: *const libc::c_uchar,
) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int {
        ::core::ptr::write_volatile(
            (&mut result as *mut libc::c_double as *mut libc::c_uchar)
                .offset(
                    *(&internalEndianMagic as *const libc::c_double
                        as *mut libc::c_uchar)
                        .offset((7 as libc::c_int - i) as isize) as isize,
                ),
            *values.offset(i as isize),
        );
        i += 1;
        i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_nzero() -> libc::c_double {
    return trio_make_double(ieee_754_negzero_array.as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn trio_pinf() -> libc::c_double {
    static mut result: libc::c_double = 0.0f64;
    if result == 0.0f64 {
        result = ::core::f32::INFINITY as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_ninf() -> libc::c_double {
    static mut result: libc::c_double = 0.0f64;
    if result == 0.0f64 {
        result = -trio_pinf();
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_nan() -> libc::c_double {
    static mut result: libc::c_double = 0.0f64;
    if result == 0.0f64 {
        result = nan(b"\0" as *const u8 as *const libc::c_char);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn trio_isnan(mut number: libc::c_double) -> libc::c_int {
    return number.is_nan() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn trio_isinf(mut number: libc::c_double) -> libc::c_int {
    return if if number.is_infinite() {
        if number.is_sign_positive() { 1 } else { -1 }
    } else {
        0
    } != 0
    {
        if number > 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) }
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn trio_signbit(mut number: libc::c_double) -> libc::c_int {
    let mut is_negative: libc::c_int = 0;
    trio_fpclassify_and_signbit(number, &mut is_negative);
    return is_negative;
}
