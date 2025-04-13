use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut ti_indicators: [ti_indicator_info; 0];
}
pub type ti_indicator_start_function = Option::<
    unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
>;
pub type ti_indicator_function = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const *const libc::c_double,
        *const libc::c_double,
        *const *mut libc::c_double,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream {
    pub index: libc::c_int,
    pub progress: libc::c_int,
}
pub type ti_indicator_stream_new = Option::<
    unsafe extern "C" fn(*const libc::c_double, *mut *mut ti_stream) -> libc::c_int,
>;
pub type ti_indicator_stream_run = Option::<
    unsafe extern "C" fn(
        *mut ti_stream,
        libc::c_int,
        *const *const libc::c_double,
        *const *mut libc::c_double,
    ) -> libc::c_int,
>;
pub type ti_indicator_stream_free = Option::<unsafe extern "C" fn(*mut ti_stream) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: *const libc::c_char,
    pub full_name: *const libc::c_char,
    pub start: ti_indicator_start_function,
    pub indicator: ti_indicator_function,
    pub indicator_ref: ti_indicator_function,
    pub type_0: libc::c_int,
    pub inputs: libc::c_int,
    pub options: libc::c_int,
    pub outputs: libc::c_int,
    pub input_names: [*const libc::c_char; 16],
    pub option_names: [*const libc::c_char; 16],
    pub output_names: [*const libc::c_char; 16],
    pub stream_new: ti_indicator_stream_new,
    pub stream_run: ti_indicator_stream_run,
    pub stream_free: ti_indicator_stream_free,
}
#[no_mangle]
pub unsafe extern "C" fn ti_indicator_count() -> libc::c_int {
    return 104 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_run(
    mut stream: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    return ((*ti_indicators.as_mut_ptr().offset((*stream).index as isize)).stream_run)
        .expect("non-null function pointer")(stream, size, inputs, outputs);
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_get_info(
    mut stream: *mut ti_stream,
) -> *mut ti_indicator_info {
    return ti_indicators.as_mut_ptr().offset((*stream).index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_get_progress(
    mut stream: *mut ti_stream,
) -> libc::c_int {
    return (*stream).progress;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stream_free(mut stream: *mut ti_stream) {
    ((*ti_indicators.as_mut_ptr().offset((*stream).index as isize)).stream_free)
        .expect("non-null function pointer")(stream);
}
#[no_mangle]
pub unsafe extern "C" fn ti_find_indicator(
    mut name: *const libc::c_char,
) -> *const ti_indicator_info {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = 0;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        let c: libc::c_int = strcmp(
            name,
            (*ti_indicators.as_mut_ptr().offset(i as isize)).name,
        );
        if c == 0 as libc::c_int {
            return ti_indicators.as_mut_ptr().offset(i as isize)
        } else if c > 0 as libc::c_int {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const ti_indicator_info;
}
