use ::libc;
extern "C" {
    pub type ti_stream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn ti_find_indicator(name: *const libc::c_char) -> *const ti_indicator_info;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn next_line(mut fp: *mut FILE) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 131072] = [0; 131072];
    while !(fgets(buf.as_mut_ptr(), 131072 as libc::c_int, fp)).is_null() {
        if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue;
        }
        if buf[0 as libc::c_int as usize] as libc::c_int == 10 as libc::c_int
            || buf[0 as libc::c_int as usize] as libc::c_int == 13 as libc::c_int
        {
            continue;
        }
        return buf.as_mut_ptr();
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_array(
    mut fp: *mut FILE,
    mut s: *mut libc::c_double,
) -> libc::c_int {
    let mut line: *mut libc::c_char = next_line(fp);
    if *line.offset(0 as libc::c_int as isize) as libc::c_int != '{' as i32 {
        printf(b"Bad input\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    let mut num: *mut libc::c_char = strtok(
        line.offset(1 as libc::c_int as isize),
        b",}\r\n\0" as *const u8 as *const libc::c_char,
    );
    if num.is_null() {
        return 0 as libc::c_int;
    }
    let mut inp: *mut libc::c_double = s;
    loop {
        *inp = atof(num);
        inp = inp.offset(1);
        inp;
        num = strtok(
            0 as *mut libc::c_char,
            b",}\r\n\0" as *const u8 as *const libc::c_char,
        );
        if num.is_null() {
            break;
        }
    }
    return inp.offset_from(s) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn run(
    mut name: *const libc::c_char,
    mut in_0: *const libc::c_char,
    mut out: *const libc::c_char,
) {
    let mut info: *const ti_indicator_info = ti_find_indicator(name);
    if info.is_null() {
        fprintf(
            stderr,
            b"(ERROR) Couldn't find indicator %s\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
        return;
    }
    let mut fin: *mut FILE = fopen(in_0, b"r\0" as *const u8 as *const libc::c_char);
    if fin.is_null() {
        fprintf(
            stderr,
            b"(ERROR) Couldn't open %s for input.\n\0" as *const u8
                as *const libc::c_char,
            in_0,
        );
        return;
    }
    let mut line: *mut libc::c_char = next_line(fin);
    let mut options: [libc::c_double; 16] = [0.; 16];
    let mut o: *mut libc::c_double = options.as_mut_ptr();
    let mut s: *const libc::c_char = strtok(
        line,
        b" \n\r\0" as *const u8 as *const libc::c_char,
    );
    if !s.is_null() {
        loop {
            *o = atof(s);
            o = o.offset(1);
            o;
            s = strtok(
                0 as *mut libc::c_char,
                b" \n\r\0" as *const u8 as *const libc::c_char,
            );
            if s.is_null() {
                break;
            }
        }
    }
    if o.offset_from(options.as_mut_ptr()) as libc::c_long as libc::c_int
        != (*info).options
    {
        fprintf(
            stderr,
            b"(ERROR) Invalid number of options for %s. Expected %d, got %d\n\0"
                as *const u8 as *const libc::c_char,
            name,
            (*info).options,
            o.offset_from(options.as_mut_ptr()) as libc::c_long as libc::c_int,
        );
        return;
    }
    let mut i: libc::c_int = 0;
    let mut inputs: [*mut libc::c_double; 16] = [
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    ];
    let mut outputs: [*mut libc::c_double; 16] = [
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    ];
    let mut input_size: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*info).inputs {
        inputs[i
            as usize] = malloc(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(8192 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_double;
        input_size = get_array(fin, inputs[i as usize]);
        i += 1;
        i;
    }
    fclose(fin);
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        outputs[i
            as usize] = malloc(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(8192 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_double;
        i += 1;
        i;
    }
    let ret: libc::c_int = ((*info).indicator)
        .expect(
            "non-null function pointer",
        )(
        input_size,
        inputs.as_mut_ptr() as *const *const libc::c_double,
        options.as_mut_ptr(),
        outputs.as_mut_ptr(),
    );
    if ret != 0 as libc::c_int {
        fprintf(
            stderr,
            b"(ERROR) Return value for %s of %d.\n\0" as *const u8
                as *const libc::c_char,
            name,
            ret,
        );
        return;
    }
    let mut fout: *mut FILE = fopen(out, b"w\0" as *const u8 as *const libc::c_char);
    if fout.is_null() {
        fprintf(
            stderr,
            b"(ERROR) Couldn't open %s for output.\n\0" as *const u8
                as *const libc::c_char,
            out,
        );
        return;
    }
    let out_size: libc::c_int = input_size
        - ((*info).start).expect("non-null function pointer")(options.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        fprintf(fout, b"{\0" as *const u8 as *const libc::c_char);
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < out_size {
            fprintf(
                fout,
                b"%f%s\0" as *const u8 as *const libc::c_char,
                *(outputs[i as usize]).offset(j as isize),
                if j == out_size - 1 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b",\0" as *const u8 as *const libc::c_char
                },
            );
            j += 1;
            j;
        }
        fprintf(fout, b"}\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    fclose(fout);
    i = 0 as libc::c_int;
    while i < (*info).inputs {
        free(inputs[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        free(outputs[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc < 4 as libc::c_int {
        printf(
            b"Usage: cli indicator_name infile outfile\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut name: *const libc::c_char = *argv.offset(1 as libc::c_int as isize);
    let mut in_0: *const libc::c_char = *argv.offset(2 as libc::c_int as isize);
    let mut out: *const libc::c_char = *argv.offset(3 as libc::c_int as isize);
    run(name, in_0, out);
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
