use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_sma_start(options: *const libc::c_double) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn ti_alma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let window: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return window - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_alma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let window: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let offset: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let sigma: libc::c_double = *options.offset(2 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if window < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if sigma <= 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if offset < 0 as libc::c_int as libc::c_double
        || offset > 1 as libc::c_int as libc::c_double
    {
        return 1 as libc::c_int;
    }
    if size <= ti_sma_start(options) {
        return 0 as libc::c_int;
    }
    let mut weights: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(window as size_t),
    ) as *mut libc::c_double;
    if weights.is_null() {
        return 2 as libc::c_int;
    }
    let m: libc::c_double = floor(
        offset * (window - 1 as libc::c_int) as libc::c_double,
    );
    let s: libc::c_double = window as libc::c_double / sigma;
    let mut norm: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < window {
        *weights
            .offset(
                i as isize,
            ) = exp(
            -(1 as libc::c_int) as libc::c_double
                * pow(i as libc::c_double - m, 2 as libc::c_int as libc::c_double)
                / (2 as libc::c_int as libc::c_double
                    * pow(s, 2 as libc::c_int as libc::c_double)),
        );
        norm += *weights.offset(i as isize);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < window {
        *weights.offset(i as isize) /= norm;
        i += 1;
        i;
    }
    i = window - 1 as libc::c_int;
    while i < size {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < window {
            sum
                += *input.offset((i - window + j + 1 as libc::c_int) as isize)
                    * *weights.offset(j as isize);
            j += 1;
            j;
        }
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = sum;
        i += 1;
        i;
    }
    free(weights as *mut libc::c_void);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_sma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_sma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/alma.c\0"
                as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_alma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3559: {
        if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_sma_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"output - outputs[0] == size - ti_sma_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/alma.c\0"
                    as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"int ti_alma(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
