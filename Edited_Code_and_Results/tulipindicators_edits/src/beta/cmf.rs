use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ti_wma_start(options: *const libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cmf_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cmf(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_wma_start(options) {
        return 0 as libc::c_int;
    }
    let mut period_volume: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut period_ad_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        period_ad_sum
            += if *high.offset(i as isize) - *low.offset(i as isize) != 0. {
                *volume.offset(i as isize)
                    * (*close.offset(i as isize) - *low.offset(i as isize)
                        - (*high.offset(i as isize) - *close.offset(i as isize)))
                    / (*high.offset(i as isize) - *low.offset(i as isize))
            } else {
                0.0f64
            };
        period_volume += *volume.offset(i as isize);
        i += 1;
        i;
    }
    i = period - 1 as libc::c_int;
    while i < size {
        period_ad_sum
            += if *high.offset(i as isize) - *low.offset(i as isize) != 0. {
                *volume.offset(i as isize)
                    * (*close.offset(i as isize) - *low.offset(i as isize)
                        - (*high.offset(i as isize) - *close.offset(i as isize)))
                    / (*high.offset(i as isize) - *low.offset(i as isize))
            } else {
                0.0f64
            };
        period_volume += *volume.offset(i as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = period_ad_sum / period_volume;
        period_ad_sum
            -= if *high.offset((i - period + 1 as libc::c_int) as isize)
                - *low.offset((i - period + 1 as libc::c_int) as isize) != 0.
            {
                *volume.offset((i - period + 1 as libc::c_int) as isize)
                    * (*close.offset((i - period + 1 as libc::c_int) as isize)
                        - *low.offset((i - period + 1 as libc::c_int) as isize)
                        - (*high.offset((i - period + 1 as libc::c_int) as isize)
                            - *close.offset((i - period + 1 as libc::c_int) as isize)))
                    / (*high.offset((i - period + 1 as libc::c_int) as isize)
                        - *low.offset((i - period + 1 as libc::c_int) as isize))
            } else {
                0.0f64
            };
        period_volume -= *volume.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_cmf_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_cmf_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/cmf.c\0"
                as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_cmf(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3549: {
        if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_cmf_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"output - outputs[0] == size - ti_cmf_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/cmf.c\0"
                    as *const u8 as *const libc::c_char,
                69 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int ti_cmf(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
