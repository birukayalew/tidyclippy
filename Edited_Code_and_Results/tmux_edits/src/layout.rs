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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn notify_window(_: *const libc::c_char, _: *mut window);
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn window_pane_resize(_: *mut window_pane, _: u_int, _: u_int);
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
#[no_mangle]
pub unsafe extern "C" fn layout_create_cell(
    mut lcparent: *mut layout_cell,
) -> *mut layout_cell {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    lc = xmalloc(::core::mem::size_of::<layout_cell>() as libc::c_ulong)
        as *mut layout_cell;
    (*lc).type_0 = LAYOUT_WINDOWPANE;
    (*lc).parent = lcparent;
    (*lc).cells.tqh_first = 0 as *mut layout_cell;
    (*lc).cells.tqh_last = &mut (*lc).cells.tqh_first;
    (*lc)
        .sx = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    (*lc)
        .sy = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    (*lc)
        .xoff = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    (*lc)
        .yoff = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    (*lc).wp = 0 as *mut window_pane;
    return lc;
}
#[no_mangle]
pub unsafe extern "C" fn layout_free_cell(mut lc: *mut layout_cell) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    match (*lc).type_0 as libc::c_uint {
        0 | 1 => {
            while !((*lc).cells.tqh_first).is_null() {
                lcchild = (*lc).cells.tqh_first;
                if !((*lcchild).entry.tqe_next).is_null() {
                    (*(*lcchild).entry.tqe_next)
                        .entry
                        .tqe_prev = (*lcchild).entry.tqe_prev;
                } else {
                    (*lc).cells.tqh_last = (*lcchild).entry.tqe_prev;
                }
                *(*lcchild).entry.tqe_prev = (*lcchild).entry.tqe_next;
                layout_free_cell(lcchild);
            }
        }
        2 => {
            if !((*lc).wp).is_null() {
                (*(*lc).wp).layout_cell = 0 as *mut layout_cell;
            }
        }
        _ => {}
    }
    free(lc as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn layout_print_cell(
    mut lc: *mut layout_cell,
    mut hdr: *const libc::c_char,
    mut n: u_int,
) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    match (*lc).type_0 as libc::c_uint {
        0 => {
            type_0 = b"LEFTRIGHT\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            type_0 = b"TOPBOTTOM\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            type_0 = b"WINDOWPANE\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            type_0 = b"UNKNOWN\0" as *const u8 as *const libc::c_char;
        }
    }
    log_debug(
        b"%s:%*s%p type %s [parent %p] wp=%p [%u,%u %ux%u]\0" as *const u8
            as *const libc::c_char,
        hdr,
        n,
        b" \0" as *const u8 as *const libc::c_char,
        lc,
        type_0,
        (*lc).parent,
        (*lc).wp,
        (*lc).xoff,
        (*lc).yoff,
        (*lc).sx,
        (*lc).sy,
    );
    match (*lc).type_0 as libc::c_uint {
        0 | 1 => {
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                layout_print_cell(
                    lcchild,
                    hdr,
                    n.wrapping_add(1 as libc::c_int as u_int),
                );
                lcchild = (*lcchild).entry.tqe_next;
            }
        }
        2 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_size(
    mut lc: *mut layout_cell,
    mut sx: u_int,
    mut sy: u_int,
    mut xoff: u_int,
    mut yoff: u_int,
) {
    (*lc).sx = sx;
    (*lc).sy = sy;
    (*lc).xoff = xoff;
    (*lc).yoff = yoff;
}
#[no_mangle]
pub unsafe extern "C" fn layout_make_leaf(
    mut lc: *mut layout_cell,
    mut wp: *mut window_pane,
) {
    (*lc).type_0 = LAYOUT_WINDOWPANE;
    (*lc).cells.tqh_first = 0 as *mut layout_cell;
    (*lc).cells.tqh_last = &mut (*lc).cells.tqh_first;
    (*wp).layout_cell = lc;
    (*lc).wp = wp;
}
#[no_mangle]
pub unsafe extern "C" fn layout_make_node(
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
) {
    if type_0 as libc::c_uint == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        fatalx(b"bad layout type\0" as *const u8 as *const libc::c_char);
    }
    (*lc).type_0 = type_0;
    (*lc).cells.tqh_first = 0 as *mut layout_cell;
    (*lc).cells.tqh_last = &mut (*lc).cells.tqh_first;
    if !((*lc).wp).is_null() {
        (*(*lc).wp).layout_cell = 0 as *mut layout_cell;
    }
    (*lc).wp = 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn layout_fix_offsets(mut lc: *mut layout_cell) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    if (*lc).type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        xoff = (*lc).xoff;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            (*lcchild).xoff = xoff;
            (*lcchild).yoff = (*lc).yoff;
            if (*lcchild).type_0 as libc::c_uint
                != LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint
            {
                layout_fix_offsets(lcchild);
            }
            xoff = xoff
                .wrapping_add(((*lcchild).sx).wrapping_add(1 as libc::c_int as u_int));
            lcchild = (*lcchild).entry.tqe_next;
        }
    } else {
        yoff = (*lc).yoff;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            (*lcchild).xoff = (*lc).xoff;
            (*lcchild).yoff = yoff;
            if (*lcchild).type_0 as libc::c_uint
                != LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint
            {
                layout_fix_offsets(lcchild);
            }
            yoff = yoff
                .wrapping_add(((*lcchild).sy).wrapping_add(1 as libc::c_int as u_int));
            lcchild = (*lcchild).entry.tqe_next;
        }
    };
}
unsafe extern "C" fn layout_need_status(
    mut lc: *mut layout_cell,
    mut at_top: libc::c_int,
) -> libc::c_int {
    let mut first_lc: *mut layout_cell = 0 as *mut layout_cell;
    if !((*lc).parent).is_null() {
        if (*(*lc).parent).type_0 as libc::c_uint
            == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
        {
            return layout_need_status((*lc).parent, at_top);
        }
        if at_top != 0 {
            first_lc = (*(*lc).parent).cells.tqh_first;
        } else {
            first_lc = *(*((*(*lc).parent).cells.tqh_last as *mut layout_cells))
                .tqh_last;
        }
        if lc == first_lc {
            return layout_need_status((*lc).parent, at_top);
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn layout_fix_panes(
    mut w: *mut window,
    mut wsx: u_int,
    mut wsy: u_int,
) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut shift: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut at_top: libc::c_int = 0;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    at_top = (status == 1 as libc::c_int) as libc::c_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        lc = (*wp).layout_cell;
        if !lc.is_null() {
            if status != 0 as libc::c_int {
                shift = layout_need_status(lc, at_top);
            } else {
                shift = 0 as libc::c_int;
            }
            (*wp).xoff = (*lc).xoff;
            (*wp).yoff = (*lc).yoff;
            if shift != 0 && at_top != 0 {
                (*wp).yoff = ((*wp).yoff).wrapping_add(1 as libc::c_int as u_int);
            }
            if (*lc).xoff >= wsx || ((*lc).xoff).wrapping_add((*lc).sx) < wsx {
                sx = (*lc).sx;
            } else {
                sx = wsx.wrapping_sub((*lc).xoff);
                if sx < 1 as libc::c_int as u_int {
                    sx = (*lc).sx;
                }
            }
            if (*lc).yoff >= wsy || ((*lc).yoff).wrapping_add((*lc).sy) < wsy {
                sy = (*lc).sy;
            } else {
                sy = wsy.wrapping_sub((*lc).yoff);
                if sy < 2 as libc::c_int as u_int {
                    sy = (*lc).sy;
                }
            }
            if shift != 0 {
                sy = sy.wrapping_sub(1 as libc::c_int as u_int);
            }
            window_pane_resize(wp, sx, sy);
        }
        wp = (*wp).entry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn layout_count_cells(mut lc: *mut layout_cell) -> u_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut count: u_int = 0;
    match (*lc).type_0 as libc::c_uint {
        2 => return 1 as libc::c_int as u_int,
        0 | 1 => {
            count = 0 as libc::c_int as u_int;
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                count = count.wrapping_add(layout_count_cells(lcchild));
                lcchild = (*lcchild).entry.tqe_next;
            }
            return count;
        }
        _ => {
            fatalx(b"bad layout type\0" as *const u8 as *const libc::c_char);
        }
    };
}
unsafe extern "C" fn layout_resize_check(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
) -> u_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut available: u_int = 0;
    let mut minimum: u_int = 0;
    if (*lc).type_0 as libc::c_uint == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        minimum = 2 as libc::c_int as u_int;
        if type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            available = (*lc).sx;
        } else {
            available = (*lc).sy;
            minimum = minimum
                .wrapping_add(
                    layout_need_status(
                        lc,
                        (options_get_number(
                            (*w).options,
                            b"pane-border-status\0" as *const u8 as *const libc::c_char,
                        ) == 1 as libc::c_int as libc::c_longlong) as libc::c_int,
                    ) as u_int,
                );
        }
        if available > minimum {
            available = available.wrapping_sub(minimum);
        } else {
            available = 0 as libc::c_int as u_int;
        }
    } else if (*lc).type_0 as libc::c_uint == type_0 as libc::c_uint {
        available = 0 as libc::c_int as u_int;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            available = available.wrapping_add(layout_resize_check(w, lcchild, type_0));
            lcchild = (*lcchild).entry.tqe_next;
        }
    } else {
        minimum = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            available = layout_resize_check(w, lcchild, type_0);
            if available < minimum {
                minimum = available;
            }
            lcchild = (*lcchild).entry.tqe_next;
        }
        available = minimum;
    }
    return available;
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_adjust(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut change: libc::c_int,
) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    if type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        (*lc).sx = ((*lc).sx).wrapping_add(change as u_int);
    } else {
        (*lc).sy = ((*lc).sy).wrapping_add(change as u_int);
    }
    if type_0 as libc::c_uint == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return;
    }
    if (*lc).type_0 as libc::c_uint != type_0 as libc::c_uint {
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            layout_resize_adjust(w, lcchild, type_0, change);
            lcchild = (*lcchild).entry.tqe_next;
        }
        return;
    }
    while change != 0 as libc::c_int {
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            if change == 0 as libc::c_int {
                break;
            }
            if change > 0 as libc::c_int {
                layout_resize_adjust(w, lcchild, type_0, 1 as libc::c_int);
                change -= 1;
                change;
            } else if layout_resize_check(w, lcchild, type_0) > 0 as libc::c_int as u_int
            {
                layout_resize_adjust(w, lcchild, type_0, -(1 as libc::c_int));
                change += 1;
                change;
            }
            lcchild = (*lcchild).entry.tqe_next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn layout_destroy_cell(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut lcroot: *mut *mut layout_cell,
) {
    let mut lcother: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    lcparent = (*lc).parent;
    if lcparent.is_null() {
        layout_free_cell(lc);
        *lcroot = 0 as *mut layout_cell;
        return;
    }
    if lc == (*lcparent).cells.tqh_first {
        lcother = (*lc).entry.tqe_next;
    } else {
        lcother = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last;
    }
    if (*lcparent).type_0 as libc::c_uint
        == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
    {
        layout_resize_adjust(
            w,
            lcother,
            (*lcparent).type_0,
            ((*lc).sx).wrapping_add(1 as libc::c_int as u_int) as libc::c_int,
        );
    } else {
        layout_resize_adjust(
            w,
            lcother,
            (*lcparent).type_0,
            ((*lc).sy).wrapping_add(1 as libc::c_int as u_int) as libc::c_int,
        );
    }
    if !((*lc).entry.tqe_next).is_null() {
        (*(*lc).entry.tqe_next).entry.tqe_prev = (*lc).entry.tqe_prev;
    } else {
        (*lcparent).cells.tqh_last = (*lc).entry.tqe_prev;
    }
    *(*lc).entry.tqe_prev = (*lc).entry.tqe_next;
    layout_free_cell(lc);
    lc = (*lcparent).cells.tqh_first;
    if ((*lc).entry.tqe_next).is_null() {
        if !((*lc).entry.tqe_next).is_null() {
            (*(*lc).entry.tqe_next).entry.tqe_prev = (*lc).entry.tqe_prev;
        } else {
            (*lcparent).cells.tqh_last = (*lc).entry.tqe_prev;
        }
        *(*lc).entry.tqe_prev = (*lc).entry.tqe_next;
        (*lc).parent = (*lcparent).parent;
        if ((*lc).parent).is_null() {
            (*lc).xoff = 0 as libc::c_int as u_int;
            (*lc).yoff = 0 as libc::c_int as u_int;
            *lcroot = lc;
        } else {
            (*lc).entry.tqe_next = (*lcparent).entry.tqe_next;
            if !((*lc).entry.tqe_next).is_null() {
                (*(*lc).entry.tqe_next).entry.tqe_prev = &mut (*lc).entry.tqe_next;
            } else {
                (*(*lc).parent).cells.tqh_last = &mut (*lc).entry.tqe_next;
            }
            (*lc).entry.tqe_prev = (*lcparent).entry.tqe_prev;
            *(*lc).entry.tqe_prev = lc;
        }
        layout_free_cell(lcparent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn layout_init(mut w: *mut window, mut wp: *mut window_pane) {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        (*w).sx,
        (*w).sy,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_leaf(lc, wp);
    layout_fix_panes(w, (*w).sx, (*w).sy);
}
#[no_mangle]
pub unsafe extern "C" fn layout_free(mut w: *mut window) {
    layout_free_cell((*w).layout_root);
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize(
    mut w: *mut window,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut lc: *mut layout_cell = (*w).layout_root;
    let mut xlimit: libc::c_int = 0;
    let mut ylimit: libc::c_int = 0;
    let mut xchange: libc::c_int = 0;
    let mut ychange: libc::c_int = 0;
    xchange = sx.wrapping_sub((*w).sx) as libc::c_int;
    xlimit = layout_resize_check(w, lc, LAYOUT_LEFTRIGHT) as libc::c_int;
    if xchange < 0 as libc::c_int && xchange < -xlimit {
        xchange = -xlimit;
    }
    if xlimit == 0 as libc::c_int {
        if sx <= (*lc).sx {
            xchange = 0 as libc::c_int;
        } else {
            xchange = sx.wrapping_sub((*lc).sx) as libc::c_int;
        }
    }
    if xchange != 0 as libc::c_int {
        layout_resize_adjust(w, lc, LAYOUT_LEFTRIGHT, xchange);
    }
    ychange = sy.wrapping_sub((*w).sy) as libc::c_int;
    ylimit = layout_resize_check(w, lc, LAYOUT_TOPBOTTOM) as libc::c_int;
    if ychange < 0 as libc::c_int && ychange < -ylimit {
        ychange = -ylimit;
    }
    if ylimit == 0 as libc::c_int {
        if sy <= (*lc).sy {
            ychange = 0 as libc::c_int;
        } else {
            ychange = sy.wrapping_sub((*lc).sy) as libc::c_int;
        }
    }
    if ychange != 0 as libc::c_int {
        layout_resize_adjust(w, lc, LAYOUT_TOPBOTTOM, ychange);
    }
    layout_fix_offsets(lc);
    layout_fix_panes(w, sx, sy);
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_pane_to(
    mut wp: *mut window_pane,
    mut type_0: layout_type,
    mut new_size: u_int,
) {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut change: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    lc = (*wp).layout_cell;
    lcparent = (*lc).parent;
    while !lcparent.is_null()
        && (*lcparent).type_0 as libc::c_uint != type_0 as libc::c_uint
    {
        lc = lcparent;
        lcparent = (*lc).parent;
    }
    if lcparent.is_null() {
        return;
    }
    if type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        size = (*lc).sx as libc::c_int;
    } else {
        size = (*lc).sy as libc::c_int;
    }
    if lc == *(*((*lcparent).cells.tqh_last as *mut layout_cells)).tqh_last {
        change = (size as u_int).wrapping_sub(new_size) as libc::c_int;
    } else {
        change = new_size.wrapping_sub(size as u_int) as libc::c_int;
    }
    layout_resize_pane(wp, type_0, change, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_pane(
    mut wp: *mut window_pane,
    mut type_0: layout_type,
    mut change: libc::c_int,
    mut opposite: libc::c_int,
) {
    let mut w: *mut window = (*wp).window;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut needed: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    lc = (*wp).layout_cell;
    lcparent = (*lc).parent;
    while !lcparent.is_null()
        && (*lcparent).type_0 as libc::c_uint != type_0 as libc::c_uint
    {
        lc = lcparent;
        lcparent = (*lc).parent;
    }
    if lcparent.is_null() {
        return;
    }
    if lc == *(*((*lcparent).cells.tqh_last as *mut layout_cells)).tqh_last {
        lc = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last;
    }
    needed = change;
    while needed != 0 as libc::c_int {
        if change > 0 as libc::c_int {
            size = layout_resize_pane_grow(w, lc, type_0, needed, opposite);
            needed -= size;
        } else {
            size = layout_resize_pane_shrink(w, lc, type_0, needed);
            needed += size;
        }
        if size == 0 as libc::c_int {
            break;
        }
    }
    layout_fix_offsets((*(*wp).window).layout_root);
    layout_fix_panes((*wp).window, (*(*wp).window).sx, (*(*wp).window).sy);
    notify_window(
        b"window-layout-changed\0" as *const u8 as *const libc::c_char,
        (*wp).window,
    );
}
unsafe extern "C" fn layout_resize_pane_grow(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut needed: libc::c_int,
    mut opposite: libc::c_int,
) -> libc::c_int {
    let mut lcadd: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcremove: *mut layout_cell = 0 as *mut layout_cell;
    let mut size: u_int = 0 as libc::c_int as u_int;
    lcadd = lc;
    lcremove = (*lc).entry.tqe_next;
    while !lcremove.is_null() {
        size = layout_resize_check(w, lcremove, type_0);
        if size > 0 as libc::c_int as u_int {
            break;
        }
        lcremove = (*lcremove).entry.tqe_next;
    }
    if opposite != 0 && lcremove.is_null() {
        lcremove = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last;
        while !lcremove.is_null() {
            size = layout_resize_check(w, lcremove, type_0);
            if size > 0 as libc::c_int as u_int {
                break;
            }
            lcremove = *(*((*lcremove).entry.tqe_prev as *mut layout_cells)).tqh_last;
        }
    }
    if lcremove.is_null() {
        return 0 as libc::c_int;
    }
    if size > needed as u_int {
        size = needed as u_int;
    }
    layout_resize_adjust(w, lcadd, type_0, size as libc::c_int);
    layout_resize_adjust(w, lcremove, type_0, size.wrapping_neg() as libc::c_int);
    return size as libc::c_int;
}
unsafe extern "C" fn layout_resize_pane_shrink(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut needed: libc::c_int,
) -> libc::c_int {
    let mut lcadd: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcremove: *mut layout_cell = 0 as *mut layout_cell;
    let mut size: u_int = 0;
    lcremove = lc;
    loop {
        size = layout_resize_check(w, lcremove, type_0);
        if size != 0 as libc::c_int as u_int {
            break;
        }
        lcremove = *(*((*lcremove).entry.tqe_prev as *mut layout_cells)).tqh_last;
        if lcremove.is_null() {
            break;
        }
    }
    if lcremove.is_null() {
        return 0 as libc::c_int;
    }
    lcadd = (*lc).entry.tqe_next;
    if lcadd.is_null() {
        return 0 as libc::c_int;
    }
    if size > -needed as u_int {
        size = -needed as u_int;
    }
    layout_resize_adjust(w, lcadd, type_0, size as libc::c_int);
    layout_resize_adjust(w, lcremove, type_0, size.wrapping_neg() as libc::c_int);
    return size as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn layout_assign_pane(
    mut lc: *mut layout_cell,
    mut wp: *mut window_pane,
) {
    layout_make_leaf(lc, wp);
    layout_fix_panes((*wp).window, (*(*wp).window).sx, (*(*wp).window).sy);
}
unsafe extern "C" fn layout_new_pane_size(
    mut w: *mut window,
    mut previous: u_int,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut size: u_int,
    mut count_left: u_int,
    mut size_left: u_int,
) -> u_int {
    let mut new_size: u_int = 0;
    let mut min: u_int = 0;
    let mut max: u_int = 0;
    let mut available: u_int = 0;
    if count_left == 1 as libc::c_int as u_int {
        return size_left;
    }
    available = layout_resize_check(w, lc, type_0);
    min = (2 as libc::c_int + 1 as libc::c_int) as u_int
        * count_left.wrapping_sub(1 as libc::c_int as u_int);
    if type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        if ((*lc).sx).wrapping_sub(available) > min {
            min = ((*lc).sx).wrapping_sub(available);
        }
        new_size = (*lc).sx * size / previous;
    } else {
        if ((*lc).sy).wrapping_sub(available) > min {
            min = ((*lc).sy).wrapping_sub(available);
        }
        new_size = (*lc).sy * size / previous;
    }
    max = size_left.wrapping_sub(min);
    if new_size > max {
        new_size = max;
    }
    if new_size < 2 as libc::c_int as u_int {
        new_size = 2 as libc::c_int as u_int;
    }
    return new_size;
}
unsafe extern "C" fn layout_set_size_check(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut new_size: u_int = 0;
    let mut available: u_int = 0;
    let mut previous: u_int = 0;
    let mut count: u_int = 0;
    let mut idx: u_int = 0;
    if (*lc).type_0 as libc::c_uint == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return (size >= 2 as libc::c_int) as libc::c_int;
    }
    available = size as u_int;
    count = 0 as libc::c_int as u_int;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        count = count.wrapping_add(1);
        count;
        lcchild = (*lcchild).entry.tqe_next;
    }
    if (*lc).type_0 as libc::c_uint == type_0 as libc::c_uint {
        if type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
            previous = (*lc).sx;
        } else {
            previous = (*lc).sy;
        }
        idx = 0 as libc::c_int as u_int;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            new_size = layout_new_pane_size(
                w,
                previous,
                lcchild,
                type_0,
                size as u_int,
                count.wrapping_sub(idx),
                available,
            );
            if new_size > available {
                return 0 as libc::c_int;
            }
            available = available
                .wrapping_sub(new_size.wrapping_add(1 as libc::c_int as u_int));
            if layout_set_size_check(w, lcchild, type_0, new_size as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            idx = idx.wrapping_add(1);
            idx;
            lcchild = (*lcchild).entry.tqe_next;
        }
    } else {
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            if !((*lcchild).type_0 as libc::c_uint
                == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint)
            {
                if layout_set_size_check(w, lcchild, type_0, size) == 0 {
                    return 0 as libc::c_int;
                }
            }
            lcchild = (*lcchild).entry.tqe_next;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn layout_resize_child_cells(
    mut w: *mut window,
    mut lc: *mut layout_cell,
) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut previous: u_int = 0;
    let mut available: u_int = 0;
    let mut count: u_int = 0;
    let mut idx: u_int = 0;
    if (*lc).type_0 as libc::c_uint == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return;
    }
    count = 0 as libc::c_int as u_int;
    previous = 0 as libc::c_int as u_int;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        count = count.wrapping_add(1);
        count;
        if (*lc).type_0 as libc::c_uint
            == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
        {
            previous = previous.wrapping_add((*lcchild).sx);
        } else if (*lc).type_0 as libc::c_uint
            == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
        {
            previous = previous.wrapping_add((*lcchild).sy);
        }
        lcchild = (*lcchild).entry.tqe_next;
    }
    previous = previous.wrapping_add(count.wrapping_sub(1 as libc::c_int as u_int));
    available = 0 as libc::c_int as u_int;
    if (*lc).type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        available = (*lc).sx;
    } else if (*lc).type_0 as libc::c_uint
        == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
    {
        available = (*lc).sy;
    }
    idx = 0 as libc::c_int as u_int;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        if (*lc).type_0 as libc::c_uint
            == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
        {
            (*lcchild).sx = (*lc).sx;
            (*lcchild).xoff = (*lc).xoff;
        } else {
            (*lcchild)
                .sx = layout_new_pane_size(
                w,
                previous,
                lcchild,
                (*lc).type_0,
                (*lc).sx,
                count.wrapping_sub(idx),
                available,
            );
            available = available
                .wrapping_sub(((*lcchild).sx).wrapping_add(1 as libc::c_int as u_int));
        }
        if (*lc).type_0 as libc::c_uint
            == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
        {
            (*lcchild).sy = (*lc).sy;
        } else {
            (*lcchild)
                .sy = layout_new_pane_size(
                w,
                previous,
                lcchild,
                (*lc).type_0,
                (*lc).sy,
                count.wrapping_sub(idx),
                available,
            );
            available = available
                .wrapping_sub(((*lcchild).sy).wrapping_add(1 as libc::c_int as u_int));
        }
        layout_resize_child_cells(w, lcchild);
        idx = idx.wrapping_add(1);
        idx;
        lcchild = (*lcchild).entry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn layout_split_pane(
    mut wp: *mut window_pane,
    mut type_0: layout_type,
    mut size: libc::c_int,
    mut insert_before: libc::c_int,
    mut full_size: libc::c_int,
) -> *mut layout_cell {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcnew: *mut layout_cell = 0 as *mut layout_cell;
    let mut lc1: *mut layout_cell = 0 as *mut layout_cell;
    let mut lc2: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut size1: u_int = 0;
    let mut size2: u_int = 0;
    let mut new_size: u_int = 0;
    let mut saved_size: u_int = 0;
    let mut resize_first: u_int = 0 as libc::c_int as u_int;
    if full_size != 0 {
        lc = (*(*wp).window).layout_root;
    } else {
        lc = (*wp).layout_cell;
    }
    sx = (*lc).sx;
    sy = (*lc).sy;
    xoff = (*lc).xoff;
    yoff = (*lc).yoff;
    match type_0 as libc::c_uint {
        0 => {
            if sx < (2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as u_int {
                return 0 as *mut layout_cell;
            }
        }
        1 => {
            if sy < (2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as u_int {
                return 0 as *mut layout_cell;
            }
        }
        _ => {
            fatalx(b"bad layout type\0" as *const u8 as *const libc::c_char);
        }
    }
    if type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint {
        saved_size = sx;
    } else {
        saved_size = sy;
    }
    if size < 0 as libc::c_int {
        size2 = (saved_size.wrapping_add(1 as libc::c_int as u_int)
            / 2 as libc::c_int as u_int)
            .wrapping_sub(1 as libc::c_int as u_int);
    } else if insert_before != 0 {
        size2 = saved_size
            .wrapping_sub(size as u_int)
            .wrapping_sub(1 as libc::c_int as u_int);
    } else {
        size2 = size as u_int;
    }
    if size2 < 2 as libc::c_int as u_int {
        size2 = 2 as libc::c_int as u_int;
    } else if size2 > saved_size.wrapping_sub(2 as libc::c_int as u_int) {
        size2 = saved_size.wrapping_sub(2 as libc::c_int as u_int);
    }
    size1 = saved_size.wrapping_sub(1 as libc::c_int as u_int).wrapping_sub(size2);
    if insert_before != 0 {
        new_size = size2;
    } else {
        new_size = size1;
    }
    if full_size != 0
        && layout_set_size_check((*wp).window, lc, type_0, new_size as libc::c_int) == 0
    {
        return 0 as *mut layout_cell;
    }
    if !((*lc).parent).is_null()
        && (*(*lc).parent).type_0 as libc::c_uint == type_0 as libc::c_uint
    {
        lcparent = (*lc).parent;
        lcnew = layout_create_cell(lcparent);
        if insert_before != 0 {
            (*lcnew).entry.tqe_prev = (*lc).entry.tqe_prev;
            (*lcnew).entry.tqe_next = lc;
            *(*lc).entry.tqe_prev = lcnew;
            (*lc).entry.tqe_prev = &mut (*lcnew).entry.tqe_next;
        } else {
            (*lcnew).entry.tqe_next = (*lc).entry.tqe_next;
            if !((*lcnew).entry.tqe_next).is_null() {
                (*(*lcnew).entry.tqe_next).entry.tqe_prev = &mut (*lcnew).entry.tqe_next;
            } else {
                (*lcparent).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
            }
            (*lc).entry.tqe_next = lcnew;
            (*lcnew).entry.tqe_prev = &mut (*lc).entry.tqe_next;
        }
    } else if full_size != 0 && ((*lc).parent).is_null()
        && (*lc).type_0 as libc::c_uint == type_0 as libc::c_uint
    {
        if (*lc).type_0 as libc::c_uint
            == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
        {
            (*lc).sx = new_size;
            layout_resize_child_cells((*wp).window, lc);
            (*lc).sx = saved_size;
        } else if (*lc).type_0 as libc::c_uint
            == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
        {
            (*lc).sy = new_size;
            layout_resize_child_cells((*wp).window, lc);
            (*lc).sy = saved_size;
        }
        resize_first = 1 as libc::c_int as u_int;
        lcnew = layout_create_cell(lc);
        size = saved_size.wrapping_sub(1 as libc::c_int as u_int).wrapping_sub(new_size)
            as libc::c_int;
        if (*lc).type_0 as libc::c_uint
            == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
        {
            layout_set_size(
                lcnew,
                size as u_int,
                sy,
                0 as libc::c_int as u_int,
                0 as libc::c_int as u_int,
            );
        } else if (*lc).type_0 as libc::c_uint
            == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
        {
            layout_set_size(
                lcnew,
                sx,
                size as u_int,
                0 as libc::c_int as u_int,
                0 as libc::c_int as u_int,
            );
        }
        if insert_before != 0 {
            (*lcnew).entry.tqe_next = (*lc).cells.tqh_first;
            if !((*lcnew).entry.tqe_next).is_null() {
                (*(*lc).cells.tqh_first).entry.tqe_prev = &mut (*lcnew).entry.tqe_next;
            } else {
                (*lc).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
            }
            (*lc).cells.tqh_first = lcnew;
            (*lcnew).entry.tqe_prev = &mut (*lc).cells.tqh_first;
        } else {
            (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
            (*lcnew).entry.tqe_prev = (*lc).cells.tqh_last;
            *(*lc).cells.tqh_last = lcnew;
            (*lc).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
        }
    } else {
        lcparent = layout_create_cell((*lc).parent);
        layout_make_node(lcparent, type_0);
        layout_set_size(lcparent, sx, sy, xoff, yoff);
        if ((*lc).parent).is_null() {
            (*(*wp).window).layout_root = lcparent;
        } else {
            (*lcparent).entry.tqe_next = (*lc).entry.tqe_next;
            if !((*lcparent).entry.tqe_next).is_null() {
                (*(*lcparent).entry.tqe_next)
                    .entry
                    .tqe_prev = &mut (*lcparent).entry.tqe_next;
            } else {
                (*(*lc).parent).cells.tqh_last = &mut (*lcparent).entry.tqe_next;
            }
            (*lcparent).entry.tqe_prev = (*lc).entry.tqe_prev;
            *(*lcparent).entry.tqe_prev = lcparent;
        }
        (*lc).parent = lcparent;
        (*lc).entry.tqe_next = (*lcparent).cells.tqh_first;
        if !((*lc).entry.tqe_next).is_null() {
            (*(*lcparent).cells.tqh_first).entry.tqe_prev = &mut (*lc).entry.tqe_next;
        } else {
            (*lcparent).cells.tqh_last = &mut (*lc).entry.tqe_next;
        }
        (*lcparent).cells.tqh_first = lc;
        (*lc).entry.tqe_prev = &mut (*lcparent).cells.tqh_first;
        lcnew = layout_create_cell(lcparent);
        if insert_before != 0 {
            (*lcnew).entry.tqe_next = (*lcparent).cells.tqh_first;
            if !((*lcnew).entry.tqe_next).is_null() {
                (*(*lcparent).cells.tqh_first)
                    .entry
                    .tqe_prev = &mut (*lcnew).entry.tqe_next;
            } else {
                (*lcparent).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
            }
            (*lcparent).cells.tqh_first = lcnew;
            (*lcnew).entry.tqe_prev = &mut (*lcparent).cells.tqh_first;
        } else {
            (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
            (*lcnew).entry.tqe_prev = (*lcparent).cells.tqh_last;
            *(*lcparent).cells.tqh_last = lcnew;
            (*lcparent).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
        }
    }
    if insert_before != 0 {
        lc1 = lcnew;
        lc2 = lc;
    } else {
        lc1 = lc;
        lc2 = lcnew;
    }
    if resize_first == 0
        && type_0 as libc::c_uint == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
    {
        layout_set_size(lc1, size1, sy, xoff, yoff);
        layout_set_size(
            lc2,
            size2,
            sy,
            xoff.wrapping_add((*lc1).sx).wrapping_add(1 as libc::c_int as u_int),
            yoff,
        );
    } else if resize_first == 0
        && type_0 as libc::c_uint == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
    {
        layout_set_size(lc1, sx, size1, xoff, yoff);
        layout_set_size(
            lc2,
            sx,
            size2,
            xoff,
            yoff.wrapping_add((*lc1).sy).wrapping_add(1 as libc::c_int as u_int),
        );
    }
    if full_size != 0 {
        if resize_first == 0 {
            layout_resize_child_cells((*wp).window, lc);
        }
        layout_fix_offsets((*(*wp).window).layout_root);
    } else {
        layout_make_leaf(lc, wp);
    }
    return lcnew;
}
#[no_mangle]
pub unsafe extern "C" fn layout_close_pane(mut wp: *mut window_pane) {
    let mut w: *mut window = (*wp).window;
    layout_destroy_cell(w, (*wp).layout_cell, &mut (*w).layout_root);
    if !((*w).layout_root).is_null() {
        layout_fix_offsets((*w).layout_root);
        layout_fix_panes(w, (*w).sx, (*w).sy);
    }
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
}
#[no_mangle]
pub unsafe extern "C" fn layout_spread_cell(
    mut w: *mut window,
    mut parent: *mut layout_cell,
) -> libc::c_int {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut number: u_int = 0;
    let mut each: u_int = 0;
    let mut size: u_int = 0;
    let mut change: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    number = 0 as libc::c_int as u_int;
    lc = (*parent).cells.tqh_first;
    while !lc.is_null() {
        number = number.wrapping_add(1);
        number;
        lc = (*lc).entry.tqe_next;
    }
    if number <= 1 as libc::c_int as u_int {
        return 0 as libc::c_int;
    }
    if (*parent).type_0 as libc::c_uint
        == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
    {
        size = (*parent).sx;
    } else if (*parent).type_0 as libc::c_uint
        == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
    {
        size = (*parent).sy;
    } else {
        return 0 as libc::c_int
    }
    each = size.wrapping_sub(number.wrapping_sub(1 as libc::c_int as u_int)) / number;
    changed = 0 as libc::c_int;
    lc = (*parent).cells.tqh_first;
    while !lc.is_null() {
        if ((*lc).entry.tqe_next).is_null() {
            each = size
                .wrapping_sub(
                    each.wrapping_add(1 as libc::c_int as u_int)
                        * number.wrapping_sub(1 as libc::c_int as u_int),
                );
        }
        change = 0 as libc::c_int;
        if (*parent).type_0 as libc::c_uint
            == LAYOUT_LEFTRIGHT as libc::c_int as libc::c_uint
        {
            change = each.wrapping_sub((*lc).sx as libc::c_int as u_int) as libc::c_int;
            layout_resize_adjust(w, lc, LAYOUT_LEFTRIGHT, change);
        } else if (*parent).type_0 as libc::c_uint
            == LAYOUT_TOPBOTTOM as libc::c_int as libc::c_uint
        {
            change = each.wrapping_sub((*lc).sy as libc::c_int as u_int) as libc::c_int;
            layout_resize_adjust(w, lc, LAYOUT_TOPBOTTOM, change);
        }
        if change != 0 as libc::c_int {
            changed = 1 as libc::c_int;
        }
        lc = (*lc).entry.tqe_next;
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn layout_spread_out(mut wp: *mut window_pane) {
    let mut parent: *mut layout_cell = 0 as *mut layout_cell;
    let mut w: *mut window = (*wp).window;
    parent = (*(*wp).layout_cell).parent;
    if parent.is_null() {
        return;
    }
    loop {
        if layout_spread_cell(w, parent) != 0 {
            layout_fix_offsets(parent);
            layout_fix_panes(w, (*w).sx, (*w).sy);
            break;
        } else {
            parent = (*parent).parent;
            if parent.is_null() {
                break;
            }
        }
    };
}
