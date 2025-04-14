#![feature(extern_types)]
use libc::{FILE, fileno};

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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut libc::FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn BZ2_bzopen(
        path: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn BZ2_bzdopen(fd: libc::c_int, mode: *const libc::c_char) -> *mut libc::c_void;
    fn BZ2_bzread(
        b: *mut libc::c_void,
        buf: *mut libc::c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    fn BZ2_bzwrite(
        b: *mut libc::c_void,
        buf: *mut libc::c_void,
        len: libc::c_int,
    ) -> libc::c_int;
    fn BZ2_bzclose(b: *mut libc::c_void);
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
// pub type FILE = _IO_FILE;
pub type BZFILE = ();
#[no_mangle]
pub unsafe extern "C" fn usage() {
    puts(
        b"usage: minibz2 [-d] [-{1,2,..9}] [[srcfilename] destfilename]\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut decompress: libc::c_int = 0 as libc::c_int;
    let mut level: libc::c_int = 9 as libc::c_int;
    let mut fn_r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fn_w: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        argv = argv.offset(1);
        argv;
        argc -= 1;
        if !(argc != 0) {
            break;
        }
        if !(**argv as libc::c_int == '-' as i32 || **argv as libc::c_int == '/' as i32)
        {
            break;
        }
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = (*argv).offset(1 as libc::c_int as isize);
        while *p != 0 {
            if *p as libc::c_int == 'd' as i32 {
                decompress = 1 as libc::c_int;
            } else if '1' as i32 <= *p as libc::c_int && *p as libc::c_int <= '9' as i32
            {
                level = *p as libc::c_int - '0' as i32;
            } else {
                usage();
                exit(1 as libc::c_int);
            }
            p = p.offset(1);
            p;
        }
    }
    if argc >= 1 as libc::c_int {
        fn_r = *argv;
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    } else {
        fn_r = 0 as *mut libc::c_char;
    }
    if argc >= 1 as libc::c_int {
        fn_w = *argv;
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    } else {
        fn_w = 0 as *mut libc::c_char;
    }
    let mut len: libc::c_int = 0;
    let mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut mode: [libc::c_char; 10] = [0; 10];
    if decompress != 0 {
        let mut BZ2fp_r: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut fp_w: *mut FILE = 0 as *mut FILE;
        if !fn_w.is_null() {
            fp_w = fopen(fn_w, b"wb\0" as *const u8 as *const libc::c_char);
            if fp_w.is_null() {
                printf(b"can't open [%s]\n\0" as *const u8 as *const libc::c_char, fn_w);
                perror(b"reason:\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        } else {
            fp_w = stdout;
        }
        if fn_r.is_null()
            && {
                BZ2fp_r = BZ2_bzdopen(
                    fileno(stdin),
                    b"rb\0" as *const u8 as *const libc::c_char,
                );
                BZ2fp_r.is_null()
            }
            || !fn_r.is_null()
                && {
                    BZ2fp_r = BZ2_bzopen(
                        fn_r,
                        b"rb\0" as *const u8 as *const libc::c_char,
                    );
                    BZ2fp_r.is_null()
                }
        {
            printf(b"can't bz2openstream\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        loop {
            len = BZ2_bzread(
                BZ2fp_r,
                buff.as_mut_ptr() as *mut libc::c_void,
                0x1000 as libc::c_int,
            );
            if !(len > 0 as libc::c_int) {
                break;
            }
            fwrite(
                buff.as_mut_ptr() as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                len as libc::c_ulong,
                fp_w,
            );
        }
        BZ2_bzclose(BZ2fp_r);
        if fp_w != stdout {
            fclose(fp_w);
        }
    } else {
        let mut BZ2fp_w: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut fp_r: *mut FILE = 0 as *mut FILE;
        if !fn_r.is_null() {
            fp_r = fopen(fn_r, b"rb\0" as *const u8 as *const libc::c_char);
            if fp_r.is_null() {
                printf(b"can't open [%s]\n\0" as *const u8 as *const libc::c_char, fn_r);
                perror(b"reason:\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        } else {
            fp_r = stdin;
        }
        mode[0 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
        mode[1 as libc::c_int as usize] = ('0' as i32 + level) as libc::c_char;
        mode[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        if fn_w.is_null()
            && {
                BZ2fp_w = BZ2_bzdopen(fileno(stdout), mode.as_mut_ptr());
                BZ2fp_w.is_null()
            }
            || !fn_w.is_null()
                && {
                    BZ2fp_w = BZ2_bzopen(fn_w, mode.as_mut_ptr());
                    BZ2fp_w.is_null()
                }
        {
            printf(b"can't bz2openstream\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        loop {
            len = fread(
                buff.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                0x1000 as libc::c_int as libc::c_ulong,
                fp_r,
            ) as libc::c_int;
            if !(len > 0 as libc::c_int) {
                break;
            }
            BZ2_bzwrite(BZ2fp_w, buff.as_mut_ptr() as *mut libc::c_void, len);
        }
        BZ2_bzclose(BZ2fp_w);
        if fp_r != stdin {
            fclose(fp_r);
        }
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
