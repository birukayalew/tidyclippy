use ::libc;
extern "C" {
    fn forkpty(
        __amaster: *mut libc::c_int,
        __name: *mut libc::c_char,
        __termp: *const termios,
        __winp: *const winsize,
    ) -> libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[no_mangle]
pub unsafe extern "C" fn getptmfd() -> libc::c_int {
    return 2147483647 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fdforkpty(
    mut ptmfd: libc::c_int,
    mut master: *mut libc::c_int,
    mut name: *mut libc::c_char,
    mut tio: *mut termios,
    mut ws: *mut winsize,
) -> pid_t {
    return forkpty(master, name, tio, ws);
}
