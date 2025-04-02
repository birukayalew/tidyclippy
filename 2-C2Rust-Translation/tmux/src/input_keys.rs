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
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn key_string_lookup_key(_: key_code) -> *const libc::c_char;
    fn xterm_keys_lookup(_: key_code) -> *mut libc::c_char;
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    fn utf8_split(_: wchar_t, _: *mut utf8_data) -> utf8_state;
    fn log_debug(_: *const libc::c_char, _: ...);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type wchar_t = libc::c_int;
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_key_ent {
    pub key: key_code,
    pub data: *const libc::c_char,
    pub flags: libc::c_int,
}
static mut input_keys: [input_key_ent; 78] = [
    {
        let mut init = input_key_ent {
            key: KEYC_BSPACE as libc::c_int as key_code,
            data: b"\x7F\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_PASTE_START as libc::c_int as key_code,
            data: b"\x1B[200~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_PASTE_END as libc::c_int as key_code,
            data: b"\x1B[201~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F1 as libc::c_int as key_code,
            data: b"\x1BOP\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F2 as libc::c_int as key_code,
            data: b"\x1BOQ\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F3 as libc::c_int as key_code,
            data: b"\x1BOR\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F4 as libc::c_int as key_code,
            data: b"\x1BOS\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F5 as libc::c_int as key_code,
            data: b"\x1B[15~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F6 as libc::c_int as key_code,
            data: b"\x1B[17~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F7 as libc::c_int as key_code,
            data: b"\x1B[18~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F8 as libc::c_int as key_code,
            data: b"\x1B[19~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F9 as libc::c_int as key_code,
            data: b"\x1B[20~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F10 as libc::c_int as key_code,
            data: b"\x1B[21~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F11 as libc::c_int as key_code,
            data: b"\x1B[23~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F12 as libc::c_int as key_code,
            data: b"\x1B[24~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F1 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[25~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F2 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[26~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F3 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[28~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F4 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[29~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F5 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[31~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F6 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[32~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F7 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[33~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_F8 as libc::c_int as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong,
            data: b"\x1B[34~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_IC as libc::c_int as key_code,
            data: b"\x1B[2~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_DC as libc::c_int as key_code,
            data: b"\x1B[3~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_HOME as libc::c_int as key_code,
            data: b"\x1B[1~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_END as libc::c_int as key_code,
            data: b"\x1B[4~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_NPAGE as libc::c_int as key_code,
            data: b"\x1B[6~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_PPAGE as libc::c_int as key_code,
            data: b"\x1B[5~\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_BTAB as libc::c_int as key_code,
            data: b"\x1B[Z\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1B[A\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1B[B\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1B[C\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1B[D\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_UP as libc::c_int as key_code,
            data: b"\x1BOA\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_DOWN as libc::c_int as key_code,
            data: b"\x1BOB\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_RIGHT as libc::c_int as key_code,
            data: b"\x1BOC\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_LEFT as libc::c_int as key_code,
            data: b"\x1BOD\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_UP as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1BOA\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_DOWN as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1BOB\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_RIGHT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1BOC\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_LEFT as libc::c_int as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1BOD\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_UP as libc::c_int as key_code,
            data: b"\x1B[A\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_DOWN as libc::c_int as key_code,
            data: b"\x1B[B\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_RIGHT as libc::c_int as key_code,
            data: b"\x1B[C\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_LEFT as libc::c_int as key_code,
            data: b"\x1B[D\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_SLASH as libc::c_int as key_code,
            data: b"\x1BOo\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_STAR as libc::c_int as key_code,
            data: b"\x1BOj\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_MINUS as libc::c_int as key_code,
            data: b"\x1BOm\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_SEVEN as libc::c_int as key_code,
            data: b"\x1BOw\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_EIGHT as libc::c_int as key_code,
            data: b"\x1BOx\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_NINE as libc::c_int as key_code,
            data: b"\x1BOy\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_PLUS as libc::c_int as key_code,
            data: b"\x1BOk\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_FOUR as libc::c_int as key_code,
            data: b"\x1BOt\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_FIVE as libc::c_int as key_code,
            data: b"\x1BOu\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_SIX as libc::c_int as key_code,
            data: b"\x1BOv\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_ONE as libc::c_int as key_code,
            data: b"\x1BOq\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_TWO as libc::c_int as key_code,
            data: b"\x1BOr\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_THREE as libc::c_int as key_code,
            data: b"\x1BOs\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_ENTER as libc::c_int as key_code,
            data: b"\x1BOM\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_ZERO as libc::c_int as key_code,
            data: b"\x1BOp\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_PERIOD as libc::c_int as key_code,
            data: b"\x1BOn\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_SLASH as libc::c_int as key_code,
            data: b"/\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_STAR as libc::c_int as key_code,
            data: b"*\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_MINUS as libc::c_int as key_code,
            data: b"-\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_SEVEN as libc::c_int as key_code,
            data: b"7\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_EIGHT as libc::c_int as key_code,
            data: b"8\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_NINE as libc::c_int as key_code,
            data: b"9\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_PLUS as libc::c_int as key_code,
            data: b"+\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_FOUR as libc::c_int as key_code,
            data: b"4\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_FIVE as libc::c_int as key_code,
            data: b"5\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_SIX as libc::c_int as key_code,
            data: b"6\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_ONE as libc::c_int as key_code,
            data: b"1\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_TWO as libc::c_int as key_code,
            data: b"2\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_THREE as libc::c_int as key_code,
            data: b"3\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_ENTER as libc::c_int as key_code,
            data: b"\n\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_ZERO as libc::c_int as key_code,
            data: b"0\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = input_key_ent {
            key: KEYC_KP_PERIOD as libc::c_int as key_code,
            data: b".\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn input_split2(mut c: u_int, mut dst: *mut u_char) -> size_t {
    if c > 0x7f as libc::c_int as u_int {
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = (c >> 6 as libc::c_int | 0xc0 as libc::c_int as u_int) as u_char;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = (c & 0x3f as libc::c_int as u_int | 0x80 as libc::c_int as u_int)
            as u_char;
        return 2 as libc::c_int as size_t;
    }
    *dst.offset(0 as libc::c_int as isize) = c as u_char;
    return 1 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn input_key(
    mut wp: *mut window_pane,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut ike: *const input_key_ent = 0 as *const input_key_ent;
    let mut i: u_int = 0;
    let mut dlen: size_t = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut justkey: key_code = 0;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    log_debug(
        b"writing key 0x%llx (%s) to %%%u\0" as *const u8 as *const libc::c_char,
        key,
        key_string_lookup_key(key),
        (*wp).id,
    );
    if key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
        && key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_int as libc::c_ulonglong
    {
        if !m.is_null() && (*m).wp != -(1 as libc::c_int) && (*m).wp as u_int == (*wp).id
        {
            input_key_mouse(wp, m);
        }
        return;
    }
    justkey = key
        & !(0x1000000000000 as libc::c_ulonglong | 0x200000000000 as libc::c_ulonglong);
    if justkey <= 0x7f as libc::c_int as key_code {
        if key & 0x200000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(
                (*wp).event,
                b"\x1B\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        ud.data[0 as libc::c_int as usize] = justkey as u_char;
        bufferevent_write(
            (*wp).event,
            &mut *(ud.data).as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
                as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        return;
    }
    if justkey > 0x7f as libc::c_int as key_code
        && justkey < 0x10000000 as libc::c_ulonglong
    {
        if utf8_split(justkey as wchar_t, &mut ud) as libc::c_uint
            != UTF8_DONE as libc::c_int as libc::c_uint
        {
            return;
        }
        if key & 0x200000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(
                (*wp).event,
                b"\x1B\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        bufferevent_write(
            (*wp).event,
            (ud.data).as_mut_ptr() as *const libc::c_void,
            ud.size as size_t,
        );
        return;
    }
    if options_get_number(
        (*(*wp).window).options,
        b"xterm-keys\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        out = xterm_keys_lookup(key);
        if !out.is_null() {
            bufferevent_write((*wp).event, out as *const libc::c_void, strlen(out));
            free(out as *mut libc::c_void);
            return;
        }
    }
    key &= !(0x1000000000000 as libc::c_ulonglong);
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[input_key_ent; 78]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<input_key_ent>() as libc::c_ulong)
    {
        ike = &*input_keys.as_ptr().offset(i as isize) as *const input_key_ent;
        if !((*ike).flags & 0x1 as libc::c_int != 0
            && (*(*wp).screen).mode & 0x8 as libc::c_int == 0)
        {
            if !((*ike).flags & 0x2 as libc::c_int != 0
                && (*(*wp).screen).mode & 0x4 as libc::c_int == 0)
            {
                if key & 0x200000000000 as libc::c_ulonglong != 0
                    && (*ike).key | 0x200000000000 as libc::c_ulonglong == key
                {
                    break;
                }
                if (*ike).key == key {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if i as libc::c_ulong
        == (::core::mem::size_of::<[input_key_ent; 78]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<input_key_ent>() as libc::c_ulong)
    {
        log_debug(b"key 0x%llx missing\0" as *const u8 as *const libc::c_char, key);
        return;
    }
    dlen = strlen((*ike).data);
    log_debug(
        b"found key 0x%llx: \"%s\"\0" as *const u8 as *const libc::c_char,
        key,
        (*ike).data,
    );
    if key & 0x200000000000 as libc::c_ulonglong != 0 {
        bufferevent_write(
            (*wp).event,
            b"\x1B\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    bufferevent_write((*wp).event, (*ike).data as *const libc::c_void, dlen);
}
unsafe extern "C" fn input_key_mouse(mut wp: *mut window_pane, mut m: *mut mouse_event) {
    let mut s: *mut screen = (*wp).screen;
    let mut mode: libc::c_int = (*s).mode;
    let mut buf: [libc::c_char; 40] = [0; 40];
    let mut len: size_t = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if mode & (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int)
        == 0 as libc::c_int
    {
        return;
    }
    if window_pane_visible(wp) == 0 {
        return;
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    if (*m).b & 32 as libc::c_int as u_int != 0
        && mode & (0x40 as libc::c_int | 0x1000 as libc::c_int) == 0 as libc::c_int
    {
        return;
    }
    if (*m).sgr_type != ' ' as i32 as u_int {
        if (*m).sgr_b & 32 as libc::c_int as u_int != 0
            && (*m).sgr_b & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int
            && !mode & 0x1000 as libc::c_int != 0
        {
            return;
        }
    } else if (*m).b & 32 as libc::c_int as u_int != 0
        && (*m).b & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int
        && (*m).lb & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int
        && !mode & 0x1000 as libc::c_int != 0
    {
        return
    }
    if (*m).sgr_type != ' ' as i32 as u_int && (*s).mode & 0x200 as libc::c_int != 0 {
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            b"\x1B[<%u;%u;%u%c\0" as *const u8 as *const libc::c_char,
            (*m).sgr_b,
            x.wrapping_add(1 as libc::c_int as u_int),
            y.wrapping_add(1 as libc::c_int as u_int),
            (*m).sgr_type,
        ) as size_t;
    } else if (*s).mode & 0x100 as libc::c_int != 0 {
        if (*m).b > (0x7ff as libc::c_int - 32 as libc::c_int) as u_int
            || x > (0x7ff as libc::c_int - 33 as libc::c_int) as u_int
            || y > (0x7ff as libc::c_int - 33 as libc::c_int) as u_int
        {
            return;
        }
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            b"\x1B[M\0" as *const u8 as *const libc::c_char,
        ) as size_t;
        len = len
            .wrapping_add(
                input_split2(
                    ((*m).b).wrapping_add(32 as libc::c_int as u_int),
                    &mut *buf.as_mut_ptr().offset(len as isize) as *mut libc::c_char
                        as *mut u_char,
                ),
            );
        len = len
            .wrapping_add(
                input_split2(
                    x.wrapping_add(33 as libc::c_int as u_int),
                    &mut *buf.as_mut_ptr().offset(len as isize) as *mut libc::c_char
                        as *mut u_char,
                ),
            );
        len = len
            .wrapping_add(
                input_split2(
                    y.wrapping_add(33 as libc::c_int as u_int),
                    &mut *buf.as_mut_ptr().offset(len as isize) as *mut libc::c_char
                        as *mut u_char,
                ),
            );
    } else {
        if (*m).b > 223 as libc::c_int as u_int {
            return;
        }
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            b"\x1B[M\0" as *const u8 as *const libc::c_char,
        ) as size_t;
        let fresh0 = len;
        len = len.wrapping_add(1);
        buf[fresh0
            as usize] = ((*m).b).wrapping_add(32 as libc::c_int as u_int)
            as libc::c_char;
        let fresh1 = len;
        len = len.wrapping_add(1);
        buf[fresh1
            as usize] = x.wrapping_add(33 as libc::c_int as u_int) as libc::c_char;
        let fresh2 = len;
        len = len.wrapping_add(1);
        buf[fresh2
            as usize] = y.wrapping_add(33 as libc::c_int as u_int) as libc::c_char;
    }
    log_debug(
        b"writing mouse %.*s to %%%u\0" as *const u8 as *const libc::c_char,
        len as libc::c_int,
        buf.as_mut_ptr(),
        (*wp).id,
    );
    bufferevent_write((*wp).event, buf.as_mut_ptr() as *const libc::c_void, len);
}
