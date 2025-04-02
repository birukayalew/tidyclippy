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
    pub type screen_write_collect_line;
    pub type screen_write_collect_item;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn cmd_template_replace(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn cmd_list_free(_: *mut cmd_list);
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut cmd_find_state,
        _: *mut mouse_event,
        _: libc::c_int,
    ) -> *mut cmdq_item;
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn cmd_string_parse(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut cmd_list;
    fn server_redraw_window(_: *mut window);
    fn server_unzoom_window(_: *mut window);
    fn status_message_set(_: *mut client, _: *const libc::c_char, _: ...);
    fn status_prompt_set(
        _: *mut client,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: prompt_input_cb,
        _: prompt_free_cb,
        _: *mut libc::c_void,
        _: libc::c_int,
    );
    static grid_default_cell: grid_cell;
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    fn screen_write_stop(_: *mut screen_write_ctx);
    fn screen_write_puts(
        _: *mut screen_write_ctx,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    );
    fn screen_write_nputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    );
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int);
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn screen_free(_: *mut screen);
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    fn window_zoom(_: *mut window_pane) -> libc::c_int;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
pub struct mode_tree_data {
    pub dead: libc::c_int,
    pub references: u_int,
    pub zoomed: libc::c_int,
    pub wp: *mut window_pane,
    pub modedata: *mut libc::c_void,
    pub sort_list: *mut *const libc::c_char,
    pub sort_size: u_int,
    pub sort_type: u_int,
    pub buildcb: mode_tree_build_cb,
    pub drawcb: mode_tree_draw_cb,
    pub searchcb: mode_tree_search_cb,
    pub children: mode_tree_list,
    pub saved: mode_tree_list,
    pub line_list: *mut mode_tree_line,
    pub line_size: u_int,
    pub depth: u_int,
    pub width: u_int,
    pub height: u_int,
    pub offset: u_int,
    pub current: u_int,
    pub screen: screen,
    pub preview: libc::c_int,
    pub search: *mut libc::c_char,
    pub filter: *mut libc::c_char,
    pub no_matches: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_tree_line {
    pub item: *mut mode_tree_item,
    pub depth: u_int,
    pub last: libc::c_int,
    pub flat: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_tree_item {
    pub parent: *mut mode_tree_item,
    pub itemdata: *mut libc::c_void,
    pub line: u_int,
    pub tag: uint64_t,
    pub name: *const libc::c_char,
    pub text: *const libc::c_char,
    pub expanded: libc::c_int,
    pub tagged: libc::c_int,
    pub children: mode_tree_list,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub tqe_next: *mut mode_tree_item,
    pub tqe_prev: *mut *mut mode_tree_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_tree_list {
    pub tqh_first: *mut mode_tree_item,
    pub tqh_last: *mut *mut mode_tree_item,
}
pub type mode_tree_search_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub type mode_tree_draw_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut screen_write_ctx,
        u_int,
        u_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub item: *mut screen_write_collect_item,
    pub list: *mut screen_write_collect_line,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type mode_tree_build_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        u_int,
        *mut uint64_t,
        *const libc::c_char,
    ) -> (),
>;
pub type C2RustUnnamed_35 = libc::c_uint;
pub const KEYC_KP_PERIOD: C2RustUnnamed_35 = 268435564;
pub const KEYC_KP_ZERO: C2RustUnnamed_35 = 268435563;
pub const KEYC_KP_ENTER: C2RustUnnamed_35 = 268435562;
pub const KEYC_KP_THREE: C2RustUnnamed_35 = 268435561;
pub const KEYC_KP_TWO: C2RustUnnamed_35 = 268435560;
pub const KEYC_KP_ONE: C2RustUnnamed_35 = 268435559;
pub const KEYC_KP_SIX: C2RustUnnamed_35 = 268435558;
pub const KEYC_KP_FIVE: C2RustUnnamed_35 = 268435557;
pub const KEYC_KP_FOUR: C2RustUnnamed_35 = 268435556;
pub const KEYC_KP_PLUS: C2RustUnnamed_35 = 268435555;
pub const KEYC_KP_NINE: C2RustUnnamed_35 = 268435554;
pub const KEYC_KP_EIGHT: C2RustUnnamed_35 = 268435553;
pub const KEYC_KP_SEVEN: C2RustUnnamed_35 = 268435552;
pub const KEYC_KP_MINUS: C2RustUnnamed_35 = 268435551;
pub const KEYC_KP_STAR: C2RustUnnamed_35 = 268435550;
pub const KEYC_KP_SLASH: C2RustUnnamed_35 = 268435549;
pub const KEYC_RIGHT: C2RustUnnamed_35 = 268435548;
pub const KEYC_LEFT: C2RustUnnamed_35 = 268435547;
pub const KEYC_DOWN: C2RustUnnamed_35 = 268435546;
pub const KEYC_UP: C2RustUnnamed_35 = 268435545;
pub const KEYC_BTAB: C2RustUnnamed_35 = 268435544;
pub const KEYC_PPAGE: C2RustUnnamed_35 = 268435543;
pub const KEYC_NPAGE: C2RustUnnamed_35 = 268435542;
pub const KEYC_END: C2RustUnnamed_35 = 268435541;
pub const KEYC_HOME: C2RustUnnamed_35 = 268435540;
pub const KEYC_DC: C2RustUnnamed_35 = 268435539;
pub const KEYC_IC: C2RustUnnamed_35 = 268435538;
pub const KEYC_F12: C2RustUnnamed_35 = 268435537;
pub const KEYC_F11: C2RustUnnamed_35 = 268435536;
pub const KEYC_F10: C2RustUnnamed_35 = 268435535;
pub const KEYC_F9: C2RustUnnamed_35 = 268435534;
pub const KEYC_F8: C2RustUnnamed_35 = 268435533;
pub const KEYC_F7: C2RustUnnamed_35 = 268435532;
pub const KEYC_F6: C2RustUnnamed_35 = 268435531;
pub const KEYC_F5: C2RustUnnamed_35 = 268435530;
pub const KEYC_F4: C2RustUnnamed_35 = 268435529;
pub const KEYC_F3: C2RustUnnamed_35 = 268435528;
pub const KEYC_F2: C2RustUnnamed_35 = 268435527;
pub const KEYC_F1: C2RustUnnamed_35 = 268435526;
pub const KEYC_BSPACE: C2RustUnnamed_35 = 268435525;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed_35 = 268435524;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed_35 = 268435523;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed_35 = 268435522;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed_35 = 268435521;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed_35 = 268435520;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed_35 = 268435519;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed_35 = 268435518;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed_35 = 268435517;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed_35 = 268435516;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed_35 = 268435515;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed_35 = 268435514;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed_35 = 268435513;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed_35 = 268435512;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed_35 = 268435511;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed_35 = 268435510;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed_35 = 268435509;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed_35 = 268435508;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed_35 = 268435507;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed_35 = 268435506;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed_35 = 268435505;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed_35 = 268435504;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed_35 = 268435503;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed_35 = 268435502;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed_35 = 268435501;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed_35 = 268435500;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed_35 = 268435499;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed_35 = 268435498;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed_35 = 268435497;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed_35 = 268435496;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed_35 = 268435495;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed_35 = 268435494;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed_35 = 268435493;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed_35 = 268435492;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed_35 = 268435491;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed_35 = 268435490;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed_35 = 268435489;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed_35 = 268435488;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed_35 = 268435487;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed_35 = 268435486;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed_35 = 268435485;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed_35 = 268435484;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed_35 = 268435483;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed_35 = 268435482;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed_35 = 268435481;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed_35 = 268435480;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed_35 = 268435479;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed_35 = 268435478;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed_35 = 268435477;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed_35 = 268435476;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed_35 = 268435475;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed_35 = 268435474;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed_35 = 268435473;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed_35 = 268435472;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed_35 = 268435471;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed_35 = 268435470;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed_35 = 268435469;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed_35 = 268435468;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed_35 = 268435467;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed_35 = 268435466;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed_35 = 268435465;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed_35 = 268435464;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed_35 = 268435463;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed_35 = 268435462;
pub const KEYC_DRAGGING: C2RustUnnamed_35 = 268435461;
pub const KEYC_MOUSE: C2RustUnnamed_35 = 268435460;
pub const KEYC_PASTE_END: C2RustUnnamed_35 = 268435459;
pub const KEYC_PASTE_START: C2RustUnnamed_35 = 268435458;
pub const KEYC_FOCUS_OUT: C2RustUnnamed_35 = 268435457;
pub const KEYC_FOCUS_IN: C2RustUnnamed_35 = 268435456;
pub type mode_tree_each_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut client,
        key_code,
    ) -> (),
>;
unsafe extern "C" fn mode_tree_find_item(
    mut mtl: *mut mode_tree_list,
    mut tag: uint64_t,
) -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut child: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        if (*mti).tag == tag {
            return mti;
        }
        child = mode_tree_find_item(&mut (*mti).children, tag);
        if !child.is_null() {
            return child;
        }
        mti = (*mti).entry.tqe_next;
    }
    return 0 as *mut mode_tree_item;
}
unsafe extern "C" fn mode_tree_free_item(mut mti: *mut mode_tree_item) {
    mode_tree_free_items(&mut (*mti).children);
    free((*mti).name as *mut libc::c_void);
    free((*mti).text as *mut libc::c_void);
    free(mti as *mut libc::c_void);
}
unsafe extern "C" fn mode_tree_free_items(mut mtl: *mut mode_tree_list) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut mti1: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while !mti.is_null()
        && {
            mti1 = (*mti).entry.tqe_next;
            1 as libc::c_int != 0
        }
    {
        if !((*mti).entry.tqe_next).is_null() {
            (*(*mti).entry.tqe_next).entry.tqe_prev = (*mti).entry.tqe_prev;
        } else {
            (*mtl).tqh_last = (*mti).entry.tqe_prev;
        }
        *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
        mode_tree_free_item(mti);
        mti = mti1;
    }
}
unsafe extern "C" fn mode_tree_check_selected(mut mtd: *mut mode_tree_data) {
    if (*mtd).current > ((*mtd).height).wrapping_sub(1 as libc::c_int as u_int) {
        (*mtd)
            .offset = ((*mtd).current)
            .wrapping_sub((*mtd).height)
            .wrapping_add(1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn mode_tree_clear_lines(mut mtd: *mut mode_tree_data) {
    free((*mtd).line_list as *mut libc::c_void);
    (*mtd).line_list = 0 as *mut mode_tree_line;
    (*mtd).line_size = 0 as libc::c_int as u_int;
}
unsafe extern "C" fn mode_tree_build_lines(
    mut mtd: *mut mode_tree_data,
    mut mtl: *mut mode_tree_list,
    mut depth: u_int,
) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut i: u_int = 0;
    let mut flat: libc::c_int = 1 as libc::c_int;
    (*mtd).depth = depth;
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        (*mtd)
            .line_list = xreallocarray(
            (*mtd).line_list as *mut libc::c_void,
            ((*mtd).line_size).wrapping_add(1 as libc::c_int as u_int) as size_t,
            ::core::mem::size_of::<mode_tree_line>() as libc::c_ulong,
        ) as *mut mode_tree_line;
        let fresh0 = (*mtd).line_size;
        (*mtd).line_size = ((*mtd).line_size).wrapping_add(1);
        line = &mut *((*mtd).line_list).offset(fresh0 as isize) as *mut mode_tree_line;
        (*line).item = mti;
        (*line).depth = depth;
        (*line)
            .last = (mti == *(*((*mtl).tqh_last as *mut mode_tree_list)).tqh_last)
            as libc::c_int;
        (*mti).line = ((*mtd).line_size).wrapping_sub(1 as libc::c_int as u_int);
        if !((*mti).children.tqh_first).is_null() {
            flat = 0 as libc::c_int;
        }
        if (*mti).expanded != 0 {
            mode_tree_build_lines(
                mtd,
                &mut (*mti).children,
                depth.wrapping_add(1 as libc::c_int as u_int),
            );
        }
        mti = (*mti).entry.tqe_next;
    }
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        i = 0 as libc::c_int as u_int;
        while i < (*mtd).line_size {
            line = &mut *((*mtd).line_list).offset(i as isize) as *mut mode_tree_line;
            if (*line).item == mti {
                (*line).flat = flat;
            }
            i = i.wrapping_add(1);
            i;
        }
        mti = (*mti).entry.tqe_next;
    }
}
unsafe extern "C" fn mode_tree_clear_tagged(mut mtl: *mut mode_tree_list) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        (*mti).tagged = 0 as libc::c_int;
        mode_tree_clear_tagged(&mut (*mti).children);
        mti = (*mti).entry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_up(
    mut mtd: *mut mode_tree_data,
    mut wrap: libc::c_int,
) {
    if (*mtd).current == 0 as libc::c_int as u_int {
        if wrap != 0 {
            (*mtd).current = ((*mtd).line_size).wrapping_sub(1 as libc::c_int as u_int);
            if (*mtd).line_size >= (*mtd).height {
                (*mtd).offset = ((*mtd).line_size).wrapping_sub((*mtd).height);
            }
        }
    } else {
        (*mtd).current = ((*mtd).current).wrapping_sub(1);
        (*mtd).current;
        if (*mtd).current < (*mtd).offset {
            (*mtd).offset = ((*mtd).offset).wrapping_sub(1);
            (*mtd).offset;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_down(
    mut mtd: *mut mode_tree_data,
    mut wrap: libc::c_int,
) {
    if (*mtd).current == ((*mtd).line_size).wrapping_sub(1 as libc::c_int as u_int) {
        if wrap != 0 {
            (*mtd).current = 0 as libc::c_int as u_int;
            (*mtd).offset = 0 as libc::c_int as u_int;
        }
    } else {
        (*mtd).current = ((*mtd).current).wrapping_add(1);
        (*mtd).current;
        if (*mtd).current
            > ((*mtd).offset)
                .wrapping_add((*mtd).height)
                .wrapping_sub(1 as libc::c_int as u_int)
        {
            (*mtd).offset = ((*mtd).offset).wrapping_add(1);
            (*mtd).offset;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_get_current(
    mut mtd: *mut mode_tree_data,
) -> *mut libc::c_void {
    return (*(*((*mtd).line_list).offset((*mtd).current as isize)).item).itemdata;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_expand_current(mut mtd: *mut mode_tree_data) {
    if (*(*((*mtd).line_list).offset((*mtd).current as isize)).item).expanded == 0 {
        (*(*((*mtd).line_list).offset((*mtd).current as isize)).item)
            .expanded = 1 as libc::c_int;
        mode_tree_build(mtd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_set_current(
    mut mtd: *mut mode_tree_data,
    mut tag: uint64_t,
) {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*mtd).line_size {
        if (*(*((*mtd).line_list).offset(i as isize)).item).tag == tag {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i != (*mtd).line_size {
        (*mtd).current = i;
        if (*mtd).current > ((*mtd).height).wrapping_sub(1 as libc::c_int as u_int) {
            (*mtd)
                .offset = ((*mtd).current)
                .wrapping_sub((*mtd).height)
                .wrapping_add(1 as libc::c_int as u_int);
        } else {
            (*mtd).offset = 0 as libc::c_int as u_int;
        }
    } else {
        (*mtd).current = 0 as libc::c_int as u_int;
        (*mtd).offset = 0 as libc::c_int as u_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_count_tagged(mut mtd: *mut mode_tree_data) -> u_int {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut tagged: u_int = 0;
    tagged = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as u_int;
    while i < (*mtd).line_size {
        mti = (*((*mtd).line_list).offset(i as isize)).item;
        if (*mti).tagged != 0 {
            tagged = tagged.wrapping_add(1);
            tagged;
        }
        i = i.wrapping_add(1);
        i;
    }
    return tagged;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_each_tagged(
    mut mtd: *mut mode_tree_data,
    mut cb: mode_tree_each_cb,
    mut c: *mut client,
    mut key: key_code,
    mut current: libc::c_int,
) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut fired: libc::c_int = 0;
    fired = 0 as libc::c_int;
    i = 0 as libc::c_int as u_int;
    while i < (*mtd).line_size {
        mti = (*((*mtd).line_list).offset(i as isize)).item;
        if (*mti).tagged != 0 {
            fired = 1 as libc::c_int;
            cb
                .expect(
                    "non-null function pointer",
                )((*mtd).modedata, (*mti).itemdata, c, key);
        }
        i = i.wrapping_add(1);
        i;
    }
    if fired == 0 && current != 0 {
        mti = (*((*mtd).line_list).offset((*mtd).current as isize)).item;
        cb.expect("non-null function pointer")((*mtd).modedata, (*mti).itemdata, c, key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_start(
    mut wp: *mut window_pane,
    mut args: *mut args,
    mut buildcb: mode_tree_build_cb,
    mut drawcb: mode_tree_draw_cb,
    mut searchcb: mode_tree_search_cb,
    mut modedata: *mut libc::c_void,
    mut sort_list: *mut *const libc::c_char,
    mut sort_size: u_int,
    mut s: *mut *mut screen,
) -> *mut mode_tree_data {
    let mut mtd: *mut mode_tree_data = 0 as *mut mode_tree_data;
    let mut sort: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    mtd = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<mode_tree_data>() as libc::c_ulong,
    ) as *mut mode_tree_data;
    (*mtd).references = 1 as libc::c_int as u_int;
    (*mtd).wp = wp;
    (*mtd).modedata = modedata;
    (*mtd).sort_list = sort_list;
    (*mtd).sort_size = sort_size;
    (*mtd).sort_type = 0 as libc::c_int as u_int;
    (*mtd).preview = (args_has(args, 'N' as i32 as u_char) == 0) as libc::c_int;
    sort = args_get(args, 'O' as i32 as u_char);
    if !sort.is_null() {
        i = 0 as libc::c_int as u_int;
        while i < sort_size {
            if strcasecmp(sort, *sort_list.offset(i as isize)) == 0 as libc::c_int {
                (*mtd).sort_type = i;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if args_has(args, 'f' as i32 as u_char) != 0 {
        (*mtd).filter = xstrdup(args_get(args, 'f' as i32 as u_char));
    } else {
        (*mtd).filter = 0 as *mut libc::c_char;
    }
    (*mtd).buildcb = buildcb;
    (*mtd).drawcb = drawcb;
    (*mtd).searchcb = searchcb;
    (*mtd).children.tqh_first = 0 as *mut mode_tree_item;
    (*mtd).children.tqh_last = &mut (*mtd).children.tqh_first;
    *s = &mut (*mtd).screen;
    screen_init(
        *s,
        (*(*wp).base.grid).sx,
        (*(*wp).base.grid).sy,
        0 as libc::c_int as u_int,
    );
    (**s).mode &= !(0x1 as libc::c_int);
    return mtd;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_zoom(
    mut mtd: *mut mode_tree_data,
    mut args: *mut args,
) {
    let mut wp: *mut window_pane = (*mtd).wp;
    if args_has(args, 'Z' as i32 as u_char) != 0 {
        (*mtd).zoomed = (*(*wp).window).flags & 0x1000 as libc::c_int;
        if (*mtd).zoomed == 0 && window_zoom(wp) == 0 as libc::c_int {
            server_redraw_window((*wp).window);
        }
    } else {
        (*mtd).zoomed = -(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_build(mut mtd: *mut mode_tree_data) {
    let mut s: *mut screen = &mut (*mtd).screen;
    let mut tag: uint64_t = 0;
    if !((*mtd).line_list).is_null() {
        tag = (*(*((*mtd).line_list).offset((*mtd).current as isize)).item).tag;
    } else {
        tag = 0 as libc::c_int as uint64_t;
    }
    if !((*mtd).children.tqh_first).is_null() {
        *(*mtd).saved.tqh_last = (*mtd).children.tqh_first;
        (*(*mtd).children.tqh_first).entry.tqe_prev = (*mtd).saved.tqh_last;
        (*mtd).saved.tqh_last = (*mtd).children.tqh_last;
        (*mtd).children.tqh_first = 0 as *mut mode_tree_item;
        (*mtd).children.tqh_last = &mut (*mtd).children.tqh_first;
    }
    (*mtd).children.tqh_first = 0 as *mut mode_tree_item;
    (*mtd).children.tqh_last = &mut (*mtd).children.tqh_first;
    ((*mtd).buildcb)
        .expect(
            "non-null function pointer",
        )((*mtd).modedata, (*mtd).sort_type, &mut tag, (*mtd).filter);
    (*mtd)
        .no_matches = ((*mtd).children.tqh_first
        == 0 as *mut libc::c_void as *mut mode_tree_item) as libc::c_int;
    if (*mtd).no_matches != 0 {
        ((*mtd).buildcb)
            .expect(
                "non-null function pointer",
            )((*mtd).modedata, (*mtd).sort_type, &mut tag, 0 as *const libc::c_char);
    }
    mode_tree_free_items(&mut (*mtd).saved);
    (*mtd).saved.tqh_first = 0 as *mut mode_tree_item;
    (*mtd).saved.tqh_last = &mut (*mtd).saved.tqh_first;
    mode_tree_clear_lines(mtd);
    mode_tree_build_lines(mtd, &mut (*mtd).children, 0 as libc::c_int as u_int);
    mode_tree_set_current(mtd, tag);
    (*mtd).width = (*(*s).grid).sx;
    if (*mtd).preview != 0 {
        (*mtd)
            .height = (*(*s).grid).sy / 3 as libc::c_int as u_int
            * 2 as libc::c_int as u_int;
        if (*mtd).height > (*mtd).line_size {
            (*mtd).height = (*(*s).grid).sy / 2 as libc::c_int as u_int;
        }
        if (*mtd).height < 10 as libc::c_int as u_int {
            (*mtd).height = (*(*s).grid).sy;
        }
        if ((*(*s).grid).sy).wrapping_sub((*mtd).height) < 2 as libc::c_int as u_int {
            (*mtd).height = (*(*s).grid).sy;
        }
    } else {
        (*mtd).height = (*(*s).grid).sy;
    }
    mode_tree_check_selected(mtd);
}
unsafe extern "C" fn mode_tree_remove_ref(mut mtd: *mut mode_tree_data) {
    (*mtd).references = ((*mtd).references).wrapping_sub(1);
    if (*mtd).references == 0 as libc::c_int as u_int {
        free(mtd as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_free(mut mtd: *mut mode_tree_data) {
    let mut wp: *mut window_pane = (*mtd).wp;
    if (*mtd).zoomed == 0 as libc::c_int {
        server_unzoom_window((*wp).window);
    }
    mode_tree_free_items(&mut (*mtd).children);
    mode_tree_clear_lines(mtd);
    screen_free(&mut (*mtd).screen);
    free((*mtd).search as *mut libc::c_void);
    free((*mtd).filter as *mut libc::c_void);
    (*mtd).dead = 1 as libc::c_int;
    mode_tree_remove_ref(mtd);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_resize(
    mut mtd: *mut mode_tree_data,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut s: *mut screen = &mut (*mtd).screen;
    screen_resize(s, sx, sy, 0 as libc::c_int);
    mode_tree_build(mtd);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_add(
    mut mtd: *mut mode_tree_data,
    mut parent: *mut mode_tree_item,
    mut itemdata: *mut libc::c_void,
    mut tag: uint64_t,
    mut name: *const libc::c_char,
    mut text: *const libc::c_char,
    mut expanded: libc::c_int,
) -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut saved: *mut mode_tree_item = 0 as *mut mode_tree_item;
    log_debug(
        b"%s: %llu, %s %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"mode_tree_add\0"))
            .as_ptr(),
        tag as libc::c_ulonglong,
        name,
        text,
    );
    mti = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<mode_tree_item>() as libc::c_ulong,
    ) as *mut mode_tree_item;
    (*mti).parent = parent;
    (*mti).itemdata = itemdata;
    (*mti).tag = tag;
    (*mti).name = xstrdup(name);
    (*mti).text = xstrdup(text);
    saved = mode_tree_find_item(&mut (*mtd).saved, tag);
    if !saved.is_null() {
        if parent.is_null() || !parent.is_null() && (*parent).expanded != 0 {
            (*mti).tagged = (*saved).tagged;
        }
        (*mti).expanded = (*saved).expanded;
    } else if expanded == -(1 as libc::c_int) {
        (*mti).expanded = 1 as libc::c_int;
    } else {
        (*mti).expanded = expanded;
    }
    (*mti).children.tqh_first = 0 as *mut mode_tree_item;
    (*mti).children.tqh_last = &mut (*mti).children.tqh_first;
    if !parent.is_null() {
        (*mti).entry.tqe_next = 0 as *mut mode_tree_item;
        (*mti).entry.tqe_prev = (*parent).children.tqh_last;
        *(*parent).children.tqh_last = mti;
        (*parent).children.tqh_last = &mut (*mti).entry.tqe_next;
    } else {
        (*mti).entry.tqe_next = 0 as *mut mode_tree_item;
        (*mti).entry.tqe_prev = (*mtd).children.tqh_last;
        *(*mtd).children.tqh_last = mti;
        (*mtd).children.tqh_last = &mut (*mti).entry.tqe_next;
    }
    return mti;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_remove(
    mut mtd: *mut mode_tree_data,
    mut mti: *mut mode_tree_item,
) {
    let mut parent: *mut mode_tree_item = (*mti).parent;
    if !parent.is_null() {
        if !((*mti).entry.tqe_next).is_null() {
            (*(*mti).entry.tqe_next).entry.tqe_prev = (*mti).entry.tqe_prev;
        } else {
            (*parent).children.tqh_last = (*mti).entry.tqe_prev;
        }
        *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
    } else {
        if !((*mti).entry.tqe_next).is_null() {
            (*(*mti).entry.tqe_next).entry.tqe_prev = (*mti).entry.tqe_prev;
        } else {
            (*mtd).children.tqh_last = (*mti).entry.tqe_prev;
        }
        *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
    }
    mode_tree_free_item(mti);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_draw(mut mtd: *mut mode_tree_data) {
    let mut wp: *mut window_pane = (*mtd).wp;
    let mut s: *mut screen = &mut (*mtd).screen;
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut oo: *mut options = (*(*wp).window).options;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        item: 0 as *mut screen_write_collect_item,
        list: 0 as *mut screen_write_collect_line,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut gc0: grid_cell = grid_cell {
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
    let mut w: u_int = 0;
    let mut h: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut sy: u_int = 0;
    let mut box_x: u_int = 0;
    let mut box_y: u_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 7] = [0; 7];
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut symbol: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut keylen: libc::c_int = 0;
    if (*mtd).line_size == 0 as libc::c_int as u_int {
        return;
    }
    memcpy(
        &mut gc0 as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    style_apply(&mut gc, oo, b"mode-style\0" as *const u8 as *const libc::c_char);
    w = (*mtd).width;
    h = (*mtd).height;
    screen_write_start(&mut ctx, 0 as *mut window_pane, s);
    screen_write_clearscreen(&mut ctx, 8 as libc::c_int as u_int);
    if (*mtd).line_size > 10 as libc::c_int as u_int {
        keylen = 6 as libc::c_int;
    } else {
        keylen = 4 as libc::c_int;
    }
    i = 0 as libc::c_int as u_int;
    while i < (*mtd).line_size {
        if !(i < (*mtd).offset) {
            if i
                > ((*mtd).offset).wrapping_add(h).wrapping_sub(1 as libc::c_int as u_int)
            {
                break;
            }
            line = &mut *((*mtd).line_list).offset(i as isize) as *mut mode_tree_line;
            mti = (*line).item;
            screen_write_cursormove(
                &mut ctx,
                0 as libc::c_int as u_int,
                i.wrapping_sub((*mtd).offset),
            );
            if i < 10 as libc::c_int as u_int {
                snprintf(
                    key.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
                    b"(%c)  \0" as *const u8 as *const libc::c_char,
                    ('0' as i32 as u_int).wrapping_add(i),
                );
            } else if i < 36 as libc::c_int as u_int {
                snprintf(
                    key.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
                    b"(M-%c)\0" as *const u8 as *const libc::c_char,
                    ('a' as i32 as u_int)
                        .wrapping_add(i.wrapping_sub(10 as libc::c_int as u_int)),
                );
            } else {
                *key.as_mut_ptr() = '\0' as i32 as libc::c_char;
            }
            if (*line).flat != 0 {
                symbol = b"\0" as *const u8 as *const libc::c_char;
            } else if ((*mti).children.tqh_first).is_null() {
                symbol = b"  \0" as *const u8 as *const libc::c_char;
            } else if (*mti).expanded != 0 {
                symbol = b"- \0" as *const u8 as *const libc::c_char;
            } else {
                symbol = b"+ \0" as *const u8 as *const libc::c_char;
            }
            if (*line).depth == 0 as libc::c_int as u_int {
                start = xstrdup(symbol);
            } else {
                size = (4 as libc::c_int as u_int * (*line).depth)
                    .wrapping_add(32 as libc::c_int as u_int) as size_t;
                start = xcalloc(1 as libc::c_int as size_t, size) as *mut libc::c_char;
                j = 1 as libc::c_int as u_int;
                while j < (*line).depth {
                    if !((*mti).parent).is_null()
                        && (*((*mtd).line_list).offset((*(*mti).parent).line as isize))
                            .last != 0
                    {
                        strlcat(
                            start,
                            b"    \0" as *const u8 as *const libc::c_char,
                            size,
                        );
                    } else {
                        strlcat(
                            start,
                            b"\x01x\x01   \0" as *const u8 as *const libc::c_char,
                            size,
                        );
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                if (*line).last != 0 {
                    strlcat(
                        start,
                        b"\x01mq\x01> \0" as *const u8 as *const libc::c_char,
                        size,
                    );
                } else {
                    strlcat(
                        start,
                        b"\x01tq\x01> \0" as *const u8 as *const libc::c_char,
                        size,
                    );
                }
                strlcat(start, symbol, size);
            }
            if (*mti).tagged != 0 {
                tag = b"*\0" as *const u8 as *const libc::c_char;
            } else {
                tag = b"\0" as *const u8 as *const libc::c_char;
            }
            xasprintf(
                &mut text as *mut *mut libc::c_char,
                b"%-*s%s%s%s: %s\0" as *const u8 as *const libc::c_char,
                keylen,
                key.as_mut_ptr(),
                start,
                (*mti).name,
                tag,
                (*mti).text,
            );
            free(start as *mut libc::c_void);
            if (*mti).tagged != 0 {
                gc.attr = (gc.attr as libc::c_int ^ 0x1 as libc::c_int) as u_short;
                gc0.attr = (gc0.attr as libc::c_int ^ 0x1 as libc::c_int) as u_short;
            }
            if i != (*mtd).current {
                screen_write_nputs(
                    &mut ctx as *mut screen_write_ctx,
                    w as ssize_t,
                    &mut gc0 as *mut grid_cell,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    text,
                );
                screen_write_clearendofline(&mut ctx, 8 as libc::c_int as u_int);
            } else {
                screen_write_nputs(
                    &mut ctx as *mut screen_write_ctx,
                    w as ssize_t,
                    &mut gc as *mut grid_cell,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    text,
                );
                screen_write_clearendofline(&mut ctx, gc.bg as u_int);
            }
            free(text as *mut libc::c_void);
            if (*mti).tagged != 0 {
                gc.attr = (gc.attr as libc::c_int ^ 0x1 as libc::c_int) as u_short;
                gc0.attr = (gc0.attr as libc::c_int ^ 0x1 as libc::c_int) as u_short;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    sy = (*(*s).grid).sy;
    if (*mtd).preview == 0 || sy <= 4 as libc::c_int as u_int
        || h <= 4 as libc::c_int as u_int
        || sy.wrapping_sub(h) <= 4 as libc::c_int as u_int
        || w <= 4 as libc::c_int as u_int
    {
        screen_write_stop(&mut ctx);
        return;
    }
    line = &mut *((*mtd).line_list).offset((*mtd).current as isize)
        as *mut mode_tree_line;
    mti = (*line).item;
    screen_write_cursormove(&mut ctx, 0 as libc::c_int as u_int, h);
    screen_write_box(&mut ctx, w, sy.wrapping_sub(h));
    xasprintf(
        &mut text as *mut *mut libc::c_char,
        b" %s (sort: %s)\0" as *const u8 as *const libc::c_char,
        (*mti).name,
        *((*mtd).sort_list).offset((*mtd).sort_type as isize),
    );
    if w.wrapping_sub(2 as libc::c_int as u_int) as libc::c_ulong >= strlen(text) {
        screen_write_cursormove(&mut ctx, 1 as libc::c_int as u_int, h);
        screen_write_puts(
            &mut ctx as *mut screen_write_ctx,
            &mut gc0 as *mut grid_cell,
            b"%s\0" as *const u8 as *const libc::c_char,
            text,
        );
        if (*mtd).no_matches != 0 {
            n = (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        } else {
            n = (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        if !((*mtd).filter).is_null()
            && w.wrapping_sub(2 as libc::c_int as u_int) as libc::c_ulong
                >= (strlen(text))
                    .wrapping_add(10 as libc::c_int as libc::c_ulong)
                    .wrapping_add(n)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
        {
            screen_write_puts(
                &mut ctx as *mut screen_write_ctx,
                &mut gc0 as *mut grid_cell,
                b" (filter: \0" as *const u8 as *const libc::c_char,
            );
            if (*mtd).no_matches != 0 {
                screen_write_puts(
                    &mut ctx as *mut screen_write_ctx,
                    &mut gc as *mut grid_cell,
                    b"no matches\0" as *const u8 as *const libc::c_char,
                );
            } else {
                screen_write_puts(
                    &mut ctx as *mut screen_write_ctx,
                    &mut gc0 as *mut grid_cell,
                    b"active\0" as *const u8 as *const libc::c_char,
                );
            }
            screen_write_puts(
                &mut ctx as *mut screen_write_ctx,
                &mut gc0 as *mut grid_cell,
                b") \0" as *const u8 as *const libc::c_char,
            );
        }
    }
    free(text as *mut libc::c_void);
    box_x = w.wrapping_sub(4 as libc::c_int as u_int);
    box_y = sy.wrapping_sub(h).wrapping_sub(2 as libc::c_int as u_int);
    if box_x != 0 as libc::c_int as u_int && box_y != 0 as libc::c_int as u_int {
        screen_write_cursormove(
            &mut ctx,
            2 as libc::c_int as u_int,
            h.wrapping_add(1 as libc::c_int as u_int),
        );
        ((*mtd).drawcb)
            .expect(
                "non-null function pointer",
            )((*mtd).modedata, (*mti).itemdata, &mut ctx, box_x, box_y);
    }
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn mode_tree_search_for(
    mut mtd: *mut mode_tree_data,
) -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut last: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut next: *mut mode_tree_item = 0 as *mut mode_tree_item;
    if ((*mtd).search).is_null() {
        return 0 as *mut mode_tree_item;
    }
    last = (*((*mtd).line_list).offset((*mtd).current as isize)).item;
    mti = last;
    loop {
        if !((*mti).children.tqh_first).is_null() {
            mti = (*mti).children.tqh_first;
        } else {
            next = (*mti).entry.tqe_next;
            if !next.is_null() {
                mti = next;
            } else {
                loop {
                    mti = (*mti).parent;
                    if mti.is_null() {
                        break;
                    }
                    next = (*mti).entry.tqe_next;
                    if next.is_null() {
                        continue;
                    }
                    mti = next;
                    break;
                }
            }
        }
        if mti.is_null() {
            mti = (*mtd).children.tqh_first;
        }
        if mti == last {
            break;
        }
        if ((*mtd).searchcb).is_none() {
            if !(strstr((*mti).name, (*mtd).search)).is_null() {
                return mti;
            }
        } else if ((*mtd).searchcb)
            .expect(
                "non-null function pointer",
            )((*mtd).modedata, (*mti).itemdata, (*mtd).search) != 0
        {
            return mti
        }
    }
    return 0 as *mut mode_tree_item;
}
unsafe extern "C" fn mode_tree_search_set(mut mtd: *mut mode_tree_data) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut loop_0: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut tag: uint64_t = 0;
    mti = mode_tree_search_for(mtd);
    if mti.is_null() {
        return;
    }
    tag = (*mti).tag;
    loop_0 = (*mti).parent;
    while !loop_0.is_null() {
        (*loop_0).expanded = 1 as libc::c_int;
        loop_0 = (*loop_0).parent;
    }
    mode_tree_build(mtd);
    mode_tree_set_current(mtd, tag);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 0x1 as libc::c_int;
}
unsafe extern "C" fn mode_tree_search_callback(
    mut c: *mut client,
    mut data: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut done: libc::c_int,
) -> libc::c_int {
    let mut mtd: *mut mode_tree_data = data as *mut mode_tree_data;
    if (*mtd).dead != 0 {
        return 0 as libc::c_int;
    }
    free((*mtd).search as *mut libc::c_void);
    if s.is_null() || *s as libc::c_int == '\0' as i32 {
        (*mtd).search = 0 as *mut libc::c_char;
        return 0 as libc::c_int;
    }
    (*mtd).search = xstrdup(s);
    mode_tree_search_set(mtd);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mode_tree_search_free(mut data: *mut libc::c_void) {
    mode_tree_remove_ref(data as *mut mode_tree_data);
}
unsafe extern "C" fn mode_tree_filter_callback(
    mut c: *mut client,
    mut data: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut done: libc::c_int,
) -> libc::c_int {
    let mut mtd: *mut mode_tree_data = data as *mut mode_tree_data;
    if (*mtd).dead != 0 {
        return 0 as libc::c_int;
    }
    if !((*mtd).filter).is_null() {
        free((*mtd).filter as *mut libc::c_void);
    }
    if s.is_null() || *s as libc::c_int == '\0' as i32 {
        (*mtd).filter = 0 as *mut libc::c_char;
    } else {
        (*mtd).filter = xstrdup(s);
    }
    mode_tree_build(mtd);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 0x1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mode_tree_filter_free(mut data: *mut libc::c_void) {
    mode_tree_remove_ref(data as *mut mode_tree_data);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_key(
    mut mtd: *mut mode_tree_data,
    mut c: *mut client,
    mut key: *mut key_code,
    mut m: *mut mouse_event,
    mut xp: *mut u_int,
    mut yp: *mut u_int,
) -> libc::c_int {
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut current: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut parent: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut choice: libc::c_int = 0;
    let mut tmp: key_code = 0;
    if *key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
        && *key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_int as libc::c_ulonglong
    {
        if cmd_mouse_at((*mtd).wp, m, &mut x, &mut y, 0 as libc::c_int)
            != 0 as libc::c_int
        {
            *key = 0xffff00000000 as libc::c_ulonglong;
            return 0 as libc::c_int;
        }
        if !xp.is_null() {
            *xp = x;
        }
        if !yp.is_null() {
            *yp = y;
        }
        if x > (*mtd).width || y > (*mtd).height {
            if (*mtd).preview == 0 {
                *key = 0xffff00000000 as libc::c_ulonglong;
            }
            return 0 as libc::c_int;
        }
        if ((*mtd).offset).wrapping_add(y) < (*mtd).line_size {
            if *key == KEYC_MOUSEDOWN1_PANE as libc::c_int as key_code
                || *key == KEYC_DOUBLECLICK1_PANE as libc::c_int as key_code
            {
                (*mtd).current = ((*mtd).offset).wrapping_add(y);
            }
            if *key == KEYC_DOUBLECLICK1_PANE as libc::c_int as key_code {
                *key = '\r' as i32 as key_code;
            } else {
                *key = 0xffff00000000 as libc::c_ulonglong;
            }
        } else {
            *key = 0xffff00000000 as libc::c_ulonglong;
        }
        return 0 as libc::c_int;
    }
    line = &mut *((*mtd).line_list).offset((*mtd).current as isize)
        as *mut mode_tree_line;
    current = (*line).item;
    choice = -(1 as libc::c_int);
    if *key >= '0' as i32 as key_code && *key <= '9' as i32 as key_code {
        choice = (*key).wrapping_sub('0' as i32 as key_code) as libc::c_int;
    } else if *key
        & (0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        == 0x200000000000 as libc::c_ulonglong
    {
        tmp = *key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong);
        if tmp >= 'a' as i32 as key_code && tmp <= 'z' as i32 as key_code {
            choice = (10 as libc::c_int as key_code)
                .wrapping_add(tmp.wrapping_sub('a' as i32 as key_code)) as libc::c_int;
        }
    }
    if choice != -(1 as libc::c_int) {
        if choice as u_int > ((*mtd).line_size).wrapping_sub(1 as libc::c_int as u_int) {
            *key = 0xffff00000000 as libc::c_ulonglong;
            return 0 as libc::c_int;
        }
        (*mtd).current = choice as u_int;
        *key = '\r' as i32 as key_code;
        return 0 as libc::c_int;
    }
    match *key {
        113 | 27 | 7 => return 1 as libc::c_int,
        268435545 | 107 | 268435501 | 16 => {
            mode_tree_up(mtd, 1 as libc::c_int);
        }
        268435546 | 106 | 268435504 | 14 => {
            mode_tree_down(mtd, 1 as libc::c_int);
        }
        268435543 | 2 => {
            i = 0 as libc::c_int as u_int;
            while i < (*mtd).height {
                if (*mtd).current == 0 as libc::c_int as u_int {
                    break;
                }
                mode_tree_up(mtd, 1 as libc::c_int);
                i = i.wrapping_add(1);
                i;
            }
        }
        268435542 | 6 => {
            i = 0 as libc::c_int as u_int;
            while i < (*mtd).height {
                if (*mtd).current
                    == ((*mtd).line_size).wrapping_sub(1 as libc::c_int as u_int)
                {
                    break;
                }
                mode_tree_down(mtd, 1 as libc::c_int);
                i = i.wrapping_add(1);
                i;
            }
        }
        268435540 => {
            (*mtd).current = 0 as libc::c_int as u_int;
            (*mtd).offset = 0 as libc::c_int as u_int;
        }
        268435541 => {
            (*mtd).current = ((*mtd).line_size).wrapping_sub(1 as libc::c_int as u_int);
            if (*mtd).current > ((*mtd).height).wrapping_sub(1 as libc::c_int as u_int) {
                (*mtd)
                    .offset = ((*mtd).current)
                    .wrapping_sub((*mtd).height)
                    .wrapping_add(1 as libc::c_int as u_int);
            } else {
                (*mtd).offset = 0 as libc::c_int as u_int;
            }
        }
        116 => {
            if (*current).tagged == 0 {
                parent = (*current).parent;
                while !parent.is_null() {
                    (*parent).tagged = 0 as libc::c_int;
                    parent = (*parent).parent;
                }
                mode_tree_clear_tagged(&mut (*current).children);
                (*current).tagged = 1 as libc::c_int;
            } else {
                (*current).tagged = 0 as libc::c_int;
            }
            mode_tree_down(mtd, 0 as libc::c_int);
        }
        84 => {
            i = 0 as libc::c_int as u_int;
            while i < (*mtd).line_size {
                (*(*((*mtd).line_list).offset(i as isize)).item)
                    .tagged = 0 as libc::c_int;
                i = i.wrapping_add(1);
                i;
            }
        }
        20 => {
            i = 0 as libc::c_int as u_int;
            while i < (*mtd).line_size {
                if ((*(*((*mtd).line_list).offset(i as isize)).item).parent).is_null() {
                    (*(*((*mtd).line_list).offset(i as isize)).item)
                        .tagged = 1 as libc::c_int;
                } else {
                    (*(*((*mtd).line_list).offset(i as isize)).item)
                        .tagged = 0 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        79 => {
            (*mtd).sort_type = ((*mtd).sort_type).wrapping_add(1);
            (*mtd).sort_type;
            if (*mtd).sort_type == (*mtd).sort_size {
                (*mtd).sort_type = 0 as libc::c_int as u_int;
            }
            mode_tree_build(mtd);
        }
        268435547 | 104 | 45 => {
            if (*line).flat != 0 || (*current).expanded == 0 {
                current = (*current).parent;
            }
            if current.is_null() {
                mode_tree_up(mtd, 0 as libc::c_int);
            } else {
                (*current).expanded = 0 as libc::c_int;
                (*mtd).current = (*current).line;
                mode_tree_build(mtd);
            }
        }
        268435548 | 108 | 43 => {
            if (*line).flat != 0 || (*current).expanded != 0 {
                mode_tree_down(mtd, 0 as libc::c_int);
            } else if (*line).flat == 0 {
                (*current).expanded = 1 as libc::c_int;
                mode_tree_build(mtd);
            }
        }
        19 => {
            (*mtd).references = ((*mtd).references).wrapping_add(1);
            (*mtd).references;
            status_prompt_set(
                c,
                b"(search) \0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                Some(
                    mode_tree_search_callback
                        as unsafe extern "C" fn(
                            *mut client,
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(
                    mode_tree_search_free
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                mtd as *mut libc::c_void,
                0x8 as libc::c_int,
            );
        }
        110 => {
            mode_tree_search_set(mtd);
        }
        102 => {
            (*mtd).references = ((*mtd).references).wrapping_add(1);
            (*mtd).references;
            status_prompt_set(
                c,
                b"(filter) \0" as *const u8 as *const libc::c_char,
                (*mtd).filter,
                Some(
                    mode_tree_filter_callback
                        as unsafe extern "C" fn(
                            *mut client,
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(
                    mode_tree_filter_free
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                mtd as *mut libc::c_void,
                0x8 as libc::c_int,
            );
        }
        118 => {
            (*mtd).preview = ((*mtd).preview == 0) as libc::c_int;
            mode_tree_build(mtd);
            if (*mtd).preview != 0 {
                mode_tree_check_selected(mtd);
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_run_command(
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
    mut template: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    command = cmd_template_replace(template, name, 1 as libc::c_int);
    if command.is_null() || *command as libc::c_int == '\0' as i32 {
        free(command as *mut libc::c_void);
        return;
    }
    cmdlist = cmd_string_parse(
        command,
        0 as *const libc::c_char,
        0 as libc::c_int as u_int,
        &mut cause,
    );
    if cmdlist.is_null() {
        if !cause.is_null() && !c.is_null() {
            *cause = toupper(*cause as u_char as libc::c_int) as libc::c_char;
            status_message_set(c, b"%s\0" as *const u8 as *const libc::c_char, cause);
        }
        free(cause as *mut libc::c_void);
    } else {
        new_item = cmdq_get_command(
            cmdlist,
            fs,
            0 as *mut mouse_event,
            0 as libc::c_int,
        );
        cmdq_append(c, new_item);
        cmd_list_free(cmdlist);
    }
    free(command as *mut libc::c_void);
}
