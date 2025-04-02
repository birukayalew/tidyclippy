use ::libc;
extern "C" {
    pub type stat;
    pub type dirent;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn getpid() -> __pid_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fatal(_: *const libc::c_char, _: ...);
}
pub type __pid_t = libc::c_int;
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
#[no_mangle]
pub unsafe extern "C" fn getdtablecount() -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut g: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut n: libc::c_int = 0 as libc::c_int;
    if snprintf(
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"/proc/%ld/fd/*\0" as *const u8 as *const libc::c_char,
        getpid() as libc::c_long,
    ) < 0 as libc::c_int
    {
        fatal(b"snprintf overflow\0" as *const u8 as *const libc::c_char);
    }
    if glob(path.as_mut_ptr(), 0 as libc::c_int, None, &mut g) == 0 as libc::c_int {
        n = g.gl_pathc as libc::c_int;
    }
    globfree(&mut g);
    return n;
}
