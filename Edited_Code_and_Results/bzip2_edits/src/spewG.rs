#![feature(extern_types)]
use libc::{fileno, fdopen};

#[allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]


extern "C" {
    fn srandom(seed: libc::c_uint);
    fn random() -> libc::c_long;
}

extern "C" {
    fn setbuffer(stream: *mut libc::FILE, buf: *mut libc::c_char, size: libc::c_int);
    
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut libc::FILE;
    fn fflush(__stream: *mut libc::FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _IO_FILE {
//     pub _flags: libc::c_int,
//     pub _IO_read_ptr: *mut libc::c_char,
//     pub _IO_read_end: *mut libc::c_char,
//     pub _IO_read_base: *mut libc::c_char,
//     pub _IO_write_base: *mut libc::c_char,
//     pub _IO_write_ptr: *mut libc::c_char,
//     pub _IO_write_end: *mut libc::c_char,
//     pub _IO_buf_base: *mut libc::c_char,
//     pub _IO_buf_end: *mut libc::c_char,
//     pub _IO_save_base: *mut libc::c_char,
//     pub _IO_backup_base: *mut libc::c_char,
//     pub _IO_save_end: *mut libc::c_char,
//     pub _markers: *mut _IO_marker,
//     pub _chain: *mut _IO_FILE,
//     pub _fileno: libc::c_int,
//     pub _flags2: libc::c_int,
//     pub _old_offset: __off_t,
//     pub _cur_column: libc::c_ushort,
//     pub _vtable_offset: libc::c_schar,
//     pub _shortbuf: [libc::c_char; 1],
//     pub _lock: *mut libc::c_void,
//     pub _offset: __off64_t,
//     pub _codecvt: *mut _IO_codecvt,
//     pub _wide_data: *mut _IO_wide_data,
//     pub _freeres_list: *mut _IO_FILE,
//     pub _freeres_buf: *mut libc::c_void,
//     pub __pad5: size_t,
//     pub _mode: libc::c_int,
//     pub _unused2: [libc::c_char; 20],
// }
pub type _IO_lock_t = ();
// pub type FILE = _IO_FILE;
#[no_mangle]
pub static mut buf: [libc::c_char; 1000000] = [0; 1000000];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ii: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    unsafe {
        srandom(1 as libc::c_uint);
    }
    unsafe {
        setbuffer(stdout, buf.as_mut_ptr(), 1000000 as libc::c_int);
    }
    kk = 0 as libc::c_int;
    while kk < 5000 as libc::c_int * 515 as libc::c_int {
        p = 25 + (random() as libc::c_int % 50);
        ii = 0 as libc::c_int;
        while ii < p {
            printf(
                b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\0" as *const u8
                    as *const libc::c_char,
            );
            ii += 1;
            ii;
        }
        ii = 0 as libc::c_int;
        while ii < p - 1 as libc::c_int {
            printf(
                b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\0" as *const u8
                    as *const libc::c_char,
            );
            ii += 1;
            ii;
        }
        ii = 0 as libc::c_int;
        while ii < p + 1 as libc::c_int {
            printf(
                b"ccccccccccccccccccccccccccccccccccccc\0" as *const u8
                    as *const libc::c_char,
            );
            ii += 1;
            ii;
        }
        kk += 3 as libc::c_int;
    }
    fflush(stdout);
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
