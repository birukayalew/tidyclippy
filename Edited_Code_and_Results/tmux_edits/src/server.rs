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
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn event_reinit(base: *mut event_base) -> libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_initialized(ev: *const event) -> libc::c_int;
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
    fn exit(_: libc::c_int) -> !;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    static mut start_time: timeval;
    static mut socket_path: *const libc::c_char;
    fn setblocking(_: libc::c_int, _: libc::c_int);
    fn proc_send(
        _: *mut tmuxpeer,
        _: msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    fn proc_start(_: *const libc::c_char) -> *mut tmuxproc;
    fn proc_loop(_: *mut tmuxproc, _: Option::<unsafe extern "C" fn() -> libc::c_int>);
    fn proc_set_signals(
        _: *mut tmuxproc,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    );
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int);
    fn proc_toggle_log(_: *mut tmuxproc);
    fn start_cfg();
    fn notify_client(_: *const libc::c_char, _: *mut client);
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    static mut all_jobs: joblist;
    fn job_died(_: *mut job, _: libc::c_int);
    fn tty_create_log();
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int);
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut cmdq_item;
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn cmdq_next(_: *mut client) -> u_int;
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn cmd_wait_for_flush();
    fn key_bindings_init();
    fn server_client_create(_: libc::c_int) -> *mut client;
    fn server_client_lost(_: *mut client);
    fn server_client_loop();
    fn server_destroy_pane(_: *mut window_pane, _: libc::c_int);
    fn status_prompt_save_history();
    static mut windows: windows;
    static mut all_window_panes: window_pane_tree;
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    fn window_pane_destroy_ready(_: *mut window_pane) -> libc::c_int;
    static mut sessions: sessions;
    static mut session_groups: session_groups;
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    fn session_destroy(_: *mut session, _: *const libc::c_char);
    fn log_get_level() -> libc::c_int;
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
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
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
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
    pub message_log: C2RustUnnamed_25,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_24,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub gentry: C2RustUnnamed_11,
    pub entry: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
    pub entry: C2RustUnnamed_14,
    pub wentry: C2RustUnnamed_13,
    pub sentry: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub alerts_entry: C2RustUnnamed_17,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_16,
    pub entry: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
    pub entry: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
    pub entry: C2RustUnnamed_20,
    pub tree_entry: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
    pub lineflag: C2RustUnnamed_21,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type C2RustUnnamed_21 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed_21 = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed_21 = 1;
