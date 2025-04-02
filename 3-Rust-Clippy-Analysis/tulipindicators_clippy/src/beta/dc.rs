use ::libc;
extern "C" {
    pub type ti_stream;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub struct ti_stream_de {
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
    pub max: libc::c_double,
    pub max_idx: libc::c_int,
    pub min: libc::c_double,
    pub min_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub period: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_dc_start(mut options: *const libc::c_double) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return period - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut dc_lower: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut dc_upper: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_dc_start(options) {
        return 0 as libc::c_int;
    }
    ti_min(size, &mut real, options, &mut dc_lower);
    ti_max(size, &mut real, options, &mut dc_upper);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dc_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_dc_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_de = stream_in as *mut ti_stream_de;
    let mut progress: libc::c_int = (*stream).progress;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut dc_lower: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut dc_upper: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = (*stream).options.period;
    let mut max: libc::c_double = (*stream).state.max;
    let mut max_idx: libc::c_int = (*stream).state.max_idx;
    let mut min: libc::c_double = (*stream).state.min;
    let mut min_idx: libc::c_int = (*stream).state.min_idx;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size && progress == -period + 1 as libc::c_int {
        let mut idx: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_price {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx;
        max = *real.offset(i as isize);
        max_idx = progress;
        min = *real.offset(i as isize);
        min_idx = progress;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size && progress < 0 as libc::c_int {
        let mut idx_0: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_price {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_0 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_0;
        if max <= *real.offset(i as isize) {
            max = *real.offset(i as isize);
            max_idx = progress;
        }
        if min >= *real.offset(i as isize) {
            min = *real.offset(i as isize);
            min_idx = progress;
        }
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        let mut idx_1: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_1 == (*stream).buf_info.size_price {
            idx_1 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_1 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_1;
        if max_idx == progress - period {
            max_idx = progress;
            let mut idx_2: libc::c_int = (*stream).buf_info.index_price
                + 0 as libc::c_int;
            while idx_2 >= (*stream).buf_info.size_price {
                idx_2 -= (*stream).buf_info.size_price;
            }
            while idx_2 < 0 as libc::c_int {
                idx_2 += (*stream).buf_info.size_price;
            }
            max = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_price as isize)
                .offset(idx_2 as isize);
            let mut j: libc::c_int = 1 as libc::c_int;
            while j < period {
                let mut var1: libc::c_double = 0.;
                let mut idx_3: libc::c_int = (*stream).buf_info.index_price + -j;
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
                if var1 >= max {
                    max = var1;
                    max_idx = progress - j;
                }
                j += 1;
                j;
            }
        } else if *real.offset(i as isize) >= max {
            max = *real.offset(i as isize);
            max_idx = progress;
        }
        if min_idx == progress - period {
            min_idx = progress;
            let mut idx_4: libc::c_int = (*stream).buf_info.index_price
                + 0 as libc::c_int;
            while idx_4 >= (*stream).buf_info.size_price {
                idx_4 -= (*stream).buf_info.size_price;
            }
            while idx_4 < 0 as libc::c_int {
                idx_4 += (*stream).buf_info.size_price;
            }
            min = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_price as isize)
                .offset(idx_4 as isize);
            let mut j_0: libc::c_int = 1 as libc::c_int;
            while j_0 < period {
                let mut var1_0: libc::c_double = 0.;
                let mut idx_5: libc::c_int = (*stream).buf_info.index_price + -j_0;
                while idx_5 >= (*stream).buf_info.size_price {
                    idx_5 -= (*stream).buf_info.size_price;
                }
                while idx_5 < 0 as libc::c_int {
                    idx_5 += (*stream).buf_info.size_price;
                }
                var1_0 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                    .offset((*stream).buf_info.offset_price as isize)
                    .offset(idx_5 as isize);
                if var1_0 <= min {
                    min = var1_0;
                    min_idx = progress - j_0;
                }
                j_0 += 1;
                j_0;
            }
        } else if *real.offset(i as isize) <= min {
            min = *real.offset(i as isize);
            min_idx = progress;
        }
        let fresh0 = dc_lower;
        dc_lower = dc_lower.offset(1);
        *fresh0 = min;
        let fresh1 = dc_upper;
        dc_upper = dc_upper.offset(1);
        *fresh1 = max;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.max = max;
    (*stream).state.max_idx = max_idx;
    (*stream).state.min = min;
    (*stream).state.min_idx = min_idx;
    return 0 as libc::c_int;
}
