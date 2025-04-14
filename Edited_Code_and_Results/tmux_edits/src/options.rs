use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type environ;
    pub type hooks;
    pub type screen_titles;
    pub type input_ctx;
    pub type format_tree;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    static mut global_options: *mut options;
    static mut global_s_options: *mut options;
    static mut global_w_options: *mut options;
    static options_table: [options_table_entry; 0];
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    fn key_string_lookup_key(_: key_code) -> *const libc::c_char;
    fn colour_tostring(_: libc::c_int) -> *const libc::c_char;
    fn attributes_tostring(_: libc::c_int) -> *const libc::c_char;
    static grid_default_cell: grid_cell;
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    fn style_parse(
        _: *const grid_cell,
        _: *mut grid_cell,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn style_tostring(_: *mut grid_cell) -> *const libc::c_char;
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
    pub message_log: C2RustUnnamed_28,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_27,
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
pub struct options {
    pub tree: options_tree,
    pub parent: *mut options,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_tree {
    pub rbh_root: *mut options_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_entry {
    pub owner: *mut options,
    pub name: *const libc::c_char,
    pub tableentry: *const options_table_entry,
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub entry: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub rbe_left: *mut options_entry,
    pub rbe_right: *mut options_entry,
    pub rbe_parent: *mut options_entry,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub string: *mut libc::c_char,
    pub number: libc::c_longlong,
    pub style: grid_cell,
    pub c2rust_unnamed: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub array: *mut *const libc::c_char,
    pub arraysize: u_int,
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
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
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
    pub entry: C2RustUnnamed_17,
    pub wentry: C2RustUnnamed_16,
    pub sentry: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
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
    pub alerts_entry: C2RustUnnamed_20,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_19,
    pub entry: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
    pub entry: C2RustUnnamed_23,
    pub tree_entry: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
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
    pub lineflag: C2RustUnnamed_24,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type C2RustUnnamed_24 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed_24 = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed_24 = 1;
pub const LINE_SEL_NONE: C2RustUnnamed_24 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub offset: u_int,
    pub data: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
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
pub type C2RustUnnamed_27 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_27 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_27 = 0;
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
pub struct C2RustUnnamed_28 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
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
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
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
    pub qentry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_35,
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
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
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
pub struct C2RustUnnamed_35 {
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
    pub term_type: C2RustUnnamed_36,
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
pub type C2RustUnnamed_36 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_36 = 6;
pub const TTY_VT420: C2RustUnnamed_36 = 5;
pub const TTY_VT320: C2RustUnnamed_36 = 4;
pub const TTY_VT220: C2RustUnnamed_36 = 3;
pub const TTY_VT102: C2RustUnnamed_36 = 2;
pub const TTY_VT101: C2RustUnnamed_36 = 1;
pub const TTY_VT100: C2RustUnnamed_36 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
unsafe extern "C" fn options_tree_RB_REMOVE_COLOR(
    mut head: *mut options_tree,
    mut parent: *mut options_entry,
    mut elm: *mut options_entry,
) {
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
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
                    let mut oleft: *mut options_entry = 0 as *mut options_entry;
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
                    let mut oright: *mut options_entry = 0 as *mut options_entry;
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
unsafe extern "C" fn options_tree_RB_INSERT_COLOR(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) {
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut gparent: *mut options_entry = 0 as *mut options_entry;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
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
unsafe extern "C" fn options_tree_RB_FIND(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = options_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut options_entry;
}
unsafe extern "C" fn options_tree_RB_REMOVE(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) -> *mut options_entry {
    let mut current_block: u64;
    let mut child: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut old: *mut options_entry = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut options_entry = 0 as *mut options_entry;
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
        current_block = 18027585373076655206;
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
        options_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn options_tree_RB_MINMAX(
    mut head: *mut options_tree,
    mut val: libc::c_int,
) -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
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
unsafe extern "C" fn options_tree_RB_INSERT(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) -> *mut options_entry {
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = options_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut options_entry;
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
    options_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut options_entry;
}
unsafe extern "C" fn options_tree_RB_NEXT(
    mut elm: *mut options_entry,
) -> *mut options_entry {
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
unsafe extern "C" fn options_cmp(
    mut lhs: *mut options_entry,
    mut rhs: *mut options_entry,
) -> libc::c_int {
    return strcmp((*lhs).name, (*rhs).name);
}
#[no_mangle]
pub unsafe extern "C" fn options_parent_table_entry(
    mut oo: *mut options,
    mut s: *const libc::c_char,
) -> *const options_table_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if ((*oo).parent).is_null() {
        fatalx(b"no parent options for %s\0" as *const u8 as *const libc::c_char, s);
    }
    o = options_get_only((*oo).parent, s);
    if o.is_null() {
        fatalx(b"%s not in parent options\0" as *const u8 as *const libc::c_char, s);
    }
    return (*o).tableentry;
}
#[no_mangle]
pub unsafe extern "C" fn options_create(mut parent: *mut options) -> *mut options {
    let mut oo: *mut options = 0 as *mut options;
    oo = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<options>() as libc::c_ulong,
    ) as *mut options;
    (*oo).tree.rbh_root = 0 as *mut options_entry;
    (*oo).parent = parent;
    return oo;
}
#[no_mangle]
pub unsafe extern "C" fn options_free(mut oo: *mut options) {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    o = options_tree_RB_MINMAX(&mut (*oo).tree, -(1 as libc::c_int));
    while !o.is_null()
        && {
            tmp = options_tree_RB_NEXT(o);
            1 as libc::c_int != 0
        }
    {
        options_remove(o);
        o = tmp;
    }
    free(oo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_first(mut oo: *mut options) -> *mut options_entry {
    return options_tree_RB_MINMAX(&mut (*oo).tree, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn options_next(mut o: *mut options_entry) -> *mut options_entry {
    return options_tree_RB_NEXT(o);
}
#[no_mangle]
pub unsafe extern "C" fn options_get_only(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *mut options_entry {
    let mut o: options_entry = options_entry {
        owner: 0 as *mut options,
        name: 0 as *const libc::c_char,
        tableentry: 0 as *const options_table_entry,
        c2rust_unnamed: C2RustUnnamed_13 {
            string: 0 as *mut libc::c_char,
        },
        entry: C2RustUnnamed_12 {
            rbe_left: 0 as *mut options_entry,
            rbe_right: 0 as *mut options_entry,
            rbe_parent: 0 as *mut options_entry,
            rbe_color: 0,
        },
    };
    o.name = name;
    return options_tree_RB_FIND(&mut (*oo).tree, &mut o);
}
#[no_mangle]
pub unsafe extern "C" fn options_get(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get_only(oo, name);
    while o.is_null() {
        oo = (*oo).parent;
        if oo.is_null() {
            break;
        }
        o = options_get_only(oo, name);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_empty(
    mut oo: *mut options,
    mut oe: *const options_table_entry,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_add(oo, (*oe).name);
    (*o).tableentry = oe;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_default(
    mut oo: *mut options,
    mut oe: *const options_table_entry,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_empty(oo, oe);
    if (*oe).type_0 as libc::c_uint == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint
    {
        options_array_assign(o, (*oe).default_str);
    } else if (*oe).type_0 as libc::c_uint
        == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint
    {
        (*o).c2rust_unnamed.string = xstrdup((*oe).default_str);
    } else if (*oe).type_0 as libc::c_uint
        == OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint
    {
        memcpy(
            &mut (*o).c2rust_unnamed.style as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        style_parse(
            &grid_default_cell,
            &mut (*o).c2rust_unnamed.style,
            (*oe).default_str,
        );
    } else {
        (*o).c2rust_unnamed.number = (*oe).default_num;
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_add(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get_only(oo, name);
    if !o.is_null() {
        options_remove(o);
    }
    o = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<options_entry>() as libc::c_ulong,
    ) as *mut options_entry;
    (*o).owner = oo;
    (*o).name = xstrdup(name);
    options_tree_RB_INSERT(&mut (*oo).tree, o);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_remove(mut o: *mut options_entry) {
    let mut oo: *mut options = (*o).owner;
    let mut i: u_int = 0;
    if ((*o).tableentry).is_null()
        || (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint
    {
        free((*o).c2rust_unnamed.string as *mut libc::c_void);
    } else if !((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint
    {
        i = 0 as libc::c_int as u_int;
        while i < (*o).c2rust_unnamed.c2rust_unnamed.arraysize {
            free(
                *((*o).c2rust_unnamed.c2rust_unnamed.array).offset(i as isize)
                    as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
            i;
        }
        free((*o).c2rust_unnamed.c2rust_unnamed.array as *mut libc::c_void);
    }
    options_tree_RB_REMOVE(&mut (*oo).tree, o);
    free(o as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_name(mut o: *mut options_entry) -> *const libc::c_char {
    return (*o).name;
}
#[no_mangle]
pub unsafe extern "C" fn options_table_entry(
    mut o: *mut options_entry,
) -> *const options_table_entry {
    return (*o).tableentry;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_clear(mut o: *mut options_entry) {
    if !((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint
    {
        (*o).c2rust_unnamed.c2rust_unnamed.arraysize = 0 as libc::c_int as u_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn options_array_get(
    mut o: *mut options_entry,
    mut idx: u_int,
) -> *const libc::c_char {
    if !(!((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint)
    {
        return 0 as *const libc::c_char;
    }
    if idx >= (*o).c2rust_unnamed.c2rust_unnamed.arraysize {
        return 0 as *const libc::c_char;
    }
    return *((*o).c2rust_unnamed.c2rust_unnamed.array).offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn options_array_set(
    mut o: *mut options_entry,
    mut idx: u_int,
    mut value: *const libc::c_char,
    mut append: libc::c_int,
) -> libc::c_int {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    if !(!((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    if idx >= 1000 as libc::c_int as u_int {
        return -(1 as libc::c_int);
    }
    if idx >= (*o).c2rust_unnamed.c2rust_unnamed.arraysize {
        (*o)
            .c2rust_unnamed
            .c2rust_unnamed
            .array = xreallocarray(
            (*o).c2rust_unnamed.c2rust_unnamed.array as *mut libc::c_void,
            idx.wrapping_add(1 as libc::c_int as u_int) as size_t,
            ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
        i = (*o).c2rust_unnamed.c2rust_unnamed.arraysize;
        while i < idx.wrapping_add(1 as libc::c_int as u_int) {
            let ref mut fresh0 = *((*o).c2rust_unnamed.c2rust_unnamed.array)
                .offset(i as isize);
            *fresh0 = 0 as *const libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        (*o)
            .c2rust_unnamed
            .c2rust_unnamed
            .arraysize = idx.wrapping_add(1 as libc::c_int as u_int);
    }
    new = 0 as *mut libc::c_char;
    if !value.is_null() {
        if !(*((*o).c2rust_unnamed.c2rust_unnamed.array).offset(idx as isize)).is_null()
            && append != 0
        {
            xasprintf(
                &mut new as *mut *mut libc::c_char,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                *((*o).c2rust_unnamed.c2rust_unnamed.array).offset(idx as isize),
                value,
            );
        } else {
            new = xstrdup(value);
        }
    }
    free(
        *((*o).c2rust_unnamed.c2rust_unnamed.array).offset(idx as isize)
            as *mut libc::c_void,
    );
    let ref mut fresh1 = *((*o).c2rust_unnamed.c2rust_unnamed.array)
        .offset(idx as isize);
    *fresh1 = new;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_size(
    mut o: *mut options_entry,
    mut size: *mut u_int,
) -> libc::c_int {
    if !(!((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    if !size.is_null() {
        *size = (*o).c2rust_unnamed.c2rust_unnamed.arraysize;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_assign(
    mut o: *mut options_entry,
    mut s: *const libc::c_char,
) {
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    separator = (*(*o).tableentry).separator;
    if separator.is_null() {
        separator = b" ,\0" as *const u8 as *const libc::c_char;
    }
    string = xstrdup(s);
    copy = string;
    loop {
        next = strsep(&mut string, separator);
        if next.is_null() {
            break;
        }
        if *next as libc::c_int == '\0' as i32 {
            continue;
        }
        i = 0 as libc::c_int as u_int;
        while i < 1000 as libc::c_int as u_int {
            if i >= (*o).c2rust_unnamed.c2rust_unnamed.arraysize
                || (*((*o).c2rust_unnamed.c2rust_unnamed.array).offset(i as isize))
                    .is_null()
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i == 1000 as libc::c_int as u_int {
            break;
        }
        options_array_set(o, i, next, 0 as libc::c_int);
    }
    free(copy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_isstring(mut o: *mut options_entry) -> libc::c_int {
    if ((*o).tableentry).is_null() {
        return 1 as libc::c_int;
    }
    return (((*o).tableentry).is_null()
        || (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint
        || !((*o).tableentry).is_null()
            && (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn options_tostring(
    mut o: *mut options_entry,
    mut idx: libc::c_int,
    mut numeric: libc::c_int,
) -> *const libc::c_char {
    static mut s: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if !((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_ARRAY as libc::c_int as libc::c_uint
    {
        if idx == -(1 as libc::c_int) {
            return 0 as *const libc::c_char;
        }
        if idx as u_int >= (*o).c2rust_unnamed.c2rust_unnamed.arraysize
            || (*((*o).c2rust_unnamed.c2rust_unnamed.array).offset(idx as isize))
                .is_null()
        {
            return b"\0" as *const u8 as *const libc::c_char;
        }
        return *((*o).c2rust_unnamed.c2rust_unnamed.array).offset(idx as isize);
    }
    if !((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint
    {
        return style_tostring(&mut (*o).c2rust_unnamed.style);
    }
    if !((*o).tableentry).is_null()
        && ((*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_NUMBER as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_KEY as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_COLOUR as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_ATTRIBUTES as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_CHOICE as libc::c_int as libc::c_uint)
    {
        tmp = 0 as *const libc::c_char;
        match (*(*o).tableentry).type_0 as libc::c_uint {
            1 => {
                xsnprintf(
                    s.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    b"%lld\0" as *const u8 as *const libc::c_char,
                    (*o).c2rust_unnamed.number,
                );
            }
            2 => {
                tmp = key_string_lookup_key((*o).c2rust_unnamed.number as key_code);
            }
            3 => {
                tmp = colour_tostring((*o).c2rust_unnamed.number as libc::c_int);
            }
            4 => {
                tmp = attributes_tostring((*o).c2rust_unnamed.number as libc::c_int);
            }
            5 => {
                if numeric != 0 {
                    xsnprintf(
                        s.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"%lld\0" as *const u8 as *const libc::c_char,
                        (*o).c2rust_unnamed.number,
                    );
                } else {
                    tmp = if (*o).c2rust_unnamed.number != 0 {
                        b"on\0" as *const u8 as *const libc::c_char
                    } else {
                        b"off\0" as *const u8 as *const libc::c_char
                    };
                }
            }
            6 => {
                tmp = *((*(*o).tableentry).choices)
                    .offset((*o).c2rust_unnamed.number as isize);
            }
            0 | 7 | 8 | _ => {}
        }
        if !tmp.is_null() {
            xsnprintf(
                s.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                tmp,
            );
        }
        return s.as_mut_ptr();
    }
    if ((*o).tableentry).is_null()
        || (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint
    {
        return (*o).c2rust_unnamed.string;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn options_parse(
    mut name: *const libc::c_char,
    mut idx: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if *name as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    copy = xstrdup(name);
    cp = strchr(copy, '[' as i32);
    if cp.is_null() {
        *idx = -(1 as libc::c_int);
        return copy;
    }
    end = strchr(cp.offset(1 as libc::c_int as isize), ']' as i32);
    if end.is_null()
        || *end.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
        || *(*__ctype_b_loc())
            .offset(
                *end.offset(-(1 as libc::c_int) as isize) as u_char as libc::c_int
                    as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        free(copy as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    if sscanf(cp, b"[%d]\0" as *const u8 as *const libc::c_char, idx) != 1 as libc::c_int
        || *idx < 0 as libc::c_int
    {
        free(copy as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *cp = '\0' as i32 as libc::c_char;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn options_parse_get(
    mut oo: *mut options,
    mut s: *const libc::c_char,
    mut idx: *mut libc::c_int,
    mut only: libc::c_int,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = options_parse(s, idx);
    if name.is_null() {
        return 0 as *mut options_entry;
    }
    if only != 0 {
        o = options_get_only(oo, name);
    } else {
        o = options_get(oo, name);
    }
    free(name as *mut libc::c_void);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_match(
    mut s: *const libc::c_char,
    mut idx: *mut libc::c_int,
    mut ambiguous: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut found: *const options_table_entry = 0 as *const options_table_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namelen: size_t = 0;
    name = options_parse(s, idx);
    if name.is_null() {
        return 0 as *mut libc::c_char;
    }
    namelen = strlen(name);
    if *name as libc::c_int == '@' as i32 {
        *ambiguous = 0 as libc::c_int;
        return name;
    }
    found = 0 as *const options_table_entry;
    oe = options_table.as_ptr();
    while !((*oe).name).is_null() {
        if strcmp((*oe).name, name) == 0 as libc::c_int {
            found = oe;
            break;
        } else {
            if strncmp((*oe).name, name, namelen) == 0 as libc::c_int {
                if !found.is_null() {
                    *ambiguous = 1 as libc::c_int;
                    free(name as *mut libc::c_void);
                    return 0 as *mut libc::c_char;
                }
                found = oe;
            }
            oe = oe.offset(1);
            oe;
        }
    }
    free(name as *mut libc::c_void);
    if found.is_null() {
        *ambiguous = 0 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    return xstrdup((*found).name);
}
#[no_mangle]
pub unsafe extern "C" fn options_match_get(
    mut oo: *mut options,
    mut s: *const libc::c_char,
    mut idx: *mut libc::c_int,
    mut only: libc::c_int,
    mut ambiguous: *mut libc::c_int,
) -> *mut options_entry {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    name = options_match(s, idx, ambiguous);
    if name.is_null() {
        return 0 as *mut options_entry;
    }
    *ambiguous = 0 as libc::c_int;
    if only != 0 {
        o = options_get_only(oo, name);
    } else {
        o = options_get(oo, name);
    }
    free(name as *mut libc::c_void);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_get_string(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o.is_null() {
        fatalx(b"missing option %s\0" as *const u8 as *const libc::c_char, name);
    }
    if !(((*o).tableentry).is_null()
        || (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint)
    {
        fatalx(b"option %s is not a string\0" as *const u8 as *const libc::c_char, name);
    }
    return (*o).c2rust_unnamed.string;
}
#[no_mangle]
pub unsafe extern "C" fn options_get_number(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> libc::c_longlong {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o.is_null() {
        fatalx(b"missing option %s\0" as *const u8 as *const libc::c_char, name);
    }
    if !(!((*o).tableentry).is_null()
        && ((*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_NUMBER as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_KEY as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_COLOUR as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_ATTRIBUTES as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_CHOICE as libc::c_int as libc::c_uint))
    {
        fatalx(b"option %s is not a number\0" as *const u8 as *const libc::c_char, name);
    }
    return (*o).c2rust_unnamed.number;
}
#[no_mangle]
pub unsafe extern "C" fn options_get_style(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *const grid_cell {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o.is_null() {
        fatalx(b"missing option %s\0" as *const u8 as *const libc::c_char, name);
    }
    if !(!((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint)
    {
        fatalx(b"option %s is not a style\0" as *const u8 as *const libc::c_char, name);
    }
    return &mut (*o).c2rust_unnamed.style;
}
#[no_mangle]
pub unsafe extern "C" fn options_set_number(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut value: libc::c_longlong,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if *name as libc::c_int == '@' as i32 {
        fatalx(
            b"user option %s must be a string\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    o = options_get_only(oo, name);
    if o.is_null() {
        o = options_default(oo, options_parent_table_entry(oo, name));
        if o.is_null() {
            return 0 as *mut options_entry;
        }
    }
    if !(!((*o).tableentry).is_null()
        && ((*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_NUMBER as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_KEY as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_COLOUR as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_ATTRIBUTES as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_CHOICE as libc::c_int as libc::c_uint))
    {
        fatalx(b"option %s is not a number\0" as *const u8 as *const libc::c_char, name);
    }
    (*o).c2rust_unnamed.number = value;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_set_style(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut append: libc::c_int,
    mut value: *const libc::c_char,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut gc: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    if *name as libc::c_int == '@' as i32 {
        fatalx(
            b"user option %s must be a string\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    o = options_get_only(oo, name);
    if !o.is_null() && append != 0
        && (!((*o).tableentry).is_null()
            && (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint)
    {
        memcpy(
            &mut gc as *mut grid_cell as *mut libc::c_void,
            &mut (*o).c2rust_unnamed.style as *mut grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
    } else {
        memcpy(
            &mut gc as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
    }
    if style_parse(&grid_default_cell, &mut gc, value) == -(1 as libc::c_int) {
        return 0 as *mut options_entry;
    }
    if o.is_null() {
        o = options_default(oo, options_parent_table_entry(oo, name));
        if o.is_null() {
            return 0 as *mut options_entry;
        }
    }
    if !(!((*o).tableentry).is_null()
        && (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STYLE as libc::c_int as libc::c_uint)
    {
        fatalx(b"option %s is not a style\0" as *const u8 as *const libc::c_char, name);
    }
    memcpy(
        &mut (*o).c2rust_unnamed.style as *mut grid_cell as *mut libc::c_void,
        &mut gc as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_scope_from_flags(
    mut args: *mut args,
    mut window: libc::c_int,
    mut fs: *mut cmd_find_state,
    mut oo: *mut *mut options,
    mut cause: *mut *mut libc::c_char,
) -> options_table_scope {
    let mut s: *mut session = (*fs).s;
    let mut wl: *mut winlink = (*fs).wl;
    let mut target: *const libc::c_char = args_get(args, 't' as i32 as u_char);
    if args_has(args, 's' as i32 as u_char) != 0 {
        *oo = global_options;
        return OPTIONS_TABLE_SERVER;
    }
    if window != 0 || args_has(args, 'w' as i32 as u_char) != 0 {
        if args_has(args, 'g' as i32 as u_char) != 0 {
            *oo = global_w_options;
            return OPTIONS_TABLE_WINDOW;
        }
        if wl.is_null() {
            if !target.is_null() {
                xasprintf(
                    cause,
                    b"no such window: %s\0" as *const u8 as *const libc::c_char,
                    target,
                );
            } else {
                xasprintf(
                    cause,
                    b"no current window\0" as *const u8 as *const libc::c_char,
                );
            }
            return OPTIONS_TABLE_NONE;
        }
        *oo = (*(*wl).window).options;
        return OPTIONS_TABLE_WINDOW;
    } else {
        if args_has(args, 'g' as i32 as u_char) != 0 {
            *oo = global_s_options;
            return OPTIONS_TABLE_SESSION;
        }
        if s.is_null() {
            if !target.is_null() {
                xasprintf(
                    cause,
                    b"no such session: %s\0" as *const u8 as *const libc::c_char,
                    target,
                );
            } else {
                xasprintf(
                    cause,
                    b"no current session\0" as *const u8 as *const libc::c_char,
                );
            }
            return OPTIONS_TABLE_NONE;
        }
        *oo = (*s).options;
        return OPTIONS_TABLE_SESSION;
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_style_update_new(
    mut oo: *mut options,
    mut o: *mut options_entry,
) {
    let mut newname: *const libc::c_char = (*(*o).tableentry).style;
    let mut new: *mut options_entry = 0 as *mut options_entry;
    if newname.is_null() {
        return;
    }
    new = options_get_only(oo, newname);
    if new.is_null() {
        new = options_set_style(
            oo,
            newname,
            0 as libc::c_int,
            b"default\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(strstr((*o).name, b"-bg\0" as *const u8 as *const libc::c_char)).is_null() {
        (*new).c2rust_unnamed.style.bg = (*o).c2rust_unnamed.number as libc::c_int;
    } else if !(strstr((*o).name, b"-fg\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        (*new).c2rust_unnamed.style.fg = (*o).c2rust_unnamed.number as libc::c_int;
    } else if !(strstr((*o).name, b"-attr\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        (*new).c2rust_unnamed.style.attr = (*o).c2rust_unnamed.number as u_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn options_style_update_old(
    mut oo: *mut options,
    mut o: *mut options_entry,
) {
    let mut newname: [libc::c_char; 128] = [0; 128];
    let mut size: libc::c_int = 0;
    size = (strrchr((*o).name, '-' as i32)).offset_from((*o).name) as libc::c_long
        as libc::c_int;
    xsnprintf(
        newname.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%.*s-bg\0" as *const u8 as *const libc::c_char,
        size,
        (*o).name,
    );
    if !(options_get(oo, newname.as_mut_ptr())).is_null() {
        options_set_number(
            oo,
            newname.as_mut_ptr(),
            (*o).c2rust_unnamed.style.bg as libc::c_longlong,
        );
    }
    xsnprintf(
        newname.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%.*s-fg\0" as *const u8 as *const libc::c_char,
        size,
        (*o).name,
    );
    if !(options_get(oo, newname.as_mut_ptr())).is_null() {
        options_set_number(
            oo,
            newname.as_mut_ptr(),
            (*o).c2rust_unnamed.style.fg as libc::c_longlong,
        );
    }
    xsnprintf(
        newname.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%.*s-attr\0" as *const u8 as *const libc::c_char,
        size,
        (*o).name,
    );
    if !(options_get(oo, newname.as_mut_ptr())).is_null() {
        options_set_number(
            oo,
            newname.as_mut_ptr(),
            (*o).c2rust_unnamed.style.attr as libc::c_longlong,
        );
    }
}
