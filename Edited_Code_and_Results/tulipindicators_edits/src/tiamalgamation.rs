use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __uint64_t = libc::c_ulong;
pub type ti_indicator_start_function = Option::<
    unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
>;
pub type ti_indicator_function = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const *const libc::c_double,
        *const libc::c_double,
        *const *mut libc::c_double,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream {
    pub index: libc::c_int,
    pub progress: libc::c_int,
}
pub type ti_indicator_stream_new = Option::<
    unsafe extern "C" fn(*const libc::c_double, *mut *mut ti_stream) -> libc::c_int,
>;
pub type ti_indicator_stream_run = Option::<
    unsafe extern "C" fn(
        *mut ti_stream,
        libc::c_int,
        *const *const libc::c_double,
        *const *mut libc::c_double,
    ) -> libc::c_int,
>;
pub type ti_indicator_stream_free = Option::<unsafe extern "C" fn(*mut ti_stream) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: *const libc::c_char,
    pub full_name: *const libc::c_char,
    pub start: ti_indicator_start_function,
    pub indicator: ti_indicator_function,
    pub indicator_ref: ti_indicator_function,
    pub type_0: libc::c_int,
    pub inputs: libc::c_int,
    pub options: libc::c_int,
    pub outputs: libc::c_int,
    pub input_names: [*const libc::c_char; 16],
    pub option_names: [*const libc::c_char; 16],
    pub output_names: [*const libc::c_char; 16],
    pub stream_new: ti_indicator_stream_new,
    pub stream_run: ti_indicator_stream_run,
    pub stream_free: ti_indicator_stream_free,
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
pub struct ti_stream_sma {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub period: libc::c_int,
    pub per: libc::c_double,
    pub sum: libc::c_double,
    pub buffer_idx: libc::c_int,
    pub buffer: [libc::c_double; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_atr {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub period: libc::c_int,
    pub sum: libc::c_double,
    pub last: libc::c_double,
    pub last_close: libc::c_double,
}
pub type uint64_t = __uint64_t;
pub type TC_REAL = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_result {
    pub progress: libc::c_int,
    pub space: libc::c_int,
    pub count: libc::c_int,
    pub pattern_count: libc::c_int,
    pub hits: *mut tc_hit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_hit {
    pub index: libc::c_int,
    pub patterns: tc_set,
}
pub type tc_set = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_config {
    pub period: libc::c_int,
    pub body_none: TC_REAL,
    pub body_short: TC_REAL,
    pub body_long: TC_REAL,
    pub wick_none: TC_REAL,
    pub wick_long: TC_REAL,
    pub near: TC_REAL,
}
pub type tc_candle_function = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const *const TC_REAL,
        *const tc_config,
        *mut tc_result,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_candle_info {
    pub name: *const libc::c_char,
    pub full_name: *const libc::c_char,
    pub pattern: tc_set,
    pub candle: tc_candle_function,
}
#[no_mangle]
pub unsafe extern "C" fn ti_abs_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_abs(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = fabs(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_acos_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_acos(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = acos(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ad_start(mut options: *const libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ad(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let hl: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if hl != 0.0f64 {
            sum
                += (*close.offset(i as isize) - *low.offset(i as isize)
                    - *high.offset(i as isize) + *close.offset(i as isize)) / hl
                    * *volume.offset(i as isize);
        }
        *output.offset(i as isize) = sum;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_add_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_add(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut in2: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = *in1.offset(i as isize) + *in2.offset(i as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(1 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let start: libc::c_int = long_period - 1 as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_adosc_start(options) {
        return 0 as libc::c_int;
    }
    let short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut short_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let hl: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if hl != 0.0f64 {
            sum
                += (*close.offset(i as isize) - *low.offset(i as isize)
                    - *high.offset(i as isize) + *close.offset(i as isize)) / hl
                    * *volume.offset(i as isize);
        }
        if i == 0 as libc::c_int {
            short_ema = sum;
            long_ema = sum;
        } else {
            short_ema = (sum - short_ema) * short_per + short_ema;
            long_ema = (sum - long_ema) * long_per + long_ema;
        }
        if i >= start {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = short_ema - long_ema;
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_adosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_adosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1600 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_adosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adx_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return (*options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int)
        * 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adx(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_adx_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let invper: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let mut dp: libc::c_double = 0.;
        let mut dm: libc::c_double = 0.;
        dp = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp < 0 as libc::c_int as libc::c_double {
            dp = 0 as libc::c_int as libc::c_double;
        } else if dp > dm {
            dm = 0 as libc::c_int as libc::c_double;
        }
        if dm < 0 as libc::c_int as libc::c_double {
            dm = 0 as libc::c_int as libc::c_double;
        } else if dm > dp {
            dp = 0 as libc::c_int as libc::c_double;
        }
        dmup += dp;
        dmdown += dm;
        i += 1;
        i;
    }
    let mut adx: libc::c_double = 0.0f64;
    let mut di_up: libc::c_double = dmup;
    let mut di_down: libc::c_double = dmdown;
    let mut dm_diff: libc::c_double = fabs(di_up - di_down);
    let mut dm_sum: libc::c_double = di_up + di_down;
    let mut dx: libc::c_double = dm_diff / dm_sum * 100 as libc::c_int as libc::c_double;
    adx += dx;
    i = period;
    while i < size {
        let mut dp_0: libc::c_double = 0.;
        let mut dm_0: libc::c_double = 0.;
        dp_0 = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm_0 = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp_0 < 0 as libc::c_int as libc::c_double {
            dp_0 = 0 as libc::c_int as libc::c_double;
        } else if dp_0 > dm_0 {
            dm_0 = 0 as libc::c_int as libc::c_double;
        }
        if dm_0 < 0 as libc::c_int as libc::c_double {
            dm_0 = 0 as libc::c_int as libc::c_double;
        } else if dm_0 > dp_0 {
            dp_0 = 0 as libc::c_int as libc::c_double;
        }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let mut di_up_0: libc::c_double = dmup;
        let mut di_down_0: libc::c_double = dmdown;
        let mut dm_diff_0: libc::c_double = fabs(di_up_0 - di_down_0);
        let mut dm_sum_0: libc::c_double = di_up_0 + di_down_0;
        let mut dx_0: libc::c_double = dm_diff_0 / dm_sum_0
            * 100 as libc::c_int as libc::c_double;
        if i - period < period - 2 as libc::c_int {
            adx += dx_0;
        } else if i - period == period - 2 as libc::c_int {
            adx += dx_0;
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = adx * invper;
        } else {
            adx = adx * per + dx_0;
            let fresh2 = output;
            output = output.offset(1);
            *fresh2 = adx * invper;
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_adx_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_adx_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1653 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_adx(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adxr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return (*options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int)
        * 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adxr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_adxr_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let invper: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let mut dp: libc::c_double = 0.;
        let mut dm: libc::c_double = 0.;
        dp = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp < 0 as libc::c_int as libc::c_double {
            dp = 0 as libc::c_int as libc::c_double;
        } else if dp > dm {
            dm = 0 as libc::c_int as libc::c_double;
        }
        if dm < 0 as libc::c_int as libc::c_double {
            dm = 0 as libc::c_int as libc::c_double;
        } else if dm > dp {
            dp = 0 as libc::c_int as libc::c_double;
        }
        dmup += dp;
        dmdown += dm;
        i += 1;
        i;
    }
    let mut adx: libc::c_double = 0.0f64;
    let mut di_up: libc::c_double = dmup;
    let mut di_down: libc::c_double = dmdown;
    let mut dm_diff: libc::c_double = fabs(di_up - di_down);
    let mut dm_sum: libc::c_double = di_up + di_down;
    let mut dx: libc::c_double = dm_diff / dm_sum * 100 as libc::c_int as libc::c_double;
    adx += dx;
    let mut adxr: *mut ti_buffer = ti_buffer_new(period - 1 as libc::c_int);
    let first_adxr: libc::c_int = ti_adxr_start(options);
    i = period;
    while i < size {
        let mut dp_0: libc::c_double = 0.;
        let mut dm_0: libc::c_double = 0.;
        dp_0 = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm_0 = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp_0 < 0 as libc::c_int as libc::c_double {
            dp_0 = 0 as libc::c_int as libc::c_double;
        } else if dp_0 > dm_0 {
            dm_0 = 0 as libc::c_int as libc::c_double;
        }
        if dm_0 < 0 as libc::c_int as libc::c_double {
            dm_0 = 0 as libc::c_int as libc::c_double;
        } else if dm_0 > dp_0 {
            dp_0 = 0 as libc::c_int as libc::c_double;
        }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let mut di_up_0: libc::c_double = dmup;
        let mut di_down_0: libc::c_double = dmdown;
        let mut dm_diff_0: libc::c_double = fabs(di_up_0 - di_down_0);
        let mut dm_sum_0: libc::c_double = di_up_0 + di_down_0;
        let mut dx_0: libc::c_double = dm_diff_0 / dm_sum_0
            * 100 as libc::c_int as libc::c_double;
        if i - period < period - 2 as libc::c_int {
            adx += dx_0;
        } else if i - period == period - 2 as libc::c_int {
            adx += dx_0;
            *((*adxr).vals).as_mut_ptr().offset((*adxr).index as isize) = adx * invper;
            (*adxr).index = (*adxr).index + 1 as libc::c_int;
            if (*adxr).index >= (*adxr).size {
                (*adxr).index = 0 as libc::c_int;
            }
        } else {
            adx = adx * per + dx_0;
            if i >= first_adxr {
                let fresh3 = output;
                output = output.offset(1);
                *fresh3 = 0.5f64
                    * (adx * invper
                        + *((*adxr).vals)
                            .as_mut_ptr()
                            .offset(
                                (((*adxr).index + (*adxr).size - 1 as libc::c_int
                                    + 1 as libc::c_int) % (*adxr).size) as isize,
                            ));
            }
            *((*adxr).vals).as_mut_ptr().offset((*adxr).index as isize) = adx * invper;
            (*adxr).index = (*adxr).index + 1 as libc::c_int;
            if (*adxr).index >= (*adxr).size {
                (*adxr).index = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(adxr);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_adxr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_adxr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1712 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_adxr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ao_start(mut options: *const libc::c_double) -> libc::c_int {
    return 33 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ao(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = 34 as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_ao_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum34: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum5: libc::c_double = 0 as libc::c_int as libc::c_double;
    let per34: libc::c_double = 1.0f64 / 34.0f64;
    let per5: libc::c_double = 1.0f64 / 5.0f64;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 34 as libc::c_int {
        let mut hl: libc::c_double = 0.5f64
            * (*high.offset(i as isize) + *low.offset(i as isize));
        sum34 += hl;
        if i >= 29 as libc::c_int {
            sum5 += hl;
        }
        i += 1;
        i;
    }
    let fresh4 = output;
    output = output.offset(1);
    *fresh4 = per5 * sum5 - per34 * sum34;
    i = period;
    while i < size {
        let mut hl_0: libc::c_double = 0.5f64
            * (*high.offset(i as isize) + *low.offset(i as isize));
        sum34 += hl_0;
        sum5 += hl_0;
        sum34
            -= 0.5f64
                * (*high.offset((i - 34 as libc::c_int) as isize)
                    + *low.offset((i - 34 as libc::c_int) as isize));
        sum5
            -= 0.5f64
                * (*high.offset((i - 5 as libc::c_int) as isize)
                    + *low.offset((i - 5 as libc::c_int) as isize));
        let fresh5 = output;
        output = output.offset(1);
        *fresh5 = per5 * sum5 - per34 * sum34;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ao_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_ao_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1744 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_ao(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_apo_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_apo(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut apo: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_apo_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut short_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut long_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        short_ema = (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema = (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: libc::c_double = short_ema - long_ema;
        let fresh6 = apo;
        apo = apo.offset(1);
        *fresh6 = out;
        i += 1;
        i;
    }
    if apo.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_apo_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"apo - outputs[0] == size - ti_apo_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1771 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_apo(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_aroon_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_aroon(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut adown: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut aup: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_aroon_start(options) {
        return 0 as libc::c_int;
    }
    let scale: libc::c_double = 100.0f64 / period as libc::c_double;
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period;
    while i < size {
        bar = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *high.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *low.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let fresh7 = adown;
        adown = adown.offset(1);
        *fresh7 = (period as libc::c_double - (i - mini) as libc::c_double) * scale;
        let fresh8 = aup;
        aup = aup.offset(1);
        *fresh8 = (period as libc::c_double - (i - maxi) as libc::c_double) * scale;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if adown.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_aroon_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"adown - outputs[0] == size - ti_aroon_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1827 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_aroon(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if aup.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_aroon_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"aup - outputs[1] == size - ti_aroon_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1828 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_aroon(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_aroonosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_aroonosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_aroon_start(options) {
        return 0 as libc::c_int;
    }
    let scale: libc::c_double = 100.0f64 / period as libc::c_double;
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period;
    while i < size {
        let mut bar: libc::c_double = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *high.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *low.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let fresh9 = output;
        output = output.offset(1);
        *fresh9 = (maxi - mini) as libc::c_double * scale;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_aroonosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_aroonosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1881 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"int ti_aroonosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_asin_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_asin(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = asin(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atan_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atan(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = atan(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_atr_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut truerange: libc::c_double = 0.;
    sum
        += *high.offset(0 as libc::c_int as isize)
            - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let l: libc::c_double = *low.offset(i as isize);
        let h: libc::c_double = *high.offset(i as isize);
        let c: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
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
        sum += truerange;
        i += 1;
        i;
    }
    let mut val: libc::c_double = sum / period as libc::c_double;
    let fresh10 = output;
    output = output.offset(1);
    *fresh10 = val;
    i = period;
    while i < size {
        let l_0: libc::c_double = *low.offset(i as isize);
        let h_0: libc::c_double = *high.offset(i as isize);
        let c_0: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
        let ych_0: libc::c_double = fabs(h_0 - c_0);
        let ycl_0: libc::c_double = fabs(l_0 - c_0);
        let mut v_0: libc::c_double = h_0 - l_0;
        if ych_0 > v_0 {
            v_0 = ych_0;
        }
        if ycl_0 > v_0 {
            v_0 = ycl_0;
        }
        truerange = v_0;
        val = (truerange - val) * per + val;
        let fresh11 = output;
        output = output.offset(1);
        *fresh11 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_atr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_atr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1913 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_atr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_atr_start(options) {
        return 0 as libc::c_int;
    }
    let tr_start: libc::c_int = ti_tr_start(options);
    let tr_size: libc::c_int = size - tr_start;
    let mut truerange: *mut libc::c_double = malloc(
        (tr_size as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if truerange.is_null() {
        return 2 as libc::c_int;
    }
    let mut tr_outputs: [*mut libc::c_double; 1] = [truerange];
    let tr_ret: libc::c_int = ti_tr(
        size,
        inputs,
        0 as *const libc::c_double,
        tr_outputs.as_mut_ptr(),
    );
    if tr_ret != 0 as libc::c_int {
        free(truerange as *mut libc::c_void);
        return tr_ret;
    }
    let mut wilders_inputs: [*const libc::c_double; 1] = [
        truerange as *const libc::c_double,
    ];
    let wilders_ret: libc::c_int = ti_wilders(
        tr_size,
        wilders_inputs.as_mut_ptr(),
        options,
        outputs,
    );
    free(truerange as *mut libc::c_void);
    if size - ti_atr_start(options) == size - ti_wilders_start(options) {} else {
        __assert_fail(
            b"size - ti_atr_start(options) == size - ti_wilders_start(options)\0"
                as *const u8 as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            1933 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_atr_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return wilders_ret;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr_stream_new(
    mut options: *const libc::c_double,
    mut stream_in: *mut *mut ti_stream,
) -> libc::c_int {
    let mut stream: *mut *mut ti_stream_atr = stream_in as *mut *mut ti_stream_atr;
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *stream = malloc(::core::mem::size_of::<ti_stream_atr>() as libc::c_ulong)
        as *mut ti_stream_atr;
    if (*stream).is_null() {
        return 2 as libc::c_int;
    }
    (**stream).index = 13 as libc::c_int;
    (**stream).progress = -ti_atr_start(options);
    (**stream).period = period;
    (**stream).sum = 0.0f64;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_atr = stream_in as *mut ti_stream_atr;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let per: libc::c_double = 1.0f64 / (*stream).period as libc::c_double;
    let start: libc::c_int = -((*stream).period - 1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*stream).progress < 1 as libc::c_int {
        if (*stream).progress == start {
            (*stream)
                .sum = *high.offset(0 as libc::c_int as isize)
                - *low.offset(0 as libc::c_int as isize);
            (*stream).last_close = *close.offset(0 as libc::c_int as isize);
            (*stream).progress += 1;
            (*stream).progress;
            i += 1;
            i;
        }
        while (*stream).progress <= 0 as libc::c_int && i < size {
            let mut truerange: libc::c_double = 0.;
            let ych: libc::c_double = fabs(
                *high.offset(i as isize) - (*stream).last_close,
            );
            let ycl: libc::c_double = fabs(
                *low.offset(i as isize) - (*stream).last_close,
            );
            let mut v: libc::c_double = *high.offset(i as isize)
                - *low.offset(i as isize);
            if ych > v {
                v = ych;
            }
            if ycl > v {
                v = ycl;
            }
            truerange = v;
            (*stream).sum += truerange;
            (*stream).last_close = *close.offset(i as isize);
            (*stream).progress += 1;
            (*stream).progress;
            i += 1;
            i;
        }
        if (*stream).progress == 1 as libc::c_int {
            let val: libc::c_double = (*stream).sum * per;
            (*stream).last = val;
            let fresh12 = output;
            output = output.offset(1);
            *fresh12 = val;
        }
    }
    if (*stream).progress >= 1 as libc::c_int {
        let mut val_0: libc::c_double = (*stream).last;
        while i < size {
            let mut truerange_0: libc::c_double = 0.;
            let ych_0: libc::c_double = fabs(
                *high.offset(i as isize) - (*stream).last_close,
            );
            let ycl_0: libc::c_double = fabs(
                *low.offset(i as isize) - (*stream).last_close,
            );
            let mut v_0: libc::c_double = *high.offset(i as isize)
                - *low.offset(i as isize);
            if ych_0 > v_0 {
                v_0 = ych_0;
            }
            if ycl_0 > v_0 {
                v_0 = ycl_0;
            }
            truerange_0 = v_0;
            val_0 = (truerange_0 - val_0) * per + val_0;
            let fresh13 = output;
            output = output.offset(1);
            *fresh13 = val_0;
            (*stream).last_close = *close.offset(i as isize);
            (*stream).progress += 1;
            (*stream).progress;
            i += 1;
            i;
        }
        (*stream).last = val_0;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_avgprice_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_avgprice(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut open: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = (*open.offset(i as isize) + *high.offset(i as isize)
            + *low.offset(i as isize) + *close.offset(i as isize)) * 0.25f64;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_bbands_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_bbands(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut lower: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut middle: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut upper: *mut libc::c_double = *outputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let stddev: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_bbands_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1;
        i;
    }
    let mut sd: libc::c_double = sqrt(sum2 * scale - sum * scale * (sum * scale));
    *middle = sum * scale;
    let fresh14 = lower;
    lower = lower.offset(1);
    *fresh14 = *middle - stddev * sd;
    let fresh15 = upper;
    upper = upper.offset(1);
    *fresh15 = *middle + stddev * sd;
    middle = middle.offset(1);
    middle;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2
            -= *input.offset((i - period) as isize)
                * *input.offset((i - period) as isize);
        sd = sqrt(sum2 * scale - sum * scale * (sum * scale));
        *middle = sum * scale;
        let fresh16 = upper;
        upper = upper.offset(1);
        *fresh16 = *middle + stddev * sd;
        let fresh17 = lower;
        lower = lower.offset(1);
        *fresh17 = *middle - stddev * sd;
        middle = middle.offset(1);
        middle;
        i += 1;
        i;
    }
    if lower.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_bbands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"lower - outputs[0] == size - ti_bbands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2054 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_bbands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if middle.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_bbands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"middle - outputs[1] == size - ti_bbands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2055 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_bbands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if upper.offset_from(*outputs.offset(2 as libc::c_int as isize)) as libc::c_long
        == (size - ti_bbands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"upper - outputs[2] == size - ti_bbands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2056 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_bbands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_bop_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_bop(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut open: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let mut hl: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if hl <= 0.0f64 {
            *output.offset(i as isize) = 0 as libc::c_int as libc::c_double;
        } else {
            *output
                .offset(
                    i as isize,
                ) = (*close.offset(i as isize) - *open.offset(i as isize)) / hl;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cci_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cci(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_cci_start(options) {
        return 0 as libc::c_int;
    }
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: *mut ti_buffer = ti_buffer_new(period);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let today: libc::c_double = (*high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        if (*sum).pushes >= (*sum).size {
            (*sum).sum -= *((*sum).vals).as_mut_ptr().offset((*sum).index as isize);
        }
        (*sum).sum += today;
        *((*sum).vals).as_mut_ptr().offset((*sum).index as isize) = today;
        (*sum).pushes += 1 as libc::c_int;
        (*sum).index = (*sum).index + 1 as libc::c_int;
        if (*sum).index >= (*sum).size {
            (*sum).index = 0 as libc::c_int;
        }
        let avg: libc::c_double = (*sum).sum * scale;
        if i >= period * 2 as libc::c_int - 2 as libc::c_int {
            let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
            j = 0 as libc::c_int;
            while j < period {
                acc += fabs(avg - *((*sum).vals).as_mut_ptr().offset(j as isize));
                j += 1;
                j;
            }
            let mut cci: libc::c_double = acc * scale;
            cci *= 0.015f64;
            cci = (today - avg) / cci;
            let fresh18 = output;
            output = output.offset(1);
            *fresh18 = cci;
        }
        i += 1;
        i;
    }
    ti_buffer_free(sum);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_cci_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_cci_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2112 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_cci(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ceil_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ceil(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = ceil(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cmo_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cmo(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_cmo_start(options) {
        return 0 as libc::c_int;
    }
    let mut up_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut down_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        up_sum
            += if *input.offset(i as isize)
                > *input.offset((i - 1 as libc::c_int) as isize)
            {
                *input.offset(i as isize)
                    - *input.offset((i - 1 as libc::c_int) as isize)
            } else {
                0 as libc::c_int as libc::c_double
            };
        down_sum
            += if *input.offset(i as isize)
                < *input.offset((i - 1 as libc::c_int) as isize)
            {
                *input.offset((i - 1 as libc::c_int) as isize)
                    - *input.offset(i as isize)
            } else {
                0 as libc::c_int as libc::c_double
            };
        i += 1;
        i;
    }
    let fresh19 = output;
    output = output.offset(1);
    *fresh19 = 100 as libc::c_int as libc::c_double * (up_sum - down_sum)
        / (up_sum + down_sum);
    i = period + 1 as libc::c_int;
    while i < size {
        up_sum
            -= if *input.offset((i - period) as isize)
                > *input.offset((i - period - 1 as libc::c_int) as isize)
            {
                *input.offset((i - period) as isize)
                    - *input.offset((i - period - 1 as libc::c_int) as isize)
            } else {
                0 as libc::c_int as libc::c_double
            };
        down_sum
            -= if *input.offset((i - period) as isize)
                < *input.offset((i - period - 1 as libc::c_int) as isize)
            {
                *input.offset((i - period - 1 as libc::c_int) as isize)
                    - *input.offset((i - period) as isize)
            } else {
                0 as libc::c_int as libc::c_double
            };
        up_sum
            += if *input.offset(i as isize)
                > *input.offset((i - 1 as libc::c_int) as isize)
            {
                *input.offset(i as isize)
                    - *input.offset((i - 1 as libc::c_int) as isize)
            } else {
                0 as libc::c_int as libc::c_double
            };
        down_sum
            += if *input.offset(i as isize)
                < *input.offset((i - 1 as libc::c_int) as isize)
            {
                *input.offset((i - 1 as libc::c_int) as isize)
                    - *input.offset(i as isize)
            } else {
                0 as libc::c_int as libc::c_double
            };
        let fresh20 = output;
        output = output.offset(1);
        *fresh20 = 100 as libc::c_int as libc::c_double * (up_sum - down_sum)
            / (up_sum + down_sum);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_cmo_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_cmo_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2139 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_cmo(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cos_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cos(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = cos(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cosh_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cosh(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = cosh(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_crossany_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_crossany(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut a: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut b: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let fresh21 = output;
        output = output.offset(1);
        *fresh21 = (*a.offset(i as isize) > *b.offset(i as isize)
            && *a.offset((i - 1 as libc::c_int) as isize)
                <= *b.offset((i - 1 as libc::c_int) as isize)
            || *a.offset(i as isize) < *b.offset(i as isize)
                && *a.offset((i - 1 as libc::c_int) as isize)
                    >= *b.offset((i - 1 as libc::c_int) as isize)) as libc::c_int
            as libc::c_double;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_crossover_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_crossover(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut a: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut b: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let fresh22 = output;
        output = output.offset(1);
        *fresh22 = (*a.offset(i as isize) > *b.offset(i as isize)
            && *a.offset((i - 1 as libc::c_int) as isize)
                <= *b.offset((i - 1 as libc::c_int) as isize)) as libc::c_int
            as libc::c_double;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cvi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let n: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return n * 2 as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cvi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_cvi_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut lag: *mut ti_buffer = ti_buffer_new(period);
    let mut val: libc::c_double = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period * 2 as libc::c_int - 1 as libc::c_int {
        val = (*high.offset(i as isize) - *low.offset(i as isize) - val) * per + val;
        *((*lag).vals).as_mut_ptr().offset((*lag).index as isize) = val;
        (*lag).index = (*lag).index + 1 as libc::c_int;
        if (*lag).index >= (*lag).size {
            (*lag).index = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    i = period * 2 as libc::c_int - 1 as libc::c_int;
    while i < size {
        val = (*high.offset(i as isize) - *low.offset(i as isize) - val) * per + val;
        let old: libc::c_double = *((*lag).vals)
            .as_mut_ptr()
            .offset((*lag).index as isize);
        let fresh23 = output;
        output = output.offset(1);
        *fresh23 = 100.0f64 * (val - old) / old;
        *((*lag).vals).as_mut_ptr().offset((*lag).index as isize) = val;
        (*lag).index = (*lag).index + 1 as libc::c_int;
        if (*lag).index >= (*lag).size {
            (*lag).index = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    ti_buffer_free(lag);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_cvi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_cvi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2201 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_cvi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_decay_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_decay(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    let fresh24 = output;
    output = output.offset(1);
    *fresh24 = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let mut d: libc::c_double = *output.offset(-(1 as libc::c_int) as isize) - scale;
        let fresh25 = output;
        output = output.offset(1);
        *fresh25 = if *input.offset(i as isize) > d {
            *input.offset(i as isize)
        } else {
            d
        };
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dema_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dema(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_dema_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let per1: libc::c_double = 1.0f64 - per;
    let mut ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut ema2: libc::c_double = ema;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        ema = ema * per1 + *input.offset(i as isize) * per;
        if i == period - 1 as libc::c_int {
            ema2 = ema;
        }
        if i >= period - 1 as libc::c_int {
            ema2 = ema2 * per1 + ema * per;
            if i >= (period - 1 as libc::c_int) * 2 as libc::c_int {
                *output = ema * 2 as libc::c_int as libc::c_double - ema2;
                output = output.offset(1);
                output;
            }
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dema_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_dema_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2249 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_dema(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_di_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_di(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut plus_di: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut minus_di: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_di_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let mut atr: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let mut truerange: libc::c_double = 0.;
        let l: libc::c_double = *low.offset(i as isize);
        let h: libc::c_double = *high.offset(i as isize);
        let c: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
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
        let mut dp: libc::c_double = 0.;
        let mut dm: libc::c_double = 0.;
        dp = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp < 0 as libc::c_int as libc::c_double {
            dp = 0 as libc::c_int as libc::c_double;
        } else if dp > dm {
            dm = 0 as libc::c_int as libc::c_double;
        }
        if dm < 0 as libc::c_int as libc::c_double {
            dm = 0 as libc::c_int as libc::c_double;
        } else if dm > dp {
            dp = 0 as libc::c_int as libc::c_double;
        }
        dmup += dp;
        dmdown += dm;
        i += 1;
        i;
    }
    let fresh26 = plus_di;
    plus_di = plus_di.offset(1);
    *fresh26 = 100.0f64 * dmup / atr;
    let fresh27 = minus_di;
    minus_di = minus_di.offset(1);
    *fresh27 = 100.0f64 * dmdown / atr;
    i = period;
    while i < size {
        let mut truerange_0: libc::c_double = 0.;
        let l_0: libc::c_double = *low.offset(i as isize);
        let h_0: libc::c_double = *high.offset(i as isize);
        let c_0: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
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
        atr = atr * per + truerange_0;
        let mut dp_0: libc::c_double = 0.;
        let mut dm_0: libc::c_double = 0.;
        dp_0 = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm_0 = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp_0 < 0 as libc::c_int as libc::c_double {
            dp_0 = 0 as libc::c_int as libc::c_double;
        } else if dp_0 > dm_0 {
            dm_0 = 0 as libc::c_int as libc::c_double;
        }
        if dm_0 < 0 as libc::c_int as libc::c_double {
            dm_0 = 0 as libc::c_int as libc::c_double;
        } else if dm_0 > dp_0 {
            dp_0 = 0 as libc::c_int as libc::c_double;
        }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let fresh28 = plus_di;
        plus_di = plus_di.offset(1);
        *fresh28 = 100.0f64 * dmup / atr;
        let fresh29 = minus_di;
        minus_di = minus_di.offset(1);
        *fresh29 = 100.0f64 * dmdown / atr;
        i += 1;
        i;
    }
    if plus_di.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_di_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"plus_di - outputs[0] == size - ti_di_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2291 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_di(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if minus_di.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_di_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"minus_di - outputs[1] == size - ti_di_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2292 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_di(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_div_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_div(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut in2: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = *in1.offset(i as isize) / *in2.offset(i as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dm_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dm(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut plus_dm: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut minus_dm: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_dm_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let mut dp: libc::c_double = 0.;
        let mut dm: libc::c_double = 0.;
        dp = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp < 0 as libc::c_int as libc::c_double {
            dp = 0 as libc::c_int as libc::c_double;
        } else if dp > dm {
            dm = 0 as libc::c_int as libc::c_double;
        }
        if dm < 0 as libc::c_int as libc::c_double {
            dm = 0 as libc::c_int as libc::c_double;
        } else if dm > dp {
            dp = 0 as libc::c_int as libc::c_double;
        }
        dmup += dp;
        dmdown += dm;
        i += 1;
        i;
    }
    let fresh30 = plus_dm;
    plus_dm = plus_dm.offset(1);
    *fresh30 = dmup;
    let fresh31 = minus_dm;
    minus_dm = minus_dm.offset(1);
    *fresh31 = dmdown;
    i = period;
    while i < size {
        let mut dp_0: libc::c_double = 0.;
        let mut dm_0: libc::c_double = 0.;
        dp_0 = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm_0 = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp_0 < 0 as libc::c_int as libc::c_double {
            dp_0 = 0 as libc::c_int as libc::c_double;
        } else if dp_0 > dm_0 {
            dm_0 = 0 as libc::c_int as libc::c_double;
        }
        if dm_0 < 0 as libc::c_int as libc::c_double {
            dm_0 = 0 as libc::c_int as libc::c_double;
        } else if dm_0 > dp_0 {
            dp_0 = 0 as libc::c_int as libc::c_double;
        }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let fresh32 = plus_dm;
        plus_dm = plus_dm.offset(1);
        *fresh32 = dmup;
        let fresh33 = minus_dm;
        minus_dm = minus_dm.offset(1);
        *fresh33 = dmdown;
        i += 1;
        i;
    }
    if plus_dm.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dm_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"plus_dm - outputs[0] == size - ti_dm_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2327 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_dm(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if minus_dm.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dm_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"minus_dm - outputs[1] == size - ti_dm_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2328 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_dm(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dpo_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dpo(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let back: libc::c_int = period / 2 as libc::c_int + 1 as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_dpo_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let fresh34 = output;
    output = output.offset(1);
    *fresh34 = *input.offset((period - 1 as libc::c_int - back) as isize) - sum * scale;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        let fresh35 = output;
        output = output.offset(1);
        *fresh35 = *input.offset((i - back) as isize) - sum * scale;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dpo_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_dpo_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2353 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_dpo(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dx_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dx(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_dx_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let mut dp: libc::c_double = 0.;
        let mut dm: libc::c_double = 0.;
        dp = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp < 0 as libc::c_int as libc::c_double {
            dp = 0 as libc::c_int as libc::c_double;
        } else if dp > dm {
            dm = 0 as libc::c_int as libc::c_double;
        }
        if dm < 0 as libc::c_int as libc::c_double {
            dm = 0 as libc::c_int as libc::c_double;
        } else if dm > dp {
            dp = 0 as libc::c_int as libc::c_double;
        }
        dmup += dp;
        dmdown += dm;
        i += 1;
        i;
    }
    let mut di_up: libc::c_double = dmup;
    let mut di_down: libc::c_double = dmdown;
    let mut dm_diff: libc::c_double = fabs(di_up - di_down);
    let mut dm_sum: libc::c_double = di_up + di_down;
    let mut dx: libc::c_double = dm_diff / dm_sum * 100 as libc::c_int as libc::c_double;
    let fresh36 = output;
    output = output.offset(1);
    *fresh36 = dx;
    i = period;
    while i < size {
        let mut dp_0: libc::c_double = 0.;
        let mut dm_0: libc::c_double = 0.;
        dp_0 = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm_0 = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp_0 < 0 as libc::c_int as libc::c_double {
            dp_0 = 0 as libc::c_int as libc::c_double;
        } else if dp_0 > dm_0 {
            dm_0 = 0 as libc::c_int as libc::c_double;
        }
        if dm_0 < 0 as libc::c_int as libc::c_double {
            dm_0 = 0 as libc::c_int as libc::c_double;
        } else if dm_0 > dp_0 {
            dp_0 = 0 as libc::c_int as libc::c_double;
        }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let mut di_up_0: libc::c_double = dmup;
        let mut di_down_0: libc::c_double = dmdown;
        let mut dm_diff_0: libc::c_double = fabs(di_up_0 - di_down_0);
        let mut dm_sum_0: libc::c_double = di_up_0 + di_down_0;
        let mut dx_0: libc::c_double = dm_diff_0 / dm_sum_0
            * 100 as libc::c_int as libc::c_double;
        let fresh37 = output;
        output = output.offset(1);
        *fresh37 = dx_0;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dx_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_dx_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_dx(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_edecay_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_edecay(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 - 1.0f64 / period as libc::c_double;
    let fresh38 = output;
    output = output.offset(1);
    *fresh38 = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let mut d: libc::c_double = *output.offset(-(1 as libc::c_int) as isize) * scale;
        let fresh39 = output;
        output = output.offset(1);
        *fresh39 = if *input.offset(i as isize) > d {
            *input.offset(i as isize)
        } else {
            d
        };
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ema_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ema(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_ema_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut val: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let fresh40 = output;
    output = output.offset(1);
    *fresh40 = val;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        val = (*input.offset(i as isize) - val) * per + val;
        let fresh41 = output;
        output = output.offset(1);
        *fresh41 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ema_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_ema_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2434 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_ema(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_emv_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_emv(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_emv_start(options) {
        return 0 as libc::c_int;
    }
    let mut last: libc::c_double = (*high.offset(0 as libc::c_int as isize)
        + *low.offset(0 as libc::c_int as isize)) * 0.5f64;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let mut hl: libc::c_double = (*high.offset(i as isize) + *low.offset(i as isize))
            * 0.5f64;
        let mut br: libc::c_double = *volume.offset(i as isize) / 10000.0f64
            / (*high.offset(i as isize) - *low.offset(i as isize));
        let fresh42 = output;
        output = output.offset(1);
        *fresh42 = (hl - last) / br;
        last = hl;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_emv_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_emv_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2456 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_emv(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_exp_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_exp(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = exp(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fisher_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fisher(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut fisher: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut signal: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_fisher_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = 0.5f64
        * (*high.offset(0 as libc::c_int as isize)
            + *low.offset(0 as libc::c_int as isize));
    let mut min: libc::c_double = 0.5f64
        * (*high.offset(0 as libc::c_int as isize)
            + *low.offset(0 as libc::c_int as isize));
    let mut val1: libc::c_double = 0.0f64;
    let mut bar: libc::c_double = 0.;
    let mut fish: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        if maxi < trail {
            maxi = trail;
            max = 0.5f64 * (*high.offset(maxi as isize) + *low.offset(maxi as isize));
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = 0.5f64 * (*high.offset(j as isize) + *low.offset(j as isize));
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        if mini < trail {
            mini = trail;
            min = 0.5f64 * (*high.offset(mini as isize) + *low.offset(mini as isize));
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = 0.5f64 * (*high.offset(j as isize) + *low.offset(j as isize));
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let mut mm: libc::c_double = max - min;
        if mm == 0.0f64 {
            mm = 0.001f64;
        }
        val1 = 0.33f64 * 2.0f64
            * ((0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize)) - min) / mm
                - 0.5f64) + 0.67f64 * val1;
        if val1 > 0.99f64 {
            val1 = 0.999f64;
        }
        if val1 < -0.99f64 {
            val1 = -0.999f64;
        }
        let fresh43 = signal;
        signal = signal.offset(1);
        *fresh43 = fish;
        fish = 0.5f64 * log((1.0f64 + val1) / (1.0f64 - val1)) + 0.5f64 * fish;
        let fresh44 = fisher;
        fisher = fisher.offset(1);
        *fresh44 = fish;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if fisher.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_fisher_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"fisher - outputs[0] == size - ti_fisher_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2520 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_fisher(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if signal.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_fisher_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"signal - outputs[1] == size - ti_fisher_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2521 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_fisher(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_floor_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_floor(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = floor(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_fosc_start(options) {
        return 0 as libc::c_int;
    }
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let p: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut tsf: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        x += (i + 1 as libc::c_int) as libc::c_double;
        x2 += ((i + 1 as libc::c_int) * (i + 1 as libc::c_int)) as libc::c_double;
        xy += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y += *input.offset(i as isize);
        i += 1;
        i;
    }
    x += period as libc::c_double;
    x2 += (period * period) as libc::c_double;
    let bd: libc::c_double = 1.0f64 / (period as libc::c_double * x2 - x * x);
    i = period - 1 as libc::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as libc::c_double;
        y += *input.offset(i as isize);
        let b: libc::c_double = (period as libc::c_double * xy - x * y) * bd;
        let a: libc::c_double = (y - b * x) * p;
        if i >= period {
            let fresh45 = output;
            output = output.offset(1);
            *fresh45 = 100 as libc::c_int as libc::c_double
                * (*input.offset(i as isize) - tsf) / *input.offset(i as isize);
        }
        tsf = a + b * (period + 1 as libc::c_int) as libc::c_double;
        xy -= y;
        y -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_fosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_fosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2535 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_fosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_hma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let periodsqrt: libc::c_int = sqrt(period as libc::c_double) as libc::c_int;
    return period + periodsqrt - 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_hma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_hma_start(options) {
        return 0 as libc::c_int;
    }
    let period2: libc::c_int = period / 2 as libc::c_int;
    let periodsqrt: libc::c_int = sqrt(period as libc::c_double) as libc::c_int;
    let weights: libc::c_double = (period * (period + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let weights2: libc::c_double = (period2 * (period2 + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let weightssqrt: libc::c_double = (periodsqrt * (periodsqrt + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sumsqrt: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sumsqrt: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        weight_sum
            += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        sum += *input.offset(i as isize);
        if i >= period - period2 {
            weight_sum2
                += *input.offset(i as isize)
                    * (i + 1 as libc::c_int - (period - period2)) as libc::c_double;
            sum2 += *input.offset(i as isize);
        }
        i += 1;
        i;
    }
    let mut buff: *mut ti_buffer = ti_buffer_new(periodsqrt);
    i = period - 1 as libc::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize) * period as libc::c_double;
        sum += *input.offset(i as isize);
        weight_sum2 += *input.offset(i as isize) * period2 as libc::c_double;
        sum2 += *input.offset(i as isize);
        let wma: libc::c_double = weight_sum / weights;
        let wma2: libc::c_double = weight_sum2 / weights2;
        let diff: libc::c_double = 2 as libc::c_int as libc::c_double * wma2 - wma;
        weight_sumsqrt += diff * periodsqrt as libc::c_double;
        sumsqrt += diff;
        *((*buff).vals).as_mut_ptr().offset((*buff).index as isize) = diff;
        (*buff).index = (*buff).index + 1 as libc::c_int;
        if (*buff).index >= (*buff).size {
            (*buff).index = 0 as libc::c_int;
        }
        if i >= period - 1 as libc::c_int + (periodsqrt - 1 as libc::c_int) {
            let fresh46 = output;
            output = output.offset(1);
            *fresh46 = weight_sumsqrt / weightssqrt;
            weight_sumsqrt -= sumsqrt;
            sumsqrt
                -= *((*buff).vals)
                    .as_mut_ptr()
                    .offset(
                        (((*buff).index + (*buff).size - 1 as libc::c_int
                            + 1 as libc::c_int) % (*buff).size) as isize,
                    );
        } else {
            weight_sumsqrt -= sumsqrt;
        }
        weight_sum -= sum;
        sum -= *input.offset((i - period + 1 as libc::c_int) as isize);
        weight_sum2 -= sum2;
        sum2 -= *input.offset((i - period2 + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    ti_buffer_free(buff);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_hma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_hma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2594 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_hma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kama_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kama(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_kama_start(options) {
        return 0 as libc::c_int;
    }
    let short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (2.0f64 + 1 as libc::c_int as libc::c_double);
    let long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (30.0f64 + 1 as libc::c_int as libc::c_double);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        sum
            += fabs(
                *input.offset(i as isize)
                    - *input.offset((i - 1 as libc::c_int) as isize),
            );
        i += 1;
        i;
    }
    let mut kama: libc::c_double = *input.offset((period - 1 as libc::c_int) as isize);
    let fresh47 = output;
    output = output.offset(1);
    *fresh47 = kama;
    let mut er: libc::c_double = 0.;
    let mut sc: libc::c_double = 0.;
    i = period;
    while i < size {
        sum
            += fabs(
                *input.offset(i as isize)
                    - *input.offset((i - 1 as libc::c_int) as isize),
            );
        if i > period {
            sum
                -= fabs(
                    *input.offset((i - period) as isize)
                        - *input.offset((i - period - 1 as libc::c_int) as isize),
                );
        }
        if sum != 0.0f64 {
            er = fabs(*input.offset(i as isize) - *input.offset((i - period) as isize))
                / sum;
        } else {
            er = 1.0f64;
        }
        sc = pow(
            er * (short_per - long_per) + long_per,
            2 as libc::c_int as libc::c_double,
        );
        kama = kama + sc * (*input.offset(i as isize) - kama);
        let fresh48 = output;
        output = output.offset(1);
        *fresh48 = kama;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_kama_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_kama_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2630 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_kama(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kvo_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kvo(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_kvo_start(options) {
        return 0 as libc::c_int;
    }
    let short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut cm: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut prev_hlc: libc::c_double = *high.offset(0 as libc::c_int as isize)
        + *low.offset(0 as libc::c_int as isize)
        + *close.offset(0 as libc::c_int as isize);
    let mut trend: libc::c_int = -(1 as libc::c_int);
    let mut short_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let hlc: libc::c_double = *high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize);
        let dm: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if hlc > prev_hlc && trend != 1 as libc::c_int {
            trend = 1 as libc::c_int;
            cm = *high.offset((i - 1 as libc::c_int) as isize)
                - *low.offset((i - 1 as libc::c_int) as isize);
        } else if hlc < prev_hlc && trend != 0 as libc::c_int {
            trend = 0 as libc::c_int;
            cm = *high.offset((i - 1 as libc::c_int) as isize)
                - *low.offset((i - 1 as libc::c_int) as isize);
        }
        cm += dm;
        let vf: libc::c_double = *volume.offset(i as isize)
            * fabs(
                dm / cm * 2 as libc::c_int as libc::c_double
                    - 1 as libc::c_int as libc::c_double,
            ) * 100 as libc::c_int as libc::c_double
            * (if trend != 0 { 1.0f64 } else { -1.0f64 });
        if i == 1 as libc::c_int {
            short_ema = vf;
            long_ema = vf;
        } else {
            short_ema = (vf - short_ema) * short_per + short_ema;
            long_ema = (vf - long_ema) * long_per + long_ema;
        }
        let fresh49 = output;
        output = output.offset(1);
        *fresh49 = short_ema - long_ema;
        prev_hlc = hlc;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_kvo_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_kvo_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2677 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_kvo(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_lag_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_lag(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_lag_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = period;
    while i < size {
        let fresh50 = output;
        output = output.offset(1);
        *fresh50 = *input.offset((i - period) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_lag_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_lag_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2693 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_lag(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_linreg_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_linreg(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_linreg_start(options) {
        return 0 as libc::c_int;
    }
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let p: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        x += (i + 1 as libc::c_int) as libc::c_double;
        x2 += ((i + 1 as libc::c_int) * (i + 1 as libc::c_int)) as libc::c_double;
        xy += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y += *input.offset(i as isize);
        i += 1;
        i;
    }
    x += period as libc::c_double;
    x2 += (period * period) as libc::c_double;
    let bd: libc::c_double = 1.0f64 / (period as libc::c_double * x2 - x * x);
    i = period - 1 as libc::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as libc::c_double;
        y += *input.offset(i as isize);
        let b: libc::c_double = (period as libc::c_double * xy - x * y) * bd;
        let a: libc::c_double = (y - b * x) * p;
        let fresh51 = output;
        output = output.offset(1);
        *fresh51 = a + b * period as libc::c_double;
        xy -= y;
        y -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_linreg_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_linreg_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2706 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_linreg(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_linregintercept_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_linregintercept(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_linregintercept_start(options) {
        return 0 as libc::c_int;
    }
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let p: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        x += (i + 1 as libc::c_int) as libc::c_double;
        x2 += ((i + 1 as libc::c_int) * (i + 1 as libc::c_int)) as libc::c_double;
        xy += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y += *input.offset(i as isize);
        i += 1;
        i;
    }
    x += period as libc::c_double;
    x2 += (period * period) as libc::c_double;
    let bd: libc::c_double = 1.0f64 / (period as libc::c_double * x2 - x * x);
    i = period - 1 as libc::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as libc::c_double;
        y += *input.offset(i as isize);
        let b: libc::c_double = (period as libc::c_double * xy - x * y) * bd;
        let a: libc::c_double = (y - b * x) * p;
        let fresh52 = output;
        output = output.offset(1);
        *fresh52 = a + b * 1 as libc::c_int as libc::c_double;
        xy -= y;
        y -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_linregintercept_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_linregintercept_start(options)\0"
                as *const u8 as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2719 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"int ti_linregintercept(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_linregslope_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_linregslope(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_linregslope_start(options) {
        return 0 as libc::c_int;
    }
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        x += (i + 1 as libc::c_int) as libc::c_double;
        x2 += ((i + 1 as libc::c_int) * (i + 1 as libc::c_int)) as libc::c_double;
        xy += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y += *input.offset(i as isize);
        i += 1;
        i;
    }
    x += period as libc::c_double;
    x2 += (period * period) as libc::c_double;
    let bd: libc::c_double = 1.0f64 / (period as libc::c_double * x2 - x * x);
    i = period - 1 as libc::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as libc::c_double;
        y += *input.offset(i as isize);
        let b: libc::c_double = (period as libc::c_double * xy - x * y) * bd;
        let fresh53 = output;
        output = output.offset(1);
        *fresh53 = b;
        xy -= y;
        y -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_linregslope_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_linregslope_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2732 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[libc::c_char; 80],
            >(
                b"int ti_linregslope(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ln_start(mut options: *const libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ln(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = log(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_log10_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_log10(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = log10(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_macd_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    return long_period - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_macd(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut macd: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut signal: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut hist: *mut libc::c_double = *outputs.offset(2 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let signal_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if signal_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_macd_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut signal_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (signal_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    if short_period == 12 as libc::c_int && long_period == 26 as libc::c_int {
        short_per = 0.15f64;
        long_per = 0.075f64;
    }
    let mut short_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut long_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut signal_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        short_ema = (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema = (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: libc::c_double = short_ema - long_ema;
        if i == long_period - 1 as libc::c_int {
            signal_ema = out;
        }
        if i >= long_period - 1 as libc::c_int {
            signal_ema = (out - signal_ema) * signal_per + signal_ema;
            let fresh54 = macd;
            macd = macd.offset(1);
            *fresh54 = out;
            let fresh55 = signal;
            signal = signal.offset(1);
            *fresh55 = signal_ema;
            let fresh56 = hist;
            hist = hist.offset(1);
            *fresh56 = out - signal_ema;
        }
        i += 1;
        i;
    }
    if macd.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_macd_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"macd - outputs[0] == size - ti_macd_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2779 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_macd(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if signal.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_macd_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"signal - outputs[1] == size - ti_macd_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2780 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_macd(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if hist.offset_from(*outputs.offset(2 as libc::c_int as isize)) as libc::c_long
        == (size - ti_macd_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"hist - outputs[2] == size - ti_macd_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2781 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_macd(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_marketfi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_marketfi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_marketfi_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let fresh57 = output;
        output = output.offset(1);
        *fresh57 = (*high.offset(i as isize) - *low.offset(i as isize))
            / *volume.offset(i as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_marketfi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_marketfi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2799 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"int ti_marketfi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mass_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let mut sum_p: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int - 1 as libc::c_int;
    return 16 as libc::c_int + sum_p;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mass(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_mass_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (9.0f64 + 1 as libc::c_int as libc::c_double);
    let per1: libc::c_double = 1.0f64 - per;
    let mut ema: libc::c_double = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let mut ema2: libc::c_double = ema;
    let mut sum: *mut ti_buffer = ti_buffer_new(period);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let mut hl: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        ema = ema * per1 + hl * per;
        if i == 8 as libc::c_int {
            ema2 = ema;
        }
        if i >= 8 as libc::c_int {
            ema2 = ema2 * per1 + ema * per;
            if i >= 16 as libc::c_int {
                if (*sum).pushes >= (*sum).size {
                    (*sum).sum
                        -= *((*sum).vals).as_mut_ptr().offset((*sum).index as isize);
                }
                (*sum).sum += ema / ema2;
                *((*sum).vals).as_mut_ptr().offset((*sum).index as isize) = ema / ema2;
                (*sum).pushes += 1 as libc::c_int;
                (*sum).index = (*sum).index + 1 as libc::c_int;
                if (*sum).index >= (*sum).size {
                    (*sum).index = 0 as libc::c_int;
                }
                if i >= 16 as libc::c_int + period - 1 as libc::c_int {
                    let fresh58 = output;
                    output = output.offset(1);
                    *fresh58 = (*sum).sum;
                }
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(sum);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_mass_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_mass_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2836 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_mass(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_max_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_max(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_max_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        let mut bar: libc::c_double = *input.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *input.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *input.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        let fresh59 = output;
        output = output.offset(1);
        *fresh59 = max;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_max_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_max_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2870 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_max(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_max_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_max_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = period - 1 as libc::c_int;
    while i < size {
        let mut max: libc::c_double = *input
            .offset((i - period + 1 as libc::c_int) as isize);
        let mut j: libc::c_int = i - period + 2 as libc::c_int;
        while j <= i {
            max = if max > *input.offset(j as isize) {
                max
            } else {
                *input.offset(j as isize)
            };
            j += 1;
            j;
        }
        let fresh60 = output;
        output = output.offset(1);
        *fresh60 = max;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_md_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_md(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_md_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let today: libc::c_double = *input.offset(i as isize);
        sum += today;
        if i >= period {
            sum -= *input.offset((i - period) as isize);
        }
        let avg: libc::c_double = sum * scale;
        if i >= period - 1 as libc::c_int {
            let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
            j = 0 as libc::c_int;
            while j < period {
                acc += fabs(avg - *input.offset((i - j) as isize));
                j += 1;
                j;
            }
            let fresh61 = output;
            output = output.offset(1);
            *fresh61 = acc * scale;
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_md_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_md_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2913 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_md(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_medprice_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_medprice(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = (*high.offset(i as isize) + *low.offset(i as isize)) * 0.5f64;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mfi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mfi(
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
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_mfi_start(options) {
        return 0 as libc::c_int;
    }
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut ytyp: libc::c_double = (*high.offset(0 as libc::c_int as isize)
        + *low.offset(0 as libc::c_int as isize)
        + *close.offset(0 as libc::c_int as isize)) * (1.0f64 / 3.0f64);
    let mut i: libc::c_int = 0;
    let mut up: *mut ti_buffer = ti_buffer_new(period);
    let mut down: *mut ti_buffer = ti_buffer_new(period);
    i = 1 as libc::c_int;
    while i < size {
        let typ: libc::c_double = (*high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        let bar: libc::c_double = typ * *volume.offset(i as isize);
        if typ > ytyp {
            if (*up).pushes >= (*up).size {
                (*up).sum -= *((*up).vals).as_mut_ptr().offset((*up).index as isize);
            }
            (*up).sum += bar;
            *((*up).vals).as_mut_ptr().offset((*up).index as isize) = bar;
            (*up).pushes += 1 as libc::c_int;
            (*up).index = (*up).index + 1 as libc::c_int;
            if (*up).index >= (*up).size {
                (*up).index = 0 as libc::c_int;
            }
            if (*down).pushes >= (*down).size {
                (*down).sum
                    -= *((*down).vals).as_mut_ptr().offset((*down).index as isize);
            }
            (*down).sum += 0.0f64;
            *((*down).vals).as_mut_ptr().offset((*down).index as isize) = 0.0f64;
            (*down).pushes += 1 as libc::c_int;
            (*down).index = (*down).index + 1 as libc::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as libc::c_int;
            }
        } else if typ < ytyp {
            if (*down).pushes >= (*down).size {
                (*down).sum
                    -= *((*down).vals).as_mut_ptr().offset((*down).index as isize);
            }
            (*down).sum += bar;
            *((*down).vals).as_mut_ptr().offset((*down).index as isize) = bar;
            (*down).pushes += 1 as libc::c_int;
            (*down).index = (*down).index + 1 as libc::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as libc::c_int;
            }
            if (*up).pushes >= (*up).size {
                (*up).sum -= *((*up).vals).as_mut_ptr().offset((*up).index as isize);
            }
            (*up).sum += 0.0f64;
            *((*up).vals).as_mut_ptr().offset((*up).index as isize) = 0.0f64;
            (*up).pushes += 1 as libc::c_int;
            (*up).index = (*up).index + 1 as libc::c_int;
            if (*up).index >= (*up).size {
                (*up).index = 0 as libc::c_int;
            }
        } else {
            if (*up).pushes >= (*up).size {
                (*up).sum -= *((*up).vals).as_mut_ptr().offset((*up).index as isize);
            }
            (*up).sum += 0.0f64;
            *((*up).vals).as_mut_ptr().offset((*up).index as isize) = 0.0f64;
            (*up).pushes += 1 as libc::c_int;
            (*up).index = (*up).index + 1 as libc::c_int;
            if (*up).index >= (*up).size {
                (*up).index = 0 as libc::c_int;
            }
            if (*down).pushes >= (*down).size {
                (*down).sum
                    -= *((*down).vals).as_mut_ptr().offset((*down).index as isize);
            }
            (*down).sum += 0.0f64;
            *((*down).vals).as_mut_ptr().offset((*down).index as isize) = 0.0f64;
            (*down).pushes += 1 as libc::c_int;
            (*down).index = (*down).index + 1 as libc::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as libc::c_int;
            }
        }
        ytyp = typ;
        if i >= period {
            let fresh62 = output;
            output = output.offset(1);
            *fresh62 = (*up).sum / ((*up).sum + (*down).sum) * 100.0f64;
        }
        i += 1;
        i;
    }
    ti_buffer_free(up);
    ti_buffer_free(down);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_mfi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_mfi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            2967 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_mfi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_min_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_min(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_min_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut min: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        let mut bar: libc::c_double = *input.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *input.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *input.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let fresh63 = output;
        output = output.offset(1);
        *fresh63 = min;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_min_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_min_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3001 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_min(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_min_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_min_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = period - 1 as libc::c_int;
    while i < size {
        let mut min: libc::c_double = *input
            .offset((i - period + 1 as libc::c_int) as isize);
        let mut j: libc::c_int = i - period + 2 as libc::c_int;
        while j <= i {
            min = if min < *input.offset(j as isize) {
                min
            } else {
                *input.offset(j as isize)
            };
            j += 1;
            j;
        }
        let fresh64 = output;
        output = output.offset(1);
        *fresh64 = min;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_min_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_min_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3017 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_min_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mom_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mom(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_mom_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = period;
    while i < size {
        let fresh65 = output;
        output = output.offset(1);
        *fresh65 = *input.offset(i as isize) - *input.offset((i - period) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_mom_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_mom_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3033 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_mom(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_msw_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_msw(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut sine: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut lead: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_msw_start(options) {
        return 0 as libc::c_int;
    }
    let pi: libc::c_double = std::f64::consts::PI;
    let tpi: libc::c_double = 2 as libc::c_int as libc::c_double * pi;
    let mut weight: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut phase: libc::c_double = 0.;
    let mut rp: libc::c_double = 0.;
    let mut ip: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period;
    while i < size {
        rp = 0 as libc::c_int as libc::c_double;
        ip = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < period {
            weight = *input.offset((i - j) as isize);
            rp = rp + cos(tpi * j as libc::c_double / period as libc::c_double) * weight;
            ip = ip + sin(tpi * j as libc::c_double / period as libc::c_double) * weight;
            j += 1;
            j;
        }
        if fabs(rp) > 0.001f64 {
            phase = atan(ip / rp);
        } else {
            phase = tpi / 2.0f64
                * (if ip < 0 as libc::c_int as libc::c_double {
                    -1.0f64
                } else {
                    1.0f64
                });
        }
        if rp < 0.0f64 {
            phase += pi;
        }
        phase += pi / 2.0f64;
        if phase < 0.0f64 {
            phase += tpi;
        }
        if phase > tpi {
            phase -= tpi;
        }
        let fresh66 = sine;
        sine = sine.offset(1);
        *fresh66 = sin(phase);
        let fresh67 = lead;
        lead = lead.offset(1);
        *fresh67 = sin(phase + pi / 4.0f64);
        i += 1;
        i;
    }
    if sine.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_msw_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"sine - outputs[0] == size - ti_msw_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3071 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_msw(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if lead.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_msw_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"lead - outputs[1] == size - ti_msw_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3072 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_msw(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mul_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mul(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut in2: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = *in1.offset(i as isize) * *in2.offset(i as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_natr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_natr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_natr_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut truerange: libc::c_double = 0.;
    sum
        += *high.offset(0 as libc::c_int as isize)
            - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let l: libc::c_double = *low.offset(i as isize);
        let h: libc::c_double = *high.offset(i as isize);
        let c: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
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
        sum += truerange;
        i += 1;
        i;
    }
    let mut val: libc::c_double = sum / period as libc::c_double;
    let fresh68 = output;
    output = output.offset(1);
    *fresh68 = 100 as libc::c_int as libc::c_double * val
        / *close.offset((period - 1 as libc::c_int) as isize);
    i = period;
    while i < size {
        let l_0: libc::c_double = *low.offset(i as isize);
        let h_0: libc::c_double = *high.offset(i as isize);
        let c_0: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
        let ych_0: libc::c_double = fabs(h_0 - c_0);
        let ycl_0: libc::c_double = fabs(l_0 - c_0);
        let mut v_0: libc::c_double = h_0 - l_0;
        if ych_0 > v_0 {
            v_0 = ych_0;
        }
        if ycl_0 > v_0 {
            v_0 = ycl_0;
        }
        truerange = v_0;
        val = (truerange - val) * per + val;
        let fresh69 = output;
        output = output.offset(1);
        *fresh69 = 100 as libc::c_int as libc::c_double * val
            / *close.offset(i as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_natr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_natr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_natr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_nvi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_nvi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_nvi_start(options) {
        return 0 as libc::c_int;
    }
    let mut nvi: libc::c_double = 1000 as libc::c_int as libc::c_double;
    let fresh70 = output;
    output = output.offset(1);
    *fresh70 = nvi;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        if *volume.offset(i as isize) < *volume.offset((i - 1 as libc::c_int) as isize) {
            nvi
                += (*close.offset(i as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize))
                    / *close.offset((i - 1 as libc::c_int) as isize) * nvi;
        }
        let fresh71 = output;
        output = output.offset(1);
        *fresh71 = nvi;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_nvi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_nvi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3125 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_nvi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_obv_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_obv(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let fresh72 = output;
    output = output.offset(1);
    *fresh72 = sum;
    let mut prev: libc::c_double = *close.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        if *close.offset(i as isize) > prev {
            sum += *volume.offset(i as isize);
        } else if *close.offset(i as isize) < prev {
            sum -= *volume.offset(i as isize);
        }
        prev = *close.offset(i as isize);
        let fresh73 = output;
        output = output.offset(1);
        *fresh73 = sum;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ppo_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ppo(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut ppo: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_ppo_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut short_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut long_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        short_ema = (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema = (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: libc::c_double = 100.0f64 * (short_ema - long_ema) / long_ema;
        let fresh74 = ppo;
        ppo = ppo.offset(1);
        *fresh74 = out;
        i += 1;
        i;
    }
    if ppo.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ppo_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"ppo - outputs[0] == size - ti_ppo_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3177 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_ppo(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_psar_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_psar(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let accel_step: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let accel_max: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if accel_step <= 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if accel_max <= accel_step {
        return 1 as libc::c_int;
    }
    if size < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut lng: libc::c_int = 0;
    if *high.offset(0 as libc::c_int as isize) + *low.offset(0 as libc::c_int as isize)
        <= *high.offset(1 as libc::c_int as isize)
            + *low.offset(1 as libc::c_int as isize)
    {
        lng = 1 as libc::c_int;
    } else {
        lng = 0 as libc::c_int;
    }
    let mut sar: libc::c_double = 0.;
    let mut extreme: libc::c_double = 0.;
    if lng != 0 {
        extreme = *high.offset(0 as libc::c_int as isize);
        sar = *low.offset(0 as libc::c_int as isize);
    } else {
        extreme = *low.offset(0 as libc::c_int as isize);
        sar = *high.offset(0 as libc::c_int as isize);
    }
    let mut accel: libc::c_double = accel_step;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        sar = (extreme - sar) * accel + sar;
        if lng != 0 {
            if i >= 2 as libc::c_int
                && sar > *low.offset((i - 2 as libc::c_int) as isize)
            {
                sar = *low.offset((i - 2 as libc::c_int) as isize);
            }
            if sar > *low.offset((i - 1 as libc::c_int) as isize) {
                sar = *low.offset((i - 1 as libc::c_int) as isize);
            }
            if accel < accel_max && *high.offset(i as isize) > extreme {
                accel += accel_step;
                if accel > accel_max {
                    accel = accel_max;
                }
            }
            if *high.offset(i as isize) > extreme {
                extreme = *high.offset(i as isize);
            }
        } else {
            if i >= 2 as libc::c_int
                && sar < *high.offset((i - 2 as libc::c_int) as isize)
            {
                sar = *high.offset((i - 2 as libc::c_int) as isize);
            }
            if sar < *high.offset((i - 1 as libc::c_int) as isize) {
                sar = *high.offset((i - 1 as libc::c_int) as isize);
            }
            if accel < accel_max && *low.offset(i as isize) < extreme {
                accel += accel_step;
                if accel > accel_max {
                    accel = accel_max;
                }
            }
            if *low.offset(i as isize) < extreme {
                extreme = *low.offset(i as isize);
            }
        }
        if lng != 0 && *low.offset(i as isize) < sar
            || lng == 0 && *high.offset(i as isize) > sar
        {
            accel = accel_step;
            sar = extreme;
            lng = (lng == 0) as libc::c_int;
            if lng == 0 {
                extreme = *low.offset(i as isize);
            } else {
                extreme = *high.offset(i as isize);
            }
        }
        let fresh75 = output;
        output = output.offset(1);
        *fresh75 = sar;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_psar_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_psar_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3236 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_psar(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pvi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pvi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_pvi_start(options) {
        return 0 as libc::c_int;
    }
    let mut pvi: libc::c_double = 1000 as libc::c_int as libc::c_double;
    let fresh76 = output;
    output = output.offset(1);
    *fresh76 = pvi;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        if *volume.offset(i as isize) > *volume.offset((i - 1 as libc::c_int) as isize) {
            pvi
                += (*close.offset(i as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize))
                    / *close.offset((i - 1 as libc::c_int) as isize) * pvi;
        }
        let fresh77 = output;
        output = output.offset(1);
        *fresh77 = pvi;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_pvi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_pvi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3258 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_pvi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_qstick_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_qstick(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut open: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_qstick_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *close.offset(i as isize) - *open.offset(i as isize);
        i += 1;
        i;
    }
    let fresh78 = output;
    output = output.offset(1);
    *fresh78 = sum * scale;
    i = period;
    while i < size {
        sum += *close.offset(i as isize) - *open.offset(i as isize);
        sum
            -= *close.offset((i - period) as isize)
                - *open.offset((i - period) as isize);
        let fresh79 = output;
        output = output.offset(1);
        *fresh79 = sum * scale;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_qstick_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_qstick_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3283 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_qstick(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_roc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_roc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_roc_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = period;
    while i < size {
        let fresh80 = output;
        output = output.offset(1);
        *fresh80 = (*input.offset(i as isize) - *input.offset((i - period) as isize))
            / *input.offset((i - period) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_roc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_roc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3299 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_roc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rocr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rocr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rocr_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = period;
    while i < size {
        let fresh81 = output;
        output = output.offset(1);
        *fresh81 = *input.offset(i as isize) / *input.offset((i - period) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_rocr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_rocr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3315 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_rocr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_round_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_round(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = floor(*in1.offset(i as isize) + 0.5f64);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rsi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_rsi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rsi_start(options) {
        return 0 as libc::c_int;
    }
    let mut smooth_up: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut smooth_down: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        let upward: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up += upward;
        smooth_down += downward;
        i += 1;
        i;
    }
    smooth_up /= period as libc::c_double;
    smooth_down /= period as libc::c_double;
    let fresh82 = output;
    output = output.offset(1);
    *fresh82 = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
    i = period + 1 as libc::c_int;
    while i < size {
        let upward_0: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward_0: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up = (upward_0 - smooth_up) * per + smooth_up;
        smooth_down = (downward_0 - smooth_down) * per + smooth_down;
        let fresh83 = output;
        output = output.offset(1);
        *fresh83 = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_rsi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_rsi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3347 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_rsi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sin_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sin(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = sin(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sinh_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sinh(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = sinh(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_sma_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let fresh84 = output;
    output = output.offset(1);
    *fresh84 = sum * scale;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        let fresh85 = output;
        output = output.offset(1);
        *fresh85 = sum * scale;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_sma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_sma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3373 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_sma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sma_stream_new(
    mut options: *const libc::c_double,
    mut stream_in: *mut *mut ti_stream,
) -> libc::c_int {
    let mut stream: *mut *mut ti_stream_sma = stream_in as *mut *mut ti_stream_sma;
    let mut period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *stream = malloc(
        (::core::mem::size_of::<ti_stream_sma>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(period as libc::c_ulong),
            ),
    ) as *mut ti_stream_sma;
    if stream.is_null() {
        return 2 as libc::c_int;
    }
    (**stream).index = 72 as libc::c_int;
    (**stream).progress = -ti_sma_start(options);
    (**stream).period = period;
    (**stream).per = 1.0f64 / period as libc::c_double;
    (**stream).sum = 0.0f64;
    (**stream).buffer_idx = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sma_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_sma = stream_in as *mut ti_stream_sma;
    let mut progress: libc::c_int = (*stream).progress;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut period: libc::c_int = (*stream).period;
    let mut sma: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut per: libc::c_double = (*stream).per;
    let mut sum: libc::c_double = (*stream).sum;
    let mut buffer_idx: libc::c_int = (*stream).buffer_idx;
    let mut buffer: *mut libc::c_double = ((*stream).buffer).as_mut_ptr();
    progress == -period + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while progress < 1 as libc::c_int && i < size {
        buffer_idx += 1;
        buffer_idx;
        let ref mut fresh86 = *buffer.offset(buffer_idx as isize);
        *fresh86 = *real.offset(i as isize) * per;
        sum += *fresh86;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        let fresh87 = sma;
        sma = sma.offset(1);
        *fresh87 = sum;
    }
    while i < size {
        buffer_idx += 1;
        buffer_idx;
        if buffer_idx == period {
            buffer_idx = 0 as libc::c_int;
        }
        sum -= *buffer.offset(buffer_idx as isize);
        let ref mut fresh88 = *buffer.offset(buffer_idx as isize);
        *fresh88 = *real.offset(i as isize) * per;
        sum += *fresh88;
        let fresh89 = sma;
        sma = sma.offset(1);
        *fresh89 = sum;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).sum = sum;
    (*stream).buffer_idx = buffer_idx;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sma_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_sqrt_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sqrt(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = sqrt(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stddev_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stddev(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stddev_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1;
        i;
    }
    let mut s2s2: libc::c_double = sum2 * scale - sum * scale * (sum * scale);
    if s2s2 > 0.0f64 {
        s2s2 = sqrt(s2s2);
    }
    let fresh90 = output;
    output = output.offset(1);
    *fresh90 = s2s2;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2
            -= *input.offset((i - period) as isize)
                * *input.offset((i - period) as isize);
        let mut s2s2_0: libc::c_double = sum2 * scale - sum * scale * (sum * scale);
        if s2s2_0 > 0.0f64 {
            s2s2_0 = sqrt(s2s2_0);
        }
        let fresh91 = output;
        output = output.offset(1);
        *fresh91 = s2s2_0;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stddev_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_stddev_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3463 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_stddev(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stderr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stderr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stderr_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mul: libc::c_double = 1.0f64 / sqrt(period as libc::c_double);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1;
        i;
    }
    let mut s2s2: libc::c_double = sum2 * scale - sum * scale * (sum * scale);
    if s2s2 > 0.0f64 {
        s2s2 = sqrt(s2s2);
    }
    let fresh92 = output;
    output = output.offset(1);
    *fresh92 = mul * s2s2;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2
            -= *input.offset((i - period) as isize)
                * *input.offset((i - period) as isize);
        let mut s2s2_0: libc::c_double = sum2 * scale - sum * scale * (sum * scale);
        if s2s2_0 > 0.0f64 {
            s2s2_0 = sqrt(s2s2_0);
        }
        let fresh93 = output;
        output = output.offset(1);
        *fresh93 = mul * s2s2_0;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stderr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_stderr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3498 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_stderr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stoch_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let kperiod: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let kslow: libc::c_int = *options.offset(1 as libc::c_int as isize) as libc::c_int;
    let dperiod: libc::c_int = *options.offset(2 as libc::c_int as isize) as libc::c_int;
    return kperiod + kslow + dperiod - 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stoch(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let kperiod: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let kslow: libc::c_int = *options.offset(1 as libc::c_int as isize) as libc::c_int;
    let dperiod: libc::c_int = *options.offset(2 as libc::c_int as isize) as libc::c_int;
    let kper: libc::c_double = 1.0f64 / kslow as libc::c_double;
    let dper: libc::c_double = 1.0f64 / dperiod as libc::c_double;
    let mut stoch: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut stoch_ma: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if kperiod < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if kslow < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if dperiod < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stoch_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut k_sum: *mut ti_buffer = ti_buffer_new(kslow);
    let mut d_sum: *mut ti_buffer = ti_buffer_new(dperiod);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if i >= kperiod {
            trail += 1;
            trail;
        }
        bar = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *high.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *low.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let kdiff: libc::c_double = max - min;
        let kfast: libc::c_double = if kdiff == 0.0f64 {
            0.0f64
        } else {
            100 as libc::c_int as libc::c_double
                * ((*close.offset(i as isize) - min) / kdiff)
        };
        if (*k_sum).pushes >= (*k_sum).size {
            (*k_sum).sum
                -= *((*k_sum).vals).as_mut_ptr().offset((*k_sum).index as isize);
        }
        (*k_sum).sum += kfast;
        *((*k_sum).vals).as_mut_ptr().offset((*k_sum).index as isize) = kfast;
        (*k_sum).pushes += 1 as libc::c_int;
        (*k_sum).index = (*k_sum).index + 1 as libc::c_int;
        if (*k_sum).index >= (*k_sum).size {
            (*k_sum).index = 0 as libc::c_int;
        }
        if i >= kperiod - 1 as libc::c_int + kslow - 1 as libc::c_int {
            let k: libc::c_double = (*k_sum).sum * kper;
            if (*d_sum).pushes >= (*d_sum).size {
                (*d_sum).sum
                    -= *((*d_sum).vals).as_mut_ptr().offset((*d_sum).index as isize);
            }
            (*d_sum).sum += k;
            *((*d_sum).vals).as_mut_ptr().offset((*d_sum).index as isize) = k;
            (*d_sum).pushes += 1 as libc::c_int;
            (*d_sum).index = (*d_sum).index + 1 as libc::c_int;
            if (*d_sum).index >= (*d_sum).size {
                (*d_sum).index = 0 as libc::c_int;
            }
            if i
                >= kperiod - 1 as libc::c_int + kslow - 1 as libc::c_int + dperiod
                    - 1 as libc::c_int
            {
                let fresh94 = stoch;
                stoch = stoch.offset(1);
                *fresh94 = k;
                let fresh95 = stoch_ma;
                stoch_ma = stoch_ma.offset(1);
                *fresh95 = (*d_sum).sum * dper;
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(k_sum);
    ti_buffer_free(d_sum);
    if stoch.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stoch_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"stoch - outputs[0] == size - ti_stoch_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3577 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_stoch(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if stoch_ma.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stoch_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"stoch_ma - outputs[1] == size - ti_stoch_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3578 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_stoch(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stochrsi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int * 2 as libc::c_int
        - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stochrsi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stochrsi_start(options) {
        return 0 as libc::c_int;
    }
    let mut rsi: *mut ti_buffer = ti_buffer_new(period);
    let mut smooth_up: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut smooth_down: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        let upward: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up += upward;
        smooth_down += downward;
        i += 1;
        i;
    }
    smooth_up /= period as libc::c_double;
    smooth_down /= period as libc::c_double;
    let mut r: libc::c_double = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
    if (*rsi).pushes >= (*rsi).size {
        (*rsi).sum -= *((*rsi).vals).as_mut_ptr().offset((*rsi).index as isize);
    }
    (*rsi).sum += r;
    *((*rsi).vals).as_mut_ptr().offset((*rsi).index as isize) = r;
    (*rsi).pushes += 1 as libc::c_int;
    (*rsi).index = (*rsi).index + 1 as libc::c_int;
    if (*rsi).index >= (*rsi).size {
        (*rsi).index = 0 as libc::c_int;
    }
    let mut min: libc::c_double = r;
    let mut max: libc::c_double = r;
    let mut mini: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = 0 as libc::c_int;
    i = period + 1 as libc::c_int;
    while i < size {
        let upward_0: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward_0: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up = (upward_0 - smooth_up) * per + smooth_up;
        smooth_down = (downward_0 - smooth_down) * per + smooth_down;
        r = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        if r > max {
            max = r;
            maxi = (*rsi).index;
        } else if maxi == (*rsi).index {
            max = r;
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < (*rsi).size {
                if !(j == (*rsi).index) {
                    if *((*rsi).vals).as_mut_ptr().offset(j as isize) > max {
                        max = *((*rsi).vals).as_mut_ptr().offset(j as isize);
                        maxi = j;
                    }
                }
                j += 1;
                j;
            }
        }
        if r < min {
            min = r;
            mini = (*rsi).index;
        } else if mini == (*rsi).index {
            min = r;
            let mut j_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int;
            while j_0 < (*rsi).size {
                if !(j_0 == (*rsi).index) {
                    if *((*rsi).vals).as_mut_ptr().offset(j_0 as isize) < min {
                        min = *((*rsi).vals).as_mut_ptr().offset(j_0 as isize);
                        mini = j_0;
                    }
                }
                j_0 += 1;
                j_0;
            }
        }
        *((*rsi).vals).as_mut_ptr().offset((*rsi).index as isize) = r;
        (*rsi).index = (*rsi).index + 1 as libc::c_int;
        if (*rsi).index >= (*rsi).size {
            (*rsi).index = 0 as libc::c_int;
        }
        if i > period * 2 as libc::c_int - 2 as libc::c_int {
            let diff: libc::c_double = max - min;
            if diff == 0.0f64 {
                let fresh96 = output;
                output = output.offset(1);
                *fresh96 = 0.0f64;
            } else {
                let fresh97 = output;
                output = output.offset(1);
                *fresh97 = (r - min) / diff;
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(rsi);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stochrsi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_stochrsi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3653 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"int ti_stochrsi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sub_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sub(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut in2: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = *in1.offset(i as isize) - *in2.offset(i as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sum_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_sum(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_sum_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let fresh98 = output;
    output = output.offset(1);
    *fresh98 = sum;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        let fresh99 = output;
        output = output.offset(1);
        *fresh99 = sum;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_sum_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_sum_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3677 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_sum(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tan_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tan(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = tan(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tanh_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tanh(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output.offset(i as isize) = tanh(*in1.offset(i as isize));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tema_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tema(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_tema_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let per1: libc::c_double = 1.0f64 - per;
    let mut ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut ema2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ema3: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        ema = ema * per1 + *input.offset(i as isize) * per;
        if i == period - 1 as libc::c_int {
            ema2 = ema;
        }
        if i >= period - 1 as libc::c_int {
            ema2 = ema2 * per1 + ema * per;
            if i == (period - 1 as libc::c_int) * 2 as libc::c_int {
                ema3 = ema2;
            }
            if i >= (period - 1 as libc::c_int) * 2 as libc::c_int {
                ema3 = ema3 * per1 + ema2 * per;
                if i >= (period - 1 as libc::c_int) * 3 as libc::c_int {
                    *output = 3 as libc::c_int as libc::c_double * ema
                        - 3 as libc::c_int as libc::c_double * ema2 + ema3;
                    output = output.offset(1);
                    output;
                }
            }
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_tema_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_tema_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3717 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_tema(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_todeg_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_todeg(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = *in1.offset(i as isize) * (180.0f64 / 3.14159265358979323846f64);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_torad_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_torad(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = *in1.offset(i as isize) * (3.14159265358979323846f64 / 180.0f64);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tr_start(mut options: *const libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut truerange: libc::c_double = 0.;
    *output
        .offset(
            0 as libc::c_int as isize,
        ) = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let l: libc::c_double = *low.offset(i as isize);
        let h: libc::c_double = *high.offset(i as isize);
        let c: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
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
        *output.offset(i as isize) = truerange;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trima_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trima(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_trima_start(options) {
        return 0 as libc::c_int;
    }
    if period <= 2 as libc::c_int {
        return ti_sma(size, inputs, options, outputs);
    }
    let mut weights: libc::c_double = 1 as libc::c_int as libc::c_double
        / (if period % 2 as libc::c_int != 0 {
            (period / 2 as libc::c_int + 1 as libc::c_int)
                * (period / 2 as libc::c_int + 1 as libc::c_int)
        } else {
            (period / 2 as libc::c_int + 1 as libc::c_int) * (period / 2 as libc::c_int)
        }) as libc::c_double;
    let mut weight_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lead_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut trail_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let lead_period: libc::c_int = if period % 2 as libc::c_int != 0 {
        period / 2 as libc::c_int
    } else {
        period / 2 as libc::c_int - 1 as libc::c_int
    };
    let trail_period: libc::c_int = lead_period + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        weight_sum += *input.offset(i as isize) * w as libc::c_double;
        if i + 1 as libc::c_int > period - lead_period {
            lead_sum += *input.offset(i as isize);
        }
        if i + 1 as libc::c_int <= trail_period {
            trail_sum += *input.offset(i as isize);
        }
        if (i + 1 as libc::c_int) < trail_period {
            w += 1;
            w;
        }
        if i + 1 as libc::c_int >= period - lead_period {
            w -= 1;
            w;
        }
        i += 1;
        i;
    }
    let mut lsi: libc::c_int = period - 1 as libc::c_int - lead_period
        + 1 as libc::c_int;
    let mut tsi1: libc::c_int = period - 1 as libc::c_int - period + 1 as libc::c_int
        + trail_period;
    let mut tsi2: libc::c_int = period - 1 as libc::c_int - period + 1 as libc::c_int;
    i = period - 1 as libc::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize);
        let fresh100 = output;
        output = output.offset(1);
        *fresh100 = weight_sum * weights;
        lead_sum += *input.offset(i as isize);
        weight_sum += lead_sum;
        weight_sum -= trail_sum;
        let fresh101 = lsi;
        lsi = lsi + 1;
        lead_sum -= *input.offset(fresh101 as isize);
        let fresh102 = tsi1;
        tsi1 = tsi1 + 1;
        trail_sum += *input.offset(fresh102 as isize);
        let fresh103 = tsi2;
        tsi2 = tsi2 + 1;
        trail_sum -= *input.offset(fresh103 as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_trima_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_trima_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3780 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_trima(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trix_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 3 as libc::c_int + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trix(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_trix_start(options) {
        return 0 as libc::c_int;
    }
    let start: libc::c_int = period * 3 as libc::c_int - 2 as libc::c_int;
    if start == ti_trix_start(options) {} else {
        __assert_fail(
            b"start == ti_trix_start(options)\0" as *const u8 as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3794 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_trix(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut ema1: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut ema2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ema3: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < start {
        ema1 = (*input.offset(i as isize) - ema1) * per + ema1;
        if i == period - 1 as libc::c_int {
            ema2 = ema1;
        } else if i > period - 1 as libc::c_int {
            ema2 = (ema1 - ema2) * per + ema2;
            if i == period * 2 as libc::c_int - 2 as libc::c_int {
                ema3 = ema2;
            } else if i > period * 2 as libc::c_int - 2 as libc::c_int {
                ema3 = (ema2 - ema3) * per + ema3;
            }
        }
        i += 1;
        i;
    }
    i = start;
    while i < size {
        ema1 = (*input.offset(i as isize) - ema1) * per + ema1;
        ema2 = (ema1 - ema2) * per + ema2;
        let last: libc::c_double = ema3;
        ema3 = (ema2 - ema3) * per + ema3;
        let fresh104 = output;
        output = output.offset(1);
        *fresh104 = (ema3 - last) / ema3 * 100.0f64;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_trix_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_trix_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3819 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_trix(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trunc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trunc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in1: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = *in1.offset(i as isize) as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsf_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsf(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_tsf_start(options) {
        return 0 as libc::c_int;
    }
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let p: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        x += (i + 1 as libc::c_int) as libc::c_double;
        x2 += ((i + 1 as libc::c_int) * (i + 1 as libc::c_int)) as libc::c_double;
        xy += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y += *input.offset(i as isize);
        i += 1;
        i;
    }
    x += period as libc::c_double;
    x2 += (period * period) as libc::c_double;
    let bd: libc::c_double = 1.0f64 / (period as libc::c_double * x2 - x * x);
    i = period - 1 as libc::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as libc::c_double;
        y += *input.offset(i as isize);
        let b: libc::c_double = (period as libc::c_double * xy - x * y) * bd;
        let a: libc::c_double = (y - b * x) * p;
        let fresh105 = output;
        output = output.offset(1);
        *fresh105 = a + b * (period + 1 as libc::c_int) as libc::c_double;
        xy -= y;
        y -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_tsf_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_tsf_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3833 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_tsf(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_typprice_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_typprice(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = (*high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ultosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(2 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ultosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let medium_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if medium_period < short_period {
        return 1 as libc::c_int;
    }
    if long_period < medium_period {
        return 1 as libc::c_int;
    }
    if size <= ti_ultosc_start(options) {
        return 0 as libc::c_int;
    }
    let mut bp_buf: *mut ti_buffer = ti_buffer_new(long_period);
    let mut r_buf: *mut ti_buffer = ti_buffer_new(long_period);
    let mut bp_short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut bp_medium_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut r_short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut r_medium_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let true_low: libc::c_double = if *low.offset(i as isize)
            < *close.offset((i - 1 as libc::c_int) as isize)
        {
            *low.offset(i as isize)
        } else {
            *close.offset((i - 1 as libc::c_int) as isize)
        };
        let true_high: libc::c_double = if *high.offset(i as isize)
            > *close.offset((i - 1 as libc::c_int) as isize)
        {
            *high.offset(i as isize)
        } else {
            *close.offset((i - 1 as libc::c_int) as isize)
        };
        let bp: libc::c_double = *close.offset(i as isize) - true_low;
        let r: libc::c_double = true_high - true_low;
        bp_short_sum += bp;
        bp_medium_sum += bp;
        r_short_sum += r;
        r_medium_sum += r;
        if (*bp_buf).pushes >= (*bp_buf).size {
            (*bp_buf).sum
                -= *((*bp_buf).vals).as_mut_ptr().offset((*bp_buf).index as isize);
        }
        (*bp_buf).sum += bp;
        *((*bp_buf).vals).as_mut_ptr().offset((*bp_buf).index as isize) = bp;
        (*bp_buf).pushes += 1 as libc::c_int;
        (*bp_buf).index = (*bp_buf).index + 1 as libc::c_int;
        if (*bp_buf).index >= (*bp_buf).size {
            (*bp_buf).index = 0 as libc::c_int;
        }
        if (*r_buf).pushes >= (*r_buf).size {
            (*r_buf).sum
                -= *((*r_buf).vals).as_mut_ptr().offset((*r_buf).index as isize);
        }
        (*r_buf).sum += r;
        *((*r_buf).vals).as_mut_ptr().offset((*r_buf).index as isize) = r;
        (*r_buf).pushes += 1 as libc::c_int;
        (*r_buf).index = (*r_buf).index + 1 as libc::c_int;
        if (*r_buf).index >= (*r_buf).size {
            (*r_buf).index = 0 as libc::c_int;
        }
        if i > short_period {
            let mut short_index: libc::c_int = (*bp_buf).index - short_period
                - 1 as libc::c_int;
            if short_index < 0 as libc::c_int {
                short_index += long_period;
            }
            bp_short_sum -= *((*bp_buf).vals).as_mut_ptr().offset(short_index as isize);
            r_short_sum -= *((*r_buf).vals).as_mut_ptr().offset(short_index as isize);
            if i > medium_period {
                let mut medium_index: libc::c_int = (*bp_buf).index - medium_period
                    - 1 as libc::c_int;
                if medium_index < 0 as libc::c_int {
                    medium_index += long_period;
                }
                bp_medium_sum
                    -= *((*bp_buf).vals).as_mut_ptr().offset(medium_index as isize);
                r_medium_sum
                    -= *((*r_buf).vals).as_mut_ptr().offset(medium_index as isize);
            }
        }
        if i >= long_period {
            let first: libc::c_double = 4 as libc::c_int as libc::c_double * bp_short_sum
                / r_short_sum;
            let second: libc::c_double = 2 as libc::c_int as libc::c_double
                * bp_medium_sum / r_medium_sum;
            let third: libc::c_double = 1 as libc::c_int as libc::c_double
                * (*bp_buf).sum / (*r_buf).sum;
            let ult: libc::c_double = (first + second + third) * 100.0f64 / 7.0f64;
            let fresh106 = output;
            output = output.offset(1);
            *fresh106 = ult;
        }
        i += 1;
        i;
    }
    ti_buffer_free(bp_buf);
    ti_buffer_free(r_buf);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ultosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_ultosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3905 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_ultosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_var_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_var(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_var_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1;
        i;
    }
    let fresh107 = output;
    output = output.offset(1);
    *fresh107 = sum2 * scale - sum * scale * (sum * scale);
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2
            -= *input.offset((i - period) as isize)
                * *input.offset((i - period) as isize);
        let fresh108 = output;
        output = output.offset(1);
        *fresh108 = sum2 * scale - sum * scale * (sum * scale);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_var_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_var_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3933 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_var(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vhf_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vhf(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in_0: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_vhf_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 1 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *in_0.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *in_0.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut yc: libc::c_double = *in_0.offset(0 as libc::c_int as isize);
    let mut c: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i < period {
        c = *in_0.offset(i as isize);
        sum += fabs(c - yc);
        yc = c;
        i += 1;
        i;
    }
    i = period;
    while i < size {
        c = *in_0.offset(i as isize);
        sum += fabs(c - yc);
        yc = c;
        if i > period {
            sum
                -= fabs(
                    *in_0.offset((i - period) as isize)
                        - *in_0.offset((i - period - 1 as libc::c_int) as isize),
                );
        }
        bar = c;
        if maxi < trail {
            maxi = trail;
            max = *in_0.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *in_0.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = c;
        if mini < trail {
            mini = trail;
            min = *in_0.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *in_0.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let fresh109 = output;
        output = output.offset(1);
        *fresh109 = fabs(max - min) / sum;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vhf_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vhf_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            3998 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_vhf(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vidya_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(1 as libc::c_int as isize) as libc::c_int - 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vidya(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let alpha: libc::c_double = *options.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_div: libc::c_double = 1.0f64 / short_period as libc::c_double;
    let long_div: libc::c_double = 1.0f64 / long_period as libc::c_double;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if alpha < 0.0f64 || alpha > 1.0f64 {
        return 1 as libc::c_int;
    }
    if size <= ti_vidya_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut short_sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < long_period {
        long_sum += *input.offset(i as isize);
        long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        if i >= long_period - short_period {
            short_sum += *input.offset(i as isize);
            short_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        }
        i += 1;
        i;
    }
    let mut val: libc::c_double = *input
        .offset((long_period - 2 as libc::c_int) as isize);
    let fresh110 = output;
    output = output.offset(1);
    *fresh110 = val;
    if (long_period - 1 as libc::c_int) < size {
        let mut short_stddev: libc::c_double = sqrt(
            short_sum2 * short_div - short_sum * short_div * (short_sum * short_div),
        );
        let mut long_stddev: libc::c_double = sqrt(
            long_sum2 * long_div - long_sum * long_div * (long_sum * long_div),
        );
        let mut k: libc::c_double = short_stddev / long_stddev;
        // if k != k {
        //     k = 0 as libc::c_int as libc::c_double;
        // }
        k *= alpha;
        val = (*input.offset((long_period - 1 as libc::c_int) as isize) - val) * k + val;
        let fresh111 = output;
        output = output.offset(1);
        *fresh111 = val;
    }
    i = long_period;
    while i < size {
        long_sum += *input.offset(i as isize);
        long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        short_sum += *input.offset(i as isize);
        short_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        long_sum -= *input.offset((i - long_period) as isize);
        long_sum2
            -= *input.offset((i - long_period) as isize)
                * *input.offset((i - long_period) as isize);
        short_sum -= *input.offset((i - short_period) as isize);
        short_sum2
            -= *input.offset((i - short_period) as isize)
                * *input.offset((i - short_period) as isize);
        let mut short_stddev_0: libc::c_double = sqrt(
            short_sum2 * short_div - short_sum * short_div * (short_sum * short_div),
        );
        let mut long_stddev_0: libc::c_double = sqrt(
            long_sum2 * long_div - long_sum * long_div * (long_sum * long_div),
        );
        let mut k_0: libc::c_double = short_stddev_0 / long_stddev_0;
        // if k_0 != k_0 {
        //     k_0 = 0 as libc::c_int as libc::c_double;
        // }
        k_0 *= alpha;
        val = (*input.offset(i as isize) - val) * k_0 + val;
        let fresh112 = output;
        output = output.offset(1);
        *fresh112 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vidya_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vidya_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4060 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_vidya(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_volatility_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_volatility(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    let annual: libc::c_double = sqrt(252 as libc::c_int as libc::c_double);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_volatility_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        let c: libc::c_double = *input.offset(i as isize)
            / *input.offset((i - 1 as libc::c_int) as isize) - 1.0f64;
        sum += c;
        sum2 += c * c;
        i += 1;
        i;
    }
    let fresh113 = output;
    output = output.offset(1);
    *fresh113 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
    i = period + 1 as libc::c_int;
    while i < size {
        let c_0: libc::c_double = *input.offset(i as isize)
            / *input.offset((i - 1 as libc::c_int) as isize) - 1.0f64;
        sum += c_0;
        sum2 += c_0 * c_0;
        let cp: libc::c_double = *input.offset((i - period) as isize)
            / *input.offset((i - period - 1 as libc::c_int) as isize) - 1.0f64;
        sum -= cp;
        sum2 -= cp * cp;
        let fresh114 = output;
        output = output.offset(1);
        *fresh114 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_volatility_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_volatility_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4092 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"int ti_volatility(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(1 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let short_div: libc::c_double = 1.0f64 / short_period as libc::c_double;
    let long_div: libc::c_double = 1.0f64 / long_period as libc::c_double;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_vosc_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < long_period {
        if i >= long_period - short_period {
            short_sum += *input.offset(i as isize);
        }
        long_sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let savg: libc::c_double = short_sum * short_div;
    let lavg: libc::c_double = long_sum * long_div;
    let fresh115 = output;
    output = output.offset(1);
    *fresh115 = 100.0f64 * (savg - lavg) / lavg;
    i = long_period;
    while i < size {
        short_sum += *input.offset(i as isize);
        short_sum -= *input.offset((i - short_period) as isize);
        long_sum += *input.offset(i as isize);
        long_sum -= *input.offset((i - long_period) as isize);
        let savg_0: libc::c_double = short_sum * short_div;
        let lavg_0: libc::c_double = long_sum * long_div;
        let fresh116 = output;
        output = output.offset(1);
        *fresh116 = 100.0f64 * (savg_0 - lavg_0) / lavg_0;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4131 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_vosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_vwma_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut vsum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize) * *volume.offset(i as isize);
        vsum += *volume.offset(i as isize);
        i += 1;
        i;
    }
    let fresh117 = output;
    output = output.offset(1);
    *fresh117 = sum / vsum;
    i = period;
    while i < size {
        sum += *input.offset(i as isize) * *volume.offset(i as isize);
        sum
            -= *input.offset((i - period) as isize)
                * *volume.offset((i - period) as isize);
        vsum += *volume.offset(i as isize);
        vsum -= *volume.offset((i - period) as isize);
        let fresh118 = output;
        output = output.offset(1);
        *fresh118 = sum / vsum;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vwma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vwma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4159 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_vwma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wad_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wad(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    if size <= ti_wad_start(options) {
        return 0 as libc::c_int;
    }
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut yc: libc::c_double = *close.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let c: libc::c_double = *close.offset(i as isize);
        if c > yc {
            sum
                += c
                    - (if yc < *low.offset(i as isize) {
                        yc
                    } else {
                        *low.offset(i as isize)
                    });
        } else if c < yc {
            sum
                += c
                    - (if yc > *high.offset(i as isize) {
                        yc
                    } else {
                        *high.offset(i as isize)
                    });
        }
        let fresh119 = output;
        output = output.offset(1);
        *fresh119 = sum;
        yc = *close.offset(i as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_wad_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_wad_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4187 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_wad(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wcprice_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wcprice(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        *output
            .offset(
                i as isize,
            ) = (*high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize) + *close.offset(i as isize)) * 0.25f64;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wilders_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wilders(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_wilders_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let mut val: libc::c_double = sum / period as libc::c_double;
    let fresh120 = output;
    output = output.offset(1);
    *fresh120 = val;
    i = period;
    while i < size {
        val = (*input.offset(i as isize) - val) * per + val;
        let fresh121 = output;
        output = output.offset(1);
        *fresh121 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_wilders_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_wilders_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4227 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_wilders(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_willr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_willr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_willr_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        bar = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *high.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *low.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let highlow: libc::c_double = max - min;
        let r: libc::c_double = if highlow == 0.0f64 {
            0.0f64
        } else {
            -(100 as libc::c_int) as libc::c_double
                * ((max - *close.offset(i as isize)) / highlow)
        };
        let fresh122 = output;
        output = output.offset(1);
        *fresh122 = r;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_willr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_willr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4283 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_willr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_wma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_wma_start(options) {
        return 0 as libc::c_int;
    }
    let weights: libc::c_double = (period * (period + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        weight_sum
            += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    i = period - 1 as libc::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize) * period as libc::c_double;
        sum += *input.offset(i as isize);
        let fresh123 = output;
        output = output.offset(1);
        *fresh123 = weight_sum / weights;
        weight_sum -= sum;
        sum -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_wma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_wma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4310 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_wma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_zlema_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return (*options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int)
        / 2 as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_zlema(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let lag: libc::c_int = (period - 1 as libc::c_int) / 2 as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_zlema_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut val: libc::c_double = *input.offset((lag - 1 as libc::c_int) as isize);
    let fresh124 = output;
    output = output.offset(1);
    *fresh124 = val;
    let mut i: libc::c_int = 0;
    i = lag;
    while i < size {
        let mut c: libc::c_double = *input.offset(i as isize);
        let mut l: libc::c_double = *input.offset((i - lag) as isize);
        val = (c + (c - l) - val) * per + val;
        let fresh125 = output;
        output = output.offset(1);
        *fresh125 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_zlema_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_zlema_start(options)\0" as *const u8
                as *const libc::c_char,
            b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
            4333 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_zlema(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_buffer_new(mut size: libc::c_int) -> *mut ti_buffer {
    let s: libc::c_int = ::core::mem::size_of::<ti_buffer>() as libc::c_ulong
        as libc::c_int
        + (size - 1 as libc::c_int)
            * ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int;
    let mut ret: *mut ti_buffer = malloc(s as libc::c_uint as libc::c_ulong)
        as *mut ti_buffer;
    (*ret).size = size;
    (*ret).pushes = 0 as libc::c_int;
    (*ret).index = 0 as libc::c_int;
    (*ret).sum = 0 as libc::c_int as libc::c_double;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ti_buffer_free(mut buffer: *mut ti_buffer) {
    free(buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_version() -> *const libc::c_char {
    return b"0.9.2\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ti_build() -> libc::c_long {
    return 1743602909 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn ti_indicator_count() -> libc::c_int {
    return 104 as libc::c_int;
}
#[no_mangle]
pub static mut ti_indicators: [ti_indicator_info; 105] = unsafe {
    [
        {
            let mut init = ti_indicator_info {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Absolute Value\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_abs_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_abs
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"abs\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Arccosine\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_acos_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_acos
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"acos\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ad\0" as *const u8 as *const libc::c_char,
                full_name: b"Accumulation/Distribution Line\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_ad_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ad
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 4 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ad\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"add\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Addition\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_add_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_add
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"add\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"adosc\0" as *const u8 as *const libc::c_char,
                full_name: b"Accumulation/Distribution Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_adosc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_adosc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 4 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"adosc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"adx\0" as *const u8 as *const libc::c_char,
                full_name: b"Average Directional Movement Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_adx_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_adx
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"adx\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"adxr\0" as *const u8 as *const libc::c_char,
                full_name: b"Average Directional Movement Rating\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_adxr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_adxr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"adxr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ao\0" as *const u8 as *const libc::c_char,
                full_name: b"Awesome Oscillator\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_ao_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ao
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ao\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"apo\0" as *const u8 as *const libc::c_char,
                full_name: b"Absolute Price Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_apo_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_apo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"apo\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"aroon\0" as *const u8 as *const libc::c_char,
                full_name: b"Aroon\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_aroon_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_aroon
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 2 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"aroon_down\0" as *const u8 as *const libc::c_char,
                    b"aroon_up\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"aroonosc\0" as *const u8 as *const libc::c_char,
                full_name: b"Aroon Oscillator\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_aroonosc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_aroonosc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"aroonosc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Arcsine\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_asin_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_asin
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"asin\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"atan\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Arctangent\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_atan_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_atan
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"atan\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"atr\0" as *const u8 as *const libc::c_char,
                full_name: b"Average True Range\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_atr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_atr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: Some(
                    ti_atr_ref
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"atr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: Some(
                    ti_atr_stream_new
                        as unsafe extern "C" fn(
                            *const libc::c_double,
                            *mut *mut ti_stream,
                        ) -> libc::c_int,
                ),
                stream_run: Some(
                    ti_atr_stream_run
                        as unsafe extern "C" fn(
                            *mut ti_stream,
                            libc::c_int,
                            *const *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                stream_free: Some(
                    ti_atr_stream_free as unsafe extern "C" fn(*mut ti_stream) -> (),
                ),
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"avgprice\0" as *const u8 as *const libc::c_char,
                full_name: b"Average Price\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_avgprice_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_avgprice
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 4 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"open\0" as *const u8 as *const libc::c_char,
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"avgprice\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"bbands\0" as *const u8 as *const libc::c_char,
                full_name: b"Bollinger Bands\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_bbands_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_bbands
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 3 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    b"stddev\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"bbands_lower\0" as *const u8 as *const libc::c_char,
                    b"bbands_middle\0" as *const u8 as *const libc::c_char,
                    b"bbands_upper\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"bop\0" as *const u8 as *const libc::c_char,
                full_name: b"Balance of Power\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_bop_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_bop
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 4 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"open\0" as *const u8 as *const libc::c_char,
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"bop\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"cci\0" as *const u8 as *const libc::c_char,
                full_name: b"Commodity Channel Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_cci_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_cci
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"cci\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ceil\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Ceiling\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_ceil_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ceil
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ceil\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"cmo\0" as *const u8 as *const libc::c_char,
                full_name: b"Chande Momentum Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_cmo_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_cmo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"cmo\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Cosine\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_cos_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_cos
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"cos\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"cosh\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Hyperbolic Cosine\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_cosh_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_cosh
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"cosh\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"crossany\0" as *const u8 as *const libc::c_char,
                full_name: b"Crossany\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_crossany_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_crossany
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"crossany\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"crossover\0" as *const u8 as *const libc::c_char,
                full_name: b"Crossover\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_crossover_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_crossover
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"crossover\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"cvi\0" as *const u8 as *const libc::c_char,
                full_name: b"Chaikins Volatility\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_cvi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_cvi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"cvi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"decay\0" as *const u8 as *const libc::c_char,
                full_name: b"Linear Decay\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_decay_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_decay
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"decay\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"dema\0" as *const u8 as *const libc::c_char,
                full_name: b"Double Exponential Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_dema_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_dema
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"dema\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"di\0" as *const u8 as *const libc::c_char,
                full_name: b"Directional Indicator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_di_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_di
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 2 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"plus_di\0" as *const u8 as *const libc::c_char,
                    b"minus_di\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"div\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Division\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_div_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_div
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"div\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"dm\0" as *const u8 as *const libc::c_char,
                full_name: b"Directional Movement\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_dm_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_dm
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 2 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"plus_dm\0" as *const u8 as *const libc::c_char,
                    b"minus_dm\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"dpo\0" as *const u8 as *const libc::c_char,
                full_name: b"Detrended Price Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_dpo_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_dpo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"dpo\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"dx\0" as *const u8 as *const libc::c_char,
                full_name: b"Directional Movement Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_dx_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_dx
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"dx\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"edecay\0" as *const u8 as *const libc::c_char,
                full_name: b"Exponential Decay\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_edecay_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_edecay
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"edecay\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ema\0" as *const u8 as *const libc::c_char,
                full_name: b"Exponential Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_ema_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ema
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ema\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"emv\0" as *const u8 as *const libc::c_char,
                full_name: b"Ease of Movement\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_emv_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_emv
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"emv\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"exp\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Exponential\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_exp_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_exp
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"exp\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"fisher\0" as *const u8 as *const libc::c_char,
                full_name: b"Fisher Transform\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_fisher_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_fisher
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 2 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"fisher\0" as *const u8 as *const libc::c_char,
                    b"fisher_signal\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"floor\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Floor\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_floor_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_floor
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"floor\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"fosc\0" as *const u8 as *const libc::c_char,
                full_name: b"Forecast Oscillator\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_fosc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_fosc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"fosc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"hma\0" as *const u8 as *const libc::c_char,
                full_name: b"Hull Moving Average\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_hma_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_hma
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"hma\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"kama\0" as *const u8 as *const libc::c_char,
                full_name: b"Kaufman Adaptive Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_kama_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_kama
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"kama\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"kvo\0" as *const u8 as *const libc::c_char,
                full_name: b"Klinger Volume Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_kvo_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_kvo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 4 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"kvo\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"lag\0" as *const u8 as *const libc::c_char,
                full_name: b"Lag\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_lag_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_lag
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"lag\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"linreg\0" as *const u8 as *const libc::c_char,
                full_name: b"Linear Regression\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_linreg_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_linreg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"linreg\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"linregintercept\0" as *const u8 as *const libc::c_char,
                full_name: b"Linear Regression Intercept\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_linregintercept_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_linregintercept
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"linregintercept\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"linregslope\0" as *const u8 as *const libc::c_char,
                full_name: b"Linear Regression Slope\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_linregslope_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_linregslope
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"linregslope\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ln\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Natural Log\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_ln_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ln
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ln\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"log10\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Base-10 Log\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_log10_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_log10
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"log10\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"macd\0" as *const u8 as *const libc::c_char,
                full_name: b"Moving Average Convergence/Divergence\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_macd_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_macd
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 3 as libc::c_int,
                outputs: 3 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    b"signal_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"macd\0" as *const u8 as *const libc::c_char,
                    b"macd_signal\0" as *const u8 as *const libc::c_char,
                    b"macd_histogram\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"marketfi\0" as *const u8 as *const libc::c_char,
                full_name: b"Market Facilitation Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_marketfi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_marketfi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"marketfi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"mass\0" as *const u8 as *const libc::c_char,
                full_name: b"Mass Index\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_mass_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_mass
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"mass\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"max\0" as *const u8 as *const libc::c_char,
                full_name: b"Maximum In Period\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_max_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_max
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: Some(
                    ti_max_ref
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"max\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"md\0" as *const u8 as *const libc::c_char,
                full_name: b"Mean Deviation Over Period\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_md_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_md
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"md\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"medprice\0" as *const u8 as *const libc::c_char,
                full_name: b"Median Price\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_medprice_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_medprice
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"medprice\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"mfi\0" as *const u8 as *const libc::c_char,
                full_name: b"Money Flow Index\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_mfi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_mfi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 4 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"mfi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"min\0" as *const u8 as *const libc::c_char,
                full_name: b"Minimum In Period\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_min_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_min
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: Some(
                    ti_min_ref
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"min\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"mom\0" as *const u8 as *const libc::c_char,
                full_name: b"Momentum\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_mom_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_mom
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"mom\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"msw\0" as *const u8 as *const libc::c_char,
                full_name: b"Mesa Sine Wave\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_msw_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_msw
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 2 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"msw_sine\0" as *const u8 as *const libc::c_char,
                    b"msw_lead\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"mul\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Multiplication\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_mul_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_mul
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"mul\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"natr\0" as *const u8 as *const libc::c_char,
                full_name: b"Normalized Average True Range\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_natr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_natr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"natr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"nvi\0" as *const u8 as *const libc::c_char,
                full_name: b"Negative Volume Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_nvi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_nvi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"nvi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"obv\0" as *const u8 as *const libc::c_char,
                full_name: b"On Balance Volume\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_obv_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_obv
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"obv\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ppo\0" as *const u8 as *const libc::c_char,
                full_name: b"Percentage Price Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_ppo_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ppo
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ppo\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"psar\0" as *const u8 as *const libc::c_char,
                full_name: b"Parabolic SAR\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_psar_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_psar
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"acceleration_factor_step\0" as *const u8 as *const libc::c_char,
                    b"acceleration_factor_maximum\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"psar\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"pvi\0" as *const u8 as *const libc::c_char,
                full_name: b"Positive Volume Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_pvi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_pvi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"pvi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"qstick\0" as *const u8 as *const libc::c_char,
                full_name: b"Qstick\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_qstick_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_qstick
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"open\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"qstick\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"roc\0" as *const u8 as *const libc::c_char,
                full_name: b"Rate of Change\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_roc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_roc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"roc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"rocr\0" as *const u8 as *const libc::c_char,
                full_name: b"Rate of Change Ratio\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_rocr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_rocr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"rocr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"round\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Round\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_round_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_round
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"round\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"rsi\0" as *const u8 as *const libc::c_char,
                full_name: b"Relative Strength Index\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_rsi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_rsi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"rsi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Sine\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_sin_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_sin
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"sin\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"sinh\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Hyperbolic Sine\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_sinh_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_sinh
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"sinh\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"sma\0" as *const u8 as *const libc::c_char,
                full_name: b"Simple Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_sma_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_sma
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"sma\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: Some(
                    ti_sma_stream_new
                        as unsafe extern "C" fn(
                            *const libc::c_double,
                            *mut *mut ti_stream,
                        ) -> libc::c_int,
                ),
                stream_run: Some(
                    ti_sma_stream_run
                        as unsafe extern "C" fn(
                            *mut ti_stream,
                            libc::c_int,
                            *const *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                stream_free: Some(
                    ti_sma_stream_free as unsafe extern "C" fn(*mut ti_stream) -> (),
                ),
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"sqrt\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Square Root\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_sqrt_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_sqrt
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"sqrt\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"stddev\0" as *const u8 as *const libc::c_char,
                full_name: b"Standard Deviation Over Period\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_stddev_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_stddev
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"stddev\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"stderr\0" as *const u8 as *const libc::c_char,
                full_name: b"Standard Error Over Period\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_stderr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_stderr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"stderr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"stoch\0" as *const u8 as *const libc::c_char,
                full_name: b"Stochastic Oscillator\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_stoch_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_stoch
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 3 as libc::c_int,
                outputs: 2 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"k_period\0" as *const u8 as *const libc::c_char,
                    b"k_slowing_period\0" as *const u8 as *const libc::c_char,
                    b"d_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"stoch_k\0" as *const u8 as *const libc::c_char,
                    b"stoch_d\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"stochrsi\0" as *const u8 as *const libc::c_char,
                full_name: b"Stochastic RSI\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_stochrsi_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_stochrsi
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"stochrsi\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"sub\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Subtraction\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_sub_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_sub
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"sub\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"sum\0" as *const u8 as *const libc::c_char,
                full_name: b"Sum Over Period\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_sum_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_sum
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"sum\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Tangent\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_tan_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_tan
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"tan\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"tanh\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Hyperbolic Tangent\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_tanh_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_tanh
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"tanh\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"tema\0" as *const u8 as *const libc::c_char,
                full_name: b"Triple Exponential Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_tema_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_tema
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"tema\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"todeg\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Degree Conversion\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_todeg_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_todeg
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"degrees\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"torad\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Radian Conversion\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_torad_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_torad
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"radians\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"tr\0" as *const u8 as *const libc::c_char,
                full_name: b"True Range\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_tr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_tr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"tr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"trima\0" as *const u8 as *const libc::c_char,
                full_name: b"Triangular Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_trima_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_trima
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"trima\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"trix\0" as *const u8 as *const libc::c_char,
                full_name: b"Trix\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_trix_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_trix
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"trix\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"trunc\0" as *const u8 as *const libc::c_char,
                full_name: b"Vector Truncate\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_trunc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_trunc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 4 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"trunc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"tsf\0" as *const u8 as *const libc::c_char,
                full_name: b"Time Series Forecast\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_tsf_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_tsf
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"tsf\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"typprice\0" as *const u8 as *const libc::c_char,
                full_name: b"Typical Price\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_typprice_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_typprice
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"typprice\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"ultosc\0" as *const u8 as *const libc::c_char,
                full_name: b"Ultimate Oscillator\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_ultosc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_ultosc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 3 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"medium_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"ultosc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"var\0" as *const u8 as *const libc::c_char,
                full_name: b"Variance Over Period\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_var_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_var
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 3 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"var\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"vhf\0" as *const u8 as *const libc::c_char,
                full_name: b"Vertical Horizontal Filter\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_vhf_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_vhf
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"vhf\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"vidya\0" as *const u8 as *const libc::c_char,
                full_name: b"Variable Index Dynamic Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_vidya_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_vidya
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 3 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    b"alpha\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"vidya\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"volatility\0" as *const u8 as *const libc::c_char,
                full_name: b"Annualized Historical Volatility\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_volatility_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_volatility
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"volatility\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"vosc\0" as *const u8 as *const libc::c_char,
                full_name: b"Volume Oscillator\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_vosc_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_vosc
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 2 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"short_period\0" as *const u8 as *const libc::c_char,
                    b"long_period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"vosc\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"vwma\0" as *const u8 as *const libc::c_char,
                full_name: b"Volume Weighted Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_vwma_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_vwma
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 2 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"close\0" as *const u8 as *const libc::c_char,
                    b"volume\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"vwma\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"wad\0" as *const u8 as *const libc::c_char,
                full_name: b"Williams Accumulation/Distribution\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_wad_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_wad
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"wad\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"wcprice\0" as *const u8 as *const libc::c_char,
                full_name: b"Weighted Close Price\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_wcprice_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_wcprice
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"wcprice\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"wilders\0" as *const u8 as *const libc::c_char,
                full_name: b"Wilders Smoothing\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_wilders_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_wilders
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"wilders\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"willr\0" as *const u8 as *const libc::c_char,
                full_name: b"Williams %R\0" as *const u8 as *const libc::c_char,
                start: Some(
                    ti_willr_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_willr
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 2 as libc::c_int,
                inputs: 3 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"high\0" as *const u8 as *const libc::c_char,
                    b"low\0" as *const u8 as *const libc::c_char,
                    b"close\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"willr\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"wma\0" as *const u8 as *const libc::c_char,
                full_name: b"Weighted Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_wma_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_wma
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"wma\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: b"zlema\0" as *const u8 as *const libc::c_char,
                full_name: b"Zero-Lag Exponential Moving Average\0" as *const u8
                    as *const libc::c_char,
                start: Some(
                    ti_zlema_start
                        as unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
                ),
                indicator: Some(
                    ti_zlema
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const libc::c_double,
                            *const libc::c_double,
                            *const *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                indicator_ref: None,
                type_0: 1 as libc::c_int,
                inputs: 1 as libc::c_int,
                options: 1 as libc::c_int,
                outputs: 1 as libc::c_int,
                input_names: [
                    b"real\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    b"period\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    b"zlema\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
        {
            let mut init = ti_indicator_info {
                name: 0 as *const libc::c_char,
                full_name: 0 as *const libc::c_char,
                start: None,
                indicator: None,
                indicator_ref: None,
                type_0: 0 as libc::c_int,
                inputs: 0 as libc::c_int,
                options: 0 as libc::c_int,
                outputs: 0 as libc::c_int,
                input_names: [
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                option_names: [
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                output_names: [
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
                stream_new: None,
                stream_run: None,
                stream_free: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn ti_stream_run(
    mut stream: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    return (ti_indicators[(*stream).index as usize].stream_run)
        .expect("non-null function pointer")(stream, size, inputs, outputs);
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_get_info(
    mut stream: *mut ti_stream,
) -> *mut ti_indicator_info {
    return ti_indicators.as_mut_ptr().offset((*stream).index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_get_progress(
    mut stream: *mut ti_stream,
) -> libc::c_int {
    return (*stream).progress;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_free(mut stream: *mut ti_stream) {
    (ti_indicators[(*stream).index as usize].stream_free)
        .expect("non-null function pointer")(stream);
}
#[no_mangle]
pub unsafe extern "C" fn ti_find_indicator(
    mut name: *const libc::c_char,
) -> *const ti_indicator_info {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::core::mem::size_of::<[ti_indicator_info; 105]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<ti_indicator_info>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        let c: libc::c_int = strcmp(name, ti_indicators[i as usize].name);
        if c == 0 as libc::c_int {
            return ti_indicators.as_mut_ptr().offset(i as isize)
        } else if c > 0 as libc::c_int {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const ti_indicator_info;
}
#[no_mangle]
pub unsafe extern "C" fn tc_version() -> *const libc::c_char {
    return b"0.9.2\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn tc_build() -> libc::c_long {
    return 1743602909 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn tc_candle_count() -> libc::c_int {
    return 26 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_config_default() -> *const tc_config {
    static mut default_config: tc_config = {
        let mut init = tc_config {
            period: 10 as libc::c_int,
            body_none: 0.05f64,
            body_short: 0.5f64,
            body_long: 1.4f64,
            wick_none: 0.05f64,
            wick_long: 0.6f64,
            near: 0.3f64,
        };
        init
    };
    return &mut default_config;
}
#[no_mangle]
pub static mut tc_candles: [tc_candle_info; 27] = unsafe {
    [
        {
            let mut init = tc_candle_info {
                name: b"abandoned_baby_bear\0" as *const u8 as *const libc::c_char,
                full_name: b"Abandoned Baby Bear\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 0 as libc::c_int) as tc_set,
                candle: Some(
                    tc_abandoned_baby_bear
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"abandoned_baby_bull\0" as *const u8 as *const libc::c_char,
                full_name: b"Abandoned Baby Bull\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 1 as libc::c_int) as tc_set,
                candle: Some(
                    tc_abandoned_baby_bull
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"big_black_candle\0" as *const u8 as *const libc::c_char,
                full_name: b"Big Black Candle\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 2 as libc::c_int) as tc_set,
                candle: Some(
                    tc_big_black_candle
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"big_white_candle\0" as *const u8 as *const libc::c_char,
                full_name: b"Big White Candle\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 3 as libc::c_int) as tc_set,
                candle: Some(
                    tc_big_white_candle
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"black_marubozu\0" as *const u8 as *const libc::c_char,
                full_name: b"Black Marubozu\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 4 as libc::c_int) as tc_set,
                candle: Some(
                    tc_black_marubozu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 5 as libc::c_int) as tc_set,
                candle: Some(
                    tc_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"dragonfly_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Dragonfly Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 6 as libc::c_int) as tc_set,
                candle: Some(
                    tc_dragonfly_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"engulfing_bear\0" as *const u8 as *const libc::c_char,
                full_name: b"Engulfing Bear\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 7 as libc::c_int) as tc_set,
                candle: Some(
                    tc_engulfing_bear
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"engulfing_bull\0" as *const u8 as *const libc::c_char,
                full_name: b"Engulfing Bull\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 8 as libc::c_int) as tc_set,
                candle: Some(
                    tc_engulfing_bull
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"evening_doji_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Evening Doji Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 9 as libc::c_int) as tc_set,
                candle: Some(
                    tc_evening_doji_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"evening_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Evening Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 10 as libc::c_int) as tc_set,
                candle: Some(
                    tc_evening_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"four_price_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Four Price Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 11 as libc::c_int) as tc_set,
                candle: Some(
                    tc_four_price_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"gravestone_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Gravestone Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 12 as libc::c_int) as tc_set,
                candle: Some(
                    tc_gravestone_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"hammer\0" as *const u8 as *const libc::c_char,
                full_name: b"Hammer\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 13 as libc::c_int) as tc_set,
                candle: Some(
                    tc_hammer
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"hanging_man\0" as *const u8 as *const libc::c_char,
                full_name: b"Hanging Man\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 14 as libc::c_int) as tc_set,
                candle: Some(
                    tc_hanging_man
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"inverted_hammer\0" as *const u8 as *const libc::c_char,
                full_name: b"Inverted Hammer\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 15 as libc::c_int) as tc_set,
                candle: Some(
                    tc_inverted_hammer
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"long_legged_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Long Legged Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 16 as libc::c_int) as tc_set,
                candle: Some(
                    tc_long_legged_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"marubozu\0" as *const u8 as *const libc::c_char,
                full_name: b"Marubozu\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 17 as libc::c_int) as tc_set,
                candle: Some(
                    tc_marubozu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"morning_doji_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Morning Doji Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 18 as libc::c_int) as tc_set,
                candle: Some(
                    tc_morning_doji_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"morning_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Morning Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 19 as libc::c_int) as tc_set,
                candle: Some(
                    tc_morning_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"shooting_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Shooting Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 20 as libc::c_int) as tc_set,
                candle: Some(
                    tc_shooting_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"spinning_top\0" as *const u8 as *const libc::c_char,
                full_name: b"Spinning Top\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 21 as libc::c_int) as tc_set,
                candle: Some(
                    tc_spinning_top
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"star\0" as *const u8 as *const libc::c_char,
                full_name: b"Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 22 as libc::c_int) as tc_set,
                candle: Some(
                    tc_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"three_black_crows\0" as *const u8 as *const libc::c_char,
                full_name: b"Three Black Crows\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 23 as libc::c_int) as tc_set,
                candle: Some(
                    tc_three_black_crows
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"three_white_soldiers\0" as *const u8 as *const libc::c_char,
                full_name: b"Three White Soldiers\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 24 as libc::c_int) as tc_set,
                candle: Some(
                    tc_three_white_soldiers
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"white_marubozu\0" as *const u8 as *const libc::c_char,
                full_name: b"White Marubozu\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 25 as libc::c_int) as tc_set,
                candle: Some(
                    tc_white_marubozu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: 0 as *const libc::c_char,
                full_name: 0 as *const libc::c_char,
                pattern: 0 as libc::c_int as tc_set,
                candle: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn tc_find_candle(
    mut name: *const libc::c_char,
) -> *const tc_candle_info {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::core::mem::size_of::<[tc_candle_info; 27]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<tc_candle_info>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        let c: libc::c_int = strcmp(name, tc_candles[i as usize].name);
        if c == 0 as libc::c_int {
            return tc_candles.as_mut_ptr().offset(i as isize)
        } else if c > 0 as libc::c_int {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const tc_candle_info;
}
#[no_mangle]
pub unsafe extern "C" fn tc_get_info(mut pattern: tc_set) -> *const tc_candle_info {
    if pattern == 0 {
        return 0 as *const tc_candle_info;
    }
    let mut k: tc_set = pattern & !pattern.wrapping_sub(1 as libc::c_int as tc_set);
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::core::mem::size_of::<[tc_candle_info; 27]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<tc_candle_info>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        if k == tc_candles[i as usize].pattern {
            return tc_candles.as_mut_ptr().offset(i as isize)
        } else if k > tc_candles[i as usize].pattern {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const tc_candle_info;
}
#[no_mangle]
pub unsafe extern "C" fn tc_config_set_to_default(mut config: *mut tc_config) {
    memcpy(
        config as *mut libc::c_void,
        tc_config_default() as *const libc::c_void,
        ::core::mem::size_of::<tc_config>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tc_result_new() -> *mut tc_result {
    let mut k: *mut tc_result = malloc(
        ::core::mem::size_of::<tc_result>() as libc::c_ulong,
    ) as *mut tc_result;
    if k.is_null() {
        return 0 as *mut tc_result;
    }
    memset(
        k as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<tc_result>() as libc::c_ulong,
    );
    return k;
}
unsafe extern "C" fn tc_result_reset(mut result: *mut tc_result) {
    (*result).progress = 0 as libc::c_int;
    (*result).count = 0 as libc::c_int;
    (*result).pattern_count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_result_free(mut result: *mut tc_result) {
    if result.is_null() {
        return;
    }
    free((*result).hits as *mut libc::c_void);
    free(result as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tc_result_count(mut result: *const tc_result) -> libc::c_int {
    return (*result).count;
}
#[no_mangle]
pub unsafe extern "C" fn tc_result_pattern_count(
    mut result: *const tc_result,
) -> libc::c_int {
    return (*result).pattern_count;
}
#[no_mangle]
pub unsafe extern "C" fn tc_result_get(
    mut result: *const tc_result,
    mut index: libc::c_int,
) -> tc_hit {
    if index < (*result).count {
        return *((*result).hits).offset(index as isize);
    }
    let mut hit: tc_hit = {
        let mut init = tc_hit {
            index: 0 as libc::c_int,
            patterns: 0,
        };
        init
    };
    return hit;
}
#[no_mangle]
pub unsafe extern "C" fn tc_result_at(
    mut result: *const tc_result,
    mut index: libc::c_int,
) -> tc_set {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (*result).count - 1 as libc::c_int;
    if imax == 0 {
        return 0 as libc::c_int as tc_set;
    }
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        if i >= 0 as libc::c_int {} else {
            __assert_fail(
                b"i >= 0\0" as *const u8 as *const libc::c_char,
                b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
                4506 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                    .as_ptr(),
            );
        };
        if i < (*result).count {} else {
            __assert_fail(
                b"i < result->count\0" as *const u8 as *const libc::c_char,
                b"tiamalgamation.c\0" as *const u8 as *const libc::c_char,
                4507 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                    .as_ptr(),
            );
        };
        if index == (*((*result).hits).offset(i as isize)).index {
            return (*((*result).hits).offset(i as isize)).patterns
        } else if index > (*((*result).hits).offset(i as isize)).index {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int as tc_set;
}
unsafe extern "C" fn tc_result_add(
    mut result: *mut tc_result,
    mut hit: tc_hit,
) -> libc::c_int {
    (*result).pattern_count += 1;
    (*result).pattern_count;
    if (*result).count > 0 as libc::c_int {
        if (*((*result).hits).offset(((*result).count - 1 as libc::c_int) as isize))
            .index == hit.index
        {
            (*((*result).hits).offset(((*result).count - 1 as libc::c_int) as isize))
                .patterns |= hit.patterns;
            return 0 as libc::c_int;
        }
    }
    if (*result).space == 0 as libc::c_int {
        (*result).space = 2048 as libc::c_int;
        (*result).count = 0 as libc::c_int;
        (*result)
            .hits = malloc(
            (::core::mem::size_of::<tc_hit>() as libc::c_ulong)
                .wrapping_mul((*result).space as libc::c_uint as libc::c_ulong),
        ) as *mut tc_hit;
    } else if (*result).space == (*result).count {
        (*result).space *= 2 as libc::c_int;
        let mut hits: *mut tc_hit = realloc(
            (*result).hits as *mut libc::c_void,
            (::core::mem::size_of::<tc_hit>() as libc::c_ulong)
                .wrapping_mul((*result).space as libc::c_uint as libc::c_ulong),
        ) as *mut tc_hit;
        if hits.is_null() {
            return 2 as libc::c_int;
        }
        (*result).hits = hits;
    }
    *((*result).hits).offset((*result).count as isize) = hit;
    (*result).count += 1;
    (*result).count;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_run(
    mut patterns: tc_set,
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    tc_result_reset(output);
    if patterns & patterns.wrapping_sub(1 as libc::c_int as tc_set)
        == 0 as libc::c_int as tc_set
    {
        if patterns == ((1 as libc::c_long) << 0 as libc::c_int) as tc_set {
            return tc_abandoned_baby_bear(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 1 as libc::c_int) as tc_set {
            return tc_abandoned_baby_bull(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 2 as libc::c_int) as tc_set {
            return tc_big_black_candle(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 3 as libc::c_int) as tc_set {
            return tc_big_white_candle(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 4 as libc::c_int) as tc_set {
            return tc_black_marubozu(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 5 as libc::c_int) as tc_set {
            return tc_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 6 as libc::c_int) as tc_set {
            return tc_dragonfly_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 7 as libc::c_int) as tc_set {
            return tc_engulfing_bear(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 8 as libc::c_int) as tc_set {
            return tc_engulfing_bull(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 9 as libc::c_int) as tc_set {
            return tc_evening_doji_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 10 as libc::c_int) as tc_set {
            return tc_evening_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 11 as libc::c_int) as tc_set {
            return tc_four_price_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 12 as libc::c_int) as tc_set {
            return tc_gravestone_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 13 as libc::c_int) as tc_set {
            return tc_hammer(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 14 as libc::c_int) as tc_set {
            return tc_hanging_man(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 15 as libc::c_int) as tc_set {
            return tc_inverted_hammer(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 16 as libc::c_int) as tc_set {
            return tc_long_legged_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 17 as libc::c_int) as tc_set {
            return tc_marubozu(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 18 as libc::c_int) as tc_set {
            return tc_morning_doji_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 19 as libc::c_int) as tc_set {
            return tc_morning_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 20 as libc::c_int) as tc_set {
            return tc_shooting_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 21 as libc::c_int) as tc_set {
            return tc_spinning_top(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 22 as libc::c_int) as tc_set {
            return tc_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 23 as libc::c_int) as tc_set {
            return tc_three_black_crows(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 24 as libc::c_int) as tc_set {
            return tc_three_white_soldiers(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 25 as libc::c_int) as tc_set {
            return tc_white_marubozu(size, inputs, options, output);
        }
    }
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if ((1 as libc::c_long) << 0 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *low.offset((i - 1 as libc::c_int) as isize)
                >= *high.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && *high.offset(i as isize) <= *low.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 0 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        if ((1 as libc::c_long) << 1 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *high.offset((i - 1 as libc::c_int) as isize)
                <= *low.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && *low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_0: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 1 as libc::c_int) as tc_set,
                };
                init
            };
            let r_0: libc::c_int = tc_result_add(output, hit_0);
            if r_0 != 0 as libc::c_int {
                return r_0;
            }
        }
        if ((1 as libc::c_long) << 2 as libc::c_int) as tc_set & patterns != 0
            && *open.offset(i as isize) > *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit_1: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 2 as libc::c_int) as tc_set,
                };
                init
            };
            let r_1: libc::c_int = tc_result_add(output, hit_1);
            if r_1 != 0 as libc::c_int {
                return r_1;
            }
        }
        if ((1 as libc::c_long) << 3 as libc::c_int) as tc_set & patterns != 0
            && *open.offset(i as isize) < *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit_2: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 3 as libc::c_int) as tc_set,
                };
                init
            };
            let r_2: libc::c_int = tc_result_add(output, hit_2);
            if r_2 != 0 as libc::c_int {
                return r_2;
            }
        }
        if ((1 as libc::c_long) << 4 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
        {
            let hit_3: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 4 as libc::c_int) as tc_set,
                };
                init
            };
            let r_3: libc::c_int = tc_result_add(output, hit_3);
            if r_3 != 0 as libc::c_int {
                return r_3;
            }
        }
        if ((1 as libc::c_long) << 5 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
        {
            let hit_4: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 5 as libc::c_int) as tc_set,
                };
                init
            };
            let r_4: libc::c_int = tc_result_add(output, hit_4);
            if r_4 != 0 as libc::c_int {
                return r_4;
            }
        }
        if ((1 as libc::c_long) << 6 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit_5: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 6 as libc::c_int) as tc_set,
                };
                init
            };
            let r_5: libc::c_int = tc_result_add(output, hit_5);
            if r_5 != 0 as libc::c_int {
                return r_5;
            }
        }
        if ((1 as libc::c_long) << 7 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && *open.offset(i as isize) > *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) < *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit_6: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 7 as libc::c_int) as tc_set,
                };
                init
            };
            let r_6: libc::c_int = tc_result_add(output, hit_6);
            if r_6 != 0 as libc::c_int {
                return r_6;
            }
        }
        if ((1 as libc::c_long) << 8 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && *open.offset(i as isize) < *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) > *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit_7: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 8 as libc::c_int) as tc_set,
                };
                init
            };
            let r_7: libc::c_int = tc_result_add(output, hit_7);
            if r_7 != 0 as libc::c_int {
                return r_7;
            }
        }
        if ((1 as libc::c_long) << 9 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_8: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 9 as libc::c_int) as tc_set,
                };
                init
            };
            let r_8: libc::c_int = tc_result_add(output, hit_8);
            if r_8 != 0 as libc::c_int {
                return r_8;
            }
        }
        if ((1 as libc::c_long) << 10 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_9: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 10 as libc::c_int) as tc_set,
                };
                init
            };
            let r_9: libc::c_int = tc_result_add(output, hit_9);
            if r_9 != 0 as libc::c_int {
                return r_9;
            }
        }
        if ((1 as libc::c_long) << 11 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit_10: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 11 as libc::c_int) as tc_set,
                };
                init
            };
            let r_10: libc::c_int = tc_result_add(output, hit_10);
            if r_10 != 0 as libc::c_int {
                return r_10;
            }
        }
        if ((1 as libc::c_long) << 12 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit_11: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 12 as libc::c_int) as tc_set,
                };
                init
            };
            let r_11: libc::c_int = tc_result_add(output, hit_11);
            if r_11 != 0 as libc::c_int {
                return r_11;
            }
        }
        if ((1 as libc::c_long) << 13 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= *low.offset((i - 1 as libc::c_int) as isize)
                    + (*options).near * avg_total
        {
            let hit_12: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 13 as libc::c_int) as tc_set,
                };
                init
            };
            let r_12: libc::c_int = tc_result_add(output, hit_12);
            if r_12 != 0 as libc::c_int {
                return r_12;
            }
        }
        if ((1 as libc::c_long) << 14 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= *high.offset((i - 1 as libc::c_int) as isize)
                    - (*options).near * avg_total
        {
            let hit_13: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 14 as libc::c_int) as tc_set,
                };
                init
            };
            let r_13: libc::c_int = tc_result_add(output, hit_13);
            if r_13 != 0 as libc::c_int {
                return r_13;
            }
        }
        if ((1 as libc::c_long) << 15 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit_14: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 15 as libc::c_int) as tc_set,
                };
                init
            };
            let r_14: libc::c_int = tc_result_add(output, hit_14);
            if r_14 != 0 as libc::c_int {
                return r_14;
            }
        }
        if ((1 as libc::c_long) << 16 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit_15: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 16 as libc::c_int) as tc_set,
                };
                init
            };
            let r_15: libc::c_int = tc_result_add(output, hit_15);
            if r_15 != 0 as libc::c_int {
                return r_15;
            }
        }
        if ((1 as libc::c_long) << 17 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit_16: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 17 as libc::c_int) as tc_set,
                };
                init
            };
            let r_16: libc::c_int = tc_result_add(output, hit_16);
            if r_16 != 0 as libc::c_int {
                return r_16;
            }
        }
        if ((1 as libc::c_long) << 18 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_17: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 18 as libc::c_int) as tc_set,
                };
                init
            };
            let r_17: libc::c_int = tc_result_add(output, hit_17);
            if r_17 != 0 as libc::c_int {
                return r_17;
            }
        }
        if ((1 as libc::c_long) << 19 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_18: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 19 as libc::c_int) as tc_set,
                };
                init
            };
            let r_18: libc::c_int = tc_result_add(output, hit_18);
            if r_18 != 0 as libc::c_int {
                return r_18;
            }
        }
        if ((1 as libc::c_long) << 20 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit_19: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 20 as libc::c_int) as tc_set,
                };
                init
            };
            let r_19: libc::c_int = tc_result_add(output, hit_19);
            if r_19 != 0 as libc::c_int {
                return r_19;
            }
        }
        if ((1 as libc::c_long) << 21 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
            && bottom - *low.offset(i as isize)
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
        {
            let hit_20: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 21 as libc::c_int) as tc_set,
                };
                init
            };
            let r_20: libc::c_int = tc_result_add(output, hit_20);
            if r_20 != 0 as libc::c_int {
                return r_20;
            }
        }
        if ((1 as libc::c_long) << 22 as libc::c_int) as tc_set & patterns != 0
            && i >= 1 as libc::c_int
            && (*low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
                || *high.offset(i as isize)
                    <= *low.offset((i - 1 as libc::c_int) as isize))
        {
            let hit_21: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 22 as libc::c_int) as tc_set,
                };
                init
            };
            let r_21: libc::c_int = tc_result_add(output, hit_21);
            if r_21 != 0 as libc::c_int {
                return r_21;
            }
        }
        if ((1 as libc::c_long) << 23 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit_22: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 23 as libc::c_int) as tc_set,
                };
                init
            };
            let r_22: libc::c_int = tc_result_add(output, hit_22);
            if r_22 != 0 as libc::c_int {
                return r_22;
            }
        }
        if ((1 as libc::c_long) << 24 as libc::c_int) as tc_set & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit_23: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 24 as libc::c_int) as tc_set,
                };
                init
            };
            let r_23: libc::c_int = tc_result_add(output, hit_23);
            if r_23 != 0 as libc::c_int {
                return r_23;
            }
        }
        if ((1 as libc::c_long) << 25 as libc::c_int) as tc_set & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
        {
            let hit_24: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 25 as libc::c_int) as tc_set,
                };
                init
            };
            let r_24: libc::c_int = tc_result_add(output, hit_24);
            if r_24 != 0 as libc::c_int {
                return r_24;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_abandoned_baby_bear(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *low.offset((i - 1 as libc::c_int) as isize)
                >= *high.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && *high.offset(i as isize) <= *low.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 0 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_abandoned_baby_bull(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *high.offset((i - 1 as libc::c_int) as isize)
                <= *low.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && *low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 1 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_big_black_candle(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if *open.offset(i as isize) > *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 2 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_big_white_candle(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if *open.offset(i as isize) < *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 3 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_black_marubozu(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 4 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 5 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_dragonfly_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 6 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_engulfing_bear(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int && *open.offset(i as isize) > *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) < *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 7 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_engulfing_bull(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int && *open.offset(i as isize) < *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) > *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 8 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_evening_doji_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 9 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_evening_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 10 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_four_price_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 11 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_gravestone_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 12 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_hammer(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= *low.offset((i - 1 as libc::c_int) as isize)
                    + (*options).near * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 13 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_hanging_man(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= *high.offset((i - 1 as libc::c_int) as isize)
                    - (*options).near * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 14 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_inverted_hammer(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 15 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_long_legged_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 16 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_marubozu(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 17 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_morning_doji_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 18 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_morning_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 19 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_shooting_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 20 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_spinning_top(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_short * avg_body
            && *high.offset(i as isize) - top
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
            && bottom - *low.offset(i as isize)
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 21 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && (*low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
                || *high.offset(i as isize)
                    <= *low.offset((i - 1 as libc::c_int) as isize))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 22 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_three_black_crows(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 23 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_three_white_soldiers(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 24 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tc_white_marubozu(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 25 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
