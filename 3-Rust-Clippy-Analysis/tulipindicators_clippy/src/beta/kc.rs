use ::libc;
extern "C" {
    pub type ti_stream;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_kc {
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
    pub offset_close: libc::c_int,
    pub size_close: libc::c_int,
    pub index_close: libc::c_int,
    pub padding_close: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub price_ema: libc::c_double,
    pub tr_ema: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub period: libc::c_double,
    pub multiple: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn ti_kc_start(mut options: *const libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let multiple: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut kc_lower: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut kc_middle: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut kc_upper: *mut libc::c_double = *outputs.offset(2 as libc::c_int as isize);
    if period < 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period + 1 as libc::c_int as libc::c_double);
    let mut price_ema: libc::c_double = *close.offset(0 as libc::c_int as isize);
    let mut tr_ema: libc::c_double = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let fresh0 = kc_lower;
    kc_lower = kc_lower.offset(1);
    *fresh0 = price_ema - multiple * tr_ema;
    let fresh1 = kc_middle;
    kc_middle = kc_middle.offset(1);
    *fresh1 = price_ema;
    let fresh2 = kc_upper;
    kc_upper = kc_upper.offset(1);
    *fresh2 = price_ema + multiple * tr_ema;
    let mut truerange: libc::c_double = 0.;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < size {
        price_ema = (*close.offset(i as isize) - price_ema) * per + price_ema;
        CALC_TRUERANGE();
        tr_ema = (truerange - tr_ema) * per + tr_ema;
        let fresh3 = kc_lower;
        kc_lower = kc_lower.offset(1);
        *fresh3 = price_ema - multiple * tr_ema;
        let fresh4 = kc_middle;
        kc_middle = kc_middle.offset(1);
        *fresh4 = price_ema;
        let fresh5 = kc_upper;
        kc_upper = kc_upper.offset(1);
        *fresh5 = price_ema + multiple * tr_ema;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kc_stream_new(
    mut options: *const libc::c_double,
    mut stream_in: *mut *mut ti_stream,
) -> libc::c_int {
    let mut stream: *mut *mut ti_stream_kc = stream_in as *mut *mut ti_stream_kc;
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let multiple: libc::c_double = *options.offset(1 as libc::c_int as isize);
    if period < 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    *stream = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ti_stream_kc>() as libc::c_ulong,
    ) as *mut ti_stream_kc;
    if (*stream).is_null() {
        return 2 as libc::c_int;
    }
    (**stream).progress = 0 as libc::c_int;
    (**stream).options.period = period;
    (**stream).options.multiple = multiple;
    (**stream)
        .buf_info
        .offset_close = *(&mut (**stream).buf_info.offset_close as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (**stream).buf_info.offset_close as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (**stream).buf_info.size_close = 2 as libc::c_int;
    (**stream).buf_info.index_close = -(1 as libc::c_int);
    *stream = realloc(
        *stream as *mut libc::c_void,
        (::core::mem::size_of::<ti_stream_kc>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_double>()
                    * (*((&mut (**stream).buf_info as *mut C2RustUnnamed)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                        .offset(-(3 as libc::c_int as isize))
                        + *((&mut (**stream).buf_info as *mut C2RustUnnamed)
                            .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                            .offset(-(4 as libc::c_int as isize))) as usize)
                    as libc::c_ulong,
            ),
    ) as *mut ti_stream_kc;
    if (*stream).is_null() {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kc_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_kc_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_kc = stream_in as *mut ti_stream_kc;
    let mut progress: libc::c_int = (*stream).progress;
    let period: libc::c_double = (*stream).options.period;
    let multiple: libc::c_double = (*stream).options.multiple;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut kc_lower: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut kc_middle: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut kc_upper: *mut libc::c_double = *outputs.offset(2 as libc::c_int as isize);
    let mut price_ema: libc::c_double = (*stream).state.price_ema;
    let mut tr_ema: libc::c_double = (*stream).state.tr_ema;
    let mut var1: libc::c_double = 0.;
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period + 1 as libc::c_int as libc::c_double);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size && progress == 0 as libc::c_int {
        let mut idx: libc::c_int = (*stream).buf_info.index_close + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_close {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx as isize) = *close.offset(i as isize);
        (*stream).buf_info.index_close = idx;
        price_ema = *close.offset(i as isize);
        tr_ema = *high.offset(i as isize) - *low.offset(i as isize);
        let fresh6 = kc_lower;
        kc_lower = kc_lower.offset(1);
        *fresh6 = price_ema - multiple * tr_ema;
        let fresh7 = kc_middle;
        kc_middle = kc_middle.offset(1);
        *fresh7 = price_ema;
        let fresh8 = kc_upper;
        kc_upper = kc_upper.offset(1);
        *fresh8 = price_ema + multiple * tr_ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        let mut idx_0: libc::c_int = (*stream).buf_info.index_close + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_close {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_0 as isize) = *close.offset(i as isize);
        (*stream).buf_info.index_close = idx_0;
        price_ema = (*close.offset(i as isize) - price_ema) * per + price_ema;
        let mut idx_1: libc::c_int = (*stream).buf_info.index_close
            + -(1 as libc::c_int);
        while idx_1 >= (*stream).buf_info.size_close {
            idx_1 -= (*stream).buf_info.size_close;
        }
        while idx_1 < 0 as libc::c_int {
            idx_1 += (*stream).buf_info.size_close;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_1 as isize);
        let mut truerange: libc::c_double = 0.;
        let ych: libc::c_double = fabs(*high.offset(i as isize) - var1);
        let ycl: libc::c_double = fabs(*low.offset(i as isize) - var1);
        let mut v: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if ych > v {
            v = ych;
        }
        if ycl > v {
            v = ycl;
        }
        truerange = v;
        tr_ema = (truerange - tr_ema) * per + tr_ema;
        let fresh9 = kc_lower;
        kc_lower = kc_lower.offset(1);
        *fresh9 = price_ema - multiple * tr_ema;
        let fresh10 = kc_middle;
        kc_middle = kc_middle.offset(1);
        *fresh10 = price_ema;
        let fresh11 = kc_upper;
        kc_upper = kc_upper.offset(1);
        *fresh11 = price_ema + multiple * tr_ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.price_ema = price_ema;
    (*stream).state.tr_ema = tr_ema;
    return 0 as libc::c_int;
}
