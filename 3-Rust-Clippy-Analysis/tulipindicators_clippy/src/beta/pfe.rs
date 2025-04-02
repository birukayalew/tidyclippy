use ::libc;
extern "C" {
    pub type ti_stream;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_ema(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_buffer_new(size: libc::c_int) -> *mut ti_buffer;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_pfe {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub period: libc::c_int,
    pub ema_period: libc::c_int,
    pub ema: libc::c_double,
    pub numer: libc::c_double,
    pub last_removed: libc::c_double,
    pub buffers_idx: libc::c_int,
    pub buffer2_sum: libc::c_double,
    pub buffer: [[libc::c_double; 2]; 0],
}
#[no_mangle]
pub unsafe extern "C" fn ti_pfe_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return period;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pfe(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let ema_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut pfe: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_pfe_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2.0f64 / (ema_period as libc::c_double + 1.0f64);
    let mut denom: *mut ti_buffer = ti_buffer_new(period);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        if (*denom).pushes >= (*denom).size {
            (*denom).sum
                -= *((*denom).vals).as_mut_ptr().offset((*denom).index as isize);
        }
        (*denom).sum
            += sqrt(
                pow(
                    *real.offset(i as isize)
                        - *real.offset((i - 1 as libc::c_int) as isize),
                    2 as libc::c_int as libc::c_double,
                ) + 1.0f64,
            );
        *((*denom).vals)
            .as_mut_ptr()
            .offset(
                (*denom).index as isize,
            ) = sqrt(
            pow(
                *real.offset(i as isize) - *real.offset((i - 1 as libc::c_int) as isize),
                2 as libc::c_int as libc::c_double,
            ) + 1.0f64,
        );
        (*denom).pushes += 1 as libc::c_int;
        (*denom).index = (*denom).index + 1 as libc::c_int;
        if (*denom).index >= (*denom).size {
            (*denom).index = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if (*denom).pushes >= (*denom).size {
        (*denom).sum -= *((*denom).vals).as_mut_ptr().offset((*denom).index as isize);
    }
    (*denom).sum
        += sqrt(
            pow(
                *real.offset(i as isize) - *real.offset((i - 1 as libc::c_int) as isize),
                2 as libc::c_int as libc::c_double,
            ) + 1.0f64,
        );
    *((*denom).vals)
        .as_mut_ptr()
        .offset(
            (*denom).index as isize,
        ) = sqrt(
        pow(
            *real.offset(i as isize) - *real.offset((i - 1 as libc::c_int) as isize),
            2 as libc::c_int as libc::c_double,
        ) + 1.0f64,
    );
    (*denom).pushes += 1 as libc::c_int;
    (*denom).index = (*denom).index + 1 as libc::c_int;
    if (*denom).index >= (*denom).size {
        (*denom).index = 0 as libc::c_int;
    }
    let mut numer: libc::c_double = (if *real.offset(i as isize)
        - *real.offset((i - period) as isize) > 0 as libc::c_int as libc::c_double
    {
        1.0f64
    } else {
        -1.0f64
    }) * 100.0f64
        * sqrt(
            pow(
                *real.offset(i as isize) - *real.offset((i - period) as isize),
                2 as libc::c_int as libc::c_double,
            ) + 100.0f64,
        );
    let mut ema: libc::c_double = numer / (*denom).sum;
    let fresh0 = pfe;
    pfe = pfe.offset(1);
    *fresh0 = ema;
    i = period + 1 as libc::c_int;
    while i < size {
        if (*denom).pushes >= (*denom).size {
            (*denom).sum
                -= *((*denom).vals).as_mut_ptr().offset((*denom).index as isize);
        }
        (*denom).sum
            += sqrt(
                pow(
                    *real.offset(i as isize)
                        - *real.offset((i - 1 as libc::c_int) as isize),
                    2 as libc::c_int as libc::c_double,
                ) + 1.0f64,
            );
        *((*denom).vals)
            .as_mut_ptr()
            .offset(
                (*denom).index as isize,
            ) = sqrt(
            pow(
                *real.offset(i as isize) - *real.offset((i - 1 as libc::c_int) as isize),
                2 as libc::c_int as libc::c_double,
            ) + 1.0f64,
        );
        (*denom).pushes += 1 as libc::c_int;
        (*denom).index = (*denom).index + 1 as libc::c_int;
        if (*denom).index >= (*denom).size {
            (*denom).index = 0 as libc::c_int;
        }
        let mut numer2: libc::c_double = (if *real.offset(i as isize)
            - *real.offset((i - period) as isize) > 0 as libc::c_int as libc::c_double
        {
            1.0f64
        } else {
            -1.0f64
        }) * 100.0f64
            * sqrt(
                pow(
                    *real.offset(i as isize) - *real.offset((i - period) as isize),
                    2 as libc::c_int as libc::c_double,
                ) + 100.0f64,
            );
        ema = (numer2 / (*denom).sum - ema) * per + ema;
        let fresh1 = pfe;
        pfe = pfe.offset(1);
        *fresh1 = ema;
        i += 1;
        i;
    }
    free(denom as *mut libc::c_void);
    if pfe.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_pfe_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"pfe - outputs[0] == size - ti_pfe_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/pfe.c\0"
                as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_pfe(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3573: {
        if pfe.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_pfe_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"pfe - outputs[0] == size - ti_pfe_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/pfe.c\0"
                    as *const u8 as *const libc::c_char,
                69 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int ti_pfe(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pfe_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let ema_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut pfe: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_pfe_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = period;
    while i < size {
        let mut div: libc::c_double = 0.0f64;
        let mut j: libc::c_int = i - period + 1 as libc::c_int;
        while j <= i {
            div
                += sqrt(
                    pow(
                        *real.offset(j as isize)
                            - *real.offset((j - 1 as libc::c_int) as isize),
                        2 as libc::c_int as libc::c_double,
                    ) + 1 as libc::c_int as libc::c_double,
                );
            j += 1;
            j;
        }
        let fresh2 = pfe;
        pfe = pfe.offset(1);
        *fresh2 = (if *real.offset(i as isize) - *real.offset((i - period) as isize)
            > 0 as libc::c_int as libc::c_double
        {
            1.0f64
        } else {
            -1.0f64
        }) * 100.0f64
            * sqrt(
                pow(
                    *real.offset(i as isize) - *real.offset((i - period) as isize),
                    2 as libc::c_int as libc::c_double,
                ) + 100.0f64,
            ) / div;
        i += 1;
        i;
    }
    let mut ti_ema_inputs: [*const libc::c_double; 1] = [
        *outputs.offset(0 as libc::c_int as isize) as *const libc::c_double,
    ];
    let ti_ema_options: [libc::c_double; 1] = [ema_period as libc::c_double];
    ti_ema(size - period, ti_ema_inputs.as_mut_ptr(), ti_ema_options.as_ptr(), outputs);
    if pfe.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_pfe_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"pfe - outputs[0] == size - ti_pfe_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/pfe.c\0"
                as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_pfe_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4354: {
        if pfe.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_pfe_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"pfe - outputs[0] == size - ti_pfe_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/pfe.c\0"
                    as *const u8 as *const libc::c_char,
                100 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"int ti_pfe_ref(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pfe_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_pfe_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_pfe = stream_in as *mut ti_stream_pfe;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = (*stream).period;
    let ema_period: libc::c_double = (*stream).ema_period as libc::c_double;
    let mut pfe: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut ema: libc::c_double = (*stream).ema;
    let mut numer: libc::c_double = (*stream).numer;
    let mut last_removed: libc::c_double = (*stream).last_removed;
    let mut buffers_idx: libc::c_int = (*stream).buffers_idx;
    let mut buffer2_sum: libc::c_double = (*stream).buffer2_sum;
    let mut buffer: *mut [[libc::c_double; 2]; 0] = &mut (*stream).buffer;
    let per: libc::c_double = 2.0f64 / (ema_period + 1.0f64);
    let mut progress: libc::c_int = (*stream).progress;
    let mut i: libc::c_int = 0 as libc::c_int;
    if progress == -period {
        buffers_idx += 1;
        buffers_idx;
        (*(*buffer)
            .as_mut_ptr()
            .offset(
                buffers_idx as isize,
            ))[0 as libc::c_int as usize] = *real.offset(i as isize);
        (*(*buffer)
            .as_mut_ptr()
            .offset(
                buffers_idx as isize,
            ))[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        buffer2_sum += 0 as libc::c_int as libc::c_double;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while progress < 0 as libc::c_int && i < size {
        let mut prev: libc::c_double = 0.;
        let mut idx: libc::c_int = buffers_idx - 0 as libc::c_int;
        if idx < 0 as libc::c_int {
            idx += period;
        } else if idx >= period {
            idx -= period;
        }
        prev = (*(*buffer).as_mut_ptr().offset(idx as isize))[0 as libc::c_int as usize];
        buffers_idx += 1;
        buffers_idx;
        (*(*buffer)
            .as_mut_ptr()
            .offset(
                buffers_idx as isize,
            ))[0 as libc::c_int as usize] = *real.offset(i as isize);
        (*(*buffer)
            .as_mut_ptr()
            .offset(
                buffers_idx as isize,
            ))[1 as libc::c_int
            as usize] = sqrt(
            pow(*real.offset(i as isize) - prev, 2 as libc::c_int as libc::c_double)
                + 1.0f64,
        );
        buffer2_sum
            += sqrt(
                pow(*real.offset(i as isize) - prev, 2 as libc::c_int as libc::c_double)
                    + 1.0f64,
            );
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if progress == 0 as libc::c_int && i < size {
        let mut prev_0: libc::c_double = 0.;
        let mut idx_0: libc::c_int = buffers_idx - 0 as libc::c_int;
        if idx_0 < 0 as libc::c_int {
            idx_0 += period;
        } else if idx_0 >= period {
            idx_0 -= period;
        }
        prev_0 = (*(*buffer)
            .as_mut_ptr()
            .offset(idx_0 as isize))[0 as libc::c_int as usize];
        buffers_idx += 1;
        if buffers_idx == period {
            buffers_idx = 0 as libc::c_int;
        }
        last_removed = (*(*buffer)
            .as_mut_ptr()
            .offset(buffers_idx as isize))[0 as libc::c_int as usize];
        (*(*buffer)
            .as_mut_ptr()
            .offset(
                buffers_idx as isize,
            ))[0 as libc::c_int as usize] = *real.offset(i as isize);
        buffer2_sum
            -= (*(*buffer)
                .as_mut_ptr()
                .offset(buffers_idx as isize))[1 as libc::c_int as usize];
        let ref mut fresh3 = (*(*buffer)
            .as_mut_ptr()
            .offset(buffers_idx as isize))[1 as libc::c_int as usize];
        *fresh3 = sqrt(
            pow(*real.offset(i as isize) - prev_0, 2 as libc::c_int as libc::c_double)
                + 1.0f64,
        );
        buffer2_sum += *fresh3;
        numer = (if *real.offset(i as isize) - last_removed
            > 0 as libc::c_int as libc::c_double
        {
            1.0f64
        } else {
            -1.0f64
        }) * 100.0f64
            * sqrt(
                pow(
                    *real.offset(i as isize) - last_removed,
                    2 as libc::c_int as libc::c_double,
                ) + 100.0f64,
            );
        ema = numer / buffer2_sum;
        let fresh4 = pfe;
        pfe = pfe.offset(1);
        *fresh4 = ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        let mut prev_1: libc::c_double = 0.;
        let mut idx_1: libc::c_int = buffers_idx - 0 as libc::c_int;
        if idx_1 < 0 as libc::c_int {
            idx_1 += period;
        } else if idx_1 >= period {
            idx_1 -= period;
        }
        prev_1 = (*(*buffer)
            .as_mut_ptr()
            .offset(idx_1 as isize))[0 as libc::c_int as usize];
        buffers_idx += 1;
        if buffers_idx == period {
            buffers_idx = 0 as libc::c_int;
        }
        last_removed = (*(*buffer)
            .as_mut_ptr()
            .offset(buffers_idx as isize))[0 as libc::c_int as usize];
        (*(*buffer)
            .as_mut_ptr()
            .offset(
                buffers_idx as isize,
            ))[0 as libc::c_int as usize] = *real.offset(i as isize);
        buffer2_sum
            -= (*(*buffer)
                .as_mut_ptr()
                .offset(buffers_idx as isize))[1 as libc::c_int as usize];
        let ref mut fresh5 = (*(*buffer)
            .as_mut_ptr()
            .offset(buffers_idx as isize))[1 as libc::c_int as usize];
        *fresh5 = sqrt(
            pow(*real.offset(i as isize) - prev_1, 2 as libc::c_int as libc::c_double)
                + 1.0f64,
        );
        buffer2_sum += *fresh5;
        numer = (if *real.offset(i as isize) - last_removed
            > 0 as libc::c_int as libc::c_double
        {
            1.0f64
        } else {
            -1.0f64
        }) * 100.0f64
            * sqrt(
                pow(
                    *real.offset(i as isize) - last_removed,
                    2 as libc::c_int as libc::c_double,
                ) + 100.0f64,
            );
        ema = (numer / buffer2_sum - ema) * per + ema;
        let fresh6 = pfe;
        pfe = pfe.offset(1);
        *fresh6 = ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).ema = ema;
    (*stream).numer = numer;
    (*stream).last_removed = last_removed;
    (*stream).buffers_idx = buffers_idx;
    (*stream).buffer2_sum = buffer2_sum;
    return 0 as libc::c_int;
}
