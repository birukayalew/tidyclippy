use ::libc;
extern "C" {
    pub type ti_stream;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_rvi {
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
    pub offset_price: libc::c_int,
    pub size_price: libc::c_int,
    pub index_price: libc::c_int,
    pub padding_price: libc::c_int,
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
    pub gains_ema: libc::c_double,
    pub losses_ema: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub sma_period: libc::c_int,
    pub stddev_period: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_rvi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let stddev_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    return stddev_period - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rvi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let sma_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let stddev_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut rvi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if sma_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if stddev_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rvi_start(options) {
        return 0 as libc::c_int;
    }
    let mut y_sum: libc::c_double = 0.0f64;
    let mut xy_sum: libc::c_double = 0.0f64;
    let x_sum: libc::c_double = (stddev_period * (stddev_period + 1 as libc::c_int))
        as libc::c_double / 2.0f64;
    let xsq_sum: libc::c_double = (stddev_period * (stddev_period + 1 as libc::c_int)
        * (2 as libc::c_int * stddev_period + 1 as libc::c_int)) as libc::c_double
        / 6.0f64;
    let mut gains_ema: libc::c_double = 0.0f64;
    let mut losses_ema: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < stddev_period {
        xy_sum += *real.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y_sum += *real.offset(i as isize);
        i += 1;
        i;
    }
    i -= 1;
    i;
    let mut b: libc::c_double = (xy_sum / stddev_period as libc::c_double
        - x_sum / stddev_period as libc::c_double * y_sum
            / stddev_period as libc::c_double)
        / (xsq_sum / stddev_period as libc::c_double
            - x_sum / stddev_period as libc::c_double
                * (x_sum / stddev_period as libc::c_double));
    let mut a: libc::c_double = y_sum / stddev_period as libc::c_double
        - b * x_sum / stddev_period as libc::c_double;
    let mut higher: libc::c_double = *real.offset(i as isize)
        - (a + b * stddev_period as libc::c_double);
    if higher > 0 as libc::c_int as libc::c_double {
        gains_ema = higher * higher / stddev_period as libc::c_double;
    } else {
        losses_ema = higher * higher / stddev_period as libc::c_double;
    }
    if gains_ema + losses_ema == 0 as libc::c_int as libc::c_double {
        let fresh0 = rvi;
        rvi = rvi.offset(1);
        *fresh0 = 50.0f64;
    } else {
        let fresh1 = rvi;
        rvi = rvi.offset(1);
        *fresh1 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
    }
    i += 1;
    i;
    while i < size {
        xy_sum += -y_sum + *real.offset(i as isize) * stddev_period as libc::c_double;
        y_sum += -*real.offset((i - stddev_period) as isize) + *real.offset(i as isize);
        let mut b_0: libc::c_double = (xy_sum / stddev_period as libc::c_double
            - x_sum / stddev_period as libc::c_double * y_sum
                / stddev_period as libc::c_double)
            / (xsq_sum / stddev_period as libc::c_double
                - x_sum / stddev_period as libc::c_double
                    * (x_sum / stddev_period as libc::c_double));
        let mut a_0: libc::c_double = y_sum / stddev_period as libc::c_double
            - b_0 * x_sum / stddev_period as libc::c_double;
        let mut higher_0: libc::c_double = *real.offset(i as isize)
            - (a_0 + b_0 * stddev_period as libc::c_double);
        if higher_0 > 0 as libc::c_int as libc::c_double {
            gains_ema = (higher_0 * higher_0 / stddev_period as libc::c_double
                - gains_ema) * 2.0f64 / (sma_period + 1 as libc::c_int) as libc::c_double
                + gains_ema;
        } else {
            losses_ema = (higher_0 * higher_0 / stddev_period as libc::c_double
                - losses_ema) * 2.0f64
                / (sma_period + 1 as libc::c_int) as libc::c_double + losses_ema;
        }
        if gains_ema + losses_ema == 0 as libc::c_int as libc::c_double {
            let fresh2 = rvi;
            rvi = rvi.offset(1);
            *fresh2 = 50.0f64;
        } else {
            let fresh3 = rvi;
            rvi = rvi.offset(1);
            *fresh3 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rvi_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_rvi_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_rvi = stream_in as *mut ti_stream_rvi;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut rvi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut progress: libc::c_int = (*stream).progress;
    let sma_period: libc::c_int = (*stream).options.sma_period;
    let stddev_period: libc::c_int = (*stream).options.stddev_period;
    let mut y_sum: libc::c_double = (*stream).state.y_sum;
    let mut xy_sum: libc::c_double = (*stream).state.xy_sum;
    let mut gains_ema: libc::c_double = (*stream).state.gains_ema;
    let mut losses_ema: libc::c_double = (*stream).state.losses_ema;
    let mut x_sum: libc::c_double = (*stream).constants.x_sum;
    let mut xsq_sum: libc::c_double = (*stream).constants.xsq_sum;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut var1: libc::c_double = 0.;
    while i < size && progress <= 0 as libc::c_int {
        let mut idx: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_price {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx;
        xy_sum
            += *real.offset(i as isize)
                * (progress - (-stddev_period + 1 as libc::c_int) + 1 as libc::c_int)
                    as libc::c_double;
        y_sum += *real.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        i -= 1;
        i;
        let mut b: libc::c_double = (xy_sum / stddev_period as libc::c_double
            - x_sum / stddev_period as libc::c_double * y_sum
                / stddev_period as libc::c_double)
            / (xsq_sum / stddev_period as libc::c_double
                - x_sum / stddev_period as libc::c_double
                    * (x_sum / stddev_period as libc::c_double));
        let mut a: libc::c_double = y_sum / stddev_period as libc::c_double
            - b * x_sum / stddev_period as libc::c_double;
        let mut higher: libc::c_double = *real.offset(i as isize)
            - (a + b * stddev_period as libc::c_double);
        if higher > 0 as libc::c_int as libc::c_double {
            gains_ema = higher * higher / stddev_period as libc::c_double;
        } else {
            losses_ema = higher * higher / stddev_period as libc::c_double;
        }
        if gains_ema + losses_ema == 0 as libc::c_int as libc::c_double {
            let fresh4 = rvi;
            rvi = rvi.offset(1);
            *fresh4 = 50.0f64;
        } else {
            let fresh5 = rvi;
            rvi = rvi.offset(1);
            *fresh5 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
        }
        i += 1;
        i;
    }
    while i < size {
        let mut idx_0: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_price {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_0 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_0;
        xy_sum += -y_sum + *real.offset(i as isize) * stddev_period as libc::c_double;
        let mut idx_1: libc::c_int = (*stream).buf_info.index_price + -stddev_period;
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
        y_sum += -var1 + *real.offset(i as isize);
        let mut b_0: libc::c_double = (xy_sum / stddev_period as libc::c_double
            - x_sum / stddev_period as libc::c_double * y_sum
                / stddev_period as libc::c_double)
            / (xsq_sum / stddev_period as libc::c_double
                - x_sum / stddev_period as libc::c_double
                    * (x_sum / stddev_period as libc::c_double));
        let mut a_0: libc::c_double = y_sum / stddev_period as libc::c_double
            - b_0 * x_sum / stddev_period as libc::c_double;
        let mut higher_0: libc::c_double = *real.offset(i as isize)
            - (a_0 + b_0 * stddev_period as libc::c_double);
        if higher_0 > 0 as libc::c_int as libc::c_double {
            gains_ema = (higher_0 * higher_0 / stddev_period as libc::c_double
                - gains_ema) * 2.0f64 / (sma_period + 1 as libc::c_int) as libc::c_double
                + gains_ema;
        } else {
            losses_ema = (higher_0 * higher_0 / stddev_period as libc::c_double
                - losses_ema) * 2.0f64
                / (sma_period + 1 as libc::c_int) as libc::c_double + losses_ema;
        }
        if gains_ema + losses_ema == 0 as libc::c_int as libc::c_double {
            let fresh6 = rvi;
            rvi = rvi.offset(1);
            *fresh6 = 50.0f64;
        } else {
            let fresh7 = rvi;
            rvi = rvi.offset(1);
            *fresh7 = gains_ema / (gains_ema + losses_ema) * 100.0f64;
        }
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.y_sum = y_sum;
    (*stream).state.xy_sum = xy_sum;
    (*stream).state.gains_ema = gains_ema;
    (*stream).state.losses_ema = losses_ema;
    (*stream).constants.x_sum = x_sum;
    (*stream).constants.xsq_sum = xsq_sum;
    return 0 as libc::c_int;
}
