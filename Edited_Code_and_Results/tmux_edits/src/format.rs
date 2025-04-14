use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type environ;
    pub type options;
    pub type hooks;
    pub type screen_titles;
    pub type input_ctx;
    pub type tty_code;
    pub type tmuxpeer;
    pub type options_entry;
    pub type paste_buffer;
    fn getpid() -> __pid_t;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char) -> *mut libc::c_char;
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
    fn evbuffer_new() -> *mut evbuffer;
    fn evbuffer_free(buf: *mut evbuffer);
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    fn evbuffer_add(
        buf: *mut evbuffer,
        data: *const libc::c_void,
        datlen: size_t,
    ) -> libc::c_int;
    fn evbuffer_add_printf(
        buf: *mut evbuffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    fn evbuffer_readline(buffer: *mut evbuffer) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
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
    static mut start_time: timeval;
    static mut socket_path: *const libc::c_char;
    fn paste_buffer_name(_: *mut paste_buffer) -> *const libc::c_char;
    fn paste_buffer_created(_: *mut paste_buffer) -> time_t;
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t) -> *const libc::c_char;
    fn paste_make_sample(_: *mut paste_buffer) -> *mut libc::c_char;
    fn format_add(
        _: *mut format_tree,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    fn options_tostring(
        _: *mut options_entry,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *const libc::c_char;
    fn options_parse_get(
        _: *mut options,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> *mut options_entry;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn job_run(
        _: *const libc::c_char,
        _: *mut session,
        _: *const libc::c_char,
        _: job_update_cb,
        _: job_complete_cb,
        _: job_free_cb,
        _: *mut libc::c_void,
        _: libc::c_int,
    ) -> *mut job;
    fn job_free(_: *mut job);
    fn environ_find(_: *mut environ, _: *const libc::c_char) -> *mut environ_entry;
    fn cmd_stringify_argv(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    static mut clients: clients;
    fn server_client_get_key_table(_: *mut client) -> *const libc::c_char;
    fn server_client_unref(_: *mut client);
    fn server_status_client(_: *mut client);
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    fn winlink_count(_: *mut winlinks) -> u_int;
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    fn window_count_panes(_: *mut window) -> u_int;
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    fn window_pane_search(_: *mut window_pane, _: *const libc::c_char) -> u_int;
    fn window_printable_flags(_: *mut winlink) -> *const libc::c_char;
    fn layout_dump(_: *mut layout_cell) -> *mut libc::c_char;
    fn window_copy_add_formats(_: *mut window_pane, _: *mut format_tree);
    fn parse_window_name(_: *const libc::c_char) -> *mut libc::c_char;
    fn session_alive(_: *mut session) -> libc::c_int;
    fn session_is_linked(_: *mut session, _: *mut window) -> libc::c_int;
    fn session_group_contains(_: *mut session) -> *mut session_group;
    fn session_group_count(_: *mut session_group) -> u_int;
    fn utf8_rtrimcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    fn utf8_trimcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    fn osdep_get_name(_: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char;
    fn osdep_get_cwd(_: libc::c_int) -> *mut libc::c_char;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uint8_t = __uint8_t;
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
    pub message_log: C2RustUnnamed_24,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_23,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub gentry: C2RustUnnamed_10,
    pub entry: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub entry: C2RustUnnamed_13,
    pub wentry: C2RustUnnamed_12,
    pub sentry: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
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
    pub alerts_entry: C2RustUnnamed_16,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_15,
    pub entry: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
    pub entry: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
    pub entry: C2RustUnnamed_19,
    pub tree_entry: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
    pub lineflag: C2RustUnnamed_20,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type C2RustUnnamed_20 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed_20 = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed_20 = 1;
pub const LINE_SEL_NONE: C2RustUnnamed_20 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub offset: u_int,
    pub data: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
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
pub type C2RustUnnamed_23 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_23 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_23 = 0;
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
pub struct C2RustUnnamed_24 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
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
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
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
    pub qentry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
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
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_tree {
    pub w: *mut window,
    pub wl: *mut winlink,
    pub s: *mut session,
    pub wp: *mut window_pane,
    pub client: *mut client,
    pub tag: u_int,
    pub flags: libc::c_int,
    pub tree: format_entry_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_entry_tree {
    pub rbh_root: *mut format_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_entry {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub t: time_t,
    pub cb: format_cb,
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub rbe_left: *mut format_entry,
    pub rbe_right: *mut format_entry,
    pub rbe_parent: *mut format_entry,
    pub rbe_color: libc::c_int,
}
pub type format_cb = Option::<
    unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_job_tree {
    pub rbh_root: *mut format_job,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_job {
    pub client: *mut client,
    pub tag: u_int,
    pub cmd: *const libc::c_char,
    pub expanded: *const libc::c_char,
    pub last: time_t,
    pub out: *mut libc::c_char,
    pub updated: libc::c_int,
    pub job: *mut job,
    pub status: libc::c_int,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub rbe_left: *mut format_job,
    pub rbe_right: *mut format_job,
    pub rbe_parent: *mut format_job,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub state: C2RustUnnamed_37,
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
    pub entry: C2RustUnnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type job_free_cb = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type job_complete_cb = Option::<unsafe extern "C" fn(*mut job) -> ()>;
pub type job_update_cb = Option::<unsafe extern "C" fn(*mut job) -> ()>;
pub type C2RustUnnamed_37 = libc::c_uint;
pub const JOB_CLOSED: C2RustUnnamed_37 = 2;
pub const JOB_DEAD: C2RustUnnamed_37 = 1;
pub const JOB_RUNNING: C2RustUnnamed_37 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_40,
    pub entry: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
static mut format_job_event: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_7 {
            tqe_next: 0 as *const event_callback as *mut event_callback,
            tqe_prev: 0 as *const *mut event_callback as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_6 {
            evcb_callback: None,
        },
        evcb_arg: 0 as *const libc::c_void as *mut libc::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_4 {
        ev_next_with_common_timeout: C2RustUnnamed_5 {
            tqe_next: 0 as *const event as *mut event,
            tqe_prev: 0 as *const *mut event as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: 0 as *const event_base as *mut event_base,
    ev_: C2RustUnnamed {
        ev_io: C2RustUnnamed_2 {
            ev_io_next: C2RustUnnamed_3 {
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
static mut format_jobs: format_job_tree = {
    let mut init = format_job_tree {
        rbh_root: 0 as *const format_job as *mut format_job,
    };
    init
};
unsafe extern "C" fn format_job_tree_RB_INSERT(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) -> *mut format_job {
    let mut tmp: *mut format_job = 0 as *mut format_job;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = format_job_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut format_job;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm;
        } else {
            (*parent).entry.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    format_job_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut format_job;
}
unsafe extern "C" fn format_job_tree_RB_INSERT_COLOR(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) {
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut gparent: *mut format_job = 0 as *mut format_job;
    let mut tmp: *mut format_job = 0 as *mut format_job;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).entry.rbe_right == elm {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !((*parent).entry.rbe_right).is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !((*tmp).entry.rbe_parent).is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !((*tmp).entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*gparent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent;
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).entry.rbe_left == elm {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !((*parent).entry.rbe_left).is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !((*tmp).entry.rbe_parent).is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !((*tmp).entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*gparent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent;
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn format_job_tree_RB_NEXT(
    mut elm: *mut format_job,
) -> *mut format_job {
    if !((*elm).entry.rbe_right).is_null() {
        elm = (*elm).entry.rbe_right;
        while !((*elm).entry.rbe_left).is_null() {
            elm = (*elm).entry.rbe_left;
        }
    } else if !((*elm).entry.rbe_parent).is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
    {
        elm = (*elm).entry.rbe_parent;
    } else {
        while !((*elm).entry.rbe_parent).is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
        {
            elm = (*elm).entry.rbe_parent;
        }
        elm = (*elm).entry.rbe_parent;
    }
    return elm;
}
unsafe extern "C" fn format_job_tree_RB_FIND(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = format_job_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut format_job;
}
unsafe extern "C" fn format_job_tree_RB_REMOVE(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) -> *mut format_job {
    let mut current_block: u64;
    let mut child: *mut format_job = 0 as *mut format_job;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut old: *mut format_job = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut format_job = 0 as *mut format_job;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child;
            } else {
                (*parent).entry.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm;
        }
        (*elm).entry = (*old).entry;
        if !((*old).entry.rbe_parent).is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm;
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !((*old).entry.rbe_right).is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 4275996524042205385;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child;
                } else {
                    (*parent).entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        format_job_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn format_job_tree_RB_REMOVE_COLOR(
    mut head: *mut format_job_tree,
    mut parent: *mut format_job,
    mut elm: *mut format_job,
) {
    let mut tmp: *mut format_job = 0 as *mut format_job;
    while (elm.is_null() || (*elm).entry.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*parent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                tmp = (*parent).entry.rbe_right;
            }
            if (((*tmp).entry.rbe_left).is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && (((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent;
            } else {
                if ((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oleft: *mut format_job = 0 as *mut format_job;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !((*tmp).entry.rbe_left).is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp;
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !((*oleft).entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft;
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !((*oleft).entry.rbe_parent).is_null();
                    tmp = (*parent).entry.rbe_right;
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*parent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*parent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                tmp = (*parent).entry.rbe_left;
            }
            if (((*tmp).entry.rbe_left).is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && (((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent;
            } else {
                if ((*tmp).entry.rbe_left).is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oright: *mut format_job = 0 as *mut format_job;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !((*tmp).entry.rbe_right).is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp;
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !((*oright).entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright;
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !((*oright).entry.rbe_parent).is_null();
                    tmp = (*parent).entry.rbe_left;
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*parent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0 as libc::c_int;
    }
}
unsafe extern "C" fn format_job_tree_RB_MINMAX(
    mut head: *mut format_job_tree,
    mut val: libc::c_int,
) -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut parent: *mut format_job = 0 as *mut format_job;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else {
            tmp = (*tmp).entry.rbe_right;
        }
    }
    return parent;
}
unsafe extern "C" fn format_job_cmp(
    mut fj1: *mut format_job,
    mut fj2: *mut format_job,
) -> libc::c_int {
    if (*fj1).tag < (*fj2).tag {
        return -(1 as libc::c_int);
    }
    if (*fj1).tag > (*fj2).tag {
        return 1 as libc::c_int;
    }
    return strcmp((*fj1).cmd, (*fj2).cmd);
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_FIND(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = format_entry_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut format_entry;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_REMOVE(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut current_block: u64;
    let mut child: *mut format_entry = 0 as *mut format_entry;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut old: *mut format_entry = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut format_entry = 0 as *mut format_entry;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child;
            } else {
                (*parent).entry.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm;
        }
        (*elm).entry = (*old).entry;
        if !((*old).entry.rbe_parent).is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm;
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !((*old).entry.rbe_right).is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 16377762561021665021;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child;
                } else {
                    (*parent).entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        format_entry_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_INSERT(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = format_entry_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut format_entry;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm;
        } else {
            (*parent).entry.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    format_entry_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut format_entry;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_NEXT(
    mut elm: *mut format_entry,
) -> *mut format_entry {
    if !((*elm).entry.rbe_right).is_null() {
        elm = (*elm).entry.rbe_right;
        while !((*elm).entry.rbe_left).is_null() {
            elm = (*elm).entry.rbe_left;
        }
    } else if !((*elm).entry.rbe_parent).is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
    {
        elm = (*elm).entry.rbe_parent;
    } else {
        while !((*elm).entry.rbe_parent).is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
        {
            elm = (*elm).entry.rbe_parent;
        }
        elm = (*elm).entry.rbe_parent;
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_MINMAX(
    mut head: *mut format_entry_tree,
    mut val: libc::c_int,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else {
            tmp = (*tmp).entry.rbe_right;
        }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_NFIND(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut res: *mut format_entry = 0 as *mut format_entry;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = format_entry_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            res = tmp;
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_REMOVE_COLOR(
    mut head: *mut format_entry_tree,
    mut parent: *mut format_entry,
    mut elm: *mut format_entry,
) {
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
    while (elm.is_null() || (*elm).entry.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*parent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                tmp = (*parent).entry.rbe_right;
            }
            if (((*tmp).entry.rbe_left).is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && (((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent;
            } else {
                if ((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oleft: *mut format_entry = 0 as *mut format_entry;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !((*tmp).entry.rbe_left).is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp;
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !((*oleft).entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft;
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !((*oleft).entry.rbe_parent).is_null();
                    tmp = (*parent).entry.rbe_right;
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*parent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*parent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                tmp = (*parent).entry.rbe_left;
            }
            if (((*tmp).entry.rbe_left).is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && (((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent;
            } else {
                if ((*tmp).entry.rbe_left).is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oright: *mut format_entry = 0 as *mut format_entry;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !((*tmp).entry.rbe_right).is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp;
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !((*oright).entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright;
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !((*oright).entry.rbe_parent).is_null();
                    tmp = (*parent).entry.rbe_left;
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*parent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_PREV(
    mut elm: *mut format_entry,
) -> *mut format_entry {
    if !((*elm).entry.rbe_left).is_null() {
        elm = (*elm).entry.rbe_left;
        while !((*elm).entry.rbe_right).is_null() {
            elm = (*elm).entry.rbe_right;
        }
    } else if !((*elm).entry.rbe_parent).is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
    {
        elm = (*elm).entry.rbe_parent;
    } else {
        while !((*elm).entry.rbe_parent).is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
        {
            elm = (*elm).entry.rbe_parent;
        }
        elm = (*elm).entry.rbe_parent;
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn format_entry_tree_RB_INSERT_COLOR(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) {
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut gparent: *mut format_entry = 0 as *mut format_entry;
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).entry.rbe_right == elm {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !((*parent).entry.rbe_right).is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !((*tmp).entry.rbe_parent).is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !((*tmp).entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*gparent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent;
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).entry.rbe_left == elm {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !((*parent).entry.rbe_left).is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !((*tmp).entry.rbe_parent).is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !((*tmp).entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*gparent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent;
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn format_entry_cmp(
    mut fe1: *mut format_entry,
    mut fe2: *mut format_entry,
) -> libc::c_int {
    return strcmp((*fe1).key, (*fe2).key);
}
static mut format_upper: [*const libc::c_char; 26] = [
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"pane_id\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"window_flags\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"host\0" as *const u8 as *const libc::c_char,
    b"window_index\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"pane_index\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"session_name\0" as *const u8 as *const libc::c_char,
    b"pane_title\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"window_name\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut format_lower: [*const libc::c_char; 26] = [
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"host_short\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn format_job_update(mut job: *mut job) {
    let mut fj: *mut format_job = (*job).data as *mut format_job;
    let mut evb: *mut evbuffer = (*(*job).event).input;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: time_t = 0;
    loop {
        next = evbuffer_readline(evb);
        if next.is_null() {
            break;
        }
        free(line as *mut libc::c_void);
        line = next;
    }
    if line.is_null() {
        return;
    }
    (*fj).updated = 1 as libc::c_int;
    free((*fj).out as *mut libc::c_void);
    (*fj).out = line;
    log_debug(
        b"%s: %p %s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"format_job_update\0"))
            .as_ptr(),
        fj,
        (*fj).cmd,
        (*fj).out,
    );
    t = time(0 as *mut time_t);
    if (*fj).status != 0 && (*fj).last != t {
        if !((*fj).client).is_null() {
            server_status_client((*fj).client);
        }
        (*fj).last = t;
    }
}
unsafe extern "C" fn format_job_complete(mut job: *mut job) {
    let mut fj: *mut format_job = (*job).data as *mut format_job;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    (*fj).job = 0 as *mut job;
    buf = 0 as *mut libc::c_char;
    line = evbuffer_readline((*(*job).event).input);
    if line.is_null() {
        len = evbuffer_get_length((*(*job).event).input);
        buf = xmalloc(len.wrapping_add(1 as libc::c_int as size_t)) as *mut libc::c_char;
        if len != 0 as libc::c_int as size_t {
            memcpy(
                buf as *mut libc::c_void,
                evbuffer_pullup((*(*job).event).input, -(1 as libc::c_int) as ssize_t)
                    as *const libc::c_void,
                len,
            );
        }
        *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    } else {
        buf = line;
    }
    log_debug(
        b"%s: %p %s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"format_job_complete\0"))
            .as_ptr(),
        fj,
        (*fj).cmd,
        buf,
    );
    if *buf as libc::c_int != '\0' as i32 || (*fj).updated == 0 {
        free((*fj).out as *mut libc::c_void);
        (*fj).out = buf;
    } else {
        free(buf as *mut libc::c_void);
    }
    if (*fj).status != 0 {
        if !((*fj).client).is_null() {
            server_status_client((*fj).client);
        }
        (*fj).status = 0 as libc::c_int;
    }
}
unsafe extern "C" fn format_job_get(
    mut ft: *mut format_tree,
    mut cmd: *const libc::c_char,
) -> *mut libc::c_char {
    let mut jobs: *mut format_job_tree = 0 as *mut format_job_tree;
    let mut fj0: format_job = format_job {
        client: 0 as *mut client,
        tag: 0,
        cmd: 0 as *const libc::c_char,
        expanded: 0 as *const libc::c_char,
        last: 0,
        out: 0 as *mut libc::c_char,
        updated: 0,
        job: 0 as *mut job,
        status: 0,
        entry: C2RustUnnamed_35 {
            rbe_left: 0 as *mut format_job,
            rbe_right: 0 as *mut format_job,
            rbe_parent: 0 as *mut format_job,
            rbe_color: 0,
        },
    };
    let mut fj: *mut format_job = 0 as *mut format_job;
    let mut t: time_t = 0;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut force: libc::c_int = 0;
    if ((*ft).client).is_null() {
        jobs = &mut format_jobs;
    } else if !((*(*ft).client).jobs).is_null() {
        jobs = (*(*ft).client).jobs;
    } else {
        (*(*ft).client)
            .jobs = xmalloc(::core::mem::size_of::<format_job_tree>() as libc::c_ulong)
            as *mut format_job_tree;
        jobs = (*(*ft).client).jobs;
        (*jobs).rbh_root = 0 as *mut format_job;
    }
    fj0.tag = (*ft).tag;
    fj0.cmd = cmd;
    fj = format_job_tree_RB_FIND(jobs, &mut fj0);
    if fj.is_null() {
        fj = xcalloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<format_job>() as libc::c_ulong,
        ) as *mut format_job;
        (*fj).client = (*ft).client;
        (*fj).tag = (*ft).tag;
        (*fj).cmd = xstrdup(cmd);
        (*fj).expanded = 0 as *const libc::c_char;
        xasprintf(
            &mut (*fj).out as *mut *mut libc::c_char,
            b"<'%s' not ready>\0" as *const u8 as *const libc::c_char,
            (*fj).cmd,
        );
        format_job_tree_RB_INSERT(jobs, fj);
    }
    expanded = format_expand(ft, cmd);
    if ((*fj).expanded).is_null() || strcmp(expanded, (*fj).expanded) != 0 as libc::c_int
    {
        free((*fj).expanded as *mut libc::c_void);
        (*fj).expanded = xstrdup(expanded);
        force = 1 as libc::c_int;
    } else {
        force = (*ft).flags & 0x2 as libc::c_int;
    }
    t = time(0 as *mut time_t);
    if ((*fj).job).is_null() && (force != 0 || (*fj).last != t) {
        (*fj)
            .job = job_run(
            expanded,
            0 as *mut session,
            0 as *const libc::c_char,
            Some(format_job_update as unsafe extern "C" fn(*mut job) -> ()),
            Some(format_job_complete as unsafe extern "C" fn(*mut job) -> ()),
            None,
            fj as *mut libc::c_void,
            0x1 as libc::c_int,
        );
        if ((*fj).job).is_null() {
            free((*fj).out as *mut libc::c_void);
            xasprintf(
                &mut (*fj).out as *mut *mut libc::c_char,
                b"<'%s' didn't start>\0" as *const u8 as *const libc::c_char,
                (*fj).cmd,
            );
        }
        (*fj).last = t;
        (*fj).updated = 0 as libc::c_int;
    }
    if (*ft).flags & 0x1 as libc::c_int != 0 {
        (*fj).status = 1 as libc::c_int;
    }
    free(expanded as *mut libc::c_void);
    return format_expand(ft, (*fj).out);
}
unsafe extern "C" fn format_job_tidy(
    mut jobs: *mut format_job_tree,
    mut force: libc::c_int,
) {
    let mut fj: *mut format_job = 0 as *mut format_job;
    let mut fj1: *mut format_job = 0 as *mut format_job;
    let mut now: time_t = 0;
    now = time(0 as *mut time_t);
    fj = format_job_tree_RB_MINMAX(jobs, -(1 as libc::c_int));
    while !fj.is_null()
        && {
            fj1 = format_job_tree_RB_NEXT(fj);
            1 as libc::c_int != 0
        }
    {
        if !(force == 0
            && ((*fj).last > now || now - (*fj).last < 3600 as libc::c_int as time_t))
        {
            format_job_tree_RB_REMOVE(jobs, fj);
            log_debug(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"format_job_tidy\0"))
                    .as_ptr(),
                (*fj).cmd,
            );
            if !((*fj).job).is_null() {
                job_free((*fj).job);
            }
            free((*fj).expanded as *mut libc::c_void);
            free((*fj).cmd as *mut libc::c_void);
            free((*fj).out as *mut libc::c_void);
            free(fj as *mut libc::c_void);
        }
        fj = fj1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn format_lost_client(mut c: *mut client) {
    if !((*c).jobs).is_null() {
        format_job_tidy((*c).jobs, 1 as libc::c_int);
    }
    free((*c).jobs as *mut libc::c_void);
}
unsafe extern "C" fn format_job_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = 0 as *mut client;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 60 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    format_job_tidy(&mut format_jobs, 0 as libc::c_int);
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).jobs).is_null() {
            format_job_tidy((*c).jobs, 0 as libc::c_int);
        }
        c = (*c).entry.tqe_next;
    }
    event_del(&mut format_job_event);
    event_add(&mut format_job_event, &mut tv);
}
unsafe extern "C" fn format_cb_host(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut host: [libc::c_char; 65] = [0; 65];
    if gethostname(
        host.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        (*fe).value = xstrdup(b"\0" as *const u8 as *const libc::c_char);
    } else {
        (*fe).value = xstrdup(host.as_mut_ptr());
    };
}
unsafe extern "C" fn format_cb_host_short(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut host: [libc::c_char; 65] = [0; 65];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if gethostname(
        host.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        (*fe).value = xstrdup(b"\0" as *const u8 as *const libc::c_char);
    } else {
        cp = strchr(host.as_mut_ptr(), '.' as i32);
        if !cp.is_null() {
            *cp = '\0' as i32 as libc::c_char;
        }
        (*fe).value = xstrdup(host.as_mut_ptr());
    };
}
unsafe extern "C" fn format_cb_pid(mut ft: *mut format_tree, mut fe: *mut format_entry) {
    xasprintf(
        &mut (*fe).value as *mut *mut libc::c_char,
        b"%ld\0" as *const u8 as *const libc::c_char,
        getpid() as libc::c_long,
    );
}
unsafe extern "C" fn format_cb_session_alerts(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut s: *mut session = (*ft).s;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut alerts: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if s.is_null() {
        return;
    }
    *alerts.as_mut_ptr() = '\0' as i32 as libc::c_char;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if !((*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
            == 0 as libc::c_int)
        {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                (*wl).idx,
            );
            if *alerts.as_mut_ptr() as libc::c_int != '\0' as i32 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b",\0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            strlcat(
                alerts.as_mut_ptr(),
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
            if (*wl).flags & 0x2 as libc::c_int != 0 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b"#\0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            if (*wl).flags & 0x1 as libc::c_int != 0 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b"!\0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            if (*wl).flags & 0x4 as libc::c_int != 0 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b"~\0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
        }
        wl = winlinks_RB_NEXT(wl);
    }
    (*fe).value = xstrdup(alerts.as_mut_ptr());
}
unsafe extern "C" fn format_cb_session_stack(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut s: *mut session = (*ft).s;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut result: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if s.is_null() {
        return;
    }
    xsnprintf(
        result.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*(*s).curw).idx,
    );
    wl = (*s).lastw.tqh_first;
    while !wl.is_null() {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            (*wl).idx,
        );
        if *result.as_mut_ptr() as libc::c_int != '\0' as i32 {
            strlcat(
                result.as_mut_ptr(),
                b",\0" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
        strlcat(
            result.as_mut_ptr(),
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        wl = (*wl).sentry.tqe_next;
    }
    (*fe).value = xstrdup(result.as_mut_ptr());
}
unsafe extern "C" fn format_cb_window_stack_index(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut s: *mut session = (*(*ft).wl).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut idx: u_int = 0;
    idx = 0 as libc::c_int as u_int;
    wl = (*s).lastw.tqh_first;
    while !wl.is_null() {
        idx = idx.wrapping_add(1);
        idx;
        if wl == (*ft).wl {
            break;
        }
        wl = (*wl).sentry.tqe_next;
    }
    if !wl.is_null() {
        xasprintf(
            &mut (*fe).value as *mut *mut libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            idx,
        );
    } else {
        (*fe).value = xstrdup(b"0\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn format_cb_window_layout(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut w: *mut window = (*ft).w;
    if w.is_null() {
        return;
    }
    if !((*w).saved_layout_root).is_null() {
        (*fe).value = layout_dump((*w).saved_layout_root);
    } else {
        (*fe).value = layout_dump((*w).layout_root);
    };
}
unsafe extern "C" fn format_cb_window_visible_layout(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut w: *mut window = (*ft).w;
    if w.is_null() {
        return;
    }
    (*fe).value = layout_dump((*w).layout_root);
}
unsafe extern "C" fn format_cb_start_command(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut wp: *mut window_pane = (*ft).wp;
    if wp.is_null() {
        return;
    }
    (*fe).value = cmd_stringify_argv((*wp).argc, (*wp).argv);
}
unsafe extern "C" fn format_cb_current_command(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return;
    }
    cmd = osdep_get_name((*wp).fd, ((*wp).tty).as_mut_ptr());
    if cmd.is_null() || *cmd as libc::c_int == '\0' as i32 {
        free(cmd as *mut libc::c_void);
        cmd = cmd_stringify_argv((*wp).argc, (*wp).argv);
        if cmd.is_null() || *cmd as libc::c_int == '\0' as i32 {
            free(cmd as *mut libc::c_void);
            cmd = xstrdup((*wp).shell);
        }
    }
    (*fe).value = parse_window_name(cmd);
    free(cmd as *mut libc::c_void);
}
unsafe extern "C" fn format_cb_current_path(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return;
    }
    cwd = osdep_get_cwd((*wp).fd);
    if !cwd.is_null() {
        (*fe).value = xstrdup(cwd);
    }
}
unsafe extern "C" fn format_cb_history_bytes(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut gd: *mut grid = 0 as *mut grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut size: libc::c_ulonglong = 0;
    let mut i: u_int = 0;
    if wp.is_null() {
        return;
    }
    gd = (*wp).base.grid;
    size = 0 as libc::c_int as libc::c_ulonglong;
    i = 0 as libc::c_int as u_int;
    while i < (*gd).hsize {
        gl = &mut *((*gd).linedata).offset(i as isize) as *mut grid_line;
        size = size
            .wrapping_add(
                ((*gl).cellsize as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<grid_cell_entry>() as libc::c_ulong,
                    ) as libc::c_ulonglong,
            );
        size = size
            .wrapping_add(
                ((*gl).extdsize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<grid_cell>() as libc::c_ulong)
                    as libc::c_ulonglong,
            );
        i = i.wrapping_add(1);
        i;
    }
    size = size
        .wrapping_add(
            ((*gd).hsize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong)
                as libc::c_ulonglong,
        );
    xasprintf(
        &mut (*fe).value as *mut *mut libc::c_char,
        b"%llu\0" as *const u8 as *const libc::c_char,
        size,
    );
}
unsafe extern "C" fn format_cb_pane_tabs(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut i: u_int = 0;
    let mut size: libc::c_int = 0;
    if wp.is_null() {
        return;
    }
    buffer = evbuffer_new();
    i = 0 as libc::c_int as u_int;
    while i < (*(*wp).base.grid).sx {
        if !(*((*wp).base.tabs).offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (i & 0x7 as libc::c_int as u_int) == 0)
        {
            if evbuffer_get_length(buffer) > 0 as libc::c_int as size_t {
                evbuffer_add(
                    buffer,
                    b",\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
            evbuffer_add_printf(buffer, b"%u\0" as *const u8 as *const libc::c_char, i);
        }
        i = i.wrapping_add(1);
        i;
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut (*fe).value as *mut *mut libc::c_char,
            b"%.*s\0" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
}
unsafe extern "C" fn format_cb_session_group_list(
    mut ft: *mut format_tree,
    mut fe: *mut format_entry,
) {
    let mut s: *mut session = (*ft).s;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut loop_0: *mut session = 0 as *mut session;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    if s.is_null() {
        return;
    }
    sg = session_group_contains(s);
    if sg.is_null() {
        return;
    }
    buffer = evbuffer_new();
    loop_0 = (*sg).sessions.tqh_first;
    while !loop_0.is_null() {
        if evbuffer_get_length(buffer) > 0 as libc::c_int as size_t {
            evbuffer_add(
                buffer,
                b",\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        evbuffer_add_printf(
            buffer,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*loop_0).name,
        );
        loop_0 = (*loop_0).gentry.tqe_next;
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut (*fe).value as *mut *mut libc::c_char,
            b"%.*s\0" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
}
unsafe extern "C" fn format_merge(mut ft: *mut format_tree, mut from: *mut format_tree) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    fe = format_entry_tree_RB_MINMAX(&mut (*from).tree, -(1 as libc::c_int));
    while !fe.is_null() {
        if !((*fe).value).is_null() {
            format_add(
                ft,
                (*fe).key,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*fe).value,
            );
        }
        fe = format_entry_tree_RB_NEXT(fe);
    }
}
#[no_mangle]
pub unsafe extern "C" fn format_create(
    mut c: *mut client,
    mut item: *mut cmdq_item,
    mut tag: libc::c_int,
    mut flags: libc::c_int,
) -> *mut format_tree {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    if event_initialized(&mut format_job_event) == 0 {
        event_set(
            &mut format_job_event,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                format_job_timer
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        format_job_timer(
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            0 as *mut libc::c_void,
        );
    }
    ft = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<format_tree>() as libc::c_ulong,
    ) as *mut format_tree;
    (*ft).tree.rbh_root = 0 as *mut format_entry;
    if !c.is_null() {
        (*ft).client = c;
        (*(*ft).client).references += 1;
        (*(*ft).client).references;
    }
    (*ft).tag = tag as u_int;
    (*ft).flags = flags;
    format_add(
        ft,
        b"version\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        b"master\0" as *const u8 as *const libc::c_char,
    );
    format_add_cb(
        ft,
        b"host\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_host
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add_cb(
        ft,
        b"host_short\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_host_short
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add_cb(
        ft,
        b"pid\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_pid
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add(
        ft,
        b"socket_path\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        socket_path,
    );
    format_add_tv(
        ft,
        b"start_time\0" as *const u8 as *const libc::c_char,
        &mut start_time,
    );
    if !item.is_null() {
        if !((*item).cmd).is_null() {
            format_add(
                ft,
                b"command\0" as *const u8 as *const libc::c_char,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*(*(*item).cmd).entry).name,
            );
        }
        if !((*item).shared).is_null() && !((*(*item).shared).formats).is_null() {
            format_merge(ft, (*(*item).shared).formats);
        }
    }
    return ft;
}
#[no_mangle]
pub unsafe extern "C" fn format_free(mut ft: *mut format_tree) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe1: *mut format_entry = 0 as *mut format_entry;
    fe = format_entry_tree_RB_MINMAX(&mut (*ft).tree, -(1 as libc::c_int));
    while !fe.is_null()
        && {
            fe1 = format_entry_tree_RB_NEXT(fe);
            1 as libc::c_int != 0
        }
    {
        format_entry_tree_RB_REMOVE(&mut (*ft).tree, fe);
        free((*fe).value as *mut libc::c_void);
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        fe = fe1;
    }
    if !((*ft).client).is_null() {
        server_client_unref((*ft).client);
    }
    free(ft as *mut libc::c_void);
}
unsafe extern "C" fn format_add_tv(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut tv: *mut timeval,
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    fe = xmalloc(::core::mem::size_of::<format_entry>() as libc::c_ulong)
        as *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now = format_entry_tree_RB_INSERT(&mut (*ft).tree, fe);
    if !fe_now.is_null() {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now;
    }
    (*fe).cb = None;
    (*fe).t = (*tv).tv_sec;
    (*fe).value = 0 as *mut libc::c_char;
}
unsafe extern "C" fn format_add_cb(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut cb: format_cb,
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    fe = xmalloc(::core::mem::size_of::<format_entry>() as libc::c_ulong)
        as *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now = format_entry_tree_RB_INSERT(&mut (*ft).tree, fe);
    if !fe_now.is_null() {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now;
    }
    (*fe).cb = cb;
    (*fe).t = 0 as libc::c_int as time_t;
    (*fe).value = 0 as *mut libc::c_char;
}
unsafe extern "C" fn format_find(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut modifiers: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_find: format_entry = format_entry {
        key: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
        t: 0,
        cb: None,
        entry: C2RustUnnamed_31 {
            rbe_left: 0 as *mut format_entry,
            rbe_right: 0 as *mut format_entry,
            rbe_parent: 0 as *mut format_entry,
            rbe_color: 0,
        },
    };
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    static mut s: [libc::c_char; 64] = [0; 64];
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut found: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved: *mut libc::c_char = 0 as *mut libc::c_char;
    if !modifiers & 0x1 as libc::c_int != 0 {
        o = options_parse_get(global_options, key, &mut idx, 0 as libc::c_int);
        if o.is_null() && !((*ft).w).is_null() {
            o = options_parse_get((*(*ft).w).options, key, &mut idx, 0 as libc::c_int);
        }
        if o.is_null() {
            o = options_parse_get(global_w_options, key, &mut idx, 0 as libc::c_int);
        }
        if o.is_null() && !((*ft).s).is_null() {
            o = options_parse_get((*(*ft).s).options, key, &mut idx, 0 as libc::c_int);
        }
        if o.is_null() {
            o = options_parse_get(global_s_options, key, &mut idx, 0 as libc::c_int);
        }
        if !o.is_null() {
            found = options_tostring(o, idx, 1 as libc::c_int);
            current_block = 4231418125067221820;
        } else {
            current_block = 11050875288958768710;
        }
    } else {
        current_block = 11050875288958768710;
    }
    match current_block {
        11050875288958768710 => {
            found = 0 as *const libc::c_char;
            fe_find.key = key as *mut libc::c_char;
            fe = format_entry_tree_RB_FIND(&mut (*ft).tree, &mut fe_find);
            if !fe.is_null() {
                if modifiers & 0x1 as libc::c_int != 0 {
                    if (*fe).t == 0 as libc::c_int as time_t {
                        return 0 as *mut libc::c_char;
                    }
                    ctime_r(&mut (*fe).t, s.as_mut_ptr());
                    s[strcspn(
                        s.as_mut_ptr(),
                        b"\n\0" as *const u8 as *const libc::c_char,
                    ) as usize] = '\0' as i32 as libc::c_char;
                    found = s.as_mut_ptr();
                } else if (*fe).t != 0 as libc::c_int as time_t {
                    xsnprintf(
                        s.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                        b"%lld\0" as *const u8 as *const libc::c_char,
                        (*fe).t as libc::c_longlong,
                    );
                    found = s.as_mut_ptr();
                } else {
                    if ((*fe).value).is_null() && ((*fe).cb).is_some() {
                        ((*fe).cb).expect("non-null function pointer")(ft, fe);
                        if ((*fe).value).is_null() {
                            (*fe)
                                .value = xstrdup(b"\0" as *const u8 as *const libc::c_char);
                        }
                    }
                    found = (*fe).value;
                }
            } else {
                if !modifiers & 0x1 as libc::c_int != 0 {
                    envent = 0 as *mut environ_entry;
                    if !((*ft).s).is_null() {
                        envent = environ_find((*(*ft).s).environ, key);
                    }
                    if envent.is_null() {
                        envent = environ_find(global_environ, key);
                    }
                    if !envent.is_null() {
                        found = (*envent).value;
                        current_block = 4231418125067221820;
                    } else {
                        current_block = 2873832966593178012;
                    }
                } else {
                    current_block = 2873832966593178012;
                }
                match current_block {
                    4231418125067221820 => {}
                    _ => return 0 as *mut libc::c_char,
                }
            }
        }
        _ => {}
    }
    if found.is_null() {
        return 0 as *mut libc::c_char;
    }
    copy = xstrdup(found);
    if modifiers & 0x2 as libc::c_int != 0 {
        saved = copy;
        copy = xstrdup(__xpg_basename(saved));
        free(saved as *mut libc::c_void);
    }
    if modifiers & 0x4 as libc::c_int != 0 {
        saved = copy;
        copy = xstrdup(dirname(saved));
        free(saved as *mut libc::c_void);
    }
    return copy;
}
unsafe extern "C" fn format_skip(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut brackets: libc::c_int = 0 as libc::c_int;
    while *s as libc::c_int != '\0' as i32 {
        if *s as libc::c_int == '{' as i32 {
            brackets += 1;
            brackets;
        }
        if *s as libc::c_int == '}' as i32 {
            brackets -= 1;
            brackets;
        }
        if *s as libc::c_int == ',' as i32 && brackets == 0 as libc::c_int {
            break;
        }
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    return s;
}
unsafe extern "C" fn format_choose(
    mut s: *mut libc::c_char,
    mut left: *mut *mut libc::c_char,
    mut right: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = format_skip(s);
    if cp.is_null() {
        return -(1 as libc::c_int);
    }
    *cp = '\0' as i32 as libc::c_char;
    *left = s;
    *right = cp.offset(1 as libc::c_int as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn format_true(mut s: *const libc::c_char) -> libc::c_int {
    if !s.is_null() && *s as libc::c_int != '\0' as i32
        && (*s.offset(0 as libc::c_int as isize) as libc::c_int != '0' as i32
            || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn format_replace(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut keylen: size_t,
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut off: *mut size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut wp: *mut window_pane = (*ft).wp;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuelen: size_t = 0;
    let mut newlen: size_t = 0;
    let mut fromlen: size_t = 0;
    let mut tolen: size_t = 0;
    let mut used: size_t = 0;
    let mut limit: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut modifiers: libc::c_int = 0 as libc::c_int;
    let mut compare: libc::c_int = 0 as libc::c_int;
    let mut search: libc::c_int = 0 as libc::c_int;
    copy = xmalloc(keylen.wrapping_add(1 as libc::c_int as size_t)) as *mut libc::c_char;
    copy0 = copy;
    memcpy(copy as *mut libc::c_void, key as *const libc::c_void, keylen);
    *copy.offset(keylen as isize) = '\0' as i32 as libc::c_char;
    match *copy.offset(0 as libc::c_int as isize) as libc::c_int {
        109 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32) {
                compare = -(2 as libc::c_int);
                copy = copy.offset(2 as libc::c_int as isize);
            }
        }
        67 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32) {
                search = 1 as libc::c_int;
                copy = copy.offset(2 as libc::c_int as isize);
            }
        }
        124 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != '|' as i32
                || *copy.offset(2 as libc::c_int as isize) as libc::c_int != ':' as i32)
            {
                compare = -(3 as libc::c_int);
                copy = copy.offset(3 as libc::c_int as isize);
            }
        }
        38 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != '&' as i32
                || *copy.offset(2 as libc::c_int as isize) as libc::c_int != ':' as i32)
            {
                compare = -(4 as libc::c_int);
                copy = copy.offset(3 as libc::c_int as isize);
            }
        }
        33 => {
            if *copy.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                && *copy.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32
            {
                compare = -(1 as libc::c_int);
                copy = copy.offset(3 as libc::c_int as isize);
            }
        }
        61 => {
            if *copy.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                && *copy.offset(2 as libc::c_int as isize) as libc::c_int == ':' as i32
            {
                compare = 1 as libc::c_int;
                copy = copy.offset(3 as libc::c_int as isize);
            } else {
                *__errno_location() = 0 as libc::c_int;
                limit = strtol(
                    copy.offset(1 as libc::c_int as isize),
                    &mut endptr,
                    10 as libc::c_int,
                );
                if !(*__errno_location() == 34 as libc::c_int
                    && (limit
                        == -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                        || limit == 9223372036854775807 as libc::c_long))
                {
                    if !(*endptr as libc::c_int != ':' as i32) {
                        copy = endptr.offset(1 as libc::c_int as isize);
                    }
                }
            }
        }
        98 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32) {
                modifiers |= 0x2 as libc::c_int;
                copy = copy.offset(2 as libc::c_int as isize);
            }
        }
        100 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32) {
                modifiers |= 0x4 as libc::c_int;
                copy = copy.offset(2 as libc::c_int as isize);
            }
        }
        116 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32) {
                modifiers |= 0x1 as libc::c_int;
                copy = copy.offset(2 as libc::c_int as isize);
            }
        }
        115 => {
            if !(*copy.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32) {
                from = copy.offset(2 as libc::c_int as isize);
                copy = from;
                while *copy as libc::c_int != '\0' as i32
                    && *copy as libc::c_int != '/' as i32
                {
                    copy = copy.offset(1);
                    copy;
                }
                if *copy.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
                    || copy == from
                {
                    copy = copy0;
                } else {
                    *copy
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    to = copy.offset(1 as libc::c_int as isize);
                    copy = to;
                    while *copy as libc::c_int != '\0' as i32
                        && *copy as libc::c_int != '/' as i32
                    {
                        copy = copy.offset(1);
                        copy;
                    }
                    if *copy.offset(0 as libc::c_int as isize) as libc::c_int
                        != '/' as i32
                        || *copy.offset(1 as libc::c_int as isize) as libc::c_int
                            != ':' as i32
                    {
                        copy = copy0;
                    } else {
                        *copy
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        modifiers |= 0x8 as libc::c_int;
                        copy = copy.offset(2 as libc::c_int as isize);
                    }
                }
            }
        }
        _ => {}
    }
    if search != 0 {
        if wp.is_null() {
            value = xstrdup(b"0\0" as *const u8 as *const libc::c_char);
        } else {
            xasprintf(
                &mut value as *mut *mut libc::c_char,
                b"%u\0" as *const u8 as *const libc::c_char,
                window_pane_search(wp, copy),
            );
        }
    } else {
        if compare != 0 as libc::c_int {
            if format_choose(copy, &mut left, &mut right) != 0 as libc::c_int {
                current_block = 15487814550564020245;
            } else {
                left = format_expand(ft, left);
                right = format_expand(ft, right);
                if (compare == -(3 as libc::c_int)
                    && (format_true(left) != 0 || format_true(right) != 0)) ||
                    (compare == -(4 as libc::c_int)
                    && (format_true(left) != 0 && format_true(right) != 0)) ||
                    (compare == 1 as libc::c_int
                        && strcmp(left, right) == 0 as libc::c_int) ||
                    (compare == -(1 as libc::c_int)
                    && strcmp(left, right) != 0 as libc::c_int) ||
                    (compare == -(2 as libc::c_int)
                    && fnmatch(left, right, 0 as libc::c_int) == 0 as libc::c_int)
                {
                    value = xstrdup(b"1\0" as *const u8 as *const libc::c_char);
                } else {
                    value = xstrdup(b"0\0" as *const u8 as *const libc::c_char);
                }
                free(right as *mut libc::c_void);
                free(left as *mut libc::c_void);
                current_block = 18325745679564279244;
            }
        } else if *copy as libc::c_int == '?' as i32 {
            ptr = format_skip(copy);
            if ptr.is_null() {
                current_block = 15487814550564020245;
            } else {
                *ptr = '\0' as i32 as libc::c_char;
                found = format_find(
                    ft,
                    copy.offset(1 as libc::c_int as isize),
                    modifiers,
                );
                if found.is_null() {
                    found = format_expand(ft, copy.offset(1 as libc::c_int as isize));
                }
                if format_choose(
                    ptr.offset(1 as libc::c_int as isize),
                    &mut left,
                    &mut right,
                ) != 0 as libc::c_int
                {
                    current_block = 15487814550564020245;
                } else {
                    if format_true(found) != 0 {
                        value = format_expand(ft, left);
                    } else {
                        value = format_expand(ft, right);
                    }
                    free(found as *mut libc::c_void);
                    current_block = 18325745679564279244;
                }
            }
        } else {
            value = format_find(ft, copy, modifiers);
            if value.is_null() {
                value = xstrdup(b"\0" as *const u8 as *const libc::c_char);
            }
            current_block = 18325745679564279244;
        }
        match current_block {
            18325745679564279244 => {}
            _ => {
                free(copy0 as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        }
    }
    if modifiers & 0x8 as libc::c_int != 0 {
        fromlen = strlen(from);
        tolen = strlen(to);
        newlen = (strlen(value)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        new = xmalloc(newlen) as *mut libc::c_char;
        copy = new;
        ptr = value;
        while *ptr as libc::c_int != '\0' as i32 {
            if strncmp(ptr, from, fromlen) != 0 as libc::c_int {
                let fresh0 = ptr;
                ptr = ptr.offset(1);
                let fresh1 = new;
                new = new.offset(1);
                *fresh1 = *fresh0;
            } else {
                used = new.offset_from(copy) as libc::c_long as size_t;
                newlen = newlen.wrapping_add(tolen);
                copy = xrealloc(copy as *mut libc::c_void, newlen) as *mut libc::c_char;
                new = copy.offset(used as isize);
                memcpy(new as *mut libc::c_void, to as *const libc::c_void, tolen);
                new = new.offset(tolen as isize);
                ptr = ptr.offset(fromlen as isize);
            }
        }
        *new = '\0' as i32 as libc::c_char;
        free(value as *mut libc::c_void);
        value = copy;
    }
    if limit > 0 as libc::c_int as libc::c_long {
        new = utf8_trimcstr(value, limit as u_int);
        free(value as *mut libc::c_void);
        value = new;
    } else if limit < 0 as libc::c_int as libc::c_long {
        new = utf8_rtrimcstr(value, -limit as u_int);
        free(value as *mut libc::c_void);
        value = new;
    }
    valuelen = strlen(value);
    while (*len).wrapping_sub(*off) < valuelen.wrapping_add(1 as libc::c_int as size_t) {
        *buf = xreallocarray(*buf as *mut libc::c_void, 2 as libc::c_int as size_t, *len)
            as *mut libc::c_char;
        *len = *len * 2 as libc::c_int as size_t;
    }
    memcpy(
        (*buf).offset(*off as isize) as *mut libc::c_void,
        value as *const libc::c_void,
        valuelen,
    );
    *off = (*off).wrapping_add(valuelen);
    free(value as *mut libc::c_void);
    free(copy0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn format_expand_time(
    mut ft: *mut format_tree,
    mut fmt: *const libc::c_char,
    mut t: time_t,
) -> *mut libc::c_char {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut s: [libc::c_char; 2048] = [0; 2048];
    if fmt.is_null() || *fmt as libc::c_int == '\0' as i32 {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    tm = localtime(&mut t);
    if strftime(
        s.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
        fmt,
        tm,
    ) == 0 as libc::c_int as size_t
    {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    return format_expand(ft, s.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn format_expand(
    mut ft: *mut format_tree,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut saved: *const libc::c_char = fmt;
    let mut off: size_t = 0;
    let mut len: size_t = 0;
    let mut n: size_t = 0;
    let mut outlen: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut brackets: libc::c_int = 0;
    if fmt.is_null() {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    len = 64 as libc::c_int as size_t;
    buf = xmalloc(len) as *mut libc::c_char;
    off = 0 as libc::c_int as size_t;
    while *fmt as libc::c_int != '\0' as i32 {
        if *fmt as libc::c_int != '#' as i32 {
            while len.wrapping_sub(off) < 2 as libc::c_int as size_t {
                buf = xreallocarray(
                    buf as *mut libc::c_void,
                    2 as libc::c_int as size_t,
                    len,
                ) as *mut libc::c_char;
                len = len * 2 as libc::c_int as size_t;
            }
            let fresh2 = fmt;
            fmt = fmt.offset(1);
            let fresh3 = off;
            off = off.wrapping_add(1);
            *buf.offset(fresh3 as isize) = *fresh2;
        } else {
            fmt = fmt.offset(1);
            fmt;
            let fresh4 = fmt;
            fmt = fmt.offset(1);
            ch = *fresh4 as u_char as libc::c_int;
            match ch {
                40 => {
                    brackets = 1 as libc::c_int;
                    ptr = fmt;
                    while *ptr as libc::c_int != '\0' as i32 {
                        if *ptr as libc::c_int == '(' as i32 {
                            brackets += 1;
                            brackets;
                        }
                        if *ptr as libc::c_int == ')' as i32
                            && {
                                brackets -= 1;
                                brackets == 0 as libc::c_int
                            }
                        {
                            break;
                        }
                        ptr = ptr.offset(1);
                        ptr;
                    }
                    if *ptr as libc::c_int != ')' as i32 || brackets != 0 as libc::c_int
                    {
                        break;
                    }
                    n = ptr.offset_from(fmt) as libc::c_long as size_t;
                    if (*ft).flags & 0x4 as libc::c_int != 0 {
                        out = xstrdup(b"\0" as *const u8 as *const libc::c_char);
                    } else {
                        name = xstrndup(fmt, n);
                        out = format_job_get(ft, name);
                        free(name as *mut libc::c_void);
                    }
                    outlen = strlen(out);
                    while len.wrapping_sub(off)
                        < outlen.wrapping_add(1 as libc::c_int as size_t)
                    {
                        buf = xreallocarray(
                            buf as *mut libc::c_void,
                            2 as libc::c_int as size_t,
                            len,
                        ) as *mut libc::c_char;
                        len = len * 2 as libc::c_int as size_t;
                    }
                    memcpy(
                        buf.offset(off as isize) as *mut libc::c_void,
                        out as *const libc::c_void,
                        outlen,
                    );
                    off = off.wrapping_add(outlen);
                    free(out as *mut libc::c_void);
                    fmt = fmt
                        .offset(n.wrapping_add(1 as libc::c_int as size_t) as isize);
                }
                123 => {
                    brackets = 1 as libc::c_int;
                    ptr = fmt;
                    while *ptr as libc::c_int != '\0' as i32 {
                        if *ptr as libc::c_int == '{' as i32 {
                            brackets += 1;
                            brackets;
                        }
                        if *ptr as libc::c_int == '}' as i32
                            && {
                                brackets -= 1;
                                brackets == 0 as libc::c_int
                            }
                        {
                            break;
                        }
                        ptr = ptr.offset(1);
                        ptr;
                    }
                    if *ptr as libc::c_int != '}' as i32 || brackets != 0 as libc::c_int
                    {
                        break;
                    }
                    n = ptr.offset_from(fmt) as libc::c_long as size_t;
                    if format_replace(ft, fmt, n, &mut buf, &mut len, &mut off)
                        != 0 as libc::c_int
                    {
                        break;
                    }
                    fmt = fmt
                        .offset(n.wrapping_add(1 as libc::c_int as size_t) as isize);
                }
                35 => {
                    while len.wrapping_sub(off) < 2 as libc::c_int as size_t {
                        buf = xreallocarray(
                            buf as *mut libc::c_void,
                            2 as libc::c_int as size_t,
                            len,
                        ) as *mut libc::c_char;
                        len = len * 2 as libc::c_int as size_t;
                    }
                    let fresh5 = off;
                    off = off.wrapping_add(1);
                    *buf.offset(fresh5 as isize) = '#' as i32 as libc::c_char;
                }
                _ => {
                    s = 0 as *const libc::c_char;
                    if ch >= 'A' as i32 && ch <= 'Z' as i32 {
                        s = format_upper[(ch - 'A' as i32) as usize];
                    } else if ch >= 'a' as i32 && ch <= 'z' as i32 {
                        s = format_lower[(ch - 'a' as i32) as usize];
                    }
                    if s.is_null() {
                        while len.wrapping_sub(off) < 3 as libc::c_int as size_t {
                            buf = xreallocarray(
                                buf as *mut libc::c_void,
                                2 as libc::c_int as size_t,
                                len,
                            ) as *mut libc::c_char;
                            len = len * 2 as libc::c_int as size_t;
                        }
                        let fresh6 = off;
                        off = off.wrapping_add(1);
                        *buf.offset(fresh6 as isize) = '#' as i32 as libc::c_char;
                        let fresh7 = off;
                        off = off.wrapping_add(1);
                        *buf.offset(fresh7 as isize) = ch as libc::c_char;
                    } else {
                        n = strlen(s);
                        if format_replace(ft, s, n, &mut buf, &mut len, &mut off)
                            != 0 as libc::c_int
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
    *buf.offset(off as isize) = '\0' as i32 as libc::c_char;
    log_debug(b"format '%s' -> '%s'\0" as *const u8 as *const libc::c_char, saved, buf);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn format_single(
    mut item: *mut cmdq_item,
    mut fmt: *const libc::c_char,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    if !item.is_null() {
        ft = format_create((*item).client, item, 0 as libc::c_int, 0 as libc::c_int);
    } else {
        ft = format_create(0 as *mut client, item, 0 as libc::c_int, 0 as libc::c_int);
    }
    format_defaults(ft, c, s, wl, wp);
    expanded = format_expand(ft, fmt);
    format_free(ft);
    return expanded;
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults(
    mut ft: *mut format_tree,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) {
    if !c.is_null() && !s.is_null() && (*c).session != s {
        log_debug(
            b"%s: session does not match\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"format_defaults\0"))
                .as_ptr(),
        );
    }
    format_add(
        ft,
        b"session_format\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (s != 0 as *mut libc::c_void as *mut session) as libc::c_int,
    );
    format_add(
        ft,
        b"window_format\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (wl != 0 as *mut libc::c_void as *mut winlink) as libc::c_int,
    );
    format_add(
        ft,
        b"pane_format\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (wp != 0 as *mut libc::c_void as *mut window_pane) as libc::c_int,
    );
    if s.is_null() && !c.is_null() {
        s = (*c).session;
    }
    if wl.is_null() && !s.is_null() {
        wl = (*s).curw;
    }
    if wp.is_null() && !wl.is_null() {
        wp = (*(*wl).window).active;
    }
    if !c.is_null() {
        format_defaults_client(ft, c);
    }
    if !s.is_null() {
        format_defaults_session(ft, s);
    }
    if !wl.is_null() {
        format_defaults_winlink(ft, wl);
    }
    if !wp.is_null() {
        format_defaults_pane(ft, wp);
    }
}
unsafe extern "C" fn format_defaults_session(
    mut ft: *mut format_tree,
    mut s: *mut session,
) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    (*ft).s = s;
    format_add(
        ft,
        b"session_name\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*s).name,
    );
    format_add(
        ft,
        b"session_windows\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        winlink_count(&mut (*s).windows),
    );
    format_add(
        ft,
        b"session_width\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*s).sx,
    );
    format_add(
        ft,
        b"session_height\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*s).sy,
    );
    format_add(
        ft,
        b"session_id\0" as *const u8 as *const libc::c_char,
        b"$%u\0" as *const u8 as *const libc::c_char,
        (*s).id,
    );
    sg = session_group_contains(s);
    format_add(
        ft,
        b"session_grouped\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (sg != 0 as *mut libc::c_void as *mut session_group) as libc::c_int,
    );
    if !sg.is_null() {
        format_add(
            ft,
            b"session_group\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*sg).name,
        );
        format_add(
            ft,
            b"session_group_size\0" as *const u8 as *const libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            session_group_count(sg),
        );
        format_add_cb(
            ft,
            b"session_group_list\0" as *const u8 as *const libc::c_char,
            Some(
                format_cb_session_group_list
                    as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
            ),
        );
    }
    format_add_tv(
        ft,
        b"session_created\0" as *const u8 as *const libc::c_char,
        &mut (*s).creation_time,
    );
    format_add_tv(
        ft,
        b"session_last_attached\0" as *const u8 as *const libc::c_char,
        &mut (*s).last_attached_time,
    );
    format_add_tv(
        ft,
        b"session_activity\0" as *const u8 as *const libc::c_char,
        &mut (*s).activity_time,
    );
    format_add(
        ft,
        b"session_attached\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*s).attached,
    );
    format_add(
        ft,
        b"session_many_attached\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*s).attached > 1 as libc::c_int as u_int) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"session_alerts\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_session_alerts
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add_cb(
        ft,
        b"session_stack\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_session_stack
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
}
unsafe extern "C" fn format_defaults_client(
    mut ft: *mut format_tree,
    mut c: *mut client,
) {
    let mut s: *mut session = 0 as *mut session;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut types: [*const libc::c_char; 7] = [
        b"VT100\0" as *const u8 as *const libc::c_char,
        b"VT101\0" as *const u8 as *const libc::c_char,
        b"VT102\0" as *const u8 as *const libc::c_char,
        b"VT220\0" as *const u8 as *const libc::c_char,
        b"VT320\0" as *const u8 as *const libc::c_char,
        b"VT420\0" as *const u8 as *const libc::c_char,
        b"Unknown\0" as *const u8 as *const libc::c_char,
    ];
    if ((*ft).s).is_null() {
        (*ft).s = (*c).session;
    }
    format_add(
        ft,
        b"client_name\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).name,
    );
    format_add(
        ft,
        b"client_pid\0" as *const u8 as *const libc::c_char,
        b"%ld\0" as *const u8 as *const libc::c_char,
        (*c).pid as libc::c_long,
    );
    format_add(
        ft,
        b"client_height\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*tty).sy,
    );
    format_add(
        ft,
        b"client_width\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*tty).sx,
    );
    format_add(
        ft,
        b"client_tty\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).ttyname,
    );
    format_add(
        ft,
        b"client_control_mode\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*c).flags & 0x2000 as libc::c_int != 0) as libc::c_int,
    );
    if !((*tty).term_name).is_null() {
        format_add(
            ft,
            b"client_termname\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*tty).term_name,
        );
    }
    if !((*tty).term_name).is_null() {
        format_add(
            ft,
            b"client_termtype\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            types[(*tty).term_type as usize],
        );
    }
    format_add_tv(
        ft,
        b"client_created\0" as *const u8 as *const libc::c_char,
        &mut (*c).creation_time,
    );
    format_add_tv(
        ft,
        b"client_activity\0" as *const u8 as *const libc::c_char,
        &mut (*c).activity_time,
    );
    format_add(
        ft,
        b"client_written\0" as *const u8 as *const libc::c_char,
        b"%zu\0" as *const u8 as *const libc::c_char,
        (*c).written,
    );
    format_add(
        ft,
        b"client_discarded\0" as *const u8 as *const libc::c_char,
        b"%zu\0" as *const u8 as *const libc::c_char,
        (*c).discarded,
    );
    name = server_client_get_key_table(c);
    if strcmp((*(*c).keytable).name, name) == 0 as libc::c_int {
        format_add(
            ft,
            b"client_prefix\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"client_prefix\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    format_add(
        ft,
        b"client_key_table\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*(*c).keytable).name,
    );
    if (*tty).flags & 0x8 as libc::c_int != 0 {
        format_add(
            ft,
            b"client_utf8\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"client_utf8\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if (*c).flags & 0x800 as libc::c_int != 0 {
        format_add(
            ft,
            b"client_readonly\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"client_readonly\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    s = (*c).session;
    if !s.is_null() {
        format_add(
            ft,
            b"client_session\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*s).name,
        );
    }
    s = (*c).last_session;
    if !s.is_null() && session_alive(s) != 0 {
        format_add(
            ft,
            b"client_last_session\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*s).name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults_window(
    mut ft: *mut format_tree,
    mut w: *mut window,
) {
    (*ft).w = w;
    format_add_tv(
        ft,
        b"window_activity\0" as *const u8 as *const libc::c_char,
        &mut (*w).activity_time,
    );
    format_add(
        ft,
        b"window_id\0" as *const u8 as *const libc::c_char,
        b"@%u\0" as *const u8 as *const libc::c_char,
        (*w).id,
    );
    format_add(
        ft,
        b"window_name\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*w).name,
    );
    format_add(
        ft,
        b"window_width\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*w).sx,
    );
    format_add(
        ft,
        b"window_height\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*w).sy,
    );
    format_add_cb(
        ft,
        b"window_layout\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_layout
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add_cb(
        ft,
        b"window_visible_layout\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_visible_layout
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add(
        ft,
        b"window_panes\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        window_count_panes(w),
    );
    format_add(
        ft,
        b"window_zoomed_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*w).flags & 0x1000 as libc::c_int != 0) as libc::c_int,
    );
}
unsafe extern "C" fn format_defaults_winlink(
    mut ft: *mut format_tree,
    mut wl: *mut winlink,
) {
    let mut s: *mut session = (*wl).session;
    let mut w: *mut window = (*wl).window;
    if ((*ft).w).is_null() {
        (*ft).w = (*wl).window;
    }
    (*ft).wl = wl;
    format_defaults_window(ft, w);
    format_add(
        ft,
        b"window_index\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*wl).idx,
    );
    format_add_cb(
        ft,
        b"window_stack_index\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_stack_index
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add(
        ft,
        b"window_flags\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        window_printable_flags(wl),
    );
    format_add(
        ft,
        b"window_active\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (wl == (*s).curw) as libc::c_int,
    );
    format_add(
        ft,
        b"window_bell_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wl).flags & 0x1 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"window_activity_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wl).flags & 0x2 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"window_silence_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wl).flags & 0x4 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"window_last_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (wl == (*s).lastw.tqh_first) as libc::c_int,
    );
    format_add(
        ft,
        b"window_linked\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        session_is_linked(s, (*wl).window),
    );
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults_pane(
    mut ft: *mut format_tree,
    mut wp: *mut window_pane,
) {
    let mut gd: *mut grid = (*wp).base.grid;
    let mut status: libc::c_int = (*wp).status;
    let mut idx: u_int = 0;
    if ((*ft).w).is_null() {
        (*ft).w = (*wp).window;
    }
    (*ft).wp = wp;
    format_add(
        ft,
        b"history_size\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*gd).hsize,
    );
    format_add(
        ft,
        b"history_limit\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*gd).hlimit,
    );
    format_add_cb(
        ft,
        b"history_bytes\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_history_bytes
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    if window_pane_index(wp, &mut idx) != 0 as libc::c_int {
        fatalx(b"index not found\0" as *const u8 as *const libc::c_char);
    }
    format_add(
        ft,
        b"pane_index\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        idx,
    );
    format_add(
        ft,
        b"pane_width\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).sx,
    );
    format_add(
        ft,
        b"pane_height\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).sy,
    );
    format_add(
        ft,
        b"pane_title\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*wp).base.title,
    );
    format_add(
        ft,
        b"pane_id\0" as *const u8 as *const libc::c_char,
        b"%%%u\0" as *const u8 as *const libc::c_char,
        (*wp).id,
    );
    format_add(
        ft,
        b"pane_active\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (wp == (*(*wp).window).active) as libc::c_int,
    );
    format_add(
        ft,
        b"pane_input_off\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).flags & 0x40 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"pane_pipe\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).pipe_fd != -(1 as libc::c_int)) as libc::c_int,
    );
    if (*wp).flags & 0x200 as libc::c_int != 0
        && status & 0x7f as libc::c_int == 0 as libc::c_int
    {
        format_add(
            ft,
            b"pane_dead_status\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        );
    }
    format_add(
        ft,
        b"pane_dead\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).fd == -(1 as libc::c_int)) as libc::c_int,
    );
    if window_pane_visible(wp) != 0 {
        format_add(
            ft,
            b"pane_left\0" as *const u8 as *const libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            (*wp).xoff,
        );
        format_add(
            ft,
            b"pane_top\0" as *const u8 as *const libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            (*wp).yoff,
        );
        format_add(
            ft,
            b"pane_right\0" as *const u8 as *const libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            ((*wp).xoff).wrapping_add((*wp).sx).wrapping_sub(1 as libc::c_int as u_int),
        );
        format_add(
            ft,
            b"pane_bottom\0" as *const u8 as *const libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            ((*wp).yoff).wrapping_add((*wp).sy).wrapping_sub(1 as libc::c_int as u_int),
        );
        format_add(
            ft,
            b"pane_at_left\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            ((*wp).xoff == 0 as libc::c_int as u_int) as libc::c_int,
        );
        format_add(
            ft,
            b"pane_at_top\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            ((*wp).yoff == 0 as libc::c_int as u_int) as libc::c_int,
        );
        format_add(
            ft,
            b"pane_at_right\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            (((*wp).xoff).wrapping_add((*wp).sx) == (*(*wp).window).sx) as libc::c_int,
        );
        format_add(
            ft,
            b"pane_at_bottom\0" as *const u8 as *const libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            (((*wp).yoff).wrapping_add((*wp).sy) == (*(*wp).window).sy) as libc::c_int,
        );
    }
    format_add(
        ft,
        b"pane_in_mode\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).screen != &mut (*wp).base as *mut screen) as libc::c_int,
    );
    if !((*wp).mode).is_null() {
        format_add(
            ft,
            b"pane_mode\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*(*wp).mode).name,
        );
    }
    format_add(
        ft,
        b"pane_synchronized\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (options_get_number(
            (*(*wp).window).options,
            b"synchronize-panes\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int,
    );
    if !((*wp).searchstr).is_null() {
        format_add(
            ft,
            b"pane_search_string\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*wp).searchstr,
        );
    }
    format_add(
        ft,
        b"pane_tty\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        ((*wp).tty).as_mut_ptr(),
    );
    format_add(
        ft,
        b"pane_pid\0" as *const u8 as *const libc::c_char,
        b"%ld\0" as *const u8 as *const libc::c_char,
        (*wp).pid as libc::c_long,
    );
    format_add_cb(
        ft,
        b"pane_start_command\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_start_command
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add_cb(
        ft,
        b"pane_current_command\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_current_command
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add_cb(
        ft,
        b"pane_current_path\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_current_path
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
    format_add(
        ft,
        b"cursor_x\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).base.cx,
    );
    format_add(
        ft,
        b"cursor_y\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).base.cy,
    );
    format_add(
        ft,
        b"scroll_region_upper\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).base.rupper,
    );
    format_add(
        ft,
        b"scroll_region_lower\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).base.rlower,
    );
    window_copy_add_formats(wp, ft);
    format_add(
        ft,
        b"alternate_on\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        if !((*wp).saved_grid).is_null() { 1 as libc::c_int } else { 0 as libc::c_int },
    );
    format_add(
        ft,
        b"alternate_saved_x\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).saved_cx,
    );
    format_add(
        ft,
        b"alternate_saved_y\0" as *const u8 as *const libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wp).saved_cy,
    );
    format_add(
        ft,
        b"cursor_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x1 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"insert_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x2 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"keypad_cursor_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x4 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"keypad_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x8 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"wrap_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x10 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_any_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode
            & (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int) != 0)
            as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_standard_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x20 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_button_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x40 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_all_flag\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x1000 as libc::c_int != 0) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"pane_tabs\0" as *const u8 as *const libc::c_char,
        Some(
            format_cb_pane_tabs
                as unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn format_defaults_paste_buffer(
    mut ft: *mut format_tree,
    mut pb: *mut paste_buffer,
) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut size: size_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = tv.tv_usec;
    tv.tv_sec = paste_buffer_created(pb);
    paste_buffer_data(pb, &mut size);
    format_add(
        ft,
        b"buffer_size\0" as *const u8 as *const libc::c_char,
        b"%zu\0" as *const u8 as *const libc::c_char,
        size,
    );
    format_add(
        ft,
        b"buffer_name\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        paste_buffer_name(pb),
    );
    format_add_tv(ft, b"buffer_created\0" as *const u8 as *const libc::c_char, &mut tv);
    s = paste_make_sample(pb);
    format_add(
        ft,
        b"buffer_sample\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        s,
    );
    free(s as *mut libc::c_void);
}
