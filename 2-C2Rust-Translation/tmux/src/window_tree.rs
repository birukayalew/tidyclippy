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
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    fn format_single(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int);
    fn cmd_find_from_winlink_pane(
        _: *mut cmd_find_state,
        _: *mut winlink,
        _: *mut window_pane,
        _: libc::c_int,
    );
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut cmdq_item;
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn server_kill_pane(_: *mut window_pane);
    fn server_kill_window(_: *mut window);
    fn server_destroy_session(_: *mut session);
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
    fn screen_write_puts(
        _: *mut screen_write_ctx,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    );
    fn screen_write_vline(
        _: *mut screen_write_ctx,
        _: u_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn screen_write_preview(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
    );
    fn screen_write_cursormove(_: *mut screen_write_ctx, _: u_int, _: u_int);
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    fn winlink_count(_: *mut winlinks) -> u_int;
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    fn window_count_panes(_: *mut window) -> u_int;
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    fn window_pane_reset_mode(_: *mut window_pane);
    fn mode_tree_count_tagged(_: *mut mode_tree_data) -> u_int;
    fn mode_tree_get_current(_: *mut mode_tree_data) -> *mut libc::c_void;
    fn mode_tree_expand_current(_: *mut mode_tree_data);
    fn mode_tree_set_current(_: *mut mode_tree_data, _: uint64_t);
    fn mode_tree_each_tagged(
        _: *mut mode_tree_data,
        _: mode_tree_each_cb,
        _: *mut client,
        _: key_code,
        _: libc::c_int,
    );
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
    fn mode_tree_remove(_: *mut mode_tree_data, _: *mut mode_tree_item);
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
    static mut sessions: sessions;
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    fn session_find_by_id(_: u_int) -> *mut session;
    fn session_destroy(_: *mut session, _: *const libc::c_char);
    fn session_group_contains(_: *mut session) -> *mut session_group;
    fn osdep_get_name(_: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char;
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_36,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sessions {
    pub rbh_root: *mut session,
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
pub struct window_tree_modedata {
    pub wp: *mut window_pane,
    pub dead: libc::c_int,
    pub references: libc::c_int,
    pub data: *mut mode_tree_data,
    pub format: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub squash_groups: libc::c_int,
    pub item_list: *mut *mut window_tree_itemdata,
    pub item_size: u_int,
    pub entered: *const libc::c_char,
    pub fs: cmd_find_state,
    pub type_0: window_tree_type,
    pub offset: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub start: u_int,
    pub end: u_int,
    pub each: u_int,
}
pub type window_tree_type = libc::c_uint;
pub const WINDOW_TREE_PANE: window_tree_type = 3;
pub const WINDOW_TREE_WINDOW: window_tree_type = 2;
pub const WINDOW_TREE_SESSION: window_tree_type = 1;
pub const WINDOW_TREE_NONE: window_tree_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_tree_itemdata {
    pub type_0: window_tree_type,
    pub session: libc::c_int,
    pub winlink: libc::c_int,
    pub pane: libc::c_int,
}
pub const WINDOW_TREE_BY_TIME: window_tree_sort_type = 2;
pub const WINDOW_TREE_BY_NAME: window_tree_sort_type = 1;
pub const WINDOW_TREE_BY_INDEX: window_tree_sort_type = 0;
pub type window_tree_sort_type = libc::c_uint;
#[no_mangle]
pub static mut window_tree_mode: window_mode = unsafe {
    {
        let mut init = window_mode {
            name: b"tree-mode\0" as *const u8 as *const libc::c_char,
            init: Some(
                window_tree_init
                    as unsafe extern "C" fn(
                        *mut window_pane,
                        *mut cmd_find_state,
                        *mut args,
                    ) -> *mut screen,
            ),
            free: Some(window_tree_free as unsafe extern "C" fn(*mut window_pane) -> ()),
            resize: Some(
                window_tree_resize
                    as unsafe extern "C" fn(*mut window_pane, u_int, u_int) -> (),
            ),
            key: Some(
                window_tree_key
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
static mut window_tree_sort_list: [*const libc::c_char; 3] = [
    b"index\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    b"time\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn window_tree_pull_item(
    mut item: *mut window_tree_itemdata,
    mut sp: *mut *mut session,
    mut wlp: *mut *mut winlink,
    mut wp: *mut *mut window_pane,
) {
    *wp = 0 as *mut window_pane;
    *wlp = 0 as *mut winlink;
    *sp = session_find_by_id((*item).session as u_int);
    if (*sp).is_null() {
        return;
    }
    if (*item).type_0 as libc::c_uint
        == WINDOW_TREE_SESSION as libc::c_int as libc::c_uint
    {
        *wlp = (**sp).curw;
        *wp = (*(**wlp).window).active;
        return;
    }
    *wlp = winlink_find_by_index(&mut (**sp).windows, (*item).winlink);
    if (*wlp).is_null() {
        *sp = 0 as *mut session;
        return;
    }
    if (*item).type_0 as libc::c_uint
        == WINDOW_TREE_WINDOW as libc::c_int as libc::c_uint
    {
        *wp = (*(**wlp).window).active;
        return;
    }
    *wp = window_pane_find_by_id((*item).pane as u_int);
    if window_has_pane((**wlp).window, *wp) == 0 {
        *wp = 0 as *mut window_pane;
    }
    if (*wp).is_null() {
        *sp = 0 as *mut session;
        *wlp = 0 as *mut winlink;
        return;
    }
}
unsafe extern "C" fn window_tree_add_item(
    mut data: *mut window_tree_modedata,
) -> *mut window_tree_itemdata {
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    (*data)
        .item_list = xreallocarray(
        (*data).item_list as *mut libc::c_void,
        ((*data).item_size).wrapping_add(1 as libc::c_int as u_int) as size_t,
        ::core::mem::size_of::<*mut window_tree_itemdata>() as libc::c_ulong,
    ) as *mut *mut window_tree_itemdata;
    let fresh0 = (*data).item_size;
    (*data).item_size = ((*data).item_size).wrapping_add(1);
    let ref mut fresh1 = *((*data).item_list).offset(fresh0 as isize);
    *fresh1 = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<window_tree_itemdata>() as libc::c_ulong,
    ) as *mut window_tree_itemdata;
    item = *fresh1;
    return item;
}
unsafe extern "C" fn window_tree_free_item(mut item: *mut window_tree_itemdata) {
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_cmp_session_name(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const session = a0 as *const *const session;
    let mut b: *const *const session = b0 as *const *const session;
    return strcmp((**a).name, (**b).name);
}
unsafe extern "C" fn window_tree_cmp_session_time(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const session = a0 as *const *const session;
    let mut b: *const *const session = b0 as *const *const session;
    if if (**a).activity_time.tv_sec == (**b).activity_time.tv_sec {
        ((**a).activity_time.tv_usec > (**b).activity_time.tv_usec) as libc::c_int
    } else {
        ((**a).activity_time.tv_sec > (**b).activity_time.tv_sec) as libc::c_int
    } != 0
    {
        return -(1 as libc::c_int);
    }
    if if (**a).activity_time.tv_sec == (**b).activity_time.tv_sec {
        ((**a).activity_time.tv_usec < (**b).activity_time.tv_usec) as libc::c_int
    } else {
        ((**a).activity_time.tv_sec < (**b).activity_time.tv_sec) as libc::c_int
    } != 0
    {
        return 1 as libc::c_int;
    }
    return strcmp((**a).name, (**b).name);
}
unsafe extern "C" fn window_tree_cmp_window_name(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const winlink = a0 as *const *const winlink;
    let mut b: *const *const winlink = b0 as *const *const winlink;
    return strcmp((*(**a).window).name, (*(**b).window).name);
}
unsafe extern "C" fn window_tree_cmp_window_time(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const winlink = a0 as *const *const winlink;
    let mut b: *const *const winlink = b0 as *const *const winlink;
    if if (*(**a).window).activity_time.tv_sec == (*(**b).window).activity_time.tv_sec {
        ((*(**a).window).activity_time.tv_usec > (*(**b).window).activity_time.tv_usec)
            as libc::c_int
    } else {
        ((*(**a).window).activity_time.tv_sec > (*(**b).window).activity_time.tv_sec)
            as libc::c_int
    } != 0
    {
        return -(1 as libc::c_int);
    }
    if if (*(**a).window).activity_time.tv_sec == (*(**b).window).activity_time.tv_sec {
        ((*(**a).window).activity_time.tv_usec < (*(**b).window).activity_time.tv_usec)
            as libc::c_int
    } else {
        ((*(**a).window).activity_time.tv_sec < (*(**b).window).activity_time.tv_sec)
            as libc::c_int
    } != 0
    {
        return 1 as libc::c_int;
    }
    return strcmp((*(**a).window).name, (*(**b).window).name);
}
unsafe extern "C" fn window_tree_cmp_pane_time(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_pane = a0 as *const *const window_pane;
    let mut b: *const *const window_pane = b0 as *const *const window_pane;
    if (**a).active_point < (**b).active_point {
        return -(1 as libc::c_int);
    }
    if (**a).active_point > (**b).active_point {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_tree_build_pane(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
    mut modedata: *mut libc::c_void,
    mut parent: *mut mode_tree_item,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: u_int = 0;
    window_pane_index(wp, &mut idx);
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_PANE;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = (*wl).idx;
    (*item).pane = (*wp).id as libc::c_int;
    text = format_single(
        0 as *mut cmdq_item,
        (*data).format,
        0 as *mut client,
        s,
        wl,
        wp,
    );
    xasprintf(
        &mut name as *mut *mut libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        idx,
    );
    mode_tree_add(
        (*data).data,
        parent,
        item as *mut libc::c_void,
        wp as uint64_t,
        name,
        text,
        -(1 as libc::c_int),
    );
    free(text as *mut libc::c_void);
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_filter_pane(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
    mut filter: *const libc::c_char,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    if filter.is_null() {
        return 1 as libc::c_int;
    }
    cp = format_single(0 as *mut cmdq_item, filter, 0 as *mut client, s, wl, wp);
    result = format_true(cp);
    free(cp as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn window_tree_build_window(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut modedata: *mut libc::c_void,
    mut sort_type: u_int,
    mut parent: *mut mode_tree_item,
    mut filter: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut l: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    let mut expanded: libc::c_int = 0;
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_WINDOW;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = (*wl).idx;
    (*item).pane = -(1 as libc::c_int);
    text = format_single(
        0 as *mut cmdq_item,
        (*data).format,
        0 as *mut client,
        s,
        wl,
        0 as *mut window_pane,
    );
    xasprintf(
        &mut name as *mut *mut libc::c_char,
        b"%u\0" as *const u8 as *const libc::c_char,
        (*wl).idx,
    );
    if (*data).type_0 as libc::c_uint
        == WINDOW_TREE_SESSION as libc::c_int as libc::c_uint
        || (*data).type_0 as libc::c_uint
            == WINDOW_TREE_WINDOW as libc::c_int as libc::c_uint
    {
        expanded = 0 as libc::c_int;
    } else {
        expanded = 1 as libc::c_int;
    }
    mti = mode_tree_add(
        (*data).data,
        parent,
        item as *mut libc::c_void,
        wl as uint64_t,
        name,
        text,
        expanded,
    );
    free(text as *mut libc::c_void);
    free(name as *mut libc::c_void);
    wp = (*(*wl).window).panes.tqh_first;
    if !wp.is_null() {
        if ((*wp).entry.tqe_next).is_null() {
            if !(window_tree_filter_pane(s, wl, wp, filter) == 0) {
                return 1 as libc::c_int;
            }
        } else {
            l = 0 as *mut *mut window_pane;
            n = 0 as libc::c_int as u_int;
            wp = (*(*wl).window).panes.tqh_first;
            while !wp.is_null() {
                if !(window_tree_filter_pane(s, wl, wp, filter) == 0) {
                    l = xreallocarray(
                        l as *mut libc::c_void,
                        n.wrapping_add(1 as libc::c_int as u_int) as size_t,
                        ::core::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh2 = n;
                    n = n.wrapping_add(1);
                    let ref mut fresh3 = *l.offset(fresh2 as isize);
                    *fresh3 = wp;
                }
                wp = (*wp).entry.tqe_next;
            }
            if !(n == 0 as libc::c_int as u_int) {
                match sort_type {
                    2 => {
                        qsort(
                            l as *mut libc::c_void,
                            n as size_t,
                            ::core::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                            Some(
                                window_tree_cmp_pane_time
                                    as unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_void,
                                    ) -> libc::c_int,
                            ),
                        );
                    }
                    0 | 1 | _ => {}
                }
                i = 0 as libc::c_int as u_int;
                while i < n {
                    window_tree_build_pane(s, wl, *l.offset(i as isize), modedata, mti);
                    i = i.wrapping_add(1);
                    i;
                }
                free(l as *mut libc::c_void);
                return 1 as libc::c_int;
            }
        }
    }
    window_tree_free_item(item);
    (*data).item_size = ((*data).item_size).wrapping_sub(1);
    (*data).item_size;
    mode_tree_remove((*data).data, mti);
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_tree_build_session(
    mut s: *mut session,
    mut modedata: *mut libc::c_void,
    mut sort_type: u_int,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut l: *mut *mut winlink = 0 as *mut *mut winlink;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    let mut empty: u_int = 0;
    let mut expanded: libc::c_int = 0;
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_SESSION;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = -(1 as libc::c_int);
    (*item).pane = -(1 as libc::c_int);
    text = format_single(
        0 as *mut cmdq_item,
        (*data).format,
        0 as *mut client,
        s,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    if (*data).type_0 as libc::c_uint
        == WINDOW_TREE_SESSION as libc::c_int as libc::c_uint
    {
        expanded = 0 as libc::c_int;
    } else {
        expanded = 1 as libc::c_int;
    }
    mti = mode_tree_add(
        (*data).data,
        0 as *mut mode_tree_item,
        item as *mut libc::c_void,
        s as uint64_t,
        (*s).name,
        text,
        expanded,
    );
    free(text as *mut libc::c_void);
    l = 0 as *mut *mut winlink;
    n = 0 as libc::c_int as u_int;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        l = xreallocarray(
            l as *mut libc::c_void,
            n.wrapping_add(1 as libc::c_int as u_int) as size_t,
            ::core::mem::size_of::<*mut winlink>() as libc::c_ulong,
        ) as *mut *mut winlink;
        let fresh4 = n;
        n = n.wrapping_add(1);
        let ref mut fresh5 = *l.offset(fresh4 as isize);
        *fresh5 = wl;
        wl = winlinks_RB_NEXT(wl);
    }
    match sort_type {
        1 => {
            qsort(
                l as *mut libc::c_void,
                n as size_t,
                ::core::mem::size_of::<*mut winlink>() as libc::c_ulong,
                Some(
                    window_tree_cmp_window_name
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        2 => {
            qsort(
                l as *mut libc::c_void,
                n as size_t,
                ::core::mem::size_of::<*mut winlink>() as libc::c_ulong,
                Some(
                    window_tree_cmp_window_time
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        0 | _ => {}
    }
    empty = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as u_int;
    while i < n {
        if window_tree_build_window(
            s,
            *l.offset(i as isize),
            modedata,
            sort_type,
            mti,
            filter,
        ) == 0
        {
            empty = empty.wrapping_add(1);
            empty;
        }
        i = i.wrapping_add(1);
        i;
    }
    if empty == n {
        window_tree_free_item(item);
        (*data).item_size = ((*data).item_size).wrapping_sub(1);
        (*data).item_size;
        mode_tree_remove((*data).data, mti);
    }
    free(l as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_build(
    mut modedata: *mut libc::c_void,
    mut sort_type: u_int,
    mut tag: *mut uint64_t,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut s: *mut session = 0 as *mut session;
    let mut l: *mut *mut session = 0 as *mut *mut session;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut current: *mut session_group = 0 as *mut session_group;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    current = session_group_contains((*data).fs.s);
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        window_tree_free_item(*((*data).item_list).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_tree_itemdata;
    (*data).item_size = 0 as libc::c_int as u_int;
    l = 0 as *mut *mut session;
    n = 0 as libc::c_int as u_int;
    let mut current_block_11: u64;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        if (*data).squash_groups != 0
            && {
                sg = session_group_contains(s);
                !sg.is_null()
            }
        {
            if sg == current && s != (*data).fs.s
                || sg != current && s != (*sg).sessions.tqh_first
            {
                current_block_11 = 8515828400728868193;
            } else {
                current_block_11 = 7651349459974463963;
            }
        } else {
            current_block_11 = 7651349459974463963;
        }
        match current_block_11 {
            7651349459974463963 => {
                l = xreallocarray(
                    l as *mut libc::c_void,
                    n.wrapping_add(1 as libc::c_int as u_int) as size_t,
                    ::core::mem::size_of::<*mut session>() as libc::c_ulong,
                ) as *mut *mut session;
                let fresh6 = n;
                n = n.wrapping_add(1);
                let ref mut fresh7 = *l.offset(fresh6 as isize);
                *fresh7 = s;
            }
            _ => {}
        }
        s = sessions_RB_NEXT(s);
    }
    match sort_type {
        1 => {
            qsort(
                l as *mut libc::c_void,
                n as size_t,
                ::core::mem::size_of::<*mut session>() as libc::c_ulong,
                Some(
                    window_tree_cmp_session_name
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        2 => {
            qsort(
                l as *mut libc::c_void,
                n as size_t,
                ::core::mem::size_of::<*mut session>() as libc::c_ulong,
                Some(
                    window_tree_cmp_session_time
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        0 | _ => {}
    }
    i = 0 as libc::c_int as u_int;
    while i < n {
        window_tree_build_session(*l.offset(i as isize), modedata, sort_type, filter);
        i = i.wrapping_add(1);
        i;
    }
    free(l as *mut libc::c_void);
    match (*data).type_0 as libc::c_uint {
        1 => {
            *tag = (*data).fs.s as uint64_t;
        }
        2 => {
            *tag = (*data).fs.wl as uint64_t;
        }
        3 => {
            if window_count_panes((*(*data).fs.wl).window) == 1 as libc::c_int as u_int {
                *tag = (*data).fs.wl as uint64_t;
            } else {
                *tag = (*data).fs.wp as uint64_t;
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_tree_draw_label(
    mut ctx: *mut screen_write_ctx,
    mut px: u_int,
    mut py: u_int,
    mut sx: u_int,
    mut sy: u_int,
    mut gc: *const grid_cell,
    mut label: *const libc::c_char,
) {
    let mut len: size_t = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    len = strlen(label);
    if sx == 0 as libc::c_int as u_int || sy == 1 as libc::c_int as u_int
        || len > sx as size_t
    {
        return;
    }
    ox = ((sx as size_t).wrapping_sub(len).wrapping_add(1 as libc::c_int as size_t)
        / 2 as libc::c_int as size_t) as u_int;
    oy = sy.wrapping_add(1 as libc::c_int as u_int) / 2 as libc::c_int as u_int;
    if ox > 1 as libc::c_int as u_int
        && (ox as size_t).wrapping_add(len)
            < sx.wrapping_sub(1 as libc::c_int as u_int) as size_t
        && sy >= 3 as libc::c_int as u_int
    {
        screen_write_cursormove(
            ctx,
            px.wrapping_add(ox).wrapping_sub(1 as libc::c_int as u_int),
            py.wrapping_add(oy).wrapping_sub(1 as libc::c_int as u_int),
        );
        screen_write_box(
            ctx,
            len.wrapping_add(2 as libc::c_int as size_t) as u_int,
            3 as libc::c_int as u_int,
        );
    }
    screen_write_cursormove(ctx, px.wrapping_add(ox), py.wrapping_add(oy));
    screen_write_puts(ctx, gc, b"%s\0" as *const u8 as *const libc::c_char, label);
}
unsafe extern "C" fn window_tree_draw_session(
    mut data: *mut window_tree_modedata,
    mut s: *mut session,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut oo: *mut options = (*s).options;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut w: *mut window = 0 as *mut window;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    let mut loop_0: u_int = 0;
    let mut total: u_int = 0;
    let mut visible: u_int = 0;
    let mut each: u_int = 0;
    let mut width: u_int = 0;
    let mut offset: u_int = 0;
    let mut current: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut remaining: u_int = 0;
    let mut i: u_int = 0;
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
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    total = winlink_count(&mut (*s).windows);
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    colour = options_get_number(
        oo,
        b"display-panes-colour\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    active_colour = options_get_number(
        oo,
        b"display-panes-active-colour\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if sx / total < 24 as libc::c_int as u_int {
        visible = sx / 24 as libc::c_int as u_int;
        if visible == 0 as libc::c_int as u_int {
            visible = 1 as libc::c_int as u_int;
        }
    } else {
        visible = total;
    }
    current = 0 as libc::c_int as u_int;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if wl == (*s).curw {
            break;
        }
        current = current.wrapping_add(1);
        current;
        wl = winlinks_RB_NEXT(wl);
    }
    if current < visible {
        start = 0 as libc::c_int as u_int;
        end = visible;
    } else if current >= total.wrapping_sub(visible) {
        start = total.wrapping_sub(visible);
        end = total;
    } else {
        start = current.wrapping_sub(visible / 2 as libc::c_int as u_int);
        end = start.wrapping_add(visible);
    }
    if (*data).offset < -(start as libc::c_int) {
        (*data).offset = -(start as libc::c_int);
    }
    if (*data).offset > total.wrapping_sub(end) as libc::c_int {
        (*data).offset = total.wrapping_sub(end) as libc::c_int;
    }
    start = start.wrapping_add((*data).offset as u_int);
    end = end.wrapping_add((*data).offset as u_int);
    left = (start != 0 as libc::c_int as u_int) as libc::c_int;
    right = (end != total) as libc::c_int;
    if left != 0 && right != 0 && sx <= 6 as libc::c_int as u_int
        || (left != 0 || right != 0) && sx <= 3 as libc::c_int as u_int
    {
        right = 0 as libc::c_int;
        left = right;
    }
    if left != 0 && right != 0 {
        each = sx.wrapping_sub(6 as libc::c_int as u_int) / visible;
        remaining = sx
            .wrapping_sub(6 as libc::c_int as u_int)
            .wrapping_sub(visible * each);
    } else if left != 0 || right != 0 {
        each = sx.wrapping_sub(3 as libc::c_int as u_int) / visible;
        remaining = sx
            .wrapping_sub(3 as libc::c_int as u_int)
            .wrapping_sub(visible * each);
    } else {
        each = sx / visible;
        remaining = sx.wrapping_sub(visible * each);
    }
    if each == 0 as libc::c_int as u_int {
        return;
    }
    if left != 0 {
        (*data).left = cx.wrapping_add(2 as libc::c_int as u_int) as libc::c_int;
        screen_write_cursormove(ctx, cx.wrapping_add(2 as libc::c_int as u_int), cy);
        screen_write_vline(ctx, sy, 0 as libc::c_int, 0 as libc::c_int);
        screen_write_cursormove(
            ctx,
            cx,
            cy.wrapping_add(sy / 2 as libc::c_int as u_int),
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const grid_cell,
            b"<\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).left = -(1 as libc::c_int);
    }
    if right != 0 {
        (*data)
            .right = cx.wrapping_add(sx).wrapping_sub(3 as libc::c_int as u_int)
            as libc::c_int;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(3 as libc::c_int as u_int),
            cy,
        );
        screen_write_vline(ctx, sy, 0 as libc::c_int, 0 as libc::c_int);
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(1 as libc::c_int as u_int),
            cy.wrapping_add(sy / 2 as libc::c_int as u_int),
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const grid_cell,
            b">\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).right = -(1 as libc::c_int);
    }
    (*data).start = start;
    (*data).end = end;
    (*data).each = each;
    loop_0 = 0 as libc::c_int as u_int;
    i = loop_0;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if loop_0 == end {
            break;
        }
        if loop_0 < start {
            loop_0 = loop_0.wrapping_add(1);
            loop_0;
        } else {
            w = (*wl).window;
            if wl == (*s).curw {
                gc.fg = active_colour;
            } else {
                gc.fg = colour;
            }
            if left != 0 {
                offset = (3 as libc::c_int as u_int).wrapping_add(i * each);
            } else {
                offset = i * each;
            }
            if loop_0 == end.wrapping_sub(1 as libc::c_int as u_int) {
                width = each.wrapping_add(remaining);
            } else {
                width = each.wrapping_sub(1 as libc::c_int as u_int);
            }
            screen_write_cursormove(ctx, cx.wrapping_add(offset), cy);
            screen_write_preview(ctx, &mut (*(*w).active).base, width, sy);
            xasprintf(
                &mut label as *mut *mut libc::c_char,
                b" %u:%s \0" as *const u8 as *const libc::c_char,
                (*wl).idx,
                (*w).name,
            );
            if strlen(label) > width as libc::c_ulong {
                xasprintf(
                    &mut label as *mut *mut libc::c_char,
                    b" %u \0" as *const u8 as *const libc::c_char,
                    (*wl).idx,
                );
            }
            window_tree_draw_label(
                ctx,
                cx.wrapping_add(offset),
                cy,
                width,
                sy,
                &mut gc,
                label,
            );
            free(label as *mut libc::c_void);
            if loop_0 != end.wrapping_sub(1 as libc::c_int as u_int) {
                screen_write_cursormove(
                    ctx,
                    cx.wrapping_add(offset).wrapping_add(width),
                    cy,
                );
                screen_write_vline(ctx, sy, 0 as libc::c_int, 0 as libc::c_int);
            }
            loop_0 = loop_0.wrapping_add(1);
            loop_0;
            i = i.wrapping_add(1);
            i;
        }
        wl = winlinks_RB_NEXT(wl);
    }
}
unsafe extern "C" fn window_tree_draw_window(
    mut data: *mut window_tree_modedata,
    mut s: *mut session,
    mut w: *mut window,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut oo: *mut options = (*s).options;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    let mut loop_0: u_int = 0;
    let mut total: u_int = 0;
    let mut visible: u_int = 0;
    let mut each: u_int = 0;
    let mut width: u_int = 0;
    let mut offset: u_int = 0;
    let mut current: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut remaining: u_int = 0;
    let mut i: u_int = 0;
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
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut pane_idx: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    total = window_count_panes(w);
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    colour = options_get_number(
        oo,
        b"display-panes-colour\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    active_colour = options_get_number(
        oo,
        b"display-panes-active-colour\0" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if sx / total < 24 as libc::c_int as u_int {
        visible = sx / 24 as libc::c_int as u_int;
        if visible == 0 as libc::c_int as u_int {
            visible = 1 as libc::c_int as u_int;
        }
    } else {
        visible = total;
    }
    current = 0 as libc::c_int as u_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if wp == (*w).active {
            break;
        }
        current = current.wrapping_add(1);
        current;
        wp = (*wp).entry.tqe_next;
    }
    if current < visible {
        start = 0 as libc::c_int as u_int;
        end = visible;
    } else if current >= total.wrapping_sub(visible) {
        start = total.wrapping_sub(visible);
        end = total;
    } else {
        start = current.wrapping_sub(visible / 2 as libc::c_int as u_int);
        end = start.wrapping_add(visible);
    }
    if (*data).offset < -(start as libc::c_int) {
        (*data).offset = -(start as libc::c_int);
    }
    if (*data).offset > total.wrapping_sub(end) as libc::c_int {
        (*data).offset = total.wrapping_sub(end) as libc::c_int;
    }
    start = start.wrapping_add((*data).offset as u_int);
    end = end.wrapping_add((*data).offset as u_int);
    left = (start != 0 as libc::c_int as u_int) as libc::c_int;
    right = (end != total) as libc::c_int;
    if left != 0 && right != 0 && sx <= 6 as libc::c_int as u_int
        || (left != 0 || right != 0) && sx <= 3 as libc::c_int as u_int
    {
        right = 0 as libc::c_int;
        left = right;
    }
    if left != 0 && right != 0 {
        each = sx.wrapping_sub(6 as libc::c_int as u_int) / visible;
        remaining = sx
            .wrapping_sub(6 as libc::c_int as u_int)
            .wrapping_sub(visible * each);
    } else if left != 0 || right != 0 {
        each = sx.wrapping_sub(3 as libc::c_int as u_int) / visible;
        remaining = sx
            .wrapping_sub(3 as libc::c_int as u_int)
            .wrapping_sub(visible * each);
    } else {
        each = sx / visible;
        remaining = sx.wrapping_sub(visible * each);
    }
    if each == 0 as libc::c_int as u_int {
        return;
    }
    if left != 0 {
        (*data).left = cx.wrapping_add(2 as libc::c_int as u_int) as libc::c_int;
        screen_write_cursormove(ctx, cx.wrapping_add(2 as libc::c_int as u_int), cy);
        screen_write_vline(ctx, sy, 0 as libc::c_int, 0 as libc::c_int);
        screen_write_cursormove(
            ctx,
            cx,
            cy.wrapping_add(sy / 2 as libc::c_int as u_int),
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const grid_cell,
            b"<\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).left = -(1 as libc::c_int);
    }
    if right != 0 {
        (*data)
            .right = cx.wrapping_add(sx).wrapping_sub(3 as libc::c_int as u_int)
            as libc::c_int;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(3 as libc::c_int as u_int),
            cy,
        );
        screen_write_vline(ctx, sy, 0 as libc::c_int, 0 as libc::c_int);
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(1 as libc::c_int as u_int),
            cy.wrapping_add(sy / 2 as libc::c_int as u_int),
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const grid_cell,
            b">\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).right = -(1 as libc::c_int);
    }
    (*data).start = start;
    (*data).end = end;
    (*data).each = each;
    loop_0 = 0 as libc::c_int as u_int;
    i = loop_0;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if loop_0 == end {
            break;
        }
        if loop_0 < start {
            loop_0 = loop_0.wrapping_add(1);
            loop_0;
        } else {
            if wp == (*w).active {
                gc.fg = active_colour;
            } else {
                gc.fg = colour;
            }
            if left != 0 {
                offset = (3 as libc::c_int as u_int).wrapping_add(i * each);
            } else {
                offset = i * each;
            }
            if loop_0 == end.wrapping_sub(1 as libc::c_int as u_int) {
                width = each.wrapping_add(remaining);
            } else {
                width = each.wrapping_sub(1 as libc::c_int as u_int);
            }
            screen_write_cursormove(ctx, cx.wrapping_add(offset), cy);
            screen_write_preview(ctx, &mut (*wp).base, width, sy);
            if window_pane_index(wp, &mut pane_idx as *mut libc::c_int as *mut u_int)
                != 0 as libc::c_int
            {
                pane_idx = loop_0 as libc::c_int;
            }
            xasprintf(
                &mut label as *mut *mut libc::c_char,
                b" %u \0" as *const u8 as *const libc::c_char,
                pane_idx,
            );
            window_tree_draw_label(
                ctx,
                cx.wrapping_add(offset),
                cy,
                each,
                sy,
                &mut gc,
                label,
            );
            free(label as *mut libc::c_void);
            if loop_0 != end.wrapping_sub(1 as libc::c_int as u_int) {
                screen_write_cursormove(
                    ctx,
                    cx.wrapping_add(offset).wrapping_add(width),
                    cy,
                );
                screen_write_vline(ctx, sy, 0 as libc::c_int, 0 as libc::c_int);
            }
            loop_0 = loop_0.wrapping_add(1);
            loop_0;
            i = i.wrapping_add(1);
            i;
        }
        wp = (*wp).entry.tqe_next;
    }
}
unsafe extern "C" fn window_tree_draw(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut sp: *mut session = 0 as *mut session;
    let mut wlp: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    window_tree_pull_item(item, &mut sp, &mut wlp, &mut wp);
    if wp.is_null() {
        return;
    }
    match (*item).type_0 as libc::c_uint {
        1 => {
            window_tree_draw_session(
                modedata as *mut window_tree_modedata,
                sp,
                ctx,
                sx,
                sy,
            );
        }
        2 => {
            window_tree_draw_window(
                modedata as *mut window_tree_modedata,
                sp,
                (*wlp).window,
                ctx,
                sx,
                sy,
            );
        }
        3 => {
            screen_write_preview(ctx, &mut (*wp).base, sx, sy);
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_tree_search(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ss: *const libc::c_char,
) -> libc::c_int {
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    match (*item).type_0 as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 => {
            if s.is_null() {
                return 0 as libc::c_int;
            }
            return (strstr((*s).name, ss) != 0 as *mut libc::c_void as *mut libc::c_char)
                as libc::c_int;
        }
        2 => {
            if s.is_null() || wl.is_null() {
                return 0 as libc::c_int;
            }
            return (strstr((*(*wl).window).name, ss)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
        }
        3 => {
            if !(s.is_null() || wl.is_null() || wp.is_null()) {
                cmd = osdep_get_name((*wp).fd, ((*wp).tty).as_mut_ptr());
                if cmd.is_null() || *cmd as libc::c_int == '\0' as i32 {
                    return 0 as libc::c_int;
                }
                return (strstr(cmd, ss) != 0 as *mut libc::c_void as *mut libc::c_char)
                    as libc::c_int;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_tree_init(
    mut wp: *mut window_pane,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut screen {
    let mut data: *mut window_tree_modedata = 0 as *mut window_tree_modedata;
    let mut s: *mut screen = 0 as *mut screen;
    data = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<window_tree_modedata>() as libc::c_ulong,
    ) as *mut window_tree_modedata;
    (*wp).modedata = data as *mut libc::c_void;
    if args_has(args, 's' as i32 as u_char) != 0 {
        (*data).type_0 = WINDOW_TREE_SESSION;
    } else if args_has(args, 'w' as i32 as u_char) != 0 {
        (*data).type_0 = WINDOW_TREE_WINDOW;
    } else {
        (*data).type_0 = WINDOW_TREE_PANE;
    }
    memcpy(
        &mut (*data).fs as *mut cmd_find_state as *mut libc::c_void,
        fs as *const libc::c_void,
        ::core::mem::size_of::<cmd_find_state>() as libc::c_ulong,
    );
    (*data).wp = wp;
    (*data).references = 1 as libc::c_int;
    if args.is_null() || args_has(args, 'F' as i32 as u_char) == 0 {
        (*data)
            .format = xstrdup(
            b"#{?pane_format,#{pane_current_command} \"#{pane_title}\",#{?window_format,#{window_name}#{window_flags} (#{window_panes} panes)#{?#{==:#{window_panes},1}, \"#{pane_title}\",},#{session_windows} windows#{?session_grouped, (group #{session_group}: #{session_group_list}),}#{?session_attached, (attached),}}}\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).format = xstrdup(args_get(args, 'F' as i32 as u_char));
    }
    if args.is_null() || (*args).argc == 0 as libc::c_int {
        (*data)
            .command = xstrdup(
            b"switch-client -t '%%'\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).command = xstrdup(*((*args).argv).offset(0 as libc::c_int as isize));
    }
    (*data).squash_groups = (args_has(args, 'G' as i32 as u_char) == 0) as libc::c_int;
    (*data)
        .data = mode_tree_start(
        wp,
        args,
        Some(
            window_tree_build
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    u_int,
                    *mut uint64_t,
                    *const libc::c_char,
                ) -> (),
        ),
        Some(
            window_tree_draw
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut screen_write_ctx,
                    u_int,
                    u_int,
                ) -> (),
        ),
        Some(
            window_tree_search
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        data as *mut libc::c_void,
        window_tree_sort_list.as_mut_ptr(),
        (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as u_int,
        &mut s,
    );
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    (*data).type_0 = WINDOW_TREE_NONE;
    return s;
}
unsafe extern "C" fn window_tree_destroy(mut data: *mut window_tree_modedata) {
    let mut i: u_int = 0;
    (*data).references -= 1;
    if (*data).references != 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        window_tree_free_item(*((*data).item_list).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free((*data).item_list as *mut libc::c_void);
    free((*data).format as *mut libc::c_void);
    free((*data).command as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_free(mut wp: *mut window_pane) {
    let mut data: *mut window_tree_modedata = (*wp).modedata
        as *mut window_tree_modedata;
    if data.is_null() {
        return;
    }
    (*data).dead = 1 as libc::c_int;
    mode_tree_free((*data).data);
    window_tree_destroy(data);
}
unsafe extern "C" fn window_tree_resize(
    mut wp: *mut window_pane,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_tree_modedata = (*wp).modedata
        as *mut window_tree_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_tree_get_target(
    mut item: *mut window_tree_itemdata,
    mut fs: *mut cmd_find_state,
) -> *mut libc::c_char {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    target = 0 as *mut libc::c_char;
    match (*item).type_0 as libc::c_uint {
        1 => {
            if !s.is_null() {
                xasprintf(
                    &mut target as *mut *mut libc::c_char,
                    b"=%s:\0" as *const u8 as *const libc::c_char,
                    (*s).name,
                );
            }
        }
        2 => {
            if !(s.is_null() || wl.is_null()) {
                xasprintf(
                    &mut target as *mut *mut libc::c_char,
                    b"=%s:%u.\0" as *const u8 as *const libc::c_char,
                    (*s).name,
                    (*wl).idx,
                );
            }
        }
        3 => {
            if !(s.is_null() || wl.is_null() || wp.is_null()) {
                xasprintf(
                    &mut target as *mut *mut libc::c_char,
                    b"=%s:%u.%%%u\0" as *const u8 as *const libc::c_char,
                    (*s).name,
                    (*wl).idx,
                    (*wp).id,
                );
            }
        }
        0 | _ => {}
    }
    if target.is_null() {
        cmd_find_clear_state(fs, 0 as libc::c_int);
    } else {
        cmd_find_from_winlink_pane(fs, wl, wp, 0 as libc::c_int);
    }
    return target;
}
unsafe extern "C" fn window_tree_command_each(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut c: *mut client,
    mut key: key_code,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    name = window_tree_get_target(item, &mut fs);
    if !name.is_null() {
        mode_tree_run_command(c, &mut fs, (*data).entered, name);
    }
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_command_done(
    mut item: *mut cmdq_item,
    mut modedata: *mut libc::c_void,
) -> cmd_retval {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    if (*data).dead == 0 {
        mode_tree_build((*data).data);
        mode_tree_draw((*data).data);
        (*(*data).wp).flags |= 0x1 as libc::c_int;
    }
    window_tree_destroy(data);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn window_tree_command_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    if s.is_null() || *s as libc::c_int == '\0' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    (*data).entered = s;
    mode_tree_each_tagged(
        (*data).data,
        Some(
            window_tree_command_each
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut client,
                    key_code,
                ) -> (),
        ),
        c,
        0xffff00000000 as libc::c_ulonglong,
        1 as libc::c_int,
    );
    (*data).entered = 0 as *const libc::c_char;
    (*data).references += 1;
    (*data).references;
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"window_tree_command_done\0" as *const u8 as *const libc::c_char,
            Some(
                window_tree_command_done
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            data as *mut libc::c_void,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_tree_command_free(mut modedata: *mut libc::c_void) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    window_tree_destroy(data);
}
unsafe extern "C" fn window_tree_kill_each(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut c: *mut client,
    mut key: key_code,
) {
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    match (*item).type_0 as libc::c_uint {
        1 => {
            if !s.is_null() {
                server_destroy_session(s);
                session_destroy(
                    s,
                    (*::core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"window_tree_kill_each\0"))
                        .as_ptr(),
                );
            }
        }
        2 => {
            if !wl.is_null() {
                server_kill_window((*wl).window);
            }
        }
        3 => {
            if !wp.is_null() {
                server_kill_pane(wp);
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_tree_kill_current_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut mtd: *mut mode_tree_data = (*data).data;
    if s.is_null() || *s as libc::c_int == '\0' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if tolower(*s.offset(0 as libc::c_int as isize) as u_char as libc::c_int)
        != 'y' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return 0 as libc::c_int;
    }
    window_tree_kill_each(
        data as *mut libc::c_void,
        mode_tree_get_current(mtd),
        c,
        0xffff00000000 as libc::c_ulonglong,
    );
    (*data).references += 1;
    (*data).references;
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"window_tree_command_done\0" as *const u8 as *const libc::c_char,
            Some(
                window_tree_command_done
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            data as *mut libc::c_void,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_tree_kill_tagged_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut mtd: *mut mode_tree_data = (*data).data;
    if s.is_null() || *s as libc::c_int == '\0' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if tolower(*s.offset(0 as libc::c_int as isize) as u_char as libc::c_int)
        != 'y' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return 0 as libc::c_int;
    }
    mode_tree_each_tagged(
        mtd,
        Some(
            window_tree_kill_each
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut client,
                    key_code,
                ) -> (),
        ),
        c,
        0xffff00000000 as libc::c_ulonglong,
        1 as libc::c_int,
    );
    (*data).references += 1;
    (*data).references;
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"window_tree_command_done\0" as *const u8 as *const libc::c_char,
            Some(
                window_tree_command_done
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            data as *mut libc::c_void,
        ),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_tree_mouse(
    mut data: *mut window_tree_modedata,
    mut key: key_code,
    mut x: u_int,
    mut item: *mut window_tree_itemdata,
) -> key_code {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut loop_0: u_int = 0;
    if key != KEYC_MOUSEDOWN1_PANE as libc::c_int as key_code {
        return 0xffff00000000 as libc::c_ulonglong;
    }
    if (*data).left != -(1 as libc::c_int) && x <= (*data).left as u_int {
        return '<' as i32 as key_code;
    }
    if (*data).right != -(1 as libc::c_int) && x >= (*data).right as u_int {
        return '>' as i32 as key_code;
    }
    if (*data).left != -(1 as libc::c_int) {
        x = x.wrapping_sub((*data).left as u_int);
    } else if x != 0 as libc::c_int as u_int {
        x = x.wrapping_sub(1);
        x;
    }
    if x == 0 as libc::c_int as u_int || (*data).end == 0 as libc::c_int as u_int {
        x = 0 as libc::c_int as u_int;
    } else {
        x = x / (*data).each;
        if ((*data).start).wrapping_add(x) >= (*data).end {
            x = ((*data).end).wrapping_sub(1 as libc::c_int as u_int);
        }
    }
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    if (*item).type_0 as libc::c_uint
        == WINDOW_TREE_SESSION as libc::c_int as libc::c_uint
    {
        if s.is_null() {
            return 0xffff00000000 as libc::c_ulonglong;
        }
        mode_tree_expand_current((*data).data);
        loop_0 = 0 as libc::c_int as u_int;
        wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
        while !wl.is_null() {
            if loop_0 == ((*data).start).wrapping_add(x) {
                break;
            }
            loop_0 = loop_0.wrapping_add(1);
            loop_0;
            wl = winlinks_RB_NEXT(wl);
        }
        if !wl.is_null() {
            mode_tree_set_current((*data).data, wl as uint64_t);
        }
        return '\r' as i32 as key_code;
    }
    if (*item).type_0 as libc::c_uint
        == WINDOW_TREE_WINDOW as libc::c_int as libc::c_uint
    {
        if wl.is_null() {
            return 0xffff00000000 as libc::c_ulonglong;
        }
        mode_tree_expand_current((*data).data);
        loop_0 = 0 as libc::c_int as u_int;
        wp = (*(*wl).window).panes.tqh_first;
        while !wp.is_null() {
            if loop_0 == ((*data).start).wrapping_add(x) {
                break;
            }
            loop_0 = loop_0.wrapping_add(1);
            loop_0;
            wp = (*wp).entry.tqe_next;
        }
        if !wp.is_null() {
            mode_tree_set_current((*data).data, wp as uint64_t);
        }
        return '\r' as i32 as key_code;
    }
    return 0xffff00000000 as libc::c_ulonglong;
}
unsafe extern "C" fn window_tree_key(
    mut wp: *mut window_pane,
    mut c: *mut client,
    mut s: *mut session,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut data: *mut window_tree_modedata = (*wp).modedata
        as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut new_item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut finished: libc::c_int = 0;
    let mut tagged: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut idx: u_int = 0;
    let mut ns: *mut session = 0 as *mut session;
    let mut nwl: *mut winlink = 0 as *mut winlink;
    let mut nwp: *mut window_pane = 0 as *mut window_pane;
    item = mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
    finished = mode_tree_key((*data).data, c, &mut key, m, &mut x, &mut y);
    new_item = mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
    if item != new_item {
        item = new_item;
        (*data).offset = 0 as libc::c_int;
    }
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
        key = window_tree_mouse(data, key, x, item);
    }
    match key {
        60 => {
            (*data).offset -= 1;
            (*data).offset;
        }
        62 => {
            (*data).offset += 1;
            (*data).offset;
        }
        120 => {
            window_tree_pull_item(item, &mut ns, &mut nwl, &mut nwp);
            match (*item).type_0 as libc::c_uint {
                1 => {
                    if !ns.is_null() {
                        xasprintf(
                            &mut prompt as *mut *mut libc::c_char,
                            b"Kill session %s? \0" as *const u8 as *const libc::c_char,
                            (*ns).name,
                        );
                    }
                }
                2 => {
                    if !nwl.is_null() {
                        xasprintf(
                            &mut prompt as *mut *mut libc::c_char,
                            b"Kill window %u? \0" as *const u8 as *const libc::c_char,
                            (*nwl).idx,
                        );
                    }
                }
                3 => {
                    if !(nwp.is_null()
                        || window_pane_index(nwp, &mut idx) != 0 as libc::c_int)
                    {
                        xasprintf(
                            &mut prompt as *mut *mut libc::c_char,
                            b"Kill pane %u? \0" as *const u8 as *const libc::c_char,
                            idx,
                        );
                    }
                }
                0 | _ => {}
            }
            if !prompt.is_null() {
                (*data).references += 1;
                (*data).references;
                status_prompt_set(
                    c,
                    prompt,
                    b"\0" as *const u8 as *const libc::c_char,
                    Some(
                        window_tree_kill_current_callback
                            as unsafe extern "C" fn(
                                *mut client,
                                *mut libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_tree_command_free
                            as unsafe extern "C" fn(*mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1 as libc::c_int | 0x8 as libc::c_int,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        88 => {
            tagged = mode_tree_count_tagged((*data).data);
            if !(tagged == 0 as libc::c_int as u_int) {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"Kill %u tagged? \0" as *const u8 as *const libc::c_char,
                    tagged,
                );
                (*data).references += 1;
                (*data).references;
                status_prompt_set(
                    c,
                    prompt,
                    b"\0" as *const u8 as *const libc::c_char,
                    Some(
                        window_tree_kill_tagged_callback
                            as unsafe extern "C" fn(
                                *mut client,
                                *mut libc::c_void,
                                *const libc::c_char,
                                libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_tree_command_free
                            as unsafe extern "C" fn(*mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1 as libc::c_int | 0x8 as libc::c_int,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        58 => {
            tagged = mode_tree_count_tagged((*data).data);
            if tagged != 0 as libc::c_int as u_int {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"(%u tagged) \0" as *const u8 as *const libc::c_char,
                    tagged,
                );
            } else {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"(current) \0" as *const u8 as *const libc::c_char,
                );
            }
            (*data).references += 1;
            (*data).references;
            status_prompt_set(
                c,
                prompt,
                b"\0" as *const u8 as *const libc::c_char,
                Some(
                    window_tree_command_callback
                        as unsafe extern "C" fn(
                            *mut client,
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(
                    window_tree_command_free
                        as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                data as *mut libc::c_void,
                0x8 as libc::c_int,
            );
            free(prompt as *mut libc::c_void);
        }
        13 => {
            item = mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
            name = window_tree_get_target(item, &mut fs);
            if !name.is_null() {
                mode_tree_run_command(
                    c,
                    0 as *mut cmd_find_state,
                    (*data).command,
                    name,
                );
            }
            finished = 1 as libc::c_int;
            free(name as *mut libc::c_void);
        }
        _ => {}
    }
    if finished != 0 {
        window_pane_reset_mode(wp);
    } else {
        mode_tree_draw((*data).data);
        (*wp).flags |= 0x1 as libc::c_int;
    };
}
