use ::libc;
extern "C" {
    pub type ti_stream;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_add(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_ema(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_rmi {
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
    pub offset_price: libc::c_int,
    pub size_price: libc::c_int,
    pub index_price: libc::c_int,
    pub padding_price: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub gains_ema: libc::c_double,
    pub losses_ema: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub period: libc::c_int,
    pub lookback_period: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let lookback_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    return lookback_period;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let lookback_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut rmi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if lookback_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rmi_start(options) {
        return 0 as libc::c_int;
    }
    let mut gains_ema: libc::c_double = 0.;
    let mut losses_ema: libc::c_double = 0.;
    let mut i: libc::c_int = lookback_period;
    gains_ema = if 0 as libc::c_int as libc::c_double
        > *real.offset(i as isize) - *real.offset((i - lookback_period) as isize)
    {
        0 as libc::c_int as libc::c_double
    } else {
        *real.offset(i as isize) - *real.offset((i - lookback_period) as isize)
    };
    losses_ema = if 0 as libc::c_int as libc::c_double
        > *real.offset((i - lookback_period) as isize) - *real.offset(i as isize)
    {
        0 as libc::c_int as libc::c_double
    } else {
        *real.offset((i - lookback_period) as isize) - *real.offset(i as isize)
    };
    i += 1;
    i;
    let fresh0 = rmi;
    rmi = rmi.offset(1);
    *fresh0 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
    while i < size {
        gains_ema = ((if 0 as libc::c_int as libc::c_double
            > *real.offset(i as isize) - *real.offset((i - lookback_period) as isize)
        {
            0 as libc::c_int as libc::c_double
        } else {
            *real.offset(i as isize) - *real.offset((i - lookback_period) as isize)
        }) - gains_ema) * 2.0f64 / (1 as libc::c_int + period) as libc::c_double
            + gains_ema;
        losses_ema = ((if 0 as libc::c_int as libc::c_double
            > *real.offset((i - lookback_period) as isize) - *real.offset(i as isize)
        {
            0 as libc::c_int as libc::c_double
        } else {
            *real.offset((i - lookback_period) as isize) - *real.offset(i as isize)
        }) - losses_ema) * 2.0f64 / (1 as libc::c_int + period) as libc::c_double
            + losses_ema;
        let fresh1 = rmi;
        rmi = rmi.offset(1);
        *fresh1 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmi_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let lookback_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut rmi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if lookback_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rmi_start(options) {
        return 0 as libc::c_int;
    }
    let mut start: libc::c_int = ti_rmi_start(options);
    let mut gains: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() * (size - start) as usize)
            as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut losses: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() * (size - start) as usize)
            as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut i: libc::c_int = lookback_period;
    while i < size {
        *gains
            .offset(
                (i - start) as isize,
            ) = if 0 as libc::c_int as libc::c_double
            > *real.offset(i as isize) - *real.offset((i - lookback_period) as isize)
        {
            0 as libc::c_int as libc::c_double
        } else {
            *real.offset(i as isize) - *real.offset((i - lookback_period) as isize)
        };
        *losses
            .offset(
                (i - start) as isize,
            ) = if 0 as libc::c_int as libc::c_double
            > *real.offset((i - lookback_period) as isize) - *real.offset(i as isize)
        {
            0 as libc::c_int as libc::c_double
        } else {
            *real.offset((i - lookback_period) as isize) - *real.offset(i as isize)
        };
        i += 1;
        i;
    }
    let mut ti_ema_gains: [*const libc::c_double; 1] = [gains as *const libc::c_double];
    let mut ti_ema_losses: [*const libc::c_double; 1] = [
        losses as *const libc::c_double,
    ];
    ti_ema(size - start, ti_ema_gains.as_mut_ptr(), options, &mut gains);
    ti_ema(size - start, ti_ema_losses.as_mut_ptr(), options, &mut losses);
    let mut ti_add_inputs: [*const libc::c_double; 2] = [
        gains as *const libc::c_double,
        losses as *const libc::c_double,
    ];
    ti_add(
        size - start,
        ti_add_inputs.as_mut_ptr(),
        0 as *const libc::c_double,
        &mut losses,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size - start {
        let fresh2 = rmi;
        rmi = rmi.offset(1);
        *fresh2 = *gains.offset(i_0 as isize) / *losses.offset(i_0 as isize) * 100.0f64;
        i_0 += 1;
        i_0;
    }
    free(gains as *mut libc::c_void);
    free(losses as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmi_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_rmi_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_rmi = stream_in as *mut ti_stream_rmi;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut rmi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut progress: libc::c_int = (*stream).progress;
    let period: libc::c_int = (*stream).options.period;
    let lookback_period: libc::c_int = (*stream).options.lookback_period;
    let mut gains_ema: libc::c_double = (*stream).state.gains_ema;
    let mut losses_ema: libc::c_double = (*stream).state.losses_ema;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut var1: libc::c_double = 0.;
    while i < size && progress < 0 as libc::c_int {
        let mut idx: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_price {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i < size && progress == 0 as libc::c_int {
        let mut idx_0: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_price {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_0 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_0;
        let mut idx_1: libc::c_int = (*stream).buf_info.index_price + -lookback_period;
        while idx_1 >= (*stream).buf_info.size_price {
            idx_1 -= (*stream).buf_info.size_price;
        }
        while idx_1 < 0 as libc::c_int {
            idx_1 += (*stream).buf_info.size_price;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_1 as isize);
        gains_ema = if 0 as libc::c_int as libc::c_double
            > *real.offset(i as isize) - var1
        {
            0 as libc::c_int as libc::c_double
        } else {
            *real.offset(i as isize) - var1
        };
        losses_ema = if 0 as libc::c_int as libc::c_double
            > var1 - *real.offset(i as isize)
        {
            0 as libc::c_int as libc::c_double
        } else {
            var1 - *real.offset(i as isize)
        };
        let fresh3 = rmi;
        rmi = rmi.offset(1);
        *fresh3 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        let mut idx_2: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_2 == (*stream).buf_info.size_price {
            idx_2 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_2 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_2;
        let mut idx_3: libc::c_int = (*stream).buf_info.index_price + -lookback_period;
        while idx_3 >= (*stream).buf_info.size_price {
            idx_3 -= (*stream).buf_info.size_price;
        }
        while idx_3 < 0 as libc::c_int {
            idx_3 += (*stream).buf_info.size_price;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_3 as isize);
        gains_ema = ((if 0 as libc::c_int as libc::c_double
            > *real.offset(i as isize) - var1
        {
            0 as libc::c_int as libc::c_double
        } else {
            *real.offset(i as isize) - var1
        }) - gains_ema) * 2.0f64 / (period + 1 as libc::c_int) as libc::c_double
            + gains_ema;
        losses_ema = ((if 0 as libc::c_int as libc::c_double
            > var1 - *real.offset(i as isize)
        {
            0 as libc::c_int as libc::c_double
        } else {
            var1 - *real.offset(i as isize)
        }) - losses_ema) * 2.0f64 / (period + 1 as libc::c_int) as libc::c_double
            + losses_ema;
        let fresh4 = rmi;
        rmi = rmi.offset(1);
        *fresh4 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).options.period = period;
    (*stream).options.lookback_period = lookback_period;
    (*stream).state.gains_ema = gains_ema;
    (*stream).state.losses_ema = losses_ema;
    return 0 as libc::c_int;
}
