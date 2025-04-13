use ::libc;
extern "C" {
    pub type ti_stream;
    static mut ti_indicators: [ti_indicator_info; 0];
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
unsafe fn main_0() -> libc::c_int {
    printf(
        b"This program is an example of looping through\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"each of the available indicators.\n\n\0" as *const u8 as *const libc::c_char,
    );
    let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
    while !((*info).name).is_null() {
        let mut i: libc::c_int = 0;
        printf(
            b"%s (%s) has type %d with: %d inputs, %d options, %d outputs.\n\0"
                as *const u8 as *const libc::c_char,
            (*info).name,
            (*info).full_name,
            (*info).type_0,
            (*info).inputs,
            (*info).options,
            (*info).outputs,
        );
        printf(b"   inputs: \0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*info).inputs {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if i != 0 {
                    b", \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*info).input_names[i as usize],
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"   options: \0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*info).options {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if i != 0 {
                    b", \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*info).option_names[i as usize],
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"   outputs: \0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*info).outputs {
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if i != 0 {
                    b", \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*info).output_names[i as usize],
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        info = info.offset(1);
        info;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
