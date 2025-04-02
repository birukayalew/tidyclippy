use ::libc;
extern "C" {
    pub type ti_stream;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data {
    pub buf_info: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub padding: libc::c_double,
    pub zero: [libc::c_int; 4],
    pub offset_smooth: libc::c_int,
    pub size_smooth: libc::c_int,
    pub index_smooth: libc::c_int,
    pub padding_smooth: libc::c_int,
    pub offset_detrender: libc::c_int,
    pub size_detrender: libc::c_int,
    pub index_detrender: libc::c_int,
    pub padding_detrender: libc::c_int,
    pub offset_I1: libc::c_int,
    pub size_I1: libc::c_int,
    pub index_I1: libc::c_int,
    pub padding_I1: libc::c_int,
    pub offset_Q1: libc::c_int,
    pub size_Q1: libc::c_int,
    pub index_Q1: libc::c_int,
    pub padding_Q1: libc::c_int,
    pub offset_jI: libc::c_int,
    pub size_jI: libc::c_int,
    pub index_jI: libc::c_int,
    pub padding_jI: libc::c_int,
    pub offset_jQ: libc::c_int,
    pub size_jQ: libc::c_int,
    pub index_jQ: libc::c_int,
    pub padding_jQ: libc::c_int,
    pub offset_I2: libc::c_int,
    pub size_I2: libc::c_int,
    pub index_I2: libc::c_int,
    pub padding_I2: libc::c_int,
    pub offset_Q2: libc::c_int,
    pub size_Q2: libc::c_int,
    pub index_Q2: libc::c_int,
    pub padding_Q2: libc::c_int,
    pub offset_Re: libc::c_int,
    pub size_Re: libc::c_int,
    pub index_Re: libc::c_int,
    pub padding_Re: libc::c_int,
    pub offset_Im: libc::c_int,
    pub size_Im: libc::c_int,
    pub index_Im: libc::c_int,
    pub padding_Im: libc::c_int,
    pub offset_period: libc::c_int,
    pub size_period: libc::c_int,
    pub index_period: libc::c_int,
    pub padding_period: libc::c_int,
    pub offset_smoothperiod: libc::c_int,
    pub size_smoothperiod: libc::c_int,
    pub index_smoothperiod: libc::c_int,
    pub padding_smoothperiod: libc::c_int,
    pub offset_phase: libc::c_int,
    pub size_phase: libc::c_int,
    pub index_phase: libc::c_int,
    pub padding_phase: libc::c_int,
    pub offset_deltaphase: libc::c_int,
    pub size_deltaphase: libc::c_int,
    pub index_deltaphase: libc::c_int,
    pub padding_deltaphase: libc::c_int,
    pub offset_alpha: libc::c_int,
    pub size_alpha: libc::c_int,
    pub index_alpha: libc::c_int,
    pub padding_alpha: libc::c_int,
    pub offset_mama: libc::c_int,
    pub size_mama: libc::c_int,
    pub index_mama: libc::c_int,
    pub padding_mama: libc::c_int,
    pub offset_fama: libc::c_int,
    pub size_fama: libc::c_int,
    pub index_fama: libc::c_int,
    pub padding_fama: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_mama {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub options: [libc::c_double; 2],
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
    pub offset_smooth: libc::c_int,
    pub size_smooth: libc::c_int,
    pub index_smooth: libc::c_int,
    pub padding_smooth: libc::c_int,
    pub offset_detrender: libc::c_int,
    pub size_detrender: libc::c_int,
    pub index_detrender: libc::c_int,
    pub padding_detrender: libc::c_int,
    pub offset_I1: libc::c_int,
    pub size_I1: libc::c_int,
    pub index_I1: libc::c_int,
    pub padding_I1: libc::c_int,
    pub offset_Q1: libc::c_int,
    pub size_Q1: libc::c_int,
    pub index_Q1: libc::c_int,
    pub padding_Q1: libc::c_int,
    pub offset_jI: libc::c_int,
    pub size_jI: libc::c_int,
    pub index_jI: libc::c_int,
    pub padding_jI: libc::c_int,
    pub offset_jQ: libc::c_int,
    pub size_jQ: libc::c_int,
    pub index_jQ: libc::c_int,
    pub padding_jQ: libc::c_int,
    pub offset_I2: libc::c_int,
    pub size_I2: libc::c_int,
    pub index_I2: libc::c_int,
    pub padding_I2: libc::c_int,
    pub offset_Q2: libc::c_int,
    pub size_Q2: libc::c_int,
    pub index_Q2: libc::c_int,
    pub padding_Q2: libc::c_int,
    pub offset_Re: libc::c_int,
    pub size_Re: libc::c_int,
    pub index_Re: libc::c_int,
    pub padding_Re: libc::c_int,
    pub offset_Im: libc::c_int,
    pub size_Im: libc::c_int,
    pub index_Im: libc::c_int,
    pub padding_Im: libc::c_int,
    pub offset_period: libc::c_int,
    pub size_period: libc::c_int,
    pub index_period: libc::c_int,
    pub padding_period: libc::c_int,
    pub offset_smoothperiod: libc::c_int,
    pub size_smoothperiod: libc::c_int,
    pub index_smoothperiod: libc::c_int,
    pub padding_smoothperiod: libc::c_int,
    pub offset_phase: libc::c_int,
    pub size_phase: libc::c_int,
    pub index_phase: libc::c_int,
    pub padding_phase: libc::c_int,
    pub offset_deltaphase: libc::c_int,
    pub size_deltaphase: libc::c_int,
    pub index_deltaphase: libc::c_int,
    pub padding_deltaphase: libc::c_int,
    pub offset_alpha: libc::c_int,
    pub size_alpha: libc::c_int,
    pub index_alpha: libc::c_int,
    pub padding_alpha: libc::c_int,
    pub offset_mama: libc::c_int,
    pub size_mama: libc::c_int,
    pub index_mama: libc::c_int,
    pub padding_mama: libc::c_int,
    pub offset_fama: libc::c_int,
    pub size_fama: libc::c_int,
    pub index_fama: libc::c_int,
    pub padding_fama: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn ti_mama_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 6 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mama(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let fastlimit: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let slowlimit: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if *options.offset(i as isize) < 0.0f64 || *options.offset(i as isize) > 1.0f64 {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut mama: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut fama: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut price: *const libc::c_double = real;
    let mut data_0: *mut data = 0 as *mut data;
    data_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<data>() as libc::c_ulong,
    ) as *mut data;
    (*data_0)
        .buf_info
        .offset_smooth = *(&mut (*data_0).buf_info.offset_smooth as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_smooth as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_smooth = 7 as libc::c_int;
    (*data_0).buf_info.index_smooth = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_detrender = *(&mut (*data_0).buf_info.offset_detrender
        as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_detrender as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_detrender = 7 as libc::c_int;
    (*data_0).buf_info.index_detrender = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_I1 = *(&mut (*data_0).buf_info.offset_I1 as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_I1 as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_I1 = 7 as libc::c_int;
    (*data_0).buf_info.index_I1 = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_Q1 = *(&mut (*data_0).buf_info.offset_Q1 as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_Q1 as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_Q1 = 7 as libc::c_int;
    (*data_0).buf_info.index_Q1 = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_jI = *(&mut (*data_0).buf_info.offset_jI as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_jI as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_jI = 1 as libc::c_int;
    (*data_0).buf_info.index_jI = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_jQ = *(&mut (*data_0).buf_info.offset_jQ as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_jQ as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_jQ = 1 as libc::c_int;
    (*data_0).buf_info.index_jQ = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_I2 = *(&mut (*data_0).buf_info.offset_I2 as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_I2 as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_I2 = 2 as libc::c_int;
    (*data_0).buf_info.index_I2 = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_Q2 = *(&mut (*data_0).buf_info.offset_Q2 as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_Q2 as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_Q2 = 2 as libc::c_int;
    (*data_0).buf_info.index_Q2 = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_Re = *(&mut (*data_0).buf_info.offset_Re as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_Re as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_Re = 2 as libc::c_int;
    (*data_0).buf_info.index_Re = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_Im = *(&mut (*data_0).buf_info.offset_Im as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_Im as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_Im = 2 as libc::c_int;
    (*data_0).buf_info.index_Im = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_period = *(&mut (*data_0).buf_info.offset_period as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_period as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_period = 2 as libc::c_int;
    (*data_0).buf_info.index_period = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_smoothperiod = *(&mut (*data_0).buf_info.offset_smoothperiod
        as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_smoothperiod as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_smoothperiod = 2 as libc::c_int;
    (*data_0).buf_info.index_smoothperiod = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_phase = *(&mut (*data_0).buf_info.offset_phase as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_phase as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_phase = 2 as libc::c_int;
    (*data_0).buf_info.index_phase = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_deltaphase = *(&mut (*data_0).buf_info.offset_deltaphase
        as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_deltaphase as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_deltaphase = 1 as libc::c_int;
    (*data_0).buf_info.index_deltaphase = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_alpha = *(&mut (*data_0).buf_info.offset_alpha as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_alpha as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_alpha = 1 as libc::c_int;
    (*data_0).buf_info.index_alpha = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_mama = *(&mut (*data_0).buf_info.offset_mama as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_mama as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_mama = 1 as libc::c_int;
    (*data_0).buf_info.index_mama = -(1 as libc::c_int);
    (*data_0)
        .buf_info
        .offset_fama = *(&mut (*data_0).buf_info.offset_fama as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut (*data_0).buf_info.offset_fama as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    (*data_0).buf_info.size_fama = 1 as libc::c_int;
    (*data_0).buf_info.index_fama = -(1 as libc::c_int);
    data_0 = realloc(
        data_0 as *mut libc::c_void,
        (::core::mem::size_of::<data>() as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<libc::c_double>()
                    * (*((&mut (*data_0).buf_info as *mut C2RustUnnamed)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                        .offset(-(3 as libc::c_int as isize))
                        + *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
                            .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                            .offset(-(4 as libc::c_int as isize))) as usize)
                    as libc::c_ulong,
            ),
    ) as *mut data;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 7 as libc::c_int {
        let mut idx: libc::c_int = (*data_0).buf_info.index_smooth + 1 as libc::c_int;
        if idx == (*data_0).buf_info.size_smooth {
            idx = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smooth as isize)
            .offset(idx as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_smooth = idx;
        let mut idx_0: libc::c_int = (*data_0).buf_info.index_detrender
            + 1 as libc::c_int;
        if idx_0 == (*data_0).buf_info.size_detrender {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(idx_0 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_detrender = idx_0;
        let mut idx_1: libc::c_int = (*data_0).buf_info.index_I1 + 1 as libc::c_int;
        if idx_1 == (*data_0).buf_info.size_I1 {
            idx_1 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_1 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_I1 = idx_1;
        let mut idx_2: libc::c_int = (*data_0).buf_info.index_Q1 + 1 as libc::c_int;
        if idx_2 == (*data_0).buf_info.size_Q1 {
            idx_2 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(idx_2 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_Q1 = idx_2;
        let mut idx_3: libc::c_int = (*data_0).buf_info.index_jI + 1 as libc::c_int;
        if idx_3 == (*data_0).buf_info.size_jI {
            idx_3 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_jI as isize)
            .offset(idx_3 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_jI = idx_3;
        let mut idx_4: libc::c_int = (*data_0).buf_info.index_jQ + 1 as libc::c_int;
        if idx_4 == (*data_0).buf_info.size_jQ {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_jQ as isize)
            .offset(idx_4 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_jQ = idx_4;
        let mut idx_5: libc::c_int = (*data_0).buf_info.index_I2 + 1 as libc::c_int;
        if idx_5 == (*data_0).buf_info.size_I2 {
            idx_5 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_5 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_I2 = idx_5;
        let mut idx_6: libc::c_int = (*data_0).buf_info.index_Q2 + 1 as libc::c_int;
        if idx_6 == (*data_0).buf_info.size_Q2 {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_6 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_Q2 = idx_6;
        let mut idx_7: libc::c_int = (*data_0).buf_info.index_Re + 1 as libc::c_int;
        if idx_7 == (*data_0).buf_info.size_Re {
            idx_7 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Re as isize)
            .offset(idx_7 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_Re = idx_7;
        let mut idx_8: libc::c_int = (*data_0).buf_info.index_Im + 1 as libc::c_int;
        if idx_8 == (*data_0).buf_info.size_Im {
            idx_8 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Im as isize)
            .offset(idx_8 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_Im = idx_8;
        let mut idx_9: libc::c_int = (*data_0).buf_info.index_period + 1 as libc::c_int;
        if idx_9 == (*data_0).buf_info.size_period {
            idx_9 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_9 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_period = idx_9;
        let mut idx_10: libc::c_int = (*data_0).buf_info.index_smoothperiod
            + 1 as libc::c_int;
        if idx_10 == (*data_0).buf_info.size_smoothperiod {
            idx_10 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smoothperiod as isize)
            .offset(idx_10 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_smoothperiod = idx_10;
        let mut idx_11: libc::c_int = (*data_0).buf_info.index_phase + 1 as libc::c_int;
        if idx_11 == (*data_0).buf_info.size_phase {
            idx_11 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_phase as isize)
            .offset(idx_11 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_phase = idx_11;
        let mut idx_12: libc::c_int = (*data_0).buf_info.index_deltaphase
            + 1 as libc::c_int;
        if idx_12 == (*data_0).buf_info.size_deltaphase {
            idx_12 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_deltaphase as isize)
            .offset(idx_12 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_deltaphase = idx_12;
        let mut idx_13: libc::c_int = (*data_0).buf_info.index_alpha + 1 as libc::c_int;
        if idx_13 == (*data_0).buf_info.size_alpha {
            idx_13 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_alpha as isize)
            .offset(idx_13 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_alpha = idx_13;
        let mut idx_14: libc::c_int = (*data_0).buf_info.index_mama + 1 as libc::c_int;
        if idx_14 == (*data_0).buf_info.size_mama {
            idx_14 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_mama as isize)
            .offset(idx_14 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_mama = idx_14;
        let mut idx_15: libc::c_int = (*data_0).buf_info.index_fama + 1 as libc::c_int;
        if idx_15 == (*data_0).buf_info.size_fama {
            idx_15 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_fama as isize)
            .offset(idx_15 as isize) = 0 as libc::c_int as libc::c_double;
        (*data_0).buf_info.index_fama = idx_15;
        i_0 += 1;
        i_0;
    }
    let mut var1: libc::c_double = 0.;
    let mut var2: libc::c_double = 0.;
    let mut var3: libc::c_double = 0.;
    let mut var4: libc::c_double = 0.;
    let mut var5: libc::c_double = 0.;
    let mut i_1: libc::c_int = 6 as libc::c_int;
    while i_1 < size {
        let mut idx_16: libc::c_int = (*data_0).buf_info.index_smooth + 1 as libc::c_int;
        if idx_16 == (*data_0).buf_info.size_smooth {
            idx_16 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smooth as isize)
            .offset(
                idx_16 as isize,
            ) = (4 as libc::c_int as libc::c_double * *price.offset(i_1 as isize)
            + 3 as libc::c_int as libc::c_double
                * *price.offset((i_1 - 1 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *price.offset((i_1 - 2 as libc::c_int) as isize)
            + *price.offset((i_1 - 3 as libc::c_int) as isize)) / 10.0f64;
        (*data_0).buf_info.index_smooth = idx_16;
        let mut idx_17: libc::c_int = (*data_0).buf_info.index_smooth + 0 as libc::c_int;
        while idx_17 >= (*data_0).buf_info.size_smooth {
            idx_17 -= (*data_0).buf_info.size_smooth;
        }
        while idx_17 < 0 as libc::c_int {
            idx_17 += (*data_0).buf_info.size_smooth;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smooth as isize)
            .offset(idx_17 as isize);
        let mut idx_18: libc::c_int = (*data_0).buf_info.index_smooth
            + -(2 as libc::c_int);
        while idx_18 >= (*data_0).buf_info.size_smooth {
            idx_18 -= (*data_0).buf_info.size_smooth;
        }
        while idx_18 < 0 as libc::c_int {
            idx_18 += (*data_0).buf_info.size_smooth;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smooth as isize)
            .offset(idx_18 as isize);
        let mut idx_19: libc::c_int = (*data_0).buf_info.index_smooth
            + -(4 as libc::c_int);
        while idx_19 >= (*data_0).buf_info.size_smooth {
            idx_19 -= (*data_0).buf_info.size_smooth;
        }
        while idx_19 < 0 as libc::c_int {
            idx_19 += (*data_0).buf_info.size_smooth;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smooth as isize)
            .offset(idx_19 as isize);
        let mut idx_20: libc::c_int = (*data_0).buf_info.index_smooth
            + -(6 as libc::c_int);
        while idx_20 >= (*data_0).buf_info.size_smooth {
            idx_20 -= (*data_0).buf_info.size_smooth;
        }
        while idx_20 < 0 as libc::c_int {
            idx_20 += (*data_0).buf_info.size_smooth;
        }
        var4 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smooth as isize)
            .offset(idx_20 as isize);
        let mut idx_21: libc::c_int = (*data_0).buf_info.index_period + 0 as libc::c_int;
        while idx_21 >= (*data_0).buf_info.size_period {
            idx_21 -= (*data_0).buf_info.size_period;
        }
        while idx_21 < 0 as libc::c_int {
            idx_21 += (*data_0).buf_info.size_period;
        }
        var5 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_21 as isize);
        let mut idx_22: libc::c_int = (*data_0).buf_info.index_detrender
            + 1 as libc::c_int;
        if idx_22 == (*data_0).buf_info.size_detrender {
            idx_22 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(
                idx_22 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*data_0).buf_info.index_detrender = idx_22;
        let mut idx_23: libc::c_int = (*data_0).buf_info.index_detrender
            + 0 as libc::c_int;
        while idx_23 >= (*data_0).buf_info.size_detrender {
            idx_23 -= (*data_0).buf_info.size_detrender;
        }
        while idx_23 < 0 as libc::c_int {
            idx_23 += (*data_0).buf_info.size_detrender;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(idx_23 as isize);
        let mut idx_24: libc::c_int = (*data_0).buf_info.index_detrender
            + -(2 as libc::c_int);
        while idx_24 >= (*data_0).buf_info.size_detrender {
            idx_24 -= (*data_0).buf_info.size_detrender;
        }
        while idx_24 < 0 as libc::c_int {
            idx_24 += (*data_0).buf_info.size_detrender;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(idx_24 as isize);
        let mut idx_25: libc::c_int = (*data_0).buf_info.index_detrender
            + -(4 as libc::c_int);
        while idx_25 >= (*data_0).buf_info.size_detrender {
            idx_25 -= (*data_0).buf_info.size_detrender;
        }
        while idx_25 < 0 as libc::c_int {
            idx_25 += (*data_0).buf_info.size_detrender;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(idx_25 as isize);
        let mut idx_26: libc::c_int = (*data_0).buf_info.index_detrender
            + -(6 as libc::c_int);
        while idx_26 >= (*data_0).buf_info.size_detrender {
            idx_26 -= (*data_0).buf_info.size_detrender;
        }
        while idx_26 < 0 as libc::c_int {
            idx_26 += (*data_0).buf_info.size_detrender;
        }
        var4 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(idx_26 as isize);
        let mut idx_27: libc::c_int = (*data_0).buf_info.index_period + 0 as libc::c_int;
        while idx_27 >= (*data_0).buf_info.size_period {
            idx_27 -= (*data_0).buf_info.size_period;
        }
        while idx_27 < 0 as libc::c_int {
            idx_27 += (*data_0).buf_info.size_period;
        }
        var5 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_27 as isize);
        let mut idx_28: libc::c_int = (*data_0).buf_info.index_Q1 + 1 as libc::c_int;
        if idx_28 == (*data_0).buf_info.size_Q1 {
            idx_28 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(
                idx_28 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*data_0).buf_info.index_Q1 = idx_28;
        let mut idx_29: libc::c_int = (*data_0).buf_info.index_detrender
            + -(3 as libc::c_int);
        while idx_29 >= (*data_0).buf_info.size_detrender {
            idx_29 -= (*data_0).buf_info.size_detrender;
        }
        while idx_29 < 0 as libc::c_int {
            idx_29 += (*data_0).buf_info.size_detrender;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_detrender as isize)
            .offset(idx_29 as isize);
        let mut idx_30: libc::c_int = (*data_0).buf_info.index_I1 + 1 as libc::c_int;
        if idx_30 == (*data_0).buf_info.size_I1 {
            idx_30 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_30 as isize) = var1;
        (*data_0).buf_info.index_I1 = idx_30;
        let mut idx_31: libc::c_int = (*data_0).buf_info.index_I1 + 0 as libc::c_int;
        while idx_31 >= (*data_0).buf_info.size_I1 {
            idx_31 -= (*data_0).buf_info.size_I1;
        }
        while idx_31 < 0 as libc::c_int {
            idx_31 += (*data_0).buf_info.size_I1;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_31 as isize);
        let mut idx_32: libc::c_int = (*data_0).buf_info.index_I1 + -(2 as libc::c_int);
        while idx_32 >= (*data_0).buf_info.size_I1 {
            idx_32 -= (*data_0).buf_info.size_I1;
        }
        while idx_32 < 0 as libc::c_int {
            idx_32 += (*data_0).buf_info.size_I1;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_32 as isize);
        let mut idx_33: libc::c_int = (*data_0).buf_info.index_I1 + -(4 as libc::c_int);
        while idx_33 >= (*data_0).buf_info.size_I1 {
            idx_33 -= (*data_0).buf_info.size_I1;
        }
        while idx_33 < 0 as libc::c_int {
            idx_33 += (*data_0).buf_info.size_I1;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_33 as isize);
        let mut idx_34: libc::c_int = (*data_0).buf_info.index_I1 + -(6 as libc::c_int);
        while idx_34 >= (*data_0).buf_info.size_I1 {
            idx_34 -= (*data_0).buf_info.size_I1;
        }
        while idx_34 < 0 as libc::c_int {
            idx_34 += (*data_0).buf_info.size_I1;
        }
        var4 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_34 as isize);
        let mut idx_35: libc::c_int = (*data_0).buf_info.index_period + 0 as libc::c_int;
        while idx_35 >= (*data_0).buf_info.size_period {
            idx_35 -= (*data_0).buf_info.size_period;
        }
        while idx_35 < 0 as libc::c_int {
            idx_35 += (*data_0).buf_info.size_period;
        }
        var5 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_35 as isize);
        let mut idx_36: libc::c_int = (*data_0).buf_info.index_jI + 1 as libc::c_int;
        if idx_36 == (*data_0).buf_info.size_jI {
            idx_36 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_jI as isize)
            .offset(
                idx_36 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*data_0).buf_info.index_jI = idx_36;
        let mut idx_37: libc::c_int = (*data_0).buf_info.index_Q1 + 0 as libc::c_int;
        while idx_37 >= (*data_0).buf_info.size_Q1 {
            idx_37 -= (*data_0).buf_info.size_Q1;
        }
        while idx_37 < 0 as libc::c_int {
            idx_37 += (*data_0).buf_info.size_Q1;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(idx_37 as isize);
        let mut idx_38: libc::c_int = (*data_0).buf_info.index_Q1 + -(2 as libc::c_int);
        while idx_38 >= (*data_0).buf_info.size_Q1 {
            idx_38 -= (*data_0).buf_info.size_Q1;
        }
        while idx_38 < 0 as libc::c_int {
            idx_38 += (*data_0).buf_info.size_Q1;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(idx_38 as isize);
        let mut idx_39: libc::c_int = (*data_0).buf_info.index_Q1 + -(4 as libc::c_int);
        while idx_39 >= (*data_0).buf_info.size_Q1 {
            idx_39 -= (*data_0).buf_info.size_Q1;
        }
        while idx_39 < 0 as libc::c_int {
            idx_39 += (*data_0).buf_info.size_Q1;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(idx_39 as isize);
        let mut idx_40: libc::c_int = (*data_0).buf_info.index_Q1 + -(6 as libc::c_int);
        while idx_40 >= (*data_0).buf_info.size_Q1 {
            idx_40 -= (*data_0).buf_info.size_Q1;
        }
        while idx_40 < 0 as libc::c_int {
            idx_40 += (*data_0).buf_info.size_Q1;
        }
        var4 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(idx_40 as isize);
        let mut idx_41: libc::c_int = (*data_0).buf_info.index_period + 0 as libc::c_int;
        while idx_41 >= (*data_0).buf_info.size_period {
            idx_41 -= (*data_0).buf_info.size_period;
        }
        while idx_41 < 0 as libc::c_int {
            idx_41 += (*data_0).buf_info.size_period;
        }
        var5 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_41 as isize);
        let mut idx_42: libc::c_int = (*data_0).buf_info.index_jQ + 1 as libc::c_int;
        if idx_42 == (*data_0).buf_info.size_jQ {
            idx_42 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_jQ as isize)
            .offset(
                idx_42 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*data_0).buf_info.index_jQ = idx_42;
        let mut idx_43: libc::c_int = (*data_0).buf_info.index_I1 + 0 as libc::c_int;
        while idx_43 >= (*data_0).buf_info.size_I1 {
            idx_43 -= (*data_0).buf_info.size_I1;
        }
        while idx_43 < 0 as libc::c_int {
            idx_43 += (*data_0).buf_info.size_I1;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_43 as isize);
        let mut idx_44: libc::c_int = (*data_0).buf_info.index_jQ + 0 as libc::c_int;
        while idx_44 >= (*data_0).buf_info.size_jQ {
            idx_44 -= (*data_0).buf_info.size_jQ;
        }
        while idx_44 < 0 as libc::c_int {
            idx_44 += (*data_0).buf_info.size_jQ;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_jQ as isize)
            .offset(idx_44 as isize);
        let mut idx_45: libc::c_int = (*data_0).buf_info.index_I2 + 0 as libc::c_int;
        while idx_45 >= (*data_0).buf_info.size_I2 {
            idx_45 -= (*data_0).buf_info.size_I2;
        }
        while idx_45 < 0 as libc::c_int {
            idx_45 += (*data_0).buf_info.size_I2;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_45 as isize);
        let mut idx_46: libc::c_int = (*data_0).buf_info.index_I2 + 1 as libc::c_int;
        if idx_46 == (*data_0).buf_info.size_I2 {
            idx_46 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_46 as isize) = 0.2f64 * (var1 - var2) + 0.8f64 * var3;
        (*data_0).buf_info.index_I2 = idx_46;
        let mut idx_47: libc::c_int = (*data_0).buf_info.index_Q1 + 0 as libc::c_int;
        while idx_47 >= (*data_0).buf_info.size_Q1 {
            idx_47 -= (*data_0).buf_info.size_Q1;
        }
        while idx_47 < 0 as libc::c_int {
            idx_47 += (*data_0).buf_info.size_Q1;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q1 as isize)
            .offset(idx_47 as isize);
        let mut idx_48: libc::c_int = (*data_0).buf_info.index_jI + 0 as libc::c_int;
        while idx_48 >= (*data_0).buf_info.size_jI {
            idx_48 -= (*data_0).buf_info.size_jI;
        }
        while idx_48 < 0 as libc::c_int {
            idx_48 += (*data_0).buf_info.size_jI;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_jI as isize)
            .offset(idx_48 as isize);
        let mut idx_49: libc::c_int = (*data_0).buf_info.index_Q2 + 0 as libc::c_int;
        while idx_49 >= (*data_0).buf_info.size_Q2 {
            idx_49 -= (*data_0).buf_info.size_Q2;
        }
        while idx_49 < 0 as libc::c_int {
            idx_49 += (*data_0).buf_info.size_Q2;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_49 as isize);
        let mut idx_50: libc::c_int = (*data_0).buf_info.index_Q2 + 1 as libc::c_int;
        if idx_50 == (*data_0).buf_info.size_Q2 {
            idx_50 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_50 as isize) = 0.2f64 * (var1 + var2) + 0.8f64 * var3;
        (*data_0).buf_info.index_Q2 = idx_50;
        let mut idx_51: libc::c_int = (*data_0).buf_info.index_I2 + 0 as libc::c_int;
        while idx_51 >= (*data_0).buf_info.size_I2 {
            idx_51 -= (*data_0).buf_info.size_I2;
        }
        while idx_51 < 0 as libc::c_int {
            idx_51 += (*data_0).buf_info.size_I2;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_51 as isize);
        let mut idx_52: libc::c_int = (*data_0).buf_info.index_I2 + -(1 as libc::c_int);
        while idx_52 >= (*data_0).buf_info.size_I2 {
            idx_52 -= (*data_0).buf_info.size_I2;
        }
        while idx_52 < 0 as libc::c_int {
            idx_52 += (*data_0).buf_info.size_I2;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_52 as isize);
        let mut idx_53: libc::c_int = (*data_0).buf_info.index_Q2 + 0 as libc::c_int;
        while idx_53 >= (*data_0).buf_info.size_Q2 {
            idx_53 -= (*data_0).buf_info.size_Q2;
        }
        while idx_53 < 0 as libc::c_int {
            idx_53 += (*data_0).buf_info.size_Q2;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_53 as isize);
        let mut idx_54: libc::c_int = (*data_0).buf_info.index_Q2 + -(1 as libc::c_int);
        while idx_54 >= (*data_0).buf_info.size_Q2 {
            idx_54 -= (*data_0).buf_info.size_Q2;
        }
        while idx_54 < 0 as libc::c_int {
            idx_54 += (*data_0).buf_info.size_Q2;
        }
        var4 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_54 as isize);
        let mut idx_55: libc::c_int = (*data_0).buf_info.index_Re + 0 as libc::c_int;
        while idx_55 >= (*data_0).buf_info.size_Re {
            idx_55 -= (*data_0).buf_info.size_Re;
        }
        while idx_55 < 0 as libc::c_int {
            idx_55 += (*data_0).buf_info.size_Re;
        }
        var5 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Re as isize)
            .offset(idx_55 as isize);
        let mut idx_56: libc::c_int = (*data_0).buf_info.index_Re + 1 as libc::c_int;
        if idx_56 == (*data_0).buf_info.size_Re {
            idx_56 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Re as isize)
            .offset(
                idx_56 as isize,
            ) = 0.2f64 * (var1 * var2 + var3 * var4) + 0.8f64 * var5;
        (*data_0).buf_info.index_Re = idx_56;
        let mut idx_57: libc::c_int = (*data_0).buf_info.index_I2 + 0 as libc::c_int;
        while idx_57 >= (*data_0).buf_info.size_I2 {
            idx_57 -= (*data_0).buf_info.size_I2;
        }
        while idx_57 < 0 as libc::c_int {
            idx_57 += (*data_0).buf_info.size_I2;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_57 as isize);
        let mut idx_58: libc::c_int = (*data_0).buf_info.index_Q2 + -(1 as libc::c_int);
        while idx_58 >= (*data_0).buf_info.size_Q2 {
            idx_58 -= (*data_0).buf_info.size_Q2;
        }
        while idx_58 < 0 as libc::c_int {
            idx_58 += (*data_0).buf_info.size_Q2;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_58 as isize);
        let mut idx_59: libc::c_int = (*data_0).buf_info.index_Q2 + 0 as libc::c_int;
        while idx_59 >= (*data_0).buf_info.size_Q2 {
            idx_59 -= (*data_0).buf_info.size_Q2;
        }
        while idx_59 < 0 as libc::c_int {
            idx_59 += (*data_0).buf_info.size_Q2;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Q2 as isize)
            .offset(idx_59 as isize);
        let mut idx_60: libc::c_int = (*data_0).buf_info.index_I2 + -(1 as libc::c_int);
        while idx_60 >= (*data_0).buf_info.size_I2 {
            idx_60 -= (*data_0).buf_info.size_I2;
        }
        while idx_60 < 0 as libc::c_int {
            idx_60 += (*data_0).buf_info.size_I2;
        }
        var4 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I2 as isize)
            .offset(idx_60 as isize);
        let mut idx_61: libc::c_int = (*data_0).buf_info.index_Im + 0 as libc::c_int;
        while idx_61 >= (*data_0).buf_info.size_Im {
            idx_61 -= (*data_0).buf_info.size_Im;
        }
        while idx_61 < 0 as libc::c_int {
            idx_61 += (*data_0).buf_info.size_Im;
        }
        var5 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Im as isize)
            .offset(idx_61 as isize);
        let mut idx_62: libc::c_int = (*data_0).buf_info.index_Im + 1 as libc::c_int;
        if idx_62 == (*data_0).buf_info.size_Im {
            idx_62 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Im as isize)
            .offset(
                idx_62 as isize,
            ) = 0.2f64 * (var1 * var2 - var3 * var4) + 0.8f64 * var5;
        (*data_0).buf_info.index_Im = idx_62;
        let mut period_: libc::c_double = 0.0f64;
        let mut idx_63: libc::c_int = (*data_0).buf_info.index_Im + 0 as libc::c_int;
        while idx_63 >= (*data_0).buf_info.size_Im {
            idx_63 -= (*data_0).buf_info.size_Im;
        }
        while idx_63 < 0 as libc::c_int {
            idx_63 += (*data_0).buf_info.size_Im;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Im as isize)
            .offset(idx_63 as isize);
        let mut idx_64: libc::c_int = (*data_0).buf_info.index_Re + 0 as libc::c_int;
        while idx_64 >= (*data_0).buf_info.size_Re {
            idx_64 -= (*data_0).buf_info.size_Re;
        }
        while idx_64 < 0 as libc::c_int {
            idx_64 += (*data_0).buf_info.size_Re;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_Re as isize)
            .offset(idx_64 as isize);
        let mut idx_65: libc::c_int = (*data_0).buf_info.index_period + 0 as libc::c_int;
        while idx_65 >= (*data_0).buf_info.size_period {
            idx_65 -= (*data_0).buf_info.size_period;
        }
        while idx_65 < 0 as libc::c_int {
            idx_65 += (*data_0).buf_info.size_period;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_65 as isize);
        if var1 != 0.0f64 && var2 != 0.0f64 {
            period_ = 360.0f64 / atan(var1 / var2);
        }
        if period_ > 1.5f64 * var3 {
            period_ = 1.5f64 * var3;
        }
        if period_ < 0.67f64 * var3 {
            period_ = 0.67f64 * var3;
        }
        if period_ < 6.0f64 {
            period_ = 6.0f64;
        }
        if period_ > 50.0f64 {
            period_ = 50.0f64;
        }
        let mut idx_66: libc::c_int = (*data_0).buf_info.index_period + 1 as libc::c_int;
        if idx_66 == (*data_0).buf_info.size_period {
            idx_66 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_66 as isize) = 0.2f64 * period_ + 0.8f64 * var3;
        (*data_0).buf_info.index_period = idx_66;
        let mut idx_67: libc::c_int = (*data_0).buf_info.index_period + 0 as libc::c_int;
        while idx_67 >= (*data_0).buf_info.size_period {
            idx_67 -= (*data_0).buf_info.size_period;
        }
        while idx_67 < 0 as libc::c_int {
            idx_67 += (*data_0).buf_info.size_period;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_period as isize)
            .offset(idx_67 as isize);
        let mut idx_68: libc::c_int = (*data_0).buf_info.index_smoothperiod
            + 0 as libc::c_int;
        while idx_68 >= (*data_0).buf_info.size_smoothperiod {
            idx_68 -= (*data_0).buf_info.size_smoothperiod;
        }
        while idx_68 < 0 as libc::c_int {
            idx_68 += (*data_0).buf_info.size_smoothperiod;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smoothperiod as isize)
            .offset(idx_68 as isize);
        let mut idx_69: libc::c_int = (*data_0).buf_info.index_smoothperiod
            + 1 as libc::c_int;
        if idx_69 == (*data_0).buf_info.size_smoothperiod {
            idx_69 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_smoothperiod as isize)
            .offset(idx_69 as isize) = 0.33f64 * var1 + 0.67f64 * var2;
        (*data_0).buf_info.index_smoothperiod = idx_69;
        let mut idx_70: libc::c_int = (*data_0).buf_info.index_I1 + 0 as libc::c_int;
        while idx_70 >= (*data_0).buf_info.size_I1 {
            idx_70 -= (*data_0).buf_info.size_I1;
        }
        while idx_70 < 0 as libc::c_int {
            idx_70 += (*data_0).buf_info.size_I1;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_I1 as isize)
            .offset(idx_70 as isize);
        if var1 != 0.0f64 {
            let mut idx_71: libc::c_int = (*data_0).buf_info.index_Q1 + 0 as libc::c_int;
            while idx_71 >= (*data_0).buf_info.size_Q1 {
                idx_71 -= (*data_0).buf_info.size_Q1;
            }
            while idx_71 < 0 as libc::c_int {
                idx_71 += (*data_0).buf_info.size_Q1;
            }
            var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*data_0).buf_info.offset_Q1 as isize)
                .offset(idx_71 as isize);
            let mut idx_72: libc::c_int = (*data_0).buf_info.index_phase
                + 1 as libc::c_int;
            if idx_72 == (*data_0).buf_info.size_phase {
                idx_72 = 0 as libc::c_int;
            }
            *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*data_0).buf_info.offset_phase as isize)
                .offset(idx_72 as isize) = atan(var2 / var1);
            (*data_0).buf_info.index_phase = idx_72;
        } else {
            let mut idx_73: libc::c_int = (*data_0).buf_info.index_phase
                + 1 as libc::c_int;
            if idx_73 == (*data_0).buf_info.size_phase {
                idx_73 = 0 as libc::c_int;
            }
            *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*data_0).buf_info.offset_phase as isize)
                .offset(idx_73 as isize) = 0 as libc::c_int as libc::c_double;
            (*data_0).buf_info.index_phase = idx_73;
        }
        let mut idx_74: libc::c_int = (*data_0).buf_info.index_phase
            + -(1 as libc::c_int);
        while idx_74 >= (*data_0).buf_info.size_phase {
            idx_74 -= (*data_0).buf_info.size_phase;
        }
        while idx_74 < 0 as libc::c_int {
            idx_74 += (*data_0).buf_info.size_phase;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_phase as isize)
            .offset(idx_74 as isize);
        let mut idx_75: libc::c_int = (*data_0).buf_info.index_phase + 0 as libc::c_int;
        while idx_75 >= (*data_0).buf_info.size_phase {
            idx_75 -= (*data_0).buf_info.size_phase;
        }
        while idx_75 < 0 as libc::c_int {
            idx_75 += (*data_0).buf_info.size_phase;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_phase as isize)
            .offset(idx_75 as isize);
        let mut idx_76: libc::c_int = (*data_0).buf_info.index_deltaphase
            + 1 as libc::c_int;
        if idx_76 == (*data_0).buf_info.size_deltaphase {
            idx_76 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_deltaphase as isize)
            .offset(
                idx_76 as isize,
            ) = if 1 as libc::c_int as libc::c_double > var1 - var2 {
            1 as libc::c_int as libc::c_double
        } else {
            var1 - var2
        };
        (*data_0).buf_info.index_deltaphase = idx_76;
        let mut idx_77: libc::c_int = (*data_0).buf_info.index_deltaphase
            + 0 as libc::c_int;
        while idx_77 >= (*data_0).buf_info.size_deltaphase {
            idx_77 -= (*data_0).buf_info.size_deltaphase;
        }
        while idx_77 < 0 as libc::c_int {
            idx_77 += (*data_0).buf_info.size_deltaphase;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_deltaphase as isize)
            .offset(idx_77 as isize);
        let mut idx_78: libc::c_int = (*data_0).buf_info.index_alpha + 1 as libc::c_int;
        if idx_78 == (*data_0).buf_info.size_alpha {
            idx_78 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_alpha as isize)
            .offset(
                idx_78 as isize,
            ) = if slowlimit > fastlimit / var1 { slowlimit } else { fastlimit / var1 };
        (*data_0).buf_info.index_alpha = idx_78;
        let mut idx_79: libc::c_int = (*data_0).buf_info.index_alpha + 0 as libc::c_int;
        while idx_79 >= (*data_0).buf_info.size_alpha {
            idx_79 -= (*data_0).buf_info.size_alpha;
        }
        while idx_79 < 0 as libc::c_int {
            idx_79 += (*data_0).buf_info.size_alpha;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_alpha as isize)
            .offset(idx_79 as isize);
        let mut idx_80: libc::c_int = (*data_0).buf_info.index_mama + 0 as libc::c_int;
        while idx_80 >= (*data_0).buf_info.size_mama {
            idx_80 -= (*data_0).buf_info.size_mama;
        }
        while idx_80 < 0 as libc::c_int {
            idx_80 += (*data_0).buf_info.size_mama;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_mama as isize)
            .offset(idx_80 as isize);
        let mut idx_81: libc::c_int = (*data_0).buf_info.index_mama + 1 as libc::c_int;
        if idx_81 == (*data_0).buf_info.size_mama {
            idx_81 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_mama as isize)
            .offset(
                idx_81 as isize,
            ) = var1 * *price.offset(i_1 as isize) + (1.0f64 - var1) * var2;
        (*data_0).buf_info.index_mama = idx_81;
        let mut idx_82: libc::c_int = (*data_0).buf_info.index_alpha + 0 as libc::c_int;
        while idx_82 >= (*data_0).buf_info.size_alpha {
            idx_82 -= (*data_0).buf_info.size_alpha;
        }
        while idx_82 < 0 as libc::c_int {
            idx_82 += (*data_0).buf_info.size_alpha;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_alpha as isize)
            .offset(idx_82 as isize);
        let mut idx_83: libc::c_int = (*data_0).buf_info.index_mama + 0 as libc::c_int;
        while idx_83 >= (*data_0).buf_info.size_mama {
            idx_83 -= (*data_0).buf_info.size_mama;
        }
        while idx_83 < 0 as libc::c_int {
            idx_83 += (*data_0).buf_info.size_mama;
        }
        var2 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_mama as isize)
            .offset(idx_83 as isize);
        let mut idx_84: libc::c_int = (*data_0).buf_info.index_fama + 0 as libc::c_int;
        while idx_84 >= (*data_0).buf_info.size_fama {
            idx_84 -= (*data_0).buf_info.size_fama;
        }
        while idx_84 < 0 as libc::c_int {
            idx_84 += (*data_0).buf_info.size_fama;
        }
        var3 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_fama as isize)
            .offset(idx_84 as isize);
        let mut idx_85: libc::c_int = (*data_0).buf_info.index_fama + 1 as libc::c_int;
        if idx_85 == (*data_0).buf_info.size_fama {
            idx_85 = 0 as libc::c_int;
        }
        *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_fama as isize)
            .offset(
                idx_85 as isize,
            ) = 0.5f64 * var1 * var2 + (1.0f64 - 0.5f64 * var1) * var3;
        (*data_0).buf_info.index_fama = idx_85;
        let mut idx_86: libc::c_int = (*data_0).buf_info.index_mama + 0 as libc::c_int;
        while idx_86 >= (*data_0).buf_info.size_mama {
            idx_86 -= (*data_0).buf_info.size_mama;
        }
        while idx_86 < 0 as libc::c_int {
            idx_86 += (*data_0).buf_info.size_mama;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_mama as isize)
            .offset(idx_86 as isize);
        let fresh0 = mama;
        mama = mama.offset(1);
        *fresh0 = var1;
        let mut idx_87: libc::c_int = (*data_0).buf_info.index_fama + 0 as libc::c_int;
        while idx_87 >= (*data_0).buf_info.size_fama {
            idx_87 -= (*data_0).buf_info.size_fama;
        }
        while idx_87 < 0 as libc::c_int {
            idx_87 += (*data_0).buf_info.size_fama;
        }
        var1 = *((&mut (*data_0).buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*data_0).buf_info.offset_fama as isize)
            .offset(idx_87 as isize);
        let fresh1 = fama;
        fama = fama.offset(1);
        *fresh1 = var1;
        i_1 += 1;
        i_1;
    }
    free(data_0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mama_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let fastlimit: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let slowlimit: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut mama: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut fama: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if *options.offset(i as isize) < 0.0f64 || *options.offset(i as isize) > 1.0f64 {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut price: *const libc::c_double = real;
    let usize: libc::c_uint = size as libc::c_uint;
    let mut smooth: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut detrender: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut I1: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Q1: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut jI: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut jQ: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut I2: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Q2: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Re: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut Im: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut period: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut smoothperiod: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut phase: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut deltaphase: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut alpha: *mut libc::c_double = calloc(
        usize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut i_0: libc::c_int = 6 as libc::c_int;
    while i_0 < size {
        *smooth
            .offset(
                i_0 as isize,
            ) = (4 as libc::c_int as libc::c_double * *price.offset(i_0 as isize)
            + 3 as libc::c_int as libc::c_double
                * *price.offset((i_0 - 1 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *price.offset((i_0 - 2 as libc::c_int) as isize)
            + *price.offset((i_0 - 3 as libc::c_int) as isize)) / 10.0f64;
        *detrender
            .offset(
                i_0 as isize,
            ) = (0.0962f64 * *smooth.offset(i_0 as isize)
            + 0.5769f64 * *smooth.offset((i_0 - 2 as libc::c_int) as isize)
            - 0.5769f64 * *smooth.offset((i_0 - 4 as libc::c_int) as isize)
            - 0.0962f64 * *smooth.offset((i_0 - 6 as libc::c_int) as isize))
            * (0.075f64 * *period.offset((i_0 - 1 as libc::c_int) as isize) + 0.54f64);
        *Q1
            .offset(
                i_0 as isize,
            ) = (0.0962f64 * *detrender.offset(i_0 as isize)
            + 0.5769f64 * *detrender.offset((i_0 - 2 as libc::c_int) as isize)
            - 0.5769f64 * *detrender.offset((i_0 - 4 as libc::c_int) as isize)
            - 0.0962f64 * *detrender.offset((i_0 - 6 as libc::c_int) as isize))
            * (0.075f64 * *period.offset((i_0 - 1 as libc::c_int) as isize) + 0.54f64);
        *I1.offset(i_0 as isize) = *detrender.offset((i_0 - 3 as libc::c_int) as isize);
        *jI
            .offset(
                i_0 as isize,
            ) = (0.0962f64 * *I1.offset(i_0 as isize)
            + 0.5769f64 * *I1.offset((i_0 - 2 as libc::c_int) as isize)
            - 0.5769f64 * *I1.offset((i_0 - 4 as libc::c_int) as isize)
            - 0.0962f64 * *I1.offset((i_0 - 6 as libc::c_int) as isize))
            * (0.075f64 * *period.offset((i_0 - 1 as libc::c_int) as isize) + 0.54f64);
        *jQ
            .offset(
                i_0 as isize,
            ) = (0.0962f64 * *Q1.offset(i_0 as isize)
            + 0.5769f64 * *Q1.offset((i_0 - 2 as libc::c_int) as isize)
            - 0.5769f64 * *Q1.offset((i_0 - 4 as libc::c_int) as isize)
            - 0.0962f64 * *Q1.offset((i_0 - 6 as libc::c_int) as isize))
            * (0.075f64 * *period.offset((i_0 - 1 as libc::c_int) as isize) + 0.54f64);
        *I2.offset(i_0 as isize) = *I1.offset(i_0 as isize) - *jQ.offset(i_0 as isize);
        *Q2.offset(i_0 as isize) = *Q1.offset(i_0 as isize) + *jI.offset(i_0 as isize);
        *I2
            .offset(
                i_0 as isize,
            ) = 0.2f64 * *I2.offset(i_0 as isize)
            + 0.8f64 * *I2.offset((i_0 - 1 as libc::c_int) as isize);
        *Q2
            .offset(
                i_0 as isize,
            ) = 0.2f64 * *Q2.offset(i_0 as isize)
            + 0.8f64 * *Q2.offset((i_0 - 1 as libc::c_int) as isize);
        *Re
            .offset(
                i_0 as isize,
            ) = *I2.offset(i_0 as isize) * *I2.offset((i_0 - 1 as libc::c_int) as isize)
            + *Q2.offset(i_0 as isize) * *Q2.offset((i_0 - 1 as libc::c_int) as isize);
        *Im
            .offset(
                i_0 as isize,
            ) = *I2.offset(i_0 as isize) * *Q2.offset((i_0 - 1 as libc::c_int) as isize)
            - *Q2.offset(i_0 as isize) * *I2.offset((i_0 - 1 as libc::c_int) as isize);
        *Re
            .offset(
                i_0 as isize,
            ) = 0.2f64 * *Re.offset(i_0 as isize)
            + 0.8f64 * *Re.offset((i_0 - 1 as libc::c_int) as isize);
        *Im
            .offset(
                i_0 as isize,
            ) = 0.2f64 * *Im.offset(i_0 as isize)
            + 0.8f64 * *Im.offset((i_0 - 1 as libc::c_int) as isize);
        if *Im.offset(i_0 as isize) != 0.0f64 && *Re.offset(i_0 as isize) != 0.0f64 {
            *period
                .offset(
                    i_0 as isize,
                ) = 360.0f64 / atan(*Im.offset(i_0 as isize) / *Re.offset(i_0 as isize));
        }
        if *period.offset(i_0 as isize)
            > 1.5f64 * *period.offset((i_0 - 1 as libc::c_int) as isize)
        {
            *period
                .offset(
                    i_0 as isize,
                ) = 1.5f64 * *period.offset((i_0 - 1 as libc::c_int) as isize);
        }
        if *period.offset(i_0 as isize)
            < 0.67f64 * *period.offset((i_0 - 1 as libc::c_int) as isize)
        {
            *period
                .offset(
                    i_0 as isize,
                ) = 0.67f64 * *period.offset((i_0 - 1 as libc::c_int) as isize);
        }
        if *period.offset(i_0 as isize) < 6.0f64 {
            *period.offset(i_0 as isize) = 6.0f64;
        }
        if *period.offset(i_0 as isize) > 50.0f64 {
            *period.offset(i_0 as isize) = 50.0f64;
        }
        *period
            .offset(
                i_0 as isize,
            ) = 0.2f64 * *period.offset(i_0 as isize)
            + 0.8f64 * *period.offset((i_0 - 1 as libc::c_int) as isize);
        *smoothperiod
            .offset(
                i_0 as isize,
            ) = 0.33f64 * *period.offset(i_0 as isize)
            + 0.67f64 * *smoothperiod.offset((i_0 - 1 as libc::c_int) as isize);
        if *I1.offset(i_0 as isize) != 0.0f64 {
            *phase
                .offset(
                    i_0 as isize,
                ) = atan(*Q1.offset(i_0 as isize) / *I1.offset(i_0 as isize));
        }
        *deltaphase
            .offset(
                i_0 as isize,
            ) = *phase.offset((i_0 - 1 as libc::c_int) as isize)
            - *phase.offset(i_0 as isize);
        if *deltaphase.offset(i_0 as isize) < 1.0f64 {
            *deltaphase.offset(i_0 as isize) = 1.0f64;
        }
        *alpha.offset(i_0 as isize) = fastlimit / *deltaphase.offset(i_0 as isize);
        if *alpha.offset(i_0 as isize) < slowlimit {
            *alpha.offset(i_0 as isize) = slowlimit;
        }
        *mama = *alpha.offset(i_0 as isize) * *price.offset(i_0 as isize)
            + (1.0f64 - *alpha.offset(i_0 as isize))
                * (if i_0 > 6 as libc::c_int {
                    *mama.offset(-(1 as libc::c_int) as isize)
                } else {
                    0 as libc::c_int as libc::c_double
                });
        mama = mama.offset(1);
        mama;
        *fama = 0.5f64 * *alpha.offset(i_0 as isize)
            * *mama.offset(-(1 as libc::c_int) as isize)
            + (1.0f64 - 0.5f64 * *alpha.offset(i_0 as isize))
                * (if i_0 > 6 as libc::c_int {
                    *fama.offset(-(1 as libc::c_int) as isize)
                } else {
                    0 as libc::c_int as libc::c_double
                });
        fama = fama.offset(1);
        fama;
        i_0 += 1;
        i_0;
    }
    free(smooth as *mut libc::c_void);
    free(detrender as *mut libc::c_void);
    free(I1 as *mut libc::c_void);
    free(Q1 as *mut libc::c_void);
    free(jI as *mut libc::c_void);
    free(jQ as *mut libc::c_void);
    free(I2 as *mut libc::c_void);
    free(Q2 as *mut libc::c_void);
    free(Re as *mut libc::c_void);
    free(Im as *mut libc::c_void);
    free(period as *mut libc::c_void);
    free(smoothperiod as *mut libc::c_void);
    free(phase as *mut libc::c_void);
    free(deltaphase as *mut libc::c_void);
    free(alpha as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mama_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_mama = stream_in as *mut ti_stream_mama;
    let mut progress: libc::c_int = (*stream).progress;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let fastlimit: libc::c_double = (*stream).options[0 as libc::c_int as usize];
    let slowlimit: libc::c_double = (*stream).options[1 as libc::c_int as usize];
    let mut mama: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut fama: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut price: *const libc::c_double = real;
    progress == -(6 as libc::c_int);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size && progress < 0 as libc::c_int {
        let mut idx: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx == (*stream).buf_info.size_price {
            idx = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx as isize) = *price.offset(i as isize);
        (*stream).buf_info.index_price = idx;
        let mut idx_0: libc::c_int = (*stream).buf_info.index_smooth + 1 as libc::c_int;
        if idx_0 == (*stream).buf_info.size_smooth {
            idx_0 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smooth as isize)
            .offset(idx_0 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_smooth = idx_0;
        let mut idx_1: libc::c_int = (*stream).buf_info.index_detrender
            + 1 as libc::c_int;
        if idx_1 == (*stream).buf_info.size_detrender {
            idx_1 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(idx_1 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_detrender = idx_1;
        let mut idx_2: libc::c_int = (*stream).buf_info.index_I1 + 1 as libc::c_int;
        if idx_2 == (*stream).buf_info.size_I1 {
            idx_2 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_2 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_I1 = idx_2;
        let mut idx_3: libc::c_int = (*stream).buf_info.index_Q1 + 1 as libc::c_int;
        if idx_3 == (*stream).buf_info.size_Q1 {
            idx_3 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(idx_3 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_Q1 = idx_3;
        let mut idx_4: libc::c_int = (*stream).buf_info.index_jI + 1 as libc::c_int;
        if idx_4 == (*stream).buf_info.size_jI {
            idx_4 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_jI as isize)
            .offset(idx_4 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_jI = idx_4;
        let mut idx_5: libc::c_int = (*stream).buf_info.index_jQ + 1 as libc::c_int;
        if idx_5 == (*stream).buf_info.size_jQ {
            idx_5 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_jQ as isize)
            .offset(idx_5 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_jQ = idx_5;
        let mut idx_6: libc::c_int = (*stream).buf_info.index_I2 + 1 as libc::c_int;
        if idx_6 == (*stream).buf_info.size_I2 {
            idx_6 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_6 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_I2 = idx_6;
        let mut idx_7: libc::c_int = (*stream).buf_info.index_Q2 + 1 as libc::c_int;
        if idx_7 == (*stream).buf_info.size_Q2 {
            idx_7 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_7 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_Q2 = idx_7;
        let mut idx_8: libc::c_int = (*stream).buf_info.index_Re + 1 as libc::c_int;
        if idx_8 == (*stream).buf_info.size_Re {
            idx_8 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Re as isize)
            .offset(idx_8 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_Re = idx_8;
        let mut idx_9: libc::c_int = (*stream).buf_info.index_Im + 1 as libc::c_int;
        if idx_9 == (*stream).buf_info.size_Im {
            idx_9 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Im as isize)
            .offset(idx_9 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_Im = idx_9;
        let mut idx_10: libc::c_int = (*stream).buf_info.index_period + 1 as libc::c_int;
        if idx_10 == (*stream).buf_info.size_period {
            idx_10 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_10 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_period = idx_10;
        let mut idx_11: libc::c_int = (*stream).buf_info.index_smoothperiod
            + 1 as libc::c_int;
        if idx_11 == (*stream).buf_info.size_smoothperiod {
            idx_11 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smoothperiod as isize)
            .offset(idx_11 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_smoothperiod = idx_11;
        let mut idx_12: libc::c_int = (*stream).buf_info.index_phase + 1 as libc::c_int;
        if idx_12 == (*stream).buf_info.size_phase {
            idx_12 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_phase as isize)
            .offset(idx_12 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_phase = idx_12;
        let mut idx_13: libc::c_int = (*stream).buf_info.index_deltaphase
            + 1 as libc::c_int;
        if idx_13 == (*stream).buf_info.size_deltaphase {
            idx_13 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_deltaphase as isize)
            .offset(idx_13 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_deltaphase = idx_13;
        let mut idx_14: libc::c_int = (*stream).buf_info.index_alpha + 1 as libc::c_int;
        if idx_14 == (*stream).buf_info.size_alpha {
            idx_14 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_alpha as isize)
            .offset(idx_14 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_alpha = idx_14;
        let mut idx_15: libc::c_int = (*stream).buf_info.index_mama + 1 as libc::c_int;
        if idx_15 == (*stream).buf_info.size_mama {
            idx_15 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_mama as isize)
            .offset(idx_15 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_mama = idx_15;
        let mut idx_16: libc::c_int = (*stream).buf_info.index_fama + 1 as libc::c_int;
        if idx_16 == (*stream).buf_info.size_fama {
            idx_16 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_fama as isize)
            .offset(idx_16 as isize) = 0 as libc::c_int as libc::c_double;
        (*stream).buf_info.index_fama = idx_16;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        let mut var1: libc::c_double = 0.;
        let mut var2: libc::c_double = 0.;
        let mut var3: libc::c_double = 0.;
        let mut var4: libc::c_double = 0.;
        let mut var5: libc::c_double = 0.;
        let mut idx_17: libc::c_int = (*stream).buf_info.index_price + 1 as libc::c_int;
        if idx_17 == (*stream).buf_info.size_price {
            idx_17 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_17 as isize) = *price.offset(i as isize);
        (*stream).buf_info.index_price = idx_17;
        let mut idx_18: libc::c_int = (*stream).buf_info.index_price + 0 as libc::c_int;
        while idx_18 >= (*stream).buf_info.size_price {
            idx_18 -= (*stream).buf_info.size_price;
        }
        while idx_18 < 0 as libc::c_int {
            idx_18 += (*stream).buf_info.size_price;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_18 as isize);
        let mut idx_19: libc::c_int = (*stream).buf_info.index_price
            + -(1 as libc::c_int);
        while idx_19 >= (*stream).buf_info.size_price {
            idx_19 -= (*stream).buf_info.size_price;
        }
        while idx_19 < 0 as libc::c_int {
            idx_19 += (*stream).buf_info.size_price;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_19 as isize);
        let mut idx_20: libc::c_int = (*stream).buf_info.index_price
            + -(2 as libc::c_int);
        while idx_20 >= (*stream).buf_info.size_price {
            idx_20 -= (*stream).buf_info.size_price;
        }
        while idx_20 < 0 as libc::c_int {
            idx_20 += (*stream).buf_info.size_price;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_20 as isize);
        let mut idx_21: libc::c_int = (*stream).buf_info.index_price
            + -(3 as libc::c_int);
        while idx_21 >= (*stream).buf_info.size_price {
            idx_21 -= (*stream).buf_info.size_price;
        }
        while idx_21 < 0 as libc::c_int {
            idx_21 += (*stream).buf_info.size_price;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_21 as isize);
        let mut idx_22: libc::c_int = (*stream).buf_info.index_smooth + 1 as libc::c_int;
        if idx_22 == (*stream).buf_info.size_smooth {
            idx_22 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smooth as isize)
            .offset(
                idx_22 as isize,
            ) = (4 as libc::c_int as libc::c_double * var1
            + 3 as libc::c_int as libc::c_double * var2
            + 2 as libc::c_int as libc::c_double * var3 + var4) / 10.0f64;
        (*stream).buf_info.index_smooth = idx_22;
        let mut idx_23: libc::c_int = (*stream).buf_info.index_smooth + 0 as libc::c_int;
        while idx_23 >= (*stream).buf_info.size_smooth {
            idx_23 -= (*stream).buf_info.size_smooth;
        }
        while idx_23 < 0 as libc::c_int {
            idx_23 += (*stream).buf_info.size_smooth;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smooth as isize)
            .offset(idx_23 as isize);
        let mut idx_24: libc::c_int = (*stream).buf_info.index_smooth
            + -(2 as libc::c_int);
        while idx_24 >= (*stream).buf_info.size_smooth {
            idx_24 -= (*stream).buf_info.size_smooth;
        }
        while idx_24 < 0 as libc::c_int {
            idx_24 += (*stream).buf_info.size_smooth;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smooth as isize)
            .offset(idx_24 as isize);
        let mut idx_25: libc::c_int = (*stream).buf_info.index_smooth
            + -(4 as libc::c_int);
        while idx_25 >= (*stream).buf_info.size_smooth {
            idx_25 -= (*stream).buf_info.size_smooth;
        }
        while idx_25 < 0 as libc::c_int {
            idx_25 += (*stream).buf_info.size_smooth;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smooth as isize)
            .offset(idx_25 as isize);
        let mut idx_26: libc::c_int = (*stream).buf_info.index_smooth
            + -(6 as libc::c_int);
        while idx_26 >= (*stream).buf_info.size_smooth {
            idx_26 -= (*stream).buf_info.size_smooth;
        }
        while idx_26 < 0 as libc::c_int {
            idx_26 += (*stream).buf_info.size_smooth;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smooth as isize)
            .offset(idx_26 as isize);
        let mut idx_27: libc::c_int = (*stream).buf_info.index_period + 0 as libc::c_int;
        while idx_27 >= (*stream).buf_info.size_period {
            idx_27 -= (*stream).buf_info.size_period;
        }
        while idx_27 < 0 as libc::c_int {
            idx_27 += (*stream).buf_info.size_period;
        }
        var5 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_27 as isize);
        let mut idx_28: libc::c_int = (*stream).buf_info.index_detrender
            + 1 as libc::c_int;
        if idx_28 == (*stream).buf_info.size_detrender {
            idx_28 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(
                idx_28 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*stream).buf_info.index_detrender = idx_28;
        let mut idx_29: libc::c_int = (*stream).buf_info.index_detrender
            + 0 as libc::c_int;
        while idx_29 >= (*stream).buf_info.size_detrender {
            idx_29 -= (*stream).buf_info.size_detrender;
        }
        while idx_29 < 0 as libc::c_int {
            idx_29 += (*stream).buf_info.size_detrender;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(idx_29 as isize);
        let mut idx_30: libc::c_int = (*stream).buf_info.index_detrender
            + -(2 as libc::c_int);
        while idx_30 >= (*stream).buf_info.size_detrender {
            idx_30 -= (*stream).buf_info.size_detrender;
        }
        while idx_30 < 0 as libc::c_int {
            idx_30 += (*stream).buf_info.size_detrender;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(idx_30 as isize);
        let mut idx_31: libc::c_int = (*stream).buf_info.index_detrender
            + -(4 as libc::c_int);
        while idx_31 >= (*stream).buf_info.size_detrender {
            idx_31 -= (*stream).buf_info.size_detrender;
        }
        while idx_31 < 0 as libc::c_int {
            idx_31 += (*stream).buf_info.size_detrender;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(idx_31 as isize);
        let mut idx_32: libc::c_int = (*stream).buf_info.index_detrender
            + -(6 as libc::c_int);
        while idx_32 >= (*stream).buf_info.size_detrender {
            idx_32 -= (*stream).buf_info.size_detrender;
        }
        while idx_32 < 0 as libc::c_int {
            idx_32 += (*stream).buf_info.size_detrender;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(idx_32 as isize);
        let mut idx_33: libc::c_int = (*stream).buf_info.index_period + 0 as libc::c_int;
        while idx_33 >= (*stream).buf_info.size_period {
            idx_33 -= (*stream).buf_info.size_period;
        }
        while idx_33 < 0 as libc::c_int {
            idx_33 += (*stream).buf_info.size_period;
        }
        var5 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_33 as isize);
        let mut idx_34: libc::c_int = (*stream).buf_info.index_Q1 + 1 as libc::c_int;
        if idx_34 == (*stream).buf_info.size_Q1 {
            idx_34 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(
                idx_34 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*stream).buf_info.index_Q1 = idx_34;
        let mut idx_35: libc::c_int = (*stream).buf_info.index_detrender
            + -(3 as libc::c_int);
        while idx_35 >= (*stream).buf_info.size_detrender {
            idx_35 -= (*stream).buf_info.size_detrender;
        }
        while idx_35 < 0 as libc::c_int {
            idx_35 += (*stream).buf_info.size_detrender;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_detrender as isize)
            .offset(idx_35 as isize);
        let mut idx_36: libc::c_int = (*stream).buf_info.index_I1 + 1 as libc::c_int;
        if idx_36 == (*stream).buf_info.size_I1 {
            idx_36 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_36 as isize) = var1;
        (*stream).buf_info.index_I1 = idx_36;
        let mut idx_37: libc::c_int = (*stream).buf_info.index_I1 + 0 as libc::c_int;
        while idx_37 >= (*stream).buf_info.size_I1 {
            idx_37 -= (*stream).buf_info.size_I1;
        }
        while idx_37 < 0 as libc::c_int {
            idx_37 += (*stream).buf_info.size_I1;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_37 as isize);
        let mut idx_38: libc::c_int = (*stream).buf_info.index_I1 + -(2 as libc::c_int);
        while idx_38 >= (*stream).buf_info.size_I1 {
            idx_38 -= (*stream).buf_info.size_I1;
        }
        while idx_38 < 0 as libc::c_int {
            idx_38 += (*stream).buf_info.size_I1;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_38 as isize);
        let mut idx_39: libc::c_int = (*stream).buf_info.index_I1 + -(4 as libc::c_int);
        while idx_39 >= (*stream).buf_info.size_I1 {
            idx_39 -= (*stream).buf_info.size_I1;
        }
        while idx_39 < 0 as libc::c_int {
            idx_39 += (*stream).buf_info.size_I1;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_39 as isize);
        let mut idx_40: libc::c_int = (*stream).buf_info.index_I1 + -(6 as libc::c_int);
        while idx_40 >= (*stream).buf_info.size_I1 {
            idx_40 -= (*stream).buf_info.size_I1;
        }
        while idx_40 < 0 as libc::c_int {
            idx_40 += (*stream).buf_info.size_I1;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_40 as isize);
        let mut idx_41: libc::c_int = (*stream).buf_info.index_period + 0 as libc::c_int;
        while idx_41 >= (*stream).buf_info.size_period {
            idx_41 -= (*stream).buf_info.size_period;
        }
        while idx_41 < 0 as libc::c_int {
            idx_41 += (*stream).buf_info.size_period;
        }
        var5 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_41 as isize);
        let mut idx_42: libc::c_int = (*stream).buf_info.index_jI + 1 as libc::c_int;
        if idx_42 == (*stream).buf_info.size_jI {
            idx_42 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_jI as isize)
            .offset(
                idx_42 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*stream).buf_info.index_jI = idx_42;
        let mut idx_43: libc::c_int = (*stream).buf_info.index_Q1 + 0 as libc::c_int;
        while idx_43 >= (*stream).buf_info.size_Q1 {
            idx_43 -= (*stream).buf_info.size_Q1;
        }
        while idx_43 < 0 as libc::c_int {
            idx_43 += (*stream).buf_info.size_Q1;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(idx_43 as isize);
        let mut idx_44: libc::c_int = (*stream).buf_info.index_Q1 + -(2 as libc::c_int);
        while idx_44 >= (*stream).buf_info.size_Q1 {
            idx_44 -= (*stream).buf_info.size_Q1;
        }
        while idx_44 < 0 as libc::c_int {
            idx_44 += (*stream).buf_info.size_Q1;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(idx_44 as isize);
        let mut idx_45: libc::c_int = (*stream).buf_info.index_Q1 + -(4 as libc::c_int);
        while idx_45 >= (*stream).buf_info.size_Q1 {
            idx_45 -= (*stream).buf_info.size_Q1;
        }
        while idx_45 < 0 as libc::c_int {
            idx_45 += (*stream).buf_info.size_Q1;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(idx_45 as isize);
        let mut idx_46: libc::c_int = (*stream).buf_info.index_Q1 + -(6 as libc::c_int);
        while idx_46 >= (*stream).buf_info.size_Q1 {
            idx_46 -= (*stream).buf_info.size_Q1;
        }
        while idx_46 < 0 as libc::c_int {
            idx_46 += (*stream).buf_info.size_Q1;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(idx_46 as isize);
        let mut idx_47: libc::c_int = (*stream).buf_info.index_period + 0 as libc::c_int;
        while idx_47 >= (*stream).buf_info.size_period {
            idx_47 -= (*stream).buf_info.size_period;
        }
        while idx_47 < 0 as libc::c_int {
            idx_47 += (*stream).buf_info.size_period;
        }
        var5 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_47 as isize);
        let mut idx_48: libc::c_int = (*stream).buf_info.index_jQ + 1 as libc::c_int;
        if idx_48 == (*stream).buf_info.size_jQ {
            idx_48 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_jQ as isize)
            .offset(
                idx_48 as isize,
            ) = (0.0962f64 * var1 + 0.5769f64 * var2 - 0.5769f64 * var3
            - 0.0962f64 * var4) * (0.075f64 * var5 + 0.54f64);
        (*stream).buf_info.index_jQ = idx_48;
        let mut idx_49: libc::c_int = (*stream).buf_info.index_I1 + 0 as libc::c_int;
        while idx_49 >= (*stream).buf_info.size_I1 {
            idx_49 -= (*stream).buf_info.size_I1;
        }
        while idx_49 < 0 as libc::c_int {
            idx_49 += (*stream).buf_info.size_I1;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_49 as isize);
        let mut idx_50: libc::c_int = (*stream).buf_info.index_jQ + 0 as libc::c_int;
        while idx_50 >= (*stream).buf_info.size_jQ {
            idx_50 -= (*stream).buf_info.size_jQ;
        }
        while idx_50 < 0 as libc::c_int {
            idx_50 += (*stream).buf_info.size_jQ;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_jQ as isize)
            .offset(idx_50 as isize);
        let mut idx_51: libc::c_int = (*stream).buf_info.index_I2 + 0 as libc::c_int;
        while idx_51 >= (*stream).buf_info.size_I2 {
            idx_51 -= (*stream).buf_info.size_I2;
        }
        while idx_51 < 0 as libc::c_int {
            idx_51 += (*stream).buf_info.size_I2;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_51 as isize);
        let mut idx_52: libc::c_int = (*stream).buf_info.index_I2 + 1 as libc::c_int;
        if idx_52 == (*stream).buf_info.size_I2 {
            idx_52 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_52 as isize) = 0.2f64 * (var1 - var2) + 0.8f64 * var3;
        (*stream).buf_info.index_I2 = idx_52;
        let mut idx_53: libc::c_int = (*stream).buf_info.index_Q1 + 0 as libc::c_int;
        while idx_53 >= (*stream).buf_info.size_Q1 {
            idx_53 -= (*stream).buf_info.size_Q1;
        }
        while idx_53 < 0 as libc::c_int {
            idx_53 += (*stream).buf_info.size_Q1;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q1 as isize)
            .offset(idx_53 as isize);
        let mut idx_54: libc::c_int = (*stream).buf_info.index_jI + 0 as libc::c_int;
        while idx_54 >= (*stream).buf_info.size_jI {
            idx_54 -= (*stream).buf_info.size_jI;
        }
        while idx_54 < 0 as libc::c_int {
            idx_54 += (*stream).buf_info.size_jI;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_jI as isize)
            .offset(idx_54 as isize);
        let mut idx_55: libc::c_int = (*stream).buf_info.index_Q2 + 0 as libc::c_int;
        while idx_55 >= (*stream).buf_info.size_Q2 {
            idx_55 -= (*stream).buf_info.size_Q2;
        }
        while idx_55 < 0 as libc::c_int {
            idx_55 += (*stream).buf_info.size_Q2;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_55 as isize);
        let mut idx_56: libc::c_int = (*stream).buf_info.index_Q2 + 1 as libc::c_int;
        if idx_56 == (*stream).buf_info.size_Q2 {
            idx_56 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_56 as isize) = 0.2f64 * (var1 + var2) + 0.8f64 * var3;
        (*stream).buf_info.index_Q2 = idx_56;
        let mut idx_57: libc::c_int = (*stream).buf_info.index_I2 + 0 as libc::c_int;
        while idx_57 >= (*stream).buf_info.size_I2 {
            idx_57 -= (*stream).buf_info.size_I2;
        }
        while idx_57 < 0 as libc::c_int {
            idx_57 += (*stream).buf_info.size_I2;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_57 as isize);
        let mut idx_58: libc::c_int = (*stream).buf_info.index_I2 + -(1 as libc::c_int);
        while idx_58 >= (*stream).buf_info.size_I2 {
            idx_58 -= (*stream).buf_info.size_I2;
        }
        while idx_58 < 0 as libc::c_int {
            idx_58 += (*stream).buf_info.size_I2;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_58 as isize);
        let mut idx_59: libc::c_int = (*stream).buf_info.index_Q2 + 0 as libc::c_int;
        while idx_59 >= (*stream).buf_info.size_Q2 {
            idx_59 -= (*stream).buf_info.size_Q2;
        }
        while idx_59 < 0 as libc::c_int {
            idx_59 += (*stream).buf_info.size_Q2;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_59 as isize);
        let mut idx_60: libc::c_int = (*stream).buf_info.index_Q2 + -(1 as libc::c_int);
        while idx_60 >= (*stream).buf_info.size_Q2 {
            idx_60 -= (*stream).buf_info.size_Q2;
        }
        while idx_60 < 0 as libc::c_int {
            idx_60 += (*stream).buf_info.size_Q2;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_60 as isize);
        let mut idx_61: libc::c_int = (*stream).buf_info.index_Re + 0 as libc::c_int;
        while idx_61 >= (*stream).buf_info.size_Re {
            idx_61 -= (*stream).buf_info.size_Re;
        }
        while idx_61 < 0 as libc::c_int {
            idx_61 += (*stream).buf_info.size_Re;
        }
        var5 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Re as isize)
            .offset(idx_61 as isize);
        let mut idx_62: libc::c_int = (*stream).buf_info.index_Re + 1 as libc::c_int;
        if idx_62 == (*stream).buf_info.size_Re {
            idx_62 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Re as isize)
            .offset(
                idx_62 as isize,
            ) = 0.2f64 * (var1 * var2 + var3 * var4) + 0.8f64 * var5;
        (*stream).buf_info.index_Re = idx_62;
        let mut idx_63: libc::c_int = (*stream).buf_info.index_I2 + 0 as libc::c_int;
        while idx_63 >= (*stream).buf_info.size_I2 {
            idx_63 -= (*stream).buf_info.size_I2;
        }
        while idx_63 < 0 as libc::c_int {
            idx_63 += (*stream).buf_info.size_I2;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_63 as isize);
        let mut idx_64: libc::c_int = (*stream).buf_info.index_Q2 + -(1 as libc::c_int);
        while idx_64 >= (*stream).buf_info.size_Q2 {
            idx_64 -= (*stream).buf_info.size_Q2;
        }
        while idx_64 < 0 as libc::c_int {
            idx_64 += (*stream).buf_info.size_Q2;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_64 as isize);
        let mut idx_65: libc::c_int = (*stream).buf_info.index_Q2 + 0 as libc::c_int;
        while idx_65 >= (*stream).buf_info.size_Q2 {
            idx_65 -= (*stream).buf_info.size_Q2;
        }
        while idx_65 < 0 as libc::c_int {
            idx_65 += (*stream).buf_info.size_Q2;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Q2 as isize)
            .offset(idx_65 as isize);
        let mut idx_66: libc::c_int = (*stream).buf_info.index_I2 + -(1 as libc::c_int);
        while idx_66 >= (*stream).buf_info.size_I2 {
            idx_66 -= (*stream).buf_info.size_I2;
        }
        while idx_66 < 0 as libc::c_int {
            idx_66 += (*stream).buf_info.size_I2;
        }
        var4 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I2 as isize)
            .offset(idx_66 as isize);
        let mut idx_67: libc::c_int = (*stream).buf_info.index_Im + 0 as libc::c_int;
        while idx_67 >= (*stream).buf_info.size_Im {
            idx_67 -= (*stream).buf_info.size_Im;
        }
        while idx_67 < 0 as libc::c_int {
            idx_67 += (*stream).buf_info.size_Im;
        }
        var5 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Im as isize)
            .offset(idx_67 as isize);
        let mut idx_68: libc::c_int = (*stream).buf_info.index_Im + 1 as libc::c_int;
        if idx_68 == (*stream).buf_info.size_Im {
            idx_68 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Im as isize)
            .offset(
                idx_68 as isize,
            ) = 0.2f64 * (var1 * var2 - var3 * var4) + 0.8f64 * var5;
        (*stream).buf_info.index_Im = idx_68;
        let mut period_: libc::c_double = 0.0f64;
        let mut idx_69: libc::c_int = (*stream).buf_info.index_Im + 0 as libc::c_int;
        while idx_69 >= (*stream).buf_info.size_Im {
            idx_69 -= (*stream).buf_info.size_Im;
        }
        while idx_69 < 0 as libc::c_int {
            idx_69 += (*stream).buf_info.size_Im;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Im as isize)
            .offset(idx_69 as isize);
        let mut idx_70: libc::c_int = (*stream).buf_info.index_Re + 0 as libc::c_int;
        while idx_70 >= (*stream).buf_info.size_Re {
            idx_70 -= (*stream).buf_info.size_Re;
        }
        while idx_70 < 0 as libc::c_int {
            idx_70 += (*stream).buf_info.size_Re;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_Re as isize)
            .offset(idx_70 as isize);
        let mut idx_71: libc::c_int = (*stream).buf_info.index_period + 0 as libc::c_int;
        while idx_71 >= (*stream).buf_info.size_period {
            idx_71 -= (*stream).buf_info.size_period;
        }
        while idx_71 < 0 as libc::c_int {
            idx_71 += (*stream).buf_info.size_period;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_71 as isize);
        if var1 != 0.0f64 && var2 != 0.0f64 {
            period_ = 360.0f64 / atan(var1 / var2);
        }
        if period_ > 1.5f64 * var3 {
            period_ = 1.5f64 * var3;
        }
        if period_ < 0.67f64 * var3 {
            period_ = 0.67f64 * var3;
        }
        if period_ < 6.0f64 {
            period_ = 6.0f64;
        }
        if period_ > 50.0f64 {
            period_ = 50.0f64;
        }
        let mut idx_72: libc::c_int = (*stream).buf_info.index_period + 1 as libc::c_int;
        if idx_72 == (*stream).buf_info.size_period {
            idx_72 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_72 as isize) = 0.2f64 * period_ + 0.8f64 * var3;
        (*stream).buf_info.index_period = idx_72;
        let mut idx_73: libc::c_int = (*stream).buf_info.index_period + 0 as libc::c_int;
        while idx_73 >= (*stream).buf_info.size_period {
            idx_73 -= (*stream).buf_info.size_period;
        }
        while idx_73 < 0 as libc::c_int {
            idx_73 += (*stream).buf_info.size_period;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_period as isize)
            .offset(idx_73 as isize);
        let mut idx_74: libc::c_int = (*stream).buf_info.index_smoothperiod
            + 0 as libc::c_int;
        while idx_74 >= (*stream).buf_info.size_smoothperiod {
            idx_74 -= (*stream).buf_info.size_smoothperiod;
        }
        while idx_74 < 0 as libc::c_int {
            idx_74 += (*stream).buf_info.size_smoothperiod;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smoothperiod as isize)
            .offset(idx_74 as isize);
        let mut idx_75: libc::c_int = (*stream).buf_info.index_smoothperiod
            + 1 as libc::c_int;
        if idx_75 == (*stream).buf_info.size_smoothperiod {
            idx_75 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_smoothperiod as isize)
            .offset(idx_75 as isize) = 0.33f64 * var1 + 0.67f64 * var2;
        (*stream).buf_info.index_smoothperiod = idx_75;
        let mut idx_76: libc::c_int = (*stream).buf_info.index_I1 + 0 as libc::c_int;
        while idx_76 >= (*stream).buf_info.size_I1 {
            idx_76 -= (*stream).buf_info.size_I1;
        }
        while idx_76 < 0 as libc::c_int {
            idx_76 += (*stream).buf_info.size_I1;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_I1 as isize)
            .offset(idx_76 as isize);
        if var1 != 0.0f64 {
            let mut idx_77: libc::c_int = (*stream).buf_info.index_Q1 + 0 as libc::c_int;
            while idx_77 >= (*stream).buf_info.size_Q1 {
                idx_77 -= (*stream).buf_info.size_Q1;
            }
            while idx_77 < 0 as libc::c_int {
                idx_77 += (*stream).buf_info.size_Q1;
            }
            var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_Q1 as isize)
                .offset(idx_77 as isize);
            let mut idx_78: libc::c_int = (*stream).buf_info.index_phase
                + 1 as libc::c_int;
            if idx_78 == (*stream).buf_info.size_phase {
                idx_78 = 0 as libc::c_int;
            }
            *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_phase as isize)
                .offset(idx_78 as isize) = atan(var2 / var1);
            (*stream).buf_info.index_phase = idx_78;
        } else {
            let mut idx_79: libc::c_int = (*stream).buf_info.index_phase
                + 1 as libc::c_int;
            if idx_79 == (*stream).buf_info.size_phase {
                idx_79 = 0 as libc::c_int;
            }
            *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
                .offset(1 as libc::c_int as isize) as *mut libc::c_double)
                .offset((*stream).buf_info.offset_phase as isize)
                .offset(idx_79 as isize) = 0 as libc::c_int as libc::c_double;
            (*stream).buf_info.index_phase = idx_79;
        }
        let mut idx_80: libc::c_int = (*stream).buf_info.index_phase
            + -(1 as libc::c_int);
        while idx_80 >= (*stream).buf_info.size_phase {
            idx_80 -= (*stream).buf_info.size_phase;
        }
        while idx_80 < 0 as libc::c_int {
            idx_80 += (*stream).buf_info.size_phase;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_phase as isize)
            .offset(idx_80 as isize);
        let mut idx_81: libc::c_int = (*stream).buf_info.index_phase + 0 as libc::c_int;
        while idx_81 >= (*stream).buf_info.size_phase {
            idx_81 -= (*stream).buf_info.size_phase;
        }
        while idx_81 < 0 as libc::c_int {
            idx_81 += (*stream).buf_info.size_phase;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_phase as isize)
            .offset(idx_81 as isize);
        let mut idx_82: libc::c_int = (*stream).buf_info.index_deltaphase
            + 1 as libc::c_int;
        if idx_82 == (*stream).buf_info.size_deltaphase {
            idx_82 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_deltaphase as isize)
            .offset(
                idx_82 as isize,
            ) = if 1 as libc::c_int as libc::c_double > var1 - var2 {
            1 as libc::c_int as libc::c_double
        } else {
            var1 - var2
        };
        (*stream).buf_info.index_deltaphase = idx_82;
        let mut idx_83: libc::c_int = (*stream).buf_info.index_deltaphase
            + 0 as libc::c_int;
        while idx_83 >= (*stream).buf_info.size_deltaphase {
            idx_83 -= (*stream).buf_info.size_deltaphase;
        }
        while idx_83 < 0 as libc::c_int {
            idx_83 += (*stream).buf_info.size_deltaphase;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_deltaphase as isize)
            .offset(idx_83 as isize);
        let mut idx_84: libc::c_int = (*stream).buf_info.index_alpha + 1 as libc::c_int;
        if idx_84 == (*stream).buf_info.size_alpha {
            idx_84 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_alpha as isize)
            .offset(
                idx_84 as isize,
            ) = if slowlimit > fastlimit / var1 { slowlimit } else { fastlimit / var1 };
        (*stream).buf_info.index_alpha = idx_84;
        let mut idx_85: libc::c_int = (*stream).buf_info.index_alpha + 0 as libc::c_int;
        while idx_85 >= (*stream).buf_info.size_alpha {
            idx_85 -= (*stream).buf_info.size_alpha;
        }
        while idx_85 < 0 as libc::c_int {
            idx_85 += (*stream).buf_info.size_alpha;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_alpha as isize)
            .offset(idx_85 as isize);
        let mut idx_86: libc::c_int = (*stream).buf_info.index_mama + 0 as libc::c_int;
        while idx_86 >= (*stream).buf_info.size_mama {
            idx_86 -= (*stream).buf_info.size_mama;
        }
        while idx_86 < 0 as libc::c_int {
            idx_86 += (*stream).buf_info.size_mama;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_mama as isize)
            .offset(idx_86 as isize);
        let mut idx_87: libc::c_int = (*stream).buf_info.index_price + 0 as libc::c_int;
        while idx_87 >= (*stream).buf_info.size_price {
            idx_87 -= (*stream).buf_info.size_price;
        }
        while idx_87 < 0 as libc::c_int {
            idx_87 += (*stream).buf_info.size_price;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_price as isize)
            .offset(idx_87 as isize);
        let mut idx_88: libc::c_int = (*stream).buf_info.index_mama + 1 as libc::c_int;
        if idx_88 == (*stream).buf_info.size_mama {
            idx_88 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_mama as isize)
            .offset(idx_88 as isize) = var1 * var3 + (1.0f64 - var1) * var2;
        (*stream).buf_info.index_mama = idx_88;
        let mut idx_89: libc::c_int = (*stream).buf_info.index_alpha + 0 as libc::c_int;
        while idx_89 >= (*stream).buf_info.size_alpha {
            idx_89 -= (*stream).buf_info.size_alpha;
        }
        while idx_89 < 0 as libc::c_int {
            idx_89 += (*stream).buf_info.size_alpha;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_alpha as isize)
            .offset(idx_89 as isize);
        let mut idx_90: libc::c_int = (*stream).buf_info.index_mama + 0 as libc::c_int;
        while idx_90 >= (*stream).buf_info.size_mama {
            idx_90 -= (*stream).buf_info.size_mama;
        }
        while idx_90 < 0 as libc::c_int {
            idx_90 += (*stream).buf_info.size_mama;
        }
        var2 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_mama as isize)
            .offset(idx_90 as isize);
        let mut idx_91: libc::c_int = (*stream).buf_info.index_fama + 0 as libc::c_int;
        while idx_91 >= (*stream).buf_info.size_fama {
            idx_91 -= (*stream).buf_info.size_fama;
        }
        while idx_91 < 0 as libc::c_int {
            idx_91 += (*stream).buf_info.size_fama;
        }
        var3 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_fama as isize)
            .offset(idx_91 as isize);
        let mut idx_92: libc::c_int = (*stream).buf_info.index_fama + 1 as libc::c_int;
        if idx_92 == (*stream).buf_info.size_fama {
            idx_92 = 0 as libc::c_int;
        }
        *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_fama as isize)
            .offset(
                idx_92 as isize,
            ) = 0.5f64 * var1 * var2 + (1.0f64 - 0.5f64 * var1) * var3;
        (*stream).buf_info.index_fama = idx_92;
        let mut idx_93: libc::c_int = (*stream).buf_info.index_mama + 0 as libc::c_int;
        while idx_93 >= (*stream).buf_info.size_mama {
            idx_93 -= (*stream).buf_info.size_mama;
        }
        while idx_93 < 0 as libc::c_int {
            idx_93 += (*stream).buf_info.size_mama;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_mama as isize)
            .offset(idx_93 as isize);
        let fresh2 = mama;
        mama = mama.offset(1);
        *fresh2 = var1;
        let mut idx_94: libc::c_int = (*stream).buf_info.index_fama + 0 as libc::c_int;
        while idx_94 >= (*stream).buf_info.size_fama {
            idx_94 -= (*stream).buf_info.size_fama;
        }
        while idx_94 < 0 as libc::c_int {
            idx_94 += (*stream).buf_info.size_fama;
        }
        var1 = *((&mut (*stream).buf_info as *mut C2RustUnnamed_0)
            .offset(1 as libc::c_int as isize) as *mut libc::c_double)
            .offset((*stream).buf_info.offset_fama as isize)
            .offset(idx_94 as isize);
        let fresh3 = fama;
        fama = fama.offset(1);
        *fresh3 = var1;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mama_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
