use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fparseln(
        _: *mut FILE,
        _: *mut size_t,
        _: *mut size_t,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn find_home() -> *const libc::c_char;
    fn cfg_add_cause(_: *const libc::c_char, _: ...);
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    fn format_create(
        _: *mut client,
        _: *mut cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut format_tree;
    fn format_free(_: *mut format_tree);
    fn format_expand(_: *mut format_tree, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn cmdq_insert_after(_: *mut cmdq_item, _: *mut cmdq_item);
    fn cmdq_append(_: *mut client, _: *mut cmdq_item);
    fn cmdq_print(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    fn cmd_string_parse(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut cmd_list;
    static mut clients: clients;
    fn status_prompt_load_history();
    fn window_pane_set_mode(
        _: *mut window_pane,
        _: *const window_mode,
        _: *mut cmd_find_state,
        _: *mut args,
    ) -> libc::c_int;
    static window_copy_mode: window_mode;
    fn window_copy_init_for_output(_: *mut window_pane);
    fn window_copy_add(_: *mut window_pane, _: *const libc::c_char, _: ...);
    static mut sessions: sessions;
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    fn log_debug(_: *const libc::c_char, _: ...);
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
    pub message_log: C2RustUnnamed_25,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_24,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub wlmouse: libc::c_int,
    pub references: libc::c_int,
    pub entry: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub gentry: C2RustUnnamed_11,
    pub entry: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
    pub entry: C2RustUnnamed_14,
    pub wentry: C2RustUnnamed_13,
    pub sentry: C2RustUnnamed_12,
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
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub alerts_entry: C2RustUnnamed_17,
    pub options: *mut options,
    pub style: grid_cell,
    pub active_style: grid_cell,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_16,
    pub entry: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
    pub entry: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
    pub entry: C2RustUnnamed_20,
    pub tree_entry: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
    pub lineflag: C2RustUnnamed_21,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type C2RustUnnamed_21 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed_21 = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed_21 = 1;
pub const LINE_SEL_NONE: C2RustUnnamed_21 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub offset: u_int,
    pub data: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
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
pub type C2RustUnnamed_24 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_24 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_24 = 0;
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
pub struct C2RustUnnamed_25 {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: time_t,
    pub entry: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub list: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
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
    pub qentry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_32,
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
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
pub struct C2RustUnnamed_32 {
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
    pub term_type: C2RustUnnamed_33,
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
pub type C2RustUnnamed_33 = libc::c_uint;
pub const TTY_UNKNOWN: C2RustUnnamed_33 = 6;
pub const TTY_VT420: C2RustUnnamed_33 = 5;
pub const TTY_VT320: C2RustUnnamed_33 = 4;
pub const TTY_VT220: C2RustUnnamed_33 = 3;
pub const TTY_VT102: C2RustUnnamed_33 = 2;
pub const TTY_VT101: C2RustUnnamed_33 = 1;
pub const TTY_VT100: C2RustUnnamed_33 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub references: u_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_cond {
    pub line: size_t,
    pub met: libc::c_int,
    pub skip: libc::c_int,
    pub saw_else: libc::c_int,
    pub entry: C2RustUnnamed_35,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut cfg_cond,
    pub tqe_prev: *mut *mut cfg_cond,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_conds {
    pub tqh_first: *mut cfg_cond,
    pub tqh_last: *mut *mut cfg_cond,
}
static mut cfg_file: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut cfg_finished: libc::c_int = 0;
#[no_mangle]
pub static mut cfg_causes: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut cfg_ncauses: u_int = 0;
static mut cfg_item: *mut cmdq_item = 0 as *const cmdq_item as *mut cmdq_item;
unsafe extern "C" fn cfg_client_done(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    if cfg_finished == 0 {
        return CMD_RETURN_WAIT;
    }
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn cfg_done(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    if cfg_finished != 0 {
        return CMD_RETURN_NORMAL;
    }
    cfg_finished = 1 as libc::c_int;
    if !(sessions.rbh_root).is_null() {
        cfg_show_causes(sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int)));
    }
    if !cfg_item.is_null() {
        (*cfg_item).flags &= !(0x2 as libc::c_int);
    }
    status_prompt_load_history();
    return CMD_RETURN_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn set_cfg_file(mut path: *const libc::c_char) {
    free(cfg_file as *mut libc::c_void);
    cfg_file = xstrdup(path);
}
#[no_mangle]
pub unsafe extern "C" fn start_cfg() {
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    if !c.is_null() {
        cfg_item = cmdq_get_callback1(
            b"cfg_client_done\0" as *const u8 as *const libc::c_char,
            Some(
                cfg_client_done
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            0 as *mut libc::c_void,
        );
        cmdq_append(c, cfg_item);
    }
    load_cfg(
        b"/etc/tmux.conf\0" as *const u8 as *const libc::c_char,
        0 as *mut client,
        0 as *mut cmdq_item,
        1 as libc::c_int,
    );
    if cfg_file.is_null()
        && {
            home = find_home();
            !home.is_null()
        }
    {
        xasprintf(
            &mut cfg_file as *mut *mut libc::c_char,
            b"%s/.tmux.conf\0" as *const u8 as *const libc::c_char,
            home,
        );
        quiet = 1 as libc::c_int;
    }
    if !cfg_file.is_null() {
        load_cfg(cfg_file, 0 as *mut client, 0 as *mut cmdq_item, quiet);
    }
    cmdq_append(
        0 as *mut client,
        cmdq_get_callback1(
            b"cfg_done\0" as *const u8 as *const libc::c_char,
            Some(
                cfg_done
                    as unsafe extern "C" fn(
                        *mut cmdq_item,
                        *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            0 as *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn cfg_check_condition(
    mut path: *const libc::c_char,
    mut line: size_t,
    mut p: *const libc::c_char,
    mut skip: *mut libc::c_int,
) -> libc::c_int {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    while *(*__ctype_b_loc()).offset(*p as u_char as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
        p;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        cfg_add_cause(
            b"%s:%zu: invalid condition\0" as *const u8 as *const libc::c_char,
            path,
            line,
        );
        *skip = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    ft = format_create(
        0 as *mut client,
        0 as *mut cmdq_item,
        0 as libc::c_int,
        0x4 as libc::c_int,
    );
    s = format_expand(ft, p);
    result = format_true(s);
    free(s as *mut libc::c_void);
    format_free(ft);
    *skip = result;
    return result;
}
unsafe extern "C" fn cfg_handle_if(
    mut path: *const libc::c_char,
    mut line: size_t,
    mut conds: *mut cfg_conds,
    mut p: *const libc::c_char,
) {
    let mut cond: *mut cfg_cond = 0 as *mut cfg_cond;
    let mut parent: *mut cfg_cond = (*conds).tqh_first;
    cond = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<cfg_cond>() as libc::c_ulong,
    ) as *mut cfg_cond;
    (*cond).line = line;
    if parent.is_null() || (*parent).met != 0 {
        (*cond).met = cfg_check_condition(path, line, p, &mut (*cond).skip);
    } else {
        (*cond).skip = 1 as libc::c_int;
    }
    (*cond).saw_else = 0 as libc::c_int;
    (*cond).entry.tqe_next = (*conds).tqh_first;
    if !((*cond).entry.tqe_next).is_null() {
        (*(*conds).tqh_first).entry.tqe_prev = &mut (*cond).entry.tqe_next;
    } else {
        (*conds).tqh_last = &mut (*cond).entry.tqe_next;
    }
    (*conds).tqh_first = cond;
    (*cond).entry.tqe_prev = &mut (*conds).tqh_first;
}
unsafe extern "C" fn cfg_handle_elif(
    mut path: *const libc::c_char,
    mut line: size_t,
    mut conds: *mut cfg_conds,
    mut p: *const libc::c_char,
) {
    let mut cond: *mut cfg_cond = (*conds).tqh_first;
    if cond.is_null() || (*cond).saw_else != 0 {
        cfg_add_cause(
            b"%s:%zu: unexpected %%elif\0" as *const u8 as *const libc::c_char,
            path,
            line,
        );
    } else if (*cond).skip == 0 {
        (*cond).met = cfg_check_condition(path, line, p, &mut (*cond).skip);
    } else {
        (*cond).met = 0 as libc::c_int;
    };
}
unsafe extern "C" fn cfg_handle_else(
    mut path: *const libc::c_char,
    mut line: size_t,
    mut conds: *mut cfg_conds,
) {
    let mut cond: *mut cfg_cond = (*conds).tqh_first;
    if cond.is_null() || (*cond).saw_else != 0 {
        cfg_add_cause(
            b"%s:%zu: unexpected %%else\0" as *const u8 as *const libc::c_char,
            path,
            line,
        );
        return;
    }
    (*cond).saw_else = 1 as libc::c_int;
    (*cond).met = ((*cond).skip == 0) as libc::c_int;
    (*cond).skip = 1 as libc::c_int;
}
unsafe extern "C" fn cfg_handle_endif(
    mut path: *const libc::c_char,
    mut line: size_t,
    mut conds: *mut cfg_conds,
) {
    let mut cond: *mut cfg_cond = (*conds).tqh_first;
    if cond.is_null() {
        cfg_add_cause(
            b"%s:%zu: unexpected %%endif\0" as *const u8 as *const libc::c_char,
            path,
            line,
        );
        return;
    }
    if !((*cond).entry.tqe_next).is_null() {
        (*(*cond).entry.tqe_next).entry.tqe_prev = (*cond).entry.tqe_prev;
    } else {
        (*conds).tqh_last = (*cond).entry.tqe_prev;
    }
    *(*cond).entry.tqe_prev = (*cond).entry.tqe_next;
    free(cond as *mut libc::c_void);
}
unsafe extern "C" fn cfg_handle_directive(
    mut p: *const libc::c_char,
    mut path: *const libc::c_char,
    mut line: size_t,
    mut conds: *mut cfg_conds,
) {
    let mut n: libc::c_int = 0 as libc::c_int;
    while *p.offset(n as isize) as libc::c_int != '\0' as i32
        && *(*__ctype_b_loc())
            .offset(*p.offset(n as isize) as u_char as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        n += 1;
        n;
    }
    if strncmp(p, b"%if\0" as *const u8 as *const libc::c_char, n as libc::c_ulong)
        == 0 as libc::c_int
    {
        cfg_handle_if(path, line, conds, p.offset(n as isize));
    } else if strncmp(
        p,
        b"%elif\0" as *const u8 as *const libc::c_char,
        n as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        cfg_handle_elif(path, line, conds, p.offset(n as isize));
    } else if strcmp(p, b"%else\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        cfg_handle_else(path, line, conds);
    } else if strcmp(p, b"%endif\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        cfg_handle_endif(path, line, conds);
    } else {
        cfg_add_cause(
            b"%s:%zu: invalid directive: %s\0" as *const u8 as *const libc::c_char,
            path,
            line,
            p,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_cfg(
    mut path: *const libc::c_char,
    mut c: *mut client,
    mut item: *mut cmdq_item,
    mut quiet: libc::c_int,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let delim: [libc::c_char; 3] = [
        '\\' as i32 as libc::c_char,
        '\\' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    let mut found: u_int = 0 as libc::c_int as u_int;
    let mut line: size_t = 0 as libc::c_int as size_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut cond: *mut cfg_cond = 0 as *mut cfg_cond;
    let mut cond1: *mut cfg_cond = 0 as *mut cfg_cond;
    let mut conds: cfg_conds = cfg_conds {
        tqh_first: 0 as *mut cfg_cond,
        tqh_last: 0 as *mut *mut cfg_cond,
    };
    conds.tqh_first = 0 as *mut cfg_cond;
    conds.tqh_last = &mut conds.tqh_first;
    log_debug(b"loading %s\0" as *const u8 as *const libc::c_char, path);
    f = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        if *__errno_location() == 2 as libc::c_int && quiet != 0 {
            return 0 as libc::c_int;
        }
        cfg_add_cause(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            path,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    loop {
        buf = fparseln(f, 0 as *mut size_t, &mut line, delim.as_ptr(), 0 as libc::c_int);
        if buf.is_null() {
            break;
        }
        log_debug(b"%s: %s\0" as *const u8 as *const libc::c_char, path, buf);
        p = buf;
        while *(*__ctype_b_loc()).offset(*p as u_char as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            free(buf as *mut libc::c_void);
        } else {
            q = p.offset(strlen(p) as isize).offset(-(1 as libc::c_int as isize));
            while q != p
                && *(*__ctype_b_loc()).offset(*q as u_char as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let fresh0 = q;
                q = q.offset(-1);
                *fresh0 = '\0' as i32 as libc::c_char;
            }
            if *p as libc::c_int == '%' as i32 {
                cfg_handle_directive(p, path, line, &mut conds);
            } else {
                cond = conds.tqh_first;
                if !cond.is_null() && (*cond).met == 0 {
                    continue;
                }
                cmdlist = cmd_string_parse(p, path, line as u_int, &mut cause1);
                if cmdlist.is_null() {
                    free(buf as *mut libc::c_void);
                    if cause1.is_null() {
                        continue;
                    }
                    cfg_add_cause(
                        b"%s:%zu: %s\0" as *const u8 as *const libc::c_char,
                        path,
                        line,
                        cause1,
                    );
                    free(cause1 as *mut libc::c_void);
                } else {
                    free(buf as *mut libc::c_void);
                    new_item = cmdq_get_command(
                        cmdlist,
                        0 as *mut cmd_find_state,
                        0 as *mut mouse_event,
                        0 as libc::c_int,
                    );
                    if !item.is_null() {
                        cmdq_insert_after(item, new_item);
                    } else {
                        cmdq_append(c, new_item);
                    }
                    cmd_list_free(cmdlist);
                    found = found.wrapping_add(1);
                    found;
                }
            }
        }
    }
    fclose(f);
    cond = *(*(conds.tqh_last as *mut cfg_conds)).tqh_last;
    while !cond.is_null()
        && {
            cond1 = *(*((*cond).entry.tqe_prev as *mut cfg_conds)).tqh_last;
            1 as libc::c_int != 0
        }
    {
        cfg_add_cause(
            b"%s:%zu: unterminated %%if\0" as *const u8 as *const libc::c_char,
            path,
            (*cond).line,
        );
        if !((*cond).entry.tqe_next).is_null() {
            (*(*cond).entry.tqe_next).entry.tqe_prev = (*cond).entry.tqe_prev;
        } else {
            conds.tqh_last = (*cond).entry.tqe_prev;
        }
        *(*cond).entry.tqe_prev = (*cond).entry.tqe_next;
        free(cond as *mut libc::c_void);
        cond = cond1;
    }
    return found as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_print_causes(mut item: *mut cmdq_item) {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < cfg_ncauses {
        cmdq_print(
            item,
            b"%s\0" as *const u8 as *const libc::c_char,
            *cfg_causes.offset(i as isize),
        );
        free(*cfg_causes.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    free(cfg_causes as *mut libc::c_void);
    cfg_causes = 0 as *mut *mut libc::c_char;
    cfg_ncauses = 0 as libc::c_int as u_int;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_show_causes(mut s: *mut session) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    if s.is_null() || cfg_ncauses == 0 as libc::c_int as u_int {
        return;
    }
    wp = (*(*(*s).curw).window).active;
    window_pane_set_mode(
        wp,
        &window_copy_mode,
        0 as *mut cmd_find_state,
        0 as *mut args,
    );
    window_copy_init_for_output(wp);
    i = 0 as libc::c_int as u_int;
    while i < cfg_ncauses {
        window_copy_add(
            wp,
            b"%s\0" as *const u8 as *const libc::c_char,
            *cfg_causes.offset(i as isize),
        );
        free(*cfg_causes.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    free(cfg_causes as *mut libc::c_void);
    cfg_causes = 0 as *mut *mut libc::c_char;
    cfg_ncauses = 0 as libc::c_int as u_int;
}