pub const LINE_SEL_NONE: C2RustUnnamed_21 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub offset: u_int,
    pub data: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
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
pub type C2RustUnnamed_24 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_24 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_24 = 0;
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
pub struct C2RustUnnamed_25 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
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
    pub qentry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_32,
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
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
pub struct C2RustUnnamed_32 {
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
    pub term_type: C2RustUnnamed_33,
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
pub type C2RustUnnamed_33 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_33 = 6;
pub const TTY_VT420: C2RustUnnamed_33 = 5;
pub const TTY_VT320: C2RustUnnamed_33 = 4;
pub const TTY_VT220: C2RustUnnamed_33 = 3;
pub const TTY_VT102: C2RustUnnamed_33 = 2;
pub const TTY_VT101: C2RustUnnamed_33 = 1;
pub const TTY_VT100: C2RustUnnamed_33 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
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
pub struct job {
    pub state: C2RustUnnamed_36,
    pub flags: libc::c_int,
    pub cmd: *mut libc::c_char,
    pub pid: pid_t,
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub updatecb: job_update_cb,
    pub completecb: job_complete_cb,
    pub freecb: job_free_cb,
    pub data: *mut libc::c_void,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type job_free_cb = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type job_complete_cb = Option::<unsafe extern "C" fn(*mut job) -> ()>;
pub type job_update_cb = Option::<unsafe extern "C" fn(*mut job) -> ()>;
pub type C2RustUnnamed_36 = libc::c_uint;
pub const JOB_CLOSED: C2RustUnnamed_36 = 2;
pub const JOB_DEAD: C2RustUnnamed_36 = 1;
pub const JOB_RUNNING: C2RustUnnamed_36 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct joblist {
    pub lh_first: *mut job,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_38,
    pub entry: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[no_mangle]
pub static mut clients: clients = clients {
    tqh_first: 0 as *const client as *mut client,
    tqh_last: 0 as *const *mut client as *mut *mut client,
};
#[no_mangle]
pub static mut server_proc: *mut tmuxproc = 0 as *const tmuxproc as *mut tmuxproc;
static mut server_fd: libc::c_int = 0;
static mut server_exit: libc::c_int = 0;
static mut server_ev_accept: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: 0 as *const event_callback as *mut event_callback,
            tqe_prev: 0 as *const *mut event_callback as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: 0 as *const libc::c_void as *mut libc::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: 0 as *const event as *mut event,
            tqe_prev: 0 as *const *mut event as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: 0 as *const event_base as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
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
#[no_mangle]
pub static mut marked_pane: cmd_find_state = cmd_find_state {
    flags: 0,
    current: 0 as *const cmd_find_state as *mut cmd_find_state,
    s: 0 as *const session as *mut session,
    wl: 0 as *const winlink as *mut winlink,
    w: 0 as *const window as *mut window,
    wp: 0 as *const window_pane as *mut window_pane,
    idx: 0,
};
#[no_mangle]
pub unsafe extern "C" fn server_set_marked(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) {
    cmd_find_clear_state(&mut marked_pane, 0 as libc::c_int);
    marked_pane.s = s;
    marked_pane.wl = wl;
    marked_pane.w = (*wl).window;
    marked_pane.wp = wp;
}
#[no_mangle]
pub unsafe extern "C" fn server_clear_marked() {
    cmd_find_clear_state(&mut marked_pane, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn server_is_marked(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) -> libc::c_int {
    if s.is_null() || wl.is_null() || wp.is_null() {
        return 0 as libc::c_int;
    }
    if marked_pane.s != s || marked_pane.wl != wl {
        return 0 as libc::c_int;
    }
    if marked_pane.wp != wp {
        return 0 as libc::c_int;
    }
    return server_check_marked();
}
#[no_mangle]
pub unsafe extern "C" fn server_check_marked() -> libc::c_int {
    return cmd_find_valid_state(&mut marked_pane);
}
unsafe extern "C" fn server_create_socket(
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut size: size_t = 0;
    let mut mask: mode_t = 0;
    let mut fd: libc::c_int = 0;
    let mut saved_errno: libc::c_int = 0;
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    size = strlcpy(
        (sa.sun_path).as_mut_ptr(),
        socket_path,
        ::core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    if size >= ::core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
    } else {
        unlink((sa.sun_path).as_mut_ptr());
        fd = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
        if !(fd == -(1 as libc::c_int)) {
            mask = umask(
                (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as __mode_t,
            );
            if bind(
                fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut sa as *mut sockaddr_un as *mut sockaddr,
                },
                ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
            ) == -(1 as libc::c_int)
            {
                saved_errno = *__errno_location();
                close(fd);
                *__errno_location() = saved_errno;
            } else {
                umask(mask);
                if listen(fd, 128 as libc::c_int) == -(1 as libc::c_int) {
                    saved_errno = *__errno_location();
                    close(fd);
                    *__errno_location() = saved_errno;
                } else {
                    setblocking(fd, 0 as libc::c_int);
                    return fd;
                }
            }
        }
    }
    if !cause.is_null() {
        xasprintf(
            cause,
            b"error creating %s (%s)\0" as *const u8 as *const libc::c_char,
            socket_path,
            strerror(*__errno_location()),
        );
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn server_start_error(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    let mut error: *mut libc::c_char = data as *mut libc::c_char;
    cmdq_error(item, b"%s\0" as *const u8 as *const libc::c_char, error);
    free(error as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn server_start(
    mut client: *mut tmuxproc,
    mut base: *mut event_base,
    mut lockfd: libc::c_int,
    mut lockfile: *mut libc::c_char,
) -> libc::c_int {
    let mut pair: [libc::c_int; 2] = [0; 2];
    let mut job: *mut job = 0 as *mut job;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oldset: sigset_t = __sigset_t { __val: [0; 16] };
    let mut c: *mut client = 0 as *mut client;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if socketpair(
        1 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
        pair.as_mut_ptr(),
    ) != 0 as libc::c_int
    {
        fatal(b"socketpair failed\0" as *const u8 as *const libc::c_char);
    }
    sigfillset(&mut set);
    sigprocmask(0 as libc::c_int, &mut set, &mut oldset);
    match fork() {
        -1 => {
            fatal(b"fork failed\0" as *const u8 as *const libc::c_char);
        }
        0 => {}
        _ => {
            sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
            close(pair[1 as libc::c_int as usize]);
            return pair[0 as libc::c_int as usize];
        }
    }
    close(pair[0 as libc::c_int as usize]);
    if daemon(1 as libc::c_int, 0 as libc::c_int) != 0 as libc::c_int {
        fatal(b"daemon failed\0" as *const u8 as *const libc::c_char);
    }
    proc_clear_signals(client, 0 as libc::c_int);
    if event_reinit(base) != 0 as libc::c_int {
        fatalx(b"event_reinit failed\0" as *const u8 as *const libc::c_char);
    }
    server_proc = proc_start(b"server\0" as *const u8 as *const libc::c_char);
    proc_set_signals(
        server_proc,
        Some(server_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    if log_get_level() > 1 as libc::c_int {
        tty_create_log();
    }
    // if 0 as libc::c_int != 0 as libc::c_int {
    //     fatal(b"pledge failed\0" as *const u8 as *const libc::c_char);
    // }
    windows.rbh_root = 0 as *mut window;
    all_window_panes.rbh_root = 0 as *mut window_pane;
    clients.tqh_first = 0 as *mut client;
    clients.tqh_last = &mut clients.tqh_first;
    sessions.rbh_root = 0 as *mut session;
    session_groups.rbh_root = 0 as *mut session_group;
    key_bindings_init();
    gettimeofday(&mut start_time, 0 as *mut libc::c_void);
    server_fd = server_create_socket(&mut cause);
    if server_fd != -(1 as libc::c_int) {
        server_update_socket();
    }
    c = server_client_create(pair[1 as libc::c_int as usize]);
    if lockfd >= 0 as libc::c_int {
        unlink(lockfile);
        free(lockfile as *mut libc::c_void);
        close(lockfd);
    }
    if !cause.is_null() {
        cmdq_append(
            c,
            cmdq_get_callback1(
                b"server_start_error\0" as *const u8 as *const libc::c_char,
                Some(
                    server_start_error
                        as unsafe extern "C" fn(
                            *mut cmdq_item,
                            *mut libc::c_void,
                        ) -> cmd_retval,
                ),
                cause as *mut libc::c_void,
            ),
        );
        (*c).flags |= 0x4 as libc::c_int;
    }
    start_cfg();
    server_add_accept(0 as libc::c_int);
    proc_loop(server_proc, Some(server_loop as unsafe extern "C" fn() -> libc::c_int));
    job = all_jobs.lh_first;
    while !job.is_null() {
        if (*job).pid != -(1 as libc::c_int) {
            kill((*job).pid, 15 as libc::c_int);
        }
        job = (*job).entry.le_next;
    }
    status_prompt_save_history();
    exit(0 as libc::c_int);
}
unsafe extern "C" fn server_loop() -> libc::c_int {
    let mut c: *mut client = 0 as *mut client;
    let mut items: u_int = 0;
    let mut job: *mut job = 0 as *mut job;
    loop {
        items = cmdq_next(0 as *mut client);
        c = clients.tqh_first;
        while !c.is_null() {
            if (*c).flags & 0x40000 as libc::c_int != 0 {
                items = items.wrapping_add(cmdq_next(c));
            }
            c = (*c).entry.tqe_next;
        }
        if !(items != 0 as libc::c_int as u_int) {
            break;
        }
    }
    server_client_loop();
    if options_get_number(
        global_options,
        b"exit-empty\0" as *const u8 as *const libc::c_char,
    ) == 0 && server_exit == 0
    {
        return 0 as libc::c_int;
    }
    if options_get_number(
        global_options,
        b"exit-unattached\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if !(sessions.rbh_root).is_null() {
            return 0 as libc::c_int;
        }
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session).is_null() {
            return 0 as libc::c_int;
        }
        c = (*c).entry.tqe_next;
    }
    cmd_wait_for_flush();
    if !(clients.tqh_first).is_null() {
        return 0 as libc::c_int;
    }
    job = all_jobs.lh_first;
    while !job.is_null() {
        if !(*job).flags & 0x1 as libc::c_int != 0
            && (*job).state as libc::c_uint == JOB_RUNNING as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        job = (*job).entry.le_next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn server_send_exit() {
    let mut c: *mut client = 0 as *mut client;
    let mut c1: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut s1: *mut session = 0 as *mut session;
    cmd_wait_for_flush();
    c = clients.tqh_first;
    while !c.is_null()
        && {
            c1 = (*c).entry.tqe_next;
            1 as libc::c_int != 0
        }
    {
        if (*c).flags & 0x40 as libc::c_int != 0 {
            server_client_lost(c);
        } else {
            if (*c).flags & 0x80 as libc::c_int != 0 {
                notify_client(
                    b"client-detached\0" as *const u8 as *const libc::c_char,
                    c,
                );
            }
            proc_send(
                (*c).peer,
                MSG_SHUTDOWN,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        (*c).session = 0 as *mut session;
        c = c1;
    }
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null()
        && {
            s1 = sessions_RB_NEXT(s);
            1 as libc::c_int != 0
        }
    {
        session_destroy(
            s,
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"server_send_exit\0"))
                .as_ptr(),
        );
        s = s1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_update_socket() {
    let mut s: *mut session = 0 as *mut session;
    static mut last: libc::c_int = -(1 as libc::c_int);
    let mut n: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    n = 0 as libc::c_int;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        if (*s).flags & 0x1 as libc::c_int == 0 {
            n += 1;
            n;
            break;
        } else {
            s = sessions_RB_NEXT(s);
        }
    }
    if n != last {
        last = n;
        if stat(socket_path, &mut sb) != 0 as libc::c_int {
            return;
        }
        mode = (sb.st_mode
            & (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t) as libc::c_int;
        if n != 0 as libc::c_int {
            if mode & 0o400 as libc::c_int != 0 {
                mode |= 0o100 as libc::c_int;
            }
            if mode & 0o400 as libc::c_int >> 3 as libc::c_int != 0 {
                mode |= 0o100 as libc::c_int >> 3 as libc::c_int;
            }
            if mode & 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int != 0 {
                mode |= 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int;
            }
        } else {
            mode
                &= !(0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int);
        }
        chmod(socket_path, mode as __mode_t);
    }
}
unsafe extern "C" fn server_accept(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut slen: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as libc::c_ulong
        as socklen_t;
    let mut newfd: libc::c_int = 0;
    server_add_accept(0 as libc::c_int);
    if events as libc::c_int & 0x2 as libc::c_int == 0 {
        return;
    }
    newfd = accept(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut sa as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut slen,
    );
    if newfd == -(1 as libc::c_int) {
        if *__errno_location() == 11 as libc::c_int
            || *__errno_location() == 4 as libc::c_int
            || *__errno_location() == 103 as libc::c_int
        {
            return;
        }
        if *__errno_location() == 23 as libc::c_int
            || *__errno_location() == 24 as libc::c_int
        {
            server_add_accept(1 as libc::c_int);
            return;
        }
        fatal(b"accept failed\0" as *const u8 as *const libc::c_char);
    }
    if server_exit != 0 {
        close(newfd);
        return;
    }
    server_client_create(newfd);
}
#[no_mangle]
pub unsafe extern "C" fn server_add_accept(mut timeout: libc::c_int) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: timeout as __time_t,
            tv_usec: 0 as libc::c_int as __suseconds_t,
        };
        init
    };
    if event_initialized(&mut server_ev_accept) != 0 {
        event_del(&mut server_ev_accept);
    }
    if timeout == 0 as libc::c_int {
        event_set(
            &mut server_ev_accept,
            server_fd,
            0x2 as libc::c_int as libc::c_short,
            Some(
                server_accept
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        event_add(&mut server_ev_accept, 0 as *const timeval);
    } else {
        event_set(
            &mut server_ev_accept,
            server_fd,
            0x1 as libc::c_int as libc::c_short,
            Some(
                server_accept
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        event_add(&mut server_ev_accept, &mut tv);
    };
}
unsafe extern "C" fn server_signal(mut sig: libc::c_int) {
    let mut fd: libc::c_int = 0;
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"server_signal\0"))
            .as_ptr(),
        strsignal(sig),
    );
    match sig {
        15 => {
            server_exit = 1 as libc::c_int;
            server_send_exit();
        }
        17 => {
            server_child_signal();
        }
        10 => {
            event_del(&mut server_ev_accept);
            fd = server_create_socket(0 as *mut *mut libc::c_char);
            if fd != -(1 as libc::c_int) {
                close(server_fd);
                server_fd = fd;
                server_update_socket();
            }
            server_add_accept(0 as libc::c_int);
        }
        12 => {
            proc_toggle_log(server_proc);
        }
        _ => {}
    };
}
unsafe extern "C" fn server_child_signal() {
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    loop {
        pid = waitpid(
            -(1 as libc::c_int),
            &mut status,
            1 as libc::c_int | 2 as libc::c_int,
        );
        match pid {
            -1 => {
                if *__errno_location() == 10 as libc::c_int {
                    return;
                }
                fatal(b"waitpid failed\0" as *const u8 as *const libc::c_char);
            }
            0 => return,
            _ => {}
        }
        if status & 0xff as libc::c_int == 0x7f as libc::c_int {
            server_child_stopped(pid, status);
        } else if status & 0x7f as libc::c_int == 0 as libc::c_int
            || ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
                as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            server_child_exited(pid, status);
        }
    };
}
unsafe extern "C" fn server_child_exited(mut pid: pid_t, mut status: libc::c_int) {
    let mut w: *mut window = 0 as *mut window;
    let mut w1: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut job: *mut job = 0 as *mut job;
    w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
    while !w.is_null()
        && {
            w1 = windows_RB_NEXT(w);
            1 as libc::c_int != 0
        }
    {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).pid == pid {
                (*wp).status = status;
                (*wp).flags |= 0x200 as libc::c_int;
                log_debug(
                    b"%%%u exited\0" as *const u8 as *const libc::c_char,
                    (*wp).id,
                );
                (*wp).flags |= 0x100 as libc::c_int;
                if window_pane_destroy_ready(wp) != 0 {
                    server_destroy_pane(wp, 1 as libc::c_int);
                }
                break;
            } else {
                wp = (*wp).entry.tqe_next;
            }
        }
        w = w1;
    }
    job = all_jobs.lh_first;
    while !job.is_null() {
        if pid == (*job).pid {
            job_died(job, status);
            break;
        } else {
            job = (*job).entry.le_next;
        }
    }
}
unsafe extern "C" fn server_child_stopped(mut pid: pid_t, mut status: libc::c_int) {
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int == 21 as libc::c_int
        || (status & 0xff00 as libc::c_int) >> 8 as libc::c_int == 22 as libc::c_int
    {
        return;
    }
    w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
    while !w.is_null() {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).pid == pid {
                if killpg(pid, 18 as libc::c_int) != 0 as libc::c_int {
                    kill(pid, 18 as libc::c_int);
                }
            }
            wp = (*wp).entry.tqe_next;
        }
        w = windows_RB_NEXT(w);
    }
}
