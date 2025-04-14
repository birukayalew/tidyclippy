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
    pub type paste_buffer;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t) -> *const libc::c_char;
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut paste_buffer;
    fn paste_get_name(_: *const libc::c_char) -> *mut paste_buffer;
    fn paste_set(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn format_add(
        _: *mut format_tree,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    fn format_single(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
    fn options_get_string(
        _: *mut options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn job_run(
        _: *const libc::c_char,
        _: *mut session,
        _: *const libc::c_char,
        _: job_update_cb,
        _: job_complete_cb,
        _: job_free_cb,
        _: *mut libc::c_void,
        _: libc::c_int,
    ) -> *mut job;
    fn tty_acs_get(_: *mut tty, _: u_char) -> *const libc::c_char;
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn cmd_mouse_pane(
        _: *mut mouse_event,
        _: *mut *mut session,
        _: *mut *mut winlink,
    ) -> *mut window_pane;
    static grid_default_cell: grid_cell;
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell);
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    fn screen_write_stop(_: *mut screen_write_ctx);
    fn screen_write_strlen(_: *const libc::c_char, _: ...) -> size_t;
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
    fn screen_write_vnputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    );
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const grid_cell, _: u_char);
    fn screen_write_copy(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *mut bitstr_t,
        _: *const grid_cell,
    );
    fn screen_write_insertline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_deleteline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int, _: u_int);
    fn screen_write_carriagereturn(_: *mut screen_write_ctx);
    fn screen_write_setselection(_: *mut screen_write_ctx, _: *mut u_char, _: u_int);
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn screen_free(_: *mut screen);
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    fn screen_set_selection(
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *mut grid_cell,
    );
    fn screen_clear_selection(_: *mut screen);
    fn screen_hide_selection(_: *mut screen);
    fn window_pane_reset_mode(_: *mut window_pane);
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub state: C2RustUnnamed_35,
    pub flags: libc::c_int,
    pub cmd: *mut libc::c_char,
    pub pid: pid_t,
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub updatecb: job_update_cb,
    pub completecb: job_complete_cb,
    pub freecb: job_free_cb,
    pub data: *mut libc::c_void,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type job_free_cb = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type job_complete_cb = Option::<unsafe extern "C" fn(*mut job) -> ()>;
