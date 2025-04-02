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
    pub type screen_write_collect_line;
    pub type screen_write_collect_item;
    pub type format_tree;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    pub type options_entry;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __b64_pton(
        _: *const libc::c_char,
        _: *mut libc::c_uchar,
        _: size_t,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn bsearch(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut libc::c_void;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn evbuffer_new() -> *mut evbuffer;
    fn evbuffer_free(buf: *mut evbuffer);
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    fn evbuffer_add(
        buf: *mut evbuffer,
        data: *const libc::c_void,
        datlen: size_t,
    ) -> libc::c_int;
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut global_options: *mut options;
    fn paste_add(_: *mut libc::c_char, _: size_t);
    fn notify_input(_: *mut window_pane, _: *mut evbuffer);
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn options_set_number(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut options_entry;
    fn alerts_queue(_: *mut window, _: libc::c_int);
    fn server_status_window(_: *mut window);
    fn colour_join_rgb(_: u_char, _: u_char, _: u_char) -> libc::c_int;
    static grid_default_cell: grid_cell;
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    fn screen_write_stop(_: *mut screen_write_ctx);
    fn screen_write_reset(_: *mut screen_write_ctx);
    fn screen_write_backspace(_: *mut screen_write_ctx);
    fn screen_write_mode_set(_: *mut screen_write_ctx, _: libc::c_int);
    fn screen_write_mode_clear(_: *mut screen_write_ctx, _: libc::c_int);
    fn screen_write_cursorup(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_cursordown(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_cursorright(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_cursorleft(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_alignmenttest(_: *mut screen_write_ctx);
    fn screen_write_insertcharacter(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_deletecharacter(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_clearcharacter(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_insertline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_deleteline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_clearline(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_clearstartofline(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_reverseindex(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_scrollregion(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int, _: u_int);
    fn screen_write_scrollup(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_carriagereturn(_: *mut screen_write_ctx);
    fn screen_write_clearendofscreen(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_clearstartofscreen(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int);
    fn screen_write_clearhistory(_: *mut screen_write_ctx);
    fn screen_write_collect_end(_: *mut screen_write_ctx);
    fn screen_write_collect_add(_: *mut screen_write_ctx, _: *const grid_cell);
    fn screen_write_setselection(_: *mut screen_write_ctx, _: *mut u_char, _: u_int);
    fn screen_write_rawstring(_: *mut screen_write_ctx, _: *mut u_char, _: u_int);
    fn screen_set_cursor_style(_: *mut screen, _: u_int);
    fn screen_set_cursor_colour(_: *mut screen, _: *const libc::c_char);
    fn screen_set_title(_: *mut screen, _: *const libc::c_char);
    fn screen_push_title(_: *mut screen);
    fn screen_pop_title(_: *mut screen);
    fn window_update_activity(_: *mut window);
    fn window_pane_alternate_on(_: *mut window_pane, _: *mut grid_cell, _: libc::c_int);
    fn window_pane_alternate_off(_: *mut window_pane, _: *mut grid_cell, _: libc::c_int);
    fn window_pane_set_palette(_: *mut window_pane, _: u_int, _: libc::c_int);
    fn window_pane_unset_palette(_: *mut window_pane, _: u_int);
    fn window_pane_reset_palette(_: *mut window_pane);
    fn window_set_name(_: *mut window, _: *const libc::c_char);
    fn utf8_set(_: *mut utf8_data, _: u_char);
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_isvalid(_: *const libc::c_char) -> libc::c_int;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    fn input_reply(_: *mut input_ctx, _: *const libc::c_char, _: ...);
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
    pub message_log: C2RustUnnamed_26,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_25,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_ctx {
    pub wp: *mut window_pane,
    pub ctx: screen_write_ctx,
    pub cell: input_cell,
    pub old_cell: input_cell,
    pub old_cx: u_int,
    pub old_cy: u_int,
    pub interm_buf: [u_char; 4],
    pub interm_len: size_t,
    pub param_buf: [u_char; 64],
    pub param_len: size_t,
    pub input_buf: *mut u_char,
    pub input_len: size_t,
    pub input_space: size_t,
    pub param_list: [input_param; 24],
    pub param_list_len: u_int,
    pub utf8data: utf8_data,
    pub utf8started: libc::c_int,
    pub ch: libc::c_int,
    pub last: libc::c_int,
    pub flags: libc::c_int,
    pub state: *const input_state,
    pub timer: event,
    pub since_ground: *mut evbuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_state {
    pub name: *const libc::c_char,
    pub enter: Option::<unsafe extern "C" fn(*mut input_ctx) -> ()>,
    pub exit: Option::<unsafe extern "C" fn(*mut input_ctx) -> ()>,
    pub transitions: *const input_transition,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_transition {
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub handler: Option::<unsafe extern "C" fn(*mut input_ctx) -> libc::c_int>,
    pub state: *const input_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_param {
    pub type_0: C2RustUnnamed_24,
    pub c2rust_unnamed: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub num: libc::c_int,
    pub str_0: *mut libc::c_char,
}
pub type C2RustUnnamed_24 = libc::c_uint;
pub const INPUT_STRING: C2RustUnnamed_24 = 2;
pub const INPUT_NUMBER: C2RustUnnamed_24 = 1;
pub const INPUT_MISSING: C2RustUnnamed_24 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_cell {
    pub cell: grid_cell,
    pub set: libc::c_int,
    pub g0set: libc::c_int,
    pub g1set: libc::c_int,
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
pub type C2RustUnnamed_25 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_25 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_25 = 0;
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
pub struct C2RustUnnamed_26 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
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
    pub entry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
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
    pub qentry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_33,
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
    pub entry: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
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
pub struct C2RustUnnamed_33 {
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
    pub term_type: C2RustUnnamed_34,
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
pub type C2RustUnnamed_34 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_34 = 6;
pub const TTY_VT420: C2RustUnnamed_34 = 5;
pub const TTY_VT320: C2RustUnnamed_34 = 4;
pub const TTY_VT220: C2RustUnnamed_34 = 3;
pub const TTY_VT102: C2RustUnnamed_34 = 2;
pub const TTY_VT101: C2RustUnnamed_34 = 1;
pub const TTY_VT100: C2RustUnnamed_34 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
pub const INPUT_ESC_ST: input_esc_type = 14;
pub const INPUT_ESC_SCSG1_OFF: input_esc_type = 12;
pub const INPUT_ESC_SCSG1_ON: input_esc_type = 13;
pub const INPUT_ESC_SCSG0_OFF: input_esc_type = 10;
pub const INPUT_ESC_SCSG0_ON: input_esc_type = 11;
pub const INPUT_ESC_DECALN: input_esc_type = 0;
pub const INPUT_ESC_DECRC: input_esc_type = 3;
pub const INPUT_ESC_DECSC: input_esc_type = 4;
pub const INPUT_ESC_DECKPNM: input_esc_type = 2;
pub const INPUT_ESC_DECKPAM: input_esc_type = 1;
pub const INPUT_ESC_RI: input_esc_type = 8;
pub const INPUT_ESC_HTS: input_esc_type = 5;
pub const INPUT_ESC_NEL: input_esc_type = 7;
pub const INPUT_ESC_IND: input_esc_type = 6;
pub const INPUT_ESC_RIS: input_esc_type = 9;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_table_entry {
    pub ch: libc::c_int,
    pub interm: *const libc::c_char,
    pub type_0: libc::c_int,
}
pub const INPUT_CSI_DECSCUSR: input_csi_type = 11;
pub const INPUT_CSI_VPA: input_csi_type = 31;
pub const INPUT_CSI_TBC: input_csi_type = 30;
pub const INPUT_CSI_SU: input_csi_type = 29;
pub const INPUT_CSI_SM_PRIVATE: input_csi_type = 28;
pub const INPUT_CSI_SM: input_csi_type = 27;
pub const INPUT_CSI_SGR: input_csi_type = 26;
pub const INPUT_CSI_SCP: input_csi_type = 25;
pub const INPUT_CSI_RM_PRIVATE: input_csi_type = 24;
pub const INPUT_CSI_RM: input_csi_type = 23;
pub const INPUT_CSI_RCP: input_csi_type = 21;
pub const INPUT_CSI_REP: input_csi_type = 22;
pub const INPUT_CSI_IL: input_csi_type = 20;
pub const INPUT_CSI_ICH: input_csi_type = 19;
pub const INPUT_CSI_HPA: input_csi_type = 18;
pub const INPUT_CSI_EL: input_csi_type = 17;
pub const INPUT_CSI_ED: input_csi_type = 16;
pub const INPUT_CSI_DSR: input_csi_type = 14;
pub const INPUT_CSI_DL: input_csi_type = 13;
pub const INPUT_CSI_DECSTBM: input_csi_type = 12;
pub const INPUT_CSI_DCH: input_csi_type = 10;
pub const INPUT_CSI_ECH: input_csi_type = 15;
pub const INPUT_CSI_DA_TWO: input_csi_type = 9;
pub const INPUT_CSI_DA: input_csi_type = 8;
pub const INPUT_CSI_CPL: input_csi_type = 2;
pub const INPUT_CSI_CNL: input_csi_type = 1;
pub const INPUT_CSI_CUU: input_csi_type = 7;
pub const INPUT_CSI_WINOPS: input_csi_type = 32;
pub const INPUT_CSI_CUP: input_csi_type = 6;
pub const INPUT_CSI_CUF: input_csi_type = 5;
pub const INPUT_CSI_CUD: input_csi_type = 4;
pub const INPUT_CSI_CUB: input_csi_type = 3;
pub const INPUT_CSI_CBT: input_csi_type = 0;
pub type input_esc_type = libc::c_uint;
pub type input_csi_type = libc::c_uint;
static mut input_esc_table: [input_table_entry; 15] = [
    {
        let mut init = input_table_entry {
            ch: '0' as i32,
            interm: b"(\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG0_ON as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '0' as i32,
            interm: b")\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG1_ON as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '7' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECSC as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '8' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECRC as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '8' as i32,
            interm: b"#\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECALN as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '=' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECKPAM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '>' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECKPNM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'B' as i32,
            interm: b"(\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG0_OFF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'B' as i32,
            interm: b")\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG1_OFF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'D' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_IND as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'E' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_NEL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'H' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_HTS as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'M' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_RI as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '\\' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_ST as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'c' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_RIS as libc::c_int,
        };
        init
    },
];
static mut input_csi_table: [input_table_entry; 34] = [
    {
        let mut init = input_table_entry {
            ch: '@' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_ICH as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'A' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUU as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'B' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUD as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'C' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'D' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUB as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'E' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CNL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'F' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CPL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'G' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_HPA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'H' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'J' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_ED as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'K' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_EL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'L' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_IL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'M' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'P' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DCH as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'S' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SU as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'X' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_ECH as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'Z' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CBT as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'b' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_REP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'c' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'c' as i32,
            interm: b">\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DA_TWO as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'd' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_VPA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'f' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'g' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_TBC as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'h' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'h' as i32,
            interm: b"?\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SM_PRIVATE as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'l' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_RM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'l' as i32,
            interm: b"?\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_RM_PRIVATE as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'm' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SGR as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'n' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DSR as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'q' as i32,
            interm: b" \0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DECSCUSR as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'r' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DECSTBM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 's' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SCP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 't' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_WINOPS as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'u' as i32,
            interm: b"\0" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_RCP as libc::c_int,
        };
        init
    },
];
static mut input_state_ground: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"ground\0" as *const u8 as *const libc::c_char,
            enter: Some(input_ground as unsafe extern "C" fn(*mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_ground_table.as_ptr(),
        };
        init
    }
};
static mut input_state_esc_enter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"esc_enter\0" as *const u8 as *const libc::c_char,
            enter: Some(input_clear as unsafe extern "C" fn(*mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_esc_enter_table.as_ptr(),
        };
        init
    }
};
static mut input_state_esc_intermediate: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"esc_intermediate\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_esc_intermediate_table.as_ptr(),
        };
        init
    }
};
static mut input_state_csi_enter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_enter\0" as *const u8 as *const libc::c_char,
            enter: Some(input_clear as unsafe extern "C" fn(*mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_csi_enter_table.as_ptr(),
        };
        init
    }
};
static mut input_state_csi_parameter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_parameter\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_csi_parameter_table.as_ptr(),
        };
        init
    }
};
static mut input_state_csi_intermediate: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_intermediate\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_csi_intermediate_table.as_ptr(),
        };
        init
    }
};
static mut input_state_csi_ignore: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_ignore\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_csi_ignore_table.as_ptr(),
        };
        init
    }
};
static mut input_state_dcs_enter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_enter\0" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_dcs as unsafe extern "C" fn(*mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_dcs_enter_table.as_ptr(),
        };
        init
    }
};
static mut input_state_dcs_parameter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_parameter\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_parameter_table.as_ptr(),
        };
        init
    }
};
static mut input_state_dcs_intermediate: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_intermediate\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_intermediate_table.as_ptr(),
        };
        init
    }
};
static mut input_state_dcs_handler: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_handler\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_handler_table.as_ptr(),
        };
        init
    }
};
static mut input_state_dcs_escape: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_escape\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_escape_table.as_ptr(),
        };
        init
    }
};
static mut input_state_dcs_ignore: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_ignore\0" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_ignore_table.as_ptr(),
        };
        init
    }
};
static mut input_state_osc_string: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"osc_string\0" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_osc as unsafe extern "C" fn(*mut input_ctx) -> ()),
            exit: Some(input_exit_osc as unsafe extern "C" fn(*mut input_ctx) -> ()),
            transitions: input_state_osc_string_table.as_ptr(),
        };
        init
    }
};
static mut input_state_apc_string: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"apc_string\0" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_apc as unsafe extern "C" fn(*mut input_ctx) -> ()),
            exit: Some(input_exit_apc as unsafe extern "C" fn(*mut input_ctx) -> ()),
            transitions: input_state_apc_string_table.as_ptr(),
        };
        init
    }
};
static mut input_state_rename_string: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"rename_string\0" as *const u8 as *const libc::c_char,
            enter: Some(
                input_enter_rename as unsafe extern "C" fn(*mut input_ctx) -> (),
            ),
            exit: Some(input_exit_rename as unsafe extern "C" fn(*mut input_ctx) -> ()),
            transitions: input_state_rename_string_table.as_ptr(),
        };
        init
    }
};
static mut input_state_consume_st: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"consume_st\0" as *const u8 as *const libc::c_char,
            enter: Some(
                input_enter_rename as unsafe extern "C" fn(*mut input_ctx) -> (),
            ),
            exit: None,
            transitions: input_state_consume_st_table.as_ptr(),
        };
        init
    }
};
static mut input_state_ground_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_print as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0x7f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x80 as libc::c_int,
                last: 0xff as libc::c_int,
                handler: Some(
                    input_top_bit_set
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_esc_enter_table: [input_transition; 23] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_esc_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x4f as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x50 as libc::c_int,
                last: 0x50 as libc::c_int,
                handler: None,
                state: &input_state_dcs_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x51 as libc::c_int,
                last: 0x57 as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x58 as libc::c_int,
                last: 0x58 as libc::c_int,
                handler: None,
                state: &input_state_consume_st as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x59 as libc::c_int,
                last: 0x59 as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5a as libc::c_int,
                last: 0x5a as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5b as libc::c_int,
                last: 0x5b as libc::c_int,
                handler: None,
                state: &input_state_csi_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5c as libc::c_int,
                last: 0x5c as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5d as libc::c_int,
                last: 0x5d as libc::c_int,
                handler: None,
                state: &input_state_osc_string as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5e as libc::c_int,
                last: 0x5e as libc::c_int,
                handler: None,
                state: &input_state_consume_st as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5f as libc::c_int,
                last: 0x5f as libc::c_int,
                handler: None,
                state: &input_state_apc_string as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x60 as libc::c_int,
                last: 0x6a as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x6b as libc::c_int,
                last: 0x6b as libc::c_int,
                handler: None,
                state: &input_state_rename_string as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x6c as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_esc_intermediate_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_esc_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_csi_enter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x39 as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3a as libc::c_int,
                last: 0x3a as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3b as libc::c_int,
                last: 0x3b as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3c as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_csi_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_csi_parameter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x39 as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3a as libc::c_int,
                last: 0x3a as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3b as libc::c_int,
                last: 0x3b as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3c as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: None,
                state: &input_state_csi_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_csi_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_csi_intermediate_table: [input_transition; 11] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: None,
                state: &input_state_csi_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_csi_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_csi_ignore_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: None,
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_dcs_enter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x39 as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3a as libc::c_int,
                last: 0x3a as libc::c_int,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3b as libc::c_int,
                last: 0x3b as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3c as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_dcs_parameter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x39 as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3a as libc::c_int,
                last: 0x3a as libc::c_int,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3b as libc::c_int,
                last: 0x3b as libc::c_int,
                handler: Some(
                    input_parameter
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3c as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_dcs_intermediate_table: [input_transition; 11] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0x2f as libc::c_int,
                handler: Some(
                    input_intermediate
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30 as libc::c_int,
                last: 0x3f as libc::c_int,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40 as libc::c_int,
                last: 0x7e as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7f as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_dcs_handler_table: [input_transition; 4] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_dcs_escape as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0xff as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_dcs_escape_table: [input_transition; 4] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x5b as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5c as libc::c_int,
                last: 0x5c as libc::c_int,
                handler: Some(
                    input_dcs_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5d as libc::c_int,
                last: 0xff as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_dcs_ignore_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_osc_string_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x6 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7 as libc::c_int,
                last: 0x7 as libc::c_int,
                handler: None,
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x8 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0xff as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_apc_string_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0xff as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_rename_string_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0xff as libc::c_int,
                handler: Some(
                    input_input as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
static mut input_state_consume_st_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18 as libc::c_int,
                last: 0x18 as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1a as libc::c_int,
                last: 0x1a as libc::c_int,
                handler: Some(
                    input_c0_dispatch
                        as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1b as libc::c_int,
                last: 0x1b as libc::c_int,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0 as libc::c_int,
                last: 0x17 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19 as libc::c_int,
                last: 0x19 as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1c as libc::c_int,
                last: 0x1f as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20 as libc::c_int,
                last: 0xff as libc::c_int,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1 as libc::c_int),
                last: -(1 as libc::c_int),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
unsafe extern "C" fn input_table_compare(
    mut key: *const libc::c_void,
    mut value: *const libc::c_void,
) -> libc::c_int {
    let mut ictx: *const input_ctx = key as *const input_ctx;
    let mut entry: *const input_table_entry = value as *const input_table_entry;
    if (*ictx).ch != (*entry).ch {
        return (*ictx).ch - (*entry).ch;
    }
    return strcmp(((*ictx).interm_buf).as_ptr() as *const libc::c_char, (*entry).interm);
}
unsafe extern "C" fn input_timer_callback(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut ictx: *mut input_ctx = arg as *mut input_ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    log_debug(
        b"%s: %%%u %s expired\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"input_timer_callback\0"))
            .as_ptr(),
        (*wp).id,
        (*(*ictx).state).name,
    );
    input_reset(wp, 0 as libc::c_int);
}
unsafe extern "C" fn input_start_timer(mut ictx: *mut input_ctx) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 100000 as libc::c_int as __suseconds_t,
        };
        init
    };
    event_del(&mut (*ictx).timer);
    event_add(&mut (*ictx).timer, &mut tv);
}
unsafe extern "C" fn input_reset_cell(mut ictx: *mut input_ctx) {
    memcpy(
        &mut (*ictx).cell.cell as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    (*ictx).cell.set = 0 as libc::c_int;
    (*ictx).cell.g1set = 0 as libc::c_int;
    (*ictx).cell.g0set = (*ictx).cell.g1set;
    memcpy(
        &mut (*ictx).old_cell as *mut input_cell as *mut libc::c_void,
        &mut (*ictx).cell as *mut input_cell as *const libc::c_void,
        ::core::mem::size_of::<input_cell>() as libc::c_ulong,
    );
    (*ictx).old_cx = 0 as libc::c_int as u_int;
    (*ictx).old_cy = 0 as libc::c_int as u_int;
}
#[no_mangle]
pub unsafe extern "C" fn input_init(mut wp: *mut window_pane) {
    let mut ictx: *mut input_ctx = 0 as *mut input_ctx;
    (*wp)
        .ictx = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<input_ctx>() as libc::c_ulong,
    ) as *mut input_ctx;
    ictx = (*wp).ictx;
    (*ictx).input_space = 32 as libc::c_int as size_t;
    (*ictx).input_buf = xmalloc(32 as libc::c_int as size_t) as *mut u_char;
    (*ictx).since_ground = evbuffer_new();
    event_set(
        &mut (*ictx).timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            input_timer_callback
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        ictx as *mut libc::c_void,
    );
    input_reset(wp, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn input_free(mut wp: *mut window_pane) {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 as libc::c_uint
            == INPUT_STRING as libc::c_int as libc::c_uint
        {
            free(
                (*ictx).param_list[i as usize].c2rust_unnamed.str_0 as *mut libc::c_void,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    event_del(&mut (*ictx).timer);
    free((*ictx).input_buf as *mut libc::c_void);
    evbuffer_free((*ictx).since_ground);
    free(ictx as *mut libc::c_void);
    (*wp).ictx = 0 as *mut input_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn input_reset(mut wp: *mut window_pane, mut clear: libc::c_int) {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    input_reset_cell(ictx);
    if clear != 0 {
        if ((*wp).mode).is_null() {
            screen_write_start(&mut (*ictx).ctx, wp, &mut (*wp).base);
        } else {
            screen_write_start(&mut (*ictx).ctx, 0 as *mut window_pane, &mut (*wp).base);
        }
        screen_write_reset(&mut (*ictx).ctx);
        screen_write_stop(&mut (*ictx).ctx);
    }
    input_clear(ictx);
    (*ictx).last = -(1 as libc::c_int);
    (*ictx).state = &input_state_ground;
    (*ictx).flags = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn input_pending(mut wp: *mut window_pane) -> *mut evbuffer {
    return (*(*wp).ictx).since_ground;
}
unsafe extern "C" fn input_set_state(
    mut wp: *mut window_pane,
    mut itr: *const input_transition,
) {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    if ((*(*ictx).state).exit).is_some() {
        ((*(*ictx).state).exit).expect("non-null function pointer")(ictx);
    }
    (*ictx).state = (*itr).state;
    if ((*(*ictx).state).enter).is_some() {
        ((*(*ictx).state).enter).expect("non-null function pointer")(ictx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn input_parse(mut wp: *mut window_pane) {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    let mut itr: *const input_transition = 0 as *const input_transition;
    let mut evb: *mut evbuffer = (*(*wp).event).input;
    let mut buf: *mut u_char = 0 as *mut u_char;
    let mut len: size_t = 0;
    let mut off: size_t = 0;
    if evbuffer_get_length(evb) == 0 as libc::c_int as size_t {
        return;
    }
    window_update_activity((*wp).window);
    (*wp).flags |= 0x80 as libc::c_int;
    if ((*wp).mode).is_null() {
        screen_write_start(&mut (*ictx).ctx, wp, &mut (*wp).base);
    } else {
        screen_write_start(&mut (*ictx).ctx, 0 as *mut window_pane, &mut (*wp).base);
    }
    (*ictx).wp = wp;
    buf = evbuffer_pullup(evb, -(1 as libc::c_int) as ssize_t);
    len = evbuffer_get_length(evb);
    off = 0 as libc::c_int as size_t;
    notify_input(wp, evb);
    log_debug(
        b"%s: %%%u %s, %zu bytes: %.*s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"input_parse\0"))
            .as_ptr(),
        (*wp).id,
        (*(*ictx).state).name,
        len,
        len as libc::c_int,
        buf,
    );
    while off < len {
        let fresh0 = off;
        off = off.wrapping_add(1);
        (*ictx).ch = *buf.offset(fresh0 as isize) as libc::c_int;
        itr = (*(*ictx).state).transitions;
        while (*itr).first != -(1 as libc::c_int) && (*itr).last != -(1 as libc::c_int) {
            if (*ictx).ch >= (*itr).first && (*ictx).ch <= (*itr).last {
                break;
            }
            itr = itr.offset(1);
            itr;
        }
        if (*itr).first == -(1 as libc::c_int) || (*itr).last == -(1 as libc::c_int) {
            fatalx(b"no transition from state\0" as *const u8 as *const libc::c_char);
        }
        if (*itr).handler
            != Some(input_print as unsafe extern "C" fn(*mut input_ctx) -> libc::c_int)
        {
            screen_write_collect_end(&mut (*ictx).ctx);
        }
        if ((*itr).handler).is_some()
            && ((*itr).handler).expect("non-null function pointer")(ictx)
                != 0 as libc::c_int
        {
            continue;
        }
        if !((*itr).state).is_null() {
            input_set_state(wp, itr);
        }
        if (*ictx).state != &input_state_ground as *const input_state {
            evbuffer_add(
                (*ictx).since_ground,
                &mut (*ictx).ch as *mut libc::c_int as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    screen_write_stop(&mut (*ictx).ctx);
    evbuffer_drain(evb, len);
}
unsafe extern "C" fn input_split(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut input_param = 0 as *mut input_param;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 as libc::c_uint
            == INPUT_STRING as libc::c_int as libc::c_uint
        {
            free(
                (*ictx).param_list[i as usize].c2rust_unnamed.str_0 as *mut libc::c_void,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ictx).param_list_len = 0 as libc::c_int as u_int;
    if (*ictx).param_len == 0 as libc::c_int as size_t {
        return 0 as libc::c_int;
    }
    ip = &mut *((*ictx).param_list).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut input_param;
    ptr = ((*ictx).param_buf).as_mut_ptr() as *mut libc::c_char;
    loop {
        out = strsep(&mut ptr, b";\0" as *const u8 as *const libc::c_char);
        if out.is_null() {
            break;
        }
        if *out as libc::c_int == '\0' as i32 {
            (*ip).type_0 = INPUT_MISSING;
        } else if !(strchr(out, ':' as i32)).is_null() {
            (*ip).type_0 = INPUT_STRING;
            (*ip).c2rust_unnamed.str_0 = xstrdup(out);
        } else {
            (*ip).type_0 = INPUT_NUMBER;
            (*ip)
                .c2rust_unnamed
                .num = strtonum(
                out,
                0 as libc::c_int as libc::c_longlong,
                2147483647 as libc::c_int as libc::c_longlong,
                &mut errstr,
            ) as libc::c_int;
            if !errstr.is_null() {
                return -(1 as libc::c_int);
            }
        }
        (*ictx).param_list_len = ((*ictx).param_list_len).wrapping_add(1);
        ip = &mut *((*ictx).param_list)
            .as_mut_ptr()
            .offset((*ictx).param_list_len as isize) as *mut input_param;
        if (*ictx).param_list_len as libc::c_ulong
            == (::core::mem::size_of::<[input_param; 24]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<input_param>() as libc::c_ulong)
        {
            return -(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        ip = &mut *((*ictx).param_list).as_mut_ptr().offset(i as isize)
            as *mut input_param;
        if (*ip).type_0 as libc::c_uint == INPUT_MISSING as libc::c_int as libc::c_uint {
            log_debug(b"parameter %u: missing\0" as *const u8 as *const libc::c_char, i);
        } else if (*ip).type_0 as libc::c_uint
            == INPUT_STRING as libc::c_int as libc::c_uint
        {
            log_debug(
                b"parameter %u: string %s\0" as *const u8 as *const libc::c_char,
                i,
                (*ip).c2rust_unnamed.str_0,
            );
        } else if (*ip).type_0 as libc::c_uint
            == INPUT_NUMBER as libc::c_int as libc::c_uint
        {
            log_debug(
                b"parameter %u: number %d\0" as *const u8 as *const libc::c_char,
                i,
                (*ip).c2rust_unnamed.num,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_get(
    mut ictx: *mut input_ctx,
    mut validx: u_int,
    mut minval: libc::c_int,
    mut defval: libc::c_int,
) -> libc::c_int {
    let mut ip: *mut input_param = 0 as *mut input_param;
    let mut retval: libc::c_int = 0;
    if validx >= (*ictx).param_list_len {
        return defval;
    }
    ip = &mut *((*ictx).param_list).as_mut_ptr().offset(validx as isize)
        as *mut input_param;
    if (*ip).type_0 as libc::c_uint == INPUT_MISSING as libc::c_int as libc::c_uint {
        return defval;
    }
    if (*ip).type_0 as libc::c_uint == INPUT_STRING as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    retval = (*ip).c2rust_unnamed.num;
    if retval < minval {
        return minval;
    }
    return retval;
}
unsafe extern "C" fn input_clear(mut ictx: *mut input_ctx) {
    event_del(&mut (*ictx).timer);
    *((*ictx).interm_buf).as_mut_ptr() = '\0' as i32 as u_char;
    (*ictx).interm_len = 0 as libc::c_int as size_t;
    *((*ictx).param_buf).as_mut_ptr() = '\0' as i32 as u_char;
    (*ictx).param_len = 0 as libc::c_int as size_t;
    *(*ictx).input_buf = '\0' as i32 as u_char;
    (*ictx).input_len = 0 as libc::c_int as size_t;
    (*ictx).flags &= !(0x1 as libc::c_int);
}
unsafe extern "C" fn input_ground(mut ictx: *mut input_ctx) {
    event_del(&mut (*ictx).timer);
    evbuffer_drain((*ictx).since_ground, evbuffer_get_length((*ictx).since_ground));
    if (*ictx).input_space > 32 as libc::c_int as size_t {
        (*ictx).input_space = 32 as libc::c_int as size_t;
        (*ictx)
            .input_buf = xrealloc(
            (*ictx).input_buf as *mut libc::c_void,
            32 as libc::c_int as size_t,
        ) as *mut u_char;
    }
}
unsafe extern "C" fn input_print(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut set: libc::c_int = 0;
    (*ictx).utf8started = 0 as libc::c_int;
    set = if (*ictx).cell.set == 0 as libc::c_int {
        (*ictx).cell.g0set
    } else {
        (*ictx).cell.g1set
    };
    if set == 1 as libc::c_int {
        (*ictx)
            .cell
            .cell
            .attr = ((*ictx).cell.cell.attr as libc::c_int | 0x80 as libc::c_int)
            as u_short;
    } else {
        (*ictx)
            .cell
            .cell
            .attr = ((*ictx).cell.cell.attr as libc::c_int & !(0x80 as libc::c_int))
            as u_short;
    }
    utf8_set(&mut (*ictx).cell.cell.data, (*ictx).ch as u_char);
    screen_write_collect_add(&mut (*ictx).ctx, &mut (*ictx).cell.cell);
    (*ictx).last = (*ictx).ch;
    (*ictx)
        .cell
        .cell
        .attr = ((*ictx).cell.cell.attr as libc::c_int & !(0x80 as libc::c_int))
        as u_short;
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_intermediate(mut ictx: *mut input_ctx) -> libc::c_int {
    if (*ictx).interm_len
        == (::core::mem::size_of::<[u_char; 4]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        (*ictx).flags |= 0x1 as libc::c_int;
    } else {
        let fresh1 = (*ictx).interm_len;
        (*ictx).interm_len = ((*ictx).interm_len).wrapping_add(1);
        (*ictx).interm_buf[fresh1 as usize] = (*ictx).ch as u_char;
        (*ictx).interm_buf[(*ictx).interm_len as usize] = '\0' as i32 as u_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_parameter(mut ictx: *mut input_ctx) -> libc::c_int {
    if (*ictx).param_len
        == (::core::mem::size_of::<[u_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        (*ictx).flags |= 0x1 as libc::c_int;
    } else {
        let fresh2 = (*ictx).param_len;
        (*ictx).param_len = ((*ictx).param_len).wrapping_add(1);
        (*ictx).param_buf[fresh2 as usize] = (*ictx).ch as u_char;
        (*ictx).param_buf[(*ictx).param_len as usize] = '\0' as i32 as u_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_input(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut available: size_t = 0;
    available = (*ictx).input_space;
    while ((*ictx).input_len).wrapping_add(1 as libc::c_int as size_t) >= available {
        available = available * 2 as libc::c_int as size_t;
        if available > 1048576 as libc::c_int as size_t {
            (*ictx).flags |= 0x1 as libc::c_int;
            return 0 as libc::c_int;
        }
        (*ictx)
            .input_buf = xrealloc((*ictx).input_buf as *mut libc::c_void, available)
            as *mut u_char;
        (*ictx).input_space = available;
    }
    let fresh3 = (*ictx).input_len;
    (*ictx).input_len = ((*ictx).input_len).wrapping_add(1);
    *((*ictx).input_buf).offset(fresh3 as isize) = (*ictx).ch as u_char;
    *((*ictx).input_buf).offset((*ictx).input_len as isize) = '\0' as i32 as u_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_c0_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut s: *mut screen = (*sctx).s;
    (*ictx).utf8started = 0 as libc::c_int;
    log_debug(
        b"%s: '%c'\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"input_c0_dispatch\0"))
            .as_ptr(),
        (*ictx).ch,
    );
    let mut current_block_10: u64;
    match (*ictx).ch {
        0 => {
            current_block_10 = 7175849428784450219;
        }
        7 => {
            alerts_queue((*wp).window, 0x1 as libc::c_int);
            current_block_10 = 7175849428784450219;
        }
        8 => {
            screen_write_backspace(sctx);
            current_block_10 = 7175849428784450219;
        }
        9 => {
            if (*s).cx >= ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
                current_block_10 = 7175849428784450219;
            } else {
                loop {
                    (*s).cx = ((*s).cx).wrapping_add(1);
                    (*s).cx;
                    if *((*s).tabs).offset(((*s).cx >> 3 as libc::c_int) as isize)
                        as libc::c_int
                        & (1 as libc::c_int) << ((*s).cx & 0x7 as libc::c_int as u_int)
                        != 0
                    {
                        break;
                    }
                    if !((*s).cx
                        < ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int))
                    {
                        break;
                    }
                }
                current_block_10 = 7175849428784450219;
            }
        }
        10 => {
            current_block_10 = 2067411670301728034;
        }
        11 | 12 => {
            current_block_10 = 2067411670301728034;
        }
        13 => {
            screen_write_carriagereturn(sctx);
            current_block_10 = 7175849428784450219;
        }
        14 => {
            (*ictx).cell.set = 1 as libc::c_int;
            current_block_10 = 7175849428784450219;
        }
        15 => {
            (*ictx).cell.set = 0 as libc::c_int;
            current_block_10 = 7175849428784450219;
        }
        _ => {
            log_debug(
                b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"input_c0_dispatch\0"))
                    .as_ptr(),
                (*ictx).ch,
            );
            current_block_10 = 7175849428784450219;
        }
    }
    match current_block_10 {
        2067411670301728034 => {
            screen_write_linefeed(sctx, 0 as libc::c_int, (*ictx).cell.cell.bg as u_int);
        }
        _ => {}
    }
    (*ictx).last = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_esc_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut s: *mut screen = (*sctx).s;
    let mut entry: *mut input_table_entry = 0 as *mut input_table_entry;
    if (*ictx).flags & 0x1 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    log_debug(
        b"%s: '%c', %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"input_esc_dispatch\0"))
            .as_ptr(),
        (*ictx).ch,
        ((*ictx).interm_buf).as_mut_ptr(),
    );
    entry = bsearch(
        ictx as *const libc::c_void,
        input_esc_table.as_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[input_table_entry; 15]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<input_table_entry>() as libc::c_ulong),
        ::core::mem::size_of::<input_table_entry>() as libc::c_ulong,
        Some(
            input_table_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut input_table_entry;
    if entry.is_null() {
        log_debug(
            b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"input_esc_dispatch\0"))
                .as_ptr(),
            (*ictx).ch,
        );
        return 0 as libc::c_int;
    }
    match (*entry).type_0 {
        9 => {
            window_pane_reset_palette((*ictx).wp);
            input_reset_cell(ictx);
            screen_write_reset(sctx);
        }
        6 => {
            screen_write_linefeed(sctx, 0 as libc::c_int, (*ictx).cell.cell.bg as u_int);
        }
        7 => {
            screen_write_carriagereturn(sctx);
            screen_write_linefeed(sctx, 0 as libc::c_int, (*ictx).cell.cell.bg as u_int);
        }
        5 => {
            if (*s).cx < (*(*s).grid).sx {
                let ref mut fresh4 = *((*s).tabs)
                    .offset(((*s).cx >> 3 as libc::c_int) as isize);
                *fresh4 = (*fresh4 as libc::c_int
                    | (1 as libc::c_int) << ((*s).cx & 0x7 as libc::c_int as u_int))
                    as bitstr_t;
            }
        }
        8 => {
            screen_write_reverseindex(sctx, (*ictx).cell.cell.bg as u_int);
        }
        1 => {
            screen_write_mode_set(sctx, 0x8 as libc::c_int);
        }
        2 => {
            screen_write_mode_clear(sctx, 0x8 as libc::c_int);
        }
        4 => {
            memcpy(
                &mut (*ictx).old_cell as *mut input_cell as *mut libc::c_void,
                &mut (*ictx).cell as *mut input_cell as *const libc::c_void,
                ::core::mem::size_of::<input_cell>() as libc::c_ulong,
            );
            (*ictx).old_cx = (*s).cx;
            (*ictx).old_cy = (*s).cy;
        }
        3 => {
            memcpy(
                &mut (*ictx).cell as *mut input_cell as *mut libc::c_void,
                &mut (*ictx).old_cell as *mut input_cell as *const libc::c_void,
                ::core::mem::size_of::<input_cell>() as libc::c_ulong,
            );
            screen_write_cursormove(sctx, (*ictx).old_cx, (*ictx).old_cy);
        }
        0 => {
            screen_write_alignmenttest(sctx);
        }
        11 => {
            (*ictx).cell.g0set = 1 as libc::c_int;
        }
        10 => {
            (*ictx).cell.g0set = 0 as libc::c_int;
        }
        13 => {
            (*ictx).cell.g1set = 1 as libc::c_int;
        }
        12 => {
            (*ictx).cell.g1set = 0 as libc::c_int;
        }
        14 | _ => {}
    }
    (*ictx).last = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_csi_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut s: *mut screen = (*sctx).s;
    let mut entry: *mut input_table_entry = 0 as *mut input_table_entry;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut cx: u_int = 0;
    let mut bg: u_int = (*ictx).cell.cell.bg as u_int;
    if (*ictx).flags & 0x1 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    log_debug(
        b"%s: '%c' \"%s\" \"%s\"\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"input_csi_dispatch\0"))
            .as_ptr(),
        (*ictx).ch,
        ((*ictx).interm_buf).as_mut_ptr(),
        ((*ictx).param_buf).as_mut_ptr(),
    );
    if input_split(ictx) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    entry = bsearch(
        ictx as *const libc::c_void,
        input_csi_table.as_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[input_table_entry; 34]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<input_table_entry>() as libc::c_ulong),
        ::core::mem::size_of::<input_table_entry>() as libc::c_ulong,
        Some(
            input_table_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut input_table_entry;
    if entry.is_null() {
        log_debug(
            b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"input_csi_dispatch\0"))
                .as_ptr(),
            (*ictx).ch,
        );
        return 0 as libc::c_int;
    }
    match (*entry).type_0 {
        0 => {
            cx = (*s).cx;
            if cx > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
                cx = ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int);
            }
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if !(n == -(1 as libc::c_int)) {
                while cx > 0 as libc::c_int as u_int
                    && {
                        let fresh5 = n;
                        n = n - 1;
                        fresh5 > 0 as libc::c_int
                    }
                {
                    loop {
                        cx = cx.wrapping_sub(1);
                        cx;
                        if !(cx > 0 as libc::c_int as u_int
                            && *((*s).tabs).offset((cx >> 3 as libc::c_int) as isize)
                                as libc::c_int
                                & (1 as libc::c_int) << (cx & 0x7 as libc::c_int as u_int)
                                == 0)
                        {
                            break;
                        }
                    }
                }
                (*s).cx = cx;
            }
        }
        3 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_cursorleft(sctx, n as u_int);
            }
        }
        4 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_cursordown(sctx, n as u_int);
            }
        }
        5 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_cursorright(sctx, n as u_int);
            }
        }
        6 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            m = input_get(
                ictx,
                1 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) && m != -(1 as libc::c_int) {
                screen_write_cursormove(
                    sctx,
                    (m - 1 as libc::c_int) as u_int,
                    (n - 1 as libc::c_int) as u_int,
                );
            }
        }
        32 => {
            input_csi_dispatch_winops(ictx);
        }
        7 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_cursorup(sctx, n as u_int);
            }
        }
        1 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_carriagereturn(sctx);
                screen_write_cursordown(sctx, n as u_int);
            }
        }
        2 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_carriagereturn(sctx);
                screen_write_cursorup(sctx, n as u_int);
            }
        }
        8 => {
            match input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) {
                -1 => {}
                0 => {
                    input_reply(
                        ictx,
                        b"\x1B[?1;2c\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    log_debug(
                        b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"input_csi_dispatch\0"))
                            .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        9 => {
            match input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) {
                -1 => {}
                0 => {
                    input_reply(
                        ictx,
                        b"\x1B[>84;0;0c\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    log_debug(
                        b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"input_csi_dispatch\0"))
                            .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        15 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_clearcharacter(sctx, n as u_int, bg);
            }
        }
        10 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_deletecharacter(sctx, n as u_int, bg);
            }
        }
        12 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            m = input_get(
                ictx,
                1 as libc::c_int as u_int,
                1 as libc::c_int,
                (*(*s).grid).sy as libc::c_int,
            );
            if n != -(1 as libc::c_int) && m != -(1 as libc::c_int) {
                screen_write_scrollregion(
                    sctx,
                    (n - 1 as libc::c_int) as u_int,
                    (m - 1 as libc::c_int) as u_int,
                );
            }
        }
        13 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_deleteline(sctx, n as u_int, bg);
            }
        }
        14 => {
            match input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) {
                -1 => {}
                5 => {
                    input_reply(ictx, b"\x1B[0n\0" as *const u8 as *const libc::c_char);
                }
                6 => {
                    input_reply(
                        ictx,
                        b"\x1B[%u;%uR\0" as *const u8 as *const libc::c_char,
                        ((*s).cy).wrapping_add(1 as libc::c_int as u_int),
                        ((*s).cx).wrapping_add(1 as libc::c_int as u_int),
                    );
                }
                _ => {
                    log_debug(
                        b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"input_csi_dispatch\0"))
                            .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        16 => {
            match input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) {
                -1 => {}
                0 => {
                    screen_write_clearendofscreen(sctx, bg);
                }
                1 => {
                    screen_write_clearstartofscreen(sctx, bg);
                }
                2 => {
                    screen_write_clearscreen(sctx, bg);
                }
                3 => {
                    if input_get(
                        ictx,
                        1 as libc::c_int as u_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    ) == 0 as libc::c_int
                    {
                        screen_write_clearhistory(sctx);
                    }
                }
                _ => {
                    log_debug(
                        b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"input_csi_dispatch\0"))
                            .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        17 => {
            match input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) {
                -1 => {}
                0 => {
                    screen_write_clearendofline(sctx, bg);
                }
                1 => {
                    screen_write_clearstartofline(sctx, bg);
                }
                2 => {
                    screen_write_clearline(sctx, bg);
                }
                _ => {
                    log_debug(
                        b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"input_csi_dispatch\0"))
                            .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        18 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_cursormove(sctx, (n - 1 as libc::c_int) as u_int, (*s).cy);
            }
        }
        19 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_insertcharacter(sctx, n as u_int, bg);
            }
        }
        20 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_insertline(sctx, n as u_int, bg);
            }
        }
        22 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if !(n == -(1 as libc::c_int)) {
                if !((*ictx).last == -(1 as libc::c_int)) {
                    (*ictx).ch = (*ictx).last;
                    i = 0 as libc::c_int;
                    while i < n {
                        input_print(ictx);
                        i += 1;
                        i;
                    }
                }
            }
        }
        21 => {
            memcpy(
                &mut (*ictx).cell as *mut input_cell as *mut libc::c_void,
                &mut (*ictx).old_cell as *mut input_cell as *const libc::c_void,
                ::core::mem::size_of::<input_cell>() as libc::c_ulong,
            );
            screen_write_cursormove(sctx, (*ictx).old_cx, (*ictx).old_cy);
        }
        23 => {
            input_csi_dispatch_rm(ictx);
        }
        24 => {
            input_csi_dispatch_rm_private(ictx);
        }
        25 => {
            memcpy(
                &mut (*ictx).old_cell as *mut input_cell as *mut libc::c_void,
                &mut (*ictx).cell as *mut input_cell as *const libc::c_void,
                ::core::mem::size_of::<input_cell>() as libc::c_ulong,
            );
            (*ictx).old_cx = (*s).cx;
            (*ictx).old_cy = (*s).cy;
        }
        26 => {
            input_csi_dispatch_sgr(ictx);
        }
        27 => {
            input_csi_dispatch_sm(ictx);
        }
        28 => {
            input_csi_dispatch_sm_private(ictx);
        }
        29 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_scrollup(sctx, n as u_int, bg);
            }
        }
        30 => {
            match input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ) {
                -1 => {}
                0 => {
                    if (*s).cx < (*(*s).grid).sx {
                        let ref mut fresh6 = *((*s).tabs)
                            .offset(((*s).cx >> 3 as libc::c_int) as isize);
                        *fresh6 = (*fresh6 as libc::c_int
                            & !((1 as libc::c_int)
                                << ((*s).cx & 0x7 as libc::c_int as u_int))) as bitstr_t;
                    }
                }
                3 => {
                    let mut _name: *mut bitstr_t = (*s).tabs;
                    let mut _start: libc::c_int = 0 as libc::c_int;
                    let mut _stop: libc::c_int = ((*(*s).grid).sx)
                        .wrapping_sub(1 as libc::c_int as u_int) as libc::c_int;
                    while _start <= _stop {
                        let ref mut fresh7 = *_name
                            .offset((_start >> 3 as libc::c_int) as isize);
                        *fresh7 = (*fresh7 as libc::c_int
                            & !((1 as libc::c_int) << (_start & 0x7 as libc::c_int)))
                            as bitstr_t;
                        _start += 1;
                        _start;
                    }
                }
                _ => {
                    log_debug(
                        b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 19],
                            &[libc::c_char; 19],
                        >(b"input_csi_dispatch\0"))
                            .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        31 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_write_cursormove(sctx, (*s).cx, (n - 1 as libc::c_int) as u_int);
            }
        }
        11 => {
            n = input_get(
                ictx,
                0 as libc::c_int as u_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            if n != -(1 as libc::c_int) {
                screen_set_cursor_style(s, n as u_int);
            }
        }
        _ => {}
    }
    (*ictx).last = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_csi_dispatch_rm(mut ictx: *mut input_ctx) {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0 as libc::c_int, -(1 as libc::c_int)) {
            -1 => {}
            4 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x2 as libc::c_int);
            }
            34 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x80 as libc::c_int);
            }
            _ => {
                log_debug(
                    b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"input_csi_dispatch_rm\0"))
                        .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn input_csi_dispatch_rm_private(mut ictx: *mut input_ctx) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0 as libc::c_int, -(1 as libc::c_int)) {
            -1 => {}
            1 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x4 as libc::c_int);
            }
            3 => {
                screen_write_cursormove(
                    &mut (*ictx).ctx,
                    0 as libc::c_int as u_int,
                    0 as libc::c_int as u_int,
                );
                screen_write_clearscreen(
                    &mut (*ictx).ctx,
                    (*ictx).cell.cell.bg as u_int,
                );
            }
            7 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x10 as libc::c_int);
            }
            12 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x80 as libc::c_int);
            }
            25 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x1 as libc::c_int);
            }
            1000 | 1001 | 1002 | 1003 => {
                screen_write_mode_clear(
                    &mut (*ictx).ctx,
                    0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int,
                );
            }
            1004 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x800 as libc::c_int);
            }
            1005 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x100 as libc::c_int);
            }
            1006 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x200 as libc::c_int);
            }
            47 | 1047 => {
                window_pane_alternate_off(wp, &mut (*ictx).cell.cell, 0 as libc::c_int);
            }
            1049 => {
                window_pane_alternate_off(wp, &mut (*ictx).cell.cell, 1 as libc::c_int);
            }
            2004 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x400 as libc::c_int);
            }
            _ => {
                log_debug(
                    b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 30],
                        &[libc::c_char; 30],
                    >(b"input_csi_dispatch_rm_private\0"))
                        .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn input_csi_dispatch_sm(mut ictx: *mut input_ctx) {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0 as libc::c_int, -(1 as libc::c_int)) {
            -1 => {}
            4 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x2 as libc::c_int);
            }
            34 => {
                screen_write_mode_clear(&mut (*ictx).ctx, 0x80 as libc::c_int);
            }
            _ => {
                log_debug(
                    b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"input_csi_dispatch_sm\0"))
                        .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn input_csi_dispatch_sm_private(mut ictx: *mut input_ctx) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0 as libc::c_int, -(1 as libc::c_int)) {
            -1 => {}
            1 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x4 as libc::c_int);
            }
            3 => {
                screen_write_cursormove(
                    &mut (*ictx).ctx,
                    0 as libc::c_int as u_int,
                    0 as libc::c_int as u_int,
                );
                screen_write_clearscreen(
                    &mut (*ictx).ctx,
                    (*ictx).cell.cell.bg as u_int,
                );
            }
            7 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x10 as libc::c_int);
            }
            12 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x80 as libc::c_int);
            }
            25 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x1 as libc::c_int);
            }
            1000 => {
                screen_write_mode_clear(
                    &mut (*ictx).ctx,
                    0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int,
                );
                screen_write_mode_set(&mut (*ictx).ctx, 0x20 as libc::c_int);
            }
            1002 => {
                screen_write_mode_clear(
                    &mut (*ictx).ctx,
                    0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int,
                );
                screen_write_mode_set(&mut (*ictx).ctx, 0x40 as libc::c_int);
            }
            1003 => {
                screen_write_mode_clear(
                    &mut (*ictx).ctx,
                    0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int,
                );
                screen_write_mode_set(&mut (*ictx).ctx, 0x1000 as libc::c_int);
            }
            1004 => {
                if !((*(*ictx).ctx.s).mode & 0x800 as libc::c_int != 0) {
                    screen_write_mode_set(&mut (*ictx).ctx, 0x800 as libc::c_int);
                    (*wp).flags |= 0x20 as libc::c_int;
                }
            }
            1005 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x100 as libc::c_int);
            }
            1006 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x200 as libc::c_int);
            }
            47 | 1047 => {
                window_pane_alternate_on(wp, &mut (*ictx).cell.cell, 0 as libc::c_int);
            }
            1049 => {
                window_pane_alternate_on(wp, &mut (*ictx).cell.cell, 1 as libc::c_int);
            }
            2004 => {
                screen_write_mode_set(&mut (*ictx).ctx, 0x400 as libc::c_int);
            }
            _ => {
                log_debug(
                    b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 30],
                        &[libc::c_char; 30],
                    >(b"input_csi_dispatch_sm_private\0"))
                        .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn input_csi_dispatch_winops(mut ictx: *mut input_ctx) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    m = 0 as libc::c_int;
    loop {
        n = input_get(ictx, m as u_int, 0 as libc::c_int, -(1 as libc::c_int));
        if !(n != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_17: u64;
        match n {
            1 | 2 | 5 | 6 | 7 | 11 | 13 | 14 | 19 | 20 | 21 | 24 => {
                current_block_17 = 18386322304582297246;
            }
            3 | 4 | 8 => {
                m += 1;
                m;
                if input_get(ictx, m as u_int, 0 as libc::c_int, -(1 as libc::c_int))
                    == -(1 as libc::c_int)
                {
                    return;
                }
                current_block_17 = 1185040227185769797;
            }
            9 | 10 => {
                current_block_17 = 1185040227185769797;
            }
            22 => {
                m += 1;
                m;
                match input_get(
                    ictx,
                    m as u_int,
                    0 as libc::c_int,
                    -(1 as libc::c_int),
                ) {
                    -1 => return,
                    0 | 2 => {
                        screen_push_title((*ictx).ctx.s);
                    }
                    _ => {}
                }
                current_block_17 = 18386322304582297246;
            }
            23 => {
                m += 1;
                m;
                match input_get(
                    ictx,
                    m as u_int,
                    0 as libc::c_int,
                    -(1 as libc::c_int),
                ) {
                    -1 => return,
                    0 | 2 => {
                        screen_pop_title((*ictx).ctx.s);
                        server_status_window((*(*ictx).wp).window);
                    }
                    _ => {}
                }
                current_block_17 = 18386322304582297246;
            }
            18 => {
                input_reply(
                    ictx,
                    b"\x1B[8;%u;%ut\0" as *const u8 as *const libc::c_char,
                    (*wp).sy,
                    (*wp).sx,
                );
                current_block_17 = 18386322304582297246;
            }
            _ => {
                log_debug(
                    b"%s: unknown '%c'\0" as *const u8 as *const libc::c_char,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"input_csi_dispatch_winops\0"))
                        .as_ptr(),
                    (*ictx).ch,
                );
                current_block_17 = 18386322304582297246;
            }
        }
        match current_block_17 {
            1185040227185769797 => {
                m += 1;
                m;
                if input_get(ictx, m as u_int, 0 as libc::c_int, -(1 as libc::c_int))
                    == -(1 as libc::c_int)
                {
                    return;
                }
            }
            _ => {}
        }
        m += 1;
        m;
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr_256_do(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut gc: *mut grid_cell = &mut (*ictx).cell.cell;
    if c == -(1 as libc::c_int) || c > 255 as libc::c_int {
        if fgbg == 38 as libc::c_int {
            (*gc).fg = 8 as libc::c_int;
        } else if fgbg == 48 as libc::c_int {
            (*gc).bg = 8 as libc::c_int;
        }
    } else if fgbg == 38 as libc::c_int {
        (*gc).fg = c | 0x1000000 as libc::c_int;
    } else if fgbg == 48 as libc::c_int {
        (*gc).bg = c | 0x1000000 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn input_csi_dispatch_sgr_256(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut i: *mut u_int,
) {
    let mut c: libc::c_int = 0;
    c = input_get(
        ictx,
        (*i).wrapping_add(1 as libc::c_int as u_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
    );
    if input_csi_dispatch_sgr_256_do(ictx, fgbg, c) != 0 {
        *i = (*i).wrapping_add(1);
        *i;
    }
}
unsafe extern "C" fn input_csi_dispatch_sgr_rgb_do(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut gc: *mut grid_cell = &mut (*ictx).cell.cell;
    if r == -(1 as libc::c_int) || r > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    if g == -(1 as libc::c_int) || g > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    if b == -(1 as libc::c_int) || b > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    if fgbg == 38 as libc::c_int {
        (*gc).fg = colour_join_rgb(r as u_char, g as u_char, b as u_char);
    } else if fgbg == 48 as libc::c_int {
        (*gc).bg = colour_join_rgb(r as u_char, g as u_char, b as u_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn input_csi_dispatch_sgr_rgb(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut i: *mut u_int,
) {
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    r = input_get(
        ictx,
        (*i).wrapping_add(1 as libc::c_int as u_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
    );
    g = input_get(
        ictx,
        (*i).wrapping_add(2 as libc::c_int as u_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
    );
    b = input_get(
        ictx,
        (*i).wrapping_add(3 as libc::c_int as u_int),
        0 as libc::c_int,
        -(1 as libc::c_int),
    );
    if input_csi_dispatch_sgr_rgb_do(ictx, fgbg, r, g, b) != 0 {
        *i = (*i).wrapping_add(3 as libc::c_int as u_int);
    }
}
unsafe extern "C" fn input_csi_dispatch_sgr_colon(
    mut ictx: *mut input_ctx,
    mut i: u_int,
) {
    let mut s: *mut libc::c_char = (*ictx).param_list[i as usize].c2rust_unnamed.str_0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: [libc::c_int; 8] = [0; 8];
    let mut n: u_int = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    n = 0 as libc::c_int as u_int;
    while (n as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        p[n as usize] = -(1 as libc::c_int);
        n = n.wrapping_add(1);
        n;
    }
    n = 0 as libc::c_int as u_int;
    copy = xstrdup(s);
    ptr = copy;
    loop {
        out = strsep(&mut ptr, b":\0" as *const u8 as *const libc::c_char);
        if out.is_null() {
            break;
        }
        if *out as libc::c_int != '\0' as i32 {
            let fresh8 = n;
            n = n.wrapping_add(1);
            p[fresh8
                as usize] = strtonum(
                out,
                0 as libc::c_int as libc::c_longlong,
                2147483647 as libc::c_int as libc::c_longlong,
                &mut errstr,
            ) as libc::c_int;
            if !errstr.is_null()
                || n as libc::c_ulong
                    == (::core::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
            {
                free(copy as *mut libc::c_void);
                return;
            }
        }
        log_debug(
            b"%s: %u = %d\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"input_csi_dispatch_sgr_colon\0"))
                .as_ptr(),
            n.wrapping_sub(1 as libc::c_int as u_int),
            p[n.wrapping_sub(1 as libc::c_int as u_int) as usize],
        );
    }
    free(copy as *mut libc::c_void);
    if n == 0 as libc::c_int as u_int
        || p[0 as libc::c_int as usize] != 38 as libc::c_int
            && p[0 as libc::c_int as usize] != 48 as libc::c_int
    {
        return;
    }
    if p[1 as libc::c_int as usize] == -(1 as libc::c_int) {
        i = 2 as libc::c_int as u_int;
    } else {
        i = 1 as libc::c_int as u_int;
    }
    match p[i as usize] {
        2 => {
            if !(n < i.wrapping_add(4 as libc::c_int as u_int)) {
                input_csi_dispatch_sgr_rgb_do(
                    ictx,
                    p[0 as libc::c_int as usize],
                    p[i.wrapping_add(1 as libc::c_int as u_int) as usize],
                    p[i.wrapping_add(2 as libc::c_int as u_int) as usize],
                    p[i.wrapping_add(3 as libc::c_int as u_int) as usize],
                );
            }
        }
        5 => {
            if !(n < i.wrapping_add(2 as libc::c_int as u_int)) {
                input_csi_dispatch_sgr_256_do(
                    ictx,
                    p[0 as libc::c_int as usize],
                    p[i.wrapping_add(1 as libc::c_int as u_int) as usize],
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn input_csi_dispatch_sgr(mut ictx: *mut input_ctx) {
    let mut gc: *mut grid_cell = &mut (*ictx).cell.cell;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    if (*ictx).param_list_len == 0 as libc::c_int as u_int {
        memcpy(
            gc as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        return;
    }
    i = 0 as libc::c_int as u_int;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 as libc::c_uint
            == INPUT_STRING as libc::c_int as libc::c_uint
        {
            input_csi_dispatch_sgr_colon(ictx, i);
        } else {
            n = input_get(ictx, i, 0 as libc::c_int, 0 as libc::c_int);
            if !(n == -(1 as libc::c_int)) {
                if n == 38 as libc::c_int || n == 48 as libc::c_int {
                    i = i.wrapping_add(1);
                    i;
                    match input_get(ictx, i, 0 as libc::c_int, -(1 as libc::c_int)) {
                        2 => {
                            input_csi_dispatch_sgr_rgb(ictx, n, &mut i);
                        }
                        5 => {
                            input_csi_dispatch_sgr_256(ictx, n, &mut i);
                        }
                        _ => {}
                    }
                } else {
                    match n {
                        0 => {
                            memcpy(
                                gc as *mut libc::c_void,
                                &grid_default_cell as *const grid_cell
                                    as *const libc::c_void,
                                ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
                            );
                        }
                        1 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x1 as libc::c_int)
                                as u_short;
                        }
                        2 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x2 as libc::c_int)
                                as u_short;
                        }
                        3 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x40 as libc::c_int)
                                as u_short;
                        }
                        4 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x4 as libc::c_int)
                                as u_short;
                        }
                        5 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x8 as libc::c_int)
                                as u_short;
                        }
                        7 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x10 as libc::c_int)
                                as u_short;
                        }
                        8 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x20 as libc::c_int)
                                as u_short;
                        }
                        9 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int | 0x100 as libc::c_int)
                                as u_short;
                        }
                        22 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int
                                & !(0x1 as libc::c_int | 0x2 as libc::c_int)) as u_short;
                        }
                        23 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int & !(0x40 as libc::c_int))
                                as u_short;
                        }
                        24 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int & !(0x4 as libc::c_int))
                                as u_short;
                        }
                        25 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int & !(0x8 as libc::c_int))
                                as u_short;
                        }
                        27 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int & !(0x10 as libc::c_int))
                                as u_short;
                        }
                        28 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int & !(0x20 as libc::c_int))
                                as u_short;
                        }
                        29 => {
                            (*gc)
                                .attr = ((*gc).attr as libc::c_int
                                & !(0x100 as libc::c_int)) as u_short;
                        }
                        30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 => {
                            (*gc).fg = n - 30 as libc::c_int;
                        }
                        39 => {
                            (*gc).fg = 8 as libc::c_int;
                        }
                        40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 => {
                            (*gc).bg = n - 40 as libc::c_int;
                        }
                        49 => {
                            (*gc).bg = 8 as libc::c_int;
                        }
                        90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                            (*gc).fg = n;
                        }
                        100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 => {
                            (*gc).bg = n - 10 as libc::c_int;
                        }
                        _ => {}
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn input_enter_dcs(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_enter_dcs\0"))
            .as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1 as libc::c_int);
}
unsafe extern "C" fn input_dcs_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let prefix: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &[libc::c_char; 6],
    >(b"tmux;\0");
    let prefix_len: u_int = (::core::mem::size_of::<[libc::c_char; 6]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as u_int;
    if (*ictx).flags & 0x1 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    log_debug(
        b"%s: \"%s\"\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"input_dcs_dispatch\0"))
            .as_ptr(),
        (*ictx).input_buf,
    );
    if (*ictx).input_len >= prefix_len as size_t
        && strncmp(
            (*ictx).input_buf as *const libc::c_char,
            prefix.as_ptr(),
            prefix_len as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        screen_write_rawstring(
            &mut (*ictx).ctx,
            ((*ictx).input_buf).offset(prefix_len as isize),
            ((*ictx).input_len).wrapping_sub(prefix_len as size_t) as u_int,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_enter_osc(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_enter_osc\0"))
            .as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1 as libc::c_int);
}
unsafe extern "C" fn input_exit_osc(mut ictx: *mut input_ctx) {
    let mut p: *mut u_char = (*ictx).input_buf;
    let mut option: u_int = 0;
    if (*ictx).flags & 0x1 as libc::c_int != 0 {
        return;
    }
    if (*ictx).input_len < 1 as libc::c_int as size_t || (*p as libc::c_int) < '0' as i32
        || *p as libc::c_int > '9' as i32
    {
        return;
    }
    log_debug(
        b"%s: \"%s\"\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"input_exit_osc\0"))
            .as_ptr(),
        p,
    );
    option = 0 as libc::c_int as u_int;
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        let fresh9 = p;
        p = p.offset(1);
        option = (option * 10 as libc::c_int as u_int)
            .wrapping_add(*fresh9 as u_int)
            .wrapping_sub('0' as i32 as u_int);
    }
    if *p as libc::c_int == ';' as i32 {
        p = p.offset(1);
        p;
    }
    match option {
        0 | 2 => {
            if utf8_isvalid(p as *const libc::c_char) != 0 {
                screen_set_title((*ictx).ctx.s, p as *const libc::c_char);
                server_status_window((*(*ictx).wp).window);
            }
        }
        4 => {
            input_osc_4((*ictx).wp, p as *const libc::c_char);
        }
        10 => {
            input_osc_10((*ictx).wp, p as *const libc::c_char);
        }
        11 => {
            input_osc_11((*ictx).wp, p as *const libc::c_char);
        }
        12 => {
            if utf8_isvalid(p as *const libc::c_char) != 0
                && *p as libc::c_int != '?' as i32
            {
                screen_set_cursor_colour((*ictx).ctx.s, p as *const libc::c_char);
            }
        }
        52 => {
            input_osc_52((*ictx).wp, p as *const libc::c_char);
        }
        104 => {
            input_osc_104((*ictx).wp, p as *const libc::c_char);
        }
        112 => {
            if *p as libc::c_int == '\0' as i32 {
                screen_set_cursor_colour(
                    (*ictx).ctx.s,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        _ => {
            log_debug(
                b"%s: unknown '%u'\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"input_exit_osc\0"))
                    .as_ptr(),
                option,
            );
        }
    };
}
unsafe extern "C" fn input_enter_apc(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_enter_apc\0"))
            .as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1 as libc::c_int);
}
unsafe extern "C" fn input_exit_apc(mut ictx: *mut input_ctx) {
    if (*ictx).flags & 0x1 as libc::c_int != 0 {
        return;
    }
    log_debug(
        b"%s: \"%s\"\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"input_exit_apc\0"))
            .as_ptr(),
        (*ictx).input_buf,
    );
    if utf8_isvalid((*ictx).input_buf as *const libc::c_char) == 0 {
        return;
    }
    screen_set_title((*ictx).ctx.s, (*ictx).input_buf as *const libc::c_char);
    server_status_window((*(*ictx).wp).window);
}
unsafe extern "C" fn input_enter_rename(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"input_enter_rename\0"))
            .as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1 as libc::c_int);
}
unsafe extern "C" fn input_exit_rename(mut ictx: *mut input_ctx) {
    if (*ictx).flags & 0x1 as libc::c_int != 0 {
        return;
    }
    if options_get_number(
        (*(*(*ictx).wp).window).options,
        b"allow-rename\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return;
    }
    log_debug(
        b"%s: \"%s\"\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"input_exit_rename\0"))
            .as_ptr(),
        (*ictx).input_buf,
    );
    if utf8_isvalid((*ictx).input_buf as *const libc::c_char) == 0 {
        return;
    }
    window_set_name((*(*ictx).wp).window, (*ictx).input_buf as *const libc::c_char);
    options_set_number(
        (*(*(*ictx).wp).window).options,
        b"automatic-rename\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_longlong,
    );
    server_status_window((*(*ictx).wp).window);
}
unsafe extern "C" fn input_top_bit_set(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut ud: *mut utf8_data = &mut (*ictx).utf8data;
    (*ictx).last = -(1 as libc::c_int);
    if (*ictx).utf8started == 0 {
        if utf8_open(ud, (*ictx).ch as u_char) as libc::c_uint
            != UTF8_MORE as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        (*ictx).utf8started = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    match utf8_append(ud, (*ictx).ch as u_char) as libc::c_uint {
        0 => return 0 as libc::c_int,
        2 => {
            (*ictx).utf8started = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
        1 | _ => {}
    }
    (*ictx).utf8started = 0 as libc::c_int;
    log_debug(
        b"%s %hhu '%*s' (width %hhu)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"input_top_bit_set\0"))
            .as_ptr(),
        (*ud).size as libc::c_int,
        (*ud).size as libc::c_int,
        ((*ud).data).as_mut_ptr(),
        (*ud).width as libc::c_int,
    );
    utf8_copy(&mut (*ictx).cell.cell.data, ud);
    screen_write_collect_add(&mut (*ictx).ctx, &mut (*ictx).cell.cell);
    return 0 as libc::c_int;
}
unsafe extern "C" fn input_osc_4(mut wp: *mut window_pane, mut p: *const libc::c_char) {
    let mut current_block: u64;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_long = 0;
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    s = xstrdup(p);
    copy = s;
    loop {
        if !(!s.is_null() && *s as libc::c_int != '\0' as i32) {
            current_block = 1394248824506584008;
            break;
        }
        idx = strtol(s, &mut next, 10 as libc::c_int);
        let fresh10 = next;
        next = next.offset(1);
        if *fresh10 as libc::c_int != ';' as i32 {
            current_block = 14563345840316904204;
            break;
        }
        if idx < 0 as libc::c_int as libc::c_long
            || idx >= 0x100 as libc::c_int as libc::c_long
        {
            current_block = 14563345840316904204;
            break;
        }
        s = strsep(&mut next, b";\0" as *const u8 as *const libc::c_char);
        if sscanf(
            s,
            b"rgb:%2x/%2x/%2x\0" as *const u8 as *const libc::c_char,
            &mut r as *mut u_int,
            &mut g as *mut u_int,
            &mut b as *mut u_int,
        ) != 3 as libc::c_int
        {
            s = next;
        } else {
            window_pane_set_palette(
                wp,
                idx as u_int,
                colour_join_rgb(r as u_char, g as u_char, b as u_char),
            );
            s = next;
        }
    }
    match current_block {
        14563345840316904204 => {
            log_debug(b"bad OSC 4: %s\0" as *const u8 as *const libc::c_char, p);
            free(copy as *mut libc::c_void);
            return;
        }
        _ => {
            free(copy as *mut libc::c_void);
            return;
        }
    };
}
unsafe extern "C" fn input_osc_10(mut wp: *mut window_pane, mut p: *const libc::c_char) {
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if sscanf(
        p,
        b"rgb:%2x/%2x/%2x\0" as *const u8 as *const libc::c_char,
        &mut r as *mut u_int,
        &mut g as *mut u_int,
        &mut b as *mut u_int,
    ) != 3 as libc::c_int
    {
        log_debug(b"bad OSC 10: %s\0" as *const u8 as *const libc::c_char, p);
        return;
    } else {
        (*wp).colgc.fg = colour_join_rgb(r as u_char, g as u_char, b as u_char);
        (*wp).flags |= 0x1 as libc::c_int;
        return;
    };
}
unsafe extern "C" fn input_osc_11(mut wp: *mut window_pane, mut p: *const libc::c_char) {
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if sscanf(
        p,
        b"rgb:%2x/%2x/%2x\0" as *const u8 as *const libc::c_char,
        &mut r as *mut u_int,
        &mut g as *mut u_int,
        &mut b as *mut u_int,
    ) != 3 as libc::c_int
    {
        log_debug(b"bad OSC 11: %s\0" as *const u8 as *const libc::c_char, p);
        return;
    } else {
        (*wp).colgc.bg = colour_join_rgb(r as u_char, g as u_char, b as u_char);
        (*wp).flags |= 0x1 as libc::c_int;
        return;
    };
}
unsafe extern "C" fn input_osc_52(mut wp: *mut window_pane, mut p: *const libc::c_char) {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut out: *mut u_char = 0 as *mut u_char;
    let mut outlen: libc::c_int = 0;
    let mut state: libc::c_int = 0;
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
    state = options_get_number(
        global_options,
        b"set-clipboard\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if state != 2 as libc::c_int {
        return;
    }
    end = strchr(p, ';' as i32);
    if end.is_null() {
        return;
    }
    end = end.offset(1);
    end;
    if *end as libc::c_int == '\0' as i32 {
        return;
    }
    len = (strlen(end))
        .wrapping_div(4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(3 as libc::c_int as libc::c_ulong);
    if len == 0 as libc::c_int as size_t {
        return;
    }
    out = xmalloc(len) as *mut u_char;
    outlen = __b64_pton(end, out, len);
    if outlen == -(1 as libc::c_int) {
        free(out as *mut libc::c_void);
        return;
    }
    screen_write_start(&mut ctx, wp, 0 as *mut screen);
    screen_write_setselection(&mut ctx, out, outlen as u_int);
    screen_write_stop(&mut ctx);
    notify_pane(b"pane-set-clipboard\0" as *const u8 as *const libc::c_char, wp);
    paste_add(out as *mut libc::c_char, outlen as size_t);
}
unsafe extern "C" fn input_osc_104(
    mut wp: *mut window_pane,
    mut p: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_long = 0;
    if *p as libc::c_int == '\0' as i32 {
        window_pane_reset_palette(wp);
        return;
    }
    s = xstrdup(p);
    copy = s;
    loop {
        if !(*s as libc::c_int != '\0' as i32) {
            current_block = 1917311967535052937;
            break;
        }
        idx = strtol(s, &mut s, 10 as libc::c_int);
        if *s as libc::c_int != '\0' as i32 && *s as libc::c_int != ';' as i32 {
            current_block = 5056682364410342875;
            break;
        }
        if idx < 0 as libc::c_int as libc::c_long
            || idx >= 0x100 as libc::c_int as libc::c_long
        {
            current_block = 5056682364410342875;
            break;
        }
        window_pane_unset_palette(wp, idx as u_int);
        if *s as libc::c_int == ';' as i32 {
            s = s.offset(1);
            s;
        }
    }
    match current_block {
        5056682364410342875 => {
            log_debug(b"bad OSC 104: %s\0" as *const u8 as *const libc::c_char, p);
            free(copy as *mut libc::c_void);
            return;
        }
        _ => {
            free(copy as *mut libc::c_void);
            return;
        }
    };
}
