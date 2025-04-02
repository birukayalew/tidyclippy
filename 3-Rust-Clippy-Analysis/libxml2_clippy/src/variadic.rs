use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchema;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
    pub type _xmlTextWriter;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrlen(str: *const xmlChar) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
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
    fn __xmlIOErr(domain: libc::c_int, code: libc::c_int, extra: *const libc::c_char);
    fn xmlGetLineNo(node: *const xmlNode) -> libc::c_long;
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn xmlParserPrintFileInfo(input: xmlParserInputPtr);
    fn xmlParserPrintFileContext(input: xmlParserInputPtr);
    fn xmlResetError(err: xmlErrorPtr);
    fn xmlCopyError(from: xmlErrorPtr, to: xmlErrorPtr) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlLastError() -> *mut xmlError;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn __xmlStructuredErrorContext() -> *mut *mut libc::c_void;
    fn __xmlGetWarningsDefaultValue() -> *mut libc::c_int;
    fn xmlReportError(
        err: xmlErrorPtr,
        ctxt: xmlParserCtxtPtr,
        str: *const libc::c_char,
        channel: xmlGenericErrorFunc,
        data: *mut libc::c_void,
    );
    fn xmlTextReaderGenericError(
        ctxt: *mut libc::c_void,
        severity: xmlParserSeverities,
        str: *mut libc::c_char,
    );
    fn xmlNanoFTPReadResponse(ctx: *mut libc::c_void) -> libc::c_int;
    fn xmlTextWriterWriteVFormatComment(
        writer: xmlTextWriterPtr,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatElementNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatRaw(
        writer: xmlTextWriterPtr,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatString(
        writer: xmlTextWriterPtr,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatAttributeNS(
        writer: xmlTextWriterPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        namespaceURI: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatPI(
        writer: xmlTextWriterPtr,
        target: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatCDATA(
        writer: xmlTextWriterPtr,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatDTD(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        pubid: *const xmlChar,
        sysid: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatDTDElement(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatDTDAttlist(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlTextWriterWriteVFormatDTDInternalEntity(
        writer: xmlTextWriterPtr,
        pe: libc::c_int,
        name: *const xmlChar,
        format: *const libc::c_char,
        argptr: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn xmlWriterErrMsg(
        ctxt: xmlTextWriterPtr,
        error: xmlParserErrors,
        msg: *const libc::c_char,
    );
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmlNanoFTPGetResponse(ctx: *mut libc::c_void) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
pub type xmlChar = libc::c_uchar;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub struct _xmlParserInputBuffer {
    pub context: *mut libc::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: libc::c_int,
    pub error: libc::c_int,
    pub rawconsumed: libc::c_ulong,
}
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
pub type xmlInputCloseCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type xmlInputReadCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const libc::c_char,
    pub directory: *const libc::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: libc::c_int,
    pub line: libc::c_int,
    pub col: libc::c_int,
    pub consumed: libc::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: libc::c_int,
    pub id: libc::c_int,
}
pub type xmlParserInputDeallocate = Option::<unsafe extern "C" fn(*mut xmlChar) -> ()>;
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut libc::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: libc::c_int,
    pub replaceEntities: libc::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: libc::c_int,
    pub html: libc::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: libc::c_int,
    pub inputMax: libc::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: libc::c_int,
    pub nodeMax: libc::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: libc::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: libc::c_int,
    pub hasExternalSubset: libc::c_int,
    pub hasPErefs: libc::c_int,
    pub external: libc::c_int,
    pub valid: libc::c_int,
    pub validate: libc::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: libc::c_int,
    pub directory: *mut libc::c_char,
    pub name: *const xmlChar,
    pub nameNr: libc::c_int,
    pub nameMax: libc::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: libc::c_long,
    pub checkIndex: libc::c_long,
    pub keepBlanks: libc::c_int,
    pub disableSAX: libc::c_int,
    pub inSubset: libc::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut libc::c_int,
    pub spaceNr: libc::c_int,
    pub spaceMax: libc::c_int,
    pub spaceTab: *mut libc::c_int,
    pub depth: libc::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: libc::c_int,
    pub nodelen: libc::c_int,
    pub nodemem: libc::c_int,
    pub pedantic: libc::c_int,
    pub _private: *mut libc::c_void,
    pub loadsubset: libc::c_int,
    pub linenumbers: libc::c_int,
    pub catalogs: *mut libc::c_void,
    pub recovery: libc::c_int,
    pub progressive: libc::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: libc::c_int,
    pub docdict: libc::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: libc::c_int,
    pub nsNr: libc::c_int,
    pub nsMax: libc::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut libc::c_int,
    pub pushTab: *mut *mut libc::c_void,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: libc::c_int,
    pub options: libc::c_int,
    pub dictNames: libc::c_int,
    pub freeElemsNr: libc::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: libc::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: libc::c_ulong,
    pub sizeentities: libc::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: libc::c_int,
    pub nodeInfoMax: libc::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: libc::c_int,
    pub sizeentcopy: libc::c_ulong,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: libc::c_ulong,
    pub begin_line: libc::c_ulong,
    pub end_pos: libc::c_ulong,
    pub end_line: libc::c_ulong,
}
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
pub type xmlParserMode = libc::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: libc::c_int,
    pub code: libc::c_int,
    pub message: *mut libc::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut libc::c_char,
    pub line: libc::c_int,
    pub str1: *mut libc::c_char,
    pub str2: *mut libc::c_char,
    pub str3: *mut libc::c_char,
    pub int1: libc::c_int,
    pub int2: libc::c_int,
    pub ctxt: *mut libc::c_void,
    pub node: *mut libc::c_void,
}
pub type xmlErrorLevel = libc::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputState = libc::c_int;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut libc::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: libc::c_int,
    pub nodeMax: libc::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub finishDtd: libc::c_uint,
    pub doc: xmlDocPtr,
    pub valid: libc::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: libc::c_int,
    pub vstateMax: libc::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: libc::c_ulong,
    pub length: libc::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: libc::c_uint,
    pub _private: *mut libc::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlStructuredErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
>;
pub type xmlErrorPtr = *mut xmlError;
pub type endElementNsSAX2Func = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type startElementNsSAX2Func = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        libc::c_int,
        *mut *const xmlChar,
        libc::c_int,
        libc::c_int,
        *mut *const xmlChar,
    ) -> (),
>;
pub type externalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type cdataBlockSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, libc::c_int) -> (),
>;
pub type getParameterEntitySAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> xmlEntityPtr,
>;
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: libc::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: libc::c_int,
    pub checked: libc::c_int,
}
pub type xmlEntityType = libc::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type errorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type warningSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type commentSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type processingInstructionSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, *const xmlChar) -> (),
>;
pub type ignorableWhitespaceSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, libc::c_int) -> (),
>;
pub type charactersSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, libc::c_int) -> (),
>;
pub type referenceSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type endElementSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type startElementSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, *mut *const xmlChar) -> (),
>;
pub type endDocumentSAXFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type startDocumentSAXFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, xmlSAXLocatorPtr) -> (),
>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar>,
    pub getSystemId: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar>,
    pub getLineNumber: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub getColumnNumber: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    >,
}
pub type unparsedEntityDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type elementDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        libc::c_int,
        xmlElementContentPtr,
    ) -> (),
