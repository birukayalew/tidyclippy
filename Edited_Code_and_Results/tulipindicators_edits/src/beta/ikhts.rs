#![feature(label_break_value)]

use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_max_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_max(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_min(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ikhts_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ikhts(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_ikhts_start(options) {
        return 0 as libc::c_int;
    }
    let out_size: libc::c_int = size - ti_max_start(options);
    let mut buff: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(out_size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut max: *mut libc::c_double = &mut *buff.offset(0 as libc::c_int as isize)
        as *mut libc::c_double;
    let mut min: *mut libc::c_double = &mut *buff.offset(out_size as isize)
        as *mut libc::c_double;
    let mut tmp_outs: [*mut libc::c_double; 1] = [0 as *mut libc::c_double; 1];
    tmp_outs[0 as libc::c_int as usize] = max;
    let mut ret: libc::c_int = ti_max(
        size,
        &*inputs.offset(0 as libc::c_int as isize),
        options,
        tmp_outs.as_mut_ptr(),
    );
    if ret == 0 as libc::c_int {} else {
        __assert_fail(
            b"ret == TI_OKAY\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/ikhts.c\0"
                as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_ikhts(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3783: {
        if ret == 0 as libc::c_int {} else {
            __assert_fail(
                b"ret == TI_OKAY\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/ikhts.c\0"
                    as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"int ti_ikhts(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    tmp_outs[0 as libc::c_int as usize] = min;
    ret = ti_min(
        size,
        &*inputs.offset(1 as libc::c_int as isize),
        options,
        tmp_outs.as_mut_ptr(),
    );
    if ret == 0 as libc::c_int {} else {
        __assert_fail(
            b"ret == TI_OKAY\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/ikhts.c\0"
                as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_ikhts(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3702: {
        if ret == 0 as libc::c_int {} else {
            __assert_fail(
                b"ret == TI_OKAY\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/ikhts.c\0"
                    as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"int ti_ikhts(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < out_size {
        let fresh0 = max;
        max = max.offset(1);
        let fresh1 = min;
        min = min.offset(1);
        let fresh2 = output;
        output = output.offset(1);
        *fresh2 = (*fresh0 + *fresh1) / 2.0f64;
        i += 1;
        i;
    }
    free(buff as *mut libc::c_void);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ikhts_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_ikhts_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/ikhts.c\0"
                as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_ikhts(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3554: {
        if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_ikhts_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"output - outputs[0] == size - ti_ikhts_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/ikhts.c\0"
                    as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"int ti_ikhts(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
