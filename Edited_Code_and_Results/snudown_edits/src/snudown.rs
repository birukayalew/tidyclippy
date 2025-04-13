#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type sd_markdown;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn PyErr_SetString(exception: *mut PyObject, string: *const libc::c_char);
    static mut PyExc_ValueError: *mut PyObject;
    fn _PyArg_ParseTupleAndKeywords_SizeT(
        _: *mut PyObject,
        _: *mut PyObject,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn _Py_BuildValue_SizeT(_: *const libc::c_char, _: ...) -> *mut PyObject;
    fn PyModule_AddIntConstant(
        _: *mut PyObject,
        _: *const libc::c_char,
        _: libc::c_long,
    ) -> libc::c_int;
    fn PyModule_AddStringConstant(
        _: *mut PyObject,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn PyModule_Create2(_: *mut PyModuleDef, apiver: libc::c_int) -> *mut PyObject;
    fn bufnew(_: size_t) -> *mut buf;
    fn bufput(_: *mut buf, _: *const libc::c_void, _: size_t);
    fn bufputs(_: *mut buf, _: *const libc::c_char);
    fn bufputc(_: *mut buf, _: libc::c_int);
    fn bufrelease(_: *mut buf);
    fn sd_markdown_new(
        extensions: libc::c_uint,
        max_nesting: size_t,
        max_table_cols: size_t,
        callbacks: *const sd_callbacks,
        opaque: *mut libc::c_void,
    ) -> *mut sd_markdown;
    fn sd_markdown_render(
        ob: *mut buf,
        document: *const uint8_t,
        doc_size: size_t,
        md: *mut sd_markdown,
    );
    fn sdhtml_renderer(
        callbacks: *mut sd_callbacks,
        options_ptr: *mut html_renderopt,
        render_flags: libc::c_uint,
    );
    fn sdhtml_toc_renderer(
        callbacks: *mut sd_callbacks,
        options_ptr: *mut html_renderopt,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type Py_ssize_t = ssize_t;
pub type Py_hash_t = Py_ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyModuleDef {
    pub m_base: PyModuleDef_Base,
    pub m_name: *const libc::c_char,
    pub m_doc: *const libc::c_char,
    pub m_size: Py_ssize_t,
    pub m_methods: *mut PyMethodDef,
    pub m_slots: *mut PyModuleDef_Slot,
    pub m_traverse: traverseproc,
    pub m_clear: inquiry,
    pub m_free: freefunc,
}
pub type freefunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type inquiry = Option::<unsafe extern "C" fn(*mut PyObject) -> libc::c_int>;
pub type PyObject = _object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object {
    pub c2rust_unnamed: C2RustUnnamed,
    pub ob_type: *mut PyTypeObject,
}
pub type PyTypeObject = _typeobject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _typeobject {
    pub ob_base: PyVarObject,
    pub tp_name: *const libc::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_vectorcall_offset: Py_ssize_t,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_as_async: *mut PyAsyncMethods,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: libc::c_ulong,
    pub tp_doc: *const libc::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut PyTypeObject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut libc::c_void,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: libc::c_uint,
    pub tp_finalize: destructor,
    pub tp_vectorcall: vectorcallfunc,
    pub tp_watched: libc::c_uchar,
}
pub type vectorcallfunc = Option::<
    unsafe extern "C" fn(
        *mut PyObject,
        *const *mut PyObject,
        size_t,
        *mut PyObject,
    ) -> *mut PyObject,
>;
pub type destructor = Option::<unsafe extern "C" fn(*mut PyObject) -> ()>;
pub type newfunc = Option::<
    unsafe extern "C" fn(
        *mut PyTypeObject,
        *mut PyObject,
        *mut PyObject,
    ) -> *mut PyObject,
>;
pub type allocfunc = Option::<
    unsafe extern "C" fn(*mut PyTypeObject, Py_ssize_t) -> *mut PyObject,
>;
pub type initproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type descrsetfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type descrgetfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *const libc::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *const libc::c_char,
    pub closure: *mut libc::c_void,
}
pub type setter = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut libc::c_void) -> libc::c_int,
>;
pub type getter = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_void) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMemberDef {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub offset: Py_ssize_t,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}
pub type PyCFunction = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type iternextfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type getiterfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type richcmpfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, libc::c_int) -> *mut PyObject,
>;
pub type traverseproc = Option::<
    unsafe extern "C" fn(*mut PyObject, visitproc, *mut libc::c_void) -> libc::c_int,