>;
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
pub type xmlElementContentOccur = libc::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = libc::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        libc::c_int,
        libc::c_int,
        *const xmlChar,
        xmlEnumerationPtr,
    ) -> (),
>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type entityDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        libc::c_int,
        *const xmlChar,
        *const xmlChar,
        *mut xmlChar,
    ) -> (),
>;
pub type getEntitySAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> xmlEntityPtr,
>;
pub type resolveEntitySAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type hasExternalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type hasInternalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type isStandaloneSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type internalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
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
pub type xmlParserErrors = libc::c_uint;
pub const XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub const XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const XML_IO_ESRCH: xmlParserErrors = 1540;
pub const XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const XML_IO_EROFS: xmlParserErrors = 1538;
pub const XML_IO_ERANGE: xmlParserErrors = 1537;
pub const XML_IO_EPIPE: xmlParserErrors = 1536;
pub const XML_IO_EPERM: xmlParserErrors = 1535;
pub const XML_IO_ENXIO: xmlParserErrors = 1534;
pub const XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const XML_IO_ENOENT: xmlParserErrors = 1524;
pub const XML_IO_ENODEV: xmlParserErrors = 1523;
pub const XML_IO_ENFILE: xmlParserErrors = 1522;
pub const XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const XML_IO_EMLINK: xmlParserErrors = 1519;
pub const XML_IO_EMFILE: xmlParserErrors = 1518;
pub const XML_IO_EISDIR: xmlParserErrors = 1517;
pub const XML_IO_EIO: xmlParserErrors = 1516;
pub const XML_IO_EINVAL: xmlParserErrors = 1515;
pub const XML_IO_EINTR: xmlParserErrors = 1514;
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const XML_IO_EFBIG: xmlParserErrors = 1512;
pub const XML_IO_EFAULT: xmlParserErrors = 1511;
pub const XML_IO_EEXIST: xmlParserErrors = 1510;
pub const XML_IO_EDOM: xmlParserErrors = 1509;
pub const XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const XML_IO_ECHILD: xmlParserErrors = 1507;
pub const XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const XML_IO_EBUSY: xmlParserErrors = 1505;
pub const XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const XML_IO_EBADF: xmlParserErrors = 1503;
pub const XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const XML_IO_EACCES: xmlParserErrors = 1501;
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const XML_WAR_NS_URI: xmlParserErrors = 99;
pub const XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaSAXPlug {
    pub magic: libc::c_uint,
    pub user_sax_ptr: *mut xmlSAXHandlerPtr,
    pub user_sax: xmlSAXHandlerPtr,
    pub user_data_ptr: *mut *mut libc::c_void,
    pub user_data: *mut libc::c_void,
    pub schemas_sax: xmlSAXHandler,
    pub ctxt: xmlSchemaValidCtxtPtr,
}
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
pub type xmlParserSeverities = libc::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextReader {
    pub mode: libc::c_int,
    pub doc: xmlDocPtr,
    pub validate: xmlTextReaderValidate,
    pub allocs: libc::c_int,
    pub state: xmlTextReaderState,
    pub ctxt: xmlParserCtxtPtr,
    pub sax: xmlSAXHandlerPtr,
    pub input: xmlParserInputBufferPtr,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub characters: charactersSAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub base: libc::c_uint,
    pub cur: libc::c_uint,
    pub node: xmlNodePtr,
    pub curnode: xmlNodePtr,
    pub depth: libc::c_int,
    pub faketext: xmlNodePtr,
    pub preserve: libc::c_int,
    pub buffer: xmlBufPtr,
    pub dict: xmlDictPtr,
    pub ent: xmlNodePtr,
    pub entNr: libc::c_int,
    pub entMax: libc::c_int,
    pub entTab: *mut xmlNodePtr,
    pub errorFunc: xmlTextReaderErrorFunc,
    pub errorFuncArg: *mut libc::c_void,
    pub rngSchemas: xmlRelaxNGPtr,
    pub rngValidCtxt: xmlRelaxNGValidCtxtPtr,
    pub rngPreserveCtxt: libc::c_int,
    pub rngValidErrors: libc::c_int,
    pub rngFullNode: xmlNodePtr,
    pub xsdSchemas: xmlSchemaPtr,
    pub xsdValidCtxt: xmlSchemaValidCtxtPtr,
    pub xsdPreserveCtxt: libc::c_int,
    pub xsdValidErrors: libc::c_int,
    pub xsdPlug: xmlSchemaSAXPlugPtr,
    pub xinclude: libc::c_int,
    pub xinclude_name: *const xmlChar,
    pub xincctxt: xmlXIncludeCtxtPtr,
    pub in_xinclude: libc::c_int,
    pub patternNr: libc::c_int,
    pub patternMax: libc::c_int,
    pub patternTab: *mut xmlPatternPtr,
    pub preserves: libc::c_int,
    pub parserFlags: libc::c_int,
    pub sErrorFunc: xmlStructuredErrorFunc,
}
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlTextReaderErrorFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        xmlParserSeverities,
        xmlTextReaderLocatorPtr,
    ) -> (),
