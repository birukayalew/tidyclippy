use ::libc;
extern "C" {
    pub type ti_stream;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_roc_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_roc(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_wma(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub buf_info: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub padding: libc::c_double,
    pub zero: [libc::c_int; 4],
    pub offset_price: libc::c_int,
    pub size_price: libc::c_int,
    pub index_price: libc::c_int,
    pub padding_price: libc::c_int,
    pub offset_rocs: libc::c_int,
    pub size_rocs: libc::c_int,
    pub index_rocs: libc::c_int,
    pub padding_rocs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_copp {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub options: C2RustUnnamed_3,
    pub state: C2RustUnnamed_2,
    pub buf_info: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub padding: libc::c_double,
    pub zero: [libc::c_int; 4],
    pub offset_price: libc::c_int,
    pub size_price: libc::c_int,
    pub index_price: libc::c_int,
    pub padding_price: libc::c_int,
    pub offset_rocs: libc::c_int,
    pub size_rocs: libc::c_int,
    pub index_rocs: libc::c_int,
    pub padding_rocs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub price_sum: libc::c_double,
    pub denominator: libc::c_double,
    pub rocs_per: libc::c_double,
    pub flat_rocs_sum: libc::c_double,
    pub weighted_rocs_sum: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub roc_shorter_period: libc::c_int,
    pub roc_longer_period: libc::c_int,
    pub wma_period: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_copp_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let roc_longer_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let wma_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    return ((roc_longer_period + wma_period) as libc::c_double - 1.0f64) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_copp(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let roc_shorter_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let roc_longer_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let wma_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    let mut copp: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if roc_longer_period < roc_shorter_period {
        return 1 as libc::c_int;
    }
    if size <= ti_copp_start(options) {
        return 0 as libc::c_int;
    }
    let mut buffers: *mut C2RustUnnamed = 0 as *mut C2RustUnnamed;
    buffers = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
    ) as *mut C2RustUnnamed;
    (*buffers)
        .buf_info
        .offset_price = *(&mut (*buffers).buf_info.offset_price as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*buffers).buf_info.offset_price as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*buffers).buf_info.size_price = roc_longer_period + 1 as libc::c_int;
    (*buffers).buf_info.index_price = -(1 as libc::c_int);
    (*buffers)
        .buf_info
        .offset_rocs = *(&mut (*buffers).buf_info.offset_rocs as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*buffers).buf_info.offset_rocs as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*buffers).buf_info.size_rocs = wma_period + 1 as libc::c_int;
    (*buffers).buf_info.index_rocs = -(1 as libc::c_int);
    buffers = realloc(
        buffers as *mut libc::c_void,
        (::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_double>()
                    * (*((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                        .offset(-(3 as libc::c_int as isize))
                        + *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
                            .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                            .offset(-(4 as libc::c_int as isize))) as usize)
                    as libc::c_ulong,
            ),
    ) as *mut C2RustUnnamed;
    let mut denominator: libc::c_double = 1.0f64
        / (wma_period as libc::c_double * (wma_period as libc::c_double + 1.0f64)
            / 2.0f64);
    let mut flat_rocs_sum: libc::c_double = 0.0f64;
    let mut weighted_rocs_sum: libc::c_double = 0.0f64;
    let mut var1: libc::c_double = 0.;
    let mut var2: libc::c_double = 0.;
    let mut var3: libc::c_double = 0.;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < roc_longer_period {
        let mut idx: libc::c_int = (*buffers).buf_info.index_price + 1 as libc::c_int;
        if idx == (*buffers).buf_info.size_price {
            idx = 0 as libc::c_int;
        }
        *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx as isize) = *real.offset(i_0 as isize);
        (*buffers).buf_info.index_price = idx;
        i_0 += 1;
        i_0;
    }
    while i_0 <= roc_longer_period + wma_period - 1 as libc::c_int {
        let mut idx_0: libc::c_int = (*buffers).buf_info.index_price + 1 as libc::c_int;
        if idx_0 == (*buffers).buf_info.size_price {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_0 as isize) = *real.offset(i_0 as isize);
        (*buffers).buf_info.index_price = idx_0;
        let mut idx_1: libc::c_int = (*buffers).buf_info.index_price
            + -roc_shorter_period;
        while idx_1 >= (*buffers).buf_info.size_price {
            idx_1 -= (*buffers).buf_info.size_price;
        }
        while idx_1 < 0 as libc::c_int {
            idx_1 += (*buffers).buf_info.size_price;
        }
        var1 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_1 as isize);
        let mut idx_2: libc::c_int = (*buffers).buf_info.index_price
            + -roc_longer_period;
        while idx_2 >= (*buffers).buf_info.size_price {
            idx_2 -= (*buffers).buf_info.size_price;
        }
        while idx_2 < 0 as libc::c_int {
            idx_2 += (*buffers).buf_info.size_price;
        }
        var2 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_2 as isize);
        let mut idx_3: libc::c_int = (*buffers).buf_info.index_price + 0 as libc::c_int;
        while idx_3 >= (*buffers).buf_info.size_price {
            idx_3 -= (*buffers).buf_info.size_price;
        }
        while idx_3 < 0 as libc::c_int {
            idx_3 += (*buffers).buf_info.size_price;
        }
        var3 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_3 as isize);
        let mut idx_4: libc::c_int = (*buffers).buf_info.index_rocs + 1 as libc::c_int;
        if idx_4 == (*buffers).buf_info.size_rocs {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_rocs as isize)
            .offset(
                idx_4 as isize,
            ) = (var3 / var1 - 1 as libc::c_int as libc::c_double
            + (var3 / var2 - 1 as libc::c_int as libc::c_double)) / 2.0f64 * 100.0f64;
        (*buffers).buf_info.index_rocs = idx_4;
        let mut idx_5: libc::c_int = (*buffers).buf_info.index_rocs + 0 as libc::c_int;
        while idx_5 >= (*buffers).buf_info.size_rocs {
            idx_5 -= (*buffers).buf_info.size_rocs;
        }
        while idx_5 < 0 as libc::c_int {
            idx_5 += (*buffers).buf_info.size_rocs;
        }
        var1 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_rocs as isize)
            .offset(idx_5 as isize);
        flat_rocs_sum += var1;
        weighted_rocs_sum
            += var1 * (i_0 + 1 as libc::c_int - roc_longer_period) as libc::c_double;
        i_0 += 1;
        i_0;
    }
    if i_0 == roc_longer_period + wma_period {
        let fresh0 = copp;
        copp = copp.offset(1);
        *fresh0 = weighted_rocs_sum * denominator;
    }
    while i_0 < size {
        let mut idx_6: libc::c_int = (*buffers).buf_info.index_price + 1 as libc::c_int;
        if idx_6 == (*buffers).buf_info.size_price {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_6 as isize) = *real.offset(i_0 as isize);
        (*buffers).buf_info.index_price = idx_6;
        let mut idx_7: libc::c_int = (*buffers).buf_info.index_price
            + -roc_shorter_period;
        while idx_7 >= (*buffers).buf_info.size_price {
            idx_7 -= (*buffers).buf_info.size_price;
        }
        while idx_7 < 0 as libc::c_int {
            idx_7 += (*buffers).buf_info.size_price;
        }
        var1 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_7 as isize);
        let mut idx_8: libc::c_int = (*buffers).buf_info.index_price
            + -roc_longer_period;
        while idx_8 >= (*buffers).buf_info.size_price {
            idx_8 -= (*buffers).buf_info.size_price;
        }
        while idx_8 < 0 as libc::c_int {
            idx_8 += (*buffers).buf_info.size_price;
        }
        var2 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_8 as isize);
        let mut idx_9: libc::c_int = (*buffers).buf_info.index_price + 0 as libc::c_int;
        while idx_9 >= (*buffers).buf_info.size_price {
            idx_9 -= (*buffers).buf_info.size_price;
        }
        while idx_9 < 0 as libc::c_int {
            idx_9 += (*buffers).buf_info.size_price;
        }
        var3 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_price as isize)
            .offset(idx_9 as isize);
        let mut idx_10: libc::c_int = (*buffers).buf_info.index_rocs + 1 as libc::c_int;
        if idx_10 == (*buffers).buf_info.size_rocs {
            idx_10 = 0 as libc::c_int;
        }
        *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_rocs as isize)
            .offset(
                idx_10 as isize,
            ) = (var3 / var1 - 1 as libc::c_int as libc::c_double
            + (var3 / var2 - 1 as libc::c_int as libc::c_double)) / 2.0f64 * 100.0f64;
        (*buffers).buf_info.index_rocs = idx_10;
        let mut idx_11: libc::c_int = (*buffers).buf_info.index_rocs + 0 as libc::c_int;
        while idx_11 >= (*buffers).buf_info.size_rocs {
            idx_11 -= (*buffers).buf_info.size_rocs;
        }
        while idx_11 < 0 as libc::c_int {
            idx_11 += (*buffers).buf_info.size_rocs;
        }
        var1 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_rocs as isize)
            .offset(idx_11 as isize);
        let mut idx_12: libc::c_int = (*buffers).buf_info.index_rocs + -wma_period;
        while idx_12 >= (*buffers).buf_info.size_rocs {
            idx_12 -= (*buffers).buf_info.size_rocs;
        }
        while idx_12 < 0 as libc::c_int {
            idx_12 += (*buffers).buf_info.size_rocs;
        }
        var2 = *((&mut (*buffers).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*buffers).buf_info.offset_rocs as isize)
            .offset(idx_12 as isize);
        weighted_rocs_sum += -flat_rocs_sum + var1 * wma_period as libc::c_double;
        flat_rocs_sum += -var2 + var1;
        let fresh1 = copp;
        copp = copp.offset(1);
        *fresh1 = weighted_rocs_sum * denominator;
        i_0 += 1;
        i_0;
    }
    free(buffers as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_copp_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let roc_shorter_period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let roc_longer_period: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let wma_period: libc::c_double = *options.offset(2 as libc::c_int as isize);
    let mut copp: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if roc_longer_period < roc_shorter_period {
        return 1 as libc::c_int;
    }
    if size < ti_copp_start(options) {
        return 0 as libc::c_int;
    }
    let mut roc_short_len: libc::c_int = size - ti_roc_start(&roc_shorter_period);
    let mut roc_short: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() * roc_short_len as usize)
            as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut roc_long_len: libc::c_int = size - ti_roc_start(&roc_longer_period);
    let mut roc_long: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() * roc_long_len as usize)
            as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut interm: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() * roc_long_len as usize)
            as libc::c_ulong,
    ) as *mut libc::c_double;
    ti_roc(size, &mut real, &roc_shorter_period, &mut roc_short);
    ti_roc(size, &mut real, &roc_longer_period, &mut roc_long);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < roc_long_len {
        *interm
            .offset(
                i_0 as isize,
            ) = (*roc_long.offset(i_0 as isize)
            + *roc_short.offset((i_0 + (roc_short_len - roc_long_len)) as isize))
            * 100.0f64 / 2.0f64;
        i_0 += 1;
        i_0;
    }
    let mut ti_wma_inputs: [*const libc::c_double; 1] = [
        interm as *const libc::c_double,
    ];
    ti_wma(roc_long_len, ti_wma_inputs.as_mut_ptr(), &wma_period, &mut copp);
    free(roc_short as *mut libc::c_void);
    free(roc_long as *mut libc::c_void);
    free(interm as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_copp_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_copp_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_copp = stream_in as *mut ti_stream_copp;
    let mut progress: libc::c_int = (*stream).progress;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut copp: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let roc_shorter_period: libc::c_int = (*stream).options.roc_shorter_period;
    let roc_longer_period: libc::c_int = (*stream).options.roc_longer_period;
    let wma_period: libc::c_int = (*stream).options.wma_period;
    let mut denominator: libc::c_double = (*stream).state.denominator;
    let mut rocs_per: libc::c_double = (*stream).state.rocs_per;
    let mut flat_rocs_sum: libc::c_double = (*stream).state.flat_rocs_sum;
    let mut weighted_rocs_sum: libc::c_double = (*stream).state.weighted_rocs_sum;
    let mut var1: libc::c_double = 0.;
    let mut var2: libc::c_double = 0.;
    let mut var3: libc::c_double = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size
        && (progress as libc::c_double)
            < -((roc_longer_period + wma_period) as libc::c_double - 1.0f64)
                + roc_longer_period as libc::c_double
    {
        let mut idx: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_price {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size
        && progress as libc::c_double
            <= -((roc_longer_period + wma_period) as libc::c_double - 1.0f64)
                + roc_longer_period as libc::c_double + wma_period as libc::c_double
                - 1 as libc::c_int as libc::c_double
    {
        let mut idx_0: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_price {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_0 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_0;
        let mut idx_1: libc::c_int = (*stream).buf_info.index_price
            + -roc_shorter_period;
        while idx_1 >= (*stream).buf_info.size_price {
            idx_1 -= (*stream).buf_info.size_price;
        }
        while idx_1 < 0 as libc::c_int {
            idx_1 += (*stream).buf_info.size_price;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_1 as isize);
        let mut idx_2: libc::c_int = (*stream).buf_info.index_price + -roc_longer_period;
        while idx_2 >= (*stream).buf_info.size_price {
            idx_2 -= (*stream).buf_info.size_price;
        }
        while idx_2 < 0 as libc::c_int {
            idx_2 += (*stream).buf_info.size_price;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_2 as isize);
        let mut idx_3: libc::c_int = (*stream).buf_info.index_price + 0 as libc::c_int;
        while idx_3 >= (*stream).buf_info.size_price {
            idx_3 -= (*stream).buf_info.size_price;
        }
        while idx_3 < 0 as libc::c_int {
            idx_3 += (*stream).buf_info.size_price;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_3 as isize);
        let mut idx_4: libc::c_int = (*stream).buf_info.index_rocs + 1 as libc::c_int;
        if idx_4 == (*stream).buf_info.size_rocs {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_rocs as isize)
            .offset(
                idx_4 as isize,
            ) = (var3 / var1 - 1 as libc::c_int as libc::c_double
            + (var3 / var2 - 1 as libc::c_int as libc::c_double)) * rocs_per;
        (*stream).buf_info.index_rocs = idx_4;
        let mut idx_5: libc::c_int = (*stream).buf_info.index_rocs + 0 as libc::c_int;
        while idx_5 >= (*stream).buf_info.size_rocs {
            idx_5 -= (*stream).buf_info.size_rocs;
        }
        while idx_5 < 0 as libc::c_int {
            idx_5 += (*stream).buf_info.size_rocs;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_rocs as isize)
            .offset(idx_5 as isize);
        flat_rocs_sum += var1;
        weighted_rocs_sum
            += var1
                * ((progress + 1 as libc::c_int) as libc::c_double
                    - (-((roc_longer_period + wma_period) as libc::c_double - 1.0f64)
                        + roc_longer_period as libc::c_double));
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int
        && progress as libc::c_double
            == -((roc_longer_period + wma_period) as libc::c_double - 1.0f64)
                + roc_longer_period as libc::c_double + wma_period as libc::c_double
    {
        let fresh2 = copp;
        copp = copp.offset(1);
        *fresh2 = weighted_rocs_sum * denominator;
    }
    while i < size {
        let mut idx_6: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_6 == (*stream).buf_info.size_price {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_6 as isize) = *real.offset(i as isize);
        (*stream).buf_info.index_price = idx_6;
        let mut idx_7: libc::c_int = (*stream).buf_info.index_price
            + -roc_shorter_period;
        while idx_7 >= (*stream).buf_info.size_price {
            idx_7 -= (*stream).buf_info.size_price;
        }
        while idx_7 < 0 as libc::c_int {
            idx_7 += (*stream).buf_info.size_price;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_7 as isize);
        let mut idx_8: libc::c_int = (*stream).buf_info.index_price + -roc_longer_period;
        while idx_8 >= (*stream).buf_info.size_price {
            idx_8 -= (*stream).buf_info.size_price;
        }
        while idx_8 < 0 as libc::c_int {
            idx_8 += (*stream).buf_info.size_price;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_8 as isize);
        let mut idx_9: libc::c_int = (*stream).buf_info.index_price + 0 as libc::c_int;
        while idx_9 >= (*stream).buf_info.size_price {
            idx_9 -= (*stream).buf_info.size_price;
        }
        while idx_9 < 0 as libc::c_int {
            idx_9 += (*stream).buf_info.size_price;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_9 as isize);
        let mut idx_10: libc::c_int = (*stream).buf_info.index_rocs + 1 as libc::c_int;
        if idx_10 == (*stream).buf_info.size_rocs {
            idx_10 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_rocs as isize)
            .offset(
                idx_10 as isize,
            ) = (var3 / var1 - 1 as libc::c_int as libc::c_double
            + (var3 / var2 - 1 as libc::c_int as libc::c_double)) * rocs_per;
        (*stream).buf_info.index_rocs = idx_10;
        let mut idx_11: libc::c_int = (*stream).buf_info.index_rocs + 0 as libc::c_int;
        while idx_11 >= (*stream).buf_info.size_rocs {
            idx_11 -= (*stream).buf_info.size_rocs;
        }
        while idx_11 < 0 as libc::c_int {
            idx_11 += (*stream).buf_info.size_rocs;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_rocs as isize)
            .offset(idx_11 as isize);
        let mut idx_12: libc::c_int = (*stream).buf_info.index_rocs + -wma_period;
        while idx_12 >= (*stream).buf_info.size_rocs {
            idx_12 -= (*stream).buf_info.size_rocs;
        }
        while idx_12 < 0 as libc::c_int {
            idx_12 += (*stream).buf_info.size_rocs;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_1)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_rocs as isize)
            .offset(idx_12 as isize);
        weighted_rocs_sum += -flat_rocs_sum + var1 * wma_period as libc::c_double;
        flat_rocs_sum += -var2 + var1;
        let fresh3 = copp;
        copp = copp.offset(1);
        *fresh3 = weighted_rocs_sum * denominator;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.flat_rocs_sum = flat_rocs_sum;
    (*stream).state.weighted_rocs_sum = weighted_rocs_sum;
    return 0 as libc::c_int;
}
