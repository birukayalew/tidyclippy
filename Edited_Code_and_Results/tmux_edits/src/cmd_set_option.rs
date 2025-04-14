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
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    static mut global_s_options: *mut options;
    static mut global_w_options: *mut options;
    fn format_single(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    fn options_empty(
        _: *mut options,
        _: *const options_table_entry,
    ) -> *mut options_entry;
    fn options_default(
        _: *mut options,
        _: *const options_table_entry,
    ) -> *mut options_entry;
    fn options_table_entry(_: *mut options_entry) -> *const options_table_entry;
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_get(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_remove(_: *mut options_entry);
    fn options_array_clear(_: *mut options_entry);
    fn options_array_get(_: *mut options_entry, _: u_int) -> *const libc::c_char;
    fn options_array_set(
        _: *mut options_entry,
        _: u_int,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn options_array_size(_: *mut options_entry, _: *mut u_int) -> libc::c_int;
    fn options_array_assign(_: *mut options_entry, _: *const libc::c_char);
    fn options_match(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn options_set_string(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> *mut options_entry;
    fn options_set_number(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut options_entry;
    fn options_set_style(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> *mut options_entry;
    fn options_scope_from_flags(
        _: *mut args,
        _: libc::c_int,
        _: *mut cmd_find_state,
        _: *mut *mut options,
        _: *mut *mut libc::c_char,
    ) -> options_table_scope;
    fn options_style_update_new(_: *mut options, _: *mut options_entry);
    fn options_style_update_old(_: *mut options, _: *mut options_entry);
    fn tty_keys_build(_: *mut tty);
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    fn cmd_find_client(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut client;
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn key_string_lookup_string(_: *const libc::c_char) -> key_code;
    fn alerts_reset_all();
    static mut clients: clients;
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
    fn server_redraw_client(_: *mut client);
    fn status_timer_start_all();
    fn status_update_saved(s: *mut session);
    fn recalculate_sizes();
    fn colour_fromstring(s: *const libc::c_char) -> libc::c_int;
    fn attributes_fromstring(_: *const libc::c_char) -> libc::c_int;
    static mut windows: windows;
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int);
    static mut sessions: sessions;
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
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
pub struct windows {
    pub rbh_root: *mut window,
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
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_ARRAY: options_table_type = 8;
pub const OPTIONS_TABLE_STYLE: options_table_type = 7;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 6;
pub const OPTIONS_TABLE_FLAG: options_table_type = 5;
pub const OPTIONS_TABLE_ATTRIBUTES: options_table_type = 4;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: options_table_scope,
    pub minimum: u_int,
    pub maximum: u_int,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub separator: *const libc::c_char,
    pub style: *const libc::c_char,
}
#[no_mangle]
pub static mut cmd_set_option_entry: cmd_entry = unsafe {
    {
        let mut init = cmd_entry {
            name: b"set-option\0" as *const u8 as *const libc::c_char,
            alias: b"set\0" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_31 {
                    template: b"aFgoqst:uw\0" as *const u8 as *const libc::c_char,
                    lower: 1 as libc::c_int,
                    upper: 2 as libc::c_int,
                };
                init
            },
            usage: b"[-aFgosquw] [-t target-window] option [value]\0" as *const u8
                as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: {
                let mut init = cmd_entry_flag {
                    flag: 't' as i32 as libc::c_char,
                    type_0: CMD_FIND_WINDOW,
                    flags: 0x40 as libc::c_int,
                };
                init
            },
            flags: 0x4 as libc::c_int,
            exec: Some(
                cmd_set_option_exec
                    as unsafe extern "C" fn(*mut cmd, *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut cmd_set_window_option_entry: cmd_entry = unsafe {
    {
        let mut init = cmd_entry {
            name: b"set-window-option\0" as *const u8 as *const libc::c_char,
            alias: b"setw\0" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_31 {
                    template: b"aFgoqt:u\0" as *const u8 as *const libc::c_char,
                    lower: 1 as libc::c_int,
                    upper: 2 as libc::c_int,
                };
                init
            },
            usage: b"[-aFgoqu] [-t target-window] option [value]\0" as *const u8
                as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: {
                let mut init = cmd_entry_flag {
                    flag: 't' as i32 as libc::c_char,
                    type_0: CMD_FIND_WINDOW,
                    flags: 0x40 as libc::c_int,
                };
                init
            },
            flags: 0x4 as libc::c_int,
            exec: Some(
                cmd_set_option_exec
                    as unsafe extern "C" fn(*mut cmd, *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
unsafe extern "C" fn cmd_set_option_exec(
    mut self_0: *mut cmd,
    mut item: *mut cmdq_item,
) -> cmd_retval {
    let mut current_block: u64;
    let mut args: *mut args = (*self_0).args;
    let mut append: libc::c_int = args_has(args, 'a' as i32 as u_char);
    let mut fs: *mut cmd_find_state = &mut (*item).target;
    let mut c: *mut client = 0 as *mut client;
    let mut loop_0: *mut client = 0 as *mut client;
    let mut s: *mut session = (*fs).s;
    let mut wl: *mut winlink = (*fs).wl;
    let mut w: *mut window = 0 as *mut window;
    let mut scope: options_table_scope = OPTIONS_TABLE_NONE;
    let mut oo: *mut options = 0 as *mut options;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argument: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *const libc::c_char = 0 as *const libc::c_char;
    let mut window: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut already: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut ambiguous: libc::c_int = 0;
    c = cmd_find_client(item, 0 as *const libc::c_char, 1 as libc::c_int);
    argument = format_single(
        item,
        *((*args).argv).offset(0 as libc::c_int as isize),
        c,
        s,
        wl,
        0 as *mut window_pane,
    );
    name = options_match(argument, &mut idx, &mut ambiguous);
    if name.is_null() {
        if args_has(args, 'q' as i32 as u_char) != 0 {
            current_block = 13842905646530631182;
        } else {
            if ambiguous != 0 {
                cmdq_error(
                    item,
                    b"ambiguous option: %s\0" as *const u8 as *const libc::c_char,
                    argument,
                );
            } else {
                cmdq_error(
                    item,
                    b"invalid option: %s\0" as *const u8 as *const libc::c_char,
                    argument,
                );
            }
            current_block = 5525421777548740323;
        }
    } else {
        if (*args).argc < 2 as libc::c_int {
            value = 0 as *mut libc::c_char;
        } else if args_has(args, 'F' as i32 as u_char) != 0 {
            value = format_single(
                item,
                *((*args).argv).offset(1 as libc::c_int as isize),
                c,
                s,
                wl,
                0 as *mut window_pane,
            );
        } else {
            value = xstrdup(*((*args).argv).offset(1 as libc::c_int as isize));
        }
        if *name as libc::c_int == '@' as i32 {
            window = ((*self_0).entry
                == &cmd_set_window_option_entry as *const cmd_entry) as libc::c_int;
            scope = options_scope_from_flags(args, window, fs, &mut oo, &mut cause);
        } else if !(options_get_only(global_options, name)).is_null() {
            scope = OPTIONS_TABLE_SERVER;
        } else if !(options_get_only(global_s_options, name)).is_null() {
            scope = OPTIONS_TABLE_SESSION;
        } else if !(options_get_only(global_w_options, name)).is_null() {
            scope = OPTIONS_TABLE_WINDOW;
        } else {
            scope = OPTIONS_TABLE_NONE;
            xasprintf(
                &mut cause as *mut *mut libc::c_char,
                b"unknown option: %s\0" as *const u8 as *const libc::c_char,
                argument,
            );
        }
        if scope as libc::c_uint == OPTIONS_TABLE_NONE as libc::c_int as libc::c_uint {
            if args_has(args, 'q' as i32 as u_char) != 0 {
                current_block = 13842905646530631182;
            } else {
                cmdq_error(item, b"%s\0" as *const u8 as *const libc::c_char, cause);
                free(cause as *mut libc::c_void);
                current_block = 5525421777548740323;
            }
        } else {
            if scope as libc::c_uint
                == OPTIONS_TABLE_SERVER as libc::c_int as libc::c_uint
            {
                oo = global_options;
                current_block = 16738040538446813684;
            } else if scope as libc::c_uint
                == OPTIONS_TABLE_SESSION as libc::c_int as libc::c_uint
            {
                if args_has((*self_0).args, 'g' as i32 as u_char) != 0 {
                    oo = global_s_options;
                    current_block = 16738040538446813684;
                } else if s.is_null() {
                    target = args_get(args, 't' as i32 as u_char);
                    if !target.is_null() {
                        cmdq_error(
                            item,
                            b"no such session: %s\0" as *const u8 as *const libc::c_char,
                            target,
                        );
                    } else {
                        cmdq_error(
                            item,
                            b"no current session\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 5525421777548740323;
                } else {
                    oo = (*s).options;
                    current_block = 16738040538446813684;
                }
            } else if scope as libc::c_uint
                == OPTIONS_TABLE_WINDOW as libc::c_int as libc::c_uint
            {
                if args_has((*self_0).args, 'g' as i32 as u_char) != 0 {
                    oo = global_w_options;
                    current_block = 16738040538446813684;
                } else if wl.is_null() {
                    target = args_get(args, 't' as i32 as u_char);
                    if !target.is_null() {
                        cmdq_error(
                            item,
                            b"no such window: %s\0" as *const u8 as *const libc::c_char,
                            target,
                        );
                    } else {
                        cmdq_error(
                            item,
                            b"no current window\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 5525421777548740323;
                } else {
                    oo = (*(*wl).window).options;
                    current_block = 16738040538446813684;
                }
            } else {
                current_block = 16738040538446813684;
            }
            match current_block {
                5525421777548740323 => {}
                _ => {
                    o = options_get_only(oo, name);
                    parent = options_get(oo, name);
                    if idx != -(1 as libc::c_int) {
                        if *name as libc::c_int == '@' as i32
                            || options_array_size(parent, 0 as *mut u_int)
                                == -(1 as libc::c_int)
                        {
                            cmdq_error(
                                item,
                                b"not an array: %s\0" as *const u8 as *const libc::c_char,
                                argument,
                            );
                            current_block = 5525421777548740323;
                        } else {
                            current_block = 2290177392965769716;
                        }
                    } else {
                        current_block = 2290177392965769716;
                    }
                    match current_block {
                        5525421777548740323 => {}
                        _ => {
                            if args_has(args, 'u' as i32 as u_char) == 0
                                && args_has(args, 'o' as i32 as u_char) != 0
                            {
                                if idx == -(1 as libc::c_int) {
                                    already = (o
                                        != 0 as *mut libc::c_void as *mut options_entry)
                                        as libc::c_int;
                                } else if o.is_null() {
                                    already = 0 as libc::c_int;
                                } else {
                                    already = (options_array_get(o, idx as u_int)
                                        != 0 as *mut libc::c_void as *const libc::c_char)
                                        as libc::c_int;
                                }
                                if already != 0 {
                                    if args_has(args, 'q' as i32 as u_char) != 0 {
                                        current_block = 13842905646530631182;
                                    } else {
                                        cmdq_error(
                                            item,
                                            b"already set: %s\0" as *const u8 as *const libc::c_char,
                                            argument,
                                        );
                                        current_block = 5525421777548740323;
                                    }
                                } else {
                                    current_block = 15594603006322722090;
                                }
                            } else {
                                current_block = 15594603006322722090;
                            }
                            match current_block {
                                13842905646530631182 => {}
                                5525421777548740323 => {}
                                _ => {
                                    if args_has(args, 'u' as i32 as u_char) != 0 {
                                        if o.is_null() {
                                            current_block = 13842905646530631182;
                                        } else {
                                            if idx == -(1 as libc::c_int) {
                                                if *name as libc::c_int == '@' as i32 {
                                                    options_remove(o);
                                                } else if oo == global_options || oo == global_s_options
                                                    || oo == global_w_options
                                                {
                                                    options_default(oo, options_table_entry(o));
                                                } else {
                                                    options_remove(o);
                                                }
                                            } else {
                                                options_array_set(
                                                    o,
                                                    idx as u_int,
                                                    0 as *const libc::c_char,
                                                    0 as libc::c_int,
                                                );
                                            }
                                            current_block = 12070711452894729854;
                                        }
                                    } else if *name as libc::c_int == '@' as i32 {
                                        if value.is_null() {
                                            cmdq_error(
                                                item,
                                                b"empty value\0" as *const u8 as *const libc::c_char,
                                            );
                                            current_block = 5525421777548740323;
                                        } else {
                                            options_set_string(
                                                oo,
                                                name,
                                                append,
                                                b"%s\0" as *const u8 as *const libc::c_char,
                                                value,
                                            );
                                            current_block = 12070711452894729854;
                                        }
                                    } else if idx == -(1 as libc::c_int)
                                        && options_array_size(parent, 0 as *mut u_int)
                                            == -(1 as libc::c_int)
                                    {
                                        error = cmd_set_option_set(self_0, item, oo, parent, value);
                                        if error != 0 as libc::c_int {
                                            current_block = 5525421777548740323;
                                        } else {
                                            current_block = 12070711452894729854;
                                        }
                                    } else if value.is_null() {
                                        cmdq_error(
                                            item,
                                            b"empty value\0" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 5525421777548740323;
                                    } else {
                                        if o.is_null() {
                                            o = options_empty(oo, options_table_entry(parent));
                                        }
                                        if idx == -(1 as libc::c_int) {
                                            if append == 0 {
                                                options_array_clear(o);
                                            }
                                            options_array_assign(o, value);
                                            current_block = 12070711452894729854;
                                        } else if options_array_set(o, idx as u_int, value, append)
                                            != 0 as libc::c_int
                                        {
                                            cmdq_error(
                                                item,
                                                b"invalid index: %s\0" as *const u8 as *const libc::c_char,
                                                argument,
                                            );
                                            current_block = 5525421777548740323;
                                        } else {
                                            current_block = 12070711452894729854;
                                        }
                                    }
                                    match current_block {
                                        5525421777548740323 => {}
                                        13842905646530631182 => {}
                                        _ => {
                                            if strcmp(
                                                name,
                                                b"automatic-rename\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                            {
                                                w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
                                                while !w.is_null() {
                                                    if !((*w).active).is_null() {
                                                        if options_get_number(
                                                            (*w).options,
                                                            b"automatic-rename\0" as *const u8 as *const libc::c_char,
                                                        ) != 0
                                                        {
                                                            (*(*w).active).flags |= 0x80 as libc::c_int;
                                                        }
                                                    }
                                                    w = windows_RB_NEXT(w);
                                                }
                                            }
                                            if strcmp(
                                                name,
                                                b"key-table\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                            {
                                                loop_0 = clients.tqh_first;
                                                while !loop_0.is_null() {
                                                    server_client_set_key_table(
                                                        loop_0,
                                                        0 as *const libc::c_char,
                                                    );
                                                    loop_0 = (*loop_0).entry.tqe_next;
                                                }
                                            }
                                            if strcmp(
                                                name,
                                                b"user-keys\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                            {
                                                loop_0 = clients.tqh_first;
                                                while !loop_0.is_null() {
                                                    if (*loop_0).tty.flags & 0x20 as libc::c_int != 0 {
                                                        tty_keys_build(&mut (*loop_0).tty);
                                                    }
                                                    loop_0 = (*loop_0).entry.tqe_next;
                                                }
                                            }
                                            if strcmp(
                                                name,
                                                b"status\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                                || strcmp(
                                                    name,
                                                    b"status-interval\0" as *const u8 as *const libc::c_char,
                                                ) == 0 as libc::c_int
                                            {
                                                status_timer_start_all();
                                            }
                                            if strcmp(
                                                name,
                                                b"monitor-silence\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                            {
                                                alerts_reset_all();
                                            }
                                            if strcmp(
                                                name,
                                                b"window-style\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                                || strcmp(
                                                    name,
                                                    b"window-active-style\0" as *const u8 as *const libc::c_char,
                                                ) == 0 as libc::c_int
                                            {
                                                w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
                                                while !w.is_null() {
                                                    (*w).flags |= 0x8000 as libc::c_int;
                                                    w = windows_RB_NEXT(w);
                                                }
                                            }
                                            if strcmp(
                                                name,
                                                b"pane-border-status\0" as *const u8 as *const libc::c_char,
                                            ) == 0 as libc::c_int
                                            {
                                                w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
                                                while !w.is_null() {
                                                    layout_fix_panes(w, (*w).sx, (*w).sy);
                                                    w = windows_RB_NEXT(w);
                                                }
                                            }
                                            s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
                                            while !s.is_null() {
                                                status_update_saved(s);
                                                s = sessions_RB_NEXT(s);
                                            }
                                            recalculate_sizes();
                                            loop_0 = clients.tqh_first;
                                            while !loop_0.is_null() {
                                                if !((*loop_0).session).is_null() {
                                                    server_redraw_client(loop_0);
                                                }
                                                loop_0 = (*loop_0).entry.tqe_next;
                                            }
                                            current_block = 13842905646530631182;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        5525421777548740323 => {
            free(argument as *mut libc::c_void);
            free(value as *mut libc::c_void);
            free(name as *mut libc::c_void);
            return CMD_RETURN_ERROR;
        }
        _ => {
            free(argument as *mut libc::c_void);
            free(value as *mut libc::c_void);
            free(name as *mut libc::c_void);
            return CMD_RETURN_NORMAL;
        }
    };
}
unsafe extern "C" fn cmd_set_option_set(
    mut self_0: *mut cmd,
    mut item: *mut cmdq_item,
    mut oo: *mut options,
    mut parent: *mut options_entry,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut args: *mut args = (*self_0).args;
    let mut append: libc::c_int = args_has(args, 'a' as i32 as u_char);
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut number: libc::c_longlong = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: key_code = 0;
    oe = options_table_entry(parent);
    if value.is_null()
        && (*oe).type_0 as libc::c_uint
            != OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint
        && (*oe).type_0 as libc::c_uint
            != OPTIONS_TABLE_CHOICE as libc::c_int as libc::c_uint
    {
        cmdq_error(item, b"empty value\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    match (*oe).type_0 as libc::c_uint {
        0 => {
            options_set_string(
                oo,
                (*oe).name,
                append,
                b"%s\0" as *const u8 as *const libc::c_char,
                value,
            );
            return 0 as libc::c_int;
        }
        1 => {
            number = strtonum(
                value,
                (*oe).minimum as libc::c_longlong,
                (*oe).maximum as libc::c_longlong,
                &mut errstr,
            );
            if !errstr.is_null() {
                cmdq_error(
                    item,
                    b"value is %s: %s\0" as *const u8 as *const libc::c_char,
                    errstr,
                    value,
                );
                return -(1 as libc::c_int);
            }
            options_set_number(oo, (*oe).name, number);
            return 0 as libc::c_int;
        }
        2 => {
            key = key_string_lookup_string(value);
            if key == 0xfffe00000000 as libc::c_ulonglong {
                cmdq_error(
                    item,
                    b"bad key: %s\0" as *const u8 as *const libc::c_char,
                    value,
                );
                return -(1 as libc::c_int);
            }
            options_set_number(oo, (*oe).name, key as libc::c_longlong);
            return 0 as libc::c_int;
        }
        3 => {
            number = colour_fromstring(value) as libc::c_longlong;
            if number == -(1 as libc::c_int) as libc::c_longlong {
                cmdq_error(
                    item,
                    b"bad colour: %s\0" as *const u8 as *const libc::c_char,
                    value,
                );
                return -(1 as libc::c_int);
            }
            o = options_set_number(oo, (*oe).name, number);
            options_style_update_new(oo, o);
            return 0 as libc::c_int;
        }
        4 => {
            number = attributes_fromstring(value) as libc::c_longlong;
            if number == -(1 as libc::c_int) as libc::c_longlong {
                cmdq_error(
                    item,
                    b"bad attributes: %s\0" as *const u8 as *const libc::c_char,
                    value,
                );
                return -(1 as libc::c_int);
            }
            o = options_set_number(oo, (*oe).name, number);
            options_style_update_new(oo, o);
            return 0 as libc::c_int;
        }
        5 => return cmd_set_option_flag(item, oe, oo, value),
        6 => return cmd_set_option_choice(item, oe, oo, value),
        7 => {
            o = options_set_style(oo, (*oe).name, append, value);
            if o.is_null() {
                cmdq_error(
                    item,
                    b"bad style: %s\0" as *const u8 as *const libc::c_char,
                    value,
                );
                return -(1 as libc::c_int);
            }
            options_style_update_old(oo, o);
            return 0 as libc::c_int;
        }
        8 | _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cmd_set_option_flag(
    mut item: *mut cmdq_item,
    mut oe: *const options_table_entry,
    mut oo: *mut options,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut flag: libc::c_int = 0;
    if value.is_null() || *value as libc::c_int == '\0' as i32 {
        flag = (options_get_number(oo, (*oe).name) == 0) as libc::c_int;
    } else if strcmp(value, b"1\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcasecmp(value, b"on\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcasecmp(value, b"yes\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        flag = 1 as libc::c_int;
    } else if strcmp(value, b"0\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcasecmp(value, b"off\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcasecmp(value, b"no\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        flag = 0 as libc::c_int;
    } else {
        cmdq_error(item, b"bad value: %s\0" as *const u8 as *const libc::c_char, value);
        return -(1 as libc::c_int);
    }
    options_set_number(oo, (*oe).name, flag as libc::c_longlong);
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_set_option_choice(
    mut item: *mut cmdq_item,
    mut oe: *const options_table_entry,
    mut oo: *mut options,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut cp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut choice: libc::c_int = -(1 as libc::c_int);
    if value.is_null() {
        choice = options_get_number(oo, (*oe).name) as libc::c_int;
        if choice < 2 as libc::c_int {
            choice = (choice == 0) as libc::c_int;
        }
    } else {
        n = 0 as libc::c_int;
        cp = (*oe).choices;
        while !(*cp).is_null() {
            if strcmp(*cp, value) == 0 as libc::c_int {
                choice = n;
            }
            n += 1;
            n;
            cp = cp.offset(1);
            cp;
        }
        if choice == -(1 as libc::c_int) {
            cmdq_error(
                item,
                b"unknown value: %s\0" as *const u8 as *const libc::c_char,
                value,
            );
            return -(1 as libc::c_int);
        }
    }
    options_set_number(oo, (*oe).name, choice as libc::c_longlong);
    return 0 as libc::c_int;
}
