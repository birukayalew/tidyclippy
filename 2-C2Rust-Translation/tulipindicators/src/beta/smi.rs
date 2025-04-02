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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_smi {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub options: C2RustUnnamed_1,
    pub state: C2RustUnnamed_0,
    pub buf_info: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub padding: libc::c_double,
    pub zero: [libc::c_int; 4],
    pub offset_low: libc::c_int,
    pub size_low: libc::c_int,
    pub index_low: libc::c_int,
    pub padding_low: libc::c_int,
    pub offset_high: libc::c_int,
    pub size_high: libc::c_int,
    pub index_high: libc::c_int,
    pub padding_high: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ema_r_num: libc::c_double,
    pub ema_s_num: libc::c_double,
    pub ema_r_den: libc::c_double,
    pub ema_s_den: libc::c_double,
    pub ll: libc::c_double,
    pub hh: libc::c_double,
    pub ll_idx: libc::c_int,
    pub hh_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub q_period: libc::c_double,
    pub r_period: libc::c_double,
    pub s_period: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn ti_smi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let q_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    return q_period - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_smi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let q_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let r_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let s_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    let mut smi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut progress: libc::c_int = -q_period + 1 as libc::c_int;
    let mut ema_r_num: libc::c_double = ::core::f32::NAN as libc::c_double;
    let mut ema_s_num: libc::c_double = ::core::f32::NAN as libc::c_double;
    let mut ema_r_den: libc::c_double = ::core::f32::NAN as libc::c_double;
    let mut ema_s_den: libc::c_double = ::core::f32::NAN as libc::c_double;
    let mut ll: libc::c_double = 0.;
    let mut hh: libc::c_double = 0.;
    let mut hh_idx: libc::c_int = 0;
    let mut ll_idx: libc::c_int = 0;
    let mut var1: libc::c_double = 0.;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size && progress == -q_period + 1 as libc::c_int {
        hh = *high.offset(i_0 as isize);
        hh_idx = progress;
        ll = *low.offset(i_0 as isize);
        ll_idx = progress;
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    while i_0 < size && progress < 0 as libc::c_int {
        if hh <= *high.offset(i_0 as isize) {
            hh = *high.offset(i_0 as isize);
            hh_idx = progress;
        }
        if ll >= *low.offset(i_0 as isize) {
            ll = *low.offset(i_0 as isize);
            ll_idx = progress;
        }
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    while i_0 < size && progress == 0 as libc::c_int {
        if hh <= *high.offset(i_0 as isize) {
            hh = *high.offset(i_0 as isize);
            hh_idx = progress;
        }
        if ll >= *low.offset(i_0 as isize) {
            ll = *low.offset(i_0 as isize);
            ll_idx = progress;
        }
        ema_s_num = *close.offset(i_0 as isize) - 0.5f64 * (hh + ll);
        ema_r_num = ema_s_num;
        ema_s_den = hh - ll;
        ema_r_den = ema_s_den;
        let fresh0 = smi;
        smi = smi.offset(1);
        *fresh0 = 100 as libc::c_int as libc::c_double * ema_s_num
            / (0.5f64 * ema_s_den);
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    while i_0 < size {
        if hh_idx == progress - q_period {
            hh = *high.offset(i_0 as isize);
            hh_idx = progress;
            let mut j: libc::c_int = 1 as libc::c_int;
            while j < q_period {
                var1 = *high.offset((i_0 - j) as isize);
                if var1 > hh {
                    hh = var1;
                    hh_idx = progress - j;
                }
                j += 1;
                j;
            }
        } else if hh <= *high.offset(i_0 as isize) {
            hh = *high.offset(i_0 as isize);
            hh_idx = progress;
        }
        if ll_idx == progress - q_period {
            ll = *low.offset(i_0 as isize);
            ll_idx = progress;
            let mut j_0: libc::c_int = 1 as libc::c_int;
            while j_0 < q_period {
                var1 = *low.offset((i_0 - j_0) as isize);
                if var1 < ll {
                    ll = var1;
                    ll_idx = progress - j_0;
                }
                j_0 += 1;
                j_0;
            }
        } else if ll >= *low.offset(i_0 as isize) {
            ll = *low.offset(i_0 as isize);
            ll_idx = progress;
        }
        ema_r_num = (*close.offset(i_0 as isize) - 0.5f64 * (hh + ll) - ema_r_num)
            * (2.0f64 / (1.0f64 + r_period as libc::c_double)) + ema_r_num;
        ema_s_num = (ema_r_num - ema_s_num)
            * (2.0f64 / (1.0f64 + s_period as libc::c_double)) + ema_s_num;
        ema_r_den = (hh - ll - ema_r_den)
            * (2.0f64 / (1.0f64 + r_period as libc::c_double)) + ema_r_den;
        ema_s_den = (ema_r_den - ema_s_den)
            * (2.0f64 / (1.0f64 + s_period as libc::c_double)) + ema_s_den;
        let fresh1 = smi;
        smi = smi.offset(1);
        *fresh1 = 100.0f64 * ema_s_num / (0.5f64 * ema_s_den);
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_smi_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut q_period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let mut r_period: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut s_period: libc::c_double = *options.offset(2 as libc::c_int as isize);
    let mut smi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let outsize: libc::c_int = size - ti_max_start(options);
    let mut max: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut min: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut ti_max_inputs: [*const libc::c_double; 1] = [high];
    ti_max(size, ti_max_inputs.as_mut_ptr(), &mut q_period, &mut max);
    let mut ti_min_inputs: [*const libc::c_double; 1] = [low];
    ti_min(size, ti_min_inputs.as_mut_ptr(), &mut q_period, &mut min);
    let mut num: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut den: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i_0: libc::c_int = 0;
    i_0 = 0 as libc::c_int;
    while i_0 < outsize {
        *num
            .offset(
                i_0 as isize,
            ) = *close.offset((size - outsize + i_0) as isize)
            - 0.5f64 * (*max.offset(i_0 as isize) + *min.offset(i_0 as isize));
        *den
            .offset(
                i_0 as isize,
            ) = *max.offset(i_0 as isize) - *min.offset(i_0 as isize);
        i_0 += 1;
        i_0;
    }
    let mut num_input: [*const libc::c_double; 1] = [num as *const libc::c_double];
    ti_ema(outsize, num_input.as_mut_ptr(), &mut r_period, &mut num);
    ti_ema(outsize, num_input.as_mut_ptr(), &mut s_period, &mut num);
    let mut den_input: [*const libc::c_double; 1] = [den as *const libc::c_double];
    ti_ema(outsize, den_input.as_mut_ptr(), &mut r_period, &mut den);
    ti_ema(outsize, den_input.as_mut_ptr(), &mut s_period, &mut den);
    i_0 = 0 as libc::c_int;
    while i_0 < outsize {
        *smi
            .offset(
                i_0 as isize,
            ) = 100.0f64 * *num.offset(i_0 as isize)
            / (0.5f64 * *den.offset(i_0 as isize));
        i_0 += 1;
        i_0;
    }
    free(max as *mut libc::c_void);
    free(min as *mut libc::c_void);
    free(num as *mut libc::c_void);
    free(den as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_smi_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_smi_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_smi = stream_in as *mut ti_stream_smi;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut smi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let q_period: libc::c_double = (*stream).options.q_period;
    let r_period: libc::c_double = (*stream).options.r_period;
    let s_period: libc::c_double = (*stream).options.s_period;
    let mut progress: libc::c_int = (*stream).progress;
    let mut ema_r_num: libc::c_double = (*stream).state.ema_r_num;
    let mut ema_s_num: libc::c_double = (*stream).state.ema_s_num;
    let mut ema_r_den: libc::c_double = (*stream).state.ema_r_den;
    let mut ema_s_den: libc::c_double = (*stream).state.ema_s_den;
    let mut ll: libc::c_double = (*stream).state.ll;
    let mut hh: libc::c_double = (*stream).state.hh;
    let mut hh_idx: libc::c_int = (*stream).state.hh_idx;
    let mut ll_idx: libc::c_int = (*stream).state.ll_idx;
    let mut var1: libc::c_double = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size
        && progress as libc::c_double == -q_period + 1 as libc::c_int as libc::c_double
    {
        let mut idx: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_low {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx;
        let mut idx_0: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_high {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_0 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_0;
        hh = *high.offset(i as isize);
        hh_idx = progress;
        ll = *low.offset(i as isize);
        ll_idx = progress;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size && progress < 0 as libc::c_int {
        let mut idx_1: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_1 == (*stream).buf_info.size_low {
            idx_1 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_1 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_1;
        let mut idx_2: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_2 == (*stream).buf_info.size_high {
            idx_2 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_2 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_2;
        if hh <= *high.offset(i as isize) {
            hh = *high.offset(i as isize);
            hh_idx = progress;
        }
        if ll >= *low.offset(i as isize) {
            ll = *low.offset(i as isize);
            ll_idx = progress;
        }
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size && progress == 0 as libc::c_int {
        let mut idx_3: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_3 == (*stream).buf_info.size_low {
            idx_3 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_3 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_3;
        let mut idx_4: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_4 == (*stream).buf_info.size_high {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_4 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_4;
        if hh <= *high.offset(i as isize) {
            hh = *high.offset(i as isize);
            hh_idx = progress;
        }
        if ll >= *low.offset(i as isize) {
            ll = *low.offset(i as isize);
            ll_idx = progress;
        }
        ema_s_num = *close.offset(i as isize) - 0.5f64 * (hh + ll);
        ema_r_num = ema_s_num;
        ema_s_den = hh - ll;
        ema_r_den = ema_s_den;
        let fresh2 = smi;
        smi = smi.offset(1);
        *fresh2 = 100 as libc::c_int as libc::c_double * ema_s_num
            / (0.5f64 * ema_s_den);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        let mut idx_5: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_5 == (*stream).buf_info.size_low {
            idx_5 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_5 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_5;
        let mut idx_6: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_6 == (*stream).buf_info.size_high {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_6 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_6;
        if hh_idx as libc::c_double == progress as libc::c_double - q_period {
            hh = *high.offset(i as isize);
            hh_idx = progress;
            let mut j: libc::c_int = 1 as libc::c_int;
            while (j as libc::c_double) < q_period {
                let mut idx_7: libc::c_int = (*stream).buf_info.index_high + -j;
                while idx_7 >= (*stream).buf_info.size_high {
                    idx_7 -= (*stream).buf_info.size_high;
                }
                while idx_7 < 0 as libc::c_int {
                    idx_7 += (*stream).buf_info.size_high;
                }
                var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                    .offset((*stream).buf_info.offset_high as isize)
                    .offset(idx_7 as isize);
                if var1 > hh {
                    hh = var1;
                    hh_idx = progress - j;
                }
                j += 1;
                j;
            }
        } else if hh <= *high.offset(i as isize) {
            hh = *high.offset(i as isize);
            hh_idx = progress;
        }
        if ll_idx as libc::c_double == progress as libc::c_double - q_period {
            ll = *low.offset(i as isize);
            ll_idx = progress;
            let mut j_0: libc::c_int = 1 as libc::c_int;
            while (j_0 as libc::c_double) < q_period {
                let mut idx_8: libc::c_int = (*stream).buf_info.index_low + -j_0;
                while idx_8 >= (*stream).buf_info.size_low {
                    idx_8 -= (*stream).buf_info.size_low;
                }
                while idx_8 < 0 as libc::c_int {
                    idx_8 += (*stream).buf_info.size_low;
                }
                var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                    .offset((*stream).buf_info.offset_low as isize)
                    .offset(idx_8 as isize);
                if var1 < ll {
                    ll = var1;
                    ll_idx = progress - j_0;
                }
                j_0 += 1;
                j_0;
            }
        } else if ll >= *low.offset(i as isize) {
            ll = *low.offset(i as isize);
            ll_idx = progress;
        }
        ema_r_num = (*close.offset(i as isize) - 0.5f64 * (hh + ll) - ema_r_num)
            * (2.0f64 / (1.0f64 + r_period)) + ema_r_num;
        ema_s_num = (ema_r_num - ema_s_num) * (2.0f64 / (1.0f64 + s_period)) + ema_s_num;
        ema_r_den = (hh - ll - ema_r_den) * (2.0f64 / (1.0f64 + r_period)) + ema_r_den;
        ema_s_den = (ema_r_den - ema_s_den) * (2.0f64 / (1.0f64 + s_period)) + ema_s_den;
        let fresh3 = smi;
        smi = smi.offset(1);
        *fresh3 = 100.0f64 * ema_s_num / (0.5f64 * ema_s_den);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.ema_r_num = ema_r_num;
    (*stream).state.ema_s_num = ema_s_num;
    (*stream).state.ema_r_den = ema_r_den;
    (*stream).state.ema_s_den = ema_s_den;
    (*stream).state.ll = ll;
    (*stream).state.hh = hh;
    (*stream).state.hh_idx = hh_idx;
    (*stream).state.ll_idx = ll_idx;
    return 0 as libc::c_int;
}
