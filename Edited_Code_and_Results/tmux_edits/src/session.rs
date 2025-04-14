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
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
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
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_hooks: *mut hooks;
    static mut global_s_options: *mut options;
    fn areshell(_: *const libc::c_char) -> libc::c_int;
    fn hooks_create(_: *mut hooks) -> *mut hooks;
    fn hooks_free(_: *mut hooks);
    fn notify_session(_: *const libc::c_char, _: *mut session);
    fn notify_session_window(_: *const libc::c_char, _: *mut session, _: *mut window);
    fn options_create(_: *mut options) -> *mut options;
    fn options_free(_: *mut options);
    fn options_get_string(
        _: *mut options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn environ_create() -> *mut environ;
    fn environ_free(_: *mut environ);
    fn environ_copy(_: *mut environ, _: *mut environ);
    fn environ_for_session(_: *mut session, _: libc::c_int) -> *mut environ;
    fn server_lock_session(_: *mut session);
    fn status_update_saved(s: *mut session);
    fn recalculate_sizes();
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window) -> *mut winlink;
    fn winlink_find_by_window_id(_: *mut winlinks, _: u_int) -> *mut winlink;
    fn winlink_add(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlink_set_window(_: *mut winlink, _: *mut window);
    fn winlink_remove(_: *mut winlinks, _: *mut winlink);
    fn winlink_next(_: *mut winlink) -> *mut winlink;
    fn winlink_previous(_: *mut winlink) -> *mut winlink;
    fn winlink_stack_push(_: *mut winlink_stack, _: *mut winlink);
    fn winlink_stack_remove(_: *mut winlink_stack, _: *mut winlink);
    fn window_update_activity(_: *mut window);
    fn window_create_spawn(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut environ,
        _: *mut termios,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut window;
    fn winlink_clear_flags(_: *mut winlink);
    fn log_debug(_: *const libc::c_char, _: ...);
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_35,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sessions {
    pub rbh_root: *mut session,
}
#[no_mangle]
pub static mut sessions: sessions = sessions {
    rbh_root: 0 as *const session as *mut session,
};
static mut next_session_id: u_int = 0;
#[no_mangle]
pub static mut session_groups: session_groups = session_groups {
    rbh_root: 0 as *const session_group as *mut session_group,
};
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_REMOVE_COLOR(
    mut head: *mut sessions,
    mut parent: *mut session,
    mut elm: *mut session,
) {
    let mut tmp: *mut session = 0 as *mut session;
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
                    let mut oleft: *mut session = 0 as *mut session;
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
                    let mut oright: *mut session = 0 as *mut session;
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
pub unsafe extern "C" fn sessions_RB_INSERT(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut tmp: *mut session = 0 as *mut session;
    let mut parent: *mut session = 0 as *mut session;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = session_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut session;
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
    sessions_RB_INSERT_COLOR(head, elm);
    return 0 as *mut session;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_INSERT_COLOR(
    mut head: *mut sessions,
    mut elm: *mut session,
) {
    let mut parent: *mut session = 0 as *mut session;
    let mut gparent: *mut session = 0 as *mut session;
    let mut tmp: *mut session = 0 as *mut session;
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
pub unsafe extern "C" fn sessions_RB_MINMAX(
    mut head: *mut sessions,
    mut val: libc::c_int,
) -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut parent: *mut session = 0 as *mut session;
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
pub unsafe extern "C" fn sessions_RB_REMOVE(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut current_block: u64;
    let mut child: *mut session = 0 as *mut session;
    let mut parent: *mut session = 0 as *mut session;
    let mut old: *mut session = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut session = 0 as *mut session;
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
        current_block = 4147148131057302291;
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
        sessions_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_PREV(mut elm: *mut session) -> *mut session {
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
pub unsafe extern "C" fn sessions_RB_NEXT(mut elm: *mut session) -> *mut session {
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
pub unsafe extern "C" fn sessions_RB_NFIND(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut res: *mut session = 0 as *mut session;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_cmp(elm, tmp);
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
pub unsafe extern "C" fn sessions_RB_FIND(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut session;
}
#[no_mangle]
pub unsafe extern "C" fn session_cmp(
    mut s1: *mut session,
    mut s2: *mut session,
) -> libc::c_int {
    return strcmp((*s1).name, (*s2).name);
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_NFIND(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut res: *mut session_group = 0 as *mut session_group;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_group_cmp(elm, tmp);
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
pub unsafe extern "C" fn session_groups_RB_REMOVE(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut current_block: u64;
    let mut child: *mut session_group = 0 as *mut session_group;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut old: *mut session_group = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut session_group = 0 as *mut session_group;
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
        current_block = 4471724981936197409;
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
        session_groups_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_FIND(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_group_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut session_group;
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_INSERT_COLOR(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) {
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut gparent: *mut session_group = 0 as *mut session_group;
    let mut tmp: *mut session_group = 0 as *mut session_group;
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
pub unsafe extern "C" fn session_groups_RB_REMOVE_COLOR(
    mut head: *mut session_groups,
    mut parent: *mut session_group,
    mut elm: *mut session_group,
) {
    let mut tmp: *mut session_group = 0 as *mut session_group;
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
                    let mut oleft: *mut session_group = 0 as *mut session_group;
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
                    let mut oright: *mut session_group = 0 as *mut session_group;
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
pub unsafe extern "C" fn session_groups_RB_NEXT(
    mut elm: *mut session_group,
) -> *mut session_group {
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
pub unsafe extern "C" fn session_groups_RB_INSERT(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut tmp: *mut session_group = 0 as *mut session_group;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = session_group_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut session_group;
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
    session_groups_RB_INSERT_COLOR(head, elm);
    return 0 as *mut session_group;
}
#[no_mangle]
pub unsafe extern "C" fn session_groups_RB_MINMAX(
    mut head: *mut session_groups,
    mut val: libc::c_int,
) -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut parent: *mut session_group = 0 as *mut session_group;
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
pub unsafe extern "C" fn session_groups_RB_PREV(
    mut elm: *mut session_group,
) -> *mut session_group {
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
pub unsafe extern "C" fn session_group_cmp(
    mut s1: *mut session_group,
    mut s2: *mut session_group,
) -> libc::c_int {
    return strcmp((*s1).name, (*s2).name);
}
#[no_mangle]
pub unsafe extern "C" fn session_alive(mut s: *mut session) -> libc::c_int {
    let mut s_loop: *mut session = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if s_loop == s {
            return 1 as libc::c_int;
        }
        s_loop = sessions_RB_NEXT(s_loop);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn session_find(mut name: *const libc::c_char) -> *mut session {
    let mut s: session = session {
        id: 0,
        name: 0 as *mut libc::c_char,
        cwd: 0 as *const libc::c_char,
        creation_time: timeval { tv_sec: 0, tv_usec: 0 },
        last_attached_time: timeval { tv_sec: 0, tv_usec: 0 },
        activity_time: timeval { tv_sec: 0, tv_usec: 0 },
        last_activity_time: timeval { tv_sec: 0, tv_usec: 0 },
        lock_timer: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_7 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_6 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_4 {
                ev_next_with_common_timeout: C2RustUnnamed_5 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed {
                ev_io: C2RustUnnamed_2 {
                    ev_io_next: C2RustUnnamed_3 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
        },
        sx: 0,
        sy: 0,
        curw: 0 as *mut winlink,
        lastw: winlink_stack {
            tqh_first: 0 as *mut winlink,
            tqh_last: 0 as *mut *mut winlink,
        },
        windows: winlinks {
            rbh_root: 0 as *mut winlink,
        },
        statusat: 0,
        hooks: 0 as *mut hooks,
        options: 0 as *mut options,
        flags: 0,
        attached: 0,
        tio: 0 as *mut termios,
        environ: 0 as *mut environ,
        references: 0,
        gentry: C2RustUnnamed_10 {
            tqe_next: 0 as *mut session,
            tqe_prev: 0 as *mut *mut session,
        },
        entry: C2RustUnnamed_9 {
            rbe_left: 0 as *mut session,
            rbe_right: 0 as *mut session,
            rbe_parent: 0 as *mut session,
            rbe_color: 0,
        },
    };
    s.name = name as *mut libc::c_char;
    return sessions_RB_FIND(&mut sessions, &mut s);
}
#[no_mangle]
pub unsafe extern "C" fn session_find_by_id_str(
    mut s: *const libc::c_char,
) -> *mut session {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != '$' as i32 {
        return 0 as *mut session;
    }
    id = strtonum(
        s.offset(1 as libc::c_int as isize),
        0 as libc::c_int as libc::c_longlong,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_longlong,
        &mut errstr,
    ) as u_int;
    if !errstr.is_null() {
        return 0 as *mut session;
    }
    return session_find_by_id(id);
}
#[no_mangle]
pub unsafe extern "C" fn session_find_by_id(mut id: u_int) -> *mut session {
    let mut s: *mut session = 0 as *mut session;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        if (*s).id == id {
            return s;
        }
        s = sessions_RB_NEXT(s);
    }
    return 0 as *mut session;
}
#[no_mangle]
pub unsafe extern "C" fn session_create(
    mut prefix: *const libc::c_char,
    mut name: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut env: *mut environ,
    mut tio: *mut termios,
    mut idx: libc::c_int,
    mut sx: u_int,
    mut sy: u_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut session {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    s = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<session>() as libc::c_ulong,
    ) as *mut session;
    (*s).references = 1 as libc::c_int;
    (*s).flags = 0 as libc::c_int;
    (*s).cwd = xstrdup(cwd);
    (*s).curw = 0 as *mut winlink;
    (*s).lastw.tqh_first = 0 as *mut winlink;
    (*s).lastw.tqh_last = &mut (*s).lastw.tqh_first;
    (*s).windows.rbh_root = 0 as *mut winlink;
    (*s).environ = environ_create();
    if !env.is_null() {
        environ_copy(env, (*s).environ);
    }
    (*s).options = options_create(global_s_options);
    (*s).hooks = hooks_create(global_hooks);
    status_update_saved(s);
    (*s).tio = 0 as *mut termios;
    if !tio.is_null() {
        (*s)
            .tio = xmalloc(::core::mem::size_of::<termios>() as libc::c_ulong)
            as *mut termios;
        memcpy(
            (*s).tio as *mut libc::c_void,
            tio as *const libc::c_void,
            ::core::mem::size_of::<termios>() as libc::c_ulong,
        );
    }
    (*s).sx = sx;
    (*s).sy = sy;
    if !name.is_null() {
        (*s).name = xstrdup(name);
        let fresh0 = next_session_id;
        next_session_id = next_session_id.wrapping_add(1);
        (*s).id = fresh0;
    } else {
        (*s).name = 0 as *mut libc::c_char;
        loop {
            let fresh1 = next_session_id;
            next_session_id = next_session_id.wrapping_add(1);
            (*s).id = fresh1;
            free((*s).name as *mut libc::c_void);
            if !prefix.is_null() {
                xasprintf(
                    &mut (*s).name as *mut *mut libc::c_char,
                    b"%s-%u\0" as *const u8 as *const libc::c_char,
                    prefix,
                    (*s).id,
                );
            } else {
                xasprintf(
                    &mut (*s).name as *mut *mut libc::c_char,
                    b"%u\0" as *const u8 as *const libc::c_char,
                    (*s).id,
                );
            }
            if (sessions_RB_FIND(&mut sessions, s)).is_null() {
                break;
            }
        }
    }
    sessions_RB_INSERT(&mut sessions, s);
    log_debug(
        b"new session %s $%u\0" as *const u8 as *const libc::c_char,
        (*s).name,
        (*s).id,
    );
    if gettimeofday(&mut (*s).creation_time, 0 as *mut libc::c_void) != 0 as libc::c_int
    {
        fatal(b"gettimeofday failed\0" as *const u8 as *const libc::c_char);
    }
    session_update_activity(s, &mut (*s).creation_time);
    if argc >= 0 as libc::c_int {
        wl = session_new(s, 0 as *const libc::c_char, argc, argv, path, cwd, idx, cause);
        if wl.is_null() {
            session_destroy(
                s,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"session_create\0"))
                    .as_ptr(),
            );
            return 0 as *mut session;
        }
        session_select(s, (*(*s).windows.rbh_root).idx);
    }
    log_debug(b"session %s created\0" as *const u8 as *const libc::c_char, (*s).name);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn session_add_ref(
    mut s: *mut session,
    mut from: *const libc::c_char,
) {
    (*s).references += 1;
    (*s).references;
    log_debug(
        b"%s: %s %s, now %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"session_add_ref\0"))
            .as_ptr(),
        (*s).name,
        from,
        (*s).references,
    );
}
#[no_mangle]
pub unsafe extern "C" fn session_remove_ref(
    mut s: *mut session,
    mut from: *const libc::c_char,
) {
    (*s).references -= 1;
    (*s).references;
    log_debug(
        b"%s: %s %s, now %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"session_remove_ref\0"))
            .as_ptr(),
        (*s).name,
        from,
        (*s).references,
    );
    if (*s).references == 0 as libc::c_int {
        event_once(
            -(1 as libc::c_int),
            0x1 as libc::c_int as libc::c_short,
            Some(
                session_free
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            s as *mut libc::c_void,
            0 as *const timeval,
        );
    }
}
unsafe extern "C" fn session_free(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut s: *mut session = arg as *mut session;
    log_debug(
        b"session %s freed (%d references)\0" as *const u8 as *const libc::c_char,
        (*s).name,
        (*s).references,
    );
    if (*s).references == 0 as libc::c_int {
        environ_free((*s).environ);
        options_free((*s).options);
        hooks_free((*s).hooks);
        free((*s).name as *mut libc::c_void);
        free(s as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn session_destroy(
    mut s: *mut session,
    mut from: *const libc::c_char,
) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    log_debug(
        b"session %s destroyed (%s)\0" as *const u8 as *const libc::c_char,
        (*s).name,
        from,
    );
    (*s).curw = 0 as *mut winlink;
    sessions_RB_REMOVE(&mut sessions, s);
    notify_session(b"session-closed\0" as *const u8 as *const libc::c_char, s);
    free((*s).tio as *mut libc::c_void);
    if event_initialized(&mut (*s).lock_timer) != 0 {
        event_del(&mut (*s).lock_timer);
    }
    session_group_remove(s);
    while !((*s).lastw.tqh_first).is_null() {
        winlink_stack_remove(&mut (*s).lastw, (*s).lastw.tqh_first);
    }
    while !((*s).windows.rbh_root).is_null() {
        wl = (*s).windows.rbh_root;
        notify_session_window(
            b"window-unlinked\0" as *const u8 as *const libc::c_char,
            s,
            (*wl).window,
        );
        winlink_remove(&mut (*s).windows, wl);
    }
    free((*s).cwd as *mut libc::c_void);
    session_remove_ref(
        s,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"session_destroy\0"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn session_check_name(
    mut name: *const libc::c_char,
) -> libc::c_int {
    return (*name as libc::c_int != '\0' as i32
        && *name
            .offset(strcspn(name, b":.\0" as *const u8 as *const libc::c_char) as isize)
            as libc::c_int == '\0' as i32) as libc::c_int;
}
unsafe extern "C" fn session_lock_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut s: *mut session = arg as *mut session;
    if (*s).flags & 0x1 as libc::c_int != 0 {
        return;
    }
    log_debug(
        b"session %s locked, activity time %lld\0" as *const u8 as *const libc::c_char,
        (*s).name,
        (*s).activity_time.tv_sec as libc::c_longlong,
    );
    server_lock_session(s);
    recalculate_sizes();
}
#[no_mangle]
pub unsafe extern "C" fn session_update_activity(
    mut s: *mut session,
    mut from: *mut timeval,
) {
    let mut last: *mut timeval = &mut (*s).last_activity_time;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    memcpy(
        last as *mut libc::c_void,
        &mut (*s).activity_time as *mut timeval as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    if from.is_null() {
        gettimeofday(&mut (*s).activity_time, 0 as *mut libc::c_void);
    } else {
        memcpy(
            &mut (*s).activity_time as *mut timeval as *mut libc::c_void,
            from as *const libc::c_void,
            ::core::mem::size_of::<timeval>() as libc::c_ulong,
        );
    }
    log_debug(
        b"session %s activity %lld.%06d (last %lld.%06d)\0" as *const u8
            as *const libc::c_char,
        (*s).name,
        (*s).activity_time.tv_sec as libc::c_longlong,
        (*s).activity_time.tv_usec as libc::c_int,
        (*last).tv_sec as libc::c_longlong,
        (*last).tv_usec as libc::c_int,
    );
    if event_initialized(&mut (*s).lock_timer) != 0 {
        event_del(&mut (*s).lock_timer);
    } else {
        event_set(
            &mut (*s).lock_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                session_lock_timer
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            s as *mut libc::c_void,
        );
    }
    if !(*s).flags & 0x1 as libc::c_int != 0 {
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        tv.tv_sec = tv.tv_usec;
        tv
            .tv_sec = options_get_number(
            (*s).options,
            b"lock-after-time\0" as *const u8 as *const libc::c_char,
        ) as __time_t;
        if tv.tv_sec != 0 as libc::c_int as __time_t {
            event_add(&mut (*s).lock_timer, &mut tv);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn session_next_session(mut s: *mut session) -> *mut session {
    let mut s2: *mut session = 0 as *mut session;
    if (sessions.rbh_root).is_null() || session_alive(s) == 0 {
        return 0 as *mut session;
    }
    s2 = sessions_RB_NEXT(s);
    if s2.is_null() {
        s2 = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    }
    if s2 == s {
        return 0 as *mut session;
    }
    return s2;
}
#[no_mangle]
pub unsafe extern "C" fn session_previous_session(mut s: *mut session) -> *mut session {
    let mut s2: *mut session = 0 as *mut session;
    if (sessions.rbh_root).is_null() || session_alive(s) == 0 {
        return 0 as *mut session;
    }
    s2 = sessions_RB_PREV(s);
    if s2.is_null() {
        s2 = sessions_RB_MINMAX(&mut sessions, 1 as libc::c_int);
    }
    if s2 == s {
        return 0 as *mut session;
    }
    return s2;
}
#[no_mangle]
pub unsafe extern "C" fn session_new(
    mut s: *mut session,
    mut name: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut idx: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut winlink {
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut env: *mut environ = 0 as *mut environ;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut hlimit: u_int = 0;
    wl = winlink_add(&mut (*s).windows, idx);
    if wl.is_null() {
        xasprintf(cause, b"index in use: %d\0" as *const u8 as *const libc::c_char, idx);
        return 0 as *mut winlink;
    }
    (*wl).session = s;
    shell = options_get_string(
        (*s).options,
        b"default-shell\0" as *const u8 as *const libc::c_char,
    );
    if *shell as libc::c_int == '\0' as i32 || areshell(shell) != 0 {
        shell = b"/bin/sh\0" as *const u8 as *const libc::c_char;
    }
    hlimit = options_get_number(
        (*s).options,
        b"history-limit\0" as *const u8 as *const libc::c_char,
    ) as u_int;
    env = environ_for_session(s, 0 as libc::c_int);
    w = window_create_spawn(
        name,
        argc,
        argv,
        path,
        shell,
        cwd,
        env,
        (*s).tio,
        (*s).sx,
        (*s).sy,
        hlimit,
        cause,
    );
    if w.is_null() {
        winlink_remove(&mut (*s).windows, wl);
        environ_free(env);
        return 0 as *mut winlink;
    }
    winlink_set_window(wl, w);
    environ_free(env);
    notify_session_window(b"window-linked\0" as *const u8 as *const libc::c_char, s, w);
    session_group_synchronize_from(s);
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn session_attach(
    mut s: *mut session,
    mut w: *mut window,
    mut idx: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlink_add(&mut (*s).windows, idx);
    if wl.is_null() {
        xasprintf(cause, b"index in use: %d\0" as *const u8 as *const libc::c_char, idx);
        return 0 as *mut winlink;
    }
    (*wl).session = s;
    winlink_set_window(wl, w);
    notify_session_window(b"window-linked\0" as *const u8 as *const libc::c_char, s, w);
    session_group_synchronize_from(s);
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn session_detach(
    mut s: *mut session,
    mut wl: *mut winlink,
) -> libc::c_int {
    if (*s).curw == wl && session_last(s) != 0 as libc::c_int
        && session_previous(s, 0 as libc::c_int) != 0 as libc::c_int
    {
        session_next(s, 0 as libc::c_int);
    }
    (*wl).flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
    notify_session_window(
        b"window-unlinked\0" as *const u8 as *const libc::c_char,
        s,
        (*wl).window,
    );
    winlink_stack_remove(&mut (*s).lastw, wl);
    winlink_remove(&mut (*s).windows, wl);
    session_group_synchronize_from(s);
    if ((*s).windows.rbh_root).is_null() {
        session_destroy(
            s,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"session_detach\0"))
                .as_ptr(),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn session_has(
    mut s: *mut session,
    mut w: *mut window,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if (*wl).session == s {
            return 1 as libc::c_int;
        }
        wl = (*wl).wentry.tqe_next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn session_is_linked(
    mut s: *mut session,
    mut w: *mut window,
) -> libc::c_int {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if !sg.is_null() {
        return ((*w).references != session_group_count(sg)) as libc::c_int;
    }
    return ((*w).references != 1 as libc::c_int as u_int) as libc::c_int;
}
unsafe extern "C" fn session_next_alert(mut wl: *mut winlink) -> *mut winlink {
    while !wl.is_null() {
        if (*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0
        {
            break;
        }
        wl = winlink_next(wl);
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn session_next(
    mut s: *mut session,
    mut alert: libc::c_int,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if ((*s).curw).is_null() {
        return -(1 as libc::c_int);
    }
    wl = winlink_next((*s).curw);
    if alert != 0 {
        wl = session_next_alert(wl);
    }
    if wl.is_null() {
        wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
        if alert != 0
            && {
                wl = session_next_alert(wl);
                wl.is_null()
            }
        {
            return -(1 as libc::c_int);
        }
    }
    return session_set_current(s, wl);
}
unsafe extern "C" fn session_previous_alert(mut wl: *mut winlink) -> *mut winlink {
    while !wl.is_null() {
        if (*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0
        {
            break;
        }
        wl = winlink_previous(wl);
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn session_previous(
    mut s: *mut session,
    mut alert: libc::c_int,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if ((*s).curw).is_null() {
        return -(1 as libc::c_int);
    }
    wl = winlink_previous((*s).curw);
    if alert != 0 {
        wl = session_previous_alert(wl);
    }
    if wl.is_null() {
        wl = winlinks_RB_MINMAX(&mut (*s).windows, 1 as libc::c_int);
        if alert != 0
            && {
                wl = session_previous_alert(wl);
                wl.is_null()
            }
        {
            return -(1 as libc::c_int);
        }
    }
    return session_set_current(s, wl);
}
#[no_mangle]
pub unsafe extern "C" fn session_select(
    mut s: *mut session,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlink_find_by_index(&mut (*s).windows, idx);
    return session_set_current(s, wl);
}
#[no_mangle]
pub unsafe extern "C" fn session_last(mut s: *mut session) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = (*s).lastw.tqh_first;
    if wl.is_null() {
        return -(1 as libc::c_int);
    }
    if wl == (*s).curw {
        return 1 as libc::c_int;
    }
    return session_set_current(s, wl);
}
#[no_mangle]
pub unsafe extern "C" fn session_set_current(
    mut s: *mut session,
    mut wl: *mut winlink,
) -> libc::c_int {
    if wl.is_null() {
        return -(1 as libc::c_int);
    }
    if wl == (*s).curw {
        return 1 as libc::c_int;
    }
    winlink_stack_remove(&mut (*s).lastw, wl);
    winlink_stack_push(&mut (*s).lastw, (*s).curw);
    (*s).curw = wl;
    winlink_clear_flags(wl);
    window_update_activity((*wl).window);
    notify_session(b"session-window-changed\0" as *const u8 as *const libc::c_char, s);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn session_group_contains(
    mut target: *mut session,
) -> *mut session_group {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s: *mut session = 0 as *mut session;
    sg = session_groups_RB_MINMAX(&mut session_groups, -(1 as libc::c_int));
    while !sg.is_null() {
        s = (*sg).sessions.tqh_first;
        while !s.is_null() {
            if s == target {
                return sg;
            }
            s = (*s).gentry.tqe_next;
        }
        sg = session_groups_RB_NEXT(sg);
    }
    return 0 as *mut session_group;
}
#[no_mangle]
pub unsafe extern "C" fn session_group_find(
    mut name: *const libc::c_char,
) -> *mut session_group {
    let mut sg: session_group = session_group {
        name: 0 as *const libc::c_char,
        sessions: C2RustUnnamed_35 {
            tqh_first: 0 as *mut session,
            tqh_last: 0 as *mut *mut session,
        },
        entry: C2RustUnnamed_34 {
            rbe_left: 0 as *mut session_group,
            rbe_right: 0 as *mut session_group,
            rbe_parent: 0 as *mut session_group,
            rbe_color: 0,
        },
    };
    sg.name = name;
    return session_groups_RB_FIND(&mut session_groups, &mut sg);
}
#[no_mangle]
pub unsafe extern "C" fn session_group_new(
    mut name: *const libc::c_char,
) -> *mut session_group {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_find(name);
    if !sg.is_null() {
        return sg;
    }
    sg = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<session_group>() as libc::c_ulong,
    ) as *mut session_group;
    (*sg).name = xstrdup(name);
    (*sg).sessions.tqh_first = 0 as *mut session;
    (*sg).sessions.tqh_last = &mut (*sg).sessions.tqh_first;
    session_groups_RB_INSERT(&mut session_groups, sg);
    return sg;
}
#[no_mangle]
pub unsafe extern "C" fn session_group_add(
    mut sg: *mut session_group,
    mut s: *mut session,
) {
    if (session_group_contains(s)).is_null() {
        (*s).gentry.tqe_next = 0 as *mut session;
        (*s).gentry.tqe_prev = (*sg).sessions.tqh_last;
        *(*sg).sessions.tqh_last = s;
        (*sg).sessions.tqh_last = &mut (*s).gentry.tqe_next;
    }
}
unsafe extern "C" fn session_group_remove(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if sg.is_null() {
        return;
    }
    if !((*s).gentry.tqe_next).is_null() {
        (*(*s).gentry.tqe_next).gentry.tqe_prev = (*s).gentry.tqe_prev;
    } else {
        (*sg).sessions.tqh_last = (*s).gentry.tqe_prev;
    }
    *(*s).gentry.tqe_prev = (*s).gentry.tqe_next;
    if ((*sg).sessions.tqh_first).is_null() {
        session_groups_RB_REMOVE(&mut session_groups, sg);
        free(sg as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn session_group_count(mut sg: *mut session_group) -> u_int {
    let mut s: *mut session = 0 as *mut session;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    s = (*sg).sessions.tqh_first;
    while !s.is_null() {
        n = n.wrapping_add(1);
        n;
        s = (*s).gentry.tqe_next;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn session_group_synchronize_to(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut target: *mut session = 0 as *mut session;
    sg = session_group_contains(s);
    if sg.is_null() {
        return;
    }
    target = 0 as *mut session;
    target = (*sg).sessions.tqh_first;
    while !target.is_null() {
        if target != s {
            break;
        }
        target = (*target).gentry.tqe_next;
    }
    if !target.is_null() {
        session_group_synchronize1(target, s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn session_group_synchronize_from(mut target: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s: *mut session = 0 as *mut session;
    sg = session_group_contains(target);
    if sg.is_null() {
        return;
    }
    s = (*sg).sessions.tqh_first;
    while !s.is_null() {
        if s != target {
            session_group_synchronize1(target, s);
        }
        s = (*s).gentry.tqe_next;
    }
}
unsafe extern "C" fn session_group_synchronize1(
    mut target: *mut session,
    mut s: *mut session,
) {
    let mut old_windows: winlinks = winlinks {
        rbh_root: 0 as *mut winlink,
    };
    let mut ww: *mut winlinks = 0 as *mut winlinks;
    let mut old_lastw: winlink_stack = winlink_stack {
        tqh_first: 0 as *mut winlink,
        tqh_last: 0 as *mut *mut winlink,
    };
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl2: *mut winlink = 0 as *mut winlink;
    ww = &mut (*target).windows;
    if ((*ww).rbh_root).is_null() {
        return;
    }
    if !((*s).curw).is_null() && (winlink_find_by_index(ww, (*(*s).curw).idx)).is_null()
        && session_last(s) != 0 as libc::c_int
        && session_previous(s, 0 as libc::c_int) != 0 as libc::c_int
    {
        session_next(s, 0 as libc::c_int);
    }
    memcpy(
        &mut old_windows as *mut winlinks as *mut libc::c_void,
        &mut (*s).windows as *mut winlinks as *const libc::c_void,
        ::core::mem::size_of::<winlinks>() as libc::c_ulong,
    );
    (*s).windows.rbh_root = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(ww, -(1 as libc::c_int));
    while !wl.is_null() {
        wl2 = winlink_add(&mut (*s).windows, (*wl).idx);
        (*wl2).session = s;
        winlink_set_window(wl2, (*wl).window);
        notify_session_window(
            b"window-linked\0" as *const u8 as *const libc::c_char,
            s,
            (*wl2).window,
        );
        (*wl2).flags
            |= (*wl).flags
                & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
        wl = winlinks_RB_NEXT(wl);
    }
    if !((*s).curw).is_null() {
        (*s).curw = winlink_find_by_index(&mut (*s).windows, (*(*s).curw).idx);
    } else {
        (*s).curw = winlink_find_by_index(&mut (*s).windows, (*(*target).curw).idx);
    }
    memcpy(
        &mut old_lastw as *mut winlink_stack as *mut libc::c_void,
        &mut (*s).lastw as *mut winlink_stack as *const libc::c_void,
        ::core::mem::size_of::<winlink_stack>() as libc::c_ulong,
    );
    (*s).lastw.tqh_first = 0 as *mut winlink;
    (*s).lastw.tqh_last = &mut (*s).lastw.tqh_first;
    wl = old_lastw.tqh_first;
    while !wl.is_null() {
        wl2 = winlink_find_by_index(&mut (*s).windows, (*wl).idx);
        if !wl2.is_null() {
            (*wl2).sentry.tqe_next = 0 as *mut winlink;
            (*wl2).sentry.tqe_prev = (*s).lastw.tqh_last;
            *(*s).lastw.tqh_last = wl2;
            (*s).lastw.tqh_last = &mut (*wl2).sentry.tqe_next;
        }
        wl = (*wl).sentry.tqe_next;
    }
    while !(old_windows.rbh_root).is_null() {
        wl = old_windows.rbh_root;
        wl2 = winlink_find_by_window_id(&mut (*s).windows, (*(*wl).window).id);
        if wl2.is_null() {
            notify_session_window(
                b"window-unlinked\0" as *const u8 as *const libc::c_char,
                s,
                (*wl).window,
            );
        }
        winlink_remove(&mut old_windows, wl);
    }
}
#[no_mangle]
pub unsafe extern "C" fn session_renumber_windows(mut s: *mut session) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl1: *mut winlink = 0 as *mut winlink;
    let mut wl_new: *mut winlink = 0 as *mut winlink;
    let mut old_wins: winlinks = winlinks {
        rbh_root: 0 as *mut winlink,
    };
    let mut old_lastw: winlink_stack = winlink_stack {
        tqh_first: 0 as *mut winlink,
        tqh_last: 0 as *mut *mut winlink,
    };
    let mut new_idx: libc::c_int = 0;
    let mut new_curw_idx: libc::c_int = 0;
    memcpy(
        &mut old_wins as *mut winlinks as *mut libc::c_void,
        &mut (*s).windows as *mut winlinks as *const libc::c_void,
        ::core::mem::size_of::<winlinks>() as libc::c_ulong,
    );
    (*s).windows.rbh_root = 0 as *mut winlink;
    new_idx = options_get_number(
        (*s).options,
        b"base-index\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    new_curw_idx = 0 as libc::c_int;
    wl = winlinks_RB_MINMAX(&mut old_wins, -(1 as libc::c_int));
    while !wl.is_null() {
        wl_new = winlink_add(&mut (*s).windows, new_idx);
        (*wl_new).session = s;
        winlink_set_window(wl_new, (*wl).window);
        (*wl_new).flags
            |= (*wl).flags
                & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
        if wl == (*s).curw {
            new_curw_idx = (*wl_new).idx;
        }
        new_idx += 1;
        new_idx;
        wl = winlinks_RB_NEXT(wl);
    }
    memcpy(
        &mut old_lastw as *mut winlink_stack as *mut libc::c_void,
        &mut (*s).lastw as *mut winlink_stack as *const libc::c_void,
        ::core::mem::size_of::<winlink_stack>() as libc::c_ulong,
    );
    (*s).lastw.tqh_first = 0 as *mut winlink;
    (*s).lastw.tqh_last = &mut (*s).lastw.tqh_first;
    wl = old_lastw.tqh_first;
    while !wl.is_null() {
        wl_new = winlink_find_by_window(&mut (*s).windows, (*wl).window);
        if !wl_new.is_null() {
            (*wl_new).sentry.tqe_next = 0 as *mut winlink;
            (*wl_new).sentry.tqe_prev = (*s).lastw.tqh_last;
            *(*s).lastw.tqh_last = wl_new;
            (*s).lastw.tqh_last = &mut (*wl_new).sentry.tqe_next;
        }
        wl = (*wl).sentry.tqe_next;
    }
    (*s).curw = winlink_find_by_index(&mut (*s).windows, new_curw_idx);
    wl = winlinks_RB_MINMAX(&mut old_wins, -(1 as libc::c_int));
    while !wl.is_null()
        && {
            wl1 = winlinks_RB_NEXT(wl);
            1 as libc::c_int != 0
        }
    {
        winlink_remove(&mut old_wins, wl);
        wl = wl1;
    }
}
