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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn notify_window(_: *const libc::c_char, _: *mut window);
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn server_redraw_window(_: *mut window);
    fn window_count_panes(_: *mut window) -> u_int;
    fn layout_create_cell(_: *mut layout_cell) -> *mut layout_cell;
    fn layout_print_cell(_: *mut layout_cell, _: *const libc::c_char, _: u_int);
    fn layout_set_size(_: *mut layout_cell, _: u_int, _: u_int, _: u_int, _: u_int);
    fn layout_make_leaf(_: *mut layout_cell, _: *mut window_pane);
    fn layout_make_node(_: *mut layout_cell, _: layout_type);
    fn layout_fix_offsets(_: *mut layout_cell);
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int);
    fn layout_resize_adjust(
        _: *mut window,
        _: *mut layout_cell,
        _: layout_type,
        _: libc::c_int,
    );
    fn layout_free(_: *mut window);
    fn layout_spread_cell(_: *mut window, _: *mut layout_cell) -> libc::c_int;
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
pub struct C2RustUnnamed_34 {
    pub name: *const libc::c_char,
    pub arrange: Option::<unsafe extern "C" fn(*mut window) -> ()>,
}
static mut layout_sets: [C2RustUnnamed_34; 5] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_34 {
                name: b"even-horizontal\0" as *const u8 as *const libc::c_char,
                arrange: Some(
                    layout_set_even_h as unsafe extern "C" fn(*mut window) -> (),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_34 {
                name: b"even-vertical\0" as *const u8 as *const libc::c_char,
                arrange: Some(
                    layout_set_even_v as unsafe extern "C" fn(*mut window) -> (),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_34 {
                name: b"main-horizontal\0" as *const u8 as *const libc::c_char,
                arrange: Some(
                    layout_set_main_h as unsafe extern "C" fn(*mut window) -> (),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_34 {
                name: b"main-vertical\0" as *const u8 as *const libc::c_char,
                arrange: Some(
                    layout_set_main_v as unsafe extern "C" fn(*mut window) -> (),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_34 {
                name: b"tiled\0" as *const u8 as *const libc::c_char,
                arrange: Some(
                    layout_set_tiled as unsafe extern "C" fn(*mut window) -> (),
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn layout_set_lookup(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: u_int = 0;
    let mut matched: libc::c_int = -(1 as libc::c_int);
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_34; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong)
    {
        if strncmp(layout_sets[i as usize].name, name, strlen(name)) == 0 as libc::c_int
        {
            if matched != -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            matched = i as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return matched;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_select(
    mut w: *mut window,
    mut layout: u_int,
) -> u_int {
    if layout as libc::c_ulong
        > (::core::mem::size_of::<[C2RustUnnamed_34; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        layout = (::core::mem::size_of::<[C2RustUnnamed_34; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as u_int;
    }
    if (layout_sets[layout as usize].arrange).is_some() {
        (layout_sets[layout as usize].arrange).expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_next(mut w: *mut window) -> u_int {
    let mut layout: u_int = 0;
    if (*w).lastlayout == -(1 as libc::c_int) {
        layout = 0 as libc::c_int as u_int;
    } else {
        layout = ((*w).lastlayout + 1 as libc::c_int) as u_int;
        if layout as libc::c_ulong
            > (::core::mem::size_of::<[C2RustUnnamed_34; 5]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            layout = 0 as libc::c_int as u_int;
        }
    }
    if (layout_sets[layout as usize].arrange).is_some() {
        (layout_sets[layout as usize].arrange).expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_previous(mut w: *mut window) -> u_int {
    let mut layout: u_int = 0;
    if (*w).lastlayout == -(1 as libc::c_int) {
        layout = (::core::mem::size_of::<[C2RustUnnamed_34; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as u_int;
    } else {
        layout = (*w).lastlayout as u_int;
        if layout == 0 as libc::c_int as u_int {
            layout = (::core::mem::size_of::<[C2RustUnnamed_34; 5]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_34>() as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as u_int;
        } else {
            layout = layout.wrapping_sub(1);
            layout;
        }
    }
    if (layout_sets[layout as usize].arrange).is_some() {
        (layout_sets[layout as usize].arrange).expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
unsafe extern "C" fn layout_set_even(mut w: *mut window, mut type_0: layout_type) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcnew: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"layout_set_even\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    n = window_count_panes(w);
    if n <= 1 as libc::c_int as u_int {
        return;
    }
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        (*w).sx,
        (*w).sy,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_node(lc, type_0);
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        lcnew = layout_create_cell(lc);
        layout_make_leaf(lcnew, wp);
        (*lcnew).sx = (*w).sx;
        (*lcnew).sy = (*w).sy;
        (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
        (*lcnew).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcnew;
        (*lc).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
        wp = (*wp).entry.tqe_next;
    }
    layout_spread_cell(w, lc);
    layout_fix_offsets(lc);
    layout_fix_panes(w, (*w).sx, (*w).sy);
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"layout_set_even\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
    server_redraw_window(w);
}
unsafe extern "C" fn layout_set_even_h(mut w: *mut window) {
    layout_set_even(w, LAYOUT_LEFTRIGHT);
}
unsafe extern "C" fn layout_set_even_v(mut w: *mut window) {
    layout_set_even(w, LAYOUT_TOPBOTTOM);
}
unsafe extern "C" fn layout_set_main_h(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcmain: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcrow: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut mainheight: u_int = 0;
    let mut otherheight: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    let mut totalrows: u_int = 0;
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"layout_set_main_h\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    n = window_count_panes(w);
    if n <= 1 as libc::c_int as u_int {
        return;
    }
    n = n.wrapping_sub(1);
    n;
    columns = ((*w).sx).wrapping_add(1 as libc::c_int as u_int)
        / (2 as libc::c_int + 1 as libc::c_int) as u_int;
    if columns == 0 as libc::c_int as u_int {
        columns = 1 as libc::c_int as u_int;
    }
    rows = (1 as libc::c_int as u_int)
        .wrapping_add(n.wrapping_sub(1 as libc::c_int as u_int) / columns);
    columns = (1 as libc::c_int as u_int)
        .wrapping_add(n.wrapping_sub(1 as libc::c_int as u_int) / rows);
    width = ((*w).sx).wrapping_sub(n.wrapping_sub(1 as libc::c_int as u_int)) / columns;
    mainheight = (options_get_number(
        (*w).options,
        b"main-pane-height\0" as *const u8 as *const libc::c_char,
    ) + 1 as libc::c_int as libc::c_longlong) as u_int;
    otherheight = (options_get_number(
        (*w).options,
        b"other-pane-height\0" as *const u8 as *const libc::c_char,
    ) + 1 as libc::c_int as libc::c_longlong) as u_int;
    if otherheight > 1 as libc::c_int as u_int
        && ((*w).sy).wrapping_sub(otherheight) > mainheight
    {
        mainheight = ((*w).sy).wrapping_sub(otherheight);
    }
    if mainheight < (2 as libc::c_int + 1 as libc::c_int) as u_int {
        mainheight = (2 as libc::c_int + 1 as libc::c_int) as u_int;
    }
    totalrows = (rows * (2 as libc::c_int + 1 as libc::c_int) as u_int)
        .wrapping_sub(1 as libc::c_int as u_int);
    if mainheight.wrapping_add(totalrows) > (*w).sy {
        if totalrows
            .wrapping_add(2 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int) > (*w).sy
        {
            mainheight = (2 as libc::c_int + 2 as libc::c_int) as u_int;
        } else {
            mainheight = ((*w).sy).wrapping_sub(totalrows);
        }
        height = 2 as libc::c_int as u_int;
    } else {
        height = ((*w).sy)
            .wrapping_sub(mainheight)
            .wrapping_sub(rows.wrapping_sub(1 as libc::c_int as u_int)) / rows;
    }
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        (*w).sx,
        mainheight
            .wrapping_add(rows * height.wrapping_add(1 as libc::c_int as u_int))
            .wrapping_sub(1 as libc::c_int as u_int),
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_node(lc, LAYOUT_TOPBOTTOM);
    lcmain = layout_create_cell(lc);
    layout_set_size(
        lcmain,
        (*w).sx,
        mainheight.wrapping_sub(1 as libc::c_int as u_int),
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_leaf(lcmain, (*w).panes.tqh_first);
    (*lcmain).entry.tqe_next = 0 as *mut layout_cell;
    (*lcmain).entry.tqe_prev = (*lc).cells.tqh_last;
    *(*lc).cells.tqh_last = lcmain;
    (*lc).cells.tqh_last = &mut (*lcmain).entry.tqe_next;
    wp = (*(*w).panes.tqh_first).entry.tqe_next;
    j = 0 as libc::c_int as u_int;
    while j < rows {
        if wp.is_null() {
            break;
        }
        lcrow = layout_create_cell(lc);
        layout_set_size(
            lcrow,
            (*w).sx,
            height,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
        (*lcrow).entry.tqe_next = 0 as *mut layout_cell;
        (*lcrow).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcrow;
        (*lc).cells.tqh_last = &mut (*lcrow).entry.tqe_next;
        if columns == 1 as libc::c_int as u_int {
            layout_make_leaf(lcrow, wp);
            wp = (*wp).entry.tqe_next;
        } else {
            layout_make_node(lcrow, LAYOUT_LEFTRIGHT);
            i = 0 as libc::c_int as u_int;
            while i < columns {
                lcchild = layout_create_cell(lcrow);
                layout_set_size(
                    lcchild,
                    width,
                    height,
                    0 as libc::c_int as u_int,
                    0 as libc::c_int as u_int,
                );
                layout_make_leaf(lcchild, wp);
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lcrow).cells.tqh_last;
                *(*lcrow).cells.tqh_last = lcchild;
                (*lcrow).cells.tqh_last = &mut (*lcchild).entry.tqe_next;
                wp = (*wp).entry.tqe_next;
                if wp.is_null() {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            if i == columns {
                i = i.wrapping_sub(1);
                i;
            }
            used = (i.wrapping_add(1 as libc::c_int as u_int)
                * width.wrapping_add(1 as libc::c_int as u_int))
                .wrapping_sub(1 as libc::c_int as u_int);
            if !((*w).sx <= used) {
                lcchild = *(*((*lcrow).cells.tqh_last as *mut layout_cells)).tqh_last;
                layout_resize_adjust(
                    w,
                    lcchild,
                    LAYOUT_LEFTRIGHT,
                    ((*w).sx).wrapping_sub(used) as libc::c_int,
                );
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    used = mainheight
        .wrapping_add(rows * height)
        .wrapping_add(rows)
        .wrapping_sub(1 as libc::c_int as u_int);
    if (*w).sy > used {
        lcrow = *(*((*lc).cells.tqh_last as *mut layout_cells)).tqh_last;
        layout_resize_adjust(
            w,
            lcrow,
            LAYOUT_TOPBOTTOM,
            ((*w).sy).wrapping_sub(used) as libc::c_int,
        );
    }
    layout_fix_offsets(lc);
    layout_fix_panes(w, (*w).sx, (*w).sy);
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"layout_set_main_h\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
    server_redraw_window(w);
}
unsafe extern "C" fn layout_set_main_v(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcmain: *mut layout_cell = 0 as *mut layout_cell;
    let mut lccolumn: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut mainwidth: u_int = 0;
    let mut otherwidth: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    let mut totalcolumns: u_int = 0;
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"layout_set_main_v\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    n = window_count_panes(w);
    if n <= 1 as libc::c_int as u_int {
        return;
    }
    n = n.wrapping_sub(1);
    n;
    rows = ((*w).sy).wrapping_add(1 as libc::c_int as u_int)
        / (2 as libc::c_int + 1 as libc::c_int) as u_int;
    if rows == 0 as libc::c_int as u_int {
        rows = 1 as libc::c_int as u_int;
    }
    columns = (1 as libc::c_int as u_int)
        .wrapping_add(n.wrapping_sub(1 as libc::c_int as u_int) / rows);
    rows = (1 as libc::c_int as u_int)
        .wrapping_add(n.wrapping_sub(1 as libc::c_int as u_int) / columns);
    height = ((*w).sy).wrapping_sub(n.wrapping_sub(1 as libc::c_int as u_int)) / rows;
    mainwidth = (options_get_number(
        (*w).options,
        b"main-pane-width\0" as *const u8 as *const libc::c_char,
    ) + 1 as libc::c_int as libc::c_longlong) as u_int;
    otherwidth = (options_get_number(
        (*w).options,
        b"other-pane-width\0" as *const u8 as *const libc::c_char,
    ) + 1 as libc::c_int as libc::c_longlong) as u_int;
    if otherwidth > 1 as libc::c_int as u_int
        && ((*w).sx).wrapping_sub(otherwidth) > mainwidth
    {
        mainwidth = ((*w).sx).wrapping_sub(otherwidth);
    }
    if mainwidth < (2 as libc::c_int + 1 as libc::c_int) as u_int {
        mainwidth = (2 as libc::c_int + 1 as libc::c_int) as u_int;
    }
    totalcolumns = (columns * (2 as libc::c_int + 1 as libc::c_int) as u_int)
        .wrapping_sub(1 as libc::c_int as u_int);
    if mainwidth.wrapping_add(totalcolumns) > (*w).sx {
        if totalcolumns
            .wrapping_add(2 as libc::c_int as u_int)
            .wrapping_add(1 as libc::c_int as u_int) > (*w).sx
        {
            mainwidth = (2 as libc::c_int + 2 as libc::c_int) as u_int;
        } else {
            mainwidth = ((*w).sx).wrapping_sub(totalcolumns);
        }
        width = 2 as libc::c_int as u_int;
    } else {
        width = ((*w).sx)
            .wrapping_sub(mainwidth)
            .wrapping_sub(columns.wrapping_sub(1 as libc::c_int as u_int)) / columns;
    }
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        mainwidth
            .wrapping_add(columns * width.wrapping_add(1 as libc::c_int as u_int))
            .wrapping_sub(1 as libc::c_int as u_int),
        (*w).sy,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_node(lc, LAYOUT_LEFTRIGHT);
    lcmain = layout_create_cell(lc);
    layout_set_size(
        lcmain,
        mainwidth.wrapping_sub(1 as libc::c_int as u_int),
        (*w).sy,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_leaf(lcmain, (*w).panes.tqh_first);
    (*lcmain).entry.tqe_next = 0 as *mut layout_cell;
    (*lcmain).entry.tqe_prev = (*lc).cells.tqh_last;
    *(*lc).cells.tqh_last = lcmain;
    (*lc).cells.tqh_last = &mut (*lcmain).entry.tqe_next;
    wp = (*(*w).panes.tqh_first).entry.tqe_next;
    j = 0 as libc::c_int as u_int;
    while j < columns {
        if wp.is_null() {
            break;
        }
        lccolumn = layout_create_cell(lc);
        layout_set_size(
            lccolumn,
            width,
            (*w).sy,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
        (*lccolumn).entry.tqe_next = 0 as *mut layout_cell;
        (*lccolumn).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lccolumn;
        (*lc).cells.tqh_last = &mut (*lccolumn).entry.tqe_next;
        if rows == 1 as libc::c_int as u_int {
            layout_make_leaf(lccolumn, wp);
            wp = (*wp).entry.tqe_next;
        } else {
            layout_make_node(lccolumn, LAYOUT_TOPBOTTOM);
            i = 0 as libc::c_int as u_int;
            while i < rows {
                lcchild = layout_create_cell(lccolumn);
                layout_set_size(
                    lcchild,
                    width,
                    height,
                    0 as libc::c_int as u_int,
                    0 as libc::c_int as u_int,
                );
                layout_make_leaf(lcchild, wp);
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lccolumn).cells.tqh_last;
                *(*lccolumn).cells.tqh_last = lcchild;
                (*lccolumn).cells.tqh_last = &mut (*lcchild).entry.tqe_next;
                wp = (*wp).entry.tqe_next;
                if wp.is_null() {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            if i == rows {
                i = i.wrapping_sub(1);
                i;
            }
            used = (i.wrapping_add(1 as libc::c_int as u_int)
                * height.wrapping_add(1 as libc::c_int as u_int))
                .wrapping_sub(1 as libc::c_int as u_int);
            if !((*w).sy <= used) {
                lcchild = *(*((*lccolumn).cells.tqh_last as *mut layout_cells)).tqh_last;
                layout_resize_adjust(
                    w,
                    lcchild,
                    LAYOUT_TOPBOTTOM,
                    ((*w).sy).wrapping_sub(used) as libc::c_int,
                );
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    used = mainwidth
        .wrapping_add(columns * width)
        .wrapping_add(columns)
        .wrapping_sub(1 as libc::c_int as u_int);
    if (*w).sx > used {
        lccolumn = *(*((*lc).cells.tqh_last as *mut layout_cells)).tqh_last;
        layout_resize_adjust(
            w,
            lccolumn,
            LAYOUT_LEFTRIGHT,
            ((*w).sx).wrapping_sub(used) as libc::c_int,
        );
    }
    layout_fix_offsets(lc);
    layout_fix_panes(w, (*w).sx, (*w).sy);
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"layout_set_main_v\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
    server_redraw_window(w);
}
unsafe extern "C" fn layout_set_tiled(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcrow: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"layout_set_tiled\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    n = window_count_panes(w);
    if n <= 1 as libc::c_int as u_int {
        return;
    }
    columns = 1 as libc::c_int as u_int;
    rows = columns;
    while rows * columns < n {
        rows = rows.wrapping_add(1);
        rows;
        if rows * columns < n {
            columns = columns.wrapping_add(1);
            columns;
        }
    }
    width = ((*w).sx).wrapping_sub(columns.wrapping_sub(1 as libc::c_int as u_int))
        / columns;
    if width < 2 as libc::c_int as u_int {
        width = 2 as libc::c_int as u_int;
    }
    height = ((*w).sy).wrapping_sub(rows.wrapping_sub(1 as libc::c_int as u_int)) / rows;
    if height < 2 as libc::c_int as u_int {
        height = 2 as libc::c_int as u_int;
    }
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        (width.wrapping_add(1 as libc::c_int as u_int) * columns)
            .wrapping_sub(1 as libc::c_int as u_int),
        (height.wrapping_add(1 as libc::c_int as u_int) * rows)
            .wrapping_sub(1 as libc::c_int as u_int),
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    layout_make_node(lc, LAYOUT_TOPBOTTOM);
    wp = (*w).panes.tqh_first;
    j = 0 as libc::c_int as u_int;
    while j < rows {
        if wp.is_null() {
            break;
        }
        lcrow = layout_create_cell(lc);
        layout_set_size(
            lcrow,
            (*w).sx,
            height,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
        (*lcrow).entry.tqe_next = 0 as *mut layout_cell;
        (*lcrow).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcrow;
        (*lc).cells.tqh_last = &mut (*lcrow).entry.tqe_next;
        if n.wrapping_sub(j * columns) == 1 as libc::c_int as u_int
            || columns == 1 as libc::c_int as u_int
        {
            layout_make_leaf(lcrow, wp);
            wp = (*wp).entry.tqe_next;
        } else {
            layout_make_node(lcrow, LAYOUT_LEFTRIGHT);
            i = 0 as libc::c_int as u_int;
            while i < columns {
                lcchild = layout_create_cell(lcrow);
                layout_set_size(
                    lcchild,
                    width,
                    height,
                    0 as libc::c_int as u_int,
                    0 as libc::c_int as u_int,
                );
                layout_make_leaf(lcchild, wp);
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lcrow).cells.tqh_last;
                *(*lcrow).cells.tqh_last = lcchild;
                (*lcrow).cells.tqh_last = &mut (*lcchild).entry.tqe_next;
                wp = (*wp).entry.tqe_next;
                if wp.is_null() {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            if i == columns {
                i = i.wrapping_sub(1);
                i;
            }
            used = (i.wrapping_add(1 as libc::c_int as u_int)
                * width.wrapping_add(1 as libc::c_int as u_int))
                .wrapping_sub(1 as libc::c_int as u_int);
            if !((*w).sx <= used) {
                lcchild = *(*((*lcrow).cells.tqh_last as *mut layout_cells)).tqh_last;
                layout_resize_adjust(
                    w,
                    lcchild,
                    LAYOUT_LEFTRIGHT,
                    ((*w).sx).wrapping_sub(used) as libc::c_int,
                );
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    used = (rows * height).wrapping_add(rows).wrapping_sub(1 as libc::c_int as u_int);
    if (*w).sy > used {
        lcrow = *(*((*lc).cells.tqh_last as *mut layout_cells)).tqh_last;
        layout_resize_adjust(
            w,
            lcrow,
            LAYOUT_TOPBOTTOM,
            ((*w).sy).wrapping_sub(used) as libc::c_int,
        );
    }
    layout_fix_offsets(lc);
    layout_fix_panes(w, (*w).sx, (*w).sy);
    layout_print_cell(
        (*w).layout_root,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"layout_set_tiled\0"))
            .as_ptr(),
        1 as libc::c_int as u_int,
    );
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
    server_redraw_window(w);
}
