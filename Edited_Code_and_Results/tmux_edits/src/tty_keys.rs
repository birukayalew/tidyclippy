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
    pub type format_tree;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type options_entry;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_pending(
        ev: *const event,
        events: libc::c_short,
        tv: *mut timeval,
    ) -> libc::c_int;
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
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    static mut global_options: *mut options;
    fn options_get(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_array_get(_: *mut options_entry, _: u_int) -> *const libc::c_char;
    fn options_array_size(_: *mut options_entry, _: *mut u_int) -> libc::c_int;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn tty_set_type(_: *mut tty, _: libc::c_int);
    fn tty_term_string(_: *mut tty_term, _: tty_code_code) -> *const libc::c_char;
    fn key_string_lookup_key(_: key_code) -> *const libc::c_char;
    fn server_client_handle_key(_: *mut client, _: key_code);
    fn xterm_keys_find(
        _: *const libc::c_char,
        _: size_t,
        _: *mut size_t,
        _: *mut key_code,
    ) -> libc::c_int;
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_combine(_: *const utf8_data, _: *mut wchar_t) -> utf8_state;
    fn log_debug(_: *const libc::c_char, _: ...);
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
pub type wchar_t = libc::c_int;
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
    pub args: C2RustUnnamed_31,
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
pub struct C2RustUnnamed_31 {
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
    pub term_type: C2RustUnnamed_32,
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
pub type C2RustUnnamed_32 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_32 = 6;
pub const TTY_VT420: C2RustUnnamed_32 = 5;
pub const TTY_VT320: C2RustUnnamed_32 = 4;
pub const TTY_VT220: C2RustUnnamed_32 = 3;
pub const TTY_VT102: C2RustUnnamed_32 = 2;
pub const TTY_VT101: C2RustUnnamed_32 = 1;
pub const TTY_VT100: C2RustUnnamed_32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type C2RustUnnamed_34 = libc::c_uint;
pub const KEYC_KP_PERIOD: C2RustUnnamed_34 = 268435564;
pub const KEYC_KP_ZERO: C2RustUnnamed_34 = 268435563;
pub const KEYC_KP_ENTER: C2RustUnnamed_34 = 268435562;
pub const KEYC_KP_THREE: C2RustUnnamed_34 = 268435561;
pub const KEYC_KP_TWO: C2RustUnnamed_34 = 268435560;
pub const KEYC_KP_ONE: C2RustUnnamed_34 = 268435559;
pub const KEYC_KP_SIX: C2RustUnnamed_34 = 268435558;
pub const KEYC_KP_FIVE: C2RustUnnamed_34 = 268435557;
pub const KEYC_KP_FOUR: C2RustUnnamed_34 = 268435556;
pub const KEYC_KP_PLUS: C2RustUnnamed_34 = 268435555;
pub const KEYC_KP_NINE: C2RustUnnamed_34 = 268435554;
pub const KEYC_KP_EIGHT: C2RustUnnamed_34 = 268435553;
pub const KEYC_KP_SEVEN: C2RustUnnamed_34 = 268435552;
pub const KEYC_KP_MINUS: C2RustUnnamed_34 = 268435551;
pub const KEYC_KP_STAR: C2RustUnnamed_34 = 268435550;
pub const KEYC_KP_SLASH: C2RustUnnamed_34 = 268435549;
pub const KEYC_RIGHT: C2RustUnnamed_34 = 268435548;
pub const KEYC_LEFT: C2RustUnnamed_34 = 268435547;
pub const KEYC_DOWN: C2RustUnnamed_34 = 268435546;
pub const KEYC_UP: C2RustUnnamed_34 = 268435545;
pub const KEYC_BTAB: C2RustUnnamed_34 = 268435544;
pub const KEYC_PPAGE: C2RustUnnamed_34 = 268435543;
pub const KEYC_NPAGE: C2RustUnnamed_34 = 268435542;
pub const KEYC_END: C2RustUnnamed_34 = 268435541;
pub const KEYC_HOME: C2RustUnnamed_34 = 268435540;
pub const KEYC_DC: C2RustUnnamed_34 = 268435539;
pub const KEYC_IC: C2RustUnnamed_34 = 268435538;
pub const KEYC_F12: C2RustUnnamed_34 = 268435537;
pub const KEYC_F11: C2RustUnnamed_34 = 268435536;
pub const KEYC_F10: C2RustUnnamed_34 = 268435535;
pub const KEYC_F9: C2RustUnnamed_34 = 268435534;
pub const KEYC_F8: C2RustUnnamed_34 = 268435533;
pub const KEYC_F7: C2RustUnnamed_34 = 268435532;
pub const KEYC_F6: C2RustUnnamed_34 = 268435531;
pub const KEYC_F5: C2RustUnnamed_34 = 268435530;
pub const KEYC_F4: C2RustUnnamed_34 = 268435529;
pub const KEYC_F3: C2RustUnnamed_34 = 268435528;
pub const KEYC_F2: C2RustUnnamed_34 = 268435527;
pub const KEYC_F1: C2RustUnnamed_34 = 268435526;
pub const KEYC_BSPACE: C2RustUnnamed_34 = 268435525;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed_34 = 268435524;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed_34 = 268435523;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed_34 = 268435522;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed_34 = 268435521;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed_34 = 268435520;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed_34 = 268435519;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed_34 = 268435518;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed_34 = 268435517;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed_34 = 268435516;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed_34 = 268435515;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed_34 = 268435514;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed_34 = 268435513;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed_34 = 268435512;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed_34 = 268435511;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed_34 = 268435510;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed_34 = 268435509;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed_34 = 268435508;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed_34 = 268435507;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed_34 = 268435506;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed_34 = 268435505;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed_34 = 268435504;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed_34 = 268435503;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed_34 = 268435502;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed_34 = 268435501;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed_34 = 268435500;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed_34 = 268435499;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed_34 = 268435498;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed_34 = 268435497;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed_34 = 268435496;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed_34 = 268435495;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed_34 = 268435494;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed_34 = 268435493;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed_34 = 268435492;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed_34 = 268435491;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed_34 = 268435490;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed_34 = 268435489;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed_34 = 268435488;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed_34 = 268435487;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed_34 = 268435486;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed_34 = 268435485;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed_34 = 268435484;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed_34 = 268435483;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed_34 = 268435482;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed_34 = 268435481;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed_34 = 268435480;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed_34 = 268435479;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed_34 = 268435478;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed_34 = 268435477;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed_34 = 268435476;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed_34 = 268435475;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed_34 = 268435474;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed_34 = 268435473;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed_34 = 268435472;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed_34 = 268435471;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed_34 = 268435470;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed_34 = 268435469;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed_34 = 268435468;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed_34 = 268435467;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed_34 = 268435466;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed_34 = 268435465;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed_34 = 268435464;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed_34 = 268435463;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed_34 = 268435462;
pub const KEYC_DRAGGING: C2RustUnnamed_34 = 268435461;
pub const KEYC_MOUSE: C2RustUnnamed_34 = 268435460;
pub const KEYC_PASTE_END: C2RustUnnamed_34 = 268435459;
pub const KEYC_PASTE_START: C2RustUnnamed_34 = 268435458;
pub const KEYC_FOCUS_OUT: C2RustUnnamed_34 = 268435457;
pub const KEYC_FOCUS_IN: C2RustUnnamed_34 = 268435456;
pub type tty_code_code = libc::c_uint;
pub const TTYC_XT: tty_code_code = 207;
pub const TTYC_XENL: tty_code_code = 206;
pub const TTYC_VPA: tty_code_code = 205;
pub const TTYC_U8: tty_code_code = 204;
pub const TTYC_TSL: tty_code_code = 203;
pub const TTYC_TC: tty_code_code = 202;
pub const TTYC_SS: tty_code_code = 201;
pub const TTYC_SMXX: tty_code_code = 200;
pub const TTYC_SMUL: tty_code_code = 199;
pub const TTYC_SMSO: tty_code_code = 198;
pub const TTYC_SMKX: tty_code_code = 197;
pub const TTYC_SMCUP: tty_code_code = 196;
pub const TTYC_SMACS: tty_code_code = 195;
pub const TTYC_SITM: tty_code_code = 194;
pub const TTYC_SGR0: tty_code_code = 193;
pub const TTYC_SETRGBF: tty_code_code = 192;
pub const TTYC_SETRGBB: tty_code_code = 191;
pub const TTYC_SETAF: tty_code_code = 190;
pub const TTYC_SETAB: tty_code_code = 189;
pub const TTYC_SE: tty_code_code = 188;
pub const TTYC_RMKX: tty_code_code = 187;
pub const TTYC_RMCUP: tty_code_code = 186;
pub const TTYC_RMACS: tty_code_code = 185;
pub const TTYC_RI: tty_code_code = 184;
pub const TTYC_RGB: tty_code_code = 183;
pub const TTYC_REV: tty_code_code = 182;
pub const TTYC_OP: tty_code_code = 181;
pub const TTYC_MS: tty_code_code = 180;
pub const TTYC_KUP7: tty_code_code = 179;
pub const TTYC_KUP6: tty_code_code = 178;
pub const TTYC_KUP5: tty_code_code = 177;
pub const TTYC_KUP4: tty_code_code = 176;
pub const TTYC_KUP3: tty_code_code = 175;
pub const TTYC_KUP2: tty_code_code = 174;
pub const TTYC_KRIT7: tty_code_code = 173;
pub const TTYC_KRIT6: tty_code_code = 172;
pub const TTYC_KRIT5: tty_code_code = 171;
pub const TTYC_KRIT4: tty_code_code = 170;
pub const TTYC_KRIT3: tty_code_code = 169;
pub const TTYC_KRIT2: tty_code_code = 168;
pub const TTYC_KRI: tty_code_code = 167;
pub const TTYC_KPRV7: tty_code_code = 166;
pub const TTYC_KPRV6: tty_code_code = 165;
pub const TTYC_KPRV5: tty_code_code = 164;
pub const TTYC_KPRV4: tty_code_code = 163;
pub const TTYC_KPRV3: tty_code_code = 162;
pub const TTYC_KPRV2: tty_code_code = 161;
pub const TTYC_KPP: tty_code_code = 160;
pub const TTYC_KNXT7: tty_code_code = 159;
pub const TTYC_KNXT6: tty_code_code = 158;
pub const TTYC_KNXT5: tty_code_code = 157;
pub const TTYC_KNXT4: tty_code_code = 156;
pub const TTYC_KNXT3: tty_code_code = 155;
pub const TTYC_KNXT2: tty_code_code = 154;
pub const TTYC_KNP: tty_code_code = 153;
pub const TTYC_KMOUS: tty_code_code = 152;
pub const TTYC_KLFT7: tty_code_code = 151;
pub const TTYC_KLFT6: tty_code_code = 150;
pub const TTYC_KLFT5: tty_code_code = 149;
pub const TTYC_KLFT4: tty_code_code = 148;
pub const TTYC_KLFT3: tty_code_code = 147;
pub const TTYC_KLFT2: tty_code_code = 146;
pub const TTYC_KIND: tty_code_code = 145;
pub const TTYC_KICH1: tty_code_code = 144;
pub const TTYC_KIC7: tty_code_code = 143;
pub const TTYC_KIC6: tty_code_code = 142;
pub const TTYC_KIC5: tty_code_code = 141;
pub const TTYC_KIC4: tty_code_code = 140;
pub const TTYC_KIC3: tty_code_code = 139;
pub const TTYC_KIC2: tty_code_code = 138;
pub const TTYC_KHOME: tty_code_code = 137;
pub const TTYC_KHOM7: tty_code_code = 136;
pub const TTYC_KHOM6: tty_code_code = 135;
pub const TTYC_KHOM5: tty_code_code = 134;
pub const TTYC_KHOM4: tty_code_code = 133;
pub const TTYC_KHOM3: tty_code_code = 132;
pub const TTYC_KHOM2: tty_code_code = 131;
pub const TTYC_KF9: tty_code_code = 130;
pub const TTYC_KF8: tty_code_code = 129;
pub const TTYC_KF7: tty_code_code = 128;
pub const TTYC_KF63: tty_code_code = 127;
pub const TTYC_KF62: tty_code_code = 126;
pub const TTYC_KF61: tty_code_code = 125;
pub const TTYC_KF60: tty_code_code = 124;
pub const TTYC_KF6: tty_code_code = 123;
pub const TTYC_KF59: tty_code_code = 122;
pub const TTYC_KF58: tty_code_code = 121;
pub const TTYC_KF57: tty_code_code = 120;
pub const TTYC_KF56: tty_code_code = 119;
pub const TTYC_KF55: tty_code_code = 118;
pub const TTYC_KF54: tty_code_code = 117;
pub const TTYC_KF53: tty_code_code = 116;
pub const TTYC_KF52: tty_code_code = 115;
pub const TTYC_KF51: tty_code_code = 114;
pub const TTYC_KF50: tty_code_code = 113;
pub const TTYC_KF5: tty_code_code = 112;
pub const TTYC_KF49: tty_code_code = 111;
pub const TTYC_KF48: tty_code_code = 110;
pub const TTYC_KF47: tty_code_code = 109;
pub const TTYC_KF46: tty_code_code = 108;
pub const TTYC_KF45: tty_code_code = 107;
pub const TTYC_KF44: tty_code_code = 106;
pub const TTYC_KF43: tty_code_code = 105;
pub const TTYC_KF42: tty_code_code = 104;
pub const TTYC_KF41: tty_code_code = 103;
pub const TTYC_KF40: tty_code_code = 102;
pub const TTYC_KF4: tty_code_code = 101;
pub const TTYC_KF39: tty_code_code = 100;
pub const TTYC_KF38: tty_code_code = 99;
pub const TTYC_KF37: tty_code_code = 98;
pub const TTYC_KF36: tty_code_code = 97;
pub const TTYC_KF35: tty_code_code = 96;
pub const TTYC_KF34: tty_code_code = 95;
pub const TTYC_KF33: tty_code_code = 94;
pub const TTYC_KF32: tty_code_code = 93;
pub const TTYC_KF31: tty_code_code = 92;
pub const TTYC_KF30: tty_code_code = 91;
pub const TTYC_KF3: tty_code_code = 90;
pub const TTYC_KF29: tty_code_code = 89;
pub const TTYC_KF28: tty_code_code = 88;
pub const TTYC_KF27: tty_code_code = 87;
pub const TTYC_KF26: tty_code_code = 86;
pub const TTYC_KF25: tty_code_code = 85;
pub const TTYC_KF24: tty_code_code = 84;
pub const TTYC_KF23: tty_code_code = 83;
pub const TTYC_KF22: tty_code_code = 82;
pub const TTYC_KF21: tty_code_code = 81;
pub const TTYC_KF20: tty_code_code = 80;
pub const TTYC_KF2: tty_code_code = 79;
pub const TTYC_KF19: tty_code_code = 78;
pub const TTYC_KF18: tty_code_code = 77;
pub const TTYC_KF17: tty_code_code = 76;
pub const TTYC_KF16: tty_code_code = 75;
pub const TTYC_KF15: tty_code_code = 74;
pub const TTYC_KF14: tty_code_code = 73;
pub const TTYC_KF13: tty_code_code = 72;
pub const TTYC_KF12: tty_code_code = 71;
pub const TTYC_KF11: tty_code_code = 70;
pub const TTYC_KF10: tty_code_code = 69;
pub const TTYC_KF1: tty_code_code = 68;
pub const TTYC_KEND7: tty_code_code = 67;
pub const TTYC_KEND6: tty_code_code = 66;
pub const TTYC_KEND5: tty_code_code = 65;
pub const TTYC_KEND4: tty_code_code = 64;
pub const TTYC_KEND3: tty_code_code = 63;
pub const TTYC_KEND2: tty_code_code = 62;
pub const TTYC_KEND: tty_code_code = 61;
pub const TTYC_KDN7: tty_code_code = 60;
pub const TTYC_KDN6: tty_code_code = 59;
pub const TTYC_KDN5: tty_code_code = 58;
pub const TTYC_KDN4: tty_code_code = 57;
pub const TTYC_KDN3: tty_code_code = 56;
pub const TTYC_KDN2: tty_code_code = 55;
pub const TTYC_KDCH1: tty_code_code = 54;
pub const TTYC_KDC7: tty_code_code = 53;
pub const TTYC_KDC6: tty_code_code = 52;
pub const TTYC_KDC5: tty_code_code = 51;
pub const TTYC_KDC4: tty_code_code = 50;
pub const TTYC_KDC3: tty_code_code = 49;
pub const TTYC_KDC2: tty_code_code = 48;
pub const TTYC_KCUU1: tty_code_code = 47;
pub const TTYC_KCUF1: tty_code_code = 46;
pub const TTYC_KCUD1: tty_code_code = 45;
pub const TTYC_KCUB1: tty_code_code = 44;
pub const TTYC_KCBT: tty_code_code = 43;
pub const TTYC_INVIS: tty_code_code = 42;
pub const TTYC_INDN: tty_code_code = 41;
pub const TTYC_IL1: tty_code_code = 40;
pub const TTYC_IL: tty_code_code = 39;
pub const TTYC_ICH1: tty_code_code = 38;
pub const TTYC_ICH: tty_code_code = 37;
pub const TTYC_HPA: tty_code_code = 36;
pub const TTYC_HOME: tty_code_code = 35;
pub const TTYC_FSL: tty_code_code = 34;
pub const TTYC_ENACS: tty_code_code = 33;
pub const TTYC_EL1: tty_code_code = 32;
pub const TTYC_EL: tty_code_code = 31;
pub const TTYC_ED: tty_code_code = 30;
pub const TTYC_ECH: tty_code_code = 29;
pub const TTYC_E3: tty_code_code = 28;
pub const TTYC_DL1: tty_code_code = 27;
pub const TTYC_DL: tty_code_code = 26;
pub const TTYC_DIM: tty_code_code = 25;
pub const TTYC_DCH1: tty_code_code = 24;
pub const TTYC_DCH: tty_code_code = 23;
pub const TTYC_CVVIS: tty_code_code = 22;
pub const TTYC_CUU1: tty_code_code = 21;
pub const TTYC_CUU: tty_code_code = 20;
pub const TTYC_CUP: tty_code_code = 19;
pub const TTYC_CUF1: tty_code_code = 18;
pub const TTYC_CUF: tty_code_code = 17;
pub const TTYC_CUD1: tty_code_code = 16;
pub const TTYC_CUD: tty_code_code = 15;
pub const TTYC_CUB1: tty_code_code = 14;
pub const TTYC_CUB: tty_code_code = 13;
pub const TTYC_CSR: tty_code_code = 12;
pub const TTYC_CS: tty_code_code = 11;
pub const TTYC_CR: tty_code_code = 10;
pub const TTYC_COLORS: tty_code_code = 9;
pub const TTYC_CNORM: tty_code_code = 8;
pub const TTYC_CLEAR: tty_code_code = 7;
pub const TTYC_CIVIS: tty_code_code = 6;
pub const TTYC_BOLD: tty_code_code = 5;
pub const TTYC_BLINK: tty_code_code = 4;
pub const TTYC_BEL: tty_code_code = 3;
pub const TTYC_BCE: tty_code_code = 2;
pub const TTYC_ACSC: tty_code_code = 1;
pub const TTYC_AX: tty_code_code = 0;
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_default_key_code {
    pub code: tty_code_code,
    pub key: key_code,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_default_key_raw {
    pub string: *const libc::c_char,
    pub key: key_code,
}
static mut tty_default_raw_keys: [tty_default_key_raw; 94] = [
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOo\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SLASH as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOj\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_STAR as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOm\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_MINUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOw\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SEVEN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOx\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_EIGHT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOy\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_NINE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOk\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_PLUS as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOt\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_FOUR as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOu\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_FIVE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOv\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SIX as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOq\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ONE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOr\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_TWO as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOs\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_THREE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOM\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ENTER as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOp\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ZERO as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOn\0" as *const u8 as *const libc::c_char,
            key: KEYC_KP_PERIOD as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOA\0" as *const u8 as *const libc::c_char,
            key: KEYC_UP as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOB\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOWN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOC\0" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOD\0" as *const u8 as *const libc::c_char,
            key: KEYC_LEFT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[A\0" as *const u8 as *const libc::c_char,
            key: KEYC_UP as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[B\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOWN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[C\0" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[D\0" as *const u8 as *const libc::c_char,
            key: KEYC_LEFT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOH\0" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOF\0" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[H\0" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[F\0" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOa\0" as *const u8 as *const libc::c_char,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOb\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOc\0" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1BOd\0" as *const u8 as *const libc::c_char,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[a\0" as *const u8 as *const libc::c_char,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[b\0" as *const u8 as *const libc::c_char,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[c\0" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[d\0" as *const u8 as *const libc::c_char,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[11^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[12^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[13^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[14^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[15^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[17^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[18^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[19^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[20^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[21^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[23^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[24^\0" as *const u8 as *const libc::c_char,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[2^\0" as *const u8 as *const libc::c_char,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[3^\0" as *const u8 as *const libc::c_char,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[7^\0" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[8^\0" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[6^\0" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[5^\0" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[11$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[12$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[13$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[14$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[15$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[17$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[18$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[19$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[20$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[21$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[23$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[24$\0" as *const u8 as *const libc::c_char,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[2$\0" as *const u8 as *const libc::c_char,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[3$\0" as *const u8 as *const libc::c_char,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[7$\0" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[8$\0" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[6$\0" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[5$\0" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[11@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[12@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[13@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[14@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[15@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[17@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[18@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[19@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[20@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[21@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[23@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[24@\0" as *const u8 as *const libc::c_char,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[2@\0" as *const u8 as *const libc::c_char,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[3@\0" as *const u8 as *const libc::c_char,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[7@\0" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[8@\0" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[6@\0" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[5@\0" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[I\0" as *const u8 as *const libc::c_char,
            key: KEYC_FOCUS_IN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[O\0" as *const u8 as *const libc::c_char,
            key: KEYC_FOCUS_OUT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[200~\0" as *const u8 as *const libc::c_char,
            key: KEYC_PASTE_START as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_raw {
            string: b"\x1B[201~\0" as *const u8 as *const libc::c_char,
            key: KEYC_PASTE_END as libc::c_int as key_code,
        };
        init
    },
];
static mut tty_default_code_keys: [tty_default_key_code; 136] = [
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF1,
            key: KEYC_F1 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF2,
            key: KEYC_F2 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF3,
            key: KEYC_F3 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF4,
            key: KEYC_F4 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF5,
            key: KEYC_F5 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF6,
            key: KEYC_F6 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF7,
            key: KEYC_F7 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF8,
            key: KEYC_F8 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF9,
            key: KEYC_F9 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF10,
            key: KEYC_F10 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF11,
            key: KEYC_F11 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF12,
            key: KEYC_F12 as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF13,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF14,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF15,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF16,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF17,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF18,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF19,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF20,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF21,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF22,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF23,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF24,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF25,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF26,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF27,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF28,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF29,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF30,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF31,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF32,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF33,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF34,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF35,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF36,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF37,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF38,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF39,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF40,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF41,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF42,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF43,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF44,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF45,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF46,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF47,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF48,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF49,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF50,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF51,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF52,
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF53,
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF54,
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF55,
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF56,
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF57,
            key: KEYC_F9 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF58,
            key: KEYC_F10 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF59,
            key: KEYC_F11 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF60,
            key: KEYC_F12 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF61,
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF62,
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KF63,
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KICH1,
            key: KEYC_IC as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDCH1,
            key: KEYC_DC as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOME,
            key: KEYC_HOME as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND,
            key: KEYC_END as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNP,
            key: KEYC_NPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPP,
            key: KEYC_PPAGE as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KCBT,
            key: KEYC_BTAB as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KCUU1,
            key: KEYC_UP as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KCUD1,
            key: KEYC_DOWN as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KCUB1,
            key: KEYC_LEFT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KCUF1,
            key: KEYC_RIGHT as libc::c_int as key_code,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDC2,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDC3,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDC4,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDC5,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDC6,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDC7,
            key: KEYC_DC as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIND,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDN2,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDN3,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDN4,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDN5,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDN6,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KDN7,
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND2,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND3,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND4,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND5,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND6,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KEND7,
            key: KEYC_END as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOM2,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOM3,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOM4,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOM5,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOM6,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KHOM7,
            key: KEYC_HOME as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIC2,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIC3,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIC4,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIC5,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIC6,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KIC7,
            key: KEYC_IC as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KLFT2,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KLFT3,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KLFT4,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KLFT5,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KLFT6,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KLFT7,
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNXT2,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNXT3,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNXT4,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNXT5,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNXT6,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KNXT7,
            key: KEYC_NPAGE as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPRV2,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPRV3,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPRV4,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPRV5,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPRV6,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KPRV7,
            key: KEYC_PPAGE as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRIT2,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRIT3,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRIT4,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRIT5,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRIT6,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRIT7,
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KRI,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KUP2,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KUP3,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KUP4,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KUP5,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KUP6,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = tty_default_key_code {
            code: TTYC_KUP7,
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x200000000000 as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong,
        };
        init
    },
];
unsafe extern "C" fn tty_keys_add(
    mut tty: *mut tty,
    mut s: *const libc::c_char,
    mut key: key_code,
) {
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    let mut size: size_t = 0;
    let mut keystr: *const libc::c_char = 0 as *const libc::c_char;
    keystr = key_string_lookup_key(key);
    tk = tty_keys_find(tty, s, strlen(s), &mut size);
    if tk.is_null() {
        log_debug(
            b"new key %s: 0x%llx (%s)\0" as *const u8 as *const libc::c_char,
            s,
            key,
            keystr,
        );
        tty_keys_add1(&mut (*tty).key_tree, s, key);
    } else {
        log_debug(
            b"replacing key %s: 0x%llx (%s)\0" as *const u8 as *const libc::c_char,
            s,
            key,
            keystr,
        );
        (*tk).key = key;
    };
}
unsafe extern "C" fn tty_keys_add1(
    mut tkp: *mut *mut tty_key,
    mut s: *const libc::c_char,
    mut key: key_code,
) {
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    tk = *tkp;
    if tk.is_null() {
        *tkp = xcalloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<tty_key>() as libc::c_ulong,
        ) as *mut tty_key;
        tk = *tkp;
        (*tk).ch = *s;
        (*tk).key = 0xfffe00000000 as libc::c_ulonglong;
    }
    if *s as libc::c_int == (*tk).ch as libc::c_int {
        s = s.offset(1);
        s;
        if *s as libc::c_int == '\0' as i32 {
            (*tk).key = key;
            return;
        }
        tkp = &mut (*tk).next;
    } else if (*s as libc::c_int) < (*tk).ch as libc::c_int {
        tkp = &mut (*tk).left;
    } else if *s as libc::c_int > (*tk).ch as libc::c_int {
        tkp = &mut (*tk).right;
    }
    tty_keys_add1(tkp, s, key);
}
#[no_mangle]
pub unsafe extern "C" fn tty_keys_build(mut tty: *mut tty) {
    let mut tdkr: *const tty_default_key_raw = 0 as *const tty_default_key_raw;
    let mut tdkc: *const tty_default_key_code = 0 as *const tty_default_key_code;
    let mut i: u_int = 0;
    let mut size: u_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if !((*tty).key_tree).is_null() {
        tty_keys_free(tty);
    }
    (*tty).key_tree = 0 as *mut tty_key;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[tty_default_key_raw; 94]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<tty_default_key_raw>() as libc::c_ulong)
    {
        tdkr = &*tty_default_raw_keys.as_ptr().offset(i as isize)
            as *const tty_default_key_raw;
        s = (*tdkr).string;
        if *s as libc::c_int != '\0' as i32 {
            tty_keys_add(tty, s, (*tdkr).key);
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[tty_default_key_code; 136]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<tty_default_key_code>() as libc::c_ulong,
            )
    {
        tdkc = &*tty_default_code_keys.as_ptr().offset(i as isize)
            as *const tty_default_key_code;
        s = tty_term_string((*tty).term, (*tdkc).code);
        if *s as libc::c_int != '\0' as i32 {
            tty_keys_add(tty, s, (*tdkc).key);
        }
        i = i.wrapping_add(1);
        i;
    }
    o = options_get(global_options, b"user-keys\0" as *const u8 as *const libc::c_char);
    if !o.is_null() && options_array_size(o, &mut size) != -(1 as libc::c_int) {
        i = 0 as libc::c_int as u_int;
        while i < size {
            value = options_array_get(o, i);
            if !value.is_null() {
                tty_keys_add(
                    tty,
                    value,
                    (0x20000000 as libc::c_ulonglong)
                        .wrapping_add(i as libc::c_ulonglong),
                );
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_keys_free(mut tty: *mut tty) {
    tty_keys_free1((*tty).key_tree);
}
unsafe extern "C" fn tty_keys_free1(mut tk: *mut tty_key) {
    if !((*tk).next).is_null() {
        tty_keys_free1((*tk).next);
    }
    if !((*tk).left).is_null() {
        tty_keys_free1((*tk).left);
    }
    if !((*tk).right).is_null() {
        tty_keys_free1((*tk).right);
    }
    free(tk as *mut libc::c_void);
}
unsafe extern "C" fn tty_keys_find(
    mut tty: *mut tty,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut size: *mut size_t,
) -> *mut tty_key {
    *size = 0 as libc::c_int as size_t;
    return tty_keys_find1((*tty).key_tree, buf, len, size);
}
unsafe extern "C" fn tty_keys_find1(
    mut tk: *mut tty_key,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut size: *mut size_t,
) -> *mut tty_key {
    if tk.is_null() {
        return 0 as *mut tty_key;
    }
    if (*tk).ch as libc::c_int == *buf as libc::c_int {
        buf = buf.offset(1);
        buf;
        len = len.wrapping_sub(1);
        len;
        *size = (*size).wrapping_add(1);
        *size;
        if len == 0 as libc::c_int as size_t
            || ((*tk).next).is_null() && (*tk).key != 0xfffe00000000 as libc::c_ulonglong
        {
            return tk;
        }
        tk = (*tk).next;
    } else if (*buf as libc::c_int) < (*tk).ch as libc::c_int {
        tk = (*tk).left;
    } else if *buf as libc::c_int > (*tk).ch as libc::c_int {
        tk = (*tk).right;
    }
    return tty_keys_find1(tk, buf, len, size);
}
unsafe extern "C" fn tty_keys_next1(
    mut tty: *mut tty,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut key: *mut key_code,
    mut size: *mut size_t,
    mut expired: libc::c_int,
) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    let mut tk1: *mut tty_key = 0 as *mut tty_key;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut more: utf8_state = UTF8_MORE;
    let mut i: u_int = 0;
    let mut wc: wchar_t = 0;
    let mut n: libc::c_int = 0;
    log_debug(
        b"%s: next key is %zu (%.*s) (expired=%d)\0" as *const u8 as *const libc::c_char,
        (*c).name,
        len,
        len as libc::c_int,
        buf,
        expired,
    );
    tk = tty_keys_find(tty, buf, len, size);
    if !tk.is_null() && (*tk).key != 0xfffe00000000 as libc::c_ulonglong {
        tk1 = tk;
        loop {
            log_debug(
                b"%s: keys in list: %#llx\0" as *const u8 as *const libc::c_char,
                (*c).name,
                (*tk1).key,
            );
            tk1 = (*tk1).next;
            if tk1.is_null() {
                break;
            }
        }
        if !((*tk).next).is_null() && expired == 0 {
            return 1 as libc::c_int;
        }
        *key = (*tk).key;
        return 0 as libc::c_int;
    }
    n = xterm_keys_find(buf, len, size, key);
    if n == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if n == 1 as libc::c_int && expired == 0 {
        return 1 as libc::c_int;
    }
    more = utf8_open(&mut ud, *buf as u_char);
    if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
        *size = ud.size as size_t;
        if len < ud.size as size_t {
            if expired == 0 {
                return 1 as libc::c_int;
            }
            return -(1 as libc::c_int);
        }
        i = 1 as libc::c_int as u_int;
        while i < ud.size as u_int {
            more = utf8_append(&mut ud, *buf.offset(i as isize) as u_char);
            i = i.wrapping_add(1);
            i;
        }
        if more as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        if utf8_combine(&mut ud, &mut wc) as libc::c_uint
            != UTF8_DONE as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int);
        }
        *key = wc as key_code;
        log_debug(
            b"%s: UTF-8 key %.*s %#llx\0" as *const u8 as *const libc::c_char,
            (*c).name,
            ud.size as libc::c_int,
            buf,
            *key,
        );
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_keys_next(mut tty: *mut tty) -> key_code {
    let mut current_block: u64;
    let mut c: *mut client = (*tty).client;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut size: size_t = 0;
    let mut bspace: cc_t = 0;
    let mut delay: libc::c_int = 0;
    let mut expired: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut key: key_code = 0;
    buf = evbuffer_pullup((*tty).in_0, -(1 as libc::c_int) as ssize_t)
        as *const libc::c_char;
    len = evbuffer_get_length((*tty).in_0);
    if len == 0 as libc::c_int as size_t {
        return 0 as libc::c_int as key_code;
    }
    log_debug(
        b"%s: keys are %zu (%.*s)\0" as *const u8 as *const libc::c_char,
        (*c).name,
        len,
        len as libc::c_int,
        buf,
    );
    match tty_keys_device_attributes(tty, buf, len, &mut size) {
        0 => {
            key = 0xfffe00000000 as libc::c_ulonglong;
            current_block = 10452571879034897134;
        }
        -1 => {
            current_block = 13183875560443969876;
        }
        1 => {
            current_block = 15364893815785058502;
        }
        _ => {
            current_block = 13183875560443969876;
        }
    }
    match current_block {
        13183875560443969876 => {
            match tty_keys_mouse(tty, buf, len, &mut size) {
                0 => {
                    current_block = 17183070534439490203;
                    match current_block {
                        12770421059189612756 => {
                            current_block = 15364893815785058502;
                        }
                        17183070534439490203 => {
                            key = KEYC_MOUSE as libc::c_int as key_code;
                            current_block = 10452571879034897134;
                        }
                        _ => {
                            key = KEYC_MOUSE as libc::c_int as key_code;
                            log_debug(
                                b"%s: discard key %.*s %#llx\0" as *const u8
                                    as *const libc::c_char,
                                (*c).name,
                                size as libc::c_int,
                                buf,
                                key,
                            );
                            evbuffer_drain((*tty).in_0, size);
                            return 1 as libc::c_int as key_code;
                        }
                    }
                }
                -2 => {
                    current_block = 5224479313017524111;
                    match current_block {
                        12770421059189612756 => {
                            current_block = 15364893815785058502;
                        }
                        17183070534439490203 => {
                            key = KEYC_MOUSE as libc::c_int as key_code;
                            current_block = 10452571879034897134;
                        }
                        _ => {
                            key = KEYC_MOUSE as libc::c_int as key_code;
                            log_debug(
                                b"%s: discard key %.*s %#llx\0" as *const u8
                                    as *const libc::c_char,
                                (*c).name,
                                size as libc::c_int,
                                buf,
                                key,
                            );
                            evbuffer_drain((*tty).in_0, size);
                            return 1 as libc::c_int as key_code;
                        }
                    }
                }
                1 => {
                    current_block = 12770421059189612756;
                    match current_block {
                        12770421059189612756 => {
                            current_block = 15364893815785058502;
                        }
                        17183070534439490203 => {
                            key = KEYC_MOUSE as libc::c_int as key_code;
                            current_block = 10452571879034897134;
                        }
                        _ => {
                            key = KEYC_MOUSE as libc::c_int as key_code;
                            log_debug(
                                b"%s: discard key %.*s %#llx\0" as *const u8
                                    as *const libc::c_char,
                                (*c).name,
                                size as libc::c_int,
                                buf,
                                key,
                            );
                            evbuffer_drain((*tty).in_0, size);
                            return 1 as libc::c_int as key_code;
                        }
                    }
                }
                -1 | _ => {
                    current_block = 13163678995505928440;
                }
            }
        }
        _ => {}
    }
    loop {
        match current_block {
            13163678995505928440 => {
                n = tty_keys_next1(tty, buf, len, &mut key, &mut size, expired);
                if n == 0 as libc::c_int {
                    current_block = 10452571879034897134;
                    continue;
                }
                if n == 1 as libc::c_int {
                    current_block = 15364893815785058502;
                    continue;
                }
                if *buf as libc::c_int == '\u{1b}' as i32 {
                    n = tty_keys_next1(
                        tty,
                        buf.offset(1 as libc::c_int as isize),
                        len.wrapping_sub(1 as libc::c_int as size_t),
                        &mut key,
                        &mut size,
                        expired,
                    );
                    if n == 0 as libc::c_int {
                        if key & 0x1000000000000 as libc::c_ulonglong != 0 {
                            key = '\u{1b}' as i32 as key_code;
                            size = 1 as libc::c_int as size_t;
                            current_block = 10452571879034897134;
                            continue;
                        } else {
                            key |= 0x200000000000 as libc::c_ulonglong;
                            size = size.wrapping_add(1);
                            size;
                            current_block = 10452571879034897134;
                            continue;
                        }
                    } else if n == 1 as libc::c_int {
                        current_block = 15364893815785058502;
                        continue;
                    }
                }
                if *buf as libc::c_int == '\u{1b}' as i32
                    && len >= 2 as libc::c_int as size_t
                {
                    key = *buf.offset(1 as libc::c_int as isize) as u_char
                        as libc::c_ulonglong | 0x200000000000 as libc::c_ulonglong;
                    size = 2 as libc::c_int as size_t;
                } else {
                    key = *buf.offset(0 as libc::c_int as isize) as u_char as key_code;
                    size = 1 as libc::c_int as size_t;
                }
                current_block = 10452571879034897134;
            }
            10452571879034897134 => {
                log_debug(
                    b"%s: complete key %.*s %#llx\0" as *const u8 as *const libc::c_char,
                    (*c).name,
                    size as libc::c_int,
                    buf,
                    key,
                );
                bspace = (*tty).tio.c_cc[2 as libc::c_int as usize];
                if bspace as libc::c_int != '\0' as i32
                    && key
                        & !(0x200000000000 as libc::c_ulonglong
                            | 0x400000000000 as libc::c_ulonglong
                            | 0x800000000000 as libc::c_ulonglong
                            | 0x1000000000000 as libc::c_ulonglong)
                        == bspace as libc::c_ulonglong
                {
                    key = key
                        & (0x200000000000 as libc::c_ulonglong
                            | 0x400000000000 as libc::c_ulonglong
                            | 0x800000000000 as libc::c_ulonglong
                            | 0x1000000000000 as libc::c_ulonglong)
                        | KEYC_BSPACE as libc::c_int as libc::c_ulonglong;
                }
                evbuffer_drain((*tty).in_0, size);
                if event_initialized(&mut (*tty).key_timer) != 0 {
                    event_del(&mut (*tty).key_timer);
                }
                (*tty).flags &= !(0x4 as libc::c_int);
                if key == KEYC_FOCUS_OUT as libc::c_int as key_code {
                    (*(*tty).client).flags &= !(0x8000 as libc::c_int);
                    return 1 as libc::c_int as key_code;
                } else if key == KEYC_FOCUS_IN as libc::c_int as key_code {
                    (*(*tty).client).flags |= 0x8000 as libc::c_int;
                    return 1 as libc::c_int as key_code;
                }
                if key != 0xfffe00000000 as libc::c_ulonglong {
                    server_client_handle_key((*tty).client, key);
                }
                return 1 as libc::c_int as key_code;
            }
            _ => {
                log_debug(
                    b"%s: partial key %.*s\0" as *const u8 as *const libc::c_char,
                    (*c).name,
                    len as libc::c_int,
                    buf,
                );
                if (*tty).flags & 0x4 as libc::c_int != 0 {
                    if event_initialized(&mut (*tty).key_timer) != 0
                        && event_pending(
                            &mut (*tty).key_timer,
                            0x1 as libc::c_int as libc::c_short,
                            0 as *mut timeval,
                        ) == 0
                    {
                        expired = 1 as libc::c_int;
                        current_block = 13163678995505928440;
                    } else {
                        return 0 as libc::c_int as key_code
                    }
                } else {
                    delay = options_get_number(
                        global_options,
                        b"escape-time\0" as *const u8 as *const libc::c_char,
                    ) as libc::c_int;
                    tv.tv_sec = (delay / 1000 as libc::c_int) as __time_t;
                    tv
                        .tv_usec = (delay % 1000 as libc::c_int) as libc::c_long
                        * 1000 as libc::c_long;
                    if event_initialized(&mut (*tty).key_timer) != 0 {
                        event_del(&mut (*tty).key_timer);
                    }
                    event_set(
                        &mut (*tty).key_timer,
                        -(1 as libc::c_int),
                        0 as libc::c_int as libc::c_short,
                        Some(
                            tty_keys_callback
                                as unsafe extern "C" fn(
                                    libc::c_int,
                                    libc::c_short,
                                    *mut libc::c_void,
                                ) -> (),
                        ),
                        tty as *mut libc::c_void,
                    );
                    event_add(&mut (*tty).key_timer, &mut tv);
                    (*tty).flags |= 0x4 as libc::c_int;
                    return 0 as libc::c_int as key_code;
                }
            }
        }
    };
}
unsafe extern "C" fn tty_keys_callback(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut tty: *mut tty = data as *mut tty;
    if (*tty).flags & 0x4 as libc::c_int != 0 {
        while tty_keys_next(tty) != 0 {}
    }
}
unsafe extern "C" fn tty_keys_mouse(
    mut tty: *mut tty,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut size: *mut size_t,
) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut m: *mut mouse_event = &mut (*tty).mouse;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut b: u_int = 0;
    let mut sgr_b: u_int = 0;
    let mut sgr_type: u_char = 0;
    let mut ch: u_char = 0;
    *size = 0 as libc::c_int as size_t;
    sgr_b = 0 as libc::c_int as u_int;
    b = sgr_b;
    y = b;
    x = y;
    sgr_type = ' ' as i32 as u_char;
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int != '\u{1b}' as i32 {
        return -(1 as libc::c_int);
    }
    if len == 1 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        return -(1 as libc::c_int);
    }
    if len == 2 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int == 'M' as i32 {
        *size = 3 as libc::c_int as size_t;
        i = 0 as libc::c_int as u_int;
        while i < 3 as libc::c_int as u_int {
            if len <= *size {
                return 1 as libc::c_int;
            }
            let fresh0 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh0 as isize) as u_char;
            if i == 0 as libc::c_int as u_int {
                b = ch as u_int;
            } else if i == 1 as libc::c_int as u_int {
                x = ch as u_int;
            } else {
                y = ch as u_int;
            }
            i = i.wrapping_add(1);
            i;
        }
        log_debug(
            b"%s: mouse input: %.*s\0" as *const u8 as *const libc::c_char,
            (*c).name,
            *size as libc::c_int,
            buf,
        );
        if b < 32 as libc::c_int as u_int {
            return -(1 as libc::c_int);
        }
        b = b.wrapping_sub(32 as libc::c_int as u_int);
        if x >= 33 as libc::c_int as u_int {
            x = x.wrapping_sub(33 as libc::c_int as u_int);
        } else {
            x = (256 as libc::c_int as u_int).wrapping_sub(x);
        }
        if y >= 33 as libc::c_int as u_int {
            y = y.wrapping_sub(33 as libc::c_int as u_int);
        } else {
            y = (256 as libc::c_int as u_int).wrapping_sub(y);
        }
    } else if *buf.offset(2 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        *size = 3 as libc::c_int as size_t;
        loop {
            if len <= *size {
                return 1 as libc::c_int;
            }
            let fresh1 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh1 as isize) as u_char;
            if ch as libc::c_int == ';' as i32 {
                break;
            }
            if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                return -(1 as libc::c_int);
            }
            sgr_b = (10 as libc::c_int as u_int * sgr_b)
                .wrapping_add((ch as libc::c_int - '0' as i32) as u_int);
        }
        loop {
            if len <= *size {
                return 1 as libc::c_int;
            }
            let fresh2 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh2 as isize) as u_char;
            if ch as libc::c_int == ';' as i32 {
                break;
            }
            if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                return -(1 as libc::c_int);
            }
            x = (10 as libc::c_int as u_int * x)
                .wrapping_add((ch as libc::c_int - '0' as i32) as u_int);
        }
        loop {
            if len <= *size {
                return 1 as libc::c_int;
            }
            let fresh3 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh3 as isize) as u_char;
            if ch as libc::c_int == 'M' as i32 || ch as libc::c_int == 'm' as i32 {
                break;
            }
            if (ch as libc::c_int) < '0' as i32 || ch as libc::c_int > '9' as i32 {
                return -(1 as libc::c_int);
            }
            y = (10 as libc::c_int as u_int * y)
                .wrapping_add((ch as libc::c_int - '0' as i32) as u_int);
        }
        log_debug(
            b"%s: mouse input (SGR): %.*s\0" as *const u8 as *const libc::c_char,
            (*c).name,
            *size as libc::c_int,
            buf,
        );
        if x < 1 as libc::c_int as u_int || y < 1 as libc::c_int as u_int {
            return -(1 as libc::c_int);
        }
        x = x.wrapping_sub(1);
        x;
        y = y.wrapping_sub(1);
        y;
        b = sgr_b;
        sgr_type = ch;
        if sgr_type as libc::c_int == 'm' as i32 {
            b |= 3 as libc::c_int as u_int;
        }
        if sgr_type as libc::c_int == 'm' as i32
            && sgr_b & 64 as libc::c_int as u_int != 0
        {
            return -(2 as libc::c_int);
        }
    } else {
        return -(1 as libc::c_int)
    }
    (*m).lx = (*m).x;
    (*m).x = x;
    (*m).ly = (*m).y;
    (*m).y = y;
    (*m).lb = (*m).b;
    (*m).b = b;
    (*m).sgr_type = sgr_type as u_int;
    (*m).sgr_b = sgr_b;
    return 0 as libc::c_int;
}
unsafe extern "C" fn tty_keys_device_attributes(
    mut tty: *mut tty,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut size: *mut size_t,
) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut a: u_int = 0;
    let mut b: u_int = 0;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut types: [*const libc::c_char; 7] = [
        b"VT100\0" as *const u8 as *const libc::c_char,
        b"VT101\0" as *const u8 as *const libc::c_char,
        b"VT102\0" as *const u8 as *const libc::c_char,
        b"VT220\0" as *const u8 as *const libc::c_char,
        b"VT320\0" as *const u8 as *const libc::c_char,
        b"VT420\0" as *const u8 as *const libc::c_char,
        b"Unknown\0" as *const u8 as *const libc::c_char,
    ];
    let mut type_0: libc::c_int = 0;
    *size = 0 as libc::c_int as size_t;
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int != '\u{1b}' as i32 {
        return -(1 as libc::c_int);
    }
    if len == 1 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        return -(1 as libc::c_int);
    }
    if len == 2 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int != '?' as i32 {
        return -(1 as libc::c_int);
    }
    if len == 3 as libc::c_int as size_t {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && *buf.offset((3 as libc::c_int as u_int).wrapping_add(i) as isize)
            as libc::c_int != 'c' as i32
    {
        if (3 as libc::c_int as u_int).wrapping_add(i) as size_t == len {
            return 1 as libc::c_int;
        }
        tmp[i
            as usize] = *buf
            .offset((3 as libc::c_int as u_int).wrapping_add(i) as isize);
        i = i.wrapping_add(1);
        i;
    }
    if i as libc::c_ulong
        == (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    tmp[i as usize] = '\0' as i32 as libc::c_char;
    *size = (4 as libc::c_int as u_int).wrapping_add(i) as size_t;
    a = strtoul(tmp.as_mut_ptr(), &mut endptr, 10 as libc::c_int) as u_int;
    if *endptr as libc::c_int == ';' as i32 {
        b = strtoul(
            endptr.offset(1 as libc::c_int as isize),
            &mut endptr,
            10 as libc::c_int,
        ) as u_int;
        if *endptr as libc::c_int != '\0' as i32 && *endptr as libc::c_int != ';' as i32
        {
            b = 0 as libc::c_int as u_int;
        }
    } else {
        b = 0 as libc::c_int as u_int;
        a = b;
    }
    type_0 = TTY_UNKNOWN as libc::c_int;
    match a {
        1 => {
            if b == 2 as libc::c_int as u_int {
                type_0 = TTY_VT100 as libc::c_int;
            } else if b == 0 as libc::c_int as u_int {
                type_0 = TTY_VT101 as libc::c_int;
            }
        }
        6 => {
            type_0 = TTY_VT102 as libc::c_int;
        }
        62 => {
            type_0 = TTY_VT220 as libc::c_int;
        }
        63 => {
            type_0 = TTY_VT320 as libc::c_int;
        }
        64 => {
            type_0 = TTY_VT420 as libc::c_int;
        }
        _ => {}
    }
    tty_set_type(tty, type_0);
    log_debug(
        b"%s: received DA %.*s (%s)\0" as *const u8 as *const libc::c_char,
        (*c).name,
        *size as libc::c_int,
        buf,
        types[type_0 as usize],
    );
    return 0 as libc::c_int;
}
