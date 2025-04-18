#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: libc::c_int,
    pub hidden_layers: libc::c_int,
    pub hidden: libc::c_int,
    pub outputs: libc::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: libc::c_int,
    pub total_neurons: libc::c_int,
    pub weight: *mut libc::c_double,
    pub output: *mut libc::c_double,
    pub delta: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid(mut a: libc::c_double) -> libc::c_double {
    if a < -45.0f64 {
        return 0 as libc::c_int as libc::c_double;
    }
    if a > 45.0f64 {
        return 1 as libc::c_int as libc::c_double;
    }
    return 1.0f64 / (1 as libc::c_int as libc::c_double + exp(-a));
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_sigmoid_cached(
    mut a: libc::c_double,
) -> libc::c_double {
    let min: libc::c_double = -15.0f64;
    let max: libc::c_double = 15.0f64;
    static mut interval: libc::c_double = 0.;
    static mut initialized: libc::c_int = 0 as libc::c_int;
    static mut lookup: [libc::c_double; 4096] = [0.; 4096];
    if initialized == 0 {
        interval = (max - min) / 4096 as libc::c_int as libc::c_double;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            lookup[i
                as usize] = genann_act_sigmoid(min + interval * i as libc::c_double);
            i += 1;
            i;
        }
        initialized = 1 as libc::c_int;
    }
    let mut i_0: libc::c_int = 0;
    i_0 = ((a - min) / interval + 0.5f64) as libc::c_int;
    if i_0 <= 0 as libc::c_int {
        return lookup[0 as libc::c_int as usize];
    }
    if i_0 >= 4096 as libc::c_int {
        return lookup[(4096 as libc::c_int - 1 as libc::c_int) as usize];
    }
    return lookup[i_0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_threshold(mut a: libc::c_double) -> libc::c_double {
    return (a > 0 as libc::c_int as libc::c_double) as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn genann_act_linear(mut a: libc::c_double) -> libc::c_double {
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn genann_init(
    mut inputs: libc::c_int,
    mut hidden_layers: libc::c_int,
    mut hidden: libc::c_int,
    mut outputs: libc::c_int,
) -> *mut genann {
    if hidden_layers < 0 as libc::c_int {
        return 0 as *mut genann;
    }
    if inputs < 1 as libc::c_int {
        return 0 as *mut genann;
    }
    if outputs < 1 as libc::c_int {
        return 0 as *mut genann;
    }
    if hidden_layers > 0 as libc::c_int && hidden < 1 as libc::c_int {
        return 0 as *mut genann;
    }
    let hidden_weights: libc::c_int = if hidden_layers != 0 {
        (inputs + 1 as libc::c_int) * hidden
            + (hidden_layers - 1 as libc::c_int) * (hidden + 1 as libc::c_int) * hidden
    } else {
        0 as libc::c_int
    };
    let output_weights: libc::c_int = (if hidden_layers != 0 {
        hidden + 1 as libc::c_int
    } else {
        inputs + 1 as libc::c_int
    }) * outputs;
    let total_weights: libc::c_int = hidden_weights + output_weights;
    let total_neurons: libc::c_int = inputs + hidden * hidden_layers + outputs;
    let size: libc::c_int = (::core::mem::size_of::<genann>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(
                    (total_weights + total_neurons + (total_neurons - inputs))
                        as libc::c_ulong,
                ),
        ) as libc::c_int;
    let mut ret: *mut genann = malloc(size as libc::c_ulong) as *mut genann;
    if ret.is_null() {
        return 0 as *mut genann;
    }
    (*ret).inputs = inputs;
    (*ret).hidden_layers = hidden_layers;
    (*ret).hidden = hidden;
    (*ret).outputs = outputs;
    (*ret).total_weights = total_weights;
    (*ret).total_neurons = total_neurons;
    (*ret)
        .weight = (ret as *mut libc::c_char)
        .offset(::core::mem::size_of::<genann>() as libc::c_ulong as isize)
        as *mut libc::c_double;
    (*ret).output = ((*ret).weight).offset((*ret).total_weights as isize);
    (*ret).delta = ((*ret).output).offset((*ret).total_neurons as isize);
    genann_randomize(ret);
    (*ret)
        .activation_hidden = Some(
        genann_act_sigmoid_cached
            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    (*ret)
        .activation_output = Some(
        genann_act_sigmoid_cached
            as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_read(mut in_0: *mut FILE) -> *mut genann {
    let mut inputs: libc::c_int = 0;
    let mut hidden_layers: libc::c_int = 0;
    let mut hidden: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    rc = fscanf(
        in_0,
        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
        &mut inputs as *mut libc::c_int,
        &mut hidden_layers as *mut libc::c_int,
        &mut hidden as *mut libc::c_int,
        &mut outputs as *mut libc::c_int,
    );
    if rc < 4 as libc::c_int || *__errno_location() != 0 as libc::c_int {
        perror(b"fscanf\0" as *const u8 as *const libc::c_char);
        return 0 as *mut genann;
    }
    let mut ann: *mut genann = genann_init(inputs, hidden_layers, hidden, outputs);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ann).total_weights {
        *__errno_location() = 0 as libc::c_int;
        rc = fscanf(
            in_0,
            b" %le\0" as *const u8 as *const libc::c_char,
            ((*ann).weight).offset(i as isize),
        );
        if rc < 1 as libc::c_int || *__errno_location() != 0 as libc::c_int {
            perror(b"fscanf\0" as *const u8 as *const libc::c_char);
            genann_free(ann);
            return 0 as *mut genann;
        }
        i += 1;
        i;
    }
    return ann;
}
#[no_mangle]
pub unsafe extern "C" fn genann_copy(mut ann: *const genann) -> *mut genann {
    let size: libc::c_int = (::core::mem::size_of::<genann>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(
                    ((*ann).total_weights + (*ann).total_neurons
                        + ((*ann).total_neurons - (*ann).inputs)) as libc::c_ulong,
                ),
        ) as libc::c_int;
    let mut ret: *mut genann = malloc(size as libc::c_ulong) as *mut genann;
    if ret.is_null() {
        return 0 as *mut genann;
    }
    memcpy(ret as *mut libc::c_void, ann as *const libc::c_void, size as libc::c_ulong);
    (*ret)
        .weight = (ret as *mut libc::c_char)
        .offset(::core::mem::size_of::<genann>() as libc::c_ulong as isize)
        as *mut libc::c_double;
    (*ret).output = ((*ret).weight).offset((*ret).total_weights as isize);
    (*ret).delta = ((*ret).output).offset((*ret).total_neurons as isize);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_randomize(mut ann: *mut genann) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ann).total_weights {
        let mut r: libc::c_double = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double;
        *((*ann).weight).offset(i as isize) = r - 0.5f64;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genann_free(mut ann: *mut genann) {
    free(ann as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn genann_run(
    mut ann: *const genann,
    mut inputs: *const libc::c_double,
) -> *const libc::c_double {
    let mut w: *const libc::c_double = (*ann).weight;
    let mut o: *mut libc::c_double = ((*ann).output).offset((*ann).inputs as isize);
    let mut i: *const libc::c_double = (*ann).output;
    memcpy(
        (*ann).output as *mut libc::c_void,
        inputs as *const libc::c_void,
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*ann).inputs as libc::c_ulong),
    );
    let mut h: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let act: genann_actfun = (*ann).activation_hidden;
    let acto: genann_actfun = (*ann).activation_output;
    h = 0 as libc::c_int;
    while h < (*ann).hidden_layers {
        j = 0 as libc::c_int;
        while j < (*ann).hidden {
            let fresh0 = w;
            w = w.offset(1);
            let mut sum: libc::c_double = *fresh0 * -1.0f64;
            k = 0 as libc::c_int;
            while k < (if h == 0 as libc::c_int { (*ann).inputs } else { (*ann).hidden })
            {
                let fresh1 = w;
                w = w.offset(1);
                sum += *fresh1 * *i.offset(k as isize);
                k += 1;
                k;
            }
            let fresh2 = o;
            o = o.offset(1);
            *fresh2 = act.expect("non-null function pointer")(sum);
            j += 1;
            j;
        }
        i = i
            .offset(
                (if h == 0 as libc::c_int { (*ann).inputs } else { (*ann).hidden })
                    as isize,
            );
        h += 1;
        h;
    }
    let mut ret: *const libc::c_double = o;
    j = 0 as libc::c_int;
    while j < (*ann).outputs {
        let fresh3 = w;
        w = w.offset(1);
        let mut sum_0: libc::c_double = *fresh3 * -1.0f64;
        k = 0 as libc::c_int;
        while k < (if (*ann).hidden_layers != 0 { (*ann).hidden } else { (*ann).inputs })
        {
            let fresh4 = w;
            w = w.offset(1);
            sum_0 += *fresh4 * *i.offset(k as isize);
            k += 1;
            k;
        }
        let fresh5 = o;
        o = o.offset(1);
        *fresh5 = acto.expect("non-null function pointer")(sum_0);
        j += 1;
        j;
    }
    if w.offset_from((*ann).weight) as libc::c_long
        == (*ann).total_weights as libc::c_long
    {} else {
        __assert_fail(
            b"w - ann->weight == ann->total_weights\0" as *const u8
                as *const libc::c_char,
            b"genann.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"const double *genann_run(const genann *, const double *)\0"))
                .as_ptr(),
        );
    }
    'c_1500: {
        if w.offset_from((*ann).weight) as libc::c_long
            == (*ann).total_weights as libc::c_long
        {} else {
            __assert_fail(
                b"w - ann->weight == ann->total_weights\0" as *const u8
                    as *const libc::c_char,
                b"genann.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"const double *genann_run(const genann *, const double *)\0"))
                    .as_ptr(),
            );
        }
    };
    if o.offset_from((*ann).output) as libc::c_long
        == (*ann).total_neurons as libc::c_long
    {} else {
        __assert_fail(
            b"o - ann->output == ann->total_neurons\0" as *const u8
                as *const libc::c_char,
            b"genann.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"const double *genann_run(const genann *, const double *)\0"))
                .as_ptr(),
        );
    }
    'c_1435: {
        if o.offset_from((*ann).output) as libc::c_long
            == (*ann).total_neurons as libc::c_long
        {} else {
            __assert_fail(
                b"o - ann->output == ann->total_neurons\0" as *const u8
                    as *const libc::c_char,
                b"genann.c\0" as *const u8 as *const libc::c_char,
                226 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"const double *genann_run(const genann *, const double *)\0"))
                    .as_ptr(),
            );
        }
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn genann_train(
    mut ann: *const genann,
    mut inputs: *const libc::c_double,
    mut desired_outputs: *const libc::c_double,
    mut learning_rate: libc::c_double,
) {
    genann_run(ann, inputs);
    let mut h: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut o: *const libc::c_double = ((*ann).output)
        .offset((*ann).inputs as isize)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut d: *mut libc::c_double = ((*ann).delta)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut t: *const libc::c_double = desired_outputs;
    if (*ann).activation_output
        == Some(
            genann_act_linear as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
        )
    {
        j = 0 as libc::c_int;
        while j < (*ann).outputs {
            let fresh6 = t;
            t = t.offset(1);
            let fresh7 = o;
            o = o.offset(1);
            let fresh8 = d;
            d = d.offset(1);
            *fresh8 = *fresh6 - *fresh7;
            j += 1;
            j;
        }
    } else {
        j = 0 as libc::c_int;
        while j < (*ann).outputs {
            let fresh9 = d;
            d = d.offset(1);
            *fresh9 = (*t - *o) * *o * (1.0f64 - *o);
            o = o.offset(1);
            o;
            t = t.offset(1);
            t;
            j += 1;
            j;
        }
    }
    h = (*ann).hidden_layers - 1 as libc::c_int;
    while h >= 0 as libc::c_int {
        let mut o_0: *const libc::c_double = ((*ann).output)
            .offset((*ann).inputs as isize)
            .offset((h * (*ann).hidden) as isize);
        let mut d_0: *mut libc::c_double = ((*ann).delta)
            .offset((h * (*ann).hidden) as isize);
        let dd: *const libc::c_double = ((*ann).delta)
            .offset(((h + 1 as libc::c_int) * (*ann).hidden) as isize);
        let ww: *const libc::c_double = ((*ann).weight)
            .offset((((*ann).inputs + 1 as libc::c_int) * (*ann).hidden) as isize)
            .offset((((*ann).hidden + 1 as libc::c_int) * (*ann).hidden * h) as isize);
        j = 0 as libc::c_int;
        while j < (*ann).hidden {
            let mut delta: libc::c_double = 0 as libc::c_int as libc::c_double;
            k = 0 as libc::c_int;
            while k
                < (if h == (*ann).hidden_layers - 1 as libc::c_int {
                    (*ann).outputs
                } else {
                    (*ann).hidden
                })
            {
                let forward_delta: libc::c_double = *dd.offset(k as isize);
                let windex: libc::c_int = k * ((*ann).hidden + 1 as libc::c_int)
                    + (j + 1 as libc::c_int);
                let forward_weight: libc::c_double = *ww.offset(windex as isize);
                delta += forward_delta * forward_weight;
                k += 1;
                k;
            }
            *d_0 = *o_0 * (1.0f64 - *o_0) * delta;
            d_0 = d_0.offset(1);
            d_0;
            o_0 = o_0.offset(1);
            o_0;
            j += 1;
            j;
        }
        h -= 1;
        h;
    }
    let mut d_1: *const libc::c_double = ((*ann).delta)
        .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
    let mut w: *mut libc::c_double = ((*ann).weight)
        .offset(
            (if (*ann).hidden_layers != 0 {
                ((*ann).inputs + 1 as libc::c_int) * (*ann).hidden
                    + ((*ann).hidden + 1 as libc::c_int) * (*ann).hidden
                        * ((*ann).hidden_layers - 1 as libc::c_int)
            } else {
                0 as libc::c_int
            }) as isize,
        );
    let i: *const libc::c_double = ((*ann).output)
        .offset(
            (if (*ann).hidden_layers != 0 {
                (*ann).inputs + (*ann).hidden * ((*ann).hidden_layers - 1 as libc::c_int)
            } else {
                0 as libc::c_int
            }) as isize,
        );
    j = 0 as libc::c_int;
    while j < (*ann).outputs {
        k = 0 as libc::c_int;
        while k
            < (if (*ann).hidden_layers != 0 { (*ann).hidden } else { (*ann).inputs })
                + 1 as libc::c_int
        {
            if k == 0 as libc::c_int {
                let fresh10 = w;
                w = w.offset(1);
                *fresh10 += *d_1 * learning_rate * -1.0f64;
            } else {
                let fresh11 = w;
                w = w.offset(1);
                *fresh11
                    += *d_1 * learning_rate * *i.offset((k - 1 as libc::c_int) as isize);
            }
            k += 1;
            k;
        }
        d_1 = d_1.offset(1);
        d_1;
        j += 1;
        j;
    }
    if w.offset_from((*ann).weight) as libc::c_long
        == (*ann).total_weights as libc::c_long
    {} else {
        __assert_fail(
            b"w - ann->weight == ann->total_weights\0" as *const u8
                as *const libc::c_char,
            b"genann.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"void genann_train(const genann *, const double *, const double *, double)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1988: {
        if w.offset_from((*ann).weight) as libc::c_long
            == (*ann).total_weights as libc::c_long
        {} else {
            __assert_fail(
                b"w - ann->weight == ann->total_weights\0" as *const u8
                    as *const libc::c_char,
                b"genann.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[libc::c_char; 74],
                >(
                    b"void genann_train(const genann *, const double *, const double *, double)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    h = (*ann).hidden_layers - 1 as libc::c_int;
    while h >= 0 as libc::c_int {
        let mut d_2: *const libc::c_double = ((*ann).delta)
            .offset((h * (*ann).hidden) as isize);
        let mut i_0: *const libc::c_double = ((*ann).output)
            .offset(
                (if h != 0 {
                    (*ann).inputs + (*ann).hidden * (h - 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        let mut w_0: *mut libc::c_double = ((*ann).weight)
            .offset(
                (if h != 0 {
                    ((*ann).inputs + 1 as libc::c_int) * (*ann).hidden
                        + ((*ann).hidden + 1 as libc::c_int) * (*ann).hidden
                            * (h - 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                }) as isize,
            );
        j = 0 as libc::c_int;
        while j < (*ann).hidden {
            k = 0 as libc::c_int;
            while k
                < (if h == 0 as libc::c_int { (*ann).inputs } else { (*ann).hidden })
                    + 1 as libc::c_int
            {
                if k == 0 as libc::c_int {
                    let fresh12 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh12 += *d_2 * learning_rate * -1.0f64;
                } else {
                    let fresh13 = w_0;
                    w_0 = w_0.offset(1);
                    *fresh13
                        += *d_2 * learning_rate
                            * *i_0.offset((k - 1 as libc::c_int) as isize);
                }
                k += 1;
                k;
            }
            d_2 = d_2.offset(1);
            d_2;
            j += 1;
            j;
        }
        h -= 1;
        h;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genann_write(mut ann: *const genann, mut out: *mut FILE) {
    fprintf(
        out,
        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
        (*ann).inputs,
        (*ann).hidden_layers,
        (*ann).hidden,
        (*ann).outputs,
    );
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ann).total_weights {
        fprintf(
            out,
            b" %.20e\0" as *const u8 as *const libc::c_char,
            *((*ann).weight).offset(i as isize),
        );
        i += 1;
        i;
    }
}
