
#![feature(extern_types)]

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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn BZ2_bzBuffToBuffCompress(
        dest: *mut libc::c_char,
        destLen: *mut libc::c_uint,
        source: *mut libc::c_char,
        sourceLen: libc::c_uint,
        blockSize100k: libc::c_int,
        verbosity: libc::c_int,
        workFactor: libc::c_int,
    ) -> libc::c_int;
    fn BZ2_bzBuffToBuffDecompress(
        dest: *mut libc::c_char,
        destLen: *mut libc::c_uint,
        source: *mut libc::c_char,
        sourceLen: libc::c_uint,
        small: libc::c_int,
        verbosity: libc::c_int,
    ) -> libc::c_int;
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
pub type uchar = libc::c_uchar;
#[no_mangle]
pub static mut inbuf: [uchar; 1000000] = [0; 1000000];
#[no_mangle]
pub static mut outbuf: [uchar; 2000000] = [0; 2000000];
#[no_mangle]
pub static mut zbuf: [uchar; 1010600] = [0; 1010600];
#[no_mangle]
pub static mut nIn: libc::c_int = 0;
#[no_mangle]
pub static mut nOut: libc::c_int = 0;
#[no_mangle]
pub static mut nZ: libc::c_int = 0;
static mut bzerrorstrings: [*mut libc::c_char; 15] = [
    b"OK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SEQUENCE_ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"PARAM_ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MEM_ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"DATA_ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"DATA_ERROR_MAGIC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IO_ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"UNEXPECTED_EOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OUTBUFF_FULL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn flip_bit(mut bit: libc::c_int) {
    let mut byteno: libc::c_int = bit / 8 as libc::c_int;
    let mut bitno: libc::c_int = bit % 8 as libc::c_int;
    let mut mask: uchar = ((1 as libc::c_int) << bitno) as uchar;
    zbuf[byteno
        as usize] = (zbuf[byteno as usize] as libc::c_int ^ mask as libc::c_int)
        as uchar;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut r: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if argc != 2 as libc::c_int {
        fprintf(
            stderr,
            b"usage: unzcrash filename\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    f = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"unzcrash: can't open %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    nIn = fread(
        inbuf.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1000000 as libc::c_int as libc::c_ulong,
        f,
    ) as libc::c_int;
    fprintf(stderr, b"%d bytes read\n\0" as *const u8 as *const libc::c_char, nIn);
    nZ = 1000000 as libc::c_int;
    r = BZ2_bzBuffToBuffCompress(
        zbuf.as_mut_ptr() as *mut libc::c_char,
        &mut nZ as *mut libc::c_int as *mut libc::c_uint,
        inbuf.as_mut_ptr() as *mut libc::c_char,
        nIn as libc::c_uint,
        9 as libc::c_int,
        0 as libc::c_int,
        30 as libc::c_int,
    );
    if r == 0 as libc::c_int {} else {
        __assert_fail(
            b"r == BZ_OK\0" as *const u8 as *const libc::c_char,
            b"unzcrash.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    };
    fprintf(stderr, b"%d after compression\n\0" as *const u8 as *const libc::c_char, nZ);
    bit = 0 as libc::c_int;
    while bit < nZ * 8 as libc::c_int {
        fprintf(stderr, b"bit %d  \0" as *const u8 as *const libc::c_char, bit);
        flip_bit(bit);
        nOut = 1000000 as libc::c_int + 1000000 as libc::c_int;
        r = BZ2_bzBuffToBuffDecompress(
            outbuf.as_mut_ptr() as *mut libc::c_char,
            &mut nOut as *mut libc::c_int as *mut libc::c_uint,
            zbuf.as_mut_ptr() as *mut libc::c_char,
            nZ as libc::c_uint,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        fprintf(
            stderr,
            b" %d  %s \0" as *const u8 as *const libc::c_char,
            r,
            bzerrorstrings[-r as usize],
        );
        if r != 0 as libc::c_int {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        } else if nOut != nIn {
            fprintf(
                stderr,
                b"nIn/nOut mismatch %d %d\n\0" as *const u8 as *const libc::c_char,
                nIn,
                nOut,
            );
            return 1 as libc::c_int;
        } else {
            i = 0 as libc::c_int;
            while i < nOut {
                if inbuf[i as usize] as libc::c_int != outbuf[i as usize] as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"mismatch at %d\n\0" as *const u8 as *const libc::c_char,
                        i,
                    );
                    return 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            if i == nOut {
                fprintf(stderr, b"really ok!\n\0" as *const u8 as *const libc::c_char);
            }
        }
        flip_bit(bit);
        bit += 1;
        bit;
    }
    fprintf(stderr, b"all ok\n\0" as *const u8 as *const libc::c_char);
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
