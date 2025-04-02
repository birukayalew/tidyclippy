use ::libc;
extern "C" {
    pub type ti_stream;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_ema(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_fi {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub per: libc::c_double,
    pub ema: libc::c_double,
    pub previous_close: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn ti_fi_start(mut options: *const libc::c_double) -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let mut fi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if size <= ti_fi_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2.0f64 / (period + 1.0f64);
    let mut ema: libc::c_double = *volume.offset(1 as libc::c_int as isize)
        * (*close.offset(1 as libc::c_int as isize)
            - *close.offset(0 as libc::c_int as isize));
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < size {
        ema = (*volume.offset(i as isize)
            * (*close.offset(i as isize)
                - *close.offset((i - 1 as libc::c_int) as isize)) - ema) * per + ema;
        let fresh0 = fi;
        fi = fi.offset(1);
        *fresh0 = ema;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fi_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let mut fi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_fi_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < size {
        *fi
            .offset(
                (i - 1 as libc::c_int) as isize,
            ) = *volume.offset(i as isize)
            * (*close.offset(i as isize)
                - *close.offset((i - 1 as libc::c_int) as isize));
        i += 1;
        i;
    }
    let mut ti_ema_inputs: [*const libc::c_double; 1] = [fi as *const libc::c_double];
    ti_ema(size - 1 as libc::c_int, ti_ema_inputs.as_mut_ptr(), &period, &mut fi);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fi_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ti_fi_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_fi = stream_in as *mut ti_stream_fi;
    let mut progress: libc::c_int = (*stream).progress;
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut fi: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut per: libc::c_double = (*stream).per;
    let mut ema: libc::c_double = (*stream).ema;
    let mut previous_close: libc::c_double = (*stream).previous_close;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size && progress == -(1 as libc::c_int) {
        previous_close = *close.offset(i as isize);
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size && progress == 0 as libc::c_int {
        ema = *volume.offset(i as isize) * (*close.offset(i as isize) - previous_close);
        previous_close = *close.offset(i as isize);
        let fresh1 = fi;
        fi = fi.offset(1);
        *fresh1 = ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    while i < size {
        ema = (*volume.offset(i as isize) * (*close.offset(i as isize) - previous_close)
            - ema) * per + ema;
        previous_close = *close.offset(i as isize);
        let fresh2 = fi;
        fi = fi.offset(1);
        *fresh2 = ema;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).ema = ema;
    (*stream).previous_close = previous_close;
    return 0 as libc::c_int;
}
