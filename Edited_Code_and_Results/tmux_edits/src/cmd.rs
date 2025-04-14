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
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    fn options_array_get(_: *mut options_entry, _: u_int) -> *const libc::c_char;
    fn options_array_size(_: *mut options_entry, _: *mut u_int) -> libc::c_int;
    fn args_parse(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut args;
    fn args_free(_: *mut args);
    fn args_print(_: *mut args) -> *mut libc::c_char;
    fn cmd_string_split(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    static mut server_proc: *mut tmuxproc;
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window) -> *mut winlink;
    fn window_find_by_id(_: u_int) -> *mut window;
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    fn session_find_by_id(_: u_int) -> *mut session;
    fn log_debug(_: *const libc::c_char, _: ...);
    static cmd_attach_session_entry: cmd_entry;
    static cmd_bind_key_entry: cmd_entry;
    static cmd_break_pane_entry: cmd_entry;
    static cmd_capture_pane_entry: cmd_entry;
    static cmd_choose_buffer_entry: cmd_entry;
    static cmd_choose_client_entry: cmd_entry;
    static cmd_choose_tree_entry: cmd_entry;
    static cmd_clear_history_entry: cmd_entry;
    static cmd_clock_mode_entry: cmd_entry;
    static cmd_command_prompt_entry: cmd_entry;
    static cmd_confirm_before_entry: cmd_entry;
    static cmd_copy_mode_entry: cmd_entry;
    static cmd_delete_buffer_entry: cmd_entry;
    static cmd_detach_client_entry: cmd_entry;
    static cmd_display_message_entry: cmd_entry;
    static cmd_display_panes_entry: cmd_entry;
    static cmd_find_window_entry: cmd_entry;
    static cmd_has_session_entry: cmd_entry;
    static cmd_if_shell_entry: cmd_entry;
    static cmd_join_pane_entry: cmd_entry;
    static cmd_kill_pane_entry: cmd_entry;
    static cmd_kill_server_entry: cmd_entry;
    static cmd_kill_session_entry: cmd_entry;
    static cmd_kill_window_entry: cmd_entry;
    static cmd_last_pane_entry: cmd_entry;
    static cmd_last_window_entry: cmd_entry;
    static cmd_link_window_entry: cmd_entry;
    static cmd_list_buffers_entry: cmd_entry;
    static cmd_list_clients_entry: cmd_entry;
    static cmd_list_commands_entry: cmd_entry;
    static cmd_list_keys_entry: cmd_entry;
    static cmd_list_panes_entry: cmd_entry;
    static cmd_list_sessions_entry: cmd_entry;
    static cmd_list_windows_entry: cmd_entry;
    static cmd_load_buffer_entry: cmd_entry;
    static cmd_lock_client_entry: cmd_entry;
    static cmd_lock_server_entry: cmd_entry;
    static cmd_lock_session_entry: cmd_entry;
    static cmd_move_pane_entry: cmd_entry;
    static cmd_move_window_entry: cmd_entry;
    static cmd_new_session_entry: cmd_entry;
    static cmd_new_window_entry: cmd_entry;
    static cmd_next_layout_entry: cmd_entry;
    static cmd_next_window_entry: cmd_entry;
    static cmd_paste_buffer_entry: cmd_entry;
    static cmd_pipe_pane_entry: cmd_entry;
    static cmd_previous_layout_entry: cmd_entry;
    static cmd_previous_window_entry: cmd_entry;
    static cmd_refresh_client_entry: cmd_entry;
    static cmd_rename_session_entry: cmd_entry;
    static cmd_rename_window_entry: cmd_entry;
    static cmd_resize_pane_entry: cmd_entry;
    static cmd_respawn_pane_entry: cmd_entry;
    static cmd_respawn_window_entry: cmd_entry;
    static cmd_rotate_window_entry: cmd_entry;
    static cmd_run_shell_entry: cmd_entry;
    static cmd_save_buffer_entry: cmd_entry;
    static cmd_select_layout_entry: cmd_entry;
    static cmd_select_pane_entry: cmd_entry;
    static cmd_select_window_entry: cmd_entry;
    static cmd_send_keys_entry: cmd_entry;
    static cmd_send_prefix_entry: cmd_entry;
    static cmd_set_buffer_entry: cmd_entry;
    static cmd_set_environment_entry: cmd_entry;
    static cmd_set_hook_entry: cmd_entry;
    static cmd_set_option_entry: cmd_entry;
    static cmd_set_window_option_entry: cmd_entry;
    static cmd_show_buffer_entry: cmd_entry;
    static cmd_show_environment_entry: cmd_entry;
    static cmd_show_hooks_entry: cmd_entry;
    static cmd_show_messages_entry: cmd_entry;
    static cmd_show_options_entry: cmd_entry;
    static cmd_show_window_options_entry: cmd_entry;
    static cmd_source_file_entry: cmd_entry;
    static cmd_split_window_entry: cmd_entry;
    static cmd_start_server_entry: cmd_entry;
    static cmd_suspend_client_entry: cmd_entry;
    static cmd_swap_pane_entry: cmd_entry;
    static cmd_swap_window_entry: cmd_entry;
    static cmd_switch_client_entry: cmd_entry;
    static cmd_unbind_key_entry: cmd_entry;
    static cmd_unlink_window_entry: cmd_entry;
    static cmd_wait_for_entry: cmd_entry;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
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
#[no_mangle]
pub static mut cmd_table: [*const cmd_entry; 84] = unsafe {
    [
        &cmd_attach_session_entry as *const cmd_entry,
        &cmd_bind_key_entry as *const cmd_entry,
        &cmd_break_pane_entry as *const cmd_entry,
        &cmd_capture_pane_entry as *const cmd_entry,
        &cmd_choose_buffer_entry as *const cmd_entry,
        &cmd_choose_client_entry as *const cmd_entry,
        &cmd_choose_tree_entry as *const cmd_entry,
        &cmd_clear_history_entry as *const cmd_entry,
        &cmd_clock_mode_entry as *const cmd_entry,
        &cmd_command_prompt_entry as *const cmd_entry,
        &cmd_confirm_before_entry as *const cmd_entry,
        &cmd_copy_mode_entry as *const cmd_entry,
        &cmd_delete_buffer_entry as *const cmd_entry,
        &cmd_detach_client_entry as *const cmd_entry,
        &cmd_display_message_entry as *const cmd_entry,
        &cmd_display_panes_entry as *const cmd_entry,
        &cmd_find_window_entry as *const cmd_entry,
        &cmd_has_session_entry as *const cmd_entry,
        &cmd_if_shell_entry as *const cmd_entry,
        &cmd_join_pane_entry as *const cmd_entry,
        &cmd_kill_pane_entry as *const cmd_entry,
        &cmd_kill_server_entry as *const cmd_entry,
        &cmd_kill_session_entry as *const cmd_entry,
        &cmd_kill_window_entry as *const cmd_entry,
        &cmd_last_pane_entry as *const cmd_entry,
        &cmd_last_window_entry as *const cmd_entry,
        &cmd_link_window_entry as *const cmd_entry,
        &cmd_list_buffers_entry as *const cmd_entry,
        &cmd_list_clients_entry as *const cmd_entry,
        &cmd_list_commands_entry as *const cmd_entry,
        &cmd_list_keys_entry as *const cmd_entry,
        &cmd_list_panes_entry as *const cmd_entry,
        &cmd_list_sessions_entry as *const cmd_entry,
        &cmd_list_windows_entry as *const cmd_entry,
        &cmd_load_buffer_entry as *const cmd_entry,
        &cmd_lock_client_entry as *const cmd_entry,
        &cmd_lock_server_entry as *const cmd_entry,
        &cmd_lock_session_entry as *const cmd_entry,
        &cmd_move_pane_entry as *const cmd_entry,
        &cmd_move_window_entry as *const cmd_entry,
        &cmd_new_session_entry as *const cmd_entry,
        &cmd_new_window_entry as *const cmd_entry,
        &cmd_next_layout_entry as *const cmd_entry,
        &cmd_next_window_entry as *const cmd_entry,
        &cmd_paste_buffer_entry as *const cmd_entry,
        &cmd_pipe_pane_entry as *const cmd_entry,
        &cmd_previous_layout_entry as *const cmd_entry,
        &cmd_previous_window_entry as *const cmd_entry,
        &cmd_refresh_client_entry as *const cmd_entry,
        &cmd_rename_session_entry as *const cmd_entry,
        &cmd_rename_window_entry as *const cmd_entry,
        &cmd_resize_pane_entry as *const cmd_entry,
        &cmd_respawn_pane_entry as *const cmd_entry,
        &cmd_respawn_window_entry as *const cmd_entry,
        &cmd_rotate_window_entry as *const cmd_entry,
        &cmd_run_shell_entry as *const cmd_entry,
        &cmd_save_buffer_entry as *const cmd_entry,
        &cmd_select_layout_entry as *const cmd_entry,
        &cmd_select_pane_entry as *const cmd_entry,
        &cmd_select_window_entry as *const cmd_entry,
        &cmd_send_keys_entry as *const cmd_entry,
        &cmd_send_prefix_entry as *const cmd_entry,
        &cmd_set_buffer_entry as *const cmd_entry,
        &cmd_set_environment_entry as *const cmd_entry,
        &cmd_set_hook_entry as *const cmd_entry,
        &cmd_set_option_entry as *const cmd_entry,
        &cmd_set_window_option_entry as *const cmd_entry,
        &cmd_show_buffer_entry as *const cmd_entry,
        &cmd_show_environment_entry as *const cmd_entry,
        &cmd_show_hooks_entry as *const cmd_entry,
        &cmd_show_messages_entry as *const cmd_entry,
        &cmd_show_options_entry as *const cmd_entry,
        &cmd_show_window_options_entry as *const cmd_entry,
        &cmd_source_file_entry as *const cmd_entry,
        &cmd_split_window_entry as *const cmd_entry,
        &cmd_start_server_entry as *const cmd_entry,
        &cmd_suspend_client_entry as *const cmd_entry,
        &cmd_swap_pane_entry as *const cmd_entry,
        &cmd_swap_window_entry as *const cmd_entry,
        &cmd_switch_client_entry as *const cmd_entry,
        &cmd_unbind_key_entry as *const cmd_entry,
        &cmd_unlink_window_entry as *const cmd_entry,
        &cmd_wait_for_entry as *const cmd_entry,
        0 as *const cmd_entry,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn cmd_pack_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut arglen: size_t = 0;
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *buf = '\0' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        if strlcpy(buf, *argv.offset(i as isize), len) >= len {
            return -(1 as libc::c_int);
        }
        arglen = (strlen(*argv.offset(i as isize)))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        buf = buf.offset(arglen as isize);
        len = len.wrapping_sub(arglen);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_unpack_argv(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut arglen: size_t = 0;
    if argc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *argv = xcalloc(
        argc as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    *buf
        .offset(
            len.wrapping_sub(1 as libc::c_int as size_t) as isize,
        ) = '\0' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        if len == 0 as libc::c_int as size_t {
            cmd_free_argv(argc, *argv);
            return -(1 as libc::c_int);
        }
        arglen = (strlen(buf)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let ref mut fresh0 = *(*argv).offset(i as isize);
        *fresh0 = xstrdup(buf);
        buf = buf.offset(arglen as isize);
        len = len.wrapping_sub(arglen);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_copy_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return 0 as *mut *mut libc::c_char;
    }
    new_argv = xcalloc(
        (argc + 1 as libc::c_int) as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null() {
            let ref mut fresh1 = *new_argv.offset(i as isize);
            *fresh1 = xstrdup(*argv.offset(i as isize));
        }
        i += 1;
        i;
    }
    return new_argv;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_free_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < argc {
        free(*argv.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(argv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_stringify_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: size_t = 0;
    if argc == 0 as libc::c_int {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    len = 0 as libc::c_int as size_t;
    buf = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
        buf = xrealloc(buf as *mut libc::c_void, len) as *mut libc::c_char;
        if i == 0 as libc::c_int {
            *buf = '\0' as i32 as libc::c_char;
        } else {
            strlcat(buf, b" \0" as *const u8 as *const libc::c_char, len);
        }
        strlcat(buf, *argv.offset(i as isize), len);
        i += 1;
        i;
    }
    return buf;
}
unsafe extern "C" fn cmd_try_alias(
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut old_argc: libc::c_int = *argc;
    let mut new_argc: libc::c_int = 0;
    let mut old_argv: *mut *mut libc::c_char = *argv;
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: u_int = 0;
    let mut idx: u_int = 0;
    let mut i: libc::c_int = 0;
    let mut wanted: size_t = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    o = options_get_only(
        global_options,
        b"command-alias\0" as *const u8 as *const libc::c_char,
    );
    if o.is_null() || options_array_size(o, &mut size) == -(1 as libc::c_int)
        || size == 0 as libc::c_int as u_int
    {
        return -(1 as libc::c_int);
    }
    wanted = strlen(*old_argv.offset(0 as libc::c_int as isize));
    idx = 0 as libc::c_int as u_int;
    while idx < size {
        s = options_array_get(o, idx);
        if !s.is_null() {
            cp = strchr(s, '=' as i32);
            if !(cp.is_null() || cp.offset_from(s) as libc::c_long as size_t != wanted) {
                if strncmp(*old_argv.offset(0 as libc::c_int as isize), s, wanted)
                    == 0 as libc::c_int
                {
                    break;
                }
            }
        }
        idx = idx.wrapping_add(1);
        idx;
    }
    if idx == size {
        return -(1 as libc::c_int);
    }
    if cmd_string_split(
        cp.offset(1 as libc::c_int as isize),
        &mut new_argc,
        &mut new_argv,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    *argc = new_argc + old_argc - 1 as libc::c_int;
    *argv = xcalloc(
        (*argc + 1 as libc::c_int) as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < new_argc {
        let ref mut fresh2 = *(*argv).offset(i as isize);
        *fresh2 = xstrdup(*new_argv.offset(i as isize));
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < old_argc {
        let ref mut fresh3 = *(*argv).offset((new_argc + i - 1 as libc::c_int) as isize);
        *fresh3 = xstrdup(*old_argv.offset(i as isize));
        i += 1;
        i;
    }
    log_debug(
        b"alias: %s=%s\0" as *const u8 as *const libc::c_char,
        *old_argv.offset(0 as libc::c_int as isize),
        cp.offset(1 as libc::c_int as isize),
    );
    i = 0 as libc::c_int;
    while i < *argc {
        log_debug(
            b"alias: argv[%d] = %s\0" as *const u8 as *const libc::c_char,
            i,
            *(*argv).offset(i as isize),
        );
        i += 1;
        i;
    }
    cmd_free_argv(new_argc, new_argv);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_parse(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut file: *const libc::c_char,
    mut line: u_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut cmd {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut entryp: *mut *const cmd_entry = 0 as *mut *const cmd_entry;
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut args: *mut args = 0 as *mut args;
    let mut s: [libc::c_char; 8192] = [0; 8192];
    let mut ambiguous: libc::c_int = 0;
    let mut allocated: libc::c_int = 0 as libc::c_int;
    *cause = 0 as *mut libc::c_char;
    if argc == 0 as libc::c_int {
        xasprintf(cause, b"no command\0" as *const u8 as *const libc::c_char);
        return 0 as *mut cmd;
    }
    name = *argv.offset(0 as libc::c_int as isize);
    loop {
        ambiguous = 0 as libc::c_int;
        entry = 0 as *const cmd_entry;
        entryp = cmd_table.as_mut_ptr();
        while !(*entryp).is_null() {
            if !((**entryp).alias).is_null()
                && strcmp((**entryp).alias, *argv.offset(0 as libc::c_int as isize))
                    == 0 as libc::c_int
            {
                ambiguous = 0 as libc::c_int;
                entry = *entryp;
                break;
            } else {
                if !(strncmp(
                    (**entryp).name,
                    *argv.offset(0 as libc::c_int as isize),
                    strlen(*argv.offset(0 as libc::c_int as isize)),
                ) != 0 as libc::c_int)
                {
                    if !entry.is_null() {
                        ambiguous = 1 as libc::c_int;
                    }
                    entry = *entryp;
                    if strcmp((*entry).name, *argv.offset(0 as libc::c_int as isize))
                        == 0 as libc::c_int
                    {
                        break;
                    }
                }
                entryp = entryp.offset(1);
                entryp;
            }
        }
        if !((ambiguous != 0 || entry.is_null()) && !server_proc.is_null()
            && allocated == 0 && cmd_try_alias(&mut argc, &mut argv) == 0 as libc::c_int)
        {
            break;
        }
        allocated = 1 as libc::c_int;
    }
    if ambiguous != 0 {
        *s.as_mut_ptr() = '\0' as i32 as libc::c_char;
        entryp = cmd_table.as_mut_ptr();
        while !(*entryp).is_null() {
            if !(strncmp(
                (**entryp).name,
                *argv.offset(0 as libc::c_int as isize),
                strlen(*argv.offset(0 as libc::c_int as isize)),
            ) != 0 as libc::c_int)
            {
                if strlcat(
                    s.as_mut_ptr(),
                    (**entryp).name,
                    ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                ) >= ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                {
                    break;
                }
                if strlcat(
                    s.as_mut_ptr(),
                    b", \0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                ) >= ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                {
                    break;
                }
            }
            entryp = entryp.offset(1);
            entryp;
        }
        s[(strlen(s.as_mut_ptr())).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        xasprintf(
            cause,
            b"ambiguous command: %s, could be: %s\0" as *const u8 as *const libc::c_char,
            name,
            s.as_mut_ptr(),
        );
        return 0 as *mut cmd;
    } else {
        if entry.is_null() {
            xasprintf(
                cause,
                b"unknown command: %s\0" as *const u8 as *const libc::c_char,
                name,
            );
            return 0 as *mut cmd;
        }
        args = args_parse((*entry).args.template, argc, argv);
        if !args.is_null() {
            if !((*entry).args.lower != -(1 as libc::c_int)
                && (*args).argc < (*entry).args.lower)
            {
                if !((*entry).args.upper != -(1 as libc::c_int)
                    && (*args).argc > (*entry).args.upper)
                {
                    cmd = xcalloc(
                        1 as libc::c_int as size_t,
                        ::core::mem::size_of::<cmd>() as libc::c_ulong,
                    ) as *mut cmd;
                    (*cmd).entry = entry;
                    (*cmd).args = args;
                    if !file.is_null() {
                        (*cmd).file = xstrdup(file);
                    }
                    (*cmd).line = line;
                    if allocated != 0 {
                        cmd_free_argv(argc, argv);
                    }
                    return cmd;
                }
            }
        }
        if !args.is_null() {
            args_free(args);
        }
        xasprintf(
            cause,
            b"usage: %s %s\0" as *const u8 as *const libc::c_char,
            (*entry).name,
            (*entry).usage,
        );
        return 0 as *mut cmd;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmd_print(mut cmd: *mut cmd) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = args_print((*cmd).args);
    if *s as libc::c_int != '\0' as i32 {
        xasprintf(
            &mut out as *mut *mut libc::c_char,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            (*(*cmd).entry).name,
            s,
        );
    } else {
        out = xstrdup((*(*cmd).entry).name);
    }
    free(s as *mut libc::c_void);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_at(
    mut wp: *mut window_pane,
    mut m: *mut mouse_event,
    mut xp: *mut u_int,
    mut yp: *mut u_int,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if last != 0 {
        x = (*m).lx;
        y = (*m).ly;
    } else {
        x = (*m).x;
        y = (*m).y;
    }
    if (*m).statusat == 0 as libc::c_int && y > 0 as libc::c_int as u_int {
        y = y.wrapping_sub(1);
        y;
    } else if (*m).statusat > 0 as libc::c_int && y >= (*m).statusat as u_int {
        y = ((*m).statusat - 1 as libc::c_int) as u_int;
    }
    if x < (*wp).xoff || x >= ((*wp).xoff).wrapping_add((*wp).sx) {
        return -(1 as libc::c_int);
    }
    if y < (*wp).yoff || y >= ((*wp).yoff).wrapping_add((*wp).sy) {
        return -(1 as libc::c_int);
    }
    if !xp.is_null() {
        *xp = x.wrapping_sub((*wp).xoff);
    }
    if !yp.is_null() {
        *yp = y.wrapping_sub((*wp).yoff);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_window(
    mut m: *mut mouse_event,
    mut sp: *mut *mut session,
) -> *mut winlink {
    let mut s: *mut session = 0 as *mut session;
    let mut w: *mut window = 0 as *mut window;
    if (*m).valid == 0 || (*m).s == -(1 as libc::c_int) || (*m).w == -(1 as libc::c_int)
    {
        return 0 as *mut winlink;
    }
    s = session_find_by_id((*m).s as u_int);
    if s.is_null() {
        return 0 as *mut winlink;
    }
    w = window_find_by_id((*m).w as u_int);
    if w.is_null() {
        return 0 as *mut winlink;
    }
    if !sp.is_null() {
        *sp = s;
    }
    return winlink_find_by_window(&mut (*s).windows, w);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_pane(
    mut m: *mut mouse_event,
    mut sp: *mut *mut session,
    mut wlp: *mut *mut winlink,
) -> *mut window_pane {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    wl = cmd_mouse_window(m, sp);
    if wl.is_null() {
        return 0 as *mut window_pane;
    }
    wp = window_pane_find_by_id((*m).wp as u_int);
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    if window_has_pane((*wl).window, wp) == 0 {
        return 0 as *mut window_pane;
    }
    if !wlp.is_null() {
        *wlp = wl;
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_template_replace(
    mut template: *const libc::c_char,
    mut s: *const libc::c_char,
    mut idx: libc::c_int,
) -> *mut libc::c_char {
    let mut ch: libc::c_char = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let quote: [libc::c_char; 4] = *::core::mem::transmute::<
        &[u8; 4],
        &[libc::c_char; 4],
    >(b"\"\\$\0");
    let mut replaced: libc::c_int = 0;
    let mut quoted: libc::c_int = 0;
    let mut len: size_t = 0;
    if (strchr(template, '%' as i32)).is_null() {
        return xstrdup(template);
    }
    buf = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
    *buf = '\0' as i32 as libc::c_char;
    len = 0 as libc::c_int as size_t;
    replaced = 0 as libc::c_int;
    ptr = template;
    let mut current_block_26: u64;
    while *ptr as libc::c_int != '\0' as i32 {
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        ch = *fresh4;
        match ch as libc::c_int {
            37 => {
                if (*ptr as libc::c_int) < '1' as i32 || *ptr as libc::c_int > '9' as i32
                    || *ptr as libc::c_int - '0' as i32 != idx
                {
                    if *ptr as libc::c_int != '%' as i32 || replaced != 0 {
                        current_block_26 = 11298138898191919651;
                    } else {
                        replaced = 1 as libc::c_int;
                        current_block_26 = 2868539653012386629;
                    }
                } else {
                    current_block_26 = 2868539653012386629;
                }
                match current_block_26 {
                    11298138898191919651 => {}
                    _ => {
                        ptr = ptr.offset(1);
                        ptr;
                        quoted = (*ptr as libc::c_int == '%' as i32) as libc::c_int;
                        if quoted != 0 {
                            ptr = ptr.offset(1);
                            ptr;
                        }
                        buf = xrealloc(
                            buf as *mut libc::c_void,
                            len
                                .wrapping_add(
                                    (strlen(s)).wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as *mut libc::c_char;
                        cp = s;
                        while *cp as libc::c_int != '\0' as i32 {
                            if quoted != 0
                                && !(strchr(quote.as_ptr(), *cp as libc::c_int)).is_null()
                            {
                                let fresh5 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh5 as isize) = '\\' as i32 as libc::c_char;
                            }
                            if quoted != 0 && *cp as libc::c_int == ';' as i32 {
                                let fresh6 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh6 as isize) = '\\' as i32 as libc::c_char;
                                let fresh7 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh7 as isize) = '\\' as i32 as libc::c_char;
                            }
                            let fresh8 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh8 as isize) = *cp;
                            cp = cp.offset(1);
                            cp;
                        }
                        *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
                        continue;
                    }
                }
            }
            _ => {}
        }
        buf = xrealloc(
            buf as *mut libc::c_void,
            len.wrapping_add(2 as libc::c_int as size_t),
        ) as *mut libc::c_char;
        let fresh9 = len;
        len = len.wrapping_add(1);
        *buf.offset(fresh9 as isize) = ch;
        *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    return buf;
}
