#![feature(label_break_value)]

use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint64_t = libc::c_ulong;
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
pub static mut tc_candles: [tc_candle_info; 0] = [tc_candle_info {
    name: 0 as *const libc::c_char,
    full_name: 0 as *const libc::c_char,
    pattern: 0,
    candle: None,
}; 0];
#[no_mangle]
pub unsafe extern "C" fn tc_find_candle(
    mut name: *const libc::c_char,
) -> *const tc_candle_info {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::core::mem::size_of::<[tc_candle_info; 0]>()
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
    let mut imax: libc::c_int = (::core::mem::size_of::<[tc_candle_info; 0]>()
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
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/templates/candles.c\0"
                    as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_3279: {
            if i >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"i >= 0\0" as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/templates/candles.c\0"
                        as *const u8 as *const libc::c_char,
                    180 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if i < (*result).count {} else {
            __assert_fail(
                b"i < result->count\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/templates/candles.c\0"
                    as *const u8 as *const libc::c_char,
                181 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_3232: {
            if i < (*result).count {} else {
                __assert_fail(
                    b"i < result->count\0" as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/templates/candles.c\0"
                        as *const u8 as *const libc::c_char,
                    181 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                        .as_ptr(),
                );
            }
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
#[no_mangle]
pub unsafe extern "C" fn tc_run(
    mut patterns: tc_set,
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    tc_result_reset(output);
    patterns & patterns.wrapping_sub(1 as libc::c_int as tc_set)
        == 0 as libc::c_int as tc_set;
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
