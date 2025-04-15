#![feature(label_break_value)]

use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_ema_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_ema(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_roc_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_roc(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kst_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let roc4: libc::c_int = *options.offset(3 as libc::c_int as isize) as libc::c_int;
    return roc4;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kst(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let roc1: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let roc2: libc::c_int = *options.offset(1 as libc::c_int as isize) as libc::c_int;
    let roc3: libc::c_int = *options.offset(2 as libc::c_int as isize) as libc::c_int;
    let roc4: libc::c_int = *options.offset(3 as libc::c_int as isize) as libc::c_int;
    let ma1: libc::c_int = *options.offset(4 as libc::c_int as isize) as libc::c_int;
    let ma2: libc::c_int = *options.offset(5 as libc::c_int as isize) as libc::c_int;
    let ma3: libc::c_int = *options.offset(6 as libc::c_int as isize) as libc::c_int;
    let ma4: libc::c_int = *options.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut kst: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut kst_signal: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if !(roc1 < roc2 && roc2 < roc3 && roc3 < roc4) {
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut max_ma: libc::c_double = 0.;
    max_ma = ma1 as libc::c_double;
    if max_ma < ma2 as libc::c_double {
        max_ma = ma2 as libc::c_double;
    }
    if max_ma < ma3 as libc::c_double {
        max_ma = ma3 as libc::c_double;
    }
    if max_ma < ma4 as libc::c_double {
        max_ma = ma4 as libc::c_double;
    }
    let mut per1: libc::c_double = 2.0f64 / (ma1 + 1 as libc::c_int) as libc::c_double;
    let mut per2: libc::c_double = 2.0f64 / (ma2 + 1 as libc::c_int) as libc::c_double;
    let mut per3: libc::c_double = 2.0f64 / (ma3 + 1 as libc::c_int) as libc::c_double;
    let mut per4: libc::c_double = 2.0f64 / (ma4 + 1 as libc::c_int) as libc::c_double;
    let mut per_signal: libc::c_double = 2.0f64 / (9.0f64 + 1.0f64);
    let mut _1: libc::c_double = (*real.offset(roc1 as isize)
        - *real.offset((roc1 - roc2) as isize)) / *real.offset((roc1 - roc2) as isize);
    let mut _2: libc::c_double = (*real.offset(roc2 as isize)
        - *real.offset((roc3 - roc2) as isize)) / *real.offset((roc3 - roc2) as isize);
    let mut _3: libc::c_double = (*real.offset(roc3 as isize)
        - *real.offset((roc3 - roc2) as isize)) / *real.offset((roc3 - roc2) as isize);
    let mut _4: libc::c_double = (*real.offset(roc4 as isize)
        - *real.offset((roc4 - roc3) as isize)) / *real.offset((roc4 - roc3) as isize);
    let mut i_0: libc::c_int = roc1 + 1 as libc::c_int;
    while i_0 < roc4 + 1 as libc::c_int && i_0 < size {
        _1 = ((*real.offset(i_0 as isize) - *real.offset((i_0 - roc1) as isize))
            / *real.offset((i_0 - roc1) as isize) - _1) * per1 + _1;
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = roc2 + 1 as libc::c_int;
    while i_1 < roc4 + 1 as libc::c_int && i_1 < size {
        _2 = ((*real.offset(i_1 as isize) - *real.offset((i_1 - roc2) as isize))
            / *real.offset((i_1 - roc2) as isize) - _2) * per2 + _2;
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = roc3 + 1 as libc::c_int;
    while i_2 < roc4 + 1 as libc::c_int && i_2 < size {
        _3 = ((*real.offset(i_2 as isize) - *real.offset((i_2 - roc3) as isize))
            / *real.offset((i_2 - roc3) as isize) - _3) * per3 + _3;
        i_2 += 1;
        i_2;
    }
    let mut i_3: libc::c_int = roc4 + 1 as libc::c_int;
    while i_3 < roc4 + 1 as libc::c_int && i_3 < size {
        _4 = ((*real.offset(i_3 as isize) - *real.offset((i_3 - roc4) as isize))
            / *real.offset((i_3 - roc4) as isize) - _4) * per4 + _4;
        i_3 += 1;
        i_3;
    }
    let mut val: libc::c_double = (_1 * 1.0f64 + _2 * 2.0f64 + _3 * 3.0f64 + _4 * 4.0f64)
        / 10.0f64;
    let fresh0 = kst_signal;
    kst_signal = kst_signal.offset(1);
    *fresh0 = val;
    let mut _signal: libc::c_double = val;
    let fresh1 = kst;
    kst = kst.offset(1);
    *fresh1 = _signal;
    let mut i_4: libc::c_int = roc4 + 1 as libc::c_int;
    while i_4 < size {
        _1 = ((*real.offset(i_4 as isize) - *real.offset((i_4 - roc1) as isize))
            / *real.offset((i_4 - roc1) as isize) - _1) * per1 + _1;
        _2 = ((*real.offset(i_4 as isize) - *real.offset((i_4 - roc2) as isize))
            / *real.offset((i_4 - roc2) as isize) - _2) * per2 + _2;
        _3 = ((*real.offset(i_4 as isize) - *real.offset((i_4 - roc3) as isize))
            / *real.offset((i_4 - roc3) as isize) - _3) * per3 + _3;
        _4 = ((*real.offset(i_4 as isize) - *real.offset((i_4 - roc4) as isize))
            / *real.offset((i_4 - roc4) as isize) - _4) * per4 + _4;
        val = (_1 * 1.0f64 + _2 * 2.0f64 + _3 * 3.0f64 + _4 * 4.0f64) / 10.0f64;
        let fresh2 = kst;
        kst = kst.offset(1);
        *fresh2 = val;
        _signal = (val - _signal) * per_signal + _signal;
        let fresh3 = kst_signal;
        kst_signal = kst_signal.offset(1);
        *fresh3 = _signal;
        i_4 += 1;
        i_4;
    }
    if kst.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_kst_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"kst - outputs[0] == size - ti_kst_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/kst.c\0"
                as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_kst(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3572: {
        if kst.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_kst_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"kst - outputs[0] == size - ti_kst_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/kst.c\0"
                    as *const u8 as *const libc::c_char,
                111 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int ti_kst(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kst_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut roc: *const libc::c_double = options;
    let mut ma: *const libc::c_double = options.offset(4 as libc::c_int as isize);
    let mut kst: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut kst_signal: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if !(*roc.offset(0 as libc::c_int as isize) < *roc.offset(1 as libc::c_int as isize)
        && *roc.offset(1 as libc::c_int as isize)
            < *roc.offset(2 as libc::c_int as isize)
        && *roc.offset(2 as libc::c_int as isize)
            < *roc.offset(3 as libc::c_int as isize))
    {
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut size_roc: [libc::c_int; 4] = [0; 4];
    let mut size_ema: [libc::c_int; 4] = [0; 4];
    let mut roc_mem: [*mut libc::c_double; 4] = [0 as *mut libc::c_double; 4];
    let mut ema_mem: [*mut libc::c_double; 4] = [0 as *mut libc::c_double; 4];
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        size_roc[i_0 as usize] = size - ti_roc_start(roc.offset(i_0 as isize));
        size_ema[i_0
            as usize] = size_roc[i_0 as usize] - ti_ema_start(ma.offset(i_0 as isize));
        roc_mem[i_0
            as usize] = malloc(
            (::core::mem::size_of::<libc::c_double>() * size_roc[i_0 as usize] as usize)
                as libc::c_ulong,
        ) as *mut libc::c_double;
        ti_roc(
            size,
            &mut real,
            roc.offset(i_0 as isize),
            roc_mem.as_mut_ptr().offset(i_0 as isize),
        );
        ema_mem[i_0
            as usize] = malloc(
            (::core::mem::size_of::<libc::c_double>() * size_ema[i_0 as usize] as usize)
                as libc::c_ulong,
        ) as *mut libc::c_double;
        let mut ti_ema_inputs: [*const libc::c_double; 1] = [
            roc_mem[i_0 as usize] as *const libc::c_double,
        ];
        ti_ema(
            size_roc[i_0 as usize],
            ti_ema_inputs.as_mut_ptr(),
            ma.offset(i_0 as isize),
            ema_mem.as_mut_ptr().offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    let mut min_len: libc::c_int = size_ema[0 as libc::c_int as usize];
    let mut i_1: libc::c_int = 1 as libc::c_int;
    while i_1 < 4 as libc::c_int {
        if min_len > size_ema[i_1 as usize] {
            min_len = size_ema[i_1 as usize];
        }
        i_1 += 1;
        i_1;
    }
    let mut adj: [libc::c_int; 4] = [0; 4];
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 4 as libc::c_int {
        adj[i_2 as usize] = size_ema[i_2 as usize] - min_len;
        i_2 += 1;
        i_2;
    }
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < min_len {
        let fresh4 = kst;
        kst = kst.offset(1);
        *fresh4 = (*(ema_mem[0 as libc::c_int as usize])
            .offset((i_3 + adj[0 as libc::c_int as usize]) as isize) * 1.0f64
            + *(ema_mem[1 as libc::c_int as usize])
                .offset((i_3 + adj[1 as libc::c_int as usize]) as isize) * 2.0f64
            + *(ema_mem[2 as libc::c_int as usize])
                .offset((i_3 + adj[2 as libc::c_int as usize]) as isize) * 3.0f64
            + *(ema_mem[3 as libc::c_int as usize])
                .offset((i_3 + adj[3 as libc::c_int as usize]) as isize) * 4.0f64)
            / 10.0f64;
        i_3 += 1;
        i_3;
    }
    let mut signal_period: libc::c_double = 9.0f64;
    let mut ti_ema_inputs_0: [*const libc::c_double; 1] = [
        *outputs.offset(0 as libc::c_int as isize) as *const libc::c_double,
    ];
    ti_ema(min_len, ti_ema_inputs_0.as_mut_ptr(), &mut signal_period, &mut kst_signal);
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < 4 as libc::c_int {
        free(roc_mem[i_4 as usize] as *mut libc::c_void);
        free(ema_mem[i_4 as usize] as *mut libc::c_void);
        i_4 += 1;
        i_4;
    }
    if kst.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_kst_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"kst - outputs[0] == size - ti_kst_start(options)\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/kst.c\0"
                as *const u8 as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_kst_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4523: {
        if kst.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
            == (size - ti_kst_start(options)) as libc::c_long
        {} else {
            __assert_fail(
                b"kst - outputs[0] == size - ti_kst_start(options)\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tulipindicators/beta/kst.c\0"
                    as *const u8 as *const libc::c_char,
                171 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"int ti_kst_ref(int, const double *const *, const double *, double *const *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
