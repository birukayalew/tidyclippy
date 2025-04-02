use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn ti_sma_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_sma(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_array(mut p: *const libc::c_double, size: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if i != 0 {
            printf(b", \0" as *const u8 as *const libc::c_char);
        }
        printf(b"%.1f\0" as *const u8 as *const libc::c_char, *p.offset(i as isize));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let data_in: [libc::c_double; 10] = [
        5 as libc::c_int as libc::c_double,
        8 as libc::c_int as libc::c_double,
        12 as libc::c_int as libc::c_double,
        11 as libc::c_int as libc::c_double,
        9 as libc::c_int as libc::c_double,
        8 as libc::c_int as libc::c_double,
        7 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
        11 as libc::c_int as libc::c_double,
        13 as libc::c_int as libc::c_double,
    ];
    let input_length: libc::c_int = (::core::mem::size_of::<[libc::c_double; 10]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as libc::c_int;
    printf(
        b"We have %d bars of input data.\n\0" as *const u8 as *const libc::c_char,
        input_length,
    );
    print_array(data_in.as_ptr(), input_length);
    let options: [libc::c_double; 1] = [3 as libc::c_int as libc::c_double];
    printf(b"Our option array is: \0" as *const u8 as *const libc::c_char);
    print_array(
        options.as_ptr(),
        (::core::mem::size_of::<[libc::c_double; 1]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int,
    );
    let start: libc::c_int = ti_sma_start(options.as_ptr());
    printf(b"The start amount is: %d\n\0" as *const u8 as *const libc::c_char, start);
    let output_length: libc::c_int = input_length - start;
    let mut data_out: *mut libc::c_double = malloc(
        (output_length as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if !data_out.is_null() {} else {
        __assert_fail(
            b"data_out != 0\0" as *const u8 as *const libc::c_char,
            b"example1.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    };
    printf(
        b"The output length is: %d\n\0" as *const u8 as *const libc::c_char,
        output_length,
    );
    let mut all_inputs: [*const libc::c_double; 1] = [data_in.as_ptr()];
    let mut all_outputs: [*mut libc::c_double; 1] = [data_out];
    let mut error: libc::c_int = ti_sma(
        input_length,
        all_inputs.as_mut_ptr(),
        options.as_ptr(),
        all_outputs.as_mut_ptr(),
    );
    if error == 0 as libc::c_int {} else {
        __assert_fail(
            b"error == TI_OKAY\0" as *const u8 as *const libc::c_char,
            b"example1.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"int main()\0"))
                .as_ptr(),
        );
    };
    printf(b"The output data is: \0" as *const u8 as *const libc::c_char);
    print_array(data_out, output_length);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
