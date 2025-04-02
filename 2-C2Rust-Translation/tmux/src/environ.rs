use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type options;
    pub type hooks;
    pub type screen_titles;
    pub type input_ctx;
    pub type format_tree;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type options_entry;
    fn free(_: *mut libc::c_void);
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    static mut environ: *mut *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut global_options: *mut options;
    static mut global_environ: *mut environ;
    static mut socket_path: *const libc::c_char;
    fn options_get(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_array_get(_: *mut options_entry, _: u_int) -> *const libc::c_char;
    fn options_array_size(_: *mut options_entry, _: *mut u_int) -> libc::c_int;
    fn options_get_string(
        _: *mut options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn environ_set(
        _: *mut environ,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
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
pub struct environ {
    pub rbh_root: *mut environ_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn environ_RB_REMOVE_COLOR(
    mut head: *mut environ,
    mut parent: *mut environ_entry,
    mut elm: *mut environ_entry,
) {
    let mut tmp: *mut environ_entry = 0 as *mut environ_entry;
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
                    let mut oleft: *mut environ_entry = 0 as *mut environ_entry;
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
                    let mut oright: *mut environ_entry = 0 as *mut environ_entry;
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
pub unsafe extern "C" fn environ_RB_INSERT_COLOR(
    mut head: *mut environ,
    mut elm: *mut environ_entry,
) {
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    let mut gparent: *mut environ_entry = 0 as *mut environ_entry;
    let mut tmp: *mut environ_entry = 0 as *mut environ_entry;
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
#[no_mangle]
pub unsafe extern "C" fn environ_RB_REMOVE(
    mut head: *mut environ,
    mut elm: *mut environ_entry,
) -> *mut environ_entry {
    let mut current_block: u64;
    let mut child: *mut environ_entry = 0 as *mut environ_entry;
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    let mut old: *mut environ_entry = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut environ_entry = 0 as *mut environ_entry;
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
        current_block = 16245875567908022332;
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
        environ_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn environ_RB_FIND(
    mut head: *mut environ,
    mut elm: *mut environ_entry,
) -> *mut environ_entry {
    let mut tmp: *mut environ_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = environ_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut environ_entry;
}
#[no_mangle]
pub unsafe extern "C" fn environ_RB_PREV(
    mut elm: *mut environ_entry,
) -> *mut environ_entry {
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
pub unsafe extern "C" fn environ_RB_NFIND(
    mut head: *mut environ,
    mut elm: *mut environ_entry,
) -> *mut environ_entry {
    let mut tmp: *mut environ_entry = (*head).rbh_root;
    let mut res: *mut environ_entry = 0 as *mut environ_entry;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = environ_cmp(elm, tmp);
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
pub unsafe extern "C" fn environ_RB_INSERT(
    mut head: *mut environ,
    mut elm: *mut environ_entry,
) -> *mut environ_entry {
    let mut tmp: *mut environ_entry = 0 as *mut environ_entry;
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = environ_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut environ_entry;
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
    environ_RB_INSERT_COLOR(head, elm);
    return 0 as *mut environ_entry;
}
#[no_mangle]
pub unsafe extern "C" fn environ_RB_MINMAX(
    mut head: *mut environ,
    mut val: libc::c_int,
) -> *mut environ_entry {
    let mut tmp: *mut environ_entry = (*head).rbh_root;
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
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
pub unsafe extern "C" fn environ_RB_NEXT(
    mut elm: *mut environ_entry,
) -> *mut environ_entry {
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
unsafe extern "C" fn environ_cmp(
    mut envent1: *mut environ_entry,
    mut envent2: *mut environ_entry,
) -> libc::c_int {
    return strcmp((*envent1).name, (*envent2).name);
}
#[no_mangle]
pub unsafe extern "C" fn environ_create() -> *mut environ {
    let mut env: *mut environ = 0 as *mut environ;
    env = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<environ>() as libc::c_ulong,
    ) as *mut environ;
    (*env).rbh_root = 0 as *mut environ_entry;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn environ_free(mut env: *mut environ) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut envent1: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_RB_MINMAX(env, -(1 as libc::c_int));
    while !envent.is_null()
        && {
            envent1 = environ_RB_NEXT(envent);
            1 as libc::c_int != 0
        }
    {
        environ_RB_REMOVE(env, envent);
        free((*envent).name as *mut libc::c_void);
        free((*envent).value as *mut libc::c_void);
        free(envent as *mut libc::c_void);
        envent = envent1;
    }
    free(env as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn environ_first(mut env: *mut environ) -> *mut environ_entry {
    return environ_RB_MINMAX(env, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn environ_next(
    mut envent: *mut environ_entry,
) -> *mut environ_entry {
    return environ_RB_NEXT(envent);
}
#[no_mangle]
pub unsafe extern "C" fn environ_copy(
    mut srcenv: *mut environ,
    mut dstenv: *mut environ,
) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_RB_MINMAX(srcenv, -(1 as libc::c_int));
    while !envent.is_null() {
        if ((*envent).value).is_null() {
            environ_clear(dstenv, (*envent).name);
        } else {
            environ_set(
                dstenv,
                (*envent).name,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*envent).value,
            );
        }
        envent = environ_RB_NEXT(envent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn environ_find(
    mut env: *mut environ,
    mut name: *const libc::c_char,
) -> *mut environ_entry {
    let mut envent: environ_entry = environ_entry {
        name: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
        entry: C2RustUnnamed_11 {
            rbe_left: 0 as *mut environ_entry,
            rbe_right: 0 as *mut environ_entry,
            rbe_parent: 0 as *mut environ_entry,
            rbe_color: 0,
        },
    };
    envent.name = name as *mut libc::c_char;
    return environ_RB_FIND(env, &mut envent);
}
#[no_mangle]
pub unsafe extern "C" fn environ_clear(
    mut env: *mut environ,
    mut name: *const libc::c_char,
) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_find(env, name);
    if !envent.is_null() {
        free((*envent).value as *mut libc::c_void);
        (*envent).value = 0 as *mut libc::c_char;
    } else {
        envent = xmalloc(::core::mem::size_of::<environ_entry>() as libc::c_ulong)
            as *mut environ_entry;
        (*envent).name = xstrdup(name);
        (*envent).value = 0 as *mut libc::c_char;
        environ_RB_INSERT(env, envent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn environ_put(
    mut env: *mut environ,
    mut var: *const libc::c_char,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = strchr(var, '=' as i32);
    if value.is_null() {
        return;
    }
    value = value.offset(1);
    value;
    name = xstrdup(var);
    *name
        .offset(
            strcspn(name, b"=\0" as *const u8 as *const libc::c_char) as isize,
        ) = '\0' as i32 as libc::c_char;
    environ_set(env, name, b"%s\0" as *const u8 as *const libc::c_char, value);
    free(name as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn environ_unset(
    mut env: *mut environ,
    mut name: *const libc::c_char,
) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_find(env, name);
    if envent.is_null() {
        return;
    }
    environ_RB_REMOVE(env, envent);
    free((*envent).name as *mut libc::c_void);
    free((*envent).value as *mut libc::c_void);
    free(envent as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn environ_update(
    mut oo: *mut options,
    mut src: *mut environ,
    mut dst: *mut environ,
) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut size: u_int = 0;
    let mut idx: u_int = 0;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    o = options_get(oo, b"update-environment\0" as *const u8 as *const libc::c_char);
    if o.is_null() || options_array_size(o, &mut size) == -(1 as libc::c_int) {
        return;
    }
    idx = 0 as libc::c_int as u_int;
    while idx < size {
        value = options_array_get(o, idx);
        if !value.is_null() {
            envent = environ_find(src, value);
            if envent.is_null() {
                environ_clear(dst, value);
            } else {
                environ_set(
                    dst,
                    (*envent).name,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*envent).value,
                );
            }
        }
        idx = idx.wrapping_add(1);
        idx;
    }
}
#[no_mangle]
pub unsafe extern "C" fn environ_push(mut env: *mut environ) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    environ = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    envent = environ_RB_MINMAX(env, -(1 as libc::c_int));
    while !envent.is_null() {
        if !((*envent).value).is_null() && *(*envent).name as libc::c_int != '\0' as i32
        {
            setenv((*envent).name, (*envent).value, 1 as libc::c_int);
        }
        envent = environ_RB_NEXT(envent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn environ_for_session(
    mut s: *mut session,
    mut no_TERM: libc::c_int,
) -> *mut environ {
    let mut env: *mut environ = 0 as *mut environ;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    env = environ_create();
    environ_copy(global_environ, env);
    if !s.is_null() {
        environ_copy((*s).environ, env);
    }
    if no_TERM == 0 {
        value = options_get_string(
            global_options,
            b"default-terminal\0" as *const u8 as *const libc::c_char,
        );
        environ_set(
            env,
            b"TERM\0" as *const u8 as *const libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char,
            value,
        );
    }
    if !s.is_null() {
        idx = (*s).id as libc::c_int;
    } else {
        idx = -(1 as libc::c_int);
    }
    environ_set(
        env,
        b"TMUX\0" as *const u8 as *const libc::c_char,
        b"%s,%ld,%d\0" as *const u8 as *const libc::c_char,
        socket_path,
        getpid() as libc::c_long,
        idx,
    );
    return env;
}
