use ::libc;
extern "C" {
    pub type event_base;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_get_version() -> *const libc::c_char;
    fn event_loop(_: libc::c_int) -> libc::c_int;
    fn event_get_method() -> *const libc::c_char;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn setproctitle(_: *const libc::c_char, _: ...);
    fn msgbuf_write(_: *mut msgbuf) -> libc::c_int;
    fn imsg_init(_: *mut imsgbuf, _: libc::c_int);
    fn imsg_read(_: *mut imsgbuf) -> ssize_t;
    fn imsg_get(_: *mut imsgbuf, _: *mut imsg) -> ssize_t;
    fn imsg_compose(
        _: *mut imsgbuf,
        _: uint32_t,
        _: uint32_t,
        _: pid_t,
        _: libc::c_int,
        _: *const libc::c_void,
        _: uint16_t,
    ) -> libc::c_int;
    fn imsg_free(_: *mut imsg);
    fn imsg_clear(_: *mut imsgbuf);
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut socket_path: *const libc::c_char;
    fn log_open(_: *const libc::c_char);
    fn log_toggle(_: *const libc::c_char);
    fn log_debug(_: *const libc::c_char, _: ...);
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_4,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub evcb_callback: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
    >,
    pub evcb_selfcb: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
    pub evcb_evfinalize: Option::<
        unsafe extern "C" fn(*mut event, *mut libc::c_void) -> (),
    >,
    pub evcb_cbfinalize: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_17,
    pub _timer: C2RustUnnamed_16,
    pub _rt: C2RustUnnamed_15,
    pub _sigchld: C2RustUnnamed_14,
    pub _sigfault: C2RustUnnamed_11,
    pub _sigpoll: C2RustUnnamed_10,
    pub _sigsys: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub _addr_bnd: C2RustUnnamed_13,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_18,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ibuf {
    pub entry: C2RustUnnamed_19,
    pub buf: *mut u_char,
    pub size: size_t,
    pub max: size_t,
    pub wpos: size_t,
    pub rpos: size_t,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut ibuf,
    pub tqe_prev: *mut *mut ibuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msgbuf {
    pub bufs: C2RustUnnamed_20,
    pub queued: uint32_t,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub tqh_first: *mut ibuf,
    pub tqh_last: *mut *mut ibuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ibuf_read {
    pub buf: [u_char; 65535],
    pub rptr: *mut u_char,
    pub wpos: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg_fd {
    pub entry: C2RustUnnamed_21,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub tqe_next: *mut imsg_fd,
    pub tqe_prev: *mut *mut imsg_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsgbuf {
    pub fds: C2RustUnnamed_22,
    pub r: ibuf_read,
    pub w: msgbuf,
    pub fd: libc::c_int,
    pub pid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub tqh_first: *mut imsg_fd,
    pub tqh_last: *mut *mut imsg_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tmuxpeer {
    pub parent: *mut tmuxproc,
    pub ibuf: imsgbuf,
    pub event: event,
    pub flags: libc::c_int,
    pub dispatchcb: Option::<unsafe extern "C" fn(*mut imsg, *mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tmuxproc {
    pub name: *const libc::c_char,
    pub exit: libc::c_int,
    pub signalcb: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub ev_sighup: event,
    pub ev_sigchld: event,
    pub ev_sigcont: event,
    pub ev_sigterm: event,
    pub ev_sigusr1: event,
    pub ev_sigusr2: event,
    pub ev_sigwinch: event,
}
pub type msgtype = libc::c_uint;
pub const MSG_EXEC: msgtype = 217;
pub const MSG_WAKEUP: msgtype = 216;
pub const MSG_UNLOCK: msgtype = 215;
pub const MSG_SUSPEND: msgtype = 214;
pub const MSG_STDOUT: msgtype = 213;
pub const MSG_STDIN: msgtype = 212;
pub const MSG_STDERR: msgtype = 211;
pub const MSG_SHUTDOWN: msgtype = 210;
pub const MSG_SHELL: msgtype = 209;
pub const MSG_RESIZE: msgtype = 208;
pub const MSG_READY: msgtype = 207;
pub const MSG_LOCK: msgtype = 206;
pub const MSG_EXITING: msgtype = 205;
pub const MSG_EXITED: msgtype = 204;
pub const MSG_EXIT: msgtype = 203;
pub const MSG_DETACHKILL: msgtype = 202;
pub const MSG_DETACH: msgtype = 201;
pub const MSG_COMMAND: msgtype = 200;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
pub const MSG_IDENTIFY_TERM: msgtype = 101;
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
pub const MSG_VERSION: msgtype = 12;
unsafe extern "C" fn proc_event_cb(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut peer: *mut tmuxpeer = arg as *mut tmuxpeer;
    let mut n: ssize_t = 0;
    let mut imsg: imsg = imsg {
        hdr: imsg_hdr {
            type_0: 0,
            len: 0,
            flags: 0,
            peerid: 0,
            pid: 0,
        },
        fd: 0,
        data: 0 as *mut libc::c_void,
    };
    if (*peer).flags & 0x1 as libc::c_int == 0
        && events as libc::c_int & 0x2 as libc::c_int != 0
    {
        n = imsg_read(&mut (*peer).ibuf);
        if n == -(1 as libc::c_int) as ssize_t
            && *__errno_location() != 11 as libc::c_int
            || n == 0 as libc::c_int as ssize_t
        {
            ((*peer).dispatchcb)
                .expect("non-null function pointer")(0 as *mut imsg, (*peer).arg);
            return;
        }
        loop {
            n = imsg_get(&mut (*peer).ibuf, &mut imsg);
            if n == -(1 as libc::c_int) as ssize_t {
                ((*peer).dispatchcb)
                    .expect("non-null function pointer")(0 as *mut imsg, (*peer).arg);
                return;
            }
            if n == 0 as libc::c_int as ssize_t {
                break;
            }
            log_debug(
                b"peer %p message %d\0" as *const u8 as *const libc::c_char,
                peer,
                imsg.hdr.type_0,
            );
            if peer_check_version(peer, &mut imsg) != 0 as libc::c_int {
                if imsg.fd != -(1 as libc::c_int) {
                    close(imsg.fd);
                }
                imsg_free(&mut imsg);
                break;
            } else {
                ((*peer).dispatchcb)
                    .expect("non-null function pointer")(&mut imsg, (*peer).arg);
                imsg_free(&mut imsg);
            }
        }
    }
    if events as libc::c_int & 0x4 as libc::c_int != 0 {
        if msgbuf_write(&mut (*peer).ibuf.w) <= 0 as libc::c_int
            && *__errno_location() != 11 as libc::c_int
        {
            ((*peer).dispatchcb)
                .expect("non-null function pointer")(0 as *mut imsg, (*peer).arg);
            return;
        }
    }
    if (*peer).flags & 0x1 as libc::c_int != 0
        && (*peer).ibuf.w.queued == 0 as libc::c_int as uint32_t
    {
        ((*peer).dispatchcb)
            .expect("non-null function pointer")(0 as *mut imsg, (*peer).arg);
        return;
    }
    proc_update_event(peer);
}
unsafe extern "C" fn proc_signal_cb(
    mut signo: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut tp: *mut tmuxproc = arg as *mut tmuxproc;
    ((*tp).signalcb).expect("non-null function pointer")(signo);
}
unsafe extern "C" fn peer_check_version(
    mut peer: *mut tmuxpeer,
    mut imsg: *mut imsg,
) -> libc::c_int {
    let mut version: libc::c_int = 0;
    version = ((*imsg).hdr.peerid & 0xff as libc::c_int as uint32_t) as libc::c_int;
    if (*imsg).hdr.type_0 != MSG_VERSION as libc::c_int as uint32_t
        && version != 8 as libc::c_int
    {
        log_debug(
            b"peer %p bad version %d\0" as *const u8 as *const libc::c_char,
            peer,
            version,
        );
        proc_send(
            peer,
            MSG_VERSION,
            -(1 as libc::c_int),
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        );
        (*peer).flags |= 0x1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn proc_update_event(mut peer: *mut tmuxpeer) {
    let mut events: libc::c_short = 0;
    event_del(&mut (*peer).event);
    events = 0x2 as libc::c_int as libc::c_short;
    if (*peer).ibuf.w.queued > 0 as libc::c_int as uint32_t {
        events = (events as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    event_set(
        &mut (*peer).event,
        (*peer).ibuf.fd,
        events,
        Some(
            proc_event_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        peer as *mut libc::c_void,
    );
    event_add(&mut (*peer).event, 0 as *const timeval);
}
#[no_mangle]
pub unsafe extern "C" fn proc_send(
    mut peer: *mut tmuxpeer,
    mut type_0: msgtype,
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut ibuf: *mut imsgbuf = &mut (*peer).ibuf;
    let mut vp: *mut libc::c_void = buf as *mut libc::c_void;
    let mut retval: libc::c_int = 0;
    if (*peer).flags & 0x1 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    log_debug(
        b"sending message %d to peer %p (%zu bytes)\0" as *const u8
            as *const libc::c_char,
        type_0 as libc::c_uint,
        peer,
        len,
    );
    retval = imsg_compose(
        ibuf,
        type_0 as uint32_t,
        8 as libc::c_int as uint32_t,
        -(1 as libc::c_int),
        fd,
        vp,
        len as uint16_t,
    );
    if retval != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    proc_update_event(peer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn proc_start(mut name: *const libc::c_char) -> *mut tmuxproc {
    let mut tp: *mut tmuxproc = 0 as *mut tmuxproc;
    let mut u: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    log_open(name);
    setproctitle(b"%s (%s)\0" as *const u8 as *const libc::c_char, name, socket_path);
    if uname(&mut u) < 0 as libc::c_int {
        memset(
            &mut u as *mut utsname as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<utsname>() as libc::c_ulong,
        );
    }
    log_debug(
        b"%s started (%ld): version %s, socket %s, protocol %d\0" as *const u8
            as *const libc::c_char,
        name,
        getpid() as libc::c_long,
        b"master\0" as *const u8 as *const libc::c_char,
        socket_path,
        8 as libc::c_int,
    );
    log_debug(
        b"on %s %s %s; libevent %s (%s)\0" as *const u8 as *const libc::c_char,
        (u.sysname).as_mut_ptr(),
        (u.release).as_mut_ptr(),
        (u.version).as_mut_ptr(),
        event_get_version(),
        event_get_method(),
    );
    tp = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<tmuxproc>() as libc::c_ulong,
    ) as *mut tmuxproc;
    (*tp).name = xstrdup(name);
    return tp;
}
#[no_mangle]
pub unsafe extern "C" fn proc_loop(
    mut tp: *mut tmuxproc,
    mut loopcb: Option::<unsafe extern "C" fn() -> libc::c_int>,
) {
    log_debug(b"%s loop enter\0" as *const u8 as *const libc::c_char, (*tp).name);
    loop {
        event_loop(0x1 as libc::c_int);
        if !((*tp).exit == 0
            && (loopcb.is_none() || loopcb.expect("non-null function pointer")() == 0))
        {
            break;
        }
    }
    log_debug(b"%s loop exit\0" as *const u8 as *const libc::c_char, (*tp).name);
}
#[no_mangle]
pub unsafe extern "C" fn proc_exit(mut tp: *mut tmuxproc) {
    (*tp).exit = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn proc_set_signals(
    mut tp: *mut tmuxproc,
    mut signalcb: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
) {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_18 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    (*tp).signalcb = signalcb;
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0x10000000 as libc::c_int;
    sa
        .__sigaction_handler
        .sa_handler = ::core::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(13 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(20 as libc::c_int, &mut sa, 0 as *mut sigaction);
    event_set(
        &mut (*tp).ev_sighup,
        1 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sighup, 0 as *const timeval);
    event_set(
        &mut (*tp).ev_sigchld,
        17 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sigchld, 0 as *const timeval);
    event_set(
        &mut (*tp).ev_sigcont,
        18 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sigcont, 0 as *const timeval);
    event_set(
        &mut (*tp).ev_sigterm,
        15 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sigterm, 0 as *const timeval);
    event_set(
        &mut (*tp).ev_sigusr1,
        10 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sigusr1, 0 as *const timeval);
    event_set(
        &mut (*tp).ev_sigusr2,
        12 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sigusr2, 0 as *const timeval);
    event_set(
        &mut (*tp).ev_sigwinch,
        28 as libc::c_int,
        (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            proc_signal_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        tp as *mut libc::c_void,
    );
    event_add(&mut (*tp).ev_sigwinch, 0 as *const timeval);
}
#[no_mangle]
pub unsafe extern "C" fn proc_clear_signals(
    mut tp: *mut tmuxproc,
    mut defaults: libc::c_int,
) {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_18 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0x10000000 as libc::c_int;
    sa.__sigaction_handler.sa_handler = None;
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(13 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(20 as libc::c_int, &mut sa, 0 as *mut sigaction);
    event_del(&mut (*tp).ev_sighup);
    event_del(&mut (*tp).ev_sigchld);
    event_del(&mut (*tp).ev_sigcont);
    event_del(&mut (*tp).ev_sigterm);
    event_del(&mut (*tp).ev_sigusr1);
    event_del(&mut (*tp).ev_sigusr2);
    event_del(&mut (*tp).ev_sigwinch);
    if defaults != 0 {
        sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction);
        sigaction(17 as libc::c_int, &mut sa, 0 as *mut sigaction);
        sigaction(18 as libc::c_int, &mut sa, 0 as *mut sigaction);
        sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction);
        sigaction(10 as libc::c_int, &mut sa, 0 as *mut sigaction);
        sigaction(12 as libc::c_int, &mut sa, 0 as *mut sigaction);
        sigaction(28 as libc::c_int, &mut sa, 0 as *mut sigaction);
    }
}
#[no_mangle]
pub unsafe extern "C" fn proc_add_peer(
    mut tp: *mut tmuxproc,
    mut fd: libc::c_int,
    mut dispatchcb: Option::<unsafe extern "C" fn(*mut imsg, *mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> *mut tmuxpeer {
    let mut peer: *mut tmuxpeer = 0 as *mut tmuxpeer;
    peer = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<tmuxpeer>() as libc::c_ulong,
    ) as *mut tmuxpeer;
    (*peer).parent = tp;
    (*peer).dispatchcb = dispatchcb;
    (*peer).arg = arg;
    imsg_init(&mut (*peer).ibuf, fd);
    event_set(
        &mut (*peer).event,
        fd,
        0x2 as libc::c_int as libc::c_short,
        Some(
            proc_event_cb
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        peer as *mut libc::c_void,
    );
    log_debug(
        b"add peer %p: %d (%p)\0" as *const u8 as *const libc::c_char,
        peer,
        fd,
        arg,
    );
    proc_update_event(peer);
    return peer;
}
#[no_mangle]
pub unsafe extern "C" fn proc_remove_peer(mut peer: *mut tmuxpeer) {
    log_debug(b"remove peer %p\0" as *const u8 as *const libc::c_char, peer);
    event_del(&mut (*peer).event);
    imsg_clear(&mut (*peer).ibuf);
    close((*peer).ibuf.fd);
    free(peer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn proc_kill_peer(mut peer: *mut tmuxpeer) {
    (*peer).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn proc_toggle_log(mut tp: *mut tmuxproc) {
    log_toggle((*tp).name);
}
