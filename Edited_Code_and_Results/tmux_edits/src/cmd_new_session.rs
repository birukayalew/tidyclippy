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
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut global_s_options: *mut options;
    static mut global_environ: *mut environ;
    fn proc_send(
        _: *mut tmuxpeer,
        _: msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    static mut cfg_finished: libc::c_int;
    fn cfg_show_causes(_: *mut session);
    fn format_single(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    fn hooks_insert(
        _: *mut hooks,
        _: *mut cmdq_item,
        _: *mut cmd_find_state,
        _: *const libc::c_char,
        _: ...
    );
    fn notify_client(_: *const libc::c_char, _: *mut client);
    fn notify_session(_: *const libc::c_char, _: *mut session);
    fn options_get_string(
        _: *mut options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn options_set_number(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut options_entry;
    fn environ_create() -> *mut environ;
    fn environ_free(_: *mut environ);
    fn environ_find(_: *mut environ, _: *const libc::c_char) -> *mut environ_entry;
    fn environ_update(_: *mut options, _: *mut environ, _: *mut environ);
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session, _: libc::c_int);
    fn cmd_attach_session(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> cmd_retval;
    fn cmdq_print(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn server_update_socket();
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
    fn server_client_check_nested(_: *mut client) -> libc::c_int;
    fn server_client_open(_: *mut client, _: *mut *mut libc::c_char) -> libc::c_int;
    fn server_redraw_client(_: *mut client);
    fn status_timer_start(_: *mut client);
    fn recalculate_sizes();
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn window_set_name(_: *mut window, _: *const libc::c_char);
    fn session_find(_: *const libc::c_char) -> *mut session;
    fn session_create(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut environ,
        _: *mut termios,
        _: libc::c_int,
        _: u_int,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut session;
    fn session_check_name(_: *const libc::c_char) -> libc::c_int;
    fn session_update_activity(_: *mut session, _: *mut timeval);
    fn session_select(_: *mut session, _: libc::c_int) -> libc::c_int;
    fn session_group_contains(_: *mut session) -> *mut session_group;
    fn session_group_find(_: *const libc::c_char) -> *mut session_group;
    fn session_group_new(_: *const libc::c_char) -> *mut session_group;
    fn session_group_add(_: *mut session_group, _: *mut session);
    fn session_group_synchronize_to(_: *mut session);
    fn fatal(_: *const libc::c_char, _: ...) -> !;
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
pub type msgtype = libc::c_uint;
pub const MSG_EXEC: msgtype = 217;
pub const MSG_WAKEUP: msgtype = 216;
pub const MSG_UNLOCK: msgtype = 215;
pub const MSG_SUSPEND: msgtype = 214;
pub const MSG_STDOUT: msgtype = 213;
pub const MSG_STDIN: msgtype = 212;
pub const MSG_STDERR: msgtype = 211;
pub const MSG_SHUTDOWN: msgtype = 210;
pub const MSG_SHELL: msgtype = 209;
pub const MSG_RESIZE: msgtype = 208;
pub const MSG_READY: msgtype = 207;
pub const MSG_LOCK: msgtype = 206;
pub const MSG_EXITING: msgtype = 205;
pub const MSG_EXITED: msgtype = 204;
pub const MSG_EXIT: msgtype = 203;
pub const MSG_DETACHKILL: msgtype = 202;
pub const MSG_DETACH: msgtype = 201;
pub const MSG_COMMAND: msgtype = 200;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
pub const MSG_IDENTIFY_TERM: msgtype = 101;
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
pub const MSG_VERSION: msgtype = 12;
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_36,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[no_mangle]
pub static mut cmd_new_session_entry: cmd_entry = unsafe {
    {
        let mut init = cmd_entry {
            name: b"new-session\0" as *const u8 as *const libc::c_char,
            alias: b"new\0" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_31 {
                    template: b"Ac:dDEF:n:Ps:t:x:y:\0" as *const u8
                        as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: -(1 as libc::c_int),
                };
                init
            },
            usage: b"[-AdDEP] [-c start-directory] [-F format] [-n window-name] [-s session-name] [-t target-session] [-x width] [-y height] [command]\0"
                as *const u8 as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: {
                let mut init = cmd_entry_flag {
                    flag: 't' as i32 as libc::c_char,
                    type_0: CMD_FIND_SESSION,
                    flags: 0x40 as libc::c_int,
                };
                init
            },
            flags: 0x1 as libc::c_int,
            exec: Some(
                cmd_new_session_exec
                    as unsafe extern "C" fn(*mut cmd, *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut cmd_has_session_entry: cmd_entry = unsafe {
    {
        let mut init = cmd_entry {
            name: b"has-session\0" as *const u8 as *const libc::c_char,
            alias: b"has\0" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_31 {
                    template: b"t:\0" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 0 as libc::c_int,
                };
                init
            },
            usage: b"[-t target-session]\0" as *const u8 as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: {
                let mut init = cmd_entry_flag {
                    flag: 't' as i32 as libc::c_char,
                    type_0: CMD_FIND_SESSION,
                    flags: 0 as libc::c_int,
                };
                init
            },
            flags: 0 as libc::c_int,
            exec: Some(
                cmd_new_session_exec
                    as unsafe extern "C" fn(*mut cmd, *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
unsafe extern "C" fn cmd_new_session_exec(
    mut self_0: *mut cmd,
    mut item: *mut cmdq_item,
) -> cmd_retval {
    let mut current_block: u64;
    let mut args: *mut args = (*self_0).args;
    let mut c: *mut client = (*item).client;
    let mut s: *mut session = 0 as *mut session;
    let mut as_0: *mut session = 0 as *mut session;
    let mut groupwith: *mut session = 0 as *mut session;
    let mut w: *mut window = 0 as *mut window;
    let mut env: *mut environ = 0 as *mut environ;
    let mut tio: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut tiop: *mut termios = 0 as *mut termios;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut group: *const libc::c_char = 0 as *const libc::c_char;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut detached: libc::c_int = 0;
    let mut already_attached: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut is_control: libc::c_int = 0 as libc::c_int;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    if (*self_0).entry == &cmd_has_session_entry as *const cmd_entry {
        return CMD_RETURN_NORMAL;
    }
    if args_has(args, 't' as i32 as u_char) != 0
        && ((*args).argc != 0 as libc::c_int
            || args_has(args, 'n' as i32 as u_char) != 0)
    {
        cmdq_error(
            item,
            b"command or window name given with target\0" as *const u8
                as *const libc::c_char,
        );
        return CMD_RETURN_ERROR;
    }
    newname = 0 as *mut libc::c_char;
    if args_has(args, 's' as i32 as u_char) != 0 {
        newname = format_single(
            item,
            args_get(args, 's' as i32 as u_char),
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        );
        if session_check_name(newname) == 0 {
            cmdq_error(
                item,
                b"bad session name: %s\0" as *const u8 as *const libc::c_char,
                newname,
            );
            current_block = 3504328375851313951;
        } else {
            as_0 = session_find(newname);
            if !as_0.is_null() {
                if args_has(args, 'A' as i32 as u_char) != 0 {
                    retval = cmd_attach_session(
                        item,
                        newname,
                        args_has(args, 'D' as i32 as u_char),
                        0 as libc::c_int,
                        0 as *const libc::c_char,
                        args_has(args, 'E' as i32 as u_char),
                    );
                    free(newname as *mut libc::c_void);
                    return retval;
                }
                cmdq_error(
                    item,
                    b"duplicate session: %s\0" as *const u8 as *const libc::c_char,
                    newname,
                );
                current_block = 3504328375851313951;
            } else {
                current_block = 11584701595673473500;
            }
        }
    } else {
        current_block = 11584701595673473500;
    }
    match current_block {
        11584701595673473500 => {
            group = args_get(args, 't' as i32 as u_char);
            if !group.is_null() {
                groupwith = (*item).target.s;
                if groupwith.is_null() {
                    if session_check_name(group) == 0 {
                        cmdq_error(
                            item,
                            b"bad group name: %s\0" as *const u8 as *const libc::c_char,
                            group,
                        );
                        current_block = 3504328375851313951;
                    } else {
                        sg = session_group_find(group);
                        current_block = 14763689060501151050;
                    }
                } else {
                    sg = session_group_contains(groupwith);
                    current_block = 14763689060501151050;
                }
                match current_block {
                    3504328375851313951 => {}
                    _ => {
                        if !sg.is_null() {
                            prefix = (*sg).name;
                        } else if !groupwith.is_null() {
                            prefix = (*groupwith).name;
                        } else {
                            prefix = group;
                        }
                        current_block = 7333393191927787629;
                    }
                }
            } else {
                groupwith = 0 as *mut session;
                sg = 0 as *mut session_group;
                prefix = 0 as *const libc::c_char;
                current_block = 7333393191927787629;
            }
            match current_block {
                3504328375851313951 => {}
                _ => {
                    detached = args_has(args, 'd' as i32 as u_char);
                    if c.is_null() {
                        detached = 1 as libc::c_int;
                    } else if (*c).flags & 0x2000 as libc::c_int != 0 {
                        is_control = 1 as libc::c_int;
                    }
                    already_attached = 0 as libc::c_int;
                    if !c.is_null() && !((*c).session).is_null() {
                        already_attached = 1 as libc::c_int;
                    }
                    tmp = args_get(args, 'c' as i32 as u_char);
                    if !tmp.is_null() {
                        cwd = format_single(
                            item,
                            tmp,
                            c,
                            0 as *mut session,
                            0 as *mut winlink,
                            0 as *mut window_pane,
                        );
                    } else if !c.is_null() && ((*c).session).is_null()
                        && !((*c).cwd).is_null()
                    {
                        cwd = xstrdup((*c).cwd);
                    } else {
                        cwd = xstrdup(b".\0" as *const u8 as *const libc::c_char);
                    }
                    if detached == 0 && already_attached == 0
                        && (*c).tty.fd != -(1 as libc::c_int)
                    {
                        if server_client_check_nested((*item).client) != 0 {
                            cmdq_error(
                                item,
                                b"sessions should be nested with care, unset $TMUX to force\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            return CMD_RETURN_ERROR;
                        }
                        if tcgetattr((*c).tty.fd, &mut tio) != 0 as libc::c_int {
                            fatal(
                                b"tcgetattr failed\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        tiop = &mut tio;
                    } else {
                        tiop = 0 as *mut termios;
                    }
                    if detached == 0 && already_attached == 0 {
                        if server_client_open(c, &mut cause) != 0 as libc::c_int {
                            cmdq_error(
                                item,
                                b"open terminal failed: %s\0" as *const u8
                                    as *const libc::c_char,
                                cause,
                            );
                            free(cause as *mut libc::c_void);
                            current_block = 3504328375851313951;
                        } else {
                            current_block = 8545136480011357681;
                        }
                    } else {
                        current_block = 8545136480011357681;
                    }
                    match current_block {
                        3504328375851313951 => {}
                        _ => {
                            if detached == 0 {
                                sx = (*c).tty.sx;
                                sy = (*c).tty.sy;
                                if is_control == 0 && sy > 0 as libc::c_int as u_int
                                    && options_get_number(
                                        global_s_options,
                                        b"status\0" as *const u8 as *const libc::c_char,
                                    ) != 0
                                {
                                    sy = sy.wrapping_sub(1);
                                    sy;
                                }
                            } else {
                                sx = 80 as libc::c_int as u_int;
                                sy = 24 as libc::c_int as u_int;
                            }
                            if (is_control != 0 || detached != 0)
                                && args_has(args, 'x' as i32 as u_char) != 0
                            {
                                sx = strtonum(
                                    args_get(args, 'x' as i32 as u_char),
                                    1 as libc::c_int as libc::c_longlong,
                                    (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                        as libc::c_longlong,
                                    &mut errstr,
                                ) as u_int;
                                if !errstr.is_null() {
                                    cmdq_error(
                                        item,
                                        b"width %s\0" as *const u8 as *const libc::c_char,
                                        errstr,
                                    );
                                    current_block = 3504328375851313951;
                                } else {
                                    current_block = 2631791190359682872;
                                }
                            } else {
                                current_block = 2631791190359682872;
                            }
                            match current_block {
                                3504328375851313951 => {}
                                _ => {
                                    if (is_control != 0 || detached != 0)
                                        && args_has(args, 'y' as i32 as u_char) != 0
                                    {
                                        sy = strtonum(
                                            args_get(args, 'y' as i32 as u_char),
                                            1 as libc::c_int as libc::c_longlong,
                                            (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                                as libc::c_longlong,
                                            &mut errstr,
                                        ) as u_int;
                                        if !errstr.is_null() {
                                            cmdq_error(
                                                item,
                                                b"height %s\0" as *const u8 as *const libc::c_char,
                                                errstr,
                                            );
                                            current_block = 3504328375851313951;
                                        } else {
                                            current_block = 5684854171168229155;
                                        }
                                    } else {
                                        current_block = 5684854171168229155;
                                    }
                                    match current_block {
                                        3504328375851313951 => {}
                                        _ => {
                                            if sx == 0 as libc::c_int as u_int {
                                                sx = 1 as libc::c_int as u_int;
                                            }
                                            if sy == 0 as libc::c_int as u_int {
                                                sy = 1 as libc::c_int as u_int;
                                            }
                                            argc = -(1 as libc::c_int);
                                            argv = 0 as *mut *mut libc::c_char;
                                            if args_has(args, 't' as i32 as u_char) == 0
                                                && (*args).argc != 0 as libc::c_int
                                            {
                                                argc = (*args).argc;
                                                argv = (*args).argv;
                                            } else if sg.is_null() && groupwith.is_null() {
                                                cmd = options_get_string(
                                                    global_s_options,
                                                    b"default-command\0" as *const u8 as *const libc::c_char,
                                                );
                                                if !cmd.is_null() && *cmd as libc::c_int != '\0' as i32 {
                                                    argc = 1 as libc::c_int;
                                                    argv = &mut cmd as *mut *const libc::c_char
                                                        as *mut *mut libc::c_char;
                                                } else {
                                                    argc = 0 as libc::c_int;
                                                    argv = 0 as *mut *mut libc::c_char;
                                                }
                                            }
                                            path = 0 as *const libc::c_char;
                                            if !c.is_null() && ((*c).session).is_null() {
                                                envent = environ_find(
                                                    (*c).environ,
                                                    b"PATH\0" as *const u8 as *const libc::c_char,
                                                );
                                            } else {
                                                envent = environ_find(
                                                    global_environ,
                                                    b"PATH\0" as *const u8 as *const libc::c_char,
                                                );
                                            }
                                            if !envent.is_null() {
                                                path = (*envent).value;
                                            }
                                            env = environ_create();
                                            if !c.is_null() && args_has(args, 'E' as i32 as u_char) == 0
                                            {
                                                environ_update(global_s_options, (*c).environ, env);
                                            }
                                            idx = (-(1 as libc::c_int) as libc::c_longlong
                                                - options_get_number(
                                                    global_s_options,
                                                    b"base-index\0" as *const u8 as *const libc::c_char,
                                                )) as libc::c_int;
                                            s = session_create(
                                                prefix,
                                                newname,
                                                argc,
                                                argv,
                                                path,
                                                cwd,
                                                env,
                                                tiop,
                                                idx,
                                                sx,
                                                sy,
                                                &mut cause,
                                            );
                                            environ_free(env);
                                            if s.is_null() {
                                                cmdq_error(
                                                    item,
                                                    b"create session failed: %s\0" as *const u8
                                                        as *const libc::c_char,
                                                    cause,
                                                );
                                                free(cause as *mut libc::c_void);
                                            } else {
                                                if argc >= 0 as libc::c_int
                                                    && {
                                                        tmp = args_get(args, 'n' as i32 as u_char);
                                                        !tmp.is_null()
                                                    }
                                                {
                                                    cp = format_single(
                                                        item,
                                                        tmp,
                                                        c,
                                                        s,
                                                        0 as *mut winlink,
                                                        0 as *mut window_pane,
                                                    );
                                                    w = (*(*s).curw).window;
                                                    window_set_name(w, cp);
                                                    options_set_number(
                                                        (*w).options,
                                                        b"automatic-rename\0" as *const u8 as *const libc::c_char,
                                                        0 as libc::c_int as libc::c_longlong,
                                                    );
                                                    free(cp as *mut libc::c_void);
                                                }
                                                if !group.is_null() {
                                                    if sg.is_null() {
                                                        if !groupwith.is_null() {
                                                            sg = session_group_new((*groupwith).name);
                                                            session_group_add(sg, groupwith);
                                                        } else {
                                                            sg = session_group_new(group);
                                                        }
                                                    }
                                                    session_group_add(sg, s);
                                                    session_group_synchronize_to(s);
                                                    session_select(
                                                        s,
                                                        (*winlinks_RB_MINMAX(
                                                            &mut (*s).windows,
                                                            -(1 as libc::c_int),
                                                        ))
                                                            .idx,
                                                    );
                                                }
                                                notify_session(
                                                    b"session-created\0" as *const u8 as *const libc::c_char,
                                                    s,
                                                );
                                                if detached == 0 {
                                                    if already_attached == 0 {
                                                        if !(*c).flags & 0x2000 as libc::c_int != 0 {
                                                            proc_send(
                                                                (*c).peer,
                                                                MSG_READY,
                                                                -(1 as libc::c_int),
                                                                0 as *const libc::c_void,
                                                                0 as libc::c_int as size_t,
                                                            );
                                                        }
                                                    } else if !((*c).session).is_null() {
                                                        (*c).last_session = (*c).session;
                                                    }
                                                    (*c).session = s;
                                                    if !(*(*item).shared).flags & 0x1 as libc::c_int != 0 {
                                                        server_client_set_key_table(c, 0 as *const libc::c_char);
                                                    }
                                                    status_timer_start(c);
                                                    notify_client(
                                                        b"client-session-changed\0" as *const u8
                                                            as *const libc::c_char,
                                                        c,
                                                    );
                                                    session_update_activity(s, 0 as *mut timeval);
                                                    gettimeofday(
                                                        &mut (*s).last_attached_time,
                                                        0 as *mut libc::c_void,
                                                    );
                                                    server_redraw_client(c);
                                                }
                                                recalculate_sizes();
                                                server_update_socket();
                                                if cfg_finished != 0 {
                                                    cfg_show_causes(s);
                                                }
                                                if args_has(args, 'P' as i32 as u_char) != 0 {
                                                    template = args_get(args, 'F' as i32 as u_char);
                                                    if template.is_null() {
                                                        template = b"#{session_name}:\0" as *const u8
                                                            as *const libc::c_char;
                                                    }
                                                    cp = format_single(
                                                        item,
                                                        template,
                                                        c,
                                                        s,
                                                        0 as *mut winlink,
                                                        0 as *mut window_pane,
                                                    );
                                                    cmdq_print(
                                                        item,
                                                        b"%s\0" as *const u8 as *const libc::c_char,
                                                        cp,
                                                    );
                                                    free(cp as *mut libc::c_void);
                                                }
                                                if detached == 0 {
                                                    (*c).flags |= 0x80 as libc::c_int;
                                                    cmd_find_from_session(
                                                        &mut (*(*item).shared).current,
                                                        s,
                                                        0 as libc::c_int,
                                                    );
                                                }
                                                cmd_find_from_session(&mut fs, s, 0 as libc::c_int);
                                                hooks_insert(
                                                    (*s).hooks,
                                                    item,
                                                    &mut fs as *mut cmd_find_state,
                                                    b"after-new-session\0" as *const u8 as *const libc::c_char,
                                                );
                                                free(cwd as *mut libc::c_void);
                                                free(newname as *mut libc::c_void);
                                                return CMD_RETURN_NORMAL;
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
        _ => {}
    }
    free(cwd as *mut libc::c_void);
    free(newname as *mut libc::c_void);
    return CMD_RETURN_ERROR;
}
