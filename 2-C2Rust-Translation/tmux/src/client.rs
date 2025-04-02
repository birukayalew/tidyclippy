use ::libc;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type environ;
    pub type options;
    pub type hooks;
    pub type screen_titles;
    pub type input_ctx;
    pub type format_tree;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type tmuxproc;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn closefrom(__lowfd: libc::c_int);
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn cfgetispeed(__termios_p: *const termios) -> speed_t;
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn cfmakeraw(__termios_p: *mut termios);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    static mut global_s_options: *mut options;
    static mut global_w_options: *mut options;
    static mut global_environ: *mut environ;
    static mut socket_path: *const libc::c_char;
    static mut shell_command: *const libc::c_char;
    static mut ptm_fd: libc::c_int;
    fn setblocking(_: libc::c_int, _: libc::c_int);
    fn find_home() -> *const libc::c_char;
    fn proc_send(
        _: *mut tmuxpeer,
        _: msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    fn proc_start(_: *const libc::c_char) -> *mut tmuxproc;
    fn proc_loop(_: *mut tmuxproc, _: Option::<unsafe extern "C" fn() -> libc::c_int>);
    fn proc_exit(_: *mut tmuxproc);
    fn proc_set_signals(
        _: *mut tmuxproc,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    );
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int);
    fn proc_add_peer(
        _: *mut tmuxproc,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut imsg, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> *mut tmuxpeer;
    fn options_free(_: *mut options);
    fn environ_free(_: *mut environ);
    fn cmd_pack_argv(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    fn cmd_list_parse(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut cmd_list;
    fn cmd_list_free(_: *mut cmd_list);
    fn server_start(
        _: *mut tmuxproc,
        _: *mut event_base,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_16,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_11,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ev_io: C2RustUnnamed_14,
    pub ev_signal: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub ev_signal_next: C2RustUnnamed_13,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub ev_io_next: C2RustUnnamed_15,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub ev_next_with_common_timeout: C2RustUnnamed_17,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_19,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_18,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
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
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferevent {
    pub ev_base: *mut event_base,
    pub be_ops: *const bufferevent_ops,
    pub ev_read: event,
    pub ev_write: event,
    pub input: *mut evbuffer,
    pub output: *mut evbuffer,
    pub wm_read: event_watermark,
    pub wm_write: event_watermark,
    pub readcb: bufferevent_data_cb,
    pub writecb: bufferevent_data_cb,
    pub errorcb: bufferevent_event_cb,
    pub cbarg: *mut libc::c_void,
    pub timeout_read: timeval,
    pub timeout_write: timeval,
    pub enabled: libc::c_short,
}
pub type bufferevent_event_cb = Option::<
    unsafe extern "C" fn(*mut bufferevent, libc::c_short, *mut libc::c_void) -> (),
>;
pub type bufferevent_data_cb = Option::<
    unsafe extern "C" fn(*mut bufferevent, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
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
pub type bitstr_t = libc::c_uchar;
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
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut tmuxpeer,
    pub queue: cmdq_list,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub event: event,
    pub retval: libc::c_int,
    pub creation_time: timeval,
    pub activity_time: timeval,
    pub environ: *mut environ,
    pub jobs: *mut format_job_tree,
    pub title: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub term: *mut libc::c_char,
    pub ttyname: *mut libc::c_char,
    pub tty: tty,
    pub written: size_t,
    pub discarded: size_t,
    pub redraw: size_t,
    pub stdin_callback: Option::<
        unsafe extern "C" fn(*mut client, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub stdin_callback_data: *mut libc::c_void,
    pub stdin_data: *mut evbuffer,
    pub stdin_closed: libc::c_int,
    pub stdout_data: *mut evbuffer,
    pub stderr_data: *mut evbuffer,
    pub repeat_timer: event,
    pub click_timer: event,
    pub click_button: u_int,
    pub status: status_line,
    pub flags: libc::c_int,
    pub keytable: *mut key_table,
    pub identify_timer: event,
    pub identify_callback: Option::<
        unsafe extern "C" fn(*mut client, *mut window_pane) -> (),
    >,
    pub identify_callback_data: *mut libc::c_void,
    pub message_string: *mut libc::c_char,
    pub message_timer: event,
    pub message_next: u_int,
    pub message_log: C2RustUnnamed_36,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_35,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub creation_time: timeval,
    pub last_attached_time: timeval,
    pub activity_time: timeval,
    pub last_activity_time: timeval,
    pub lock_timer: event,
    pub sx: u_int,
    pub sy: u_int,
    pub curw: *mut winlink,
    pub lastw: winlink_stack,
    pub windows: winlinks,
    pub statusat: libc::c_int,
    pub hooks: *mut hooks,
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut environ,
    pub references: libc::c_int,
    pub gentry: C2RustUnnamed_22,
    pub entry: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub status_width: size_t,
    pub status_cell: grid_cell,
    pub status_text: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_25,
    pub wentry: C2RustUnnamed_24,
    pub sentry: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub name_event: event,
    pub name_time: timeval,
    pub alerts_timer: event,
    pub activity_time: timeval,
    pub active: *mut window_pane,
    pub last: *mut window_pane,
    pub panes: window_panes,
    pub lastlayout: libc::c_int,
    pub layout_root: *mut layout_cell,
    pub saved_layout_root: *mut layout_cell,
    pub old_layout: *mut libc::c_char,
    pub sx: u_int,
    pub sy: u_int,
    pub flags: libc::c_int,
    pub alerts_queued: libc::c_int,
    pub alerts_entry: C2RustUnnamed_28,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_27,
    pub entry: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane {
    pub id: u_int,
    pub active_point: u_int,
    pub window: *mut window,
    pub layout_cell: *mut layout_cell,
    pub saved_layout_cell: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub osx: u_int,
    pub osy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub flags: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub pid: pid_t,
    pub tty: [libc::c_char; 32],
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub resize_timer: event,
    pub ictx: *mut input_ctx,
    pub colgc: grid_cell,
    pub palette: *mut libc::c_int,
    pub pipe_fd: libc::c_int,
    pub pipe_event: *mut bufferevent,
    pub pipe_off: size_t,
    pub screen: *mut screen,
    pub base: screen,
    pub status_screen: screen,
    pub status_size: size_t,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut grid,
    pub saved_cell: grid_cell,
    pub mode: *const window_mode,
    pub modedata: *mut libc::c_void,
    pub modetimer: event,
    pub modelast: time_t,
    pub modeprefix: u_int,
    pub searchstr: *mut libc::c_char,
    pub entry: C2RustUnnamed_31,
    pub tree_entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut window_pane,
            *mut cmd_find_state,
            *mut args,
        ) -> *mut screen,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut window_pane) -> ()>,
    pub resize: Option::<unsafe extern "C" fn(*mut window_pane, u_int, u_int) -> ()>,
    pub key: Option::<
        unsafe extern "C" fn(
            *mut window_pane,
            *mut client,
            *mut session,
            key_code,
            *mut mouse_event,
        ) -> (),
    >,
    pub key_table: Option::<
        unsafe extern "C" fn(*mut window_pane) -> *const libc::c_char,
    >,
    pub command: Option::<
        unsafe extern "C" fn(
            *mut window_pane,
            *mut client,
            *mut session,
            *mut args,
            *mut mouse_event,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouse_event {
    pub valid: libc::c_int,
    pub key: key_code,
    pub statusat: libc::c_int,
    pub x: u_int,
    pub y: u_int,
    pub b: u_int,
    pub lx: u_int,
    pub ly: u_int,
    pub lb: u_int,
    pub s: libc::c_int,
    pub w: libc::c_int,
    pub wp: libc::c_int,
    pub sgr_type: u_int,
    pub sgr_b: u_int,
}
pub type key_code = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen {
    pub title: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: screen_sel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: C2RustUnnamed_32,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type C2RustUnnamed_32 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed_32 = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed_32 = 1;
pub const LINE_SEL_NONE: C2RustUnnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_cell,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub c2rust_unnamed: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub offset: u_int,
    pub data: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type layout_type = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type C2RustUnnamed_35 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_35 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_35 = 0;
pub type prompt_free_cb = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type prompt_input_cb = Option::<
    unsafe extern "C" fn(
        *mut client,
        *mut libc::c_void,
        *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub flags: libc::c_int,
    pub qentry: C2RustUnnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_43,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option::<unsafe extern "C" fn(*mut cmd, *mut cmdq_item) -> cmd_retval>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdq_item {
    pub name: *const libc::c_char,
    pub queue: *mut cmdq_list,
    pub next: *mut cmdq_item,
    pub client: *mut client,
    pub type_0: cmdq_type,
    pub group: u_int,
    pub number: u_int,
    pub time: time_t,
    pub flags: libc::c_int,
    pub shared: *mut cmdq_shared,
    pub source: cmd_find_state,
    pub target: cmd_find_state,
    pub cmdlist: *mut cmd_list,
    pub cmd: *mut cmd,
    pub cb: cmdq_cb,
    pub data: *mut libc::c_void,
    pub entry: C2RustUnnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type cmdq_cb = Option::<
    unsafe extern "C" fn(*mut cmdq_item, *mut libc::c_void) -> cmd_retval,
>;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdq_shared {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut format_tree,
    pub mouse: mouse_event,
    pub current: cmd_find_state,
}
pub type cmdq_type = libc::c_uint;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const CMDQ_COMMAND: cmdq_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdq_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_line {
    pub timer: event,
    pub status: screen,
    pub old_status: *mut screen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty {
    pub client: *mut client,
    pub sx: u_int,
    pub sy: u_int,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub mode: libc::c_int,
    pub rlower: u_int,
    pub rupper: u_int,
    pub rleft: u_int,
    pub rright: u_int,
    pub fd: libc::c_int,
    pub event_in: event,
    pub in_0: *mut evbuffer,
    pub event_out: event,
    pub out: *mut evbuffer,
    pub timer: event,
    pub discarded: size_t,
    pub tio: termios,
    pub cell: grid_cell,
    pub last_wp: libc::c_int,
    pub last_cell: grid_cell,
    pub flags: libc::c_int,
    pub term: *mut tty_term,
    pub term_name: *mut libc::c_char,
    pub term_flags: libc::c_int,
    pub term_type: C2RustUnnamed_44,
    pub mouse: mouse_event,
    pub mouse_drag_flag: libc::c_int,
    pub mouse_drag_update: Option::<
        unsafe extern "C" fn(*mut client, *mut mouse_event) -> (),
    >,
    pub mouse_drag_release: Option::<
        unsafe extern "C" fn(*mut client, *mut mouse_event) -> (),
    >,
    pub key_timer: event,
    pub key_tree: *mut tty_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
pub type C2RustUnnamed_44 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_44 = 6;
pub const TTY_VT420: C2RustUnnamed_44 = 5;
pub const TTY_VT320: C2RustUnnamed_44 = 4;
pub const TTY_VT220: C2RustUnnamed_44 = 3;
pub const TTY_VT102: C2RustUnnamed_44 = 2;
pub const TTY_VT101: C2RustUnnamed_44 = 1;
pub const TTY_VT100: C2RustUnnamed_44 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_command_data {
    pub argc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_stdin_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_stdout_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_stderr_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
pub const CLIENT_EXIT_SERVER_EXITED: C2RustUnnamed_46 = 7;
pub const CLIENT_EXIT_EXITED: C2RustUnnamed_46 = 6;
pub const CLIENT_EXIT_LOST_SERVER: C2RustUnnamed_46 = 5;
pub const CLIENT_EXIT_TERMINATED: C2RustUnnamed_46 = 4;
pub const CLIENT_EXIT_LOST_TTY: C2RustUnnamed_46 = 3;
pub const CLIENT_EXIT_DETACHED_HUP: C2RustUnnamed_46 = 2;
pub const CLIENT_EXIT_DETACHED: C2RustUnnamed_46 = 1;
pub const CLIENT_EXIT_NONE: C2RustUnnamed_46 = 0;
pub type C2RustUnnamed_46 = libc::c_uint;
static mut client_proc: *mut tmuxproc = 0 as *const tmuxproc as *mut tmuxproc;
static mut client_peer: *mut tmuxpeer = 0 as *const tmuxpeer as *mut tmuxpeer;
static mut client_flags: libc::c_int = 0;
static mut client_stdin: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_19 {
            tqe_next: 0 as *const event_callback as *mut event_callback,
            tqe_prev: 0 as *const *mut event_callback as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_18 {
            evcb_callback: None,
        },
        evcb_arg: 0 as *const libc::c_void as *mut libc::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_16 {
        ev_next_with_common_timeout: C2RustUnnamed_17 {
            tqe_next: 0 as *const event as *mut event,
            tqe_prev: 0 as *const *mut event as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: 0 as *const event_base as *mut event_base,
    ev_: C2RustUnnamed_11 {
        ev_io: C2RustUnnamed_14 {
            ev_io_next: C2RustUnnamed_15 {
                le_next: 0 as *const event as *mut event,
                le_prev: 0 as *const *mut event as *mut *mut event,
            },
            ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
        },
    },
    ev_events: 0,
    ev_res: 0,
    ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
};
static mut client_exitreason: C2RustUnnamed_46 = CLIENT_EXIT_NONE;
static mut client_exitval: libc::c_int = 0;
static mut client_exittype: msgtype = 0 as msgtype;
static mut client_exitsession: *const libc::c_char = 0 as *const libc::c_char;
static mut client_execshell: *const libc::c_char = 0 as *const libc::c_char;
static mut client_execcmd: *const libc::c_char = 0 as *const libc::c_char;
static mut client_attached: libc::c_int = 0;
unsafe extern "C" fn client_get_lock(mut lockfile: *mut libc::c_char) -> libc::c_int {
    let mut lockfd: libc::c_int = 0;
    log_debug(b"lock file is %s\0" as *const u8 as *const libc::c_char, lockfile);
    lockfd = open(
        lockfile,
        0o1 as libc::c_int | 0o100 as libc::c_int,
        0o600 as libc::c_int,
    );
    if lockfd == -(1 as libc::c_int) {
        log_debug(
            b"open failed: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if flock(lockfd, 2 as libc::c_int | 4 as libc::c_int) == -(1 as libc::c_int) {
        log_debug(
            b"flock failed: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if *__errno_location() != 11 as libc::c_int {
            return lockfd;
        }
        while flock(lockfd, 2 as libc::c_int) == -(1 as libc::c_int)
            && *__errno_location() == 4 as libc::c_int
        {}
        close(lockfd);
        return -(2 as libc::c_int);
    }
    log_debug(b"flock succeeded\0" as *const u8 as *const libc::c_char);
    return lockfd;
}
unsafe extern "C" fn client_connect(
    mut base: *mut event_base,
    mut path: *const libc::c_char,
    mut start_server: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut size: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut lockfd: libc::c_int = -(1 as libc::c_int);
    let mut locked: libc::c_int = 0 as libc::c_int;
    let mut lockfile: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    size = strlcpy(
        (sa.sun_path).as_mut_ptr(),
        path,
        ::core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    if size >= ::core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    log_debug(b"socket is %s\0" as *const u8 as *const libc::c_char, path);
    loop {
        fd = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        log_debug(b"trying connect\0" as *const u8 as *const libc::c_char);
        if !(connect(
            fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &mut sa as *mut sockaddr_un as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
        ) == -(1 as libc::c_int))
        {
            current_block = 17478428563724192186;
            break;
        }
        log_debug(
            b"connect failed: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if *__errno_location() != 111 as libc::c_int
            && *__errno_location() != 2 as libc::c_int
        {
            current_block = 18072425028949421867;
            break;
        }
        if start_server == 0 {
            current_block = 18072425028949421867;
            break;
        }
        close(fd);
        if locked == 0 {
            xasprintf(
                &mut lockfile as *mut *mut libc::c_char,
                b"%s.lock\0" as *const u8 as *const libc::c_char,
                path,
            );
            lockfd = client_get_lock(lockfile);
            if lockfd < 0 as libc::c_int {
                log_debug(
                    b"didn't get lock (%d)\0" as *const u8 as *const libc::c_char,
                    lockfd,
                );
                free(lockfile as *mut libc::c_void);
                lockfile = 0 as *mut libc::c_char;
                if lockfd == -(2 as libc::c_int) {
                    continue;
                }
            }
            log_debug(b"got lock (%d)\0" as *const u8 as *const libc::c_char, lockfd);
            locked = 1 as libc::c_int;
        } else {
            if lockfd >= 0 as libc::c_int && unlink(path) != 0 as libc::c_int
                && *__errno_location() != 2 as libc::c_int
            {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
                return -(1 as libc::c_int);
            }
            fd = server_start(client_proc, base, lockfd, lockfile);
            current_block = 17478428563724192186;
            break;
        }
    }
    match current_block {
        18072425028949421867 => {
            if locked != 0 {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
            }
            close(fd);
            return -(1 as libc::c_int);
        }
        _ => {
            if locked != 0 && lockfd >= 0 as libc::c_int {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
            }
            setblocking(fd, 0 as libc::c_int);
            return fd;
        }
    };
}
unsafe extern "C" fn client_exit_message() -> *const libc::c_char {
    static mut msg: [libc::c_char; 256] = [0; 256];
    match client_exitreason as libc::c_uint {
        1 => {
            if !client_exitsession.is_null() {
                xsnprintf(
                    msg.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"detached (from session %s)\0" as *const u8 as *const libc::c_char,
                    client_exitsession,
                );
                return msg.as_mut_ptr();
            }
            return b"detached\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            if !client_exitsession.is_null() {
                xsnprintf(
                    msg.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"detached and SIGHUP (from session %s)\0" as *const u8
                        as *const libc::c_char,
                    client_exitsession,
                );
                return msg.as_mut_ptr();
            }
            return b"detached and SIGHUP\0" as *const u8 as *const libc::c_char;
        }
        3 => return b"lost tty\0" as *const u8 as *const libc::c_char,
        4 => return b"terminated\0" as *const u8 as *const libc::c_char,
        5 => return b"lost server\0" as *const u8 as *const libc::c_char,
        6 => return b"exited\0" as *const u8 as *const libc::c_char,
        7 => return b"server exited\0" as *const u8 as *const libc::c_char,
        0 | _ => {}
    }
    return b"unknown reason\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn client_main(
    mut base: *mut event_base,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut data: *mut msg_command_data = 0 as *mut msg_command_data;
    let mut cmdflags: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ttynam: *const libc::c_char = 0 as *const libc::c_char;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut ppid: pid_t = 0;
    let mut msg: msgtype = 0 as msgtype;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut tio: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut saved_tio: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut size: size_t = 0;
    signal(
        17 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    client_flags = flags;
    cmdflags = 0 as libc::c_int;
    if !shell_command.is_null() {
        msg = MSG_SHELL;
        cmdflags = 0x1 as libc::c_int;
    } else if argc == 0 as libc::c_int {
        msg = MSG_COMMAND;
        cmdflags = 0x1 as libc::c_int;
    } else {
        msg = MSG_COMMAND;
        cmdlist = cmd_list_parse(
            argc,
            argv,
            0 as *const libc::c_char,
            0 as libc::c_int as u_int,
            &mut cause,
        );
        if !cmdlist.is_null() {
            cmd = (*cmdlist).list.tqh_first;
            while !cmd.is_null() {
                if (*(*cmd).entry).flags & 0x1 as libc::c_int != 0 {
                    cmdflags |= 0x1 as libc::c_int;
                }
                cmd = (*cmd).qentry.tqe_next;
            }
            cmd_list_free(cmdlist);
        }
    }
    client_proc = proc_start(b"client\0" as *const u8 as *const libc::c_char);
    proc_set_signals(
        client_proc,
        Some(client_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    fd = client_connect(base, socket_path, cmdflags & 0x1 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        if *__errno_location() == 111 as libc::c_int {
            fprintf(
                stderr,
                b"no server running on %s\n\0" as *const u8 as *const libc::c_char,
                socket_path,
            );
        } else {
            fprintf(
                stderr,
                b"error connecting to %s (%s)\n\0" as *const u8 as *const libc::c_char,
                socket_path,
                strerror(*__errno_location()),
            );
        }
        return 1 as libc::c_int;
    }
    client_peer = proc_add_peer(
        client_proc,
        fd,
        Some(
            client_dispatch as unsafe extern "C" fn(*mut imsg, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
    cwd = getenv(b"PWD\0" as *const u8 as *const libc::c_char);
    if cwd.is_null()
        && {
            cwd = getcwd(
                path.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            );
            cwd.is_null()
        }
        && {
            cwd = find_home();
            cwd.is_null()
        }
    {
        cwd = b"/\0" as *const u8 as *const libc::c_char;
    }
    ttynam = ttyname(0 as libc::c_int);
    if ttynam.is_null() {
        ttynam = b"\0" as *const u8 as *const libc::c_char;
    }
    if 0 as libc::c_int != 0 as libc::c_int {
        fatal(b"pledge failed\0" as *const u8 as *const libc::c_char);
    }
    if ptm_fd != -(1 as libc::c_int) {
        close(ptm_fd);
    }
    options_free(global_options);
    options_free(global_s_options);
    options_free(global_w_options);
    environ_free(global_environ);
    setblocking(0 as libc::c_int, 0 as libc::c_int);
    event_set(
        &mut client_stdin,
        0 as libc::c_int,
        (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            client_stdin_callback
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    if client_flags & 0x4000 as libc::c_int != 0 {
        if tcgetattr(0 as libc::c_int, &mut saved_tio) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"tcgetattr failed: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 1 as libc::c_int;
        }
        cfmakeraw(&mut tio);
        tio.c_iflag = (0o400 as libc::c_int | 0o4000 as libc::c_int) as tcflag_t;
        tio.c_oflag = (0o1 as libc::c_int | 0o4 as libc::c_int) as tcflag_t;
        tio
            .c_cflag = (0o200 as libc::c_int | 0o60 as libc::c_int
            | 0o2000 as libc::c_int) as tcflag_t;
        tio.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
        tio.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
        cfsetispeed(&mut tio, cfgetispeed(&mut saved_tio));
        cfsetospeed(&mut tio, cfgetospeed(&mut saved_tio));
        tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut tio);
    }
    client_send_identify(ttynam, cwd);
    if msg as libc::c_uint == MSG_COMMAND as libc::c_int as libc::c_uint {
        size = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int;
        while i < argc {
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (strlen(*argv.offset(i as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            i += 1;
            i;
        }
        if size
            > (16384 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    ::core::mem::size_of::<msg_command_data>() as libc::c_ulong,
                )
        {
            fprintf(stderr, b"command too long\n\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
        data = xmalloc(
            (::core::mem::size_of::<msg_command_data>() as libc::c_ulong)
                .wrapping_add(size),
        ) as *mut msg_command_data;
        (*data).argc = argc;
        if cmd_pack_argv(
            argc,
            argv,
            data.offset(1 as libc::c_int as isize) as *mut libc::c_char,
            size,
        ) != 0 as libc::c_int
        {
            fprintf(stderr, b"command too long\n\0" as *const u8 as *const libc::c_char);
            free(data as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        size = (size as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<msg_command_data>() as libc::c_ulong)
            as size_t as size_t;
        if proc_send(
            client_peer,
            msg,
            -(1 as libc::c_int),
            data as *const libc::c_void,
            size,
        ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"failed to send command\n\0" as *const u8 as *const libc::c_char,
            );
            free(data as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        free(data as *mut libc::c_void);
    } else if msg as libc::c_uint == MSG_SHELL as libc::c_int as libc::c_uint {
        proc_send(
            client_peer,
            msg,
            -(1 as libc::c_int),
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        );
    }
    proc_loop(client_proc, None);
    if client_exittype as libc::c_uint == MSG_EXEC as libc::c_int as libc::c_uint {
        if client_flags & 0x4000 as libc::c_int != 0 {
            tcsetattr(1 as libc::c_int, 2 as libc::c_int, &mut saved_tio);
        }
        client_exec(client_execshell, client_execcmd);
    }
    if client_attached != 0 {
        if client_exitreason as libc::c_uint
            != CLIENT_EXIT_NONE as libc::c_int as libc::c_uint
        {
            printf(
                b"[%s]\n\0" as *const u8 as *const libc::c_char,
                client_exit_message(),
            );
        }
        ppid = getppid();
        if client_exittype as libc::c_uint
            == MSG_DETACHKILL as libc::c_int as libc::c_uint && ppid > 1 as libc::c_int
        {
            kill(ppid, 1 as libc::c_int);
        }
    } else if client_flags & 0x4000 as libc::c_int != 0 {
        if client_exitreason as libc::c_uint
            != CLIENT_EXIT_NONE as libc::c_int as libc::c_uint
        {
            printf(
                b"%%exit %s\n\0" as *const u8 as *const libc::c_char,
                client_exit_message(),
            );
        } else {
            printf(b"%%exit\n\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\x1B\\\0" as *const u8 as *const libc::c_char);
        tcsetattr(1 as libc::c_int, 2 as libc::c_int, &mut saved_tio);
    } else if client_exitreason as libc::c_uint
        != CLIENT_EXIT_NONE as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            client_exit_message(),
        );
    }
    setblocking(0 as libc::c_int, 1 as libc::c_int);
    return client_exitval;
}
unsafe extern "C" fn client_send_identify(
    mut ttynam: *const libc::c_char,
    mut cwd: *const libc::c_char,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ss: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sslen: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = client_flags;
    let mut pid: pid_t = 0;
    proc_send(
        client_peer,
        MSG_IDENTIFY_FLAGS,
        -(1 as libc::c_int),
        &mut flags as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    s = getenv(b"TERM\0" as *const u8 as *const libc::c_char);
    if s.is_null() {
        s = b"\0" as *const u8 as *const libc::c_char;
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_TERM,
        -(1 as libc::c_int),
        s as *const libc::c_void,
        (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    proc_send(
        client_peer,
        MSG_IDENTIFY_TTYNAME,
        -(1 as libc::c_int),
        ttynam as *const libc::c_void,
        (strlen(ttynam)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    proc_send(
        client_peer,
        MSG_IDENTIFY_CWD,
        -(1 as libc::c_int),
        cwd as *const libc::c_void,
        (strlen(cwd)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    fd = dup(0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        fatal(b"dup failed\0" as *const u8 as *const libc::c_char);
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_STDIN,
        fd,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    pid = getpid();
    proc_send(
        client_peer,
        MSG_IDENTIFY_CLIENTPID,
        -(1 as libc::c_int),
        &mut pid as *mut pid_t as *const libc::c_void,
        ::core::mem::size_of::<pid_t>() as libc::c_ulong,
    );
    ss = environ;
    while !(*ss).is_null() {
        sslen = (strlen(*ss)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if !(sslen
            > (16384 as libc::c_int as libc::c_ulong)
                .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong))
        {
            proc_send(
                client_peer,
                MSG_IDENTIFY_ENVIRON,
                -(1 as libc::c_int),
                *ss as *const libc::c_void,
                sslen,
            );
        }
        ss = ss.offset(1);
        ss;
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_DONE,
        -(1 as libc::c_int),
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn client_stdin_callback(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut data: msg_stdin_data = msg_stdin_data {
        size: 0,
        data: [0; 8192],
    };
    data
        .size = read(
        0 as libc::c_int,
        (data.data).as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    );
    if data.size < 0 as libc::c_int as ssize_t
        && (*__errno_location() == 4 as libc::c_int
            || *__errno_location() == 11 as libc::c_int)
    {
        return;
    }
    proc_send(
        client_peer,
        MSG_STDIN,
        -(1 as libc::c_int),
        &mut data as *mut msg_stdin_data as *const libc::c_void,
        ::core::mem::size_of::<msg_stdin_data>() as libc::c_ulong,
    );
    if data.size <= 0 as libc::c_int as ssize_t {
        event_del(&mut client_stdin);
    }
}
unsafe extern "C" fn client_write(
    mut fd: libc::c_int,
    mut data: *const libc::c_char,
    mut size: size_t,
) {
    let mut used: ssize_t = 0;
    log_debug(
        b"%s: %.*s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"client_write\0"))
            .as_ptr(),
        size as libc::c_int,
        data,
    );
    while size != 0 as libc::c_int as size_t {
        used = write(fd, data as *const libc::c_void, size);
        if used == -(1 as libc::c_int) as ssize_t {
            if !(*__errno_location() == 4 as libc::c_int
                || *__errno_location() == 11 as libc::c_int)
            {
                break;
            }
        } else {
            data = data.offset(used as isize);
            size = size.wrapping_sub(used as size_t);
        }
    }
}
unsafe extern "C" fn client_exec(
    mut shell: *const libc::c_char,
    mut shellcmd: *const libc::c_char,
) -> ! {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut argv0: *mut libc::c_char = 0 as *mut libc::c_char;
    log_debug(
        b"shell %s, command %s\0" as *const u8 as *const libc::c_char,
        shell,
        shellcmd,
    );
    ptr = strrchr(shell, '/' as i32);
    if !ptr.is_null()
        && *ptr.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        name = ptr.offset(1 as libc::c_int as isize);
    } else {
        name = shell;
    }
    if client_flags & 0x2 as libc::c_int != 0 {
        xasprintf(
            &mut argv0 as *mut *mut libc::c_char,
            b"-%s\0" as *const u8 as *const libc::c_char,
            name,
        );
    } else {
        xasprintf(
            &mut argv0 as *mut *mut libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    setenv(b"SHELL\0" as *const u8 as *const libc::c_char, shell, 1 as libc::c_int);
    proc_clear_signals(client_proc, 1 as libc::c_int);
    setblocking(0 as libc::c_int, 1 as libc::c_int);
    setblocking(1 as libc::c_int, 1 as libc::c_int);
    setblocking(2 as libc::c_int, 1 as libc::c_int);
    closefrom(2 as libc::c_int + 1 as libc::c_int);
    execl(
        shell,
        argv0,
        b"-c\0" as *const u8 as *const libc::c_char,
        shellcmd,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    fatal(b"execl failed\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn client_signal(mut sig: libc::c_int) {
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut status: libc::c_int = 0;
    if sig == 17 as libc::c_int {
        waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int);
    } else if client_attached == 0 {
        if sig == 15 as libc::c_int {
            proc_exit(client_proc);
        }
    } else {
        match sig {
            1 => {
                client_exitreason = CLIENT_EXIT_LOST_TTY;
                client_exitval = 1 as libc::c_int;
                proc_send(
                    client_peer,
                    MSG_EXITING,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            15 => {
                client_exitreason = CLIENT_EXIT_TERMINATED;
                client_exitval = 1 as libc::c_int;
                proc_send(
                    client_peer,
                    MSG_EXITING,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            28 => {
                proc_send(
                    client_peer,
                    MSG_RESIZE,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            18 => {
                memset(
                    &mut sigact as *mut sigaction as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<sigaction>() as libc::c_ulong,
                );
                sigemptyset(&mut sigact.sa_mask);
                sigact.sa_flags = 0x10000000 as libc::c_int;
                sigact
                    .__sigaction_handler
                    .sa_handler = ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t);
                if sigaction(20 as libc::c_int, &mut sigact, 0 as *mut sigaction)
                    != 0 as libc::c_int
                {
                    fatal(b"sigaction failed\0" as *const u8 as *const libc::c_char);
                }
                proc_send(
                    client_peer,
                    MSG_WAKEUP,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn client_dispatch(mut imsg: *mut imsg, mut arg: *mut libc::c_void) {
    if imsg.is_null() {
        client_exitreason = CLIENT_EXIT_LOST_SERVER;
        client_exitval = 1 as libc::c_int;
        proc_exit(client_proc);
        return;
    }
    if client_attached != 0 {
        client_dispatch_attached(imsg);
    } else {
        client_dispatch_wait(imsg);
    };
}
unsafe extern "C" fn client_dispatch_wait(mut imsg: *mut imsg) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: ssize_t = 0;
    let mut stdoutdata: msg_stdout_data = msg_stdout_data {
        size: 0,
        data: [0; 8192],
    };
    let mut stderrdata: msg_stderr_data = msg_stderr_data {
        size: 0,
        data: [0; 8192],
    };
    let mut retval: libc::c_int = 0;
    static mut pledge_applied: libc::c_int = 0;
    if pledge_applied == 0 {
        if 0 as libc::c_int != 0 as libc::c_int {
            fatal(b"pledge failed\0" as *const u8 as *const libc::c_char);
        }
        pledge_applied = 1 as libc::c_int;
    }
    data = (*imsg).data as *mut libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        203 | 210 => {
            if datalen as libc::c_ulong
                != ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                && datalen != 0 as libc::c_int as ssize_t
            {
                fatalx(b"bad MSG_EXIT size\0" as *const u8 as *const libc::c_char);
            }
            if datalen as libc::c_ulong
                == ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                memcpy(
                    &mut retval as *mut libc::c_int as *mut libc::c_void,
                    data as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                );
                client_exitval = retval;
            }
            proc_exit(client_proc);
        }
        207 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_READY size\0" as *const u8 as *const libc::c_char);
            }
            event_del(&mut client_stdin);
            client_attached = 1 as libc::c_int;
            proc_send(
                client_peer,
                MSG_RESIZE,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        212 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_STDIN size\0" as *const u8 as *const libc::c_char);
            }
            event_add(&mut client_stdin, 0 as *const timeval);
        }
        213 => {
            if datalen as libc::c_ulong
                != ::core::mem::size_of::<msg_stdout_data>() as libc::c_ulong
            {
                fatalx(b"bad MSG_STDOUT size\0" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut stdoutdata as *mut msg_stdout_data as *mut libc::c_void,
                data as *const libc::c_void,
                ::core::mem::size_of::<msg_stdout_data>() as libc::c_ulong,
            );
            client_write(
                1 as libc::c_int,
                (stdoutdata.data).as_mut_ptr(),
                stdoutdata.size as size_t,
            );
        }
        211 => {
            if datalen as libc::c_ulong
                != ::core::mem::size_of::<msg_stderr_data>() as libc::c_ulong
            {
                fatalx(b"bad MSG_STDERR size\0" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut stderrdata as *mut msg_stderr_data as *mut libc::c_void,
                data as *const libc::c_void,
                ::core::mem::size_of::<msg_stderr_data>() as libc::c_ulong,
            );
            client_write(
                2 as libc::c_int,
                (stderrdata.data).as_mut_ptr(),
                stderrdata.size as size_t,
            );
        }
        12 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_VERSION size\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                stderr,
                b"protocol version mismatch (client %d, server %u)\n\0" as *const u8
                    as *const libc::c_char,
                8 as libc::c_int,
                (*imsg).hdr.peerid & 0xff as libc::c_int as uint32_t,
            );
            client_exitval = 1 as libc::c_int;
            proc_exit(client_proc);
        }
        209 => {
            if datalen == 0 as libc::c_int as ssize_t
                || *data.offset((datalen - 1 as libc::c_int as ssize_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(b"bad MSG_SHELL string\0" as *const u8 as *const libc::c_char);
            }
            client_exec(data, shell_command);
        }
        201 | 202 => {
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        204 => {
            proc_exit(client_proc);
        }
        _ => {}
    };
}
unsafe extern "C" fn client_dispatch_attached(mut imsg: *mut imsg) {
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: ssize_t = 0;
    data = (*imsg).data as *mut libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        201 | 202 => {
            if datalen == 0 as libc::c_int as ssize_t
                || *data.offset((datalen - 1 as libc::c_int as ssize_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(b"bad MSG_DETACH string\0" as *const u8 as *const libc::c_char);
            }
            client_exitsession = xstrdup(data);
            client_exittype = (*imsg).hdr.type_0 as msgtype;
            if (*imsg).hdr.type_0 == MSG_DETACHKILL as libc::c_int as uint32_t {
                client_exitreason = CLIENT_EXIT_DETACHED_HUP;
            } else {
                client_exitreason = CLIENT_EXIT_DETACHED;
            }
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        217 => {
            if datalen == 0 as libc::c_int as ssize_t
                || *data.offset((datalen - 1 as libc::c_int as ssize_t) as isize)
                    as libc::c_int != '\0' as i32
                || (strlen(data)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == datalen as size_t
            {
                fatalx(b"bad MSG_EXEC string\0" as *const u8 as *const libc::c_char);
            }
            client_execcmd = xstrdup(data);
            client_execshell = xstrdup(
                data.offset(strlen(data) as isize).offset(1 as libc::c_int as isize),
            );
            client_exittype = (*imsg).hdr.type_0 as msgtype;
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        203 => {
            if datalen != 0 as libc::c_int as ssize_t
                && datalen as libc::c_ulong
                    != ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                fatalx(b"bad MSG_EXIT size\0" as *const u8 as *const libc::c_char);
            }
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
            client_exitreason = CLIENT_EXIT_EXITED;
        }
        204 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_EXITED size\0" as *const u8 as *const libc::c_char);
            }
            proc_exit(client_proc);
        }
        210 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_SHUTDOWN size\0" as *const u8 as *const libc::c_char);
            }
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
            client_exitreason = CLIENT_EXIT_SERVER_EXITED;
            client_exitval = 1 as libc::c_int;
        }
        214 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_SUSPEND size\0" as *const u8 as *const libc::c_char);
            }
            memset(
                &mut sigact as *mut sigaction as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<sigaction>() as libc::c_ulong,
            );
            sigemptyset(&mut sigact.sa_mask);
            sigact.sa_flags = 0x10000000 as libc::c_int;
            sigact.__sigaction_handler.sa_handler = None;
            if sigaction(20 as libc::c_int, &mut sigact, 0 as *mut sigaction)
                != 0 as libc::c_int
            {
                fatal(b"sigaction failed\0" as *const u8 as *const libc::c_char);
            }
            kill(getpid(), 20 as libc::c_int);
        }
        206 => {
            if datalen == 0 as libc::c_int as ssize_t
                || *data.offset((datalen - 1 as libc::c_int as ssize_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(b"bad MSG_LOCK string\0" as *const u8 as *const libc::c_char);
            }
            system(data);
            proc_send(
                client_peer,
                MSG_UNLOCK,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        _ => {}
    };
}
