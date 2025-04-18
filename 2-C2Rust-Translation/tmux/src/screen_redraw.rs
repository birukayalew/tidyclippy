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
    pub type screen_write_collect_item;
    pub type screen_write_collect_line;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn format_create(
        _: *mut client,
        _: *mut cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut format_tree;
    fn format_free(_: *mut format_tree);
    fn format_expand(_: *mut format_tree, _: *const libc::c_char) -> *mut libc::c_char;
    fn format_defaults(
        _: *mut format_tree,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    );
    fn options_get_string(
        _: *mut options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn tty_attributes(_: *mut tty, _: *const grid_cell, _: *const window_pane);
    fn tty_reset(_: *mut tty);
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int);
    fn tty_puts(_: *mut tty, _: *const libc::c_char);
    fn tty_putc(_: *mut tty, _: u_char);
    fn tty_draw_pane(_: *mut tty, _: *const window_pane, _: u_int, _: u_int, _: u_int);
    fn tty_draw_line(
        _: *mut tty,
        _: *const window_pane,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    static mut marked_pane: cmd_find_state;
    fn server_is_marked(
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> libc::c_int;
    fn status_at_line(_: *mut client) -> libc::c_int;
    fn status_line_size(_: *mut session) -> u_int;
    fn status_redraw(_: *mut client) -> libc::c_int;
    fn status_message_redraw(_: *mut client) -> libc::c_int;
    fn status_prompt_redraw(_: *mut client) -> libc::c_int;
    static grid_default_cell: grid_cell;
    fn grid_compare(_: *mut grid, _: *mut grid) -> libc::c_int;
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    fn screen_write_stop(_: *mut screen_write_ctx);
    fn screen_write_cstrlen(_: *const libc::c_char, _: ...) -> size_t;
    fn screen_write_cnputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    );
    fn screen_write_clearline(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn screen_free(_: *mut screen);
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    fn window_count_panes(_: *mut window) -> u_int;
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    static window_clock_table: [[[libc::c_char; 5]; 5]; 14];
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
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
unsafe extern "C" fn screen_redraw_cell_border1(
    mut wp: *mut window_pane,
    mut px: u_int,
    mut py: u_int,
) -> libc::c_int {
    if px >= (*wp).xoff && px < ((*wp).xoff).wrapping_add((*wp).sx) && py >= (*wp).yoff
        && py < ((*wp).yoff).wrapping_add((*wp).sy)
    {
        return 0 as libc::c_int;
    }
    if ((*wp).yoff == 0 as libc::c_int as u_int
        || py >= ((*wp).yoff).wrapping_sub(1 as libc::c_int as u_int))
        && py <= ((*wp).yoff).wrapping_add((*wp).sy)
    {
        if (*wp).xoff != 0 as libc::c_int as u_int
            && px == ((*wp).xoff).wrapping_sub(1 as libc::c_int as u_int)
        {
            return 1 as libc::c_int;
        }
        if px == ((*wp).xoff).wrapping_add((*wp).sx) {
            return 2 as libc::c_int;
        }
    }
    if ((*wp).xoff == 0 as libc::c_int as u_int
        || px >= ((*wp).xoff).wrapping_sub(1 as libc::c_int as u_int))
        && px <= ((*wp).xoff).wrapping_add((*wp).sx)
    {
        if (*wp).yoff != 0 as libc::c_int as u_int
            && py == ((*wp).yoff).wrapping_sub(1 as libc::c_int as u_int)
        {
            return 3 as libc::c_int;
        }
        if py == ((*wp).yoff).wrapping_add((*wp).sy) {
            return 4 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn screen_redraw_cell_border(
    mut c: *mut client,
    mut px: u_int,
    mut py: u_int,
) -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut retval: libc::c_int = 0;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            retval = screen_redraw_cell_border1(wp, px, py);
            if retval != -(1 as libc::c_int) {
                return (retval != 0) as libc::c_int;
            }
        }
        wp = (*wp).entry.tqe_next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn screen_redraw_check_cell(
    mut c: *mut client,
    mut px: u_int,
    mut py: u_int,
    mut pane_status: libc::c_int,
    mut wpp: *mut *mut window_pane,
) -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut borders: libc::c_int = 0;
    let mut right: u_int = 0;
    let mut line: u_int = 0;
    *wpp = 0 as *mut window_pane;
    if px > (*w).sx || py > (*w).sy {
        return 12 as libc::c_int;
    }
    if pane_status != 0 as libc::c_int {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if !(window_pane_visible(wp) == 0) {
                if pane_status == 1 as libc::c_int {
                    line = ((*wp).yoff).wrapping_sub(1 as libc::c_int as u_int);
                } else {
                    line = ((*wp).yoff).wrapping_add((*wp).sy);
                }
                right = (((*wp).xoff).wrapping_add(2 as libc::c_int as u_int) as size_t)
                    .wrapping_add((*wp).status_size)
                    .wrapping_sub(1 as libc::c_int as size_t) as u_int;
                if py == line
                    && px >= ((*wp).xoff).wrapping_add(2 as libc::c_int as u_int)
                    && px <= right
                {
                    return 0 as libc::c_int;
                }
            }
            wp = (*wp).entry.tqe_next;
        }
    }
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            *wpp = wp;
            if !((*wp).xoff != 0 as libc::c_int as u_int
                && px < ((*wp).xoff).wrapping_sub(1 as libc::c_int as u_int)
                || px > ((*wp).xoff).wrapping_add((*wp).sx)
                || (*wp).yoff != 0 as libc::c_int as u_int
                    && py < ((*wp).yoff).wrapping_sub(1 as libc::c_int as u_int)
                || py > ((*wp).yoff).wrapping_add((*wp).sy))
            {
                if screen_redraw_cell_border(c, px, py) == 0 {
                    return 0 as libc::c_int;
                }
                borders = 0 as libc::c_int;
                if px == 0 as libc::c_int as u_int
                    || screen_redraw_cell_border(
                        c,
                        px.wrapping_sub(1 as libc::c_int as u_int),
                        py,
                    ) != 0
                {
                    borders |= 8 as libc::c_int;
                }
                if px <= (*w).sx
                    && screen_redraw_cell_border(
                        c,
                        px.wrapping_add(1 as libc::c_int as u_int),
                        py,
                    ) != 0
                {
                    borders |= 4 as libc::c_int;
                }
                if pane_status == 1 as libc::c_int {
                    if py != 0 as libc::c_int as u_int
                        && screen_redraw_cell_border(
                            c,
                            px,
                            py.wrapping_sub(1 as libc::c_int as u_int),
                        ) != 0
                    {
                        borders |= 2 as libc::c_int;
                    }
                } else if py == 0 as libc::c_int as u_int
                    || screen_redraw_cell_border(
                        c,
                        px,
                        py.wrapping_sub(1 as libc::c_int as u_int),
                    ) != 0
                {
                    borders |= 2 as libc::c_int;
                }
                if py <= (*w).sy
                    && screen_redraw_cell_border(
                        c,
                        px,
                        py.wrapping_add(1 as libc::c_int as u_int),
                    ) != 0
                {
                    borders |= 1 as libc::c_int;
                }
                match borders {
                    15 => return 11 as libc::c_int,
                    14 => return 8 as libc::c_int,
                    13 => return 7 as libc::c_int,
                    12 => return 2 as libc::c_int,
                    11 => return 10 as libc::c_int,
                    10 => return 6 as libc::c_int,
                    9 => return 4 as libc::c_int,
                    7 => return 9 as libc::c_int,
                    6 => return 5 as libc::c_int,
                    5 => return 3 as libc::c_int,
                    3 => return 1 as libc::c_int,
                    _ => {}
                }
            }
        }
        wp = (*wp).entry.tqe_next;
    }
    return 12 as libc::c_int;
}
unsafe extern "C" fn screen_redraw_check_is(
    mut px: u_int,
    mut py: u_int,
    mut type_0: libc::c_int,
    mut pane_status: libc::c_int,
    mut w: *mut window,
    mut wantwp: *mut window_pane,
    mut wp: *mut window_pane,
) -> libc::c_int {
    let mut border: libc::c_int = 0;
    border = screen_redraw_cell_border1(wantwp, px, py);
    if border == 0 as libc::c_int || border == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if pane_status == 1 as libc::c_int && border == 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    if pane_status == 2 as libc::c_int && border == 3 as libc::c_int {
        return 0 as libc::c_int;
    }
    if window_count_panes(w) != 2 as libc::c_int as u_int {
        return 1 as libc::c_int;
    }
    if wp.is_null() || (type_0 == 12 as libc::c_int || type_0 == 0 as libc::c_int) {
        return 1 as libc::c_int;
    }
    if pane_status != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*wp).xoff == 0 as libc::c_int as u_int && (*wp).sx == (*w).sx {
        if (*wp).yoff == 0 as libc::c_int as u_int {
            if wp == wantwp {
                return (px <= (*wp).sx / 2 as libc::c_int as u_int) as libc::c_int;
            }
            return (px > (*wp).sx / 2 as libc::c_int as u_int) as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if (*wp).yoff == 0 as libc::c_int as u_int && (*wp).sy == (*w).sy {
        if (*wp).xoff == 0 as libc::c_int as u_int {
            if wp == wantwp {
                return (py <= (*wp).sy / 2 as libc::c_int as u_int) as libc::c_int;
            }
            return (py > (*wp).sy / 2 as libc::c_int as u_int) as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn screen_redraw_make_pane_status(
    mut c: *mut client,
    mut w: *mut window,
    mut wp: *mut window_pane,
) -> libc::c_int {
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
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlen: size_t = 0;
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
    let mut old: screen = screen {
        title: 0 as *mut libc::c_char,
        titles: 0 as *mut screen_titles,
        grid: 0 as *mut grid,
        cx: 0,
        cy: 0,
        cstyle: 0,
        ccolour: 0 as *mut libc::c_char,
        rupper: 0,
        rlower: 0,
        mode: 0,
        tabs: 0 as *mut bitstr_t,
        sel: screen_sel {
            flag: 0,
            hidden: 0,
            rectflag: 0,
            lineflag: LINE_SEL_NONE,
            modekeys: 0,
            sx: 0,
            sy: 0,
            ex: 0,
            ey: 0,
            cell: grid_cell {
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
            },
        },
    };
    if wp == (*w).active {
        style_apply(
            &mut gc,
            (*w).options,
            b"pane-active-border-style\0" as *const u8 as *const libc::c_char,
        );
    } else {
        style_apply(
            &mut gc,
            (*w).options,
            b"pane-border-style\0" as *const u8 as *const libc::c_char,
        );
    }
    fmt = options_get_string(
        (*w).options,
        b"pane-border-format\0" as *const u8 as *const libc::c_char,
    );
    ft = format_create(
        c,
        0 as *mut cmdq_item,
        (0x80000000 as libc::c_uint | (*wp).id) as libc::c_int,
        0 as libc::c_int,
    );
    format_defaults(ft, c, 0 as *mut session, 0 as *mut winlink, wp);
    memcpy(
        &mut old as *mut screen as *mut libc::c_void,
        &mut (*wp).status_screen as *mut screen as *const libc::c_void,
        ::core::mem::size_of::<screen>() as libc::c_ulong,
    );
    screen_init(
        &mut (*wp).status_screen,
        (*wp).sx,
        1 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    (*wp).status_screen.mode = 0 as libc::c_int;
    out = format_expand(ft, fmt);
    outlen = screen_write_cstrlen(b"%s\0" as *const u8 as *const libc::c_char, out);
    if outlen > ((*wp).sx).wrapping_sub(4 as libc::c_int as u_int) as size_t {
        outlen = ((*wp).sx).wrapping_sub(4 as libc::c_int as u_int) as size_t;
    }
    screen_resize(
        &mut (*wp).status_screen,
        outlen as u_int,
        1 as libc::c_int as u_int,
        0 as libc::c_int,
    );
    screen_write_start(&mut ctx, 0 as *mut window_pane, &mut (*wp).status_screen);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    screen_write_clearline(&mut ctx, 8 as libc::c_int as u_int);
    screen_write_cnputs(
        &mut ctx as *mut screen_write_ctx,
        outlen as ssize_t,
        &mut gc as *mut grid_cell,
        b"%s\0" as *const u8 as *const libc::c_char,
        out,
    );
    screen_write_stop(&mut ctx);
    free(out as *mut libc::c_void);
    format_free(ft);
    (*wp).status_size = outlen;
    if grid_compare((*wp).status_screen.grid, old.grid) == 0 as libc::c_int {
        screen_free(&mut old);
        return 0 as libc::c_int;
    }
    screen_free(&mut old);
    return 1 as libc::c_int;
}
unsafe extern "C" fn screen_redraw_draw_pane_status(
    mut c: *mut client,
    mut pane_status: libc::c_int,
) {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut oo: *mut options = (*(*c).session).options;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut spos: libc::c_int = 0;
    let mut yoff: u_int = 0;
    spos = options_get_number(
        oo,
        b"status-position\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            if pane_status == 1 as libc::c_int {
                yoff = ((*wp).yoff).wrapping_sub(1 as libc::c_int as u_int);
            } else {
                yoff = ((*wp).yoff).wrapping_add((*wp).sy);
            }
            if spos == 0 as libc::c_int {
                yoff = yoff.wrapping_add(1 as libc::c_int as u_int);
            }
            tty_draw_line(
                tty,
                0 as *const window_pane,
                &mut (*wp).status_screen,
                0 as libc::c_int as u_int,
                ((*wp).xoff).wrapping_add(2 as libc::c_int as u_int),
                yoff,
            );
        }
        wp = (*wp).entry.tqe_next;
    }
    tty_cursor(tty, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
}
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_update(mut c: *mut client) {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wo: *mut options = (*w).options;
    let mut redraw: libc::c_int = 0;
    if !((*c).message_string).is_null() {
        redraw = status_message_redraw(c);
    } else if !((*c).prompt_string).is_null() {
        redraw = status_prompt_redraw(c);
    } else {
        redraw = status_redraw(c);
    }
    if redraw == 0 {
        (*c).flags &= !(0x10 as libc::c_int);
    }
    if options_get_number(
        wo,
        b"pane-border-status\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int as libc::c_longlong
    {
        redraw = 0 as libc::c_int;
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if screen_redraw_make_pane_status(c, w, wp) != 0 {
                redraw = 1 as libc::c_int;
            }
            wp = (*wp).entry.tqe_next;
        }
        if redraw != 0 {
            (*c).flags |= 0x400 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_screen(
    mut c: *mut client,
    mut draw_panes: libc::c_int,
    mut draw_status: libc::c_int,
    mut draw_borders: libc::c_int,
) {
    let mut oo: *mut options = (*(*c).session).options;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wo: *mut options = (*w).options;
    let mut top: u_int = 0;
    let mut lines: u_int = 0;
    let mut position: libc::c_int = 0;
    let mut pane_status: libc::c_int = 0;
    if (*c).flags & 0x40 as libc::c_int != 0 {
        return;
    }
    if (*c).flags & 0x800000 as libc::c_int != 0 {
        lines = 0 as libc::c_int as u_int;
    } else {
        lines = status_line_size((*c).session);
    }
    if !((*c).message_string).is_null() || !((*c).prompt_string).is_null() {
        lines = if lines == 0 as libc::c_int as u_int {
            1 as libc::c_int as u_int
        } else {
            lines
        };
    }
    position = options_get_number(
        oo,
        b"status-position\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if lines != 0 as libc::c_int as u_int && position == 0 as libc::c_int {
        top = 1 as libc::c_int as u_int;
    } else {
        top = 0 as libc::c_int as u_int;
    }
    if lines == 0 as libc::c_int as u_int {
        draw_status = 0 as libc::c_int;
    }
    if draw_borders != 0 {
        pane_status = options_get_number(
            wo,
            b"pane-border-status\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        screen_redraw_draw_borders(c, pane_status, lines, top);
        if pane_status != 0 as libc::c_int {
            screen_redraw_draw_pane_status(c, pane_status);
        }
    }
    if draw_panes != 0 {
        screen_redraw_draw_panes(c, lines, top);
    }
    if draw_status != 0 {
        screen_redraw_draw_status(c, lines, top);
    }
    tty_reset(tty);
}
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_pane(
    mut c: *mut client,
    mut wp: *mut window_pane,
) {
    let mut i: u_int = 0;
    let mut yoff: u_int = 0;
    if window_pane_visible(wp) == 0 {
        return;
    }
    yoff = (*wp).yoff;
    if status_at_line(c) == 0 as libc::c_int {
        yoff = yoff.wrapping_add(status_line_size((*c).session));
    }
    log_debug(
        b"%s: redraw pane %%%u (at %u,%u)\0" as *const u8 as *const libc::c_char,
        (*c).name,
        (*wp).id,
        (*wp).xoff,
        yoff,
    );
    i = 0 as libc::c_int as u_int;
    while i < (*wp).sy {
        tty_draw_pane(&mut (*c).tty, wp, i, (*wp).xoff, yoff);
        i = i.wrapping_add(1);
        i;
    }
    tty_reset(&mut (*c).tty);
}
unsafe extern "C" fn screen_redraw_draw_borders(
    mut c: *mut client,
    mut pane_status: libc::c_int,
    mut lines: u_int,
    mut top: u_int,
) {
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = (*(*s).curw).window;
    let mut oo: *mut options = (*w).options;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut m_active_gc: grid_cell = grid_cell {
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
    let mut active_gc: grid_cell = grid_cell {
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
    let mut m_other_gc: grid_cell = grid_cell {
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
    let mut other_gc: grid_cell = grid_cell {
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
    let mut msg_gc: grid_cell = grid_cell {
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
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut type_0: u_int = 0;
    let mut msgx: u_int = 0 as libc::c_int as u_int;
    let mut msgy: u_int = 0 as libc::c_int as u_int;
    let mut active: libc::c_int = 0;
    let mut small: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut msg: [libc::c_char; 256] = [0; 256];
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut msglen: size_t = 0 as libc::c_int as size_t;
    small = (((*tty).sy).wrapping_sub(lines).wrapping_add(top) > (*w).sy
        || (*tty).sx > (*w).sx) as libc::c_int;
    if small != 0 {
        flags = (*w).flags & (0x2000 as libc::c_int | 0x4000 as libc::c_int);
        if flags == 0x2000 as libc::c_int | 0x4000 as libc::c_int {
            tmp = b"force-width, force-height\0" as *const u8 as *const libc::c_char;
        } else if flags == 0x2000 as libc::c_int {
            tmp = b"force-width\0" as *const u8 as *const libc::c_char;
        } else if flags == 0x4000 as libc::c_int {
            tmp = b"force-height\0" as *const u8 as *const libc::c_char;
        } else if (*c).flags & 0x800000 as libc::c_int != 0 {
            tmp = b"status line\0" as *const u8 as *const libc::c_char;
        } else {
            tmp = b"a smaller client\0" as *const u8 as *const libc::c_char;
        }
        xsnprintf(
            msg.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"(size %ux%u from %s)\0" as *const u8 as *const libc::c_char,
            (*w).sx,
            (*w).sy,
            tmp,
        );
        msglen = strlen(msg.as_mut_ptr());
        if ((*tty).sy)
            .wrapping_sub(1 as libc::c_int as u_int)
            .wrapping_sub(lines)
            .wrapping_add(top) > (*w).sy && (*tty).sx as size_t >= msglen
        {
            msgx = ((*tty).sx as size_t).wrapping_sub(msglen) as u_int;
            msgy = ((*tty).sy)
                .wrapping_sub(1 as libc::c_int as u_int)
                .wrapping_sub(lines)
                .wrapping_add(top);
        } else if ((*tty).sx).wrapping_sub((*w).sx) as size_t > msglen {
            msgx = ((*tty).sx as size_t).wrapping_sub(msglen) as u_int;
            msgy = ((*tty).sy)
                .wrapping_sub(1 as libc::c_int as u_int)
                .wrapping_sub(lines)
                .wrapping_add(top);
        } else {
            small = 0 as libc::c_int;
        }
    }
    style_apply(
        &mut other_gc,
        oo,
        b"pane-border-style\0" as *const u8 as *const libc::c_char,
    );
    style_apply(
        &mut active_gc,
        oo,
        b"pane-active-border-style\0" as *const u8 as *const libc::c_char,
    );
    other_gc.attr = 0x80 as libc::c_int as u_short;
    active_gc.attr = other_gc.attr;
    memcpy(
        &mut m_other_gc as *mut grid_cell as *mut libc::c_void,
        &mut other_gc as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    m_other_gc.attr = (m_other_gc.attr as libc::c_int ^ 0x10 as libc::c_int) as u_short;
    memcpy(
        &mut m_active_gc as *mut grid_cell as *mut libc::c_void,
        &mut active_gc as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    m_active_gc
        .attr = (m_active_gc.attr as libc::c_int ^ 0x10 as libc::c_int) as u_short;
    j = 0 as libc::c_int as u_int;
    while j < ((*tty).sy).wrapping_sub(lines) {
        i = 0 as libc::c_int as u_int;
        while i < (*tty).sx {
            type_0 = screen_redraw_check_cell(c, i, j, pane_status, &mut wp) as u_int;
            if !(type_0 == 0 as libc::c_int as u_int) {
                if !(type_0 == 12 as libc::c_int as u_int && small != 0 && i > msgx
                    && j == msgy)
                {
                    active = screen_redraw_check_is(
                        i,
                        j,
                        type_0 as libc::c_int,
                        pane_status,
                        w,
                        (*w).active,
                        wp,
                    );
                    if server_is_marked(s, (*s).curw, marked_pane.wp) != 0
                        && screen_redraw_check_is(
                            i,
                            j,
                            type_0 as libc::c_int,
                            pane_status,
                            w,
                            marked_pane.wp,
                            wp,
                        ) != 0
                    {
                        if active != 0 {
                            tty_attributes(
                                tty,
                                &mut m_active_gc,
                                0 as *const window_pane,
                            );
                        } else {
                            tty_attributes(
                                tty,
                                &mut m_other_gc,
                                0 as *const window_pane,
                            );
                        }
                    } else if active != 0 {
                        tty_attributes(tty, &mut active_gc, 0 as *const window_pane);
                    } else {
                        tty_attributes(tty, &mut other_gc, 0 as *const window_pane);
                    }
                    if top != 0 {
                        tty_cursor(tty, i, lines.wrapping_add(j));
                    } else {
                        tty_cursor(tty, i, j);
                    }
                    tty_putc(
                        tty,
                        (*::core::mem::transmute::<
                            &[u8; 14],
                            &[libc::c_char; 14],
                        >(b" xqlkmjwvtun~\0"))[type_0 as usize] as u_char,
                    );
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    if small != 0 {
        memcpy(
            &mut msg_gc as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        tty_attributes(tty, &mut msg_gc, 0 as *const window_pane);
        tty_cursor(tty, msgx, msgy);
        tty_puts(tty, msg.as_mut_ptr());
    }
}
unsafe extern "C" fn screen_redraw_draw_panes(
    mut c: *mut client,
    mut lines: u_int,
    mut top: u_int,
) {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    let mut y: u_int = 0;
    if top != 0 {
        y = lines;
    } else {
        y = 0 as libc::c_int as u_int;
    }
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            i = 0 as libc::c_int as u_int;
            while i < (*wp).sy {
                tty_draw_pane(tty, wp, i, (*wp).xoff, y.wrapping_add((*wp).yoff));
                i = i.wrapping_add(1);
                i;
            }
            if (*c).flags & 0x100 as libc::c_int != 0 {
                screen_redraw_draw_number(c, wp, lines, top);
            }
        }
        wp = (*wp).entry.tqe_next;
    }
}
unsafe extern "C" fn screen_redraw_draw_status(
    mut c: *mut client,
    mut lines: u_int,
    mut top: u_int,
) {
    let mut tty: *mut tty = &mut (*c).tty;
    let mut i: u_int = 0;
    let mut y: u_int = 0;
    if top != 0 {
        y = 0 as libc::c_int as u_int;
    } else {
        y = ((*tty).sy).wrapping_sub(lines);
    }
    i = 0 as libc::c_int as u_int;
    while i < lines {
        tty_draw_line(
            tty,
            0 as *const window_pane,
            &mut (*c).status.status,
            i,
            0 as libc::c_int as u_int,
            y,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn screen_redraw_draw_number(
    mut c: *mut client,
    mut wp: *mut window_pane,
    mut lines: u_int,
    mut top: u_int,
) {
    let mut tty: *mut tty = &mut (*c).tty;
    let mut s: *mut session = (*c).session;
    let mut oo: *mut options = (*s).options;
    let mut w: *mut window = (*wp).window;
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
    let mut idx: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if window_pane_index(wp, &mut idx) != 0 as libc::c_int {
        fatalx(b"index not found\0" as *const u8 as *const libc::c_char);
    }
    len = xsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%u\0" as *const u8 as *const libc::c_char,
        idx,
    ) as size_t;
    if ((*wp).sx as size_t) < len {
        return;
    }
    colour = options_get_number(
        oo,
        b"display-panes-colour\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    active_colour = options_get_number(
        oo,
        b"display-panes-active-colour\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    px = (*wp).sx / 2 as libc::c_int as u_int;
    py = (*wp).sy / 2 as libc::c_int as u_int;
    xoff = (*wp).xoff;
    yoff = (*wp).yoff;
    if top != 0 {
        yoff = yoff.wrapping_add(lines);
    }
    if ((*wp).sx as size_t) < len * 6 as libc::c_int as size_t
        || (*wp).sy < 5 as libc::c_int as u_int
    {
        tty_cursor(
            tty,
            (xoff.wrapping_add(px) as size_t)
                .wrapping_sub(len / 2 as libc::c_int as size_t) as u_int,
            yoff.wrapping_add(py),
        );
    } else {
        px = (px as size_t).wrapping_sub(len * 3 as libc::c_int as size_t) as u_int
            as u_int;
        py = py.wrapping_sub(2 as libc::c_int as u_int);
        memcpy(
            &mut gc as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        if (*w).active == wp {
            gc.bg = active_colour;
        } else {
            gc.bg = colour;
        }
        gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
        tty_attributes(tty, &mut gc, wp);
        ptr = buf.as_mut_ptr();
        while *ptr as libc::c_int != '\0' as i32 {
            if !((*ptr as libc::c_int) < '0' as i32 || *ptr as libc::c_int > '9' as i32)
            {
                idx = (*ptr as libc::c_int - '0' as i32) as u_int;
                j = 0 as libc::c_int as u_int;
                while j < 5 as libc::c_int as u_int {
                    i = px;
                    while i < px.wrapping_add(5 as libc::c_int as u_int) {
                        tty_cursor(
                            tty,
                            xoff.wrapping_add(i),
                            yoff.wrapping_add(py).wrapping_add(j),
                        );
                        if window_clock_table[idx
                            as usize][j as usize][i.wrapping_sub(px) as usize] != 0
                        {
                            tty_putc(tty, ' ' as i32 as u_char);
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                px = px.wrapping_add(6 as libc::c_int as u_int);
            }
            ptr = ptr.offset(1);
            ptr;
        }
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%ux%u\0" as *const u8 as *const libc::c_char,
            (*wp).sx,
            (*wp).sy,
        ) as size_t;
        if ((*wp).sx as size_t) < len || (*wp).sy < 6 as libc::c_int as u_int {
            return;
        }
        tty_cursor(
            tty,
            (xoff.wrapping_add((*wp).sx) as size_t).wrapping_sub(len) as u_int,
            yoff,
        );
    }
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    if (*w).active == wp {
        gc.fg = active_colour;
    } else {
        gc.fg = colour;
    }
    gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    tty_attributes(tty, &mut gc, wp);
    tty_puts(tty, buf.as_mut_ptr());
    tty_cursor(tty, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
}
