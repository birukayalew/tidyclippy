#![feature(extern_types)]

use ::libc;
use libc::{alarm, usleep};
    pub type _IO_wide_data = libc::c_void;
    pub type _IO_codecvt = libc::c_void;
    pub type _IO_marker = libc::c_void;
    pub type _XGC = libc::c_void;
    pub type _XDisplay = libc::c_void;
    pub type _XPrivate = libc::c_void;
    pub type _XrmHashBucketRec = libc::c_void;
extern "C" {

    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn XCreateImage(
        _: *mut Display,
        _: *mut Visual,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut XImage;
    fn XGetSubImage(
        _: *mut Display,
        _: Drawable,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_ulong,
        _: libc::c_int,
        _: *mut XImage,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XCreateFontCursor(_: *mut Display, _: libc::c_uint) -> Cursor;
    fn XCreateGC(
        _: *mut Display,
        _: Drawable,
        _: libc::c_ulong,
        _: *mut XGCValues,
    ) -> GC;
    fn XCreateWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_uint,
        _: *mut Visual,
        _: libc::c_ulong,
        _: *mut XSetWindowAttributes,
    ) -> Window;
    fn XKeycodeToKeysym(_: *mut Display, _: KeyCode, _: libc::c_int) -> KeySym;
    fn XSetWMProtocols(
        _: *mut Display,
        _: Window,
        _: *mut Atom,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_uchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XDefineCursor(_: *mut Display, _: Window, _: Cursor) -> libc::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    fn XParseGeometry(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
    fn XPending(_: *mut Display) -> libc::c_int;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XAllocClassHint() -> *mut XClassHint;
    fn XAllocSizeHints() -> *mut XSizeHints;
    fn XSetWMProperties(
        _: *mut Display,
        _: Window,
        _: *mut XTextProperty,
        _: *mut XTextProperty,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut XSizeHints,
        _: *mut XWMHints,
        _: *mut XClassHint,
    );
    fn XStringListToTextProperty(
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut XTextProperty,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub xoffset: libc::c_int,
    pub format: libc::c_int,
    pub data: *mut libc::c_char,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub depth: libc::c_int,
    pub bytes_per_line: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option::<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
            libc::c_uint,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option::<unsafe extern "C" fn(*mut _XImage) -> libc::c_int>,
    pub get_pixel: Option::<
        unsafe extern "C" fn(*mut _XImage, libc::c_int, libc::c_int) -> libc::c_ulong,
    >,
    pub put_pixel: Option::<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub sub_image: Option::<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_uint,
            libc::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option::<
        unsafe extern "C" fn(*mut _XImage, libc::c_long) -> libc::c_int,
    >,
}
pub type XImage = _XImage;
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: C2RustUnnamed_1,
    pub max_aspect: C2RustUnnamed_1,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: libc::c_long,
    pub input: libc::c_int,
    pub initial_state: libc::c_int,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: libc::c_int,
    pub icon_y: libc::c_int,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut libc::c_uchar,
    pub encoding: Atom,
    pub format: libc::c_int,
    pub nitems: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut libc::c_char,
    pub res_class: *mut libc::c_char,
}
#[no_mangle]
pub static mut dpy: *mut Display = 0 as *const Display as *mut Display;
#[no_mangle]
pub static mut scr: *mut Screen = 0 as *const Screen as *mut Screen;
#[no_mangle]
pub static mut win: Window = 0;
#[no_mangle]
pub static mut wm_delete_window: Atom = 0;
#[no_mangle]
pub static mut wm_protocols: Atom = 0;
#[no_mangle]
pub static mut wm_dock: Atom = 0;
#[no_mangle]
pub static mut wm_windowtype: Atom = 0;
#[no_mangle]
pub static mut status: libc::c_int = 0;
#[no_mangle]
pub static mut gc: GC = 0 as *const _XGC as *mut _XGC;
#[no_mangle]
pub static mut when_button: Cursor = 0;
#[no_mangle]
pub static mut crosshair: Cursor = 0;
#[no_mangle]
pub static mut progname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut set_title: libc::c_int = 0;
#[no_mangle]
pub static mut xgrab: libc::c_int = 0;
#[no_mangle]
pub static mut ygrab: libc::c_int = 0;
#[no_mangle]
pub static mut magx: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut magy: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut flipxy: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut flipx: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut flipy: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut xzoom_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut yzoom_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut gridx: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut gridy: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut width: [libc::c_int; 2] = [0 as libc::c_int, 256 as libc::c_int];
#[no_mangle]
pub static mut height: [libc::c_int; 2] = [0 as libc::c_int, 256 as libc::c_int];
#[no_mangle]
pub static mut depth: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut ximage: [*mut XImage; 2] = [0 as *const XImage as *mut XImage; 2];
#[no_mangle]
pub static mut created_images: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut delays: [libc::c_int; 5] = [
    200000 as libc::c_int,
    100000 as libc::c_int,
    50000 as libc::c_int,
    10000 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub static mut delay_index: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut delay: libc::c_int = 200000 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn timeout_func(mut signum: libc::c_int) {
    set_title = 1 as libc::c_int;
    // signum = signum;
}
#[no_mangle]
pub unsafe extern "C" fn allocate_images() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
        data = malloc(
            ((*(dpy as _XPrivDisplay)).bitmap_unit / 8 as libc::c_int * width[i as usize]
                * height[i as usize]) as libc::c_ulong,
        ) as *mut libc::c_char;
        ximage[i
            as usize] = XCreateImage(
            dpy,
            (*scr).root_visual,
            (*scr).root_depth as libc::c_uint,
            2 as libc::c_int,
            0 as libc::c_int,
            data,
            width[i as usize] as libc::c_uint,
            height[i as usize] as libc::c_uint,
            32 as libc::c_int,
            0 as libc::c_int,
        );
        if (ximage[i as usize]).is_null() {
            perror(b"XCreateImage\0" as *const u8 as *const libc::c_char);
            exit(-(1 as libc::c_int));
        }
        i += 1;
        i;
    }
    created_images = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn destroy_images() {
    let mut i: libc::c_int = 0;
    if created_images == 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        free((*ximage[i as usize]).data as *mut libc::c_void);
        (*ximage[i as usize]).data = 0 as *mut libc::c_char;
        (Some(
            ((**ximage.as_mut_ptr().offset(i as isize)).f.destroy_image)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(ximage[i as usize]);
        i += 1;
        i;
    }
    created_images = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Usage() {
    fprintf(
        stderr,
        b"Usage: %s [ args ]\nCommand line args:\n-display displayname\n-mag magnification [ magnification ]\n-geometry geometry\n-source geometry\n-dock\n-x\n-y\n-xy\n\nWindow commands:\n+: Zoom in\n-: Zoom out\nx: Flip right and left\ny: Flip top and bottom\nz: Rotate 90 degrees counter-clockwize\nw: Next '+' or '-' only change width scaling\nh: Next '+' or '-' only change height scaling\nd: Change delay between frames\nq: Quit\nArrow keys: Scroll in direction of arrow\nMouse button drag: Set top-left corner of viewed area\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn resize(
    mut new_width: libc::c_int,
    mut new_height: libc::c_int,
) {
    destroy_images();
    if flipxy != 0 {
        height[0 as libc::c_int as usize] = (new_width + magx - 1 as libc::c_int) / magx;
        width[0 as libc::c_int as usize] = (new_height + magy - 1 as libc::c_int) / magy;
    } else {
        width[0 as libc::c_int as usize] = (new_width + magx - 1 as libc::c_int) / magx;
        height[0 as libc::c_int
            as usize] = (new_height + magy - 1 as libc::c_int) / magy;
    }
    if width[0 as libc::c_int as usize] < 1 as libc::c_int {
        width[0 as libc::c_int as usize] = 1 as libc::c_int;
    }
    if width[0 as libc::c_int as usize] > (*scr).width {
        width[0 as libc::c_int as usize] = (*scr).width;
    }
    if height[0 as libc::c_int as usize] < 1 as libc::c_int {
        height[0 as libc::c_int as usize] = 1 as libc::c_int;
    }
    if height[0 as libc::c_int as usize] > (*scr).height {
        height[0 as libc::c_int as usize] = (*scr).height;
    }
    if flipxy != 0 {
        width[1 as libc::c_int as usize] = magx * height[0 as libc::c_int as usize];
        height[1 as libc::c_int as usize] = magy * width[0 as libc::c_int as usize];
    } else {
        width[1 as libc::c_int as usize] = magx * width[0 as libc::c_int as usize];
        height[1 as libc::c_int as usize] = magy * height[0 as libc::c_int as usize];
    }
    allocate_images();
    if width[1 as libc::c_int as usize] > new_width {
        width[1 as libc::c_int as usize] = new_width;
    }
    if height[1 as libc::c_int as usize] > new_height {
        height[1 as libc::c_int as usize] = new_height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn scale8() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    j = if flipxy != 0 {
        width[0 as libc::c_int as usize] - 1 as libc::c_int
    } else {
        height[0 as libc::c_int as usize] - 1 as libc::c_int
    };
    loop {
        let mut p1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut p2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut p2step: libc::c_int = 0;
        let mut p1_save: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        p1 = &mut *((**ximage.as_mut_ptr().offset(1 as libc::c_int as isize)).data)
            .offset(
                (((**ximage.as_mut_ptr().offset(1 as libc::c_int as isize)).xoffset
                    + 0 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        (j * magy
                            * (**ximage.as_mut_ptr().offset(1 as libc::c_int as isize))
                                .bytes_per_line) as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_char as *mut libc::c_uchar;
        p1_save = p1;
        p2 = &mut *((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).data)
            .offset(
                (((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).xoffset
                    + 0 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        ((if flipy != 0 {
                            *height.as_mut_ptr().offset(0 as libc::c_int as isize)
                                - 1 as libc::c_int - j
                        } else {
                            j
                        })
                            * (**ximage.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .bytes_per_line) as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_char as *mut libc::c_uchar;
        if flipxy != 0 {
            p2 = &mut *((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).data)
                .offset(
                    (((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).xoffset
                        + (if flipy != 0 {
                            j
                        } else {
                            *width.as_mut_ptr().offset(0 as libc::c_int as isize)
                                - 1 as libc::c_int - j
                        })) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        )
                        .wrapping_add(
                            (0 as libc::c_int
                                * (**ximage.as_mut_ptr().offset(0 as libc::c_int as isize))
                                    .bytes_per_line) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_char as *mut libc::c_uchar;
            p2step = ((*ximage[0 as libc::c_int as usize]).bytes_per_line
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as libc::c_int;
            if flipx != 0 {
                p2 = p2
                    .offset(
                        (p2step * (height[0 as libc::c_int as usize] - 1 as libc::c_int))
                            as isize,
                    );
                p2step = -p2step;
            }
            i = height[0 as libc::c_int as usize];
            loop {
                let mut c: libc::c_uchar = *p2;
                p2 = p2.offset(p2step as isize);
                k = magx;
                loop {
                    let fresh0 = p1;
                    p1 = p1.offset(1);
                    *fresh0 = c;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        } else if flipx != 0 {
            p2 = p2.offset(width[0 as libc::c_int as usize] as isize);
            i = width[0 as libc::c_int as usize];
            loop {
                p2 = p2.offset(-1);
                let mut c_0: libc::c_uchar = *p2;
                k = magx;
                loop {
                    let fresh1 = p1;
                    p1 = p1.offset(1);
                    *fresh1 = c_0;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            i = width[0 as libc::c_int as usize];
            loop {
                let fresh2 = p2;
                p2 = p2.offset(1);
                let mut c_1: libc::c_uchar = *fresh2;
                k = magx;
                loop {
                    let fresh3 = p1;
                    p1 = p1.offset(1);
                    *fresh3 = c_1;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if gridy != 0 && magx >= 2 as libc::c_int {
            p1 = p1_save.offset(-(1 as libc::c_int as isize));
            i = magx;
            k = if flipxy != 0 {
                height[0 as libc::c_int as usize]
            } else {
                width[0 as libc::c_int as usize]
            };
            loop {
                p1 = p1.offset(i as isize);
                *p1 = (*p1 as libc::c_int
                    ^ !(0 as libc::c_int as libc::c_uchar as libc::c_int))
                    as libc::c_uchar;
                k -= 1;
                if !(k > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if magy > 1 as libc::c_int {
            p1 = p1_save;
            p2 = p1;
            p2step = ((*ximage[1 as libc::c_int as usize]).bytes_per_line
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as libc::c_int;
            i = (width[1 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as libc::c_int;
            k = magy - 1 as libc::c_int;
            loop {
                p2 = p2.offset(p2step as isize);
                memcpy(
                    p2 as *mut libc::c_void,
                    p1 as *const libc::c_void,
                    i as libc::c_ulong,
                );
                k -= 1;
                if !(k > 0 as libc::c_int) {
                    break;
                }
            }
            if gridx != 0 && magy >= 2 as libc::c_int {
                k = width[1 as libc::c_int as usize];
                loop {
                    let fresh4 = p2;
                    p2 = p2.offset(1);
                    *fresh4 = (*fresh4 as libc::c_int
                        ^ !(0 as libc::c_int as libc::c_uchar as libc::c_int))
                        as libc::c_uchar;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
            }
        }
        j -= 1;
        if !(j >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn scale16() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    j = if flipxy != 0 {
        width[0 as libc::c_int as usize] - 1 as libc::c_int
    } else {
        height[0 as libc::c_int as usize] - 1 as libc::c_int
    };
    loop {
        let mut p1: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
        let mut p2: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
        let mut p2step: libc::c_int = 0;
        let mut p1_save: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
        p1 = &mut *((**ximage.as_mut_ptr().offset(1 as libc::c_int as isize)).data)
            .offset(
                (((**ximage.as_mut_ptr().offset(1 as libc::c_int as isize)).xoffset
                    + 0 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        (j * magy
                            * (**ximage.as_mut_ptr().offset(1 as libc::c_int as isize))
                                .bytes_per_line) as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_char as *mut libc::c_ushort;
        p1_save = p1;
        p2 = &mut *((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).data)
            .offset(
                (((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).xoffset
                    + 0 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        ((if flipy != 0 {
                            *height.as_mut_ptr().offset(0 as libc::c_int as isize)
                                - 1 as libc::c_int - j
                        } else {
                            j
                        })
                            * (**ximage.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .bytes_per_line) as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_char as *mut libc::c_ushort;
        if flipxy != 0 {
            p2 = &mut *((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).data)
                .offset(
                    (((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).xoffset
                        + (if flipy != 0 {
                            j
                        } else {
                            *width.as_mut_ptr().offset(0 as libc::c_int as isize)
                                - 1 as libc::c_int - j
                        })) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        )
                        .wrapping_add(
                            (0 as libc::c_int
                                * (**ximage.as_mut_ptr().offset(0 as libc::c_int as isize))
                                    .bytes_per_line) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_char as *mut libc::c_ushort;
            p2step = ((*ximage[0 as libc::c_int as usize]).bytes_per_line
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                as libc::c_int;
            if flipx != 0 {
                p2 = p2
                    .offset(
                        (p2step * (height[0 as libc::c_int as usize] - 1 as libc::c_int))
                            as isize,
                    );
                p2step = -p2step;
            }
            i = height[0 as libc::c_int as usize];
            loop {
                let mut c: libc::c_ushort = *p2;
                p2 = p2.offset(p2step as isize);
                k = magx;
                loop {
                    let fresh5 = p1;
                    p1 = p1.offset(1);
                    *fresh5 = c;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        } else if flipx != 0 {
            p2 = p2.offset(width[0 as libc::c_int as usize] as isize);
            i = width[0 as libc::c_int as usize];
            loop {
                p2 = p2.offset(-1);
                let mut c_0: libc::c_ushort = *p2;
                k = magx;
                loop {
                    let fresh6 = p1;
                    p1 = p1.offset(1);
                    *fresh6 = c_0;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            i = width[0 as libc::c_int as usize];
            loop {
                let fresh7 = p2;
                p2 = p2.offset(1);
                let mut c_1: libc::c_ushort = *fresh7;
                k = magx;
                loop {
                    let fresh8 = p1;
                    p1 = p1.offset(1);
                    *fresh8 = c_1;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if gridy != 0 && magx >= 2 as libc::c_int {
            p1 = p1_save.offset(-(1 as libc::c_int as isize));
            i = magx;
            k = if flipxy != 0 {
                height[0 as libc::c_int as usize]
            } else {
                width[0 as libc::c_int as usize]
            };
            loop {
                p1 = p1.offset(i as isize);
                *p1 = (*p1 as libc::c_int
                    ^ !(0 as libc::c_int as libc::c_ushort as libc::c_int))
                    as libc::c_ushort;
                k -= 1;
                if !(k > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if magy > 1 as libc::c_int {
            p1 = p1_save;
            p2 = p1;
            p2step = ((*ximage[1 as libc::c_int as usize]).bytes_per_line
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                as libc::c_int;
            i = (width[1 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
                as libc::c_int;
            k = magy - 1 as libc::c_int;
            loop {
                p2 = p2.offset(p2step as isize);
                memcpy(
                    p2 as *mut libc::c_void,
                    p1 as *const libc::c_void,
                    i as libc::c_ulong,
                );
                k -= 1;
                if !(k > 0 as libc::c_int) {
                    break;
                }
            }
            if gridx != 0 && magy >= 2 as libc::c_int {
                k = width[1 as libc::c_int as usize];
                loop {
                    let fresh9 = p2;
                    p2 = p2.offset(1);
                    *fresh9 = (*fresh9 as libc::c_int
                        ^ !(0 as libc::c_int as libc::c_ushort as libc::c_int))
                        as libc::c_ushort;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
            }
        }
        j -= 1;
        if !(j >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn scale32() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    j = if flipxy != 0 {
        width[0 as libc::c_int as usize] - 1 as libc::c_int
    } else {
        height[0 as libc::c_int as usize] - 1 as libc::c_int
    };
    loop {
        let mut p1: *mut libc::c_uint = 0 as *mut libc::c_uint;
        let mut p2: *mut libc::c_uint = 0 as *mut libc::c_uint;
        let mut p2step: libc::c_int = 0;
        let mut p1_save: *mut libc::c_uint = 0 as *mut libc::c_uint;
        p1 = &mut *((**ximage.as_mut_ptr().offset(1 as libc::c_int as isize)).data)
            .offset(
                (((**ximage.as_mut_ptr().offset(1 as libc::c_int as isize)).xoffset
                    + 0 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        (j * magy
                            * (**ximage.as_mut_ptr().offset(1 as libc::c_int as isize))
                                .bytes_per_line) as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_char as *mut libc::c_uint;
        p1_save = p1;
        p2 = &mut *((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).data)
            .offset(
                (((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).xoffset
                    + 0 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        ((if flipy != 0 {
                            *height.as_mut_ptr().offset(0 as libc::c_int as isize)
                                - 1 as libc::c_int - j
                        } else {
                            j
                        })
                            * (**ximage.as_mut_ptr().offset(0 as libc::c_int as isize))
                                .bytes_per_line) as libc::c_ulong,
                    ) as isize,
            ) as *mut libc::c_char as *mut libc::c_uint;
        if flipxy != 0 {
            p2 = &mut *((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).data)
                .offset(
                    (((**ximage.as_mut_ptr().offset(0 as libc::c_int as isize)).xoffset
                        + (if flipy != 0 {
                            j
                        } else {
                            *width.as_mut_ptr().offset(0 as libc::c_int as isize)
                                - 1 as libc::c_int - j
                        })) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                        )
                        .wrapping_add(
                            (0 as libc::c_int
                                * (**ximage.as_mut_ptr().offset(0 as libc::c_int as isize))
                                    .bytes_per_line) as libc::c_ulong,
                        ) as isize,
                ) as *mut libc::c_char as *mut libc::c_uint;
            p2step = ((*ximage[0 as libc::c_int as usize]).bytes_per_line
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                as libc::c_int;
            if flipx != 0 {
                p2 = p2
                    .offset(
                        (p2step * (height[0 as libc::c_int as usize] - 1 as libc::c_int))
                            as isize,
                    );
                p2step = -p2step;
            }
            i = height[0 as libc::c_int as usize];
            loop {
                let mut c: libc::c_uint = *p2;
                p2 = p2.offset(p2step as isize);
                k = magx;
                loop {
                    let fresh10 = p1;
                    p1 = p1.offset(1);
                    *fresh10 = c;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        } else if flipx != 0 {
            p2 = p2.offset(width[0 as libc::c_int as usize] as isize);
            i = width[0 as libc::c_int as usize];
            loop {
                p2 = p2.offset(-1);
                let mut c_0: libc::c_uint = *p2;
                k = magx;
                loop {
                    let fresh11 = p1;
                    p1 = p1.offset(1);
                    *fresh11 = c_0;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            i = width[0 as libc::c_int as usize];
            loop {
                let fresh12 = p2;
                p2 = p2.offset(1);
                let mut c_1: libc::c_uint = *fresh12;
                k = magx;
                loop {
                    let fresh13 = p1;
                    p1 = p1.offset(1);
                    *fresh13 = c_1;
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if gridy != 0 && magx >= 2 as libc::c_int {
            p1 = p1_save.offset(-(1 as libc::c_int as isize));
            i = magx;
            k = if flipxy != 0 {
                height[0 as libc::c_int as usize]
            } else {
                width[0 as libc::c_int as usize]
            };
            loop {
                p1 = p1.offset(i as isize);
                *p1 ^= !(0 as libc::c_int as libc::c_uint);
                k -= 1;
                if !(k > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if magy > 1 as libc::c_int {
            p1 = p1_save;
            p2 = p1;
            p2step = ((*ximage[1 as libc::c_int as usize]).bytes_per_line
                as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                as libc::c_int;
            i = (width[1 as libc::c_int as usize] as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                as libc::c_int;
            k = magy - 1 as libc::c_int;
            loop {
                p2 = p2.offset(p2step as isize);
                memcpy(
                    p2 as *mut libc::c_void,
                    p1 as *const libc::c_void,
                    i as libc::c_ulong,
                );
                k -= 1;
                if !(k > 0 as libc::c_int) {
                    break;
                }
            }
            if gridx != 0 && magy >= 2 as libc::c_int {
                k = width[1 as libc::c_int as usize];
                loop {
                    let fresh14 = p2;
                    p2 = p2.offset(1);
                    *fresh14 ^= !(0 as libc::c_int as libc::c_uint);
                    k -= 1;
                    if !(k > 0 as libc::c_int) {
                        break;
                    }
                }
            }
        }
        j -= 1;
        if !(j >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xzoom(mut buttonpressed: libc::c_int) {
    let mut title: [libc::c_char; 80] = [0; 80];
    XGetSubImage(
        dpy,
        (*scr).root,
        xgrab,
        ygrab,
        width[0 as libc::c_int as usize] as libc::c_uint,
        height[0 as libc::c_int as usize] as libc::c_uint,
        !(0 as libc::c_long) as libc::c_ulong,
        2 as libc::c_int,
        ximage[0 as libc::c_int as usize],
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if depth == 8 as libc::c_int as libc::c_uint {
        scale8();
    } else if depth as libc::c_ulong
        <= (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
    {
        scale16();
    } else if depth as libc::c_ulong
        <= (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        scale32();
    }
    XPutImage(
        dpy,
        win,
        gc,
        ximage[1 as libc::c_int as usize],
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        width[1 as libc::c_int as usize] as libc::c_uint,
        height[1 as libc::c_int as usize] as libc::c_uint,
    );
    if set_title != 0 {
        if magx == magy && flipx == 0 && flipy == 0 && flipxy == 0 {
            sprintf(
                title.as_mut_ptr(),
                b"%s x%d\0" as *const u8 as *const libc::c_char,
                progname,
                magx,
            );
        } else {
            sprintf(
                title.as_mut_ptr(),
                b"%s X %s%d%s Y %s%d\0" as *const u8 as *const libc::c_char,
                progname,
                if flipx != 0 {
                    b"-\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                magx,
                if flipxy != 0 {
                    b" <=>\0" as *const u8 as *const libc::c_char
                } else {
                    b";\0" as *const u8 as *const libc::c_char
                },
                if flipy != 0 {
                    b"-\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                magy,
            );
        }
        XChangeProperty(
            dpy,
            win,
            39 as libc::c_int as Atom,
            31 as libc::c_int as Atom,
            8 as libc::c_int,
            0 as libc::c_int,
            title.as_mut_ptr() as *mut libc::c_uchar,
            strlen(title.as_mut_ptr()) as libc::c_int,
        );
        set_title = 0 as libc::c_int;
    }
    XSync(dpy, 0 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut xswa: XSetWindowAttributes = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    let mut event: XEvent = _XEvent { type_0: 0 };
    let mut str: XTextProperty = XTextProperty {
        value: 0 as *mut libc::c_uchar,
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    let mut sizeh: *mut XSizeHints = 0 as *mut XSizeHints;
    let mut ch: *mut XClassHint = 0 as *mut XClassHint;
    let mut buttonpressed: libc::c_int = 0 as libc::c_int;
    let mut visible: libc::c_int = 0 as libc::c_int;
    let mut scroll: libc::c_int = 1 as libc::c_int;
    let mut title: [libc::c_char; 80] = [0; 80];
    let mut gcv: XGCValues = XGCValues {
        function: 0,
        plane_mask: 0,
        foreground: 0,
        background: 0,
        line_width: 0,
        line_style: 0,
        cap_style: 0,
        join_style: 0,
        fill_style: 0,
        fill_rule: 0,
        arc_mode: 0,
        tile: 0,
        stipple: 0,
        ts_x_origin: 0,
        ts_y_origin: 0,
        font: 0,
        subwindow_mode: 0,
        graphics_exposures: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        dash_offset: 0,
        dashes: 0,
    };
    let mut dpyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut source_geom_mask: libc::c_int = 0 as libc::c_int;
    let mut dest_geom_mask: libc::c_int = 0 as libc::c_int;
    let mut copy_from_src_mask: libc::c_int = 0;
    let mut xpos: libc::c_int = 0 as libc::c_int;
    let mut ypos: libc::c_int = 0 as libc::c_int;
    let mut isdock: libc::c_int = 0 as libc::c_int;
    atexit(Some(destroy_images as unsafe extern "C" fn() -> ()));
    progname = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if !progname.is_null() {
        progname = progname.offset(1);
        progname;
    } else {
        progname = *argv.offset(0 as libc::c_int as isize);
    }
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        argv;
        if *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '=' as i32
        {
            dest_geom_mask = XParseGeometry(
                *argv.offset(0 as libc::c_int as isize),
                &mut xpos,
                &mut ypos,
                &mut *width.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_uint,
                &mut *height.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_uint,
            );
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-mag\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            magx = if argc > 0 as libc::c_int {
                atoi(*argv.offset(0 as libc::c_int as isize))
            } else {
                -(1 as libc::c_int)
            };
            if magx <= 0 as libc::c_int {
                Usage();
            }
            magy = if argc > 1 as libc::c_int {
                atoi(*argv.offset(1 as libc::c_int as isize))
            } else {
                -(1 as libc::c_int)
            };
            if magy <= 0 as libc::c_int {
                magy = magx;
            } else {
                argv = argv.offset(1);
                argv;
                argc -= 1;
                argc;
            }
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-x\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flipx = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-y\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flipy = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-z\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"-xy\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            flipxy = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-source\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            if argc < 1 as libc::c_int {
                Usage();
            }
            source_geom_mask = XParseGeometry(
                *argv.offset(0 as libc::c_int as isize),
                &mut xgrab,
                &mut ygrab,
                &mut *width.as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_uint,
                &mut *height.as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_uint,
            );
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-dest\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"-geometry\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            if argc < 1 as libc::c_int {
                Usage();
            }
            dest_geom_mask = XParseGeometry(
                *argv.offset(0 as libc::c_int as isize),
                &mut xpos,
                &mut ypos,
                &mut *width.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_uint,
                &mut *height.as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_uint,
            );
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"-display\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            if argc < 1 as libc::c_int {
                Usage();
            }
            dpyname = *argv.offset(0 as libc::c_int as isize);
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-delay\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            if argc < 1 as libc::c_int {
                Usage();
            }
            if sscanf(
                *argv.offset(0 as libc::c_int as isize),
                b"%u\0" as *const u8 as *const libc::c_char,
                &mut delay as *mut libc::c_int,
            ) != 1 as libc::c_int
            {
                Usage();
            }
            delay *= 1000 as libc::c_int;
        } else if strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"-dock\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            isdock = 1 as libc::c_int;
        } else {
            Usage();
        }
    }
    dpy = XOpenDisplay(dpyname);
    if dpy.is_null() {
        perror(b"Cannot open display\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    copy_from_src_mask = 0 as libc::c_int;
    if source_geom_mask & 0x4 as libc::c_int != 0 {
        if flipxy != 0 {
            height[1 as libc::c_int as usize] = magy * width[0 as libc::c_int as usize];
            copy_from_src_mask |= 0x8 as libc::c_int;
        } else {
            width[1 as libc::c_int as usize] = magx * width[0 as libc::c_int as usize];
            copy_from_src_mask |= 0x4 as libc::c_int;
        }
    }
    if source_geom_mask & 0x8 as libc::c_int != 0 {
        if flipxy != 0 {
            width[1 as libc::c_int as usize] = magx * height[0 as libc::c_int as usize];
            copy_from_src_mask |= 0x4 as libc::c_int;
        } else {
            height[1 as libc::c_int as usize] = magy * height[0 as libc::c_int as usize];
            copy_from_src_mask |= 0x8 as libc::c_int;
        }
    }
    if copy_from_src_mask & dest_geom_mask != 0 {
        fprintf(
            stderr,
            b"Conflicting dimensions between source and dest geometry\n\0" as *const u8
                as *const libc::c_char,
        );
        Usage();
    }
    scr = &mut *((*(dpy as _XPrivDisplay)).screens)
        .offset((*(dpy as _XPrivDisplay)).default_screen as isize) as *mut Screen;
    depth = (*scr).root_depth as libc::c_uint;
    if depth < 8 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"%s: need at least 8 bits/pixel\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        exit(1 as libc::c_int);
    }
    if source_geom_mask & 0x10 as libc::c_int != 0 {
        xgrab += (*scr).width;
    }
    if source_geom_mask & 0x20 as libc::c_int != 0 {
        ygrab += (*scr).height;
    }
    if dest_geom_mask & 0x10 as libc::c_int != 0 {
        xpos += (*scr).width;
    }
    if dest_geom_mask & 0x20 as libc::c_int != 0 {
        ypos += (*scr).height;
    }
    printf(
        b"=%dx%d+%d+%d\n\0" as *const u8 as *const libc::c_char,
        width[1 as libc::c_int as usize],
        height[1 as libc::c_int as usize],
        xpos,
        ypos,
    );
    xswa
        .event_mask = (1 as libc::c_long) << 2 as libc::c_int
        | (1 as libc::c_long) << 3 as libc::c_int
        | (1 as libc::c_long) << 13 as libc::c_int;
    xswa.event_mask |= (1 as libc::c_long) << 17 as libc::c_int;
    xswa.event_mask
        |= (1 as libc::c_long) << 0 as libc::c_int
            | (1 as libc::c_long) << 1 as libc::c_int;
    xswa.event_mask |= (1 as libc::c_long) << 16 as libc::c_int;
    xswa.background_pixel = (*scr).black_pixel;
    win = XCreateWindow(
        dpy,
        (*scr).root,
        xpos,
        ypos,
        width[1 as libc::c_int as usize] as libc::c_uint,
        height[1 as libc::c_int as usize] as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        (*scr).root_depth,
        1 as libc::c_int as libc::c_uint,
        (*scr).root_visual,
        ((1 as libc::c_long) << 11 as libc::c_int
            | (1 as libc::c_long) << 1 as libc::c_int) as libc::c_ulong,
        &mut xswa,
    );
    XChangeProperty(
        dpy,
        win,
        37 as libc::c_int as Atom,
        31 as libc::c_int as Atom,
        8 as libc::c_int,
        0 as libc::c_int,
        progname as *mut libc::c_uchar,
        strlen(progname) as libc::c_int,
    );
    if isdock == 0 {
        sizeh = XAllocSizeHints();
        (*sizeh)
            .flags = (1 as libc::c_long) << 5 as libc::c_int
            | (1 as libc::c_long) << 4 as libc::c_int;
        (*sizeh).max_width = width[1 as libc::c_int as usize];
        (*sizeh).min_width = (*sizeh).max_width;
        (*sizeh).max_height = height[1 as libc::c_int as usize];
        (*sizeh).min_height = (*sizeh).max_height;
    }
    XStringListToTextProperty(&mut progname, 1 as libc::c_int, &mut str);
    ch = XAllocClassHint();
    (*ch).res_class = progname;
    (*ch).res_name = progname;
    XSetWMProperties(
        dpy,
        win,
        &mut str,
        &mut str,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
        sizeh,
        0 as *mut XWMHints,
        ch,
    );
    if isdock != 0 {
        wm_windowtype = XInternAtom(
            dpy,
            b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        wm_dock = XInternAtom(
            dpy,
            b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        XChangeProperty(
            dpy,
            win,
            wm_windowtype,
            4 as libc::c_int as Atom,
            32 as libc::c_int,
            0 as libc::c_int,
            &mut wm_dock as *mut Atom as *mut libc::c_uchar,
            1 as libc::c_int,
        );
    }
    wm_delete_window = XInternAtom(
        dpy,
        b"WM_DELETE_WINDOW\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    wm_protocols = XInternAtom(
        dpy,
        b"WM_PROTOCOLS\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    status = XSetWMProtocols(dpy, win, &mut wm_delete_window, 1 as libc::c_int);
    set_title = 1 as libc::c_int;
    status = XMapWindow(dpy, win);
    gcv.plane_mask = !(0 as libc::c_long) as libc::c_ulong;
    gcv.subwindow_mode = 1 as libc::c_int;
    gcv.function = 0x3 as libc::c_int;
    gcv.foreground = (*scr).white_pixel;
    gcv.background = (*scr).black_pixel;
    gc = XCreateGC(
        dpy,
        (*scr).root,
        ((1 as libc::c_long) << 0 as libc::c_int
            | (1 as libc::c_long) << 1 as libc::c_int
            | (1 as libc::c_long) << 15 as libc::c_int
            | (1 as libc::c_long) << 2 as libc::c_int
            | (1 as libc::c_long) << 3 as libc::c_int) as libc::c_ulong,
        &mut gcv,
    );
    resize(width[1 as libc::c_int as usize], height[1 as libc::c_int as usize]);
    when_button = XCreateFontCursor(dpy, 144 as libc::c_int as libc::c_uint);
    crosshair = XCreateFontCursor(dpy, 34 as libc::c_int as libc::c_uint);
    XDefineCursor(dpy, win, crosshair);
    loop {
        while XPending(dpy) != 0 {
            XNextEvent(dpy, &mut event);
            match event.type_0 {
                33 => {
                    if event.xclient.message_type == wm_protocols
                        && event.xclient.data.l[0 as libc::c_int as usize] as Atom
                            == wm_delete_window
                    {
                        exit(0 as libc::c_int);
                    }
                }
                22 => {
                    if event.xconfigure.width != width[1 as libc::c_int as usize]
                        || event.xconfigure.height != height[1 as libc::c_int as usize]
                    {
                        resize(event.xconfigure.width, event.xconfigure.height);
                    }
                }
                15 => {
                    visible = (event.xvisibility.state != 2 as libc::c_int)
                        as libc::c_int;
                }
                3 => {
                    match XKeycodeToKeysym(
                        dpy,
                        event.xkey.keycode as KeyCode,
                        0 as libc::c_int,
                    ) {
                        65507 | 65508 => {
                            scroll = 1 as libc::c_int;
                        }
                        _ => {}
                    }
                }
                2 => {
                    match XKeycodeToKeysym(
                        dpy,
                        event.xkey.keycode as KeyCode,
                        0 as libc::c_int,
                    ) {
                        65507 | 65508 => {
                            scroll = 10 as libc::c_int;
                        }
                        43 | 61 | 65451 => {
                            if yzoom_flag == 0 {
                                magx += 1;
                                magx;
                            }
                            if xzoom_flag == 0 {
                                magy += 1;
                                magy;
                            }
                            yzoom_flag = 0 as libc::c_int;
                            xzoom_flag = yzoom_flag;
                            resize(
                                width[1 as libc::c_int as usize],
                                height[1 as libc::c_int as usize],
                            );
                            set_title = 1 as libc::c_int;
                        }
                        45 | 65453 => {
                            if yzoom_flag == 0 {
                                magx -= 1;
                                magx;
                            }
                            if xzoom_flag == 0 {
                                magy -= 1;
                                magy;
                            }
                            yzoom_flag = 0 as libc::c_int;
                            xzoom_flag = yzoom_flag;
                            if magx < 1 as libc::c_int {
                                magx = 1 as libc::c_int;
                            }
                            if magy < 1 as libc::c_int {
                                magy = 1 as libc::c_int;
                            }
                            resize(
                                width[1 as libc::c_int as usize],
                                height[1 as libc::c_int as usize],
                            );
                            set_title = 1 as libc::c_int;
                        }
                        65361 | 65430 => {
                            if flipxy != 0 {
                                if flipx != 0 {
                                    ygrab += scroll;
                                } else {
                                    ygrab -= scroll;
                                }
                            } else if flipx != 0 {
                                xgrab += scroll;
                            } else {
                                xgrab -= scroll;
                            }
                        }
                        65363 | 65432 => {
                            if flipxy != 0 {
                                if flipx != 0 {
                                    ygrab -= scroll;
                                } else {
                                    ygrab += scroll;
                                }
                            } else if flipx != 0 {
                                xgrab -= scroll;
                            } else {
                                xgrab += scroll;
                            }
                        }
                        65362 | 65431 => {
                            if flipxy != 0 {
                                if flipy != 0 {
                                    xgrab -= scroll;
                                } else {
                                    xgrab += scroll;
                                }
                            } else if flipy != 0 {
                                ygrab += scroll;
                            } else {
                                ygrab -= scroll;
                            }
                        }
                        65364 | 65433 => {
                            if flipxy != 0 {
                                if flipy != 0 {
                                    xgrab += scroll;
                                } else {
                                    xgrab -= scroll;
                                }
                            } else if flipy != 0 {
                                ygrab -= scroll;
                            } else {
                                ygrab += scroll;
                            }
                        }
                        120 => {
                            flipx = (flipx == 0) as libc::c_int;
                            set_title = 1 as libc::c_int;
                        }
                        121 => {
                            flipy = (flipy == 0) as libc::c_int;
                            set_title = 1 as libc::c_int;
                        }
                        122 => {
                            if flipx ^ flipy ^ flipxy != 0 {
                                flipx = (flipx == 0) as libc::c_int;
                                flipy = (flipy == 0) as libc::c_int;
                            }
                            flipxy = (flipxy == 0) as libc::c_int;
                            resize(
                                width[1 as libc::c_int as usize],
                                height[1 as libc::c_int as usize],
                            );
                            set_title = 1 as libc::c_int;
                        }
                        119 => {
                            xzoom_flag = 1 as libc::c_int;
                            yzoom_flag = 0 as libc::c_int;
                        }
                        104 => {
                            yzoom_flag = 1 as libc::c_int;
                            xzoom_flag = 0 as libc::c_int;
                        }
                        103 => {
                            gridx = (gridx == 0) as libc::c_int;
                            gridy = (gridy == 0) as libc::c_int;
                        }
                        100 => {
                            delay_index += 1;
                            if delay_index >= 5 as libc::c_int {
                                delay_index = 0 as libc::c_int;
                            }
                            delay = delays[delay_index as usize];
                            sprintf(
                                title.as_mut_ptr(),
                                b"delay = %d ms\0" as *const u8 as *const libc::c_char,
                                delay / 1000 as libc::c_int,
                            );
                            XChangeProperty(
                                dpy,
                                win,
                                39 as libc::c_int as Atom,
                                31 as libc::c_int as Atom,
                                8 as libc::c_int,
                                0 as libc::c_int,
                                title.as_mut_ptr() as *mut libc::c_uchar,
                                strlen(title.as_mut_ptr()) as libc::c_int,
                            );
                            signal(
                                14 as libc::c_int,
                                Some(
                                    timeout_func as unsafe extern "C" fn(libc::c_int) -> (),
                                ),
                            );
                            // Convert to u32
                            alarm((2 as libc::c_int).try_into().unwrap());
                        }
                        113 => {
                            exit(0 as libc::c_int);
                        }
                        _ => {}
                    }
                }
                4 => {
                    xgrab = event.xbutton.x_root;
                    ygrab = event.xbutton.y_root;
                    XDefineCursor(dpy, win, when_button);
                    buttonpressed = 1 as libc::c_int;
                }
                5 => {
                    XDefineCursor(dpy, win, crosshair);
                    buttonpressed = 0 as libc::c_int;
                }
                6 => {
                    if buttonpressed != 0 {
                        xgrab = event.xmotion.x_root;
                        ygrab = event.xmotion.y_root;
                    }
                }
                21 | _ => {}
            }
            if xgrab < 0 as libc::c_int {
                xgrab = 0 as libc::c_int;
            }
            if xgrab > (*scr).width - width[0 as libc::c_int as usize] {
                xgrab = (*scr).width - width[0 as libc::c_int as usize];
            }
            if ygrab < 0 as libc::c_int {
                ygrab = 0 as libc::c_int;
            }
            if ygrab > (*scr).height - height[0 as libc::c_int as usize] {
                ygrab = (*scr).height - height[0 as libc::c_int as usize];
            }
        }
        if visible != 0 {
            xzoom(buttonpressed);
        }
        if buttonpressed == 0 && delay > 0 as libc::c_int {
            // Similarly for usleep
            usleep(delay.try_into().unwrap());
        }
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
