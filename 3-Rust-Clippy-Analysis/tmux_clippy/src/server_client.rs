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
    pub type tmuxproc;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_del(_: *mut event) -> libc::c_int;
    fn event_pending(
        ev: *const event,
        events: libc::c_short,
        tv: *mut timeval,
    ) -> libc::c_int;
    fn event_initialized(ev: *const event) -> libc::c_int;
    fn event_once(
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
        _: *const timeval,
    ) -> libc::c_int;
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
    fn evbuffer_add_printf(
        buf: *mut evbuffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    static mut global_s_options: *mut options;
    fn areshell(_: *const libc::c_char) -> libc::c_int;
    fn setblocking(_: libc::c_int, _: libc::c_int);
    fn find_home() -> *const libc::c_char;
    fn proc_send(
        _: *mut tmuxpeer,
        _: msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    fn proc_add_peer(
        _: *mut tmuxproc,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*mut imsg, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> *mut tmuxpeer;
    fn proc_remove_peer(_: *mut tmuxpeer);
    fn proc_kill_peer(_: *mut tmuxpeer);
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
    fn format_lost_client(_: *mut client);
    fn notify_client(_: *const libc::c_char, _: *mut client);
    fn options_get_string(
        _: *mut options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn environ_create() -> *mut environ;
    fn environ_free(_: *mut environ);
    fn environ_find(_: *mut environ, _: *const libc::c_char) -> *mut environ_entry;
    fn environ_put(_: *mut environ, _: *const libc::c_char);
    fn tty_reset(_: *mut tty);
    fn tty_region_off(_: *mut tty);
    fn tty_margin_off(_: *mut tty);
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int);
    fn tty_init(
        _: *mut tty,
        _: *mut client,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn tty_resize(_: *mut tty);
    fn tty_start_tty(_: *mut tty);
    fn tty_stop_tty(_: *mut tty);
    fn tty_set_title(_: *mut tty, _: *const libc::c_char);
    fn tty_update_mode(_: *mut tty, _: libc::c_int, _: *mut screen);
    fn tty_open(_: *mut tty, _: *mut *mut libc::c_char) -> libc::c_int;
    fn tty_close(_: *mut tty);
    fn tty_free(_: *mut tty);
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session, _: libc::c_int);
    fn cmd_find_from_mouse(
        _: *mut cmd_find_state,
        _: *mut mouse_event,
        _: libc::c_int,
    ) -> libc::c_int;
    fn cmd_unpack_argv(
        _: *mut libc::c_char,
        _: size_t,
        _: libc::c_int,
        _: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    fn cmd_list_parse(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut cmd_list;
    fn cmd_list_free(_: *mut cmd_list);
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut cmd_find_state,
        _: *mut mouse_event,
        _: libc::c_int,
    ) -> *mut cmdq_item;
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut cmdq_item;
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn key_bindings_RB_FIND(
        _: *mut key_bindings,
        _: *mut key_binding,
    ) -> *mut key_binding;
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int) -> *mut key_table;
    fn key_bindings_unref_table(_: *mut key_table);
    fn key_bindings_dispatch(
        _: *mut key_binding,
        _: *mut cmdq_item,
        _: *mut client,
        _: *mut mouse_event,
        _: *mut cmd_find_state,
    );
    static mut server_proc: *mut tmuxproc;
    static mut clients: clients;
    fn server_update_socket();
    fn server_add_accept(_: libc::c_int);
    fn server_redraw_client(_: *mut client);
    fn server_status_client(_: *mut client);
    fn server_check_unattached();
    fn status_at_line(_: *mut client) -> libc::c_int;
    fn status_line_size(_: *mut session) -> u_int;
    fn status_get_window_at(_: *mut client, _: u_int) -> *mut window;
    fn status_message_clear(_: *mut client);
    fn status_prompt_clear(_: *mut client);
    fn status_prompt_key(_: *mut client, _: key_code) -> libc::c_int;
    fn recalculate_sizes();
    fn screen_redraw_update(_: *mut client);
    fn screen_redraw_screen(
        _: *mut client,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn screen_redraw_pane(_: *mut client, _: *mut window_pane);
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn screen_free(_: *mut screen);
    static mut windows: windows;
    static mut all_window_panes: window_pane_tree;
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    fn window_pane_tree_RB_MINMAX(
        _: *mut window_pane_tree,
        _: libc::c_int,
    ) -> *mut window_pane;
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    fn window_get_active_at(_: *mut window, _: u_int, _: u_int) -> *mut window_pane;
    fn window_unzoom(_: *mut window) -> libc::c_int;
    fn window_pane_at_index(_: *mut window, _: u_int) -> *mut window_pane;
    fn window_pane_key(
        _: *mut window_pane,
        _: *mut client,
        _: *mut session,
        _: key_code,
        _: *mut mouse_event,
    );
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    fn check_window_name(_: *mut window);
    fn control_callback(_: *mut client, _: libc::c_int, _: *mut libc::c_void);
    fn session_update_activity(_: *mut session, _: *mut timeval);
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}
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
pub struct msg_command_data {
    pub argc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_stdin_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_stdout_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_stderr_data {
    pub size: ssize_t,
    pub data: [libc::c_char; 8192],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const BORDER: C2RustUnnamed_36 = 3;
pub type C2RustUnnamed_36 = libc::c_uint;
pub const STATUS: C2RustUnnamed_36 = 2;
pub const PANE: C2RustUnnamed_36 = 1;
pub const NOWHERE: C2RustUnnamed_36 = 0;
pub const TRIPLE: C2RustUnnamed_37 = 7;
pub const DOUBLE: C2RustUnnamed_37 = 6;
pub const DOWN: C2RustUnnamed_37 = 2;
pub const UP: C2RustUnnamed_37 = 3;
pub const WHEEL: C2RustUnnamed_37 = 5;
pub const DRAG: C2RustUnnamed_37 = 4;
pub const MOVE: C2RustUnnamed_37 = 1;
pub const NOTYPE: C2RustUnnamed_37 = 0;
pub type C2RustUnnamed_37 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn server_client_how_many() -> u_int {
    let mut c: *mut client = 0 as *mut client;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session).is_null() && !(*c).flags & 0x1000 as libc::c_int != 0 {
            n = n.wrapping_add(1);
            n;
        }
        c = (*c).entry.tqe_next;
    }
    return n;
}
unsafe extern "C" fn server_client_callback_identify(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    server_client_clear_identify(data as *mut client, 0 as *mut window_pane);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_set_identify(
    mut c: *mut client,
    mut delay: u_int,
) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = (delay / 1000 as libc::c_int as u_int) as __time_t;
    tv
        .tv_usec = (delay % 1000 as libc::c_int as u_int) as libc::c_long
        * 1000 as libc::c_long;
    if event_initialized(&mut (*c).identify_timer) != 0 {
        event_del(&mut (*c).identify_timer);
    }
    event_set(
        &mut (*c).identify_timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            server_client_callback_identify
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    if delay != 0 as libc::c_int as u_int {
        event_add(&mut (*c).identify_timer, &mut tv);
    }
    (*c).flags |= 0x100 as libc::c_int;
    (*c).tty.flags |= 0x2 as libc::c_int | 0x1 as libc::c_int;
    server_redraw_client(c);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_clear_identify(
    mut c: *mut client,
    mut wp: *mut window_pane,
) {
    if !(*c).flags & 0x100 as libc::c_int != 0 {
        return;
    }
    (*c).flags &= !(0x100 as libc::c_int);
    if ((*c).identify_callback).is_some() {
        ((*c).identify_callback).expect("non-null function pointer")(c, wp);
    }
    (*c).tty.flags &= !(0x2 as libc::c_int | 0x1 as libc::c_int);
    server_redraw_client(c);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_check_nested(mut c: *mut client) -> libc::c_int {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    envent = environ_find((*c).environ, b"TMUX\0" as *const u8 as *const libc::c_char);
    if envent.is_null() || *(*envent).value as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    wp = window_pane_tree_RB_MINMAX(&mut all_window_panes, -(1 as libc::c_int));
    while !wp.is_null() {
        if strcmp(((*wp).tty).as_mut_ptr(), (*c).ttyname) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        wp = window_pane_tree_RB_NEXT(wp);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_set_key_table(
    mut c: *mut client,
    mut name: *const libc::c_char,
) {
    if name.is_null() {
        name = server_client_get_key_table(c);
    }
    key_bindings_unref_table((*c).keytable);
    (*c).keytable = key_bindings_get_table(name, 1 as libc::c_int);
    (*(*c).keytable).references = ((*(*c).keytable).references).wrapping_add(1);
    (*(*c).keytable).references;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_get_key_table(
    mut c: *mut client,
) -> *const libc::c_char {
    let mut s: *mut session = (*c).session;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if s.is_null() {
        return b"root\0" as *const u8 as *const libc::c_char;
    }
    name = options_get_string(
        (*s).options,
        b"key-table\0" as *const u8 as *const libc::c_char,
    );
    if *name as libc::c_int == '\0' as i32 {
        return b"root\0" as *const u8 as *const libc::c_char;
    }
    return name;
}
unsafe extern "C" fn server_client_is_default_key_table(
    mut c: *mut client,
    mut table: *mut key_table,
) -> libc::c_int {
    return (strcmp((*table).name, server_client_get_key_table(c)) == 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_create(mut fd: libc::c_int) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    setblocking(fd, 0 as libc::c_int);
    c = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<client>() as libc::c_ulong,
    ) as *mut client;
    (*c).references = 1 as libc::c_int;
    (*c)
        .peer = proc_add_peer(
        server_proc,
        fd,
        Some(
            server_client_dispatch
                as unsafe extern "C" fn(*mut imsg, *mut libc::c_void) -> (),
        ),
        c as *mut libc::c_void,
    );
    if gettimeofday(&mut (*c).creation_time, 0 as *mut libc::c_void) != 0 as libc::c_int
    {
        fatal(b"gettimeofday failed\0" as *const u8 as *const libc::c_char);
    }
    memcpy(
        &mut (*c).activity_time as *mut timeval as *mut libc::c_void,
        &mut (*c).creation_time as *mut timeval as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    (*c).environ = environ_create();
    (*c).fd = -(1 as libc::c_int);
    (*c).cwd = 0 as *const libc::c_char;
    (*c).queue.tqh_first = 0 as *mut cmdq_item;
    (*c).queue.tqh_last = &mut (*c).queue.tqh_first;
    (*c).stdin_data = evbuffer_new();
    (*c).stdout_data = evbuffer_new();
    (*c).stderr_data = evbuffer_new();
    (*c).tty.fd = -(1 as libc::c_int);
    (*c).title = 0 as *mut libc::c_char;
    (*c).session = 0 as *mut session;
    (*c).last_session = 0 as *mut session;
    (*c).tty.sx = 80 as libc::c_int as u_int;
    (*c).tty.sy = 24 as libc::c_int as u_int;
    screen_init(
        &mut (*c).status.status,
        (*c).tty.sx,
        1 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    (*c).message_string = 0 as *mut libc::c_char;
    (*c).message_log.tqh_first = 0 as *mut message_entry;
    (*c).message_log.tqh_last = &mut (*c).message_log.tqh_first;
    (*c).prompt_string = 0 as *mut libc::c_char;
    (*c).prompt_buffer = 0 as *mut utf8_data;
    (*c).prompt_index = 0 as libc::c_int as size_t;
    (*c).flags |= 0x8000 as libc::c_int;
    (*c)
        .keytable = key_bindings_get_table(
        b"root\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    (*(*c).keytable).references = ((*(*c).keytable).references).wrapping_add(1);
    (*(*c).keytable).references;
    event_set(
        &mut (*c).repeat_timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            server_client_repeat_timer
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    event_set(
        &mut (*c).click_timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            server_client_click_timer
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    (*c).entry.tqe_next = 0 as *mut client;
    (*c).entry.tqe_prev = clients.tqh_last;
    *clients.tqh_last = c;
    clients.tqh_last = &mut (*c).entry.tqe_next;
    log_debug(b"new client %p\0" as *const u8 as *const libc::c_char, c);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_open(
    mut c: *mut client,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    if (*c).flags & 0x2000 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if strcmp((*c).ttyname, b"/dev/tty\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *cause = xstrdup(b"can't use /dev/tty\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if (*c).flags & 0x1 as libc::c_int == 0 {
        *cause = xstrdup(b"not a terminal\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if tty_open(&mut (*c).tty, cause) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_lost(mut c: *mut client) {
    let mut msg: *mut message_entry = 0 as *mut message_entry;
    let mut msg1: *mut message_entry = 0 as *mut message_entry;
    (*c).flags |= 0x200 as libc::c_int;
    server_client_clear_identify(c, 0 as *mut window_pane);
    status_prompt_clear(c);
    status_message_clear(c);
    if ((*c).stdin_callback).is_some() {
        ((*c).stdin_callback)
            .expect(
                "non-null function pointer",
            )(c, 1 as libc::c_int, (*c).stdin_callback_data);
    }
    if !((*c).entry.tqe_next).is_null() {
        (*(*c).entry.tqe_next).entry.tqe_prev = (*c).entry.tqe_prev;
    } else {
        clients.tqh_last = (*c).entry.tqe_prev;
    }
    *(*c).entry.tqe_prev = (*c).entry.tqe_next;
    log_debug(b"lost client %p\0" as *const u8 as *const libc::c_char, c);
    if (*c).flags & 0x1 as libc::c_int != 0 {
        tty_free(&mut (*c).tty);
    }
    free((*c).ttyname as *mut libc::c_void);
    free((*c).term as *mut libc::c_void);
    evbuffer_free((*c).stdin_data);
    evbuffer_free((*c).stdout_data);
    if (*c).stderr_data != (*c).stdout_data {
        evbuffer_free((*c).stderr_data);
    }
    if event_initialized(&mut (*c).status.timer) != 0 {
        event_del(&mut (*c).status.timer);
    }
    screen_free(&mut (*c).status.status);
    if !((*c).status.old_status).is_null() {
        screen_free((*c).status.old_status);
        free((*c).status.old_status as *mut libc::c_void);
    }
    free((*c).title as *mut libc::c_void);
    free((*c).cwd as *mut libc::c_void);
    event_del(&mut (*c).repeat_timer);
    event_del(&mut (*c).click_timer);
    key_bindings_unref_table((*c).keytable);
    if event_initialized(&mut (*c).identify_timer) != 0 {
        event_del(&mut (*c).identify_timer);
    }
    free((*c).message_string as *mut libc::c_void);
    if event_initialized(&mut (*c).message_timer) != 0 {
        event_del(&mut (*c).message_timer);
    }
    msg = (*c).message_log.tqh_first;
    while !msg.is_null()
        && {
            msg1 = (*msg).entry.tqe_next;
            1 as libc::c_int != 0
        }
    {
        free((*msg).msg as *mut libc::c_void);
        if !((*msg).entry.tqe_next).is_null() {
            (*(*msg).entry.tqe_next).entry.tqe_prev = (*msg).entry.tqe_prev;
        } else {
            (*c).message_log.tqh_last = (*msg).entry.tqe_prev;
        }
        *(*msg).entry.tqe_prev = (*msg).entry.tqe_next;
        free(msg as *mut libc::c_void);
        msg = msg1;
    }
    free((*c).prompt_string as *mut libc::c_void);
    free((*c).prompt_buffer as *mut libc::c_void);
    format_lost_client(c);
    environ_free((*c).environ);
    proc_remove_peer((*c).peer);
    (*c).peer = 0 as *mut tmuxpeer;
    server_client_unref(c);
    server_add_accept(0 as libc::c_int);
    recalculate_sizes();
    server_check_unattached();
    server_update_socket();
}
#[no_mangle]
pub unsafe extern "C" fn server_client_unref(mut c: *mut client) {
    log_debug(
        b"unref client %p (%d references)\0" as *const u8 as *const libc::c_char,
        c,
        (*c).references,
    );
    (*c).references -= 1;
    (*c).references;
    if (*c).references == 0 as libc::c_int {
        event_once(
            -(1 as libc::c_int),
            0x1 as libc::c_int as libc::c_short,
            Some(
                server_client_free
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
            0 as *const timeval,
        );
    }
}
unsafe extern "C" fn server_client_free(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    log_debug(
        b"free client %p (%d references)\0" as *const u8 as *const libc::c_char,
        c,
        (*c).references,
    );
    if !((*c).queue.tqh_first).is_null() {
        fatalx(b"queue not empty\0" as *const u8 as *const libc::c_char);
    }
    if (*c).references == 0 as libc::c_int {
        free((*c).name as *mut libc::c_void);
        free(c as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_client_suspend(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    if s.is_null() || (*c).flags & 0x1000 as libc::c_int != 0 {
        return;
    }
    tty_stop_tty(&mut (*c).tty);
    (*c).flags |= 0x40 as libc::c_int;
    proc_send(
        (*c).peer,
        MSG_SUSPEND,
        -(1 as libc::c_int),
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn server_client_detach(mut c: *mut client, mut msgtype: msgtype) {
    let mut s: *mut session = (*c).session;
    if s.is_null() || (*c).flags & 0x1000 as libc::c_int != 0 {
        return;
    }
    (*c).flags |= 0x1000 as libc::c_int;
    notify_client(b"client-detached\0" as *const u8 as *const libc::c_char, c);
    proc_send(
        (*c).peer,
        msgtype,
        -(1 as libc::c_int),
        (*s).name as *const libc::c_void,
        (strlen((*s).name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn server_client_exec(
    mut c: *mut client,
    mut cmd: *const libc::c_char,
) {
    let mut s: *mut session = (*c).session;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdsize: size_t = 0;
    let mut shellsize: size_t = 0;
    if *cmd as libc::c_int == '\0' as i32 {
        return;
    }
    cmdsize = (strlen(cmd)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !s.is_null() {
        shell = options_get_string(
            (*s).options,
            b"default-shell\0" as *const u8 as *const libc::c_char,
        );
    } else {
        shell = options_get_string(
            global_s_options,
            b"default-shell\0" as *const u8 as *const libc::c_char,
        );
    }
    shellsize = (strlen(shell)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    msg = xmalloc(cmdsize.wrapping_add(shellsize)) as *mut libc::c_char;
    memcpy(msg as *mut libc::c_void, cmd as *const libc::c_void, cmdsize);
    memcpy(
        msg.offset(cmdsize as isize) as *mut libc::c_void,
        shell as *const libc::c_void,
        shellsize,
    );
    proc_send(
        (*c).peer,
        MSG_EXEC,
        -(1 as libc::c_int),
        msg as *const libc::c_void,
        cmdsize.wrapping_add(shellsize),
    );
    free(msg as *mut libc::c_void);
}
unsafe extern "C" fn server_client_check_mouse(mut c: *mut client) -> key_code {
    let mut current_block: u64;
    let mut s: *mut session = (*c).session;
    let mut m: *mut mouse_event = &mut (*c).tty.mouse;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut b: u_int = 0;
    let mut flag: libc::c_int = 0;
    let mut key: key_code = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut type_0: C2RustUnnamed_37 = NOTYPE;
    let mut where_0: C2RustUnnamed_36 = NOWHERE;
    type_0 = NOTYPE;
    where_0 = NOWHERE;
    log_debug(
        b"mouse %02x at %u,%u (last %u,%u) (%d)\0" as *const u8 as *const libc::c_char,
        (*m).b,
        (*m).x,
        (*m).y,
        (*m).lx,
        (*m).ly,
        (*c).tty.mouse_drag_flag,
    );
    if (*m).sgr_type != ' ' as i32 as u_int
        && (*m).sgr_b & 32 as libc::c_int as u_int != 0
        && (*m).sgr_b & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int
        || (*m).sgr_type == ' ' as i32 as u_int
            && (*m).b & 32 as libc::c_int as u_int != 0
            && (*m).b & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int
            && (*m).lb & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int
    {
        type_0 = MOVE;
        x = (*m).x;
        y = (*m).y;
        b = 0 as libc::c_int as u_int;
        log_debug(b"move at %u,%u\0" as *const u8 as *const libc::c_char, x, y);
    } else if (*m).b & 32 as libc::c_int as u_int != 0 {
        type_0 = DRAG;
        if (*c).tty.mouse_drag_flag != 0 {
            x = (*m).x;
            y = (*m).y;
            b = (*m).b;
            log_debug(
                b"drag update at %u,%u\0" as *const u8 as *const libc::c_char,
                x,
                y,
            );
        } else {
            x = (*m).lx;
            y = (*m).ly;
            b = (*m).lb;
            log_debug(
                b"drag start at %u,%u\0" as *const u8 as *const libc::c_char,
                x,
                y,
            );
        }
    } else if (*m).b & 64 as libc::c_int as u_int != 0 {
        type_0 = WHEEL;
        x = (*m).x;
        y = (*m).y;
        b = (*m).b;
        log_debug(b"wheel at %u,%u\0" as *const u8 as *const libc::c_char, x, y);
    } else if (*m).b & 3 as libc::c_int as u_int == 3 as libc::c_int as u_int {
        type_0 = UP;
        x = (*m).x;
        y = (*m).y;
        b = (*m).lb;
        log_debug(b"up at %u,%u\0" as *const u8 as *const libc::c_char, x, y);
    } else {
        if (*c).flags & 0x100000 as libc::c_int != 0 {
            event_del(&mut (*c).click_timer);
            (*c).flags &= !(0x100000 as libc::c_int);
            if (*m).b == (*c).click_button {
                type_0 = DOUBLE;
                x = (*m).x;
                y = (*m).y;
                b = (*m).b;
                log_debug(
                    b"double-click at %u,%u\0" as *const u8 as *const libc::c_char,
                    x,
                    y,
                );
                flag = 0x200000 as libc::c_int;
                current_block = 15790018738659186292;
            } else {
                current_block = 7333393191927787629;
            }
        } else if (*c).flags & 0x200000 as libc::c_int != 0 {
            event_del(&mut (*c).click_timer);
            (*c).flags &= !(0x200000 as libc::c_int);
            if (*m).b == (*c).click_button {
                type_0 = TRIPLE;
                x = (*m).x;
                y = (*m).y;
                b = (*m).b;
                log_debug(
                    b"triple-click at %u,%u\0" as *const u8 as *const libc::c_char,
                    x,
                    y,
                );
                current_block = 8169881805552918302;
            } else {
                current_block = 7333393191927787629;
            }
        } else {
            current_block = 7333393191927787629;
        }
        match current_block {
            8169881805552918302 => {}
            _ => {
                match current_block {
                    7333393191927787629 => {
                        type_0 = DOWN;
                        x = (*m).x;
                        y = (*m).y;
                        b = (*m).b;
                        log_debug(
                            b"down at %u,%u\0" as *const u8 as *const libc::c_char,
                            x,
                            y,
                        );
                        flag = 0x100000 as libc::c_int;
                    }
                    _ => {}
                }
                if 300 as libc::c_int != 0 as libc::c_int {
                    (*c).flags |= flag;
                    (*c).click_button = (*m).b;
                    tv.tv_sec = (300 as libc::c_int / 1000 as libc::c_int) as __time_t;
                    tv
                        .tv_usec = (300 as libc::c_int % 1000 as libc::c_int)
                        as libc::c_long * 1000 as libc::c_long;
                    event_del(&mut (*c).click_timer);
                    event_add(&mut (*c).click_timer, &mut tv);
                }
            }
        }
    }
    if type_0 as libc::c_uint == NOTYPE as libc::c_int as libc::c_uint {
        return 0xfffe00000000 as libc::c_ulonglong;
    }
    (*m).s = (*s).id as libc::c_int;
    (*m).statusat = status_at_line(c);
    if (*m).statusat != -(1 as libc::c_int) && y == (*m).statusat as u_int {
        w = status_get_window_at(c, x);
        if w.is_null() {
            return 0xfffe00000000 as libc::c_ulonglong;
        }
        (*m).w = (*w).id as libc::c_int;
        where_0 = STATUS;
    } else {
        (*m).w = -(1 as libc::c_int);
    }
    if where_0 as libc::c_uint == NOWHERE as libc::c_int as libc::c_uint {
        if (*m).statusat == 0 as libc::c_int && y > 0 as libc::c_int as u_int {
            y = y.wrapping_sub(1);
            y;
        } else if (*m).statusat > 0 as libc::c_int && y >= (*m).statusat as u_int {
            y = ((*m).statusat - 1 as libc::c_int) as u_int;
        }
        wp = (*(*(*s).curw).window).panes.tqh_first;
        while !wp.is_null() {
            if ((*wp).xoff).wrapping_add((*wp).sx) == x
                && (*wp).yoff <= (1 as libc::c_int as u_int).wrapping_add(y)
                && ((*wp).yoff).wrapping_add((*wp).sy) >= y
                || ((*wp).yoff).wrapping_add((*wp).sy) == y
                    && (*wp).xoff <= (1 as libc::c_int as u_int).wrapping_add(x)
                    && ((*wp).xoff).wrapping_add((*wp).sx) >= x
            {
                break;
            }
            wp = (*wp).entry.tqe_next;
        }
        if !wp.is_null() {
            where_0 = BORDER;
        } else {
            wp = window_get_active_at((*(*s).curw).window, x, y);
            if !wp.is_null() {
                where_0 = PANE;
                log_debug(
                    b"mouse at %u,%u is on pane %%%u\0" as *const u8
                        as *const libc::c_char,
                    x,
                    y,
                    (*wp).id,
                );
            }
        }
        if where_0 as libc::c_uint == NOWHERE as libc::c_int as libc::c_uint {
            return 0xfffe00000000 as libc::c_ulonglong;
        }
        (*m).wp = (*wp).id as libc::c_int;
        (*m).w = (*(*wp).window).id as libc::c_int;
    } else {
        (*m).wp = -(1 as libc::c_int);
    }
    if type_0 as libc::c_uint != DRAG as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != WHEEL as libc::c_int as libc::c_uint
        && (*c).tty.mouse_drag_flag != 0
    {
        if ((*c).tty.mouse_drag_release).is_some() {
            ((*c).tty.mouse_drag_release).expect("non-null function pointer")(c, m);
        }
        (*c).tty.mouse_drag_update = None;
        (*c).tty.mouse_drag_release = None;
        match (*c).tty.mouse_drag_flag {
            1 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND1_PANE as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND1_STATUS as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND1_BORDER as libc::c_int as key_code;
                }
            }
            2 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND2_PANE as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND2_STATUS as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND2_BORDER as libc::c_int as key_code;
                }
            }
            3 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND3_PANE as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND3_STATUS as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = KEYC_MOUSEDRAGEND3_BORDER as libc::c_int as key_code;
                }
            }
            _ => {
                key = KEYC_MOUSE as libc::c_int as key_code;
            }
        }
        (*c).tty.mouse_drag_flag = 0 as libc::c_int;
        return key;
    }
    key = 0xfffe00000000 as libc::c_ulonglong;
    match type_0 as libc::c_uint {
        1 => {
            if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                key = KEYC_MOUSEMOVE_PANE as libc::c_int as key_code;
            }
            if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                key = KEYC_MOUSEMOVE_STATUS as libc::c_int as key_code;
            }
            if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                key = KEYC_MOUSEMOVE_BORDER as libc::c_int as key_code;
            }
        }
        4 => {
            if ((*c).tty.mouse_drag_update).is_some() {
                key = KEYC_DRAGGING as libc::c_int as key_code;
            } else {
                match b & 3 as libc::c_int as u_int {
                    0 => {
                        if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG1_PANE as libc::c_int as key_code;
                        }
                        if where_0 as libc::c_uint
                            == STATUS as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG1_STATUS as libc::c_int as key_code;
                        }
                        if where_0 as libc::c_uint
                            == BORDER as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG1_BORDER as libc::c_int as key_code;
                        }
                    }
                    1 => {
                        if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG2_PANE as libc::c_int as key_code;
                        }
                        if where_0 as libc::c_uint
                            == STATUS as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG2_STATUS as libc::c_int as key_code;
                        }
                        if where_0 as libc::c_uint
                            == BORDER as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG2_BORDER as libc::c_int as key_code;
                        }
                    }
                    2 => {
                        if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG3_PANE as libc::c_int as key_code;
                        }
                        if where_0 as libc::c_uint
                            == STATUS as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG3_STATUS as libc::c_int as key_code;
                        }
                        if where_0 as libc::c_uint
                            == BORDER as libc::c_int as libc::c_uint
                        {
                            key = KEYC_MOUSEDRAG3_BORDER as libc::c_int as key_code;
                        }
                    }
                    _ => {}
                }
            }
            (*c)
                .tty
                .mouse_drag_flag = (b & 3 as libc::c_int as u_int)
                .wrapping_add(1 as libc::c_int as u_int) as libc::c_int;
        }
        5 => {
            if b & 3 as libc::c_int as u_int == 0 as libc::c_int as u_int {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = KEYC_WHEELUP_PANE as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = KEYC_WHEELUP_STATUS as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = KEYC_WHEELUP_BORDER as libc::c_int as key_code;
                }
            } else {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = KEYC_WHEELDOWN_PANE as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = KEYC_WHEELDOWN_STATUS as libc::c_int as key_code;
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = KEYC_WHEELDOWN_BORDER as libc::c_int as key_code;
                }
            }
        }
        3 => {
            match b & 3 as libc::c_int as u_int {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP1_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP1_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP1_BORDER as libc::c_int as key_code;
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP2_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP2_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP2_BORDER as libc::c_int as key_code;
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP3_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP3_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEUP3_BORDER as libc::c_int as key_code;
                    }
                }
                _ => {}
            }
        }
        2 => {
            match b & 3 as libc::c_int as u_int {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN1_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN1_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN1_BORDER as libc::c_int as key_code;
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN2_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN2_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN2_BORDER as libc::c_int as key_code;
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN3_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN3_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_MOUSEDOWN3_BORDER as libc::c_int as key_code;
                    }
                }
                _ => {}
            }
        }
        6 => {
            match b & 3 as libc::c_int as u_int {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK1_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK1_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK1_BORDER as libc::c_int as key_code;
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK2_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK2_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK2_BORDER as libc::c_int as key_code;
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK3_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK3_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_DOUBLECLICK3_BORDER as libc::c_int as key_code;
                    }
                }
                _ => {}
            }
        }
        7 => {
            match b & 3 as libc::c_int as u_int {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK1_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK1_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK1_BORDER as libc::c_int as key_code;
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK2_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK2_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK2_BORDER as libc::c_int as key_code;
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK3_PANE as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK3_STATUS as libc::c_int as key_code;
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = KEYC_TRIPLECLICK3_BORDER as libc::c_int as key_code;
                    }
                }
                _ => {}
            }
        }
        0 | _ => {}
    }
    if key == 0xfffe00000000 as libc::c_ulonglong {
        return 0xfffe00000000 as libc::c_ulonglong;
    }
    if b & 8 as libc::c_int as u_int != 0 {
        key |= 0x200000000000 as libc::c_ulonglong;
    }
    if b & 16 as libc::c_int as u_int != 0 {
        key |= 0x400000000000 as libc::c_ulonglong;
    }
    if b & 4 as libc::c_int as u_int != 0 {
        key |= 0x800000000000 as libc::c_ulonglong;
    }
    return key;
}
unsafe extern "C" fn server_client_assume_paste(mut s: *mut session) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: libc::c_int = 0;
    t = options_get_number(
        (*s).options,
        b"assume-paste-time\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if t == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    tv.tv_sec = (*s).activity_time.tv_sec - (*s).last_activity_time.tv_sec;
    tv.tv_usec = (*s).activity_time.tv_usec - (*s).last_activity_time.tv_usec;
    if tv.tv_usec < 0 as libc::c_int as __suseconds_t {
        tv.tv_sec -= 1;
        tv.tv_sec;
        tv.tv_usec += 1000000 as libc::c_int as __suseconds_t;
    }
    if tv.tv_sec == 0 as libc::c_int as __time_t
        && tv.tv_usec < (t * 1000 as libc::c_int) as __suseconds_t
    {
        log_debug(
            b"session %s pasting (flag %d)\0" as *const u8 as *const libc::c_char,
            (*s).name,
            ((*s).flags & 0x2 as libc::c_int != 0) as libc::c_int,
        );
        if (*s).flags & 0x2 as libc::c_int != 0 {
            return 1 as libc::c_int;
        }
        (*s).flags |= 0x2 as libc::c_int;
        return 0 as libc::c_int;
    }
    log_debug(
        b"session %s not pasting\0" as *const u8 as *const libc::c_char,
        (*s).name,
    );
    (*s).flags &= !(0x2 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_handle_key(
    mut c: *mut client,
    mut key: key_code,
) {
    let mut m: *mut mouse_event = &mut (*c).tty.mouse;
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut first: *mut key_table = 0 as *mut key_table;
    let mut bd_find: key_binding = key_binding {
        key: 0,
        cmdlist: 0 as *mut cmd_list,
        flags: 0,
        entry: C2RustUnnamed_27 {
            rbe_left: 0 as *mut key_binding,
            rbe_right: 0 as *mut key_binding,
            rbe_parent: 0 as *mut key_binding,
            rbe_color: 0,
        },
    };
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut xtimeout: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut key0: key_code = 0;
    if s.is_null()
        || (*c).flags & (0x200 as libc::c_int | 0x40 as libc::c_int) != 0 as libc::c_int
    {
        return;
    }
    w = (*(*s).curw).window;
    if gettimeofday(&mut (*c).activity_time, 0 as *mut libc::c_void) != 0 as libc::c_int
    {
        fatal(b"gettimeofday failed\0" as *const u8 as *const libc::c_char);
    }
    session_update_activity(s, &mut (*c).activity_time);
    if (*c).flags & 0x100 as libc::c_int != 0 && key >= '0' as i32 as key_code
        && key <= '9' as i32 as key_code
    {
        if (*c).flags & 0x800 as libc::c_int != 0 {
            return;
        }
        window_unzoom(w);
        wp = window_pane_at_index(w, key.wrapping_sub('0' as i32 as key_code) as u_int);
        if !wp.is_null() && window_pane_visible(wp) == 0 {
            wp = 0 as *mut window_pane;
        }
        server_client_clear_identify(c, wp);
        return;
    }
    if (*c).flags & 0x800 as libc::c_int == 0 {
        status_message_clear(c);
        server_client_clear_identify(c, 0 as *mut window_pane);
    }
    if !((*c).prompt_string).is_null() {
        if (*c).flags & 0x800 as libc::c_int != 0 {
            return;
        }
        if status_prompt_key(c, key) == 0 as libc::c_int {
            return;
        }
    }
    (*m).valid = 0 as libc::c_int;
    if key == KEYC_MOUSE as libc::c_int as key_code {
        if (*c).flags & 0x800 as libc::c_int != 0 {
            return;
        }
        key = server_client_check_mouse(c);
        if key == 0xfffe00000000 as libc::c_ulonglong {
            return;
        }
        (*m).valid = 1 as libc::c_int;
        (*m).key = key;
        if key == KEYC_DRAGGING as libc::c_int as key_code {
            ((*c).tty.mouse_drag_update).expect("non-null function pointer")(c, m);
            return;
        }
    } else {
        (*m).valid = 0 as libc::c_int;
    }
    if !(key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
        && key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_int as libc::c_ulonglong)
        || cmd_find_from_mouse(&mut fs, m, 0 as libc::c_int) != 0 as libc::c_int
    {
        cmd_find_from_session(&mut fs, s, 0 as libc::c_int);
    }
    wp = fs.wp;
    if !(key
        & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
            | 0x800000000000 as libc::c_ulonglong | 0x1000000000000 as libc::c_ulonglong)
        >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
        && key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_int as libc::c_ulonglong
        && options_get_number(
            (*s).options,
            b"mouse\0" as *const u8 as *const libc::c_char,
        ) == 0)
    {
        if !(!(key
            & !(0x200000000000 as libc::c_ulonglong | 0x400000000000 as libc::c_ulonglong
                | 0x800000000000 as libc::c_ulonglong
                | 0x1000000000000 as libc::c_ulonglong)
            >= KEYC_MOUSE as libc::c_int as libc::c_ulonglong
            && key
                & !(0x200000000000 as libc::c_ulonglong
                    | 0x400000000000 as libc::c_ulonglong
                    | 0x800000000000 as libc::c_ulonglong
                    | 0x1000000000000 as libc::c_ulonglong)
                < KEYC_BSPACE as libc::c_int as libc::c_ulonglong)
            && server_client_assume_paste(s) != 0)
        {
            if server_client_is_default_key_table(c, (*c).keytable) != 0 && !wp.is_null()
                && !((*wp).mode).is_null() && ((*(*wp).mode).key_table).is_some()
            {
                table = key_bindings_get_table(
                    ((*(*wp).mode).key_table).expect("non-null function pointer")(wp),
                    1 as libc::c_int,
                );
            } else {
                table = (*c).keytable;
            }
            first = table;
            loop {
                key0 = key & !(0x1000000000000 as libc::c_ulonglong);
                if (key0
                    == options_get_number(
                        (*s).options,
                        b"prefix\0" as *const u8 as *const libc::c_char,
                    ) as key_code
                    || key0
                        == options_get_number(
                            (*s).options,
                            b"prefix2\0" as *const u8 as *const libc::c_char,
                        ) as key_code)
                    && strcmp(
                        (*table).name,
                        b"prefix\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    server_client_set_key_table(
                        c,
                        b"prefix\0" as *const u8 as *const libc::c_char,
                    );
                    server_status_client(c);
                    return;
                }
                flags = (*c).flags;
                if wp.is_null() {
                    log_debug(
                        b"key table %s (no pane)\0" as *const u8 as *const libc::c_char,
                        (*table).name,
                    );
                } else {
                    log_debug(
                        b"key table %s (pane %%%u)\0" as *const u8
                            as *const libc::c_char,
                        (*table).name,
                        (*wp).id,
                    );
                }
                if (*c).flags & 0x20 as libc::c_int != 0 {
                    log_debug(
                        b"currently repeating\0" as *const u8 as *const libc::c_char,
                    );
                }
                bd_find.key = key0;
                bd = key_bindings_RB_FIND(&mut (*table).key_bindings, &mut bd_find);
                if !bd.is_null() {
                    if (*c).flags & 0x20 as libc::c_int != 0
                        && !(*bd).flags & 0x1 as libc::c_int != 0
                    {
                        server_client_set_key_table(c, 0 as *const libc::c_char);
                        (*c).flags &= !(0x20 as libc::c_int);
                        server_status_client(c);
                        table = (*c).keytable;
                    } else {
                        log_debug(
                            b"found in key table %s\0" as *const u8
                                as *const libc::c_char,
                            (*table).name,
                        );
                        (*table).references = ((*table).references).wrapping_add(1);
                        (*table).references;
                        xtimeout = options_get_number(
                            (*s).options,
                            b"repeat-time\0" as *const u8 as *const libc::c_char,
                        ) as libc::c_int;
                        if xtimeout != 0 as libc::c_int
                            && (*bd).flags & 0x1 as libc::c_int != 0
                        {
                            (*c).flags |= 0x20 as libc::c_int;
                            tv.tv_sec = (xtimeout / 1000 as libc::c_int) as __time_t;
                            tv
                                .tv_usec = (xtimeout % 1000 as libc::c_int) as libc::c_long
                                * 1000 as libc::c_long;
                            event_del(&mut (*c).repeat_timer);
                            event_add(&mut (*c).repeat_timer, &mut tv);
                        } else {
                            (*c).flags &= !(0x20 as libc::c_int);
                            server_client_set_key_table(c, 0 as *const libc::c_char);
                        }
                        server_status_client(c);
                        key_bindings_dispatch(bd, 0 as *mut cmdq_item, c, m, &mut fs);
                        key_bindings_unref_table(table);
                        return;
                    }
                } else {
                    log_debug(
                        b"not found in key table %s\0" as *const u8
                            as *const libc::c_char,
                        (*table).name,
                    );
                    if server_client_is_default_key_table(c, table) == 0
                        || (*c).flags & 0x20 as libc::c_int != 0
                    {
                        server_client_set_key_table(c, 0 as *const libc::c_char);
                        (*c).flags &= !(0x20 as libc::c_int);
                        server_status_client(c);
                        table = (*c).keytable;
                    } else {
                        if first != table && !flags & 0x20 as libc::c_int != 0 {
                            server_client_set_key_table(c, 0 as *const libc::c_char);
                            server_status_client(c);
                            return;
                        }
                        break;
                    }
                }
            }
        }
    }
    if (*c).flags & 0x800 as libc::c_int != 0 {
        return;
    }
    if !wp.is_null() {
        window_pane_key(wp, c, s, key, m);
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_client_loop() {
    let mut c: *mut client = 0 as *mut client;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut focus: libc::c_int = 0;
    c = clients.tqh_first;
    while !c.is_null() {
        server_client_check_exit(c);
        if !((*c).session).is_null() {
            server_client_check_redraw(c);
            server_client_reset_state(c);
        }
        c = (*c).entry.tqe_next;
    }
    focus = options_get_number(
        global_options,
        b"focus-events\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
    while !w.is_null() {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).fd != -(1 as libc::c_int) {
                if focus != 0 {
                    server_client_check_focus(wp);
                }
                server_client_check_resize(wp);
            }
            (*wp).flags &= !(0x1 as libc::c_int);
            wp = (*wp).entry.tqe_next;
        }
        check_window_name(w);
        w = windows_RB_NEXT(w);
    }
}
unsafe extern "C" fn server_client_resize_force(
    mut wp: *mut window_pane,
) -> libc::c_int {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 100000 as libc::c_int as __suseconds_t,
        };
        init
    };
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if (*wp).flags & 0x10 as libc::c_int != 0 {
        (*wp).flags &= !(0x10 as libc::c_int);
        return 0 as libc::c_int;
    }
    if (*wp).sx != (*wp).osx || (*wp).sy != (*wp).osy
        || (*wp).sx <= 1 as libc::c_int as u_int || (*wp).sy <= 1 as libc::c_int as u_int
    {
        return 0 as libc::c_int;
    }
    memset(
        &mut ws as *mut winsize as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<winsize>() as libc::c_ulong,
    );
    ws.ws_col = (*wp).sx as libc::c_ushort;
    ws.ws_row = ((*wp).sy).wrapping_sub(1 as libc::c_int as u_int) as libc::c_ushort;
    if (*wp).fd != -(1 as libc::c_int)
        && ioctl(
            (*wp).fd,
            0x5414 as libc::c_int as libc::c_ulong,
            &mut ws as *mut winsize,
        ) == -(1 as libc::c_int)
    {
        fatal(b"ioctl failed\0" as *const u8 as *const libc::c_char);
    }
    log_debug(
        b"%s: %%%u forcing resize\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"server_client_resize_force\0"))
            .as_ptr(),
        (*wp).id,
    );
    event_add(&mut (*wp).resize_timer, &mut tv);
    (*wp).flags |= 0x10 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn server_client_resize_event(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    event_del(&mut (*wp).resize_timer);
    if (*wp).flags & 0x8 as libc::c_int == 0 {
        return;
    }
    if server_client_resize_force(wp) != 0 {
        return;
    }
    memset(
        &mut ws as *mut winsize as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<winsize>() as libc::c_ulong,
    );
    ws.ws_col = (*wp).sx as libc::c_ushort;
    ws.ws_row = (*wp).sy as libc::c_ushort;
    if (*wp).fd != -(1 as libc::c_int)
        && ioctl(
            (*wp).fd,
            0x5414 as libc::c_int as libc::c_ulong,
            &mut ws as *mut winsize,
        ) == -(1 as libc::c_int)
    {
        fatal(b"ioctl failed\0" as *const u8 as *const libc::c_char);
    }
    log_debug(
        b"%s: %%%u resize to %u,%u\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"server_client_resize_event\0"))
            .as_ptr(),
        (*wp).id,
        (*wp).sx,
        (*wp).sy,
    );
    (*wp).flags &= !(0x8 as libc::c_int);
    (*wp).osx = (*wp).sx;
    (*wp).osy = (*wp).sy;
}
unsafe extern "C" fn server_client_check_resize(mut wp: *mut window_pane) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 250000 as libc::c_int as __suseconds_t,
        };
        init
    };
    if (*wp).flags & 0x8 as libc::c_int == 0 {
        return;
    }
    log_debug(
        b"%s: %%%u resize to %u,%u\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 27],
            &[libc::c_char; 27],
        >(b"server_client_check_resize\0"))
            .as_ptr(),
        (*wp).id,
        (*wp).sx,
        (*wp).sy,
    );
    if event_initialized(&mut (*wp).resize_timer) == 0 {
        event_set(
            &mut (*wp).resize_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                server_client_resize_event
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            wp as *mut libc::c_void,
        );
    }
    if event_pending(
        &mut (*wp).resize_timer,
        0x1 as libc::c_int as libc::c_short,
        0 as *mut timeval,
    ) == 0
    {
        server_client_resize_event(
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            wp as *mut libc::c_void,
        );
    }
    if !((*wp).saved_grid).is_null()
        && event_pending(
            &mut (*wp).resize_timer,
            0x1 as libc::c_int as libc::c_short,
            0 as *mut timeval,
        ) != 0
    {
        return;
    }
    event_del(&mut (*wp).resize_timer);
    event_add(&mut (*wp).resize_timer, &mut tv);
}
unsafe extern "C" fn server_client_check_focus(mut wp: *mut window_pane) {
    let mut current_block: u64;
    let mut c: *mut client = 0 as *mut client;
    let mut push: libc::c_int = 0;
    push = (*wp).flags & 0x20 as libc::c_int;
    (*wp).flags &= !(0x20 as libc::c_int);
    if (*wp).base.mode & 0x800 as libc::c_int == 0 {
        return;
    }
    if !((*(*wp).window).active != wp) {
        if !((*wp).screen != &mut (*wp).base as *mut screen) {
            c = clients.tqh_first;
            loop {
                if c.is_null() {
                    current_block = 5678390700704197724;
                    break;
                }
                if !(((*c).session).is_null() || (*c).flags & 0x8000 as libc::c_int == 0)
                {
                    if !((*(*c).session).flags & 0x1 as libc::c_int != 0) {
                        if (*(*(*c).session).curw).window == (*wp).window {
                            current_block = 15899604225187311481;
                            break;
                        }
                    }
                }
                c = (*c).entry.tqe_next;
            }
            match current_block {
                5678390700704197724 => {}
                _ => {
                    if push != 0 || (*wp).flags & 0x4 as libc::c_int == 0 {
                        bufferevent_write(
                            (*wp).event,
                            b"\x1B[I\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            3 as libc::c_int as size_t,
                        );
                    }
                    (*wp).flags |= 0x4 as libc::c_int;
                    return;
                }
            }
        }
    }
    if push != 0 || (*wp).flags & 0x4 as libc::c_int != 0 {
        bufferevent_write(
            (*wp).event,
            b"\x1B[O\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
    }
    (*wp).flags &= !(0x4 as libc::c_int);
}
unsafe extern "C" fn server_client_reset_state(mut c: *mut client) {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = (*w).active;
    let mut loop_0: *mut window_pane = 0 as *mut window_pane;
    let mut s: *mut screen = (*wp).screen;
    let mut oo: *mut options = (*(*c).session).options;
    let mut lines: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    if (*c).flags & (0x2000 as libc::c_int | 0x40 as libc::c_int) != 0 {
        return;
    }
    tty_region_off(&mut (*c).tty);
    tty_margin_off(&mut (*c).tty);
    if status_at_line(c) != 0 as libc::c_int {
        lines = 0 as libc::c_int;
    } else {
        lines = status_line_size((*c).session) as libc::c_int;
    }
    if window_pane_visible(wp) == 0
        || ((*wp).yoff).wrapping_add((*s).cy)
            >= ((*c).tty.sy).wrapping_sub(lines as u_int)
    {
        tty_cursor(&mut (*c).tty, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
    } else {
        tty_cursor(
            &mut (*c).tty,
            ((*wp).xoff).wrapping_add((*s).cx),
            (lines as u_int).wrapping_add((*wp).yoff).wrapping_add((*s).cy),
        );
    }
    mode = (*s).mode;
    if options_get_number(oo, b"mouse\0" as *const u8 as *const libc::c_char) != 0 {
        mode &= !(0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int);
        loop_0 = (*w).panes.tqh_first;
        while !loop_0.is_null() {
            if (*(*loop_0).screen).mode & 0x1000 as libc::c_int != 0 {
                mode |= 0x1000 as libc::c_int;
            }
            loop_0 = (*loop_0).entry.tqe_next;
        }
        if !mode & 0x1000 as libc::c_int != 0 {
            mode |= 0x40 as libc::c_int;
        }
    }
    if !((*c).prompt_string).is_null() {
        mode &= !(0x400 as libc::c_int);
    }
    tty_update_mode(&mut (*c).tty, mode, s);
    tty_reset(&mut (*c).tty);
}
unsafe extern "C" fn server_client_repeat_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    if (*c).flags & 0x20 as libc::c_int != 0 {
        server_client_set_key_table(c, 0 as *const libc::c_char);
        (*c).flags &= !(0x20 as libc::c_int);
        server_status_client(c);
    }
}
unsafe extern "C" fn server_client_click_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    (*c).flags &= !(0x100000 as libc::c_int | 0x200000 as libc::c_int);
}
unsafe extern "C" fn server_client_check_exit(mut c: *mut client) {
    if (*c).flags & 0x4 as libc::c_int == 0 {
        return;
    }
    if evbuffer_get_length((*c).stdin_data) != 0 as libc::c_int as size_t {
        return;
    }
    if evbuffer_get_length((*c).stdout_data) != 0 as libc::c_int as size_t {
        return;
    }
    if evbuffer_get_length((*c).stderr_data) != 0 as libc::c_int as size_t {
        return;
    }
    if (*c).flags & 0x80 as libc::c_int != 0 {
        notify_client(b"client-detached\0" as *const u8 as *const libc::c_char, c);
    }
    proc_send(
        (*c).peer,
        MSG_EXIT,
        -(1 as libc::c_int),
        &mut (*c).retval as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    (*c).flags &= !(0x4 as libc::c_int);
}
unsafe extern "C" fn server_client_redraw_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    log_debug(b"redraw timer fired\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn server_client_check_redraw(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut needed: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut masked: libc::c_int = 0;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 1000 as libc::c_int as __suseconds_t,
        };
        init
    };
    static mut ev: event = event {
        ev_evcallback: event_callback {
            evcb_active_next: C2RustUnnamed_7 {
                tqe_next: 0 as *const event_callback as *mut event_callback,
                tqe_prev: 0 as *const *mut event_callback as *mut *mut event_callback,
            },
            evcb_flags: 0,
            evcb_pri: 0,
            evcb_closure: 0,
            evcb_cb_union: C2RustUnnamed_6 {
                evcb_callback: None,
            },
            evcb_arg: 0 as *const libc::c_void as *mut libc::c_void,
        },
        ev_timeout_pos: C2RustUnnamed_4 {
            ev_next_with_common_timeout: C2RustUnnamed_5 {
                tqe_next: 0 as *const event as *mut event,
                tqe_prev: 0 as *const *mut event as *mut *mut event,
            },
        },
        ev_fd: 0,
        ev_base: 0 as *const event_base as *mut event_base,
        ev_: C2RustUnnamed {
            ev_io: C2RustUnnamed_2 {
                ev_io_next: C2RustUnnamed_3 {
                    le_next: 0 as *const event as *mut event,
                    le_prev: 0 as *const *mut event as *mut *mut event,
                },
                ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
            },
        },
        ev_events: 0,
        ev_res: 0,
        ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
    };
    let mut left: size_t = 0;
    if (*c).flags & (0x2000 as libc::c_int | 0x40 as libc::c_int) != 0 {
        return;
    }
    needed = 0 as libc::c_int;
    if (*c).flags & 0x8 as libc::c_int != 0 {
        needed = 1 as libc::c_int;
    } else {
        wp = (*(*(*(*c).session).curw).window).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).flags & 0x1 as libc::c_int != 0 {
                needed = 1 as libc::c_int;
                break;
            } else {
                wp = (*wp).entry.tqe_next;
            }
        }
    }
    if needed != 0
        && {
            left = evbuffer_get_length((*tty).out);
            left != 0 as libc::c_int as size_t
        }
    {
        log_debug(
            b"%s: redraw deferred (%zu left)\0" as *const u8 as *const libc::c_char,
            (*c).name,
            left,
        );
        if event_initialized(&mut ev) == 0 {
            event_set(
                &mut ev,
                -(1 as libc::c_int),
                0 as libc::c_int as libc::c_short,
                Some(
                    server_client_redraw_timer
                        as unsafe extern "C" fn(
                            libc::c_int,
                            libc::c_short,
                            *mut libc::c_void,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
        }
        if event_pending(&mut ev, 0x1 as libc::c_int as libc::c_short, 0 as *mut timeval)
            == 0
        {
            log_debug(b"redraw timer started\0" as *const u8 as *const libc::c_char);
            event_add(&mut ev, &mut tv);
        }
        (*c).flags |= 0x8 as libc::c_int;
        return;
    } else if needed != 0 {
        log_debug(b"%s: redraw needed\0" as *const u8 as *const libc::c_char, (*c).name);
    }
    if (*c).flags & (0x8 as libc::c_int | 0x10 as libc::c_int) != 0 {
        if options_get_number(
            (*s).options,
            b"set-titles\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            server_client_set_title(c);
        }
        screen_redraw_update(c);
    }
    flags = (*tty).flags
        & (0x80 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int);
    (*tty)
        .flags = (*tty).flags & !(0x80 as libc::c_int | 0x2 as libc::c_int)
        | 0x1 as libc::c_int;
    if (*c).flags & 0x8 as libc::c_int != 0 {
        tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
        screen_redraw_screen(c, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
        (*c).flags &= !(0x10 as libc::c_int | 0x400 as libc::c_int);
    } else {
        wp = (*(*(*(*c).session).curw).window).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).flags & 0x1 as libc::c_int != 0 {
                tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
                screen_redraw_pane(c, wp);
            }
            wp = (*wp).entry.tqe_next;
        }
    }
    masked = (*c).flags & (0x400 as libc::c_int | 0x10 as libc::c_int);
    if masked != 0 as libc::c_int {
        tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
    }
    if masked == 0x400 as libc::c_int {
        screen_redraw_screen(c, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    } else if masked == 0x10 as libc::c_int {
        screen_redraw_screen(c, 0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    } else if masked != 0 as libc::c_int {
        screen_redraw_screen(c, 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    }
    (*tty).flags = (*tty).flags & !(0x2 as libc::c_int | 0x1 as libc::c_int) | flags;
    tty_update_mode(tty, (*tty).mode, 0 as *mut screen);
    (*c).flags
        &= !(0x8 as libc::c_int | 0x400 as libc::c_int | 0x10 as libc::c_int
            | 0x80000 as libc::c_int);
    if needed != 0 {
        (*c).redraw = evbuffer_get_length((*tty).out);
        log_debug(
            b"%s: redraw added %zu bytes\0" as *const u8 as *const libc::c_char,
            (*c).name,
            (*c).redraw,
        );
    }
}
unsafe extern "C" fn server_client_set_title(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    template = options_get_string(
        (*s).options,
        b"set-titles-string\0" as *const u8 as *const libc::c_char,
    );
    ft = format_create(c, 0 as *mut cmdq_item, 0 as libc::c_int, 0 as libc::c_int);
    format_defaults(ft, c, 0 as *mut session, 0 as *mut winlink, 0 as *mut window_pane);
    title = format_expand_time(ft, template, time(0 as *mut time_t));
    if ((*c).title).is_null() || strcmp(title, (*c).title) != 0 as libc::c_int {
        free((*c).title as *mut libc::c_void);
        (*c).title = xstrdup(title);
        tty_set_title(&mut (*c).tty, (*c).title);
    }
    free(title as *mut libc::c_void);
    format_free(ft);
}
unsafe extern "C" fn server_client_dispatch(
    mut imsg: *mut imsg,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    let mut stdindata: msg_stdin_data = msg_stdin_data {
        size: 0,
        data: [0; 8192],
    };
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut datalen: ssize_t = 0;
    let mut s: *mut session = 0 as *mut session;
    if (*c).flags & 0x200 as libc::c_int != 0 {
        return;
    }
    if imsg.is_null() {
        server_client_lost(c);
        return;
    }
    data = (*imsg).data as *const libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        100 | 101 | 102 | 108 | 104 | 105 | 107 | 106 => {
            server_client_dispatch_identify(c, imsg);
        }
        200 => {
            server_client_dispatch_command(c, imsg);
        }
        212 => {
            if datalen as libc::c_ulong
                != ::core::mem::size_of::<msg_stdin_data>() as libc::c_ulong
            {
                fatalx(b"bad MSG_STDIN size\0" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut stdindata as *mut msg_stdin_data as *mut libc::c_void,
                data as *const libc::c_void,
                ::core::mem::size_of::<msg_stdin_data>() as libc::c_ulong,
            );
            if !((*c).stdin_callback).is_none() {
                if stdindata.size <= 0 as libc::c_int as ssize_t {
                    (*c).stdin_closed = 1 as libc::c_int;
                } else {
                    evbuffer_add(
                        (*c).stdin_data,
                        (stdindata.data).as_mut_ptr() as *const libc::c_void,
                        stdindata.size as size_t,
                    );
                }
                ((*c).stdin_callback)
                    .expect(
                        "non-null function pointer",
                    )(c, (*c).stdin_closed, (*c).stdin_callback_data);
            }
        }
        208 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_RESIZE size\0" as *const u8 as *const libc::c_char);
            }
            if !((*c).flags & 0x2000 as libc::c_int != 0) {
                tty_resize(&mut (*c).tty);
                recalculate_sizes();
                server_redraw_client(c);
                if !((*c).session).is_null() {
                    notify_client(
                        b"client-resized\0" as *const u8 as *const libc::c_char,
                        c,
                    );
                }
            }
        }
        205 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_EXITING size\0" as *const u8 as *const libc::c_char);
            }
            (*c).session = 0 as *mut session;
            tty_close(&mut (*c).tty);
            proc_send(
                (*c).peer,
                MSG_EXITED,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        216 | 215 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_WAKEUP size\0" as *const u8 as *const libc::c_char);
            }
            if !((*c).flags & 0x40 as libc::c_int == 0) {
                (*c).flags &= !(0x40 as libc::c_int);
                if !((*c).tty.fd == -(1 as libc::c_int)) {
                    s = (*c).session;
                    if gettimeofday(&mut (*c).activity_time, 0 as *mut libc::c_void)
                        != 0 as libc::c_int
                    {
                        fatal(
                            b"gettimeofday failed\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    tty_start_tty(&mut (*c).tty);
                    server_redraw_client(c);
                    recalculate_sizes();
                    if !s.is_null() {
                        session_update_activity(s, &mut (*c).activity_time);
                    }
                }
            }
        }
        209 => {
            if datalen != 0 as libc::c_int as ssize_t {
                fatalx(b"bad MSG_SHELL size\0" as *const u8 as *const libc::c_char);
            }
            server_client_dispatch_shell(c);
        }
        _ => {}
    };
}
unsafe extern "C" fn server_client_command_done(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    let mut c: *mut client = (*item).client;
    if !(*c).flags & 0x80 as libc::c_int != 0 {
        (*c).flags |= 0x4 as libc::c_int;
    }
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn server_client_command_error(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    let mut error: *mut libc::c_char = data as *mut libc::c_char;
    cmdq_error(item, b"%s\0" as *const u8 as *const libc::c_char, error);
    free(error as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn server_client_dispatch_command(
    mut c: *mut client,
    mut imsg: *mut imsg,
) {
    let mut data: msg_command_data = msg_command_data { argc: 0 };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*c).flags & 0x4 as libc::c_int != 0 {
        return;
    }
    if ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong)
        < ::core::mem::size_of::<msg_command_data>() as libc::c_ulong
    {
        fatalx(b"bad MSG_COMMAND size\0" as *const u8 as *const libc::c_char);
    }
    memcpy(
        &mut data as *mut msg_command_data as *mut libc::c_void,
        (*imsg).data,
        ::core::mem::size_of::<msg_command_data>() as libc::c_ulong,
    );
    buf = ((*imsg).data as *mut libc::c_char)
        .offset(::core::mem::size_of::<msg_command_data>() as libc::c_ulong as isize);
    len = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<msg_command_data>() as libc::c_ulong);
    if len > 0 as libc::c_int as size_t
        && *buf.offset(len.wrapping_sub(1 as libc::c_int as size_t) as isize)
            as libc::c_int != '\0' as i32
    {
        fatalx(b"bad MSG_COMMAND string\0" as *const u8 as *const libc::c_char);
    }
    argc = data.argc;
    if cmd_unpack_argv(buf, len, argc, &mut argv) != 0 as libc::c_int {
        cause = xstrdup(b"command too long\0" as *const u8 as *const libc::c_char);
    } else {
        if argc == 0 as libc::c_int {
            argc = 1 as libc::c_int;
            argv = xcalloc(
                1 as libc::c_int as size_t,
                ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            *argv = xstrdup(b"new-session\0" as *const u8 as *const libc::c_char);
        }
        cmdlist = cmd_list_parse(
            argc,
            argv,
            0 as *const libc::c_char,
            0 as libc::c_int as u_int,
            &mut cause,
        );
        if cmdlist.is_null() {
            cmd_free_argv(argc, argv);
        } else {
            cmd_free_argv(argc, argv);
            cmdq_append(
                c,
                cmdq_get_command(
                    cmdlist,
                    0 as *mut cmd_find_state,
                    0 as *mut mouse_event,
                    0 as libc::c_int,
                ),
            );
            cmdq_append(
                c,
                cmdq_get_callback1(
                    b"server_client_command_done\0" as *const u8 as *const libc::c_char,
                    Some(
                        server_client_command_done
                            as unsafe extern "C" fn(
                                *mut cmdq_item,
                                *mut libc::c_void,
                            ) -> cmd_retval,
                    ),
                    0 as *mut libc::c_void,
                ),
            );
            cmd_list_free(cmdlist);
            return;
        }
    }
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"server_client_command_error\0" as *const u8 as *const libc::c_char,
            Some(
                server_client_command_error
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            cause as *mut libc::c_void,
        ),
    );
    if !cmdlist.is_null() {
        cmd_list_free(cmdlist);
    }
    (*c).flags |= 0x4 as libc::c_int;
}
unsafe extern "C" fn server_client_dispatch_identify(
    mut c: *mut client,
    mut imsg: *mut imsg,
) {
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut datalen: size_t = 0;
    let mut flags: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*c).flags & 0x40000 as libc::c_int != 0 {
        fatalx(b"out-of-order identify message\0" as *const u8 as *const libc::c_char);
    }
    data = (*imsg).data as *const libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    match (*imsg).hdr.type_0 {
        100 => {
            if datalen != ::core::mem::size_of::<libc::c_int>() as libc::c_ulong {
                fatalx(
                    b"bad MSG_IDENTIFY_FLAGS size\0" as *const u8 as *const libc::c_char,
                );
            }
            memcpy(
                &mut flags as *mut libc::c_int as *mut libc::c_void,
                data as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            (*c).flags |= flags;
            log_debug(
                b"client %p IDENTIFY_FLAGS %#x\0" as *const u8 as *const libc::c_char,
                c,
                flags,
            );
        }
        101 => {
            if datalen == 0 as libc::c_int as size_t
                || *data
                    .offset(datalen.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(
                    b"bad MSG_IDENTIFY_TERM string\0" as *const u8 as *const libc::c_char,
                );
            }
            (*c).term = xstrdup(data);
            log_debug(
                b"client %p IDENTIFY_TERM %s\0" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        102 => {
            if datalen == 0 as libc::c_int as size_t
                || *data
                    .offset(datalen.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(
                    b"bad MSG_IDENTIFY_TTYNAME string\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*c).ttyname = xstrdup(data);
            log_debug(
                b"client %p IDENTIFY_TTYNAME %s\0" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        108 => {
            if datalen == 0 as libc::c_int as size_t
                || *data
                    .offset(datalen.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(
                    b"bad MSG_IDENTIFY_CWD string\0" as *const u8 as *const libc::c_char,
                );
            }
            if access(data, 1 as libc::c_int) == 0 as libc::c_int {
                (*c).cwd = xstrdup(data);
            } else {
                home = find_home();
                if !home.is_null() {
                    (*c).cwd = xstrdup(home);
                } else {
                    (*c).cwd = xstrdup(b"/\0" as *const u8 as *const libc::c_char);
                }
            }
            log_debug(
                b"client %p IDENTIFY_CWD %s\0" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        104 => {
            if datalen != 0 as libc::c_int as size_t {
                fatalx(
                    b"bad MSG_IDENTIFY_STDIN size\0" as *const u8 as *const libc::c_char,
                );
            }
            (*c).fd = (*imsg).fd;
            log_debug(
                b"client %p IDENTIFY_STDIN %d\0" as *const u8 as *const libc::c_char,
                c,
                (*imsg).fd,
            );
        }
        105 => {
            if datalen == 0 as libc::c_int as size_t
                || *data
                    .offset(datalen.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int != '\0' as i32
            {
                fatalx(
                    b"bad MSG_IDENTIFY_ENVIRON string\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !(strchr(data, '=' as i32)).is_null() {
                environ_put((*c).environ, data);
            }
            log_debug(
                b"client %p IDENTIFY_ENVIRON %s\0" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        107 => {
            if datalen != ::core::mem::size_of::<pid_t>() as libc::c_ulong {
                fatalx(
                    b"bad MSG_IDENTIFY_CLIENTPID size\0" as *const u8
                        as *const libc::c_char,
                );
            }
            memcpy(
                &mut (*c).pid as *mut pid_t as *mut libc::c_void,
                data as *const libc::c_void,
                ::core::mem::size_of::<pid_t>() as libc::c_ulong,
            );
            log_debug(
                b"client %p IDENTIFY_CLIENTPID %ld\0" as *const u8
                    as *const libc::c_char,
                c,
                (*c).pid as libc::c_long,
            );
        }
        _ => {}
    }
    if (*imsg).hdr.type_0 != MSG_IDENTIFY_DONE as libc::c_int as uint32_t {
        return;
    }
    (*c).flags |= 0x40000 as libc::c_int;
    if *(*c).ttyname as libc::c_int != '\0' as i32 {
        name = xstrdup((*c).ttyname);
    } else {
        xasprintf(
            &mut name as *mut *mut libc::c_char,
            b"client-%ld\0" as *const u8 as *const libc::c_char,
            (*c).pid as libc::c_long,
        );
    }
    (*c).name = name;
    log_debug(
        b"client %p name is %s\0" as *const u8 as *const libc::c_char,
        c,
        (*c).name,
    );
    if (*c).flags & 0x2000 as libc::c_int != 0 {
        (*c)
            .stdin_callback = Some(
            control_callback
                as unsafe extern "C" fn(
                    *mut client,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        );
        evbuffer_free((*c).stderr_data);
        (*c).stderr_data = (*c).stdout_data;
        if (*c).flags & 0x4000 as libc::c_int != 0 {
            evbuffer_add_printf(
                (*c).stdout_data,
                b"\x1BP1000p\0" as *const u8 as *const libc::c_char,
            );
        }
        proc_send(
            (*c).peer,
            MSG_STDIN,
            -(1 as libc::c_int),
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        );
        (*c).tty.fd = -(1 as libc::c_int);
        close((*c).fd);
        (*c).fd = -(1 as libc::c_int);
        return;
    }
    if (*c).fd == -(1 as libc::c_int) {
        return;
    }
    if tty_init(&mut (*c).tty, c, (*c).fd, (*c).term) != 0 as libc::c_int {
        close((*c).fd);
        (*c).fd = -(1 as libc::c_int);
        return;
    }
    if (*c).flags & 0x10000 as libc::c_int != 0 {
        (*c).tty.flags |= 0x8 as libc::c_int;
    }
    if (*c).flags & 0x20000 as libc::c_int != 0 {
        (*c).tty.term_flags |= 0x1 as libc::c_int;
    }
    tty_resize(&mut (*c).tty);
    if (*c).flags & 0x2000 as libc::c_int == 0 {
        (*c).flags |= 0x1 as libc::c_int;
    }
}
unsafe extern "C" fn server_client_dispatch_shell(mut c: *mut client) {
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    shell = options_get_string(
        global_s_options,
        b"default-shell\0" as *const u8 as *const libc::c_char,
    );
    if *shell as libc::c_int == '\0' as i32 || areshell(shell) != 0 {
        shell = b"/bin/sh\0" as *const u8 as *const libc::c_char;
    }
    proc_send(
        (*c).peer,
        MSG_SHELL,
        -(1 as libc::c_int),
        shell as *const libc::c_void,
        (strlen(shell)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    proc_kill_peer((*c).peer);
}
unsafe extern "C" fn server_client_stdout_cb(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    if !(*c).flags & 0x200 as libc::c_int != 0 {
        server_client_push_stdout(c);
    }
    server_client_unref(c);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_push_stdout(mut c: *mut client) {
    let mut data: msg_stdout_data = msg_stdout_data {
        size: 0,
        data: [0; 8192],
    };
    let mut sent: size_t = 0;
    let mut left: size_t = 0;
    left = evbuffer_get_length((*c).stdout_data);
    while left != 0 as libc::c_int as size_t {
        sent = left;
        if sent > ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong {
            sent = ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong;
        }
        memcpy(
            (data.data).as_mut_ptr() as *mut libc::c_void,
            evbuffer_pullup((*c).stdout_data, -(1 as libc::c_int) as ssize_t)
                as *const libc::c_void,
            sent,
        );
        data.size = sent as ssize_t;
        if proc_send(
            (*c).peer,
            MSG_STDOUT,
            -(1 as libc::c_int),
            &mut data as *mut msg_stdout_data as *const libc::c_void,
            ::core::mem::size_of::<msg_stdout_data>() as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            break;
        }
        evbuffer_drain((*c).stdout_data, sent);
        left = evbuffer_get_length((*c).stdout_data);
        log_debug(
            b"%s: client %p, sent %zu, left %zu\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"server_client_push_stdout\0"))
                .as_ptr(),
            c,
            sent,
            left,
        );
    }
    if left != 0 as libc::c_int as size_t {
        (*c).references += 1;
        (*c).references;
        event_once(
            -(1 as libc::c_int),
            0x1 as libc::c_int as libc::c_short,
            Some(
                server_client_stdout_cb
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
            0 as *const timeval,
        );
        log_debug(
            b"%s: client %p, queued\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"server_client_push_stdout\0"))
                .as_ptr(),
            c,
        );
    }
}
unsafe extern "C" fn server_client_stderr_cb(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    if !(*c).flags & 0x200 as libc::c_int != 0 {
        server_client_push_stderr(c);
    }
    server_client_unref(c);
}
#[no_mangle]
pub unsafe extern "C" fn server_client_push_stderr(mut c: *mut client) {
    let mut data: msg_stderr_data = msg_stderr_data {
        size: 0,
        data: [0; 8192],
    };
    let mut sent: size_t = 0;
    let mut left: size_t = 0;
    if (*c).stderr_data == (*c).stdout_data {
        server_client_push_stdout(c);
        return;
    }
    left = evbuffer_get_length((*c).stderr_data);
    while left != 0 as libc::c_int as size_t {
        sent = left;
        if sent > ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong {
            sent = ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong;
        }
        memcpy(
            (data.data).as_mut_ptr() as *mut libc::c_void,
            evbuffer_pullup((*c).stderr_data, -(1 as libc::c_int) as ssize_t)
                as *const libc::c_void,
            sent,
        );
        data.size = sent as ssize_t;
        if proc_send(
            (*c).peer,
            MSG_STDERR,
            -(1 as libc::c_int),
            &mut data as *mut msg_stderr_data as *const libc::c_void,
            ::core::mem::size_of::<msg_stderr_data>() as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            break;
        }
        evbuffer_drain((*c).stderr_data, sent);
        left = evbuffer_get_length((*c).stderr_data);
        log_debug(
            b"%s: client %p, sent %zu, left %zu\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"server_client_push_stderr\0"))
                .as_ptr(),
            c,
            sent,
            left,
        );
    }
    if left != 0 as libc::c_int as size_t {
        (*c).references += 1;
        (*c).references;
        event_once(
            -(1 as libc::c_int),
            0x1 as libc::c_int as libc::c_short,
            Some(
                server_client_stderr_cb
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
            0 as *const timeval,
        );
        log_debug(
            b"%s: client %p, queued\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"server_client_push_stderr\0"))
                .as_ptr(),
            c,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_client_get_cwd(
    mut c: *mut client,
) -> *const libc::c_char {
    let mut s: *mut session = 0 as *mut session;
    if !c.is_null() && ((*c).session).is_null() && !((*c).cwd).is_null() {
        return (*c).cwd;
    }
    if !c.is_null()
        && {
            s = (*c).session;
            !s.is_null()
        } && !((*s).cwd).is_null()
    {
        return (*s).cwd;
    }
    return b".\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn server_client_get_path(
    mut c: *mut client,
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resolved: [libc::c_char; 4096] = [0; 4096];
    if *file as libc::c_int == '/' as i32 {
        path = xstrdup(file);
    } else {
        xasprintf(
            &mut path as *mut *mut libc::c_char,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            server_client_get_cwd(c),
            file,
        );
    }
    if (realpath(path, resolved.as_mut_ptr())).is_null() {
        return path;
    }
    free(path as *mut libc::c_void);
    return xstrdup(resolved.as_mut_ptr());
}
