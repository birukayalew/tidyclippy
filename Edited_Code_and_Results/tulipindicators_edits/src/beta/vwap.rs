use ::libc;
extern "C" {
    pub type ti_stream;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_typprice_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_typprice(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_vwap {
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
    pub offset_volume: libc::c_int,
    pub size_volume: libc::c_int,
    pub index_volume: libc::c_int,
    pub padding_volume: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: libc::c_double,
    pub den: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub period: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwap_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwap(
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
    let mut vwap: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut progress: libc::c_int = -period + 1 as libc::c_int;
    let mut num: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut den: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size && progress < 1 as libc::c_int {
        num
            += (*high.offset(i as isize) + *low.offset(i as isize)
                + *close.offset(i as isize)) / 3.0f64 * *volume.offset(i as isize);
        den += *volume.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        let fresh0 = vwap;
        vwap = vwap.offset(1);
        *fresh0 = num / den;
    }
    while i < size {
        num
            += (*high.offset(i as isize) + *low.offset(i as isize)
                + *close.offset(i as isize)) / 3.0f64 * *volume.offset(i as isize)
                - (*high.offset((i - period) as isize)
                    + *low.offset((i - period) as isize)
                    + *close.offset((i - period) as isize)) / 3.0f64
                    * *volume.offset((i - period) as isize);
        den += *volume.offset(i as isize) - *volume.offset((i - period) as isize);
        let fresh1 = vwap;
        vwap = vwap.offset(1);
        *fresh1 = num / den;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwap_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let mut vwap: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    let mut outsize: libc::c_int = size - ti_typprice_start(&period);
    let mut typprice: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    ti_typprice(size, inputs, 0 as *const libc::c_double, &mut typprice);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut vsum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size && (i as libc::c_double) < period {
        sum += *typprice.offset(i as isize) * *volume.offset(i as isize);
        vsum += *volume.offset(i as isize);
        i += 1;
        i;
    }
    let fresh2 = vwap;
    vwap = vwap.offset(1);
    *fresh2 = sum / vsum;
    i = period as libc::c_int;
    while i < size {
        sum += *typprice.offset(i as isize) * *volume.offset(i as isize);
        sum
            -= *typprice.offset((i - period as libc::c_int) as isize)
                * *volume.offset((i - period as libc::c_int) as isize);
        vsum += *volume.offset(i as isize);
        vsum -= *volume.offset((i - period as libc::c_int) as isize);
        let fresh3 = vwap;
        vwap = vwap.offset(1);
        *fresh3 = sum / vsum;
        i += 1;
        i;
    }
    free(typprice as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwap_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_vwap_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_vwap = stream_in as *mut ti_stream_vwap;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let mut vwap: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut progress: libc::c_int = (*stream).progress;
    let period: libc::c_int = (*stream).options.period;
    let mut num: libc::c_double = (*stream).state.num;
    let mut den: libc::c_double = (*stream).state.den;
    let mut var1: libc::c_double = 0.;
    let mut var2: libc::c_double = 0.;
    let mut var3: libc::c_double = 0.;
    let mut var4: libc::c_double = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size && progress < 1 as libc::c_int {
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
        let mut idx_2: libc::c_int = (*stream).buf_info.index_volume + 1 as libc::c_int;
        if idx_2 == (*stream).buf_info.size_volume {
            idx_2 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_volume as isize)
            .offset(idx_2 as isize) = *volume.offset(i as isize);
        (*stream).buf_info.index_volume = idx_2;
        num
            += (*high.offset(i as isize) + *low.offset(i as isize)
                + *close.offset(i as isize)) / 3.0f64 * *volume.offset(i as isize);
        den += *volume.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        let fresh4 = vwap;
        vwap = vwap.offset(1);
        *fresh4 = num / den;
    }
    while i < size {
        let mut idx_3: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_3 == (*stream).buf_info.size_high {
            idx_3 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_3 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_3;
        let mut idx_4: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_4 == (*stream).buf_info.size_low {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_4 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_4;
        let mut idx_5: libc::c_int = (*stream).buf_info.index_close + 1 as libc::c_int;
        if idx_5 == (*stream).buf_info.size_close {
            idx_5 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_5 as isize) = *close.offset(i as isize);
        (*stream).buf_info.index_close = idx_5;
        let mut idx_6: libc::c_int = (*stream).buf_info.index_volume + 1 as libc::c_int;
        if idx_6 == (*stream).buf_info.size_volume {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_volume as isize)
            .offset(idx_6 as isize) = *volume.offset(i as isize);
        (*stream).buf_info.index_volume = idx_6;
        let mut idx_7: libc::c_int = (*stream).buf_info.index_high + -period;
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
        let mut idx_8: libc::c_int = (*stream).buf_info.index_low + -period;
        while idx_8 >= (*stream).buf_info.size_low {
            idx_8 -= (*stream).buf_info.size_low;
        }
        while idx_8 < 0 as libc::c_int {
            idx_8 += (*stream).buf_info.size_low;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_8 as isize);
        let mut idx_9: libc::c_int = (*stream).buf_info.index_close + -period;
        while idx_9 >= (*stream).buf_info.size_close {
            idx_9 -= (*stream).buf_info.size_close;
        }
        while idx_9 < 0 as libc::c_int {
            idx_9 += (*stream).buf_info.size_close;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_close as isize)
            .offset(idx_9 as isize);
        let mut idx_10: libc::c_int = (*stream).buf_info.index_volume + -period;
        while idx_10 >= (*stream).buf_info.size_volume {
            idx_10 -= (*stream).buf_info.size_volume;
        }
        while idx_10 < 0 as libc::c_int {
            idx_10 += (*stream).buf_info.size_volume;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_volume as isize)
            .offset(idx_10 as isize);
        num
            += (*high.offset(i as isize) + *low.offset(i as isize)
                + *close.offset(i as isize)) / 3.0f64 * *volume.offset(i as isize)
                - (var1 + var2 + var3) / 3.0f64 * var4;
        den += *volume.offset(i as isize) - var4;
        let fresh5 = vwap;
        vwap = vwap.offset(1);
        *fresh5 = num / den;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.num = num;
    (*stream).state.den = den;
    return 0 as libc::c_int;
}
