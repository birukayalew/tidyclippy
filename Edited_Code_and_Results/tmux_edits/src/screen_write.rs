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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn tty_write(
        _: Option::<unsafe extern "C" fn(*mut tty, *const tty_ctx) -> ()>,
        _: *mut tty_ctx,
    );
    fn tty_cmd_alignmenttest(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_cell(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_cells(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearendofline(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearendofscreen(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearline(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearscreen(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearstartofline(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearstartofscreen(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_deletecharacter(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_clearcharacter(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_deleteline(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_insertcharacter(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_insertline(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_scrollup(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_reverseindex(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_setselection(_: *mut tty, _: *const tty_ctx);
    fn tty_cmd_rawstring(_: *mut tty, _: *const tty_ctx);
    static grid_default_cell: grid_cell;
    fn grid_cells_equal(_: *const grid_cell, _: *const grid_cell) -> libc::c_int;
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell);
    fn grid_move_lines(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    fn grid_view_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell);
    fn grid_view_set_cell(_: *mut grid, _: u_int, _: u_int, _: *const grid_cell);
    fn grid_view_set_cells(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: size_t,
    );
    fn grid_view_clear_history(_: *mut grid, _: u_int);
    fn grid_view_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int, _: u_int);
    fn grid_view_scroll_region_up(_: *mut grid, _: u_int, _: u_int, _: u_int);
    fn grid_view_scroll_region_down(_: *mut grid, _: u_int, _: u_int, _: u_int);
    fn grid_view_insert_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    fn grid_view_insert_lines_region(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    fn grid_view_delete_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    fn grid_view_delete_lines_region(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    fn grid_view_insert_cells(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    fn grid_view_delete_cells(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    fn screen_reset_tabs(_: *mut screen);
    fn screen_check_selection(_: *mut screen, _: u_int, _: u_int) -> libc::c_int;
    fn screen_select_cell(_: *mut screen, _: *mut grid_cell, _: *const grid_cell);
    fn utf8_set(_: *mut utf8_data, _: u_char);
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_write_collect_item {
    pub x: u_int,
    pub wrapped: libc::c_int,
    pub used: u_int,
    pub data: [libc::c_char; 256],
    pub gc: grid_cell,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub tqe_next: *mut screen_write_collect_item,
    pub tqe_prev: *mut *mut screen_write_collect_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_write_collect_line {
    pub items: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqh_first: *mut screen_write_collect_item,
    pub tqh_last: *mut *mut screen_write_collect_item,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_ctx {
    pub wp: *mut window_pane,
    pub cell: *const grid_cell,
    pub wrapped: libc::c_int,
    pub num: u_int,
    pub ptr: *mut libc::c_void,
    pub ocx: u_int,
    pub ocy: u_int,
    pub orupper: u_int,
    pub orlower: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub bg: u_int,
}
static mut screen_write_pad_cell: grid_cell = {
    let mut init = grid_cell {
        flags: 0x4 as libc::c_int as u_char,
        attr: 0 as libc::c_int as u_short,
        fg: 8 as libc::c_int,
        bg: 8 as libc::c_int,
        data: {
            let mut init = utf8_data {
                data: [0 as libc::c_int as u_char, 0, 0, 0, 0, 0, 0, 0, 0],
                have: 0 as libc::c_int as u_char,
                size: 0 as libc::c_int as u_char,
                width: 0 as libc::c_int as u_char,
            };
            init
        },
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn screen_write_start(
    mut ctx: *mut screen_write_ctx,
    mut wp: *mut window_pane,
    mut s: *mut screen,
) {
    let mut tmp: [libc::c_char; 16] = [0; 16];
    let mut y: u_int = 0;
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<screen_write_ctx>() as libc::c_ulong,
    );
    (*ctx).wp = wp;
    if !wp.is_null() && s.is_null() {
        (*ctx).s = (*wp).screen;
    } else {
        (*ctx).s = s;
    }
    (*ctx)
        .list = xcalloc(
        (*(*(*ctx).s).grid).sy as size_t,
        ::core::mem::size_of::<screen_write_collect_line>() as libc::c_ulong,
    ) as *mut screen_write_collect_line;
    y = 0 as libc::c_int as u_int;
    while y < (*(*(*ctx).s).grid).sy {
        let ref mut fresh0 = (*((*ctx).list).offset(y as isize)).items.tqh_first;
        *fresh0 = 0 as *mut screen_write_collect_item;
        let ref mut fresh1 = (*((*ctx).list).offset(y as isize)).items.tqh_last;
        *fresh1 = &mut (*((*ctx).list).offset(y as isize)).items.tqh_first;
        y = y.wrapping_add(1);
        y;
    }
    (*ctx)
        .item = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<screen_write_collect_item>() as libc::c_ulong,
    ) as *mut screen_write_collect_item;
    (*ctx).scrolled = 0 as libc::c_int as u_int;
    (*ctx).bg = 8 as libc::c_int as u_int;
    if !wp.is_null() {
        snprintf(
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"pane %%%u\0" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
    }
    log_debug(
        b"%s: size %ux%u, %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"screen_write_start\0"))
            .as_ptr(),
        (*(*(*ctx).s).grid).sx,
        (*(*(*ctx).s).grid).sy,
        if wp.is_null() {
            b"no pane\0" as *const u8 as *const libc::c_char
        } else {
            tmp.as_mut_ptr() as *const libc::c_char
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_stop(mut ctx: *mut screen_write_ctx) {
    screen_write_collect_end(ctx);
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    log_debug(
        b"%s: %u cells (%u written, %u skipped)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"screen_write_stop\0"))
            .as_ptr(),
        (*ctx).cells,
        (*ctx).written,
        (*ctx).skipped,
    );
    free((*ctx).item as *mut libc::c_void);
    free((*ctx).list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_reset(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    screen_reset_tabs(s);
    screen_write_scrollregion(
        ctx,
        0 as libc::c_int as u_int,
        ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int),
    );
    (*s).mode
        &= !(0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int
            | 0x800 as libc::c_int);
    (*s).mode
        &= !(0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int
            | 0x100 as libc::c_int | 0x200 as libc::c_int);
    screen_write_clearscreen(ctx, 8 as libc::c_int as u_int);
    screen_write_cursormove(ctx, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_putc(
    mut ctx: *mut screen_write_ctx,
    mut gcp: *const grid_cell,
    mut ch: u_char,
) {
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
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        gcp as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    utf8_set(&mut gc.data, ch);
    screen_write_cell(ctx, &mut gc);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_vnputs(
    mut ctx: *mut screen_write_ctx,
    mut maxlen: ssize_t,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
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
    let mut ud: *mut utf8_data = &mut gc.data;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut left: size_t = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        gcp as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    ptr = msg as *mut u_char;
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr as libc::c_int > 0x7f as libc::c_int
            && utf8_open(ud, *ptr) as libc::c_uint
                == UTF8_MORE as libc::c_int as libc::c_uint
        {
            ptr = ptr.offset(1);
            ptr;
            left = strlen(ptr as *const libc::c_char);
            if left < ((*ud).size as size_t).wrapping_sub(1 as libc::c_int as size_t) {
                break;
            }
            loop {
                more = utf8_append(ud, *ptr);
                if !(more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint) {
                    break;
                }
                ptr = ptr.offset(1);
                ptr;
            }
            ptr = ptr.offset(1);
            ptr;
            if more as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
                continue;
            }
            if maxlen > 0 as libc::c_int as ssize_t
                && size.wrapping_add((*ud).width as size_t) > maxlen as size_t
            {
                while size < maxlen as size_t {
                    screen_write_putc(ctx, &mut gc, ' ' as i32 as u_char);
                    size = size.wrapping_add(1);
                    size;
                }
                break;
            } else {
                size = size.wrapping_add((*ud).width as size_t);
                screen_write_cell(ctx, &mut gc);
            }
        } else {
            if maxlen > 0 as libc::c_int as ssize_t
                && size.wrapping_add(1 as libc::c_int as size_t) > maxlen as size_t
            {
                break;
            }
            if *ptr as libc::c_int == '\u{1}' as i32 {
                gc.attr = (gc.attr as libc::c_int ^ 0x80 as libc::c_int) as u_short;
            } else if *ptr as libc::c_int > 0x1f as libc::c_int
                && (*ptr as libc::c_int) < 0x7f as libc::c_int
            {
                size = size.wrapping_add(1);
                size;
                screen_write_putc(ctx, &mut gc, *ptr);
            }
            ptr = ptr.offset(1);
            ptr;
        }
    }
    free(msg as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_copy(
    mut ctx: *mut screen_write_ctx,
    mut src: *mut screen,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
    mut mbs: *mut bitstr_t,
    mut mgc: *const grid_cell,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*src).grid;
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
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut b: u_int = 0;
    if nx == 0 as libc::c_int as u_int || ny == 0 as libc::c_int as u_int {
        return;
    }
    cx = (*s).cx;
    cy = (*s).cy;
    yy = py;
    while yy < py.wrapping_add(ny) {
        xx = px;
        while xx < px.wrapping_add(nx) {
            grid_get_cell(gd, xx, yy, &mut gc);
            if !mbs.is_null() {
                b = (yy * (*(*src).grid).sx).wrapping_add(xx);
                if *mbs.offset((b >> 3 as libc::c_int) as isize) as libc::c_int
                    & (1 as libc::c_int) << (b & 0x7 as libc::c_int as u_int) != 0
                {
                    gc.attr = (*mgc).attr;
                    gc.fg = (*mgc).fg;
                    gc.bg = (*mgc).bg;
                }
            }
            if xx.wrapping_add(gc.data.width as u_int) <= px.wrapping_add(nx) {
                screen_write_cell(ctx, &mut gc);
            }
            xx = xx.wrapping_add(1);
            xx;
        }
        cy = cy.wrapping_add(1);
        cy;
        screen_write_cursormove(ctx, cx, cy);
        yy = yy.wrapping_add(1);
        yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_fast_copy(
    mut ctx: *mut screen_write_ctx,
    mut src: *mut screen,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*src).grid;
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
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    if nx == 0 as libc::c_int as u_int || ny == 0 as libc::c_int as u_int {
        return;
    }
    cy = (*s).cy;
    yy = py;
    while yy < py.wrapping_add(ny) {
        if yy >= ((*gd).hsize).wrapping_add((*gd).sy) {
            break;
        }
        cx = (*s).cx;
        xx = px;
        while xx < px.wrapping_add(nx) {
            if xx >= (*((*gd).linedata).offset(yy as isize)).cellsize {
                break;
            }
            grid_get_cell(gd, xx, yy, &mut gc);
            if xx.wrapping_add(gc.data.width as u_int) > px.wrapping_add(nx) {
                break;
            }
            if grid_cells_equal(&mut gc, &grid_default_cell) == 0 {
                grid_view_set_cell((*(*ctx).s).grid, cx, cy, &mut gc);
            }
            cx = cx.wrapping_add(1);
            cx;
            xx = xx.wrapping_add(1);
            xx;
        }
        cy = cy.wrapping_add(1);
        cy;
        yy = yy.wrapping_add(1);
        yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_hline(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gc.attr = (gc.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
    screen_write_putc(
        ctx,
        &mut gc,
        (if left != 0 { 't' as i32 } else { 'q' as i32 }) as u_char,
    );
    i = 1 as libc::c_int as u_int;
    while i < nx.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_putc(ctx, &mut gc, 'q' as i32 as u_char);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_putc(
        ctx,
        &mut gc,
        (if right != 0 { 'u' as i32 } else { 'q' as i32 }) as u_char,
    );
    screen_write_cursormove(ctx, cx, cy);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_vline(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
    mut top: libc::c_int,
    mut bottom: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gc.attr = (gc.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
    screen_write_putc(
        ctx,
        &mut gc,
        (if top != 0 { 'w' as i32 } else { 'x' as i32 }) as u_char,
    );
    i = 1 as libc::c_int as u_int;
    while i < ny.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_cursormove(ctx, cx, cy.wrapping_add(i));
        screen_write_putc(ctx, &mut gc, 'x' as i32 as u_char);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_cursormove(
        ctx,
        cx,
        cy.wrapping_add(ny).wrapping_sub(1 as libc::c_int as u_int),
    );
    screen_write_putc(
        ctx,
        &mut gc,
        (if bottom != 0 { 'v' as i32 } else { 'x' as i32 }) as u_char,
    );
    screen_write_cursormove(ctx, cx, cy);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_box(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gc.attr = (gc.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
    screen_write_putc(ctx, &mut gc, 'l' as i32 as u_char);
    i = 1 as libc::c_int as u_int;
    while i < nx.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_putc(ctx, &mut gc, 'q' as i32 as u_char);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_putc(ctx, &mut gc, 'k' as i32 as u_char);
    screen_write_cursormove(
        ctx,
        cx,
        cy.wrapping_add(ny).wrapping_sub(1 as libc::c_int as u_int),
    );
    screen_write_putc(ctx, &mut gc, 'm' as i32 as u_char);
    i = 1 as libc::c_int as u_int;
    while i < nx.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_putc(ctx, &mut gc, 'q' as i32 as u_char);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_putc(ctx, &mut gc, 'j' as i32 as u_char);
    i = 1 as libc::c_int as u_int;
    while i < ny.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_cursormove(ctx, cx, cy.wrapping_add(i));
        screen_write_putc(ctx, &mut gc, 'x' as i32 as u_char);
        i = i.wrapping_add(1);
        i;
    }
    i = 1 as libc::c_int as u_int;
    while i < ny.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(nx).wrapping_sub(1 as libc::c_int as u_int),
            cy.wrapping_add(i),
        );
        screen_write_putc(ctx, &mut gc, 'x' as i32 as u_char);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_cursormove(ctx, cx, cy);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_preview(
    mut ctx: *mut screen_write_ctx,
    mut src: *mut screen,
    mut nx: u_int,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    if (*src).mode & 0x1 as libc::c_int != 0 {
        px = (*src).cx;
        if px < nx / 3 as libc::c_int as u_int {
            px = 0 as libc::c_int as u_int;
        } else {
            px = px.wrapping_sub(nx / 3 as libc::c_int as u_int);
        }
        if px.wrapping_add(nx) > (*(*src).grid).sx {
            if nx > (*(*src).grid).sx {
                px = 0 as libc::c_int as u_int;
            } else {
                px = ((*(*src).grid).sx).wrapping_sub(nx);
            }
        }
        py = (*src).cy;
        if py < ny / 3 as libc::c_int as u_int {
            py = 0 as libc::c_int as u_int;
        } else {
            py = py.wrapping_sub(ny / 3 as libc::c_int as u_int);
        }
        if py.wrapping_add(ny) > (*(*src).grid).sy {
            if ny > (*(*src).grid).sy {
                py = 0 as libc::c_int as u_int;
            } else {
                py = ((*(*src).grid).sy).wrapping_sub(ny);
            }
        }
    } else {
        px = 0 as libc::c_int as u_int;
        py = 0 as libc::c_int as u_int;
    }
    screen_write_fast_copy(
        ctx,
        src,
        px,
        ((*(*src).grid).hsize).wrapping_add(py),
        nx,
        ny,
    );
    if (*src).mode & 0x1 as libc::c_int != 0 {
        grid_view_get_cell((*src).grid, (*src).cx, (*src).cy, &mut gc);
        gc.attr = (gc.attr as libc::c_int | 0x10 as libc::c_int) as u_short;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(((*src).cx).wrapping_sub(px)),
            cy.wrapping_add(((*src).cy).wrapping_sub(py)),
        );
        screen_write_cell(ctx, &mut gc);
    }
}
unsafe extern "C" fn screen_write_initctx(
    mut ctx: *mut screen_write_ctx,
    mut ttyctx: *mut tty_ctx,
) {
    let mut s: *mut screen = (*ctx).s;
    memset(
        ttyctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<tty_ctx>() as libc::c_ulong,
    );
    (*ttyctx).wp = (*ctx).wp;
    (*ttyctx).ocx = (*s).cx;
    (*ttyctx).ocy = (*s).cy;
    (*ttyctx).orlower = (*s).rlower;
    (*ttyctx).orupper = (*s).rupper;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_mode_set(
    mut ctx: *mut screen_write_ctx,
    mut mode: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    (*s).mode |= mode;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_mode_clear(
    mut ctx: *mut screen_write_ctx,
    mut mode: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    (*s).mode &= !mode;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorup(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if ny == 0 as libc::c_int as u_int {
        ny = 1 as libc::c_int as u_int;
    }
    if (*s).cy < (*s).rupper {
        if ny > (*s).cy {
            ny = (*s).cy;
        }
    } else if ny > ((*s).cy).wrapping_sub((*s).rupper) {
        ny = ((*s).cy).wrapping_sub((*s).rupper);
    }
    if (*s).cx == (*(*s).grid).sx {
        (*s).cx = ((*s).cx).wrapping_sub(1);
        (*s).cx;
    }
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    (*s).cy = ((*s).cy).wrapping_sub(ny);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursordown(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if ny == 0 as libc::c_int as u_int {
        ny = 1 as libc::c_int as u_int;
    }
    if (*s).cy > (*s).rlower {
        if ny
            > ((*(*s).grid).sy)
                .wrapping_sub(1 as libc::c_int as u_int)
                .wrapping_sub((*s).cy)
        {
            ny = ((*(*s).grid).sy)
                .wrapping_sub(1 as libc::c_int as u_int)
                .wrapping_sub((*s).cy);
        }
    } else if ny > ((*s).rlower).wrapping_sub((*s).cy) {
        ny = ((*s).rlower).wrapping_sub((*s).cy);
    }
    if (*s).cx == (*(*s).grid).sx {
        (*s).cx = ((*s).cx).wrapping_sub(1);
        (*s).cx;
    }
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    (*s).cy = ((*s).cy).wrapping_add(ny);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorright(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if nx == 0 as libc::c_int as u_int {
        nx = 1 as libc::c_int as u_int;
    }
    if nx
        > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int).wrapping_sub((*s).cx)
    {
        nx = ((*(*s).grid).sx)
            .wrapping_sub(1 as libc::c_int as u_int)
            .wrapping_sub((*s).cx);
    }
    if nx == 0 as libc::c_int as u_int {
        return;
    }
    (*s).cx = ((*s).cx).wrapping_add(nx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorleft(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if nx == 0 as libc::c_int as u_int {
        nx = 1 as libc::c_int as u_int;
    }
    if nx > (*s).cx {
        nx = (*s).cx;
    }
    if nx == 0 as libc::c_int as u_int {
        return;
    }
    (*s).cx = ((*s).cx).wrapping_sub(nx);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_backspace(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    if (*s).cx == 0 as libc::c_int as u_int {
        if (*s).cy == 0 as libc::c_int as u_int {
            return;
        }
        gl = &mut *((*(*s).grid).linedata)
            .offset(
                ((*(*s).grid).hsize)
                    .wrapping_add((*s).cy)
                    .wrapping_sub(1 as libc::c_int as u_int) as isize,
            ) as *mut grid_line;
        if (*gl).flags & 0x1 as libc::c_int != 0 {
            (*s).cy = ((*s).cy).wrapping_sub(1);
            (*s).cy;
            (*s).cx = ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int);
        }
    } else {
        (*s).cx = ((*s).cx).wrapping_sub(1);
        (*s).cx;
    };
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_alignmenttest(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
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
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    screen_write_initctx(ctx, &mut ttyctx);
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    utf8_set(&mut gc.data, 'E' as i32 as u_char);
    yy = 0 as libc::c_int as u_int;
    while yy < (*(*s).grid).sy {
        xx = 0 as libc::c_int as u_int;
        while xx < (*(*s).grid).sx {
            grid_view_set_cell((*s).grid, xx, yy, &mut gc);
            xx = xx.wrapping_add(1);
            xx;
        }
        yy = yy.wrapping_add(1);
        yy;
    }
    (*s).cx = 0 as libc::c_int as u_int;
    (*s).cy = 0 as libc::c_int as u_int;
    (*s).rupper = 0 as libc::c_int as u_int;
    (*s).rlower = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    screen_write_collect_clear(
        ctx,
        0 as libc::c_int as u_int,
        ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int),
    );
    tty_write(
        Some(
            tty_cmd_alignmenttest as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_insertcharacter(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    if nx == 0 as libc::c_int as u_int {
        nx = 1 as libc::c_int as u_int;
    }
    if nx > ((*(*s).grid).sx).wrapping_sub((*s).cx) {
        nx = ((*(*s).grid).sx).wrapping_sub((*s).cx);
    }
    if nx == 0 as libc::c_int as u_int {
        return;
    }
    if (*s).cx > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    grid_view_insert_cells((*s).grid, (*s).cx, (*s).cy, nx, bg);
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    ttyctx.num = nx;
    tty_write(
        Some(
            tty_cmd_insertcharacter
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_deletecharacter(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    if nx == 0 as libc::c_int as u_int {
        nx = 1 as libc::c_int as u_int;
    }
    if nx > ((*(*s).grid).sx).wrapping_sub((*s).cx) {
        nx = ((*(*s).grid).sx).wrapping_sub((*s).cx);
    }
    if nx == 0 as libc::c_int as u_int {
        return;
    }
    if (*s).cx > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    grid_view_delete_cells((*s).grid, (*s).cx, (*s).cy, nx, bg);
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    ttyctx.num = nx;
    tty_write(
        Some(
            tty_cmd_deletecharacter
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearcharacter(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    if nx == 0 as libc::c_int as u_int {
        nx = 1 as libc::c_int as u_int;
    }
    if nx > ((*(*s).grid).sx).wrapping_sub((*s).cx) {
        nx = ((*(*s).grid).sx).wrapping_sub((*s).cx);
    }
    if nx == 0 as libc::c_int as u_int {
        return;
    }
    if (*s).cx > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    grid_view_clear((*s).grid, (*s).cx, (*s).cy, nx, 1 as libc::c_int as u_int, bg);
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    ttyctx.num = nx;
    tty_write(
        Some(
            tty_cmd_clearcharacter
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_insertline(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    if ny == 0 as libc::c_int as u_int {
        ny = 1 as libc::c_int as u_int;
    }
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        if ny > ((*(*s).grid).sy).wrapping_sub((*s).cy) {
            ny = ((*(*s).grid).sy).wrapping_sub((*s).cy);
        }
        if ny == 0 as libc::c_int as u_int {
            return;
        }
        screen_write_initctx(ctx, &mut ttyctx);
        ttyctx.bg = bg;
        grid_view_insert_lines(gd, (*s).cy, ny, bg);
        screen_write_collect_flush(ctx, 0 as libc::c_int);
        ttyctx.num = ny;
        tty_write(
            Some(
                tty_cmd_insertline
                    as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
            ),
            &mut ttyctx,
        );
        return;
    }
    if ny > ((*s).rlower).wrapping_add(1 as libc::c_int as u_int).wrapping_sub((*s).cy) {
        ny = ((*s).rlower).wrapping_add(1 as libc::c_int as u_int).wrapping_sub((*s).cy);
    }
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        grid_view_insert_lines(gd, (*s).cy, ny, bg);
    } else {
        grid_view_insert_lines_region(gd, (*s).rlower, (*s).cy, ny, bg);
    }
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    ttyctx.num = ny;
    tty_write(
        Some(tty_cmd_insertline as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_deleteline(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    if ny == 0 as libc::c_int as u_int {
        ny = 1 as libc::c_int as u_int;
    }
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        if ny > ((*(*s).grid).sy).wrapping_sub((*s).cy) {
            ny = ((*(*s).grid).sy).wrapping_sub((*s).cy);
        }
        if ny == 0 as libc::c_int as u_int {
            return;
        }
        screen_write_initctx(ctx, &mut ttyctx);
        ttyctx.bg = bg;
        grid_view_delete_lines(gd, (*s).cy, ny, bg);
        screen_write_collect_flush(ctx, 0 as libc::c_int);
        ttyctx.num = ny;
        tty_write(
            Some(
                tty_cmd_deleteline
                    as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
            ),
            &mut ttyctx,
        );
        return;
    }
    if ny > ((*s).rlower).wrapping_add(1 as libc::c_int as u_int).wrapping_sub((*s).cy) {
        ny = ((*s).rlower).wrapping_add(1 as libc::c_int as u_int).wrapping_sub((*s).cy);
    }
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        grid_view_delete_lines(gd, (*s).cy, ny, bg);
    } else {
        grid_view_delete_lines_region(gd, (*s).rlower, (*s).cy, ny, bg);
    }
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    ttyctx.num = ny;
    tty_write(
        Some(tty_cmd_deleteline as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearline(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    gl = &mut *((*(*s).grid).linedata)
        .offset(((*(*s).grid).hsize).wrapping_add((*s).cy) as isize) as *mut grid_line;
    if (*gl).cellsize == 0 as libc::c_int as u_int && bg == 8 as libc::c_int as u_int {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    grid_view_clear(
        (*s).grid,
        0 as libc::c_int as u_int,
        (*s).cy,
        sx,
        1 as libc::c_int as u_int,
        bg,
    );
    screen_write_collect_clear(ctx, (*s).cy, 1 as libc::c_int as u_int);
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    tty_write(
        Some(tty_cmd_clearline as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearendofline(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    gl = &mut *((*(*s).grid).linedata)
        .offset(((*(*s).grid).hsize).wrapping_add((*s).cy) as isize) as *mut grid_line;
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as u_int)
        || (*s).cx >= (*gl).cellsize && bg == 8 as libc::c_int as u_int
    {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    grid_view_clear(
        (*s).grid,
        (*s).cx,
        (*s).cy,
        sx.wrapping_sub((*s).cx),
        1 as libc::c_int as u_int,
        bg,
    );
    if (*s).cx == 0 as libc::c_int as u_int {
        screen_write_collect_clear(ctx, (*s).cy, 1 as libc::c_int as u_int);
    }
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    tty_write(
        Some(
            tty_cmd_clearendofline
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearstartofline(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as u_int) {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            sx,
            1 as libc::c_int as u_int,
            bg,
        );
    } else {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            ((*s).cx).wrapping_add(1 as libc::c_int as u_int),
            1 as libc::c_int as u_int,
            bg,
        );
    }
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as u_int) {
        screen_write_collect_clear(ctx, (*s).cy, 1 as libc::c_int as u_int);
    }
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    tty_write(
        Some(
            tty_cmd_clearstartofline
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursormove(
    mut ctx: *mut screen_write_ctx,
    mut px: u_int,
    mut py: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if px > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
        px = ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int);
    }
    if py > ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
        py = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    }
    (*s).cx = px;
    (*s).cy = py;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_reverseindex(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*s).cy == (*s).rupper {
        grid_view_scroll_region_down((*s).grid, (*s).rupper, (*s).rlower, bg);
    } else if (*s).cy > 0 as libc::c_int as u_int {
        (*s).cy = ((*s).cy).wrapping_sub(1);
        (*s).cy;
    }
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    tty_write(
        Some(
            tty_cmd_reverseindex as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrollregion(
    mut ctx: *mut screen_write_ctx,
    mut rupper: u_int,
    mut rlower: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if rupper > ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
        rupper = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    }
    if rlower > ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
        rlower = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    }
    if rupper >= rlower {
        return;
    }
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    (*s).cx = 0 as libc::c_int as u_int;
    (*s).cy = 0 as libc::c_int as u_int;
    (*s).rupper = rupper;
    (*s).rlower = rlower;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_linefeed(
    mut ctx: *mut screen_write_ctx,
    mut wrapped: libc::c_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    gl = &mut *((*gd).linedata).offset(((*gd).hsize).wrapping_add((*s).cy) as isize)
        as *mut grid_line;
    if wrapped != 0 {
        (*gl).flags |= 0x1 as libc::c_int;
    } else {
        (*gl).flags &= !(0x1 as libc::c_int);
    }
    log_debug(
        b"%s: at %u,%u (region %u-%u)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"screen_write_linefeed\0"))
            .as_ptr(),
        (*s).cx,
        (*s).cy,
        (*s).rupper,
        (*s).rlower,
    );
    if bg != (*ctx).bg {
        screen_write_collect_flush(ctx, 1 as libc::c_int);
        (*ctx).bg = bg;
    }
    if (*s).cy == (*s).rlower {
        grid_view_scroll_region_up(gd, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_scroll(ctx);
        (*ctx).scrolled = ((*ctx).scrolled).wrapping_add(1);
        (*ctx).scrolled;
    } else if (*s).cy < ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
        (*s).cy = ((*s).cy).wrapping_add(1);
        (*s).cy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrollup(
    mut ctx: *mut screen_write_ctx,
    mut lines: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut i: u_int = 0;
    if lines == 0 as libc::c_int as u_int {
        lines = 1 as libc::c_int as u_int;
    } else if lines
        > ((*s).rlower).wrapping_sub((*s).rupper).wrapping_add(1 as libc::c_int as u_int)
    {
        lines = ((*s).rlower)
            .wrapping_sub((*s).rupper)
            .wrapping_add(1 as libc::c_int as u_int);
    }
    if bg != (*ctx).bg {
        screen_write_collect_flush(ctx, 1 as libc::c_int);
        (*ctx).bg = bg;
    }
    i = 0 as libc::c_int as u_int;
    while i < lines {
        grid_view_scroll_region_up(gd, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_scroll(ctx);
        i = i.wrapping_add(1);
        i;
    }
    (*ctx).scrolled = ((*ctx).scrolled).wrapping_add(lines);
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_carriagereturn(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    (*s).cx = 0 as libc::c_int as u_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearendofscreen(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*s).cx == 0 as libc::c_int as u_int && (*s).cy == 0 as libc::c_int as u_int
        && (*gd).flags & 0x1 as libc::c_int != 0
    {
        grid_view_clear_history(gd, bg);
    } else {
        if (*s).cx <= sx.wrapping_sub(1 as libc::c_int as u_int) {
            grid_view_clear(
                gd,
                (*s).cx,
                (*s).cy,
                sx.wrapping_sub((*s).cx),
                1 as libc::c_int as u_int,
                bg,
            );
        }
        grid_view_clear(
            gd,
            0 as libc::c_int as u_int,
            ((*s).cy).wrapping_add(1 as libc::c_int as u_int),
            sx,
            sy.wrapping_sub(((*s).cy).wrapping_add(1 as libc::c_int as u_int)),
            bg,
        );
    }
    screen_write_collect_clear(
        ctx,
        ((*s).cy).wrapping_add(1 as libc::c_int as u_int),
        sy.wrapping_sub(((*s).cy).wrapping_add(1 as libc::c_int as u_int)),
    );
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    tty_write(
        Some(
            tty_cmd_clearendofscreen
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearstartofscreen(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*s).cy > 0 as libc::c_int as u_int {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            sx,
            (*s).cy,
            bg,
        );
    }
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as u_int) {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            sx,
            1 as libc::c_int as u_int,
            bg,
        );
    } else {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            ((*s).cx).wrapping_add(1 as libc::c_int as u_int),
            1 as libc::c_int as u_int,
            bg,
        );
    }
    screen_write_collect_clear(ctx, 0 as libc::c_int as u_int, (*s).cy);
    screen_write_collect_flush(ctx, 0 as libc::c_int);
    tty_write(
        Some(
            tty_cmd_clearstartofscreen
                as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearscreen(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.bg = bg;
    if (*(*s).grid).flags & 0x1 as libc::c_int != 0 {
        grid_view_clear_history((*s).grid, bg);
    } else {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            sx,
            sy,
            bg,
        );
    }
    screen_write_collect_clear(ctx, 0 as libc::c_int as u_int, sy);
    tty_write(
        Some(
            tty_cmd_clearscreen as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearhistory(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    grid_move_lines(
        gd,
        0 as libc::c_int as u_int,
        (*gd).hsize,
        (*gd).sy,
        8 as libc::c_int as u_int,
    );
    (*gd).hsize = 0 as libc::c_int as u_int;
    (*gd).hscrolled = (*gd).hsize;
}
unsafe extern "C" fn screen_write_collect_clear(
    mut ctx: *mut screen_write_ctx,
    mut y: u_int,
    mut n: u_int,
) {
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut i: u_int = 0;
    let mut size: size_t = 0;
    i = y;
    while i < y.wrapping_add(n) {
        if !((*((*ctx).list).offset(i as isize)).items.tqh_first).is_null() {
            size = 0 as libc::c_int as size_t;
            ci = (*((*ctx).list).offset(i as isize)).items.tqh_first;
            while !ci.is_null()
                && {
                    tmp = (*ci).entry.tqe_next;
                    1 as libc::c_int != 0
                }
            {
                size = size.wrapping_add((*ci).used as size_t);
                if !((*ci).entry.tqe_next).is_null() {
                    (*(*ci).entry.tqe_next).entry.tqe_prev = (*ci).entry.tqe_prev;
                } else {
                    let ref mut fresh2 = (*((*ctx).list).offset(i as isize))
                        .items
                        .tqh_last;
                    *fresh2 = (*ci).entry.tqe_prev;
                }
                *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
                free(ci as *mut libc::c_void);
                ci = tmp;
            }
            (*ctx)
                .skipped = ((*ctx).skipped as size_t).wrapping_add(size) as u_int
                as u_int;
            log_debug(
                b"%s: dropped %zu bytes (line %u)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"screen_write_collect_clear\0"))
                    .as_ptr(),
                size,
                i,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn screen_write_collect_scroll(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut cl: *mut screen_write_collect_line = 0 as *mut screen_write_collect_line;
    let mut y: u_int = 0;
    log_debug(
        b"%s: at %u,%u (region %u-%u)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 28],
            &[libc::c_char; 28],
        >(b"screen_write_collect_scroll\0"))
            .as_ptr(),
        (*s).cx,
        (*s).cy,
        (*s).rupper,
        (*s).rlower,
    );
    screen_write_collect_clear(ctx, (*s).rupper, 1 as libc::c_int as u_int);
    y = (*s).rupper;
    while y < (*s).rlower {
        cl = &mut *((*ctx).list)
            .offset(y.wrapping_add(1 as libc::c_int as u_int) as isize)
            as *mut screen_write_collect_line;
        if !((*cl).items.tqh_first).is_null() {
            let ref mut fresh3 = *(*((*ctx).list).offset(y as isize)).items.tqh_last;
            *fresh3 = (*cl).items.tqh_first;
            (*(*cl).items.tqh_first)
                .entry
                .tqe_prev = (*((*ctx).list).offset(y as isize)).items.tqh_last;
            let ref mut fresh4 = (*((*ctx).list).offset(y as isize)).items.tqh_last;
            *fresh4 = (*cl).items.tqh_last;
            (*cl).items.tqh_first = 0 as *mut screen_write_collect_item;
            (*cl).items.tqh_last = &mut (*cl).items.tqh_first;
        }
        (*cl).items.tqh_first = 0 as *mut screen_write_collect_item;
        (*cl).items.tqh_last = &mut (*cl).items.tqh_first;
        y = y.wrapping_add(1);
        y;
    }
}
unsafe extern "C" fn screen_write_collect_flush(
    mut ctx: *mut screen_write_ctx,
    mut scroll_only: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut y: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut items: u_int = 0 as libc::c_int as u_int;
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut written: size_t = 0 as libc::c_int as size_t;
    if (*ctx).scrolled != 0 as libc::c_int as u_int {
        log_debug(
            b"%s: scrolled %u (region %u-%u)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"screen_write_collect_flush\0"))
                .as_ptr(),
            (*ctx).scrolled,
            (*s).rupper,
            (*s).rlower,
        );
        if (*ctx).scrolled
            > ((*s).rlower)
                .wrapping_sub((*s).rupper)
                .wrapping_add(1 as libc::c_int as u_int)
        {
            (*ctx)
                .scrolled = ((*s).rlower)
                .wrapping_sub((*s).rupper)
                .wrapping_add(1 as libc::c_int as u_int);
        }
        screen_write_initctx(ctx, &mut ttyctx);
        ttyctx.num = (*ctx).scrolled;
        ttyctx.bg = (*ctx).bg;
        tty_write(
            Some(
                tty_cmd_scrollup as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
            ),
            &mut ttyctx,
        );
    }
    (*ctx).scrolled = 0 as libc::c_int as u_int;
    (*ctx).bg = 8 as libc::c_int as u_int;
    if scroll_only != 0 {
        return;
    }
    cx = (*s).cx;
    cy = (*s).cy;
    y = 0 as libc::c_int as u_int;
    while y < (*(*s).grid).sy {
        ci = (*((*ctx).list).offset(y as isize)).items.tqh_first;
        while !ci.is_null()
            && {
                tmp = (*ci).entry.tqe_next;
                1 as libc::c_int != 0
            }
        {
            screen_write_cursormove(ctx, (*ci).x, y);
            screen_write_initctx(ctx, &mut ttyctx);
            ttyctx.cell = &mut (*ci).gc;
            ttyctx.wrapped = (*ci).wrapped;
            ttyctx.ptr = ((*ci).data).as_mut_ptr() as *mut libc::c_void;
            ttyctx.num = (*ci).used;
            tty_write(
                Some(
                    tty_cmd_cells as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
                ),
                &mut ttyctx,
            );
            items = items.wrapping_add(1);
            items;
            written = written.wrapping_add((*ci).used as size_t);
            if !((*ci).entry.tqe_next).is_null() {
                (*(*ci).entry.tqe_next).entry.tqe_prev = (*ci).entry.tqe_prev;
            } else {
                let ref mut fresh5 = (*((*ctx).list).offset(y as isize)).items.tqh_last;
                *fresh5 = (*ci).entry.tqe_prev;
            }
            *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
            free(ci as *mut libc::c_void);
            ci = tmp;
        }
        y = y.wrapping_add(1);
        y;
    }
    (*s).cx = cx;
    (*s).cy = cy;
    log_debug(
        b"%s: flushed %u items (%zu bytes)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"screen_write_collect_flush\0"))
            .as_ptr(),
        items,
        written,
    );
    (*ctx).written = ((*ctx).written as size_t).wrapping_add(written) as u_int as u_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_collect_end(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = (*ctx).item;
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
    let mut xx: u_int = 0;
    if (*ci).used == 0 as libc::c_int as u_int {
        return;
    }
    (*ci).data[(*ci).used as usize] = '\0' as i32 as libc::c_char;
    (*ci).x = (*s).cx;
    (*ci).entry.tqe_next = 0 as *mut screen_write_collect_item;
    (*ci).entry.tqe_prev = (*((*ctx).list).offset((*s).cy as isize)).items.tqh_last;
    let ref mut fresh6 = *(*((*ctx).list).offset((*s).cy as isize)).items.tqh_last;
    *fresh6 = ci;
    let ref mut fresh7 = (*((*ctx).list).offset((*s).cy as isize)).items.tqh_last;
    *fresh7 = &mut (*ci).entry.tqe_next;
    (*ctx)
        .item = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<screen_write_collect_item>() as libc::c_ulong,
    ) as *mut screen_write_collect_item;
    log_debug(
        b"%s: %u %s (at %u,%u)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 25],
            &[libc::c_char; 25],
        >(b"screen_write_collect_end\0"))
            .as_ptr(),
        (*ci).used,
        ((*ci).data).as_mut_ptr(),
        (*s).cx,
        (*s).cy,
    );
    if (*s).cx != 0 as libc::c_int as u_int {
        xx = (*s).cx;
        while xx > 0 as libc::c_int as u_int {
            grid_view_get_cell((*s).grid, xx, (*s).cy, &mut gc);
            if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            grid_view_set_cell((*s).grid, xx, (*s).cy, &grid_default_cell);
            xx = xx.wrapping_sub(1);
            xx;
        }
        if gc.data.width as libc::c_int > 1 as libc::c_int {
            grid_view_set_cell((*s).grid, xx, (*s).cy, &grid_default_cell);
        }
    }
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &mut (*ci).gc as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    grid_view_set_cells(
        (*s).grid,
        (*s).cx,
        (*s).cy,
        &mut gc,
        ((*ci).data).as_mut_ptr(),
        (*ci).used as size_t,
    );
    (*s).cx = ((*s).cx).wrapping_add((*ci).used);
    xx = (*s).cx;
    while xx < (*(*s).grid).sx {
        grid_view_get_cell((*s).grid, xx, (*s).cy, &mut gc);
        if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            break;
        }
        grid_view_set_cell((*s).grid, xx, (*s).cy, &grid_default_cell);
        xx = xx.wrapping_add(1);
        xx;
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_collect_add(
    mut ctx: *mut screen_write_ctx,
    mut gc: *const grid_cell,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut sx: u_int = (*(*s).grid).sx;
    let mut collect: libc::c_int = 0;
    collect = 1 as libc::c_int;
    if (*gc).data.width as libc::c_int != 1 as libc::c_int
        || (*gc).data.size as libc::c_int != 1 as libc::c_int
        || *((*gc).data.data).as_ptr() as libc::c_int >= 0x7f as libc::c_int
        || (*gc).attr as libc::c_int & 0x80 as libc::c_int != 0
        || !(*s).mode & 0x10 as libc::c_int != 0
        || (*s).mode & 0x2 as libc::c_int != 0
        || (*s).sel.flag != 0
    {
        collect = 0 as libc::c_int;
    } 
    if collect == 0 {
        screen_write_collect_end(ctx);
        screen_write_collect_flush(ctx, 0 as libc::c_int);
        screen_write_cell(ctx, gc);
        return;
    }
    (*ctx).cells = ((*ctx).cells).wrapping_add(1);
    (*ctx).cells;
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as u_int)
        || (*(*ctx).item).used
            > sx.wrapping_sub(1 as libc::c_int as u_int).wrapping_sub((*s).cx)
    {
        screen_write_collect_end(ctx);
    }
    ci = (*ctx).item;
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as u_int) {
        log_debug(
            b"%s: wrapped at %u,%u\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"screen_write_collect_add\0"))
                .as_ptr(),
            (*s).cx,
            (*s).cy,
        );
        (*ci).wrapped = 1 as libc::c_int;
        screen_write_linefeed(ctx, 1 as libc::c_int, 8 as libc::c_int as u_int);
        (*s).cx = 0 as libc::c_int as u_int;
    }
    if (*ci).used == 0 as libc::c_int as u_int {
        memcpy(
            &mut (*ci).gc as *mut grid_cell as *mut libc::c_void,
            gc as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
    }
    let fresh8 = (*ci).used;
    (*ci).used = ((*ci).used).wrapping_add(1);
    (*ci)
        .data[fresh8
        as usize] = (*gc).data.data[0 as libc::c_int as usize] as libc::c_char;
    if (*ci).used as libc::c_ulong
        == (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        screen_write_collect_end(ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cell(
    mut ctx: *mut screen_write_ctx,
    mut gc: *const grid_cell,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut tmp_gc: grid_cell = grid_cell {
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
    let mut now_gc: grid_cell = grid_cell {
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
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    let mut width: u_int = (*gc).data.width as u_int;
    let mut xx: u_int = 0;
    let mut last: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut selected: libc::c_int = 0;
    let mut skip: libc::c_int = 1 as libc::c_int;
    if (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0 {
        return;
    }
    (*ctx).cells = ((*ctx).cells).wrapping_add(1);
    (*ctx).cells;
    if width == 0 as libc::c_int as u_int {
        screen_write_collect_flush(ctx, 0 as libc::c_int);
        gc = screen_write_combine(ctx, &(*gc).data, &mut xx);
        if !gc.is_null() {
            cx = (*s).cx;
            cy = (*s).cy;
            screen_write_cursormove(ctx, xx, (*s).cy);
            screen_write_initctx(ctx, &mut ttyctx);
            ttyctx.cell = gc;
            tty_write(
                Some(
                    tty_cmd_cell as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
                ),
                &mut ttyctx,
            );
            (*s).cx = cx;
            (*s).cy = cy;
        }
        return;
    }
    screen_write_collect_flush(ctx, 1 as libc::c_int);
    if !(*s).mode & 0x10 as libc::c_int != 0 && width > 1 as libc::c_int as u_int
        && (width > sx || (*s).cx != sx && (*s).cx > sx.wrapping_sub(width))
    {
        return;
    }
    if (*s).mode & 0x2 as libc::c_int != 0 {
        grid_view_insert_cells(
            (*s).grid,
            (*s).cx,
            (*s).cy,
            width,
            8 as libc::c_int as u_int,
        );
        skip = 0 as libc::c_int;
    }
    if (*s).mode & 0x10 as libc::c_int != 0 && (*s).cx > sx.wrapping_sub(width) {
        log_debug(
            b"%s: wrapped at %u,%u\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"screen_write_cell\0"))
                .as_ptr(),
            (*s).cx,
            (*s).cy,
        );
        screen_write_linefeed(ctx, 1 as libc::c_int, 8 as libc::c_int as u_int);
        (*s).cx = 0 as libc::c_int as u_int;
        screen_write_collect_flush(ctx, 1 as libc::c_int);
    }
    if (*s).cx > sx.wrapping_sub(width)
        || (*s).cy > sy.wrapping_sub(1 as libc::c_int as u_int)
    {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx);
    gl = &mut *((*(*s).grid).linedata)
        .offset(((*(*s).grid).hsize).wrapping_add((*s).cy) as isize) as *mut grid_line;
    if (*gl).flags & 0x2 as libc::c_int != 0 {
        grid_view_get_cell(gd, (*s).cx, (*s).cy, &mut now_gc);
        if screen_write_overwrite(ctx, &mut now_gc, width) != 0 {
            skip = 0 as libc::c_int;
        }
    }
    xx = ((*s).cx).wrapping_add(1 as libc::c_int as u_int);
    while xx < ((*s).cx).wrapping_add(width) {
        log_debug(
            b"%s: new padding at %u,%u\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"screen_write_cell\0"))
                .as_ptr(),
            xx,
            (*s).cy,
        );
        grid_view_set_cell(gd, xx, (*s).cy, &screen_write_pad_cell);
        skip = 0 as libc::c_int;
        xx = xx.wrapping_add(1);
        xx;
    }
    if skip != 0 {
        if (*s).cx >= (*gl).cellsize {
            skip = grid_cells_equal(gc, &grid_default_cell);
        } else {
            gce = &mut *((*gl).celldata).offset((*s).cx as isize)
                as *mut grid_cell_entry;
            if (*gce).flags as libc::c_int & 0x8 as libc::c_int != 0
            || (*gc).flags as libc::c_int != (*gce).flags as libc::c_int
            ||  (*gc).attr as libc::c_int
            != (*gce).c2rust_unnamed.data.attr as libc::c_int
            || (*gc).fg != (*gce).c2rust_unnamed.data.fg as libc::c_int
            || (*gc).bg != (*gce).c2rust_unnamed.data.bg as libc::c_int
            || (*gc).data.width as libc::c_int != 1 as libc::c_int
            || (*gc).data.size as libc::c_int != 1 as libc::c_int
            || (*gce).c2rust_unnamed.data.data as libc::c_int
            != (*gc).data.data[0 as libc::c_int as usize] as libc::c_int
            {
                skip = 0 as libc::c_int;
            } 
        }
    }
    selected = screen_check_selection(s, (*s).cx, (*s).cy);
    if selected != 0 && !((*gc).flags as libc::c_int) & 0x10 as libc::c_int != 0 {
        memcpy(
            &mut tmp_gc as *mut grid_cell as *mut libc::c_void,
            gc as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        tmp_gc.flags = (tmp_gc.flags as libc::c_int | 0x10 as libc::c_int) as u_char;
        grid_view_set_cell(gd, (*s).cx, (*s).cy, &mut tmp_gc);
    } else if selected == 0 && (*gc).flags as libc::c_int & 0x10 as libc::c_int != 0 {
        memcpy(
            &mut tmp_gc as *mut grid_cell as *mut libc::c_void,
            gc as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        tmp_gc.flags = (tmp_gc.flags as libc::c_int & !(0x10 as libc::c_int)) as u_char;
        grid_view_set_cell(gd, (*s).cx, (*s).cy, &mut tmp_gc);
    } else if skip == 0 {
        grid_view_set_cell(gd, (*s).cx, (*s).cy, gc);
    }
    if selected != 0 {
        skip = 0 as libc::c_int;
    }
    last = ((*s).mode & 0x10 as libc::c_int == 0) as libc::c_int as u_int;
    if (*s).cx <= sx.wrapping_sub(last).wrapping_sub(width) {
        (*s).cx = ((*s).cx).wrapping_add(width);
    } else {
        (*s).cx = sx.wrapping_sub(last);
    }
    if (*s).mode & 0x2 as libc::c_int != 0 {
        screen_write_collect_flush(ctx, 0 as libc::c_int);
        ttyctx.num = width;
        tty_write(
            Some(
                tty_cmd_insertcharacter
                    as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
            ),
            &mut ttyctx,
        );
    }
    if skip == 0 {
        if selected != 0 {
            screen_select_cell(s, &mut tmp_gc, gc);
            ttyctx.cell = &mut tmp_gc;
        } else {
            ttyctx.cell = gc;
        }
        tty_write(
            Some(tty_cmd_cell as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> ()),
            &mut ttyctx,
        );
        (*ctx).written = ((*ctx).written).wrapping_add(1);
        (*ctx).written;
    } else {
        (*ctx).skipped = ((*ctx).skipped).wrapping_add(1);
        (*ctx).skipped;
    };
}
unsafe extern "C" fn screen_write_combine(
    mut ctx: *mut screen_write_ctx,
    mut ud: *const utf8_data,
    mut xx: *mut u_int,
) -> *const grid_cell {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    static mut gc: grid_cell = grid_cell {
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
    let mut n: u_int = 0;
    if (*s).cx == 0 as libc::c_int as u_int {
        return 0 as *const grid_cell;
    }
    if (*ud).size as libc::c_int == 0 as libc::c_int {
        fatalx(b"UTF-8 data empty\0" as *const u8 as *const libc::c_char);
    }
    n = 1 as libc::c_int as u_int;
    while n <= (*s).cx {
        grid_view_get_cell(gd, ((*s).cx).wrapping_sub(n), (*s).cy, &mut gc);
        if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            break;
        }
        n = n.wrapping_add(1);
        n;
    }
    if n > (*s).cx {
        return 0 as *const grid_cell;
    }
    *xx = ((*s).cx).wrapping_sub(n);
    if (gc.data.size as libc::c_int + (*ud).size as libc::c_int) as libc::c_ulong
        > ::core::mem::size_of::<[u_char; 9]>() as libc::c_ulong
    {
        return 0 as *const grid_cell;
    }
    log_debug(
        b"%s: %.*s onto %.*s at %u,%u\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"screen_write_combine\0"))
            .as_ptr(),
        (*ud).size as libc::c_int,
        ((*ud).data).as_ptr(),
        gc.data.size as libc::c_int,
        (gc.data.data).as_mut_ptr(),
        *xx,
        (*s).cy,
    );
    memcpy(
        (gc.data.data).as_mut_ptr().offset(gc.data.size as libc::c_int as isize)
            as *mut libc::c_void,
        ((*ud).data).as_ptr() as *const libc::c_void,
        (*ud).size as libc::c_ulong,
    );
    gc.data.size = (gc.data.size as libc::c_int + (*ud).size as libc::c_int) as u_char;
    grid_view_set_cell(gd, *xx, (*s).cy, &mut gc);
    return &mut gc;
}
unsafe extern "C" fn screen_write_overwrite(
    mut ctx: *mut screen_write_ctx,
    mut gc: *mut grid_cell,
    mut width: u_int,
) -> libc::c_int {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut tmp_gc: grid_cell = grid_cell {
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
    let mut xx: u_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    if (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0 {
        xx = ((*s).cx).wrapping_add(1 as libc::c_int as u_int);
        loop {
            xx = xx.wrapping_sub(1);
            if !(xx > 0 as libc::c_int as u_int) {
                break;
            }
            grid_view_get_cell(gd, xx, (*s).cy, &mut tmp_gc);
            if !(tmp_gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            log_debug(
                b"%s: padding at %u,%u\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"screen_write_overwrite\0"))
                    .as_ptr(),
                xx,
                (*s).cy,
            );
            grid_view_set_cell(gd, xx, (*s).cy, &grid_default_cell);
        }
        log_debug(
            b"%s: character at %u,%u\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"screen_write_overwrite\0"))
                .as_ptr(),
            xx,
            (*s).cy,
        );
        grid_view_set_cell(gd, xx, (*s).cy, &grid_default_cell);
        done = 1 as libc::c_int;
    }
    if width != 1 as libc::c_int as u_int
        || (*gc).data.width as libc::c_int != 1 as libc::c_int
        || (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0
    {
        xx = ((*s).cx).wrapping_add(width).wrapping_sub(1 as libc::c_int as u_int);
        loop {
            xx = xx.wrapping_add(1);
            if !(xx < (*(*s).grid).sx) {
                break;
            }
            grid_view_get_cell(gd, xx, (*s).cy, &mut tmp_gc);
            if !(tmp_gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            log_debug(
                b"%s: overwrite at %u,%u\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"screen_write_overwrite\0"))
                    .as_ptr(),
                xx,
                (*s).cy,
            );
            grid_view_set_cell(gd, xx, (*s).cy, &grid_default_cell);
            done = 1 as libc::c_int;
        }
    }
    return done;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_setselection(
    mut ctx: *mut screen_write_ctx,
    mut str: *mut u_char,
    mut len: u_int,
) {
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.ptr = str as *mut libc::c_void;
    ttyctx.num = len;
    tty_write(
        Some(
            tty_cmd_setselection as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_rawstring(
    mut ctx: *mut screen_write_ctx,
    mut str: *mut u_char,
    mut len: u_int,
) {
    let mut ttyctx: tty_ctx = tty_ctx {
        wp: 0 as *mut window_pane,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        bg: 0,
    };
    screen_write_initctx(ctx, &mut ttyctx);
    ttyctx.ptr = str as *mut libc::c_void;
    ttyctx.num = len;
    tty_write(
        Some(tty_cmd_rawstring as unsafe extern "C" fn(*mut tty, *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
