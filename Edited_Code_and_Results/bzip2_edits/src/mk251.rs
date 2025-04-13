#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn putchar(__c: libc::c_int) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 48500000 as libc::c_int {
        putchar(251 as libc::c_int);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
