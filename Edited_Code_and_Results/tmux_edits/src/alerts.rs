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
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_initialized(ev: *const event) -> libc::c_int;
    fn event_once(
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
        _: *const timeval,
    ) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn notify_winlink(_: *const libc::c_char, _: *mut winlink);
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn tty_putcode(_: *mut tty, _: tty_code_code);
    static mut clients: clients;
    fn server_status_session(_: *mut session);
    fn status_message_set(_: *mut client, _: *const libc::c_char, _: ...);
    static mut windows: windows;
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn window_add_ref(_: *mut window, _: *const libc::c_char);
    fn window_remove_ref(_: *mut window, _: *const libc::c_char);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct windows {
    pub rbh_root: *mut window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub tqh_first: *mut window,
    pub tqh_last: *mut *mut window,
}
static mut alerts_fired: libc::c_int = 0;
static mut alerts_list: C2RustUnnamed_34 = C2RustUnnamed_34 {
    tqh_first: 0 as *const window as *mut window,
    tqh_last: 0 as *const *mut window as *mut *mut window,
};
unsafe extern "C" fn alerts_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut w: *mut window = arg as *mut window;
    log_debug(
        b"@%u alerts timer expired\0" as *const u8 as *const libc::c_char,
        (*w).id,
    );
    alerts_queue(w, 0x8 as libc::c_int);
}
unsafe extern "C" fn alerts_callback(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut w: *mut window = 0 as *mut window;
    let mut w1: *mut window = 0 as *mut window;
    let mut alerts: libc::c_int = 0;
    w = alerts_list.tqh_first;
    while !w.is_null()
        && {
            w1 = (*w).alerts_entry.tqe_next;
            1 as libc::c_int != 0
        }
    {
        alerts = alerts_check_all(w);
        log_debug(
            b"@%u alerts check, alerts %#x\0" as *const u8 as *const libc::c_char,
            (*w).id,
            alerts,
        );
        (*w).alerts_queued = 0 as libc::c_int;
        if !((*w).alerts_entry.tqe_next).is_null() {
            (*(*w).alerts_entry.tqe_next)
                .alerts_entry
                .tqe_prev = (*w).alerts_entry.tqe_prev;
        } else {
            alerts_list.tqh_last = (*w).alerts_entry.tqe_prev;
        }
        *(*w).alerts_entry.tqe_prev = (*w).alerts_entry.tqe_next;
        (*w).flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int);
        window_remove_ref(
            w,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"alerts_callback\0"))
                .as_ptr(),
        );
        w = w1;
    }
    alerts_fired = 0 as libc::c_int;
}
unsafe extern "C" fn alerts_action_applies(
    mut wl: *mut winlink,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut action: libc::c_int = 0;
    action = options_get_number((*(*wl).session).options, name) as libc::c_int;
    if action == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if action == 2 as libc::c_int {
        return (wl == (*(*wl).session).curw) as libc::c_int;
    }
    if action == 3 as libc::c_int {
        return (wl != (*(*wl).session).curw) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn alerts_check_all(mut w: *mut window) -> libc::c_int {
    let mut alerts: libc::c_int = 0;
    alerts = alerts_check_bell(w);
    alerts |= alerts_check_activity(w);
    alerts |= alerts_check_silence(w);
    return alerts;
}
#[no_mangle]
pub unsafe extern "C" fn alerts_check_session(mut s: *mut session) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        alerts_check_all((*wl).window);
        wl = winlinks_RB_NEXT(wl);
    }
}
unsafe extern "C" fn alerts_enabled(
    mut w: *mut window,
    mut flags: libc::c_int,
) -> libc::c_int {
    if flags & 0x1 as libc::c_int != 0 {
        if options_get_number(
            (*w).options,
            b"monitor-bell\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            return 1 as libc::c_int;
        }
    }
    if flags & 0x2 as libc::c_int != 0 {
        if options_get_number(
            (*w).options,
            b"monitor-activity\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            return 1 as libc::c_int;
        }
    }
    if flags & 0x8 as libc::c_int != 0 {
        if options_get_number(
            (*w).options,
            b"monitor-silence\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int as libc::c_longlong
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn alerts_reset_all() {
    let mut w: *mut window = 0 as *mut window;
    w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
    while !w.is_null() {
        alerts_reset(w);
        w = windows_RB_NEXT(w);
    }
}
unsafe extern "C" fn alerts_reset(mut w: *mut window) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if event_initialized(&mut (*w).alerts_timer) == 0 {
        event_set(
            &mut (*w).alerts_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                alerts_timer
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            w as *mut libc::c_void,
        );
    }
    (*w).flags &= !(0x8 as libc::c_int);
    event_del(&mut (*w).alerts_timer);
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = tv.tv_usec;
    tv
        .tv_sec = options_get_number(
        (*w).options,
        b"monitor-silence\0" as *const u8 as *const libc::c_char,
    ) as __time_t;
    log_debug(
        b"@%u alerts timer reset %u\0" as *const u8 as *const libc::c_char,
        (*w).id,
        tv.tv_sec as u_int,
    );
    if tv.tv_sec != 0 as libc::c_int as __time_t {
        event_add(&mut (*w).alerts_timer, &mut tv);
    }
}
#[no_mangle]
pub unsafe extern "C" fn alerts_queue(mut w: *mut window, mut flags: libc::c_int) {
    alerts_reset(w);
    if (*w).flags & flags != flags {
        (*w).flags |= flags;
        log_debug(
            b"@%u alerts flags added %#x\0" as *const u8 as *const libc::c_char,
            (*w).id,
            flags,
        );
    }
    if alerts_enabled(w, flags) != 0 {
        if (*w).alerts_queued == 0 {
            (*w).alerts_queued = 1 as libc::c_int;
            (*w).alerts_entry.tqe_next = 0 as *mut window;
            (*w).alerts_entry.tqe_prev = alerts_list.tqh_last;
            *alerts_list.tqh_last = w;
            alerts_list.tqh_last = &mut (*w).alerts_entry.tqe_next;
            window_add_ref(
                w,
                (*::core::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"alerts_queue\0"))
                    .as_ptr(),
            );
        }
        if alerts_fired == 0 {
            log_debug(
                b"alerts check queued (by @%u)\0" as *const u8 as *const libc::c_char,
                (*w).id,
            );
            event_once(
                -(1 as libc::c_int),
                0x1 as libc::c_int as libc::c_short,
                Some(
                    alerts_callback
                        as unsafe extern "C" fn(
                            libc::c_int,
                            libc::c_short,
                            *mut libc::c_void,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
                0 as *const timeval,
            );
            alerts_fired = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn alerts_check_bell(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if !(*w).flags & 0x1 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if options_get_number(
        (*w).options,
        b"monitor-bell\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        (*(*wl).session).flags &= !(0x4 as libc::c_int);
        wl = (*wl).wentry.tqe_next;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        s = (*wl).session;
        if (*s).curw != wl {
            (*wl).flags |= 0x1 as libc::c_int;
            server_status_session(s);
        }
        if !(alerts_action_applies(
            wl,
            b"bell-action\0" as *const u8 as *const libc::c_char,
        ) == 0)
        {
            notify_winlink(b"alert-bell\0" as *const u8 as *const libc::c_char, wl);
            if !((*s).flags & 0x4 as libc::c_int != 0) {
                (*s).flags |= 0x4 as libc::c_int;
                alerts_set_message(
                    wl,
                    b"Bell\0" as *const u8 as *const libc::c_char,
                    b"visual-bell\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        wl = (*wl).wentry.tqe_next;
    }
    return 0x1 as libc::c_int;
}
unsafe extern "C" fn alerts_check_activity(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if !(*w).flags & 0x2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if options_get_number(
        (*w).options,
        b"monitor-activity\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        (*(*wl).session).flags &= !(0x4 as libc::c_int);
        wl = (*wl).wentry.tqe_next;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if !((*wl).flags & 0x2 as libc::c_int != 0) {
            s = (*wl).session;
            if (*s).curw != wl {
                (*wl).flags |= 0x2 as libc::c_int;
                server_status_session(s);
            }
            if !(alerts_action_applies(
                wl,
                b"activity-action\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                notify_winlink(
                    b"alert-activity\0" as *const u8 as *const libc::c_char,
                    wl,
                );
                if !((*s).flags & 0x4 as libc::c_int != 0) {
                    (*s).flags |= 0x4 as libc::c_int;
                    alerts_set_message(
                        wl,
                        b"Activity\0" as *const u8 as *const libc::c_char,
                        b"visual-activity\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        wl = (*wl).wentry.tqe_next;
    }
    return 0x2 as libc::c_int;
}
unsafe extern "C" fn alerts_check_silence(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if !(*w).flags & 0x8 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if options_get_number(
        (*w).options,
        b"monitor-silence\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int as libc::c_longlong
    {
        return 0 as libc::c_int;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        (*(*wl).session).flags &= !(0x4 as libc::c_int);
        wl = (*wl).wentry.tqe_next;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if !((*wl).flags & 0x4 as libc::c_int != 0) {
            s = (*wl).session;
            if (*s).curw != wl {
                (*wl).flags |= 0x4 as libc::c_int;
                server_status_session(s);
            }
            if !(alerts_action_applies(
                wl,
                b"silence-action\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                notify_winlink(
                    b"alert-silence\0" as *const u8 as *const libc::c_char,
                    wl,
                );
                if !((*s).flags & 0x4 as libc::c_int != 0) {
                    (*s).flags |= 0x4 as libc::c_int;
                    alerts_set_message(
                        wl,
                        b"Silence\0" as *const u8 as *const libc::c_char,
                        b"visual-silence\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        wl = (*wl).wentry.tqe_next;
    }
    return 0x8 as libc::c_int;
}
unsafe extern "C" fn alerts_set_message(
    mut wl: *mut winlink,
    mut type_0: *const libc::c_char,
    mut option: *const libc::c_char,
) {
    let mut c: *mut client = 0 as *mut client;
    let mut visual: libc::c_int = 0;
    visual = options_get_number((*(*wl).session).options, option) as libc::c_int;
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session != (*wl).session || (*c).flags & 0x2000 as libc::c_int != 0) {
            if visual == 0 as libc::c_int || visual == 2 as libc::c_int {
                tty_putcode(&mut (*c).tty, TTYC_BEL);
            }
            if !(visual == 0 as libc::c_int) {
                if (*(*c).session).curw == wl {
                    status_message_set(
                        c,
                        b"%s in current window\0" as *const u8 as *const libc::c_char,
                        type_0,
                    );
                } else {
                    status_message_set(
                        c,
                        b"%s in window %d\0" as *const u8 as *const libc::c_char,
                        type_0,
                        (*wl).idx,
                    );
                }
            }
        }
        c = (*c).entry.tqe_next;
    }
}
unsafe extern "C" fn run_static_initializers() {
    alerts_list = {
        let mut init = C2RustUnnamed_34 {
            tqh_first: 0 as *mut window,
            tqh_last: &mut alerts_list.tqh_first,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
