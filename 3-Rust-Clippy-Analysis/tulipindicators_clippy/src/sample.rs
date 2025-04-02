use ::libc;
extern "C" {
    pub type ti_stream;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut ti_indicators: [ti_indicator_info; 0];
    fn ti_find_indicator(name: *const libc::c_char) -> *const ti_indicator_info;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[no_mangle]
pub static mut out: [[libc::c_double; 15]; 5] = [[0.; 15]; 5];
#[no_mangle]
pub static mut datet: [*const libc::c_char; 15] = [
    b"2005-11-01\0" as *const u8 as *const libc::c_char,
    b"2005-11-02\0" as *const u8 as *const libc::c_char,
    b"2005-11-03\0" as *const u8 as *const libc::c_char,
    b"2005-11-04\0" as *const u8 as *const libc::c_char,
    b"2005-11-07\0" as *const u8 as *const libc::c_char,
    b"2005-11-08\0" as *const u8 as *const libc::c_char,
    b"2005-11-09\0" as *const u8 as *const libc::c_char,
    b"2005-11-10\0" as *const u8 as *const libc::c_char,
    b"2005-11-11\0" as *const u8 as *const libc::c_char,
    b"2005-11-14\0" as *const u8 as *const libc::c_char,
    b"2005-11-15\0" as *const u8 as *const libc::c_char,
    b"2005-11-16\0" as *const u8 as *const libc::c_char,
    b"2005-11-17\0" as *const u8 as *const libc::c_char,
    b"2005-11-18\0" as *const u8 as *const libc::c_char,
    b"2005-11-21\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut date: [libc::c_double; 15] = [
    51101 as libc::c_int as libc::c_double,
    51102 as libc::c_int as libc::c_double,
    51103 as libc::c_int as libc::c_double,
    51104 as libc::c_int as libc::c_double,
    51107 as libc::c_int as libc::c_double,
    51108 as libc::c_int as libc::c_double,
    51109 as libc::c_int as libc::c_double,
    51110 as libc::c_int as libc::c_double,
    51111 as libc::c_int as libc::c_double,
    51114 as libc::c_int as libc::c_double,
    51115 as libc::c_int as libc::c_double,
    51116 as libc::c_int as libc::c_double,
    51117 as libc::c_int as libc::c_double,
    51118 as libc::c_int as libc::c_double,
    51121 as libc::c_int as libc::c_double,
];
#[no_mangle]
pub static mut open: [libc::c_double; 15] = [
    81.85f64,
    81.2f64,
    81.55f64,
    82.91f64,
    83.1f64,
    83.41f64,
    82.71f64,
    82.7f64,
    84.2f64,
    84.25f64,
    84.03f64,
    85.45f64,
    86.18f64,
    88 as libc::c_int as libc::c_double,
    87.6f64,
];
#[no_mangle]
pub static mut high: [libc::c_double; 15] = [
    82.15f64,
    81.89f64,
    83.03f64,
    83.3f64,
    83.85f64,
    83.9f64,
    83.33f64,
    84.3f64,
    84.84f64,
    85 as libc::c_int as libc::c_double,
    85.9f64,
    86.58f64,
    86.98f64,
    88 as libc::c_int as libc::c_double,
    87.87f64,
];
#[no_mangle]
pub static mut low: [libc::c_double; 15] = [
    81.29f64,
    80.64f64,
    81.31f64,
    82.65f64,
    83.07f64,
    83.11f64,
    82.49f64,
    82.3f64,
    84.15f64,
    84.11f64,
    84.03f64,
    85.39f64,
    85.76f64,
    87.17f64,
    87.01f64,
];
#[no_mangle]
pub static mut close: [libc::c_double; 15] = [
    81.59f64,
    81.06f64,
    82.87f64,
    83 as libc::c_int as libc::c_double,
    83.61f64,
    83.15f64,
    82.84f64,
    83.99f64,
    84.55f64,
    84.36f64,
    85.53f64,
    86.54f64,
    86.89f64,
    87.77f64,
    87.29f64,
];
#[no_mangle]
pub static mut volume: [libc::c_double; 15] = [
    5653100 as libc::c_int as libc::c_double,
    6447400 as libc::c_int as libc::c_double,
    7690900 as libc::c_int as libc::c_double,
    3831400 as libc::c_int as libc::c_double,
    4455100 as libc::c_int as libc::c_double,
    3798000 as libc::c_int as libc::c_double,
    3936200 as libc::c_int as libc::c_double,
    4732000 as libc::c_int as libc::c_double,
    4841300 as libc::c_int as libc::c_double,
    3915300 as libc::c_int as libc::c_double,
    6830800 as libc::c_int as libc::c_double,
    6694100 as libc::c_int as libc::c_double,
    5293600 as libc::c_int as libc::c_double,
    7985800 as libc::c_int as libc::c_double,
    4807900 as libc::c_int as libc::c_double,
];
#[no_mangle]
pub static mut alternative: [libc::c_double; 15] = [
    0.2f64,
    0.3f64,
    0.4f64,
    0.3f64,
    0.5f64,
    0.7f64,
    0.75f64,
    0.9f64,
    0.9f64,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    0.2f64,
    0.1f64,
    -0.1f64,
    -0.5f64,
];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
    if argc < 2 as libc::c_int {
        printf(b"No indicator given.\n\0" as *const u8 as *const libc::c_char);
        printf(b"Example:\n\0" as *const u8 as *const libc::c_char);
        printf(b"\tsample ma 5\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if strcmp(
        *argv.offset(1 as libc::c_int as isize),
        b"--version\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        printf(
            b"TI VERSION: %s, TI BUILD: %d\n\0" as *const u8 as *const libc::c_char,
            b"0.9.2\0" as *const u8 as *const libc::c_char,
            1743602909 as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if strcmp(
        *argv.offset(1 as libc::c_int as isize),
        b"--list\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        loop {
            if (*info).type_0 == 1 as libc::c_int {
                printf(b"type overlay \0" as *const u8 as *const libc::c_char);
            } else if (*info).type_0 == 2 as libc::c_int {
                printf(b"type indicator \0" as *const u8 as *const libc::c_char);
            } else if (*info).type_0 == 3 as libc::c_int {
                printf(b"type math \0" as *const u8 as *const libc::c_char);
            } else if (*info).type_0 == 4 as libc::c_int {
                printf(b"type simple \0" as *const u8 as *const libc::c_char);
            } else {
                printf(b"type unknown \0" as *const u8 as *const libc::c_char);
            }
            printf(b"name %s \0" as *const u8 as *const libc::c_char, (*info).name);
            printf(
                b"full_name {%s} \0" as *const u8 as *const libc::c_char,
                (*info).full_name,
            );
            let mut i: libc::c_int = 0;
            printf(b"inputs {\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < (*info).inputs {
                printf(
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    if i != 0 {
                        b" \0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*info).input_names[i as usize],
                );
                i += 1;
                i;
            }
            printf(b"} \0" as *const u8 as *const libc::c_char);
            printf(b"options {\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < (*info).options {
                printf(
                    b"%s{%s}\0" as *const u8 as *const libc::c_char,
                    if i != 0 {
                        b" \0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*info).option_names[i as usize],
                );
                i += 1;
                i;
            }
            printf(b"} \0" as *const u8 as *const libc::c_char);
            printf(b"outputs {\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < (*info).outputs {
                printf(
                    b"%s{%s}\0" as *const u8 as *const libc::c_char,
                    if i != 0 {
                        b" \0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*info).output_names[i as usize],
                );
                i += 1;
                i;
            }
            printf(b"}\0" as *const u8 as *const libc::c_char);
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            info = info.offset(1);
            if ((*info).name).is_null() {
                break;
            }
        }
        return 0 as libc::c_int;
    }
    info = ti_find_indicator(*argv.offset(1 as libc::c_int as isize));
    if info.is_null() {
        printf(
            b"Couldn't find indicator: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    let mut inputs: [*const libc::c_double; 5] = [
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
    ];
    let mut outputs: [*mut libc::c_double; 5] = [
        (out[0 as libc::c_int as usize]).as_mut_ptr(),
        (out[1 as libc::c_int as usize]).as_mut_ptr(),
        (out[2 as libc::c_int as usize]).as_mut_ptr(),
        (out[3 as libc::c_int as usize]).as_mut_ptr(),
        (out[4 as libc::c_int as usize]).as_mut_ptr(),
    ];
    let mut o: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut r2: libc::c_int = 0 as libc::c_int;
    let mut v: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*info).inputs {
        if strcmp(
            (*info).input_names[j as usize],
            b"open\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            inputs[j as usize] = open.as_mut_ptr();
            o = 1 as libc::c_int;
        } else if strcmp(
            (*info).input_names[j as usize],
            b"high\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            inputs[j as usize] = high.as_mut_ptr();
            h = 1 as libc::c_int;
        } else if strcmp(
            (*info).input_names[j as usize],
            b"low\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            inputs[j as usize] = low.as_mut_ptr();
            l = 1 as libc::c_int;
        } else if strcmp(
            (*info).input_names[j as usize],
            b"close\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            inputs[j as usize] = close.as_mut_ptr();
            c = 1 as libc::c_int;
        } else if strcmp(
            (*info).input_names[j as usize],
            b"volume\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            inputs[j as usize] = volume.as_mut_ptr();
            v = 1 as libc::c_int;
        } else if strcmp(
            (*info).input_names[j as usize],
            b"real\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if r == 0 {
                inputs[j as usize] = close.as_mut_ptr();
                r = 1 as libc::c_int;
            } else {
                inputs[j as usize] = open.as_mut_ptr();
                r2 = 1 as libc::c_int;
            }
        } else {
            if 0 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"sample.c\0" as *const u8 as *const libc::c_char,
                    144 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[libc::c_char; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            };
        }
        j += 1;
        j;
    }
    let mut alts: [*const libc::c_char; 8] = [
        b"acos\0" as *const u8 as *const libc::c_char,
        b"asin\0" as *const u8 as *const libc::c_char,
        b"atan\0" as *const u8 as *const libc::c_char,
        b"cosh\0" as *const u8 as *const libc::c_char,
        b"sinh\0" as *const u8 as *const libc::c_char,
        b"tanh\0" as *const u8 as *const libc::c_char,
        b"todeg\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut alt: *mut *const libc::c_char = alts.as_mut_ptr();
    while !(*alt).is_null() {
        if strcmp(*alt, (*info).name) == 0 as libc::c_int {
            r = 0 as libc::c_int;
            a = 1 as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*info).inputs {
                inputs[j as usize] = alternative.as_mut_ptr();
                j += 1;
                j;
            }
            break;
        } else {
            alt = alt.offset(1);
            alt;
        }
    }
    let mut options: [libc::c_double; 10] = [0.; 10];
    let mut i_0: libc::c_int = 0;
    i_0 = 0 as libc::c_int;
    while i_0 < (*info).options {
        if argc < 3 as libc::c_int + i_0 {
            printf(
                b"*ERROR NOT ENOUGH OPTIONS*\n\0" as *const u8 as *const libc::c_char,
            );
            let mut k: libc::c_int = 1 as libc::c_int;
            while k < argc {
                printf(
                    b"%s \0" as *const u8 as *const libc::c_char,
                    *argv.offset(k as isize),
                );
                k += 1;
                k;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
        options[i_0 as usize] = atof(*argv.offset((2 as libc::c_int + i_0) as isize));
        i_0 += 1;
        i_0;
    }
    let mut start: libc::c_int = ((*info).start)
        .expect("non-null function pointer")(options.as_mut_ptr());
    let ret: libc::c_int = ((*info).indicator)
        .expect(
            "non-null function pointer",
        )(
        15 as libc::c_int,
        inputs.as_mut_ptr(),
        options.as_mut_ptr(),
        outputs.as_mut_ptr(),
    );
    if ret == 0 as libc::c_int {
        let mut i_1: libc::c_int = 0;
        let mut k_0: libc::c_int = 0;
        let mut bad: libc::c_int = 0 as libc::c_int;
        printf(b"date        \0" as *const u8 as *const libc::c_char);
        if o != 0 {
            printf(b" open   \0" as *const u8 as *const libc::c_char);
        }
        if h != 0 {
            printf(b" high   \0" as *const u8 as *const libc::c_char);
        }
        if l != 0 {
            printf(b" low    \0" as *const u8 as *const libc::c_char);
        }
        if c != 0 {
            printf(b" close  \0" as *const u8 as *const libc::c_char);
        }
        if v != 0 {
            printf(b" volume \0" as *const u8 as *const libc::c_char);
        }
        if r != 0 {
            printf(b" input  \0" as *const u8 as *const libc::c_char);
        }
        if r2 != 0 {
            printf(b" input2  \0" as *const u8 as *const libc::c_char);
        }
        if a != 0 {
            printf(b" input  \0" as *const u8 as *const libc::c_char);
        }
        i_1 = 0 as libc::c_int;
        while i_1 < (*info).outputs {
            printf(
                b" %s\0" as *const u8 as *const libc::c_char,
                (*info).output_names[i_1 as usize],
            );
            i_1 += 1;
            i_1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i_1 = 0 as libc::c_int;
        while i_1 < 15 as libc::c_int {
            printf(b"%s\0" as *const u8 as *const libc::c_char, datet[i_1 as usize]);
            if o != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    open[i_1 as usize],
                );
            }
            if h != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    high[i_1 as usize],
                );
            }
            if l != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    low[i_1 as usize],
                );
            }
            if c != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    close[i_1 as usize],
                );
            }
            if v != 0 {
                printf(
                    b" %8.0f\0" as *const u8 as *const libc::c_char,
                    volume[i_1 as usize],
                );
            }
            if r != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    close[i_1 as usize],
                );
            }
            if r2 != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    open[i_1 as usize],
                );
            }
            if a != 0 {
                printf(
                    b" %8.2f\0" as *const u8 as *const libc::c_char,
                    alternative[i_1 as usize],
                );
            }
            if i_1 >= start {
                k_0 = 0 as libc::c_int;
                while k_0 < (*info).outputs {
                    if out[k_0 as usize][(i_1 - start) as usize]
                        != out[k_0 as usize][(i_1 - start) as usize]
                    {
                        bad = 1 as libc::c_int;
                    }
                    printf(
                        b" %8.3f\0" as *const u8 as *const libc::c_char,
                        out[k_0 as usize][(i_1 - start) as usize],
                    );
                    k_0 += 1;
                    k_0;
                }
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            i_1 += 1;
            i_1;
        }
        if bad != 0 {
            printf(
                b"\nERROR NaN in outputs (%s).\n\0" as *const u8 as *const libc::c_char,
                (*info).name,
            );
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    } else {
        if ret == 1 as libc::c_int {
            printf(b"*ERROR INVALID OPTION*\n\0" as *const u8 as *const libc::c_char);
            let mut i_2: libc::c_int = 1 as libc::c_int;
            while i_2 < argc {
                printf(
                    b"%s \0" as *const u8 as *const libc::c_char,
                    *argv.offset(i_2 as isize),
                );
                i_2 += 1;
                i_2;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"*ERROR*\n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int;
    };
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
