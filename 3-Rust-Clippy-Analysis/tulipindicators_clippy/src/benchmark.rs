use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ti_stream;
    pub type tc_result;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn clock() -> clock_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut ti_indicators: [ti_indicator_info; 0];
    fn ti_find_indicator(name: *const libc::c_char) -> *const ti_indicator_info;
    fn ti_stream_get_progress(stream: *mut ti_stream) -> libc::c_int;
    static mut tc_candles: [tc_candle_info; 0];
    fn tc_find_candle(name: *const libc::c_char) -> *const tc_candle_info;
    fn tc_config_set_to_default(config: *mut tc_config);
    fn tc_result_new() -> *mut tc_result;
    fn tc_result_free(result: *mut tc_result);
    fn tc_run(
        patterns: tc_set,
        size: libc::c_int,
        inputs: *const *const TC_REAL,
        options: *const tc_config,
        output: *mut tc_result,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type uint64_t = __uint64_t;
pub type TC_REAL = libc::c_double;
pub type tc_set = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_config {
    pub period: libc::c_int,
    pub body_none: TC_REAL,
    pub body_short: TC_REAL,
    pub body_long: TC_REAL,
    pub wick_none: TC_REAL,
    pub wick_long: TC_REAL,
    pub near: TC_REAL,
}
pub type tc_candle_function = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const *const TC_REAL,
        *const tc_config,
        *mut tc_result,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_candle_info {
    pub name: *const libc::c_char,
    pub full_name: *const libc::c_char,
    pub pattern: tc_set,
    pub candle: tc_candle_function,
}
pub type run_fun = Option::<
    unsafe extern "C" fn(
        *const ti_indicator_info,
        *mut libc::c_double,
        libc::c_int,
    ) -> libc::c_int,
>;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub static mut bestof: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut fails: libc::c_int = 0 as libc::c_int;
#[export_name = "in"]
pub static mut in_0: [[libc::c_double; 4000]; 5] = [[0.; 4000]; 5];
#[no_mangle]
pub static mut out: [[libc::c_double; 4000]; 3] = [[0.; 4000]; 3];
#[no_mangle]
pub static mut outref: [[libc::c_double; 4000]; 3] = [[0.; 4000]; 3];
#[no_mangle]
pub static mut outstream1: [[libc::c_double; 4000]; 3] = [[0.; 4000]; 3];
#[no_mangle]
pub static mut outstreamall: [[libc::c_double; 4000]; 3] = [[0.; 4000]; 3];
#[no_mangle]
pub static mut outcnd: *mut tc_result = 0 as *const tc_result as *mut tc_result;
#[no_mangle]
pub static mut ind_offset: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn generate_inputs() {
    let mut open: *mut libc::c_double = (in_0[0 as libc::c_int as usize]).as_mut_ptr();
    let mut high: *mut libc::c_double = (in_0[1 as libc::c_int as usize]).as_mut_ptr();
    let mut low: *mut libc::c_double = (in_0[2 as libc::c_int as usize]).as_mut_ptr();
    let mut close: *mut libc::c_double = (in_0[3 as libc::c_int as usize]).as_mut_ptr();
    let mut volume: *mut libc::c_double = (in_0[4 as libc::c_int as usize]).as_mut_ptr();
    *open.offset(0 as libc::c_int as isize) = 100 as libc::c_int as libc::c_double;
    srand(22 as libc::c_int as libc::c_uint);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4000 as libc::c_int {
        let diff1: libc::c_double = (rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double - 0.5f64 + 0.01f64) * 2.5f64;
        let diff2: libc::c_double = (rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double - 0.5f64 + 0.01f64) * 2.5f64;
        let diff3: libc::c_double = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double * 0.5f64;
        let diff4: libc::c_double = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double * 0.5f64;
        let vol: libc::c_double = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double
            * 10000 as libc::c_int as libc::c_double
            + 500 as libc::c_int as libc::c_double;
        if i != 0 {
            *open
                .offset(
                    i as isize,
                ) = *open.offset((i - 1 as libc::c_int) as isize) + diff1;
        }
        *close.offset(i as isize) = *open.offset(i as isize) + diff2;
        *high
            .offset(
                i as isize,
            ) = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize) + diff3
        } else {
            *close.offset(i as isize) + diff3
        };
        *low
            .offset(
                i as isize,
            ) = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize) - diff4
        } else {
            *close.offset(i as isize) - diff4
        };
        *volume.offset(i as isize) = vol;
        if *open.offset(i as isize) <= *high.offset(i as isize) {} else {
            __assert_fail(
                b"open[i] <= high[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *close.offset(i as isize) <= *high.offset(i as isize) {} else {
            __assert_fail(
                b"close[i] <= high[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *open.offset(i as isize) >= *low.offset(i as isize) {} else {
            __assert_fail(
                b"open[i] >= low[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *close.offset(i as isize) >= *low.offset(i as isize) {} else {
            __assert_fail(
                b"close[i] >= low[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *high.offset(i as isize) >= *low.offset(i as isize) {} else {
            __assert_fail(
                b"high[i] >= low[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *high.offset(i as isize) >= *open.offset(i as isize) {} else {
            __assert_fail(
                b"high[i] >= open[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                116 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *high.offset(i as isize) >= *close.offset(i as isize) {} else {
            __assert_fail(
                b"high[i] >= close[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *low.offset(i as isize) <= *low.offset(i as isize) {} else {
            __assert_fail(
                b"low[i] <= low[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *low.offset(i as isize) <= *open.offset(i as isize) {} else {
            __assert_fail(
                b"low[i] <= open[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        if *low.offset(i as isize) <= *close.offset(i as isize) {} else {
            __assert_fail(
                b"low[i] <= close[i]\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                121 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void generate_inputs()\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    *volume.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn ti_setup(
    mut info: *const ti_indicator_info,
    mut inputs: *mut *mut libc::c_double,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*info).inputs {
        if strcmp(
            (*info).input_names[j as usize],
            b"open\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let ref mut fresh0 = *inputs.offset(j as isize);
            *fresh0 = (in_0[0 as libc::c_int as usize]).as_mut_ptr();
        } else if strcmp(
            (*info).input_names[j as usize],
            b"high\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let ref mut fresh1 = *inputs.offset(j as isize);
            *fresh1 = (in_0[1 as libc::c_int as usize]).as_mut_ptr();
        } else if strcmp(
            (*info).input_names[j as usize],
            b"low\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let ref mut fresh2 = *inputs.offset(j as isize);
            *fresh2 = (in_0[2 as libc::c_int as usize]).as_mut_ptr();
        } else if strcmp(
            (*info).input_names[j as usize],
            b"close\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let ref mut fresh3 = *inputs.offset(j as isize);
            *fresh3 = (in_0[3 as libc::c_int as usize]).as_mut_ptr();
        } else if strcmp(
            (*info).input_names[j as usize],
            b"real\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let ref mut fresh4 = *inputs.offset(j as isize);
            *fresh4 = (in_0[3 as libc::c_int as usize]).as_mut_ptr();
        } else if strcmp(
            (*info).input_names[j as usize],
            b"volume\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let ref mut fresh5 = *inputs.offset(j as isize);
            *fresh5 = (in_0[4 as libc::c_int as usize]).as_mut_ptr();
        } else {
            if 0 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"benchmark.c\0" as *const u8 as *const libc::c_char,
                    368 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 52],
                        &[libc::c_char; 52],
                    >(b"void ti_setup(const ti_indicator_info *, double **)\0"))
                        .as_ptr(),
                );
            };
        }
        j += 1;
        j;
    }
}
static mut outputs: [*mut libc::c_double; 5] = [0 as *const libc::c_double
    as *mut libc::c_double; 5];
#[no_mangle]
pub unsafe extern "C" fn run_ti(
    mut info: *const ti_indicator_info,
    mut options: *mut libc::c_double,
    mut goal: libc::c_int,
) -> libc::c_int {
    static mut inputs: [*mut libc::c_double; 5] = [
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
    ];
    if goal == 0 as libc::c_int {
        ind_offset = ((*info).start).expect("non-null function pointer")(options);
        let ret: libc::c_int = ((*info).indicator)
            .expect(
                "non-null function pointer",
            )(
            4000 as libc::c_int,
            inputs.as_mut_ptr() as *const *const libc::c_double,
            options,
            outputs.as_mut_ptr(),
        );
        if ret != 0 as libc::c_int {
            printf(b"   *ERROR* (%d)\n\0" as *const u8 as *const libc::c_char, ret);
            printf(b"options:\n\0" as *const u8 as *const libc::c_char);
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*info).options {
                printf(
                    b"%d = %f\n\0" as *const u8 as *const libc::c_char,
                    i,
                    *options.offset(i as isize),
                );
                i += 1;
                i;
            }
            return 1 as libc::c_int;
        }
    } else if goal == 1 as libc::c_int {
        ti_setup(info, inputs.as_mut_ptr());
    } else if !(goal == 2 as libc::c_int) {
        if 0 as libc::c_int != 0 {} else {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                400 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"int run_ti(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
    }
    return 0 as libc::c_int;
}
static mut outputs_0: [*mut libc::c_double; 5] = [0 as *const libc::c_double
    as *mut libc::c_double; 5];
#[no_mangle]
pub unsafe extern "C" fn run_ti_ref(
    mut info: *const ti_indicator_info,
    mut options: *mut libc::c_double,
    mut goal: libc::c_int,
) -> libc::c_int {
    static mut inputs: [*mut libc::c_double; 5] = [
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
    ];
    if goal == 0 as libc::c_int {
        ind_offset = ((*info).start).expect("non-null function pointer")(options);
        let ret: libc::c_int = ((*info).indicator_ref)
            .expect(
                "non-null function pointer",
            )(
            4000 as libc::c_int,
            inputs.as_mut_ptr() as *const *const libc::c_double,
            options,
            outputs_0.as_mut_ptr(),
        );
        if ret != 0 as libc::c_int {
            printf(b"   *ERROR* (%d)\n\0" as *const u8 as *const libc::c_char, ret);
            printf(b"options:\n\0" as *const u8 as *const libc::c_char);
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*info).options {
                printf(
                    b"%d = %f\n\0" as *const u8 as *const libc::c_char,
                    i,
                    *options.offset(i as isize),
                );
                i += 1;
                i;
            }
            return 1 as libc::c_int;
        }
    } else if goal == 1 as libc::c_int {
        ti_setup(info, inputs.as_mut_ptr());
    } else if !(goal == 2 as libc::c_int) {
        if 0 as libc::c_int != 0 {} else {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                433 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"int run_ti_ref(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
    }
    return 0 as libc::c_int;
}
static mut outputs_1: [*mut libc::c_double; 5] = [0 as *const libc::c_double
    as *mut libc::c_double; 5];
#[no_mangle]
pub unsafe extern "C" fn run_ti_stream1(
    mut info: *const ti_indicator_info,
    mut options: *mut libc::c_double,
    mut goal: libc::c_int,
) -> libc::c_int {
    static mut inputs: [*mut libc::c_double; 5] = [
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
    ];
    if goal == 0 as libc::c_int {
        ind_offset = ((*info).start).expect("non-null function pointer")(options);
        let mut ins: [*mut libc::c_double; 5] = [
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
        ];
        let mut outs: [*mut libc::c_double; 5] = [
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
        ];
        let mut stream: *mut ti_stream = 0 as *mut ti_stream;
        let sret: libc::c_int = ((*info).stream_new)
            .expect("non-null function pointer")(options, &mut stream);
        if !stream.is_null() {} else {
            __assert_fail(
                b"stream\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                455 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int run_ti_stream1(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
        if sret == 0 as libc::c_int {} else {
            __assert_fail(
                b"sret == TI_OKAY\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                456 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int run_ti_stream1(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4000 as libc::c_int {
            let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while (j as libc::c_ulong)
                < (::core::mem::size_of::<[*mut libc::c_double; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                    )
            {
                ins[j as usize] = (inputs[j as usize]).offset(i as isize);
                j = j.wrapping_add(1);
                j;
            }
            let mut k: libc::c_int = ti_stream_get_progress(stream);
            if k < 0 as libc::c_int {
                k = 0 as libc::c_int;
            }
            let mut j_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while (j_0 as libc::c_ulong)
                < (::core::mem::size_of::<[*mut libc::c_double; 5]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                    )
            {
                outs[j_0
                    as usize] = (outstream1[j_0 as usize])
                    .as_mut_ptr()
                    .offset(k as isize);
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
            let ret: libc::c_int = ((*info).stream_run)
                .expect(
                    "non-null function pointer",
                )(
                stream,
                1 as libc::c_int,
                ins.as_mut_ptr() as *const *const libc::c_double,
                outs.as_mut_ptr(),
            );
            if ret != 0 as libc::c_int {
                printf(b"   *ERROR* (%d)\n\0" as *const u8 as *const libc::c_char, ret);
                printf(b"options:\n\0" as *const u8 as *const libc::c_char);
                let mut j_1: libc::c_int = 0 as libc::c_int;
                while j_1 < (*info).options {
                    printf(
                        b"%d = %f\n\0" as *const u8 as *const libc::c_char,
                        j_1,
                        *options.offset(j_1 as isize),
                    );
                    j_1 += 1;
                    j_1;
                }
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        ((*info).stream_free).expect("non-null function pointer")(stream);
    } else if goal == 1 as libc::c_int {
        ti_setup(info, inputs.as_mut_ptr());
    } else if !(goal == 2 as libc::c_int) {
        if 0 as libc::c_int != 0 {} else {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                493 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"int run_ti_stream1(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
    }
    return 0 as libc::c_int;
}
static mut outputs_2: [*mut libc::c_double; 5] = [0 as *const libc::c_double
    as *mut libc::c_double; 5];
#[no_mangle]
pub unsafe extern "C" fn run_ti_streamall(
    mut info: *const ti_indicator_info,
    mut options: *mut libc::c_double,
    mut goal: libc::c_int,
) -> libc::c_int {
    static mut inputs: [*mut libc::c_double; 5] = [
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
        0 as *const libc::c_double as *mut libc::c_double,
    ];
    if goal == 0 as libc::c_int {
        ind_offset = ((*info).start).expect("non-null function pointer")(options);
        let mut stream: *mut ti_stream = 0 as *mut ti_stream;
        let sret: libc::c_int = ((*info).stream_new)
            .expect("non-null function pointer")(options, &mut stream);
        if !stream.is_null() {} else {
            __assert_fail(
                b"stream\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                511 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"int run_ti_streamall(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
        if sret == 0 as libc::c_int {} else {
            __assert_fail(
                b"sret == TI_OKAY\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                512 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"int run_ti_streamall(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
        let ret: libc::c_int = ((*info).stream_run)
            .expect(
                "non-null function pointer",
            )(
            stream,
            4000 as libc::c_int,
            inputs.as_mut_ptr() as *const *const libc::c_double,
            outputs_2.as_mut_ptr(),
        );
        ((*info).stream_free).expect("non-null function pointer")(stream);
        if ret != 0 as libc::c_int {
            printf(b"   *ERROR* (%d)\n\0" as *const u8 as *const libc::c_char, ret);
            printf(b"options:\n\0" as *const u8 as *const libc::c_char);
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*info).options {
                printf(
                    b"%d = %f\n\0" as *const u8 as *const libc::c_char,
                    i,
                    *options.offset(i as isize),
                );
                i += 1;
                i;
            }
            return 1 as libc::c_int;
        }
    } else if goal == 1 as libc::c_int {
        ti_setup(info, inputs.as_mut_ptr());
    } else if !(goal == 2 as libc::c_int) {
        if 0 as libc::c_int != 0 {} else {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"benchmark.c\0" as *const u8 as *const libc::c_char,
                534 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"int run_ti_streamall(const ti_indicator_info *, double *, int)\0"))
                    .as_ptr(),
            );
        };
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn simple_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
}
#[no_mangle]
pub unsafe extern "C" fn fast_slow_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options
        .offset(
            1 as libc::c_int as isize,
        ) = period + 10 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn alma_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 0.5f64;
    *options.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn bbands_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 1.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn ce_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn copp_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = 11 as libc::c_int as libc::c_double;
    *options.offset(1 as libc::c_int as isize) = 14 as libc::c_int as libc::c_double;
    *options.offset(2 as libc::c_int as isize) = period;
}
#[no_mangle]
pub unsafe extern "C" fn kc_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 0.77f64;
}
#[no_mangle]
pub unsafe extern "C" fn kst_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh6 = *options.offset(i as isize);
        *fresh6 = period + period / 4.0f64 * i as libc::c_double;
        *options.offset((i + 4 as libc::c_int) as isize) = *fresh6;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn macd_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options
        .offset(
            1 as libc::c_int as isize,
        ) = period + 10 as libc::c_int as libc::c_double;
    *options
        .offset(2 as libc::c_int as isize) = period + 1 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn mama_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = 0.5f64;
    *options.offset(1 as libc::c_int as isize) = 0.05f64;
}
#[no_mangle]
pub unsafe extern "C" fn pfe_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 5 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn posc_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn ppo_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options
        .offset(
            1 as libc::c_int as isize,
        ) = period + 10 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn psar_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double / period;
    *options
        .offset(
            1 as libc::c_int as isize,
        ) = *options.offset(0 as libc::c_int as isize)
        * 10 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rmi_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rmta_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options
        .offset(
            1 as libc::c_int as isize,
        ) = 1 as libc::c_int as libc::c_double
        - 2.0f64 / (period + 1 as libc::c_int as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn rvi_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 10 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn smi_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 25 as libc::c_int as libc::c_double;
    *options.offset(2 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn stoch_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    if ti != 0 {
        *options.offset(0 as libc::c_int as isize) = period;
        *options.offset(1 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double;
        *options.offset(2 as libc::c_int as isize) = 4 as libc::c_int as libc::c_double;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stochrsi_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = period;
    *options.offset(2 as libc::c_int as isize) = period;
    *options.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn tsi_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options.offset(1 as libc::c_int as isize) = 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn ultosc_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options
        .offset(1 as libc::c_int as isize) = period * 2 as libc::c_int as libc::c_double;
    *options
        .offset(2 as libc::c_int as isize) = period * 4 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn vidya_option_setter(
    mut period: libc::c_double,
    mut options: *mut libc::c_double,
    mut ti: libc::c_int,
) {
    *options.offset(0 as libc::c_int as isize) = period;
    *options
        .offset(
            1 as libc::c_int as isize,
        ) = period + 10 as libc::c_int as libc::c_double;
    *options.offset(2 as libc::c_int as isize) = 0.2f64;
}
#[no_mangle]
pub unsafe extern "C" fn stochrsi_output_adjust(
    mut a: libc::c_double,
) -> libc::c_double {
    return a * 0.01f64;
}
#[no_mangle]
pub unsafe extern "C" fn check_outputs(
    mut info: *const ti_indicator_info,
    mut what: *const libc::c_char,
    mut compare: *mut [libc::c_double; 4000],
) {
    let mut remap: [libc::c_int; 5] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
    ];
    let mut nanok: libc::c_int = 0 as libc::c_int;
    if strcmp((*info).name, b"asin\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        nanok = 1 as libc::c_int;
    }
    if strcmp((*info).name, b"acos\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        nanok = 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*info).outputs {
        let mut end: libc::c_int = 4000 as libc::c_int - ind_offset;
        if !(remap[j as usize] == -(1 as libc::c_int)) {
            i = 0 as libc::c_int;
            while i < end {
                let mut a: libc::c_double = out[j
                    as usize][(4000 as libc::c_int - 1 as libc::c_int - i - ind_offset)
                    as usize];
                let mut b: libc::c_double = (*compare
                    .offset(
                        j as isize,
                    ))[(4000 as libc::c_int - 1 as libc::c_int - i - ind_offset)
                    as usize];
                let mut diff: libc::c_double = fabs(a - b);
                if diff > 0.0001f64 || nanok == 0 && diff != diff {
                    fails += 1;
                    fails;
                    printf(b"Results disagree.\n\0" as *const u8 as *const libc::c_char);
                    printf(
                        b"    series         (diff)   ti   %s\n\0" as *const u8
                            as *const libc::c_char,
                        what,
                    );
                    printf(
                        b"    series %d, %4d (%f) %.5f %.5f\n\0" as *const u8
                            as *const libc::c_char,
                        j,
                        i,
                        diff,
                        a,
                        b,
                    );
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bench_run(
    mut log: *mut FILE,
    mut info: *const ti_indicator_info,
    mut run_info: *const libc::c_void,
    mut fun: run_fun,
    mut name: *const libc::c_char,
) {
    let mut options: [libc::c_double; 10] = [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut options_setter: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_double, libc::c_int) -> (),
    > = None;
    if strcmp((*info).name, b"alma\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            alma_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"adosc\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            fast_slow_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"apo\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            ppo_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"bbands\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            bbands_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"copp\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            copp_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"kc\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            kc_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"kst\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            kst_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"kvo\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            fast_slow_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"macd\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            macd_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"mama\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            mama_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"pfe\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            pfe_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"posc\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            posc_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"ppo\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            ppo_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"psar\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            psar_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"rvi\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            rvi_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"rmi\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            rmi_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"rmta\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            rmta_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"smi\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            smi_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"stoch\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            stoch_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"stochrsi\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            stochrsi_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"ultosc\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            ultosc_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"vidya\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            vidya_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    if strcmp((*info).name, b"vosc\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        options_setter = Some(
            fast_slow_option_setter
                as unsafe extern "C" fn(
                    libc::c_double,
                    *mut libc::c_double,
                    libc::c_int,
                ) -> (),
        );
    }
    let mut best_e: libc::c_int = 1e9f64 as libc::c_int;
    let mut best_p: libc::c_int = -1e9f64 as libc::c_int;
    let mut best: libc::c_int = 0;
    best = 0 as libc::c_int;
    while best < bestof {
        printf(
            b"Benchmark %25s-%-16s \0" as *const u8 as *const libc::c_char,
            (*info).name,
            name,
        );
        let mut i: libc::c_int = 0;
        let mut period: libc::c_int = 0;
        let mut iterations: libc::c_int = 0 as libc::c_int;
        let start: clock_t = clock();
        fun
            .expect(
                "non-null function pointer",
            )(
            run_info as *const ti_indicator_info,
            0 as *mut libc::c_double,
            1 as libc::c_int,
        );
        i = 0 as libc::c_int;
        while i < 1 as libc::c_int {
            period = 4 as libc::c_int;
            while period <= 150 as libc::c_int {
                if options_setter.is_some() {
                    options_setter
                        .expect(
                            "non-null function pointer",
                        )(
                        period as libc::c_double,
                        options.as_mut_ptr(),
                        (info == run_info as *const ti_indicator_info) as libc::c_int,
                    );
                } else {
                    options[0 as libc::c_int as usize] = period as libc::c_double;
                }
                let ret: libc::c_int = fun
                    .expect(
                        "non-null function pointer",
                    )(
                    run_info as *const ti_indicator_info,
                    options.as_mut_ptr(),
                    0 as libc::c_int,
                );
                if ret != 0 {
                    return;
                }
                iterations += 1;
                iterations;
                period += 1;
                period;
            }
            i += 1;
            i;
        }
        fun
            .expect(
                "non-null function pointer",
            )(
            run_info as *const ti_indicator_info,
            0 as *mut libc::c_double,
            2 as libc::c_int,
        );
        let elapsed: libc::c_int = ((clock() - start) * 1000 as libc::c_int as clock_t
            / 1000000 as libc::c_int as __clock_t) as libc::c_int;
        let performance: libc::c_int = if elapsed == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            iterations * 4000 as libc::c_int / elapsed / 1000 as libc::c_int
        };
        if elapsed != 0 {
            printf(
                b"\t%5dms\t%5dmfps\n\0" as *const u8 as *const libc::c_char,
                elapsed,
                performance,
            );
        } else {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        best_e = if elapsed < best_e { elapsed } else { best_e };
        best_p = if performance > best_p { performance } else { best_p };
        best += 1;
        best;
    }
    if !log.is_null() {
        fprintf(
            log,
            b",\n    \"%s\" => array(\"elapsed\" => %d, \"performance\" => %d)\0"
                as *const u8 as *const libc::c_char,
            name,
            best_e,
            best_p,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn bench_indicator(
    mut log: *mut FILE,
    mut info: *const ti_indicator_info,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        memset(
            (out[i as usize]).as_mut_ptr() as *mut libc::c_void,
            0xf1 as libc::c_int,
            ::core::mem::size_of::<[libc::c_double; 4000]>() as libc::c_ulong,
        );
        memset(
            (outref[i as usize]).as_mut_ptr() as *mut libc::c_void,
            0xf2 as libc::c_int,
            ::core::mem::size_of::<[libc::c_double; 4000]>() as libc::c_ulong,
        );
        memset(
            (outstream1[i as usize]).as_mut_ptr() as *mut libc::c_void,
            0xf3 as libc::c_int,
            ::core::mem::size_of::<[libc::c_double; 4000]>() as libc::c_ulong,
        );
        memset(
            (outstreamall[i as usize]).as_mut_ptr() as *mut libc::c_void,
            0xf4 as libc::c_int,
            ::core::mem::size_of::<[libc::c_double; 4000]>() as libc::c_ulong,
        );
        i += 1;
        i;
    }
    if !info.is_null() {
        if !log.is_null() {
            fprintf(
                log,
                b",\n  \"%s\" => array(\n\0" as *const u8 as *const libc::c_char,
                (*info).name,
            );
        }
        if !log.is_null() {
            fprintf(
                log,
                b"    \"name\" => \"%s\"\0" as *const u8 as *const libc::c_char,
                (*info).full_name,
            );
        }
        bench_run(
            log,
            info,
            info as *const libc::c_void,
            Some(
                run_ti
                    as unsafe extern "C" fn(
                        *const ti_indicator_info,
                        *mut libc::c_double,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            b"ti\0" as *const u8 as *const libc::c_char,
        );
    }
    if !info.is_null() && ((*info).indicator_ref).is_some() {
        bench_run(
            log,
            info,
            info as *const libc::c_void,
            Some(
                run_ti_ref
                    as unsafe extern "C" fn(
                        *const ti_indicator_info,
                        *mut libc::c_double,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            b"ti_ref\0" as *const u8 as *const libc::c_char,
        );
        check_outputs(
            info,
            b"ref\0" as *const u8 as *const libc::c_char,
            outref.as_mut_ptr(),
        );
    }
    if !info.is_null() && ((*info).stream_new).is_some() {
        bench_run(
            log,
            info,
            info as *const libc::c_void,
            Some(
                run_ti_stream1
                    as unsafe extern "C" fn(
                        *const ti_indicator_info,
                        *mut libc::c_double,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            b"ti_stream1\0" as *const u8 as *const libc::c_char,
        );
        check_outputs(
            info,
            b"stream1\0" as *const u8 as *const libc::c_char,
            outstream1.as_mut_ptr(),
        );
        bench_run(
            log,
            info,
            info as *const libc::c_void,
            Some(
                run_ti_streamall
                    as unsafe extern "C" fn(
                        *const ti_indicator_info,
                        *mut libc::c_double,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            b"ti_streamall\0" as *const u8 as *const libc::c_char,
        );
        check_outputs(
            info,
            b"streamall\0" as *const u8 as *const libc::c_char,
            outstreamall.as_mut_ptr(),
        );
    }
    if !log.is_null() {
        fprintf(log, b"\n  )\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn bench_run_candle(
    mut log: *mut FILE,
    mut info: *const tc_candle_info,
) {
    let mut name: *const libc::c_char = b"ti\0" as *const u8 as *const libc::c_char;
    let mut pattern: tc_set = (*info).pattern;
    let mut best_e: libc::c_int = 1e9f64 as libc::c_int;
    let mut best_p: libc::c_int = -1e9f64 as libc::c_int;
    let mut best: libc::c_int = 0;
    best = 0 as libc::c_int;
    while best < bestof {
        printf(
            b"Benchmark %25s-%-16s \0" as *const u8 as *const libc::c_char,
            (*info).name,
            name,
        );
        let mut i: libc::c_int = 0;
        let mut period: libc::c_int = 0;
        let mut iterations: libc::c_int = 0 as libc::c_int;
        let start: clock_t = clock();
        let mut inputs: [*const libc::c_double; 4] = [
            (in_0[0 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_double,
            (in_0[1 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_double,
            (in_0[2 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_double,
            (in_0[3 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_double,
        ];
        let mut config: tc_config = tc_config {
            period: 0,
            body_none: 0.,
            body_short: 0.,
            body_long: 0.,
            wick_none: 0.,
            wick_long: 0.,
            near: 0.,
        };
        tc_config_set_to_default(&mut config);
        i = 0 as libc::c_int;
        while i < 1 as libc::c_int {
            period = 4 as libc::c_int;
            while period <= 150 as libc::c_int {
                let ret: libc::c_int = tc_run(
                    pattern,
                    4000 as libc::c_int,
                    inputs.as_mut_ptr() as *const *const TC_REAL,
                    &mut config,
                    outcnd,
                );
                if ret != 0 {
                    return;
                }
                iterations += 1;
                iterations;
                period += 1;
                period;
            }
            i += 1;
            i;
        }
        let elapsed: libc::c_int = ((clock() - start) * 1000 as libc::c_int as clock_t
            / 1000000 as libc::c_int as __clock_t) as libc::c_int;
        let performance: libc::c_int = if elapsed == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            iterations * 4000 as libc::c_int / elapsed / 1000 as libc::c_int
        };
        if elapsed != 0 {
            printf(
                b"\t%5dms\t%5dmfps\n\0" as *const u8 as *const libc::c_char,
                elapsed,
                performance,
            );
        } else {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        best_e = if elapsed < best_e { elapsed } else { best_e };
        best_p = if performance > best_p { performance } else { best_p };
        best += 1;
        best;
    }
    if !log.is_null() {
        fprintf(
            log,
            b",\n    \"%s\" => array(\"elapsed\" => %d, \"performance\" => %d)\0"
                as *const u8 as *const libc::c_char,
            name,
            best_e,
            best_p,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn bench_candle(
    mut log: *mut FILE,
    mut info: *const tc_candle_info,
) {
    outcnd = tc_result_new();
    if !outcnd.is_null() {} else {
        __assert_fail(
            b"outcnd\0" as *const u8 as *const libc::c_char,
            b"benchmark.c\0" as *const u8 as *const libc::c_char,
            1053 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void bench_candle(FILE *, const tc_candle_info *)\0"))
                .as_ptr(),
        );
    };
    if !info.is_null() {
        if !log.is_null() {
            fprintf(
                log,
                b",\n  \"%s\" => array(\n\0" as *const u8 as *const libc::c_char,
                (*info).name,
            );
        }
        if !log.is_null() {
            fprintf(
                log,
                b"    \"name\" => \"%s\"\0" as *const u8 as *const libc::c_char,
                (*info).full_name,
            );
        }
        bench_run_candle(log, info);
        tc_result_free(outcnd);
    }
    if !log.is_null() {
        fprintf(log, b"\n  )\0" as *const u8 as *const libc::c_char);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"Tulip Charts Indicator benchmark.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Using real size of: %d.\n\n\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    generate_inputs();
    let mut ind_info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
    let mut cnd_info: *const tc_candle_info = tc_candles.as_mut_ptr();
    let candles_only: libc::c_int = (argc > 1 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"tc\0" as *const u8 as *const libc::c_char,
        ) == 0) as libc::c_int;
    if argc > 1 as libc::c_int
        && *(*__ctype_b_loc())
            .offset(
                *(*argv.offset(1 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0 && candles_only == 0
    {
        if argc > 2 as libc::c_int {
            bestof = atoi(*argv.offset(2 as libc::c_int as isize));
            if bestof == 0 {
                bestof = 1 as libc::c_int;
            }
        }
        ind_info = ti_find_indicator(*argv.offset(1 as libc::c_int as isize));
        if !ind_info.is_null() {
            bench_indicator(0 as *mut FILE, ind_info);
        }
        cnd_info = tc_find_candle(*argv.offset(1 as libc::c_int as isize));
        if !cnd_info.is_null() {
            bench_candle(0 as *mut FILE, cnd_info);
        }
    } else {
        if argc > 1 as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *(*argv.offset(1 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            bestof = atoi(*argv.offset(1 as libc::c_int as isize));
        }
        let mut log: *mut FILE = fopen(
            b"docs/benchmark.php\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if log.is_null() {
            printf(
                b"Couldn't open results file for output. Make sure ./docs/ exists.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        fprintf(log, b"<?php\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            log,
            b"#GENERATED BY benchmark.c\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(log, b"#DO NOT MODIFY DIRECTLY\n\0" as *const u8 as *const libc::c_char);
        fprintf(log, b"$benchmark = array(\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            log,
            b"  \"worksize\" => %d,\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int * (150 as libc::c_int - 4 as libc::c_int + 1 as libc::c_int)
                * 4000 as libc::c_int,
        );
        fprintf(
            log,
            b"  \"bestof\" => %d\0" as *const u8 as *const libc::c_char,
            bestof,
        );
        if candles_only == 0 {
            loop {
                if !(strcmp(
                    (*ind_info).name,
                    b"msw\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
                {
                    bench_indicator(log, ind_info);
                }
                ind_info = ind_info.offset(1);
                if ((*ind_info).name).is_null() {
                    break;
                }
            }
        }
        loop {
            bench_candle(log, cnd_info);
            cnd_info = cnd_info.offset(1);
            if ((*cnd_info).name).is_null() {
                break;
            }
        }
        let cnd_all: tc_candle_info = {
            let mut init = tc_candle_info {
                name: b"all\0" as *const u8 as *const libc::c_char,
                full_name: b"All Candle Patterns\0" as *const u8 as *const libc::c_char,
                pattern: (((1 as libc::c_long) << 26 as libc::c_int)
                    - 1 as libc::c_int as libc::c_long) as tc_set,
                candle: None,
            };
            init
        };
        bench_candle(log, &cnd_all);
        fprintf(log, b");\0" as *const u8 as *const libc::c_char);
        fclose(log);
    }
    if fails == 0 {
        printf(b"All comparisons passed.\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"%d comparisons failed.\n\0" as *const u8 as *const libc::c_char, fails);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    outputs = [
        (out[0 as libc::c_int as usize]).as_ptr() as *mut _,
        (out[1 as libc::c_int as usize]).as_ptr() as *mut _,
        (out[2 as libc::c_int as usize]).as_ptr() as *mut _,
        (out[3 as libc::c_int as usize]).as_ptr() as *mut _,
        (out[4 as libc::c_int as usize]).as_ptr() as *mut _,
    ];
    outputs_0 = [
        (outref[0 as libc::c_int as usize]).as_ptr() as *mut _,
        (outref[1 as libc::c_int as usize]).as_ptr() as *mut _,
        (outref[2 as libc::c_int as usize]).as_ptr() as *mut _,
        (outref[3 as libc::c_int as usize]).as_ptr() as *mut _,
        (outref[4 as libc::c_int as usize]).as_ptr() as *mut _,
    ];
    outputs_1 = [
        (outstream1[0 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstream1[1 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstream1[2 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstream1[3 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstream1[4 as libc::c_int as usize]).as_ptr() as *mut _,
    ];
    outputs_2 = [
        (outstreamall[0 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstreamall[1 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstreamall[2 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstreamall[3 as libc::c_int as usize]).as_ptr() as *mut _,
        (outstreamall[4 as libc::c_int as usize]).as_ptr() as *mut _,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
