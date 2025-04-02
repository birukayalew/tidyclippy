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
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpid() -> __pid_t;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn environ_find(_: *mut environ, _: *const libc::c_char) -> *mut environ_entry;
    fn cmd_mouse_window(_: *mut mouse_event, _: *mut *mut session) -> *mut winlink;
    fn cmd_mouse_pane(
        _: *mut mouse_event,
        _: *mut *mut session,
        _: *mut *mut winlink,
    ) -> *mut window_pane;
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    static mut clients: clients;
    static mut marked_pane: cmd_find_state;
    fn server_check_marked() -> libc::c_int;
    static mut all_window_panes: window_pane_tree;
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn window_pane_tree_RB_MINMAX(
        _: *mut window_pane_tree,
        _: libc::c_int,
    ) -> *mut window_pane;
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlink_next_by_number(
        _: *mut winlink,
        _: *mut session,
        _: libc::c_int,
    ) -> *mut winlink;
    fn winlink_previous_by_number(
        _: *mut winlink,
        _: *mut session,
        _: libc::c_int,
    ) -> *mut winlink;
    fn window_find_by_id_str(_: *const libc::c_char) -> *mut window;
    fn window_find_string(_: *mut window, _: *const libc::c_char) -> *mut window_pane;
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    fn window_pane_at_index(_: *mut window, _: u_int) -> *mut window_pane;
    fn window_pane_next_by_number(
        _: *mut window,
        _: *mut window_pane,
        _: u_int,
    ) -> *mut window_pane;
    fn window_pane_previous_by_number(
        _: *mut window,
        _: *mut window_pane,
        _: u_int,
    ) -> *mut window_pane;
    fn window_pane_find_by_id_str(_: *const libc::c_char) -> *mut window_pane;
    fn window_pane_find_up(_: *mut window_pane) -> *mut window_pane;
    fn window_pane_find_down(_: *mut window_pane) -> *mut window_pane;
    fn window_pane_find_left(_: *mut window_pane) -> *mut window_pane;
    fn window_pane_find_right(_: *mut window_pane) -> *mut window_pane;
    static mut sessions: sessions;
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    fn session_alive(_: *mut session) -> libc::c_int;
    fn session_find(_: *const libc::c_char) -> *mut session;
    fn session_find_by_id_str(_: *const libc::c_char) -> *mut session;
    fn session_find_by_id(_: u_int) -> *mut session;
    fn session_has(_: *mut session, _: *mut window) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
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
static mut cmd_find_session_table: [[*const libc::c_char; 2]; 1] = [
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];
static mut cmd_find_window_table: [[*const libc::c_char; 2]; 6] = [
    [
        b"{start}\0" as *const u8 as *const libc::c_char,
        b"^\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{last}\0" as *const u8 as *const libc::c_char,
        b"!\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{end}\0" as *const u8 as *const libc::c_char,
        b"$\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{next}\0" as *const u8 as *const libc::c_char,
        b"+\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{previous}\0" as *const u8 as *const libc::c_char,
        b"-\0" as *const u8 as *const libc::c_char,
    ],
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];
static mut cmd_find_pane_table: [[*const libc::c_char; 2]; 16] = [
    [
        b"{last}\0" as *const u8 as *const libc::c_char,
        b"!\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{next}\0" as *const u8 as *const libc::c_char,
        b"+\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{previous}\0" as *const u8 as *const libc::c_char,
        b"-\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{top}\0" as *const u8 as *const libc::c_char,
        b"top\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{bottom}\0" as *const u8 as *const libc::c_char,
        b"bottom\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{left}\0" as *const u8 as *const libc::c_char,
        b"left\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{right}\0" as *const u8 as *const libc::c_char,
        b"right\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{top-left}\0" as *const u8 as *const libc::c_char,
        b"top-left\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{top-right}\0" as *const u8 as *const libc::c_char,
        b"top-right\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{bottom-left}\0" as *const u8 as *const libc::c_char,
        b"bottom-left\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{bottom-right}\0" as *const u8 as *const libc::c_char,
        b"bottom-right\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{up-of}\0" as *const u8 as *const libc::c_char,
        b"{up-of}\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{down-of}\0" as *const u8 as *const libc::c_char,
        b"{down-of}\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{left-of}\0" as *const u8 as *const libc::c_char,
        b"{left-of}\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"{right-of}\0" as *const u8 as *const libc::c_char,
        b"{right-of}\0" as *const u8 as *const libc::c_char,
    ],
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];
unsafe extern "C" fn cmd_find_try_TMUX(mut c: *mut client) -> *mut session {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut pid: libc::c_longlong = 0;
    let mut session: u_int = 0;
    envent = environ_find((*c).environ, b"TMUX\0" as *const u8 as *const libc::c_char);
    if envent.is_null() {
        return 0 as *mut session;
    }
    if sscanf(
        (*envent).value,
        b"%255[^,],%lld,%d\0" as *const u8 as *const libc::c_char,
        tmp.as_mut_ptr(),
        &mut pid as *mut libc::c_longlong,
        &mut session as *mut u_int,
    ) != 3 as libc::c_int
    {
        return 0 as *mut session;
    }
    if pid != getpid() as libc::c_longlong {
        return 0 as *mut session;
    }
    log_debug(
        b"client %p TMUX %s (session @%u)\0" as *const u8 as *const libc::c_char,
        c,
        (*envent).value,
        session,
    );
    return session_find_by_id(session);
}
unsafe extern "C" fn cmd_find_inside_pane(mut c: *mut client) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if c.is_null() {
        return 0 as *mut window_pane;
    }
    wp = window_pane_tree_RB_MINMAX(&mut all_window_panes, -(1 as libc::c_int));
    while !wp.is_null() {
        if strcmp(((*wp).tty).as_mut_ptr(), (*c).ttyname) == 0 as libc::c_int {
            break;
        }
        wp = window_pane_tree_RB_NEXT(wp);
    }
    return wp;
}
unsafe extern "C" fn cmd_find_client_better(
    mut c: *mut client,
    mut than: *mut client,
) -> libc::c_int {
    if than.is_null() {
        return 1 as libc::c_int;
    }
    return if (*c).activity_time.tv_sec == (*than).activity_time.tv_sec {
        ((*c).activity_time.tv_usec > (*than).activity_time.tv_usec) as libc::c_int
    } else {
        ((*c).activity_time.tv_sec > (*than).activity_time.tv_sec) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_best_client(mut s: *mut session) -> *mut client {
    let mut c_loop: *mut client = 0 as *mut client;
    let mut c: *mut client = 0 as *mut client;
    if (*s).flags & 0x1 as libc::c_int != 0 {
        s = 0 as *mut session;
    }
    c = 0 as *mut client;
    c_loop = clients.tqh_first;
    while !c_loop.is_null() {
        if !((*c_loop).session).is_null() {
            if !(!s.is_null() && (*c_loop).session != s) {
                if cmd_find_client_better(c_loop, c) != 0 {
                    c = c_loop;
                }
            }
        }
        c_loop = (*c_loop).entry.tqe_next;
    }
    return c;
}
unsafe extern "C" fn cmd_find_session_better(
    mut s: *mut session,
    mut than: *mut session,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut attached: libc::c_int = 0;
    if than.is_null() {
        return 1 as libc::c_int;
    }
    if flags & 0x1 as libc::c_int != 0 {
        attached = !(*than).flags & 0x1 as libc::c_int;
        if attached != 0 && (*s).flags & 0x1 as libc::c_int != 0 {
            return 1 as libc::c_int
        } else if attached == 0 && !(*s).flags & 0x1 as libc::c_int != 0 {
            return 0 as libc::c_int
        }
    }
    return if (*s).activity_time.tv_sec == (*than).activity_time.tv_sec {
        ((*s).activity_time.tv_usec > (*than).activity_time.tv_usec) as libc::c_int
    } else {
        ((*s).activity_time.tv_sec > (*than).activity_time.tv_sec) as libc::c_int
    };
}
unsafe extern "C" fn cmd_find_best_session(
    mut slist: *mut *mut session,
    mut ssize: u_int,
    mut flags: libc::c_int,
) -> *mut session {
    let mut s_loop: *mut session = 0 as *mut session;
    let mut s: *mut session = 0 as *mut session;
    let mut i: u_int = 0;
    s = 0 as *mut session;
    if !slist.is_null() {
        i = 0 as libc::c_int as u_int;
        while i < ssize {
            if cmd_find_session_better(*slist.offset(i as isize), s, flags) != 0 {
                s = *slist.offset(i as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
        while !s_loop.is_null() {
            if cmd_find_session_better(s_loop, s, flags) != 0 {
                s = s_loop;
            }
            s_loop = sessions_RB_NEXT(s_loop);
        }
    }
    return s;
}
unsafe extern "C" fn cmd_find_best_session_with_window(
    mut fs: *mut cmd_find_state,
) -> libc::c_int {
    let mut slist: *mut *mut session = 0 as *mut *mut session;
    let mut ssize: u_int = 0;
    let mut s: *mut session = 0 as *mut session;
    ssize = 0 as libc::c_int as u_int;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        if !(session_has(s, (*fs).w) == 0) {
            slist = xreallocarray(
                slist as *mut libc::c_void,
                ssize.wrapping_add(1 as libc::c_int as u_int) as size_t,
                ::core::mem::size_of::<*mut session>() as libc::c_ulong,
            ) as *mut *mut session;
            let fresh0 = ssize;
            ssize = ssize.wrapping_add(1);
            let ref mut fresh1 = *slist.offset(fresh0 as isize);
            *fresh1 = s;
        }
        s = sessions_RB_NEXT(s);
    }
    if !(ssize == 0 as libc::c_int as u_int) {
        (*fs).s = cmd_find_best_session(slist, ssize, (*fs).flags);
        if !((*fs).s).is_null() {
            free(slist as *mut libc::c_void);
            return cmd_find_best_winlink_with_window(fs);
        }
    }
    free(slist as *mut libc::c_void);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_find_best_winlink_with_window(
    mut fs: *mut cmd_find_state,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl_loop: *mut winlink = 0 as *mut winlink;
    wl = 0 as *mut winlink;
    if !((*(*fs).s).curw).is_null() && (*(*(*fs).s).curw).window == (*fs).w {
        wl = (*(*fs).s).curw;
    } else {
        wl_loop = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
        while !wl_loop.is_null() {
            if (*wl_loop).window == (*fs).w {
                wl = wl_loop;
                break;
            } else {
                wl_loop = winlinks_RB_NEXT(wl_loop);
            }
        }
    }
    if wl.is_null() {
        return -(1 as libc::c_int);
    }
    (*fs).wl = wl;
    (*fs).idx = (*(*fs).wl).idx;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_find_map_table(
    mut table: *mut [*const libc::c_char; 2],
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while !((*table.offset(i as isize))[0 as libc::c_int as usize]).is_null() {
        if strcmp(s, (*table.offset(i as isize))[0 as libc::c_int as usize])
            == 0 as libc::c_int
        {
            return (*table.offset(i as isize))[1 as libc::c_int as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
unsafe extern "C" fn cmd_find_get_session(
    mut fs: *mut cmd_find_state,
    mut session: *const libc::c_char,
) -> libc::c_int {
    let mut s: *mut session = 0 as *mut session;
    let mut s_loop: *mut session = 0 as *mut session;
    let mut c: *mut client = 0 as *mut client;
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"cmd_find_get_session\0"))
            .as_ptr(),
        session,
    );
    if *session as libc::c_int == '$' as i32 {
        (*fs).s = session_find_by_id_str(session);
        if ((*fs).s).is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    (*fs).s = session_find(session);
    if !((*fs).s).is_null() {
        return 0 as libc::c_int;
    }
    c = cmd_find_client(0 as *mut cmdq_item, session, 1 as libc::c_int);
    if !c.is_null() && !((*c).session).is_null() {
        (*fs).s = (*c).session;
        return 0 as libc::c_int;
    }
    if (*fs).flags & 0x10 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    s = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if strncmp(session, (*s_loop).name, strlen(session)) == 0 as libc::c_int {
            if !s.is_null() {
                return -(1 as libc::c_int);
            }
            s = s_loop;
        }
        s_loop = sessions_RB_NEXT(s_loop);
    }
    if !s.is_null() {
        (*fs).s = s;
        return 0 as libc::c_int;
    }
    s = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if fnmatch(session, (*s_loop).name, 0 as libc::c_int) == 0 as libc::c_int {
            if !s.is_null() {
                return -(1 as libc::c_int);
            }
            s = s_loop;
        }
        s_loop = sessions_RB_NEXT(s_loop);
    }
    if !s.is_null() {
        (*fs).s = s;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_find_get_window(
    mut fs: *mut cmd_find_state,
    mut window: *const libc::c_char,
    mut only: libc::c_int,
) -> libc::c_int {
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"cmd_find_get_window\0"))
            .as_ptr(),
        window,
    );
    if *window as libc::c_int == '@' as i32 {
        (*fs).w = window_find_by_id_str(window);
        if ((*fs).w).is_null() {
            return -(1 as libc::c_int);
        }
        return cmd_find_best_session_with_window(fs);
    }
    (*fs).s = (*(*fs).current).s;
    if cmd_find_get_window_with_session(fs, window) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if only == 0 && cmd_find_get_session(fs, window) == 0 as libc::c_int {
        (*fs).wl = (*(*fs).s).curw;
        (*fs).w = (*(*fs).wl).window;
        if !(*fs).flags & 0x4 as libc::c_int != 0 {
            (*fs).idx = (*(*fs).wl).idx;
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_find_get_window_with_session(
    mut fs: *mut cmd_find_state,
    mut window: *const libc::c_char,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut s: *mut session = 0 as *mut session;
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 33],
            &[libc::c_char; 33],
        >(b"cmd_find_get_window_with_session\0"))
            .as_ptr(),
        window,
    );
    exact = (*fs).flags & 0x20 as libc::c_int;
    (*fs).wl = (*(*fs).s).curw;
    (*fs).w = (*(*fs).wl).window;
    if *window as libc::c_int == '@' as i32 {
        (*fs).w = window_find_by_id_str(window);
        if ((*fs).w).is_null() || session_has((*fs).s, (*fs).w) == 0 {
            return -(1 as libc::c_int);
        }
        return cmd_find_best_winlink_with_window(fs);
    }
    if exact == 0
        && (*window.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            || *window.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
    {
        if *window.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            n = strtonum(
                window.offset(1 as libc::c_int as isize),
                1 as libc::c_int as libc::c_longlong,
                2147483647 as libc::c_int as libc::c_longlong,
                0 as *mut *const libc::c_char,
            ) as libc::c_int;
        } else {
            n = 1 as libc::c_int;
        }
        s = (*fs).s;
        if (*fs).flags & 0x4 as libc::c_int != 0 {
            if *window.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
                if 2147483647 as libc::c_int - (*(*s).curw).idx < n {
                    return -(1 as libc::c_int);
                }
                (*fs).idx = (*(*s).curw).idx + n;
            } else {
                if n > (*(*s).curw).idx {
                    return -(1 as libc::c_int);
                }
                (*fs).idx = (*(*s).curw).idx - n;
            }
            return 0 as libc::c_int;
        }
        if *window.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
            (*fs).wl = winlink_next_by_number((*s).curw, s, n);
        } else {
            (*fs).wl = winlink_previous_by_number((*s).curw, s, n);
        }
        if !((*fs).wl).is_null() {
            (*fs).idx = (*(*fs).wl).idx;
            (*fs).w = (*(*fs).wl).window;
            return 0 as libc::c_int;
        }
    }
    if exact == 0 {
        if strcmp(window, b"!\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*fs).wl = (*(*fs).s).lastw.tqh_first;
            if ((*fs).wl).is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).idx = (*(*fs).wl).idx;
            (*fs).w = (*(*fs).wl).window;
            return 0 as libc::c_int;
        } else if strcmp(window, b"^\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*fs).wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
            if ((*fs).wl).is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).idx = (*(*fs).wl).idx;
            (*fs).w = (*(*fs).wl).window;
            return 0 as libc::c_int;
        } else if strcmp(window, b"$\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*fs).wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, 1 as libc::c_int);
            if ((*fs).wl).is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).idx = (*(*fs).wl).idx;
            (*fs).w = (*(*fs).wl).window;
            return 0 as libc::c_int;
        }
    }
    if *window.offset(0 as libc::c_int as isize) as libc::c_int != '+' as i32
        && *window.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
    {
        idx = strtonum(
            window,
            0 as libc::c_int as libc::c_longlong,
            2147483647 as libc::c_int as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        if errstr.is_null() {
            if (*fs).flags & 0x4 as libc::c_int != 0 {
                (*fs).idx = idx;
                return 0 as libc::c_int;
            }
            (*fs).wl = winlink_find_by_index(&mut (*(*fs).s).windows, idx);
            if !((*fs).wl).is_null() {
                (*fs).w = (*(*fs).wl).window;
                return 0 as libc::c_int;
            }
        }
    }
    (*fs).wl = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if strcmp(window, (*(*wl).window).name) == 0 as libc::c_int {
            if !((*fs).wl).is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).wl = wl;
        }
        wl = winlinks_RB_NEXT(wl);
    }
    if !((*fs).wl).is_null() {
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return 0 as libc::c_int;
    }
    if exact != 0 {
        return -(1 as libc::c_int);
    }
    (*fs).wl = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if strncmp(window, (*(*wl).window).name, strlen(window)) == 0 as libc::c_int {
            if !((*fs).wl).is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).wl = wl;
        }
        wl = winlinks_RB_NEXT(wl);
    }
    if !((*fs).wl).is_null() {
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return 0 as libc::c_int;
    }
    (*fs).wl = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if fnmatch(window, (*(*wl).window).name, 0 as libc::c_int) == 0 as libc::c_int {
            if !((*fs).wl).is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).wl = wl;
        }
        wl = winlinks_RB_NEXT(wl);
    }
    if !((*fs).wl).is_null() {
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_find_get_pane(
    mut fs: *mut cmd_find_state,
    mut pane: *const libc::c_char,
    mut only: libc::c_int,
) -> libc::c_int {
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"cmd_find_get_pane\0"))
            .as_ptr(),
        pane,
    );
    if *pane as libc::c_int == '%' as i32 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        (*fs).w = (*(*fs).wp).window;
        return cmd_find_best_session_with_window(fs);
    }
    (*fs).s = (*(*fs).current).s;
    (*fs).wl = (*(*fs).current).wl;
    (*fs).idx = (*(*fs).current).idx;
    (*fs).w = (*(*fs).current).w;
    if cmd_find_get_pane_with_window(fs, pane) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if only == 0 && cmd_find_get_window(fs, pane, 0 as libc::c_int) == 0 as libc::c_int {
        (*fs).wp = (*(*fs).w).active;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_find_get_pane_with_session(
    mut fs: *mut cmd_find_state,
    mut pane: *const libc::c_char,
) -> libc::c_int {
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 31],
            &[libc::c_char; 31],
        >(b"cmd_find_get_pane_with_session\0"))
            .as_ptr(),
        pane,
    );
    if *pane as libc::c_int == '%' as i32 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        (*fs).w = (*(*fs).wp).window;
        return cmd_find_best_winlink_with_window(fs);
    }
    (*fs).wl = (*(*fs).s).curw;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    return cmd_find_get_pane_with_window(fs, pane);
}
unsafe extern "C" fn cmd_find_get_pane_with_window(
    mut fs: *mut cmd_find_state,
    mut pane: *const libc::c_char,
) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 30],
            &[libc::c_char; 30],
        >(b"cmd_find_get_pane_with_window\0"))
            .as_ptr(),
        pane,
    );
    if *pane as libc::c_int == '%' as i32 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        if (*(*fs).wp).window != (*fs).w {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if strcmp(pane, b"!\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*fs).wp = (*(*fs).w).last;
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else if strcmp(pane, b"{up-of}\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*fs).wp = window_pane_find_up((*(*fs).w).active);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else if strcmp(pane, b"{down-of}\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*fs).wp = window_pane_find_down((*(*fs).w).active);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else if strcmp(pane, b"{left-of}\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*fs).wp = window_pane_find_left((*(*fs).w).active);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else if strcmp(pane, b"{right-of}\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*fs).wp = window_pane_find_right((*(*fs).w).active);
        if ((*fs).wp).is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if *pane.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *pane.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        if *pane.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            n = strtonum(
                pane.offset(1 as libc::c_int as isize),
                1 as libc::c_int as libc::c_longlong,
                2147483647 as libc::c_int as libc::c_longlong,
                0 as *mut *const libc::c_char,
            ) as u_int;
        } else {
            n = 1 as libc::c_int as u_int;
        }
        wp = (*(*fs).w).active;
        if *pane.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
            (*fs).wp = window_pane_next_by_number((*fs).w, wp, n);
        } else {
            (*fs).wp = window_pane_previous_by_number((*fs).w, wp, n);
        }
        if !((*fs).wp).is_null() {
            return 0 as libc::c_int;
        }
    }
    idx = strtonum(
        pane,
        0 as libc::c_int as libc::c_longlong,
        2147483647 as libc::c_int as libc::c_longlong,
        &mut errstr,
    ) as libc::c_int;
    if errstr.is_null() {
        (*fs).wp = window_pane_at_index((*fs).w, idx as u_int);
        if !((*fs).wp).is_null() {
            return 0 as libc::c_int;
        }
    }
    (*fs).wp = window_find_string((*fs).w, pane);
    if !((*fs).wp).is_null() {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_clear_state(
    mut fs: *mut cmd_find_state,
    mut flags: libc::c_int,
) {
    memset(
        fs as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cmd_find_state>() as libc::c_ulong,
    );
    (*fs).flags = flags;
    (*fs).idx = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_empty_state(
    mut fs: *mut cmd_find_state,
) -> libc::c_int {
    if ((*fs).s).is_null() && ((*fs).wl).is_null() && ((*fs).w).is_null()
        && ((*fs).wp).is_null()
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_valid_state(
    mut fs: *mut cmd_find_state,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if ((*fs).s).is_null() || ((*fs).wl).is_null() || ((*fs).w).is_null()
        || ((*fs).wp).is_null()
    {
        return 0 as libc::c_int;
    }
    if session_alive((*fs).s) == 0 {
        return 0 as libc::c_int;
    }
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if (*wl).window == (*fs).w && wl == (*fs).wl {
            break;
        }
        wl = winlinks_RB_NEXT(wl);
    }
    if wl.is_null() {
        return 0 as libc::c_int;
    }
    if (*fs).w != (*(*fs).wl).window {
        return 0 as libc::c_int;
    }
    return window_has_pane((*fs).w, (*fs).wp);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_copy_state(
    mut dst: *mut cmd_find_state,
    mut src: *mut cmd_find_state,
) {
    (*dst).s = (*src).s;
    (*dst).wl = (*src).wl;
    (*dst).idx = (*src).idx;
    (*dst).w = (*src).w;
    (*dst).wp = (*src).wp;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_log_state(
    mut prefix: *const libc::c_char,
    mut fs: *mut cmd_find_state,
) {
    if !((*fs).s).is_null() {
        log_debug(
            b"%s: s=$%u\0" as *const u8 as *const libc::c_char,
            prefix,
            (*(*fs).s).id,
        );
    } else {
        log_debug(b"%s: s=none\0" as *const u8 as *const libc::c_char, prefix);
    }
    if !((*fs).wl).is_null() {
        log_debug(
            b"%s: wl=%u %d w=@%u %s\0" as *const u8 as *const libc::c_char,
            prefix,
            (*(*fs).wl).idx,
            ((*(*fs).wl).window == (*fs).w) as libc::c_int,
            (*(*fs).w).id,
            (*(*fs).w).name,
        );
    } else {
        log_debug(b"%s: wl=none\0" as *const u8 as *const libc::c_char, prefix);
    }
    if !((*fs).wp).is_null() {
        log_debug(
            b"%s: wp=%%%u\0" as *const u8 as *const libc::c_char,
            prefix,
            (*(*fs).wp).id,
        );
    } else {
        log_debug(b"%s: wp=none\0" as *const u8 as *const libc::c_char, prefix);
    }
    if (*fs).idx != -(1 as libc::c_int) {
        log_debug(
            b"%s: idx=%d\0" as *const u8 as *const libc::c_char,
            prefix,
            (*fs).idx,
        );
    } else {
        log_debug(b"%s: idx=none\0" as *const u8 as *const libc::c_char, prefix);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_session(
    mut fs: *mut cmd_find_state,
    mut s: *mut session,
    mut flags: libc::c_int,
) {
    cmd_find_clear_state(fs, flags);
    (*fs).s = s;
    (*fs).wl = (*(*fs).s).curw;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"cmd_find_from_session\0"))
            .as_ptr(),
        fs,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_winlink(
    mut fs: *mut cmd_find_state,
    mut wl: *mut winlink,
    mut flags: libc::c_int,
) {
    cmd_find_clear_state(fs, flags);
    (*fs).s = (*wl).session;
    (*fs).wl = wl;
    (*fs).w = (*wl).window;
    (*fs).wp = (*(*wl).window).active;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"cmd_find_from_winlink\0"))
            .as_ptr(),
        fs,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_session_window(
    mut fs: *mut cmd_find_state,
    mut s: *mut session,
    mut w: *mut window,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).s = s;
    (*fs).w = w;
    if cmd_find_best_winlink_with_window(fs) != 0 as libc::c_int {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 29],
            &[libc::c_char; 29],
        >(b"cmd_find_from_session_window\0"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_window(
    mut fs: *mut cmd_find_state,
    mut w: *mut window,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).w = w;
    if cmd_find_best_session_with_window(fs) != 0 as libc::c_int {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    if cmd_find_best_winlink_with_window(fs) != 0 as libc::c_int {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"cmd_find_from_window\0"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_winlink_pane(
    mut fs: *mut cmd_find_state,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
    mut flags: libc::c_int,
) {
    cmd_find_clear_state(fs, flags);
    (*fs).s = (*wl).session;
    (*fs).wl = wl;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = wp;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"cmd_find_from_winlink_pane\0"))
            .as_ptr(),
        fs,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_pane(
    mut fs: *mut cmd_find_state,
    mut wp: *mut window_pane,
    mut flags: libc::c_int,
) -> libc::c_int {
    if cmd_find_from_window(fs, (*wp).window, flags) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*fs).wp = wp;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"cmd_find_from_pane\0"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_nothing(
    mut fs: *mut cmd_find_state,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs)
        .s = cmd_find_best_session(
        0 as *mut *mut session,
        0 as libc::c_int as u_int,
        flags,
    );
    if ((*fs).s).is_null() {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).wl = (*(*fs).s).curw;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"cmd_find_from_nothing\0"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_mouse(
    mut fs: *mut cmd_find_state,
    mut m: *mut mouse_event,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    if (*m).valid == 0 {
        return -(1 as libc::c_int);
    }
    (*fs).wp = cmd_mouse_pane(m, &mut (*fs).s, &mut (*fs).wl);
    if ((*fs).wp).is_null() {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).w = (*(*fs).wl).window;
    cmd_find_log_state(
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"cmd_find_from_mouse\0"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_client(
    mut fs: *mut cmd_find_state,
    mut c: *mut client,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if c.is_null() {
        return cmd_find_from_nothing(fs, flags);
    }
    if !((*c).session).is_null() {
        cmd_find_from_session(fs, (*c).session, flags);
        return 0 as libc::c_int;
    }
    cmd_find_clear_state(fs, flags);
    wp = cmd_find_inside_pane(c);
    if !wp.is_null() {
        s = cmd_find_try_TMUX(c);
        if !s.is_null() {
            wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
            while !wl.is_null() {
                if window_has_pane((*wl).window, wp) != 0 {
                    break;
                }
                wl = winlinks_RB_NEXT(wl);
            }
            if !wl.is_null() {
                (*fs).s = s;
                (*fs).wl = (*s).curw;
                (*fs).w = (*(*fs).wl).window;
                (*fs).wp = (*(*fs).w).active;
                cmd_find_log_state(
                    (*::core::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"cmd_find_from_client\0"))
                        .as_ptr(),
                    fs,
                );
                return 0 as libc::c_int;
            }
        }
        (*fs).w = (*wp).window;
        if !(cmd_find_best_session_with_window(fs) != 0 as libc::c_int) {
            (*fs).wl = (*(*fs).s).curw;
            (*fs).w = (*(*fs).wl).window;
            (*fs).wp = (*(*fs).w).active;
            cmd_find_log_state(
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"cmd_find_from_client\0"))
                    .as_ptr(),
                fs,
            );
            return 0 as libc::c_int;
        }
    }
    s = cmd_find_try_TMUX(c);
    if !s.is_null() {
        cmd_find_from_session(fs, s, flags);
        return 0 as libc::c_int;
    }
    return cmd_find_from_nothing(fs, flags);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_target(
    mut fs: *mut cmd_find_state,
    mut item: *mut cmdq_item,
    mut target: *const libc::c_char,
    mut type_0: cmd_find_type,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut m: *mut mouse_event = 0 as *mut mouse_event;
    let mut current: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *const cmd_find_state as *mut cmd_find_state,
        s: 0 as *const session as *mut session,
        wl: 0 as *const winlink as *mut winlink,
        w: 0 as *const window as *mut window,
        wp: 0 as *const window_pane as *mut window_pane,
        idx: 0,
    };
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut period: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut session: *const libc::c_char = 0 as *const libc::c_char;
    let mut window: *const libc::c_char = 0 as *const libc::c_char;
    let mut pane: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut window_only: libc::c_int = 0 as libc::c_int;
    let mut pane_only: libc::c_int = 0 as libc::c_int;
    if flags & 0x40 as libc::c_int != 0 {
        flags |= 0x2 as libc::c_int;
    }
    if type_0 as libc::c_uint == CMD_FIND_PANE as libc::c_int as libc::c_uint {
        s = b"pane\0" as *const u8 as *const libc::c_char;
    } else if type_0 as libc::c_uint == CMD_FIND_WINDOW as libc::c_int as libc::c_uint {
        s = b"window\0" as *const u8 as *const libc::c_char;
    } else if type_0 as libc::c_uint == CMD_FIND_SESSION as libc::c_int as libc::c_uint {
        s = b"session\0" as *const u8 as *const libc::c_char;
    } else {
        s = b"unknown\0" as *const u8 as *const libc::c_char;
    }
    if target.is_null() {
        log_debug(
            b"%s: target none, type %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"cmd_find_target\0"))
                .as_ptr(),
            s,
        );
    } else {
        log_debug(
            b"%s: target %s, type %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"cmd_find_target\0"))
                .as_ptr(),
            target,
            s,
        );
    }
    log_debug(
        b"%s: item %p, flags %#x\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\0"))
            .as_ptr(),
        item,
        flags,
    );
    cmd_find_clear_state(fs, flags);
    if server_check_marked() != 0 && flags & 0x8 as libc::c_int != 0 {
        (*fs).current = &mut marked_pane;
        log_debug(
            b"%s: current is marked pane\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"cmd_find_target\0"))
                .as_ptr(),
        );
        current_block = 10043043949733653460;
    } else if cmd_find_valid_state(&mut (*(*item).shared).current) != 0 {
        (*fs).current = &mut (*(*item).shared).current;
        log_debug(
            b"%s: current is from queue\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"cmd_find_target\0"))
                .as_ptr(),
        );
        current_block = 10043043949733653460;
    } else if cmd_find_from_client(&mut current, (*item).client, flags)
        == 0 as libc::c_int
    {
        (*fs).current = &mut current;
        log_debug(
            b"%s: current is from client\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"cmd_find_target\0"))
                .as_ptr(),
        );
        current_block = 10043043949733653460;
    } else {
        if !flags & 0x2 as libc::c_int != 0 {
            cmdq_error(item, b"no current target\0" as *const u8 as *const libc::c_char);
        }
        current_block = 5541695263705503054;
    }
    match current_block {
        10043043949733653460 => {
            if cmd_find_valid_state((*fs).current) == 0 {
                fatalx(
                    b"invalid current find state\0" as *const u8 as *const libc::c_char,
                );
            }
            if target.is_null() || *target as libc::c_int == '\0' as i32 {
                current_block = 6413872448950241433;
            } else if strcmp(target, b"=\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(target, b"{mouse}\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                m = &mut (*(*item).shared).mouse;
                match type_0 as libc::c_uint {
                    0 => {
                        (*fs).wp = cmd_mouse_pane(m, &mut (*fs).s, &mut (*fs).wl);
                        if !((*fs).wp).is_null() {
                            (*fs).w = (*(*fs).wl).window;
                        }
                    }
                    1 | 2 => {
                        (*fs).wl = cmd_mouse_window(m, &mut (*fs).s);
                        if !((*fs).wl).is_null() {
                            (*fs).w = (*(*fs).wl).window;
                            (*fs).wp = (*(*fs).w).active;
                        }
                    }
                    _ => {}
                }
                if ((*fs).wp).is_null() {
                    if !flags & 0x2 as libc::c_int != 0 {
                        cmdq_error(
                            item,
                            b"no mouse target\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 5541695263705503054;
                } else {
                    current_block = 2628197061846535215;
                }
            } else if strcmp(target, b"~\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(target, b"{marked}\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                if server_check_marked() == 0 {
                    if !flags & 0x2 as libc::c_int != 0 {
                        cmdq_error(
                            item,
                            b"no marked target\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 5541695263705503054;
                } else {
                    cmd_find_copy_state(fs, &mut marked_pane);
                    current_block = 2628197061846535215;
                }
            } else {
                copy = xstrdup(target);
                colon = strchr(copy, ':' as i32);
                if !colon.is_null() {
                    let fresh2 = colon;
                    colon = colon.offset(1);
                    *fresh2 = '\0' as i32 as libc::c_char;
                }
                if colon.is_null() {
                    period = strchr(copy, '.' as i32);
                } else {
                    period = strchr(colon, '.' as i32);
                }
                if !period.is_null() {
                    let fresh3 = period;
                    period = period.offset(1);
                    *fresh3 = '\0' as i32 as libc::c_char;
                }
                pane = 0 as *const libc::c_char;
                window = pane;
                session = window;
                if !colon.is_null() && !period.is_null() {
                    session = copy;
                    window = colon;
                    window_only = 1 as libc::c_int;
                    pane = period;
                    pane_only = 1 as libc::c_int;
                } else if !colon.is_null() && period.is_null() {
                    session = copy;
                    window = colon;
                    window_only = 1 as libc::c_int;
                } else if colon.is_null() && !period.is_null() {
                    window = copy;
                    pane = period;
                    pane_only = 1 as libc::c_int;
                } else if *copy as libc::c_int == '$' as i32 {
                    session = copy;
                } else if *copy as libc::c_int == '@' as i32 {
                    window = copy;
                } else if *copy as libc::c_int == '%' as i32 {
                    pane = copy;
                } else {
                    match type_0 as libc::c_uint {
                        2 => {
                            session = copy;
                        }
                        1 => {
                            window = copy;
                        }
                        0 => {
                            pane = copy;
                        }
                        _ => {}
                    }
                }
                if !session.is_null() && *session as libc::c_int == '=' as i32 {
                    session = session.offset(1);
                    session;
                    (*fs).flags |= 0x10 as libc::c_int;
                }
                if !window.is_null() && *window as libc::c_int == '=' as i32 {
                    window = window.offset(1);
                    window;
                    (*fs).flags |= 0x20 as libc::c_int;
                }
                if !session.is_null() && *session as libc::c_int == '\0' as i32 {
                    session = 0 as *const libc::c_char;
                }
                if !window.is_null() && *window as libc::c_int == '\0' as i32 {
                    window = 0 as *const libc::c_char;
                }
                if !pane.is_null() && *pane as libc::c_int == '\0' as i32 {
                    pane = 0 as *const libc::c_char;
                }
                if !session.is_null() {
                    session = cmd_find_map_table(
                        cmd_find_session_table.as_mut_ptr(),
                        session,
                    );
                }
                if !window.is_null() {
                    window = cmd_find_map_table(
                        cmd_find_window_table.as_mut_ptr(),
                        window,
                    );
                }
                if !pane.is_null() {
                    pane = cmd_find_map_table(cmd_find_pane_table.as_mut_ptr(), pane);
                }
                log_debug(
                    b"%s: target %s (flags %#x): session=%s, window=%s, pane=%s\0"
                        as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"cmd_find_target\0"))
                        .as_ptr(),
                    target,
                    flags,
                    if session.is_null() {
                        b"none\0" as *const u8 as *const libc::c_char
                    } else {
                        session
                    },
                    if window.is_null() {
                        b"none\0" as *const u8 as *const libc::c_char
                    } else {
                        window
                    },
                    if pane.is_null() {
                        b"none\0" as *const u8 as *const libc::c_char
                    } else {
                        pane
                    },
                );
                if !pane.is_null() && flags & 0x4 as libc::c_int != 0 {
                    if !flags & 0x2 as libc::c_int != 0 {
                        cmdq_error(
                            item,
                            b"can't specify pane here\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    current_block = 5541695263705503054;
                } else {
                    if !session.is_null() {
                        if cmd_find_get_session(fs, session) != 0 as libc::c_int {
                            if !flags & 0x2 as libc::c_int != 0 {
                                cmdq_error(
                                    item,
                                    b"can't find session %s\0" as *const u8
                                        as *const libc::c_char,
                                    session,
                                );
                            }
                            current_block = 5541695263705503054;
                        } else if window.is_null() && pane.is_null() {
                            (*fs).wl = (*(*fs).s).curw;
                            (*fs).idx = -(1 as libc::c_int);
                            (*fs).w = (*(*fs).wl).window;
                            (*fs).wp = (*(*fs).w).active;
                            current_block = 2628197061846535215;
                        } else if !window.is_null() && pane.is_null() {
                            if cmd_find_get_window_with_session(fs, window)
                                != 0 as libc::c_int
                            {
                                current_block = 10126172881903668360;
                            } else {
                                (*fs).wp = (*(*(*fs).wl).window).active;
                                current_block = 2628197061846535215;
                            }
                        } else if window.is_null() && !pane.is_null() {
                            if cmd_find_get_pane_with_session(fs, pane)
                                != 0 as libc::c_int
                            {
                                current_block = 17861424788070315216;
                            } else {
                                current_block = 2628197061846535215;
                            }
                        } else if cmd_find_get_window_with_session(fs, window)
                            != 0 as libc::c_int
                        {
                            current_block = 10126172881903668360;
                        } else if cmd_find_get_pane_with_window(fs, pane)
                            != 0 as libc::c_int
                        {
                            current_block = 17861424788070315216;
                        } else {
                            current_block = 2628197061846535215;
                        }
                    } else if !window.is_null() && !pane.is_null() {
                        if cmd_find_get_window(fs, window, window_only)
                            != 0 as libc::c_int
                        {
                            current_block = 10126172881903668360;
                        } else if cmd_find_get_pane_with_window(fs, pane)
                            != 0 as libc::c_int
                        {
                            current_block = 17861424788070315216;
                        } else {
                            current_block = 2628197061846535215;
                        }
                    } else if !window.is_null() && pane.is_null() {
                        if cmd_find_get_window(fs, window, window_only)
                            != 0 as libc::c_int
                        {
                            current_block = 10126172881903668360;
                        } else {
                            (*fs).wp = (*(*(*fs).wl).window).active;
                            current_block = 2628197061846535215;
                        }
                    } else if window.is_null() && !pane.is_null() {
                        if cmd_find_get_pane(fs, pane, pane_only) != 0 as libc::c_int {
                            current_block = 17861424788070315216;
                        } else {
                            current_block = 2628197061846535215;
                        }
                    } else {
                        current_block = 6413872448950241433;
                    }
                    match current_block {
                        5541695263705503054 => {}
                        2628197061846535215 => {}
                        6413872448950241433 => {}
                        _ => {
                            match current_block {
                                10126172881903668360 => {
                                    if !flags & 0x2 as libc::c_int != 0 {
                                        cmdq_error(
                                            item,
                                            b"can't find window %s\0" as *const u8
                                                as *const libc::c_char,
                                            window,
                                        );
                                    }
                                }
                                _ => {
                                    if !flags & 0x2 as libc::c_int != 0 {
                                        cmdq_error(
                                            item,
                                            b"can't find pane %s\0" as *const u8 as *const libc::c_char,
                                            pane,
                                        );
                                    }
                                }
                            }
                            current_block = 5541695263705503054;
                        }
                    }
                }
            }
            match current_block {
                5541695263705503054 => {}
                _ => {
                    match current_block {
                        6413872448950241433 => {
                            cmd_find_copy_state(fs, (*fs).current);
                            if flags & 0x4 as libc::c_int != 0 {
                                (*fs).idx = -(1 as libc::c_int);
                            }
                        }
                        _ => {}
                    }
                    (*fs).current = 0 as *mut cmd_find_state;
                    cmd_find_log_state(
                        (*::core::mem::transmute::<
                            &[u8; 16],
                            &[libc::c_char; 16],
                        >(b"cmd_find_target\0"))
                            .as_ptr(),
                        fs,
                    );
                    free(copy as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    (*fs).current = 0 as *mut cmd_find_state;
    log_debug(
        b"%s: error\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\0"))
            .as_ptr(),
    );
    free(copy as *mut libc::c_void);
    if flags & 0x40 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_find_current_client(
    mut item: *mut cmdq_item,
    mut quiet: libc::c_int,
) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *const cmd_find_state as *mut cmd_find_state,
        s: 0 as *const session as *mut session,
        wl: 0 as *const winlink as *mut winlink,
        w: 0 as *const window as *mut window,
        wp: 0 as *const window_pane as *mut window_pane,
        idx: 0,
    };
    if !((*item).client).is_null() && !((*(*item).client).session).is_null() {
        return (*item).client;
    }
    c = 0 as *mut client;
    wp = cmd_find_inside_pane((*item).client);
    if !wp.is_null() {
        cmd_find_clear_state(&mut fs, 0x2 as libc::c_int);
        fs.w = (*wp).window;
        if cmd_find_best_session_with_window(&mut fs) == 0 as libc::c_int {
            c = cmd_find_best_client(fs.s);
        }
    } else {
        s = cmd_find_best_session(
            0 as *mut *mut session,
            0 as libc::c_int as u_int,
            0x2 as libc::c_int,
        );
        if !s.is_null() {
            c = cmd_find_best_client(s);
        }
    }
    if c.is_null() && quiet == 0 {
        cmdq_error(item, b"no current client\0" as *const u8 as *const libc::c_char);
    }
    log_debug(
        b"%s: no target, return %p\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 24],
            &[libc::c_char; 24],
        >(b"cmd_find_current_client\0"))
            .as_ptr(),
        c,
    );
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_find_client(
    mut item: *mut cmdq_item,
    mut target: *const libc::c_char,
    mut quiet: libc::c_int,
) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    if target.is_null() {
        return cmd_find_current_client(item, quiet);
    }
    copy = xstrdup(target);
    size = strlen(copy);
    if size != 0 as libc::c_int as size_t
        && *copy.offset(size.wrapping_sub(1 as libc::c_int as size_t) as isize)
            as libc::c_int == ':' as i32
    {
        *copy
            .offset(
                size.wrapping_sub(1 as libc::c_int as size_t) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session).is_null() {
            if strcmp(copy, (*c).name) == 0 as libc::c_int {
                break;
            }
            if !(*(*c).ttyname as libc::c_int == '\0' as i32) {
                if strcmp(copy, (*c).ttyname) == 0 as libc::c_int {
                    break;
                }
                if !(strncmp(
                    (*c).ttyname,
                    b"/dev/\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int)
                {
                    if strcmp(
                        copy,
                        ((*c).ttyname)
                            .offset(
                                ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                    as isize,
                            )
                            .offset(-(1 as libc::c_int as isize)),
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
            }
        }
        c = (*c).entry.tqe_next;
    }
    if c.is_null() && quiet == 0 {
        cmdq_error(
            item,
            b"can't find client %s\0" as *const u8 as *const libc::c_char,
            copy,
        );
    }
    free(copy as *mut libc::c_void);
    log_debug(
        b"%s: target %s, return %p\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_client\0"))
            .as_ptr(),
        target,
        c,
    );
    return c;
}
