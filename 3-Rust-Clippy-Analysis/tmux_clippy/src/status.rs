use ::libc;
extern "C" {
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __errno_location() -> *mut libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_initialized(ev: *const event) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn fgetln(_: *mut FILE, _: *mut size_t) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    fn find_home() -> *const libc::c_char;
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t) -> *const libc::c_char;
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut paste_buffer;
    fn format_create(
        _: *mut client,
        _: *mut cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut format_tree;
    fn format_free(_: *mut format_tree);
    fn format_expand_time(
        _: *mut format_tree,
        _: *const libc::c_char,
        _: time_t,
    ) -> *mut libc::c_char;
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
    static options_table: [options_table_entry; 0];
    static mut cmd_table: [*const cmd_entry; 0];
    static mut clients: clients;
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
    fn screen_write_strlen(_: *const libc::c_char, _: ...) -> size_t;
    fn screen_write_nputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    );
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const grid_cell, _: u_char);
    fn screen_write_fast_copy(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_cell(_: *mut screen_write_ctx, _: *const grid_cell);
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn screen_reinit(_: *mut screen);
    fn screen_free(_: *mut screen);
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    static mut sessions: sessions;
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    fn session_find(_: *const libc::c_char) -> *mut session;
    fn utf8_set(_: *mut utf8_data, _: u_char);
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    fn utf8_split(_: wchar_t, _: *mut utf8_data) -> utf8_state;
    fn utf8_strlen(_: *const utf8_data) -> size_t;
    fn utf8_strwidth(_: *const utf8_data, _: ssize_t) -> u_int;
    fn utf8_fromcstr(_: *const libc::c_char) -> *mut utf8_data;
    fn utf8_tocstr(_: *mut utf8_data) -> *mut libc::c_char;
    fn log_debug(_: *const libc::c_char, _: ...);
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char);
    fn style_apply_update(_: *mut grid_cell, _: *mut options, _: *const libc::c_char);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type wchar_t = libc::c_int;
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
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type C2RustUnnamed_34 = libc::c_uint;
pub const KEYC_KP_PERIOD: C2RustUnnamed_34 = 268435564;
pub const KEYC_KP_ZERO: C2RustUnnamed_34 = 268435563;
pub const KEYC_KP_ENTER: C2RustUnnamed_34 = 268435562;
pub const KEYC_KP_THREE: C2RustUnnamed_34 = 268435561;
pub const KEYC_KP_TWO: C2RustUnnamed_34 = 268435560;
pub const KEYC_KP_ONE: C2RustUnnamed_34 = 268435559;
pub const KEYC_KP_SIX: C2RustUnnamed_34 = 268435558;
pub const KEYC_KP_FIVE: C2RustUnnamed_34 = 268435557;
pub const KEYC_KP_FOUR: C2RustUnnamed_34 = 268435556;
pub const KEYC_KP_PLUS: C2RustUnnamed_34 = 268435555;
pub const KEYC_KP_NINE: C2RustUnnamed_34 = 268435554;
pub const KEYC_KP_EIGHT: C2RustUnnamed_34 = 268435553;
pub const KEYC_KP_SEVEN: C2RustUnnamed_34 = 268435552;
pub const KEYC_KP_MINUS: C2RustUnnamed_34 = 268435551;
pub const KEYC_KP_STAR: C2RustUnnamed_34 = 268435550;
pub const KEYC_KP_SLASH: C2RustUnnamed_34 = 268435549;
pub const KEYC_RIGHT: C2RustUnnamed_34 = 268435548;
pub const KEYC_LEFT: C2RustUnnamed_34 = 268435547;
pub const KEYC_DOWN: C2RustUnnamed_34 = 268435546;
pub const KEYC_UP: C2RustUnnamed_34 = 268435545;
pub const KEYC_BTAB: C2RustUnnamed_34 = 268435544;
pub const KEYC_PPAGE: C2RustUnnamed_34 = 268435543;
pub const KEYC_NPAGE: C2RustUnnamed_34 = 268435542;
pub const KEYC_END: C2RustUnnamed_34 = 268435541;
pub const KEYC_HOME: C2RustUnnamed_34 = 268435540;
pub const KEYC_DC: C2RustUnnamed_34 = 268435539;
pub const KEYC_IC: C2RustUnnamed_34 = 268435538;
pub const KEYC_F12: C2RustUnnamed_34 = 268435537;
pub const KEYC_F11: C2RustUnnamed_34 = 268435536;
pub const KEYC_F10: C2RustUnnamed_34 = 268435535;
pub const KEYC_F9: C2RustUnnamed_34 = 268435534;
pub const KEYC_F8: C2RustUnnamed_34 = 268435533;
pub const KEYC_F7: C2RustUnnamed_34 = 268435532;
pub const KEYC_F6: C2RustUnnamed_34 = 268435531;
pub const KEYC_F5: C2RustUnnamed_34 = 268435530;
pub const KEYC_F4: C2RustUnnamed_34 = 268435529;
pub const KEYC_F3: C2RustUnnamed_34 = 268435528;
pub const KEYC_F2: C2RustUnnamed_34 = 268435527;
pub const KEYC_F1: C2RustUnnamed_34 = 268435526;
pub const KEYC_BSPACE: C2RustUnnamed_34 = 268435525;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed_34 = 268435524;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed_34 = 268435523;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed_34 = 268435522;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed_34 = 268435521;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed_34 = 268435520;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed_34 = 268435519;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed_34 = 268435518;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed_34 = 268435517;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed_34 = 268435516;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed_34 = 268435515;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed_34 = 268435514;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed_34 = 268435513;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed_34 = 268435512;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed_34 = 268435511;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed_34 = 268435510;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed_34 = 268435509;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed_34 = 268435508;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed_34 = 268435507;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed_34 = 268435506;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed_34 = 268435505;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed_34 = 268435504;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed_34 = 268435503;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed_34 = 268435502;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed_34 = 268435501;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed_34 = 268435500;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed_34 = 268435499;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed_34 = 268435498;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed_34 = 268435497;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed_34 = 268435496;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed_34 = 268435495;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed_34 = 268435494;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed_34 = 268435493;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed_34 = 268435492;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed_34 = 268435491;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed_34 = 268435490;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed_34 = 268435489;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed_34 = 268435488;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed_34 = 268435487;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed_34 = 268435486;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed_34 = 268435485;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed_34 = 268435484;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed_34 = 268435483;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed_34 = 268435482;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed_34 = 268435481;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed_34 = 268435480;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed_34 = 268435479;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed_34 = 268435478;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed_34 = 268435477;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed_34 = 268435476;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed_34 = 268435475;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed_34 = 268435474;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed_34 = 268435473;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed_34 = 268435472;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed_34 = 268435471;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed_34 = 268435470;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed_34 = 268435469;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed_34 = 268435468;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed_34 = 268435467;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed_34 = 268435466;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed_34 = 268435465;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed_34 = 268435464;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed_34 = 268435463;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed_34 = 268435462;
pub const KEYC_DRAGGING: C2RustUnnamed_34 = 268435461;
pub const KEYC_MOUSE: C2RustUnnamed_34 = 268435460;
pub const KEYC_PASTE_END: C2RustUnnamed_34 = 268435459;
pub const KEYC_PASTE_START: C2RustUnnamed_34 = 268435458;
pub const KEYC_FOCUS_OUT: C2RustUnnamed_34 = 268435457;
pub const KEYC_FOCUS_IN: C2RustUnnamed_34 = 268435456;
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
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
pub struct sessions {
    pub rbh_root: *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
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
pub type options_table_scope = libc::c_uint;
pub const OPTIONS_TABLE_WINDOW: options_table_scope = 3;
pub const OPTIONS_TABLE_SESSION: options_table_scope = 2;
pub const OPTIONS_TABLE_SERVER: options_table_scope = 1;
pub const OPTIONS_TABLE_NONE: options_table_scope = 0;
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
static mut status_prompt_hlist: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut status_prompt_hsize: u_int = 0;
unsafe extern "C" fn status_prompt_find_history_file() -> *mut libc::c_char {
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut history_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    history_file = options_get_string(
        global_options,
        b"history-file\0" as *const u8 as *const libc::c_char,
    );
    if *history_file as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    if *history_file as libc::c_int == '/' as i32 {
        return xstrdup(history_file);
    }
    if *history_file.offset(0 as libc::c_int as isize) as libc::c_int != '~' as i32
        || *history_file.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
    {
        return 0 as *mut libc::c_char;
    }
    home = find_home();
    if home.is_null() {
        return 0 as *mut libc::c_char;
    }
    xasprintf(
        &mut path as *mut *mut libc::c_char,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        home,
        history_file.offset(1 as libc::c_int as isize),
    );
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_load_history() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut history_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    history_file = status_prompt_find_history_file();
    if history_file.is_null() {
        return;
    }
    log_debug(
        b"loading history from %s\0" as *const u8 as *const libc::c_char,
        history_file,
    );
    f = fopen(history_file, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        log_debug(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            history_file,
            strerror(*__errno_location()),
        );
        free(history_file as *mut libc::c_void);
        return;
    }
    free(history_file as *mut libc::c_void);
    loop {
        line = fgetln(f, &mut length);
        if line.is_null() {
            break;
        }
        if length > 0 as libc::c_int as size_t {
            if *line.offset(length.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_int == '\n' as i32
            {
                *line
                    .offset(
                        length.wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) = '\0' as i32 as libc::c_char;
                status_prompt_add_history(line);
            } else {
                tmp = xmalloc(length.wrapping_add(1 as libc::c_int as size_t))
                    as *mut libc::c_char;
                memcpy(tmp as *mut libc::c_void, line as *const libc::c_void, length);
                *tmp.offset(length as isize) = '\0' as i32 as libc::c_char;
                status_prompt_add_history(tmp);
                free(tmp as *mut libc::c_void);
            }
        }
    }
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_save_history() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: u_int = 0;
    let mut history_file: *mut libc::c_char = 0 as *mut libc::c_char;
    history_file = status_prompt_find_history_file();
    if history_file.is_null() {
        return;
    }
    log_debug(
        b"saving history to %s\0" as *const u8 as *const libc::c_char,
        history_file,
    );
    f = fopen(history_file, b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        log_debug(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            history_file,
            strerror(*__errno_location()),
        );
        free(history_file as *mut libc::c_void);
        return;
    }
    free(history_file as *mut libc::c_void);
    i = 0 as libc::c_int as u_int;
    while i < status_prompt_hsize {
        fputs(*status_prompt_hlist.offset(i as isize), f);
        fputc('\n' as i32, f);
        i = i.wrapping_add(1);
        i;
    }
    fclose(f);
}
unsafe extern "C" fn status_timer_callback(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    let mut s: *mut session = (*c).session;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    event_del(&mut (*c).status.timer);
    if s.is_null() {
        return;
    }
    if ((*c).message_string).is_null() && ((*c).prompt_string).is_null() {
        (*c).flags |= 0x10 as libc::c_int;
    }
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = tv.tv_usec;
    tv
        .tv_sec = options_get_number(
        (*s).options,
        b"status-interval\0" as *const u8 as *const libc::c_char,
    ) as __time_t;
    if tv.tv_sec != 0 as libc::c_int as __time_t {
        event_add(&mut (*c).status.timer, &mut tv);
    }
    log_debug(
        b"client %p, status interval %d\0" as *const u8 as *const libc::c_char,
        c,
        tv.tv_sec as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn status_timer_start(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    if event_initialized(&mut (*c).status.timer) != 0 {
        event_del(&mut (*c).status.timer);
    } else {
        event_set(
            &mut (*c).status.timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                status_timer_callback
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
    }
    if !s.is_null()
        && options_get_number(
            (*s).options,
            b"status\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        status_timer_callback(
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            c as *mut libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn status_timer_start_all() {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        status_timer_start(c);
        c = (*c).entry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn status_update_saved(mut s: *mut session) {
    if options_get_number((*s).options, b"status\0" as *const u8 as *const libc::c_char)
        == 0
    {
        (*s).statusat = -(1 as libc::c_int);
    } else if options_get_number(
        (*s).options,
        b"status-position\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int as libc::c_longlong
    {
        (*s).statusat = 0 as libc::c_int;
    } else {
        (*s).statusat = 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn status_at_line(mut c: *mut client) -> libc::c_int {
    let mut s: *mut session = (*c).session;
    if (*c).flags & 0x800000 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    if (*s).statusat != 1 as libc::c_int {
        return (*s).statusat;
    }
    return ((*c).tty.sy).wrapping_sub(status_line_size(s)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn status_line_size(mut s: *mut session) -> u_int {
    if (*s).statusat == -(1 as libc::c_int) {
        return 0 as libc::c_int as u_int;
    }
    return 1 as libc::c_int as u_int;
}
unsafe extern "C" fn status_redraw_get_left(
    mut c: *mut client,
    mut t: time_t,
    mut gc: *mut grid_cell,
    mut size: *mut size_t,
) -> *mut libc::c_char {
    let mut s: *mut session = (*c).session;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut leftlen: size_t = 0;
    style_apply_update(
        gc,
        (*s).options,
        b"status-left-style\0" as *const u8 as *const libc::c_char,
    );
    template = options_get_string(
        (*s).options,
        b"status-left\0" as *const u8 as *const libc::c_char,
    );
    left = status_replace(c, 0 as *mut winlink, template, t);
    *size = options_get_number(
        (*s).options,
        b"status-left-length\0" as *const u8 as *const libc::c_char,
    ) as size_t;
    leftlen = screen_write_cstrlen(b"%s\0" as *const u8 as *const libc::c_char, left);
    if leftlen < *size {
        *size = leftlen;
    }
    return left;
}
unsafe extern "C" fn status_redraw_get_right(
    mut c: *mut client,
    mut t: time_t,
    mut gc: *mut grid_cell,
    mut size: *mut size_t,
) -> *mut libc::c_char {
    let mut s: *mut session = (*c).session;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rightlen: size_t = 0;
    style_apply_update(
        gc,
        (*s).options,
        b"status-right-style\0" as *const u8 as *const libc::c_char,
    );
    template = options_get_string(
        (*s).options,
        b"status-right\0" as *const u8 as *const libc::c_char,
    );
    right = status_replace(c, 0 as *mut winlink, template, t);
    *size = options_get_number(
        (*s).options,
        b"status-right-length\0" as *const u8 as *const libc::c_char,
    ) as size_t;
    rightlen = screen_write_cstrlen(b"%s\0" as *const u8 as *const libc::c_char, right);
    if rightlen < *size {
        *size = rightlen;
    }
    return right;
}
#[no_mangle]
pub unsafe extern "C" fn status_get_window_at(
    mut c: *mut client,
    mut x: u_int,
) -> *mut window {
    let mut s: *mut session = (*c).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut oo: *mut options = 0 as *mut options;
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut seplen: size_t = 0;
    x = x.wrapping_add((*c).wlmouse as u_int);
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        oo = (*(*wl).window).options;
        sep = options_get_string(
            oo,
            b"window-status-separator\0" as *const u8 as *const libc::c_char,
        );
        seplen = screen_write_cstrlen(b"%s\0" as *const u8 as *const libc::c_char, sep);
        if (x as size_t) < (*wl).status_width {
            return (*wl).window;
        }
        x = (x as size_t).wrapping_sub(((*wl).status_width).wrapping_add(seplen))
            as u_int as u_int;
        wl = winlinks_RB_NEXT(wl);
    }
    return 0 as *mut window;
}
#[no_mangle]
pub unsafe extern "C" fn status_redraw(mut c: *mut client) -> libc::c_int {
    let mut current_block: u64;
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
    let mut s: *mut session = (*c).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut old_status: screen = screen {
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
    let mut window_list: screen = screen {
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
    let mut stdgc: grid_cell = grid_cell {
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
    let mut lgc: grid_cell = grid_cell {
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
    let mut rgc: grid_cell = grid_cell {
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
    let mut oo: *mut options = 0 as *mut options;
    let mut t: time_t = 0;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut offset: u_int = 0;
    let mut needed: u_int = 0;
    let mut lines: u_int = 0;
    let mut wlstart: u_int = 0;
    let mut wlwidth: u_int = 0;
    let mut wlavailable: u_int = 0;
    let mut wloffset: u_int = 0;
    let mut wlsize: u_int = 0;
    let mut llen: size_t = 0;
    let mut rlen: size_t = 0;
    let mut seplen: size_t = 0;
    let mut larrow: libc::c_int = 0;
    let mut rarrow: libc::c_int = 0;
    if !((*c).status.old_status).is_null() {
        screen_free((*c).status.old_status);
        free((*c).status.old_status as *mut libc::c_void);
        (*c).status.old_status = 0 as *mut screen;
    }
    lines = status_line_size(s);
    if (*c).tty.sy == 0 as libc::c_int as u_int || lines == 0 as libc::c_int as u_int {
        return 1 as libc::c_int;
    }
    right = 0 as *mut libc::c_char;
    left = right;
    rarrow = 0 as libc::c_int;
    larrow = rarrow;
    t = time(0 as *mut time_t);
    style_apply(
        &mut stdgc,
        (*s).options,
        b"status-style\0" as *const u8 as *const libc::c_char,
    );
    memcpy(
        &mut old_status as *mut screen as *mut libc::c_void,
        &mut (*c).status.status as *mut screen as *const libc::c_void,
        ::core::mem::size_of::<screen>() as libc::c_ulong,
    );
    screen_init(&mut (*c).status.status, (*c).tty.sx, lines, 0 as libc::c_int as u_int);
    screen_write_start(&mut ctx, 0 as *mut window_pane, &mut (*c).status.status);
    offset = 0 as libc::c_int as u_int;
    while offset < lines * (*c).tty.sx {
        screen_write_putc(&mut ctx, &mut stdgc, ' ' as i32 as u_char);
        offset = offset.wrapping_add(1);
        offset;
    }
    screen_write_stop(&mut ctx);
    if !((*c).tty.sy < lines) {
        memcpy(
            &mut lgc as *mut grid_cell as *mut libc::c_void,
            &mut stdgc as *mut grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        left = status_redraw_get_left(c, t, &mut lgc, &mut llen);
        memcpy(
            &mut rgc as *mut grid_cell as *mut libc::c_void,
            &mut stdgc as *mut grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        right = status_redraw_get_right(c, t, &mut rgc, &mut rlen);
        needed = 0 as libc::c_int as u_int;
        if llen != 0 as libc::c_int as size_t {
            needed = (needed as size_t).wrapping_add(llen) as u_int as u_int;
        }
        if rlen != 0 as libc::c_int as size_t {
            needed = (needed as size_t).wrapping_add(rlen) as u_int as u_int;
        }
        if !((*c).tty.sx == 0 as libc::c_int as u_int || (*c).tty.sx <= needed) {
            wlavailable = ((*c).tty.sx).wrapping_sub(needed);
            wlwidth = 0 as libc::c_int as u_int;
            wloffset = wlwidth;
            wlstart = wloffset;
            wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
            while !wl.is_null() {
                free((*wl).status_text as *mut libc::c_void);
                memcpy(
                    &mut (*wl).status_cell as *mut grid_cell as *mut libc::c_void,
                    &mut stdgc as *mut grid_cell as *const libc::c_void,
                    ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
                );
                (*wl).status_text = status_print(c, wl, t, &mut (*wl).status_cell);
                (*wl)
                    .status_width = screen_write_cstrlen(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*wl).status_text,
                );
                if wl == (*s).curw {
                    wloffset = wlwidth;
                }
                oo = (*(*wl).window).options;
                sep = options_get_string(
                    oo,
                    b"window-status-separator\0" as *const u8 as *const libc::c_char,
                );
                seplen = screen_write_cstrlen(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    sep,
                );
                wlwidth = (wlwidth as size_t)
                    .wrapping_add(((*wl).status_width).wrapping_add(seplen)) as u_int
                    as u_int;
                wl = winlinks_RB_NEXT(wl);
            }
            screen_init(
                &mut window_list,
                wlwidth,
                1 as libc::c_int as u_int,
                0 as libc::c_int as u_int,
            );
            screen_write_start(&mut ctx, 0 as *mut window_pane, &mut window_list);
            wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
            while !wl.is_null() {
                screen_write_cnputs(
                    &mut ctx as *mut screen_write_ctx,
                    -(1 as libc::c_int) as ssize_t,
                    &mut (*wl).status_cell as *mut grid_cell,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*wl).status_text,
                );
                oo = (*(*wl).window).options;
                sep = options_get_string(
                    oo,
                    b"window-status-separator\0" as *const u8 as *const libc::c_char,
                );
                screen_write_cnputs(
                    &mut ctx as *mut screen_write_ctx,
                    -(1 as libc::c_int) as ssize_t,
                    &mut stdgc as *mut grid_cell,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    sep,
                );
                wl = winlinks_RB_NEXT(wl);
            }
            screen_write_stop(&mut ctx);
            if wlwidth <= wlavailable {
                current_block = 10457940734610349807;
            } else {
                wlsize = (*(*s).curw).status_width as u_int;
                if wloffset.wrapping_add(wlsize) < wlavailable {
                    if wlavailable > 0 as libc::c_int as u_int {
                        rarrow = 1 as libc::c_int;
                        wlavailable = wlavailable.wrapping_sub(1);
                        wlavailable;
                    }
                    wlwidth = wlavailable;
                } else {
                    if wlavailable > 0 as libc::c_int as u_int {
                        larrow = 1 as libc::c_int;
                        wlavailable = wlavailable.wrapping_sub(1);
                        wlavailable;
                    }
                    wlstart = wloffset.wrapping_add(wlsize).wrapping_sub(wlavailable);
                    if wlavailable > 0 as libc::c_int as u_int
                        && wlwidth
                            > wlstart
                                .wrapping_add(wlavailable)
                                .wrapping_add(1 as libc::c_int as u_int)
                    {
                        rarrow = 1 as libc::c_int;
                        wlstart = wlstart.wrapping_add(1);
                        wlstart;
                        wlavailable = wlavailable.wrapping_sub(1);
                        wlavailable;
                    }
                    wlwidth = wlavailable;
                }
                if wlwidth == 0 as libc::c_int as u_int
                    || wlavailable == 0 as libc::c_int as u_int
                {
                    screen_free(&mut window_list);
                    current_block = 10442084391391389971;
                } else {
                    offset = 0 as libc::c_int as u_int;
                    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
                    while !wl.is_null() {
                        if (*wl).flags
                            & (0x1 as libc::c_int | 0x2 as libc::c_int
                                | 0x4 as libc::c_int) != 0 && larrow == 1 as libc::c_int
                            && offset < wlstart
                        {
                            larrow = -(1 as libc::c_int);
                        }
                        offset = (offset as size_t).wrapping_add((*wl).status_width)
                            as u_int as u_int;
                        if (*wl).flags
                            & (0x1 as libc::c_int | 0x2 as libc::c_int
                                | 0x4 as libc::c_int) != 0 && rarrow == 1 as libc::c_int
                            && offset > wlstart.wrapping_add(wlwidth)
                        {
                            rarrow = -(1 as libc::c_int);
                        }
                        wl = winlinks_RB_NEXT(wl);
                    }
                    current_block = 10457940734610349807;
                }
            }
            match current_block {
                10442084391391389971 => {}
                _ => {
                    screen_write_start(
                        &mut ctx,
                        0 as *mut window_pane,
                        &mut (*c).status.status,
                    );
                    screen_write_cursormove(
                        &mut ctx,
                        0 as libc::c_int as u_int,
                        0 as libc::c_int as u_int,
                    );
                    if llen != 0 as libc::c_int as size_t {
                        screen_write_cnputs(
                            &mut ctx as *mut screen_write_ctx,
                            llen as ssize_t,
                            &mut lgc as *mut grid_cell,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            left,
                        );
                    }
                    if larrow != 0 as libc::c_int {
                        memcpy(
                            &mut gc as *mut grid_cell as *mut libc::c_void,
                            &mut stdgc as *mut grid_cell as *const libc::c_void,
                            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
                        );
                        if larrow == -(1 as libc::c_int) {
                            gc
                                .attr = (gc.attr as libc::c_int ^ 0x10 as libc::c_int)
                                as u_short;
                        }
                        screen_write_putc(&mut ctx, &mut gc, '<' as i32 as u_char);
                    }
                    if rarrow != 0 as libc::c_int {
                        screen_write_cursormove(
                            &mut ctx,
                            ((*c).tty.sx as size_t)
                                .wrapping_sub(rlen)
                                .wrapping_sub(1 as libc::c_int as size_t) as u_int,
                            0 as libc::c_int as u_int,
                        );
                        memcpy(
                            &mut gc as *mut grid_cell as *mut libc::c_void,
                            &mut stdgc as *mut grid_cell as *const libc::c_void,
                            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
                        );
                        if rarrow == -(1 as libc::c_int) {
                            gc
                                .attr = (gc.attr as libc::c_int ^ 0x10 as libc::c_int)
                                as u_short;
                        }
                        screen_write_putc(&mut ctx, &mut gc, '>' as i32 as u_char);
                    } else {
                        screen_write_cursormove(
                            &mut ctx,
                            ((*c).tty.sx as size_t).wrapping_sub(rlen) as u_int,
                            0 as libc::c_int as u_int,
                        );
                    }
                    if rlen != 0 as libc::c_int as size_t {
                        screen_write_cnputs(
                            &mut ctx as *mut screen_write_ctx,
                            rlen as ssize_t,
                            &mut rgc as *mut grid_cell,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            right,
                        );
                    }
                    if llen != 0 as libc::c_int as size_t {
                        wloffset = llen as u_int;
                    } else {
                        wloffset = 0 as libc::c_int as u_int;
                    }
                    if wlwidth < wlavailable {
                        match options_get_number(
                            (*s).options,
                            b"status-justify\0" as *const u8 as *const libc::c_char,
                        ) {
                            1 => {
                                wloffset = wloffset
                                    .wrapping_add(
                                        wlavailable.wrapping_sub(wlwidth)
                                            / 2 as libc::c_int as u_int,
                                    );
                            }
                            2 => {
                                wloffset = wloffset
                                    .wrapping_add(wlavailable.wrapping_sub(wlwidth));
                            }
                            _ => {}
                        }
                    }
                    if larrow != 0 as libc::c_int {
                        wloffset = wloffset.wrapping_add(1);
                        wloffset;
                    }
                    (*c)
                        .wlmouse = wloffset.wrapping_neg().wrapping_add(wlstart)
                        as libc::c_int;
                    screen_write_cursormove(
                        &mut ctx,
                        wloffset,
                        0 as libc::c_int as u_int,
                    );
                    screen_write_fast_copy(
                        &mut ctx,
                        &mut window_list,
                        wlstart,
                        0 as libc::c_int as u_int,
                        wlwidth,
                        1 as libc::c_int as u_int,
                    );
                    screen_free(&mut window_list);
                    screen_write_stop(&mut ctx);
                }
            }
        }
    }
    free(left as *mut libc::c_void);
    free(right as *mut libc::c_void);
    if grid_compare((*c).status.status.grid, old_status.grid) == 0 as libc::c_int {
        screen_free(&mut old_status);
        return 0 as libc::c_int;
    }
    screen_free(&mut old_status);
    return 1 as libc::c_int;
}
unsafe extern "C" fn status_replace(
    mut c: *mut client,
    mut wl: *mut winlink,
    mut fmt: *const libc::c_char,
    mut t: time_t,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag: u_int = 0;
    if fmt.is_null() {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    if !wl.is_null() {
        tag = 0x40000000 as libc::c_uint | (*(*wl).window).id;
    } else {
        tag = 0 as libc::c_int as u_int;
    }
    if (*c).flags & 0x80000 as libc::c_int != 0 {
        ft = format_create(
            c,
            0 as *mut cmdq_item,
            tag as libc::c_int,
            0x1 as libc::c_int | 0x2 as libc::c_int,
        );
    } else {
        ft = format_create(
            c,
            0 as *mut cmdq_item,
            tag as libc::c_int,
            0x1 as libc::c_int,
        );
    }
    format_defaults(ft, c, 0 as *mut session, wl, 0 as *mut window_pane);
    expanded = format_expand_time(ft, fmt, t);
    format_free(ft);
    return expanded;
}
unsafe extern "C" fn status_print(
    mut c: *mut client,
    mut wl: *mut winlink,
    mut t: time_t,
    mut gc: *mut grid_cell,
) -> *mut libc::c_char {
    let mut oo: *mut options = (*(*wl).window).options;
    let mut s: *mut session = (*c).session;
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    style_apply_update(
        gc,
        oo,
        b"window-status-style\0" as *const u8 as *const libc::c_char,
    );
    fmt = options_get_string(
        oo,
        b"window-status-format\0" as *const u8 as *const libc::c_char,
    );
    if wl == (*s).curw {
        style_apply_update(
            gc,
            oo,
            b"window-status-current-style\0" as *const u8 as *const libc::c_char,
        );
        fmt = options_get_string(
            oo,
            b"window-status-current-format\0" as *const u8 as *const libc::c_char,
        );
    }
    if wl == (*s).lastw.tqh_first {
        style_apply_update(
            gc,
            oo,
            b"window-status-last-style\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*wl).flags & 0x1 as libc::c_int != 0 {
        style_apply_update(
            gc,
            oo,
            b"window-status-bell-style\0" as *const u8 as *const libc::c_char,
        );
    } else if (*wl).flags & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 {
        style_apply_update(
            gc,
            oo,
            b"window-status-activity-style\0" as *const u8 as *const libc::c_char,
        );
    }
    text = status_replace(c, wl, fmt, t);
    return text;
}
#[no_mangle]
pub unsafe extern "C" fn status_message_clear(mut c: *mut client) {
    if ((*c).message_string).is_null() {
        return;
    }
    free((*c).message_string as *mut libc::c_void);
    (*c).message_string = 0 as *mut libc::c_char;
    if ((*c).prompt_string).is_null() {
        (*c).tty.flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
    }
    (*c).flags |= 0x8 as libc::c_int;
    screen_reinit(&mut (*c).status.status);
}
#[no_mangle]
pub unsafe extern "C" fn status_message_callback(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    status_message_clear(c);
}
#[no_mangle]
pub unsafe extern "C" fn status_message_redraw(mut c: *mut client) -> libc::c_int {
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
    let mut s: *mut session = (*c).session;
    let mut old_status: screen = screen {
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
    let mut len: size_t = 0;
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
    let mut lines: u_int = 0;
    let mut offset: u_int = 0;
    if (*c).tty.sx == 0 as libc::c_int as u_int
        || (*c).tty.sy == 0 as libc::c_int as u_int
    {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut old_status as *mut screen as *mut libc::c_void,
        &mut (*c).status.status as *mut screen as *const libc::c_void,
        ::core::mem::size_of::<screen>() as libc::c_ulong,
    );
    lines = status_line_size((*c).session);
    if lines <= 1 as libc::c_int as u_int {
        lines = 1 as libc::c_int as u_int;
        screen_init(
            &mut (*c).status.status,
            (*c).tty.sx,
            1 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
    } else {
        screen_init(
            &mut (*c).status.status,
            (*c).tty.sx,
            lines,
            0 as libc::c_int as u_int,
        );
    }
    len = screen_write_strlen(
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).message_string,
    );
    if len > (*c).tty.sx as size_t {
        len = (*c).tty.sx as size_t;
    }
    style_apply(
        &mut gc,
        (*s).options,
        b"message-style\0" as *const u8 as *const libc::c_char,
    );
    screen_write_start(&mut ctx, 0 as *mut window_pane, &mut (*c).status.status);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    offset = 0 as libc::c_int as u_int;
    while offset < lines * (*c).tty.sx {
        screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
        offset = offset.wrapping_add(1);
        offset;
    }
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        lines.wrapping_sub(1 as libc::c_int as u_int),
    );
    screen_write_nputs(
        &mut ctx as *mut screen_write_ctx,
        len as ssize_t,
        &mut gc as *mut grid_cell,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).message_string,
    );
    screen_write_stop(&mut ctx);
    if grid_compare((*c).status.status.grid, old_status.grid) == 0 as libc::c_int {
        screen_free(&mut old_status);
        return 0 as libc::c_int;
    }
    screen_free(&mut old_status);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_set(
    mut c: *mut client,
    mut msg: *const libc::c_char,
    mut input: *const libc::c_char,
    mut inputcb: prompt_input_cb,
    mut freecb: prompt_free_cb,
    mut data: *mut libc::c_void,
    mut flags: libc::c_int,
) {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut t: time_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    ft = format_create(c, 0 as *mut cmdq_item, 0 as libc::c_int, 0 as libc::c_int);
    format_defaults(ft, c, 0 as *mut session, 0 as *mut winlink, 0 as *mut window_pane);
    t = time(0 as *mut time_t);
    if input.is_null() {
        input = b"\0" as *const u8 as *const libc::c_char;
    }
    if flags & 0x8 as libc::c_int != 0 {
        tmp = xstrdup(input);
    } else {
        tmp = format_expand_time(ft, input, t);
    }
    status_message_clear(c);
    status_prompt_clear(c);
    if ((*c).status.old_status).is_null() {
        (*c)
            .status
            .old_status = xmalloc(::core::mem::size_of::<screen>() as libc::c_ulong)
            as *mut screen;
        memcpy(
            (*c).status.old_status as *mut libc::c_void,
            &mut (*c).status.status as *mut screen as *const libc::c_void,
            ::core::mem::size_of::<screen>() as libc::c_ulong,
        );
        screen_init(
            &mut (*c).status.status,
            (*c).tty.sx,
            1 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
    }
    (*c).prompt_string = format_expand_time(ft, msg, t);
    (*c).prompt_buffer = utf8_fromcstr(tmp);
    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
    (*c).prompt_inputcb = inputcb;
    (*c).prompt_freecb = freecb;
    (*c).prompt_data = data;
    (*c).prompt_hindex = 0 as libc::c_int as u_int;
    (*c).prompt_flags = flags;
    (*c).prompt_mode = PROMPT_ENTRY;
    if !flags & 0x4 as libc::c_int != 0 {
        (*c).tty.flags |= 0x1 as libc::c_int | 0x2 as libc::c_int;
    }
    (*c).flags |= 0x10 as libc::c_int;
    if flags & 0x4 as libc::c_int != 0 && *tmp as libc::c_int != '\0' as i32 {
        xasprintf(
            &mut cp as *mut *mut libc::c_char,
            b"=%s\0" as *const u8 as *const libc::c_char,
            tmp,
        );
        ((*c).prompt_inputcb)
            .expect(
                "non-null function pointer",
            )(c, (*c).prompt_data, cp, 0 as libc::c_int);
        free(cp as *mut libc::c_void);
    }
    free(tmp as *mut libc::c_void);
    format_free(ft);
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_clear(mut c: *mut client) {
    if ((*c).prompt_string).is_null() {
        return;
    }
    if ((*c).prompt_freecb).is_some() && !((*c).prompt_data).is_null() {
        ((*c).prompt_freecb).expect("non-null function pointer")((*c).prompt_data);
    }
    free((*c).prompt_string as *mut libc::c_void);
    (*c).prompt_string = 0 as *mut libc::c_char;
    free((*c).prompt_buffer as *mut libc::c_void);
    (*c).prompt_buffer = 0 as *mut utf8_data;
    (*c).tty.flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
    (*c).flags |= 0x8 as libc::c_int;
    screen_reinit(&mut (*c).status.status);
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_update(
    mut c: *mut client,
    mut msg: *const libc::c_char,
    mut input: *const libc::c_char,
) {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut t: time_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    ft = format_create(c, 0 as *mut cmdq_item, 0 as libc::c_int, 0 as libc::c_int);
    format_defaults(ft, c, 0 as *mut session, 0 as *mut winlink, 0 as *mut window_pane);
    t = time(0 as *mut time_t);
    tmp = format_expand_time(ft, input, t);
    free((*c).prompt_string as *mut libc::c_void);
    (*c).prompt_string = format_expand_time(ft, msg, t);
    free((*c).prompt_buffer as *mut libc::c_void);
    (*c).prompt_buffer = utf8_fromcstr(tmp);
    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
    (*c).prompt_hindex = 0 as libc::c_int as u_int;
    (*c).flags |= 0x10 as libc::c_int;
    free(tmp as *mut libc::c_void);
    format_free(ft);
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_redraw(mut c: *mut client) -> libc::c_int {
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
    let mut s: *mut session = (*c).session;
    let mut old_status: screen = screen {
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
    let mut i: u_int = 0;
    let mut offset: u_int = 0;
    let mut left: u_int = 0;
    let mut start: u_int = 0;
    let mut pcursor: u_int = 0;
    let mut pwidth: u_int = 0;
    let mut width: u_int = 0;
    let mut lines: u_int = 0;
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
    let mut cursorgc: grid_cell = grid_cell {
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
    if (*c).tty.sx == 0 as libc::c_int as u_int
        || (*c).tty.sy == 0 as libc::c_int as u_int
    {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut old_status as *mut screen as *mut libc::c_void,
        &mut (*c).status.status as *mut screen as *const libc::c_void,
        ::core::mem::size_of::<screen>() as libc::c_ulong,
    );
    lines = status_line_size((*c).session);
    if lines <= 1 as libc::c_int as u_int {
        lines = 1 as libc::c_int as u_int;
        screen_init(
            &mut (*c).status.status,
            (*c).tty.sx,
            1 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
    } else {
        screen_init(
            &mut (*c).status.status,
            (*c).tty.sx,
            lines,
            0 as libc::c_int as u_int,
        );
    }
    if (*c).prompt_mode as libc::c_uint == PROMPT_COMMAND as libc::c_int as libc::c_uint
    {
        style_apply(
            &mut gc,
            (*s).options,
            b"message-command-style\0" as *const u8 as *const libc::c_char,
        );
    } else {
        style_apply(
            &mut gc,
            (*s).options,
            b"message-style\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        &mut cursorgc as *mut grid_cell as *mut libc::c_void,
        &mut gc as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    cursorgc.attr = (cursorgc.attr as libc::c_int ^ 0x10 as libc::c_int) as u_short;
    start = screen_write_strlen(
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).prompt_string,
    ) as u_int;
    if start > (*c).tty.sx {
        start = (*c).tty.sx;
    }
    screen_write_start(&mut ctx, 0 as *mut window_pane, &mut (*c).status.status);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    offset = 0 as libc::c_int as u_int;
    while offset < lines * (*c).tty.sx {
        screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
        offset = offset.wrapping_add(1);
        offset;
    }
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    screen_write_nputs(
        &mut ctx as *mut screen_write_ctx,
        start as ssize_t,
        &mut gc as *mut grid_cell,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).prompt_string,
    );
    screen_write_cursormove(&mut ctx, start, 0 as libc::c_int as u_int);
    left = ((*c).tty.sx).wrapping_sub(start);
    if !(left == 0 as libc::c_int as u_int) {
        pcursor = utf8_strwidth((*c).prompt_buffer, (*c).prompt_index as ssize_t);
        pwidth = utf8_strwidth((*c).prompt_buffer, -(1 as libc::c_int) as ssize_t);
        if pcursor >= left {
            offset = pcursor.wrapping_sub(left).wrapping_add(1 as libc::c_int as u_int);
            pwidth = left;
        } else {
            offset = 0 as libc::c_int as u_int;
        }
        if pwidth > left {
            pwidth = left;
        }
        width = 0 as libc::c_int as u_int;
        i = 0 as libc::c_int as u_int;
        while (*((*c).prompt_buffer).offset(i as isize)).size as libc::c_int
            != 0 as libc::c_int
        {
            if width < offset {
                width = width
                    .wrapping_add(
                        (*((*c).prompt_buffer).offset(i as isize)).width as u_int,
                    );
            } else {
                if width >= offset.wrapping_add(pwidth) {
                    break;
                }
                width = width
                    .wrapping_add(
                        (*((*c).prompt_buffer).offset(i as isize)).width as u_int,
                    );
                if width > offset.wrapping_add(pwidth) {
                    break;
                }
                if i as size_t != (*c).prompt_index {
                    utf8_copy(
                        &mut gc.data,
                        &mut *((*c).prompt_buffer).offset(i as isize),
                    );
                    screen_write_cell(&mut ctx, &mut gc);
                } else {
                    utf8_copy(
                        &mut cursorgc.data,
                        &mut *((*c).prompt_buffer).offset(i as isize),
                    );
                    screen_write_cell(&mut ctx, &mut cursorgc);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if (*c).status.status.cx < (*(*c).status.status.grid).sx
            && (*c).prompt_index >= i as size_t
        {
            screen_write_putc(&mut ctx, &mut cursorgc, ' ' as i32 as u_char);
        }
    }
    screen_write_stop(&mut ctx);
    if grid_compare((*c).status.status.grid, old_status.grid) == 0 as libc::c_int {
        screen_free(&mut old_status);
        return 0 as libc::c_int;
    }
    screen_free(&mut old_status);
    return 1 as libc::c_int;
}
unsafe extern "C" fn status_prompt_in_list(
    mut ws: *const libc::c_char,
    mut ud: *const utf8_data,
) -> libc::c_int {
    if (*ud).size as libc::c_int != 1 as libc::c_int
        || (*ud).width as libc::c_int != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (strchr(ws, *((*ud).data).as_ptr() as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
unsafe extern "C" fn status_prompt_space(mut ud: *const utf8_data) -> libc::c_int {
    if (*ud).size as libc::c_int != 1 as libc::c_int
        || (*ud).width as libc::c_int != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (*((*ud).data).as_ptr() as libc::c_int == ' ' as i32) as libc::c_int;
}
unsafe extern "C" fn status_prompt_translate_key(
    mut c: *mut client,
    mut key: key_code,
    mut new_key: *mut key_code,
) -> libc::c_int {
    if (*c).prompt_mode as libc::c_uint == PROMPT_ENTRY as libc::c_int as libc::c_uint {
        's_55: {
            let mut current_block_4: u64;
            match key {
                8 => {
                    current_block_4 = 900269801797573450;
                }
                9 => {
                    current_block_4 = 900269801797573450;
                }
                21 => {
                    current_block_4 = 402291076735761312;
                }
                23 => {
                    current_block_4 = 5458313747564545892;
                }
                3 | 10 | 13 | 268435525 | 268435539 | 268435546 | 268435541 | 268435540
                | 268435547 | 268435548 | 268435545 => {
                    current_block_4 = 4507210058922081320;
                }
                27 => {
                    (*c).prompt_mode = PROMPT_COMMAND;
                    (*c).flags |= 0x10 as libc::c_int;
                    return 0 as libc::c_int;
                }
                _ => {
                    break 's_55;
                }
            }
            match current_block_4 {
                900269801797573450 => {
                    current_block_4 = 402291076735761312;
                }
                _ => {}
            }
            match current_block_4 {
                402291076735761312 => {
                    current_block_4 = 5458313747564545892;
                }
                _ => {}
            }
            match current_block_4 {
                5458313747564545892 => {}
                _ => {}
            }
            *new_key = key;
            return 1 as libc::c_int;
        }
        *new_key = key;
        return 2 as libc::c_int;
    }
    match key {
        65 | 73 | 67 | 115 | 97 => {
            (*c).prompt_mode = PROMPT_ENTRY;
            (*c).flags |= 0x10 as libc::c_int;
        }
        83 => {
            (*c).prompt_mode = PROMPT_ENTRY;
            (*c).flags |= 0x10 as libc::c_int;
            *new_key = '\u{15}' as i32 as key_code;
            return 1 as libc::c_int;
        }
        105 | 27 => {
            (*c).prompt_mode = PROMPT_ENTRY;
            (*c).flags |= 0x10 as libc::c_int;
            return 0 as libc::c_int;
        }
        _ => {}
    }
    's_265: {
        match key {
            65 | 36 => {
                *new_key = KEYC_END as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            73 | 48 | 94 => {
                *new_key = KEYC_HOME as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            67 | 68 => {
                *new_key = '\u{b}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            268435525 | 88 => {
                *new_key = KEYC_BSPACE as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            98 | 66 => {
                *new_key = 'b' as i32 as libc::c_ulonglong
                    | 0x200000000000 as libc::c_ulonglong;
                return 1 as libc::c_int;
            }
            100 => {
                *new_key = '\u{15}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            101 | 69 | 119 | 87 => {
                *new_key = 'f' as i32 as libc::c_ulonglong
                    | 0x200000000000 as libc::c_ulonglong;
                return 1 as libc::c_int;
            }
            112 => {
                *new_key = '\u{19}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            115 | 268435539 | 120 => {
                *new_key = KEYC_DC as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            268435546 | 106 => {
                *new_key = KEYC_DOWN as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            268435547 | 104 => {
                *new_key = KEYC_LEFT as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            97 | 268435548 | 108 => {
                *new_key = KEYC_RIGHT as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            268435545 | 107 => {
                *new_key = KEYC_UP as libc::c_int as key_code;
                return 1 as libc::c_int;
            }
            3 => {}
            8 | 10 | 13 => {}
            _ => {
                break 's_265;
            }
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn status_prompt_key(
    mut c: *mut client,
    mut key: key_code,
) -> libc::c_int {
    let mut current_block: u64;
    let mut oo: *mut options = (*(*c).session).options;
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut word: [libc::c_char; 64] = [0; 64];
    let mut prefix: libc::c_char = '=' as i32 as libc::c_char;
    let mut histstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut bufdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: u_char = 0;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut off: size_t = 0;
    let mut idx: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut used: size_t = 0;
    let mut tmp: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut first: *mut utf8_data = 0 as *mut utf8_data;
    let mut last: *mut utf8_data = 0 as *mut utf8_data;
    let mut ud: *mut utf8_data = 0 as *mut utf8_data;
    let mut keys: libc::c_int = 0;
    size = utf8_strlen((*c).prompt_buffer);
    if (*c).prompt_flags & 0x2 as libc::c_int != 0 {
        if key >= '0' as i32 as key_code && key <= '9' as i32 as key_code {
            current_block = 12621319171214187923;
        } else {
            s = utf8_tocstr((*c).prompt_buffer);
            ((*c).prompt_inputcb)
                .expect(
                    "non-null function pointer",
                )(c, (*c).prompt_data, s, 1 as libc::c_int);
            status_prompt_clear(c);
            free(s as *mut libc::c_void);
            return 1 as libc::c_int;
        }
    } else {
        keys = options_get_number(
            (*(*c).session).options,
            b"status-keys\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if keys == 1 as libc::c_int {
            match status_prompt_translate_key(c, key, &mut key) {
                1 => {
                    current_block = 11686527193359420982;
                }
                2 => {
                    current_block = 12621319171214187923;
                }
                _ => return 0 as libc::c_int,
            }
        } else {
            current_block = 11686527193359420982;
        }
        match current_block {
            12621319171214187923 => {}
            _ => {
                match key {
                    268435547 | 2 => {
                        current_block = 7628352842969142023;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435548 | 6 => {
                        current_block = 175827181993450723;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435540 | 1 => {
                        current_block = 8135404661094233500;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435541 | 5 => {
                        current_block = 422113013618400685;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    9 => {
                        current_block = 12948814156367993796;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435525 | 8 => {
                        current_block = 15245840686579455712;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435539 | 4 => {
                        current_block = 1031939171554788861;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    21 => {
                        current_block = 1041743423343129721;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    11 => {
                        current_block = 8743794021820299180;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    23 => {
                        current_block = 3684944600068393837;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    35184372088934 => {
                        current_block = 7125467772460904655;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    35184372088930 => {
                        current_block = 10466928731657558944;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435545 | 16 => {
                        current_block = 4977116662774201786;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    268435546 | 14 => {
                        current_block = 16544677890783681085;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    25 => {
                        current_block = 8908227913221920234;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    20 => {
                        current_block = 3724164785612952151;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    13 | 10 => {
                        current_block = 9609989243947815934;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    27 => {
                        current_block = 2749476544691729835;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    3 | 7 => {
                        current_block = 2035738867698081325;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    18 => {
                        current_block = 8823912109039068991;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    19 => {
                        current_block = 7249696149466105880;
                        match current_block {
                            8823912109039068991 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            9609989243947815934 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\0' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if ((*c).prompt_inputcb)
                                    .expect(
                                        "non-null function pointer",
                                    )(c, (*c).prompt_data, s, 1 as libc::c_int)
                                    == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 15482252313049709402;
                            }
                            3724164785612952151 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1);
                                    idx;
                                }
                                if idx >= 2 as libc::c_int as size_t {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(2 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                    );
                                    utf8_copy(
                                        &mut *((*c).prompt_buffer)
                                            .offset(
                                                idx.wrapping_sub(1 as libc::c_int as size_t) as isize,
                                            ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8908227913221920234 => {
                                pb = paste_get_top(0 as *mut *const libc::c_char);
                                if pb.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    bufdata = paste_buffer_data(pb, &mut bufsize);
                                    n = 0 as libc::c_int as size_t;
                                    while n < bufsize {
                                        ch = *bufdata.offset(n as isize) as u_char;
                                        if (ch as libc::c_int) < 32 as libc::c_int
                                            || ch as libc::c_int >= 127 as libc::c_int
                                        {
                                            break;
                                        }
                                        n = n.wrapping_add(1);
                                        n;
                                    }
                                    (*c)
                                        .prompt_buffer = xreallocarray(
                                        (*c).prompt_buffer as *mut libc::c_void,
                                        size
                                            .wrapping_add(n)
                                            .wrapping_add(1 as libc::c_int as size_t),
                                        ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ) as *mut utf8_data;
                                    if (*c).prompt_index == size {
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(n as isize) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        idx = 0 as libc::c_int as size_t;
                                        while idx < n {
                                            ud = &mut *((*c).prompt_buffer)
                                                .offset(((*c).prompt_index).wrapping_add(idx) as isize)
                                                as *mut utf8_data;
                                            utf8_set(ud, *bufdata.offset(idx as isize) as u_char);
                                            idx = idx.wrapping_add(1);
                                            idx;
                                        }
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_add(n);
                                    }
                                    current_block = 7620874232747644143;
                                }
                            }
                            16544677890783681085 => {
                                histstr = status_prompt_down_history(
                                    &mut (*c).prompt_hindex,
                                );
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            4977116662774201786 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 15482252313049709402;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 7620874232747644143;
                                }
                            }
                            10466928731657558944 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    break;
                                }
                                current_block = 7620874232747644143;
                            }
                            7125467772460904655 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\0" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as size_t
                                {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                }
                                current_block = 7620874232747644143;
                            }
                            3684944600068393837 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\0" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as size_t {
                                    idx = idx.wrapping_sub(1);
                                    idx;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *((*c).prompt_buffer).offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    idx;
                                    break;
                                }
                                memmove(
                                    ((*c).prompt_buffer).offset(idx as isize)
                                        as *mut libc::c_void,
                                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size
                                        .wrapping_add(1 as libc::c_int as size_t)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                memset(
                                    ((*c).prompt_buffer)
                                        .offset(size as isize)
                                        .offset(-(((*c).prompt_index).wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\0' as i32,
                                    ((*c).prompt_index)
                                        .wrapping_sub(idx)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                        ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 7620874232747644143;
                            }
                            8743794021820299180 => {
                                if (*c).prompt_index < size {
                                    (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                        .size = 0 as libc::c_int as u_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            1041743423343129721 => {
                                (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size = 0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 7620874232747644143;
                            }
                            1031939171554788861 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        ((*c).prompt_buffer)
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                        size
                                            .wrapping_add(1 as libc::c_int as size_t)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                            ),
                                    );
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            15245840686579455712 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char;
                                    } else {
                                        memmove(
                                            ((*c).prompt_buffer)
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                                            ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size
                                                .wrapping_add(1 as libc::c_int as size_t)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ),
                                        );
                                        (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                        (*c).prompt_index;
                                    }
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            12948814156367993796 => {
                                if (*((*c).prompt_buffer).offset(0 as libc::c_int as isize))
                                    .size as libc::c_int == 0 as libc::c_int
                                {
                                    current_block = 15482252313049709402;
                                } else {
                                    idx = (*c).prompt_index;
                                    if idx != 0 as libc::c_int as size_t {
                                        idx = idx.wrapping_sub(1);
                                        idx;
                                    }
                                    first = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while first > (*c).prompt_buffer
                                        && status_prompt_space(first) == 0
                                    {
                                        first = first.offset(-1);
                                        first;
                                    }
                                    while (*first).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(first) != 0
                                    {
                                        first = first.offset(1);
                                        first;
                                    }
                                    last = &mut *((*c).prompt_buffer).offset(idx as isize)
                                        as *mut utf8_data;
                                    while (*last).size as libc::c_int != 0 as libc::c_int
                                        && status_prompt_space(last) == 0
                                    {
                                        last = last.offset(1);
                                        last;
                                    }
                                    while last > (*c).prompt_buffer
                                        && status_prompt_space(last) != 0
                                    {
                                        last = last.offset(-1);
                                        last;
                                    }
                                    if (*last).size as libc::c_int != 0 as libc::c_int {
                                        last = last.offset(1);
                                        last;
                                    }
                                    if last <= first {
                                        current_block = 15482252313049709402;
                                    } else {
                                        used = 0 as libc::c_int as size_t;
                                        ud = first;
                                        while ud < last {
                                            if used.wrapping_add((*ud).size as size_t)
                                                >= ::core::mem::size_of::<[libc::c_char; 64]>()
                                                    as libc::c_ulong
                                            {
                                                break;
                                            }
                                            memcpy(
                                                word.as_mut_ptr().offset(used as isize)
                                                    as *mut libc::c_void,
                                                ((*ud).data).as_mut_ptr() as *const libc::c_void,
                                                (*ud).size as libc::c_ulong,
                                            );
                                            used = used.wrapping_add((*ud).size as size_t);
                                            ud = ud.offset(1);
                                            ud;
                                        }
                                        if ud != last {
                                            current_block = 15482252313049709402;
                                        } else {
                                            word[used as usize] = '\0' as i32 as libc::c_char;
                                            s = status_prompt_complete((*c).session, word.as_mut_ptr());
                                            if s.is_null() {
                                                current_block = 15482252313049709402;
                                            } else {
                                                n = size
                                                    .wrapping_sub(
                                                        last.offset_from((*c).prompt_buffer) as libc::c_long
                                                            as size_t,
                                                    )
                                                    .wrapping_add(1 as libc::c_int as size_t);
                                                memmove(
                                                    first as *mut libc::c_void,
                                                    last as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                size = size
                                                    .wrapping_sub(
                                                        last.offset_from(first) as libc::c_long as size_t,
                                                    );
                                                size = (size as libc::c_ulong).wrapping_add(strlen(s))
                                                    as size_t as size_t;
                                                off = first.offset_from((*c).prompt_buffer) as libc::c_long
                                                    as size_t;
                                                (*c)
                                                    .prompt_buffer = xreallocarray(
                                                    (*c).prompt_buffer as *mut libc::c_void,
                                                    size.wrapping_add(1 as libc::c_int as size_t),
                                                    ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                ) as *mut utf8_data;
                                                first = ((*c).prompt_buffer).offset(off as isize);
                                                memmove(
                                                    first.offset(strlen(s) as isize) as *mut libc::c_void,
                                                    first as *const libc::c_void,
                                                    n
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                                                        ),
                                                );
                                                idx = 0 as libc::c_int as size_t;
                                                while idx < strlen(s) {
                                                    utf8_set(
                                                        &mut *first.offset(idx as isize),
                                                        *s.offset(idx as isize) as u_char,
                                                    );
                                                    idx = idx.wrapping_add(1);
                                                    idx;
                                                }
                                                (*c)
                                                    .prompt_index = (first.offset_from((*c).prompt_buffer)
                                                    as libc::c_long as libc::c_ulong)
                                                    .wrapping_add(strlen(s));
                                                free(s as *mut libc::c_void);
                                                current_block = 7620874232747644143;
                                            }
                                        }
                                    }
                                }
                            }
                            422113013618400685 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            8135404661094233500 => {
                                if (*c).prompt_index != 0 as libc::c_int as size_t {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            175827181993450723 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7628352842969142023 => {
                                if (*c).prompt_index > 0 as libc::c_int as size_t {
                                    (*c).prompt_index = ((*c).prompt_index).wrapping_sub(1);
                                    (*c).prompt_index;
                                    current_block = 15482252313049709402;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            7249696149466105880 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 7620874232747644143;
                                } else {
                                    current_block = 15482252313049709402;
                                }
                            }
                            2749476544691729835 => {
                                current_block = 2035738867698081325;
                            }
                            _ => {}
                        }
                        match current_block {
                            7620874232747644143 => {}
                            _ => {
                                match current_block {
                                    2035738867698081325 => {
                                        if ((*c).prompt_inputcb)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        current_block = 12621319171214187923;
                    }
                }
            }
        }
    }
    match current_block {
        12621319171214187923 => {
            if key <= 0x1f as libc::c_int as key_code
                || key >= 0x10000000 as libc::c_ulonglong
            {
                return 0 as libc::c_int;
            }
            if utf8_split(key as wchar_t, &mut tmp) as libc::c_uint
                != UTF8_DONE as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
            (*c)
                .prompt_buffer = xreallocarray(
                (*c).prompt_buffer as *mut libc::c_void,
                size.wrapping_add(2 as libc::c_int as size_t),
                ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
            ) as *mut utf8_data;
            if (*c).prompt_index == size {
                utf8_copy(
                    &mut *((*c).prompt_buffer).offset((*c).prompt_index as isize),
                    &mut tmp,
                );
                (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                (*c).prompt_index;
                (*((*c).prompt_buffer).offset((*c).prompt_index as isize))
                    .size = 0 as libc::c_int as u_char;
            } else {
                memmove(
                    ((*c).prompt_buffer)
                        .offset((*c).prompt_index as isize)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    ((*c).prompt_buffer).offset((*c).prompt_index as isize)
                        as *const libc::c_void,
                    size
                        .wrapping_add(1 as libc::c_int as size_t)
                        .wrapping_sub((*c).prompt_index)
                        .wrapping_mul(
                            ::core::mem::size_of::<utf8_data>() as libc::c_ulong,
                        ),
                );
                utf8_copy(
                    &mut *((*c).prompt_buffer).offset((*c).prompt_index as isize),
                    &mut tmp,
                );
                (*c).prompt_index = ((*c).prompt_index).wrapping_add(1);
                (*c).prompt_index;
            }
            if (*c).prompt_flags & 0x1 as libc::c_int != 0 {
                s = utf8_tocstr((*c).prompt_buffer);
                if strlen(s) != 1 as libc::c_int as libc::c_ulong {
                    status_prompt_clear(c);
                } else if ((*c).prompt_inputcb)
                    .expect(
                        "non-null function pointer",
                    )(c, (*c).prompt_data, s, 1 as libc::c_int) == 0 as libc::c_int
                {
                    status_prompt_clear(c);
                }
                free(s as *mut libc::c_void);
            }
        }
        _ => {}
    }
    (*c).flags |= 0x10 as libc::c_int;
    if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
        s = utf8_tocstr((*c).prompt_buffer);
        xasprintf(
            &mut cp as *mut *mut libc::c_char,
            b"%c%s\0" as *const u8 as *const libc::c_char,
            prefix as libc::c_int,
            s,
        );
        ((*c).prompt_inputcb)
            .expect(
                "non-null function pointer",
            )(c, (*c).prompt_data, cp, 0 as libc::c_int);
        free(cp as *mut libc::c_void);
        free(s as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn status_prompt_up_history(
    mut idx: *mut u_int,
) -> *const libc::c_char {
    if status_prompt_hsize == 0 as libc::c_int as u_int || *idx == status_prompt_hsize {
        return 0 as *const libc::c_char;
    }
    *idx = (*idx).wrapping_add(1);
    *idx;
    return *status_prompt_hlist.offset(status_prompt_hsize.wrapping_sub(*idx) as isize);
}
unsafe extern "C" fn status_prompt_down_history(
    mut idx: *mut u_int,
) -> *const libc::c_char {
    if status_prompt_hsize == 0 as libc::c_int as u_int
        || *idx == 0 as libc::c_int as u_int
    {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    *idx = (*idx).wrapping_sub(1);
    *idx;
    if *idx == 0 as libc::c_int as u_int {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    return *status_prompt_hlist.offset(status_prompt_hsize.wrapping_sub(*idx) as isize);
}
unsafe extern "C" fn status_prompt_add_history(mut line: *const libc::c_char) {
    let mut size: size_t = 0;
    if status_prompt_hsize > 0 as libc::c_int as u_int
        && strcmp(
            *status_prompt_hlist
                .offset(
                    status_prompt_hsize.wrapping_sub(1 as libc::c_int as u_int) as isize,
                ),
            line,
        ) == 0 as libc::c_int
    {
        return;
    }
    if status_prompt_hsize == 100 as libc::c_int as u_int {
        free(
            *status_prompt_hlist.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        );
        size = ((100 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
        memmove(
            &mut *status_prompt_hlist.offset(0 as libc::c_int as isize)
                as *mut *mut libc::c_char as *mut libc::c_void,
            &mut *status_prompt_hlist.offset(1 as libc::c_int as isize)
                as *mut *mut libc::c_char as *const libc::c_void,
            size,
        );
        let ref mut fresh0 = *status_prompt_hlist
            .offset(
                status_prompt_hsize.wrapping_sub(1 as libc::c_int as u_int) as isize,
            );
        *fresh0 = xstrdup(line);
        return;
    }
    status_prompt_hlist = xreallocarray(
        status_prompt_hlist as *mut libc::c_void,
        status_prompt_hsize.wrapping_add(1 as libc::c_int as u_int) as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let fresh1 = status_prompt_hsize;
    status_prompt_hsize = status_prompt_hsize.wrapping_add(1);
    let ref mut fresh2 = *status_prompt_hlist.offset(fresh1 as isize);
    *fresh2 = xstrdup(line);
}
unsafe extern "C" fn status_prompt_complete_list(
    mut size: *mut u_int,
    mut s: *const libc::c_char,
) -> *mut *const libc::c_char {
    let mut list: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut layout: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut cmdent: *mut *const cmd_entry = 0 as *mut *const cmd_entry;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut layouts: [*const libc::c_char; 6] = [
        b"even-horizontal\0" as *const u8 as *const libc::c_char,
        b"even-vertical\0" as *const u8 as *const libc::c_char,
        b"main-horizontal\0" as *const u8 as *const libc::c_char,
        b"main-vertical\0" as *const u8 as *const libc::c_char,
        b"tiled\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    *size = 0 as libc::c_int as u_int;
    cmdent = cmd_table.as_mut_ptr();
    while !(*cmdent).is_null() {
        if strncmp((**cmdent).name, s, strlen(s)) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as u_int) as size_t,
                ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            ) as *mut *const libc::c_char;
            let fresh3 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh4 = *list.offset(fresh3 as isize);
            *fresh4 = (**cmdent).name;
        }
        cmdent = cmdent.offset(1);
        cmdent;
    }
    oe = options_table.as_ptr();
    while !((*oe).name).is_null() {
        if strncmp((*oe).name, s, strlen(s)) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as u_int) as size_t,
                ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            ) as *mut *const libc::c_char;
            let fresh5 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh6 = *list.offset(fresh5 as isize);
            *fresh6 = (*oe).name;
        }
        oe = oe.offset(1);
        oe;
    }
    layout = layouts.as_mut_ptr();
    while !(*layout).is_null() {
        if strncmp(*layout, s, strlen(s)) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as u_int) as size_t,
                ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            ) as *mut *const libc::c_char;
            let fresh7 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh8 = *list.offset(fresh7 as isize);
            *fresh8 = *layout;
        }
        layout = layout.offset(1);
        layout;
    }
    return list;
}
unsafe extern "C" fn status_prompt_complete_prefix(
    mut list: *mut *const libc::c_char,
    mut size: u_int,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    let mut j: size_t = 0;
    out = xstrdup(*list.offset(0 as libc::c_int as isize));
    i = 1 as libc::c_int as u_int;
    while i < size {
        j = strlen(*list.offset(i as isize));
        if j > strlen(out) {
            j = strlen(out);
        }
        while j > 0 as libc::c_int as size_t {
            if *out.offset(j.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_int
                != *(*list.offset(i as isize))
                    .offset(j.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int
            {
                *out
                    .offset(
                        j.wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            j = j.wrapping_sub(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return out;
}
unsafe extern "C" fn status_prompt_complete(
    mut session: *mut session,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut list: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: u_int = 0 as libc::c_int as u_int;
    let mut i: u_int = 0;
    let mut s_loop: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut w: *mut window = 0 as *mut window;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if *s as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    out = 0 as *mut libc::c_char;
    if strncmp(
        s,
        b"-t\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && strncmp(
            s,
            b"-s\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        list = status_prompt_complete_list(&mut size, s);
        if size == 0 as libc::c_int as u_int {
            out = 0 as *mut libc::c_char;
        } else if size == 1 as libc::c_int as u_int {
            xasprintf(
                &mut out as *mut *mut libc::c_char,
                b"%s \0" as *const u8 as *const libc::c_char,
                *list.offset(0 as libc::c_int as isize),
            );
        } else {
            out = status_prompt_complete_prefix(list, size);
        }
        free(list as *mut libc::c_void);
        return out;
    }
    copy = xstrdup(s);
    colon = b":\0" as *const u8 as *const libc::c_char;
    if *copy
        .offset((strlen(copy)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == ':' as i32
    {
        *copy
            .offset(
                (strlen(copy)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    } else {
        colon = b"\0" as *const u8 as *const libc::c_char;
    }
    s = copy.offset(2 as libc::c_int as isize);
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if strncmp((*s_loop).name, s, strlen(s)) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                size.wrapping_add(2 as libc::c_int as u_int) as size_t,
                ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            ) as *mut *const libc::c_char;
            let fresh9 = size;
            size = size.wrapping_add(1);
            let ref mut fresh10 = *list.offset(fresh9 as isize);
            *fresh10 = (*s_loop).name;
        }
        s_loop = sessions_RB_NEXT(s_loop);
    }
    if size == 1 as libc::c_int as u_int {
        out = xstrdup(*list.offset(0 as libc::c_int as isize));
        if !(session_find(*list.offset(0 as libc::c_int as isize))).is_null() {
            colon = b":\0" as *const u8 as *const libc::c_char;
        }
    } else if size != 0 as libc::c_int as u_int {
        out = status_prompt_complete_prefix(list, size);
    }
    if !out.is_null() {
        xasprintf(
            &mut tmp as *mut *mut libc::c_char,
            b"-%c%s%s\0" as *const u8 as *const libc::c_char,
            *copy.offset(1 as libc::c_int as isize) as libc::c_int,
            out,
            colon,
        );
        free(out as *mut libc::c_void);
        out = tmp;
    } else {
        colon = b"\0" as *const u8 as *const libc::c_char;
        if *s as libc::c_int == ':' as i32 {
            wl = winlinks_RB_MINMAX(&mut (*session).windows, -(1 as libc::c_int));
            while !wl.is_null() {
                xasprintf(
                    &mut tmp as *mut *mut libc::c_char,
                    b":%s\0" as *const u8 as *const libc::c_char,
                    (*(*wl).window).name,
                );
                if strncmp(tmp, s, strlen(s)) == 0 as libc::c_int {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ) as *mut *const libc::c_char;
                    let fresh11 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh12 = *list.offset(fresh11 as isize);
                    *fresh12 = tmp;
                } else {
                    free(tmp as *mut libc::c_void);
                    xasprintf(
                        &mut tmp as *mut *mut libc::c_char,
                        b":%d\0" as *const u8 as *const libc::c_char,
                        (*wl).idx,
                    );
                    if strncmp(tmp, s, strlen(s)) == 0 as libc::c_int {
                        list = xreallocarray(
                            list as *mut libc::c_void,
                            size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                            ::core::mem::size_of::<*const libc::c_char>()
                                as libc::c_ulong,
                        ) as *mut *const libc::c_char;
                        let fresh13 = size;
                        size = size.wrapping_add(1);
                        let ref mut fresh14 = *list.offset(fresh13 as isize);
                        *fresh14 = tmp;
                    } else {
                        free(tmp as *mut libc::c_void);
                    }
                }
                wl = winlinks_RB_NEXT(wl);
            }
        } else {
            s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
            while !s_loop.is_null() {
                wl = winlinks_RB_MINMAX(&mut (*s_loop).windows, -(1 as libc::c_int));
                while !wl.is_null() {
                    w = (*wl).window;
                    xasprintf(
                        &mut tmp as *mut *mut libc::c_char,
                        b"%s:%s\0" as *const u8 as *const libc::c_char,
                        (*s_loop).name,
                        (*w).name,
                    );
                    if strncmp(tmp, s, strlen(s)) == 0 as libc::c_int {
                        list = xreallocarray(
                            list as *mut libc::c_void,
                            size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                            ::core::mem::size_of::<*const libc::c_char>()
                                as libc::c_ulong,
                        ) as *mut *const libc::c_char;
                        let fresh15 = size;
                        size = size.wrapping_add(1);
                        let ref mut fresh16 = *list.offset(fresh15 as isize);
                        *fresh16 = tmp;
                    } else {
                        free(tmp as *mut libc::c_void);
                        xasprintf(
                            &mut tmp as *mut *mut libc::c_char,
                            b"%s:%d\0" as *const u8 as *const libc::c_char,
                            (*s_loop).name,
                            (*wl).idx,
                        );
                        if strncmp(tmp, s, strlen(s)) == 0 as libc::c_int {
                            list = xreallocarray(
                                list as *mut libc::c_void,
                                size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                                ::core::mem::size_of::<*const libc::c_char>()
                                    as libc::c_ulong,
                            ) as *mut *const libc::c_char;
                            let fresh17 = size;
                            size = size.wrapping_add(1);
                            let ref mut fresh18 = *list.offset(fresh17 as isize);
                            *fresh18 = tmp;
                        } else {
                            free(tmp as *mut libc::c_void);
                        }
                    }
                    wl = winlinks_RB_NEXT(wl);
                }
                s_loop = sessions_RB_NEXT(s_loop);
            }
        }
        if size == 1 as libc::c_int as u_int {
            out = xstrdup(*list.offset(0 as libc::c_int as isize));
            colon = b" \0" as *const u8 as *const libc::c_char;
        } else if size != 0 as libc::c_int as u_int {
            out = status_prompt_complete_prefix(list, size);
        }
        if !out.is_null() {
            xasprintf(
                &mut tmp as *mut *mut libc::c_char,
                b"-%c%s%s\0" as *const u8 as *const libc::c_char,
                *copy.offset(1 as libc::c_int as isize) as libc::c_int,
                out,
                colon,
            );
            out = tmp;
        }
        i = 0 as libc::c_int as u_int;
        while i < size {
            free(*list.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
    }
    free(copy as *mut libc::c_void);
    free(list as *mut libc::c_void);
    return out;
}
