#![feature(label_break_value)]

use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ti_buffer_new(size: libc::c_int) -> *mut ti_buffer;
    fn ti_buffer_free(buffer: *mut ti_buffer);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: libc::c_int,
    pub pushes: libc::c_int,
    pub index: libc::c_int,
    pub sum: libc::c_double,
    pub vals: [libc::c_double; 1],
}
#[no_mangle]
pub unsafe extern "C" fn ti_abands_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    return period as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_abands(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut lower_band: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut upper_band: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut middle_point: *mut libc::c_double = *outputs
        .offset(2 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_abands_start(options) {
        return 0 as libc::c_int;
    }
    let mut per: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut buffer_high: *mut ti_buffer = ti_buffer_new(period);
    let mut buffer_low: *mut ti_buffer = ti_buffer_new(period);
    let mut close_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < period {
        let mut mult: libc::c_double = 4.0f64
            * (*high.offset(i as isize) - *low.offset(i as isize))
            / (*high.offset(i as isize) + *low.offset(i as isize));
        let mut high_val: libc::c_double = (1.0f64 + mult) * *high.offset(i as isize);
        if (*buffer_high).pushes >= (*buffer_high).size {
            (*buffer_high).sum
                -= *((*buffer_high).vals)
                    .as_mut_ptr()
                    .offset((*buffer_high).index as isize);
        }
        (*buffer_high).sum += high_val;
        *((*buffer_high).vals)
            .as_mut_ptr()
            .offset((*buffer_high).index as isize) = high_val;
        (*buffer_high).pushes += 1 as libc::c_int;
        (*buffer_high).index = (*buffer_high).index + 1 as libc::c_int;
        if (*buffer_high).index >= (*buffer_high).size {
            (*buffer_high).index = 0 as libc::c_int;
        }
        let mut low_val: libc::c_double = (1.0f64 - mult) * *low.offset(i as isize);
        if (*buffer_low).pushes >= (*buffer_low).size {
            (*buffer_low).sum
                -= *((*buffer_low).vals)
                    .as_mut_ptr()
                    .offset((*buffer_low).index as isize);
        }
        (*buffer_low).sum += low_val;
        *((*buffer_low).vals)
            .as_mut_ptr()
            .offset((*buffer_low).index as isize) = low_val;
        (*buffer_low).pushes += 1 as libc::c_int;
        (*buffer_low).index = (*buffer_low).index + 1 as libc::c_int;
        if (*buffer_low).index >= (*buffer_low).size {
            (*buffer_low).index = 0 as libc::c_int;
        }
        close_sum += *close.offset(i as isize);
        i += 1;
        i;
    }
    let fresh0 = upper_band;
    upper_band = upper_band.offset(1);
    *fresh0 = (*buffer_high).sum * per;
    let fresh1 = lower_band;
    lower_band = lower_band.offset(1);
    *fresh1 = (*buffer_low).sum * per;
    let fresh2 = middle_point;
    middle_point = middle_point.offset(1);
    *fresh2 = close_sum * per;
    let mut i_0: libc::c_int = period;
    while i_0 < size {
        let mut mult_0: libc::c_double = 4.0f64
            * (*high.offset(i_0 as isize) - *low.offset(i_0 as isize))
            / (*high.offset(i_0 as isize) + *low.offset(i_0 as isize));
        let mut high_val_0: libc::c_double = (1.0f64 + mult_0)
            * *high.offset(i_0 as isize);
        if (*buffer_high).pushes >= (*buffer_high).size {
            (*buffer_high).sum
                -= *((*buffer_high).vals)
                    .as_mut_ptr()
                    .offset((*buffer_high).index as isize);
        }
        (*buffer_high).sum += high_val_0;
        *((*buffer_high).vals)
            .as_mut_ptr()
            .offset((*buffer_high).index as isize) = high_val_0;
        (*buffer_high).pushes += 1 as libc::c_int;
        (*buffer_high).index = (*buffer_high).index + 1 as libc::c_int;
        if (*buffer_high).index >= (*buffer_high).size {
            (*buffer_high).index = 0 as libc::c_int;
        }
        let mut low_val_0: libc::c_double = (1.0f64 - mult_0)
            * *low.offset(i_0 as isize);
        if (*buffer_low).pushes >= (*buffer_low).size {
            (*buffer_low).sum
                -= *((*buffer_low).vals)
                    .as_mut_ptr()
                    .offset((*buffer_low).index as isize);
        }
        (*buffer_low).sum += low_val_0;
        *((*buffer_low).vals)
            .as_mut_ptr()
            .offset((*buffer_low).index as isize) = low_val_0;
        (*buffer_low).pushes += 1 as libc::c_int;
        (*buffer_low).index = (*buffer_low).index + 1 as libc::c_int;
        if (*buffer_low).index >= (*buffer_low).size {
            (*buffer_low).index = 0 as libc::c_int;
        }
        close_sum
            += *close.offset(i_0 as isize) - *close.offset((i_0 - period) as isize);
        let fresh3 = upper_band;
        upper_band = upper_band.offset(1);
        *fresh3 = (*buffer_high).sum * per;
        let fresh4 = lower_band;
        lower_band = lower_band.offset(1);
        *fresh4 = (*buffer_low).sum * per;
        let fresh5 = middle_point;
        middle_point = middle_point.offset(1);
        *fresh5 = close_sum * per;
        i_0 += 1;
        i_0;
    }
    ti_buffer_free(buffer_high);
    ti_buffer_free(buffer_low);
    if lower_band.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_abands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"lower_band - outputs[0] == size - ti_abands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/abands.c\0"
                as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_abands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3579: {
        if lower_band.offset_from(*outputs.offset(0 as libc::c_int as isize))
            as libc::c_long == (size - ti_abands_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"lower_band - outputs[0] == size - ti_abands_start(options)\0"
                    as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/abands.c\0"
                    as *const u8 as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"int ti_abands(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_abands_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let inv_period: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut lower_band: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut upper_band: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut middle_point: *mut libc::c_double = *outputs
        .offset(2 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_abands_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = period - 1 as libc::c_int;
    while i < size {
        let mut upper: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut mid: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut lower: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut j: libc::c_int = i - period + 1 as libc::c_int;
        while j <= i {
            upper
                += *high.offset(j as isize)
                    * (1.0f64
                        + 2.0f64
                            * ((*high.offset(j as isize) - *low.offset(j as isize))
                                / ((*high.offset(j as isize) + *low.offset(j as isize))
                                    / 2.0f64) * 1000.0f64 * 0.001f64)) * inv_period;
            mid += *close.offset(j as isize) * inv_period;
            lower
                += *low.offset(j as isize)
                    * (1.0f64
                        - 2.0f64
                            * ((*high.offset(j as isize) - *low.offset(j as isize))
                                / ((*high.offset(j as isize) + *low.offset(j as isize))
                                    / 2.0f64) * 1000.0f64 * 0.001f64)) * inv_period;
            j += 1;
            j;
        }
        let fresh6 = upper_band;
        upper_band = upper_band.offset(1);
        *fresh6 = upper;
        let fresh7 = middle_point;
        middle_point = middle_point.offset(1);
        *fresh7 = mid;
        let fresh8 = lower_band;
        lower_band = lower_band.offset(1);
        *fresh8 = lower;
        i += 1;
        i;
    }
    if lower_band.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_abands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"lower_band - outputs[0] == size - ti_abands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/abands.c\0"
                as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"int ti_abands_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4400: {
        if lower_band.offset_from(*outputs.offset(0 as libc::c_int as isize))
            as libc::c_long == (size - ti_abands_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"lower_band - outputs[0] == size - ti_abands_start(options)\0"
                    as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/abands.c\0"
                    as *const u8 as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"int ti_abands_ref(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
