#![feature(label_break_value)]

use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmta_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmta(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let beta: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rmta_start(options) {
        return 0 as libc::c_int;
    }
    let alpha: libc::c_double = 1.0f64 - beta;
    let mut b: libc::c_double = (1.0f64 - alpha)
        * *input.offset(0 as libc::c_int as isize)
        + *input.offset(0 as libc::c_int as isize);
    let mut rmta: libc::c_double = (1.0f64 - alpha)
        * *input.offset(0 as libc::c_int as isize)
        + alpha * (*input.offset(0 as libc::c_int as isize) + b);
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < period - 1 as libc::c_int {
        let mut next_b: libc::c_double = (1.0f64 - alpha) * b
            + *input.offset(i as isize);
        rmta = (1.0f64 - alpha) * rmta
            + alpha * (*input.offset(i as isize) + next_b - b);
        b = next_b;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = period - 1 as libc::c_int;
    while i_0 < size {
        let mut next_b_0: libc::c_double = (1.0f64 - alpha) * b
            + *input.offset(i_0 as isize);
        rmta = (1.0f64 - alpha) * rmta
            + alpha * (*input.offset(i_0 as isize) + next_b_0 - b);
        b = next_b_0;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = rmta;
        i_0 += 1;
        i_0;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_rmta_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_rmta_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/rmta.c\0"
                as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_rmta(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3547: {
        if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_rmta_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"output - outputs[0] == size - ti_rmta_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/rmta.c\0"
                    as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"int ti_rmta(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
