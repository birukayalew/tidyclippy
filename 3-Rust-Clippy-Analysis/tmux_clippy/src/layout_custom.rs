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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn notify_window(_: *const libc::c_char, _: *mut window);
    fn window_resize(_: *mut window, _: u_int, _: u_int);
    fn window_count_panes(_: *mut window) -> u_int;
    fn layout_count_cells(_: *mut layout_cell) -> u_int;
    fn layout_create_cell(_: *mut layout_cell) -> *mut layout_cell;
    fn layout_free_cell(_: *mut layout_cell);
    fn layout_print_cell(_: *mut layout_cell, _: *const libc::c_char, _: u_int);
    fn layout_destroy_cell(
        _: *mut window,
        _: *mut layout_cell,
        _: *mut *mut layout_cell,
    );
    fn layout_make_leaf(_: *mut layout_cell, _: *mut window_pane);
    fn layout_fix_offsets(_: *mut layout_cell);
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int);
    fn layout_resize(_: *mut window, _: u_int, _: u_int);
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
    pub entry: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub gentry: C2RustUnnamed_11,
    pub entry: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
unsafe extern "C" fn layout_find_bottomright(
    mut lc: *mut layout_cell,
) -> *mut layout_cell {
    if (*lc).type_0 as libc::c_uint == LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return lc;
    }
    lc = *(*((*lc).cells.tqh_last as *mut layout_cells)).tqh_last;
    return layout_find_bottomright(lc);
}
unsafe extern "C" fn layout_checksum(mut layout: *const libc::c_char) -> u_short {
    let mut csum: u_short = 0;
    csum = 0 as libc::c_int as u_short;
    while *layout as libc::c_int != '\0' as i32 {
        csum = ((csum as libc::c_int >> 1 as libc::c_int)
            + ((csum as libc::c_int & 1 as libc::c_int) << 15 as libc::c_int))
            as u_short;
        csum = (csum as libc::c_int + *layout as libc::c_int) as u_short;
        layout = layout.offset(1);
        layout;
    }
    return csum;
}
#[no_mangle]
pub unsafe extern "C" fn layout_dump(mut root: *mut layout_cell) -> *mut libc::c_char {
    let mut layout: [libc::c_char; 8192] = [0; 8192];
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    *layout.as_mut_ptr() = '\0' as i32 as libc::c_char;
    if layout_append(
        root,
        layout.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    xasprintf(
        &mut out as *mut *mut libc::c_char,
        b"%04hx,%s\0" as *const u8 as *const libc::c_char,
        layout_checksum(layout.as_mut_ptr()) as libc::c_int,
        layout.as_mut_ptr(),
    );
    return out;
}
unsafe extern "C" fn layout_append(
    mut lc: *mut layout_cell,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut tmplen: size_t = 0;
    let mut brackets: *const libc::c_char = b"][\0" as *const u8 as *const libc::c_char;
    if len == 0 as libc::c_int as size_t {
        return -(1 as libc::c_int);
    }
    if !((*lc).wp).is_null() {
        tmplen = xsnprintf(
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%ux%u,%u,%u,%u\0" as *const u8 as *const libc::c_char,
            (*lc).sx,
            (*lc).sy,
            (*lc).xoff,
            (*lc).yoff,
            (*(*lc).wp).id,
        ) as size_t;
    } else {
        tmplen = xsnprintf(
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%ux%u,%u,%u\0" as *const u8 as *const libc::c_char,
            (*lc).sx,
            (*lc).sy,
            (*lc).xoff,
            (*lc).yoff,
        ) as size_t;
    }
    if tmplen
        > (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    if strlcat(buf, tmp.as_mut_ptr(), len) >= len {
        return -(1 as libc::c_int);
    }
    let mut current_block_21: u64;
    match (*lc).type_0 as libc::c_uint {
        0 => {
            brackets = b"}{\0" as *const u8 as *const libc::c_char;
            current_block_21 = 11056912653737305132;
        }
        1 => {
            current_block_21 = 11056912653737305132;
        }
        2 | _ => {
            current_block_21 = 11584701595673473500;
        }
    }
    match current_block_21 {
        11056912653737305132 => {
            if strlcat(buf, &*brackets.offset(1 as libc::c_int as isize), len) >= len {
                return -(1 as libc::c_int);
            }
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                if layout_append(lcchild, buf, len) != 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if strlcat(buf, b",\0" as *const u8 as *const libc::c_char, len) >= len {
                    return -(1 as libc::c_int);
                }
                lcchild = (*lcchild).entry.tqe_next;
            }
            *buf
                .offset(
                    (strlen(buf)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) = *brackets.offset(0 as libc::c_int as isize);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn layout_parse(
    mut w: *mut window,
    mut layout: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut npanes: u_int = 0;
    let mut ncells: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut csum: u_short = 0;
    if sscanf(
        layout,
        b"%hx,\0" as *const u8 as *const libc::c_char,
        &mut csum as *mut u_short,
    ) != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    layout = layout.offset(5 as libc::c_int as isize);
    if csum as libc::c_int != layout_checksum(layout) as libc::c_int {
        return -(1 as libc::c_int);
    }
    lc = layout_construct(0 as *mut layout_cell, &mut layout);
    if lc.is_null() {
        return -(1 as libc::c_int);
    }
    if *layout as libc::c_int != '\0' as i32 {
        current_block = 6434713000406556480;
    } else {
        current_block = 5720623009719927633;
    }
    loop {
        match current_block {
            6434713000406556480 => {
                layout_free_cell(lc);
                return -(1 as libc::c_int);
            }
            _ => {
                npanes = window_count_panes(w);
                ncells = layout_count_cells(lc);
                if npanes > ncells {
                    current_block = 6434713000406556480;
                    continue;
                }
                if npanes == ncells {
                    sx = (*w).sx;
                    sy = (*w).sy;
                    window_resize(w, (*lc).sx, (*lc).sy);
                    layout_free_cell((*w).layout_root);
                    (*w).layout_root = lc;
                    wp = (*w).panes.tqh_first;
                    layout_assign(&mut wp, lc);
                    layout_fix_offsets(lc);
                    layout_fix_panes(w, (*lc).sx, (*lc).sy);
                    layout_resize(w, sx, sy);
                    window_resize(w, sx, sy);
                    layout_print_cell(
                        lc,
                        (*::core::mem::transmute::<
                            &[u8; 13],
                            &[libc::c_char; 13],
                        >(b"layout_parse\0"))
                            .as_ptr(),
                        0 as libc::c_int as u_int,
                    );
                    notify_window(
                        b"window-layout-changed\0" as *const u8 as *const libc::c_char,
                        w,
                    );
                    return 0 as libc::c_int;
                } else {
                    lcchild = layout_find_bottomright(lc);
                    layout_destroy_cell(w, lcchild, &mut lc);
                    current_block = 5720623009719927633;
                }
            }
        }
    };
}
unsafe extern "C" fn layout_assign(
    mut wp: *mut *mut window_pane,
    mut lc: *mut layout_cell,
) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    match (*lc).type_0 as libc::c_uint {
        2 => {
            layout_make_leaf(lc, *wp);
            *wp = (**wp).entry.tqe_next;
            return;
        }
        0 | 1 => {
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                layout_assign(wp, lcchild);
                lcchild = (*lcchild).entry.tqe_next;
            }
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn layout_construct(
    mut lcparent: *mut layout_cell,
    mut layout: *mut *const libc::c_char,
) -> *mut layout_cell {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut saved: *const libc::c_char = 0 as *const libc::c_char;
    if *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as *mut layout_cell;
    }
    if sscanf(
        *layout,
        b"%ux%u,%u,%u\0" as *const u8 as *const libc::c_char,
        &mut sx as *mut u_int,
        &mut sy as *mut u_int,
        &mut xoff as *mut u_int,
        &mut yoff as *mut u_int,
    ) != 4 as libc::c_int
    {
        return 0 as *mut layout_cell;
    }
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *layout = (*layout).offset(1);
        *layout;
    }
    if **layout as libc::c_int != 'x' as i32 {
        return 0 as *mut layout_cell;
    }
    *layout = (*layout).offset(1);
    *layout;
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *layout = (*layout).offset(1);
        *layout;
    }
    if **layout as libc::c_int != ',' as i32 {
        return 0 as *mut layout_cell;
    }
    *layout = (*layout).offset(1);
    *layout;
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *layout = (*layout).offset(1);
        *layout;
    }
    if **layout as libc::c_int != ',' as i32 {
        return 0 as *mut layout_cell;
    }
    *layout = (*layout).offset(1);
    *layout;
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *layout = (*layout).offset(1);
        *layout;
    }
    if **layout as libc::c_int == ',' as i32 {
        saved = *layout;
        *layout = (*layout).offset(1);
        *layout;
        while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *layout = (*layout).offset(1);
            *layout;
        }
        if **layout as libc::c_int == 'x' as i32 {
            *layout = saved;
        }
    }
    lc = layout_create_cell(lcparent);
    (*lc).sx = sx;
    (*lc).sy = sy;
    (*lc).xoff = xoff;
    (*lc).yoff = yoff;
    match **layout as libc::c_int {
        44 | 125 | 93 | 0 => return lc,
        123 => {
            (*lc).type_0 = LAYOUT_LEFTRIGHT;
            current_block = 11913429853522160501;
        }
        91 => {
            (*lc).type_0 = LAYOUT_TOPBOTTOM;
            current_block = 11913429853522160501;
        }
        _ => {
            current_block = 4520352990960179858;
        }
    }
    loop {
        match current_block {
            4520352990960179858 => {
                layout_free_cell(lc);
                return 0 as *mut layout_cell;
            }
            _ => {
                *layout = (*layout).offset(1);
                *layout;
                lcchild = layout_construct(lc, layout);
                if lcchild.is_null() {
                    current_block = 4520352990960179858;
                    continue;
                }
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lc).cells.tqh_last;
                *(*lc).cells.tqh_last = lcchild;
                (*lc).cells.tqh_last = &mut (*lcchild).entry.tqe_next;
                if **layout as libc::c_int == ',' as i32 {
                    current_block = 11913429853522160501;
                    continue;
                }
                match (*lc).type_0 as libc::c_uint {
                    0 => {
                        if **layout as libc::c_int != '}' as i32 {
                            current_block = 4520352990960179858;
                        } else {
                            break;
                        }
                    }
                    1 => {
                        if **layout as libc::c_int != ']' as i32 {
                            current_block = 4520352990960179858;
                        } else {
                            break;
                        }
                    }
                    _ => {
                        current_block = 4520352990960179858;
                    }
                }
            }
        }
    }
    *layout = (*layout).offset(1);
    *layout;
    return lc;
}
