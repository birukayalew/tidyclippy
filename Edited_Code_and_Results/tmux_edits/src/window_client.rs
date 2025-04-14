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
    pub type mode_tree_data;
    pub type screen_write_collect_item;
    pub type screen_write_collect_line;
    pub type mode_tree_item;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    fn format_single(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    static mut clients: clients;
    fn server_client_how_many() -> u_int;
    fn server_client_unref(_: *mut client);
    fn server_client_suspend(_: *mut client);
    fn server_client_detach(_: *mut client, _: msgtype);
    fn screen_write_fast_copy(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    fn screen_write_hline(
        _: *mut screen_write_ctx,
        _: u_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn screen_write_preview(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
    );
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn window_pane_reset_mode(_: *mut window_pane);
    fn mode_tree_get_current(_: *mut mode_tree_data) -> *mut libc::c_void;
    fn mode_tree_each_tagged(
        _: *mut mode_tree_data,
        _: mode_tree_each_cb,
        _: *mut client,
        _: key_code,
        _: libc::c_int,
    );
    fn mode_tree_down(_: *mut mode_tree_data, _: libc::c_int);
    fn mode_tree_start(
        _: *mut window_pane,
        _: *mut args,
        _: mode_tree_build_cb,
        _: mode_tree_draw_cb,
        _: mode_tree_search_cb,
        _: *mut libc::c_void,
        _: *mut *const libc::c_char,
        _: u_int,
        _: *mut *mut screen,
    ) -> *mut mode_tree_data;
    fn mode_tree_zoom(_: *mut mode_tree_data, _: *mut args);
    fn mode_tree_build(_: *mut mode_tree_data);
    fn mode_tree_free(_: *mut mode_tree_data);
    fn mode_tree_resize(_: *mut mode_tree_data, _: u_int, _: u_int);
    fn mode_tree_add(
        _: *mut mode_tree_data,
        _: *mut mode_tree_item,
        _: *mut libc::c_void,
        _: uint64_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut mode_tree_item;
    fn mode_tree_draw(_: *mut mode_tree_data);
    fn mode_tree_key(
        _: *mut mode_tree_data,
        _: *mut client,
        _: *mut key_code,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    fn mode_tree_run_command(
        _: *mut client,
        _: *mut cmd_find_state,
        _: *const libc::c_char,
        _: *const libc::c_char,
    );
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub type mode_tree_build_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        u_int,
        *mut uint64_t,
        *const libc::c_char,
    ) -> (),
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
pub type mode_tree_search_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub type mode_tree_each_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut client,
        key_code,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_client_modedata {
    pub data: *mut mode_tree_data,
    pub format: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub item_list: *mut *mut window_client_itemdata,
    pub item_size: u_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_client_itemdata {
    pub c: *mut client,
}
pub const WINDOW_CLIENT_BY_ACTIVITY_TIME: window_client_sort_type = 3;
pub const WINDOW_CLIENT_BY_CREATION_TIME: window_client_sort_type = 2;
pub const WINDOW_CLIENT_BY_SIZE: window_client_sort_type = 1;
pub const WINDOW_CLIENT_BY_NAME: window_client_sort_type = 0;
pub type window_client_sort_type = libc::c_uint;
#[no_mangle]
pub static mut window_client_mode: window_mode = unsafe {
    {
        let mut init = window_mode {
            name: b"client-mode\0" as *const u8 as *const libc::c_char,
            init: Some(
                window_client_init
                    as unsafe extern "C" fn(
                        *mut window_pane,
                        *mut cmd_find_state,
                        *mut args,
                    ) -> *mut screen,
            ),
            free: Some(
                window_client_free as unsafe extern "C" fn(*mut window_pane) -> (),
            ),
            resize: Some(
                window_client_resize
                    as unsafe extern "C" fn(*mut window_pane, u_int, u_int) -> (),
            ),
            key: Some(
                window_client_key
                    as unsafe extern "C" fn(
                        *mut window_pane,
                        *mut client,
                        *mut session,
                        key_code,
                        *mut mouse_event,
                    ) -> (),
            ),
            key_table: None,
            command: None,
        };
        init
    }
};
static mut window_client_sort_list: [*const libc::c_char; 4] = [
    b"name\0" as *const u8 as *const libc::c_char,
    b"size\0" as *const u8 as *const libc::c_char,
    b"creation\0" as *const u8 as *const libc::c_char,
    b"activity\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn window_client_add_item(
    mut data: *mut window_client_modedata,
) -> *mut window_client_itemdata {
    let mut item: *mut window_client_itemdata = 0 as *mut window_client_itemdata;
    (*data)
        .item_list = xreallocarray(
        (*data).item_list as *mut libc::c_void,
        ((*data).item_size).wrapping_add(1 as libc::c_int as u_int) as size_t,
        ::core::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
    ) as *mut *mut window_client_itemdata;
    let fresh0 = (*data).item_size;
    (*data).item_size = ((*data).item_size).wrapping_add(1);
    let ref mut fresh1 = *((*data).item_list).offset(fresh0 as isize);
    *fresh1 = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<window_client_itemdata>() as libc::c_ulong,
    ) as *mut window_client_itemdata;
    item = *fresh1;
    return item;
}
unsafe extern "C" fn window_client_free_item(mut item: *mut window_client_itemdata) {
    server_client_unref((*item).c);
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_client_cmp_name(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_client_itemdata = a0
        as *const *const window_client_itemdata;
    let mut b: *const *const window_client_itemdata = b0
        as *const *const window_client_itemdata;
    return strcmp((*(**a).c).name, (*(**b).c).name);
}
unsafe extern "C" fn window_client_cmp_size(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_client_itemdata = a0
        as *const *const window_client_itemdata;
    let mut b: *const *const window_client_itemdata = b0
        as *const *const window_client_itemdata;
    if (*(**a).c).tty.sx < (*(**b).c).tty.sx {
        return -(1 as libc::c_int);
    }
    if (*(**a).c).tty.sx > (*(**b).c).tty.sx {
        return 1 as libc::c_int;
    }
    if (*(**a).c).tty.sy < (*(**b).c).tty.sy {
        return -(1 as libc::c_int);
    }
    if (*(**a).c).tty.sy > (*(**b).c).tty.sy {
        return 1 as libc::c_int;
    }
    return strcmp((*(**a).c).name, (*(**b).c).name);
}
unsafe extern "C" fn window_client_cmp_creation_time(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_client_itemdata = a0
        as *const *const window_client_itemdata;
    let mut b: *const *const window_client_itemdata = b0
        as *const *const window_client_itemdata;
    if if (*(**a).c).creation_time.tv_sec == (*(**b).c).creation_time.tv_sec {
        ((*(**a).c).creation_time.tv_usec > (*(**b).c).creation_time.tv_usec)
            as libc::c_int
    } else {
        ((*(**a).c).creation_time.tv_sec > (*(**b).c).creation_time.tv_sec)
            as libc::c_int
    } != 0
    {
        return -(1 as libc::c_int);
    }
    if if (*(**a).c).creation_time.tv_sec == (*(**b).c).creation_time.tv_sec {
        ((*(**a).c).creation_time.tv_usec < (*(**b).c).creation_time.tv_usec)
            as libc::c_int
    } else {
        ((*(**a).c).creation_time.tv_sec < (*(**b).c).creation_time.tv_sec)
            as libc::c_int
    } != 0
    {
        return 1 as libc::c_int;
    }
    return strcmp((*(**a).c).name, (*(**b).c).name);
}
unsafe extern "C" fn window_client_cmp_activity_time(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_client_itemdata = a0
        as *const *const window_client_itemdata;
    let mut b: *const *const window_client_itemdata = b0
        as *const *const window_client_itemdata;
    if if (*(**a).c).activity_time.tv_sec == (*(**b).c).activity_time.tv_sec {
        ((*(**a).c).activity_time.tv_usec > (*(**b).c).activity_time.tv_usec)
            as libc::c_int
    } else {
        ((*(**a).c).activity_time.tv_sec > (*(**b).c).activity_time.tv_sec)
            as libc::c_int
    } != 0
    {
        return -(1 as libc::c_int);
    }
    if if (*(**a).c).activity_time.tv_sec == (*(**b).c).activity_time.tv_sec {
        ((*(**a).c).activity_time.tv_usec < (*(**b).c).activity_time.tv_usec)
            as libc::c_int
    } else {
        ((*(**a).c).activity_time.tv_sec < (*(**b).c).activity_time.tv_sec)
            as libc::c_int
    } != 0
    {
        return 1 as libc::c_int;
    }
    return strcmp((*(**a).c).name, (*(**b).c).name);
}
unsafe extern "C" fn window_client_build(
    mut modedata: *mut libc::c_void,
    mut sort_type: u_int,
    mut tag: *mut uint64_t,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_client_modedata = modedata as *mut window_client_modedata;
    let mut item: *mut window_client_itemdata = 0 as *mut window_client_itemdata;
    let mut i: u_int = 0;
    let mut c: *mut client = 0 as *mut client;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        window_client_free_item(*((*data).item_list).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_client_itemdata;
    (*data).item_size = 0 as libc::c_int as u_int;
    c = clients.tqh_first;
    while !c.is_null() {
        if !(((*c).session).is_null() || (*c).flags & 0x1000 as libc::c_int != 0) {
            item = window_client_add_item(data);
            (*item).c = c;
            (*c).references += 1;
            (*c).references;
        }
        c = (*c).entry.tqe_next;
    }
    match sort_type {
        0 => {
            qsort(
                (*data).item_list as *mut libc::c_void,
                (*data).item_size as size_t,
                ::core::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
                Some(
                    window_client_cmp_name
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        1 => {
            qsort(
                (*data).item_list as *mut libc::c_void,
                (*data).item_size as size_t,
                ::core::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
                Some(
                    window_client_cmp_size
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        2 => {
            qsort(
                (*data).item_list as *mut libc::c_void,
                (*data).item_size as size_t,
                ::core::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
                Some(
                    window_client_cmp_creation_time
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        3 => {
            qsort(
                (*data).item_list as *mut libc::c_void,
                (*data).item_size as size_t,
                ::core::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
                Some(
                    window_client_cmp_activity_time
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        _ => {}
    }
    let mut current_block_24: u64;
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        item = *((*data).item_list).offset(i as isize);
        c = (*item).c;
        if !filter.is_null() {
            cp = format_single(
                0 as *mut cmdq_item,
                filter,
                c,
                0 as *mut session,
                0 as *mut winlink,
                0 as *mut window_pane,
            );
            if format_true(cp) == 0 {
                free(cp as *mut libc::c_void);
                current_block_24 = 12124785117276362961;
            } else {
                free(cp as *mut libc::c_void);
                current_block_24 = 4068382217303356765;
            }
        } else {
            current_block_24 = 4068382217303356765;
        }
        match current_block_24 {
            4068382217303356765 => {
                text = format_single(
                    0 as *mut cmdq_item,
                    (*data).format,
                    c,
                    0 as *mut session,
                    0 as *mut winlink,
                    0 as *mut window_pane,
                );
                mode_tree_add(
                    (*data).data,
                    0 as *mut mode_tree_item,
                    item as *mut libc::c_void,
                    c as uint64_t,
                    (*c).name,
                    text,
                    -(1 as libc::c_int),
                );
                free(text as *mut libc::c_void);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn window_client_draw(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut item: *mut window_client_itemdata = itemdata as *mut window_client_itemdata;
    let mut c: *mut client = (*item).c;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    if ((*c).session).is_null()
        || (*c).flags & (0x200 as libc::c_int | 0x1000 as libc::c_int) != 0
    {
        return;
    }
    wp = (*(*(*(*c).session).curw).window).active;
    screen_write_preview(
        ctx,
        &mut (*wp).base,
        sx,
        sy.wrapping_sub(3 as libc::c_int as u_int),
    );
    screen_write_cursormove(
        ctx,
        cx,
        cy.wrapping_add(sy).wrapping_sub(2 as libc::c_int as u_int),
    );
    screen_write_hline(ctx, sx, 0 as libc::c_int, 0 as libc::c_int);
    screen_write_cursormove(
        ctx,
        cx,
        cy.wrapping_add(sy).wrapping_sub(1 as libc::c_int as u_int),
    );
    if !((*c).status.old_status).is_null() {
        screen_write_fast_copy(
            ctx,
            (*c).status.old_status,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            sx,
            1 as libc::c_int as u_int,
        );
    } else {
        screen_write_fast_copy(
            ctx,
            &mut (*c).status.status,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            sx,
            1 as libc::c_int as u_int,
        );
    };
}
unsafe extern "C" fn window_client_init(
    mut wp: *mut window_pane,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut screen {
    let mut data: *mut window_client_modedata = 0 as *mut window_client_modedata;
    let mut s: *mut screen = 0 as *mut screen;
    data = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<window_client_modedata>() as libc::c_ulong,
    ) as *mut window_client_modedata;
    (*wp).modedata = data as *mut libc::c_void;
    if args.is_null() || args_has(args, 'F' as i32 as u_char) == 0 {
        (*data)
            .format = xstrdup(
            b"session #{session_name} (#{client_width}x#{client_height}, #{t:client_activity})\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).format = xstrdup(args_get(args, 'F' as i32 as u_char));
    }
    if args.is_null() || (*args).argc == 0 as libc::c_int {
        (*data)
            .command = xstrdup(
            b"detach-client -t '%%'\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).command = xstrdup(*((*args).argv).offset(0 as libc::c_int as isize));
    }
    (*data)
        .data = mode_tree_start(
        wp,
        args,
        Some(
            window_client_build
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    u_int,
                    *mut uint64_t,
                    *const libc::c_char,
                ) -> (),
        ),
        Some(
            window_client_draw
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut screen_write_ctx,
                    u_int,
                    u_int,
                ) -> (),
        ),
        None,
        data as *mut libc::c_void,
        window_client_sort_list.as_mut_ptr(),
        (::core::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as u_int,
        &mut s,
    );
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    return s;
}
unsafe extern "C" fn window_client_free(mut wp: *mut window_pane) {
    let mut data: *mut window_client_modedata = (*wp).modedata
        as *mut window_client_modedata;
    let mut i: u_int = 0;
    if data.is_null() {
        return;
    }
    mode_tree_free((*data).data);
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        window_client_free_item(*((*data).item_list).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free((*data).item_list as *mut libc::c_void);
    free((*data).format as *mut libc::c_void);
    free((*data).command as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_client_resize(
    mut wp: *mut window_pane,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_client_modedata = (*wp).modedata
        as *mut window_client_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_client_do_detach(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut c: *mut client,
    mut key: key_code,
) {
    let mut data: *mut window_client_modedata = modedata as *mut window_client_modedata;
    let mut item: *mut window_client_itemdata = itemdata as *mut window_client_itemdata;
    if item == mode_tree_get_current((*data).data) as *mut window_client_itemdata {
        mode_tree_down((*data).data, 0 as libc::c_int);
    }
    if key == 'd' as i32 as key_code || key == 'D' as i32 as key_code {
        server_client_detach((*item).c, MSG_DETACH);
    } else if key == 'x' as i32 as key_code || key == 'X' as i32 as key_code {
        server_client_detach((*item).c, MSG_DETACHKILL);
    } else if key == 'z' as i32 as key_code || key == 'Z' as i32 as key_code {
        server_client_suspend((*item).c);
    }
}
unsafe extern "C" fn window_client_key(
    mut wp: *mut window_pane,
    mut c: *mut client,
    mut s: *mut session,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut data: *mut window_client_modedata = (*wp).modedata
        as *mut window_client_modedata;
    let mut mtd: *mut mode_tree_data = (*data).data;
    let mut item: *mut window_client_itemdata = 0 as *mut window_client_itemdata;
    let mut finished: libc::c_int = 0;
    finished = mode_tree_key(mtd, c, &mut key, m, 0 as *mut u_int, 0 as *mut u_int);
    match key {
        100 | 120 | 122 => {
            item = mode_tree_get_current(mtd) as *mut window_client_itemdata;
            window_client_do_detach(
                data as *mut libc::c_void,
                item as *mut libc::c_void,
                c,
                key,
            );
            mode_tree_build(mtd);
        }
        68 | 88 | 90 => {
            mode_tree_each_tagged(
                mtd,
                Some(
                    window_client_do_detach
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            *mut client,
                            key_code,
                        ) -> (),
                ),
                c,
                key,
                0 as libc::c_int,
            );
            mode_tree_build(mtd);
        }
        13 => {
            item = mode_tree_get_current(mtd) as *mut window_client_itemdata;
            mode_tree_run_command(
                c,
                0 as *mut cmd_find_state,
                (*data).command,
                (*(*item).c).ttyname,
            );
            finished = 1 as libc::c_int;
        }
        _ => {}
    }
    if finished != 0 || server_client_how_many() == 0 as libc::c_int as u_int {
        window_pane_reset_mode(wp);
    } else {
        mode_tree_draw(mtd);
        (*wp).flags |= 0x1 as libc::c_int;
    };
}
