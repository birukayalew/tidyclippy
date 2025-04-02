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
    pub type options_entry;
    pub type tmuxproc;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn closefrom(__lowfd: libc::c_int);
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
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
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    fn bufferevent_free(bufev: *mut bufferevent);
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    fn bufferevent_setwatermark(
        bufev: *mut bufferevent,
        events: libc::c_short,
        lowmark: size_t,
        highmark: size_t,
    );
    fn bufferevent_new(
        fd: libc::c_int,
        readcb: bufferevent_data_cb,
        writecb: bufferevent_data_cb,
        errorcb: bufferevent_event_cb,
        cbarg: *mut libc::c_void,
    ) -> *mut bufferevent;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn fdforkpty(
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_char,
        _: *mut termios,
        _: *mut winsize,
    ) -> pid_t;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_w_options: *mut options;
    static mut ptm_fd: libc::c_int;
    fn setblocking(_: libc::c_int, _: libc::c_int);
    fn find_home() -> *const libc::c_char;
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int);
    fn notify_window(_: *const libc::c_char, _: *mut window);
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
    fn options_create(_: *mut options) -> *mut options;
    fn options_free(_: *mut options);
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn options_get_style(_: *mut options, _: *const libc::c_char) -> *const grid_cell;
    fn options_set_number(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut options_entry;
    fn environ_set(
        _: *mut environ,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    fn environ_push(_: *mut environ);
    fn environ_log(_: *mut environ, _: *const libc::c_char, _: ...);
    fn cmd_copy_argv(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    fn cmd_stringify_argv(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn alerts_queue(_: *mut window, _: libc::c_int);
    static mut server_proc: *mut tmuxproc;
    static mut marked_pane: cmd_find_state;
    fn server_clear_marked();
    fn server_check_marked() -> libc::c_int;
    fn server_status_session(_: *mut session);
    fn server_status_window(_: *mut window);
    fn server_link_window(
        _: *mut session,
        _: *mut winlink,
        _: *mut session,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn server_unlink_window(_: *mut session, _: *mut winlink);
    fn server_destroy_pane(_: *mut window_pane, _: libc::c_int);
    fn input_init(_: *mut window_pane);
    fn input_free(_: *mut window_pane);
    fn input_parse(_: *mut window_pane);
    fn input_key(_: *mut window_pane, _: key_code, _: *mut mouse_event);
    static grid_default_cell: grid_cell;
    fn grid_create(_: u_int, _: u_int, _: u_int) -> *mut grid;
    fn grid_destroy(_: *mut grid);
    fn grid_duplicate_lines(_: *mut grid, _: u_int, _: *mut grid, _: u_int, _: u_int);
    fn grid_view_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int, _: u_int);
    fn grid_view_string_cells(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: u_int,
    ) -> *mut libc::c_char;
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn screen_free(_: *mut screen);
    fn screen_set_title(_: *mut screen, _: *const libc::c_char);
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    fn layout_free_cell(_: *mut layout_cell);
    fn layout_fix_panes(_: *mut window, _: u_int, _: u_int);
    fn layout_init(_: *mut window, _: *mut window_pane);
    fn layout_free(_: *mut window);
    fn default_window_name(_: *mut window) -> *mut libc::c_char;
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn log_close();
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    fn style_equal(_: *const grid_cell, _: *const grid_cell) -> libc::c_int;
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct windows {
    pub rbh_root: *mut window,
}
#[no_mangle]
pub static mut windows: windows = windows {
    rbh_root: 0 as *const window as *mut window,
};
#[no_mangle]
pub static mut all_window_panes: window_pane_tree = window_pane_tree {
    rbh_root: 0 as *const window_pane as *mut window_pane,
};
static mut next_window_pane_id: u_int = 0;
static mut next_window_id: u_int = 0;
static mut next_active_point: u_int = 0;
#[no_mangle]
pub unsafe extern "C" fn windows_RB_FIND(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut window;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_NFIND(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut res: *mut window = 0 as *mut window;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_cmp(elm, tmp);
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
pub unsafe extern "C" fn windows_RB_NEXT(mut elm: *mut window) -> *mut window {
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
pub unsafe extern "C" fn windows_RB_REMOVE(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut current_block: u64;
    let mut child: *mut window = 0 as *mut window;
    let mut parent: *mut window = 0 as *mut window;
    let mut old: *mut window = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut window = 0 as *mut window;
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
        current_block = 16866196648023966479;
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
        windows_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_MINMAX(
    mut head: *mut windows,
    mut val: libc::c_int,
) -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut parent: *mut window = 0 as *mut window;
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
pub unsafe extern "C" fn windows_RB_INSERT_COLOR(
    mut head: *mut windows,
    mut elm: *mut window,
) {
    let mut parent: *mut window = 0 as *mut window;
    let mut gparent: *mut window = 0 as *mut window;
    let mut tmp: *mut window = 0 as *mut window;
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
pub unsafe extern "C" fn windows_RB_PREV(mut elm: *mut window) -> *mut window {
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
pub unsafe extern "C" fn windows_RB_REMOVE_COLOR(
    mut head: *mut windows,
    mut parent: *mut window,
    mut elm: *mut window,
) {
    let mut tmp: *mut window = 0 as *mut window;
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
                    let mut oleft: *mut window = 0 as *mut window;
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
                    let mut oright: *mut window = 0 as *mut window;
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
pub unsafe extern "C" fn windows_RB_INSERT(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut tmp: *mut window = 0 as *mut window;
    let mut parent: *mut window = 0 as *mut window;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = window_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut window;
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
    windows_RB_INSERT_COLOR(head, elm);
    return 0 as *mut window;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_REMOVE_COLOR(
    mut head: *mut winlinks,
    mut parent: *mut winlink,
    mut elm: *mut winlink,
) {
    let mut tmp: *mut winlink = 0 as *mut winlink;
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
                    let mut oleft: *mut winlink = 0 as *mut winlink;
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
                    let mut oright: *mut winlink = 0 as *mut winlink;
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
pub unsafe extern "C" fn winlinks_RB_INSERT_COLOR(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) {
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut gparent: *mut winlink = 0 as *mut winlink;
    let mut tmp: *mut winlink = 0 as *mut winlink;
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
pub unsafe extern "C" fn winlinks_RB_REMOVE(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut current_block: u64;
    let mut child: *mut winlink = 0 as *mut winlink;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut old: *mut winlink = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut winlink = 0 as *mut winlink;
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
        current_block = 16763536307615388290;
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
        winlinks_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_INSERT(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut tmp: *mut winlink = 0 as *mut winlink;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = winlink_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut winlink;
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
    winlinks_RB_INSERT_COLOR(head, elm);
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_FIND(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = winlink_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_NFIND(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut res: *mut winlink = 0 as *mut winlink;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = winlink_cmp(elm, tmp);
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
pub unsafe extern "C" fn winlinks_RB_NEXT(mut elm: *mut winlink) -> *mut winlink {
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
pub unsafe extern "C" fn winlinks_RB_MINMAX(
    mut head: *mut winlinks,
    mut val: libc::c_int,
) -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut parent: *mut winlink = 0 as *mut winlink;
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
pub unsafe extern "C" fn winlinks_RB_PREV(mut elm: *mut winlink) -> *mut winlink {
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
pub unsafe extern "C" fn window_pane_tree_RB_NEXT(
    mut elm: *mut window_pane,
) -> *mut window_pane {
    if !((*elm).tree_entry.rbe_right).is_null() {
        elm = (*elm).tree_entry.rbe_right;
        while !((*elm).tree_entry.rbe_left).is_null() {
            elm = (*elm).tree_entry.rbe_left;
        }
    } else if !((*elm).tree_entry.rbe_parent).is_null()
        && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_left
    {
        elm = (*elm).tree_entry.rbe_parent;
    } else {
        while !((*elm).tree_entry.rbe_parent).is_null()
            && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_right
        {
            elm = (*elm).tree_entry.rbe_parent;
        }
        elm = (*elm).tree_entry.rbe_parent;
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_REMOVE(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut current_block: u64;
    let mut child: *mut window_pane = 0 as *mut window_pane;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut old: *mut window_pane = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).tree_entry.rbe_left).is_null() {
        child = (*elm).tree_entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).tree_entry.rbe_right).is_null() {
        child = (*elm).tree_entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut window_pane = 0 as *mut window_pane;
        elm = (*elm).tree_entry.rbe_right;
        loop {
            left = (*elm).tree_entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).tree_entry.rbe_right;
        parent = (*elm).tree_entry.rbe_parent;
        color = (*elm).tree_entry.rbe_color;
        if !child.is_null() {
            (*child).tree_entry.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).tree_entry.rbe_left == elm {
                (*parent).tree_entry.rbe_left = child;
            } else {
                (*parent).tree_entry.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).tree_entry.rbe_parent == old {
            parent = elm;
        }
        (*elm).tree_entry = (*old).tree_entry;
        if !((*old).tree_entry.rbe_parent).is_null() {
            if (*(*old).tree_entry.rbe_parent).tree_entry.rbe_left == old {
                (*(*old).tree_entry.rbe_parent).tree_entry.rbe_left = elm;
            } else {
                (*(*old).tree_entry.rbe_parent).tree_entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).tree_entry.rbe_left).tree_entry.rbe_parent = elm;
        if !((*old).tree_entry.rbe_right).is_null() {
            (*(*old).tree_entry.rbe_right).tree_entry.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).tree_entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 12817450340325540726;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).tree_entry.rbe_parent;
            color = (*elm).tree_entry.rbe_color;
            if !child.is_null() {
                (*child).tree_entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).tree_entry.rbe_left == elm {
                    (*parent).tree_entry.rbe_left = child;
                } else {
                    (*parent).tree_entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        window_pane_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_INSERT_COLOR(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) {
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut gparent: *mut window_pane = 0 as *mut window_pane;
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    loop {
        parent = (*elm).tree_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).tree_entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).tree_entry.rbe_parent;
        if parent == (*gparent).tree_entry.rbe_left {
            tmp = (*gparent).tree_entry.rbe_right;
            if !tmp.is_null() && (*tmp).tree_entry.rbe_color == 1 as libc::c_int {
                (*tmp).tree_entry.rbe_color = 0 as libc::c_int;
                (*parent).tree_entry.rbe_color = 0 as libc::c_int;
                (*gparent).tree_entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).tree_entry.rbe_right == elm {
                    tmp = (*parent).tree_entry.rbe_right;
                    (*parent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                    if !((*parent).tree_entry.rbe_right).is_null() {
                        (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = parent;
                    }
                    (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                    if !((*tmp).tree_entry.rbe_parent).is_null() {
                        if parent
                            == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                        {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                        } else {
                            (*(*parent).tree_entry.rbe_parent)
                                .tree_entry
                                .rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).tree_entry.rbe_left = parent;
                    (*parent).tree_entry.rbe_parent = tmp;
                    !((*tmp).tree_entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).tree_entry.rbe_color = 0 as libc::c_int;
                (*gparent).tree_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).tree_entry.rbe_left;
                (*gparent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                if !((*gparent).tree_entry.rbe_left).is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = gparent;
                }
                (*tmp).tree_entry.rbe_parent = (*gparent).tree_entry.rbe_parent;
                if !((*tmp).tree_entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left
                    {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).tree_entry.rbe_right = gparent;
                (*gparent).tree_entry.rbe_parent = tmp;
                !((*tmp).tree_entry.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).tree_entry.rbe_left;
            if !tmp.is_null() && (*tmp).tree_entry.rbe_color == 1 as libc::c_int {
                (*tmp).tree_entry.rbe_color = 0 as libc::c_int;
                (*parent).tree_entry.rbe_color = 0 as libc::c_int;
                (*gparent).tree_entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).tree_entry.rbe_left == elm {
                    tmp = (*parent).tree_entry.rbe_left;
                    (*parent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                    if !((*parent).tree_entry.rbe_left).is_null() {
                        (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = parent;
                    }
                    (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                    if !((*tmp).tree_entry.rbe_parent).is_null() {
                        if parent
                            == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left
                        {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                        } else {
                            (*(*parent).tree_entry.rbe_parent)
                                .tree_entry
                                .rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).tree_entry.rbe_right = parent;
                    (*parent).tree_entry.rbe_parent = tmp;
                    !((*tmp).tree_entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).tree_entry.rbe_color = 0 as libc::c_int;
                (*gparent).tree_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).tree_entry.rbe_right;
                (*gparent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                if !((*gparent).tree_entry.rbe_right).is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = gparent;
                }
                (*tmp).tree_entry.rbe_parent = (*gparent).tree_entry.rbe_parent;
                if !((*tmp).tree_entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left
                    {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).tree_entry.rbe_left = gparent;
                (*gparent).tree_entry.rbe_parent = tmp;
                !((*tmp).tree_entry.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).tree_entry.rbe_color = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_REMOVE_COLOR(
    mut head: *mut window_pane_tree,
    mut parent: *mut window_pane,
    mut elm: *mut window_pane,
) {
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    while (elm.is_null() || (*elm).tree_entry.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).tree_entry.rbe_left == elm {
            tmp = (*parent).tree_entry.rbe_right;
            if (*tmp).tree_entry.rbe_color == 1 as libc::c_int {
                (*tmp).tree_entry.rbe_color = 0 as libc::c_int;
                (*parent).tree_entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).tree_entry.rbe_right;
                (*parent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                if !((*parent).tree_entry.rbe_right).is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = parent;
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !((*tmp).tree_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).tree_entry.rbe_left = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !((*tmp).tree_entry.rbe_parent).is_null();
                tmp = (*parent).tree_entry.rbe_right;
            }
            if (((*tmp).tree_entry.rbe_left).is_null()
                || (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color
                    == 0 as libc::c_int)
                && (((*tmp).tree_entry.rbe_right).is_null()
                    || (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color
                        == 0 as libc::c_int)
            {
                (*tmp).tree_entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).tree_entry.rbe_parent;
            } else {
                if ((*tmp).tree_entry.rbe_right).is_null()
                    || (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color
                        == 0 as libc::c_int
                {
                    let mut oleft: *mut window_pane = 0 as *mut window_pane;
                    oleft = (*tmp).tree_entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).tree_entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).tree_entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).tree_entry.rbe_left;
                    (*tmp).tree_entry.rbe_left = (*oleft).tree_entry.rbe_right;
                    if !((*tmp).tree_entry.rbe_left).is_null() {
                        (*(*oleft).tree_entry.rbe_right).tree_entry.rbe_parent = tmp;
                    }
                    (*oleft).tree_entry.rbe_parent = (*tmp).tree_entry.rbe_parent;
                    if !((*oleft).tree_entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left = oleft;
                        } else {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).tree_entry.rbe_right = tmp;
                    (*tmp).tree_entry.rbe_parent = oleft;
                    !((*oleft).tree_entry.rbe_parent).is_null();
                    tmp = (*parent).tree_entry.rbe_right;
                }
                (*tmp).tree_entry.rbe_color = (*parent).tree_entry.rbe_color;
                (*parent).tree_entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).tree_entry.rbe_right).is_null() {
                    (*(*tmp).tree_entry.rbe_right)
                        .tree_entry
                        .rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).tree_entry.rbe_right;
                (*parent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                if !((*parent).tree_entry.rbe_right).is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = parent;
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !((*tmp).tree_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).tree_entry.rbe_left = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !((*tmp).tree_entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).tree_entry.rbe_left;
            if (*tmp).tree_entry.rbe_color == 1 as libc::c_int {
                (*tmp).tree_entry.rbe_color = 0 as libc::c_int;
                (*parent).tree_entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).tree_entry.rbe_left;
                (*parent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                if !((*parent).tree_entry.rbe_left).is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = parent;
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !((*tmp).tree_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).tree_entry.rbe_right = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !((*tmp).tree_entry.rbe_parent).is_null();
                tmp = (*parent).tree_entry.rbe_left;
            }
            if (((*tmp).tree_entry.rbe_left).is_null()
                || (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color
                    == 0 as libc::c_int)
                && (((*tmp).tree_entry.rbe_right).is_null()
                    || (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color
                        == 0 as libc::c_int)
            {
                (*tmp).tree_entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).tree_entry.rbe_parent;
            } else {
                if ((*tmp).tree_entry.rbe_left).is_null()
                    || (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color
                        == 0 as libc::c_int
                {
                    let mut oright: *mut window_pane = 0 as *mut window_pane;
                    oright = (*tmp).tree_entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).tree_entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).tree_entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).tree_entry.rbe_right;
                    (*tmp).tree_entry.rbe_right = (*oright).tree_entry.rbe_left;
                    if !((*tmp).tree_entry.rbe_right).is_null() {
                        (*(*oright).tree_entry.rbe_left).tree_entry.rbe_parent = tmp;
                    }
                    (*oright).tree_entry.rbe_parent = (*tmp).tree_entry.rbe_parent;
                    if !((*oright).tree_entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left = oright;
                        } else {
                            (*(*tmp).tree_entry.rbe_parent)
                                .tree_entry
                                .rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).tree_entry.rbe_left = tmp;
                    (*tmp).tree_entry.rbe_parent = oright;
                    !((*oright).tree_entry.rbe_parent).is_null();
                    tmp = (*parent).tree_entry.rbe_left;
                }
                (*tmp).tree_entry.rbe_color = (*parent).tree_entry.rbe_color;
                (*parent).tree_entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).tree_entry.rbe_left).is_null() {
                    (*(*tmp).tree_entry.rbe_left)
                        .tree_entry
                        .rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).tree_entry.rbe_left;
                (*parent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                if !((*parent).tree_entry.rbe_left).is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = parent;
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !((*tmp).tree_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).tree_entry.rbe_right = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !((*tmp).tree_entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).tree_entry.rbe_color = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_INSERT(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = window_pane_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).tree_entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).tree_entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).tree_entry.rbe_parent = parent;
    (*elm).tree_entry.rbe_right = 0 as *mut window_pane;
    (*elm).tree_entry.rbe_left = (*elm).tree_entry.rbe_right;
    (*elm).tree_entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).tree_entry.rbe_left = elm;
        } else {
            (*parent).tree_entry.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    window_pane_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_FIND(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_pane_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).tree_entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).tree_entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_NFIND(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut res: *mut window_pane = 0 as *mut window_pane;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_pane_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            res = tmp;
            tmp = (*tmp).tree_entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).tree_entry.rbe_right;
        } else {
            return tmp
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_PREV(
    mut elm: *mut window_pane,
) -> *mut window_pane {
    if !((*elm).tree_entry.rbe_left).is_null() {
        elm = (*elm).tree_entry.rbe_left;
        while !((*elm).tree_entry.rbe_right).is_null() {
            elm = (*elm).tree_entry.rbe_right;
        }
    } else if !((*elm).tree_entry.rbe_parent).is_null()
        && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_right
    {
        elm = (*elm).tree_entry.rbe_parent;
    } else {
        while !((*elm).tree_entry.rbe_parent).is_null()
            && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_left
        {
            elm = (*elm).tree_entry.rbe_parent;
        }
        elm = (*elm).tree_entry.rbe_parent;
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_MINMAX(
    mut head: *mut window_pane_tree,
    mut val: libc::c_int,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).tree_entry.rbe_left;
        } else {
            tmp = (*tmp).tree_entry.rbe_right;
        }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn window_cmp(
    mut w1: *mut window,
    mut w2: *mut window,
) -> libc::c_int {
    return ((*w1).id).wrapping_sub((*w2).id) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_cmp(
    mut wl1: *mut winlink,
    mut wl2: *mut winlink,
) -> libc::c_int {
    return (*wl1).idx - (*wl2).idx;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_cmp(
    mut wp1: *mut window_pane,
    mut wp2: *mut window_pane,
) -> libc::c_int {
    return ((*wp1).id).wrapping_sub((*wp2).id) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_window(
    mut wwl: *mut winlinks,
    mut w: *mut window,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(wwl, -(1 as libc::c_int));
    while !wl.is_null() {
        if (*wl).window == w {
            return wl;
        }
        wl = winlinks_RB_NEXT(wl);
    }
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_index(
    mut wwl: *mut winlinks,
    mut idx: libc::c_int,
) -> *mut winlink {
    let mut wl: winlink = winlink {
        idx: 0,
        session: 0 as *mut session,
        window: 0 as *mut window,
        status_width: 0,
        status_cell: grid_cell {
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
        status_text: 0 as *mut libc::c_char,
        flags: 0,
        entry: C2RustUnnamed_13 {
            rbe_left: 0 as *mut winlink,
            rbe_right: 0 as *mut winlink,
            rbe_parent: 0 as *mut winlink,
            rbe_color: 0,
        },
        wentry: C2RustUnnamed_12 {
            tqe_next: 0 as *mut winlink,
            tqe_prev: 0 as *mut *mut winlink,
        },
        sentry: C2RustUnnamed_11 {
            tqe_next: 0 as *mut winlink,
            tqe_prev: 0 as *mut *mut winlink,
        },
    };
    if idx < 0 as libc::c_int {
        fatalx(b"bad index\0" as *const u8 as *const libc::c_char);
    }
    wl.idx = idx;
    return winlinks_RB_FIND(wwl, &mut wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_window_id(
    mut wwl: *mut winlinks,
    mut id: u_int,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(wwl, -(1 as libc::c_int));
    while !wl.is_null() {
        if (*(*wl).window).id == id {
            return wl;
        }
        wl = winlinks_RB_NEXT(wl);
    }
    return 0 as *mut winlink;
}
unsafe extern "C" fn winlink_next_index(
    mut wwl: *mut winlinks,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = idx;
    loop {
        if (winlink_find_by_index(wwl, i)).is_null() {
            return i;
        }
        if i == 2147483647 as libc::c_int {
            i = 0 as libc::c_int;
        } else {
            i += 1;
            i;
        }
        if !(i != idx) {
            break;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_count(mut wwl: *mut winlinks) -> u_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    wl = winlinks_RB_MINMAX(wwl, -(1 as libc::c_int));
    while !wl.is_null() {
        n = n.wrapping_add(1);
        n;
        wl = winlinks_RB_NEXT(wl);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_add(
    mut wwl: *mut winlinks,
    mut idx: libc::c_int,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if idx < 0 as libc::c_int {
        idx = winlink_next_index(wwl, -idx - 1 as libc::c_int);
        if idx == -(1 as libc::c_int) {
            return 0 as *mut winlink;
        }
    } else if !(winlink_find_by_index(wwl, idx)).is_null() {
        return 0 as *mut winlink
    }
    wl = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<winlink>() as libc::c_ulong,
    ) as *mut winlink;
    (*wl).idx = idx;
    winlinks_RB_INSERT(wwl, wl);
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_set_window(mut wl: *mut winlink, mut w: *mut window) {
    if !((*wl).window).is_null() {
        if !((*wl).wentry.tqe_next).is_null() {
            (*(*wl).wentry.tqe_next).wentry.tqe_prev = (*wl).wentry.tqe_prev;
        } else {
            (*(*wl).window).winlinks.tqh_last = (*wl).wentry.tqe_prev;
        }
        *(*wl).wentry.tqe_prev = (*wl).wentry.tqe_next;
        window_remove_ref(
            (*wl).window,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"winlink_set_window\0"))
                .as_ptr(),
        );
    }
    (*wl).wentry.tqe_next = 0 as *mut winlink;
    (*wl).wentry.tqe_prev = (*w).winlinks.tqh_last;
    *(*w).winlinks.tqh_last = wl;
    (*w).winlinks.tqh_last = &mut (*wl).wentry.tqe_next;
    (*wl).window = w;
    window_add_ref(
        w,
        (*::core::mem::transmute::<
            &[u8; 19],
            &[libc::c_char; 19],
        >(b"winlink_set_window\0"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn winlink_remove(mut wwl: *mut winlinks, mut wl: *mut winlink) {
    let mut w: *mut window = (*wl).window;
    if !w.is_null() {
        if !((*wl).wentry.tqe_next).is_null() {
            (*(*wl).wentry.tqe_next).wentry.tqe_prev = (*wl).wentry.tqe_prev;
        } else {
            (*w).winlinks.tqh_last = (*wl).wentry.tqe_prev;
        }
        *(*wl).wentry.tqe_prev = (*wl).wentry.tqe_next;
        window_remove_ref(
            w,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"winlink_remove\0"))
                .as_ptr(),
        );
    }
    winlinks_RB_REMOVE(wwl, wl);
    free((*wl).status_text as *mut libc::c_void);
    free(wl as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_next(mut wl: *mut winlink) -> *mut winlink {
    return winlinks_RB_NEXT(wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_previous(mut wl: *mut winlink) -> *mut winlink {
    return winlinks_RB_PREV(wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_next_by_number(
    mut wl: *mut winlink,
    mut s: *mut session,
    mut n: libc::c_int,
) -> *mut winlink {
    while n > 0 as libc::c_int {
        wl = winlinks_RB_NEXT(wl);
        if wl.is_null() {
            wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
        }
        n -= 1;
        n;
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_previous_by_number(
    mut wl: *mut winlink,
    mut s: *mut session,
    mut n: libc::c_int,
) -> *mut winlink {
    while n > 0 as libc::c_int {
        wl = winlinks_RB_PREV(wl);
        if wl.is_null() {
            wl = winlinks_RB_MINMAX(&mut (*s).windows, 1 as libc::c_int);
        }
        n -= 1;
        n;
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_stack_push(
    mut stack: *mut winlink_stack,
    mut wl: *mut winlink,
) {
    if wl.is_null() {
        return;
    }
    winlink_stack_remove(stack, wl);
    (*wl).sentry.tqe_next = (*stack).tqh_first;
    if !((*wl).sentry.tqe_next).is_null() {
        (*(*stack).tqh_first).sentry.tqe_prev = &mut (*wl).sentry.tqe_next;
    } else {
        (*stack).tqh_last = &mut (*wl).sentry.tqe_next;
    }
    (*stack).tqh_first = wl;
    (*wl).sentry.tqe_prev = &mut (*stack).tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_stack_remove(
    mut stack: *mut winlink_stack,
    mut wl: *mut winlink,
) {
    let mut wl2: *mut winlink = 0 as *mut winlink;
    if wl.is_null() {
        return;
    }
    wl2 = (*stack).tqh_first;
    while !wl2.is_null() {
        if wl2 == wl {
            if !((*wl).sentry.tqe_next).is_null() {
                (*(*wl).sentry.tqe_next).sentry.tqe_prev = (*wl).sentry.tqe_prev;
            } else {
                (*stack).tqh_last = (*wl).sentry.tqe_prev;
            }
            *(*wl).sentry.tqe_prev = (*wl).sentry.tqe_next;
            return;
        }
        wl2 = (*wl2).sentry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_find_by_id_str(
    mut s: *const libc::c_char,
) -> *mut window {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != '@' as i32 {
        return 0 as *mut window;
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
        return 0 as *mut window;
    }
    return window_find_by_id(id);
}
#[no_mangle]
pub unsafe extern "C" fn window_find_by_id(mut id: u_int) -> *mut window {
    let mut w: window = window {
        id: 0,
        name: 0 as *mut libc::c_char,
        name_event: event {
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
        name_time: timeval { tv_sec: 0, tv_usec: 0 },
        alerts_timer: event {
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
        activity_time: timeval { tv_sec: 0, tv_usec: 0 },
        active: 0 as *mut window_pane,
        last: 0 as *mut window_pane,
        panes: window_panes {
            tqh_first: 0 as *mut window_pane,
            tqh_last: 0 as *mut *mut window_pane,
        },
        lastlayout: 0,
        layout_root: 0 as *mut layout_cell,
        saved_layout_root: 0 as *mut layout_cell,
        old_layout: 0 as *mut libc::c_char,
        sx: 0,
        sy: 0,
        flags: 0,
        alerts_queued: 0,
        alerts_entry: C2RustUnnamed_16 {
            tqe_next: 0 as *mut window,
            tqe_prev: 0 as *mut *mut window,
        },
        options: 0 as *mut options,
        style: grid_cell {
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
        active_style: grid_cell {
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
        references: 0,
        winlinks: C2RustUnnamed_15 {
            tqh_first: 0 as *mut winlink,
            tqh_last: 0 as *mut *mut winlink,
        },
        entry: C2RustUnnamed_14 {
            rbe_left: 0 as *mut window,
            rbe_right: 0 as *mut window,
            rbe_parent: 0 as *mut window,
            rbe_color: 0,
        },
    };
    w.id = id;
    return windows_RB_FIND(&mut windows, &mut w);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_activity(mut w: *mut window) {
    gettimeofday(&mut (*w).activity_time, 0 as *mut libc::c_void);
    alerts_queue(w, 0x2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn window_create(mut sx: u_int, mut sy: u_int) -> *mut window {
    let mut w: *mut window = 0 as *mut window;
    w = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<window>() as libc::c_ulong,
    ) as *mut window;
    (*w).name = 0 as *mut libc::c_char;
    (*w).flags = 0x8000 as libc::c_int;
    (*w).panes.tqh_first = 0 as *mut window_pane;
    (*w).panes.tqh_last = &mut (*w).panes.tqh_first;
    (*w).active = 0 as *mut window_pane;
    (*w).lastlayout = -(1 as libc::c_int);
    (*w).layout_root = 0 as *mut layout_cell;
    (*w).sx = sx;
    (*w).sy = sy;
    (*w).options = options_create(global_w_options);
    (*w).references = 0 as libc::c_int as u_int;
    (*w).winlinks.tqh_first = 0 as *mut winlink;
    (*w).winlinks.tqh_last = &mut (*w).winlinks.tqh_first;
    let fresh0 = next_window_id;
    next_window_id = next_window_id.wrapping_add(1);
    (*w).id = fresh0;
    windows_RB_INSERT(&mut windows, w);
    window_update_activity(w);
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn window_create_spawn(
    mut name: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
    mut shell: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut env: *mut environ,
    mut tio: *mut termios,
    mut sx: u_int,
    mut sy: u_int,
    mut hlimit: u_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut window {
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    w = window_create(sx, sy);
    wp = window_add_pane(
        w,
        0 as *mut window_pane,
        0 as libc::c_int,
        0 as libc::c_int,
        hlimit,
    );
    layout_init(w, wp);
    if window_pane_spawn(wp, argc, argv, path, shell, cwd, env, tio, cause)
        != 0 as libc::c_int
    {
        window_destroy(w);
        return 0 as *mut window;
    }
    (*w).active = (*w).panes.tqh_first;
    if !name.is_null() {
        (*w).name = xstrdup(name);
        options_set_number(
            (*w).options,
            b"automatic-rename\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as libc::c_longlong,
        );
    } else {
        (*w).name = default_window_name(w);
    }
    notify_window(b"window-pane-changed\0" as *const u8 as *const libc::c_char, w);
    return w;
}
unsafe extern "C" fn window_destroy(mut w: *mut window) {
    log_debug(
        b"window @%u destroyed (%d references)\0" as *const u8 as *const libc::c_char,
        (*w).id,
        (*w).references,
    );
    windows_RB_REMOVE(&mut windows, w);
    if !((*w).layout_root).is_null() {
        layout_free_cell((*w).layout_root);
    }
    if !((*w).saved_layout_root).is_null() {
        layout_free_cell((*w).saved_layout_root);
    }
    free((*w).old_layout as *mut libc::c_void);
    if event_initialized(&mut (*w).name_event) != 0 {
        event_del(&mut (*w).name_event);
    }
    if event_initialized(&mut (*w).alerts_timer) != 0 {
        event_del(&mut (*w).alerts_timer);
    }
    options_free((*w).options);
    window_destroy_panes(w);
    free((*w).name as *mut libc::c_void);
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_destroy_ready(
    mut wp: *mut window_pane,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*wp).pipe_fd != -(1 as libc::c_int) {
        if evbuffer_get_length((*(*wp).pipe_event).output) != 0 as libc::c_int as size_t
        {
            return 0 as libc::c_int;
        }
        if ioctl(
            (*wp).fd,
            0x541b as libc::c_int as libc::c_ulong,
            &mut n as *mut libc::c_int,
        ) != -(1 as libc::c_int) && n > 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    if !(*wp).flags & 0x100 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_add_ref(
    mut w: *mut window,
    mut from: *const libc::c_char,
) {
    (*w).references = ((*w).references).wrapping_add(1);
    (*w).references;
    log_debug(
        b"%s: @%u %s, now %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"window_add_ref\0"))
            .as_ptr(),
        (*w).id,
        from,
        (*w).references,
    );
}
#[no_mangle]
pub unsafe extern "C" fn window_remove_ref(
    mut w: *mut window,
    mut from: *const libc::c_char,
) {
    (*w).references = ((*w).references).wrapping_sub(1);
    (*w).references;
    log_debug(
        b"%s: @%u %s, now %d\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"window_remove_ref\0"))
            .as_ptr(),
        (*w).id,
        from,
        (*w).references,
    );
    if (*w).references == 0 as libc::c_int as u_int {
        window_destroy(w);
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_set_name(
    mut w: *mut window,
    mut new_name: *const libc::c_char,
) {
    free((*w).name as *mut libc::c_void);
    utf8_stravis(
        &mut (*w).name,
        new_name,
        0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int,
    );
    notify_window(b"window-renamed\0" as *const u8 as *const libc::c_char, w);
}
#[no_mangle]
pub unsafe extern "C" fn window_resize(
    mut w: *mut window,
    mut sx: u_int,
    mut sy: u_int,
) {
    (*w).sx = sx;
    (*w).sy = sy;
}
#[no_mangle]
pub unsafe extern "C" fn window_has_pane(
    mut w: *mut window,
    mut wp: *mut window_pane,
) -> libc::c_int {
    let mut wp1: *mut window_pane = 0 as *mut window_pane;
    wp1 = (*w).panes.tqh_first;
    while !wp1.is_null() {
        if wp1 == wp {
            return 1 as libc::c_int;
        }
        wp1 = (*wp1).entry.tqe_next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_set_active_pane(
    mut w: *mut window,
    mut wp: *mut window_pane,
) -> libc::c_int {
    log_debug(
        b"%s: pane %%%u (was %%%u)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"window_set_active_pane\0"))
            .as_ptr(),
        (*wp).id,
        (*(*w).active).id,
    );
    if wp == (*w).active {
        return 0 as libc::c_int;
    }
    (*w).last = (*w).active;
    (*w).active = wp;
    while window_pane_visible((*w).active) == 0 {
        (*w).active = *(*((*(*w).active).entry.tqe_prev as *mut window_panes)).tqh_last;
        if ((*w).active).is_null() {
            (*w).active = *(*((*w).panes.tqh_last as *mut window_panes)).tqh_last;
        }
        if (*w).active == wp {
            notify_window(
                b"window-pane-changed\0" as *const u8 as *const libc::c_char,
                w,
            );
            return 1 as libc::c_int;
        }
    }
    let fresh1 = next_active_point;
    next_active_point = next_active_point.wrapping_add(1);
    (*(*w).active).active_point = fresh1;
    (*(*w).active).flags |= 0x80 as libc::c_int;
    notify_window(b"window-pane-changed\0" as *const u8 as *const libc::c_char, w);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_redraw_active_switch(
    mut w: *mut window,
    mut wp: *mut window_pane,
) {
    let mut gc: *const grid_cell = 0 as *const grid_cell;
    if wp == (*w).active {
        return;
    }
    gc = options_get_style(
        (*w).options,
        b"window-active-style\0" as *const u8 as *const libc::c_char,
    );
    if style_equal(
        gc,
        options_get_style(
            (*w).options,
            b"window-style\0" as *const u8 as *const libc::c_char,
        ),
    ) != 0
    {
        return;
    }
    if window_pane_get_palette((*w).active, (*(*w).active).colgc.fg)
        != -(1 as libc::c_int)
        || window_pane_get_palette((*w).active, (*(*w).active).colgc.bg)
            != -(1 as libc::c_int)
        || style_equal(&grid_default_cell, &mut (*(*w).active).colgc) != 0
    {
        (*(*w).active).flags |= 0x1 as libc::c_int;
    }
    if window_pane_get_palette(wp, (*wp).colgc.fg) != -(1 as libc::c_int)
        || window_pane_get_palette(wp, (*wp).colgc.bg) != -(1 as libc::c_int)
        || style_equal(&grid_default_cell, &mut (*wp).colgc) != 0
    {
        (*wp).flags |= 0x1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_get_active_at(
    mut w: *mut window,
    mut x: u_int,
    mut y: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            if !(x < (*wp).xoff || x > ((*wp).xoff).wrapping_add((*wp).sx)) {
                if !(y < (*wp).yoff || y > ((*wp).yoff).wrapping_add((*wp).sy)) {
                    return wp;
                }
            }
        }
        wp = (*wp).entry.tqe_next;
    }
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_find_string(
    mut w: *mut window,
    mut s: *const libc::c_char,
) -> *mut window_pane {
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    x = (*w).sx / 2 as libc::c_int as u_int;
    y = (*w).sy / 2 as libc::c_int as u_int;
    if strcasecmp(s, b"top\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        y = 0 as libc::c_int as u_int;
    } else if strcasecmp(s, b"bottom\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        y = ((*w).sy).wrapping_sub(1 as libc::c_int as u_int);
    } else if strcasecmp(s, b"left\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x = 0 as libc::c_int as u_int;
    } else if strcasecmp(s, b"right\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x = ((*w).sx).wrapping_sub(1 as libc::c_int as u_int);
    } else if strcasecmp(s, b"top-left\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x = 0 as libc::c_int as u_int;
        y = 0 as libc::c_int as u_int;
    } else if strcasecmp(s, b"top-right\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x = ((*w).sx).wrapping_sub(1 as libc::c_int as u_int);
        y = 0 as libc::c_int as u_int;
    } else if strcasecmp(s, b"bottom-left\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x = 0 as libc::c_int as u_int;
        y = ((*w).sy).wrapping_sub(1 as libc::c_int as u_int);
    } else if strcasecmp(s, b"bottom-right\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        x = ((*w).sx).wrapping_sub(1 as libc::c_int as u_int);
        y = ((*w).sy).wrapping_sub(1 as libc::c_int as u_int);
    } else {
        return 0 as *mut window_pane
    }
    return window_get_active_at(w, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn window_zoom(mut wp: *mut window_pane) -> libc::c_int {
    let mut w: *mut window = (*wp).window;
    let mut wp1: *mut window_pane = 0 as *mut window_pane;
    if (*w).flags & 0x1000 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    if window_pane_visible(wp) == 0 {
        return -(1 as libc::c_int);
    }
    if window_count_panes(w) == 1 as libc::c_int as u_int {
        return -(1 as libc::c_int);
    }
    if (*w).active != wp {
        window_set_active_pane(w, wp);
    }
    wp1 = (*w).panes.tqh_first;
    while !wp1.is_null() {
        (*wp1).saved_layout_cell = (*wp1).layout_cell;
        (*wp1).layout_cell = 0 as *mut layout_cell;
        wp1 = (*wp1).entry.tqe_next;
    }
    (*w).saved_layout_root = (*w).layout_root;
    layout_init(w, wp);
    (*w).flags |= 0x1000 as libc::c_int;
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_unzoom(mut w: *mut window) -> libc::c_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if (*w).flags & 0x1000 as libc::c_int == 0 {
        return -(1 as libc::c_int);
    }
    (*w).flags &= !(0x1000 as libc::c_int);
    layout_free(w);
    (*w).layout_root = (*w).saved_layout_root;
    (*w).saved_layout_root = 0 as *mut layout_cell;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        (*wp).layout_cell = (*wp).saved_layout_cell;
        (*wp).saved_layout_cell = 0 as *mut layout_cell;
        wp = (*wp).entry.tqe_next;
    }
    layout_fix_panes(w, (*w).sx, (*w).sy);
    notify_window(b"window-layout-changed\0" as *const u8 as *const libc::c_char, w);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_add_pane(
    mut w: *mut window,
    mut other: *mut window_pane,
    mut before: libc::c_int,
    mut full_size: libc::c_int,
    mut hlimit: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if other.is_null() {
        other = (*w).active;
    }
    wp = window_pane_create(w, (*w).sx, (*w).sy, hlimit);
    if ((*w).panes.tqh_first).is_null() {
        log_debug(
            b"%s: @%u at start\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"window_add_pane\0"))
                .as_ptr(),
            (*w).id,
        );
        (*wp).entry.tqe_next = (*w).panes.tqh_first;
        if !((*wp).entry.tqe_next).is_null() {
            (*(*w).panes.tqh_first).entry.tqe_prev = &mut (*wp).entry.tqe_next;
        } else {
            (*w).panes.tqh_last = &mut (*wp).entry.tqe_next;
        }
        (*w).panes.tqh_first = wp;
        (*wp).entry.tqe_prev = &mut (*w).panes.tqh_first;
    } else if before != 0 {
        log_debug(
            b"%s: @%u before %%%u\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"window_add_pane\0"))
                .as_ptr(),
            (*w).id,
            (*wp).id,
        );
        if full_size != 0 {
            (*wp).entry.tqe_next = (*w).panes.tqh_first;
            if !((*wp).entry.tqe_next).is_null() {
                (*(*w).panes.tqh_first).entry.tqe_prev = &mut (*wp).entry.tqe_next;
            } else {
                (*w).panes.tqh_last = &mut (*wp).entry.tqe_next;
            }
            (*w).panes.tqh_first = wp;
            (*wp).entry.tqe_prev = &mut (*w).panes.tqh_first;
        } else {
            (*wp).entry.tqe_prev = (*other).entry.tqe_prev;
            (*wp).entry.tqe_next = other;
            *(*other).entry.tqe_prev = wp;
            (*other).entry.tqe_prev = &mut (*wp).entry.tqe_next;
        }
    } else {
        log_debug(
            b"%s: @%u after %%%u\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"window_add_pane\0"))
                .as_ptr(),
            (*w).id,
            (*wp).id,
        );
        if full_size != 0 {
            (*wp).entry.tqe_next = 0 as *mut window_pane;
            (*wp).entry.tqe_prev = (*w).panes.tqh_last;
            *(*w).panes.tqh_last = wp;
            (*w).panes.tqh_last = &mut (*wp).entry.tqe_next;
        } else {
            (*wp).entry.tqe_next = (*other).entry.tqe_next;
            if !((*wp).entry.tqe_next).is_null() {
                (*(*wp).entry.tqe_next).entry.tqe_prev = &mut (*wp).entry.tqe_next;
            } else {
                (*w).panes.tqh_last = &mut (*wp).entry.tqe_next;
            }
            (*other).entry.tqe_next = wp;
            (*wp).entry.tqe_prev = &mut (*other).entry.tqe_next;
        }
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_lost_pane(mut w: *mut window, mut wp: *mut window_pane) {
    log_debug(
        b"%s: @%u pane %%%u\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"window_lost_pane\0"))
            .as_ptr(),
        (*w).id,
        (*wp).id,
    );
    if wp == marked_pane.wp {
        server_clear_marked();
    }
    if wp == (*w).active {
        (*w).active = (*w).last;
        (*w).last = 0 as *mut window_pane;
        if ((*w).active).is_null() {
            (*w).active = *(*((*wp).entry.tqe_prev as *mut window_panes)).tqh_last;
            if ((*w).active).is_null() {
                (*w).active = (*wp).entry.tqe_next;
            }
        }
        if !((*w).active).is_null() {
            (*(*w).active).flags |= 0x80 as libc::c_int;
            notify_window(
                b"window-pane-changed\0" as *const u8 as *const libc::c_char,
                w,
            );
        }
    } else if wp == (*w).last {
        (*w).last = 0 as *mut window_pane;
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_remove_pane(
    mut w: *mut window,
    mut wp: *mut window_pane,
) {
    window_lost_pane(w, wp);
    if !((*wp).entry.tqe_next).is_null() {
        (*(*wp).entry.tqe_next).entry.tqe_prev = (*wp).entry.tqe_prev;
    } else {
        (*w).panes.tqh_last = (*wp).entry.tqe_prev;
    }
    *(*wp).entry.tqe_prev = (*wp).entry.tqe_next;
    window_pane_destroy(wp);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_at_index(
    mut w: *mut window,
    mut idx: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    n = options_get_number(
        (*w).options,
        b"pane-base-index\0" as *const u8 as *const libc::c_char,
    ) as u_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if n == idx {
            return wp;
        }
        n = n.wrapping_add(1);
        n;
        wp = (*wp).entry.tqe_next;
    }
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_next_by_number(
    mut w: *mut window,
    mut wp: *mut window_pane,
    mut n: u_int,
) -> *mut window_pane {
    while n > 0 as libc::c_int as u_int {
        wp = (*wp).entry.tqe_next;
        if wp.is_null() {
            wp = (*w).panes.tqh_first;
        }
        n = n.wrapping_sub(1);
        n;
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_previous_by_number(
    mut w: *mut window,
    mut wp: *mut window_pane,
    mut n: u_int,
) -> *mut window_pane {
    while n > 0 as libc::c_int as u_int {
        wp = *(*((*wp).entry.tqe_prev as *mut window_panes)).tqh_last;
        if wp.is_null() {
            wp = *(*((*w).panes.tqh_last as *mut window_panes)).tqh_last;
        }
        n = n.wrapping_sub(1);
        n;
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_index(
    mut wp: *mut window_pane,
    mut i: *mut u_int,
) -> libc::c_int {
    let mut wq: *mut window_pane = 0 as *mut window_pane;
    let mut w: *mut window = (*wp).window;
    *i = options_get_number(
        (*w).options,
        b"pane-base-index\0" as *const u8 as *const libc::c_char,
    ) as u_int;
    wq = (*w).panes.tqh_first;
    while !wq.is_null() {
        if wp == wq {
            return 0 as libc::c_int;
        }
        *i = (*i).wrapping_add(1);
        *i;
        wq = (*wq).entry.tqe_next;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn window_count_panes(mut w: *mut window) -> u_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        n = n.wrapping_add(1);
        n;
        wp = (*wp).entry.tqe_next;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn window_destroy_panes(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    while !((*w).panes.tqh_first).is_null() {
        wp = (*w).panes.tqh_first;
        if !((*wp).entry.tqe_next).is_null() {
            (*(*wp).entry.tqe_next).entry.tqe_prev = (*wp).entry.tqe_prev;
        } else {
            (*w).panes.tqh_last = (*wp).entry.tqe_prev;
        }
        *(*wp).entry.tqe_prev = (*wp).entry.tqe_next;
        window_pane_destroy(wp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_printable_flags(
    mut wl: *mut winlink,
) -> *const libc::c_char {
    let mut s: *mut session = (*wl).session;
    static mut flags: [libc::c_char; 32] = [0; 32];
    let mut pos: libc::c_int = 0;
    pos = 0 as libc::c_int;
    if (*wl).flags & 0x2 as libc::c_int != 0 {
        let fresh2 = pos;
        pos = pos + 1;
        flags[fresh2 as usize] = '#' as i32 as libc::c_char;
    }
    if (*wl).flags & 0x1 as libc::c_int != 0 {
        let fresh3 = pos;
        pos = pos + 1;
        flags[fresh3 as usize] = '!' as i32 as libc::c_char;
    }
    if (*wl).flags & 0x4 as libc::c_int != 0 {
        let fresh4 = pos;
        pos = pos + 1;
        flags[fresh4 as usize] = '~' as i32 as libc::c_char;
    }
    if wl == (*s).curw {
        let fresh5 = pos;
        pos = pos + 1;
        flags[fresh5 as usize] = '*' as i32 as libc::c_char;
    }
    if wl == (*s).lastw.tqh_first {
        let fresh6 = pos;
        pos = pos + 1;
        flags[fresh6 as usize] = '-' as i32 as libc::c_char;
    }
    if server_check_marked() != 0 && wl == marked_pane.wl {
        let fresh7 = pos;
        pos = pos + 1;
        flags[fresh7 as usize] = 'M' as i32 as libc::c_char;
    }
    if (*(*wl).window).flags & 0x1000 as libc::c_int != 0 {
        let fresh8 = pos;
        pos = pos + 1;
        flags[fresh8 as usize] = 'Z' as i32 as libc::c_char;
    }
    flags[pos as usize] = '\0' as i32 as libc::c_char;
    return flags.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_by_id_str(
    mut s: *const libc::c_char,
) -> *mut window_pane {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != '%' as i32 {
        return 0 as *mut window_pane;
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
        return 0 as *mut window_pane;
    }
    return window_pane_find_by_id(id);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_by_id(mut id: u_int) -> *mut window_pane {
    let mut wp: window_pane = window_pane {
        id: 0,
        active_point: 0,
        window: 0 as *mut window,
        layout_cell: 0 as *mut layout_cell,
        saved_layout_cell: 0 as *mut layout_cell,
        sx: 0,
        sy: 0,
        osx: 0,
        osy: 0,
        xoff: 0,
        yoff: 0,
        flags: 0,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        shell: 0 as *mut libc::c_char,
        cwd: 0 as *const libc::c_char,
        pid: 0,
        tty: [0; 32],
        status: 0,
        fd: 0,
        event: 0 as *mut bufferevent,
        resize_timer: event {
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
        ictx: 0 as *mut input_ctx,
        colgc: grid_cell {
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
        palette: 0 as *mut libc::c_int,
        pipe_fd: 0,
        pipe_event: 0 as *mut bufferevent,
        pipe_off: 0,
        screen: 0 as *mut screen,
        base: screen {
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
        },
        status_screen: screen {
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
        },
        status_size: 0,
        saved_cx: 0,
        saved_cy: 0,
        saved_grid: 0 as *mut grid,
        saved_cell: grid_cell {
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
        mode: 0 as *const window_mode,
        modedata: 0 as *mut libc::c_void,
        modetimer: event {
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
        modelast: 0,
        modeprefix: 0,
        searchstr: 0 as *mut libc::c_char,
        entry: C2RustUnnamed_19 {
            tqe_next: 0 as *mut window_pane,
            tqe_prev: 0 as *mut *mut window_pane,
        },
        tree_entry: C2RustUnnamed_18 {
            rbe_left: 0 as *mut window_pane,
            rbe_right: 0 as *mut window_pane,
            rbe_parent: 0 as *mut window_pane,
            rbe_color: 0,
        },
    };
    wp.id = id;
    return window_pane_tree_RB_FIND(&mut all_window_panes, &mut wp);
}
unsafe extern "C" fn window_pane_create(
    mut w: *mut window,
    mut sx: u_int,
    mut sy: u_int,
    mut hlimit: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut host: [libc::c_char; 65] = [0; 65];
    wp = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<window_pane>() as libc::c_ulong,
    ) as *mut window_pane;
    (*wp).window = w;
    let fresh9 = next_window_pane_id;
    next_window_pane_id = next_window_pane_id.wrapping_add(1);
    (*wp).id = fresh9;
    window_pane_tree_RB_INSERT(&mut all_window_panes, wp);
    (*wp).argc = 0 as libc::c_int;
    (*wp).argv = 0 as *mut *mut libc::c_char;
    (*wp).shell = 0 as *mut libc::c_char;
    (*wp).cwd = 0 as *const libc::c_char;
    (*wp).fd = -(1 as libc::c_int);
    (*wp).event = 0 as *mut bufferevent;
    (*wp).mode = 0 as *const window_mode;
    (*wp).modeprefix = 1 as libc::c_int as u_int;
    (*wp).layout_cell = 0 as *mut layout_cell;
    (*wp).xoff = 0 as libc::c_int as u_int;
    (*wp).yoff = 0 as libc::c_int as u_int;
    (*wp).osx = sx;
    (*wp).sx = (*wp).osx;
    (*wp).osx = sy;
    (*wp).sy = (*wp).osx;
    (*wp).pipe_fd = -(1 as libc::c_int);
    (*wp).pipe_off = 0 as libc::c_int as size_t;
    (*wp).pipe_event = 0 as *mut bufferevent;
    (*wp).saved_grid = 0 as *mut grid;
    memcpy(
        &mut (*wp).colgc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    screen_init(&mut (*wp).base, sx, sy, hlimit);
    (*wp).screen = &mut (*wp).base;
    screen_init(
        &mut (*wp).status_screen,
        1 as libc::c_int as u_int,
        1 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    if gethostname(
        host.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        screen_set_title(&mut (*wp).base, host.as_mut_ptr());
    }
    input_init(wp);
    return wp;
}
unsafe extern "C" fn window_pane_destroy(mut wp: *mut window_pane) {
    window_pane_reset_mode(wp);
    free((*wp).searchstr as *mut libc::c_void);
    if (*wp).fd != -(1 as libc::c_int) {
        bufferevent_free((*wp).event);
        close((*wp).fd);
    }
    input_free(wp);
    screen_free(&mut (*wp).base);
    if !((*wp).saved_grid).is_null() {
        grid_destroy((*wp).saved_grid);
    }
    if (*wp).pipe_fd != -(1 as libc::c_int) {
        bufferevent_free((*wp).pipe_event);
        close((*wp).pipe_fd);
    }
    if event_initialized(&mut (*wp).resize_timer) != 0 {
        event_del(&mut (*wp).resize_timer);
    }
    window_pane_tree_RB_REMOVE(&mut all_window_panes, wp);
    free((*wp).cwd as *mut libc::c_void);
    free((*wp).shell as *mut libc::c_void);
    cmd_free_argv((*wp).argc, (*wp).argv);
    free((*wp).palette as *mut libc::c_void);
    free(wp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_spawn(
    mut wp: *mut window_pane,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
    mut shell: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut env: *mut environ,
    mut tio: *mut termios,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut argv0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argvp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut first: *const libc::c_char = 0 as *const libc::c_char;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut tio2: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut i: libc::c_int = 0;
    let mut set: sigset_t = __sigset_t { __val: [0; 16] };
    let mut oldset: sigset_t = __sigset_t { __val: [0; 16] };
    if (*wp).fd != -(1 as libc::c_int) {
        bufferevent_free((*wp).event);
        close((*wp).fd);
    }
    if argc > 0 as libc::c_int {
        cmd_free_argv((*wp).argc, (*wp).argv);
        (*wp).argc = argc;
        (*wp).argv = cmd_copy_argv(argc, argv);
    }
    if !shell.is_null() {
        free((*wp).shell as *mut libc::c_void);
        (*wp).shell = xstrdup(shell);
    }
    if !cwd.is_null() {
        free((*wp).cwd as *mut libc::c_void);
        (*wp).cwd = xstrdup(cwd);
    }
    (*wp).flags &= !(0x200 as libc::c_int | 0x400 as libc::c_int);
    cmd = cmd_stringify_argv((*wp).argc, (*wp).argv);
    log_debug(
        b"spawn: %s -- %s\0" as *const u8 as *const libc::c_char,
        (*wp).shell,
        cmd,
    );
    i = 0 as libc::c_int;
    while i < (*wp).argc {
        log_debug(
            b"spawn: argv[%d] = %s\0" as *const u8 as *const libc::c_char,
            i,
            *((*wp).argv).offset(i as isize),
        );
        i += 1;
        i;
    }
    environ_log(env, b"spawn: \0" as *const u8 as *const libc::c_char);
    memset(
        &mut ws as *mut winsize as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<winsize>() as libc::c_ulong,
    );
    ws.ws_col = (*(*wp).base.grid).sx as libc::c_ushort;
    ws.ws_row = (*(*wp).base.grid).sy as libc::c_ushort;
    sigfillset(&mut set);
    sigprocmask(0 as libc::c_int, &mut set, &mut oldset);
    (*wp)
        .pid = fdforkpty(
        ptm_fd,
        &mut (*wp).fd,
        ((*wp).tty).as_mut_ptr(),
        0 as *mut termios,
        &mut ws,
    );
    match (*wp).pid {
        -1 => {
            (*wp).fd = -(1 as libc::c_int);
            xasprintf(
                cause,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                cmd,
                strerror(*__errno_location()),
            );
            free(cmd as *mut libc::c_void);
            sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
            return -(1 as libc::c_int);
        }
        0 => {
            proc_clear_signals(server_proc, 1 as libc::c_int);
            sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
            cwd = 0 as *const libc::c_char;
            if chdir((*wp).cwd) == 0 as libc::c_int {
                cwd = (*wp).cwd;
            } else {
                home = find_home();
                if !home.is_null() && chdir(home) == 0 as libc::c_int {
                    cwd = home;
                } else {
                    chdir(b"/\0" as *const u8 as *const libc::c_char);
                }
            }
            if tcgetattr(0 as libc::c_int, &mut tio2) != 0 as libc::c_int {
                fatal(b"tcgetattr failed\0" as *const u8 as *const libc::c_char);
            }
            if !tio.is_null() {
                memcpy(
                    (tio2.c_cc).as_mut_ptr() as *mut libc::c_void,
                    ((*tio).c_cc).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[cc_t; 32]>() as libc::c_ulong,
                );
            }
            tio2.c_cc[2 as libc::c_int as usize] = '\u{7f}' as i32 as cc_t;
            tio2.c_iflag |= 0o40000 as libc::c_int as tcflag_t;
            if tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut tio2)
                != 0 as libc::c_int
            {
                fatal(b"tcgetattr failed\0" as *const u8 as *const libc::c_char);
            }
            log_close();
            closefrom(2 as libc::c_int + 1 as libc::c_int);
            if !path.is_null() {
                environ_set(
                    env,
                    b"PATH\0" as *const u8 as *const libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    path,
                );
            }
            if !cwd.is_null() {
                environ_set(
                    env,
                    b"PWD\0" as *const u8 as *const libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    cwd,
                );
            }
            environ_set(
                env,
                b"TMUX_PANE\0" as *const u8 as *const libc::c_char,
                b"%%%u\0" as *const u8 as *const libc::c_char,
                (*wp).id,
            );
            environ_push(env);
            setenv(
                b"SHELL\0" as *const u8 as *const libc::c_char,
                (*wp).shell,
                1 as libc::c_int,
            );
            ptr = strrchr((*wp).shell, '/' as i32);
            if (*wp).argc > 0 as libc::c_int {
                if (*wp).argc != 1 as libc::c_int {
                    argvp = cmd_copy_argv((*wp).argc, (*wp).argv);
                    execvp(
                        *argvp.offset(0 as libc::c_int as isize),
                        argvp as *const *mut libc::c_char,
                    );
                    fatal(b"execvp failed\0" as *const u8 as *const libc::c_char);
                }
                first = *((*wp).argv).offset(0 as libc::c_int as isize);
                if !ptr.is_null()
                    && *ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    xasprintf(
                        &mut argv0 as *mut *mut libc::c_char,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        ptr.offset(1 as libc::c_int as isize),
                    );
                } else {
                    xasprintf(
                        &mut argv0 as *mut *mut libc::c_char,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*wp).shell,
                    );
                }
                execl(
                    (*wp).shell,
                    argv0,
                    b"-c\0" as *const u8 as *const libc::c_char,
                    first,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                fatal(b"execl failed\0" as *const u8 as *const libc::c_char);
            }
            if !ptr.is_null()
                && *ptr.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
            {
                xasprintf(
                    &mut argv0 as *mut *mut libc::c_char,
                    b"-%s\0" as *const u8 as *const libc::c_char,
                    ptr.offset(1 as libc::c_int as isize),
                );
            } else {
                xasprintf(
                    &mut argv0 as *mut *mut libc::c_char,
                    b"-%s\0" as *const u8 as *const libc::c_char,
                    (*wp).shell,
                );
            }
            execl((*wp).shell, argv0, 0 as *mut libc::c_void as *mut libc::c_char);
            fatal(b"execl failed\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    setblocking((*wp).fd, 0 as libc::c_int);
    (*wp)
        .event = bufferevent_new(
        (*wp).fd,
        Some(
            window_pane_read_callback
                as unsafe extern "C" fn(*mut bufferevent, *mut libc::c_void) -> (),
        ),
        None,
        Some(
            window_pane_error_callback
                as unsafe extern "C" fn(
                    *mut bufferevent,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        wp as *mut libc::c_void,
    );
    bufferevent_setwatermark(
        (*wp).event,
        0x2 as libc::c_int as libc::c_short,
        0 as libc::c_int as size_t,
        4096 as libc::c_int as size_t,
    );
    bufferevent_enable(
        (*wp).event,
        (0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_short,
    );
    free(cmd as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_pane_read_callback(
    mut bufev: *mut bufferevent,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    let mut evb: *mut evbuffer = (*(*wp).event).input;
    let mut size: size_t = evbuffer_get_length(evb);
    let mut new_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_size: size_t = 0;
    new_size = size.wrapping_sub((*wp).pipe_off);
    if (*wp).pipe_fd != -(1 as libc::c_int) && new_size > 0 as libc::c_int as size_t {
        new_data = (evbuffer_pullup(evb, -(1 as libc::c_int) as ssize_t))
            .offset((*wp).pipe_off as isize) as *mut libc::c_char;
        bufferevent_write((*wp).pipe_event, new_data as *const libc::c_void, new_size);
    }
    log_debug(
        b"%%%u has %zu bytes\0" as *const u8 as *const libc::c_char,
        (*wp).id,
        size,
    );
    input_parse(wp);
    (*wp).pipe_off = evbuffer_get_length(evb);
}
unsafe extern "C" fn window_pane_error_callback(
    mut bufev: *mut bufferevent,
    mut what: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    log_debug(b"%%%u error\0" as *const u8 as *const libc::c_char, (*wp).id);
    (*wp).flags |= 0x100 as libc::c_int;
    if window_pane_destroy_ready(wp) != 0 {
        server_destroy_pane(wp, 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_resize(
    mut wp: *mut window_pane,
    mut sx: u_int,
    mut sy: u_int,
) {
    if sx == (*wp).sx && sy == (*wp).sy {
        return;
    }
    (*wp).sx = sx;
    (*wp).sy = sy;
    screen_resize(
        &mut (*wp).base,
        sx,
        sy,
        ((*wp).saved_grid == 0 as *mut libc::c_void as *mut grid) as libc::c_int,
    );
    if !((*wp).mode).is_null() {
        ((*(*wp).mode).resize).expect("non-null function pointer")(wp, sx, sy);
    }
    (*wp).flags |= 0x8 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_alternate_on(
    mut wp: *mut window_pane,
    mut gc: *mut grid_cell,
    mut cursor: libc::c_int,
) {
    let mut s: *mut screen = &mut (*wp).base;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if !((*wp).saved_grid).is_null() {
        return;
    }
    if options_get_number(
        (*(*wp).window).options,
        b"alternate-screen\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return;
    }
    sx = (*(*s).grid).sx;
    sy = (*(*s).grid).sy;
    (*wp).saved_grid = grid_create(sx, sy, 0 as libc::c_int as u_int);
    grid_duplicate_lines(
        (*wp).saved_grid,
        0 as libc::c_int as u_int,
        (*s).grid,
        (*(*s).grid).hsize,
        sy,
    );
    if cursor != 0 {
        (*wp).saved_cx = (*s).cx;
        (*wp).saved_cy = (*s).cy;
    }
    memcpy(
        &mut (*wp).saved_cell as *mut grid_cell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    grid_view_clear(
        (*s).grid,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
        sx,
        sy,
        8 as libc::c_int as u_int,
    );
    (*(*wp).base.grid).flags &= !(0x1 as libc::c_int);
    (*wp).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_alternate_off(
    mut wp: *mut window_pane,
    mut gc: *mut grid_cell,
    mut cursor: libc::c_int,
) {
    let mut s: *mut screen = &mut (*wp).base;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if ((*wp).saved_grid).is_null() {
        return;
    }
    if options_get_number(
        (*(*wp).window).options,
        b"alternate-screen\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return;
    }
    sx = (*(*s).grid).sx;
    sy = (*(*s).grid).sy;
    if sy > (*(*wp).saved_grid).sy {
        screen_resize(s, sx, (*(*wp).saved_grid).sy, 1 as libc::c_int);
    }
    grid_duplicate_lines(
        (*s).grid,
        (*(*s).grid).hsize,
        (*wp).saved_grid,
        0 as libc::c_int as u_int,
        sy,
    );
    if cursor != 0 {
        (*s).cx = (*wp).saved_cx;
    }
    if (*s).cx > ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int) {
        (*s).cx = ((*(*s).grid).sx).wrapping_sub(1 as libc::c_int as u_int);
    }
    if cursor != 0 {
        (*s).cy = (*wp).saved_cy;
    }
    if (*s).cy > ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int) {
        (*s).cy = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    }
    memcpy(
        gc as *mut libc::c_void,
        &mut (*wp).saved_cell as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    (*(*wp).base.grid).flags |= 0x1 as libc::c_int;
    if sy > (*(*wp).saved_grid).sy || sx != (*(*wp).saved_grid).sx {
        screen_resize(s, sx, sy, 1 as libc::c_int);
    }
    grid_destroy((*wp).saved_grid);
    (*wp).saved_grid = 0 as *mut grid;
    (*wp).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_palette(
    mut wp: *mut window_pane,
    mut n: u_int,
    mut colour: libc::c_int,
) {
    if n > 0xff as libc::c_int as u_int {
        return;
    }
    if ((*wp).palette).is_null() {
        (*wp)
            .palette = xcalloc(
            0x100 as libc::c_int as size_t,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
    }
    *((*wp).palette).offset(n as isize) = colour;
    (*wp).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_unset_palette(
    mut wp: *mut window_pane,
    mut n: u_int,
) {
    if n > 0xff as libc::c_int as u_int || ((*wp).palette).is_null() {
        return;
    }
    *((*wp).palette).offset(n as isize) = 0 as libc::c_int;
    (*wp).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_palette(mut wp: *mut window_pane) {
    if ((*wp).palette).is_null() {
        return;
    }
    free((*wp).palette as *mut libc::c_void);
    (*wp).palette = 0 as *mut libc::c_int;
    (*wp).flags |= 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_get_palette(
    mut wp: *const window_pane,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut new: libc::c_int = 0;
    if wp.is_null() || ((*wp).palette).is_null() {
        return -(1 as libc::c_int);
    }
    new = -(1 as libc::c_int);
    if c < 8 as libc::c_int {
        new = *((*wp).palette).offset(c as isize);
    } else if c >= 90 as libc::c_int && c <= 97 as libc::c_int {
        new = *((*wp).palette)
            .offset((8 as libc::c_int + c - 90 as libc::c_int) as isize);
    } else if c & 0x1000000 as libc::c_int != 0 {
        new = *((*wp).palette).offset((c & !(0x1000000 as libc::c_int)) as isize);
    }
    if new == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return new;
}
unsafe extern "C" fn window_pane_mode_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = arg as *mut window_pane;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 10 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    let mut n: libc::c_int = 0 as libc::c_int;
    event_del(&mut (*wp).modetimer);
    event_add(&mut (*wp).modetimer, &mut tv);
    log_debug(
        b"%%%u in mode: last=%ld\0" as *const u8 as *const libc::c_char,
        (*wp).id,
        (*wp).modelast,
    );
    if (*wp).modelast < time(0 as *mut time_t) - 180 as libc::c_int as time_t {
        if ioctl(
            (*wp).fd,
            0x541b as libc::c_int as libc::c_ulong,
            &mut n as *mut libc::c_int,
        ) == -(1 as libc::c_int) || n > 0 as libc::c_int
        {
            window_pane_reset_mode(wp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_mode(
    mut wp: *mut window_pane,
    mut mode: *const window_mode,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> libc::c_int {
    let mut s: *mut screen = 0 as *mut screen;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 10 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    if !((*wp).mode).is_null() {
        return 1 as libc::c_int;
    }
    (*wp).mode = mode;
    (*wp).modelast = time(0 as *mut time_t);
    event_set(
        &mut (*wp).modetimer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            window_pane_mode_timer
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        wp as *mut libc::c_void,
    );
    event_add(&mut (*wp).modetimer, &mut tv);
    s = ((*(*wp).mode).init).expect("non-null function pointer")(wp, fs, args);
    if !s.is_null() {
        (*wp).screen = s;
    }
    (*wp).flags |= 0x1 as libc::c_int | 0x80 as libc::c_int;
    server_status_window((*wp).window);
    notify_pane(b"pane-mode-changed\0" as *const u8 as *const libc::c_char, wp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_mode(mut wp: *mut window_pane) {
    if ((*wp).mode).is_null() {
        return;
    }
    event_del(&mut (*wp).modetimer);
    ((*(*wp).mode).free).expect("non-null function pointer")(wp);
    (*wp).mode = 0 as *const window_mode;
    (*wp).modeprefix = 1 as libc::c_int as u_int;
    (*wp).screen = &mut (*wp).base;
    (*wp).flags |= 0x1 as libc::c_int | 0x80 as libc::c_int;
    server_status_window((*wp).window);
    notify_pane(b"pane-mode-changed\0" as *const u8 as *const libc::c_char, wp);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_key(
    mut wp: *mut window_pane,
    mut c: *mut client,
    mut s: *mut session,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut wp2: *mut window_pane = 0 as *mut window_pane;
    if key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
        && key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_int as libc::c_ulonglong && m.is_null()
    {
        return;
    }
    if !((*wp).mode).is_null() {
        (*wp).modelast = time(0 as *mut time_t);
        if ((*(*wp).mode).key).is_some() {
            ((*(*wp).mode).key)
                .expect(
                    "non-null function pointer",
                )(wp, c, s, key & !(0x1000000000000 as libc::c_ulonglong), m);
        }
        return;
    }
    if (*wp).fd == -(1 as libc::c_int) || (*wp).flags & 0x40 as libc::c_int != 0 {
        return;
    }
    input_key(wp, key, m);
    if key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
        && key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_int as libc::c_ulonglong
    {
        return;
    }
    if options_get_number(
        (*(*wp).window).options,
        b"synchronize-panes\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        wp2 = (*(*wp).window).panes.tqh_first;
        while !wp2.is_null() {
            if !(wp2 == wp || !((*wp2).mode).is_null()) {
                if !((*wp2).fd == -(1 as libc::c_int)
                    || (*wp2).flags & 0x40 as libc::c_int != 0)
                {
                    if window_pane_visible(wp2) != 0 {
                        input_key(wp2, key, 0 as *mut mouse_event);
                    }
                }
            }
            wp2 = (*wp2).entry.tqe_next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_visible(mut wp: *mut window_pane) -> libc::c_int {
    let mut w: *mut window = (*wp).window;
    if ((*wp).layout_cell).is_null() {
        return 0 as libc::c_int;
    }
    if (*wp).xoff >= (*w).sx || (*wp).yoff >= (*w).sy {
        return 0 as libc::c_int;
    }
    if ((*wp).xoff).wrapping_add((*wp).sx) > (*w).sx
        || ((*wp).yoff).wrapping_add((*wp).sy) > (*w).sy
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_search(
    mut wp: *mut window_pane,
    mut searchstr: *const libc::c_char,
) -> u_int {
    let mut s: *mut screen = &mut (*wp).base;
    let mut newsearchstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    xasprintf(
        &mut newsearchstr as *mut *mut libc::c_char,
        b"*%s*\0" as *const u8 as *const libc::c_char,
        searchstr,
    );
    i = 0 as libc::c_int as u_int;
    while i < (*(*s).grid).sy {
        line = grid_view_string_cells(
            (*s).grid,
            0 as libc::c_int as u_int,
            i,
            (*(*s).grid).sx,
        );
        if fnmatch(newsearchstr, line, 0 as libc::c_int) == 0 as libc::c_int {
            free(line as *mut libc::c_void);
            break;
        } else {
            free(line as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
    }
    free(newsearchstr as *mut libc::c_void);
    if i == (*(*s).grid).sy {
        return 0 as libc::c_int as u_int;
    }
    return i.wrapping_add(1 as libc::c_int as u_int);
}
unsafe extern "C" fn window_pane_choose_best(
    mut list: *mut *mut window_pane,
    mut size: u_int,
) -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    if size == 0 as libc::c_int as u_int {
        return 0 as *mut window_pane;
    }
    best = *list.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int as u_int;
    while i < size {
        next = *list.offset(i as isize);
        if (*next).active_point > (*best).active_point {
            best = next;
        }
        i = i.wrapping_add(1);
        i;
    }
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_up(
    mut wp: *mut window_pane,
) -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut left: u_int = 0;
    let mut right: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut status: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() || window_pane_visible(wp) == 0 {
        return 0 as *mut window_pane;
    }
    status = options_get_number(
        (*(*wp).window).options,
        b"pane-border-status\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    list = 0 as *mut *mut window_pane;
    size = 0 as libc::c_int as u_int;
    edge = (*wp).yoff;
    if edge
        == (if status == 1 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int })
            as u_int
    {
        edge = ((*(*wp).window).sy)
            .wrapping_add(1 as libc::c_int as u_int)
            .wrapping_sub(
                (if status == 2 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as u_int,
            );
    }
    left = (*wp).xoff;
    right = ((*wp).xoff).wrapping_add((*wp).sx);
    next = (*(*wp).window).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp || window_pane_visible(next) == 0) {
            if !(((*next).yoff)
                .wrapping_add((*next).sy)
                .wrapping_add(1 as libc::c_int as u_int) != edge)
            {
                end = ((*next).xoff)
                    .wrapping_add((*next).sx)
                    .wrapping_sub(1 as libc::c_int as u_int);
                found = 0 as libc::c_int;
                if (*next).xoff < left && end > right {
                    found = 1 as libc::c_int;
                } else if (*next).xoff >= left && (*next).xoff <= right {
                    found = 1 as libc::c_int;
                } else if end >= left && end <= right {
                    found = 1 as libc::c_int;
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                        ::core::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh10 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh11 = *list.offset(fresh10 as isize);
                    *fresh11 = next;
                }
            }
        }
        next = (*next).entry.tqe_next;
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_down(
    mut wp: *mut window_pane,
) -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut left: u_int = 0;
    let mut right: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut status: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() || window_pane_visible(wp) == 0 {
        return 0 as *mut window_pane;
    }
    status = options_get_number(
        (*(*wp).window).options,
        b"pane-border-status\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    list = 0 as *mut *mut window_pane;
    size = 0 as libc::c_int as u_int;
    edge = ((*wp).yoff).wrapping_add((*wp).sy).wrapping_add(1 as libc::c_int as u_int);
    if edge
        >= ((*(*wp).window).sy)
            .wrapping_sub(
                (if status == 2 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as u_int,
            )
    {
        edge = (if status == 1 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as u_int;
    }
    left = (*wp).xoff;
    right = ((*wp).xoff).wrapping_add((*wp).sx);
    next = (*(*wp).window).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp || window_pane_visible(next) == 0) {
            if !((*next).yoff != edge) {
                end = ((*next).xoff)
                    .wrapping_add((*next).sx)
                    .wrapping_sub(1 as libc::c_int as u_int);
                found = 0 as libc::c_int;
                if (*next).xoff < left && end > right {
                    found = 1 as libc::c_int;
                } else if (*next).xoff >= left && (*next).xoff <= right {
                    found = 1 as libc::c_int;
                } else if end >= left && end <= right {
                    found = 1 as libc::c_int;
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                        ::core::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh12 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh13 = *list.offset(fresh12 as isize);
                    *fresh13 = next;
                }
            }
        }
        next = (*next).entry.tqe_next;
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_left(
    mut wp: *mut window_pane,
) -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut top: u_int = 0;
    let mut bottom: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() || window_pane_visible(wp) == 0 {
        return 0 as *mut window_pane;
    }
    list = 0 as *mut *mut window_pane;
    size = 0 as libc::c_int as u_int;
    edge = (*wp).xoff;
    if edge == 0 as libc::c_int as u_int {
        edge = ((*(*wp).window).sx).wrapping_add(1 as libc::c_int as u_int);
    }
    top = (*wp).yoff;
    bottom = ((*wp).yoff).wrapping_add((*wp).sy);
    next = (*(*wp).window).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp || window_pane_visible(next) == 0) {
            if !(((*next).xoff)
                .wrapping_add((*next).sx)
                .wrapping_add(1 as libc::c_int as u_int) != edge)
            {
                end = ((*next).yoff)
                    .wrapping_add((*next).sy)
                    .wrapping_sub(1 as libc::c_int as u_int);
                found = 0 as libc::c_int;
                if (*next).yoff < top && end > bottom {
                    found = 1 as libc::c_int;
                } else if (*next).yoff >= top && (*next).yoff <= bottom {
                    found = 1 as libc::c_int;
                } else if end >= top && end <= bottom {
                    found = 1 as libc::c_int;
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                        ::core::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh14 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh15 = *list.offset(fresh14 as isize);
                    *fresh15 = next;
                }
            }
        }
        next = (*next).entry.tqe_next;
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_right(
    mut wp: *mut window_pane,
) -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut top: u_int = 0;
    let mut bottom: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() || window_pane_visible(wp) == 0 {
        return 0 as *mut window_pane;
    }
    list = 0 as *mut *mut window_pane;
    size = 0 as libc::c_int as u_int;
    edge = ((*wp).xoff).wrapping_add((*wp).sx).wrapping_add(1 as libc::c_int as u_int);
    if edge >= (*(*wp).window).sx {
        edge = 0 as libc::c_int as u_int;
    }
    top = (*wp).yoff;
    bottom = ((*wp).yoff).wrapping_add((*wp).sy);
    next = (*(*wp).window).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp || window_pane_visible(next) == 0) {
            if !((*next).xoff != edge) {
                end = ((*next).yoff)
                    .wrapping_add((*next).sy)
                    .wrapping_sub(1 as libc::c_int as u_int);
                found = 0 as libc::c_int;
                if (*next).yoff < top && end > bottom {
                    found = 1 as libc::c_int;
                } else if (*next).yoff >= top && (*next).yoff <= bottom {
                    found = 1 as libc::c_int;
                } else if end >= top && end <= bottom {
                    found = 1 as libc::c_int;
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1 as libc::c_int as u_int) as size_t,
                        ::core::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh16 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh17 = *list.offset(fresh16 as isize);
                    *fresh17 = next;
                }
            }
        }
        next = (*next).entry.tqe_next;
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_clear_flags(mut wl: *mut winlink) {
    let mut loop_0: *mut winlink = 0 as *mut winlink;
    (*(*wl).window).flags
        &= !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int);
    loop_0 = (*(*wl).window).winlinks.tqh_first;
    while !loop_0.is_null() {
        if (*loop_0).flags
            & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int
        {
            (*loop_0).flags
                &= !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
            server_status_session((*loop_0).session);
        }
        loop_0 = (*loop_0).wentry.tqe_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn winlink_shuffle_up(
    mut s: *mut session,
    mut wl: *mut winlink,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    idx = (*wl).idx + 1 as libc::c_int;
    last = idx;
    while last < 2147483647 as libc::c_int {
        if (winlink_find_by_index(&mut (*s).windows, last)).is_null() {
            break;
        }
        last += 1;
        last;
    }
    if last == 2147483647 as libc::c_int {
        return -(1 as libc::c_int);
    }
    while last > idx {
        wl = winlink_find_by_index(&mut (*s).windows, last - 1 as libc::c_int);
        server_link_window(
            s,
            wl,
            s,
            last,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut *mut libc::c_char,
        );
        server_unlink_window(s, wl);
        last -= 1;
        last;
    }
    return idx;
}