pub type job_update_cb = Option::<unsafe extern "C" fn(*mut job) -> ()>;
pub type C2RustUnnamed_35 = libc::c_uint;
pub const JOB_CLOSED: C2RustUnnamed_35 = 2;
pub const JOB_DEAD: C2RustUnnamed_35 = 1;
pub const JOB_RUNNING: C2RustUnnamed_35 = 0;
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
pub struct window_copy_mode_data {
    pub screen: screen,
    pub backing: *mut screen,
    pub backing_written: libc::c_int,
    pub oy: u_int,
    pub selx: u_int,
    pub sely: u_int,
    pub endselx: u_int,
    pub endsely: u_int,
    pub cursordrag: C2RustUnnamed_36,
    pub rectflag: libc::c_int,
    pub scroll_exit: libc::c_int,
    pub cx: u_int,
    pub cy: u_int,
    pub lastcx: u_int,
    pub lastsx: u_int,
    pub searchtype: libc::c_int,
    pub searchstr: *mut libc::c_char,
    pub searchmark: *mut bitstr_t,
    pub searchcount: u_int,
    pub searchthis: libc::c_int,
    pub searchx: libc::c_int,
    pub searchy: libc::c_int,
    pub searcho: libc::c_int,
    pub jumptype: libc::c_int,
    pub jumpchar: libc::c_char,
}
pub type C2RustUnnamed_36 = libc::c_uint;
pub const CURSORDRAG_SEL: C2RustUnnamed_36 = 2;
pub const CURSORDRAG_ENDSEL: C2RustUnnamed_36 = 1;
pub const CURSORDRAG_NONE: C2RustUnnamed_36 = 0;
pub const WINDOW_COPY_REL_POS_ON_SCREEN: C2RustUnnamed_38 = 1;
pub const WINDOW_COPY_REL_POS_BELOW: C2RustUnnamed_38 = 2;
pub const WINDOW_COPY_REL_POS_ABOVE: C2RustUnnamed_38 = 0;
pub const WINDOW_COPY_SEARCHUP: C2RustUnnamed_37 = 1;
pub const WINDOW_COPY_SEARCHDOWN: C2RustUnnamed_37 = 2;
pub const WINDOW_COPY_JUMPTOFORWARD: C2RustUnnamed_37 = 5;
pub const WINDOW_COPY_JUMPTOBACKWARD: C2RustUnnamed_37 = 6;
pub const WINDOW_COPY_JUMPFORWARD: C2RustUnnamed_37 = 3;
pub const WINDOW_COPY_JUMPBACKWARD: C2RustUnnamed_37 = 4;
pub const WINDOW_COPY_OFF: C2RustUnnamed_37 = 0;
pub type C2RustUnnamed_37 = libc::c_uint;
pub type C2RustUnnamed_38 = libc::c_uint;
#[no_mangle]
pub static mut window_copy_mode: window_mode = unsafe {
    {
        let mut init = window_mode {
            name: b"copy-mode\0" as *const u8 as *const libc::c_char,
            init: Some(
                window_copy_init
                    as unsafe extern "C" fn(
                        *mut window_pane,
                        *mut cmd_find_state,
                        *mut args,
                    ) -> *mut screen,
            ),
            free: Some(window_copy_free as unsafe extern "C" fn(*mut window_pane) -> ()),
            resize: Some(
                window_copy_resize
                    as unsafe extern "C" fn(*mut window_pane, u_int, u_int) -> (),
            ),
            key: None,
            key_table: Some(
                window_copy_key_table
                    as unsafe extern "C" fn(*mut window_pane) -> *const libc::c_char,
            ),
            command: Some(
                window_copy_command
                    as unsafe extern "C" fn(
                        *mut window_pane,
                        *mut client,
                        *mut session,
                        *mut args,
                        *mut mouse_event,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn window_copy_init(
    mut wp: *mut window_pane,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut screen {
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut s: *mut screen = 0 as *mut screen;
    data = xmalloc(::core::mem::size_of::<window_copy_mode_data>() as libc::c_ulong)
        as *mut window_copy_mode_data;
    (*wp).modedata = data as *mut libc::c_void;
    (*data).oy = 0 as libc::c_int as u_int;
    (*data).cx = 0 as libc::c_int as u_int;
    (*data).cy = 0 as libc::c_int as u_int;
    (*data).cursordrag = CURSORDRAG_NONE;
    (*data).lastcx = 0 as libc::c_int as u_int;
    (*data).lastsx = 0 as libc::c_int as u_int;
    (*data).backing_written = 0 as libc::c_int;
    (*data).rectflag = 0 as libc::c_int;
    (*data).scroll_exit = 0 as libc::c_int;
    if !((*wp).searchstr).is_null() {
        (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
        (*data).searchstr = xstrdup((*wp).searchstr);
    } else {
        (*data).searchtype = WINDOW_COPY_OFF as libc::c_int;
        (*data).searchstr = 0 as *mut libc::c_char;
    }
    (*data).searchmark = 0 as *mut bitstr_t;
    (*data).searcho = -(1 as libc::c_int);
    (*data).searchy = (*data).searcho;
    (*data).searchx = (*data).searchy;
    if (*wp).fd != -(1 as libc::c_int) {
        bufferevent_disable(
            (*wp).event,
            (0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_short,
        );
    }
    (*data).jumptype = WINDOW_COPY_OFF as libc::c_int;
    (*data).jumpchar = '\0' as i32 as libc::c_char;
    s = &mut (*data).screen;
    screen_init(
        s,
        (*(*wp).base.grid).sx,
        (*(*wp).base.grid).sy,
        0 as libc::c_int as u_int,
    );
    (*s)
        .sel
        .modekeys = options_get_number(
        (*(*wp).window).options,
        b"mode-keys\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    (*data).backing = 0 as *mut screen;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_init_from_pane(
    mut wp: *mut window_pane,
    mut scroll_exit: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
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
    let mut i: u_int = 0;
    if (*wp).mode != &window_copy_mode as *const window_mode {
        fatalx(b"not in copy mode\0" as *const u8 as *const libc::c_char);
    }
    (*data).backing = &mut (*wp).base;
    (*data).cx = (*(*data).backing).cx;
    (*data).cy = (*(*data).backing).cy;
    (*data).scroll_exit = scroll_exit;
    (*s).cx = (*data).cx;
    (*s).cy = (*data).cy;
    screen_write_start(&mut ctx, 0 as *mut window_pane, s);
    i = 0 as libc::c_int as u_int;
    while i < (*(*s).grid).sy {
        window_copy_write_line(wp, &mut ctx, i);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_cursormove(&mut ctx, (*data).cx, (*data).cy);
    screen_write_stop(&mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_init_for_output(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    (*data)
        .backing = xmalloc(::core::mem::size_of::<screen>() as libc::c_ulong)
        as *mut screen;
    screen_init(
        (*data).backing,
        (*(*wp).base.grid).sx,
        (*(*wp).base.grid).sy,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
    );
}
unsafe extern "C" fn window_copy_free(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    if (*wp).fd != -(1 as libc::c_int) {
        bufferevent_enable(
            (*wp).event,
            (0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_short,
        );
    }
    free((*data).searchmark as *mut libc::c_void);
    free((*data).searchstr as *mut libc::c_void);
    if (*data).backing != &mut (*wp).base as *mut screen {
        screen_free((*data).backing);
        free((*data).backing as *mut libc::c_void);
    }
    screen_free(&mut (*data).screen);
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_vadd(
    mut wp: *mut window_pane,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut backing: *mut screen = (*data).backing;
    let mut back_ctx: screen_write_ctx = screen_write_ctx {
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
    let mut old_hsize: u_int = 0;
    let mut old_cy: u_int = 0;
    if backing == &mut (*wp).base as *mut screen {
        return;
    }
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    old_hsize = (*(*(*data).backing).grid).hsize;
    screen_write_start(&mut back_ctx, 0 as *mut window_pane, backing);
    if (*data).backing_written != 0 {
        screen_write_carriagereturn(&mut back_ctx);
        screen_write_linefeed(
            &mut back_ctx,
            0 as libc::c_int,
            8 as libc::c_int as u_int,
        );
    } else {
        (*data).backing_written = 1 as libc::c_int;
    }
    old_cy = (*backing).cy;
    screen_write_vnputs(
        &mut back_ctx,
        0 as libc::c_int as ssize_t,
        &mut gc,
        fmt,
        ap.as_va_list(),
    );
    screen_write_stop(&mut back_ctx);
    (*data)
        .oy = ((*data).oy)
        .wrapping_add(((*(*(*data).backing).grid).hsize).wrapping_sub(old_hsize));
    screen_write_start(&mut ctx, wp, &mut (*data).screen);
    if (*(*(*data).backing).grid).hsize != 0 {
        window_copy_redraw_lines(
            wp,
            0 as libc::c_int as u_int,
            1 as libc::c_int as u_int,
        );
    }
    window_copy_redraw_lines(
        wp,
        old_cy,
        ((*backing).cy).wrapping_sub(old_cy).wrapping_add(1 as libc::c_int as u_int),
    );
    screen_write_stop(&mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_pageup(
    mut wp: *mut window_pane,
    mut half_page: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut n: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint && oy == (*data).sely
    {
        window_copy_other_end(wp);
    }
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox;
    }
    (*data).cx = (*data).lastcx;
    n = 1 as libc::c_int as u_int;
    if (*(*s).grid).sy > 2 as libc::c_int as u_int {
        if half_page != 0 {
            n = (*(*s).grid).sy / 2 as libc::c_int as u_int;
        } else {
            n = ((*(*s).grid).sy).wrapping_sub(2 as libc::c_int as u_int);
        }
    }
    if ((*data).oy).wrapping_add(n) > (*(*(*data).backing).grid).hsize {
        (*data).oy = (*(*(*data).backing).grid).hsize;
    } else {
        (*data).oy = ((*data).oy).wrapping_add(n);
    }
    if (*data).screen.sel.flag == 0 || (*data).rectflag == 0 {
        py = ((*(*(*data).backing).grid).hsize)
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wp);
        }
    }
    window_copy_update_selection(wp, 1 as libc::c_int);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_pagedown(
    mut wp: *mut window_pane,
    mut half_page: libc::c_int,
    mut scroll_exit: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut n: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint && oy == (*data).sely
    {
        window_copy_other_end(wp);
    }
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox;
    }
    (*data).cx = (*data).lastcx;
    n = 1 as libc::c_int as u_int;
    if (*(*s).grid).sy > 2 as libc::c_int as u_int {
        if half_page != 0 {
            n = (*(*s).grid).sy / 2 as libc::c_int as u_int;
        } else {
            n = ((*(*s).grid).sy).wrapping_sub(2 as libc::c_int as u_int);
        }
    }
    if (*data).oy < n {
        (*data).oy = 0 as libc::c_int as u_int;
    } else {
        (*data).oy = ((*data).oy).wrapping_sub(n);
    }
    if (*data).screen.sel.flag == 0 || (*data).rectflag == 0 {
        py = ((*(*(*data).backing).grid).hsize)
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wp);
        }
    }
    if scroll_exit != 0 && (*data).oy == 0 as libc::c_int as u_int {
        return 1 as libc::c_int;
    }
    window_copy_update_selection(wp, 1 as libc::c_int);
    window_copy_redraw_screen(wp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_previous_paragraph(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut oy: u_int = 0;
    oy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    while oy > 0 as libc::c_int as u_int
        && window_copy_find_length(wp, oy) == 0 as libc::c_int as u_int
    {
        oy = oy.wrapping_sub(1);
        oy;
    }
    while oy > 0 as libc::c_int as u_int
        && window_copy_find_length(wp, oy) > 0 as libc::c_int as u_int
    {
        oy = oy.wrapping_sub(1);
        oy;
    }
    window_copy_scroll_to(wp, 0 as libc::c_int as u_int, oy);
}
unsafe extern "C" fn window_copy_next_paragraph(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut maxy: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    oy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    maxy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*(*s).grid).sy)
        .wrapping_sub(1 as libc::c_int as u_int);
    while oy < maxy && window_copy_find_length(wp, oy) == 0 as libc::c_int as u_int {
        oy = oy.wrapping_add(1);
        oy;
    }
    while oy < maxy && window_copy_find_length(wp, oy) > 0 as libc::c_int as u_int {
        oy = oy.wrapping_add(1);
        oy;
    }
    ox = window_copy_find_length(wp, oy);
    window_copy_scroll_to(wp, ox, oy);
}
unsafe extern "C" fn window_copy_resize(
    mut wp: *mut window_pane,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
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
    screen_resize(s, sx, sy, 1 as libc::c_int);
    if (*data).backing != &mut (*wp).base as *mut screen {
        screen_resize((*data).backing, sx, sy, 1 as libc::c_int);
    }
    if (*data).cy > sy.wrapping_sub(1 as libc::c_int as u_int) {
        (*data).cy = sy.wrapping_sub(1 as libc::c_int as u_int);
    }
    if (*data).cx > sx {
        (*data).cx = sx;
    }
    if (*data).oy > (*(*(*data).backing).grid).hsize {
        (*data).oy = (*(*(*data).backing).grid).hsize;
    }
    window_copy_clear_selection(wp);
    screen_write_start(&mut ctx, 0 as *mut window_pane, s);
    window_copy_write_lines(
        wp,
        &mut ctx,
        0 as libc::c_int as u_int,
        ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int),
    );
    screen_write_stop(&mut ctx);
    if !((*data).searchmark).is_null() {
        window_copy_search_marks(wp, 0 as *mut screen);
    }
    (*data).searchx = (*data).cx as libc::c_int;
    (*data).searchy = (*data).cy as libc::c_int;
    (*data).searcho = (*data).oy as libc::c_int;
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_key_table(
    mut wp: *mut window_pane,
) -> *const libc::c_char {
    if options_get_number(
        (*(*wp).window).options,
        b"mode-keys\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int as libc::c_longlong
    {
        return b"copy-mode-vi\0" as *const u8 as *const libc::c_char;
    }
    return b"copy-mode\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn window_copy_command(
    mut wp: *mut window_pane,
    mut c: *mut client,
    mut s: *mut session,
    mut args: *mut args,
    mut m: *mut mouse_event,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut sn: *mut screen = &mut (*data).screen;
    let mut command: *const libc::c_char = 0 as *const libc::c_char;
    let mut argument: *const libc::c_char = 0 as *const libc::c_char;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    let mut np: u_int = (*wp).modeprefix;
    let mut cancel: libc::c_int = 0 as libc::c_int;
    let mut redraw: libc::c_int = 0 as libc::c_int;
    let mut scroll_exit: libc::c_int = 0;
    let mut prefix: libc::c_char = 0;
    if (*args).argc == 0 as libc::c_int {
        return;
    }
    command = *((*args).argv).offset(0 as libc::c_int as isize);
    if !m.is_null() && (*m).valid != 0 {
        window_copy_move_mouse(m);
    }
    if (*args).argc == 1 as libc::c_int {
        if strcmp(command, b"append-selection\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if !s.is_null() {
                window_copy_append_selection(wp, 0 as *const libc::c_char);
            }
            window_copy_clear_selection(wp);
            redraw = 1 as libc::c_int;
        }
        if strcmp(
            command,
            b"append-selection-and-cancel\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if !s.is_null() {
                window_copy_append_selection(wp, 0 as *const libc::c_char);
            }
            window_copy_clear_selection(wp);
            redraw = 1 as libc::c_int;
            cancel = 1 as libc::c_int;
        }
        if strcmp(command, b"back-to-indentation\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_cursor_back_to_indentation(wp);
        }
        if strcmp(command, b"begin-selection\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if !m.is_null() {
                window_copy_start_drag(c, m);
            } else {
                (*sn).sel.lineflag = LINE_SEL_NONE;
                window_copy_start_selection(wp);
                redraw = 1 as libc::c_int;
            }
        }
        if strcmp(command, b"stop-selection\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).cursordrag = CURSORDRAG_NONE;
        }
        if strcmp(command, b"bottom-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).cx = 0 as libc::c_int as u_int;
            (*data).cy = ((*(*sn).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
            window_copy_update_selection(wp, 1 as libc::c_int);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"cancel\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            cancel = 1 as libc::c_int;
        }
        if strcmp(command, b"clear-selection\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_clear_selection(wp);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"copy-end-of-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_start_selection(wp);
            while np > 1 as libc::c_int as u_int {
                window_copy_cursor_down(wp, 0 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
            window_copy_cursor_end_of_line(wp);
            redraw = 1 as libc::c_int;
            if !s.is_null() {
                window_copy_copy_selection(wp, 0 as *const libc::c_char);
                cancel = 1 as libc::c_int;
            }
        }
        if strcmp(command, b"copy-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_cursor_start_of_line(wp);
            window_copy_start_selection(wp);
            while np > 1 as libc::c_int as u_int {
                window_copy_cursor_down(wp, 0 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
            window_copy_cursor_end_of_line(wp);
            redraw = 1 as libc::c_int;
            if !s.is_null() {
                window_copy_copy_selection(wp, 0 as *const libc::c_char);
                cancel = 1 as libc::c_int;
            }
        }
        if strcmp(command, b"copy-selection\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if !s.is_null() {
                window_copy_copy_selection(wp, 0 as *const libc::c_char);
            }
            window_copy_clear_selection(wp);
            redraw = 1 as libc::c_int;
        }
        if strcmp(
            command,
            b"copy-selection-and-cancel\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if !s.is_null() {
                window_copy_copy_selection(wp, 0 as *const libc::c_char);
            }
            window_copy_clear_selection(wp);
            redraw = 1 as libc::c_int;
            cancel = 1 as libc::c_int;
        }
        if strcmp(command, b"cursor-down\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_down(wp, 0 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"cursor-left\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_left(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"cursor-right\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_right(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"cursor-up\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_up(wp, 0 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"end-of-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_cursor_end_of_line(wp);
        }
        if strcmp(command, b"halfpage-down\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                command,
                b"halfpage-down-and-cancel\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            if strcmp(
                command,
                b"halfpage-down-and-cancel\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                scroll_exit = 1 as libc::c_int;
            } else {
                scroll_exit = (*data).scroll_exit;
            }
            while np != 0 as libc::c_int as u_int {
                if window_copy_pagedown(wp, 1 as libc::c_int, scroll_exit) != 0 {
                    cancel = 1 as libc::c_int;
                    break;
                } else {
                    np = np.wrapping_sub(1);
                    np;
                }
            }
        }
        if strcmp(command, b"halfpage-up\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_pageup(wp, 1 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"history-bottom\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).cx = 0 as libc::c_int as u_int;
            (*data).cy = ((*(*sn).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
            (*data).oy = 0 as libc::c_int as u_int;
            window_copy_update_selection(wp, 1 as libc::c_int);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"history-top\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).cx = 0 as libc::c_int as u_int;
            (*data).cy = 0 as libc::c_int as u_int;
            (*data).oy = (*(*(*data).backing).grid).hsize;
            window_copy_update_selection(wp, 1 as libc::c_int);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"jump-again\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            match (*data).jumptype {
                3 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                4 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump_back(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                5 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump_to(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                6 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump_to_back(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                _ => {}
            }
        }
        if strcmp(command, b"jump-reverse\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            match (*data).jumptype {
                3 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump_back(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                4 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                5 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump_to_back(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                6 => {
                    while np != 0 as libc::c_int as u_int {
                        window_copy_cursor_jump_to(wp);
                        np = np.wrapping_sub(1);
                        np;
                    }
                }
                _ => {}
            }
        }
        if strcmp(command, b"middle-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).cx = 0 as libc::c_int as u_int;
            (*data)
                .cy = ((*(*sn).grid).sy).wrapping_sub(1 as libc::c_int as u_int)
                / 2 as libc::c_int as u_int;
            window_copy_update_selection(wp, 1 as libc::c_int);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"next-paragraph\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_next_paragraph(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"next-space\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_next_word(
                    wp,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"next-space-end\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_next_word_end(
                    wp,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"next-word\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ws = options_get_string(
                (*s).options,
                b"word-separators\0" as *const u8 as *const libc::c_char,
            );
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_next_word(wp, ws);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"next-word-end\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ws = options_get_string(
                (*s).options,
                b"word-separators\0" as *const u8 as *const libc::c_char,
            );
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_next_word_end(wp, ws);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"other-end\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if np % 2 as libc::c_int as u_int != 0 as libc::c_int as u_int {
                window_copy_other_end(wp);
            }
        }
        if strcmp(command, b"page-down\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                command,
                b"page-down-and-cancel\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            if strcmp(
                command,
                b"page-down-and-cancel\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                scroll_exit = 1 as libc::c_int;
            } else {
                scroll_exit = (*data).scroll_exit;
            }
            while np != 0 as libc::c_int as u_int {
                if window_copy_pagedown(wp, 0 as libc::c_int, scroll_exit) != 0 {
                    cancel = 1 as libc::c_int;
                    break;
                } else {
                    np = np.wrapping_sub(1);
                    np;
                }
            }
        }
        if strcmp(command, b"page-up\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_pageup(wp, 0 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"previous-paragraph\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_previous_paragraph(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"previous-space\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_previous_word(
                    wp,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"previous-word\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ws = options_get_string(
                (*s).options,
                b"word-separators\0" as *const u8 as *const libc::c_char,
            );
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_previous_word(wp, ws);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"rectangle-toggle\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*sn).sel.lineflag = LINE_SEL_NONE;
            window_copy_rectangle_toggle(wp);
        }
        if strcmp(command, b"scroll-down\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                command,
                b"scroll-down-and-cancel\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            if strcmp(
                command,
                b"scroll-down-and-cancel\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                scroll_exit = 1 as libc::c_int;
            } else {
                scroll_exit = (*data).scroll_exit;
            }
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_down(wp, 1 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
            if scroll_exit != 0 && (*data).oy == 0 as libc::c_int as u_int {
                cancel = 1 as libc::c_int;
            }
        }
        if strcmp(command, b"scroll-up\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_up(wp, 1 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"search-again\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if (*data).searchtype == WINDOW_COPY_SEARCHUP as libc::c_int {
                while np != 0 as libc::c_int as u_int {
                    window_copy_search_up(wp);
                    np = np.wrapping_sub(1);
                    np;
                }
            } else if (*data).searchtype == WINDOW_COPY_SEARCHDOWN as libc::c_int {
                while np != 0 as libc::c_int as u_int {
                    window_copy_search_down(wp);
                    np = np.wrapping_sub(1);
                    np;
                }
            }
        }
        if strcmp(command, b"search-reverse\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if (*data).searchtype == WINDOW_COPY_SEARCHUP as libc::c_int {
                while np != 0 as libc::c_int as u_int {
                    window_copy_search_down(wp);
                    np = np.wrapping_sub(1);
                    np;
                }
            } else if (*data).searchtype == WINDOW_COPY_SEARCHDOWN as libc::c_int {
                while np != 0 as libc::c_int as u_int {
                    window_copy_search_up(wp);
                    np = np.wrapping_sub(1);
                    np;
                }
            }
        }
        if strcmp(command, b"select-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*sn).sel.lineflag = LINE_SEL_LEFT_RIGHT;
            (*data).rectflag = 0 as libc::c_int;
            window_copy_cursor_start_of_line(wp);
            window_copy_start_selection(wp);
            while np > 1 as libc::c_int as u_int {
                window_copy_cursor_down(wp, 0 as libc::c_int);
                np = np.wrapping_sub(1);
                np;
            }
            window_copy_cursor_end_of_line(wp);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"select-word\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*sn).sel.lineflag = LINE_SEL_LEFT_RIGHT;
            (*data).rectflag = 0 as libc::c_int;
            ws = options_get_string(
                (*s).options,
                b"word-separators\0" as *const u8 as *const libc::c_char,
            );
            window_copy_cursor_previous_word(wp, ws);
            window_copy_start_selection(wp);
            window_copy_cursor_next_word_end(wp, ws);
            redraw = 1 as libc::c_int;
        }
        if strcmp(command, b"start-of-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_cursor_start_of_line(wp);
        }
        if strcmp(command, b"top-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).cx = 0 as libc::c_int as u_int;
            (*data).cy = 0 as libc::c_int as u_int;
            window_copy_update_selection(wp, 1 as libc::c_int);
            redraw = 1 as libc::c_int;
        }
    } else if (*args).argc == 2 as libc::c_int
        && **((*args).argv).offset(1 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
    {
        argument = *((*args).argv).offset(1 as libc::c_int as isize);
        if strcmp(command, b"copy-pipe\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if !s.is_null() {
                window_copy_copy_pipe(wp, s, 0 as *const libc::c_char, argument);
            }
        }
        if strcmp(command, b"copy-pipe-and-cancel\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if !s.is_null() {
                window_copy_copy_pipe(wp, s, 0 as *const libc::c_char, argument);
                cancel = 1 as libc::c_int;
            }
        }
        if strcmp(command, b"goto-line\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            window_copy_goto_line(wp, argument);
        }
        if strcmp(command, b"jump-backward\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).jumptype = WINDOW_COPY_JUMPBACKWARD as libc::c_int;
            (*data).jumpchar = *argument;
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_jump_back(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"jump-forward\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).jumptype = WINDOW_COPY_JUMPFORWARD as libc::c_int;
            (*data).jumpchar = *argument;
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_jump(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"jump-to-backward\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).jumptype = WINDOW_COPY_JUMPTOBACKWARD as libc::c_int;
            (*data).jumpchar = *argument;
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_jump_to_back(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"jump-to-forward\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).jumptype = WINDOW_COPY_JUMPTOFORWARD as libc::c_int;
            (*data).jumpchar = *argument;
            while np != 0 as libc::c_int as u_int {
                window_copy_cursor_jump_to(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"search-backward\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
            free((*data).searchstr as *mut libc::c_void);
            (*data).searchstr = xstrdup(argument);
            while np != 0 as libc::c_int as u_int {
                window_copy_search_up(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(command, b"search-forward\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
            free((*data).searchstr as *mut libc::c_void);
            (*data).searchstr = xstrdup(argument);
            while np != 0 as libc::c_int as u_int {
                window_copy_search_down(wp);
                np = np.wrapping_sub(1);
                np;
            }
        }
        if strcmp(
            command,
            b"search-backward-incremental\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh0 = argument;
            argument = argument.offset(1);
            prefix = *fresh0;
            if (*data).searchx == -(1 as libc::c_int)
                || (*data).searchy == -(1 as libc::c_int)
            {
                (*data).searchx = (*data).cx as libc::c_int;
                (*data).searchy = (*data).cy as libc::c_int;
                (*data).searcho = (*data).oy as libc::c_int;
            } else if !((*data).searchstr).is_null()
                && strcmp(argument, (*data).searchstr) != 0 as libc::c_int
            {
                (*data).cx = (*data).searchx as u_int;
                (*data).cy = (*data).searchy as u_int;
                (*data).oy = (*data).searcho as u_int;
                redraw = 1 as libc::c_int;
            }
            if *argument as libc::c_int == '\0' as i32 {
                window_copy_clear_marks(wp);
                redraw = 1 as libc::c_int;
            } else if prefix as libc::c_int == '=' as i32
                || prefix as libc::c_int == '-' as i32
            {
                (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument);
                if window_copy_search_up(wp) == 0 {
                    window_copy_clear_marks(wp);
                    redraw = 1 as libc::c_int;
                }
            } else if prefix as libc::c_int == '+' as i32 {
                (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument);
                if window_copy_search_down(wp) == 0 {
                    window_copy_clear_marks(wp);
                    redraw = 1 as libc::c_int;
                }
            }
        }
        if strcmp(
            command,
            b"search-forward-incremental\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let fresh1 = argument;
            argument = argument.offset(1);
            prefix = *fresh1;
            if (*data).searchx == -(1 as libc::c_int)
                || (*data).searchy == -(1 as libc::c_int)
            {
                (*data).searchx = (*data).cx as libc::c_int;
                (*data).searchy = (*data).cy as libc::c_int;
                (*data).searcho = (*data).oy as libc::c_int;
            } else if !((*data).searchstr).is_null()
                && strcmp(argument, (*data).searchstr) != 0 as libc::c_int
            {
                (*data).cx = (*data).searchx as u_int;
                (*data).cy = (*data).searchy as u_int;
                (*data).oy = (*data).searcho as u_int;
                redraw = 1 as libc::c_int;
            }
            if *argument as libc::c_int == '\0' as i32 {
                window_copy_clear_marks(wp);
                redraw = 1 as libc::c_int;
            } else if prefix as libc::c_int == '=' as i32
                || prefix as libc::c_int == '+' as i32
            {
                (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument);
                if window_copy_search_down(wp) == 0 {
                    window_copy_clear_marks(wp);
                    redraw = 1 as libc::c_int;
                }
            } else if prefix as libc::c_int == '-' as i32 {
                (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument);
                if window_copy_search_up(wp) == 0 {
                    window_copy_clear_marks(wp);
                    redraw = 1 as libc::c_int;
                }
            }
        }
    }
    if strncmp(
        command,
        b"search-\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int && !((*data).searchmark).is_null()
    {
        window_copy_clear_marks(wp);
        redraw = 1 as libc::c_int;
        (*data).searchy = -(1 as libc::c_int);
        (*data).searchx = (*data).searchy;
    }
    if cancel != 0 {
        window_pane_reset_mode(wp);
    } else if redraw != 0 {
        window_copy_redraw_screen(wp);
    }
    (*wp).modeprefix = 1 as libc::c_int as u_int;
}
unsafe extern "C" fn window_copy_scroll_to(
    mut wp: *mut window_pane,
    mut px: u_int,
    mut py: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut offset: u_int = 0;
    let mut gap: u_int = 0;
    (*data).cx = px;
    if py >= ((*gd).hsize).wrapping_sub((*data).oy)
        && py < ((*gd).hsize).wrapping_sub((*data).oy).wrapping_add((*gd).sy)
    {
        (*data).cy = py.wrapping_sub(((*gd).hsize).wrapping_sub((*data).oy));
    } else {
        gap = (*gd).sy / 4 as libc::c_int as u_int;
        if py < (*gd).sy {
            offset = 0 as libc::c_int as u_int;
            (*data).cy = py;
        } else if py > ((*gd).hsize).wrapping_add((*gd).sy).wrapping_sub(gap) {
            offset = (*gd).hsize;
            (*data).cy = py.wrapping_sub((*gd).hsize);
        } else {
            offset = py.wrapping_add(gap).wrapping_sub((*gd).sy);
            (*data).cy = py.wrapping_sub(offset);
        }
        (*data).oy = ((*gd).hsize).wrapping_sub(offset);
    }
    window_copy_update_selection(wp, 1 as libc::c_int);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_search_compare(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut sgd: *mut grid,
    mut spx: u_int,
    mut cis: libc::c_int,
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
    let mut sgc: grid_cell = grid_cell {
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
    let mut ud: *const utf8_data = 0 as *const utf8_data;
    let mut sud: *const utf8_data = 0 as *const utf8_data;
    grid_get_cell(gd, px, py, &mut gc);
    ud = &mut gc.data;
    grid_get_cell(sgd, spx, 0 as libc::c_int as u_int, &mut sgc);
    sud = &mut sgc.data;
    if (*ud).size as libc::c_int != (*sud).size as libc::c_int
        || (*ud).width as libc::c_int != (*sud).width as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if cis != 0 && (*ud).size as libc::c_int == 1 as libc::c_int {
        return (tolower((*ud).data[0 as libc::c_int as usize] as libc::c_int)
            == (*sud).data[0 as libc::c_int as usize] as libc::c_int) as libc::c_int;
    }
    return (memcmp(
        ((*ud).data).as_ptr() as *const libc::c_void,
        ((*sud).data).as_ptr() as *const libc::c_void,
        (*ud).size as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn window_copy_search_lr(
    mut gd: *mut grid,
    mut sgd: *mut grid,
    mut ppx: *mut u_int,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut cis: libc::c_int,
) -> libc::c_int {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut px: u_int = 0;
    let mut matched: libc::c_int = 0;
    ax = first;
    while ax < last {
        if ax.wrapping_add((*sgd).sx) > (*gd).sx {
            break;
        }
        bx = 0 as libc::c_int as u_int;
        while bx < (*sgd).sx {
            px = ax.wrapping_add(bx);
            matched = window_copy_search_compare(gd, px, py, sgd, bx, cis);
            if matched == 0 {
                break;
            }
            bx = bx.wrapping_add(1);
            bx;
        }
        if bx == (*sgd).sx {
            *ppx = ax;
            return 1 as libc::c_int;
        }
        ax = ax.wrapping_add(1);
        ax;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_search_rl(
    mut gd: *mut grid,
    mut sgd: *mut grid,
    mut ppx: *mut u_int,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut cis: libc::c_int,
) -> libc::c_int {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut px: u_int = 0;
    let mut matched: libc::c_int = 0;
    ax = last.wrapping_add(1 as libc::c_int as u_int);
    while ax > first {
        if !(((*gd).sx).wrapping_sub(ax.wrapping_sub(1 as libc::c_int as u_int))
            < (*sgd).sx)
        {
            bx = 0 as libc::c_int as u_int;
            while bx < (*sgd).sx {
                px = ax.wrapping_sub(1 as libc::c_int as u_int).wrapping_add(bx);
                matched = window_copy_search_compare(gd, px, py, sgd, bx, cis);
                if matched == 0 {
                    break;
                }
                bx = bx.wrapping_add(1);
                bx;
            }
            if bx == (*sgd).sx {
                *ppx = ax.wrapping_sub(1 as libc::c_int as u_int);
                return 1 as libc::c_int;
            }
        }
        ax = ax.wrapping_sub(1);
        ax;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_move_left(
    mut s: *mut screen,
    mut fx: *mut u_int,
    mut fy: *mut u_int,
) {
    if *fx == 0 as libc::c_int as u_int {
        if *fy == 0 as libc::c_int as u_int {
            return;
        }
        *fx = ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int);
        *fy = (*fy).wrapping_sub(1 as libc::c_int as u_int);
    } else {
        *fx = (*fx).wrapping_sub(1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_move_right(
    mut s: *mut screen,
    mut fx: *mut u_int,
    mut fy: *mut u_int,
) {
    if *fx == ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
        if *fy == ((*(*s).grid).hsize).wrapping_add((*(*s).grid).sy) {
            return;
        }
        *fx = 0 as libc::c_int as u_int;
        *fy = (*fy).wrapping_add(1 as libc::c_int as u_int);
    } else {
        *fx = (*fx).wrapping_add(1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_is_lowercase(
    mut ptr: *const libc::c_char,
) -> libc::c_int {
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr as libc::c_int != tolower(*ptr as u_char as libc::c_int) {
            return 0 as libc::c_int;
        }
        ptr = ptr.offset(1);
        ptr;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn window_copy_search_jump(
    mut wp: *mut window_pane,
    mut gd: *mut grid,
    mut sgd: *mut grid,
    mut fx: u_int,
    mut fy: u_int,
    mut endline: u_int,
    mut cis: libc::c_int,
    mut wrap: libc::c_int,
    mut direction: libc::c_int,
) -> libc::c_int {
    let mut i: u_int = 0;
    let mut px: u_int = 0;
    let mut found: libc::c_int = 0;
    found = 0 as libc::c_int;
    if direction != 0 {
        i = fy;
        while i <= endline {
            found = window_copy_search_lr(gd, sgd, &mut px, i, fx, (*gd).sx, cis);
            if found != 0 {
                break;
            }
            fx = 0 as libc::c_int as u_int;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = fy.wrapping_add(1 as libc::c_int as u_int);
        while endline < i {
            found = window_copy_search_rl(
                gd,
                sgd,
                &mut px,
                i.wrapping_sub(1 as libc::c_int as u_int),
                0 as libc::c_int as u_int,
                fx,
                cis,
            );
            if found != 0 {
                i = i.wrapping_sub(1);
                i;
                break;
            } else {
                fx = (*gd).sx;
                i = i.wrapping_sub(1);
                i;
            }
        }
    }
    if found != 0 {
        window_copy_scroll_to(wp, px, i);
        return 1 as libc::c_int;
    }
    if wrap != 0 {
        return window_copy_search_jump(
            wp,
            gd,
            sgd,
            if direction != 0 {
                0 as libc::c_int as u_int
            } else {
                ((*gd).sx).wrapping_sub(1 as libc::c_int as u_int)
            },
            if direction != 0 {
                0 as libc::c_int as u_int
            } else {
                ((*gd).hsize)
                    .wrapping_add((*gd).sy)
                    .wrapping_sub(1 as libc::c_int as u_int)
            },
            fy,
            cis,
            0 as libc::c_int,
            direction,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_search(
    mut wp: *mut window_pane,
    mut direction: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut ss: screen = screen {
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
    let mut gd: *mut grid = (*s).grid;
    let mut fx: u_int = 0;
    let mut fy: u_int = 0;
    let mut endline: u_int = 0;
    let mut wrapflag: libc::c_int = 0;
    let mut cis: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    free((*wp).searchstr as *mut libc::c_void);
    (*wp).searchstr = xstrdup((*data).searchstr);
    fx = (*data).cx;
    fy = ((*(*(*data).backing).grid).hsize)
        .wrapping_sub((*data).oy)
        .wrapping_add((*data).cy);
    screen_init(
        &mut ss,
        screen_write_strlen(
            b"%s\0" as *const u8 as *const libc::c_char,
            (*data).searchstr,
        ) as u_int,
        1 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    screen_write_start(&mut ctx, 0 as *mut window_pane, &mut ss);
    screen_write_nputs(
        &mut ctx as *mut screen_write_ctx,
        -(1 as libc::c_int) as ssize_t,
        &grid_default_cell as *const grid_cell,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*data).searchstr,
    );
    screen_write_stop(&mut ctx);
    if direction != 0 {
        window_copy_move_right(s, &mut fx, &mut fy);
    } else {
        window_copy_move_left(s, &mut fx, &mut fy);
    }
    window_copy_clear_selection(wp);
    wrapflag = options_get_number(
        (*(*wp).window).options,
        b"wrap-search\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    cis = window_copy_is_lowercase((*data).searchstr);
    if direction != 0 {
        endline = ((*gd).hsize)
            .wrapping_add((*gd).sy)
            .wrapping_sub(1 as libc::c_int as u_int);
    } else {
        endline = 0 as libc::c_int as u_int;
    }
    found = window_copy_search_jump(
        wp,
        gd,
        ss.grid,
        fx,
        fy,
        endline,
        cis,
        wrapflag,
        direction,
    );
    if window_copy_search_marks(wp, &mut ss) != 0 {
        window_copy_redraw_screen(wp);
    }
    screen_free(&mut ss);
    return found;
}
unsafe extern "C" fn window_copy_search_marks(
    mut wp: *mut window_pane,
    mut ssp: *mut screen,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut ss: screen = screen {
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
    let mut gd: *mut grid = (*s).grid;
    let mut found: libc::c_int = 0;
    let mut cis: libc::c_int = 0;
    let mut which: libc::c_int = -(1 as libc::c_int);
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut b: u_int = 0;
    let mut nfound: u_int = 0 as libc::c_int as u_int;
    let mut width: u_int = 0;
    if ssp.is_null() {
        width = screen_write_strlen(
            b"%s\0" as *const u8 as *const libc::c_char,
            (*data).searchstr,
        ) as u_int;
        screen_init(
            &mut ss,
            width,
            1 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
        screen_write_start(&mut ctx, 0 as *mut window_pane, &mut ss);
        screen_write_nputs(
            &mut ctx as *mut screen_write_ctx,
            -(1 as libc::c_int) as ssize_t,
            &grid_default_cell as *const grid_cell,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*data).searchstr,
        );
        screen_write_stop(&mut ctx);
        ssp = &mut ss;
    } else {
        width = (*(*ssp).grid).sx;
    }
    cis = window_copy_is_lowercase((*data).searchstr);
    free((*data).searchmark as *mut libc::c_void);
    (*data)
        .searchmark = calloc(
        ((((*gd).hsize).wrapping_add((*gd).sy) * (*gd).sx)
            .wrapping_add(7 as libc::c_int as u_int) >> 3 as libc::c_int) as size_t,
        ::core::mem::size_of::<bitstr_t>() as libc::c_ulong,
    ) as *mut bitstr_t;
    py = 0 as libc::c_int as u_int;
    while py < ((*gd).hsize).wrapping_add((*gd).sy) {
        px = 0 as libc::c_int as u_int;
        loop {
            found = window_copy_search_lr(
                gd,
                (*ssp).grid,
                &mut px,
                py,
                px,
                (*gd).sx,
                cis,
            );
            if found == 0 {
                break;
            }
            nfound = nfound.wrapping_add(1);
            nfound;
            if px == (*data).cx
                && py == ((*gd).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy)
            {
                which = nfound as libc::c_int;
            }
            b = (py * (*gd).sx).wrapping_add(px);
            let mut _name: *mut bitstr_t = (*data).searchmark;
            let mut _start: libc::c_int = b as libc::c_int;
            let mut _stop: libc::c_int = b
                .wrapping_add(width)
                .wrapping_sub(1 as libc::c_int as u_int) as libc::c_int;
            while _start <= _stop {
                let ref mut fresh2 = *_name
                    .offset((_start >> 3 as libc::c_int) as isize);
                *fresh2 = (*fresh2 as libc::c_int
                    | (1 as libc::c_int) << (_start & 0x7 as libc::c_int)) as bitstr_t;
                _start += 1;
                _start;
            }
            px = px.wrapping_add(1);
            px;
        }
        py = py.wrapping_add(1);
        py;
    }
    if which != -(1 as libc::c_int) {
        (*data)
            .searchthis = (1 as libc::c_int as u_int)
            .wrapping_add(nfound)
            .wrapping_sub(which as u_int) as libc::c_int;
    } else {
        (*data).searchthis = -(1 as libc::c_int);
    }
    (*data).searchcount = nfound;
    if ssp == &mut ss as *mut screen {
        screen_free(&mut ss);
    }
    return nfound as libc::c_int;
}
unsafe extern "C" fn window_copy_clear_marks(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    free((*data).searchmark as *mut libc::c_void);
    (*data).searchmark = 0 as *mut bitstr_t;
}
unsafe extern "C" fn window_copy_search_up(mut wp: *mut window_pane) -> libc::c_int {
    return window_copy_search(wp, 0 as libc::c_int);
}
unsafe extern "C" fn window_copy_search_down(mut wp: *mut window_pane) -> libc::c_int {
    return window_copy_search(wp, 1 as libc::c_int);
}
unsafe extern "C" fn window_copy_goto_line(
    mut wp: *mut window_pane,
    mut linestr: *const libc::c_char,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut lineno: u_int = 0;
    lineno = strtonum(
        linestr,
        0 as libc::c_int as libc::c_longlong,
        (*(*(*data).backing).grid).hsize as libc::c_longlong,
        &mut errstr,
    ) as u_int;
    if !errstr.is_null() {
        return;
    }
    (*data).oy = lineno;
    window_copy_update_selection(wp, 1 as libc::c_int);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_write_line(
    mut wp: *mut window_pane,
    mut ctx: *mut screen_write_ctx,
    mut py: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut oo: *mut options = (*(*wp).window).options;
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
    let mut hdr: [libc::c_char; 512] = [0; 512];
    let mut size: size_t = 0 as libc::c_int as size_t;
    style_apply(&mut gc, oo, b"mode-style\0" as *const u8 as *const libc::c_char);
    gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    if py == 0 as libc::c_int as u_int {
        if ((*data).searchmark).is_null() {
            size = xsnprintf(
                hdr.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"[%u/%u]\0" as *const u8 as *const libc::c_char,
                (*data).oy,
                (*(*(*data).backing).grid).hsize,
            ) as size_t;
        } else if (*data).searchthis == -(1 as libc::c_int) {
            size = xsnprintf(
                hdr.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"(%u results) [%d/%u]\0" as *const u8 as *const libc::c_char,
                (*data).searchcount,
                (*data).oy,
                (*(*(*data).backing).grid).hsize,
            ) as size_t;
        } else {
            size = xsnprintf(
                hdr.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"(%u/%u results) [%d/%u]\0" as *const u8 as *const libc::c_char,
                (*data).searchthis,
                (*data).searchcount,
                (*data).oy,
                (*(*(*data).backing).grid).hsize,
            ) as size_t;
        }
        if size > (*(*s).grid).sx as size_t {
            size = (*(*s).grid).sx as size_t;
        }
        screen_write_cursormove(
            ctx,
            ((*(*s).grid).sx as size_t).wrapping_sub(size) as u_int,
            0 as libc::c_int as u_int,
        );
        screen_write_puts(
            ctx,
            &mut gc as *mut grid_cell,
            b"%s\0" as *const u8 as *const libc::c_char,
            hdr.as_mut_ptr(),
        );
    } else {
        size = 0 as libc::c_int as size_t;
    }
    if size < (*(*s).grid).sx as size_t {
        screen_write_cursormove(ctx, 0 as libc::c_int as u_int, py);
        screen_write_copy(
            ctx,
            (*data).backing,
            0 as libc::c_int as u_int,
            ((*(*(*data).backing).grid).hsize).wrapping_sub((*data).oy).wrapping_add(py),
            ((*(*s).grid).sx as size_t).wrapping_sub(size) as u_int,
            1 as libc::c_int as u_int,
            (*data).searchmark,
            &mut gc,
        );
    }
    if py == (*data).cy && (*data).cx == (*(*s).grid).sx {
        memcpy(
            &mut gc as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        screen_write_cursormove(
            ctx,
            ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int),
            py,
        );
        screen_write_putc(ctx, &mut gc, '$' as i32 as u_char);
    }
}
unsafe extern "C" fn window_copy_write_lines(
    mut wp: *mut window_pane,
    mut ctx: *mut screen_write_ctx,
    mut py: u_int,
    mut ny: u_int,
) {
    let mut yy: u_int = 0;
    yy = py;
    while yy < py.wrapping_add(ny) {
        window_copy_write_line(wp, ctx, py);
        yy = yy.wrapping_add(1);
        yy;
    }
}
unsafe extern "C" fn window_copy_redraw_selection(
    mut wp: *mut window_pane,
    mut old_y: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut new_y: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    new_y = (*data).cy;
    if old_y <= new_y {
        start = old_y;
        end = new_y;
    } else {
        start = new_y;
        end = old_y;
    }
    window_copy_redraw_lines(
        wp,
        start,
        end.wrapping_sub(start).wrapping_add(1 as libc::c_int as u_int),
    );
}
unsafe extern "C" fn window_copy_redraw_lines(
    mut wp: *mut window_pane,
    mut py: u_int,
    mut ny: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
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
    let mut i: u_int = 0;
    screen_write_start(&mut ctx, wp, 0 as *mut screen);
    i = py;
    while i < py.wrapping_add(ny) {
        window_copy_write_line(wp, &mut ctx, i);
        i = i.wrapping_add(1);
        i;
    }
    screen_write_cursormove(&mut ctx, (*data).cx, (*data).cy);
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn window_copy_redraw_screen(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    window_copy_redraw_lines(wp, 0 as libc::c_int as u_int, (*(*data).screen.grid).sy);
}
unsafe extern "C" fn window_copy_synchronize_cursor(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    xx = (*data).cx;
    yy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    match (*data).cursordrag as libc::c_uint {
        1 => {
            (*data).endselx = xx;
            (*data).endsely = yy;
        }
        2 => {
            (*data).selx = xx;
            (*data).sely = yy;
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_copy_update_cursor(
    mut wp: *mut window_pane,
    mut cx: u_int,
    mut cy: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
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
    let mut old_cx: u_int = 0;
    let mut old_cy: u_int = 0;
    old_cx = (*data).cx;
    old_cy = (*data).cy;
    (*data).cx = cx;
    (*data).cy = cy;
    if old_cx == (*(*s).grid).sx {
        window_copy_redraw_lines(wp, old_cy, 1 as libc::c_int as u_int);
    }
    if (*data).cx == (*(*s).grid).sx {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    } else {
        screen_write_start(&mut ctx, wp, 0 as *mut screen);
        screen_write_cursormove(&mut ctx, (*data).cx, (*data).cy);
        screen_write_stop(&mut ctx);
    };
}
unsafe extern "C" fn window_copy_start_selection(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    (*data).selx = (*data).cx;
    (*data)
        .sely = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).endselx = (*data).selx;
    (*data).endsely = (*data).sely;
    (*data).cursordrag = CURSORDRAG_ENDSEL;
    (*s).sel.flag = 1 as libc::c_int;
    window_copy_update_selection(wp, 1 as libc::c_int);
}
unsafe extern "C" fn window_copy_adjust_selection(
    mut wp: *mut window_pane,
    mut selx: *mut u_int,
    mut sely: *mut u_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut ty: u_int = 0;
    let mut relpos: libc::c_int = 0;
    sx = *selx;
    sy = *sely;
    ty = ((*(*(*data).backing).grid).hsize).wrapping_sub((*data).oy);
    if sy < ty {
        relpos = WINDOW_COPY_REL_POS_ABOVE as libc::c_int;
        if (*data).rectflag == 0 {
            sx = 0 as libc::c_int as u_int;
        }
        sy = 0 as libc::c_int as u_int;
    } else if sy
        > ty.wrapping_add((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int)
    {
        relpos = WINDOW_COPY_REL_POS_BELOW as libc::c_int;
        if (*data).rectflag == 0 {
            sx = ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int);
        }
        sy = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    } else {
        relpos = WINDOW_COPY_REL_POS_ON_SCREEN as libc::c_int;
        sy = sy.wrapping_sub(ty);
    }
    *selx = sx;
    *sely = sy;
    return relpos;
}
unsafe extern "C" fn window_copy_update_selection(
    mut wp: *mut window_pane,
    mut may_redraw: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut oo: *mut options = (*(*wp).window).options;
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
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut cy: u_int = 0;
    let mut endsx: u_int = 0;
    let mut endsy: u_int = 0;
    let mut startrelpos: libc::c_int = 0;
    let mut endrelpos: libc::c_int = 0;
    if (*s).sel.flag == 0
        && (*s).sel.lineflag as libc::c_uint
            == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    window_copy_synchronize_cursor(wp);
    sx = (*data).selx;
    sy = (*data).sely;
    startrelpos = window_copy_adjust_selection(wp, &mut sx, &mut sy);
    endsx = (*data).endselx;
    endsy = (*data).endsely;
    endrelpos = window_copy_adjust_selection(wp, &mut endsx, &mut endsy);
    if startrelpos == endrelpos
        && startrelpos != WINDOW_COPY_REL_POS_ON_SCREEN as libc::c_int
    {
        screen_hide_selection(s);
        return 0 as libc::c_int;
    }
    style_apply(&mut gc, oo, b"mode-style\0" as *const u8 as *const libc::c_char);
    gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    screen_set_selection(s, sx, sy, endsx, endsy, (*data).rectflag as u_int, &mut gc);
    if (*data).rectflag != 0 && may_redraw != 0 {
        cy = (*data).cy;
        if (*data).cursordrag as libc::c_uint
            == CURSORDRAG_ENDSEL as libc::c_int as libc::c_uint
        {
            if sy < cy {
                window_copy_redraw_lines(
                    wp,
                    sy,
                    cy.wrapping_sub(sy).wrapping_add(1 as libc::c_int as u_int),
                );
            } else {
                window_copy_redraw_lines(
                    wp,
                    cy,
                    sy.wrapping_sub(cy).wrapping_add(1 as libc::c_int as u_int),
                );
            }
        } else if endsy < cy {
            window_copy_redraw_lines(
                wp,
                endsy,
                cy.wrapping_sub(endsy).wrapping_add(1 as libc::c_int as u_int),
            );
        } else {
            window_copy_redraw_lines(
                wp,
                cy,
                endsy.wrapping_sub(cy).wrapping_add(1 as libc::c_int as u_int),
            );
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn window_copy_get_selection(
    mut wp: *mut window_pane,
    mut len: *mut size_t,
) -> *mut libc::c_void {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: size_t = 0;
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut ex: u_int = 0;
    let mut ey: u_int = 0;
    let mut ey_last: u_int = 0;
    let mut firstsx: u_int = 0;
    let mut lastex: u_int = 0;
    let mut restex: u_int = 0;
    let mut restsx: u_int = 0;
    let mut selx: u_int = 0;
    let mut keys: libc::c_int = 0;
    if (*s).sel.flag == 0
        && (*s).sel.lineflag as libc::c_uint
            == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        return 0 as *mut libc::c_void;
    }
    buf = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
    off = 0 as libc::c_int as size_t;
    *buf = '\0' as i32 as libc::c_char;
    xx = (*data).endselx;
    yy = (*data).endsely;
    if yy < (*data).sely || yy == (*data).sely && xx < (*data).selx {
        sx = xx;
        sy = yy;
        ex = (*data).selx;
        ey = (*data).sely;
    } else {
        sx = (*data).selx;
        sy = (*data).sely;
        ex = xx;
        ey = yy;
    }
    ey_last = window_copy_find_length(wp, ey);
    if ex > ey_last {
        ex = ey_last;
    }
    xx = (*(*s).grid).sx;
    keys = options_get_number(
        (*(*wp).window).options,
        b"mode-keys\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if (*data).rectflag != 0 {
        if (*data).cursordrag as libc::c_uint
            == CURSORDRAG_ENDSEL as libc::c_int as libc::c_uint
        {
            selx = (*data).selx;
        } else {
            selx = (*data).endselx;
        }
        if selx < (*data).cx {
            if keys == 0 as libc::c_int {
                lastex = (*data).cx;
                restex = (*data).cx;
            } else {
                lastex = ((*data).cx).wrapping_add(1 as libc::c_int as u_int);
                restex = ((*data).cx).wrapping_add(1 as libc::c_int as u_int);
            }
            firstsx = selx;
            restsx = selx;
        } else {
            lastex = selx.wrapping_add(1 as libc::c_int as u_int);
            restex = selx.wrapping_add(1 as libc::c_int as u_int);
            firstsx = (*data).cx;
            restsx = (*data).cx;
        }
    } else {
        if keys == 0 as libc::c_int {
            lastex = ex;
        } else {
            lastex = ex.wrapping_add(1 as libc::c_int as u_int);
        }
        restex = xx;
        firstsx = sx;
        restsx = 0 as libc::c_int as u_int;
    }
    i = sy;
    while i <= ey {
        window_copy_copy_line(
            wp,
            &mut buf,
            &mut off,
            i,
            if i == sy { firstsx } else { restsx },
            if i == ey { lastex } else { restex },
        );
        i = i.wrapping_add(1);
        i;
    }
    if off == 0 as libc::c_int as size_t {
        free(buf as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    if keys == 0 as libc::c_int || lastex <= ey_last {
        off = off.wrapping_sub(1 as libc::c_int as size_t);
    }
    *len = off;
    return buf as *mut libc::c_void;
}
unsafe extern "C" fn window_copy_copy_buffer(
    mut wp: *mut window_pane,
    mut bufname: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) {
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
    if options_get_number(
        global_options,
        b"set-clipboard\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int as libc::c_longlong
    {
        screen_write_start(&mut ctx, wp, 0 as *mut screen);
        screen_write_setselection(&mut ctx, buf as *mut u_char, len as u_int);
        screen_write_stop(&mut ctx);
        notify_pane(b"pane-set-clipboard\0" as *const u8 as *const libc::c_char, wp);
    }
    if paste_set(buf as *mut libc::c_char, len, bufname, 0 as *mut *mut libc::c_char)
        != 0 as libc::c_int
    {
        free(buf);
    }
}
unsafe extern "C" fn window_copy_copy_pipe(
    mut wp: *mut window_pane,
    mut s: *mut session,
    mut bufname: *const libc::c_char,
    mut arg: *const libc::c_char,
) {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut job: *mut job = 0 as *mut job;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = window_copy_get_selection(wp, &mut len);
    if buf.is_null() {
        return;
    }
    expanded = format_single(
        0 as *mut cmdq_item,
        arg,
        0 as *mut client,
        s,
        0 as *mut winlink,
        wp,
    );
    job = job_run(
        expanded,
        s,
        0 as *const libc::c_char,
        None,
        None,
        None,
        0 as *mut libc::c_void,
        0x1 as libc::c_int,
    );
    bufferevent_write((*job).event, buf, len);
    free(expanded as *mut libc::c_void);
    window_copy_copy_buffer(wp, bufname, buf, len);
}
unsafe extern "C" fn window_copy_copy_selection(
    mut wp: *mut window_pane,
    mut bufname: *const libc::c_char,
) {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    buf = window_copy_get_selection(wp, &mut len);
    if buf.is_null() {
        return;
    }
    window_copy_copy_buffer(wp, bufname, buf, len);
}
unsafe extern "C" fn window_copy_append_selection(
    mut wp: *mut window_pane,
    mut bufname: *const libc::c_char,
) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut bufdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut bufsize: size_t = 0;
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
    buf = window_copy_get_selection(wp, &mut len) as *mut libc::c_char;
    if buf.is_null() {
        return;
    }
    if options_get_number(
        global_options,
        b"set-clipboard\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int as libc::c_longlong
    {
        screen_write_start(&mut ctx, wp, 0 as *mut screen);
        screen_write_setselection(&mut ctx, buf as *mut u_char, len as u_int);
        screen_write_stop(&mut ctx);
        notify_pane(b"pane-set-clipboard\0" as *const u8 as *const libc::c_char, wp);
    }
    if bufname.is_null() || *bufname as libc::c_int == '\0' as i32 {
        pb = paste_get_top(&mut bufname);
    } else {
        pb = paste_get_name(bufname);
    }
    if !pb.is_null() {
        bufdata = paste_buffer_data(pb, &mut bufsize);
        buf = xrealloc(buf as *mut libc::c_void, len.wrapping_add(bufsize))
            as *mut libc::c_char;
        memmove(
            buf.offset(bufsize as isize) as *mut libc::c_void,
            buf as *const libc::c_void,
            len,
        );
        memcpy(buf as *mut libc::c_void, bufdata as *const libc::c_void, bufsize);
        len = len.wrapping_add(bufsize);
    }
    if paste_set(buf, len, bufname, 0 as *mut *mut libc::c_char) != 0 as libc::c_int {
        free(buf as *mut libc::c_void);
    }
}
unsafe extern "C" fn window_copy_copy_line(
    mut wp: *mut window_pane,
    mut buf: *mut *mut libc::c_char,
    mut off: *mut size_t,
    mut sy: u_int,
    mut sx: u_int,
    mut ex: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
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
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut wrapped: u_int = 0 as libc::c_int as u_int;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if sx > ex {
        return;
    }
    gl = &mut *((*gd).linedata).offset(sy as isize) as *mut grid_line;
    if (*gl).flags & 0x1 as libc::c_int != 0 && (*gl).cellsize <= (*gd).sx {
        wrapped = 1 as libc::c_int as u_int;
    }
    if wrapped != 0 {
        xx = (*gl).cellsize;
    } else {
        xx = window_copy_find_length(wp, sy);
    }
    if ex > xx {
        ex = xx;
    }
    if sx > xx {
        sx = xx;
    }
    if sx < ex {
        i = sx;
        while i < ex {
            grid_get_cell(gd, i, sy, &mut gc);
            if !(gc.flags as libc::c_int & 0x4 as libc::c_int != 0) {
                utf8_copy(&mut ud, &mut gc.data);
                if ud.size as libc::c_int == 1 as libc::c_int
                    && gc.attr as libc::c_int & 0x80 as libc::c_int != 0
                {
                    s = tty_acs_get(0 as *mut tty, ud.data[0 as libc::c_int as usize]);
                    if !s.is_null()
                        && strlen(s)
                            <= ::core::mem::size_of::<[u_char; 9]>() as libc::c_ulong
                    {
                        ud.size = strlen(s) as u_char;
                        memcpy(
                            (ud.data).as_mut_ptr() as *mut libc::c_void,
                            s as *const libc::c_void,
                            ud.size as libc::c_ulong,
                        );
                    }
                }
                *buf = xrealloc(
                    *buf as *mut libc::c_void,
                    (*off).wrapping_add(ud.size as size_t),
                ) as *mut libc::c_char;
                memcpy(
                    (*buf).offset(*off as isize) as *mut libc::c_void,
                    (ud.data).as_mut_ptr() as *const libc::c_void,
                    ud.size as libc::c_ulong,
                );
                *off = (*off).wrapping_add(ud.size as size_t);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if wrapped == 0 || ex != xx {
        *buf = xrealloc(
            *buf as *mut libc::c_void,
            (*off).wrapping_add(1 as libc::c_int as size_t),
        ) as *mut libc::c_char;
        let fresh3 = *off;
        *off = (*off).wrapping_add(1);
        *(*buf).offset(fresh3 as isize) = '\n' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn window_copy_clear_selection(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    screen_clear_selection(&mut (*data).screen);
    (*data).cursordrag = CURSORDRAG_NONE;
    py = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    px = window_copy_find_length(wp, py);
    if (*data).cx > px {
        window_copy_update_cursor(wp, px, (*data).cy);
    }
}
unsafe extern "C" fn window_copy_in_set(
    mut wp: *mut window_pane,
    mut px: u_int,
    mut py: u_int,
    mut set: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
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
    let mut ud: *const utf8_data = 0 as *const utf8_data;
    grid_get_cell((*(*data).backing).grid, px, py, &mut gc);
    ud = &mut gc.data;
    if (*ud).size as libc::c_int != 1 as libc::c_int
        || gc.flags as libc::c_int & 0x4 as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if *((*ud).data).as_ptr() as libc::c_int == 0 as libc::c_int
        || *((*ud).data).as_ptr() as libc::c_int == 0x7f as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (strchr(set, *((*ud).data).as_ptr() as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
unsafe extern "C" fn window_copy_find_length(
    mut wp: *mut window_pane,
    mut py: u_int,
) -> u_int {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    px = (*((*(*s).grid).linedata).offset(py as isize)).cellsize;
    if px > (*(*s).grid).sx {
        px = (*(*s).grid).sx;
    }
    while px > 0 as libc::c_int as u_int {
        grid_get_cell(
            (*s).grid,
            px.wrapping_sub(1 as libc::c_int as u_int),
            py,
            &mut gc,
        );
        if gc.data.size as libc::c_int != 1 as libc::c_int
            || *(gc.data.data).as_mut_ptr() as libc::c_int != ' ' as i32
        {
            break;
        }
        px = px.wrapping_sub(1);
        px;
    }
    return px;
}
unsafe extern "C" fn window_copy_cursor_start_of_line(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut s: *mut screen = &mut (*data).screen;
    let mut gd: *mut grid = (*back_s).grid;
    let mut py: u_int = 0;
    if (*data).cx == 0 as libc::c_int as u_int
        && (*s).sel.lineflag as libc::c_uint
            == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
        while py > 0 as libc::c_int as u_int
            && (*((*gd).linedata)
                .offset(py.wrapping_sub(1 as libc::c_int as u_int) as isize))
                .flags & 0x1 as libc::c_int != 0
        {
            window_copy_cursor_up(wp, 0 as libc::c_int);
            py = ((*(*back_s).grid).hsize)
                .wrapping_add((*data).cy)
                .wrapping_sub((*data).oy);
        }
    }
    window_copy_update_cursor(wp, 0 as libc::c_int as u_int, (*data).cy);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn window_copy_cursor_back_to_indentation(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
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
    px = 0 as libc::c_int as u_int;
    py = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    while px < xx {
        grid_get_cell((*(*data).backing).grid, px, py, &mut gc);
        if gc.data.size as libc::c_int != 1 as libc::c_int
            || *(gc.data.data).as_mut_ptr() as libc::c_int != ' ' as i32
        {
            break;
        }
        px = px.wrapping_add(1);
        px;
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn window_copy_cursor_end_of_line(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut s: *mut screen = &mut (*data).screen;
    let mut gd: *mut grid = (*back_s).grid;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    px = window_copy_find_length(wp, py);
    if (*data).cx == px
        && (*s).sel.lineflag as libc::c_uint
            == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        if (*data).screen.sel.flag != 0 && (*data).rectflag != 0 {
            px = (*(*back_s).grid).sx;
        }
        if (*((*gd).linedata).offset(py as isize)).flags & 0x1 as libc::c_int != 0 {
            while py < ((*gd).sy).wrapping_add((*gd).hsize)
                && (*((*gd).linedata).offset(py as isize)).flags & 0x1 as libc::c_int
                    != 0
            {
                window_copy_cursor_down(wp, 0 as libc::c_int);
                py = ((*(*back_s).grid).hsize)
                    .wrapping_add((*data).cy)
                    .wrapping_sub((*data).oy);
            }
            px = window_copy_find_length(wp, py);
        }
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn window_copy_other_end(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut selx: u_int = 0;
    let mut sely: u_int = 0;
    let mut cy: u_int = 0;
    let mut yy: u_int = 0;
    let mut hsize: u_int = 0;
    if (*s).sel.flag == 0
        && (*s).sel.lineflag as libc::c_uint
            == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        return;
    }
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint
    {
        (*s).sel.lineflag = LINE_SEL_RIGHT_LEFT;
    } else if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
    {
        (*s).sel.lineflag = LINE_SEL_LEFT_RIGHT;
    }
    match (*data).cursordrag as libc::c_uint {
        0 | 2 => {
            (*data).cursordrag = CURSORDRAG_ENDSEL;
        }
        1 => {
            (*data).cursordrag = CURSORDRAG_SEL;
        }
        _ => {}
    }
    selx = (*data).endselx;
    sely = (*data).endsely;
    if (*data).cursordrag as libc::c_uint
        == CURSORDRAG_SEL as libc::c_int as libc::c_uint
    {
        selx = (*data).selx;
        sely = (*data).sely;
    }
    cy = (*data).cy;
    yy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).cx = selx;
    hsize = (*(*(*data).backing).grid).hsize;
    if sely < hsize.wrapping_sub((*data).oy) {
        (*data).oy = hsize.wrapping_sub(sely);
        (*data).cy = 0 as libc::c_int as u_int;
    } else if sely > hsize.wrapping_sub((*data).oy).wrapping_add((*(*s).grid).sy) {
        (*data)
            .oy = hsize
            .wrapping_sub(sely)
            .wrapping_add((*(*s).grid).sy)
            .wrapping_sub(1 as libc::c_int as u_int);
        (*data).cy = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    } else {
        (*data).cy = cy.wrapping_add(sely).wrapping_sub(yy);
    }
    window_copy_update_selection(wp, 1 as libc::c_int);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_cursor_left(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut py: u_int = 0;
    let mut cx: u_int = 0;
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
    py = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    cx = (*data).cx;
    while cx > 0 as libc::c_int as u_int {
        grid_get_cell((*(*data).backing).grid, cx, py, &mut gc);
        if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            break;
        }
        cx = cx.wrapping_sub(1);
        cx;
    }
    if cx == 0 as libc::c_int as u_int && py > 0 as libc::c_int as u_int {
        window_copy_cursor_up(wp, 0 as libc::c_int);
        window_copy_cursor_end_of_line(wp);
    } else if cx > 0 as libc::c_int as u_int {
        window_copy_update_cursor(
            wp,
            cx.wrapping_sub(1 as libc::c_int as u_int),
            (*data).cy,
        );
        if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
            window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
        }
    }
}
unsafe extern "C" fn window_copy_cursor_right(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
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
    py = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    yy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*(*(*data).backing).grid).sy)
        .wrapping_sub(1 as libc::c_int as u_int);
    if (*data).screen.sel.flag != 0 && (*data).rectflag != 0 {
        px = (*(*data).screen.grid).sx;
    } else {
        px = window_copy_find_length(wp, py);
    }
    if (*data).cx >= px && py < yy {
        window_copy_cursor_start_of_line(wp);
        window_copy_cursor_down(wp, 0 as libc::c_int);
    } else if (*data).cx < px {
        cx = ((*data).cx).wrapping_add(1 as libc::c_int as u_int);
        cy = ((*(*(*data).backing).grid).hsize)
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        while cx < px {
            grid_get_cell((*(*data).backing).grid, cx, cy, &mut gc);
            if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            cx = cx.wrapping_add(1);
            cx;
        }
        window_copy_update_cursor(wp, cx, (*data).cy);
        if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
            window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
        }
    }
}
unsafe extern "C" fn window_copy_cursor_up(
    mut wp: *mut window_pane,
    mut scroll_only: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox;
    }
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint && oy == (*data).sely
    {
        window_copy_other_end(wp);
    }
    (*data).cx = (*data).lastcx;
    if scroll_only != 0 || (*data).cy == 0 as libc::c_int as u_int {
        window_copy_scroll_down(wp, 1 as libc::c_int as u_int);
        if scroll_only != 0 {
            if (*data).cy == ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
                window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
            } else {
                window_copy_redraw_lines(wp, (*data).cy, 2 as libc::c_int as u_int);
            }
        }
    } else {
        window_copy_update_cursor(
            wp,
            (*data).cx,
            ((*data).cy).wrapping_sub(1 as libc::c_int as u_int),
        );
        if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
            if (*data).cy == ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
                window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
            } else {
                window_copy_redraw_lines(wp, (*data).cy, 2 as libc::c_int as u_int);
            }
        }
    }
    if (*data).screen.sel.flag == 0 || (*data).rectflag == 0 {
        py = ((*(*(*data).backing).grid).hsize)
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wp);
        }
    }
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint
    {
        window_copy_cursor_end_of_line(wp);
    } else if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
    {
        window_copy_cursor_start_of_line(wp);
    }
}
unsafe extern "C" fn window_copy_cursor_down(
    mut wp: *mut window_pane,
    mut scroll_only: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wp, oy);
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox;
    }
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint && oy == (*data).endsely
    {
        window_copy_other_end(wp);
    }
    (*data).cx = (*data).lastcx;
    if scroll_only != 0
        || (*data).cy == ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int)
    {
        window_copy_scroll_up(wp, 1 as libc::c_int as u_int);
        if scroll_only != 0 && (*data).cy > 0 as libc::c_int as u_int {
            window_copy_redraw_lines(
                wp,
                ((*data).cy).wrapping_sub(1 as libc::c_int as u_int),
                2 as libc::c_int as u_int,
            );
        }
    } else {
        window_copy_update_cursor(
            wp,
            (*data).cx,
            ((*data).cy).wrapping_add(1 as libc::c_int as u_int),
        );
        if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
            window_copy_redraw_lines(
                wp,
                ((*data).cy).wrapping_sub(1 as libc::c_int as u_int),
                2 as libc::c_int as u_int,
            );
        }
    }
    if (*data).screen.sel.flag == 0 || (*data).rectflag == 0 {
        py = ((*(*(*data).backing).grid).hsize)
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wp, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wp);
        }
    }
    if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint
    {
        window_copy_cursor_end_of_line(wp);
    } else if (*s).sel.lineflag as libc::c_uint
        == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
    {
        window_copy_cursor_start_of_line(wp);
    }
}
unsafe extern "C" fn window_copy_cursor_jump(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    px = ((*data).cx).wrapping_add(1 as libc::c_int as u_int);
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    while px < xx {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *(gc.data.data).as_mut_ptr() as libc::c_int
                == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(wp, px, (*data).cy);
            if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
                window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        px = px.wrapping_add(1);
        px;
    }
}
unsafe extern "C" fn window_copy_cursor_jump_back(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    if px > 0 as libc::c_int as u_int {
        px = px.wrapping_sub(1);
        px;
    }
    loop {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *(gc.data.data).as_mut_ptr() as libc::c_int
                == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(wp, px, (*data).cy);
            if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
                window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        if px == 0 as libc::c_int as u_int {
            break;
        }
        px = px.wrapping_sub(1);
        px;
    };
}
unsafe extern "C" fn window_copy_cursor_jump_to(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    px = ((*data).cx).wrapping_add(2 as libc::c_int as u_int);
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    while px < xx {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *(gc.data.data).as_mut_ptr() as libc::c_int
                == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(
                wp,
                px.wrapping_sub(1 as libc::c_int as u_int),
                (*data).cy,
            );
            if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
                window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        px = px.wrapping_add(1);
        px;
    }
}
unsafe extern "C" fn window_copy_cursor_jump_to_back(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    if px > 0 as libc::c_int as u_int {
        px = px.wrapping_sub(1);
        px;
    }
    if px > 0 as libc::c_int as u_int {
        px = px.wrapping_sub(1);
        px;
    }
    loop {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *(gc.data.data).as_mut_ptr() as libc::c_int
                == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(
                wp,
                px.wrapping_add(1 as libc::c_int as u_int),
                (*data).cy,
            );
            if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
                window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        if px == 0 as libc::c_int as u_int {
            break;
        }
        px = px.wrapping_sub(1);
        px;
    };
}
unsafe extern "C" fn window_copy_cursor_next_word(
    mut wp: *mut window_pane,
    mut separators: *const libc::c_char,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut expected: libc::c_int = 0 as libc::c_int;
    px = (*data).cx;
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    yy = ((*(*back_s).grid).hsize)
        .wrapping_add((*(*back_s).grid).sy)
        .wrapping_sub(1 as libc::c_int as u_int);
    loop {
        while px > xx || window_copy_in_set(wp, px, py, separators) == expected {
            if px > xx {
                if py == yy {
                    return;
                }
                window_copy_cursor_down(wp, 0 as libc::c_int);
                px = 0 as libc::c_int as u_int;
                py = ((*(*back_s).grid).hsize)
                    .wrapping_add((*data).cy)
                    .wrapping_sub((*data).oy);
                xx = window_copy_find_length(wp, py);
            } else {
                px = px.wrapping_add(1);
                px;
            }
        }
        expected = (expected == 0) as libc::c_int;
        if !(expected == 1 as libc::c_int) {
            break;
        }
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn window_copy_cursor_next_word_end(
    mut wp: *mut window_pane,
    mut separators: *const libc::c_char,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut oo: *mut options = (*(*wp).window).options;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut keys: libc::c_int = 0;
    let mut expected: libc::c_int = 1 as libc::c_int;
    px = (*data).cx;
    py = ((*(*back_s).grid).hsize).wrapping_add((*data).cy).wrapping_sub((*data).oy);
    xx = window_copy_find_length(wp, py);
    yy = ((*(*back_s).grid).hsize)
        .wrapping_add((*(*back_s).grid).sy)
        .wrapping_sub(1 as libc::c_int as u_int);
    keys = options_get_number(oo, b"mode-keys\0" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if keys == 1 as libc::c_int && window_copy_in_set(wp, px, py, separators) == 0 {
        px = px.wrapping_add(1);
        px;
    }
    loop {
        while px > xx || window_copy_in_set(wp, px, py, separators) == expected {
            if px > xx {
                if py == yy {
                    return;
                }
                window_copy_cursor_down(wp, 0 as libc::c_int);
                px = 0 as libc::c_int as u_int;
                py = ((*(*back_s).grid).hsize)
                    .wrapping_add((*data).cy)
                    .wrapping_sub((*data).oy);
                xx = window_copy_find_length(wp, py);
            } else {
                px = px.wrapping_add(1);
                px;
            }
        }
        expected = (expected == 0) as libc::c_int;
        if !(expected == 0 as libc::c_int) {
            break;
        }
    }
    if keys == 1 as libc::c_int && px != 0 as libc::c_int as u_int {
        px = px.wrapping_sub(1);
        px;
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn window_copy_cursor_previous_word(
    mut wp: *mut window_pane,
    mut separators: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    loop {
        if px > 0 as libc::c_int as u_int {
            px = px.wrapping_sub(1);
            px;
            if window_copy_in_set(wp, px, py, separators) == 0 {
                current_block = 13109137661213826276;
                break;
            }
        } else {
            if (*data).cy == 0 as libc::c_int as u_int
                && ((*(*(*data).backing).grid).hsize == 0 as libc::c_int as u_int
                    || (*data).oy
                        >= ((*(*(*data).backing).grid).hsize)
                            .wrapping_sub(1 as libc::c_int as u_int))
            {
                current_block = 649868609856900983;
                break;
            }
            window_copy_cursor_up(wp, 0 as libc::c_int);
            py = ((*(*(*data).backing).grid).hsize)
                .wrapping_add((*data).cy)
                .wrapping_sub((*data).oy);
            px = window_copy_find_length(wp, py);
        }
    }
    match current_block {
        13109137661213826276 => {
            while px > 0 as libc::c_int as u_int
                && window_copy_in_set(
                    wp,
                    px.wrapping_sub(1 as libc::c_int as u_int),
                    py,
                    separators,
                ) == 0
            {
                px = px.wrapping_sub(1);
                px;
            }
        }
        _ => {}
    }
    window_copy_update_cursor(wp, px, (*data).cy);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_lines(wp, (*data).cy, 1 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn window_copy_scroll_up(mut wp: *mut window_pane, mut ny: u_int) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
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
    if (*data).oy < ny {
        ny = (*data).oy;
    }
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    (*data).oy = ((*data).oy).wrapping_sub(ny);
    window_copy_update_selection(wp, 0 as libc::c_int);
    screen_write_start(&mut ctx, wp, 0 as *mut screen);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    screen_write_deleteline(&mut ctx, ny, 8 as libc::c_int as u_int);
    window_copy_write_lines(wp, &mut ctx, ((*(*s).grid).sy).wrapping_sub(ny), ny);
    window_copy_write_line(wp, &mut ctx, 0 as libc::c_int as u_int);
    if (*(*s).grid).sy > 1 as libc::c_int as u_int {
        window_copy_write_line(wp, &mut ctx, 1 as libc::c_int as u_int);
    }
    if (*(*s).grid).sy > 3 as libc::c_int as u_int {
        window_copy_write_line(
            wp,
            &mut ctx,
            ((*(*s).grid).sy).wrapping_sub(2 as libc::c_int as u_int),
        );
    }
    if (*s).sel.flag != 0 && (*(*s).grid).sy > ny {
        window_copy_write_line(
            wp,
            &mut ctx,
            ((*(*s).grid).sy).wrapping_sub(ny).wrapping_sub(1 as libc::c_int as u_int),
        );
    }
    screen_write_cursormove(&mut ctx, (*data).cx, (*data).cy);
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn window_copy_scroll_down(mut wp: *mut window_pane, mut ny: u_int) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
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
    if ny > (*(*(*data).backing).grid).hsize {
        return;
    }
    if (*data).oy > ((*(*(*data).backing).grid).hsize).wrapping_sub(ny) {
        ny = ((*(*(*data).backing).grid).hsize).wrapping_sub((*data).oy);
    }
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    (*data).oy = ((*data).oy).wrapping_add(ny);
    window_copy_update_selection(wp, 0 as libc::c_int);
    screen_write_start(&mut ctx, wp, 0 as *mut screen);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    screen_write_insertline(&mut ctx, ny, 8 as libc::c_int as u_int);
    window_copy_write_lines(wp, &mut ctx, 0 as libc::c_int as u_int, ny);
    if (*s).sel.flag != 0 && (*(*s).grid).sy > ny {
        window_copy_write_line(wp, &mut ctx, ny);
    } else if ny == 1 as libc::c_int as u_int {
        window_copy_write_line(wp, &mut ctx, 1 as libc::c_int as u_int);
    }
    screen_write_cursormove(&mut ctx, (*data).cx, (*data).cy);
    screen_write_stop(&mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_add_formats(
    mut wp: *mut window_pane,
    mut ft: *mut format_tree,
) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    if (*wp).mode != &window_copy_mode as *const window_mode {
        return;
    }
    format_add(
        ft,
        b"selection_present\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*s).sel.flag,
    );
    format_add(
        ft,
        b"scroll_position\0" as *const u8 as *const libc::c_char,
        b"%d\0" as *const u8 as *const libc::c_char,
        (*data).oy,
    );
}
unsafe extern "C" fn window_copy_rectangle_toggle(mut wp: *mut window_pane) {
    let mut data: *mut window_copy_mode_data = (*wp).modedata
        as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    (*data).rectflag = ((*data).rectflag == 0) as libc::c_int;
    py = ((*(*(*data).backing).grid).hsize)
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    px = window_copy_find_length(wp, py);
    if (*data).cx > px {
        window_copy_update_cursor(wp, px, (*data).cy);
    }
    window_copy_update_selection(wp, 1 as libc::c_int);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_move_mouse(mut m: *mut mouse_event) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() || (*wp).mode != &window_copy_mode as *const window_mode {
        return;
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    window_copy_update_cursor(wp, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_start_drag(
    mut c: *mut client,
    mut m: *mut mouse_event,
) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if c.is_null() {
        return;
    }
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() || (*wp).mode != &window_copy_mode as *const window_mode {
        return;
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 1 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    (*c)
        .tty
        .mouse_drag_update = Some(
        window_copy_drag_update
            as unsafe extern "C" fn(*mut client, *mut mouse_event) -> (),
    );
    (*c).tty.mouse_drag_release = None;
    window_copy_update_cursor(wp, x, y);
    window_copy_start_selection(wp);
    window_copy_redraw_screen(wp);
}
unsafe extern "C" fn window_copy_drag_update(
    mut c: *mut client,
    mut m: *mut mouse_event,
) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut old_cy: u_int = 0;
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() || (*wp).mode != &window_copy_mode as *const window_mode {
        return;
    }
    data = (*wp).modedata as *mut window_copy_mode_data;
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    old_cy = (*data).cy;
    window_copy_update_cursor(wp, x, y);
    if window_copy_update_selection(wp, 1 as libc::c_int) != 0 {
        window_copy_redraw_selection(wp, old_cy);
    }
}