>;
pub type visitproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBufferProcs {
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type releasebufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut Py_buffer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Py_buffer {
    pub buf: *mut libc::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: libc::c_int,
    pub ndim: libc::c_int,
    pub format: *mut libc::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub internal: *mut libc::c_void,
}
pub type getbufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut Py_buffer, libc::c_int) -> libc::c_int,
>;
pub type setattrofunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type getattrofunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type reprfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type ternaryfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type hashfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> Py_hash_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type binaryfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type lenfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> Py_ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub was_sq_slice: *mut libc::c_void,
    pub sq_ass_item: ssizeobjargproc,
    pub was_sq_ass_slice: *mut libc::c_void,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
pub type ssizeargfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t) -> *mut PyObject,
>;
pub type objobjproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type ssizeobjargproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut PyObject) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_bool: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_int: unaryfunc,
    pub nb_reserved: *mut libc::c_void,
    pub nb_float: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
    pub nb_matrix_multiply: binaryfunc,
    pub nb_inplace_matrix_multiply: binaryfunc,
}
pub type unaryfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyAsyncMethods {
    pub am_await: unaryfunc,
    pub am_aiter: unaryfunc,
    pub am_anext: unaryfunc,
    pub am_send: sendfunc,
}
pub type sendfunc = Option::<
    unsafe extern "C" fn(
        *mut PyObject,
        *mut PyObject,
        *mut *mut PyObject,
    ) -> PySendResult,
