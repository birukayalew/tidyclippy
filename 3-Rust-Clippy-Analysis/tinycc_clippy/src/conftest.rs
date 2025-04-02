use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[no_mangle]
pub static mut platform_macros: [*const libc::c_char; 33] = [
    b"__i386__\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_I386\0" as *const u8 as *const libc::c_char,
    b"__x86_64__\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_X86_64\0" as *const u8 as *const libc::c_char,
    b"_WIN32\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_PE\0" as *const u8 as *const libc::c_char,
    b"__arm__\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_ARM\0" as *const u8 as *const libc::c_char,
    b"__ARM_EABI__\0" as *const u8 as *const libc::c_char,
    b"TCC_ARM_EABI\0" as *const u8 as *const libc::c_char,
    b"__aarch64__\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_ARM64\0" as *const u8 as *const libc::c_char,
    b"__riscv\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_RISCV64\0" as *const u8 as *const libc::c_char,
    b"__APPLE__\0" as *const u8 as *const libc::c_char,
    b"TCC_TARGET_MACHO\0" as *const u8 as *const libc::c_char,
    b"__FreeBSD__\0" as *const u8 as *const libc::c_char,
    b"TARGETOS_FreeBSD\0" as *const u8 as *const libc::c_char,
    b"__FreeBSD_kernel__\0" as *const u8 as *const libc::c_char,
    b"TARGETOS_FreeBSD_kernel\0" as *const u8 as *const libc::c_char,
    b"__OpenBSD__\0" as *const u8 as *const libc::c_char,
    b"TARGETOS_OpenBSD\0" as *const u8 as *const libc::c_char,
    b"__NetBSD__\0" as *const u8 as *const libc::c_char,
    b"TARGETOS_NetBSD\0" as *const u8 as *const libc::c_char,
    b"__linux__\0" as *const u8 as *const libc::c_char,
    b"TARGETOS_Linux\0" as *const u8 as *const libc::c_char,
    b"__ANDROID__\0" as *const u8 as *const libc::c_char,
    b"TARGETOS_ANDROID\0" as *const u8 as *const libc::c_char,
    b"__SIZEOF_POINTER__\0" as *const u8 as *const libc::c_char,
    b"PTR_SIZE\0" as *const u8 as *const libc::c_char,
    b"__SIZEOF_LONG__\0" as *const u8 as *const libc::c_char,
    b"LONG_SIZE\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn isid(mut c: libc::c_int) -> libc::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= '0' as i32 && c <= '9' as i32 || c == '_' as i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isspc(mut c: libc::c_int) -> libc::c_int {
    return (c as libc::c_uchar as libc::c_int <= ' ' as i32 && c != 0 as libc::c_int)
        as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut l: [libc::c_char; 1000] = [0; 1000];
    let mut l2: [libc::c_char; 1000] = [0; 1000];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut op: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut cmt: libc::c_int = 0;
    let mut cmt_n: libc::c_int = 0;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    if argc < 3 as libc::c_int {
        return 1 as libc::c_int;
    }
    fp = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    op = fopen(
        *argv.offset(2 as libc::c_int as isize),
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() || op.is_null() {
        fprintf(stderr, b"c2str: file error\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    cmt_n = 0 as libc::c_int;
    cmt = cmt_n;
    's_43: loop {
        p = l.as_mut_ptr();
        loop {
            if !(fgets(
                p,
                (::core::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong)
                    .wrapping_sub(
                        p.offset_from(l.as_mut_ptr()) as libc::c_long as libc::c_ulong,
                    ) as libc::c_int,
                fp,
            ))
                .is_null()
            {
                p = strchr(p, 0 as libc::c_int);
                while p > l.as_mut_ptr()
                    && isspc(*p.offset(-(1 as libc::c_int) as isize) as libc::c_int) != 0
                {
                    p = p.offset(-1);
                    p;
                }
                *p = 0 as libc::c_int as libc::c_char;
            } else if p == l.as_mut_ptr() {
                break 's_43;
            }
            if !(p > l.as_mut_ptr()
                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32)
            {
                break;
            }
            *p.offset(-(1 as libc::c_int) as isize) = ' ' as i32 as libc::c_char;
        }
        p = l.as_mut_ptr();
        q = l2.as_mut_ptr();
        f = 0 as libc::c_int;
        while *p as libc::c_int != 0 && isspc(*p as libc::c_int) != 0 {
            p = p.offset(1);
            p;
            f += 1;
            f;
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            && cmt == 0 as libc::c_int
        {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32 {
                cmt = 2 as libc::c_int;
            }
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                cmt = 1 as libc::c_int;
            }
        }
        if cmt != 0 {
            fprintf(op, b"%s\0" as *const u8 as *const libc::c_char, l.as_mut_ptr());
            cmt_n += 1;
            if cmt_n == 1 as libc::c_int {
                fprintf(
                    op,
                    b" (converted, do not edit this file)\0" as *const u8
                        as *const libc::c_char,
                );
            }
            fprintf(op, b"\n\0" as *const u8 as *const libc::c_char);
            if cmt == 1 as libc::c_int {
                cmt = 0 as libc::c_int;
            }
            if cmt == 2 as libc::c_int {
                p = strchr(l.as_mut_ptr(), 0 as libc::c_int);
                if p >= l.as_mut_ptr().offset(2 as libc::c_int as isize)
                    && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '/' as i32
                    && *p.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == '*' as i32
                {
                    cmt = 0 as libc::c_int;
                }
            }
        } else if f < 4 as libc::c_int {
            loop {
                f = 0 as libc::c_int;
                e = f;
                loop {
                    r = platform_macros[f as usize];
                    if r.is_null() {
                        break;
                    }
                    c = strlen(r) as libc::c_int;
                    if 0 as libc::c_int
                        == memcmp(
                            p.offset(e as isize) as *const libc::c_void,
                            r as *const libc::c_void,
                            c as libc::c_ulong,
                        )
                    {
                        p = p.offset((e + c) as isize);
                        q = strchr(
                            strcpy(q, platform_macros[(f + 1 as libc::c_int) as usize]),
                            0 as libc::c_int,
                        );
                        break;
                    } else {
                        f += 2 as libc::c_int;
                    }
                }
                !r.is_null();
                let fresh0 = p;
                p = p.offset(1);
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = *fresh0;
                if !(*fresh1 != 0) {
                    break;
                }
            }
            fprintf(op, b"%s\n\0" as *const u8 as *const libc::c_char, l2.as_mut_ptr());
        } else {
            f = 0 as libc::c_int;
            e = f;
            s = e;
            p0 = p;
            loop {
                let fresh2 = p;
                p = p.offset(1);
                c = *fresh2 as libc::c_int;
                if isspc(c) != 0 {
                    s = 1 as libc::c_int;
                } else {
                    if c == '/' as i32
                        && (*p.offset(0 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                            || *p.offset(0 as libc::c_int as isize) as libc::c_int
                                == '*' as i32)
                    {
                        c = 0 as libc::c_int;
                    } else if s != 0 && q > l2.as_mut_ptr()
                        && (isid(*q.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                            != 0 && isid(c) != 0
                            || q >= l2.as_mut_ptr().offset(2 as libc::c_int as isize)
                                && l2[0 as libc::c_int as usize] as libc::c_int
                                    == '#' as i32
                                && l2[1 as libc::c_int as usize] as libc::c_int
                                    == 'd' as i32 && f < 2 as libc::c_int && e == 0)
                    {
                        let fresh3 = q;
                        q = q.offset(1);
                        *fresh3 = ' ' as i32 as libc::c_char;
                        f += 1;
                        f;
                    }
                    s = 0 as libc::c_int;
                    if c == '(' as i32 {
                        e += 1;
                        e;
                    }
                    if c == ')' as i32 {
                        e -= 1;
                        e;
                    }
                    if c == '\\' as i32 || c == '"' as i32 {
                        let fresh4 = q;
                        q = q.offset(1);
                        *fresh4 = '\\' as i32 as libc::c_char;
                    }
                    let fresh5 = q;
                    q = q.offset(1);
                    *fresh5 = c as libc::c_char;
                    if c == 0 as libc::c_int {
                        break;
                    }
                    p0 = p;
                }
            }
            fprintf(
                op,
                b"    \"%s\\n\"%s\n\0" as *const u8 as *const libc::c_char,
                l2.as_mut_ptr(),
                p0,
            );
        }
    }
    fclose(fp);
    fclose(op);
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
