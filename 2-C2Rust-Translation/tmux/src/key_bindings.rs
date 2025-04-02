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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn cmd_list_free(_: *mut cmd_list);
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut cmd_find_state,
        _: *mut mouse_event,
        _: libc::c_int,
    ) -> *mut cmdq_item;
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut cmdq_item;
    fn cmdq_insert_after(_: *mut cmdq_item, _: *mut cmdq_item);
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn cmd_string_parse(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut cmd_list;
    static mut clients: clients;
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_INSERT_COLOR(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) {
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut gparent: *mut key_binding = 0 as *mut key_binding;
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
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
pub unsafe extern "C" fn key_bindings_RB_MINMAX(
    mut head: *mut key_bindings,
    mut val: libc::c_int,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
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
pub unsafe extern "C" fn key_bindings_RB_REMOVE_COLOR(
    mut head: *mut key_bindings,
    mut parent: *mut key_binding,
    mut elm: *mut key_binding,
) {
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
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
                    let mut oleft: *mut key_binding = 0 as *mut key_binding;
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
                    let mut oright: *mut key_binding = 0 as *mut key_binding;
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
pub unsafe extern "C" fn key_bindings_RB_REMOVE(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut current_block: u64;
    let mut child: *mut key_binding = 0 as *mut key_binding;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut old: *mut key_binding = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut key_binding = 0 as *mut key_binding;
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
        current_block = 11026823583092932056;
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
        key_bindings_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_INSERT(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = key_bindings_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut key_binding;
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
    key_bindings_RB_INSERT_COLOR(head, elm);
    return 0 as *mut key_binding;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_FIND(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = key_bindings_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut key_binding;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_RB_NFIND(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut res: *mut key_binding = 0 as *mut key_binding;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = key_bindings_cmp(elm, tmp);
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
pub unsafe extern "C" fn key_bindings_RB_NEXT(
    mut elm: *mut key_binding,
) -> *mut key_binding {
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
pub unsafe extern "C" fn key_bindings_RB_PREV(
    mut elm: *mut key_binding,
) -> *mut key_binding {
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
pub unsafe extern "C" fn key_tables_RB_INSERT_COLOR(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) {
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut gparent: *mut key_table = 0 as *mut key_table;
    let mut tmp: *mut key_table = 0 as *mut key_table;
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
pub unsafe extern "C" fn key_tables_RB_INSERT(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut tmp: *mut key_table = 0 as *mut key_table;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = key_table_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut key_table;
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
    key_tables_RB_INSERT_COLOR(head, elm);
    return 0 as *mut key_table;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_REMOVE_COLOR(
    mut head: *mut key_tables,
    mut parent: *mut key_table,
    mut elm: *mut key_table,
) {
    let mut tmp: *mut key_table = 0 as *mut key_table;
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
                    let mut oleft: *mut key_table = 0 as *mut key_table;
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
                    let mut oright: *mut key_table = 0 as *mut key_table;
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
pub unsafe extern "C" fn key_tables_RB_REMOVE(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut current_block: u64;
    let mut child: *mut key_table = 0 as *mut key_table;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut old: *mut key_table = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut key_table = 0 as *mut key_table;
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
        current_block = 11291615601710014133;
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
        key_tables_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_NFIND(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut res: *mut key_table = 0 as *mut key_table;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = key_table_cmp(elm, tmp);
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
pub unsafe extern "C" fn key_tables_RB_PREV(mut elm: *mut key_table) -> *mut key_table {
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
pub unsafe extern "C" fn key_tables_RB_FIND(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = key_table_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut key_table;
}
#[no_mangle]
pub unsafe extern "C" fn key_tables_RB_NEXT(mut elm: *mut key_table) -> *mut key_table {
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
pub unsafe extern "C" fn key_tables_RB_MINMAX(
    mut head: *mut key_tables,
    mut val: libc::c_int,
) -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut parent: *mut key_table = 0 as *mut key_table;
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
pub static mut key_tables: key_tables = {
    let mut init = key_tables {
        rbh_root: 0 as *const key_table as *mut key_table,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn key_table_cmp(
    mut e1: *mut key_table,
    mut e2: *mut key_table,
) -> libc::c_int {
    return strcmp((*e1).name, (*e2).name);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_cmp(
    mut bd1: *mut key_binding,
    mut bd2: *mut key_binding,
) -> libc::c_int {
    if (*bd1).key < (*bd2).key {
        return -(1 as libc::c_int);
    }
    if (*bd1).key > (*bd2).key {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_get_table(
    mut name: *const libc::c_char,
    mut create: libc::c_int,
) -> *mut key_table {
    let mut table_find: key_table = key_table {
        name: 0 as *const libc::c_char,
        key_bindings: key_bindings {
            rbh_root: 0 as *mut key_binding,
        },
        references: 0,
        entry: C2RustUnnamed_26 {
            rbe_left: 0 as *mut key_table,
            rbe_right: 0 as *mut key_table,
            rbe_parent: 0 as *mut key_table,
            rbe_color: 0,
        },
    };
    let mut table: *mut key_table = 0 as *mut key_table;
    table_find.name = name;
    table = key_tables_RB_FIND(&mut key_tables, &mut table_find);
    if !table.is_null() || create == 0 {
        return table;
    }
    table = xmalloc(::core::mem::size_of::<key_table>() as libc::c_ulong)
        as *mut key_table;
    (*table).name = xstrdup(name);
    (*table).key_bindings.rbh_root = 0 as *mut key_binding;
    (*table).references = 1 as libc::c_int as u_int;
    key_tables_RB_INSERT(&mut key_tables, table);
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_unref_table(mut table: *mut key_table) {
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut bd1: *mut key_binding = 0 as *mut key_binding;
    (*table).references = ((*table).references).wrapping_sub(1);
    if (*table).references != 0 as libc::c_int as u_int {
        return;
    }
    bd = key_bindings_RB_MINMAX(&mut (*table).key_bindings, -(1 as libc::c_int));
    while !bd.is_null()
        && {
            bd1 = key_bindings_RB_NEXT(bd);
            1 as libc::c_int != 0
        }
    {
        key_bindings_RB_REMOVE(&mut (*table).key_bindings, bd);
        cmd_list_free((*bd).cmdlist);
        free(bd as *mut libc::c_void);
        bd = bd1;
    }
    free((*table).name as *mut libc::c_void);
    free(table as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_add(
    mut name: *const libc::c_char,
    mut key: key_code,
    mut repeat: libc::c_int,
    mut cmdlist: *mut cmd_list,
) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd_find: key_binding = key_binding {
        key: 0,
        cmdlist: 0 as *mut cmd_list,
        flags: 0,
        entry: C2RustUnnamed_27 {
            rbe_left: 0 as *mut key_binding,
            rbe_right: 0 as *mut key_binding,
            rbe_parent: 0 as *mut key_binding,
            rbe_color: 0,
        },
    };
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 1 as libc::c_int);
    bd_find.key = key & !(0x1000000000000 as libc::c_ulonglong);
    bd = key_bindings_RB_FIND(&mut (*table).key_bindings, &mut bd_find);
    if !bd.is_null() {
        key_bindings_RB_REMOVE(&mut (*table).key_bindings, bd);
        cmd_list_free((*bd).cmdlist);
        free(bd as *mut libc::c_void);
    }
    bd = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<key_binding>() as libc::c_ulong,
    ) as *mut key_binding;
    (*bd).key = key;
    key_bindings_RB_INSERT(&mut (*table).key_bindings, bd);
    if repeat != 0 {
        (*bd).flags |= 0x1 as libc::c_int;
    }
    (*bd).cmdlist = cmdlist;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_remove(
    mut name: *const libc::c_char,
    mut key: key_code,
) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd_find: key_binding = key_binding {
        key: 0,
        cmdlist: 0 as *mut cmd_list,
        flags: 0,
        entry: C2RustUnnamed_27 {
            rbe_left: 0 as *mut key_binding,
            rbe_right: 0 as *mut key_binding,
            rbe_parent: 0 as *mut key_binding,
            rbe_color: 0,
        },
    };
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 0 as libc::c_int);
    if table.is_null() {
        return;
    }
    bd_find.key = key & !(0x1000000000000 as libc::c_ulonglong);
    bd = key_bindings_RB_FIND(&mut (*table).key_bindings, &mut bd_find);
    if bd.is_null() {
        return;
    }
    key_bindings_RB_REMOVE(&mut (*table).key_bindings, bd);
    cmd_list_free((*bd).cmdlist);
    free(bd as *mut libc::c_void);
    if ((*table).key_bindings.rbh_root).is_null() {
        key_tables_RB_REMOVE(&mut key_tables, table);
        key_bindings_unref_table(table);
    }
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_remove_table(mut name: *const libc::c_char) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut c: *mut client = 0 as *mut client;
    table = key_bindings_get_table(name, 0 as libc::c_int);
    if !table.is_null() {
        key_tables_RB_REMOVE(&mut key_tables, table);
        key_bindings_unref_table(table);
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if (*c).keytable == table {
            server_client_set_key_table(c, 0 as *const libc::c_char);
        }
        c = (*c).entry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_init() {
    static mut defaults: [*const libc::c_char; 227] = [
        b"bind C-b send-prefix\0" as *const u8 as *const libc::c_char,
        b"bind C-o rotate-window\0" as *const u8 as *const libc::c_char,
        b"bind C-z suspend-client\0" as *const u8 as *const libc::c_char,
        b"bind Space next-layout\0" as *const u8 as *const libc::c_char,
        b"bind ! break-pane\0" as *const u8 as *const libc::c_char,
        b"bind '\"' split-window\0" as *const u8 as *const libc::c_char,
        b"bind '#' list-buffers\0" as *const u8 as *const libc::c_char,
        b"bind '$' command-prompt -I'#S' \"rename-session -- '%%'\"\0" as *const u8
            as *const libc::c_char,
        b"bind % split-window -h\0" as *const u8 as *const libc::c_char,
        b"bind & confirm-before -p\"kill-window #W? (y/n)\" kill-window\0" as *const u8
            as *const libc::c_char,
        b"bind \"'\" command-prompt -pindex \"select-window -t ':%%'\"\0" as *const u8
            as *const libc::c_char,
        b"bind ( switch-client -p\0" as *const u8 as *const libc::c_char,
        b"bind ) switch-client -n\0" as *const u8 as *const libc::c_char,
        b"bind , command-prompt -I'#W' \"rename-window -- '%%'\"\0" as *const u8
            as *const libc::c_char,
        b"bind - delete-buffer\0" as *const u8 as *const libc::c_char,
        b"bind . command-prompt \"move-window -t '%%'\"\0" as *const u8
            as *const libc::c_char,
        b"bind 0 select-window -t:=0\0" as *const u8 as *const libc::c_char,
        b"bind 1 select-window -t:=1\0" as *const u8 as *const libc::c_char,
        b"bind 2 select-window -t:=2\0" as *const u8 as *const libc::c_char,
        b"bind 3 select-window -t:=3\0" as *const u8 as *const libc::c_char,
        b"bind 4 select-window -t:=4\0" as *const u8 as *const libc::c_char,
        b"bind 5 select-window -t:=5\0" as *const u8 as *const libc::c_char,
        b"bind 6 select-window -t:=6\0" as *const u8 as *const libc::c_char,
        b"bind 7 select-window -t:=7\0" as *const u8 as *const libc::c_char,
        b"bind 8 select-window -t:=8\0" as *const u8 as *const libc::c_char,
        b"bind 9 select-window -t:=9\0" as *const u8 as *const libc::c_char,
        b"bind : command-prompt\0" as *const u8 as *const libc::c_char,
        b"bind \\; last-pane\0" as *const u8 as *const libc::c_char,
        b"bind = choose-buffer -Z\0" as *const u8 as *const libc::c_char,
        b"bind ? list-keys\0" as *const u8 as *const libc::c_char,
        b"bind D choose-client -Z\0" as *const u8 as *const libc::c_char,
        b"bind E select-layout -E\0" as *const u8 as *const libc::c_char,
        b"bind L switch-client -l\0" as *const u8 as *const libc::c_char,
        b"bind M select-pane -M\0" as *const u8 as *const libc::c_char,
        b"bind [ copy-mode\0" as *const u8 as *const libc::c_char,
        b"bind ] paste-buffer\0" as *const u8 as *const libc::c_char,
        b"bind c new-window\0" as *const u8 as *const libc::c_char,
        b"bind d detach-client\0" as *const u8 as *const libc::c_char,
        b"bind f command-prompt \"find-window -- '%%'\"\0" as *const u8
            as *const libc::c_char,
        b"bind i display-message\0" as *const u8 as *const libc::c_char,
        b"bind l last-window\0" as *const u8 as *const libc::c_char,
        b"bind m select-pane -m\0" as *const u8 as *const libc::c_char,
        b"bind n next-window\0" as *const u8 as *const libc::c_char,
        b"bind o select-pane -t:.+\0" as *const u8 as *const libc::c_char,
        b"bind p previous-window\0" as *const u8 as *const libc::c_char,
        b"bind q display-panes\0" as *const u8 as *const libc::c_char,
        b"bind r refresh-client\0" as *const u8 as *const libc::c_char,
        b"bind s choose-tree -Zs\0" as *const u8 as *const libc::c_char,
        b"bind t clock-mode\0" as *const u8 as *const libc::c_char,
        b"bind w choose-tree -Zw\0" as *const u8 as *const libc::c_char,
        b"bind x confirm-before -p\"kill-pane #P? (y/n)\" kill-pane\0" as *const u8
            as *const libc::c_char,
        b"bind z resize-pane -Z\0" as *const u8 as *const libc::c_char,
        b"bind { swap-pane -U\0" as *const u8 as *const libc::c_char,
        b"bind } swap-pane -D\0" as *const u8 as *const libc::c_char,
        b"bind '~' show-messages\0" as *const u8 as *const libc::c_char,
        b"bind PPage copy-mode -u\0" as *const u8 as *const libc::c_char,
        b"bind -r Up select-pane -U\0" as *const u8 as *const libc::c_char,
        b"bind -r Down select-pane -D\0" as *const u8 as *const libc::c_char,
        b"bind -r Left select-pane -L\0" as *const u8 as *const libc::c_char,
        b"bind -r Right select-pane -R\0" as *const u8 as *const libc::c_char,
        b"bind M-1 select-layout even-horizontal\0" as *const u8 as *const libc::c_char,
        b"bind M-2 select-layout even-vertical\0" as *const u8 as *const libc::c_char,
        b"bind M-3 select-layout main-horizontal\0" as *const u8 as *const libc::c_char,
        b"bind M-4 select-layout main-vertical\0" as *const u8 as *const libc::c_char,
        b"bind M-5 select-layout tiled\0" as *const u8 as *const libc::c_char,
        b"bind M-n next-window -a\0" as *const u8 as *const libc::c_char,
        b"bind M-o rotate-window -D\0" as *const u8 as *const libc::c_char,
        b"bind M-p previous-window -a\0" as *const u8 as *const libc::c_char,
        b"bind -r M-Up resize-pane -U 5\0" as *const u8 as *const libc::c_char,
        b"bind -r M-Down resize-pane -D 5\0" as *const u8 as *const libc::c_char,
        b"bind -r M-Left resize-pane -L 5\0" as *const u8 as *const libc::c_char,
        b"bind -r M-Right resize-pane -R 5\0" as *const u8 as *const libc::c_char,
        b"bind -r C-Up resize-pane -U\0" as *const u8 as *const libc::c_char,
        b"bind -r C-Down resize-pane -D\0" as *const u8 as *const libc::c_char,
        b"bind -r C-Left resize-pane -L\0" as *const u8 as *const libc::c_char,
        b"bind -r C-Right resize-pane -R\0" as *const u8 as *const libc::c_char,
        b"bind -n MouseDown1Pane select-pane -t=\\; send-keys -M\0" as *const u8
            as *const libc::c_char,
        b"bind -n MouseDrag1Border resize-pane -M\0" as *const u8 as *const libc::c_char,
        b"bind -n MouseDown1Status select-window -t=\0" as *const u8
            as *const libc::c_char,
        b"bind -n WheelDownStatus next-window\0" as *const u8 as *const libc::c_char,
        b"bind -n WheelUpStatus previous-window\0" as *const u8 as *const libc::c_char,
        b"bind -n MouseDrag1Pane if -Ft= '#{mouse_any_flag}' 'if -Ft= \"#{pane_in_mode}\" \"copy-mode -M\" \"send-keys -M\"' 'copy-mode -M'\0"
            as *const u8 as *const libc::c_char,
        b"bind -n MouseDown3Pane if-shell -Ft= '#{mouse_any_flag}' 'select-pane -t=; send-keys -M' 'select-pane -mt='\0"
            as *const u8 as *const libc::c_char,
        b"bind -n WheelUpPane if-shell -Ft= '#{mouse_any_flag}' 'send-keys -M' 'if -Ft= \"#{pane_in_mode}\" \"send-keys -M\" \"copy-mode -et=\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-Space send -X begin-selection\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-a send -X start-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-c send -X cancel\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-e send -X end-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-f send -X cursor-right\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-b send -X cursor-left\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-g send -X clear-selection\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-k send -X copy-end-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-n send -X cursor-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-p send -X cursor-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-r command-prompt -ip'(search up)' -I'#{pane_search_string}' 'send -X search-backward-incremental \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-s command-prompt -ip'(search down)' -I'#{pane_search_string}' 'send -X search-forward-incremental \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-v send -X page-down\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-w send -X copy-selection-and-cancel\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode Escape send -X cancel\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode Space send -X page-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode , send -X jump-reverse\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode \\; send -X jump-again\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode F command-prompt -1p'(jump backward)' 'send -X jump-backward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode N send -X search-reverse\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode R send -X rectangle-toggle\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode T command-prompt -1p'(jump to backward)' 'send -X jump-to-backward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode f command-prompt -1p'(jump forward)' 'send -X jump-forward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode g command-prompt -p'(goto line)' 'send -X goto-line \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode n send -X search-again\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode q send -X cancel\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode t command-prompt -1p'(jump to forward)' 'send -X jump-to-forward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode Home send -X start-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode End send -X end-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode MouseDown1Pane select-pane\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode MouseDrag1Pane select-pane\\; send -X begin-selection\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode MouseDragEnd1Pane send -X copy-selection-and-cancel\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode WheelUpPane select-pane\\; send -N5 -X scroll-up\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode WheelDownPane select-pane\\; send -N5 -X scroll-down\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode DoubleClick1Pane select-pane\\; send -X select-word\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode TripleClick1Pane select-pane\\; send -X select-line\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode NPage send -X page-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode PPage send -X page-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode Up send -X cursor-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode Down send -X cursor-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode Left send -X cursor-left\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode Right send -X cursor-right\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-1 command-prompt -Np'(repeat)' -I1 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-2 command-prompt -Np'(repeat)' -I2 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-3 command-prompt -Np'(repeat)' -I3 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-4 command-prompt -Np'(repeat)' -I4 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-5 command-prompt -Np'(repeat)' -I5 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-6 command-prompt -Np'(repeat)' -I6 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-7 command-prompt -Np'(repeat)' -I7 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-8 command-prompt -Np'(repeat)' -I8 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-9 command-prompt -Np'(repeat)' -I9 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-< send -X history-top\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-> send -X history-bottom\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-R send -X top-line\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-b send -X previous-word\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-f send -X next-word-end\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-m send -X back-to-indentation\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-r send -X middle-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-v send -X page-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode M-w send -X copy-selection-and-cancel\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-{ send -X previous-paragraph\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-} send -X next-paragraph\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-Up send -X halfpage-up\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode M-Down send -X halfpage-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode C-Up send -X scroll-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode C-Down send -X scroll-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-c send -X cancel\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-d send -X halfpage-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-e send -X scroll-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-b send -X page-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-f send -X page-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-h send -X cursor-left\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-j send -X copy-selection-and-cancel\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Enter send -X copy-selection-and-cancel\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-u send -X halfpage-up\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-v send -X rectangle-toggle\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-y send -X scroll-up\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Escape send -X clear-selection\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Space send -X begin-selection\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi '$' send -X end-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi , send -X jump-reverse\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi / command-prompt -p'(search down)' 'send -X search-forward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 0 send -X start-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi 1 command-prompt -Np'(repeat)' -I1 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 2 command-prompt -Np'(repeat)' -I2 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 3 command-prompt -Np'(repeat)' -I3 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 4 command-prompt -Np'(repeat)' -I4 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 5 command-prompt -Np'(repeat)' -I5 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 6 command-prompt -Np'(repeat)' -I6 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 7 command-prompt -Np'(repeat)' -I7 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 8 command-prompt -Np'(repeat)' -I8 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi 9 command-prompt -Np'(repeat)' -I9 'send -N \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi : command-prompt -p'(goto line)' 'send -X goto-line \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi \\; send -X jump-again\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi ? command-prompt -p'(search up)' 'send -X search-backward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi A send -X append-selection-and-cancel\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi B send -X previous-space\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi D send -X copy-end-of-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi E send -X next-space-end\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi F command-prompt -1p'(jump backward)' 'send -X jump-backward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi G send -X history-bottom\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi H send -X top-line\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi J send -X scroll-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi K send -X scroll-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi L send -X bottom-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi M send -X middle-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi N send -X search-reverse\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi T command-prompt -1p'(jump to backward)' 'send -X jump-to-backward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi V send -X select-line\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi W send -X next-space\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi ^ send -X back-to-indentation\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi b send -X previous-word\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi e send -X next-word-end\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi f command-prompt -1p'(jump forward)' 'send -X jump-forward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi g send -X history-top\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi h send -X cursor-left\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi j send -X cursor-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi k send -X cursor-up\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi l send -X cursor-right\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi n send -X search-again\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi o send -X other-end\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi q send -X cancel\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi t command-prompt -1p'(jump to forward)' 'send -X jump-to-forward \"%%%\"'\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi v send -X rectangle-toggle\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi w send -X next-word\0" as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi { send -X previous-paragraph\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi } send -X next-paragraph\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi MouseDown1Pane select-pane\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi MouseDrag1Pane select-pane\\; send -X begin-selection\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi MouseDragEnd1Pane send -X copy-selection-and-cancel\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi WheelUpPane select-pane\\; send -N5 -X scroll-up\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi WheelDownPane select-pane\\; send -N5 -X scroll-down\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi DoubleClick1Pane select-pane\\; send -X select-word\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi TripleClick1Pane select-pane\\; send -X select-line\0"
            as *const u8 as *const libc::c_char,
        b"bind -Tcopy-mode-vi BSpace send -X cursor-left\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi NPage send -X page-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi PPage send -X page-up\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Up send -X cursor-up\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Down send -X cursor-down\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Left send -X cursor-left\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi Right send -X cursor-right\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-Up send -X scroll-up\0" as *const u8
            as *const libc::c_char,
        b"bind -Tcopy-mode-vi C-Down send -X scroll-down\0" as *const u8
            as *const libc::c_char,
    ];
    let mut i: u_int = 0;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 227]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        cmdlist = cmd_string_parse(
            defaults[i as usize],
            b"<default>\0" as *const u8 as *const libc::c_char,
            i,
            &mut cause,
        );
        if cmdlist.is_null() {
            fatalx(
                b"bad default key: %s\0" as *const u8 as *const libc::c_char,
                defaults[i as usize],
            );
        }
        cmdq_append(
            0 as *mut client,
            cmdq_get_command(
                cmdlist,
                0 as *mut cmd_find_state,
                0 as *mut mouse_event,
                0 as libc::c_int,
            ),
        );
        cmd_list_free(cmdlist);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn key_bindings_read_only(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    cmdq_error(item, b"client is read-only\0" as *const u8 as *const libc::c_char);
    return CMD_RETURN_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_dispatch(
    mut bd: *mut key_binding,
    mut item: *mut cmdq_item,
    mut c: *mut client,
    mut m: *mut mouse_event,
    mut fs: *mut cmd_find_state,
) {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut readonly: libc::c_int = 0;
    readonly = 1 as libc::c_int;
    cmd = (*(*bd).cmdlist).list.tqh_first;
    while !cmd.is_null() {
        if (*(*cmd).entry).flags & 0x2 as libc::c_int == 0 {
            readonly = 0 as libc::c_int;
        }
        cmd = (*cmd).qentry.tqe_next;
    }
    if readonly == 0 && (*c).flags & 0x800 as libc::c_int != 0 {
        new_item = cmdq_get_callback1(
            b"key_bindings_read_only\0" as *const u8 as *const libc::c_char,
            Some(
                key_bindings_read_only
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            0 as *mut libc::c_void,
        );
    } else {
        new_item = cmdq_get_command((*bd).cmdlist, fs, m, 0 as libc::c_int);
        if (*bd).flags & 0x1 as libc::c_int != 0 {
            (*(*new_item).shared).flags |= 0x1 as libc::c_int;
        }
    }
    if !item.is_null() {
        cmdq_insert_after(item, new_item);
    } else {
        cmdq_append(c, new_item);
    };
}
