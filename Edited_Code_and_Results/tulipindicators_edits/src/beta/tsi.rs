use ::libc;
extern "C" {
    pub type ti_stream;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_abs(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_ema(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_mom_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_mom(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_tsi {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub options: C2RustUnnamed_0,
    pub state: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub price: libc::c_double,
    pub y_ema_num: libc::c_double,
    pub z_ema_num: libc::c_double,
    pub y_ema_den: libc::c_double,
    pub z_ema_den: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub y_period: libc::c_double,
    pub z_period: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let y_period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let z_period: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut tsi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut progress: libc::c_int = -(1 as libc::c_int);
    let mut price: libc::c_double = 0.;
    let mut y_ema_num: libc::c_double = 0.;
    let mut z_ema_num: libc::c_double = 0.;
    let mut y_ema_den: libc::c_double = 0.;
    let mut z_ema_den: libc::c_double = 0.;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < size && progress == -(1 as libc::c_int) {
        price = *real.offset(i_0 as isize);
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    while i_0 < size && progress == 0 as libc::c_int {
        y_ema_num = *real.offset(i_0 as isize) - price;
        z_ema_num = y_ema_num;
        y_ema_den = fabs(*real.offset(i_0 as isize) - price);
        z_ema_den = y_ema_den;
        let fresh0 = tsi;
        tsi = tsi.offset(1);
        *fresh0 = 100.0f64
            * (if z_ema_den != 0. {
                z_ema_num / z_ema_den
            } else {
                0 as libc::c_int as libc::c_double
            });
        price = *real.offset(i_0 as isize);
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    while i_0 < size {
        y_ema_num = (*real.offset(i_0 as isize) - price - y_ema_num) * 2.0f64
            / (1.0f64 + y_period) + y_ema_num;
        y_ema_den = (fabs(*real.offset(i_0 as isize) - price) - y_ema_den) * 2.0f64
            / (1.0f64 + y_period) + y_ema_den;
        z_ema_num = (y_ema_num - z_ema_num) * 2.0f64 / (1.0f64 + z_period) + z_ema_num;
        z_ema_den = (y_ema_den - z_ema_den) * 2.0f64 / (1.0f64 + z_period) + z_ema_den;
        let fresh1 = tsi;
        tsi = tsi.offset(1);
        *fresh1 = 100.0f64
            * (if z_ema_den != 0. {
                z_ema_num / z_ema_den
            } else {
                0 as libc::c_int as libc::c_double
            });
        price = *real.offset(i_0 as isize);
        i_0 += 1;
        i_0;
        progress += 1;
        progress;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsi_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let y_period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let z_period: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut tsi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if *options.offset(i as isize) < 1 as libc::c_int as libc::c_double {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut _one: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut outsize: libc::c_int = size - ti_mom_start(&mut _one);
    let mut momentum: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    ti_mom(size, &mut real, &mut _one, &mut momentum);
    let mut absmomentum: *mut libc::c_double = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(outsize as libc::c_uint as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut momentum_input: [*const libc::c_double; 1] = [
        momentum as *const libc::c_double,
    ];
    ti_abs(
        outsize,
        momentum_input.as_mut_ptr(),
        0 as *const libc::c_double,
        &mut absmomentum,
    );
    ti_ema(outsize, momentum_input.as_mut_ptr(), &y_period, &mut momentum);
    ti_ema(outsize, momentum_input.as_mut_ptr(), &z_period, &mut momentum);
    let mut absmomentum_input: [*const libc::c_double; 1] = [
        absmomentum as *const libc::c_double,
    ];
    ti_ema(outsize, absmomentum_input.as_mut_ptr(), &y_period, &mut absmomentum);
    ti_ema(outsize, absmomentum_input.as_mut_ptr(), &z_period, &mut absmomentum);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < outsize {
        let fresh2 = tsi;
        tsi = tsi.offset(1);
        *fresh2 = 100.0f64 * *momentum.offset(i_0 as isize)
            / *absmomentum.offset(i_0 as isize);
        i_0 += 1;
        i_0;
    }
    free(momentum as *mut libc::c_void);
    free(absmomentum as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsi_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_tsi_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_tsi = stream_in as *mut ti_stream_tsi;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut tsi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let y_period: libc::c_double = (*stream).options.y_period;
    let z_period: libc::c_double = (*stream).options.z_period;
    let mut progress: libc::c_int = (*stream).progress;
    let mut price: libc::c_double = (*stream).state.price;
    let mut y_ema_num: libc::c_double = (*stream).state.y_ema_num;
    let mut z_ema_num: libc::c_double = (*stream).state.z_ema_num;
    let mut y_ema_den: libc::c_double = (*stream).state.y_ema_den;
    let mut z_ema_den: libc::c_double = (*stream).state.z_ema_den;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size && progress == -(1 as libc::c_int) {
        price = *real.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size && progress == 0 as libc::c_int {
        y_ema_num = *real.offset(i as isize) - price;
        z_ema_num = y_ema_num;
        y_ema_den = fabs(*real.offset(i as isize) - price);
        z_ema_den = y_ema_den;
        let fresh3 = tsi;
        tsi = tsi.offset(1);
        *fresh3 = 100.0f64
            * (if z_ema_den != 0. {
                z_ema_num / z_ema_den
            } else {
                0 as libc::c_int as libc::c_double
            });
        price = *real.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        y_ema_num = (*real.offset(i as isize) - price - y_ema_num) * 2.0f64
            / (1.0f64 + y_period) + y_ema_num;
        y_ema_den = (fabs(*real.offset(i as isize) - price) - y_ema_den) * 2.0f64
            / (1.0f64 + y_period) + y_ema_den;
        z_ema_num = (y_ema_num - z_ema_num) * 2.0f64 / (1.0f64 + z_period) + z_ema_num;
        z_ema_den = (y_ema_den - z_ema_den) * 2.0f64 / (1.0f64 + z_period) + z_ema_den;
        let fresh4 = tsi;
        tsi = tsi.offset(1);
        *fresh4 = 100.0f64
            * (if z_ema_den != 0. {
                z_ema_num / z_ema_den
            } else {
                0 as libc::c_int as libc::c_double
            });
        price = *real.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).state.price = price;
    (*stream).state.y_ema_num = y_ema_num;
    (*stream).state.z_ema_num = z_ema_num;
    (*stream).state.y_ema_den = y_ema_den;
    (*stream).state.z_ema_den = z_ema_den;
    return 0 as libc::c_int;
}
