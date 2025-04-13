use ::libc;
extern "C" {
    pub type ti_stream;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_atr_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_atr(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_max_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_max(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_min_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_min(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_ringbuffer_minmax {
    pub size: libc::c_int,
    pub end_idx: libc::c_int,
    pub min_idx: libc::c_int,
    pub max_idx: libc::c_int,
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub buffer: [[libc::c_double; 1]; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_ce {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub period: libc::c_int,
    pub coef: libc::c_double,
    pub atr: libc::c_double,
    pub prev_close: libc::c_double,
    pub LP_HP: ti_ringbuffer_minmax,
}
#[no_mangle]
pub unsafe extern "C" fn ti_ce_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ce(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let coef: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut ce_high: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut ce_low: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if size <= ti_ce_start(options) {
        return 0 as libc::c_int;
    }
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    let mut atr: libc::c_double = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let mut truerange: libc::c_double = 0.;
    let mut val: libc::c_double = 0.;
    let mut HP: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut HP_idx: libc::c_int = 0 as libc::c_int;
    let mut LP: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut LP_idx: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < period {
       // CALC_TRUERANGE();
        atr += truerange;
        val = *high.offset(i as isize);
        if HP <= val {
            HP = val;
            HP_idx = i;
        }
        val = *low.offset(i as isize);
        if LP >= val {
            LP = val;
            LP_idx = i;
        }
        i += 1;
        i;
    }
    atr /= period as libc::c_double;
    let smth: libc::c_double = (period as libc::c_double - 1.0f64)
        / period as libc::c_double;
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    let fresh0 = ce_high;
    ce_high = ce_high.offset(1);
    *fresh0 = HP - coef * atr;
    let fresh1 = ce_low;
    ce_low = ce_low.offset(1);
    *fresh1 = LP + coef * atr;
    i = period;
    while i < size {
       // CALC_TRUERANGE();
        atr = atr * smth + truerange * per;
        val = *high.offset(i as isize);
        if HP <= val {
            HP = val;
            HP_idx = i;
        } else if HP_idx == i - period {
            HP = *high.offset((i - period + 1 as libc::c_int) as isize);
            HP_idx = i - period + 1 as libc::c_int;
            let mut j: libc::c_int = 0;
            j = i - period + 2 as libc::c_int;
            while j <= i {
                val = *high.offset(j as isize);
                if HP <= val {
                    HP = val;
                    HP_idx = j;
                }
                j += 1;
                j;
            }
        }
        val = *low.offset(i as isize);
        if LP >= val {
            LP = val;
            LP_idx = i;
        } else if LP_idx == i - period {
            LP = *low.offset((i - period + 1 as libc::c_int) as isize);
            LP_idx = i - period + 1 as libc::c_int;
            let mut j_0: libc::c_int = 0;
            j_0 = i - period + 2 as libc::c_int;
            while j_0 <= i {
                val = *low.offset(j_0 as isize);
                if LP >= val {
                    LP = val;
                    LP_idx = j_0;
                }
                j_0 += 1;
                j_0;
            }
        }
        let fresh2 = ce_high;
        ce_high = ce_high.offset(1);
        *fresh2 = HP - coef * atr;
        let fresh3 = ce_low;
        ce_low = ce_low.offset(1);
        *fresh3 = LP + coef * atr;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ce_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let coef: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut ce_high: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut ce_low: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if size <= ti_ce_start(options) {
        return 0 as libc::c_int;
    }
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut atr: *mut libc::c_double = malloc(
        ((size - ti_atr_start(options)) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    ti_atr(size, inputs, options, &mut atr);
    let mut max: *mut libc::c_double = malloc(
        ((size - ti_max_start(options)) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    ti_max(size, &mut high, options, &mut max);
    let mut min: *mut libc::c_double = malloc(
        ((size - ti_min_start(options)) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    ti_min(size, &mut low, options, &mut min);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size - period + 1 as libc::c_int {
        let mut HP: libc::c_double = *max.offset(i as isize);
        let fresh4 = ce_high;
        ce_high = ce_high.offset(1);
        *fresh4 = HP - coef * *atr.offset(i as isize);
        let mut LP: libc::c_double = *min.offset(i as isize);
        let fresh5 = ce_low;
        ce_low = ce_low.offset(1);
        *fresh5 = LP + coef * *atr.offset(i as isize);
        i += 1;
        i;
    }
    free(atr as *mut libc::c_void);
    free(max as *mut libc::c_void);
    free(min as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ce_stream_new(
    mut options: *const libc::c_double,
    mut stream_in: *mut *mut ti_stream,
) -> libc::c_int {
    let mut stream: *mut *mut ti_stream_ce = stream_in as *mut *mut ti_stream_ce;
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let coef: libc::c_double = *options.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *stream = malloc(
        (::core::mem::size_of::<ti_stream_ce>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<[libc::c_double; 2]>() * period as usize)
                    as libc::c_ulong,
            )
            .wrapping_sub(
                ::core::mem::size_of::<[[libc::c_double; 1]; 1]>() as libc::c_ulong,
            ),
    ) as *mut ti_stream_ce;
    if (*stream).is_null() {
        return 2 as libc::c_int;
    }
    (**stream).progress = -ti_atr_start(options);
    (**stream).period = period;
    (**stream).coef = coef;
    (**stream).LP_HP.size = period;
    (**stream).LP_HP.end_idx = -(1 as libc::c_int);
    (**stream).LP_HP.min_idx = 0 as libc::c_int;
    (**stream).LP_HP.max_idx = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < period {
        (*((**stream).LP_HP.buffer)
            .as_mut_ptr()
            .offset(
                i as isize,
            ))[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        (*((**stream).LP_HP.buffer)
            .as_mut_ptr()
            .offset(
                i as isize,
            ))[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ce_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_ce_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_ce = stream_in as *mut ti_stream_ce;
    let mut progress: libc::c_int = (*stream).progress;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = (*stream).period;
    let coef: libc::c_double = (*stream).coef;
    let mut ce_high: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut ce_low: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut atr: libc::c_double = 0.;
    let mut prev_close: libc::c_double = 0.;
    let mut LP_HP: *mut ti_ringbuffer_minmax = &mut (*stream).LP_HP;
    let smth: libc::c_double = (period as libc::c_double - 1.0f64)
        / period as libc::c_double;
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut LP: libc::c_double = 0.;
    let mut HP: libc::c_double = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    if progress == -period + 1 as libc::c_int {
        atr = *high.offset(0 as libc::c_int as isize)
            - *low.offset(0 as libc::c_int as isize);
        let mut end_idx: libc::c_int = ((*LP_HP).end_idx + 1 as libc::c_int)
            % (*LP_HP).size;
        (*LP_HP).end_idx = end_idx;
        (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset(
                end_idx as isize,
            ))[1 as libc::c_int as usize] = *high.offset(0 as libc::c_int as isize);
        (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset(
                end_idx as isize,
            ))[0 as libc::c_int as usize] = *low.offset(0 as libc::c_int as isize);
        let mut val: libc::c_double = 0.;
        if (*LP_HP).max_idx == end_idx {
            (*LP_HP)
                .max = (*((*LP_HP).buffer)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
            (*LP_HP).max_idx = 0 as libc::c_int;
            let mut j: libc::c_int = 0;
            j = 1 as libc::c_int;
            while j < (*LP_HP).size {
                val = (*((*LP_HP).buffer)
                    .as_mut_ptr()
                    .offset(j as isize))[1 as libc::c_int as usize];
                if (*LP_HP).max <= val {
                    (*LP_HP).max = val;
                    (*LP_HP).max_idx = j;
                }
                j += 1;
                j;
            }
        } else if (*LP_HP).max <= *high.offset(0 as libc::c_int as isize) {
            (*LP_HP).max = *high.offset(0 as libc::c_int as isize);
            (*LP_HP).max_idx = end_idx;
        }
        if (*LP_HP).min_idx == end_idx {
            (*LP_HP)
                .min = (*((*LP_HP).buffer)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
            (*LP_HP).min_idx = 0 as libc::c_int;
            let mut j_0: libc::c_int = 0;
            j_0 = 1 as libc::c_int;
            while j_0 < (*LP_HP).size {
                val = (*((*LP_HP).buffer)
                    .as_mut_ptr()
                    .offset(j_0 as isize))[0 as libc::c_int as usize];
                if (*LP_HP).min >= val {
                    (*LP_HP).min = val;
                    (*LP_HP).min_idx = j_0;
                }
                j_0 += 1;
                j_0;
            }
        } else if (*LP_HP).min >= *low.offset(0 as libc::c_int as isize) {
            (*LP_HP).min = *low.offset(0 as libc::c_int as isize);
            (*LP_HP).min_idx = end_idx;
        }
        prev_close = *close.offset(0 as libc::c_int as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    } else {
        atr = (*stream).atr;
        prev_close = (*stream).prev_close;
    }
    while progress < 1 as libc::c_int && i < size {
        let mut truerange: libc::c_double = 0.;
        let l: libc::c_double = *low.offset(i as isize);
        let h: libc::c_double = *high.offset(i as isize);
        let c: libc::c_double = prev_close;
        let ych: libc::c_double = fabs(h - c);
        let ycl: libc::c_double = fabs(l - c);
        let mut v: libc::c_double = h - l;
        if ych > v {
            v = ych;
        }
        if ycl > v {
            v = ycl;
        }
        truerange = v;
        atr += truerange;
        let mut end_idx_0: libc::c_int = ((*LP_HP).end_idx + 1 as libc::c_int)
            % (*LP_HP).size;
        (*LP_HP).end_idx = end_idx_0;
        (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset(
                end_idx_0 as isize,
            ))[1 as libc::c_int as usize] = *high.offset(i as isize);
        (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset(
                end_idx_0 as isize,
            ))[0 as libc::c_int as usize] = *low.offset(i as isize);
        let mut val_0: libc::c_double = 0.;
        if (*LP_HP).max_idx == end_idx_0 {
            (*LP_HP)
                .max = (*((*LP_HP).buffer)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
            (*LP_HP).max_idx = 0 as libc::c_int;
            let mut j_1: libc::c_int = 0;
            j_1 = 1 as libc::c_int;
            while j_1 < (*LP_HP).size {
                val_0 = (*((*LP_HP).buffer)
                    .as_mut_ptr()
                    .offset(j_1 as isize))[1 as libc::c_int as usize];
                if (*LP_HP).max <= val_0 {
                    (*LP_HP).max = val_0;
                    (*LP_HP).max_idx = j_1;
                }
                j_1 += 1;
                j_1;
            }
        } else if (*LP_HP).max <= *high.offset(i as isize) {
            (*LP_HP).max = *high.offset(i as isize);
            (*LP_HP).max_idx = end_idx_0;
        }
        if (*LP_HP).min_idx == end_idx_0 {
            (*LP_HP)
                .min = (*((*LP_HP).buffer)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
            (*LP_HP).min_idx = 0 as libc::c_int;
            let mut j_2: libc::c_int = 0;
            j_2 = 1 as libc::c_int;
            while j_2 < (*LP_HP).size {
                val_0 = (*((*LP_HP).buffer)
                    .as_mut_ptr()
                    .offset(j_2 as isize))[0 as libc::c_int as usize];
                if (*LP_HP).min >= val_0 {
                    (*LP_HP).min = val_0;
                    (*LP_HP).min_idx = j_2;
                }
                j_2 += 1;
                j_2;
            }
        } else if (*LP_HP).min >= *low.offset(i as isize) {
            (*LP_HP).min = *low.offset(i as isize);
            (*LP_HP).min_idx = end_idx_0;
        }
        prev_close = *close.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        atr /= period as libc::c_double;
        LP = (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset((*LP_HP).min_idx as isize))[0 as libc::c_int as usize];
        HP = (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset((*LP_HP).max_idx as isize))[1 as libc::c_int as usize];
        let fresh6 = ce_high;
        ce_high = ce_high.offset(1);
        *fresh6 = HP - coef * atr;
        let fresh7 = ce_low;
        ce_low = ce_low.offset(1);
        *fresh7 = LP + coef * atr;
    }
    while i < size {
        let mut truerange_0: libc::c_double = 0.;
        let l_0: libc::c_double = *low.offset(i as isize);
        let h_0: libc::c_double = *high.offset(i as isize);
        let c_0: libc::c_double = prev_close;
        let ych_0: libc::c_double = fabs(h_0 - c_0);
        let ycl_0: libc::c_double = fabs(l_0 - c_0);
        let mut v_0: libc::c_double = h_0 - l_0;
        if ych_0 > v_0 {
            v_0 = ych_0;
        }
        if ycl_0 > v_0 {
            v_0 = ycl_0;
        }
        truerange_0 = v_0;
        atr = atr * smth + truerange_0 * per;
        let mut end_idx_1: libc::c_int = ((*LP_HP).end_idx + 1 as libc::c_int)
            % (*LP_HP).size;
        (*LP_HP).end_idx = end_idx_1;
        (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset(
                end_idx_1 as isize,
            ))[1 as libc::c_int as usize] = *high.offset(i as isize);
        (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset(
                end_idx_1 as isize,
            ))[0 as libc::c_int as usize] = *low.offset(i as isize);
        let mut val_1: libc::c_double = 0.;
        if (*LP_HP).max_idx == end_idx_1 {
            (*LP_HP)
                .max = (*((*LP_HP).buffer)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
            (*LP_HP).max_idx = 0 as libc::c_int;
            let mut j_3: libc::c_int = 0;
            j_3 = 1 as libc::c_int;
            while j_3 < (*LP_HP).size {
                val_1 = (*((*LP_HP).buffer)
                    .as_mut_ptr()
                    .offset(j_3 as isize))[1 as libc::c_int as usize];
                if (*LP_HP).max <= val_1 {
                    (*LP_HP).max = val_1;
                    (*LP_HP).max_idx = j_3;
                }
                j_3 += 1;
                j_3;
            }
        } else if (*LP_HP).max <= *high.offset(i as isize) {
            (*LP_HP).max = *high.offset(i as isize);
            (*LP_HP).max_idx = end_idx_1;
        }
        if (*LP_HP).min_idx == end_idx_1 {
            (*LP_HP)
                .min = (*((*LP_HP).buffer)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
            (*LP_HP).min_idx = 0 as libc::c_int;
            let mut j_4: libc::c_int = 0;
            j_4 = 1 as libc::c_int;
            while j_4 < (*LP_HP).size {
                val_1 = (*((*LP_HP).buffer)
                    .as_mut_ptr()
                    .offset(j_4 as isize))[0 as libc::c_int as usize];
                if (*LP_HP).min >= val_1 {
                    (*LP_HP).min = val_1;
                    (*LP_HP).min_idx = j_4;
                }
                j_4 += 1;
                j_4;
            }
        } else if (*LP_HP).min >= *low.offset(i as isize) {
            (*LP_HP).min = *low.offset(i as isize);
            (*LP_HP).min_idx = end_idx_1;
        }
        LP = (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset((*LP_HP).min_idx as isize))[0 as libc::c_int as usize];
        HP = (*((*LP_HP).buffer)
            .as_mut_ptr()
            .offset((*LP_HP).max_idx as isize))[1 as libc::c_int as usize];
        let fresh8 = ce_high;
        ce_high = ce_high.offset(1);
        *fresh8 = HP - coef * atr;
        let fresh9 = ce_low;
        ce_low = ce_low.offset(1);
        *fresh9 = LP + coef * atr;
        prev_close = *close.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).atr = atr;
    (*stream).prev_close = prev_close;
    return 0 as libc::c_int;
}
