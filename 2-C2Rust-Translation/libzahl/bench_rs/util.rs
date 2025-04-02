#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn getpid() -> __pid_t;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
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
#[no_mangle]
pub static mut timebuf: [libc::c_char; 512] = [0; 512];
#[no_mangle]
pub static mut freq: libc::c_ulonglong = 0;
#[no_mangle]
pub unsafe extern "C" fn benchmark_init() {
    let mut cpuset: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    libc::memset(
        &mut cpuset as *mut cpu_set_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
    );
    let mut __cpu: size_t = 0 as libc::c_int as size_t;
    if (__cpu / 8 as libc::c_int as size_t)
        < ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong
    {
        *(cpuset.__bits)
            .as_mut_ptr()
            .offset(
                __cpu
                    .wrapping_div(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            |= (1 as libc::c_int as __cpu_mask)
                << __cpu
                    .wrapping_rem(
                        (8 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                            ),
                    );
    } else {};
    sched_setaffinity(
        getpid(),
        ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut cpuset,
    );
    sprintf(
        path.as_mut_ptr(),
        b"/sys/devices/system/cpu/cpu%i/cpufreq/cpuinfo_max_freq\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int,
    );
    f = fopen(path.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    if getline(&mut line, &mut size, f) < 0 as libc::c_int as __ssize_t {
        abort();
    }
    fclose(f);
    freq = strtoull(line, 0 as *mut *mut libc::c_char, 10 as libc::c_int);
    free(line as *mut libc::c_void);
}
