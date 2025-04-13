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
pub struct ti_stream_pc {
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
    pub period: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn ti_pc_start(mut options: *const libc::c_double) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return period - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let mut pc_low: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut pc_high: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if size <= ti_pc_start(options) {
        return 0 as libc::c_int;
    }
    ti_min(size, &mut low, &period, &mut pc_low);
    ti_max(size, &mut high, &period, &mut pc_high);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_pc_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_pc_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_pc = stream_in as *mut ti_stream_pc;
    let mut progress: libc::c_int = (*stream).progress;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut pc_low: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut pc_high: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut period: libc::c_double = (*stream).options.period;
    let mut max: libc::c_double = (*stream).state.max;
    let mut max_idx: libc::c_int = (*stream).state.max_idx;
    let mut min: libc::c_double = (*stream).state.min;
    let mut min_idx: libc::c_int = (*stream).state.min_idx;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size
        && progress as libc::c_double == -period + 1 as libc::c_int as libc::c_double
    {
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
        max = *high.offset(i as isize);
        max_idx = progress;
        min = *low.offset(i as isize);
        min_idx = progress;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size && progress < 0 as libc::c_int {
        let mut idx_1: libc::c_int = (*stream).buf_info.index_high + 1 as libc::c_int;
        if idx_1 == (*stream).buf_info.size_high {
            idx_1 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_high as isize)
            .offset(idx_1 as isize) = *high.offset(i as isize);
        (*stream).buf_info.index_high = idx_1;
        let mut idx_2: libc::c_int = (*stream).buf_info.index_low + 1 as libc::c_int;
        if idx_2 == (*stream).buf_info.size_low {
            idx_2 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_low as isize)
            .offset(idx_2 as isize) = *low.offset(i as isize);
        (*stream).buf_info.index_low = idx_2;
        if max <= *high.offset(i as isize) {
            max = *high.offset(i as isize);
            max_idx = progress;
        }
        if min >= *low.offset(i as isize) {
            min = *low.offset(i as isize);
            min_idx = progress;
        }
        i += 1;
        i;
        progress += 1;
        progress;
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
        if max_idx as libc::c_double == progress as libc::c_double - period {
            max_idx = progress;
            let mut idx_5: libc::c_int = (*stream).buf_info.index_high
                + 0 as libc::c_int;
            while idx_5 >= (*stream).buf_info.size_high {
                idx_5 -= (*stream).buf_info.size_high;
            }
            while idx_5 < 0 as libc::c_int {
                idx_5 += (*stream).buf_info.size_high;
            }
            max = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_high as isize)
                .offset(idx_5 as isize);
            let mut j: libc::c_int = 1 as libc::c_int;
            while (j as libc::c_double) < period {
                let mut var1: libc::c_double = 0.;
                let mut idx_6: libc::c_int = (*stream).buf_info.index_high + -j;
                while idx_6 >= (*stream).buf_info.size_high {
                    idx_6 -= (*stream).buf_info.size_high;
                }
                while idx_6 < 0 as libc::c_int {
                    idx_6 += (*stream).buf_info.size_high;
                }
                var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                    .offset((*stream).buf_info.offset_high as isize)
                    .offset(idx_6 as isize);
                if var1 >= max {
                    max = var1;
                    max_idx = progress - j;
                }
                j += 1;
                j;
            }
        } else if *high.offset(i as isize) >= max {
            max = *high.offset(i as isize);
            max_idx = progress;
        }
        if min_idx as libc::c_double == progress as libc::c_double - period {
            min_idx = progress;
            let mut idx_7: libc::c_int = (*stream).buf_info.index_low + 0 as libc::c_int;
            while idx_7 >= (*stream).buf_info.size_low {
                idx_7 -= (*stream).buf_info.size_low;
            }
            while idx_7 < 0 as libc::c_int {
                idx_7 += (*stream).buf_info.size_low;
            }
            min = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_low as isize)
                .offset(idx_7 as isize);
            let mut j_0: libc::c_int = 1 as libc::c_int;
            while (j_0 as libc::c_double) < period {
                let mut var1_0: libc::c_double = 0.;
                let mut idx_8: libc::c_int = (*stream).buf_info.index_low + -j_0;
                while idx_8 >= (*stream).buf_info.size_low {
                    idx_8 -= (*stream).buf_info.size_low;
                }
                while idx_8 < 0 as libc::c_int {
                    idx_8 += (*stream).buf_info.size_low;
                }
                var1_0 = *((&mut (*stream).buf_info as *mut C2RustUnnamed)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                    .offset((*stream).buf_info.offset_low as isize)
                    .offset(idx_8 as isize);
                if var1_0 <= min {
                    min = var1_0;
                    min_idx = progress - j_0;
                }
                j_0 += 1;
                j_0;
            }
        } else if *low.offset(i as isize) <= min {
            min = *low.offset(i as isize);
            min_idx = progress;
        }
        let fresh0 = pc_low;
        pc_low = pc_low.offset(1);
        *fresh0 = min;
        let fresh1 = pc_high;
        pc_high = pc_high.offset(1);
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
