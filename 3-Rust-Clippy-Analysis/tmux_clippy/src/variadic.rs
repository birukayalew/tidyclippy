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
    pub type input_state;
    pub type screen_write_collect_line;
    pub type screen_write_collect_item;
    pub type tty_code;
    pub type format_job_tree;
    pub type tmuxpeer;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getprogname() -> *const libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn evbuffer_add(
        buf: *mut evbuffer,
        data: *const libc::c_void,
        datlen: size_t,
    ) -> libc::c_int;
    fn evbuffer_add_vprintf(
        buf: *mut evbuffer,
        fmt: *const libc::c_char,
        ap: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xvsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    static mut global_options: *mut options;
    fn format_create(
        _: *mut client,
        _: *mut cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut format_tree;
    fn hooks_find(_: *mut hooks, _: *const libc::c_char) -> *mut hook;
    fn options_default(
        _: *mut options,
        _: *const options_table_entry,
    ) -> *mut options_entry;
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn environ_find(_: *mut environ, _: *const libc::c_char) -> *mut environ_entry;
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut cmd_find_state,
        _: *mut mouse_event,
        _: libc::c_int,
    ) -> *mut cmdq_item;
    fn cmdq_insert_after(_: *mut cmdq_item, _: *mut cmdq_item);
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn server_client_push_stdout(_: *mut client);
    fn server_client_push_stderr(_: *mut client);
    fn status_message_clear(_: *mut client);
    fn screen_write_vnputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    );
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const grid_cell, _: u_char);
    fn screen_write_cell(_: *mut screen_write_ctx, _: *const grid_cell);
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    fn window_pane_set_mode(
        _: *mut window_pane,
        _: *const window_mode,
        _: *mut cmd_find_state,
        _: *mut args,
    ) -> libc::c_int;
    fn window_pane_reset_mode(_: *mut window_pane);
    static window_copy_mode: window_mode;
    fn window_copy_init_for_output(_: *mut window_pane);
    fn window_copy_vadd(
        _: *mut window_pane,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    );
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    fn utf8_sanitize(_: *const libc::c_char) -> *mut libc::c_char;
    fn style_parse(
        _: *const grid_cell,
        _: *mut grid_cell,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn log_vwrite(_: *const libc::c_char, _: ::core::ffi::VaList);
    fn options_add(oo: *mut options, name: *const libc::c_char) -> *mut options_entry;
    fn options_parent_table_entry(
        oo: *mut options,
        s: *const libc::c_char,
    ) -> *const options_table_entry;
    fn environ_RB_INSERT(_: *mut environ, _: *mut environ_entry) -> *mut environ_entry;
    fn environ_RB_NEXT(_: *mut environ_entry) -> *mut environ_entry;
    fn environ_RB_MINMAX(_: *mut environ, _: libc::c_int) -> *mut environ_entry;
    fn format_entry_tree_RB_INSERT(
        _: *mut format_entry_tree,
        _: *mut format_entry,
    ) -> *mut format_entry;
    fn status_message_callback(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void);
    static mut cfg_causes: *mut *mut libc::c_char;
    static mut cfg_ncauses: u_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
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
pub type va_list = __builtin_va_list;
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
pub type uint8_t = __uint8_t;
pub type bitstr_t = libc::c_uchar;
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
    pub args: C2RustUnnamed_34,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_tree {
    pub w: *mut window,
    pub wl: *mut winlink,
    pub s: *mut session,
    pub wp: *mut window_pane,
    pub client: *mut client,
    pub tag: u_int,
    pub flags: libc::c_int,
    pub tree: format_entry_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_entry_tree {
    pub rbh_root: *mut format_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_entry {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub t: time_t,
    pub cb: format_cb,
    pub entry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut format_entry,
    pub rbe_right: *mut format_entry,
    pub rbe_parent: *mut format_entry,
    pub rbe_color: libc::c_int,
}
pub type format_cb = Option::<
    unsafe extern "C" fn(*mut format_tree, *mut format_entry) -> (),
>;
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
pub struct C2RustUnnamed_34 {
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
    pub term_type: C2RustUnnamed_35,
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
pub type C2RustUnnamed_35 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_35 = 6;
pub const TTY_VT420: C2RustUnnamed_35 = 5;
pub const TTY_VT320: C2RustUnnamed_35 = 4;
pub const TTY_VT220: C2RustUnnamed_35 = 3;
pub const TTY_VT102: C2RustUnnamed_35 = 2;
pub const TTY_VT101: C2RustUnnamed_35 = 1;
pub const TTY_VT100: C2RustUnnamed_35 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_36,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_entry {
    pub owner: *mut options,
    pub name: *const libc::c_char,
    pub tableentry: *const options_table_entry,
    pub c2rust_unnamed: C2RustUnnamed_38,
    pub entry: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub rbe_left: *mut options_entry,
    pub rbe_right: *mut options_entry,
    pub rbe_parent: *mut options_entry,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_38 {
    pub string: *mut libc::c_char,
    pub number: libc::c_longlong,
    pub style: grid_cell,
    pub c2rust_unnamed: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub array: *mut *const libc::c_char,
    pub arraysize: u_int,
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hook {
    pub name: *const libc::c_char,
    pub cmdlist: *mut cmd_list,
    pub entry: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub rbe_left: *mut hook,
    pub rbe_right: *mut hook,
    pub rbe_parent: *mut hook,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn args_print_add(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    ap = args.clone();
    slen = xvasprintf(&mut s, fmt, ap.as_va_list()) as size_t;
    *len = (*len).wrapping_add(slen);
    *buf = xrealloc(*buf as *mut libc::c_void, *len) as *mut libc::c_char;
    strlcat(*buf, s, *len);
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_add_cause(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    cfg_ncauses = cfg_ncauses.wrapping_add(1);
    cfg_ncauses;
    cfg_causes = xreallocarray(
        cfg_causes as *mut libc::c_void,
        cfg_ncauses as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let ref mut fresh0 = *cfg_causes
        .offset(cfg_ncauses.wrapping_sub(1 as libc::c_int as u_int) as isize);
    *fresh0 = msg;
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_format(
    mut item: *mut cmdq_item,
    mut key: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut shared: *mut cmdq_shared = (*item).shared;
    let mut ap: ::core::ffi::VaListImpl;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut value, fmt, ap.as_va_list());
    if ((*shared).formats).is_null() {
        (*shared)
            .formats = format_create(
            0 as *mut client,
            0 as *mut cmdq_item,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    format_add(
        (*shared).formats,
        key,
        b"%s\0" as *const u8 as *const libc::c_char,
        value,
    );
    free(value as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_print(
    mut item: *mut cmdq_item,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut c: *mut client = (*item).client;
    let mut w: *mut window = 0 as *mut window;
    let mut ap: ::core::ffi::VaListImpl;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    if !c.is_null() {
        if ((*c).session).is_null() || (*c).flags & 0x2000 as libc::c_int != 0 {
            if !(*c).flags & 0x10000 as libc::c_int != 0 {
                xvasprintf(&mut tmp, fmt, ap.as_va_list());
                msg = utf8_sanitize(tmp);
                free(tmp as *mut libc::c_void);
                evbuffer_add((*c).stdout_data, msg as *const libc::c_void, strlen(msg));
                free(msg as *mut libc::c_void);
            } else {
                evbuffer_add_vprintf((*c).stdout_data, fmt, ap.as_va_list());
            }
            evbuffer_add(
                (*c).stdout_data,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            server_client_push_stdout(c);
        } else {
            w = (*(*(*c).session).curw).window;
            if (*(*w).active).mode != &window_copy_mode as *const window_mode {
                window_pane_reset_mode((*w).active);
                window_pane_set_mode(
                    (*w).active,
                    &window_copy_mode,
                    0 as *mut cmd_find_state,
                    0 as *mut args,
                );
                window_copy_init_for_output((*w).active);
            }
            window_copy_vadd((*w).active, fmt, ap.as_va_list());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn control_write(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    evbuffer_add_vprintf((*c).stdout_data, fmt, ap.as_va_list());
    evbuffer_add(
        (*c).stdout_data,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    server_client_push_stdout(c);
}
#[no_mangle]
pub unsafe extern "C" fn environ_set(
    mut env: *mut environ,
    mut name: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    envent = environ_find(env, name);
    if !envent.is_null() {
        free((*envent).value as *mut libc::c_void);
        xvasprintf(&mut (*envent).value, fmt, ap.as_va_list());
    } else {
        envent = xmalloc(::core::mem::size_of::<environ_entry>() as libc::c_ulong)
            as *mut environ_entry;
        (*envent).name = xstrdup(name);
        xvasprintf(&mut (*envent).value, fmt, ap.as_va_list());
        environ_RB_INSERT(env, envent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn environ_log(
    mut env: *mut environ,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut ap: ::core::ffi::VaListImpl;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    vasprintf(&mut prefix, fmt, ap.as_va_list());
    envent = environ_RB_MINMAX(env, -(1 as libc::c_int));
    while !envent.is_null() {
        if !((*envent).value).is_null() && *(*envent).name as libc::c_int != '\0' as i32
        {
            log_debug(
                b"%s%s=%s\0" as *const u8 as *const libc::c_char,
                prefix,
                (*envent).name,
                (*envent).value,
            );
        }
        envent = environ_RB_NEXT(envent);
    }
    free(prefix as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn format_add(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    let mut ap: ::core::ffi::VaListImpl;
    fe = xmalloc(::core::mem::size_of::<format_entry>() as libc::c_ulong)
        as *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now = format_entry_tree_RB_INSERT(&mut (*ft).tree, fe);
    if !fe_now.is_null() {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now;
    }
    (*fe).cb = None;
    (*fe).t = 0 as libc::c_int as time_t;
    ap = args.clone();
    xvasprintf(&mut (*fe).value, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn hooks_insert(
    mut hooks: *mut hooks,
    mut item: *mut cmdq_item,
    mut fs: *mut cmd_find_state,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut hook: *mut hook = 0 as *mut hook;
    let mut ap: ::core::ffi::VaListImpl;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    if (*item).flags & 0x4 as libc::c_int != 0 {
        return;
    }
    ap = args.clone();
    xvasprintf(&mut name, fmt, ap.as_va_list());
    hook = hooks_find(hooks, name);
    if hook.is_null() {
        free(name as *mut libc::c_void);
        return;
    }
    log_debug(
        b"running hook %s (parent %p)\0" as *const u8 as *const libc::c_char,
        name,
        item,
    );
    new_item = cmdq_get_command(
        (*hook).cmdlist,
        fs,
        0 as *mut mouse_event,
        0x4 as libc::c_int,
    );
    cmdq_format(
        new_item,
        b"hook\0" as *const u8 as *const libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    if !item.is_null() {
        cmdq_insert_after(item, new_item);
    } else {
        cmdq_append(0 as *mut client, new_item);
    }
    free(name as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn input_reply(
    mut ictx: *mut input_ctx,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut reply: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut reply, fmt, ap.as_va_list());
    bufferevent_write((*(*ictx).wp).event, reply as *const libc::c_void, strlen(reply));
    free(reply as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn log_debug(mut msg: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    log_vwrite(msg, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut msg: *const libc::c_char, mut args: ...) -> ! {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if asprintf(
        &mut fmt as *mut *mut libc::c_char,
        b"fatal: %s: %s\0" as *const u8 as *const libc::c_char,
        msg,
        strerror(*__errno_location()),
    ) == -(1 as libc::c_int)
    {
        exit(1 as libc::c_int);
    }
    log_vwrite(fmt, ap.as_va_list());
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fatalx(mut msg: *const libc::c_char, mut args: ...) -> ! {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if asprintf(
        &mut fmt as *mut *mut libc::c_char,
        b"fatal: %s\0" as *const u8 as *const libc::c_char,
        msg,
    ) == -(1 as libc::c_int)
    {
        exit(1 as libc::c_int);
    }
    log_vwrite(fmt, ap.as_va_list());
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn options_set_string(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut append: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut ap: ::core::ffi::VaListImpl;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut s, fmt, ap.as_va_list());
    o = options_get_only(oo, name);
    if !o.is_null() && append != 0
        && (((*o).tableentry).is_null()
            || (*(*o).tableentry).type_0 as libc::c_uint
                == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint)
    {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*o).c2rust_unnamed.string,
            s,
        );
        free(s as *mut libc::c_void);
    } else {
        value = s;
    }
    if o.is_null() && *name as libc::c_int == '@' as i32 {
        o = options_add(oo, name);
    } else if o.is_null() {
        o = options_default(oo, options_parent_table_entry(oo, name));
        if o.is_null() {
            return 0 as *mut options_entry;
        }
    }
    if !(((*o).tableentry).is_null()
        || (*(*o).tableentry).type_0 as libc::c_uint
            == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint)
    {
        fatalx(b"option %s is not a string\0" as *const u8 as *const libc::c_char, name);
    }
    free((*o).c2rust_unnamed.string as *mut libc::c_void);
    (*o).c2rust_unnamed.string = value;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cstrlen(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut ap: ::core::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    msg2 = xmalloc((strlen(msg)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    ptr = msg;
    ptr2 = msg2;
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            && *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '[' as i32
        {
            while *ptr as libc::c_int != ']' as i32 && *ptr as libc::c_int != '\0' as i32
            {
                ptr = ptr.offset(1);
                ptr;
            }
            if *ptr as libc::c_int == ']' as i32 {
                ptr = ptr.offset(1);
                ptr;
            }
        } else {
            let fresh1 = ptr;
            ptr = ptr.offset(1);
            let fresh2 = ptr2;
            ptr2 = ptr2.offset(1);
            *fresh2 = *fresh1;
        }
    }
    *ptr2 = '\0' as i32 as libc::c_char;
    size = screen_write_strlen(b"%s\0" as *const u8 as *const libc::c_char, msg2);
    free(msg as *mut libc::c_void);
    free(msg2 as *mut libc::c_void);
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_strlen(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut ap: ::core::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ud: utf8_data = utf8_data {
        data: [0; 9],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut left: size_t = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    ptr = msg as *mut u_char;
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr as libc::c_int > 0x7f as libc::c_int
            && utf8_open(&mut ud, *ptr) as libc::c_uint
                == UTF8_MORE as libc::c_int as libc::c_uint
        {
            ptr = ptr.offset(1);
            ptr;
            left = strlen(ptr as *const libc::c_char);
            if left < (ud.size as size_t).wrapping_sub(1 as libc::c_int as size_t) {
                break;
            }
            loop {
                more = utf8_append(&mut ud, *ptr);
                if !(more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint) {
                    break;
                }
                ptr = ptr.offset(1);
                ptr;
            }
            ptr = ptr.offset(1);
            ptr;
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                size = size.wrapping_add(ud.width as size_t);
            }
        } else {
            if *ptr as libc::c_int > 0x1f as libc::c_int
                && (*ptr as libc::c_int) < 0x7f as libc::c_int
            {
                size = size.wrapping_add(1);
                size;
            }
            ptr = ptr.offset(1);
            ptr;
        }
    }
    free(msg as *mut libc::c_void);
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_puts(
    mut ctx: *mut screen_write_ctx,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    screen_write_vnputs(ctx, -(1 as libc::c_int) as ssize_t, gcp, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_nputs(
    mut ctx: *mut screen_write_ctx,
    mut maxlen: ssize_t,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    screen_write_vnputs(ctx, maxlen, gcp, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_cnputs(
    mut ctx: *mut screen_write_ctx,
    mut maxlen: ssize_t,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut args: ...
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
    let mut ap: ::core::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut last: *mut u_char = 0 as *mut u_char;
    let mut left: size_t = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        gcp as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    ptr = msg as *mut u_char;
    while *ptr as libc::c_int != '\0' as i32 {
        if *ptr.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            && *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '[' as i32
        {
            ptr = ptr.offset(2 as libc::c_int as isize);
            last = ptr
                .offset(
                    strcspn(
                        ptr as *const libc::c_char,
                        b"]\0" as *const u8 as *const libc::c_char,
                    ) as isize,
                );
            if *last as libc::c_int == '\0' as i32 {
                break;
            }
            *last = '\0' as i32 as u_char;
            style_parse(gcp, &mut gc, ptr as *const libc::c_char);
            ptr = last.offset(1 as libc::c_int as isize);
        } else if *ptr as libc::c_int > 0x7f as libc::c_int
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
            if *ptr as libc::c_int > 0x1f as libc::c_int
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
pub unsafe extern "C" fn server_client_add_message(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut msg: *mut message_entry = 0 as *mut message_entry;
    let mut msg1: *mut message_entry = 0 as *mut message_entry;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    let mut limit: u_int = 0;
    ap = args.clone();
    xvasprintf(&mut s, fmt, ap.as_va_list());
    log_debug(b"message %s (client %p)\0" as *const u8 as *const libc::c_char, s, c);
    msg = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<message_entry>() as libc::c_ulong,
    ) as *mut message_entry;
    (*msg).msg_time = time(0 as *mut time_t);
    let fresh3 = (*c).message_next;
    (*c).message_next = ((*c).message_next).wrapping_add(1);
    (*msg).msg_num = fresh3;
    (*msg).msg = s;
    (*msg).entry.tqe_next = 0 as *mut message_entry;
    (*msg).entry.tqe_prev = (*c).message_log.tqh_last;
    *(*c).message_log.tqh_last = msg;
    (*c).message_log.tqh_last = &mut (*msg).entry.tqe_next;
    limit = options_get_number(
        global_options,
        b"message-limit\0" as *const u8 as *const libc::c_char,
    ) as u_int;
    msg = (*c).message_log.tqh_first;
    while !msg.is_null()
        && {
            msg1 = (*msg).entry.tqe_next;
            1 as libc::c_int != 0
        }
    {
        if ((*msg).msg_num).wrapping_add(limit) >= (*c).message_next {
            break;
        }
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
}
#[no_mangle]
pub unsafe extern "C" fn status_message_set(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ap: ::core::ffi::VaListImpl;
    let mut delay: libc::c_int = 0;
    status_message_clear(c);
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
    ap = args.clone();
    xvasprintf(&mut (*c).message_string, fmt, ap.as_va_list());
    server_client_add_message(
        c,
        b"%s\0" as *const u8 as *const libc::c_char,
        (*c).message_string,
    );
    delay = options_get_number(
        (*(*c).session).options,
        b"display-time\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if delay > 0 as libc::c_int {
        tv.tv_sec = (delay / 1000 as libc::c_int) as __time_t;
        tv
            .tv_usec = (delay % 1000 as libc::c_int) as libc::c_long
            * 1000 as libc::c_long;
        if event_initialized(&mut (*c).message_timer) != 0 {
            event_del(&mut (*c).message_timer);
        }
        event_set(
            &mut (*c).message_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                status_message_callback
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
        event_add(&mut (*c).message_timer, &mut tv);
    }
    (*c).tty.flags |= 0x1 as libc::c_int | 0x2 as libc::c_int;
    (*c).flags |= 0x10 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmdq_error(
    mut item: *mut cmdq_item,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut c: *mut client = (*item).client;
    let mut cmd: *mut cmd = (*item).cmd;
    let mut ap: ::core::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msglen: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    msglen = xvasprintf(&mut msg, fmt, ap.as_va_list()) as size_t;
    log_debug(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"cmdq_error\0"))
            .as_ptr(),
        msg,
    );
    if c.is_null() {
        cfg_add_cause(
            b"%s:%u: %s\0" as *const u8 as *const libc::c_char,
            (*cmd).file,
            (*cmd).line,
            msg,
        );
    } else if ((*c).session).is_null() || (*c).flags & 0x2000 as libc::c_int != 0 {
        if !(*c).flags & 0x10000 as libc::c_int != 0 {
            tmp = msg;
            msg = utf8_sanitize(tmp);
            free(tmp as *mut libc::c_void);
            msglen = strlen(msg);
        }
        evbuffer_add((*c).stderr_data, msg as *const libc::c_void, msglen);
        evbuffer_add(
            (*c).stderr_data,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        server_client_push_stderr(c);
        (*c).retval = 1 as libc::c_int;
    } else {
        *msg = toupper(*msg as u_char as libc::c_int) as libc::c_char;
        status_message_set(c, b"%s\0" as *const u8 as *const libc::c_char, msg);
    }
    free(msg as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_add(
    mut wp: *mut window_pane,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    window_copy_vadd(wp, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn xasprintf(
    mut ret: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    ap = args.clone();
    i = xvasprintf(ret, fmt, ap.as_va_list());
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn xsnprintf(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    ap = args.clone();
    i = xvsnprintf(str, len, fmt, ap.as_va_list());
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn setproctitle(mut fmt: *const libc::c_char, mut args: ...) {
    let mut title: [libc::c_char; 16] = [0; 16];
    let mut name: [libc::c_char; 16] = [0; 16];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    let mut used: libc::c_int = 0;
    ap = args.clone();
    vsnprintf(
        title.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    used = snprintf(
        name.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        getprogname(),
        title.as_mut_ptr(),
    );
    if used
        >= ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int
    {
        cp = strrchr(name.as_mut_ptr(), ' ' as i32);
        if !cp.is_null() {
            *cp = '\0' as i32 as libc::c_char;
        }
    }
    prctl(15 as libc::c_int, name.as_mut_ptr());
}
