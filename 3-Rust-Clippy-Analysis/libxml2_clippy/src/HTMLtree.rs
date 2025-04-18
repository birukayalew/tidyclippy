use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    fn xmlStrcasestr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlStrcasecmp(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xmlOutputBufferCreateFilename(
        URI: *const libc::c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: libc::c_int,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferWriteString(
        out: xmlOutputBufferPtr,
        str: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> libc::c_int;
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> libc::c_int;
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    fn xmlNewProp(
        node: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlSetProp(
        node: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: libc::c_int,
    ) -> *mut xmlChar;
    fn xmlFindCharEncodingHandler(
        name: *const libc::c_char,
    ) -> xmlCharEncodingHandlerPtr;
    fn xmlParseCharEncoding(name: *const libc::c_char) -> xmlCharEncoding;
    fn xmlInitParser();
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn __xmlSimpleError(
        domain: libc::c_int,
        code: libc::c_int,
        node: xmlNodePtr,
        msg: *const libc::c_char,
        extra: *const libc::c_char,
    );
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn htmlTagLookup(tag: *const xmlChar) -> *const htmlElemDesc;
    static xmlStringText: [xmlChar; 0];
    static xmlStringTextNoenc: [xmlChar; 0];
    fn xmlURIEscapeStr(str: *const xmlChar, list: *const xmlChar) -> *mut xmlChar;
    fn xmlBufCat(buf: xmlBufPtr, str: *const xmlChar) -> libc::c_int;
    fn xmlBufCCat(buf: xmlBufPtr, str: *const libc::c_char) -> libc::c_int;
    fn xmlBufWriteQuotedString(buf: xmlBufPtr, string: *const xmlChar) -> libc::c_int;
    fn xmlBufFromBuffer(buffer: xmlBufferPtr) -> xmlBufPtr;
    fn xmlBufBackToBuffer(buf: xmlBufPtr) -> xmlBufferPtr;
    fn xmlAllocOutputBufferInternal(
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlNsListDumpOutput(buf: xmlOutputBufferPtr, cur: xmlNsPtr);
}
pub type xmlChar = libc::c_uchar;
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
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut libc::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut libc::c_void;
pub type xmlCharEncodingOutputFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        *mut libc::c_int,
        *const libc::c_uchar,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlCharEncodingInputFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        *mut libc::c_int,
        *const libc::c_uchar,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut libc::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: libc::c_int,
    pub error: libc::c_int,
}
pub type xmlOutputCloseCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type xmlOutputWriteCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut libc::c_void,
    pub line: libc::c_ushort,
    pub extra: libc::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut libc::c_void,
    pub context: *mut _xmlDoc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *mut libc::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: libc::c_int,
    pub standalone: libc::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut libc::c_void,
    pub refs: *mut libc::c_void,
    pub URL: *const xmlChar,
    pub charset: libc::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut libc::c_void,
    pub parseFlags: libc::c_int,
    pub properties: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut libc::c_void,
    pub elements: *mut libc::c_void,
    pub attributes: *mut libc::c_void,
    pub entities: *mut libc::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut libc::c_void,
}
pub type xmlElementType = libc::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = xmlElementType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut libc::c_void,
}
pub type xmlAttributeType = libc::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlBufferAllocationScheme = libc::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: libc::c_uint,
    pub size: libc::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type C2RustUnnamed = libc::c_uint;
pub const XML_FROM_URI: C2RustUnnamed = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed = 28;
pub const XML_FROM_I18N: C2RustUnnamed = 27;
pub const XML_FROM_MODULE: C2RustUnnamed = 26;
pub const XML_FROM_WRITER: C2RustUnnamed = 25;
pub const XML_FROM_CHECK: C2RustUnnamed = 24;
pub const XML_FROM_VALID: C2RustUnnamed = 23;
pub const XML_FROM_XSLT: C2RustUnnamed = 22;
pub const XML_FROM_C14N: C2RustUnnamed = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed = 13;
pub const XML_FROM_XPATH: C2RustUnnamed = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed = 11;
pub const XML_FROM_HTTP: C2RustUnnamed = 10;
pub const XML_FROM_FTP: C2RustUnnamed = 9;
pub const XML_FROM_IO: C2RustUnnamed = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
pub const XML_FROM_HTML: C2RustUnnamed = 5;
pub const XML_FROM_DTD: C2RustUnnamed = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
pub type xmlCharEncoding = libc::c_int;
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
pub type htmlDocPtr = xmlDocPtr;
pub type htmlNodePtr = xmlNodePtr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _htmlElemDesc {
    pub name: *const libc::c_char,
    pub startTag: libc::c_char,
    pub endTag: libc::c_char,
    pub saveEndTag: libc::c_char,
    pub empty: libc::c_char,
    pub depr: libc::c_char,
    pub dtd: libc::c_char,
    pub isinline: libc::c_char,
    pub desc: *const libc::c_char,
    pub subelts: *mut *const libc::c_char,
    pub defaultsubelt: *const libc::c_char,
    pub attrs_opt: *mut *const libc::c_char,
    pub attrs_depr: *mut *const libc::c_char,
    pub attrs_req: *mut *const libc::c_char,
}
pub type htmlElemDesc = _htmlElemDesc;
#[no_mangle]
pub unsafe extern "C" fn htmlGetMetaEncoding(mut doc: htmlDocPtr) -> *const xmlChar {
    let mut current_block: u64;
    let mut cur: htmlNodePtr = 0 as *mut xmlNode;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    let mut encoding: *const xmlChar = 0 as *const xmlChar;
    if doc.is_null() {
        return 0 as *const xmlChar;
    }
    cur = (*doc).children;
    loop {
        if cur.is_null() {
            current_block = 1394248824506584008;
            break;
        }
        if (*cur).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && !((*cur).name).is_null()
        {
            if xmlStrEqual(
                (*cur).name,
                b"html\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                current_block = 1394248824506584008;
                break;
            }
            if xmlStrEqual(
                (*cur).name,
                b"head\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                current_block = 11008951124456638817;
                break;
            }
            if xmlStrEqual(
                (*cur).name,
                b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                current_block = 2838571290723028321;
                break;
            }
        }
        cur = (*cur).next;
    }
    match current_block {
        1394248824506584008 => {
            if cur.is_null() {
                return 0 as *const xmlChar;
            }
            cur = (*cur).children;
            loop {
                if cur.is_null() {
                    current_block = 4956146061682418353;
                    break;
                }
                if (*cur).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && !((*cur).name).is_null()
                {
                    if xmlStrEqual(
                        (*cur).name,
                        b"head\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 4956146061682418353;
                        break;
                    }
                    if xmlStrEqual(
                        (*cur).name,
                        b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 2838571290723028321;
                        break;
                    }
                }
                cur = (*cur).next;
            }
            match current_block {
                2838571290723028321 => {}
                _ => {
                    if cur.is_null() {
                        return 0 as *const xmlChar;
                    }
                    current_block = 11008951124456638817;
                }
            }
        }
        _ => {}
    }
    match current_block {
        11008951124456638817 => {
            cur = (*cur).children;
        }
        _ => {}
    }
    's_100: loop {
        if cur.is_null() {
            current_block = 2604890879466389055;
            break;
        }
        if (*cur).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && !((*cur).name).is_null()
        {
            if xmlStrEqual(
                (*cur).name,
                b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                let mut attr: xmlAttrPtr = (*cur).properties;
                let mut http: libc::c_int = 0;
                let mut value: *const xmlChar = 0 as *const xmlChar;
                content = 0 as *const xmlChar;
                http = 0 as libc::c_int;
                while !attr.is_null() {
                    if !((*attr).children).is_null()
                        && (*(*attr).children).type_0 as libc::c_uint
                            == XML_TEXT_NODE as libc::c_int as libc::c_uint
                        && ((*(*attr).children).next).is_null()
                    {
                        value = (*(*attr).children).content;
                        if xmlStrcasecmp(
                            (*attr).name,
                            b"http-equiv\0" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) == 0
                            && xmlStrcasecmp(
                                value,
                                b"Content-Type\0" as *const u8 as *const libc::c_char
                                    as *mut xmlChar,
                            ) == 0
                        {
                            http = 1 as libc::c_int;
                        } else if !value.is_null()
                            && xmlStrcasecmp(
                                (*attr).name,
                                b"content\0" as *const u8 as *const libc::c_char
                                    as *mut xmlChar,
                            ) == 0
                        {
                            content = value;
                        }
                        if http != 0 as libc::c_int && !content.is_null() {
                            current_block = 2540463515064516283;
                            break 's_100;
                        }
                    }
                    attr = (*attr).next;
                }
            }
        }
        cur = (*cur).next;
    }
    match current_block {
        2604890879466389055 => return 0 as *const xmlChar,
        _ => {
            encoding = xmlStrstr(
                content,
                b"charset=\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            );
            if encoding.is_null() {
                encoding = xmlStrstr(
                    content,
                    b"Charset=\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                );
            }
            if encoding.is_null() {
                encoding = xmlStrstr(
                    content,
                    b"CHARSET=\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                );
            }
            if !encoding.is_null() {
                encoding = encoding.offset(8 as libc::c_int as isize);
            } else {
                encoding = xmlStrstr(
                    content,
                    b"charset =\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                );
                if encoding.is_null() {
                    encoding = xmlStrstr(
                        content,
                        b"Charset =\0" as *const u8 as *const libc::c_char
                            as *mut xmlChar,
                    );
                }
                if encoding.is_null() {
                    encoding = xmlStrstr(
                        content,
                        b"CHARSET =\0" as *const u8 as *const libc::c_char
                            as *mut xmlChar,
                    );
                }
                if !encoding.is_null() {
                    encoding = encoding.offset(9 as libc::c_int as isize);
                }
            }
            if !encoding.is_null() {
                while *encoding as libc::c_int == ' ' as i32
                    || *encoding as libc::c_int == '\t' as i32
                {
                    encoding = encoding.offset(1);
                    encoding;
                }
            }
            return encoding;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htmlSetMetaEncoding(
    mut doc: htmlDocPtr,
    mut encoding: *const xmlChar,
) -> libc::c_int {
    let mut current_block: u64;
    let mut cur: htmlNodePtr = 0 as *mut xmlNode;
    let mut meta: htmlNodePtr = 0 as htmlNodePtr;
    let mut head: htmlNodePtr = 0 as htmlNodePtr;
    let mut content: *const xmlChar = 0 as *const xmlChar;
    let mut newcontent: [libc::c_char; 100] = [0; 100];
    newcontent[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if doc.is_null() {
        return -(1 as libc::c_int);
    }
    if xmlStrcasecmp(
        encoding,
        b"html\0" as *const u8 as *const libc::c_char as *mut xmlChar,
    ) == 0
    {
        return -(1 as libc::c_int);
    }
    if !encoding.is_null() {
        snprintf(
            newcontent.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            b"text/html; charset=%s\0" as *const u8 as *const libc::c_char,
            encoding as *mut libc::c_char,
        );
        newcontent[(::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    cur = (*doc).children;
    loop {
        if cur.is_null() {
            current_block = 10048703153582371463;
            break;
        }
        if (*cur).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && !((*cur).name).is_null()
        {
            if xmlStrcasecmp(
                (*cur).name,
                b"html\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) == 0 as libc::c_int
            {
                current_block = 10048703153582371463;
                break;
            }
            if xmlStrcasecmp(
                (*cur).name,
                b"head\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) == 0 as libc::c_int
            {
                current_block = 9143738939980485823;
                break;
            }
            if xmlStrcasecmp(
                (*cur).name,
                b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) == 0 as libc::c_int
            {
                current_block = 16269646940708665267;
                break;
            }
        }
        cur = (*cur).next;
    }
    match current_block {
        10048703153582371463 => {
            if cur.is_null() {
                return -(1 as libc::c_int);
            }
            cur = (*cur).children;
            loop {
                if cur.is_null() {
                    current_block = 5634871135123216486;
                    break;
                }
                if (*cur).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && !((*cur).name).is_null()
                {
                    if xmlStrcasecmp(
                        (*cur).name,
                        b"head\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) == 0 as libc::c_int
                    {
                        current_block = 5634871135123216486;
                        break;
                    }
                    if xmlStrcasecmp(
                        (*cur).name,
                        b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) == 0 as libc::c_int
                    {
                        head = (*cur).parent;
                        current_block = 16269646940708665267;
                        break;
                    }
                }
                cur = (*cur).next;
            }
            match current_block {
                16269646940708665267 => {}
                _ => {
                    if cur.is_null() {
                        return -(1 as libc::c_int);
                    }
                    current_block = 9143738939980485823;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9143738939980485823 => {
            head = cur;
            if ((*cur).children).is_null() {
                current_block = 14194835648362820970;
            } else {
                cur = (*cur).children;
                current_block = 16269646940708665267;
            }
        }
        _ => {}
    }
    match current_block {
        16269646940708665267 => {
            while !cur.is_null() {
                if (*cur).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && !((*cur).name).is_null()
                {
                    if xmlStrcasecmp(
                        (*cur).name,
                        b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) == 0 as libc::c_int
                    {
                        let mut attr: xmlAttrPtr = (*cur).properties;
                        let mut http: libc::c_int = 0;
                        let mut value: *const xmlChar = 0 as *const xmlChar;
                        content = 0 as *const xmlChar;
                        http = 0 as libc::c_int;
                        while !attr.is_null() {
                            if !((*attr).children).is_null()
                                && (*(*attr).children).type_0 as libc::c_uint
                                    == XML_TEXT_NODE as libc::c_int as libc::c_uint
                                && ((*(*attr).children).next).is_null()
                            {
                                value = (*(*attr).children).content;
                                if xmlStrcasecmp(
                                    (*attr).name,
                                    b"http-equiv\0" as *const u8 as *const libc::c_char
                                        as *mut xmlChar,
                                ) == 0
                                    && xmlStrcasecmp(
                                        value,
                                        b"Content-Type\0" as *const u8 as *const libc::c_char
                                            as *mut xmlChar,
                                    ) == 0
                                {
                                    http = 1 as libc::c_int;
                                } else if !value.is_null()
                                    && xmlStrcasecmp(
                                        (*attr).name,
                                        b"content\0" as *const u8 as *const libc::c_char
                                            as *mut xmlChar,
                                    ) == 0
                                {
                                    content = value;
                                }
                                if http != 0 as libc::c_int && !content.is_null() {
                                    break;
                                }
                            }
                            attr = (*attr).next;
                        }
                        if http != 0 as libc::c_int && !content.is_null() {
                            meta = cur;
                            break;
                        }
                    }
                }
                cur = (*cur).next;
            }
        }
        _ => {}
    }
    if meta.is_null() {
        if !encoding.is_null() && !head.is_null() {
            meta = xmlNewDocNode(
                doc,
                0 as xmlNsPtr,
                b"meta\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                0 as *const xmlChar,
            );
            if ((*head).children).is_null() {
                xmlAddChild(head, meta);
            } else {
                xmlAddPrevSibling((*head).children, meta);
            }
            xmlNewProp(
                meta,
                b"http-equiv\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                b"Content-Type\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            );
            xmlNewProp(
                meta,
                b"content\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                newcontent.as_mut_ptr() as *mut xmlChar,
            );
        }
    } else if encoding.is_null() {
        xmlUnlinkNode(meta);
        xmlFreeNode(meta);
    } else if (xmlStrcasestr(content, encoding)).is_null() {
        xmlSetProp(
            meta,
            b"content\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            newcontent.as_mut_ptr() as *mut xmlChar,
        );
    }
    return 0 as libc::c_int;
}
static mut htmlBooleanAttrs: [*const libc::c_char; 14] = [
    b"checked\0" as *const u8 as *const libc::c_char,
    b"compact\0" as *const u8 as *const libc::c_char,
    b"declare\0" as *const u8 as *const libc::c_char,
    b"defer\0" as *const u8 as *const libc::c_char,
    b"disabled\0" as *const u8 as *const libc::c_char,
    b"ismap\0" as *const u8 as *const libc::c_char,
    b"multiple\0" as *const u8 as *const libc::c_char,
    b"nohref\0" as *const u8 as *const libc::c_char,
    b"noresize\0" as *const u8 as *const libc::c_char,
    b"noshade\0" as *const u8 as *const libc::c_char,
    b"nowrap\0" as *const u8 as *const libc::c_char,
    b"readonly\0" as *const u8 as *const libc::c_char,
    b"selected\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn htmlIsBooleanAttr(mut name: *const xmlChar) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(htmlBooleanAttrs[i as usize]).is_null() {
        if xmlStrcasecmp(htmlBooleanAttrs[i as usize] as *const xmlChar, name)
            == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn htmlSaveErrMemory(mut extra: *const libc::c_char) {
    __xmlSimpleError(
        XML_FROM_OUTPUT as libc::c_int,
        XML_ERR_NO_MEMORY as libc::c_int,
        0 as xmlNodePtr,
        0 as *const libc::c_char,
        extra,
    );
}
unsafe extern "C" fn htmlSaveErr(
    mut code: libc::c_int,
    mut node: xmlNodePtr,
    mut extra: *const libc::c_char,
) {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    match code {
        1400 => {
            msg = b"string is not in UTF-8\n\0" as *const u8 as *const libc::c_char;
        }
        1401 => {
            msg = b"invalid character value\n\0" as *const u8 as *const libc::c_char;
        }
        1403 => {
            msg = b"unknown encoding %s\n\0" as *const u8 as *const libc::c_char;
        }
        1402 => {
            msg = b"HTML has no DOCTYPE\n\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            msg = b"unexpected error number\n\0" as *const u8 as *const libc::c_char;
        }
    }
    __xmlSimpleError(XML_FROM_OUTPUT as libc::c_int, code, node, msg, extra);
}
unsafe extern "C" fn htmlBufNodeDumpFormat(
    mut buf: xmlBufPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut format: libc::c_int,
) -> size_t {
    let mut use_0: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut outbuf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if cur.is_null() {
        return -(1 as libc::c_int) as size_t;
    }
    if buf.is_null() {
        return -(1 as libc::c_int) as size_t;
    }
    outbuf = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlOutputBuffer>() as libc::c_ulong)
        as xmlOutputBufferPtr;
    if outbuf.is_null() {
        htmlSaveErrMemory(
            b"allocating HTML output buffer\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as size_t;
    }
    memset(
        outbuf as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlOutputBuffer>() as libc::c_ulong,
    );
    (*outbuf).buffer = buf;
    (*outbuf).encoder = 0 as xmlCharEncodingHandlerPtr;
    (*outbuf).writecallback = None;
    (*outbuf).closecallback = None;
    (*outbuf).context = 0 as *mut libc::c_void;
    (*outbuf).written = 0 as libc::c_int;
    use_0 = xmlBufUse(buf);
    htmlNodeDumpFormatOutput(outbuf, doc, cur, 0 as *const libc::c_char, format);
    xmlFree.expect("non-null function pointer")(outbuf as *mut libc::c_void);
    ret = (xmlBufUse(buf)).wrapping_sub(use_0) as libc::c_int;
    return ret as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDump(
    mut buf: xmlBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
) -> libc::c_int {
    let mut buffer: xmlBufPtr = 0 as *mut xmlBuf;
    let mut ret: size_t = 0;
    if buf.is_null() || cur.is_null() {
        return -(1 as libc::c_int);
    }
    xmlInitParser();
    buffer = xmlBufFromBuffer(buf);
    if buffer.is_null() {
        return -(1 as libc::c_int);
    }
    ret = htmlBufNodeDumpFormat(buffer, doc, cur, 1 as libc::c_int);
    xmlBufBackToBuffer(buffer);
    if ret > 2147483647 as libc::c_int as size_t {
        return -(1 as libc::c_int);
    }
    return ret as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpFileFormat(
    mut out: *mut FILE,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const libc::c_char,
    mut format: libc::c_int,
) -> libc::c_int {
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    let mut ret: libc::c_int = 0;
    xmlInitParser();
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as libc::c_int != XML_CHAR_ENCODING_UTF8 as libc::c_int {
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as libc::c_int,
                    0 as xmlNodePtr,
                    encoding,
                );
            }
        }
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"HTML\0" as *const u8 as *const libc::c_char,
        );
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"ascii\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = xmlOutputBufferCreateFile(out, handler);
    if buf.is_null() {
        return 0 as libc::c_int;
    }
    htmlNodeDumpFormatOutput(buf, doc, cur, encoding, format);
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpFile(
    mut out: *mut FILE,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
) {
    htmlNodeDumpFileFormat(out, doc, cur, 0 as *const libc::c_char, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocDumpMemoryFormat(
    mut cur: xmlDocPtr,
    mut mem: *mut *mut xmlChar,
    mut size: *mut libc::c_int,
    mut format: libc::c_int,
) {
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
    xmlInitParser();
    if mem.is_null() || size.is_null() {
        return;
    }
    if cur.is_null() {
        *mem = 0 as *mut xmlChar;
        *size = 0 as libc::c_int;
        return;
    }
    encoding = htmlGetMetaEncoding(cur) as *const libc::c_char;
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as libc::c_int != (*cur).charset {
            if (*cur).charset != XML_CHAR_ENCODING_UTF8 as libc::c_int {
                *mem = 0 as *mut xmlChar;
                *size = 0 as libc::c_int;
                return;
            }
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as libc::c_int,
                    0 as xmlNodePtr,
                    encoding,
                );
            }
        } else {
            handler = xmlFindCharEncodingHandler(encoding);
        }
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"HTML\0" as *const u8 as *const libc::c_char,
        );
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"ascii\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = xmlAllocOutputBufferInternal(handler);
    if buf.is_null() {
        *mem = 0 as *mut xmlChar;
        *size = 0 as libc::c_int;
        return;
    }
    htmlDocContentDumpFormatOutput(buf, cur, 0 as *const libc::c_char, format);
    xmlOutputBufferFlush(buf);
    if !((*buf).conv).is_null() {
        *size = xmlBufUse((*buf).conv) as libc::c_int;
        *mem = xmlStrndup(xmlBufContent((*buf).conv as *const xmlBuf), *size);
    } else {
        *size = xmlBufUse((*buf).buffer) as libc::c_int;
        *mem = xmlStrndup(xmlBufContent((*buf).buffer as *const xmlBuf), *size);
    }
    xmlOutputBufferClose(buf);
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocDumpMemory(
    mut cur: xmlDocPtr,
    mut mem: *mut *mut xmlChar,
    mut size: *mut libc::c_int,
) {
    htmlDocDumpMemoryFormat(cur, mem, size, 1 as libc::c_int);
}
unsafe extern "C" fn htmlDtdDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut encoding: *const libc::c_char,
) {
    let mut cur: xmlDtdPtr = (*doc).intSubset;
    if cur.is_null() {
        htmlSaveErr(
            XML_SAVE_NO_DOCTYPE as libc::c_int,
            doc as xmlNodePtr,
            0 as *const libc::c_char,
        );
        return;
    }
    xmlOutputBufferWriteString(buf, b"<!DOCTYPE \0" as *const u8 as *const libc::c_char);
    xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
    if !((*cur).ExternalID).is_null() {
        xmlOutputBufferWriteString(
            buf,
            b" PUBLIC \0" as *const u8 as *const libc::c_char,
        );
        xmlBufWriteQuotedString((*buf).buffer, (*cur).ExternalID);
        if !((*cur).SystemID).is_null() {
            xmlOutputBufferWriteString(buf, b" \0" as *const u8 as *const libc::c_char);
            xmlBufWriteQuotedString((*buf).buffer, (*cur).SystemID);
        }
    } else if !((*cur).SystemID).is_null()
        && xmlStrcmp(
            (*cur).SystemID,
            b"about:legacy-compat\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
    {
        xmlOutputBufferWriteString(
            buf,
            b" SYSTEM \0" as *const u8 as *const libc::c_char,
        );
        xmlBufWriteQuotedString((*buf).buffer, (*cur).SystemID);
    }
    xmlOutputBufferWriteString(buf, b">\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn htmlAttrDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlAttrPtr,
    mut encoding: *const libc::c_char,
) {
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    if cur.is_null() {
        return;
    }
    xmlOutputBufferWriteString(buf, b" \0" as *const u8 as *const libc::c_char);
    if !((*cur).ns).is_null() && !((*(*cur).ns).prefix).is_null() {
        xmlOutputBufferWriteString(buf, (*(*cur).ns).prefix as *const libc::c_char);
        xmlOutputBufferWriteString(buf, b":\0" as *const u8 as *const libc::c_char);
    }
    xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
    if !((*cur).children).is_null() && htmlIsBooleanAttr((*cur).name) == 0 {
        value = xmlNodeListGetString(doc, (*cur).children, 0 as libc::c_int);
        if !value.is_null() {
            xmlOutputBufferWriteString(buf, b"=\0" as *const u8 as *const libc::c_char);
            if ((*cur).ns).is_null() && !((*cur).parent).is_null()
                && ((*(*cur).parent).ns).is_null()
                && (xmlStrcasecmp(
                    (*cur).name,
                    b"href\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) == 0
                    || xmlStrcasecmp(
                        (*cur).name,
                        b"action\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) == 0
                    || xmlStrcasecmp(
                        (*cur).name,
                        b"src\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) == 0
                    || xmlStrcasecmp(
                        (*cur).name,
                        b"name\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) == 0
                        && xmlStrcasecmp(
                            (*(*cur).parent).name,
                            b"a\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                        ) == 0)
            {
                let mut tmp: *mut xmlChar = value;
                xmlBufCCat((*buf).buffer, b"\"\0" as *const u8 as *const libc::c_char);
                while *tmp as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *tmp as libc::c_int
                        && *tmp as libc::c_int <= 0xa as libc::c_int
                    || *tmp as libc::c_int == 0xd as libc::c_int
                {
                    tmp = tmp.offset(1);
                    tmp;
                }
                loop {
                    let mut escaped: *mut xmlChar = 0 as *mut xmlChar;
                    let mut endChar: xmlChar = 0;
                    let mut end: *mut xmlChar = 0 as *mut xmlChar;
                    let mut start: *mut xmlChar = xmlStrstr(
                        tmp,
                        b"<!--\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) as *mut xmlChar;
                    if !start.is_null() {
                        end = xmlStrstr(
                            tmp,
                            b"-->\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                        ) as *mut xmlChar;
                        if !end.is_null() {
                            *start = '\0' as i32 as xmlChar;
                        }
                    }
                    escaped = xmlURIEscapeStr(
                        tmp,
                        b"@/:=?;#%&,+\0" as *const u8 as *const libc::c_char
                            as *mut xmlChar,
                    );
                    if !escaped.is_null() {
                        xmlBufCat((*buf).buffer, escaped);
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(escaped as *mut libc::c_void);
                    } else {
                        xmlBufCat((*buf).buffer, tmp);
                    }
                    if end.is_null() {
                        break;
                    }
                    *start = '<' as i32 as xmlChar;
                    end = end.offset(3 as libc::c_int as isize);
                    endChar = *end;
                    *end = '\0' as i32 as xmlChar;
                    xmlBufCat((*buf).buffer, start);
                    *end = endChar;
                    tmp = end;
                }
                xmlBufCCat((*buf).buffer, b"\"\0" as *const u8 as *const libc::c_char);
            } else {
                xmlBufWriteQuotedString((*buf).buffer, value);
            }
            xmlFree.expect("non-null function pointer")(value as *mut libc::c_void);
        } else {
            xmlOutputBufferWriteString(
                buf,
                b"=\"\"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn htmlAttrListDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlAttrPtr,
    mut encoding: *const libc::c_char,
) {
    if cur.is_null() {
        return;
    }
    while !cur.is_null() {
        htmlAttrDumpOutput(buf, doc, cur, encoding);
        cur = (*cur).next;
    }
}
unsafe extern "C" fn htmlNodeListDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const libc::c_char,
    mut format: libc::c_int,
) {
    if cur.is_null() {
        return;
    }
    while !cur.is_null() {
        htmlNodeDumpFormatOutput(buf, doc, cur, encoding, format);
        cur = (*cur).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpFormatOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const libc::c_char,
    mut format: libc::c_int,
) {
    let mut info: *const htmlElemDesc = 0 as *const htmlElemDesc;
    xmlInitParser();
    if cur.is_null() || buf.is_null() {
        return;
    }
    if (*cur).type_0 as libc::c_uint == XML_DTD_NODE as libc::c_int as libc::c_uint {
        return;
    }
    if (*cur).type_0 as libc::c_uint
        == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        || (*cur).type_0 as libc::c_uint
            == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        htmlDocContentDumpOutput(buf, cur as xmlDocPtr, encoding);
        return;
    }
    if (*cur).type_0 as libc::c_uint == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
    {
        htmlAttrDumpOutput(buf, doc, cur as xmlAttrPtr, encoding);
        return;
    }
    if (*cur).type_0 as libc::c_uint == XML_TEXT_NODE as libc::c_int as libc::c_uint {
        if !((*cur).content).is_null() {
            if ((*cur).name == xmlStringText.as_ptr()
                || (*cur).name != xmlStringTextNoenc.as_ptr())
                && (((*cur).parent).is_null()
                    || xmlStrcasecmp(
                        (*(*cur).parent).name,
                        b"script\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) != 0
                        && xmlStrcasecmp(
                            (*(*cur).parent).name,
                            b"style\0" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0)
            {
                let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
                buffer = xmlEncodeEntitiesReentrant(doc, (*cur).content);
                if !buffer.is_null() {
                    xmlOutputBufferWriteString(buf, buffer as *const libc::c_char);
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(buffer as *mut libc::c_void);
                }
            } else {
                xmlOutputBufferWriteString(buf, (*cur).content as *const libc::c_char);
            }
        }
        return;
    }
    if (*cur).type_0 as libc::c_uint == XML_COMMENT_NODE as libc::c_int as libc::c_uint {
        if !((*cur).content).is_null() {
            xmlOutputBufferWriteString(
                buf,
                b"<!--\0" as *const u8 as *const libc::c_char,
            );
            xmlOutputBufferWriteString(buf, (*cur).content as *const libc::c_char);
            xmlOutputBufferWriteString(
                buf,
                b"-->\0" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    if (*cur).type_0 as libc::c_uint == XML_PI_NODE as libc::c_int as libc::c_uint {
        if ((*cur).name).is_null() {
            return;
        }
        xmlOutputBufferWriteString(buf, b"<?\0" as *const u8 as *const libc::c_char);
        xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
        if !((*cur).content).is_null() {
            xmlOutputBufferWriteString(buf, b" \0" as *const u8 as *const libc::c_char);
            xmlOutputBufferWriteString(buf, (*cur).content as *const libc::c_char);
        }
        xmlOutputBufferWriteString(buf, b">\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*cur).type_0 as libc::c_uint
        == XML_ENTITY_REF_NODE as libc::c_int as libc::c_uint
    {
        xmlOutputBufferWriteString(buf, b"&\0" as *const u8 as *const libc::c_char);
        xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
        xmlOutputBufferWriteString(buf, b";\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*cur).type_0 as libc::c_uint
        == XML_CDATA_SECTION_NODE as libc::c_int as libc::c_uint
    {
        if !((*cur).content).is_null() {
            xmlOutputBufferWriteString(buf, (*cur).content as *const libc::c_char);
        }
        return;
    }
    if ((*cur).ns).is_null() {
        info = htmlTagLookup((*cur).name);
    } else {
        info = 0 as *const htmlElemDesc;
    }
    xmlOutputBufferWriteString(buf, b"<\0" as *const u8 as *const libc::c_char);
    if !((*cur).ns).is_null() && !((*(*cur).ns).prefix).is_null() {
        xmlOutputBufferWriteString(buf, (*(*cur).ns).prefix as *const libc::c_char);
        xmlOutputBufferWriteString(buf, b":\0" as *const u8 as *const libc::c_char);
    }
    xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
    if !((*cur).nsDef).is_null() {
        xmlNsListDumpOutput(buf, (*cur).nsDef);
    }
    if !((*cur).properties).is_null() {
        htmlAttrListDumpOutput(buf, doc, (*cur).properties, encoding);
    }
    if !info.is_null() && (*info).empty as libc::c_int != 0 {
        xmlOutputBufferWriteString(buf, b">\0" as *const u8 as *const libc::c_char);
        if format != 0 && (*info).isinline == 0 && !((*cur).next).is_null() {
            if (*(*cur).next).type_0 as libc::c_uint
                != XML_TEXT_NODE as libc::c_int as libc::c_uint
                && (*(*cur).next).type_0 as libc::c_uint
                    != XML_ENTITY_REF_NODE as libc::c_int as libc::c_uint
                && !((*cur).parent).is_null() && !((*(*cur).parent).name).is_null()
                && *((*(*cur).parent).name).offset(0 as libc::c_int as isize)
                    as libc::c_int != 'p' as i32
            {
                xmlOutputBufferWriteString(
                    buf,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return;
    }
    if ((*cur).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        || ((*cur).content).is_null()) && ((*cur).children).is_null()
    {
        if !info.is_null() && (*info).saveEndTag as libc::c_int != 0 as libc::c_int
            && xmlStrcmp(
                (*info).name as *mut xmlChar,
                b"html\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            && xmlStrcmp(
                (*info).name as *mut xmlChar,
                b"body\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
        {
            xmlOutputBufferWriteString(buf, b">\0" as *const u8 as *const libc::c_char);
        } else {
            xmlOutputBufferWriteString(
                buf,
                b"></\0" as *const u8 as *const libc::c_char,
            );
            if !((*cur).ns).is_null() && !((*(*cur).ns).prefix).is_null() {
                xmlOutputBufferWriteString(
                    buf,
                    (*(*cur).ns).prefix as *const libc::c_char,
                );
                xmlOutputBufferWriteString(
                    buf,
                    b":\0" as *const u8 as *const libc::c_char,
                );
            }
            xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
            xmlOutputBufferWriteString(buf, b">\0" as *const u8 as *const libc::c_char);
        }
        if format != 0 && !((*cur).next).is_null() && !info.is_null()
            && (*info).isinline == 0
        {
            if (*(*cur).next).type_0 as libc::c_uint
                != XML_TEXT_NODE as libc::c_int as libc::c_uint
                && (*(*cur).next).type_0 as libc::c_uint
                    != XML_ENTITY_REF_NODE as libc::c_int as libc::c_uint
                && !((*cur).parent).is_null() && !((*(*cur).parent).name).is_null()
                && *((*(*cur).parent).name).offset(0 as libc::c_int as isize)
                    as libc::c_int != 'p' as i32
            {
                xmlOutputBufferWriteString(
                    buf,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return;
    }
    xmlOutputBufferWriteString(buf, b">\0" as *const u8 as *const libc::c_char);
    if (*cur).type_0 as libc::c_uint != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && !((*cur).content).is_null()
    {
        xmlOutputBufferWriteString(buf, (*cur).content as *const libc::c_char);
    }
    if !((*cur).children).is_null() {
        if format != 0 && !info.is_null() && (*info).isinline == 0
            && (*(*cur).children).type_0 as libc::c_uint
                != XML_TEXT_NODE as libc::c_int as libc::c_uint
            && (*(*cur).children).type_0 as libc::c_uint
                != XML_ENTITY_REF_NODE as libc::c_int as libc::c_uint
            && (*cur).children != (*cur).last && !((*cur).name).is_null()
            && *((*cur).name).offset(0 as libc::c_int as isize) as libc::c_int
                != 'p' as i32
        {
            xmlOutputBufferWriteString(buf, b"\n\0" as *const u8 as *const libc::c_char);
        }
        htmlNodeListDumpOutput(buf, doc, (*cur).children, encoding, format);
        if format != 0 && !info.is_null() && (*info).isinline == 0
            && (*(*cur).last).type_0 as libc::c_uint
                != XML_TEXT_NODE as libc::c_int as libc::c_uint
            && (*(*cur).last).type_0 as libc::c_uint
                != XML_ENTITY_REF_NODE as libc::c_int as libc::c_uint
            && (*cur).children != (*cur).last && !((*cur).name).is_null()
            && *((*cur).name).offset(0 as libc::c_int as isize) as libc::c_int
                != 'p' as i32
        {
            xmlOutputBufferWriteString(buf, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    xmlOutputBufferWriteString(buf, b"</\0" as *const u8 as *const libc::c_char);
    if !((*cur).ns).is_null() && !((*(*cur).ns).prefix).is_null() {
        xmlOutputBufferWriteString(buf, (*(*cur).ns).prefix as *const libc::c_char);
        xmlOutputBufferWriteString(buf, b":\0" as *const u8 as *const libc::c_char);
    }
    xmlOutputBufferWriteString(buf, (*cur).name as *const libc::c_char);
    xmlOutputBufferWriteString(buf, b">\0" as *const u8 as *const libc::c_char);
    if format != 0 && !info.is_null() && (*info).isinline == 0
        && !((*cur).next).is_null()
    {
        if (*(*cur).next).type_0 as libc::c_uint
            != XML_TEXT_NODE as libc::c_int as libc::c_uint
            && (*(*cur).next).type_0 as libc::c_uint
                != XML_ENTITY_REF_NODE as libc::c_int as libc::c_uint
            && !((*cur).parent).is_null() && !((*(*cur).parent).name).is_null()
            && *((*(*cur).parent).name).offset(0 as libc::c_int as isize) as libc::c_int
                != 'p' as i32
        {
            xmlOutputBufferWriteString(buf, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const libc::c_char,
) {
    htmlNodeDumpFormatOutput(buf, doc, cur, encoding, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocContentDumpFormatOutput(
    mut buf: xmlOutputBufferPtr,
    mut cur: xmlDocPtr,
    mut encoding: *const libc::c_char,
    mut format: libc::c_int,
) {
    let mut type_0: libc::c_int = 0;
    xmlInitParser();
    if buf.is_null() || cur.is_null() {
        return;
    }
    type_0 = (*cur).type_0 as libc::c_int;
    (*cur).type_0 = XML_HTML_DOCUMENT_NODE;
    if !((*cur).intSubset).is_null() {
        htmlDtdDumpOutput(buf, cur, 0 as *const libc::c_char);
    }
    if !((*cur).children).is_null() {
        htmlNodeListDumpOutput(buf, cur, (*cur).children, encoding, format);
    }
    xmlOutputBufferWriteString(buf, b"\n\0" as *const u8 as *const libc::c_char);
    (*cur).type_0 = type_0 as xmlElementType;
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocContentDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut cur: xmlDocPtr,
    mut encoding: *const libc::c_char,
) {
    htmlDocContentDumpFormatOutput(buf, cur, encoding, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocDump(
    mut f: *mut FILE,
    mut cur: xmlDocPtr,
) -> libc::c_int {
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    xmlInitParser();
    if cur.is_null() || f.is_null() {
        return -(1 as libc::c_int);
    }
    encoding = htmlGetMetaEncoding(cur) as *const libc::c_char;
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as libc::c_int != (*cur).charset {
            if (*cur).charset != XML_CHAR_ENCODING_UTF8 as libc::c_int {
                return -(1 as libc::c_int);
            }
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as libc::c_int,
                    0 as xmlNodePtr,
                    encoding,
                );
            }
        } else {
            handler = xmlFindCharEncodingHandler(encoding);
        }
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"HTML\0" as *const u8 as *const libc::c_char,
        );
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"ascii\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = xmlOutputBufferCreateFile(f, handler);
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    htmlDocContentDumpOutput(buf, cur, 0 as *const libc::c_char);
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn htmlSaveFile(
    mut filename: *const libc::c_char,
    mut cur: xmlDocPtr,
) -> libc::c_int {
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    if cur.is_null() || filename.is_null() {
        return -(1 as libc::c_int);
    }
    xmlInitParser();
    encoding = htmlGetMetaEncoding(cur) as *const libc::c_char;
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as libc::c_int != (*cur).charset {
            if (*cur).charset != XML_CHAR_ENCODING_UTF8 as libc::c_int {
                return -(1 as libc::c_int);
            }
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as libc::c_int,
                    0 as xmlNodePtr,
                    encoding,
                );
            }
        }
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"HTML\0" as *const u8 as *const libc::c_char,
        );
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"ascii\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = xmlOutputBufferCreateFilename(filename, handler, (*cur).compression);
    if buf.is_null() {
        return 0 as libc::c_int;
    }
    htmlDocContentDumpOutput(buf, cur, 0 as *const libc::c_char);
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn htmlSaveFileFormat(
    mut filename: *const libc::c_char,
    mut cur: xmlDocPtr,
    mut encoding: *const libc::c_char,
    mut format: libc::c_int,
) -> libc::c_int {
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    let mut ret: libc::c_int = 0;
    if cur.is_null() || filename.is_null() {
        return -(1 as libc::c_int);
    }
    xmlInitParser();
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as libc::c_int != (*cur).charset {
            if (*cur).charset != XML_CHAR_ENCODING_UTF8 as libc::c_int {
                return -(1 as libc::c_int);
            }
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as libc::c_int,
                    0 as xmlNodePtr,
                    encoding,
                );
            }
        }
        htmlSetMetaEncoding(cur, encoding as *const xmlChar);
    } else {
        htmlSetMetaEncoding(
            cur,
            b"UTF-8\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"HTML\0" as *const u8 as *const libc::c_char,
        );
    }
    if handler.is_null() {
        handler = xmlFindCharEncodingHandler(
            b"ascii\0" as *const u8 as *const libc::c_char,
        );
    }
    buf = xmlOutputBufferCreateFilename(filename, handler, 0 as libc::c_int);
    if buf.is_null() {
        return 0 as libc::c_int;
    }
    htmlDocContentDumpFormatOutput(buf, cur, encoding, format);
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn htmlSaveFileEnc(
    mut filename: *const libc::c_char,
    mut cur: xmlDocPtr,
    mut encoding: *const libc::c_char,
) -> libc::c_int {
    return htmlSaveFileFormat(filename, cur, encoding, 1 as libc::c_int);
}