>;
pub type xmlTextReaderLocatorPtr = *mut libc::c_void;
pub type xmlTextReaderState = libc::c_int;
pub const XML_TEXTREADER_ERROR: xmlTextReaderState = 6;
pub const XML_TEXTREADER_DONE: xmlTextReaderState = 5;
pub const XML_TEXTREADER_BACKTRACK: xmlTextReaderState = 4;
pub const XML_TEXTREADER_EMPTY: xmlTextReaderState = 3;
pub const XML_TEXTREADER_END: xmlTextReaderState = 2;
pub const XML_TEXTREADER_ELEMENT: xmlTextReaderState = 1;
pub const XML_TEXTREADER_START: xmlTextReaderState = 0;
pub const XML_TEXTREADER_NONE: xmlTextReaderState = -1;
pub type xmlTextReaderValidate = libc::c_uint;
pub const XML_TEXTREADER_VALIDATE_XSD: xmlTextReaderValidate = 4;
pub const XML_TEXTREADER_VALIDATE_RNG: xmlTextReaderValidate = 2;
pub const XML_TEXTREADER_VALIDATE_DTD: xmlTextReaderValidate = 1;
pub const XML_TEXTREADER_NOT_VALIDATE: xmlTextReaderValidate = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub type xmlTextWriter = _xmlTextWriter;
pub type xmlTextWriterPtr = *mut xmlTextWriter;
pub type xmlNanoFTPCtxtPtr = *mut xmlNanoFTPCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNanoFTPCtxt {
    pub protocol: *mut libc::c_char,
    pub hostname: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub ftpAddr: sockaddr_storage,
    pub passive: libc::c_int,
    pub controlFd: libc::c_int,
    pub dataFd: libc::c_int,
    pub state: libc::c_int,
    pub returnValue: libc::c_int,
    pub controlBuf: [libc::c_char; 1025],
    pub controlBufIndex: libc::c_int,
    pub controlBufUsed: libc::c_int,
    pub controlBufAnswer: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type sa_family_t = libc::c_ushort;
pub type xmllintReturnCode = libc::c_uint;
pub const XMLLINT_ERR_XPATH: xmllintReturnCode = 10;
pub const XMLLINT_ERR_MEM: xmllintReturnCode = 9;
pub const XMLLINT_ERR_RDREGIS: xmllintReturnCode = 8;
pub const XMLLINT_ERR_SCHEMAPAT: xmllintReturnCode = 7;
pub const XMLLINT_ERR_OUT: xmllintReturnCode = 6;
pub const XMLLINT_ERR_SCHEMACOMP: xmllintReturnCode = 5;
pub const XMLLINT_ERR_RDFILE: xmllintReturnCode = 4;
pub const XMLLINT_ERR_VALID: xmllintReturnCode = 3;
pub const XMLLINT_ERR_DTD: xmllintReturnCode = 2;
pub const XMLLINT_ERR_UNCLASS: xmllintReturnCode = 1;
pub const XMLLINT_RETURN_OK: xmllintReturnCode = 0;
#[no_mangle]
pub static mut xmllint_callbacks: libc::c_int = 0;
#[no_mangle]
pub static mut xmllint_buffer: [libc::c_char; 50000] = [0; 50000];
#[no_mangle]
pub static mut xmllint_progresult: xmllintReturnCode = XMLLINT_RETURN_OK;
#[no_mangle]
pub static mut xmllint_noout: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut xmllint_begin: timeval = timeval { tv_sec: 0, tv_usec: 0 };
#[no_mangle]
pub static mut xmllint_end: timeval = timeval { tv_sec: 0, tv_usec: 0 };
#[no_mangle]
pub unsafe extern "C" fn xmllint_endTimer(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msec: libc::c_long = 0;
    let mut ap: ::core::ffi::VaListImpl;
    gettimeofday(&mut xmllint_end, 0 as *mut libc::c_void);
    msec = xmllint_end.tv_sec - xmllint_begin.tv_sec;
    msec *= 1000 as libc::c_int as libc::c_long;
    msec
        += (xmllint_end.tv_usec - xmllint_begin.tv_usec)
            / 1000 as libc::c_int as __suseconds_t;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b" took %ld ms\n\0" as *const u8 as *const libc::c_char, msec);
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLPrintFileContext(mut input: xmlParserInputPtr) {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut base: *const xmlChar = 0 as *const xmlChar;
    let mut len: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if input.is_null() {
        return;
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"<pre>\n\0" as *const u8 as *const libc::c_char);
    cur = (*input).cur;
    base = (*input).base;
    while cur > base
        && (*cur as libc::c_int == '\n' as i32 || *cur as libc::c_int == '\r' as i32)
    {
        cur = cur.offset(-1);
        cur;
    }
    n = 0 as libc::c_int;
    loop {
        let fresh0 = n;
        n = n + 1;
        if !(fresh0 < 80 as libc::c_int && cur > base
            && *cur as libc::c_int != '\n' as i32 && *cur as libc::c_int != '\r' as i32)
        {
            break;
        }
        cur = cur.offset(-1);
        cur;
    }
    if *cur as libc::c_int == '\n' as i32 || *cur as libc::c_int == '\r' as i32 {
        cur = cur.offset(1);
        cur;
    }
    base = cur;
    n = 0 as libc::c_int;
    while *cur as libc::c_int != 0 as libc::c_int && *cur as libc::c_int != '\n' as i32
        && *cur as libc::c_int != '\r' as i32 && n < 79 as libc::c_int
    {
        len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
        let fresh1 = cur;
        cur = cur.offset(1);
        snprintf(
            &mut *xmllint_buffer.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
                .wrapping_sub(len as libc::c_ulong),
            b"%c\0" as *const u8 as *const libc::c_char,
            *fresh1 as libc::c_int,
        );
        n += 1;
        n;
    }
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    snprintf(
        &mut *xmllint_buffer.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    cur = (*input).cur;
    while *cur as libc::c_int == '\n' as i32 || *cur as libc::c_int == '\r' as i32 {
        cur = cur.offset(-1);
        cur;
    }
    n = 0 as libc::c_int;
    while cur != base
        && {
            let fresh2 = n;
            n = n + 1;
            fresh2 < 80 as libc::c_int
        }
    {
        len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
        snprintf(
            &mut *xmllint_buffer.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
                .wrapping_sub(len as libc::c_ulong),
            b" \0" as *const u8 as *const libc::c_char,
        );
        base = base.offset(1);
        base;
    }
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    snprintf(
        &mut *xmllint_buffer.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        b"^\n\0" as *const u8 as *const libc::c_char,
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"</pre>\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLEncodeSend() {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    result = xmlEncodeEntitiesReentrant(
        0 as xmlDocPtr,
        xmllint_buffer.as_mut_ptr() as *mut xmlChar,
    ) as *mut libc::c_char;
    if !result.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"%s\0" as *const u8 as *const libc::c_char,
            result,
        );
        xmlFree.expect("non-null function pointer")(result as *mut libc::c_void);
    }
    xmllint_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLPrintFileInfo(mut input: xmlParserInputPtr) {
    let mut len: libc::c_int = 0;
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"<p>\0" as *const u8 as *const libc::c_char);
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    if !input.is_null() {
        if !((*input).filename).is_null() {
            snprintf(
                &mut *xmllint_buffer.as_mut_ptr().offset(len as isize)
                    as *mut libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
                b"%s:%d: \0" as *const u8 as *const libc::c_char,
                (*input).filename,
                (*input).line,
            );
        } else {
            snprintf(
                &mut *xmllint_buffer.as_mut_ptr().offset(len as isize)
                    as *mut libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
                    .wrapping_sub(len as libc::c_ulong),
                b"Entity: line %d: \0" as *const u8 as *const libc::c_char,
                (*input).line,
            );
        }
    }
    xmlHTMLEncodeSend();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLError(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    xmllint_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    input = (*ctxt).input;
    if !input.is_null() && ((*input).filename).is_null()
        && (*ctxt).inputNr > 1 as libc::c_int
    {
        input = *((*ctxt).inputTab)
            .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(
        *__xmlGenericErrorContext(),
        b"<b>error</b>: \0" as *const u8 as *const libc::c_char,
    );
    args_0 = args.clone();
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    vsnprintf(
        &mut *xmllint_buffer.as_mut_ptr().offset(len as isize),
        (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"</p>\n\0" as *const u8 as *const libc::c_char);
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLWarning(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    xmllint_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    input = (*ctxt).input;
    if !input.is_null() && ((*input).filename).is_null()
        && (*ctxt).inputNr > 1 as libc::c_int
    {
        input = *((*ctxt).inputTab)
            .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(
        *__xmlGenericErrorContext(),
        b"<b>warning</b>: \0" as *const u8 as *const libc::c_char,
    );
    args_0 = args.clone();
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    vsnprintf(
        &mut *xmllint_buffer.as_mut_ptr().offset(len as isize),
        (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"</p>\n\0" as *const u8 as *const libc::c_char);
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLValidityError(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    xmllint_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    input = (*ctxt).input;
    if ((*input).filename).is_null() && (*ctxt).inputNr > 1 as libc::c_int {
        input = *((*ctxt).inputTab)
            .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(
        *__xmlGenericErrorContext(),
        b"<b>validity error</b>: \0" as *const u8 as *const libc::c_char,
    );
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    args_0 = args.clone();
    vsnprintf(
        &mut *xmllint_buffer.as_mut_ptr().offset(len as isize),
        (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"</p>\n\0" as *const u8 as *const libc::c_char);
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
    xmllint_progresult = XMLLINT_ERR_VALID;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHTMLValidityWarning(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    xmllint_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    input = (*ctxt).input;
    if ((*input).filename).is_null() && (*ctxt).inputNr > 1 as libc::c_int {
        input = *((*ctxt).inputTab)
            .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(
        *__xmlGenericErrorContext(),
        b"<b>validity warning</b>: \0" as *const u8 as *const libc::c_char,
    );
    args_0 = args.clone();
    len = strlen(xmllint_buffer.as_mut_ptr()) as libc::c_int;
    vsnprintf(
        &mut *xmllint_buffer.as_mut_ptr().offset(len as isize),
        (::core::mem::size_of::<[libc::c_char; 50000]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"</p>\n\0" as *const u8 as *const libc::c_char);
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}
#[no_mangle]
pub unsafe extern "C" fn xmllint_warningDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(stdout, b"SAX.warning: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn xmllint_errorDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(stdout, b"SAX.error: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn xmllint_fatalErrorDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(stdout, b"SAX.fatalError: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn xmlGenericErrorDefaultFunc(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    if (*__xmlGenericErrorContext()).is_null() {
        let ref mut fresh3 = *__xmlGenericErrorContext();
        *fresh3 = stderr as *mut libc::c_void;
    }
    args_0 = args.clone();
    vfprintf(*__xmlGenericErrorContext() as *mut FILE, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRaiseError(
    mut schannel: xmlStructuredErrorFunc,
    mut channel: xmlGenericErrorFunc,
    mut data: *mut libc::c_void,
    mut ctx: *mut libc::c_void,
    mut nod: *mut libc::c_void,
    mut domain: libc::c_int,
    mut code: libc::c_int,
    mut level: xmlErrorLevel,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut str1: *const libc::c_char,
    mut str2: *const libc::c_char,
    mut str3: *const libc::c_char,
    mut int1: libc::c_int,
    mut col: libc::c_int,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut node: xmlNodePtr = nod as xmlNodePtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut to: xmlErrorPtr = __xmlLastError();
    let mut baseptr: xmlNodePtr = 0 as xmlNodePtr;
    if code == XML_ERR_OK as libc::c_int {
        return;
    }
    if *__xmlGetWarningsDefaultValue() == 0 as libc::c_int
        && level as libc::c_uint == XML_ERR_WARNING as libc::c_int as libc::c_uint
    {
        return;
    }
    if domain == XML_FROM_PARSER as libc::c_int || domain == XML_FROM_HTML as libc::c_int
        || domain == XML_FROM_DTD as libc::c_int
        || domain == XML_FROM_NAMESPACE as libc::c_int
        || domain == XML_FROM_IO as libc::c_int
        || domain == XML_FROM_VALID as libc::c_int
    {
        ctxt = ctx as xmlParserCtxtPtr;
        if schannel.is_none() && !ctxt.is_null() && !((*ctxt).sax).is_null()
            && (*(*ctxt).sax).initialized == 0xdeedbeaf as libc::c_uint
            && ((*(*ctxt).sax).serror).is_some()
        {
            schannel = (*(*ctxt).sax).serror;
            data = (*ctxt).userData;
        }
    }
    if schannel.is_none() {
        schannel = *__xmlStructuredError();
        if schannel.is_some() {
            data = *__xmlStructuredErrorContext();
        }
    }
    if msg.is_null() {
        str = xmlStrdup(
            b"No error message provided\0" as *const u8 as *const libc::c_char
                as *mut xmlChar,
        ) as *mut libc::c_char;
    } else {
        let mut size: libc::c_int = 0;
        let mut prev_size: libc::c_int = -(1 as libc::c_int);
        let mut chars: libc::c_int = 0;
        let mut larger: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ap: ::core::ffi::VaListImpl;
        str = xmlMalloc.expect("non-null function pointer")(150 as libc::c_int as size_t)
            as *mut libc::c_char;
        if !str.is_null() {
            size = 150 as libc::c_int;
            while size < 64000 as libc::c_int {
                ap = args.clone();
                chars = vsnprintf(str, size as libc::c_ulong, msg, ap.as_va_list());
                if chars > -(1 as libc::c_int) && chars < size {
                    if prev_size == chars {
                        break;
                    }
                    prev_size = chars;
                }
                if chars > -(1 as libc::c_int) {
                    size += chars + 1 as libc::c_int;
                } else {
                    size += 100 as libc::c_int;
                }
                larger = xmlRealloc
                    .expect(
                        "non-null function pointer",
                    )(str as *mut libc::c_void, size as size_t) as *mut libc::c_char;
                if larger.is_null() {
                    break;
                }
                str = larger;
            }
        }
    }
    if !ctxt.is_null() {
        if file.is_null() {
            input = (*ctxt).input;
            if !input.is_null() && ((*input).filename).is_null()
                && (*ctxt).inputNr > 1 as libc::c_int
            {
                input = *((*ctxt).inputTab)
                    .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
            }
            if !input.is_null() {
                file = (*input).filename;
                line = (*input).line;
                col = (*input).col;
            }
        }
        to = &mut (*ctxt).lastError;
    } else if !node.is_null() && file.is_null() {
        let mut i: libc::c_int = 0;
        if !((*node).doc).is_null() && !((*(*node).doc).URL).is_null() {
            baseptr = node;
        }
        i = 0 as libc::c_int;
        while i < 10 as libc::c_int && !node.is_null()
            && (*node).type_0 as libc::c_uint
                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            node = (*node).parent;
            i += 1;
            i;
        }
        if baseptr.is_null() && !node.is_null() && !((*node).doc).is_null()
            && !((*(*node).doc).URL).is_null()
        {
            baseptr = node;
        }
        if !node.is_null()
            && (*node).type_0 as libc::c_uint
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            line = (*node).line as libc::c_int;
        }
        if line == 0 as libc::c_int || line == 65535 as libc::c_int {
            line = xmlGetLineNo(node as *const xmlNode) as libc::c_int;
        }
    }
    xmlResetError(to);
    (*to).domain = domain;
    (*to).code = code;
    (*to).message = str;
    (*to).level = level;
    if !file.is_null() {
        (*to).file = xmlStrdup(file as *const xmlChar) as *mut libc::c_char;
    } else if !baseptr.is_null() {
        let mut prev: xmlNodePtr = baseptr;
        let mut inclcount: libc::c_int = 0 as libc::c_int;
        while !prev.is_null() {
            if ((*prev).prev).is_null() {
                prev = (*prev).parent;
            } else {
                prev = (*prev).prev;
                if (*prev).type_0 as libc::c_uint
                    == XML_XINCLUDE_START as libc::c_int as libc::c_uint
                {
                    inclcount -= 1;
                    if inclcount < 0 as libc::c_int {
                        break;
                    }
                } else if (*prev).type_0 as libc::c_uint
                    == XML_XINCLUDE_END as libc::c_int as libc::c_uint
                {
                    inclcount += 1;
                    inclcount;
                }
            }
        }
        if !prev.is_null() {
            if (*prev).type_0 as libc::c_uint
                == XML_XINCLUDE_START as libc::c_int as libc::c_uint
            {
                (*prev).type_0 = XML_ELEMENT_NODE;
                (*to)
                    .file = xmlGetProp(
                    prev as *const xmlNode,
                    b"href\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) as *mut libc::c_char;
                (*prev).type_0 = XML_XINCLUDE_START;
            } else {
                (*to)
                    .file = xmlGetProp(
                    prev as *const xmlNode,
                    b"href\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) as *mut libc::c_char;
            }
        } else {
            (*to).file = xmlStrdup((*(*baseptr).doc).URL) as *mut libc::c_char;
        }
        if ((*to).file).is_null() && !node.is_null() && !((*node).doc).is_null() {
            (*to).file = xmlStrdup((*(*node).doc).URL) as *mut libc::c_char;
        }
    }
    (*to).line = line;
    if !str1.is_null() {
        (*to).str1 = xmlStrdup(str1 as *const xmlChar) as *mut libc::c_char;
    }
    if !str2.is_null() {
        (*to).str2 = xmlStrdup(str2 as *const xmlChar) as *mut libc::c_char;
    }
    if !str3.is_null() {
        (*to).str3 = xmlStrdup(str3 as *const xmlChar) as *mut libc::c_char;
    }
    (*to).int1 = int1;
    (*to).int2 = col;
    (*to).node = node as *mut libc::c_void;
    (*to).ctxt = ctx;
    if to != __xmlLastError() {
        xmlCopyError(to, __xmlLastError());
    }
    if schannel.is_some() {
        schannel.expect("non-null function pointer")(data, to);
        return;
    }
    if !ctxt.is_null() && channel.is_none() && (*__xmlStructuredError()).is_none()
        && !((*ctxt).sax).is_null()
    {
        if level as libc::c_uint == XML_ERR_WARNING as libc::c_int as libc::c_uint {
            channel = (*(*ctxt).sax).warning;
        } else {
            channel = (*(*ctxt).sax).error;
        }
        data = (*ctxt).userData;
    } else if channel.is_none() {
        channel = *__xmlGenericError();
        if !ctxt.is_null() {
            data = ctxt as *mut libc::c_void;
        } else {
            data = *__xmlGenericErrorContext();
        }
    }
    if channel.is_none() {
        return;
    }
    if channel
        == Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        )
        || channel
            == Some(
                xmlParserWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            )
        || channel
            == Some(
                xmlParserValidityError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            )
        || channel
            == Some(
                xmlParserValidityWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            )
    {
        xmlReportError(to, ctxt, str, None, 0 as *mut libc::c_void);
    } else if channel
        == ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
            >,
            xmlGenericErrorFunc,
        >(
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
        )
        || channel
            == Some(
                xmlGenericErrorDefaultFunc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            )
    {
        xmlReportError(to, ctxt, str, channel, data);
    } else {
        channel
            .expect(
                "non-null function pointer",
            )(data, b"%s\0" as *const u8 as *const libc::c_char, str);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserError(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut cur: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if !ctxt.is_null() {
        input = (*ctxt).input;
        if !input.is_null() && ((*input).filename).is_null()
            && (*ctxt).inputNr > 1 as libc::c_int
        {
            cur = input;
            input = *((*ctxt).inputTab)
                .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
        }
        xmlParserPrintFileInfo(input);
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"error: \0" as *const u8 as *const libc::c_char);
    let mut size: libc::c_int = 0;
    let mut prev_size: libc::c_int = -(1 as libc::c_int);
    let mut chars: libc::c_int = 0;
    let mut larger: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    str = xmlMalloc.expect("non-null function pointer")(150 as libc::c_int as size_t)
        as *mut libc::c_char;
    if !str.is_null() {
        size = 150 as libc::c_int;
        while size < 64000 as libc::c_int {
            ap = args.clone();
            chars = vsnprintf(str, size as libc::c_ulong, msg, ap.as_va_list());
            if chars > -(1 as libc::c_int) && chars < size {
                if prev_size == chars {
                    break;
                }
                prev_size = chars;
            }
            if chars > -(1 as libc::c_int) {
                size += chars + 1 as libc::c_int;
            } else {
                size += 100 as libc::c_int;
            }
            larger = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(str as *mut libc::c_void, size as size_t) as *mut libc::c_char;
            if larger.is_null() {
                break;
            }
            str = larger;
        }
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"%s\0" as *const u8 as *const libc::c_char, str);
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
    if !ctxt.is_null() {
        xmlParserPrintFileContext(input);
        if !cur.is_null() {
            xmlParserPrintFileInfo(cur);
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            xmlParserPrintFileContext(cur);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserWarning(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut cur: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if !ctxt.is_null() {
        input = (*ctxt).input;
        if !input.is_null() && ((*input).filename).is_null()
            && (*ctxt).inputNr > 1 as libc::c_int
        {
            cur = input;
            input = *((*ctxt).inputTab)
                .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
        }
        xmlParserPrintFileInfo(input);
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(
        *__xmlGenericErrorContext(),
        b"warning: \0" as *const u8 as *const libc::c_char,
    );
    let mut size: libc::c_int = 0;
    let mut prev_size: libc::c_int = -(1 as libc::c_int);
    let mut chars: libc::c_int = 0;
    let mut larger: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    str = xmlMalloc.expect("non-null function pointer")(150 as libc::c_int as size_t)
        as *mut libc::c_char;
    if !str.is_null() {
        size = 150 as libc::c_int;
        while size < 64000 as libc::c_int {
            ap = args.clone();
            chars = vsnprintf(str, size as libc::c_ulong, msg, ap.as_va_list());
            if chars > -(1 as libc::c_int) && chars < size {
                if prev_size == chars {
                    break;
                }
                prev_size = chars;
            }
            if chars > -(1 as libc::c_int) {
                size += chars + 1 as libc::c_int;
            } else {
                size += 100 as libc::c_int;
            }
            larger = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(str as *mut libc::c_void, size as size_t) as *mut libc::c_char;
            if larger.is_null() {
                break;
            }
            str = larger;
        }
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"%s\0" as *const u8 as *const libc::c_char, str);
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
    if !ctxt.is_null() {
        xmlParserPrintFileContext(input);
        if !cur.is_null() {
            xmlParserPrintFileInfo(cur);
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            xmlParserPrintFileContext(cur);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserValidityError(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = xmlStrlen(msg as *const xmlChar);
    static mut had_info: libc::c_int = 0 as libc::c_int;
    if len > 1 as libc::c_int
        && *msg.offset((len - 2 as libc::c_int) as isize) as libc::c_int != ':' as i32
    {
        if !ctxt.is_null() {
            input = (*ctxt).input;
            if ((*input).filename).is_null() && (*ctxt).inputNr > 1 as libc::c_int {
                input = *((*ctxt).inputTab)
                    .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
            }
            if had_info == 0 as libc::c_int {
                xmlParserPrintFileInfo(input);
            }
        }
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"validity error: \0" as *const u8 as *const libc::c_char,
        );
        had_info = 0 as libc::c_int;
    } else {
        had_info = 1 as libc::c_int;
    }
    let mut size: libc::c_int = 0;
    let mut prev_size: libc::c_int = -(1 as libc::c_int);
    let mut chars: libc::c_int = 0;
    let mut larger: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    str = xmlMalloc.expect("non-null function pointer")(150 as libc::c_int as size_t)
        as *mut libc::c_char;
    if !str.is_null() {
        size = 150 as libc::c_int;
        while size < 64000 as libc::c_int {
            ap = args.clone();
            chars = vsnprintf(str, size as libc::c_ulong, msg, ap.as_va_list());
            if chars > -(1 as libc::c_int) && chars < size {
                if prev_size == chars {
                    break;
                }
                prev_size = chars;
            }
            if chars > -(1 as libc::c_int) {
                size += chars + 1 as libc::c_int;
            } else {
                size += 100 as libc::c_int;
            }
            larger = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(str as *mut libc::c_void, size as size_t) as *mut libc::c_char;
            if larger.is_null() {
                break;
            }
            str = larger;
        }
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"%s\0" as *const u8 as *const libc::c_char, str);
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
    if !ctxt.is_null() && !input.is_null() {
        xmlParserPrintFileContext(input);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserValidityWarning(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = 0 as xmlParserInputPtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = xmlStrlen(msg as *const xmlChar);
    if !ctxt.is_null() && len != 0 as libc::c_int
        && *msg.offset((len - 1 as libc::c_int) as isize) as libc::c_int != ':' as i32
    {
        input = (*ctxt).input;
        if ((*input).filename).is_null() && (*ctxt).inputNr > 1 as libc::c_int {
            input = *((*ctxt).inputTab)
                .offset(((*ctxt).inputNr - 2 as libc::c_int) as isize);
        }
        xmlParserPrintFileInfo(input);
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(
        *__xmlGenericErrorContext(),
        b"validity warning: \0" as *const u8 as *const libc::c_char,
    );
    let mut size: libc::c_int = 0;
    let mut prev_size: libc::c_int = -(1 as libc::c_int);
    let mut chars: libc::c_int = 0;
    let mut larger: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    str = xmlMalloc.expect("non-null function pointer")(150 as libc::c_int as size_t)
        as *mut libc::c_char;
    if !str.is_null() {
        size = 150 as libc::c_int;
        while size < 64000 as libc::c_int {
            ap = args.clone();
            chars = vsnprintf(str, size as libc::c_ulong, msg, ap.as_va_list());
            if chars > -(1 as libc::c_int) && chars < size {
                if prev_size == chars {
                    break;
                }
                prev_size = chars;
            }
            if chars > -(1 as libc::c_int) {
                size += chars + 1 as libc::c_int;
            } else {
                size += 100 as libc::c_int;
            }
            larger = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(str as *mut libc::c_void, size as size_t) as *mut libc::c_char;
            if larger.is_null() {
                break;
            }
            str = larger;
        }
    }
    (*__xmlGenericError())
        .expect(
            "non-null function pointer",
        )(*__xmlGenericErrorContext(), b"%s\0" as *const u8 as *const libc::c_char, str);
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
    if !ctxt.is_null() {
        xmlParserPrintFileContext(input);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNoValidityErr(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValidityErrorRelay(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if ((*reader).errorFunc).is_none() {
        xmlTextReaderValidityError(
            ctx,
            b"%s\0" as *const u8 as *const libc::c_char,
            str,
        );
    } else {
        ((*reader).errorFunc)
            .expect(
                "non-null function pointer",
            )(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            0 as *mut libc::c_void,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValidityWarningRelay(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if ((*reader).errorFunc).is_none() {
        xmlTextReaderValidityWarning(
            ctx,
            b"%s\0" as *const u8 as *const libc::c_char,
            str,
        );
    } else {
        ((*reader).errorFunc)
            .expect(
                "non-null function pointer",
            )(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            0 as *mut libc::c_void,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderError(
    mut ctxt: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    xmlTextReaderGenericError(
        ctxt,
        XML_PARSER_SEVERITY_ERROR,
        xmlTextReaderBuildMessage(msg, ap.as_va_list()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderWarning(
    mut ctxt: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    xmlTextReaderGenericError(
        ctxt,
        XML_PARSER_SEVERITY_WARNING,
        xmlTextReaderBuildMessage(msg, ap.as_va_list()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValidityError(
    mut ctxt: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = xmlStrlen(msg as *const xmlChar);
    if len > 1 as libc::c_int
        && *msg.offset((len - 2 as libc::c_int) as isize) as libc::c_int != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValidityWarning(
    mut ctxt: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = xmlStrlen(msg as *const xmlChar);
    if len != 0 as libc::c_int
        && *msg.offset((len - 1 as libc::c_int) as isize) as libc::c_int != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderBuildMessage(
    mut msg: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut chars: libc::c_int = 0;
    let mut larger: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aq: ::core::ffi::VaListImpl;
    loop {
        aq = ap.clone();
        chars = vsnprintf(str, size as libc::c_ulong, msg, aq.as_va_list());
        if chars < 0 as libc::c_int {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"vsnprintf failed !\n\0" as *const u8 as *const libc::c_char,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
            }
            return 0 as *mut libc::c_char;
        }
        if chars < size || size == 64000 as libc::c_int {
            break;
        }
        if chars < 64000 as libc::c_int {
            size = chars + 1 as libc::c_int;
        } else {
            size = 64000 as libc::c_int;
        }
        larger = xmlRealloc
            .expect(
                "non-null function pointer",
            )(str as *mut libc::c_void, size as size_t) as *mut libc::c_char;
        if larger.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const libc::c_char,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
            }
            return 0 as *mut libc::c_char;
        }
        str = larger;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn warningSplit(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlSchemaSAXPlugPtr = ctx as xmlSchemaSAXPlugPtr;
    if !ctxt.is_null() && !((*ctxt).user_sax).is_null()
        && ((*(*ctxt).user_sax).warning).is_some()
    {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"variadic.c\0" as *const u8 as *const libc::c_char,
            834 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn errorSplit(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlSchemaSAXPlugPtr = ctx as xmlSchemaSAXPlugPtr;
    if !ctxt.is_null() && !((*ctxt).user_sax).is_null()
        && ((*(*ctxt).user_sax).error).is_some()
    {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"variadic.c\0" as *const u8 as *const libc::c_char,
            842 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn fatalErrorSplit(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ctxt: xmlSchemaSAXPlugPtr = ctx as xmlSchemaSAXPlugPtr;
    if !ctxt.is_null() && !((*ctxt).user_sax).is_null()
        && ((*(*ctxt).user_sax).fatalError).is_some()
    {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"variadic.c\0" as *const u8 as *const libc::c_char,
            850 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrPrintf(
    mut buf: *mut xmlChar,
    mut len: libc::c_int,
    mut msg: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    if buf.is_null() || msg.is_null() {
        return -(1 as libc::c_int);
    }
    args_0 = args.clone();
    ret = vsnprintf(
        buf as *mut libc::c_char,
        len as libc::c_ulong,
        msg,
        args_0.as_va_list(),
    );
    *buf.offset((len - 1 as libc::c_int) as isize) = 0 as libc::c_int as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatComment(
    mut writer: xmlTextWriterPtr,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatComment(writer, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatRaw(
    mut writer: xmlTextWriterPtr,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatRaw(writer, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatString(
    mut writer: xmlTextWriterPtr,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    if writer.is_null() || format.is_null() {
        return -(1 as libc::c_int);
    }
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatString(writer, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatAttribute(
    mut writer: xmlTextWriterPtr,
    mut name: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatAttribute(writer, name, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatAttributeNS(
    mut writer: xmlTextWriterPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut namespaceURI: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatAttributeNS(
        writer,
        prefix,
        name,
        namespaceURI,
        format,
        ap.as_va_list(),
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatElement(
    mut writer: xmlTextWriterPtr,
    mut name: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatElement(writer, name, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatElementNS(
    mut writer: xmlTextWriterPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut namespaceURI: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatElementNS(
        writer,
        prefix,
        name,
        namespaceURI,
        format,
        ap.as_va_list(),
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatPI(
    mut writer: xmlTextWriterPtr,
    mut target: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatPI(writer, target, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatCDATA(
    mut writer: xmlTextWriterPtr,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatCDATA(writer, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatDTD(
    mut writer: xmlTextWriterPtr,
    mut name: *const xmlChar,
    mut pubid: *const xmlChar,
    mut sysid: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatDTD(
        writer,
        name,
        pubid,
        sysid,
        format,
        ap.as_va_list(),
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatDTDElement(
    mut writer: xmlTextWriterPtr,
    mut name: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatDTDElement(writer, name, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatDTDAttlist(
    mut writer: xmlTextWriterPtr,
    mut name: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatDTDAttlist(writer, name, format, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterWriteFormatDTDInternalEntity(
    mut writer: xmlTextWriterPtr,
    mut pe: libc::c_int,
    mut name: *const xmlChar,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    rc = xmlTextWriterWriteVFormatDTDInternalEntity(
        writer,
        pe,
        name,
        format,
        ap.as_va_list(),
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextWriterVSprintf(
    mut format: *const libc::c_char,
    mut argptr: ::core::ffi::VaList,
) -> *mut xmlChar {
    let mut size: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut buf: *mut xmlChar = 0 as *mut xmlChar;
    let mut locarg: ::core::ffi::VaListImpl;
    size = 8192 as libc::c_int;
    buf = xmlMalloc.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
    if buf.is_null() {
        xmlWriterErrMsg(
            0 as xmlTextWriterPtr,
            XML_ERR_NO_MEMORY,
            b"xmlTextWriterVSprintf : out of memory!\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut xmlChar;
    }
    locarg = argptr.clone();
    loop {
        count = vsnprintf(
            buf as *mut libc::c_char,
            size as libc::c_ulong,
            format,
            locarg.as_va_list(),
        );
        if !(count < 0 as libc::c_int || count == size - 1 as libc::c_int
            || count == size || count > size)
        {
            break;
        }
        xmlFree.expect("non-null function pointer")(buf as *mut libc::c_void);
        size += 8192 as libc::c_int;
        buf = xmlMalloc.expect("non-null function pointer")(size as size_t)
            as *mut xmlChar;
        if buf.is_null() {
            xmlWriterErrMsg(
                0 as xmlTextWriterPtr,
                XML_ERR_NO_MEMORY,
                b"xmlTextWriterVSprintf : out of memory!\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as *mut xmlChar;
        }
        locarg = argptr.clone();
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCloseConnection(
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut res: libc::c_int = 0;
    let mut rfd: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut efd: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if ctxt.is_null() || (*ctxt).controlFd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    close((*ctxt).dataFd);
    (*ctxt).dataFd = -(1 as libc::c_int);
    tv.tv_sec = 15 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut rfd;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1);
        __i;
    }
    rfd
        .__fds_bits[((*ctxt).controlFd
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*ctxt).controlFd
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    let mut __i_0: libc::c_uint = 0;
    let mut __arr_0: *mut fd_set = &mut efd;
    __i_0 = 0 as libc::c_int as libc::c_uint;
    while (__i_0 as libc::c_ulong)
        < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr_0).__fds_bits[__i_0 as usize] = 0 as libc::c_int as __fd_mask;
        __i_0 = __i_0.wrapping_add(1);
        __i_0;
    }
    efd
        .__fds_bits[((*ctxt).controlFd
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*ctxt).controlFd
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    res = select(
        (*ctxt).controlFd + 1 as libc::c_int,
        &mut rfd,
        0 as *mut fd_set,
        &mut efd,
        &mut tv,
    );
    if res < 0 as libc::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    if res == 0 as libc::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = -(1 as libc::c_int);
    } else {
        res = xmlNanoFTPGetResponse(ctxt as *mut libc::c_void);
        if res != 2 as libc::c_int {
            close((*ctxt).controlFd);
            (*ctxt).controlFd = -(1 as libc::c_int);
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
static mut testErrors: [libc::c_char; 32769] = [0; 32769];
static mut testErrorsSize: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut callbacks_testlimits: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn channel_testlimits(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut res: libc::c_int = 0;
    if testErrorsSize >= 32768 as libc::c_int {
        return;
    }
    args_0 = args.clone();
    res = vsnprintf(
        &mut *testErrors.as_mut_ptr().offset(testErrorsSize as isize),
        (32768 as libc::c_int - testErrorsSize) as libc::c_ulong,
        msg,
        args_0.as_va_list(),
    );
    if testErrorsSize + res >= 32768 as libc::c_int {
        testErrorsSize = 32768 as libc::c_int;
        testErrors[testErrorsSize as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        testErrorsSize += res;
    }
    testErrors[testErrorsSize as usize] = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn warningCallback_testlimits(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    callbacks_testlimits = callbacks_testlimits.wrapping_add(1);
    callbacks_testlimits;
}
#[no_mangle]
pub unsafe extern "C" fn errorCallback_testlimits(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    callbacks_testlimits = callbacks_testlimits.wrapping_add(1);
    callbacks_testlimits;
}
#[no_mangle]
pub unsafe extern "C" fn fatalErrorCallback_testlimits(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCheckResponse(
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut rfd: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if ctxt.is_null() || (*ctxt).controlFd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut rfd;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1);
        __i;
    }
    rfd
        .__fds_bits[((*ctxt).controlFd
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*ctxt).controlFd
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    match select(
        (*ctxt).controlFd + 1 as libc::c_int,
        &mut rfd,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut tv,
    ) {
        0 => return 0 as libc::c_int,
        -1 => {
            __xmlIOErr(
                XML_FROM_FTP as libc::c_int,
                0 as libc::c_int,
                b"select\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        _ => {}
    }
    return xmlNanoFTPReadResponse(ctx);
}
#[no_mangle]
pub static mut testErrors_recurse: [libc::c_char; 32769] = [0; 32769];
#[no_mangle]
pub static mut testErrorsSize_recurse: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn channel_testrecurse(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut res: libc::c_int = 0;
    if testErrorsSize_recurse >= 32768 as libc::c_int {
        return;
    }
    args_0 = args.clone();
    res = vsnprintf(
        &mut *testErrors_recurse.as_mut_ptr().offset(testErrorsSize_recurse as isize),
        (32768 as libc::c_int - testErrorsSize_recurse) as libc::c_ulong,
        msg,
        args_0.as_va_list(),
    );
    if testErrorsSize_recurse + res >= 32768 as libc::c_int {
        testErrorsSize_recurse = 32768 as libc::c_int;
        testErrors_recurse[testErrorsSize_recurse
            as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        testErrorsSize_recurse += res;
    }
    testErrors_recurse[testErrorsSize_recurse
        as usize] = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn testHTML_warningDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    fprintf(stdout, b"SAX.warning: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn testHTML_errorDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    fprintf(stdout, b"SAX.error: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn testHTML_fatalErrorDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    fprintf(stdout, b"SAX.fatalError: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub static mut testSAX_callbacks: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut testSAX_quiet: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn testSAX_warningDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    testSAX_callbacks += 1;
    testSAX_callbacks;
    if testSAX_quiet != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(stdout, b"SAX.warning: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn testSAX_errorDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    testSAX_callbacks += 1;
    testSAX_callbacks;
    if testSAX_quiet != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(stdout, b"SAX.error: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn testSAX_fatalErrorDebug(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    testSAX_callbacks += 1;
    testSAX_callbacks;
    if testSAX_quiet != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(stdout, b"SAX.fatalError: \0" as *const u8 as *const libc::c_char);
    vfprintf(stdout, msg, args_0.as_va_list());
}
static mut testSAX_begin: timeval = timeval { tv_sec: 0, tv_usec: 0 };
static mut testSAX_end: timeval = timeval { tv_sec: 0, tv_usec: 0 };
#[no_mangle]
pub unsafe extern "C" fn testSAX_startTimer() {
    gettimeofday(&mut testSAX_begin, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn testSAX_endTimer(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msec: libc::c_long = 0;
    let mut ap: ::core::ffi::VaListImpl;
    gettimeofday(&mut testSAX_end, 0 as *mut libc::c_void);
    msec = testSAX_end.tv_sec - testSAX_begin.tv_sec;
    msec *= 1000 as libc::c_int as libc::c_long;
    msec
        += (testSAX_end.tv_usec - testSAX_begin.tv_usec)
            / 1000 as libc::c_int as __suseconds_t;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b" took %ld ms\n\0" as *const u8 as *const libc::c_char, msec);
}
#[no_mangle]
pub static mut testErrors_runtest: [libc::c_char; 32769] = [0; 32769];
#[no_mangle]
pub static mut testErrorsSize_runtest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn testErrorHandler_runtest(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut res: libc::c_int = 0;
    if testErrorsSize_runtest >= 32768 as libc::c_int {
        return;
    }
    args_0 = args.clone();
    res = vsnprintf(
        &mut *testErrors_runtest.as_mut_ptr().offset(testErrorsSize_runtest as isize),
        (32768 as libc::c_int - testErrorsSize_runtest) as libc::c_ulong,
        msg,
        args_0.as_va_list(),
    );
    if testErrorsSize_runtest + res >= 32768 as libc::c_int {
        testErrorsSize_runtest = 32768 as libc::c_int;
        testErrors_runtest[testErrorsSize_runtest
            as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        testErrorsSize_runtest += res;
    }
    testErrors_runtest[testErrorsSize_runtest
        as usize] = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn channel_runtest(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut res: libc::c_int = 0;
    if testErrorsSize_runtest >= 32768 as libc::c_int {
        return;
    }
    args_0 = args.clone();
    res = vsnprintf(
        &mut *testErrors_runtest.as_mut_ptr().offset(testErrorsSize_runtest as isize),
        (32768 as libc::c_int - testErrorsSize_runtest) as libc::c_ulong,
        msg,
        args_0.as_va_list(),
    );
    if testErrorsSize_runtest + res >= 32768 as libc::c_int {
        testErrorsSize_runtest = 32768 as libc::c_int;
        testErrors_runtest[testErrorsSize_runtest
            as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        testErrorsSize_runtest += res;
    }
    testErrors_runtest[testErrorsSize_runtest
        as usize] = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub static mut callbacks_runtest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut quiet_runtest: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut SAXdebug_runtest: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn warningDebug_runtest(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    callbacks_runtest += 1;
    callbacks_runtest;
    if quiet_runtest != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(SAXdebug_runtest, b"SAX.warning: \0" as *const u8 as *const libc::c_char);
    vfprintf(SAXdebug_runtest, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn errorDebug_runtest(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    callbacks_runtest += 1;
    callbacks_runtest;
    if quiet_runtest != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(SAXdebug_runtest, b"SAX.error: \0" as *const u8 as *const libc::c_char);
    vfprintf(SAXdebug_runtest, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn fatalErrorDebug_runtest(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    callbacks_runtest += 1;
    callbacks_runtest;
    if quiet_runtest != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(SAXdebug_runtest, b"SAX.fatalError: \0" as *const u8 as *const libc::c_char);
    vfprintf(SAXdebug_runtest, msg, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn ignoreGenericError_runtest(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {}
