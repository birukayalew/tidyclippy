use ::libc;
extern "C" {
    pub type ti_stream;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_ema(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_linregslope_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_linregslope(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_posc {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub options: C2RustUnnamed_2,
    pub state: C2RustUnnamed_1,
    pub constants: C2RustUnnamed_0,
    pub buf_info: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub padding: libc::c_double,
    pub zero: [libc::c_int; 4],
    pub offset_high: libc::c_int,
    pub size_high: libc::c_int,
    pub index_high: libc::c_int,
    pub padding_high: libc::c_int,
    pub offset_low: libc::c_int,
    pub size_low: libc::c_int,
    pub index_low: libc::c_int,
    pub padding_low: libc::c_int,
    pub offset_close: libc::c_int,
    pub size_close: libc::c_int,
    pub index_close: libc::c_int,
    pub padding_close: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub x_sum: libc::c_double,
    pub xsq_sum: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub y_sum: libc::c_double,
    pub xy_sum: libc::c_double,
    pub ema: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub period: libc::c_int,
    pub ema_period: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_posc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return period - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_posc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let ema_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut posc: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ema_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_posc_start(options) {
        return 0 as libc::c_int;
    }
    let mut y_sum: libc::c_double = 0.0f64;
    let mut xy_sum: libc::c_double = 0.0f64;
    let mut ema: libc::c_double = 0.;
    let x_sum: libc::c_double = (period * (period + 1 as libc::c_int)) as libc::c_double
        / 2.0f64;
    let xsq_sum: libc::c_double = (period * (period + 1 as libc::c_int)
        * (2 as libc::c_int * period + 1 as libc::c_int)) as libc::c_double / 6.0f64;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        xy_sum += *close.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y_sum += *close.offset(i as isize);
        i += 1;
        i;
    }
    i -= 1;
    i;
    let mut b: libc::c_double = (xy_sum / period as libc::c_double
        - x_sum / period as libc::c_double * y_sum / period as libc::c_double)
        / (xsq_sum / period as libc::c_double
            - x_sum / period as libc::c_double * (x_sum / period as libc::c_double));
    let mut the_max: libc::c_double = *high.offset(i as isize);
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < period {
        if the_max < *high.offset((i - j) as isize) + j as libc::c_double * b {
            the_max = *high.offset((i - j) as isize) + j as libc::c_double * b;
        }
        j += 1;
        j;
    }
    let mut the_min: libc::c_double = *low.offset(i as isize);
    let mut j_0: libc::c_int = 1 as libc::c_int;
    while j_0 < period {
        if the_min > *low.offset((i - j_0) as isize) + j_0 as libc::c_double * b {
            the_min = *low.offset((i - j_0) as isize) + j_0 as libc::c_double * b;
        }
        j_0 += 1;
        j_0;
    }
    ema = (*close.offset(i as isize) - the_min) / (the_max - the_min) * 100.0f64;
    let fresh0 = posc;
    posc = posc.offset(1);
    *fresh0 = ema;
    i += 1;
    i;
    while i < size {
        xy_sum += -y_sum + *close.offset(i as isize) * period as libc::c_double;
        y_sum += -*close.offset((i - period) as isize) + *close.offset(i as isize);
        let mut b_0: libc::c_double = (xy_sum / period as libc::c_double
            - x_sum / period as libc::c_double * y_sum / period as libc::c_double)
            / (xsq_sum / period as libc::c_double
                - x_sum / period as libc::c_double * (x_sum / period as libc::c_double));
        let mut the_max_0: libc::c_double = *high.offset(i as isize);
        let mut j_1: libc::c_int = 1 as libc::c_int;
        while j_1 < period {
            if the_max_0 < *high.offset((i - j_1) as isize) + j_1 as libc::c_double * b_0
            {
                the_max_0 = *high.offset((i - j_1) as isize)
                    + j_1 as libc::c_double * b_0;
            }
            j_1 += 1;
            j_1;
        }
        let mut the_min_0: libc::c_double = *low.offset(i as isize);
        let mut j_2: libc::c_int = 1 as libc::c_int;
        while j_2 < period {
            if the_min_0 > *low.offset((i - j_2) as isize) + j_2 as libc::c_double * b_0
            {
                the_min_0 = *low.offset((i - j_2) as isize)
                    + j_2 as libc::c_double * b_0;
            }
            j_2 += 1;
            j_2;
        }
        let mut osc: libc::c_double = (*close.offset(i as isize) - the_min_0)
            / (the_max_0 - the_min_0) * 100.0f64;
        ema = (osc - ema) * 2.0f64 / (1 as libc::c_int + ema_period) as libc::c_double
            + ema;
        let fresh1 = posc;
        posc = posc.offset(1);
        *fresh1 = ema;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_posc_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let ema_period: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut posc: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if ema_period < 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if size <= ti_posc_start(options) {
        return 0 as libc::c_int;
    }
    let mut start: libc::c_int = ti_linregslope_start(options);
    let mut b: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() * (size - start) as usize)
            as libc::c_ulong,
    ) as *mut libc::c_double;
    ti_linregslope(size, &mut close, &period, &mut b);
    let mut i: libc::c_int = start;
    while i < size {
        let mut the_max: libc::c_double = *high.offset(i as isize);
        let mut j: libc::c_int = 1 as libc::c_int;
        while (j as libc::c_double) < period {
            if the_max
                < *high.offset((i - j) as isize)
                    + j as libc::c_double * *b.offset((i - start) as isize)
            {
                the_max = *high.offset((i - j) as isize)
                    + j as libc::c_double * *b.offset((i - start) as isize);
            }
            j += 1;
            j;
        }
        let mut the_min: libc::c_double = *low.offset(i as isize);
        let mut j_0: libc::c_int = 1 as libc::c_int;
        while (j_0 as libc::c_double) < period {
            if the_min
                > *low.offset((i - j_0) as isize)
                    + j_0 as libc::c_double * *b.offset((i - start) as isize)
            {
                the_min = *low.offset((i - j_0) as isize)
                    + j_0 as libc::c_double * *b.offset((i - start) as isize);
            }
            j_0 += 1;
            j_0;
        }
        *posc
            .offset(
                (i - start) as isize,
            ) = (*close.offset(i as isize) - the_min) / (the_max - the_min) * 100.0f64;
        i += 1;
        i;
    }
    let mut ti_ema_inputs: [*const libc::c_double; 1] = [posc as *const libc::c_double];
    ti_ema(size - start, ti_ema_inputs.as_mut_ptr(), &ema_period, &mut posc);
    free(b as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_posc_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_posc_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_posc = stream_in as *mut ti_stream_posc;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut posc: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut progress: libc::c_int = (*stream).progress;
    let period: libc::c_int = (*stream).options.period;
    let ema_period: libc::c_int = (*stream).options.ema_period;
    let mut y_sum: libc::c_double = (*stream).state.y_sum;
    let mut xy_sum: libc::c_double = (*stream).state.xy_sum;
    let mut ema: libc::c_double = (*stream).state.ema;
    let xsq_sum: libc::c_double = (*stream).constants.xsq_sum;
    let x_sum: libc::c_double = (*stream).constants.x_sum;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut var1: libc::c_double = 0.;
    while i < size && progress <= 0 as libc::c_int {
        let mut idx: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_high {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx;
        let mut idx_0: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_low {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_0 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_0;
        let mut idx_1: libc::c_int = (*stream).buf_info.index_close + 1 as libc::c_int;
        if idx_1 == (*stream).buf_info.size_close {
            idx_1 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_1 as isize) = *close.offset(i as isize);
        (*stream).buf_info.index_close = idx_1;
        xy_sum
            += *close.offset(i as isize)
                * (progress - (-period + 1 as libc::c_int) + 1 as libc::c_int)
                    as libc::c_double;
        y_sum += *close.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        i -= 1;
        i;
        let mut b: libc::c_double = (xy_sum / period as libc::c_double
            - x_sum / period as libc::c_double * y_sum / period as libc::c_double)
            / (xsq_sum / period as libc::c_double
                - x_sum / period as libc::c_double * (x_sum / period as libc::c_double));
        let mut the_max: libc::c_double = *high.offset(i as isize);
        let mut j: libc::c_int = 1 as libc::c_int;
        while j < period {
            let mut idx_2: libc::c_int = (*stream).buf_info.index_high + -j;
            while idx_2 >= (*stream).buf_info.size_high {
                idx_2 -= (*stream).buf_info.size_high;
            }
            while idx_2 < 0 as libc::c_int {
                idx_2 += (*stream).buf_info.size_high;
            }
            var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_high as isize)
                .offset(idx_2 as isize);
            if the_max < var1 + j as libc::c_double * b {
                the_max = var1 + j as libc::c_double * b;
            }
            j += 1;
            j;
        }
        let mut the_min: libc::c_double = *low.offset(i as isize);
        let mut j_0: libc::c_int = 1 as libc::c_int;
        while j_0 < period {
            let mut idx_3: libc::c_int = (*stream).buf_info.index_low + -j_0;
            while idx_3 >= (*stream).buf_info.size_low {
                idx_3 -= (*stream).buf_info.size_low;
            }
            while idx_3 < 0 as libc::c_int {
                idx_3 += (*stream).buf_info.size_low;
            }
            var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_low as isize)
                .offset(idx_3 as isize);
            if the_min > var1 + j_0 as libc::c_double * b {
                the_min = var1 + j_0 as libc::c_double * b;
            }
            j_0 += 1;
            j_0;
        }
        ema = (*close.offset(i as isize) - the_min) / (the_max - the_min) * 100.0f64;
        let fresh2 = posc;
        posc = posc.offset(1);
        *fresh2 = ema;
        i += 1;
        i;
    }
    while i < size {
        let mut idx_4: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_4 == (*stream).buf_info.size_high {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_4 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_4;
        let mut idx_5: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_5 == (*stream).buf_info.size_low {
            idx_5 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_5 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_5;
        let mut idx_6: libc::c_int = (*stream).buf_info.index_close + 1 as libc::c_int;
        if idx_6 == (*stream).buf_info.size_close {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_6 as isize) = *close.offset(i as isize);
        (*stream).buf_info.index_close = idx_6;
        xy_sum += -y_sum + *close.offset(i as isize) * period as libc::c_double;
        let mut idx_7: libc::c_int = (*stream).buf_info.index_close + -period;
        while idx_7 >= (*stream).buf_info.size_close {
            idx_7 -= (*stream).buf_info.size_close;
        }
        while idx_7 < 0 as libc::c_int {
            idx_7 += (*stream).buf_info.size_close;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_7 as isize);
        y_sum += -var1 + *close.offset(i as isize);
        let mut b_0: libc::c_double = (xy_sum / period as libc::c_double
            - x_sum / period as libc::c_double * y_sum / period as libc::c_double)
            / (xsq_sum / period as libc::c_double
                - x_sum / period as libc::c_double * (x_sum / period as libc::c_double));
        let mut the_max_0: libc::c_double = *high.offset(i as isize);
        let mut j_1: libc::c_int = 1 as libc::c_int;
        while j_1 < period {
            let mut idx_8: libc::c_int = (*stream).buf_info.index_high + -j_1;
            while idx_8 >= (*stream).buf_info.size_high {
                idx_8 -= (*stream).buf_info.size_high;
            }
            while idx_8 < 0 as libc::c_int {
                idx_8 += (*stream).buf_info.size_high;
            }
            var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_high as isize)
                .offset(idx_8 as isize);
            if the_max_0 < var1 + j_1 as libc::c_double * b_0 {
                the_max_0 = var1 + j_1 as libc::c_double * b_0;
            }
            j_1 += 1;
            j_1;
        }
        let mut the_min_0: libc::c_double = *low.offset(i as isize);
        let mut j_2: libc::c_int = 1 as libc::c_int;
        while j_2 < period {
            let mut idx_9: libc::c_int = (*stream).buf_info.index_low + -j_2;
            while idx_9 >= (*stream).buf_info.size_low {
                idx_9 -= (*stream).buf_info.size_low;
            }
            while idx_9 < 0 as libc::c_int {
                idx_9 += (*stream).buf_info.size_low;
            }
            var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_low as isize)
                .offset(idx_9 as isize);
            if the_min_0 > var1 + j_2 as libc::c_double * b_0 {
                the_min_0 = var1 + j_2 as libc::c_double * b_0;
            }
            j_2 += 1;
            j_2;
        }
        let mut osc: libc::c_double = (*close.offset(i as isize) - the_min_0)
            / (the_max_0 - the_min_0) * 100.0f64;
        ema = (osc - ema) * 2.0f64 / (1 as libc::c_int + ema_period) as libc::c_double
            + ema;
        let fresh3 = posc;
        posc = posc.offset(1);
        *fresh3 = ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.y_sum = y_sum;
    (*stream).state.xy_sum = xy_sum;
    (*stream).state.ema = ema;
    return 0 as libc::c_int;
}