>;
pub type PySendResult = libc::c_int;
pub const PYGEN_NEXT: PySendResult = 1;
pub const PYGEN_ERROR: PySendResult = -1;
pub const PYGEN_RETURN: PySendResult = 0;
pub type setattrfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_char, *mut PyObject) -> libc::c_int,
>;
pub type getattrfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_char) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyVarObject {
    pub ob_base: PyObject,
    pub ob_size: Py_ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ob_refcnt: Py_ssize_t,
    pub ob_refcnt_split: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyModuleDef_Slot {
    pub slot: libc::c_int,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyModuleDef_Base {
    pub ob_base: PyObject,
    pub m_init: Option::<unsafe extern "C" fn() -> *mut PyObject>,
    pub m_index: Py_ssize_t,
    pub m_copy: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub asize: size_t,
    pub unit: size_t,
}
pub type mkd_autolink = libc::c_uint;
pub const MKDA_EMAIL: mkd_autolink = 2;
pub const MKDA_NORMAL: mkd_autolink = 1;
pub const MKDA_NOT_AUTOLINK: mkd_autolink = 0;
pub type mkd_extensions = libc::c_uint;
pub const MKDEXT_NO_EMAIL_AUTOLINK: mkd_extensions = 512;
pub const MKDEXT_LAX_SPACING: mkd_extensions = 256;
pub const MKDEXT_SUPERSCRIPT: mkd_extensions = 128;
pub const MKDEXT_SPACE_HEADERS: mkd_extensions = 64;
pub const MKDEXT_STRIKETHROUGH: mkd_extensions = 16;
pub const MKDEXT_AUTOLINK: mkd_extensions = 8;
pub const MKDEXT_FENCED_CODE: mkd_extensions = 4;
pub const MKDEXT_TABLES: mkd_extensions = 2;
pub const MKDEXT_NO_INTRA_EMPHASIS: mkd_extensions = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sd_callbacks {
    pub blockcode: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub blockquote: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub blockspoiler: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub blockhtml: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub header: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub hrule: Option::<unsafe extern "C" fn(*mut buf, *mut libc::c_void) -> ()>,
    pub list: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub listitem: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub paragraph: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub table: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub table_row: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub table_cell: Option::<
        unsafe extern "C" fn(
            *mut buf,
            *const buf,
            libc::c_int,
            *mut libc::c_void,
            libc::c_int,
        ) -> (),
    >,
    pub autolink: Option::<
        unsafe extern "C" fn(
            *mut buf,
            *const buf,
            mkd_autolink,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub codespan: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub spoilerspan: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub double_emphasis: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub emphasis: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub image: Option::<
        unsafe extern "C" fn(
            *mut buf,
            *const buf,
            *const buf,
            *const buf,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub linebreak: Option::<
        unsafe extern "C" fn(*mut buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub link: Option::<
        unsafe extern "C" fn(
            *mut buf,
            *const buf,
            *const buf,
            *const buf,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub raw_html_tag: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub triple_emphasis: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub strikethrough: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub superscript: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> libc::c_int,
    >,
    pub entity: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub normal_text: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
    pub doc_header: Option::<unsafe extern "C" fn(*mut buf, *mut libc::c_void) -> ()>,
    pub doc_footer: Option::<unsafe extern "C" fn(*mut buf, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct html_renderopt {
    pub toc_data: C2RustUnnamed_0,
    pub toc_id_prefix: *mut libc::c_char,
    pub flags: libc::c_uint,
    pub html_element_whitelist: *mut *mut libc::c_char,
    pub html_attr_whitelist: *mut *mut libc::c_char,
    pub link_attributes: Option::<
        unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub header_count: libc::c_int,
    pub current_level: libc::c_int,
    pub level_offset: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const HTML_ALLOW_ELEMENT_WHITELIST: C2RustUnnamed_1 = 1024;
pub const HTML_ESCAPE: C2RustUnnamed_1 = 512;
pub const HTML_USE_XHTML: C2RustUnnamed_1 = 256;
pub const HTML_HARD_WRAP: C2RustUnnamed_1 = 128;
pub const HTML_TOC: C2RustUnnamed_1 = 64;
pub const HTML_SAFELINK: C2RustUnnamed_1 = 32;
pub const HTML_EXPAND_TABS: C2RustUnnamed_1 = 16;
pub const HTML_SKIP_LINKS: C2RustUnnamed_1 = 8;
pub const HTML_SKIP_IMAGES: C2RustUnnamed_1 = 4;
pub const HTML_SKIP_STYLE: C2RustUnnamed_1 = 2;
pub const HTML_SKIP_HTML: C2RustUnnamed_1 = 1;
pub type snudown_renderer_mode = libc::c_uint;
pub const RENDERER_COUNT: snudown_renderer_mode = 2;
pub const RENDERER_WIKI: snudown_renderer_mode = 1;
pub const RENDERER_USERTEXT: snudown_renderer_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snudown_renderopt {
    pub html: html_renderopt,
    pub nofollow: libc::c_int,
    pub target: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snudown_renderer {
    pub main_renderer: *mut sd_markdown,
    pub toc_renderer: *mut sd_markdown,
    pub state: *mut module_state,
    pub toc_state: *mut module_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct module_state {
    pub callbacks: sd_callbacks,
    pub options: snudown_renderopt,
}
static mut sundown: [snudown_renderer; 2] = [snudown_renderer {
    main_renderer: 0 as *const sd_markdown as *mut sd_markdown,
    toc_renderer: 0 as *const sd_markdown as *mut sd_markdown,
    state: 0 as *const module_state as *mut module_state,
    toc_state: 0 as *const module_state as *mut module_state,
}; 2];
static mut html_element_whitelist: [*mut libc::c_char; 9] = [
    b"tr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"th\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"td\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"table\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tbody\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"thead\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tfoot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"caption\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut html_attr_whitelist: [*mut libc::c_char; 6] = [
    b"colspan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rowspan\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cellspacing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cellpadding\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scope\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut usertext_toc_state: module_state = module_state {
    callbacks: sd_callbacks {
        blockcode: None,
        blockquote: None,
        blockspoiler: None,
        blockhtml: None,
        header: None,
        hrule: None,
        list: None,
        listitem: None,
        paragraph: None,
        table: None,
        table_row: None,
        table_cell: None,
        autolink: None,
        codespan: None,
        spoilerspan: None,
        double_emphasis: None,
        emphasis: None,
        image: None,
        linebreak: None,
        link: None,
        raw_html_tag: None,
        triple_emphasis: None,
        strikethrough: None,
        superscript: None,
        entity: None,
        normal_text: None,
        doc_header: None,
        doc_footer: None,
    },
    options: snudown_renderopt {
        html: html_renderopt {
            toc_data: C2RustUnnamed_0 {
                header_count: 0,
                current_level: 0,
                level_offset: 0,
            },
            toc_id_prefix: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            html_element_whitelist: 0 as *const *mut libc::c_char
                as *mut *mut libc::c_char,
            html_attr_whitelist: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            link_attributes: None,
        },
        nofollow: 0,
        target: 0 as *const libc::c_char,
    },
};
static mut wiki_toc_state: module_state = module_state {
    callbacks: sd_callbacks {
        blockcode: None,
        blockquote: None,
        blockspoiler: None,
        blockhtml: None,
        header: None,
        hrule: None,
        list: None,
        listitem: None,
        paragraph: None,
        table: None,
        table_row: None,
        table_cell: None,
        autolink: None,
        codespan: None,
        spoilerspan: None,
        double_emphasis: None,
        emphasis: None,
        image: None,
        linebreak: None,
        link: None,
        raw_html_tag: None,
        triple_emphasis: None,
        strikethrough: None,
        superscript: None,
        entity: None,
        normal_text: None,
        doc_header: None,
        doc_footer: None,
    },
    options: snudown_renderopt {
        html: html_renderopt {
            toc_data: C2RustUnnamed_0 {
                header_count: 0,
                current_level: 0,
                level_offset: 0,
            },
            toc_id_prefix: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            html_element_whitelist: 0 as *const *mut libc::c_char
                as *mut *mut libc::c_char,
            html_attr_whitelist: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            link_attributes: None,
        },
        nofollow: 0,
        target: 0 as *const libc::c_char,
    },
};
static mut usertext_state: module_state = module_state {
    callbacks: sd_callbacks {
        blockcode: None,
        blockquote: None,
        blockspoiler: None,
        blockhtml: None,
        header: None,
        hrule: None,
        list: None,
        listitem: None,
        paragraph: None,
        table: None,
        table_row: None,
        table_cell: None,
        autolink: None,
        codespan: None,
        spoilerspan: None,
        double_emphasis: None,
        emphasis: None,
        image: None,
        linebreak: None,
        link: None,
        raw_html_tag: None,
        triple_emphasis: None,
        strikethrough: None,
        superscript: None,
        entity: None,
        normal_text: None,
        doc_header: None,
        doc_footer: None,
    },
    options: snudown_renderopt {
        html: html_renderopt {
            toc_data: C2RustUnnamed_0 {
                header_count: 0,
                current_level: 0,
                level_offset: 0,
            },
            toc_id_prefix: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            html_element_whitelist: 0 as *const *mut libc::c_char
                as *mut *mut libc::c_char,
            html_attr_whitelist: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            link_attributes: None,
        },
        nofollow: 0,
        target: 0 as *const libc::c_char,
    },
};
static mut wiki_state: module_state = module_state {
    callbacks: sd_callbacks {
        blockcode: None,
        blockquote: None,
        blockspoiler: None,
        blockhtml: None,
        header: None,
        hrule: None,
        list: None,
        listitem: None,
        paragraph: None,
        table: None,
        table_row: None,
        table_cell: None,
        autolink: None,
        codespan: None,
        spoilerspan: None,
        double_emphasis: None,
        emphasis: None,
        image: None,
        linebreak: None,
        link: None,
        raw_html_tag: None,
        triple_emphasis: None,
        strikethrough: None,
        superscript: None,
        entity: None,
        normal_text: None,
        doc_header: None,
        doc_footer: None,
    },
    options: snudown_renderopt {
        html: html_renderopt {
            toc_data: C2RustUnnamed_0 {
                header_count: 0,
                current_level: 0,
                level_offset: 0,
            },
            toc_id_prefix: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            html_element_whitelist: 0 as *const *mut libc::c_char
                as *mut *mut libc::c_char,
            html_attr_whitelist: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            link_attributes: None,
        },
        nofollow: 0,
        target: 0 as *const libc::c_char,
    },
};
static mut snudown_module__doc__: [libc::c_char; 41] = unsafe {
    *::core::mem::transmute::<
        &[u8; 41],
        &[libc::c_char; 41],
    >(b"When does the narwhal bacon? At Sundown.\0")
};
static mut snudown_md__doc__: [libc::c_char; 27] = unsafe {
    *::core::mem::transmute::<
        &[u8; 27],
        &[libc::c_char; 27],
    >(b"Render a Markdown document\0")
};
static mut snudown_default_md_flags: libc::c_uint = (MKDEXT_NO_INTRA_EMPHASIS
    as libc::c_int | MKDEXT_SUPERSCRIPT as libc::c_int | MKDEXT_AUTOLINK as libc::c_int
    | MKDEXT_STRIKETHROUGH as libc::c_int | MKDEXT_TABLES as libc::c_int)
    as libc::c_uint;
static mut snudown_default_render_flags: libc::c_uint = (HTML_SKIP_HTML as libc::c_int
    | HTML_SKIP_IMAGES as libc::c_int | HTML_SAFELINK as libc::c_int
    | HTML_ESCAPE as libc::c_int | HTML_USE_XHTML as libc::c_int) as libc::c_uint;
static mut snudown_wiki_render_flags: libc::c_uint = (HTML_SKIP_HTML as libc::c_int
    | HTML_SAFELINK as libc::c_int | HTML_ALLOW_ELEMENT_WHITELIST as libc::c_int
    | HTML_ESCAPE as libc::c_int | HTML_USE_XHTML as libc::c_int) as libc::c_uint;
unsafe extern "C" fn snudown_link_attr(
    mut ob: *mut buf,
    mut link: *const buf,
    mut opaque: *mut libc::c_void,
) {
    let mut options: *mut snudown_renderopt = opaque as *mut snudown_renderopt;
    if (*options).nofollow != 0 {
        bufput(
            ob,
            b" rel=\"nofollow\"\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    if !((*options).target).is_null() {
        bufput(
            ob,
            b" target=\"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        bufputs(ob, (*options).target);
        bufputc(ob, '"' as i32);
    }
}
unsafe extern "C" fn make_custom_renderer(
    mut state: *mut module_state,
    renderflags: libc::c_uint,
    markdownflags: libc::c_uint,
    mut toc_renderer: libc::c_int,
) -> *mut sd_markdown {
    if toc_renderer != 0 {
        sdhtml_toc_renderer(
            &mut (*state).callbacks,
            &mut (*state).options as *mut snudown_renderopt as *mut html_renderopt,
        );
    } else {
        sdhtml_renderer(
            &mut (*state).callbacks,
            &mut (*state).options as *mut snudown_renderopt as *mut html_renderopt,
            renderflags,
        );
    }
    (*state)
        .options
        .html
        .link_attributes = Some(
        snudown_link_attr
            as unsafe extern "C" fn(*mut buf, *const buf, *mut libc::c_void) -> (),
    );
    (*state).options.html.html_element_whitelist = html_element_whitelist.as_mut_ptr();
    (*state).options.html.html_attr_whitelist = html_attr_whitelist.as_mut_ptr();
    return sd_markdown_new(
        markdownflags,
        16 as libc::c_int as size_t,
        64 as libc::c_int as size_t,
        &mut (*state).callbacks,
        &mut (*state).options as *mut snudown_renderopt as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn init_default_renderer(mut module: *mut PyObject) {
    PyModule_AddIntConstant(
        module,
        b"RENDERER_USERTEXT\0" as *const u8 as *const libc::c_char,
        RENDERER_USERTEXT as libc::c_int as libc::c_long,
    );
    sundown[RENDERER_USERTEXT as libc::c_int as usize]
        .main_renderer = make_custom_renderer(
        &mut usertext_state,
        snudown_default_render_flags,
        snudown_default_md_flags,
        0 as libc::c_int,
    );
    sundown[RENDERER_USERTEXT as libc::c_int as usize]
        .toc_renderer = make_custom_renderer(
        &mut usertext_toc_state,
        snudown_default_render_flags,
        snudown_default_md_flags,
        1 as libc::c_int,
    );
    sundown[RENDERER_USERTEXT as libc::c_int as usize].state = &mut usertext_state;
    sundown[RENDERER_USERTEXT as libc::c_int as usize]
        .toc_state = &mut usertext_toc_state;
}
#[no_mangle]
pub unsafe extern "C" fn init_wiki_renderer(mut module: *mut PyObject) {
    PyModule_AddIntConstant(
        module,
        b"RENDERER_WIKI\0" as *const u8 as *const libc::c_char,
        RENDERER_WIKI as libc::c_int as libc::c_long,
    );
    sundown[RENDERER_WIKI as libc::c_int as usize]
        .main_renderer = make_custom_renderer(
        &mut wiki_state,
        snudown_wiki_render_flags,
        snudown_default_md_flags,
        0 as libc::c_int,
    );
    sundown[RENDERER_WIKI as libc::c_int as usize]
        .toc_renderer = make_custom_renderer(
        &mut wiki_toc_state,
        snudown_wiki_render_flags,
        snudown_default_md_flags,
        1 as libc::c_int,
    );
    sundown[RENDERER_WIKI as libc::c_int as usize].state = &mut wiki_state;
    sundown[RENDERER_WIKI as libc::c_int as usize].toc_state = &mut wiki_toc_state;
}
unsafe extern "C" fn snudown_md(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
    mut kwargs: *mut PyObject,
) -> *mut PyObject {
    static mut kwlist: [*mut libc::c_char; 7] = [
        b"text\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"nofollow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"target\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"toc_id_prefix\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"renderer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"enable_toc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut ib: buf = buf {
        data: 0 as *mut uint8_t,
        size: 0,
        asize: 0,
        unit: 0,
    };
    let mut ob: *mut buf = 0 as *mut buf;
    let mut py_result: *mut PyObject = 0 as *mut PyObject;
    let mut result_text: *const libc::c_char = 0 as *const libc::c_char;
    let mut renderer: libc::c_int = RENDERER_USERTEXT as libc::c_int;
    let mut enable_toc: libc::c_int = 0 as libc::c_int;
    let mut _snudown: snudown_renderer = snudown_renderer {
        main_renderer: 0 as *const sd_markdown as *mut sd_markdown,
        toc_renderer: 0 as *const sd_markdown as *mut sd_markdown,
        state: 0 as *const module_state as *mut module_state,
        toc_state: 0 as *const module_state as *mut module_state,
    };
    let mut nofollow: libc::c_int = 0 as libc::c_int;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut toc_id_prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_uint = 0;
    memset(
        &mut ib as *mut buf as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<buf>() as libc::c_ulong,
    );
    if _PyArg_ParseTupleAndKeywords_SizeT(
        args,
        kwargs,
        b"s#|izzii\0" as *const u8 as *const libc::c_char,
        kwlist.as_mut_ptr(),
        &mut ib.data as *mut *mut uint8_t,
        &mut ib.size as *mut size_t,
        &mut nofollow as *mut libc::c_int,
        &mut target as *mut *mut libc::c_char,
        &mut toc_id_prefix as *mut *mut libc::c_char,
        &mut renderer as *mut libc::c_int,
        &mut enable_toc as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if renderer < 0 as libc::c_int || renderer >= RENDERER_COUNT as libc::c_int {
        PyErr_SetString(
            PyExc_ValueError,
            b"Invalid renderer\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    _snudown = sundown[renderer as usize];
    let mut options: *mut snudown_renderopt = &mut (*_snudown.state).options;
    (*options).nofollow = nofollow;
    (*options).target = target;
    ob = bufnew(128 as libc::c_int as size_t);
    flags = (*options).html.flags;
    if enable_toc != 0 {
        (*_snudown.toc_state).options.html.toc_id_prefix = toc_id_prefix;
        sd_markdown_render(ob, ib.data, ib.size, _snudown.toc_renderer);
        (*_snudown.toc_state).options.html.toc_id_prefix = 0 as *mut libc::c_char;
        (*options).html.flags |= HTML_TOC as libc::c_int as libc::c_uint;
    }
    (*options).html.toc_id_prefix = toc_id_prefix;
    sd_markdown_render(ob, ib.data, ib.size, _snudown.main_renderer);
    (*options).html.toc_id_prefix = 0 as *mut libc::c_char;
    (*options).html.flags = flags;
    result_text = b"\0" as *const u8 as *const libc::c_char;
    if !((*ob).data).is_null() {
        result_text = (*ob).data as *const libc::c_char;
    }
    py_result = _Py_BuildValue_SizeT(
        b"s#\0" as *const u8 as *const libc::c_char,
        result_text,
        (*ob).size as libc::c_int,
    );
    bufrelease(ob);
    return py_result;
}
static mut snudown_methods: [PyMethodDef; 2] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"markdown\0" as *const u8 as *const libc::c_char,
                ml_meth: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(
                    Some(
                        snudown_md
                            as unsafe extern "C" fn(
                                *mut PyObject,
                                *mut PyObject,
                                *mut PyObject,
                            ) -> *mut PyObject,
                    ),
                ),
                ml_flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                ml_doc: snudown_md__doc__.as_ptr(),
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: 0 as *const libc::c_char,
                ml_meth: None,
                ml_flags: 0 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn PyInit_snudown() -> *mut PyObject {
    let mut module: *mut PyObject = 0 as *mut PyObject;
    static mut moduledef: PyModuleDef = unsafe {
        {
            let mut init = PyModuleDef {
                m_base: {
                    let mut init = PyModuleDef_Base {
                        ob_base: {
                            let mut init = _object {
                                c2rust_unnamed: C2RustUnnamed {
                                    ob_refcnt: 1 as libc::c_int as Py_ssize_t,
                                },
                                ob_type: 0 as *const PyTypeObject as *mut PyTypeObject,
                            };
                            init
                        },
                        m_init: None,
                        m_index: 0 as libc::c_int as Py_ssize_t,
                        m_copy: 0 as *const PyObject as *mut PyObject,
                    };
                    init
                },
                m_name: b"snudown\0" as *const u8 as *const libc::c_char,
                m_doc: snudown_module__doc__.as_ptr(),
                m_size: -(1 as libc::c_int) as Py_ssize_t,
                m_methods: snudown_methods.as_ptr() as *mut _,
                m_slots: 0 as *const PyModuleDef_Slot as *mut PyModuleDef_Slot,
                m_traverse: None,
                m_clear: None,
                m_free: None,
            };
            init
        }
    };
    module = PyModule_Create2(&mut moduledef, 1013 as libc::c_int);
    if module.is_null() {
        return module;
    }
    init_default_renderer(module);
    init_wiki_renderer(module);
    PyModule_AddStringConstant(
        module,
        b"__version__\0" as *const u8 as *const libc::c_char,
        b"1.5.0\0" as *const u8 as *const libc::c_char,
    );
    return module;
}

fn main() {
    println!("Hello, Snudown!");
    // You can add the actual logic that needs to run here
}

