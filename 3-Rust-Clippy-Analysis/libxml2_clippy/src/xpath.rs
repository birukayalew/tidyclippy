use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlPattern;
    pub type _xmlStreamCtxt;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    fn xmlStrncmp(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: libc::c_int,
    ) -> libc::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlStrlen(str: *const xmlChar) -> libc::c_int;
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    fn xmlStrPrintf(
        buf: *mut xmlChar,
        len: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xmlUTF8Strsize(utf: *const xmlChar, len: libc::c_int) -> libc::c_int;
    fn xmlUTF8Strpos(utf: *const xmlChar, pos: libc::c_int) -> *const xmlChar;
    fn xmlUTF8Strloc(utf: *const xmlChar, utfchar: *const xmlChar) -> libc::c_int;
    fn xmlUTF8Strsub(
        utf: *const xmlChar,
        start: libc::c_int,
        len: libc::c_int,
    ) -> *mut xmlChar;
    fn xmlUTF8Strlen(utf: *const xmlChar) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlBuildQName(
        ncname: *const xmlChar,
        prefix: *const xmlChar,
        memory: *mut xmlChar,
        len: libc::c_int,
    ) -> *mut xmlChar;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlFreeNodeList(cur: xmlNodePtr);
    fn xmlGetNsList(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr;
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlDictReference(dict: xmlDictPtr) -> libc::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: libc::c_int,
    ) -> *const xmlChar;
    fn xmlResetError(err: xmlErrorPtr);
    fn __xmlRaiseError(
        schannel: xmlStructuredErrorFunc,
        channel: xmlGenericErrorFunc,
        data: *mut libc::c_void,
        ctx: *mut libc::c_void,
        node: *mut libc::c_void,
        domain: libc::c_int,
        code: libc::c_int,
        level: xmlErrorLevel,
        file: *const libc::c_char,
        line: libc::c_int,
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
        int1: libc::c_int,
        col: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn xmlGetID(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr;
    fn xmlHashCreate(size: libc::c_int) -> xmlHashTablePtr;
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    fn xmlHashDefaultDeallocator(entry: *mut libc::c_void, name: *const xmlChar);
    fn xmlHashAddEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn xmlHashUpdateEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut libc::c_void,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    fn xmlHashAddEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn xmlHashUpdateEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut libc::c_void,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    fn xmlHashRemoveEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    fn xmlHashRemoveEntry2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        f: xmlHashDeallocator,
    ) -> libc::c_int;
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar) -> *mut libc::c_void;
    fn xmlHashLookup2(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn xmlFreePattern(comp: xmlPatternPtr);
    fn xmlFreePatternList(comp: xmlPatternPtr);
    fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: libc::c_int,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
    fn xmlPatternStreamable(comp: xmlPatternPtr) -> libc::c_int;
    fn xmlPatternMaxDepth(comp: xmlPatternPtr) -> libc::c_int;
    fn xmlPatternMinDepth(comp: xmlPatternPtr) -> libc::c_int;
    fn xmlPatternFromRoot(comp: xmlPatternPtr) -> libc::c_int;
    fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;
    fn xmlFreeStreamCtxt(stream: xmlStreamCtxtPtr);
    fn xmlStreamPushNode(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
        nodeType: libc::c_int,
    ) -> libc::c_int;
    fn xmlStreamPush(
        stream: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> libc::c_int;
    fn xmlStreamPop(stream: xmlStreamCtxtPtr) -> libc::c_int;
    fn xmlStreamWantsAnyNode(stream: xmlStreamCtxtPtr) -> libc::c_int;
    fn xmlCharInRange(val: libc::c_uint, group: *const xmlChRangeGroup) -> libc::c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsCombiningGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsExtenderGroup: xmlChRangeGroup;
    fn xmlCopyChar(len: libc::c_int, out: *mut xmlChar, val: libc::c_int) -> libc::c_int;
    fn xmlXPtrLocationSetCreate(val: xmlXPathObjectPtr) -> xmlLocationSetPtr;
    fn xmlXPtrFreeLocationSet(obj: xmlLocationSetPtr);
    fn xmlXPtrLocationSetMerge(
        val1: xmlLocationSetPtr,
        val2: xmlLocationSetPtr,
    ) -> xmlLocationSetPtr;
    fn xmlXPtrNewRange(
        start: xmlNodePtr,
        startindex: libc::c_int,
        end: xmlNodePtr,
        endindex: libc::c_int,
    ) -> xmlXPathObjectPtr;
    fn xmlXPtrNewRangeNodeObject(
        start: xmlNodePtr,
        end: xmlXPathObjectPtr,
    ) -> xmlXPathObjectPtr;
    fn xmlXPtrLocationSetAdd(cur: xmlLocationSetPtr, val: xmlXPathObjectPtr);
    fn xmlXPtrWrapLocationSet(val: xmlLocationSetPtr) -> xmlXPathObjectPtr;
    fn xmlDebugDumpString(output: *mut FILE, str: *const xmlChar);
    fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: libc::c_int);
    fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: libc::c_int);
    fn xmlBufCreate() -> xmlBufPtr;
    fn xmlBufFree(buf: xmlBufPtr);
    fn xmlBufAdd(buf: xmlBufPtr, str: *const xmlChar, len: libc::c_int) -> libc::c_int;
}
pub type xmlChar = libc::c_uchar;
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
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
pub type ptrdiff_t = libc::c_long;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
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
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlStructuredErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
>;
pub type xmlErrorPtr = *mut xmlError;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlHashDeallocator = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
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
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: libc::c_int,
    pub max_variables_unused: libc::c_int,
    pub varHash: xmlHashTablePtr,
    pub nb_types: libc::c_int,
    pub max_types: libc::c_int,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: libc::c_int,
    pub max_funcs_unused: libc::c_int,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: libc::c_int,
    pub max_axis: libc::c_int,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: libc::c_int,
    pub user: *mut libc::c_void,
    pub contextSize: libc::c_int,
    pub proximityPosition: libc::c_int,
    pub xptr: libc::c_int,
    pub here: xmlNodePtr,
    pub origin: xmlNodePtr,
    pub nsHash: xmlHashTablePtr,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut libc::c_void,
    pub extra: *mut libc::c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut libc::c_void,
    pub tmpNsList: *mut xmlNsPtr,
    pub tmpNsNr: libc::c_int,
    pub userData: *mut libc::c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: libc::c_int,
    pub cache: *mut libc::c_void,
}
pub type xmlXPathFuncLookupFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathFunction,
>;
pub type xmlXPathFunction = Option::<
    unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: libc::c_int,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: libc::c_int,
    pub valueMax: libc::c_int,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: libc::c_int,
    pub ancestor: xmlNodePtr,
    pub valueFrame: libc::c_int,
}
pub type xmlXPathCompExprPtr = *mut xmlXPathCompExpr;
pub type xmlXPathCompExpr = _xmlXPathCompExpr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathCompExpr {
    pub nbStep: libc::c_int,
    pub maxStep: libc::c_int,
    pub steps: *mut xmlXPathStepOp,
    pub last: libc::c_int,
    pub expr: *mut xmlChar,
    pub dict: xmlDictPtr,
    pub stream: xmlPatternPtr,
}
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
pub type xmlXPathStepOp = _xmlXPathStepOp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathStepOp {
    pub op: xmlXPathOp,
    pub ch1: libc::c_int,
    pub ch2: libc::c_int,
    pub value: libc::c_int,
    pub value2: libc::c_int,
    pub value3: libc::c_int,
    pub value4: *mut libc::c_void,
    pub value5: *mut libc::c_void,
    pub cache: xmlXPathFunction,
    pub cacheURI: *mut libc::c_void,
}
pub type xmlXPathOp = libc::c_uint;
pub const XPATH_OP_RANGETO: xmlXPathOp = 18;
pub const XPATH_OP_SORT: xmlXPathOp = 17;
pub const XPATH_OP_FILTER: xmlXPathOp = 16;
pub const XPATH_OP_PREDICATE: xmlXPathOp = 15;
pub const XPATH_OP_ARG: xmlXPathOp = 14;
pub const XPATH_OP_FUNCTION: xmlXPathOp = 13;
pub const XPATH_OP_VARIABLE: xmlXPathOp = 12;
pub const XPATH_OP_VALUE: xmlXPathOp = 11;
pub const XPATH_OP_COLLECT: xmlXPathOp = 10;
pub const XPATH_OP_NODE: xmlXPathOp = 9;
pub const XPATH_OP_ROOT: xmlXPathOp = 8;
pub const XPATH_OP_UNION: xmlXPathOp = 7;
pub const XPATH_OP_MULT: xmlXPathOp = 6;
pub const XPATH_OP_PLUS: xmlXPathOp = 5;
pub const XPATH_OP_CMP: xmlXPathOp = 4;
pub const XPATH_OP_EQUAL: xmlXPathOp = 3;
pub const XPATH_OP_OR: xmlXPathOp = 2;
pub const XPATH_OP_AND: xmlXPathOp = 1;
pub const XPATH_OP_END: xmlXPathOp = 0;
pub type xmlXPathObjectPtr = *mut xmlXPathObject;
pub type xmlXPathObject = _xmlXPathObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathObject {
    pub type_0: xmlXPathObjectType,
    pub nodesetval: xmlNodeSetPtr,
    pub boolval: libc::c_int,
    pub floatval: libc::c_double,
    pub stringval: *mut xmlChar,
    pub user: *mut libc::c_void,
    pub index: libc::c_int,
    pub user2: *mut libc::c_void,
    pub index2: libc::c_int,
}
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: libc::c_int,
    pub nodeMax: libc::c_int,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlXPathObjectType = libc::c_uint;
pub const XPATH_XSLT_TREE: xmlXPathObjectType = 9;
pub const XPATH_USERS: xmlXPathObjectType = 8;
pub const XPATH_LOCATIONSET: xmlXPathObjectType = 7;
pub const XPATH_RANGE: xmlXPathObjectType = 6;
pub const XPATH_POINT: xmlXPathObjectType = 5;
pub const XPATH_STRING: xmlXPathObjectType = 4;
pub const XPATH_NUMBER: xmlXPathObjectType = 3;
pub const XPATH_BOOLEAN: xmlXPathObjectType = 2;
pub const XPATH_NODESET: xmlXPathObjectType = 1;
pub const XPATH_UNDEFINED: xmlXPathObjectType = 0;
pub type xmlXPathContextPtr = *mut xmlXPathContext;
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
pub type xmlXPathAxisFunc = Option::<
    unsafe extern "C" fn(
        xmlXPathParserContextPtr,
        xmlXPathObjectPtr,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
pub type xmlXPathConvertFunc = Option::<
    unsafe extern "C" fn(xmlXPathObjectPtr, libc::c_int) -> libc::c_int,
>;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const XPATH_FORBID_VARIABLE_ERROR: C2RustUnnamed_1 = 24;
pub const XPATH_STACK_ERROR: C2RustUnnamed_1 = 23;
pub const XPATH_INVALID_CTXT: C2RustUnnamed_1 = 22;
pub const XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_1 = 21;
pub const XPATH_ENCODING_ERROR: C2RustUnnamed_1 = 20;
pub const XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_1 = 19;
pub const XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_1 = 18;
pub const XPTR_RESOURCE_ERROR: C2RustUnnamed_1 = 17;
pub const XPTR_SYNTAX_ERROR: C2RustUnnamed_1 = 16;
pub const XPATH_MEMORY_ERROR: C2RustUnnamed_1 = 15;
pub const XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_1 = 14;
pub const XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_1 = 13;
pub const XPATH_INVALID_ARITY: C2RustUnnamed_1 = 12;
pub const XPATH_INVALID_TYPE: C2RustUnnamed_1 = 11;
pub const XPATH_INVALID_OPERAND: C2RustUnnamed_1 = 10;
pub const XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_1 = 9;
pub const XPATH_UNCLOSED_ERROR: C2RustUnnamed_1 = 8;
pub const XPATH_EXPR_ERROR: C2RustUnnamed_1 = 7;
pub const XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_1 = 6;
pub const XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_1 = 5;
pub const XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_1 = 4;
pub const XPATH_START_LITERAL_ERROR: C2RustUnnamed_1 = 3;
pub const XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_1 = 2;
pub const XPATH_NUMBER_ERROR: C2RustUnnamed_1 = 1;
pub const XPATH_EXPRESSION_OK: C2RustUnnamed_1 = 0;
pub type xmlLocationSetPtr = *mut xmlLocationSet;
pub type xmlLocationSet = _xmlLocationSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLocationSet {
    pub locNr: libc::c_int,
    pub locMax: libc::c_int,
    pub locTab: *mut xmlXPathObjectPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIM_SORT_RUN_T {
    pub start: size_t,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEMP_STORAGE_T {
    pub alloc: size_t,
    pub storage: *mut xmlNodePtr,
}
pub type uint64_t = __uint64_t;
pub type xmlPointerListPtr = *mut xmlPointerList;
pub type xmlPointerList = _xmlPointerList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlPointerList {
    pub items: *mut *mut libc::c_void,
    pub number: libc::c_int,
    pub size: libc::c_int,
}
pub type xmlXPathContextCachePtr = *mut xmlXPathContextCache;
pub type xmlXPathContextCache = _xmlXPathContextCache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContextCache {
    pub nodesetObjs: xmlPointerListPtr,
    pub stringObjs: xmlPointerListPtr,
    pub booleanObjs: xmlPointerListPtr,
    pub numberObjs: xmlPointerListPtr,
    pub miscObjs: xmlPointerListPtr,
    pub maxNodeset: libc::c_int,
    pub maxString: libc::c_int,
    pub maxBoolean: libc::c_int,
    pub maxNumber: libc::c_int,
    pub maxMisc: libc::c_int,
}
pub type xmlXPathStepOpPtr = *mut xmlXPathStepOp;
pub type xmlXPathNodeSetMergeFunction = Option::<
    unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr, libc::c_int) -> xmlNodeSetPtr,
>;
pub const AXIS_NAMESPACE: xmlXPathAxisVal = 9;
pub type xmlXPathAxisVal = libc::c_uint;
pub const AXIS_SELF: xmlXPathAxisVal = 13;
pub const AXIS_PRECEDING_SIBLING: xmlXPathAxisVal = 12;
pub const AXIS_PRECEDING: xmlXPathAxisVal = 11;
pub const AXIS_PARENT: xmlXPathAxisVal = 10;
pub const AXIS_FOLLOWING_SIBLING: xmlXPathAxisVal = 8;
pub const AXIS_FOLLOWING: xmlXPathAxisVal = 7;
pub const AXIS_DESCENDANT_OR_SELF: xmlXPathAxisVal = 6;
pub const AXIS_DESCENDANT: xmlXPathAxisVal = 5;
pub const AXIS_CHILD: xmlXPathAxisVal = 4;
pub const AXIS_ATTRIBUTE: xmlXPathAxisVal = 3;
pub const AXIS_ANCESTOR_OR_SELF: xmlXPathAxisVal = 2;
pub const AXIS_ANCESTOR: xmlXPathAxisVal = 1;
pub const NODE_TEST_NAME: xmlXPathTestVal = 5;
pub const NODE_TEST_NS: xmlXPathTestVal = 4;
pub const NODE_TEST_ALL: xmlXPathTestVal = 3;
pub const NODE_TEST_PI: xmlXPathTestVal = 2;
pub const NODE_TYPE_TEXT: xmlXPathTypeVal = 3;
pub type xmlXPathTypeVal = libc::c_uint;
pub const NODE_TYPE_PI: xmlXPathTypeVal = 7;
pub const NODE_TYPE_COMMENT: xmlXPathTypeVal = 8;
pub const NODE_TYPE_NODE: xmlXPathTypeVal = 0;
pub const NODE_TEST_TYPE: xmlXPathTestVal = 1;
pub const NODE_TEST_NONE: xmlXPathTestVal = 0;
pub type xmlXPathTestVal = libc::c_uint;
pub type xmlXPathTraversalFunction = Option::<
    unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
>;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: libc::c_int,
    pub nbLongRange: libc::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: libc::c_uint,
    pub high: libc::c_uint,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: libc::c_ushort,
    pub high: libc::c_ushort,
}
pub const XML_PATTERN_XPATH: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const XML_PATTERN_XSFIELD: C2RustUnnamed_2 = 4;
pub const XML_PATTERN_XSSEL: C2RustUnnamed_2 = 2;
pub const XML_PATTERN_DEFAULT: C2RustUnnamed_2 = 0;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn xmlXPathCmpNodesExt(
    mut node1: xmlNodePtr,
    mut node2: xmlNodePtr,
) -> libc::c_int {
    let mut current_block: u64;
    let mut depth1: libc::c_int = 0;
    let mut depth2: libc::c_int = 0;
    let mut misc: libc::c_int = 0 as libc::c_int;
    let mut precedence1: libc::c_int = 0 as libc::c_int;
    let mut precedence2: libc::c_int = 0 as libc::c_int;
    let mut miscNode1: xmlNodePtr = 0 as xmlNodePtr;
    let mut miscNode2: xmlNodePtr = 0 as xmlNodePtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut l1: ptrdiff_t = 0;
    let mut l2: ptrdiff_t = 0;
    if node1.is_null() || node2.is_null() {
        return -(2 as libc::c_int);
    }
    if node1 == node2 {
        return 0 as libc::c_int;
    }
    match (*node1).type_0 as libc::c_uint {
        1 => {
            if (*node2).type_0 as libc::c_uint
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            {
                if 0 as libc::c_int as ptrdiff_t > (*node1).content as ptrdiff_t
                    && 0 as libc::c_int as ptrdiff_t > (*node2).content as ptrdiff_t
                    && (*node1).doc == (*node2).doc
                {
                    l1 = -((*node1).content as ptrdiff_t);
                    l2 = -((*node2).content as ptrdiff_t);
                    if l1 < l2 {
                        return 1 as libc::c_int;
                    }
                    if l1 > l2 {
                        return -(1 as libc::c_int);
                    }
                    current_block = 6450636197030046351;
                } else {
                    current_block = 12503744501699438339;
                }
            } else {
                current_block = 6450636197030046351;
            }
        }
        2 => {
            precedence1 = 1 as libc::c_int;
            miscNode1 = node1;
            node1 = (*node1).parent;
            misc = 1 as libc::c_int;
            current_block = 6450636197030046351;
        }
        3 | 4 | 8 | 7 => {
            miscNode1 = node1;
            if !((*node1).prev).is_null() {
                loop {
                    node1 = (*node1).prev;
                    if (*node1).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        precedence1 = 3 as libc::c_int;
                        break;
                    } else {
                        if !((*node1).prev).is_null() {
                            continue;
                        }
                        precedence1 = 2 as libc::c_int;
                        node1 = (*node1).parent;
                        break;
                    }
                }
            } else {
                precedence1 = 2 as libc::c_int;
                node1 = (*node1).parent;
            }
            if node1.is_null()
                || (*node1).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                || 0 as libc::c_int as ptrdiff_t <= (*node1).content as ptrdiff_t
            {
                node1 = miscNode1;
                precedence1 = 0 as libc::c_int;
            } else {
                misc = 1 as libc::c_int;
            }
            current_block = 6450636197030046351;
        }
        18 => return 1 as libc::c_int,
        _ => {
            current_block = 6450636197030046351;
        }
    }
    match current_block {
        6450636197030046351 => {
            match (*node2).type_0 as libc::c_uint {
                2 => {
                    precedence2 = 1 as libc::c_int;
                    miscNode2 = node2;
                    node2 = (*node2).parent;
                    misc = 1 as libc::c_int;
                }
                3 | 4 | 8 | 7 => {
                    miscNode2 = node2;
                    if !((*node2).prev).is_null() {
                        loop {
                            node2 = (*node2).prev;
                            if (*node2).type_0 as libc::c_uint
                                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            {
                                precedence2 = 3 as libc::c_int;
                                break;
                            } else {
                                if !((*node2).prev).is_null() {
                                    continue;
                                }
                                precedence2 = 2 as libc::c_int;
                                node2 = (*node2).parent;
                                break;
                            }
                        }
                    } else {
                        precedence2 = 2 as libc::c_int;
                        node2 = (*node2).parent;
                    }
                    if node2.is_null()
                        || (*node2).type_0 as libc::c_uint
                            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                        || 0 as libc::c_int as ptrdiff_t <= (*node2).content as ptrdiff_t
                    {
                        node2 = miscNode2;
                        precedence2 = 0 as libc::c_int;
                    } else {
                        misc = 1 as libc::c_int;
                    }
                }
                18 => return 1 as libc::c_int,
                1 | _ => {}
            }
            if misc != 0 {
                if node1 == node2 {
                    if precedence1 == precedence2 {
                        cur = (*miscNode2).prev;
                        while !cur.is_null() {
                            if cur == miscNode1 {
                                return 1 as libc::c_int;
                            }
                            if (*cur).type_0 as libc::c_uint
                                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            {
                                return -(1 as libc::c_int);
                            }
                            cur = (*cur).prev;
                        }
                        return -(1 as libc::c_int);
                    } else if precedence1 < precedence2 {
                        return 1 as libc::c_int
                    } else {
                        return -(1 as libc::c_int)
                    }
                }
                if precedence2 == 3 as libc::c_int && precedence1 > 1 as libc::c_int {
                    cur = (*node1).parent;
                    while !cur.is_null() {
                        if cur == node2 {
                            return 1 as libc::c_int;
                        }
                        cur = (*cur).parent;
                    }
                }
                if precedence1 == 3 as libc::c_int && precedence2 > 1 as libc::c_int {
                    cur = (*node2).parent;
                    while !cur.is_null() {
                        if cur == node1 {
                            return -(1 as libc::c_int);
                        }
                        cur = (*cur).parent;
                    }
                }
            }
            if (*node1).type_0 as libc::c_uint
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                && (*node2).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                && 0 as libc::c_int as ptrdiff_t > (*node1).content as ptrdiff_t
                && 0 as libc::c_int as ptrdiff_t > (*node2).content as ptrdiff_t
                && (*node1).doc == (*node2).doc
            {
                l1 = -((*node1).content as ptrdiff_t);
                l2 = -((*node2).content as ptrdiff_t);
                if l1 < l2 {
                    return 1 as libc::c_int;
                }
                if l1 > l2 {
                    return -(1 as libc::c_int);
                }
            }
        }
        _ => {}
    }
    if node1 == (*node2).prev {
        return 1 as libc::c_int;
    }
    if node1 == (*node2).next {
        return -(1 as libc::c_int);
    }
    depth2 = 0 as libc::c_int;
    cur = node2;
    while !((*cur).parent).is_null() {
        if (*cur).parent == node1 {
            return 1 as libc::c_int;
        }
        depth2 += 1;
        depth2;
        cur = (*cur).parent;
    }
    root = cur;
    depth1 = 0 as libc::c_int;
    cur = node1;
    while !((*cur).parent).is_null() {
        if (*cur).parent == node2 {
            return -(1 as libc::c_int);
        }
        depth1 += 1;
        depth1;
        cur = (*cur).parent;
    }
    if root != cur {
        return -(2 as libc::c_int);
    }
    while depth1 > depth2 {
        depth1 -= 1;
        depth1;
        node1 = (*node1).parent;
    }
    while depth2 > depth1 {
        depth2 -= 1;
        depth2;
        node2 = (*node2).parent;
    }
    while (*node1).parent != (*node2).parent {
        node1 = (*node1).parent;
        node2 = (*node2).parent;
        if node1.is_null() || node2.is_null() {
            return -(2 as libc::c_int);
        }
    }
    if node1 == (*node2).prev {
        return 1 as libc::c_int;
    }
    if node1 == (*node2).next {
        return -(1 as libc::c_int);
    }
    if (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && (*node2).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && 0 as libc::c_int as ptrdiff_t > (*node1).content as ptrdiff_t
        && 0 as libc::c_int as ptrdiff_t > (*node2).content as ptrdiff_t
        && (*node1).doc == (*node2).doc
    {
        l1 = -((*node1).content as ptrdiff_t);
        l2 = -((*node2).content as ptrdiff_t);
        if l1 < l2 {
            return 1 as libc::c_int;
        }
        if l1 > l2 {
            return -(1 as libc::c_int);
        }
    }
    cur = (*node1).next;
    while !cur.is_null() {
        if cur == node2 {
            return 1 as libc::c_int;
        }
        cur = (*cur).next;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn wrap_cmp(mut x: xmlNodePtr, mut y: xmlNodePtr) -> libc::c_int {
    let mut res: libc::c_int = xmlXPathCmpNodesExt(x, y);
    return if res == -(2 as libc::c_int) { res } else { -res };
}
#[inline]
unsafe extern "C" fn compute_minrun(size: uint64_t) -> libc::c_int {
    let top_bit: libc::c_int = 64 as libc::c_int
        - (size as libc::c_ulonglong).leading_zeros() as i32;
    let shift: libc::c_int = (if top_bit > 6 as libc::c_int {
        top_bit
    } else {
        6 as libc::c_int
    }) - 6 as libc::c_int;
    let minrun: libc::c_int = (size >> shift) as libc::c_int;
    let mask: uint64_t = ((1 as libc::c_ulonglong) << shift)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    if mask & size != 0 {
        return minrun + 1 as libc::c_int;
    }
    return minrun;
}
#[inline]
unsafe extern "C" fn libxml_domnode_binary_insertion_find(
    mut dst: *mut xmlNodePtr,
    x: xmlNodePtr,
    size: size_t,
) -> size_t {
    let mut l: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut cx: xmlNodePtr = 0 as *mut xmlNode;
    l = 0 as libc::c_int as size_t;
    r = size.wrapping_sub(1 as libc::c_int as size_t);
    c = r >> 1 as libc::c_int;
    if wrap_cmp(x, *dst.offset(0 as libc::c_int as isize)) < 0 as libc::c_int {
        return 0 as libc::c_int as size_t
    } else if wrap_cmp(x, *dst.offset(r as isize)) > 0 as libc::c_int {
        return r
    }
    cx = *dst.offset(c as isize);
    loop {
        let val: libc::c_int = wrap_cmp(x, cx);
        if val < 0 as libc::c_int {
            if c.wrapping_sub(l) <= 1 as libc::c_int as size_t {
                return c;
            }
            r = c;
        } else {
            if r.wrapping_sub(c) <= 1 as libc::c_int as size_t {
                return c.wrapping_add(1 as libc::c_int as size_t);
            }
            l = c;
        }
        c = l.wrapping_add(r.wrapping_sub(l) >> 1 as libc::c_int);
        cx = *dst.offset(c as isize);
    };
}
unsafe extern "C" fn libxml_domnode_binary_insertion_sort_start(
    mut dst: *mut xmlNodePtr,
    start: size_t,
    size: size_t,
) {
    let mut i: size_t = 0;
    i = start;
    while i < size {
        let mut j: size_t = 0;
        let mut x: xmlNodePtr = 0 as *mut xmlNode;
        let mut location: size_t = 0;
        if !(wrap_cmp(
            *dst.offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize),
            *dst.offset(i as isize),
        ) <= 0 as libc::c_int)
        {
            x = *dst.offset(i as isize);
            location = libxml_domnode_binary_insertion_find(dst, x, i);
            j = i.wrapping_sub(1 as libc::c_int as size_t);
            while j >= location {
                let ref mut fresh0 = *dst
                    .offset(j.wrapping_add(1 as libc::c_int as size_t) as isize);
                *fresh0 = *dst.offset(j as isize);
                if j == 0 as libc::c_int as size_t {
                    break;
                }
                j = j.wrapping_sub(1);
                j;
            }
            let ref mut fresh1 = *dst.offset(location as isize);
            *fresh1 = x;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn libxml_domnode_binary_insertion_sort(
    mut dst: *mut xmlNodePtr,
    size: size_t,
) {
    if size <= 1 as libc::c_int as size_t {
        return;
    }
    libxml_domnode_binary_insertion_sort_start(dst, 1 as libc::c_int as size_t, size);
}
#[inline]
unsafe extern "C" fn libxml_domnode_reverse_elements(
    mut dst: *mut xmlNodePtr,
    mut start: size_t,
    mut end: size_t,
) {
    loop {
        if start >= end {
            return;
        }
        let mut __SORT_SWAP_t: xmlNodePtr = *dst.offset(start as isize);
        let ref mut fresh2 = *dst.offset(start as isize);
        *fresh2 = *dst.offset(end as isize);
        let ref mut fresh3 = *dst.offset(end as isize);
        *fresh3 = __SORT_SWAP_t;
        start = start.wrapping_add(1);
        start;
        end = end.wrapping_sub(1);
        end;
    };
}
unsafe extern "C" fn libxml_domnode_count_run(
    mut dst: *mut xmlNodePtr,
    start: size_t,
    size: size_t,
) -> size_t {
    let mut curr: size_t = 0;
    if size.wrapping_sub(start) == 1 as libc::c_int as size_t {
        return 1 as libc::c_int as size_t;
    }
    if start >= size.wrapping_sub(2 as libc::c_int as size_t) {
        if wrap_cmp(
            *dst.offset(size.wrapping_sub(2 as libc::c_int as size_t) as isize),
            *dst.offset(size.wrapping_sub(1 as libc::c_int as size_t) as isize),
        ) > 0 as libc::c_int
        {
            let mut __SORT_SWAP_t: xmlNodePtr = *dst
                .offset(size.wrapping_sub(2 as libc::c_int as size_t) as isize);
            let ref mut fresh4 = *dst
                .offset(size.wrapping_sub(2 as libc::c_int as size_t) as isize);
            *fresh4 = *dst
                .offset(size.wrapping_sub(1 as libc::c_int as size_t) as isize);
            let ref mut fresh5 = *dst
                .offset(size.wrapping_sub(1 as libc::c_int as size_t) as isize);
            *fresh5 = __SORT_SWAP_t;
        }
        return 2 as libc::c_int as size_t;
    }
    curr = start.wrapping_add(2 as libc::c_int as size_t);
    if wrap_cmp(
        *dst.offset(start as isize),
        *dst.offset(start.wrapping_add(1 as libc::c_int as size_t) as isize),
    ) <= 0 as libc::c_int
    {
        while !(curr == size.wrapping_sub(1 as libc::c_int as size_t)) {
            if wrap_cmp(
                *dst.offset(curr.wrapping_sub(1 as libc::c_int as size_t) as isize),
                *dst.offset(curr as isize),
            ) > 0 as libc::c_int
            {
                break;
            }
            curr = curr.wrapping_add(1);
            curr;
        }
        return curr.wrapping_sub(start);
    } else {
        while !(curr == size.wrapping_sub(1 as libc::c_int as size_t)) {
            if wrap_cmp(
                *dst.offset(curr.wrapping_sub(1 as libc::c_int as size_t) as isize),
                *dst.offset(curr as isize),
            ) <= 0 as libc::c_int
            {
                break;
            }
            curr = curr.wrapping_add(1);
            curr;
        }
        libxml_domnode_reverse_elements(
            dst,
            start,
            curr.wrapping_sub(1 as libc::c_int as size_t),
        );
        return curr.wrapping_sub(start);
    };
}
unsafe extern "C" fn libxml_domnode_check_invariant(
    mut stack: *mut TIM_SORT_RUN_T,
    stack_curr: libc::c_int,
) -> libc::c_int {
    let mut A: size_t = 0;
    let mut B: size_t = 0;
    let mut C: size_t = 0;
    if stack_curr < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if stack_curr == 2 as libc::c_int {
        let A1: size_t = (*stack.offset((stack_curr - 2 as libc::c_int) as isize))
            .length;
        let B1: size_t = (*stack.offset((stack_curr - 1 as libc::c_int) as isize))
            .length;
        if A1 <= B1 {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    A = (*stack.offset((stack_curr - 3 as libc::c_int) as isize)).length;
    B = (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length;
    C = (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length;
    if A <= B.wrapping_add(C) || B <= C {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn libxml_domnode_tim_sort_resize(
    mut store: *mut TEMP_STORAGE_T,
    new_size: size_t,
) {
    if (*store).alloc < new_size {
        let mut tempstore: *mut xmlNodePtr = realloc(
            (*store).storage as *mut libc::c_void,
            new_size.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if tempstore.is_null() {
            fprintf(
                stderr,
                b"Error allocating temporary storage for tim sort: need %lu bytes\0"
                    as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong)
                    .wrapping_mul(new_size),
            );
            exit(1 as libc::c_int);
        }
        (*store).storage = tempstore;
        (*store).alloc = new_size;
    }
}
unsafe extern "C" fn libxml_domnode_tim_sort_merge(
    mut dst: *mut xmlNodePtr,
    mut stack: *const TIM_SORT_RUN_T,
    stack_curr: libc::c_int,
    mut store: *mut TEMP_STORAGE_T,
) {
    let A: size_t = (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length;
    let B: size_t = (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length;
    let curr: size_t = (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).start;
    let mut storage: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    libxml_domnode_tim_sort_resize(store, if A < B { A } else { B });
    storage = (*store).storage;
    if A < B {
        memcpy(
            storage as *mut libc::c_void,
            &mut *dst.offset(curr as isize) as *mut xmlNodePtr as *const libc::c_void,
            A.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        );
        i = 0 as libc::c_int as size_t;
        j = curr.wrapping_add(A);
        k = curr;
        while k < curr.wrapping_add(A).wrapping_add(B) {
            if i < A && j < curr.wrapping_add(A).wrapping_add(B) {
                if wrap_cmp(*storage.offset(i as isize), *dst.offset(j as isize))
                    <= 0 as libc::c_int
                {
                    let fresh6 = i;
                    i = i.wrapping_add(1);
                    let ref mut fresh7 = *dst.offset(k as isize);
                    *fresh7 = *storage.offset(fresh6 as isize);
                } else {
                    let fresh8 = j;
                    j = j.wrapping_add(1);
                    let ref mut fresh9 = *dst.offset(k as isize);
                    *fresh9 = *dst.offset(fresh8 as isize);
                }
            } else {
                if !(i < A) {
                    break;
                }
                let fresh10 = i;
                i = i.wrapping_add(1);
                let ref mut fresh11 = *dst.offset(k as isize);
                *fresh11 = *storage.offset(fresh10 as isize);
            }
            k = k.wrapping_add(1);
            k;
        }
    } else {
        memcpy(
            storage as *mut libc::c_void,
            &mut *dst.offset(curr.wrapping_add(A) as isize) as *mut xmlNodePtr
                as *const libc::c_void,
            B.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        );
        i = B;
        j = curr.wrapping_add(A);
        k = curr.wrapping_add(A).wrapping_add(B);
        loop {
            let fresh12 = k;
            k = k.wrapping_sub(1);
            if !(fresh12 > curr) {
                break;
            }
            if i > 0 as libc::c_int as size_t && j > curr {
                if wrap_cmp(
                    *dst.offset(j.wrapping_sub(1 as libc::c_int as size_t) as isize),
                    *storage.offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize),
                ) > 0 as libc::c_int
                {
                    j = j.wrapping_sub(1);
                    let ref mut fresh13 = *dst.offset(k as isize);
                    *fresh13 = *dst.offset(j as isize);
                } else {
                    i = i.wrapping_sub(1);
                    let ref mut fresh14 = *dst.offset(k as isize);
                    *fresh14 = *storage.offset(i as isize);
                }
            } else {
                if !(i > 0 as libc::c_int as size_t) {
                    break;
                }
                i = i.wrapping_sub(1);
                let ref mut fresh15 = *dst.offset(k as isize);
                *fresh15 = *storage.offset(i as isize);
            }
        }
    };
}
unsafe extern "C" fn libxml_domnode_tim_sort_collapse(
    mut dst: *mut xmlNodePtr,
    mut stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: libc::c_int,
    mut store: *mut TEMP_STORAGE_T,
    size: size_t,
) -> libc::c_int {
    loop {
        let mut A: size_t = 0;
        let mut B: size_t = 0;
        let mut C: size_t = 0;
        let mut D: size_t = 0;
        let mut ABC: libc::c_int = 0;
        let mut BCD: libc::c_int = 0;
        let mut CD: libc::c_int = 0;
        if stack_curr <= 1 as libc::c_int {
            break;
        }
        if stack_curr == 2 as libc::c_int
            && ((*stack.offset(0 as libc::c_int as isize)).length)
                .wrapping_add((*stack.offset(1 as libc::c_int as isize)).length) == size
        {
            libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
            let ref mut fresh16 = (*stack.offset(0 as libc::c_int as isize)).length;
            *fresh16 = (*fresh16)
                .wrapping_add((*stack.offset(1 as libc::c_int as isize)).length);
            stack_curr -= 1;
            stack_curr;
            break;
        } else if stack_curr == 2 as libc::c_int
            && (*stack.offset(0 as libc::c_int as isize)).length
                <= (*stack.offset(1 as libc::c_int as isize)).length
        {
            libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
            let ref mut fresh17 = (*stack.offset(0 as libc::c_int as isize)).length;
            *fresh17 = (*fresh17)
                .wrapping_add((*stack.offset(1 as libc::c_int as isize)).length);
            stack_curr -= 1;
            stack_curr;
            break;
        } else {
            if stack_curr == 2 as libc::c_int {
                break;
            }
            B = (*stack.offset((stack_curr - 3 as libc::c_int) as isize)).length;
            C = (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length;
            D = (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length;
            if stack_curr >= 4 as libc::c_int {
                A = (*stack.offset((stack_curr - 4 as libc::c_int) as isize)).length;
                ABC = (A <= B.wrapping_add(C)) as libc::c_int;
            } else {
                ABC = 0 as libc::c_int;
            }
            BCD = (B <= C.wrapping_add(D) || ABC != 0) as libc::c_int;
            CD = (C <= D) as libc::c_int;
            if BCD == 0 && CD == 0 {
                break;
            }
            if BCD != 0 && CD == 0 {
                libxml_domnode_tim_sort_merge(
                    dst,
                    stack,
                    stack_curr - 1 as libc::c_int,
                    store,
                );
                let ref mut fresh18 = (*stack
                    .offset((stack_curr - 3 as libc::c_int) as isize))
                    .length;
                *fresh18 = (*fresh18)
                    .wrapping_add(
                        (*stack.offset((stack_curr - 2 as libc::c_int) as isize)).length,
                    );
                *stack
                    .offset(
                        (stack_curr - 2 as libc::c_int) as isize,
                    ) = *stack.offset((stack_curr - 1 as libc::c_int) as isize);
                stack_curr -= 1;
                stack_curr;
            } else {
                libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
                let ref mut fresh19 = (*stack
                    .offset((stack_curr - 2 as libc::c_int) as isize))
                    .length;
                *fresh19 = (*fresh19)
                    .wrapping_add(
                        (*stack.offset((stack_curr - 1 as libc::c_int) as isize)).length,
                    );
                stack_curr -= 1;
                stack_curr;
            }
        }
    }
    return stack_curr;
}
#[inline]
unsafe extern "C" fn PUSH_NEXT(
    mut dst: *mut xmlNodePtr,
    size: size_t,
    mut store: *mut TEMP_STORAGE_T,
    minrun: size_t,
    mut run_stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: *mut size_t,
    mut curr: *mut size_t,
) -> libc::c_int {
    let mut len: size_t = libxml_domnode_count_run(dst, *curr, size);
    let mut run: size_t = minrun;
    if run > size.wrapping_sub(*curr) {
        run = size.wrapping_sub(*curr);
    }
    if run > len {
        libxml_domnode_binary_insertion_sort_start(
            &mut *dst.offset(*curr as isize),
            len,
            run,
        );
        len = run;
    }
    (*run_stack.offset(*stack_curr as isize)).start = *curr;
    (*run_stack.offset(*stack_curr as isize)).length = len;
    *stack_curr = (*stack_curr).wrapping_add(1);
    *stack_curr;
    *curr = (*curr).wrapping_add(len);
    if *curr == size {
        while *stack_curr > 1 as libc::c_int as size_t {
            libxml_domnode_tim_sort_merge(
                dst,
                run_stack,
                *stack_curr as libc::c_int,
                store,
            );
            let ref mut fresh20 = (*run_stack
                .offset((*stack_curr).wrapping_sub(2 as libc::c_int as size_t) as isize))
                .length;
            *fresh20 = (*fresh20)
                .wrapping_add(
                    (*run_stack
                        .offset(
                            (*stack_curr).wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ))
                        .length,
                );
            *stack_curr = (*stack_curr).wrapping_sub(1);
            *stack_curr;
        }
        if !((*store).storage).is_null() {
            free((*store).storage as *mut libc::c_void);
            (*store).storage = 0 as *mut xmlNodePtr;
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_domnode_tim_sort(
    mut dst: *mut xmlNodePtr,
    size: size_t,
) {
    let mut minrun: size_t = 0;
    let mut _store: TEMP_STORAGE_T = TEMP_STORAGE_T {
        alloc: 0,
        storage: 0 as *mut xmlNodePtr,
    };
    let mut store: *mut TEMP_STORAGE_T = 0 as *mut TEMP_STORAGE_T;
    let mut run_stack: [TIM_SORT_RUN_T; 128] = [TIM_SORT_RUN_T {
        start: 0,
        length: 0,
    }; 128];
    let mut stack_curr: size_t = 0 as libc::c_int as size_t;
    let mut curr: size_t = 0 as libc::c_int as size_t;
    if size <= 1 as libc::c_int as size_t {
        return;
    }
    if size < 64 as libc::c_int as size_t {
        libxml_domnode_binary_insertion_sort(dst, size);
        return;
    }
    minrun = compute_minrun(size) as size_t;
    store = &mut _store;
    (*store).alloc = 0 as libc::c_int as size_t;
    (*store).storage = 0 as *mut xmlNodePtr;
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        run_stack.as_mut_ptr(),
        &mut stack_curr,
        &mut curr,
    ) == 0
    {
        return;
    }
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        run_stack.as_mut_ptr(),
        &mut stack_curr,
        &mut curr,
    ) == 0
    {
        return;
    }
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        run_stack.as_mut_ptr(),
        &mut stack_curr,
        &mut curr,
    ) == 0
    {
        return;
    }
    loop {
        if libxml_domnode_check_invariant(
            run_stack.as_mut_ptr(),
            stack_curr as libc::c_int,
        ) == 0
        {
            stack_curr = libxml_domnode_tim_sort_collapse(
                dst,
                run_stack.as_mut_ptr(),
                stack_curr as libc::c_int,
                store,
                size,
            ) as size_t;
        } else if PUSH_NEXT(
            dst,
            size,
            store,
            minrun,
            run_stack.as_mut_ptr(),
            &mut stack_curr,
            &mut curr,
        ) == 0
        {
            return
        }
    };
}
#[no_mangle]
pub static mut xmlXPathNAN: libc::c_double = 0.;
#[no_mangle]
pub static mut xmlXPathPINF: libc::c_double = 0.;
#[no_mangle]
pub static mut xmlXPathNINF: libc::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn xmlXPathInit() {
    xmlXPathNAN = ::core::f32::NAN as libc::c_double;
    xmlXPathPINF = ::core::f32::INFINITY as libc::c_double;
    xmlXPathNINF = -::core::f32::INFINITY as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsNaN(mut val: libc::c_double) -> libc::c_int {
    return val.is_nan() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsInf(mut val: libc::c_double) -> libc::c_int {
    return if if val.is_infinite() {
        if val.is_sign_positive() { 1 } else { -1 }
    } else {
        0
    } != 0
    {
        if val > 0 as libc::c_int as libc::c_double {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }
    } else {
        0 as libc::c_int
    };
}
static mut xmlXPathXMLNamespaceStruct: xmlNs = {
    let mut init = _xmlNs {
        next: 0 as *const _xmlNs as *mut _xmlNs,
        type_0: XML_NAMESPACE_DECL,
        href: b"http://www.w3.org/XML/1998/namespace\0" as *const u8
            as *const libc::c_char as *const xmlChar,
        prefix: b"xml\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        context: 0 as *const _xmlDoc as *mut _xmlDoc,
    };
    init
};
static mut xmlXPathXMLNamespace: xmlNsPtr = unsafe {
    &xmlXPathXMLNamespaceStruct as *const xmlNs as *mut xmlNs
};
static mut xmlXPathErrorMessages: [*const libc::c_char; 26] = [
    b"Ok\n\0" as *const u8 as *const libc::c_char,
    b"Number encoding\n\0" as *const u8 as *const libc::c_char,
    b"Unfinished literal\n\0" as *const u8 as *const libc::c_char,
    b"Start of literal\n\0" as *const u8 as *const libc::c_char,
    b"Expected $ for variable reference\n\0" as *const u8 as *const libc::c_char,
    b"Undefined variable\n\0" as *const u8 as *const libc::c_char,
    b"Invalid predicate\n\0" as *const u8 as *const libc::c_char,
    b"Invalid expression\n\0" as *const u8 as *const libc::c_char,
    b"Missing closing curly brace\n\0" as *const u8 as *const libc::c_char,
    b"Unregistered function\n\0" as *const u8 as *const libc::c_char,
    b"Invalid operand\n\0" as *const u8 as *const libc::c_char,
    b"Invalid type\n\0" as *const u8 as *const libc::c_char,
    b"Invalid number of arguments\n\0" as *const u8 as *const libc::c_char,
    b"Invalid context size\n\0" as *const u8 as *const libc::c_char,
    b"Invalid context position\n\0" as *const u8 as *const libc::c_char,
    b"Memory allocation error\n\0" as *const u8 as *const libc::c_char,
    b"Syntax error\n\0" as *const u8 as *const libc::c_char,
    b"Resource error\n\0" as *const u8 as *const libc::c_char,
    b"Sub resource error\n\0" as *const u8 as *const libc::c_char,
    b"Undefined namespace prefix\n\0" as *const u8 as *const libc::c_char,
    b"Encoding error\n\0" as *const u8 as *const libc::c_char,
    b"Char out of XML range\n\0" as *const u8 as *const libc::c_char,
    b"Invalid or incomplete context\n\0" as *const u8 as *const libc::c_char,
    b"Stack usage error\n\0" as *const u8 as *const libc::c_char,
    b"Forbidden variable\n\0" as *const u8 as *const libc::c_char,
    b"?? Unknown error ??\n\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn xmlXPathErrMemory(
    mut ctxt: xmlXPathContextPtr,
    mut extra: *const libc::c_char,
) {
    if !ctxt.is_null() {
        if !extra.is_null() {
            let mut buf: [xmlChar; 200] = [0; 200];
            xmlStrPrintf(
                buf.as_mut_ptr(),
                200 as libc::c_int,
                b"Memory allocation failed : %s\n\0" as *const u8 as *const libc::c_char,
                extra,
            );
            (*ctxt).lastError.message = xmlStrdup(buf.as_mut_ptr()) as *mut libc::c_char;
        } else {
            (*ctxt)
                .lastError
                .message = xmlStrdup(
                b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char
                    as *mut xmlChar,
            ) as *mut libc::c_char;
        }
        (*ctxt).lastError.domain = XML_FROM_XPATH as libc::c_int;
        (*ctxt).lastError.code = XML_ERR_NO_MEMORY as libc::c_int;
        if ((*ctxt).error).is_some() {
            ((*ctxt).error)
                .expect(
                    "non-null function pointer",
                )((*ctxt).userData, &mut (*ctxt).lastError);
        }
    } else if !extra.is_null() {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            XML_ERR_NO_MEMORY as libc::c_int,
            XML_ERR_FATAL,
            0 as *const libc::c_char,
            0 as libc::c_int,
            extra,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"Memory allocation failed : %s\n\0" as *const u8 as *const libc::c_char,
            extra,
        );
    } else {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            XML_ERR_NO_MEMORY as libc::c_int,
            XML_ERR_FATAL,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn xmlXPathPErrMemory(
    mut ctxt: xmlXPathParserContextPtr,
    mut extra: *const libc::c_char,
) {
    if ctxt.is_null() {
        xmlXPathErrMemory(0 as xmlXPathContextPtr, extra);
    } else {
        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
        xmlXPathErrMemory((*ctxt).context, extra);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathErr(
    mut ctxt: xmlXPathParserContextPtr,
    mut error: libc::c_int,
) {
    if error < 0 as libc::c_int
        || error
            > (::core::mem::size_of::<[*const libc::c_char; 26]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ) as libc::c_int - 1 as libc::c_int
    {
        error = (::core::mem::size_of::<[*const libc::c_char; 26]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int - 1 as libc::c_int;
    }
    if ctxt.is_null() {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            error + XML_XPATH_EXPRESSION_OK as libc::c_int
                - XPATH_EXPRESSION_OK as libc::c_int,
            XML_ERR_ERROR,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            xmlXPathErrorMessages[error as usize],
        );
        return;
    }
    (*ctxt).error = error;
    if ((*ctxt).context).is_null() {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            error + XML_XPATH_EXPRESSION_OK as libc::c_int
                - XPATH_EXPRESSION_OK as libc::c_int,
            XML_ERR_ERROR,
            0 as *const libc::c_char,
            0 as libc::c_int,
            (*ctxt).base as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            ((*ctxt).cur).offset_from((*ctxt).base) as libc::c_long as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            xmlXPathErrorMessages[error as usize],
        );
        return;
    }
    xmlResetError(&mut (*(*ctxt).context).lastError);
    (*(*ctxt).context).lastError.domain = XML_FROM_XPATH as libc::c_int;
    (*(*ctxt).context)
        .lastError
        .code = error + XML_XPATH_EXPRESSION_OK as libc::c_int
        - XPATH_EXPRESSION_OK as libc::c_int;
    (*(*ctxt).context).lastError.level = XML_ERR_ERROR;
    (*(*ctxt).context).lastError.str1 = xmlStrdup((*ctxt).base) as *mut libc::c_char;
    (*(*ctxt).context)
        .lastError
        .int1 = ((*ctxt).cur).offset_from((*ctxt).base) as libc::c_long as libc::c_int;
    (*(*ctxt).context)
        .lastError
        .node = (*(*ctxt).context).debugNode as *mut libc::c_void;
    if ((*(*ctxt).context).error).is_some() {
        ((*(*ctxt).context).error)
            .expect(
                "non-null function pointer",
            )((*(*ctxt).context).userData, &mut (*(*ctxt).context).lastError);
    } else {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            (*(*ctxt).context).debugNode as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            error + XML_XPATH_EXPRESSION_OK as libc::c_int
                - XPATH_EXPRESSION_OK as libc::c_int,
            XML_ERR_ERROR,
            0 as *const libc::c_char,
            0 as libc::c_int,
            (*ctxt).base as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            ((*ctxt).cur).offset_from((*ctxt).base) as libc::c_long as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            xmlXPathErrorMessages[error as usize],
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPatherror(
    mut ctxt: xmlXPathParserContextPtr,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut no: libc::c_int,
) {
    xmlXPathErr(ctxt, no);
}
unsafe extern "C" fn xmlPointerListAddSize(
    mut list: xmlPointerListPtr,
    mut item: *mut libc::c_void,
    mut initialSize: libc::c_int,
) -> libc::c_int {
    if ((*list).items).is_null() {
        if initialSize <= 0 as libc::c_int {
            initialSize = 1 as libc::c_int;
        }
        (*list)
            .items = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (initialSize as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        if ((*list).items).is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListCreate: allocating item\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*list).number = 0 as libc::c_int;
        (*list).size = initialSize;
    } else if (*list).size <= (*list).number {
        if (*list).size > 50000000 as libc::c_int {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListAddSize: re-allocating item\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*list).size *= 2 as libc::c_int;
        (*list)
            .items = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*list).items as *mut libc::c_void,
            ((*list).size as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        if ((*list).items).is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"xmlPointerListAddSize: re-allocating item\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*list).size = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    let fresh21 = (*list).number;
    (*list).number = (*list).number + 1;
    let ref mut fresh22 = *((*list).items).offset(fresh21 as isize);
    *fresh22 = item;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlPointerListCreate(
    mut initialSize: libc::c_int,
) -> xmlPointerListPtr {
    let mut ret: xmlPointerListPtr = 0 as *mut xmlPointerList;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlPointerList>() as libc::c_ulong)
        as xmlPointerListPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"xmlPointerListCreate: allocating item\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as xmlPointerListPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlPointerList>() as libc::c_ulong,
    );
    if initialSize > 0 as libc::c_int {
        xmlPointerListAddSize(ret, 0 as *mut libc::c_void, initialSize);
        (*ret).number = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn xmlPointerListFree(mut list: xmlPointerListPtr) {
    if list.is_null() {
        return;
    }
    if !((*list).items).is_null() {
        xmlFree.expect("non-null function pointer")((*list).items as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(list as *mut libc::c_void);
}
unsafe extern "C" fn xmlXPathNewCompExpr() -> xmlXPathCompExprPtr {
    let mut cur: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathCompExpr>() as libc::c_ulong)
        as xmlXPathCompExprPtr;
    if cur.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"allocating component\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathCompExprPtr;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathCompExpr>() as libc::c_ulong,
    );
    (*cur).maxStep = 10 as libc::c_int;
    (*cur).nbStep = 0 as libc::c_int;
    (*cur)
        .steps = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*cur).maxStep as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xmlXPathStepOp>() as libc::c_ulong),
    ) as *mut xmlXPathStepOp;
    if ((*cur).steps).is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"allocating steps\n\0" as *const u8 as *const libc::c_char,
        );
        xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
        return 0 as xmlXPathCompExprPtr;
    }
    memset(
        (*cur).steps as *mut libc::c_void,
        0 as libc::c_int,
        ((*cur).maxStep as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xmlXPathStepOp>() as libc::c_ulong),
    );
    (*cur).last = -(1 as libc::c_int);
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeCompExpr(mut comp: xmlXPathCompExprPtr) {
    let mut op: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let mut i: libc::c_int = 0;
    if comp.is_null() {
        return;
    }
    if ((*comp).dict).is_null() {
        i = 0 as libc::c_int;
        while i < (*comp).nbStep {
            op = &mut *((*comp).steps).offset(i as isize) as *mut xmlXPathStepOp;
            if !((*op).value4).is_null() {
                if (*op).op as libc::c_uint
                    == XPATH_OP_VALUE as libc::c_int as libc::c_uint
                {
                    xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr);
                } else {
                    xmlFree.expect("non-null function pointer")((*op).value4);
                }
            }
            if !((*op).value5).is_null() {
                xmlFree.expect("non-null function pointer")((*op).value5);
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*comp).nbStep {
            op = &mut *((*comp).steps).offset(i as isize) as *mut xmlXPathStepOp;
            if !((*op).value4).is_null() {
                if (*op).op as libc::c_uint
                    == XPATH_OP_VALUE as libc::c_int as libc::c_uint
                {
                    xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr);
                }
            }
            i += 1;
            i;
        }
        xmlDictFree((*comp).dict);
    }
    if !((*comp).steps).is_null() {
        xmlFree.expect("non-null function pointer")((*comp).steps as *mut libc::c_void);
    }
    if !((*comp).stream).is_null() {
        xmlFreePatternList((*comp).stream);
    }
    if !((*comp).expr).is_null() {
        xmlFree.expect("non-null function pointer")((*comp).expr as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(comp as *mut libc::c_void);
}
unsafe extern "C" fn xmlXPathCompExprAdd(
    mut comp: xmlXPathCompExprPtr,
    mut ch1: libc::c_int,
    mut ch2: libc::c_int,
    mut op: xmlXPathOp,
    mut value: libc::c_int,
    mut value2: libc::c_int,
    mut value3: libc::c_int,
    mut value4: *mut libc::c_void,
    mut value5: *mut libc::c_void,
) -> libc::c_int {
    if (*comp).nbStep >= (*comp).maxStep {
        let mut real: *mut xmlXPathStepOp = 0 as *mut xmlXPathStepOp;
        if (*comp).maxStep >= 1000000 as libc::c_int {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"adding step\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*comp).maxStep *= 2 as libc::c_int;
        real = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*comp).steps as *mut libc::c_void,
            ((*comp).maxStep as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlXPathStepOp>() as libc::c_ulong),
        ) as *mut xmlXPathStepOp;
        if real.is_null() {
            (*comp).maxStep /= 2 as libc::c_int;
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"adding step\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*comp).steps = real;
    }
    (*comp).last = (*comp).nbStep;
    (*((*comp).steps).offset((*comp).nbStep as isize)).ch1 = ch1;
    (*((*comp).steps).offset((*comp).nbStep as isize)).ch2 = ch2;
    (*((*comp).steps).offset((*comp).nbStep as isize)).op = op;
    (*((*comp).steps).offset((*comp).nbStep as isize)).value = value;
    (*((*comp).steps).offset((*comp).nbStep as isize)).value2 = value2;
    (*((*comp).steps).offset((*comp).nbStep as isize)).value3 = value3;
    if !((*comp).dict).is_null()
        && (op as libc::c_uint == XPATH_OP_FUNCTION as libc::c_int as libc::c_uint
            || op as libc::c_uint == XPATH_OP_VARIABLE as libc::c_int as libc::c_uint
            || op as libc::c_uint == XPATH_OP_COLLECT as libc::c_int as libc::c_uint)
    {
        if !value4.is_null() {
            let ref mut fresh23 = (*((*comp).steps).offset((*comp).nbStep as isize))
                .value4;
            *fresh23 = xmlDictLookup(
                (*comp).dict,
                value4 as *const xmlChar,
                -(1 as libc::c_int),
            ) as *mut libc::c_void as *mut xmlChar as *mut libc::c_void;
            xmlFree.expect("non-null function pointer")(value4);
        } else {
            let ref mut fresh24 = (*((*comp).steps).offset((*comp).nbStep as isize))
                .value4;
            *fresh24 = 0 as *mut libc::c_void;
        }
        if !value5.is_null() {
            let ref mut fresh25 = (*((*comp).steps).offset((*comp).nbStep as isize))
                .value5;
            *fresh25 = xmlDictLookup(
                (*comp).dict,
                value5 as *const xmlChar,
                -(1 as libc::c_int),
            ) as *mut libc::c_void as *mut xmlChar as *mut libc::c_void;
            xmlFree.expect("non-null function pointer")(value5);
        } else {
            let ref mut fresh26 = (*((*comp).steps).offset((*comp).nbStep as isize))
                .value5;
            *fresh26 = 0 as *mut libc::c_void;
        }
    } else {
        let ref mut fresh27 = (*((*comp).steps).offset((*comp).nbStep as isize)).value4;
        *fresh27 = value4;
        let ref mut fresh28 = (*((*comp).steps).offset((*comp).nbStep as isize)).value5;
        *fresh28 = value5;
    }
    let ref mut fresh29 = (*((*comp).steps).offset((*comp).nbStep as isize)).cache;
    *fresh29 = None;
    let fresh30 = (*comp).nbStep;
    (*comp).nbStep = (*comp).nbStep + 1;
    return fresh30;
}
unsafe extern "C" fn xmlXPathCompSwap(mut op: xmlXPathStepOpPtr) {
    let mut tmp: libc::c_int = 0;
    tmp = (*op).ch1;
    (*op).ch1 = (*op).ch2;
    (*op).ch2 = tmp;
}
unsafe extern "C" fn xmlXPathDebugDumpNode(
    mut output: *mut FILE,
    mut cur: xmlNodePtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(output, b"Node is NULL !\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*cur).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        || (*cur).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(output, b" /\n\0" as *const u8 as *const libc::c_char);
    } else if (*cur).type_0 as libc::c_uint
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
    {
        xmlDebugDumpAttr(output, cur as xmlAttrPtr, depth);
    } else {
        xmlDebugDumpOneNode(output, cur, depth);
    };
}
unsafe extern "C" fn xmlXPathDebugDumpNodeList(
    mut output: *mut FILE,
    mut cur: xmlNodePtr,
    mut depth: libc::c_int,
) {
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(output, b"Node is NULL !\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    while !cur.is_null() {
        tmp = cur;
        cur = (*cur).next;
        xmlDebugDumpOneNode(output, tmp, depth);
    }
}
unsafe extern "C" fn xmlXPathDebugDumpNodeSet(
    mut output: *mut FILE,
    mut cur: xmlNodeSetPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(output, b"NodeSet is NULL !\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if !cur.is_null() {
        fprintf(
            output,
            b"Set contains %d nodes:\n\0" as *const u8 as *const libc::c_char,
            (*cur).nodeNr,
        );
        i = 0 as libc::c_int;
        while i < (*cur).nodeNr {
            fprintf(
                output,
                b"%s\0" as *const u8 as *const libc::c_char,
                shift.as_mut_ptr(),
            );
            fprintf(
                output,
                b"%d\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
            xmlXPathDebugDumpNode(
                output,
                *((*cur).nodeTab).offset(i as isize),
                depth + 1 as libc::c_int,
            );
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn xmlXPathDebugDumpValueTree(
    mut output: *mut FILE,
    mut cur: xmlNodeSetPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() || (*cur).nodeNr == 0 as libc::c_int
        || (*((*cur).nodeTab).offset(0 as libc::c_int as isize)).is_null()
    {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(output, b"Value Tree is NULL !\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
    fprintf(output, b"%d\0" as *const u8 as *const libc::c_char, i + 1 as libc::c_int);
    xmlXPathDebugDumpNodeList(
        output,
        (**((*cur).nodeTab).offset(0 as libc::c_int as isize)).children,
        depth + 1 as libc::c_int,
    );
}
unsafe extern "C" fn xmlXPathDebugDumpLocationSet(
    mut output: *mut FILE,
    mut cur: xmlLocationSetPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    if cur.is_null() {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(
            output,
            b"LocationSet is NULL !\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < (*cur).locNr {
        fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
        fprintf(
            output,
            b"%d : \0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        xmlXPathDebugDumpObject(
            output,
            *((*cur).locTab).offset(i as isize),
            depth + 1 as libc::c_int,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpObject(
    mut output: *mut FILE,
    mut cur: xmlXPathObjectPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    if output.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
    if cur.is_null() {
        fprintf(
            output,
            b"Object is empty (NULL)\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    match (*cur).type_0 as libc::c_uint {
        0 => {
            fprintf(
                output,
                b"Object is uninitialized\n\0" as *const u8 as *const libc::c_char,
            );
        }
        1 => {
            fprintf(
                output,
                b"Object is a Node Set :\n\0" as *const u8 as *const libc::c_char,
            );
            xmlXPathDebugDumpNodeSet(output, (*cur).nodesetval, depth);
        }
        9 => {
            fprintf(
                output,
                b"Object is an XSLT value tree :\n\0" as *const u8 as *const libc::c_char,
            );
            xmlXPathDebugDumpValueTree(output, (*cur).nodesetval, depth);
        }
        2 => {
            fprintf(
                output,
                b"Object is a Boolean : \0" as *const u8 as *const libc::c_char,
            );
            if (*cur).boolval != 0 {
                fprintf(output, b"true\n\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(output, b"false\n\0" as *const u8 as *const libc::c_char);
            }
        }
        3 => {
            match xmlXPathIsInf((*cur).floatval) {
                1 => {
                    fprintf(
                        output,
                        b"Object is a number : Infinity\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                -1 => {
                    fprintf(
                        output,
                        b"Object is a number : -Infinity\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                _ => {
                    if xmlXPathIsNaN((*cur).floatval) != 0 {
                        fprintf(
                            output,
                            b"Object is a number : NaN\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if (*cur).floatval == 0 as libc::c_int as libc::c_double {
                        fprintf(
                            output,
                            b"Object is a number : 0\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        fprintf(
                            output,
                            b"Object is a number : %0g\n\0" as *const u8
                                as *const libc::c_char,
                            (*cur).floatval,
                        );
                    }
                }
            }
        }
        4 => {
            fprintf(
                output,
                b"Object is a string : \0" as *const u8 as *const libc::c_char,
            );
            xmlDebugDumpString(output, (*cur).stringval);
            fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf(
                output,
                b"Object is a point : index %d in node\0" as *const u8
                    as *const libc::c_char,
                (*cur).index,
            );
            xmlXPathDebugDumpNode(
                output,
                (*cur).user as xmlNodePtr,
                depth + 1 as libc::c_int,
            );
            fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            if ((*cur).user2).is_null()
                || (*cur).user2 == (*cur).user && (*cur).index == (*cur).index2
            {
                fprintf(
                    output,
                    b"Object is a collapsed range :\n\0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    output,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    shift.as_mut_ptr(),
                );
                if (*cur).index >= 0 as libc::c_int {
                    fprintf(
                        output,
                        b"index %d in \0" as *const u8 as *const libc::c_char,
                        (*cur).index,
                    );
                }
                fprintf(output, b"node\n\0" as *const u8 as *const libc::c_char);
                xmlXPathDebugDumpNode(
                    output,
                    (*cur).user as xmlNodePtr,
                    depth + 1 as libc::c_int,
                );
            } else {
                fprintf(
                    output,
                    b"Object is a range :\n\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    output,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    shift.as_mut_ptr(),
                );
                fprintf(output, b"From \0" as *const u8 as *const libc::c_char);
                if (*cur).index >= 0 as libc::c_int {
                    fprintf(
                        output,
                        b"index %d in \0" as *const u8 as *const libc::c_char,
                        (*cur).index,
                    );
                }
                fprintf(output, b"node\n\0" as *const u8 as *const libc::c_char);
                xmlXPathDebugDumpNode(
                    output,
                    (*cur).user as xmlNodePtr,
                    depth + 1 as libc::c_int,
                );
                fprintf(
                    output,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    shift.as_mut_ptr(),
                );
                fprintf(output, b"To \0" as *const u8 as *const libc::c_char);
                if (*cur).index2 >= 0 as libc::c_int {
                    fprintf(
                        output,
                        b"index %d in \0" as *const u8 as *const libc::c_char,
                        (*cur).index2,
                    );
                }
                fprintf(output, b"node\n\0" as *const u8 as *const libc::c_char);
                xmlXPathDebugDumpNode(
                    output,
                    (*cur).user2 as xmlNodePtr,
                    depth + 1 as libc::c_int,
                );
                fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        7 => {
            fprintf(
                output,
                b"Object is a Location Set:\n\0" as *const u8 as *const libc::c_char,
            );
            xmlXPathDebugDumpLocationSet(
                output,
                (*cur).user as xmlLocationSetPtr,
                depth,
            );
        }
        8 => {
            fprintf(
                output,
                b"Object is user defined\n\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn xmlXPathDebugDumpStepOp(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut op: xmlXPathStepOpPtr,
    mut depth: libc::c_int,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
    if op.is_null() {
        fprintf(output, b"Step is NULL\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    match (*op).op as libc::c_uint {
        0 => {
            fprintf(output, b"END\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        1 => {
            fprintf(output, b"AND\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        2 => {
            fprintf(output, b"OR\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        3 => {
            if (*op).value != 0 {
                fprintf(output, b"EQUAL =\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(output, b"EQUAL !=\0" as *const u8 as *const libc::c_char);
            }
            current_block = 1658462350791934405;
        }
        4 => {
            if (*op).value != 0 {
                fprintf(output, b"CMP <\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(output, b"CMP >\0" as *const u8 as *const libc::c_char);
            }
            if (*op).value2 == 0 {
                fprintf(output, b"=\0" as *const u8 as *const libc::c_char);
            }
            current_block = 1658462350791934405;
        }
        5 => {
            if (*op).value == 0 as libc::c_int {
                fprintf(output, b"PLUS -\0" as *const u8 as *const libc::c_char);
            } else if (*op).value == 1 as libc::c_int {
                fprintf(output, b"PLUS +\0" as *const u8 as *const libc::c_char);
            } else if (*op).value == 2 as libc::c_int {
                fprintf(output, b"PLUS unary -\0" as *const u8 as *const libc::c_char);
            } else if (*op).value == 3 as libc::c_int {
                fprintf(output, b"PLUS unary - -\0" as *const u8 as *const libc::c_char);
            }
            current_block = 1658462350791934405;
        }
        6 => {
            if (*op).value == 0 as libc::c_int {
                fprintf(output, b"MULT *\0" as *const u8 as *const libc::c_char);
            } else if (*op).value == 1 as libc::c_int {
                fprintf(output, b"MULT div\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(output, b"MULT mod\0" as *const u8 as *const libc::c_char);
            }
            current_block = 1658462350791934405;
        }
        7 => {
            fprintf(output, b"UNION\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        8 => {
            fprintf(output, b"ROOT\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        9 => {
            fprintf(output, b"NODE\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        17 => {
            fprintf(output, b"SORT\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        10 => {
            let mut axis: xmlXPathAxisVal = (*op).value as xmlXPathAxisVal;
            let mut test: xmlXPathTestVal = (*op).value2 as xmlXPathTestVal;
            let mut type_0: xmlXPathTypeVal = (*op).value3 as xmlXPathTypeVal;
            let mut prefix: *const xmlChar = (*op).value4 as *const xmlChar;
            let mut name: *const xmlChar = (*op).value5 as *const xmlChar;
            fprintf(output, b"COLLECT \0" as *const u8 as *const libc::c_char);
            match axis as libc::c_uint {
                1 => {
                    fprintf(
                        output,
                        b" 'ancestors' \0" as *const u8 as *const libc::c_char,
                    );
                }
                2 => {
                    fprintf(
                        output,
                        b" 'ancestors-or-self' \0" as *const u8 as *const libc::c_char,
                    );
                }
                3 => {
                    fprintf(
                        output,
                        b" 'attributes' \0" as *const u8 as *const libc::c_char,
                    );
                }
                4 => {
                    fprintf(output, b" 'child' \0" as *const u8 as *const libc::c_char);
                }
                5 => {
                    fprintf(
                        output,
                        b" 'descendant' \0" as *const u8 as *const libc::c_char,
                    );
                }
                6 => {
                    fprintf(
                        output,
                        b" 'descendant-or-self' \0" as *const u8 as *const libc::c_char,
                    );
                }
                7 => {
                    fprintf(
                        output,
                        b" 'following' \0" as *const u8 as *const libc::c_char,
                    );
                }
                8 => {
                    fprintf(
                        output,
                        b" 'following-siblings' \0" as *const u8 as *const libc::c_char,
                    );
                }
                9 => {
                    fprintf(
                        output,
                        b" 'namespace' \0" as *const u8 as *const libc::c_char,
                    );
                }
                10 => {
                    fprintf(output, b" 'parent' \0" as *const u8 as *const libc::c_char);
                }
                11 => {
                    fprintf(
                        output,
                        b" 'preceding' \0" as *const u8 as *const libc::c_char,
                    );
                }
                12 => {
                    fprintf(
                        output,
                        b" 'preceding-sibling' \0" as *const u8 as *const libc::c_char,
                    );
                }
                13 => {
                    fprintf(output, b" 'self' \0" as *const u8 as *const libc::c_char);
                }
                _ => {}
            }
            match test as libc::c_uint {
                0 => {
                    fprintf(output, b"'none' \0" as *const u8 as *const libc::c_char);
                }
                1 => {
                    fprintf(output, b"'type' \0" as *const u8 as *const libc::c_char);
                }
                2 => {
                    fprintf(output, b"'PI' \0" as *const u8 as *const libc::c_char);
                }
                3 => {
                    fprintf(output, b"'all' \0" as *const u8 as *const libc::c_char);
                }
                4 => {
                    fprintf(
                        output,
                        b"'namespace' \0" as *const u8 as *const libc::c_char,
                    );
                }
                5 => {
                    fprintf(output, b"'name' \0" as *const u8 as *const libc::c_char);
                }
                _ => {}
            }
            match type_0 as libc::c_uint {
                0 => {
                    fprintf(output, b"'node' \0" as *const u8 as *const libc::c_char);
                }
                8 => {
                    fprintf(output, b"'comment' \0" as *const u8 as *const libc::c_char);
                }
                3 => {
                    fprintf(output, b"'text' \0" as *const u8 as *const libc::c_char);
                }
                7 => {
                    fprintf(output, b"'PI' \0" as *const u8 as *const libc::c_char);
                }
                _ => {}
            }
            if !prefix.is_null() {
                fprintf(output, b"%s:\0" as *const u8 as *const libc::c_char, prefix);
            }
            if !name.is_null() {
                fprintf(
                    output,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    name as *const libc::c_char,
                );
            }
            current_block = 1658462350791934405;
        }
        11 => {
            let mut object: xmlXPathObjectPtr = (*op).value4 as xmlXPathObjectPtr;
            fprintf(output, b"ELEM \0" as *const u8 as *const libc::c_char);
            xmlXPathDebugDumpObject(output, object, 0 as libc::c_int);
            current_block = 4518392077191315049;
        }
        12 => {
            let mut prefix_0: *const xmlChar = (*op).value5 as *const xmlChar;
            let mut name_0: *const xmlChar = (*op).value4 as *const xmlChar;
            if !prefix_0.is_null() {
                fprintf(
                    output,
                    b"VARIABLE %s:%s\0" as *const u8 as *const libc::c_char,
                    prefix_0,
                    name_0,
                );
            } else {
                fprintf(
                    output,
                    b"VARIABLE %s\0" as *const u8 as *const libc::c_char,
                    name_0,
                );
            }
            current_block = 1658462350791934405;
        }
        13 => {
            let mut nbargs: libc::c_int = (*op).value;
            let mut prefix_1: *const xmlChar = (*op).value5 as *const xmlChar;
            let mut name_1: *const xmlChar = (*op).value4 as *const xmlChar;
            if !prefix_1.is_null() {
                fprintf(
                    output,
                    b"FUNCTION %s:%s(%d args)\0" as *const u8 as *const libc::c_char,
                    prefix_1,
                    name_1,
                    nbargs,
                );
            } else {
                fprintf(
                    output,
                    b"FUNCTION %s(%d args)\0" as *const u8 as *const libc::c_char,
                    name_1,
                    nbargs,
                );
            }
            current_block = 1658462350791934405;
        }
        14 => {
            fprintf(output, b"ARG\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        15 => {
            fprintf(output, b"PREDICATE\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        16 => {
            fprintf(output, b"FILTER\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        18 => {
            fprintf(output, b"RANGETO\0" as *const u8 as *const libc::c_char);
            current_block = 1658462350791934405;
        }
        _ => {
            fprintf(
                output,
                b"UNKNOWN %d\n\0" as *const u8 as *const libc::c_char,
                (*op).op as libc::c_uint,
            );
            return;
        }
    }
    match current_block {
        1658462350791934405 => {
            fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    if (*op).ch1 >= 0 as libc::c_int {
        xmlXPathDebugDumpStepOp(
            output,
            comp,
            &mut *((*comp).steps).offset((*op).ch1 as isize),
            depth + 1 as libc::c_int,
        );
    }
    if (*op).ch2 >= 0 as libc::c_int {
        xmlXPathDebugDumpStepOp(
            output,
            comp,
            &mut *((*comp).steps).offset((*op).ch2 as isize),
            depth + 1 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpCompExpr(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shift: [libc::c_char; 100] = [0; 100];
    if output.is_null() || comp.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < depth && i < 25 as libc::c_int {
        shift[(2 as libc::c_int * i + 1 as libc::c_int)
            as usize] = ' ' as i32 as libc::c_char;
        shift[(2 as libc::c_int * i)
            as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    shift[(2 as libc::c_int * i + 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    shift[(2 as libc::c_int * i)
        as usize] = shift[(2 as libc::c_int * i + 1 as libc::c_int) as usize];
    fprintf(output, b"%s\0" as *const u8 as *const libc::c_char, shift.as_mut_ptr());
    if !((*comp).stream).is_null() {
        fprintf(output, b"Streaming Expression\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            output,
            b"Compiled Expression : %d elements\n\0" as *const u8 as *const libc::c_char,
            (*comp).nbStep,
        );
        i = (*comp).last;
        xmlXPathDebugDumpStepOp(
            output,
            comp,
            &mut *((*comp).steps).offset(i as isize),
            depth + 1 as libc::c_int,
        );
    };
}
unsafe extern "C" fn xmlXPathNewCache() -> xmlXPathContextCachePtr {
    let mut ret: xmlXPathContextCachePtr = 0 as *mut xmlXPathContextCache;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathContextCache>() as libc::c_ulong)
        as xmlXPathContextCachePtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating object cache\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathContextCachePtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathContextCache>() as libc::c_ulong,
    );
    (*ret).maxNodeset = 100 as libc::c_int;
    (*ret).maxString = 100 as libc::c_int;
    (*ret).maxBoolean = 100 as libc::c_int;
    (*ret).maxNumber = 100 as libc::c_int;
    (*ret).maxMisc = 100 as libc::c_int;
    return ret;
}
unsafe extern "C" fn xmlXPathCacheFreeObjectList(mut list: xmlPointerListPtr) {
    let mut i: libc::c_int = 0;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if list.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*list).number {
        obj = *((*list).items).offset(i as isize) as xmlXPathObjectPtr;
        if !((*obj).nodesetval).is_null() {
            if !((*(*obj).nodesetval).nodeTab).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*(*obj).nodesetval).nodeTab as *mut libc::c_void);
            }
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*obj).nodesetval as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void);
        i += 1;
        i;
    }
    xmlPointerListFree(list);
}
unsafe extern "C" fn xmlXPathFreeCache(mut cache: xmlXPathContextCachePtr) {
    if cache.is_null() {
        return;
    }
    if !((*cache).nodesetObjs).is_null() {
        xmlXPathCacheFreeObjectList((*cache).nodesetObjs);
    }
    if !((*cache).stringObjs).is_null() {
        xmlXPathCacheFreeObjectList((*cache).stringObjs);
    }
    if !((*cache).booleanObjs).is_null() {
        xmlXPathCacheFreeObjectList((*cache).booleanObjs);
    }
    if !((*cache).numberObjs).is_null() {
        xmlXPathCacheFreeObjectList((*cache).numberObjs);
    }
    if !((*cache).miscObjs).is_null() {
        xmlXPathCacheFreeObjectList((*cache).miscObjs);
    }
    xmlFree.expect("non-null function pointer")(cache as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathContextSetCache(
    mut ctxt: xmlXPathContextPtr,
    mut active: libc::c_int,
    mut value: libc::c_int,
    mut options: libc::c_int,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if active != 0 {
        let mut cache: xmlXPathContextCachePtr = 0 as *mut xmlXPathContextCache;
        if ((*ctxt).cache).is_null() {
            (*ctxt).cache = xmlXPathNewCache() as *mut libc::c_void;
            if ((*ctxt).cache).is_null() {
                return -(1 as libc::c_int);
            }
        }
        cache = (*ctxt).cache as xmlXPathContextCachePtr;
        if options == 0 as libc::c_int {
            if value < 0 as libc::c_int {
                value = 100 as libc::c_int;
            }
            (*cache).maxNodeset = value;
            (*cache).maxString = value;
            (*cache).maxNumber = value;
            (*cache).maxBoolean = value;
            (*cache).maxMisc = value;
        }
    } else if !((*ctxt).cache).is_null() {
        xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr);
        (*ctxt).cache = 0 as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXPathCacheWrapNodeSet(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlNodeSetPtr,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_NODESET;
            (*ret).nodesetval = val;
            return ret;
        }
    }
    return xmlXPathWrapNodeSet(val);
}
unsafe extern "C" fn xmlXPathCacheWrapString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *mut xmlChar,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).stringObjs).is_null()
            && (*(*cache).stringObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).stringObjs).number -= 1;
            ret = *((*(*cache).stringObjs).items)
                .offset((*(*cache).stringObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_STRING;
            (*ret).stringval = val;
            return ret;
        } else if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_STRING;
            (*ret_0).stringval = val;
            return ret_0;
        }
    }
    return xmlXPathWrapString(val);
}
unsafe extern "C" fn xmlXPathCacheNewNodeSet(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlNodePtr,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).nodesetObjs).is_null()
            && (*(*cache).nodesetObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).nodesetObjs).number -= 1;
            ret = *((*(*cache).nodesetObjs).items)
                .offset((*(*cache).nodesetObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_NODESET;
            (*ret).boolval = 0 as libc::c_int;
            if !val.is_null() {
                if (*(*ret).nodesetval).nodeMax == 0 as libc::c_int
                    || (*val).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    xmlXPathNodeSetAddUnique((*ret).nodesetval, val);
                } else {
                    let ref mut fresh31 = *((*(*ret).nodesetval).nodeTab)
                        .offset(0 as libc::c_int as isize);
                    *fresh31 = val;
                    (*(*ret).nodesetval).nodeNr = 1 as libc::c_int;
                }
            }
            return ret;
        } else if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_NODESET;
            (*ret_0).boolval = 0 as libc::c_int;
            (*ret_0).nodesetval = xmlXPathNodeSetCreate(val);
            if ((*ret_0).nodesetval).is_null() {
                (*ctxt).lastError.domain = XML_FROM_XPATH as libc::c_int;
                (*ctxt).lastError.code = XML_ERR_NO_MEMORY as libc::c_int;
                return 0 as xmlXPathObjectPtr;
            }
            return ret_0;
        }
    }
    return xmlXPathNewNodeSet(val);
}
unsafe extern "C" fn xmlXPathCacheNewCString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *const libc::c_char,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).stringObjs).is_null()
            && (*(*cache).stringObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).stringObjs).number -= 1;
            ret = *((*(*cache).stringObjs).items)
                .offset((*(*cache).stringObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_STRING;
            (*ret).stringval = xmlStrdup(val as *mut xmlChar);
            return ret;
        } else if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_STRING;
            (*ret_0).stringval = xmlStrdup(val as *mut xmlChar);
            return ret_0;
        }
    }
    return xmlXPathNewCString(val);
}
unsafe extern "C" fn xmlXPathCacheNewString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *const xmlChar,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).stringObjs).is_null()
            && (*(*cache).stringObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).stringObjs).number -= 1;
            ret = *((*(*cache).stringObjs).items)
                .offset((*(*cache).stringObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_STRING;
            if !val.is_null() {
                (*ret).stringval = xmlStrdup(val);
            } else {
                (*ret)
                    .stringval = xmlStrdup(
                    b"\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
            }
            return ret;
        } else if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_STRING;
            if !val.is_null() {
                (*ret_0).stringval = xmlStrdup(val);
            } else {
                (*ret_0)
                    .stringval = xmlStrdup(
                    b"\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
            }
            return ret_0;
        }
    }
    return xmlXPathNewString(val);
}
unsafe extern "C" fn xmlXPathCacheNewBoolean(
    mut ctxt: xmlXPathContextPtr,
    mut val: libc::c_int,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).booleanObjs).is_null()
            && (*(*cache).booleanObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).booleanObjs).number -= 1;
            ret = *((*(*cache).booleanObjs).items)
                .offset((*(*cache).booleanObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_BOOLEAN;
            (*ret).boolval = (val != 0 as libc::c_int) as libc::c_int;
            return ret;
        } else if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_BOOLEAN;
            (*ret_0).boolval = (val != 0 as libc::c_int) as libc::c_int;
            return ret_0;
        }
    }
    return xmlXPathNewBoolean(val);
}
unsafe extern "C" fn xmlXPathCacheNewFloat(
    mut ctxt: xmlXPathContextPtr,
    mut val: libc::c_double,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        if !((*cache).numberObjs).is_null()
            && (*(*cache).numberObjs).number != 0 as libc::c_int
        {
            let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).numberObjs).number -= 1;
            ret = *((*(*cache).numberObjs).items)
                .offset((*(*cache).numberObjs).number as isize) as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_NUMBER;
            (*ret).floatval = val;
            return ret;
        } else if !((*cache).miscObjs).is_null()
            && (*(*cache).miscObjs).number != 0 as libc::c_int
        {
            let mut ret_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *((*(*cache).miscObjs).items)
                .offset((*(*cache).miscObjs).number as isize) as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_NUMBER;
            (*ret_0).floatval = val;
            return ret_0;
        }
    }
    return xmlXPathNewFloat(val);
}
unsafe extern "C" fn xmlXPathCacheConvertString(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return xmlXPathCacheNewCString(ctxt, b"\0" as *const u8 as *const libc::c_char);
    }
    match (*val).type_0 as libc::c_uint {
        1 | 9 => {
            res = xmlXPathCastNodeSetToString((*val).nodesetval);
        }
        4 => return val,
        2 => {
            res = xmlXPathCastBooleanToString((*val).boolval);
        }
        3 => {
            res = xmlXPathCastNumberToString((*val).floatval);
        }
        8 | 5 | 6 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                2677 as libc::c_int,
            );
        }
        0 | _ => {}
    }
    xmlXPathReleaseObject(ctxt, val);
    if res.is_null() {
        return xmlXPathCacheNewCString(ctxt, b"\0" as *const u8 as *const libc::c_char);
    }
    return xmlXPathCacheWrapString(ctxt, res);
}
unsafe extern "C" fn xmlXPathCacheObjectCopy(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    if val.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if !ctxt.is_null() && !((*ctxt).cache).is_null() {
        match (*val).type_0 as libc::c_uint {
            1 => {
                return xmlXPathCacheWrapNodeSet(
                    ctxt,
                    xmlXPathNodeSetMerge(0 as xmlNodeSetPtr, (*val).nodesetval),
                );
            }
            4 => return xmlXPathCacheNewString(ctxt, (*val).stringval),
            2 => return xmlXPathCacheNewBoolean(ctxt, (*val).boolval),
            3 => return xmlXPathCacheNewFloat(ctxt, (*val).floatval),
            _ => {}
        }
    }
    return xmlXPathObjectCopy(val);
}
unsafe extern "C" fn xmlXPathCacheConvertBoolean(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathCacheNewBoolean(ctxt, 0 as libc::c_int);
    }
    if (*val).type_0 as libc::c_uint == XPATH_BOOLEAN as libc::c_int as libc::c_uint {
        return val;
    }
    ret = xmlXPathCacheNewBoolean(ctxt, xmlXPathCastToBoolean(val));
    xmlXPathReleaseObject(ctxt, val);
    return ret;
}
unsafe extern "C" fn xmlXPathCacheConvertNumber(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathCacheNewFloat(ctxt, 0.0f64);
    }
    if (*val).type_0 as libc::c_uint == XPATH_NUMBER as libc::c_int as libc::c_uint {
        return val;
    }
    ret = xmlXPathCacheNewFloat(ctxt, xmlXPathCastToNumber(val));
    xmlXPathReleaseObject(ctxt, val);
    return ret;
}
unsafe extern "C" fn xmlXPathSetFrame(
    mut ctxt: xmlXPathParserContextPtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if ctxt.is_null() {
        return 0 as libc::c_int;
    }
    ret = (*ctxt).valueFrame;
    (*ctxt).valueFrame = (*ctxt).valueNr;
    return ret;
}
unsafe extern "C" fn xmlXPathPopFrame(
    mut ctxt: xmlXPathParserContextPtr,
    mut frame: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2805 as libc::c_int,
            XPATH_STACK_ERROR as libc::c_int,
        );
    }
    (*ctxt).valueFrame = frame;
}
#[no_mangle]
pub unsafe extern "C" fn valuePop(
    mut ctxt: xmlXPathParserContextPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() || (*ctxt).valueNr <= 0 as libc::c_int {
        return 0 as xmlXPathObjectPtr;
    }
    if (*ctxt).valueNr <= (*ctxt).valueFrame {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2827 as libc::c_int,
            XPATH_STACK_ERROR as libc::c_int,
        );
        return 0 as xmlXPathObjectPtr;
    }
    (*ctxt).valueNr -= 1;
    (*ctxt).valueNr;
    if (*ctxt).valueNr > 0 as libc::c_int {
        (*ctxt)
            .value = *((*ctxt).valueTab)
            .offset(((*ctxt).valueNr - 1 as libc::c_int) as isize);
    } else {
        (*ctxt).value = 0 as xmlXPathObjectPtr;
    }
    ret = *((*ctxt).valueTab).offset((*ctxt).valueNr as isize);
    let ref mut fresh32 = *((*ctxt).valueTab).offset((*ctxt).valueNr as isize);
    *fresh32 = 0 as xmlXPathObjectPtr;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn valuePush(
    mut ctxt: xmlXPathParserContextPtr,
    mut value: xmlXPathObjectPtr,
) -> libc::c_int {
    if ctxt.is_null() || value.is_null() {
        return -(1 as libc::c_int);
    }
    if (*ctxt).valueNr >= (*ctxt).valueMax {
        let mut tmp: *mut xmlXPathObjectPtr = 0 as *mut xmlXPathObjectPtr;
        if (*ctxt).valueMax >= 1000000 as libc::c_int {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"XPath stack depth limit reached\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
            return 0 as libc::c_int;
        }
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).valueTab as *mut libc::c_void,
            ((2 as libc::c_int * (*ctxt).valueMax) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong,
                ),
        ) as *mut xmlXPathObjectPtr;
        if tmp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"pushing value\n\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
            return 0 as libc::c_int;
        }
        (*ctxt).valueMax *= 2 as libc::c_int;
        (*ctxt).valueTab = tmp;
    }
    let ref mut fresh33 = *((*ctxt).valueTab).offset((*ctxt).valueNr as isize);
    *fresh33 = value;
    (*ctxt).value = value;
    let fresh34 = (*ctxt).valueNr;
    (*ctxt).valueNr = (*ctxt).valueNr + 1;
    return fresh34;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopBoolean(
    mut ctxt: xmlXPathParserContextPtr,
) -> libc::c_int {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_int = 0;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2893 as libc::c_int,
            XPATH_INVALID_OPERAND as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if (*obj).type_0 as libc::c_uint != XPATH_BOOLEAN as libc::c_int as libc::c_uint {
        ret = xmlXPathCastToBoolean(obj);
    } else {
        ret = (*obj).boolval;
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopNumber(
    mut ctxt: xmlXPathParserContextPtr,
) -> libc::c_double {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_double = 0.;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2920 as libc::c_int,
            XPATH_INVALID_OPERAND as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_double;
    }
    if (*obj).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        ret = xmlXPathCastToNumber(obj);
    } else {
        ret = (*obj).floatval;
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopString(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2947 as libc::c_int,
            XPATH_INVALID_OPERAND as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
        }
        return 0 as *mut xmlChar;
    }
    ret = xmlXPathCastToString(obj);
    if (*obj).stringval == ret {
        (*obj).stringval = 0 as *mut xmlChar;
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopNodeSet(
    mut ctxt: xmlXPathParserContextPtr,
) -> xmlNodeSetPtr {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if ctxt.is_null() {
        return 0 as xmlNodeSetPtr;
    }
    if ((*ctxt).value).is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2974 as libc::c_int,
            XPATH_INVALID_OPERAND as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
        }
        return 0 as xmlNodeSetPtr;
    }
    if !(!((*ctxt).value).is_null()
        && ((*(*ctxt).value).type_0 as libc::c_uint
            == XPATH_NODESET as libc::c_int as libc::c_uint
            || (*(*ctxt).value).type_0 as libc::c_uint
                == XPATH_XSLT_TREE as libc::c_int as libc::c_uint))
    {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            2978 as libc::c_int,
            XPATH_INVALID_TYPE as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_TYPE as libc::c_int;
        }
        return 0 as xmlNodeSetPtr;
    }
    obj = valuePop(ctxt);
    ret = (*obj).nodesetval;
    (*obj).nodesetval = 0 as xmlNodeSetPtr;
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathPopExternal(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut libc::c_void {
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if ctxt.is_null() || ((*ctxt).value).is_null() {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            3008 as libc::c_int,
            XPATH_INVALID_OPERAND as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
        }
        return 0 as *mut libc::c_void;
    }
    if (*(*ctxt).value).type_0 as libc::c_uint
        != XPATH_USERS as libc::c_int as libc::c_uint
    {
        xmlXPatherror(
            ctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            3012 as libc::c_int,
            XPATH_INVALID_TYPE as libc::c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_TYPE as libc::c_int;
        }
        return 0 as *mut libc::c_void;
    }
    obj = valuePop(ctxt);
    ret = (*obj).user;
    (*obj).user = 0 as *mut libc::c_void;
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
}
unsafe extern "C" fn xmlXPathFormatNumber(
    mut number: libc::c_double,
    mut buffer: *mut libc::c_char,
    mut buffersize: libc::c_int,
) {
    match xmlXPathIsInf(number) {
        1 => {
            if buffersize
                > ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as libc::c_int
            {
                snprintf(
                    buffer,
                    buffersize as libc::c_ulong,
                    b"Infinity\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        -1 => {
            if buffersize
                > ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as libc::c_int
            {
                snprintf(
                    buffer,
                    buffersize as libc::c_ulong,
                    b"-Infinity\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        _ => {
            if xmlXPathIsNaN(number) != 0 {
                if buffersize
                    > ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                        as libc::c_int
                {
                    snprintf(
                        buffer,
                        buffersize as libc::c_ulong,
                        b"NaN\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if number == 0 as libc::c_int as libc::c_double {
                snprintf(
                    buffer,
                    buffersize as libc::c_ulong,
                    b"0\0" as *const u8 as *const libc::c_char,
                );
            } else if number
                > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                && number < 2147483647 as libc::c_int as libc::c_double
                && number == number as libc::c_int as libc::c_double
            {
                let mut work: [libc::c_char; 30] = [0; 30];
                let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut value: libc::c_int = number as libc::c_int;
                ptr = &mut *buffer.offset(0 as libc::c_int as isize)
                    as *mut libc::c_char;
                if value == 0 as libc::c_int {
                    let fresh35 = ptr;
                    ptr = ptr.offset(1);
                    *fresh35 = '0' as i32 as libc::c_char;
                } else {
                    snprintf(
                        work.as_mut_ptr(),
                        29 as libc::c_int as libc::c_ulong,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        value,
                    );
                    cur = &mut *work.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut libc::c_char;
                    while *cur as libc::c_int != 0
                        && (ptr.offset_from(buffer) as libc::c_long)
                            < buffersize as libc::c_long
                    {
                        let fresh36 = cur;
                        cur = cur.offset(1);
                        let fresh37 = ptr;
                        ptr = ptr.offset(1);
                        *fresh37 = *fresh36;
                    }
                }
                if (ptr.offset_from(buffer) as libc::c_long) < buffersize as libc::c_long
                {
                    *ptr = 0 as libc::c_int as libc::c_char;
                } else if buffersize > 0 as libc::c_int {
                    ptr = ptr.offset(-1);
                    ptr;
                    *ptr = 0 as libc::c_int as libc::c_char;
                }
            } else {
                let mut work_0: [libc::c_char; 28] = [0; 28];
                let mut integer_place: libc::c_int = 0;
                let mut fraction_place: libc::c_int = 0;
                let mut ptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut after_fraction: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut absolute_value: libc::c_double = 0.;
                let mut size: libc::c_int = 0;
                absolute_value = fabs(number);
                if (absolute_value > 1E9f64 || absolute_value < 1E-5f64)
                    && absolute_value != 0.0f64
                {
                    integer_place = 15 as libc::c_int
                        + (3 as libc::c_int + 2 as libc::c_int) + 1 as libc::c_int;
                    fraction_place = 15 as libc::c_int - 1 as libc::c_int;
                    size = snprintf(
                        work_0.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
                        b"%*.*e\0" as *const u8 as *const libc::c_char,
                        integer_place,
                        fraction_place,
                        number,
                    );
                    while size > 0 as libc::c_int
                        && work_0[size as usize] as libc::c_int != 'e' as i32
                    {
                        size -= 1;
                        size;
                    }
                } else {
                    if absolute_value > 0.0f64 {
                        integer_place = log10(absolute_value) as libc::c_int;
                        if integer_place > 0 as libc::c_int {
                            fraction_place = 15 as libc::c_int - integer_place
                                - 1 as libc::c_int;
                        } else {
                            fraction_place = 15 as libc::c_int - integer_place;
                        }
                    } else {
                        fraction_place = 1 as libc::c_int;
                    }
                    size = snprintf(
                        work_0.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
                        b"%0.*f\0" as *const u8 as *const libc::c_char,
                        fraction_place,
                        number,
                    );
                }
                while work_0[0 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
                    ptr_0 = &mut *work_0.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as *mut libc::c_char;
                    loop {
                        let ref mut fresh38 = *ptr_0.offset(0 as libc::c_int as isize);
                        *fresh38 = *ptr_0.offset(1 as libc::c_int as isize);
                        if !(*fresh38 != 0) {
                            break;
                        }
                        ptr_0 = ptr_0.offset(1);
                        ptr_0;
                    }
                    size -= 1;
                    size;
                }
                after_fraction = work_0.as_mut_ptr().offset(size as isize);
                ptr_0 = after_fraction;
                loop {
                    ptr_0 = ptr_0.offset(-1);
                    if !(*ptr_0 as libc::c_int == '0' as i32) {
                        break;
                    }
                }
                if *ptr_0 as libc::c_int != '.' as i32 {
                    ptr_0 = ptr_0.offset(1);
                    ptr_0;
                }
                loop {
                    let fresh39 = after_fraction;
                    after_fraction = after_fraction.offset(1);
                    let fresh40 = ptr_0;
                    ptr_0 = ptr_0.offset(1);
                    *fresh40 = *fresh39;
                    if !(*fresh40 as libc::c_int != 0 as libc::c_int) {
                        break;
                    }
                }
                size = (strlen(work_0.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                if size > buffersize {
                    work_0[(buffersize - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as libc::c_char;
                    size = buffersize;
                }
                memmove(
                    buffer as *mut libc::c_void,
                    work_0.as_mut_ptr() as *const libc::c_void,
                    size as libc::c_ulong,
                );
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathOrderDocElems(mut doc: xmlDocPtr) -> libc::c_long {
    let mut count: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    cur = (*doc).children;
    while !cur.is_null() {
        if (*cur).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            count += 1;
            (*cur).content = -count as *mut libc::c_void as *mut xmlChar;
            if !((*cur).children).is_null() {
                cur = (*cur).children;
                continue;
            }
        }
        if !((*cur).next).is_null() {
            cur = (*cur).next;
        } else {
            loop {
                cur = (*cur).parent;
                if cur.is_null() {
                    break;
                }
                if cur == doc as xmlNodePtr {
                    cur = 0 as xmlNodePtr;
                    break;
                } else if !((*cur).next).is_null() {
                    cur = (*cur).next;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCmpNodes(
    mut node1: xmlNodePtr,
    mut node2: xmlNodePtr,
) -> libc::c_int {
    let mut depth1: libc::c_int = 0;
    let mut depth2: libc::c_int = 0;
    let mut attr1: libc::c_int = 0 as libc::c_int;
    let mut attr2: libc::c_int = 0 as libc::c_int;
    let mut attrNode1: xmlNodePtr = 0 as xmlNodePtr;
    let mut attrNode2: xmlNodePtr = 0 as xmlNodePtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    if node1.is_null() || node2.is_null() {
        return -(2 as libc::c_int);
    }
    if node1 == node2 {
        return 0 as libc::c_int;
    }
    if (*node1).type_0 as libc::c_uint
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
    {
        attr1 = 1 as libc::c_int;
        attrNode1 = node1;
        node1 = (*node1).parent;
    }
    if (*node2).type_0 as libc::c_uint
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
    {
        attr2 = 1 as libc::c_int;
        attrNode2 = node2;
        node2 = (*node2).parent;
    }
    if node1 == node2 {
        if attr1 == attr2 {
            if attr1 != 0 as libc::c_int {
                cur = (*attrNode2).prev;
                while !cur.is_null() {
                    if cur == attrNode1 {
                        return 1 as libc::c_int;
                    }
                    cur = (*cur).prev;
                }
                return -(1 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        if attr2 == 1 as libc::c_int {
            return 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if (*node1).type_0 as libc::c_uint
        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        || (*node2).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if node1 == (*node2).prev {
        return 1 as libc::c_int;
    }
    if node1 == (*node2).next {
        return -(1 as libc::c_int);
    }
    if (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && (*node2).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && 0 as libc::c_int as ptrdiff_t > (*node1).content as ptrdiff_t
        && 0 as libc::c_int as ptrdiff_t > (*node2).content as ptrdiff_t
        && (*node1).doc == (*node2).doc
    {
        let mut l1: ptrdiff_t = 0;
        let mut l2: ptrdiff_t = 0;
        l1 = -((*node1).content as ptrdiff_t);
        l2 = -((*node2).content as ptrdiff_t);
        if l1 < l2 {
            return 1 as libc::c_int;
        }
        if l1 > l2 {
            return -(1 as libc::c_int);
        }
    }
    depth2 = 0 as libc::c_int;
    cur = node2;
    while !((*cur).parent).is_null() {
        if (*cur).parent == node1 {
            return 1 as libc::c_int;
        }
        depth2 += 1;
        depth2;
        cur = (*cur).parent;
    }
    root = cur;
    depth1 = 0 as libc::c_int;
    cur = node1;
    while !((*cur).parent).is_null() {
        if (*cur).parent == node2 {
            return -(1 as libc::c_int);
        }
        depth1 += 1;
        depth1;
        cur = (*cur).parent;
    }
    if root != cur {
        return -(2 as libc::c_int);
    }
    while depth1 > depth2 {
        depth1 -= 1;
        depth1;
        node1 = (*node1).parent;
    }
    while depth2 > depth1 {
        depth2 -= 1;
        depth2;
        node2 = (*node2).parent;
    }
    while (*node1).parent != (*node2).parent {
        node1 = (*node1).parent;
        node2 = (*node2).parent;
        if node1.is_null() || node2.is_null() {
            return -(2 as libc::c_int);
        }
    }
    if node1 == (*node2).prev {
        return 1 as libc::c_int;
    }
    if node1 == (*node2).next {
        return -(1 as libc::c_int);
    }
    if (*node1).type_0 as libc::c_uint == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && (*node2).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        && 0 as libc::c_int as ptrdiff_t > (*node1).content as ptrdiff_t
        && 0 as libc::c_int as ptrdiff_t > (*node2).content as ptrdiff_t
        && (*node1).doc == (*node2).doc
    {
        let mut l1_0: ptrdiff_t = 0;
        let mut l2_0: ptrdiff_t = 0;
        l1_0 = -((*node1).content as ptrdiff_t);
        l2_0 = -((*node2).content as ptrdiff_t);
        if l1_0 < l2_0 {
            return 1 as libc::c_int;
        }
        if l1_0 > l2_0 {
            return -(1 as libc::c_int);
        }
    }
    cur = (*node1).next;
    while !cur.is_null() {
        if cur == node2 {
            return 1 as libc::c_int;
        }
        cur = (*cur).next;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetSort(mut set: xmlNodeSetPtr) {
    if set.is_null() {
        return;
    }
    libxml_domnode_tim_sort((*set).nodeTab, (*set).nodeNr as size_t);
}
unsafe extern "C" fn xmlXPathNodeSetDupNs(
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> xmlNodePtr {
    let mut cur: xmlNsPtr = 0 as *mut xmlNs;
    if ns.is_null()
        || (*ns).type_0 as libc::c_uint
            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if node.is_null()
        || (*node).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return ns as xmlNodePtr;
    }
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlNs>() as libc::c_ulong) as xmlNsPtr;
    if cur.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"duplicating namespace\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlNodePtr;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlNs>() as libc::c_ulong,
    );
    (*cur).type_0 = XML_NAMESPACE_DECL;
    if !((*ns).href).is_null() {
        (*cur).href = xmlStrdup((*ns).href);
    }
    if !((*ns).prefix).is_null() {
        (*cur).prefix = xmlStrdup((*ns).prefix);
    }
    (*cur).next = node as xmlNsPtr;
    return cur as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetFreeNs(mut ns: xmlNsPtr) {
    if ns.is_null()
        || (*ns).type_0 as libc::c_uint
            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return;
    }
    if !((*ns).next).is_null()
        && (*(*ns).next).type_0 as libc::c_uint
            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        if !((*ns).href).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*ns).href as *mut xmlChar as *mut libc::c_void);
        }
        if !((*ns).prefix).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*ns).prefix as *mut xmlChar as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetCreate(mut val: xmlNodePtr) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlNodeSet>() as libc::c_ulong) as xmlNodeSetPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating nodeset\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlNodeSetPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlNodeSet>() as libc::c_ulong,
    );
    if !val.is_null() {
        (*ret)
            .nodeTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if ((*ret).nodeTab).is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"creating nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as xmlNodeSetPtr;
        }
        memset(
            (*ret).nodeTab as *mut libc::c_void,
            0 as libc::c_int,
            10 as libc::c_int as size_t
                * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
        );
        (*ret).nodeMax = 10 as libc::c_int;
        if (*val).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = val as xmlNsPtr;
            let fresh41 = (*ret).nodeNr;
            (*ret).nodeNr = (*ret).nodeNr + 1;
            let ref mut fresh42 = *((*ret).nodeTab).offset(fresh41 as isize);
            *fresh42 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
        } else {
            let fresh43 = (*ret).nodeNr;
            (*ret).nodeNr = (*ret).nodeNr + 1;
            let ref mut fresh44 = *((*ret).nodeTab).offset(fresh43 as isize);
            *fresh44 = val;
        }
    }
    return ret;
}
unsafe extern "C" fn xmlXPathNodeSetCreateSize(mut size: libc::c_int) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlNodeSet>() as libc::c_ulong) as xmlNodeSetPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating nodeset\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlNodeSetPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlNodeSet>() as libc::c_ulong,
    );
    if size < 10 as libc::c_int {
        size = 10 as libc::c_int;
    }
    (*ret)
        .nodeTab = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
    ) as *mut xmlNodePtr;
    if ((*ret).nodeTab).is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating nodeset\n\0" as *const u8 as *const libc::c_char,
        );
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        return 0 as xmlNodeSetPtr;
    }
    memset(
        (*ret).nodeTab as *mut libc::c_void,
        0 as libc::c_int,
        size as size_t * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
    );
    (*ret).nodeMax = size;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetContains(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if cur.is_null() || val.is_null() {
        return 0 as libc::c_int;
    }
    if (*val).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        i = 0 as libc::c_int;
        while i < (*cur).nodeNr {
            if (**((*cur).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                let mut ns1: xmlNsPtr = 0 as *mut xmlNs;
                let mut ns2: xmlNsPtr = 0 as *mut xmlNs;
                ns1 = val as xmlNsPtr;
                ns2 = *((*cur).nodeTab).offset(i as isize) as xmlNsPtr;
                if ns1 == ns2 {
                    return 1 as libc::c_int;
                }
                if !((*ns1).next).is_null() && (*ns2).next == (*ns1).next
                    && xmlStrEqual((*ns1).prefix, (*ns2).prefix) != 0
                {
                    return 1 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*cur).nodeNr {
            if *((*cur).nodeTab).offset(i as isize) == val {
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAddNs(
    mut cur: xmlNodeSetPtr,
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if cur.is_null() || ns.is_null() || node.is_null()
        || (*ns).type_0 as libc::c_uint
            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        || (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*cur).nodeNr {
        if !(*((*cur).nodeTab).offset(i as isize)).is_null()
            && (**((*cur).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            && (*(*((*cur).nodeTab).offset(i as isize) as xmlNsPtr)).next
                == node as xmlNsPtr
            && xmlStrEqual(
                (*ns).prefix,
                (*(*((*cur).nodeTab).offset(i as isize) as xmlNsPtr)).prefix,
            ) != 0
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if (*cur).nodeMax == 0 as libc::c_int {
        (*cur)
            .nodeTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if ((*cur).nodeTab).is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        memset(
            (*cur).nodeTab as *mut libc::c_void,
            0 as libc::c_int,
            10 as libc::c_int as size_t
                * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
        );
        (*cur).nodeMax = 10 as libc::c_int;
    } else if (*cur).nodeNr == (*cur).nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if (*cur).nodeMax >= 10000000 as libc::c_int {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset hit limit\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        temp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*cur).nodeTab as *mut libc::c_void,
            (((*cur).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if temp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*cur).nodeMax *= 2 as libc::c_int;
        (*cur).nodeTab = temp;
    }
    let fresh45 = (*cur).nodeNr;
    (*cur).nodeNr = (*cur).nodeNr + 1;
    let ref mut fresh46 = *((*cur).nodeTab).offset(fresh45 as isize);
    *fresh46 = xmlXPathNodeSetDupNs(node, ns);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAdd(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if cur.is_null() || val.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*cur).nodeNr {
        if *((*cur).nodeTab).offset(i as isize) == val {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if (*cur).nodeMax == 0 as libc::c_int {
        (*cur)
            .nodeTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if ((*cur).nodeTab).is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        memset(
            (*cur).nodeTab as *mut libc::c_void,
            0 as libc::c_int,
            10 as libc::c_int as size_t
                * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
        );
        (*cur).nodeMax = 10 as libc::c_int;
    } else if (*cur).nodeNr == (*cur).nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if (*cur).nodeMax >= 10000000 as libc::c_int {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset hit limit\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        temp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*cur).nodeTab as *mut libc::c_void,
            (((*cur).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if temp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*cur).nodeMax *= 2 as libc::c_int;
        (*cur).nodeTab = temp;
    }
    if (*val).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        let mut ns: xmlNsPtr = val as xmlNsPtr;
        let fresh47 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh48 = *((*cur).nodeTab).offset(fresh47 as isize);
        *fresh48 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
    } else {
        let fresh49 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh50 = *((*cur).nodeTab).offset(fresh49 as isize);
        *fresh50 = val;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetAddUnique(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> libc::c_int {
    if cur.is_null() || val.is_null() {
        return -(1 as libc::c_int);
    }
    if (*cur).nodeMax == 0 as libc::c_int {
        (*cur)
            .nodeTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if ((*cur).nodeTab).is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        memset(
            (*cur).nodeTab as *mut libc::c_void,
            0 as libc::c_int,
            10 as libc::c_int as size_t
                * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
        );
        (*cur).nodeMax = 10 as libc::c_int;
    } else if (*cur).nodeNr == (*cur).nodeMax {
        let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
        if (*cur).nodeMax >= 10000000 as libc::c_int {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset hit limit\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        temp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*cur).nodeTab as *mut libc::c_void,
            (((*cur).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if temp.is_null() {
            xmlXPathErrMemory(
                0 as xmlXPathContextPtr,
                b"growing nodeset\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*cur).nodeTab = temp;
        (*cur).nodeMax *= 2 as libc::c_int;
    }
    if (*val).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        let mut ns: xmlNsPtr = val as xmlNsPtr;
        let fresh51 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh52 = *((*cur).nodeTab).offset(fresh51 as isize);
        *fresh52 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
    } else {
        let fresh53 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh54 = *((*cur).nodeTab).offset(fresh53 as isize);
        *fresh54 = val;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetMerge(
    mut val1: xmlNodeSetPtr,
    mut val2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut initNr: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut n1: xmlNodePtr = 0 as *mut xmlNode;
    let mut n2: xmlNodePtr = 0 as *mut xmlNode;
    if val2.is_null() {
        return val1;
    }
    if val1.is_null() {
        val1 = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        if val1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
    }
    initNr = (*val1).nodeNr;
    i = 0 as libc::c_int;
    while i < (*val2).nodeNr {
        n2 = *((*val2).nodeTab).offset(i as isize);
        skip = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < initNr {
            n1 = *((*val1).nodeTab).offset(j as isize);
            if n1 == n2 {
                skip = 1 as libc::c_int;
                break;
            } else {
                if (*n1).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    && (*n2).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    if (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                        && xmlStrEqual(
                            (*(n1 as xmlNsPtr)).prefix,
                            (*(n2 as xmlNsPtr)).prefix,
                        ) != 0
                    {
                        skip = 1 as libc::c_int;
                        break;
                    }
                }
                j += 1;
                j;
            }
        }
        if !(skip != 0) {
            if (*val1).nodeMax == 0 as libc::c_int {
                (*val1)
                    .nodeTab = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(
                    (10 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                        ),
                ) as *mut xmlNodePtr;
                if ((*val1).nodeTab).is_null() {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 0 as xmlNodeSetPtr;
                }
                memset(
                    (*val1).nodeTab as *mut libc::c_void,
                    0 as libc::c_int,
                    10 as libc::c_int as size_t
                        * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                );
                (*val1).nodeMax = 10 as libc::c_int;
            } else if (*val1).nodeNr == (*val1).nodeMax {
                let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                if (*val1).nodeMax >= 10000000 as libc::c_int {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset hit limit\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as xmlNodeSetPtr;
                }
                temp = xmlRealloc
                    .expect(
                        "non-null function pointer",
                    )(
                    (*val1).nodeTab as *mut libc::c_void,
                    (((*val1).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                        ),
                ) as *mut xmlNodePtr;
                if temp.is_null() {
                    xmlXPathErrMemory(
                        0 as xmlXPathContextPtr,
                        b"merging nodeset\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 0 as xmlNodeSetPtr;
                }
                (*val1).nodeTab = temp;
                (*val1).nodeMax *= 2 as libc::c_int;
            }
            if (*n2).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                let mut ns: xmlNsPtr = n2 as xmlNsPtr;
                let fresh55 = (*val1).nodeNr;
                (*val1).nodeNr = (*val1).nodeNr + 1;
                let ref mut fresh56 = *((*val1).nodeTab).offset(fresh55 as isize);
                *fresh56 = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
            } else {
                let fresh57 = (*val1).nodeNr;
                (*val1).nodeNr = (*val1).nodeNr + 1;
                let ref mut fresh58 = *((*val1).nodeTab).offset(fresh57 as isize);
                *fresh58 = n2;
            }
        }
        i += 1;
        i;
    }
    return val1;
}
unsafe extern "C" fn xmlXPathNodeSetMergeAndClear(
    mut set1: xmlNodeSetPtr,
    mut set2: xmlNodeSetPtr,
    mut hasNullEntries: libc::c_int,
) -> xmlNodeSetPtr {
    let mut current_block: u64;
    if set1.is_null() && hasNullEntries == 0 as libc::c_int {
        set1 = xmlXPathNodeSetCreateSize((*set2).nodeNr);
        if set1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
        if (*set2).nodeNr != 0 as libc::c_int {
            memcpy(
                (*set1).nodeTab as *mut libc::c_void,
                (*set2).nodeTab as *const libc::c_void,
                ((*set2).nodeNr as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            );
            (*set1).nodeNr = (*set2).nodeNr;
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut initNbSet1: libc::c_int = 0;
        let mut n1: xmlNodePtr = 0 as *mut xmlNode;
        let mut n2: xmlNodePtr = 0 as *mut xmlNode;
        if set1.is_null() {
            set1 = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        }
        if set1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
        initNbSet1 = (*set1).nodeNr;
        i = 0 as libc::c_int;
        while i < (*set2).nodeNr {
            n2 = *((*set2).nodeTab).offset(i as isize);
            if !n2.is_null() {
                j = 0 as libc::c_int;
                loop {
                    if !(j < initNbSet1) {
                        current_block = 14401909646449704462;
                        break;
                    }
                    n1 = *((*set1).nodeTab).offset(j as isize);
                    if n1 == n2 {
                        current_block = 2968425633554183086;
                        break;
                    }
                    if (*n1).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        && (*n2).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    {
                        if (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                            && xmlStrEqual(
                                (*(n1 as xmlNsPtr)).prefix,
                                (*(n2 as xmlNsPtr)).prefix,
                            ) != 0
                        {
                            let ref mut fresh59 = *((*set2).nodeTab).offset(i as isize);
                            *fresh59 = 0 as xmlNodePtr;
                            xmlXPathNodeSetFreeNs(n2 as xmlNsPtr);
                            current_block = 2968425633554183086;
                            break;
                        }
                    }
                    j += 1;
                    j;
                }
                match current_block {
                    2968425633554183086 => {}
                    _ => {
                        if (*set1).nodeMax == 0 as libc::c_int {
                            (*set1)
                                .nodeTab = xmlMalloc
                                .expect(
                                    "non-null function pointer",
                                )(
                                (10 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                                    ),
                            ) as *mut xmlNodePtr;
                            if ((*set1).nodeTab).is_null() {
                                xmlXPathErrMemory(
                                    0 as xmlXPathContextPtr,
                                    b"merging nodeset\n\0" as *const u8 as *const libc::c_char,
                                );
                                return 0 as xmlNodeSetPtr;
                            }
                            memset(
                                (*set1).nodeTab as *mut libc::c_void,
                                0 as libc::c_int,
                                10 as libc::c_int as size_t
                                    * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                            );
                            (*set1).nodeMax = 10 as libc::c_int;
                        } else if (*set1).nodeNr >= (*set1).nodeMax {
                            let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                            if (*set1).nodeMax >= 10000000 as libc::c_int {
                                xmlXPathErrMemory(
                                    0 as xmlXPathContextPtr,
                                    b"merging nodeset hit limit\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as xmlNodeSetPtr;
                            }
                            temp = xmlRealloc
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*set1).nodeTab as *mut libc::c_void,
                                (((*set1).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                                    ),
                            ) as *mut xmlNodePtr;
                            if temp.is_null() {
                                xmlXPathErrMemory(
                                    0 as xmlXPathContextPtr,
                                    b"merging nodeset\n\0" as *const u8 as *const libc::c_char,
                                );
                                return 0 as xmlNodeSetPtr;
                            }
                            (*set1).nodeTab = temp;
                            (*set1).nodeMax *= 2 as libc::c_int;
                        }
                        let fresh60 = (*set1).nodeNr;
                        (*set1).nodeNr = (*set1).nodeNr + 1;
                        let ref mut fresh61 = *((*set1).nodeTab)
                            .offset(fresh60 as isize);
                        *fresh61 = n2;
                    }
                }
            }
            i += 1;
            i;
        }
    }
    (*set2).nodeNr = 0 as libc::c_int;
    return set1;
}
unsafe extern "C" fn xmlXPathNodeSetMergeAndClearNoDupls(
    mut set1: xmlNodeSetPtr,
    mut set2: xmlNodeSetPtr,
    mut hasNullEntries: libc::c_int,
) -> xmlNodeSetPtr {
    if set2.is_null() {
        return set1;
    }
    if set1.is_null() && hasNullEntries == 0 as libc::c_int {
        set1 = xmlXPathNodeSetCreateSize((*set2).nodeNr);
        if set1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
        if (*set2).nodeNr != 0 as libc::c_int {
            memcpy(
                (*set1).nodeTab as *mut libc::c_void,
                (*set2).nodeTab as *const libc::c_void,
                ((*set2).nodeNr as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
            );
            (*set1).nodeNr = (*set2).nodeNr;
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut n2: xmlNodePtr = 0 as *mut xmlNode;
        if set1.is_null() {
            set1 = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        }
        if set1.is_null() {
            return 0 as xmlNodeSetPtr;
        }
        i = 0 as libc::c_int;
        while i < (*set2).nodeNr {
            n2 = *((*set2).nodeTab).offset(i as isize);
            if !n2.is_null() {
                if (*set1).nodeMax == 0 as libc::c_int {
                    (*set1)
                        .nodeTab = xmlMalloc
                        .expect(
                            "non-null function pointer",
                        )(
                        (10 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                            ),
                    ) as *mut xmlNodePtr;
                    if ((*set1).nodeTab).is_null() {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 0 as xmlNodeSetPtr;
                    }
                    memset(
                        (*set1).nodeTab as *mut libc::c_void,
                        0 as libc::c_int,
                        10 as libc::c_int as size_t
                            * ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                    );
                    (*set1).nodeMax = 10 as libc::c_int;
                } else if (*set1).nodeNr >= (*set1).nodeMax {
                    let mut temp: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
                    if (*set1).nodeMax >= 10000000 as libc::c_int {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset hit limit\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 0 as xmlNodeSetPtr;
                    }
                    temp = xmlRealloc
                        .expect(
                            "non-null function pointer",
                        )(
                        (*set1).nodeTab as *mut libc::c_void,
                        (((*set1).nodeMax * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong,
                            ),
                    ) as *mut xmlNodePtr;
                    if temp.is_null() {
                        xmlXPathErrMemory(
                            0 as xmlXPathContextPtr,
                            b"merging nodeset\n\0" as *const u8 as *const libc::c_char,
                        );
                        return 0 as xmlNodeSetPtr;
                    }
                    (*set1).nodeTab = temp;
                    (*set1).nodeMax *= 2 as libc::c_int;
                }
                let fresh62 = (*set1).nodeNr;
                (*set1).nodeNr = (*set1).nodeNr + 1;
                let ref mut fresh63 = *((*set1).nodeTab).offset(fresh62 as isize);
                *fresh63 = n2;
            }
            i += 1;
            i;
        }
    }
    (*set2).nodeNr = 0 as libc::c_int;
    return set1;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetDel(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) {
    let mut i: libc::c_int = 0;
    if cur.is_null() {
        return;
    }
    if val.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*cur).nodeNr {
        if *((*cur).nodeTab).offset(i as isize) == val {
            break;
        }
        i += 1;
        i;
    }
    if i >= (*cur).nodeNr {
        return;
    }
    if !(*((*cur).nodeTab).offset(i as isize)).is_null()
        && (**((*cur).nodeTab).offset(i as isize)).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        xmlXPathNodeSetFreeNs(*((*cur).nodeTab).offset(i as isize) as xmlNsPtr);
    }
    (*cur).nodeNr -= 1;
    (*cur).nodeNr;
    while i < (*cur).nodeNr {
        let ref mut fresh64 = *((*cur).nodeTab).offset(i as isize);
        *fresh64 = *((*cur).nodeTab).offset((i + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    let ref mut fresh65 = *((*cur).nodeTab).offset((*cur).nodeNr as isize);
    *fresh65 = 0 as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeSetRemove(
    mut cur: xmlNodeSetPtr,
    mut val: libc::c_int,
) {
    if cur.is_null() {
        return;
    }
    if val >= (*cur).nodeNr {
        return;
    }
    if !(*((*cur).nodeTab).offset(val as isize)).is_null()
        && (**((*cur).nodeTab).offset(val as isize)).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        xmlXPathNodeSetFreeNs(*((*cur).nodeTab).offset(val as isize) as xmlNsPtr);
    }
    (*cur).nodeNr -= 1;
    (*cur).nodeNr;
    while val < (*cur).nodeNr {
        let ref mut fresh66 = *((*cur).nodeTab).offset(val as isize);
        *fresh66 = *((*cur).nodeTab).offset((val + 1 as libc::c_int) as isize);
        val += 1;
        val;
    }
    let ref mut fresh67 = *((*cur).nodeTab).offset((*cur).nodeNr as isize);
    *fresh67 = 0 as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeNodeSet(mut obj: xmlNodeSetPtr) {
    if obj.is_null() {
        return;
    }
    if !((*obj).nodeTab).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*obj).nodeNr {
            if !(*((*obj).nodeTab).offset(i as isize)).is_null()
                && (**((*obj).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                xmlXPathNodeSetFreeNs(*((*obj).nodeTab).offset(i as isize) as xmlNsPtr);
            }
            i += 1;
            i;
        }
        xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void);
}
unsafe extern "C" fn xmlXPathNodeSetClearFromPos(
    mut set: xmlNodeSetPtr,
    mut pos: libc::c_int,
    mut hasNsNodes: libc::c_int,
) {
    if set.is_null() || pos >= (*set).nodeNr {
        return
    } else if hasNsNodes != 0 {
        let mut i: libc::c_int = 0;
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        i = pos;
        while i < (*set).nodeNr {
            node = *((*set).nodeTab).offset(i as isize);
            if !node.is_null()
                && (*node).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                xmlXPathNodeSetFreeNs(node as xmlNsPtr);
            }
            i += 1;
            i;
        }
    }
    (*set).nodeNr = pos;
}
unsafe extern "C" fn xmlXPathNodeSetClear(
    mut set: xmlNodeSetPtr,
    mut hasNsNodes: libc::c_int,
) {
    xmlXPathNodeSetClearFromPos(set, 0 as libc::c_int, hasNsNodes);
}
unsafe extern "C" fn xmlXPathNodeSetKeepLast(mut set: xmlNodeSetPtr) {
    let mut i: libc::c_int = 0;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if set.is_null() || (*set).nodeNr <= 1 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*set).nodeNr - 1 as libc::c_int {
        node = *((*set).nodeTab).offset(i as isize);
        if !node.is_null()
            && (*node).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            xmlXPathNodeSetFreeNs(node as xmlNsPtr);
        }
        i += 1;
        i;
    }
    let ref mut fresh68 = *((*set).nodeTab).offset(0 as libc::c_int as isize);
    *fresh68 = *((*set).nodeTab).offset(((*set).nodeNr - 1 as libc::c_int) as isize);
    (*set).nodeNr = 1 as libc::c_int;
}
unsafe extern "C" fn xmlXPathFreeValueTree(mut obj: xmlNodeSetPtr) {
    let mut i: libc::c_int = 0;
    if obj.is_null() {
        return;
    }
    if !((*obj).nodeTab).is_null() {
        i = 0 as libc::c_int;
        while i < (*obj).nodeNr {
            if !(*((*obj).nodeTab).offset(i as isize)).is_null() {
                if (**((*obj).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    xmlXPathNodeSetFreeNs(
                        *((*obj).nodeTab).offset(i as isize) as xmlNsPtr,
                    );
                } else {
                    xmlFreeNodeList(*((*obj).nodeTab).offset(i as isize));
                }
            }
            i += 1;
            i;
        }
        xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewNodeSet(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating nodeset\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_NODESET;
    (*ret).boolval = 0 as libc::c_int;
    (*ret).nodesetval = xmlXPathNodeSetCreate(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewValueTree(mut val: xmlNodePtr) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating result value tree\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_XSLT_TREE;
    (*ret).boolval = 1 as libc::c_int;
    (*ret).user = val as *mut libc::c_void;
    (*ret).nodesetval = xmlXPathNodeSetCreate(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewNodeSetList(
    mut val: xmlNodeSetPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut i: libc::c_int = 0;
    if val.is_null() {
        ret = 0 as xmlXPathObjectPtr;
    } else if ((*val).nodeTab).is_null() {
        ret = xmlXPathNewNodeSet(0 as xmlNodePtr);
    } else {
        ret = xmlXPathNewNodeSet(*((*val).nodeTab).offset(0 as libc::c_int as isize));
        if !ret.is_null() {
            i = 1 as libc::c_int;
            while i < (*val).nodeNr {
                if xmlXPathNodeSetAddUnique(
                    (*ret).nodesetval,
                    *((*val).nodeTab).offset(i as isize),
                ) < 0 as libc::c_int
                {
                    break;
                }
                i += 1;
                i;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapNodeSet(
    mut val: xmlNodeSetPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating node set object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_NODESET;
    (*ret).nodesetval = val;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeNodeSetList(mut obj: xmlXPathObjectPtr) {
    if obj.is_null() {
        return;
    }
    xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDifference(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut i: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
        || ((*nodes2).nodeTab).is_null()
    {
        return nodes1;
    }
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if nodes1.is_null() || (*nodes1).nodeNr == 0 as libc::c_int
        || ((*nodes1).nodeTab).is_null()
    {
        return ret;
    }
    l1 = if !nodes1.is_null() { (*nodes1).nodeNr } else { 0 as libc::c_int };
    i = 0 as libc::c_int;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 as libc::c_int && i < (*nodes1).nodeNr {
            *((*nodes1).nodeTab).offset(i as isize)
        } else {
            0 as xmlNodePtr
        };
        if xmlXPathNodeSetContains(nodes2, cur) == 0 {
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 as libc::c_int {
                break;
            }
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathIntersection(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    let mut i: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if ret.is_null() {
        return ret;
    }
    if nodes1.is_null() || (*nodes1).nodeNr == 0 as libc::c_int
        || ((*nodes1).nodeTab).is_null()
    {
        return ret;
    }
    if nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
        || ((*nodes2).nodeTab).is_null()
    {
        return ret;
    }
    l1 = if !nodes1.is_null() { (*nodes1).nodeNr } else { 0 as libc::c_int };
    i = 0 as libc::c_int;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 as libc::c_int && i < (*nodes1).nodeNr {
            *((*nodes1).nodeTab).offset(i as isize)
        } else {
            0 as xmlNodePtr
        };
        if xmlXPathNodeSetContains(nodes2, cur) != 0 {
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 as libc::c_int {
                break;
            }
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDistinctSorted(
    mut nodes: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut hash: xmlHashTablePtr = 0 as *mut xmlHashTable;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut strval: *mut xmlChar = 0 as *mut xmlChar;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if nodes.is_null() || (*nodes).nodeNr == 0 as libc::c_int
        || ((*nodes).nodeTab).is_null()
    {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    l = if !nodes.is_null() { (*nodes).nodeNr } else { 0 as libc::c_int };
    hash = xmlHashCreate(l);
    i = 0 as libc::c_int;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 as libc::c_int && i < (*nodes).nodeNr {
            *((*nodes).nodeTab).offset(i as isize)
        } else {
            0 as xmlNodePtr
        };
        strval = xmlXPathCastNodeToString(cur);
        if (xmlHashLookup(hash, strval)).is_null() {
            xmlHashAddEntry(hash, strval, strval as *mut libc::c_void);
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 as libc::c_int {
                break;
            }
        } else {
            xmlFree.expect("non-null function pointer")(strval as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    xmlHashFree(
        hash,
        Some(
            xmlHashDefaultDeallocator
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDistinct(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr {
    if nodes.is_null() || (*nodes).nodeNr == 0 as libc::c_int
        || ((*nodes).nodeTab).is_null()
    {
        return nodes;
    }
    xmlXPathNodeSetSort(nodes);
    return xmlXPathDistinctSorted(nodes);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathHasSameNodes(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if nodes1.is_null() || (*nodes1).nodeNr == 0 as libc::c_int
        || ((*nodes1).nodeTab).is_null()
        || (nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
            || ((*nodes2).nodeTab).is_null())
    {
        return 0 as libc::c_int;
    }
    l = if !nodes1.is_null() { (*nodes1).nodeNr } else { 0 as libc::c_int };
    i = 0 as libc::c_int;
    while i < l {
        cur = if !nodes1.is_null() && i >= 0 as libc::c_int && i < (*nodes1).nodeNr {
            *((*nodes1).nodeTab).offset(i as isize)
        } else {
            0 as xmlNodePtr
        };
        if xmlXPathNodeSetContains(nodes2, cur) != 0 {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeLeadingSorted(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if node.is_null() {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null() || (*nodes).nodeNr == 0 as libc::c_int
        || ((*nodes).nodeTab).is_null() || xmlXPathNodeSetContains(nodes, node) == 0
    {
        return ret;
    }
    l = if !nodes.is_null() { (*nodes).nodeNr } else { 0 as libc::c_int };
    i = 0 as libc::c_int;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 as libc::c_int && i < (*nodes).nodeNr {
            *((*nodes).nodeTab).offset(i as isize)
        } else {
            0 as xmlNodePtr
        };
        if cur == node {
            break;
        }
        if xmlXPathNodeSetAddUnique(ret, cur) < 0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeLeading(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    xmlXPathNodeSetSort(nodes);
    return xmlXPathNodeLeadingSorted(nodes, node);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathLeadingSorted(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    if nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
        || ((*nodes2).nodeTab).is_null()
    {
        return nodes1;
    }
    return xmlXPathNodeLeadingSorted(
        nodes1,
        if !nodes2.is_null() && 1 as libc::c_int >= 0 as libc::c_int
            && (1 as libc::c_int) < (*nodes2).nodeNr
        {
            *((*nodes2).nodeTab).offset(1 as libc::c_int as isize)
        } else {
            0 as xmlNodePtr
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathLeading(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    if nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
        || ((*nodes2).nodeTab).is_null()
    {
        return nodes1;
    }
    if nodes1.is_null() || (*nodes1).nodeNr == 0 as libc::c_int
        || ((*nodes1).nodeTab).is_null()
    {
        return xmlXPathNodeSetCreate(0 as xmlNodePtr);
    }
    xmlXPathNodeSetSort(nodes1);
    xmlXPathNodeSetSort(nodes2);
    return xmlXPathNodeLeadingSorted(
        nodes1,
        if !nodes2.is_null() && 1 as libc::c_int >= 0 as libc::c_int
            && (1 as libc::c_int) < (*nodes2).nodeNr
        {
            *((*nodes2).nodeTab).offset(1 as libc::c_int as isize)
        } else {
            0 as xmlNodePtr
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeTrailingSorted(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if node.is_null() {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null() || (*nodes).nodeNr == 0 as libc::c_int
        || ((*nodes).nodeTab).is_null() || xmlXPathNodeSetContains(nodes, node) == 0
    {
        return ret;
    }
    l = if !nodes.is_null() { (*nodes).nodeNr } else { 0 as libc::c_int };
    i = l - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        cur = if !nodes.is_null() && i >= 0 as libc::c_int && i < (*nodes).nodeNr {
            *((*nodes).nodeTab).offset(i as isize)
        } else {
            0 as xmlNodePtr
        };
        if cur == node {
            break;
        }
        if xmlXPathNodeSetAddUnique(ret, cur) < 0 as libc::c_int {
            break;
        }
        i -= 1;
        i;
    }
    xmlXPathNodeSetSort(ret);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeTrailing(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr {
    xmlXPathNodeSetSort(nodes);
    return xmlXPathNodeTrailingSorted(nodes, node);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrailingSorted(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    if nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
        || ((*nodes2).nodeTab).is_null()
    {
        return nodes1;
    }
    return xmlXPathNodeTrailingSorted(
        nodes1,
        if !nodes2.is_null() && 0 as libc::c_int >= 0 as libc::c_int
            && (0 as libc::c_int) < (*nodes2).nodeNr
        {
            *((*nodes2).nodeTab).offset(0 as libc::c_int as isize)
        } else {
            0 as xmlNodePtr
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrailing(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr {
    if nodes2.is_null() || (*nodes2).nodeNr == 0 as libc::c_int
        || ((*nodes2).nodeTab).is_null()
    {
        return nodes1;
    }
    if nodes1.is_null() || (*nodes1).nodeNr == 0 as libc::c_int
        || ((*nodes1).nodeTab).is_null()
    {
        return xmlXPathNodeSetCreate(0 as xmlNodePtr);
    }
    xmlXPathNodeSetSort(nodes1);
    xmlXPathNodeSetSort(nodes2);
    return xmlXPathNodeTrailingSorted(
        nodes1,
        if !nodes2.is_null() && 0 as libc::c_int >= 0 as libc::c_int
            && (0 as libc::c_int) < (*nodes2).nodeNr
        {
            *((*nodes2).nodeTab).offset(0 as libc::c_int as isize)
        } else {
            0 as xmlNodePtr
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFunc(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut f: xmlXPathFunction,
) -> libc::c_int {
    return xmlXPathRegisterFuncNS(ctxt, name, 0 as *const xmlChar, f);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFuncNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut f: xmlXPathFunction,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*ctxt).funcHash).is_null() {
        (*ctxt).funcHash = xmlHashCreate(0 as libc::c_int);
    }
    if ((*ctxt).funcHash).is_null() {
        return -(1 as libc::c_int);
    }
    if f.is_none() {
        return xmlHashRemoveEntry2((*ctxt).funcHash, name, ns_uri, None);
    }
    return xmlHashAddEntry2(
        (*ctxt).funcHash,
        name,
        ns_uri,
        ::core::mem::transmute::<xmlXPathFunction, *mut libc::c_void>(f),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterFuncLookup(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathFuncLookupFunc,
    mut funcCtxt: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    (*ctxt).funcLookupFunc = f;
    (*ctxt).funcLookupData = funcCtxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFunctionLookup(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathFunction {
    if ctxt.is_null() {
        return None;
    }
    if ((*ctxt).funcLookupFunc).is_some() {
        let mut ret: xmlXPathFunction = None;
        let mut f: xmlXPathFuncLookupFunc = None;
        f = (*ctxt).funcLookupFunc;
        ret = f
            .expect(
                "non-null function pointer",
            )((*ctxt).funcLookupData, name, 0 as *const xmlChar);
        if ret.is_some() {
            return ret;
        }
    }
    return xmlXPathFunctionLookupNS(ctxt, name, 0 as *const xmlChar);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFunctionLookupNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathFunction {
    let mut ret: xmlXPathFunction = None;
    if ctxt.is_null() {
        return None;
    }
    if name.is_null() {
        return None;
    }
    if ((*ctxt).funcLookupFunc).is_some() {
        let mut f: xmlXPathFuncLookupFunc = None;
        f = (*ctxt).funcLookupFunc;
        ret = f
            .expect("non-null function pointer")((*ctxt).funcLookupData, name, ns_uri);
        if ret.is_some() {
            return ret;
        }
    }
    if ((*ctxt).funcHash).is_null() {
        return None;
    }
    ret = ::core::mem::transmute::<
        *mut libc::c_void,
        xmlXPathFunction,
    >(xmlHashLookup2((*ctxt).funcHash, name, ns_uri));
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredFuncsCleanup(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    xmlHashFree((*ctxt).funcHash, None);
    (*ctxt).funcHash = 0 as xmlHashTablePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariable(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> libc::c_int {
    return xmlXPathRegisterVariableNS(ctxt, name, 0 as *const xmlChar, value);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if name.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*ctxt).varHash).is_null() {
        (*ctxt).varHash = xmlHashCreate(0 as libc::c_int);
    }
    if ((*ctxt).varHash).is_null() {
        return -(1 as libc::c_int);
    }
    if value.is_null() {
        return xmlHashRemoveEntry2(
            (*ctxt).varHash,
            name,
            ns_uri,
            Some(
                xmlXPathFreeObjectEntry
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
        );
    }
    return xmlHashUpdateEntry2(
        (*ctxt).varHash,
        name,
        ns_uri,
        value as *mut libc::c_void,
        Some(
            xmlXPathFreeObjectEntry
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableLookup(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathVariableLookupFunc,
    mut data: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    (*ctxt).varLookupFunc = f;
    (*ctxt).varLookupData = data;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathVariableLookup(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathObjectPtr {
    if ctxt.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if ((*ctxt).varLookupFunc).is_some() {
        let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        ret = ((*ctxt).varLookupFunc)
            .expect(
                "non-null function pointer",
            )((*ctxt).varLookupData, name, 0 as *const xmlChar);
        return ret;
    }
    return xmlXPathVariableLookupNS(ctxt, name, 0 as *const xmlChar);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathVariableLookupNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathObjectPtr {
    if ctxt.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if ((*ctxt).varLookupFunc).is_some() {
        let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        ret = ((*ctxt).varLookupFunc)
            .expect("non-null function pointer")((*ctxt).varLookupData, name, ns_uri);
        if !ret.is_null() {
            return ret;
        }
    }
    if ((*ctxt).varHash).is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if name.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    return xmlXPathCacheObjectCopy(
        ctxt,
        xmlHashLookup2((*ctxt).varHash, name, ns_uri) as xmlXPathObjectPtr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredVariablesCleanup(
    mut ctxt: xmlXPathContextPtr,
) {
    if ctxt.is_null() {
        return;
    }
    xmlHashFree(
        (*ctxt).varHash,
        Some(
            xmlXPathFreeObjectEntry
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
    (*ctxt).varHash = 0 as xmlHashTablePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterNs(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if prefix.is_null() {
        return -(1 as libc::c_int);
    }
    if *prefix.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if ((*ctxt).nsHash).is_null() {
        (*ctxt).nsHash = xmlHashCreate(10 as libc::c_int);
    }
    if ((*ctxt).nsHash).is_null() {
        return -(1 as libc::c_int);
    }
    if ns_uri.is_null() {
        return xmlHashRemoveEntry(
            (*ctxt).nsHash,
            prefix,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
        );
    }
    return xmlHashUpdateEntry(
        (*ctxt).nsHash,
        prefix,
        xmlStrdup(ns_uri) as *mut libc::c_void,
        Some(
            xmlHashDefaultDeallocator
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNsLookup(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    if ctxt.is_null() {
        return 0 as *const xmlChar;
    }
    if prefix.is_null() {
        return 0 as *const xmlChar;
    }
    if xmlStrEqual(
        prefix,
        b"xml\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) != 0
    {
        return b"http://www.w3.org/XML/1998/namespace\0" as *const u8
            as *const libc::c_char as *const xmlChar;
    }
    if !((*ctxt).namespaces).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*ctxt).nsNr {
            if !(*((*ctxt).namespaces).offset(i as isize)).is_null()
                && xmlStrEqual(
                    (**((*ctxt).namespaces).offset(i as isize)).prefix,
                    prefix,
                ) != 0
            {
                return (**((*ctxt).namespaces).offset(i as isize)).href;
            }
            i += 1;
            i;
        }
    }
    return xmlHashLookup((*ctxt).nsHash, prefix) as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisteredNsCleanup(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    xmlHashFree(
        (*ctxt).nsHash,
        Some(
            xmlHashDefaultDeallocator
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
    (*ctxt).nsHash = 0 as xmlHashTablePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewFloat(mut val: libc::c_double) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating float object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_NUMBER;
    (*ret).floatval = val;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewBoolean(mut val: libc::c_int) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating boolean object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_BOOLEAN;
    (*ret).boolval = (val != 0 as libc::c_int) as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewString(
    mut val: *const xmlChar,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating string object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_STRING;
    if !val.is_null() {
        (*ret).stringval = xmlStrdup(val);
    } else {
        (*ret)
            .stringval = xmlStrdup(
            b"\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapString(mut val: *mut xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating string object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_STRING;
    (*ret).stringval = val;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewCString(
    mut val: *const libc::c_char,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating string object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_STRING;
    (*ret).stringval = xmlStrdup(val as *mut xmlChar);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapCString(
    mut val: *mut libc::c_char,
) -> xmlXPathObjectPtr {
    return xmlXPathWrapString(val as *mut xmlChar);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathWrapExternal(
    mut val: *mut libc::c_void,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating user object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    (*ret).type_0 = XPATH_USERS;
    (*ret).user = val;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathObjectCopy(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong)
        as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"copying object\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    memcpy(
        ret as *mut libc::c_void,
        val as *const libc::c_void,
        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
    );
    match (*val).type_0 as libc::c_uint {
        4 => {
            (*ret).stringval = xmlStrdup((*val).stringval);
        }
        9 | 1 => {
            (*ret)
                .nodesetval = xmlXPathNodeSetMerge(
                0 as xmlNodeSetPtr,
                (*val).nodesetval,
            );
            (*ret).boolval = 0 as libc::c_int;
        }
        7 => {
            let mut loc: xmlLocationSetPtr = (*val).user as xmlLocationSetPtr;
            (*ret)
                .user = xmlXPtrLocationSetMerge(0 as xmlLocationSetPtr, loc)
                as *mut libc::c_void;
        }
        8 => {
            (*ret).user = (*val).user;
        }
        0 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlXPathObjectCopy: unsupported type %d\n\0" as *const u8
                    as *const libc::c_char,
                (*val).type_0 as libc::c_uint,
            );
        }
        2 | 3 | 5 | 6 | _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeObject(mut obj: xmlXPathObjectPtr) {
    if obj.is_null() {
        return;
    }
    if (*obj).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*obj).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        if (*obj).boolval != 0 {
            (*obj).type_0 = XPATH_XSLT_TREE;
            if !((*obj).nodesetval).is_null() {
                xmlXPathFreeValueTree((*obj).nodesetval);
            }
        } else if !((*obj).nodesetval).is_null() {
            xmlXPathFreeNodeSet((*obj).nodesetval);
        }
    } else if (*obj).type_0 as libc::c_uint
        == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
    {
        if !((*obj).user).is_null() {
            xmlXPtrFreeLocationSet((*obj).user as xmlLocationSetPtr);
        }
    } else if (*obj).type_0 as libc::c_uint
        == XPATH_STRING as libc::c_int as libc::c_uint
    {
        if !((*obj).stringval).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*obj).stringval as *mut libc::c_void);
        }
    }
    xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void);
}
unsafe extern "C" fn xmlXPathFreeObjectEntry(
    mut obj: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    xmlXPathFreeObject(obj as xmlXPathObjectPtr);
}
unsafe extern "C" fn xmlXPathReleaseObject(
    mut ctxt: xmlXPathContextPtr,
    mut obj: xmlXPathObjectPtr,
) {
    let mut current_block: u64;
    if obj.is_null() {
        return;
    }
    if ctxt.is_null() || ((*ctxt).cache).is_null() {
        xmlXPathFreeObject(obj);
    } else {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache
            as xmlXPathContextCachePtr;
        match (*obj).type_0 as libc::c_uint {
            1 | 9 => {
                if !((*obj).nodesetval).is_null() {
                    if (*obj).boolval != 0 {
                        (*obj).type_0 = XPATH_XSLT_TREE;
                        xmlXPathFreeValueTree((*obj).nodesetval);
                        (*obj).nodesetval = 0 as xmlNodeSetPtr;
                        current_block = 9441801433784995173;
                    } else if (*(*obj).nodesetval).nodeMax <= 40 as libc::c_int
                        && (((*cache).nodesetObjs).is_null()
                            || (*(*cache).nodesetObjs).number < (*cache).maxNodeset)
                    {
                        if ((*cache).nodesetObjs).is_null() {
                            (*cache)
                                .nodesetObjs = xmlPointerListCreate(10 as libc::c_int);
                            if ((*cache).nodesetObjs).is_null() {
                                current_block = 10631297375674284166;
                            } else {
                                current_block = 7651349459974463963;
                            }
                        } else {
                            current_block = 7651349459974463963;
                        }
                        match current_block {
                            10631297375674284166 => {}
                            _ => {
                                if xmlPointerListAddSize(
                                    (*cache).nodesetObjs,
                                    obj as *mut libc::c_void,
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                                {
                                    current_block = 10631297375674284166;
                                } else {
                                    current_block = 14867087871115734529;
                                }
                            }
                        }
                    } else {
                        xmlXPathFreeNodeSet((*obj).nodesetval);
                        (*obj).nodesetval = 0 as xmlNodeSetPtr;
                        current_block = 9441801433784995173;
                    }
                } else {
                    current_block = 9441801433784995173;
                }
            }
            4 => {
                if !((*obj).stringval).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*obj).stringval as *mut libc::c_void);
                }
                if ((*cache).stringObjs).is_null()
                    || (*(*cache).stringObjs).number < (*cache).maxString
                {
                    if ((*cache).stringObjs).is_null() {
                        (*cache).stringObjs = xmlPointerListCreate(10 as libc::c_int);
                        if ((*cache).stringObjs).is_null() {
                            current_block = 10631297375674284166;
                        } else {
                            current_block = 13472856163611868459;
                        }
                    } else {
                        current_block = 13472856163611868459;
                    }
                    match current_block {
                        10631297375674284166 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).stringObjs,
                                obj as *mut libc::c_void,
                                0 as libc::c_int,
                            ) == -(1 as libc::c_int)
                            {
                                current_block = 10631297375674284166;
                            } else {
                                current_block = 14867087871115734529;
                            }
                        }
                    }
                } else {
                    current_block = 9441801433784995173;
                }
            }
            2 => {
                if ((*cache).booleanObjs).is_null()
                    || (*(*cache).booleanObjs).number < (*cache).maxBoolean
                {
                    if ((*cache).booleanObjs).is_null() {
                        (*cache).booleanObjs = xmlPointerListCreate(10 as libc::c_int);
                        if ((*cache).booleanObjs).is_null() {
                            current_block = 10631297375674284166;
                        } else {
                            current_block = 11459959175219260272;
                        }
                    } else {
                        current_block = 11459959175219260272;
                    }
                    match current_block {
                        10631297375674284166 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).booleanObjs,
                                obj as *mut libc::c_void,
                                0 as libc::c_int,
                            ) == -(1 as libc::c_int)
                            {
                                current_block = 10631297375674284166;
                            } else {
                                current_block = 14867087871115734529;
                            }
                        }
                    }
                } else {
                    current_block = 9441801433784995173;
                }
            }
            3 => {
                if ((*cache).numberObjs).is_null()
                    || (*(*cache).numberObjs).number < (*cache).maxNumber
                {
                    if ((*cache).numberObjs).is_null() {
                        (*cache).numberObjs = xmlPointerListCreate(10 as libc::c_int);
                        if ((*cache).numberObjs).is_null() {
                            current_block = 10631297375674284166;
                        } else {
                            current_block = 15597372965620363352;
                        }
                    } else {
                        current_block = 15597372965620363352;
                    }
                    match current_block {
                        10631297375674284166 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).numberObjs,
                                obj as *mut libc::c_void,
                                0 as libc::c_int,
                            ) == -(1 as libc::c_int)
                            {
                                current_block = 10631297375674284166;
                            } else {
                                current_block = 14867087871115734529;
                            }
                        }
                    }
                } else {
                    current_block = 9441801433784995173;
                }
            }
            7 => {
                if !((*obj).user).is_null() {
                    xmlXPtrFreeLocationSet((*obj).user as xmlLocationSetPtr);
                }
                current_block = 10631297375674284166;
            }
            _ => {
                current_block = 10631297375674284166;
            }
        }
        match current_block {
            9441801433784995173 => {
                if ((*cache).miscObjs).is_null()
                    || (*(*cache).miscObjs).number < (*cache).maxMisc
                {
                    if ((*cache).miscObjs).is_null() {
                        (*cache).miscObjs = xmlPointerListCreate(10 as libc::c_int);
                        if ((*cache).miscObjs).is_null() {
                            current_block = 10631297375674284166;
                        } else {
                            current_block = 6476622998065200121;
                        }
                    } else {
                        current_block = 6476622998065200121;
                    }
                    match current_block {
                        10631297375674284166 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).miscObjs,
                                obj as *mut libc::c_void,
                                0 as libc::c_int,
                            ) == -(1 as libc::c_int)
                            {
                                current_block = 10631297375674284166;
                            } else {
                                current_block = 14867087871115734529;
                            }
                        }
                    }
                } else {
                    current_block = 10631297375674284166;
                }
            }
            _ => {}
        }
        match current_block {
            10631297375674284166 => {
                if !((*obj).nodesetval).is_null() {
                    xmlXPathFreeNodeSet((*obj).nodesetval);
                }
                xmlFree.expect("non-null function pointer")(obj as *mut libc::c_void);
            }
            _ => {
                if !((*obj).nodesetval).is_null() {
                    let mut tmpset: xmlNodeSetPtr = (*obj).nodesetval;
                    if (*tmpset).nodeNr > 1 as libc::c_int {
                        let mut i: libc::c_int = 0;
                        let mut node: xmlNodePtr = 0 as *mut xmlNode;
                        i = 0 as libc::c_int;
                        while i < (*tmpset).nodeNr {
                            node = *((*tmpset).nodeTab).offset(i as isize);
                            if !node.is_null()
                                && (*node).type_0 as libc::c_uint
                                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                            {
                                xmlXPathNodeSetFreeNs(node as xmlNsPtr);
                            }
                            i += 1;
                            i;
                        }
                    } else if (*tmpset).nodeNr == 1 as libc::c_int {
                        if !(*((*tmpset).nodeTab).offset(0 as libc::c_int as isize))
                            .is_null()
                            && (**((*tmpset).nodeTab).offset(0 as libc::c_int as isize))
                                .type_0 as libc::c_uint
                                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            xmlXPathNodeSetFreeNs(
                                *((*tmpset).nodeTab).offset(0 as libc::c_int as isize)
                                    as xmlNsPtr,
                            );
                        }
                    }
                    (*tmpset).nodeNr = 0 as libc::c_int;
                    memset(
                        obj as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
                    );
                    (*obj).nodesetval = tmpset;
                } else {
                    memset(
                        obj as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<xmlXPathObject>() as libc::c_ulong,
                    );
                }
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastBooleanToString(
    mut val: libc::c_int,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if val != 0 {
        ret = xmlStrdup(b"true\0" as *const u8 as *const libc::c_char as *const xmlChar);
    } else {
        ret = xmlStrdup(
            b"false\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNumberToString(
    mut val: libc::c_double,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    match xmlXPathIsInf(val) {
        1 => {
            ret = xmlStrdup(
                b"Infinity\0" as *const u8 as *const libc::c_char as *const xmlChar,
            );
        }
        -1 => {
            ret = xmlStrdup(
                b"-Infinity\0" as *const u8 as *const libc::c_char as *const xmlChar,
            );
        }
        _ => {
            if xmlXPathIsNaN(val) != 0 {
                ret = xmlStrdup(
                    b"NaN\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
            } else if val == 0 as libc::c_int as libc::c_double {
                ret = xmlStrdup(
                    b"0\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
            } else {
                let mut buf: [libc::c_char; 100] = [0; 100];
                xmlXPathFormatNumber(val, buf.as_mut_ptr(), 99 as libc::c_int);
                buf[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                ret = xmlStrdup(buf.as_mut_ptr() as *const xmlChar);
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeToString(mut node: xmlNodePtr) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    ret = xmlNodeGetContent(node as *const xmlNode);
    if ret.is_null() {
        ret = xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *const xmlChar);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToString(
    mut ns: xmlNodeSetPtr,
) -> *mut xmlChar {
    if ns.is_null() || (*ns).nodeNr == 0 as libc::c_int || ((*ns).nodeTab).is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *const xmlChar);
    }
    if (*ns).nodeNr > 1 as libc::c_int {
        xmlXPathNodeSetSort(ns);
    }
    return xmlXPathCastNodeToString(*((*ns).nodeTab).offset(0 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToString(
    mut val: xmlXPathObjectPtr,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *const xmlChar);
    }
    match (*val).type_0 as libc::c_uint {
        0 => {
            ret = xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *const xmlChar);
        }
        1 | 9 => {
            ret = xmlXPathCastNodeSetToString((*val).nodesetval);
        }
        4 => return xmlStrdup((*val).stringval),
        2 => {
            ret = xmlXPathCastBooleanToString((*val).boolval);
        }
        3 => {
            ret = xmlXPathCastNumberToString((*val).floatval);
        }
        8 | 5 | 6 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                5804 as libc::c_int,
            );
            ret = xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *const xmlChar);
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertString(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut res: *mut xmlChar = 0 as *mut xmlChar;
    if val.is_null() {
        return xmlXPathNewCString(b"\0" as *const u8 as *const libc::c_char);
    }
    match (*val).type_0 as libc::c_uint {
        1 | 9 => {
            res = xmlXPathCastNodeSetToString((*val).nodesetval);
        }
        4 => return val,
        2 => {
            res = xmlXPathCastBooleanToString((*val).boolval);
        }
        3 => {
            res = xmlXPathCastNumberToString((*val).floatval);
        }
        8 | 5 | 6 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                5849 as libc::c_int,
            );
        }
        0 | _ => {}
    }
    xmlXPathFreeObject(val);
    if res.is_null() {
        return xmlXPathNewCString(b"\0" as *const u8 as *const libc::c_char);
    }
    return xmlXPathWrapString(res);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastBooleanToNumber(
    mut val: libc::c_int,
) -> libc::c_double {
    if val != 0 {
        return 1.0f64;
    }
    return 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastStringToNumber(
    mut val: *const xmlChar,
) -> libc::c_double {
    return xmlXPathStringEvalNumber(val);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeToNumber(
    mut node: xmlNodePtr,
) -> libc::c_double {
    let mut strval: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_double = 0.;
    if node.is_null() {
        return ::core::f32::NAN as libc::c_double;
    }
    strval = xmlXPathCastNodeToString(node);
    if strval.is_null() {
        return ::core::f32::NAN as libc::c_double;
    }
    ret = xmlXPathCastStringToNumber(strval);
    xmlFree.expect("non-null function pointer")(strval as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToNumber(
    mut ns: xmlNodeSetPtr,
) -> libc::c_double {
    let mut str: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_double = 0.;
    if ns.is_null() {
        return ::core::f32::NAN as libc::c_double;
    }
    str = xmlXPathCastNodeSetToString(ns);
    ret = xmlXPathCastStringToNumber(str);
    xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToNumber(
    mut val: xmlXPathObjectPtr,
) -> libc::c_double {
    let mut ret: libc::c_double = 0.0f64;
    if val.is_null() {
        return ::core::f32::NAN as libc::c_double;
    }
    match (*val).type_0 as libc::c_uint {
        0 => {
            ret = ::core::f32::NAN as libc::c_double;
        }
        1 | 9 => {
            ret = xmlXPathCastNodeSetToNumber((*val).nodesetval);
        }
        4 => {
            ret = xmlXPathCastStringToNumber((*val).stringval);
        }
        3 => {
            ret = (*val).floatval;
        }
        2 => {
            ret = xmlXPathCastBooleanToNumber((*val).boolval);
        }
        8 | 5 | 6 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                5969 as libc::c_int,
            );
            ret = ::core::f32::NAN as libc::c_double;
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertNumber(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathNewFloat(0.0f64);
    }
    if (*val).type_0 as libc::c_uint == XPATH_NUMBER as libc::c_int as libc::c_uint {
        return val;
    }
    ret = xmlXPathNewFloat(xmlXPathCastToNumber(val));
    xmlXPathFreeObject(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNumberToBoolean(
    mut val: libc::c_double,
) -> libc::c_int {
    if xmlXPathIsNaN(val) != 0 || val == 0.0f64 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastStringToBoolean(
    mut val: *const xmlChar,
) -> libc::c_int {
    if val.is_null() || xmlStrlen(val) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastNodeSetToBoolean(
    mut ns: xmlNodeSetPtr,
) -> libc::c_int {
    if ns.is_null() || (*ns).nodeNr == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCastToBoolean(
    mut val: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if val.is_null() {
        return 0 as libc::c_int;
    }
    match (*val).type_0 as libc::c_uint {
        0 => {
            ret = 0 as libc::c_int;
        }
        1 | 9 => {
            ret = xmlXPathCastNodeSetToBoolean((*val).nodesetval);
        }
        4 => {
            ret = xmlXPathCastStringToBoolean((*val).stringval);
        }
        3 => {
            ret = xmlXPathCastNumberToBoolean((*val).floatval);
        }
        2 => {
            ret = (*val).boolval;
        }
        8 | 5 | 6 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                6081 as libc::c_int,
            );
            ret = 0 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathConvertBoolean(
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if val.is_null() {
        return xmlXPathNewBoolean(0 as libc::c_int);
    }
    if (*val).type_0 as libc::c_uint == XPATH_BOOLEAN as libc::c_int as libc::c_uint {
        return val;
    }
    ret = xmlXPathNewBoolean(xmlXPathCastToBoolean(val));
    xmlXPathFreeObject(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewContext(mut doc: xmlDocPtr) -> xmlXPathContextPtr {
    let mut ret: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathContext>() as libc::c_ulong)
        as xmlXPathContextPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"creating context\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathContextPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathContext>() as libc::c_ulong,
    );
    (*ret).doc = doc;
    (*ret).node = 0 as xmlNodePtr;
    (*ret).varHash = 0 as xmlHashTablePtr;
    (*ret).nb_types = 0 as libc::c_int;
    (*ret).max_types = 0 as libc::c_int;
    (*ret).types = 0 as xmlXPathTypePtr;
    (*ret).funcHash = xmlHashCreate(0 as libc::c_int);
    (*ret).nb_axis = 0 as libc::c_int;
    (*ret).max_axis = 0 as libc::c_int;
    (*ret).axis = 0 as xmlXPathAxisPtr;
    (*ret).nsHash = 0 as xmlHashTablePtr;
    (*ret).user = 0 as *mut libc::c_void;
    (*ret).contextSize = -(1 as libc::c_int);
    (*ret).proximityPosition = -(1 as libc::c_int);
    xmlXPathRegisterAllFunctions(ret);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeContext(mut ctxt: xmlXPathContextPtr) {
    if ctxt.is_null() {
        return;
    }
    if !((*ctxt).cache).is_null() {
        xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr);
    }
    xmlXPathRegisteredNsCleanup(ctxt);
    xmlXPathRegisteredFuncsCleanup(ctxt);
    xmlXPathRegisteredVariablesCleanup(ctxt);
    xmlResetError(&mut (*ctxt).lastError);
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewParserContext(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong)
        as xmlXPathParserContextPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ctxt,
            b"creating parser context\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathParserContextPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong,
    );
    (*ret).base = str;
    (*ret).cur = (*ret).base;
    (*ret).context = ctxt;
    (*ret).comp = xmlXPathNewCompExpr();
    if ((*ret).comp).is_null() {
        xmlFree
            .expect("non-null function pointer")((*ret).valueTab as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        return 0 as xmlXPathParserContextPtr;
    }
    if !ctxt.is_null() && !((*ctxt).dict).is_null() {
        (*(*ret).comp).dict = (*ctxt).dict;
        xmlDictReference((*(*ret).comp).dict);
    }
    return ret;
}
unsafe extern "C" fn xmlXPathCompParserContext(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong)
        as xmlXPathParserContextPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ctxt,
            b"creating evaluation context\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathParserContextPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXPathParserContext>() as libc::c_ulong,
    );
    (*ret)
        .valueTab = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (10 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong),
    ) as *mut xmlXPathObjectPtr;
    if ((*ret).valueTab).is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        xmlXPathErrMemory(
            ctxt,
            b"creating evaluation context\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathParserContextPtr;
    }
    (*ret).valueNr = 0 as libc::c_int;
    (*ret).valueMax = 10 as libc::c_int;
    (*ret).value = 0 as xmlXPathObjectPtr;
    (*ret).valueFrame = 0 as libc::c_int;
    (*ret).context = ctxt;
    (*ret).comp = comp;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFreeParserContext(mut ctxt: xmlXPathParserContextPtr) {
    let mut i: libc::c_int = 0;
    if !((*ctxt).valueTab).is_null() {
        i = 0 as libc::c_int;
        while i < (*ctxt).valueNr {
            if !((*ctxt).context).is_null() {
                xmlXPathReleaseObject(
                    (*ctxt).context,
                    *((*ctxt).valueTab).offset(i as isize),
                );
            } else {
                xmlXPathFreeObject(*((*ctxt).valueTab).offset(i as isize));
            }
            i += 1;
            i;
        }
        xmlFree
            .expect("non-null function pointer")((*ctxt).valueTab as *mut libc::c_void);
    }
    if !((*ctxt).comp).is_null() {
        if !((*(*ctxt).comp).stream).is_null() {
            xmlFreePatternList((*(*ctxt).comp).stream);
            (*(*ctxt).comp).stream = 0 as xmlPatternPtr;
        }
        xmlXPathFreeCompExpr((*ctxt).comp);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn xmlXPathNodeValHash(mut node: xmlNodePtr) -> libc::c_uint {
    let mut len: libc::c_int = 2 as libc::c_int;
    let mut string: *const xmlChar = 0 as *const xmlChar;
    let mut tmp: xmlNodePtr = 0 as xmlNodePtr;
    let mut ret: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if node.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    if (*node).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        tmp = xmlDocGetRootElement(node as xmlDocPtr as *const xmlDoc);
        if tmp.is_null() {
            node = (*node).children;
        } else {
            node = tmp;
        }
        if node.is_null() {
            return 0 as libc::c_int as libc::c_uint;
        }
    }
    match (*node).type_0 as libc::c_uint {
        8 | 7 | 4 | 3 => {
            string = (*node).content;
            if string.is_null() {
                return 0 as libc::c_int as libc::c_uint;
            }
            if *string.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                return 0 as libc::c_int as libc::c_uint;
            }
            return (*string.offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*string.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
        }
        18 => {
            string = (*(node as xmlNsPtr)).href;
            if string.is_null() {
                return 0 as libc::c_int as libc::c_uint;
            }
            if *string.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                return 0 as libc::c_int as libc::c_uint;
            }
            return (*string.offset(0 as libc::c_int as isize) as libc::c_uint)
                .wrapping_add(
                    (*string.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
        }
        2 => {
            tmp = (*(node as xmlAttrPtr)).children;
        }
        1 => {
            tmp = (*node).children;
        }
        _ => return 0 as libc::c_int as libc::c_uint,
    }
    while !tmp.is_null() {
        match (*tmp).type_0 as libc::c_uint {
            4 | 3 => {
                string = (*tmp).content;
            }
            _ => {
                string = 0 as *const xmlChar;
            }
        }
        if !string.is_null()
            && *string.offset(0 as libc::c_int as isize) as libc::c_int
                != 0 as libc::c_int
        {
            if len == 1 as libc::c_int {
                return ret
                    .wrapping_add(
                        (*string.offset(0 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
            }
            if *string.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                len = 1 as libc::c_int;
                ret = *string.offset(0 as libc::c_int as isize) as libc::c_uint;
            } else {
                return (*string.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*string.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
            }
        }
        if !((*tmp).children).is_null()
            && (*tmp).type_0 as libc::c_uint
                != XML_DTD_NODE as libc::c_int as libc::c_uint
        {
            if (*(*tmp).children).type_0 as libc::c_uint
                != XML_ENTITY_DECL as libc::c_int as libc::c_uint
            {
                tmp = (*tmp).children;
                continue;
            }
        }
        if tmp == node {
            break;
        }
        if !((*tmp).next).is_null() {
            tmp = (*tmp).next;
        } else {
            loop {
                tmp = (*tmp).parent;
                if tmp.is_null() {
                    break;
                }
                if tmp == node {
                    tmp = 0 as xmlNodePtr;
                    break;
                } else if !((*tmp).next).is_null() {
                    tmp = (*tmp).next;
                    break;
                } else if tmp.is_null() {
                    break;
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlXPathStringHash(mut string: *const xmlChar) -> libc::c_uint {
    if string.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    return (*string.offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_add(
            (*string.offset(1 as libc::c_int as isize) as libc::c_uint)
                << 8 as libc::c_int,
        );
}
unsafe extern "C" fn xmlXPathCompareNodeSetFloat(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg: xmlXPathObjectPtr,
    mut f: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    if f.is_null() || arg.is_null()
        || (*arg).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, arg);
        xmlXPathReleaseObject((*ctxt).context, f);
        return 0 as libc::c_int;
    }
    ns = (*arg).nodesetval;
    if !ns.is_null() {
        i = 0 as libc::c_int;
        while i < (*ns).nodeNr {
            str2 = xmlXPathCastNodeToString(*((*ns).nodeTab).offset(i as isize));
            if !str2.is_null() {
                valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                valuePush(ctxt, xmlXPathCacheObjectCopy((*ctxt).context, f));
                ret = xmlXPathCompareValues(ctxt, inf, strict);
                if ret != 0 {
                    break;
                }
            }
            i += 1;
            i;
        }
    }
    xmlXPathReleaseObject((*ctxt).context, arg);
    xmlXPathReleaseObject((*ctxt).context, f);
    return ret;
}
unsafe extern "C" fn xmlXPathCompareNodeSetString(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg: xmlXPathObjectPtr,
    mut s: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    if s.is_null() || arg.is_null()
        || (*arg).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, arg);
        xmlXPathReleaseObject((*ctxt).context, s);
        return 0 as libc::c_int;
    }
    ns = (*arg).nodesetval;
    if !ns.is_null() {
        i = 0 as libc::c_int;
        while i < (*ns).nodeNr {
            str2 = xmlXPathCastNodeToString(*((*ns).nodeTab).offset(i as isize));
            if !str2.is_null() {
                valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                valuePush(ctxt, xmlXPathCacheObjectCopy((*ctxt).context, s));
                ret = xmlXPathCompareValues(ctxt, inf, strict);
                if ret != 0 {
                    break;
                }
            }
            i += 1;
            i;
        }
    }
    xmlXPathReleaseObject((*ctxt).context, arg);
    xmlXPathReleaseObject((*ctxt).context, s);
    return ret;
}
unsafe extern "C" fn xmlXPathCompareNodeSets(
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut init: libc::c_int = 0 as libc::c_int;
    let mut val1: libc::c_double = 0.;
    let mut values2: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns1: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut ns2: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if arg1.is_null()
        || (*arg1).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg1).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathFreeObject(arg2);
        return 0 as libc::c_int;
    }
    if arg2.is_null()
        || (*arg2).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg2).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as libc::c_int;
    }
    ns1 = (*arg1).nodesetval;
    ns2 = (*arg2).nodesetval;
    if ns1.is_null() || (*ns1).nodeNr <= 0 as libc::c_int {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as libc::c_int;
    }
    if ns2.is_null() || (*ns2).nodeNr <= 0 as libc::c_int {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as libc::c_int;
    }
    values2 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*ns2).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if values2.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"comparing nodesets\n\0" as *const u8 as *const libc::c_char,
        );
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*ns1).nodeNr {
        val1 = xmlXPathCastNodeToNumber(*((*ns1).nodeTab).offset(i as isize));
        if !(xmlXPathIsNaN(val1) != 0) {
            j = 0 as libc::c_int;
            while j < (*ns2).nodeNr {
                if init == 0 as libc::c_int {
                    *values2
                        .offset(
                            j as isize,
                        ) = xmlXPathCastNodeToNumber(
                        *((*ns2).nodeTab).offset(j as isize),
                    );
                }
                if !(xmlXPathIsNaN(*values2.offset(j as isize)) != 0) {
                    if inf != 0 && strict != 0 {
                        ret = (val1 < *values2.offset(j as isize)) as libc::c_int;
                    } else if inf != 0 && strict == 0 {
                        ret = (val1 <= *values2.offset(j as isize)) as libc::c_int;
                    } else if inf == 0 && strict != 0 {
                        ret = (val1 > *values2.offset(j as isize)) as libc::c_int;
                    } else if inf == 0 && strict == 0 {
                        ret = (val1 >= *values2.offset(j as isize)) as libc::c_int;
                    }
                    if ret != 0 {
                        break;
                    }
                }
                j += 1;
                j;
            }
            if ret != 0 {
                break;
            }
            init = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    xmlFree.expect("non-null function pointer")(values2 as *mut libc::c_void);
    xmlXPathFreeObject(arg1);
    xmlXPathFreeObject(arg2);
    return ret;
}
unsafe extern "C" fn xmlXPathCompareNodeSetValue(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
    mut arg: xmlXPathObjectPtr,
    mut val: xmlXPathObjectPtr,
) -> libc::c_int {
    if val.is_null() || arg.is_null()
        || (*arg).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    match (*val).type_0 as libc::c_uint {
        3 => return xmlXPathCompareNodeSetFloat(ctxt, inf, strict, arg, val),
        1 | 9 => return xmlXPathCompareNodeSets(inf, strict, arg, val),
        4 => return xmlXPathCompareNodeSetString(ctxt, inf, strict, arg, val),
        2 => {
            valuePush(ctxt, arg);
            xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
            valuePush(ctxt, val);
            return xmlXPathCompareValues(ctxt, inf, strict);
        }
        _ => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompareNodeSetValue: Can't compare node set and object of type %d\n\0"
                    as *const u8 as *const libc::c_char,
                (*val).type_0 as libc::c_uint,
            );
            xmlXPathReleaseObject((*ctxt).context, arg);
            xmlXPathReleaseObject((*ctxt).context, val);
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn xmlXPathEqualNodeSetString(
    mut arg: xmlXPathObjectPtr,
    mut str: *const xmlChar,
    mut neq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut hash: libc::c_uint = 0;
    if str.is_null() || arg.is_null()
        || (*arg).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    ns = (*arg).nodesetval;
    if ns.is_null() || (*ns).nodeNr <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    hash = xmlXPathStringHash(str);
    i = 0 as libc::c_int;
    while i < (*ns).nodeNr {
        if xmlXPathNodeValHash(*((*ns).nodeTab).offset(i as isize)) == hash {
            str2 = xmlNodeGetContent(
                *((*ns).nodeTab).offset(i as isize) as *const xmlNode,
            );
            if !str2.is_null() && xmlStrEqual(str, str2) != 0 {
                xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                if !(neq != 0) {
                    return 1 as libc::c_int;
                }
            } else if str2.is_null()
                && xmlStrEqual(
                    str,
                    b"\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            {
                if !(neq != 0) {
                    return 1 as libc::c_int;
                }
            } else {
                if neq != 0 {
                    if !str2.is_null() {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(str2 as *mut libc::c_void);
                    }
                    return 1 as libc::c_int;
                }
                if !str2.is_null() {
                    xmlFree
                        .expect("non-null function pointer")(str2 as *mut libc::c_void);
                }
            }
        } else if neq != 0 {
            return 1 as libc::c_int
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXPathEqualNodeSetFloat(
    mut ctxt: xmlXPathParserContextPtr,
    mut arg: xmlXPathObjectPtr,
    mut f: libc::c_double,
    mut neq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut str2: *mut xmlChar = 0 as *mut xmlChar;
    let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut v: libc::c_double = 0.;
    if arg.is_null()
        || (*arg).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    ns = (*arg).nodesetval;
    if !ns.is_null() {
        i = 0 as libc::c_int;
        while i < (*ns).nodeNr {
            str2 = xmlXPathCastNodeToString(*((*ns).nodeTab).offset(i as isize));
            if !str2.is_null() {
                valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                xmlFree.expect("non-null function pointer")(str2 as *mut libc::c_void);
                xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                val = valuePop(ctxt);
                v = (*val).floatval;
                xmlXPathReleaseObject((*ctxt).context, val);
                if xmlXPathIsNaN(v) == 0 {
                    if neq == 0 && v == f {
                        ret = 1 as libc::c_int;
                        break;
                    } else if neq != 0 && v != f {
                        ret = 1 as libc::c_int;
                        break;
                    }
                } else if neq != 0 {
                    ret = 1 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
    }
    return ret;
}
unsafe extern "C" fn xmlXPathEqualNodeSets(
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
    mut neq: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hashs1: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut hashs2: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut values1: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut values2: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ns1: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut ns2: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    if arg1.is_null()
        || (*arg1).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg1).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if arg2.is_null()
        || (*arg2).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg2).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    ns1 = (*arg1).nodesetval;
    ns2 = (*arg2).nodesetval;
    if ns1.is_null() || (*ns1).nodeNr <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if ns2.is_null() || (*ns2).nodeNr <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if neq == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*ns1).nodeNr {
            j = 0 as libc::c_int;
            while j < (*ns2).nodeNr {
                if *((*ns1).nodeTab).offset(i as isize)
                    == *((*ns2).nodeTab).offset(j as isize)
                {
                    return 1 as libc::c_int;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    values1 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*ns1).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
    ) as *mut *mut xmlChar;
    if values1.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"comparing nodesets\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    hashs1 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*ns1).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    if hashs1.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"comparing nodesets\n\0" as *const u8 as *const libc::c_char,
        );
        xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    memset(
        values1 as *mut libc::c_void,
        0 as libc::c_int,
        ((*ns1).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
    );
    values2 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*ns2).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
    ) as *mut *mut xmlChar;
    if values2.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"comparing nodesets\n\0" as *const u8 as *const libc::c_char,
        );
        xmlFree.expect("non-null function pointer")(hashs1 as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    hashs2 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*ns2).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    if hashs2.is_null() {
        xmlXPathErrMemory(
            0 as xmlXPathContextPtr,
            b"comparing nodesets\n\0" as *const u8 as *const libc::c_char,
        );
        xmlFree.expect("non-null function pointer")(hashs1 as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(values2 as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    memset(
        values2 as *mut libc::c_void,
        0 as libc::c_int,
        ((*ns2).nodeNr as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*ns1).nodeNr {
        *hashs1
            .offset(
                i as isize,
            ) = xmlXPathNodeValHash(*((*ns1).nodeTab).offset(i as isize));
        j = 0 as libc::c_int;
        while j < (*ns2).nodeNr {
            if i == 0 as libc::c_int {
                *hashs2
                    .offset(
                        j as isize,
                    ) = xmlXPathNodeValHash(*((*ns2).nodeTab).offset(j as isize));
            }
            if *hashs1.offset(i as isize) != *hashs2.offset(j as isize) {
                if neq != 0 {
                    ret = 1 as libc::c_int;
                    break;
                }
            } else {
                if (*values1.offset(i as isize)).is_null() {
                    let ref mut fresh69 = *values1.offset(i as isize);
                    *fresh69 = xmlNodeGetContent(
                        *((*ns1).nodeTab).offset(i as isize) as *const xmlNode,
                    );
                }
                if (*values2.offset(j as isize)).is_null() {
                    let ref mut fresh70 = *values2.offset(j as isize);
                    *fresh70 = xmlNodeGetContent(
                        *((*ns2).nodeTab).offset(j as isize) as *const xmlNode,
                    );
                }
                ret = xmlStrEqual(
                    *values1.offset(i as isize),
                    *values2.offset(j as isize),
                ) ^ neq;
                if ret != 0 {
                    break;
                }
            }
            j += 1;
            j;
        }
        if ret != 0 {
            break;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*ns1).nodeNr {
        if !(*values1.offset(i as isize)).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )(*values1.offset(i as isize) as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < (*ns2).nodeNr {
        if !(*values2.offset(j as isize)).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )(*values2.offset(j as isize) as *mut libc::c_void);
        }
        j += 1;
        j;
    }
    xmlFree.expect("non-null function pointer")(values1 as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(values2 as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(hashs1 as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(hashs2 as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn xmlXPathEqualValuesCommon(
    mut ctxt: xmlXPathParserContextPtr,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    match (*arg1).type_0 as libc::c_uint {
        2 => {
            match (*arg2).type_0 as libc::c_uint {
                2 => {
                    ret = ((*arg1).boolval == (*arg2).boolval) as libc::c_int;
                }
                3 => {
                    ret = ((*arg1).boolval
                        == xmlXPathCastNumberToBoolean((*arg2).floatval)) as libc::c_int;
                }
                4 => {
                    if ((*arg2).stringval).is_null()
                        || *((*arg2).stringval).offset(0 as libc::c_int as isize)
                            as libc::c_int == 0 as libc::c_int
                    {
                        ret = 0 as libc::c_int;
                    } else {
                        ret = 1 as libc::c_int;
                    }
                    ret = ((*arg1).boolval == ret) as libc::c_int;
                }
                8 | 5 | 6 | 7 => {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\0" as *const u8
                            as *const libc::c_char,
                        b"xpath.c\0" as *const u8 as *const libc::c_char,
                        7015 as libc::c_int,
                    );
                }
                0 | 1 | 9 | _ => {}
            }
        }
        3 => {
            let mut current_block_37: u64;
            match (*arg2).type_0 as libc::c_uint {
                2 => {
                    ret = ((*arg2).boolval
                        == xmlXPathCastNumberToBoolean((*arg1).floatval)) as libc::c_int;
                    current_block_37 = 15004371738079956865;
                }
                4 => {
                    valuePush(ctxt, arg2);
                    xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                    arg2 = valuePop(ctxt);
                    current_block_37 = 8365749633261168231;
                }
                3 => {
                    current_block_37 = 8365749633261168231;
                }
                8 | 5 | 6 | 7 => {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\0" as *const u8
                            as *const libc::c_char,
                        b"xpath.c\0" as *const u8 as *const libc::c_char,
                        7072 as libc::c_int,
                    );
                    current_block_37 = 15004371738079956865;
                }
                0 | 1 | 9 | _ => {
                    current_block_37 = 15004371738079956865;
                }
            }
            match current_block_37 {
                8365749633261168231 => {
                    if xmlXPathIsNaN((*arg1).floatval) != 0
                        || xmlXPathIsNaN((*arg2).floatval) != 0
                    {
                        ret = 0 as libc::c_int;
                    } else if xmlXPathIsInf((*arg1).floatval) == 1 as libc::c_int {
                        if xmlXPathIsInf((*arg2).floatval) == 1 as libc::c_int {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else if xmlXPathIsInf((*arg1).floatval) == -(1 as libc::c_int) {
                        if xmlXPathIsInf((*arg2).floatval) == -(1 as libc::c_int) {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == 1 as libc::c_int {
                        if xmlXPathIsInf((*arg1).floatval) == 1 as libc::c_int {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == -(1 as libc::c_int) {
                        if xmlXPathIsInf((*arg1).floatval) == -(1 as libc::c_int) {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else {
                        ret = ((*arg1).floatval == (*arg2).floatval) as libc::c_int;
                    }
                }
                _ => {}
            }
        }
        4 => {
            match (*arg2).type_0 as libc::c_uint {
                2 => {
                    if ((*arg1).stringval).is_null()
                        || *((*arg1).stringval).offset(0 as libc::c_int as isize)
                            as libc::c_int == 0 as libc::c_int
                    {
                        ret = 0 as libc::c_int;
                    } else {
                        ret = 1 as libc::c_int;
                    }
                    ret = ((*arg2).boolval == ret) as libc::c_int;
                }
                4 => {
                    ret = xmlStrEqual((*arg1).stringval, (*arg2).stringval);
                }
                3 => {
                    valuePush(ctxt, arg1);
                    xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                    arg1 = valuePop(ctxt);
                    if xmlXPathIsNaN((*arg1).floatval) != 0
                        || xmlXPathIsNaN((*arg2).floatval) != 0
                    {
                        ret = 0 as libc::c_int;
                    } else if xmlXPathIsInf((*arg1).floatval) == 1 as libc::c_int {
                        if xmlXPathIsInf((*arg2).floatval) == 1 as libc::c_int {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else if xmlXPathIsInf((*arg1).floatval) == -(1 as libc::c_int) {
                        if xmlXPathIsInf((*arg2).floatval) == -(1 as libc::c_int) {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == 1 as libc::c_int {
                        if xmlXPathIsInf((*arg1).floatval) == 1 as libc::c_int {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == -(1 as libc::c_int) {
                        if xmlXPathIsInf((*arg1).floatval) == -(1 as libc::c_int) {
                            ret = 1 as libc::c_int;
                        } else {
                            ret = 0 as libc::c_int;
                        }
                    } else {
                        ret = ((*arg1).floatval == (*arg2).floatval) as libc::c_int;
                    }
                }
                8 | 5 | 6 | 7 => {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\0" as *const u8
                            as *const libc::c_char,
                        b"xpath.c\0" as *const u8 as *const libc::c_char,
                        7133 as libc::c_int,
                    );
                }
                0 | 1 | 9 | _ => {}
            }
        }
        8 | 5 | 6 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                7144 as libc::c_int,
            );
        }
        0 | 1 | 9 | _ => {}
    }
    xmlXPathReleaseObject((*ctxt).context, arg1);
    xmlXPathReleaseObject((*ctxt).context, arg2);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEqualValues(
    mut ctxt: xmlXPathParserContextPtr,
) -> libc::c_int {
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut argtmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as libc::c_int;
    }
    arg2 = valuePop(ctxt);
    arg1 = valuePop(ctxt);
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            xmlXPathReleaseObject((*ctxt).context, arg1);
        } else {
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return 0 as libc::c_int;
    }
    if arg1 == arg2 {
        xmlXPathFreeObject(arg1);
        return 1 as libc::c_int;
    }
    if (*arg2).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*arg2).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        || (*arg1).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*arg1).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        if (*arg1).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg1).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp;
        }
        match (*arg2).type_0 as libc::c_uint {
            1 | 9 => {
                ret = xmlXPathEqualNodeSets(arg1, arg2, 0 as libc::c_int);
            }
            2 => {
                if ((*arg1).nodesetval).is_null()
                    || (*(*arg1).nodesetval).nodeNr == 0 as libc::c_int
                {
                    ret = 0 as libc::c_int;
                } else {
                    ret = 1 as libc::c_int;
                }
                ret = (ret == (*arg2).boolval) as libc::c_int;
            }
            3 => {
                ret = xmlXPathEqualNodeSetFloat(
                    ctxt,
                    arg1,
                    (*arg2).floatval,
                    0 as libc::c_int,
                );
            }
            4 => {
                ret = xmlXPathEqualNodeSetString(
                    arg1,
                    (*arg2).stringval,
                    0 as libc::c_int,
                );
            }
            8 | 5 | 6 | 7 => {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8
                        as *const libc::c_char,
                    b"xpath.c\0" as *const u8 as *const libc::c_char,
                    7229 as libc::c_int,
                );
            }
            0 | _ => {}
        }
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
        return ret;
    }
    return xmlXPathEqualValuesCommon(ctxt, arg1, arg2);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNotEqualValues(
    mut ctxt: xmlXPathParserContextPtr,
) -> libc::c_int {
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut argtmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as libc::c_int;
    }
    arg2 = valuePop(ctxt);
    arg1 = valuePop(ctxt);
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            xmlXPathReleaseObject((*ctxt).context, arg1);
        } else {
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return 0 as libc::c_int;
    }
    if arg1 == arg2 {
        xmlXPathReleaseObject((*ctxt).context, arg1);
        return 0 as libc::c_int;
    }
    if (*arg2).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*arg2).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        || (*arg1).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*arg1).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        if (*arg1).type_0 as libc::c_uint != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*arg1).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp;
        }
        match (*arg2).type_0 as libc::c_uint {
            1 | 9 => {
                ret = xmlXPathEqualNodeSets(arg1, arg2, 1 as libc::c_int);
            }
            2 => {
                if ((*arg1).nodesetval).is_null()
                    || (*(*arg1).nodesetval).nodeNr == 0 as libc::c_int
                {
                    ret = 0 as libc::c_int;
                } else {
                    ret = 1 as libc::c_int;
                }
                ret = (ret != (*arg2).boolval) as libc::c_int;
            }
            3 => {
                ret = xmlXPathEqualNodeSetFloat(
                    ctxt,
                    arg1,
                    (*arg2).floatval,
                    1 as libc::c_int,
                );
            }
            4 => {
                ret = xmlXPathEqualNodeSetString(
                    arg1,
                    (*arg2).stringval,
                    1 as libc::c_int,
                );
            }
            8 | 5 | 6 | 7 => {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8
                        as *const libc::c_char,
                    b"xpath.c\0" as *const u8 as *const libc::c_char,
                    7314 as libc::c_int,
                );
            }
            0 | _ => {}
        }
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
        return ret;
    }
    return (xmlXPathEqualValuesCommon(ctxt, arg1, arg2) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompareValues(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: libc::c_int,
    mut strict: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut arg1i: libc::c_int = 0 as libc::c_int;
    let mut arg2i: libc::c_int = 0 as libc::c_int;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as libc::c_int;
    }
    arg2 = valuePop(ctxt);
    arg1 = valuePop(ctxt);
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            xmlXPathReleaseObject((*ctxt).context, arg1);
        } else {
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return 0 as libc::c_int;
    }
    if (*arg2).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*arg2).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        || (*arg1).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*arg1).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        if ((*arg2).type_0 as libc::c_uint
            == XPATH_NODESET as libc::c_int as libc::c_uint
            || (*arg2).type_0 as libc::c_uint
                == XPATH_XSLT_TREE as libc::c_int as libc::c_uint)
            && ((*arg1).type_0 as libc::c_uint
                == XPATH_NODESET as libc::c_int as libc::c_uint
                || (*arg1).type_0 as libc::c_uint
                    == XPATH_XSLT_TREE as libc::c_int as libc::c_uint)
        {
            ret = xmlXPathCompareNodeSets(inf, strict, arg1, arg2);
        } else if (*arg1).type_0 as libc::c_uint
            == XPATH_NODESET as libc::c_int as libc::c_uint
            || (*arg1).type_0 as libc::c_uint
                == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
        {
            ret = xmlXPathCompareNodeSetValue(ctxt, inf, strict, arg1, arg2);
        } else {
            ret = xmlXPathCompareNodeSetValue(
                ctxt,
                (inf == 0) as libc::c_int,
                strict,
                arg2,
                arg1,
            );
        }
        return ret;
    }
    if (*arg1).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        valuePush(ctxt, arg1);
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
        arg1 = valuePop(ctxt);
    }
    if (*arg1).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return 0 as libc::c_int;
    }
    if (*arg2).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        valuePush(ctxt, arg2);
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
        arg2 = valuePop(ctxt);
    }
    if (*arg2).type_0 as libc::c_uint != XPATH_NUMBER as libc::c_int as libc::c_uint {
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return 0 as libc::c_int;
    }
    if xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0 {
        ret = 0 as libc::c_int;
    } else {
        arg1i = xmlXPathIsInf((*arg1).floatval);
        arg2i = xmlXPathIsInf((*arg2).floatval);
        if inf != 0 && strict != 0 {
            if arg1i == -(1 as libc::c_int) && arg2i != -(1 as libc::c_int)
                || arg2i == 1 as libc::c_int && arg1i != 1 as libc::c_int
            {
                ret = 1 as libc::c_int;
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = ((*arg1).floatval < (*arg2).floatval) as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
            }
        } else if inf != 0 && strict == 0 {
            if arg1i == -(1 as libc::c_int) || arg2i == 1 as libc::c_int {
                ret = 1 as libc::c_int;
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = ((*arg1).floatval <= (*arg2).floatval) as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
            }
        } else if inf == 0 && strict != 0 {
            if arg1i == 1 as libc::c_int && arg2i != 1 as libc::c_int
                || arg2i == -(1 as libc::c_int) && arg1i != -(1 as libc::c_int)
            {
                ret = 1 as libc::c_int;
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = ((*arg1).floatval > (*arg2).floatval) as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
            }
        } else if inf == 0 && strict == 0 {
            if arg1i == 1 as libc::c_int || arg2i == -(1 as libc::c_int) {
                ret = 1 as libc::c_int;
            } else if arg1i == 0 as libc::c_int && arg2i == 0 as libc::c_int {
                ret = ((*arg1).floatval >= (*arg2).floatval) as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, arg1);
    xmlXPathReleaseObject((*ctxt).context, arg2);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathValueFlipSign(mut ctxt: xmlXPathParserContextPtr) {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval = -(*(*ctxt).value).floatval;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathAddValues(mut ctxt: xmlXPathParserContextPtr) {
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval += val;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubValues(mut ctxt: xmlXPathParserContextPtr) {
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval -= val;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathMultValues(mut ctxt: xmlXPathParserContextPtr) {
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval *= val;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDivValues(mut ctxt: xmlXPathParserContextPtr) {
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut val: libc::c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval /= val;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathModValues(mut ctxt: xmlXPathParserContextPtr) {
    let mut arg: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg1: libc::c_double = 0.;
    let mut arg2: libc::c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    arg2 = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    arg1 = (*(*ctxt).value).floatval;
    if arg2 == 0 as libc::c_int as libc::c_double {
        (*(*ctxt).value).floatval = ::core::f32::NAN as libc::c_double;
    } else {
        (*(*ctxt).value).floatval = fmod(arg1, arg2);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return (*(*ctxt).context).node;
    }
    return 0 as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextChild(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if ((*(*ctxt).context).node).is_null() {
            return 0 as xmlNodePtr;
        }
        match (*(*(*ctxt).context).node).type_0 as libc::c_uint {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 => {
                return (*(*(*ctxt).context).node).children;
            }
            9 | 10 | 11 | 13 | 21 => {
                return (*((*(*ctxt).context).node as xmlDocPtr)).children;
            }
            15 | 16 | 17 | 2 | 18 | 19 | 20 => return 0 as xmlNodePtr,
            _ => {}
        }
        return 0 as xmlNodePtr;
    }
    if (*cur).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        || (*cur).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    return (*cur).next;
}
unsafe extern "C" fn xmlXPathNextChildElement(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        match (*cur).type_0 as libc::c_uint {
            1 | 11 | 5 | 6 => {
                cur = (*cur).children;
                if !cur.is_null() {
                    if (*cur).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        return cur;
                    }
                    loop {
                        cur = (*cur).next;
                        if !(!cur.is_null()
                            && (*cur).type_0 as libc::c_uint
                                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint)
                        {
                            break;
                        }
                    }
                    return cur;
                }
                return 0 as xmlNodePtr;
            }
            9 | 13 | 21 => return xmlDocGetRootElement(cur as xmlDocPtr as *const xmlDoc),
            _ => return 0 as xmlNodePtr,
        }
    }
    match (*cur).type_0 as libc::c_uint {
        1 | 3 | 5 | 6 | 4 | 7 | 8 | 20 => {}
        _ => return 0 as xmlNodePtr,
    }
    if !((*cur).next).is_null() {
        if (*(*cur).next).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            return (*cur).next;
        }
        cur = (*cur).next;
        loop {
            cur = (*cur).next;
            if !(!cur.is_null()
                && (*cur).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        return cur;
    }
    return 0 as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextDescendant(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if ((*(*ctxt).context).node).is_null() {
            return 0 as xmlNodePtr;
        }
        if (*(*(*ctxt).context).node).type_0 as libc::c_uint
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
            || (*(*(*ctxt).context).node).type_0 as libc::c_uint
                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            return 0 as xmlNodePtr;
        }
        if (*(*ctxt).context).node == (*(*ctxt).context).doc as xmlNodePtr {
            return (*(*(*ctxt).context).doc).children;
        }
        return (*(*(*ctxt).context).node).children;
    }
    if (*cur).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if !((*cur).children).is_null() {
        if (*(*cur).children).type_0 as libc::c_uint
            != XML_ENTITY_DECL as libc::c_int as libc::c_uint
        {
            cur = (*cur).children;
            if (*cur).type_0 as libc::c_uint
                != XML_DTD_NODE as libc::c_int as libc::c_uint
            {
                return cur;
            }
        }
    }
    if cur == (*(*ctxt).context).node {
        return 0 as xmlNodePtr;
    }
    while !((*cur).next).is_null() {
        cur = (*cur).next;
        if (*cur).type_0 as libc::c_uint
            != XML_ENTITY_DECL as libc::c_int as libc::c_uint
            && (*cur).type_0 as libc::c_uint
                != XML_DTD_NODE as libc::c_int as libc::c_uint
        {
            return cur;
        }
    }
    loop {
        cur = (*cur).parent;
        if cur.is_null() {
            break;
        }
        if cur == (*(*ctxt).context).node {
            return 0 as xmlNodePtr;
        }
        if !((*cur).next).is_null() {
            cur = (*cur).next;
            return cur;
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextDescendantOrSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return (*(*ctxt).context).node;
    }
    if ((*(*ctxt).context).node).is_null() {
        return 0 as xmlNodePtr;
    }
    if (*(*(*ctxt).context).node).type_0 as libc::c_uint
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        || (*(*(*ctxt).context).node).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    return xmlXPathNextDescendant(ctxt, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextParent(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if ((*(*ctxt).context).node).is_null() {
            return 0 as xmlNodePtr;
        }
        match (*(*(*ctxt).context).node).type_0 as libc::c_uint {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 19 | 20 | 17 => {
                if ((*(*(*ctxt).context).node).parent).is_null() {
                    return (*(*ctxt).context).doc as xmlNodePtr;
                }
                if (*(*(*(*ctxt).context).node).parent).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && (*((*(*(*(*ctxt).context).node).parent).name)
                        .offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*(*ctxt).context).node).parent).name,
                            b"fake node libxslt\0" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0)
                {
                    return 0 as xmlNodePtr;
                }
                return (*(*(*ctxt).context).node).parent;
            }
            2 => {
                let mut att: xmlAttrPtr = (*(*ctxt).context).node as xmlAttrPtr;
                return (*att).parent;
            }
            9 | 10 | 11 | 13 | 21 => return 0 as xmlNodePtr,
            18 => {
                let mut ns: xmlNsPtr = (*(*ctxt).context).node as xmlNsPtr;
                if !((*ns).next).is_null()
                    && (*(*ns).next).type_0 as libc::c_uint
                        != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    return (*ns).next as xmlNodePtr;
                }
                return 0 as xmlNodePtr;
            }
            _ => {}
        }
    }
    return 0 as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAncestor(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if ((*(*ctxt).context).node).is_null() {
            return 0 as xmlNodePtr;
        }
        match (*(*(*ctxt).context).node).type_0 as libc::c_uint {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 16 | 17 | 12 | 19 | 20 => {
                if ((*(*(*ctxt).context).node).parent).is_null() {
                    return (*(*ctxt).context).doc as xmlNodePtr;
                }
                if (*(*(*(*ctxt).context).node).parent).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && (*((*(*(*(*ctxt).context).node).parent).name)
                        .offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*(*ctxt).context).node).parent).name,
                            b"fake node libxslt\0" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0)
                {
                    return 0 as xmlNodePtr;
                }
                return (*(*(*ctxt).context).node).parent;
            }
            2 => {
                let mut tmp: xmlAttrPtr = (*(*ctxt).context).node as xmlAttrPtr;
                return (*tmp).parent;
            }
            9 | 10 | 11 | 13 | 21 => return 0 as xmlNodePtr,
            18 => {
                let mut ns: xmlNsPtr = (*(*ctxt).context).node as xmlNsPtr;
                if !((*ns).next).is_null()
                    && (*(*ns).next).type_0 as libc::c_uint
                        != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    return (*ns).next as xmlNodePtr;
                }
                return 0 as xmlNodePtr;
            }
            _ => {}
        }
        return 0 as xmlNodePtr;
    }
    if cur == (*(*(*ctxt).context).doc).children {
        return (*(*ctxt).context).doc as xmlNodePtr;
    }
    if cur == (*(*ctxt).context).doc as xmlNodePtr {
        return 0 as xmlNodePtr;
    }
    match (*cur).type_0 as libc::c_uint {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
            if ((*cur).parent).is_null() {
                return 0 as xmlNodePtr;
            }
            if (*(*cur).parent).type_0 as libc::c_uint
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                && (*((*(*cur).parent).name).offset(0 as libc::c_int as isize)
                    as libc::c_int == ' ' as i32
                    || xmlStrEqual(
                        (*(*cur).parent).name,
                        b"fake node libxslt\0" as *const u8 as *const libc::c_char
                            as *mut xmlChar,
                    ) != 0)
            {
                return 0 as xmlNodePtr;
            }
            return (*cur).parent;
        }
        2 => {
            let mut att: xmlAttrPtr = cur as xmlAttrPtr;
            return (*att).parent;
        }
        18 => {
            let mut ns_0: xmlNsPtr = cur as xmlNsPtr;
            if !((*ns_0).next).is_null()
                && (*(*ns_0).next).type_0 as libc::c_uint
                    != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                return (*ns_0).next as xmlNodePtr;
            }
            return 0 as xmlNodePtr;
        }
        9 | 10 | 11 | 13 | 21 => return 0 as xmlNodePtr,
        _ => {}
    }
    return 0 as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAncestorOrSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return (*(*ctxt).context).node;
    }
    return xmlXPathNextAncestor(ctxt, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextFollowingSibling(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if (*(*(*ctxt).context).node).type_0 as libc::c_uint
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        || (*(*(*ctxt).context).node).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur == (*(*ctxt).context).doc as xmlNodePtr {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return (*(*(*ctxt).context).node).next;
    }
    return (*cur).next;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextPrecedingSibling(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if (*(*(*ctxt).context).node).type_0 as libc::c_uint
        == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        || (*(*(*ctxt).context).node).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur == (*(*ctxt).context).doc as xmlNodePtr {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        return (*(*(*ctxt).context).node).prev;
    }
    if !((*cur).prev).is_null()
        && (*(*cur).prev).type_0 as libc::c_uint
            == XML_DTD_NODE as libc::c_int as libc::c_uint
    {
        cur = (*cur).prev;
        if cur.is_null() {
            return (*(*(*ctxt).context).node).prev;
        }
    }
    return (*cur).prev;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextFollowing(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if !cur.is_null()
        && (*cur).type_0 as libc::c_uint
            != XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        && (*cur).type_0 as libc::c_uint
            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        && !((*cur).children).is_null()
    {
        return (*cur).children;
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if (*cur).type_0 as libc::c_uint
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        {
            cur = (*cur).parent;
        } else if (*cur).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if ((*ns).next).is_null()
                || (*(*ns).next).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                return 0 as xmlNodePtr;
            }
            cur = (*ns).next as xmlNodePtr;
        }
    }
    if cur.is_null() {
        return 0 as xmlNodePtr;
    }
    if !((*cur).next).is_null() {
        return (*cur).next;
    }
    loop {
        cur = (*cur).parent;
        if cur.is_null() {
            break;
        }
        if cur == (*(*ctxt).context).doc as xmlNodePtr {
            return 0 as xmlNodePtr;
        }
        if !((*cur).next).is_null() {
            return (*cur).next;
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
}
unsafe extern "C" fn xmlXPathIsAncestor(
    mut ancestor: xmlNodePtr,
    mut node: xmlNodePtr,
) -> libc::c_int {
    if ancestor.is_null() || node.is_null() {
        return 0 as libc::c_int;
    }
    if (*node).type_0 as libc::c_uint
        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*ancestor).type_0 as libc::c_uint
        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*ancestor).doc != (*node).doc {
        return 0 as libc::c_int;
    }
    if ancestor == (*node).doc as xmlNodePtr {
        return 1 as libc::c_int;
    }
    if node == (*ancestor).doc as xmlNodePtr {
        return 0 as libc::c_int;
    }
    while !((*node).parent).is_null() {
        if (*node).parent == ancestor {
            return 1 as libc::c_int;
        }
        node = (*node).parent;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextPreceding(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if (*cur).type_0 as libc::c_uint
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        {
            cur = (*cur).parent;
        } else if (*cur).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if ((*ns).next).is_null()
                || (*(*ns).next).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                return 0 as xmlNodePtr;
            }
            cur = (*ns).next as xmlNodePtr;
        }
    }
    if cur.is_null()
        || (*cur).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if !((*cur).prev).is_null()
        && (*(*cur).prev).type_0 as libc::c_uint
            == XML_DTD_NODE as libc::c_int as libc::c_uint
    {
        cur = (*cur).prev;
    }
    loop {
        if !((*cur).prev).is_null() {
            cur = (*cur).prev;
            while !((*cur).last).is_null() {
                cur = (*cur).last;
            }
            return cur;
        }
        cur = (*cur).parent;
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        if cur == (*(*(*ctxt).context).doc).children {
            return 0 as xmlNodePtr;
        }
        if !(xmlXPathIsAncestor(cur, (*(*ctxt).context).node) != 0) {
            break;
        }
    }
    return cur;
}
unsafe extern "C" fn xmlXPathNextPrecedingInternal(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        if (*cur).type_0 as libc::c_uint
            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
        {
            cur = (*cur).parent;
        } else if (*cur).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if ((*ns).next).is_null()
                || (*(*ns).next).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
            {
                return 0 as xmlNodePtr;
            }
            cur = (*ns).next as xmlNodePtr;
        }
        (*ctxt).ancestor = (*cur).parent;
    }
    if (*cur).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if !((*cur).prev).is_null()
        && (*(*cur).prev).type_0 as libc::c_uint
            == XML_DTD_NODE as libc::c_int as libc::c_uint
    {
        cur = (*cur).prev;
    }
    while ((*cur).prev).is_null() {
        cur = (*cur).parent;
        if cur.is_null() {
            return 0 as xmlNodePtr;
        }
        if cur == (*(*(*ctxt).context).doc).children {
            return 0 as xmlNodePtr;
        }
        if cur != (*ctxt).ancestor {
            return cur;
        }
        (*ctxt).ancestor = (*cur).parent;
    }
    cur = (*cur).prev;
    while !((*cur).last).is_null() {
        cur = (*cur).last;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextNamespace(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if (*(*(*ctxt).context).node).type_0 as libc::c_uint
        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if !((*(*ctxt).context).tmpNsList).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*(*ctxt).context).tmpNsList as *mut libc::c_void);
        }
        (*(*ctxt).context)
            .tmpNsList = xmlGetNsList(
            (*(*ctxt).context).doc as *const xmlDoc,
            (*(*ctxt).context).node as *const xmlNode,
        );
        (*(*ctxt).context).tmpNsNr = 0 as libc::c_int;
        if !((*(*ctxt).context).tmpNsList).is_null() {
            while !(*((*(*ctxt).context).tmpNsList)
                .offset((*(*ctxt).context).tmpNsNr as isize))
                .is_null()
            {
                (*(*ctxt).context).tmpNsNr += 1;
                (*(*ctxt).context).tmpNsNr;
            }
        }
        return xmlXPathXMLNamespace as xmlNodePtr;
    }
    if (*(*ctxt).context).tmpNsNr > 0 as libc::c_int {
        (*(*ctxt).context).tmpNsNr -= 1;
        return *((*(*ctxt).context).tmpNsList)
            .offset((*(*ctxt).context).tmpNsNr as isize) as xmlNodePtr;
    } else {
        if !((*(*ctxt).context).tmpNsList).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*(*ctxt).context).tmpNsList as *mut libc::c_void);
        }
        (*(*ctxt).context).tmpNsList = 0 as *mut xmlNsPtr;
        return 0 as xmlNodePtr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNextAttribute(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return 0 as xmlNodePtr;
    }
    if ((*(*ctxt).context).node).is_null() {
        return 0 as xmlNodePtr;
    }
    if (*(*(*ctxt).context).node).type_0 as libc::c_uint
        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    if cur.is_null() {
        if (*(*ctxt).context).node == (*(*ctxt).context).doc as xmlNodePtr {
            return 0 as xmlNodePtr;
        }
        return (*(*(*ctxt).context).node).properties as xmlNodePtr;
    }
    return (*cur).next as xmlNodePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRoot(mut ctxt: xmlXPathParserContextPtr) {
    if ctxt.is_null() || ((*ctxt).context).is_null() {
        return;
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).doc as xmlNodePtr),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathLastFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if (*(*ctxt).context).contextSize >= 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                (*(*ctxt).context).contextSize as libc::c_double,
            ),
        );
    } else {
        xmlXPathErr(ctxt, XPATH_INVALID_CTXT_SIZE as libc::c_int);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathPositionFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if (*(*ctxt).context).proximityPosition >= 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                (*(*ctxt).context).proximityPosition as libc::c_double,
            ),
        );
    } else {
        xmlXPathErr(ctxt, XPATH_INVALID_CTXT_POSITION as libc::c_int);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCountFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if cur.is_null() || ((*cur).nodesetval).is_null() {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat((*ctxt).context, 0 as libc::c_int as libc::c_double),
        );
    } else if (*cur).type_0 as libc::c_uint
        == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*cur).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                (*(*cur).nodesetval).nodeNr as libc::c_double,
            ),
        );
    } else if (*(*cur).nodesetval).nodeNr != 1 as libc::c_int
        || ((*(*cur).nodesetval).nodeTab).is_null()
    {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat((*ctxt).context, 0 as libc::c_int as libc::c_double),
        );
    } else {
        let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
        let mut i: libc::c_int = 0 as libc::c_int;
        tmp = *((*(*cur).nodesetval).nodeTab).offset(0 as libc::c_int as isize);
        if !tmp.is_null()
            && (*tmp).type_0 as libc::c_uint
                != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            tmp = (*tmp).children;
            while !tmp.is_null() {
                tmp = (*tmp).next;
                i += 1;
                i;
            }
        }
        valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, i as libc::c_double));
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
}
unsafe extern "C" fn xmlXPathGetElementsByIds(
    mut doc: xmlDocPtr,
    mut ids: *const xmlChar,
) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut cur: *const xmlChar = ids;
    let mut ID: *mut xmlChar = 0 as *mut xmlChar;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut elem: xmlNodePtr = 0 as xmlNodePtr;
    if ids.is_null() {
        return 0 as xmlNodeSetPtr;
    }
    ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
    if ret.is_null() {
        return ret;
    }
    while *cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *cur as libc::c_int
            && *cur as libc::c_int <= 0xa as libc::c_int
        || *cur as libc::c_int == 0xd as libc::c_int
    {
        cur = cur.offset(1);
        cur;
    }
    while *cur as libc::c_int != 0 as libc::c_int {
        while !(*cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *cur as libc::c_int
                && *cur as libc::c_int <= 0xa as libc::c_int
            || *cur as libc::c_int == 0xd as libc::c_int)
            && *cur as libc::c_int != 0 as libc::c_int
        {
            cur = cur.offset(1);
            cur;
        }
        ID = xmlStrndup(ids, cur.offset_from(ids) as libc::c_long as libc::c_int);
        if !ID.is_null() {
            attr = xmlGetID(doc, ID);
            if !attr.is_null() {
                if (*attr).type_0 as libc::c_uint
                    == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                {
                    elem = (*attr).parent;
                } else if (*attr).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                {
                    elem = attr as xmlNodePtr;
                } else {
                    elem = 0 as xmlNodePtr;
                }
                if !elem.is_null() {
                    xmlXPathNodeSetAdd(ret, elem);
                }
            }
            xmlFree.expect("non-null function pointer")(ID as *mut libc::c_void);
        }
        while *cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *cur as libc::c_int
                && *cur as libc::c_int <= 0xa as libc::c_int
            || *cur as libc::c_int == 0xd as libc::c_int
        {
            cur = cur.offset(1);
            cur;
        }
        ids = cur;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathIdFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut tokens: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    if (*obj).type_0 as libc::c_uint == XPATH_NODESET as libc::c_int as libc::c_uint
        || (*obj).type_0 as libc::c_uint
            == XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        let mut ns: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        let mut i: libc::c_int = 0;
        ret = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        if !((*obj).nodesetval).is_null() {
            i = 0 as libc::c_int;
            while i < (*(*obj).nodesetval).nodeNr {
                tokens = xmlXPathCastNodeToString(
                    *((*(*obj).nodesetval).nodeTab).offset(i as isize),
                );
                ns = xmlXPathGetElementsByIds((*(*ctxt).context).doc, tokens);
                ret = xmlXPathNodeSetMerge(ret, ns);
                xmlXPathFreeNodeSet(ns);
                if !tokens.is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(tokens as *mut libc::c_void);
                }
                i += 1;
                i;
            }
        }
        xmlXPathReleaseObject((*ctxt).context, obj);
        valuePush(ctxt, xmlXPathCacheWrapNodeSet((*ctxt).context, ret));
        return;
    }
    obj = xmlXPathCacheConvertString((*ctxt).context, obj);
    ret = xmlXPathGetElementsByIds((*(*ctxt).context).doc, (*obj).stringval);
    valuePush(ctxt, xmlXPathCacheWrapNodeSet((*ctxt).context, ret));
    xmlXPathReleaseObject((*ctxt).context, obj);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathLocalNameFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
        );
        nargs = 1 as libc::c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if ((*cur).nodesetval).is_null() || (*(*cur).nodesetval).nodeNr == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        match (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).type_0
            as libc::c_uint
        {
            1 | 2 | 7 => {
                if *((**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            (*ctxt).context,
                            b"\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            (*ctxt).context,
                            (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name,
                        ),
                    );
                }
            }
            18 => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewString(
                        (*ctxt).context,
                        (*(*((*(*cur).nodesetval).nodeTab).offset(i as isize)
                            as xmlNsPtr))
                            .prefix,
                    ),
                );
            }
            _ => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewCString(
                        (*ctxt).context,
                        b"\0" as *const u8 as *const libc::c_char,
                    ),
                );
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNamespaceURIFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
        );
        nargs = 1 as libc::c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if ((*cur).nodesetval).is_null() || (*(*cur).nodesetval).nodeNr == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        match (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).type_0
            as libc::c_uint
        {
            1 | 2 => {
                if ((**((*(*cur).nodesetval).nodeTab).offset(i as isize)).ns).is_null() {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            (*ctxt).context,
                            b"\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                } else {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            (*ctxt).context,
                            (*(**((*(*cur).nodesetval).nodeTab).offset(i as isize)).ns)
                                .href,
                        ),
                    );
                }
            }
            _ => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewCString(
                        (*ctxt).context,
                        b"\0" as *const u8 as *const libc::c_char,
                    ),
                );
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
}
unsafe extern "C" fn xmlXPathNameFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if nargs == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
        );
        nargs = 1 as libc::c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if ((*cur).nodesetval).is_null() || (*(*cur).nodesetval).nodeNr == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        match (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).type_0
            as libc::c_uint
        {
            1 | 2 => {
                if *((**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            (*ctxt).context,
                            b"\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                } else if ((**((*(*cur).nodesetval).nodeTab).offset(i as isize)).ns)
                    .is_null()
                    || ((*(**((*(*cur).nodesetval).nodeTab).offset(i as isize)).ns)
                        .prefix)
                        .is_null()
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            (*ctxt).context,
                            (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name,
                        ),
                    );
                } else {
                    let mut fullname: *mut xmlChar = 0 as *mut xmlChar;
                    fullname = xmlBuildQName(
                        (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name,
                        (*(**((*(*cur).nodesetval).nodeTab).offset(i as isize)).ns)
                            .prefix,
                        0 as *mut xmlChar,
                        0 as libc::c_int,
                    );
                    if fullname
                        == (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name
                            as *mut xmlChar
                    {
                        fullname = xmlStrdup(
                            (**((*(*cur).nodesetval).nodeTab).offset(i as isize)).name,
                        );
                    }
                    if fullname.is_null() {
                        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as libc::c_int);
                        return;
                    }
                    valuePush(ctxt, xmlXPathCacheWrapString((*ctxt).context, fullname));
                }
            }
            _ => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(
                        (*ctxt).context,
                        *((*(*cur).nodesetval).nodeTab).offset(i as isize),
                    ),
                );
                xmlXPathLocalNameFunction(ctxt, 1 as libc::c_int);
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheWrapString(
                (*ctxt).context,
                xmlXPathCastNodeToString((*(*ctxt).context).node),
            ),
        );
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if cur.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    valuePush(ctxt, xmlXPathCacheConvertString((*ctxt).context, cur));
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringLengthFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if nargs == 0 as libc::c_int {
        if ctxt.is_null() || ((*ctxt).context).is_null() {
            return;
        }
        if ((*(*ctxt).context).node).is_null() {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    (*ctxt).context,
                    0 as libc::c_int as libc::c_double,
                ),
            );
        } else {
            let mut content: *mut xmlChar = 0 as *mut xmlChar;
            content = xmlXPathCastNodeToString((*(*ctxt).context).node);
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    (*ctxt).context,
                    xmlUTF8Strlen(content) as libc::c_double,
                ),
            );
            xmlFree.expect("non-null function pointer")(content as *mut libc::c_void);
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    valuePush(
        ctxt,
        xmlXPathCacheNewFloat(
            (*ctxt).context,
            xmlUTF8Strlen((*cur).stringval) as libc::c_double,
        ),
    );
    xmlXPathReleaseObject((*ctxt).context, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathConcatFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut newobj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() {
        return;
    }
    if nargs < 2 as libc::c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
            return;
        }
        if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
            return;
        }
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    cur = valuePop(ctxt);
    if cur.is_null()
        || (*cur).type_0 as libc::c_uint != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, cur);
        return;
    }
    nargs -= 1;
    nargs;
    while nargs > 0 as libc::c_int {
        if !((*ctxt).value).is_null()
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_STRING as libc::c_int as libc::c_uint
        {
            xmlXPathStringFunction(ctxt, 1 as libc::c_int);
        }
        newobj = valuePop(ctxt);
        if newobj.is_null()
            || (*newobj).type_0 as libc::c_uint
                != XPATH_STRING as libc::c_int as libc::c_uint
        {
            xmlXPathReleaseObject((*ctxt).context, newobj);
            xmlXPathReleaseObject((*ctxt).context, cur);
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
            return;
        }
        tmp = xmlStrcat((*newobj).stringval, (*cur).stringval);
        (*newobj).stringval = (*cur).stringval;
        (*cur).stringval = tmp;
        xmlXPathReleaseObject((*ctxt).context, newobj);
        nargs -= 1;
        nargs;
    }
    valuePush(ctxt, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathContainsFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut hay: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut needle: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    needle = valuePop(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    hay = valuePop(ctxt);
    if hay.is_null()
        || (*hay).type_0 as libc::c_uint != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, hay);
        xmlXPathReleaseObject((*ctxt).context, needle);
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    if !(xmlStrstr((*hay).stringval, (*needle).stringval)).is_null() {
        valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, 1 as libc::c_int));
    } else {
        valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, 0 as libc::c_int));
    }
    xmlXPathReleaseObject((*ctxt).context, hay);
    xmlXPathReleaseObject((*ctxt).context, needle);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathStartsWithFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut hay: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut needle: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut n: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    needle = valuePop(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    hay = valuePop(ctxt);
    if hay.is_null()
        || (*hay).type_0 as libc::c_uint != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, hay);
        xmlXPathReleaseObject((*ctxt).context, needle);
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    n = xmlStrlen((*needle).stringval);
    if xmlStrncmp((*hay).stringval, (*needle).stringval, n) != 0 {
        valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, 0 as libc::c_int));
    } else {
        valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, 1 as libc::c_int));
    }
    xmlXPathReleaseObject((*ctxt).context, hay);
    xmlXPathReleaseObject((*ctxt).context, needle);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut start: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut len: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut le: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut in_0: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if nargs < 2 as libc::c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
            return;
        }
        if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
            return;
        }
    }
    if nargs > 3 as libc::c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 3 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
            return;
        }
        if (*ctxt).valueNr < (*ctxt).valueFrame + 3 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
            return;
        }
    }
    if nargs == 3 as libc::c_int {
        if !((*ctxt).value).is_null()
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_NUMBER as libc::c_int as libc::c_uint
        {
            xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
        }
        if ((*ctxt).value).is_null()
            || (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_NUMBER as libc::c_int as libc::c_uint
        {
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
            return;
        }
        len = valuePop(ctxt);
        le = (*len).floatval;
        xmlXPathReleaseObject((*ctxt).context, len);
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    start = valuePop(ctxt);
    in_0 = (*start).floatval;
    xmlXPathReleaseObject((*ctxt).context, start);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    str = valuePop(ctxt);
    m = xmlUTF8Strlen((*str).stringval as *const libc::c_uchar);
    if nargs != 3 as libc::c_int {
        le = m as libc::c_double;
        if in_0 < 1.0f64 {
            in_0 = 1.0f64;
        }
    }
    if xmlXPathIsInf(in_0) == 0 && xmlXPathIsNaN(in_0 + le) == 0 {
        i = in_0 as libc::c_int;
        if i as libc::c_double + 0.5f64 <= in_0 {
            i += 1;
            i;
        }
        if xmlXPathIsInf(le) == 1 as libc::c_int {
            l = m;
            if i < 1 as libc::c_int {
                i = 1 as libc::c_int;
            }
        } else if xmlXPathIsInf(le) == -(1 as libc::c_int) || le < 0.0f64 {
            l = 0 as libc::c_int;
        } else {
            l = le as libc::c_int;
            if l as libc::c_double + 0.5f64 <= le {
                l += 1;
                l;
            }
        }
        i -= 1 as libc::c_int;
        l += i;
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        }
        if l > m {
            l = m;
        }
        l -= i;
        ret = xmlUTF8Strsub((*str).stringval, i, l);
    } else {
        ret = 0 as *mut xmlChar;
    }
    if ret.is_null() {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const libc::c_char,
            ),
        );
    } else {
        valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, ret));
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
    }
    xmlXPathReleaseObject((*ctxt).context, str);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringBeforeFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut find: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut offset: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    find = valuePop(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    if !target.is_null() {
        point = xmlStrstr((*str).stringval, (*find).stringval);
        if !point.is_null() {
            offset = point.offset_from((*str).stringval) as libc::c_long as libc::c_int;
            xmlBufAdd(target, (*str).stringval, offset);
        }
        valuePush(
            ctxt,
            xmlXPathCacheNewString(
                (*ctxt).context,
                xmlBufContent(target as *const xmlBuf),
            ),
        );
        xmlBufFree(target);
    }
    xmlXPathReleaseObject((*ctxt).context, str);
    xmlXPathReleaseObject((*ctxt).context, find);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathSubstringAfterFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut find: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut offset: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    find = valuePop(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    if !target.is_null() {
        point = xmlStrstr((*str).stringval, (*find).stringval);
        if !point.is_null() {
            offset = point.offset_from((*str).stringval) as libc::c_long as libc::c_int
                + xmlStrlen((*find).stringval);
            xmlBufAdd(
                target,
                &mut *((*str).stringval).offset(offset as isize),
                xmlStrlen((*str).stringval) - offset,
            );
        }
        valuePush(
            ctxt,
            xmlXPathCacheNewString(
                (*ctxt).context,
                xmlBufContent(target as *const xmlBuf),
            ),
        );
        xmlBufFree(target);
    }
    xmlXPathReleaseObject((*ctxt).context, str);
    xmlXPathReleaseObject((*ctxt).context, find);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNormalizeFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut obj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut source: *mut xmlChar = 0 as *mut xmlChar;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut blank: xmlChar = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as libc::c_int {
        valuePush(
            ctxt,
            xmlXPathCacheWrapString(
                (*ctxt).context,
                xmlXPathCastNodeToString((*(*ctxt).context).node),
            ),
        );
        nargs = 1 as libc::c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    obj = valuePop(ctxt);
    source = (*obj).stringval;
    target = xmlBufCreate();
    if !target.is_null() && !source.is_null() {
        while *source as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *source as libc::c_int
                && *source as libc::c_int <= 0xa as libc::c_int
            || *source as libc::c_int == 0xd as libc::c_int
        {
            source = source.offset(1);
            source;
        }
        blank = 0 as libc::c_int as xmlChar;
        while *source != 0 {
            if *source as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *source as libc::c_int
                    && *source as libc::c_int <= 0xa as libc::c_int
                || *source as libc::c_int == 0xd as libc::c_int
            {
                blank = 0x20 as libc::c_int as xmlChar;
            } else {
                if blank != 0 {
                    xmlBufAdd(target, &mut blank, 1 as libc::c_int);
                    blank = 0 as libc::c_int as xmlChar;
                }
                xmlBufAdd(target, source, 1 as libc::c_int);
            }
            source = source.offset(1);
            source;
        }
        valuePush(
            ctxt,
            xmlXPathCacheNewString(
                (*ctxt).context,
                xmlBufContent(target as *const xmlBuf),
            ),
        );
        xmlBufFree(target);
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathTranslateFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut from: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut to: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut offset: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut ch: xmlChar = 0;
    let mut point: *const xmlChar = 0 as *const xmlChar;
    let mut cptr: *mut xmlChar = 0 as *mut xmlChar;
    if ctxt.is_null() {
        return;
    }
    if nargs != 3 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 3 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    to = valuePop(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    from = valuePop(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    if !target.is_null() {
        max = xmlUTF8Strlen((*to).stringval);
        cptr = (*str).stringval;
        loop {
            ch = *cptr;
            if !(ch != 0) {
                break;
            }
            offset = xmlUTF8Strloc((*from).stringval, cptr);
            if offset >= 0 as libc::c_int {
                if offset < max {
                    point = xmlUTF8Strpos((*to).stringval, offset);
                    if !point.is_null() {
                        xmlBufAdd(
                            target,
                            point,
                            xmlUTF8Strsize(point, 1 as libc::c_int),
                        );
                    }
                }
            } else {
                xmlBufAdd(target, cptr, xmlUTF8Strsize(cptr, 1 as libc::c_int));
            }
            cptr = cptr.offset(1);
            cptr;
            if !(ch as libc::c_int & 0x80 as libc::c_int != 0) {
                continue;
            }
            if ch as libc::c_int & 0xc0 as libc::c_int != 0xc0 as libc::c_int {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathTranslateFunction: Invalid UTF8 string\n\0" as *const u8
                        as *const libc::c_char,
                );
                break;
            } else {
                loop {
                    ch = ((ch as libc::c_int) << 1 as libc::c_int) as xmlChar;
                    if !(ch as libc::c_int & 0x80 as libc::c_int != 0) {
                        break;
                    }
                    let fresh71 = cptr;
                    cptr = cptr.offset(1);
                    if !(*fresh71 as libc::c_int & 0xc0 as libc::c_int
                        != 0x80 as libc::c_int)
                    {
                        continue;
                    }
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathTranslateFunction: Invalid UTF8 string\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    break;
                }
                if ch as libc::c_int & 0x80 as libc::c_int != 0 {
                    break;
                }
            }
        }
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewString((*ctxt).context, xmlBufContent(target as *const xmlBuf)),
    );
    xmlBufFree(target);
    xmlXPathReleaseObject((*ctxt).context, str);
    xmlXPathReleaseObject((*ctxt).context, from);
    xmlXPathReleaseObject((*ctxt).context, to);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathBooleanFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if cur.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
        return;
    }
    cur = xmlXPathCacheConvertBoolean((*ctxt).context, cur);
    valuePush(ctxt, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNotFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_BOOLEAN as libc::c_int as libc::c_uint
    {
        xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_BOOLEAN as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).boolval = ((*(*ctxt).value).boolval == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathTrueFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, 1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFalseFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 0 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, 0 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathLangFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut current_block: u64;
    let mut val: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    let mut theLang: *const xmlChar = 0 as *const xmlChar;
    let mut lang: *const xmlChar = 0 as *const xmlChar;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    val = valuePop(ctxt);
    lang = (*val).stringval;
    theLang = xmlNodeGetLang((*(*ctxt).context).node as *const xmlNode);
    if !theLang.is_null() && !lang.is_null() {
        i = 0 as libc::c_int;
        loop {
            if !(*lang.offset(i as isize) as libc::c_int != 0 as libc::c_int) {
                current_block = 224731115979188411;
                break;
            }
            if ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<xmlChar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *lang.offset(i as isize)
                            as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(*lang.offset(i as isize) as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*lang.offset(i as isize) as libc::c_int as isize);
                }
                __res
            })
                != ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<xmlChar>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *theLang.offset(i as isize)
                                as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(*theLang.offset(i as isize) as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*theLang.offset(i as isize) as libc::c_int as isize);
                    }
                    __res
                })
            {
                current_block = 1339286683097576762;
                break;
            }
            i += 1;
            i;
        }
        match current_block {
            1339286683097576762 => {}
            _ => {
                if *theLang.offset(i as isize) as libc::c_int == 0 as libc::c_int
                    || *theLang.offset(i as isize) as libc::c_int == '-' as i32
                {
                    ret = 1 as libc::c_int;
                }
            }
        }
    }
    if !theLang.is_null() {
        xmlFree.expect("non-null function pointer")(theLang as *mut libc::c_void);
    }
    xmlXPathReleaseObject((*ctxt).context, val);
    valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, ret));
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNumberFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut res: libc::c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as libc::c_int {
        if ((*(*ctxt).context).node).is_null() {
            valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, 0.0f64));
        } else {
            let mut content: *mut xmlChar = xmlNodeGetContent(
                (*(*ctxt).context).node as *const xmlNode,
            );
            res = xmlXPathStringEvalNumber(content);
            valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, res));
            xmlFree.expect("non-null function pointer")(content as *mut libc::c_void);
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    valuePush(ctxt, xmlXPathCacheConvertNumber((*ctxt).context, cur));
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathSumFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut cur: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut i: libc::c_int = 0;
    let mut res: libc::c_double = 0.0f64;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
            && (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_XSLT_TREE as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    cur = valuePop(ctxt);
    if !((*cur).nodesetval).is_null() && (*(*cur).nodesetval).nodeNr != 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < (*(*cur).nodesetval).nodeNr {
            res
                += xmlXPathCastNodeToNumber(
                    *((*(*cur).nodesetval).nodeTab).offset(i as isize),
                );
            i += 1;
            i;
        }
    }
    valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, res));
    xmlXPathReleaseObject((*ctxt).context, cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathFloorFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval = floor((*(*ctxt).value).floatval);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCeilingFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    (*(*ctxt).value).floatval = ceil((*(*ctxt).value).floatval);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRoundFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut f: libc::c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 1 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return;
    }
    f = (*(*ctxt).value).floatval;
    if f >= -0.5f64 && f < 0.5f64 {
        (*(*ctxt).value).floatval *= 0.0f64;
    } else {
        let mut rounded: libc::c_double = floor(f);
        if f - rounded >= 0.5f64 {
            rounded += 1.0f64;
        }
        (*(*ctxt).value).floatval = rounded;
    };
}
unsafe extern "C" fn xmlXPathCurrentChar(
    mut ctxt: xmlXPathParserContextPtr,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_uchar = 0;
    let mut val: libc::c_uint = 0;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    if ctxt.is_null() {
        return 0 as libc::c_int;
    }
    cur = (*ctxt).cur;
    c = *cur;
    if c as libc::c_int & 0x80 as libc::c_int != 0 {
        if !(*cur.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            != 0x80 as libc::c_int)
        {
            if c as libc::c_int & 0xe0 as libc::c_int == 0xe0 as libc::c_int {
                if *cur.offset(2 as libc::c_int as isize) as libc::c_int
                    & 0xc0 as libc::c_int != 0x80 as libc::c_int
                {
                    current_block = 17540206288142231416;
                } else if c as libc::c_int & 0xf0 as libc::c_int == 0xf0 as libc::c_int {
                    if c as libc::c_int & 0xf8 as libc::c_int != 0xf0 as libc::c_int
                        || *cur.offset(3 as libc::c_int as isize) as libc::c_int
                            & 0xc0 as libc::c_int != 0x80 as libc::c_int
                    {
                        current_block = 17540206288142231416;
                    } else {
                        *len = 4 as libc::c_int;
                        val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x7 as libc::c_int) << 18 as libc::c_int) as libc::c_uint;
                        val
                            |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int) << 12 as libc::c_int)
                                as libc::c_uint;
                        val
                            |= ((*cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
                        val
                            |= (*cur.offset(3 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int) as libc::c_uint;
                        current_block = 7175849428784450219;
                    }
                } else {
                    *len = 3 as libc::c_int;
                    val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                        & 0xf as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
                    val
                        |= ((*cur.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
                    val
                        |= (*cur.offset(2 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int) as libc::c_uint;
                    current_block = 7175849428784450219;
                }
            } else {
                *len = 2 as libc::c_int;
                val = ((*cur.offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x1f as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
                val
                    |= (*cur.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0x3f as libc::c_int) as libc::c_uint;
                current_block = 7175849428784450219;
            }
            match current_block {
                17540206288142231416 => {}
                _ => {
                    if if val < 0x100 as libc::c_int as libc::c_uint {
                        (0x9 as libc::c_int as libc::c_uint <= val
                            && val <= 0xa as libc::c_int as libc::c_uint
                            || val == 0xd as libc::c_int as libc::c_uint
                            || 0x20 as libc::c_int as libc::c_uint <= val) as libc::c_int
                    } else {
                        (0x100 as libc::c_int as libc::c_uint <= val
                            && val <= 0xd7ff as libc::c_int as libc::c_uint
                            || 0xe000 as libc::c_int as libc::c_uint <= val
                                && val <= 0xfffd as libc::c_int as libc::c_uint
                            || 0x10000 as libc::c_int as libc::c_uint <= val
                                && val <= 0x10ffff as libc::c_int as libc::c_uint)
                            as libc::c_int
                    } == 0
                    {
                        xmlXPathErr(ctxt, XPATH_INVALID_CHAR_ERROR as libc::c_int);
                        return 0 as libc::c_int;
                    }
                    return val as libc::c_int;
                }
            }
        }
        *len = 0 as libc::c_int;
        xmlXPathErr(ctxt, XPATH_ENCODING_ERROR as libc::c_int);
        return 0 as libc::c_int;
    } else {
        *len = 1 as libc::c_int;
        return *cur as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathParseNCName(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut count: libc::c_int = 0 as libc::c_int;
    if ctxt.is_null() || ((*ctxt).cur).is_null() {
        return 0 as *mut xmlChar;
    }
    in_0 = (*ctxt).cur;
    if *in_0 as libc::c_int >= 0x61 as libc::c_int
        && *in_0 as libc::c_int <= 0x7a as libc::c_int
        || *in_0 as libc::c_int >= 0x41 as libc::c_int
            && *in_0 as libc::c_int <= 0x5a as libc::c_int
        || *in_0 as libc::c_int == '_' as i32
    {
        in_0 = in_0.offset(1);
        in_0;
        while *in_0 as libc::c_int >= 0x61 as libc::c_int
            && *in_0 as libc::c_int <= 0x7a as libc::c_int
            || *in_0 as libc::c_int >= 0x41 as libc::c_int
                && *in_0 as libc::c_int <= 0x5a as libc::c_int
            || *in_0 as libc::c_int >= 0x30 as libc::c_int
                && *in_0 as libc::c_int <= 0x39 as libc::c_int
            || *in_0 as libc::c_int == '_' as i32 || *in_0 as libc::c_int == '.' as i32
            || *in_0 as libc::c_int == '-' as i32
        {
            in_0 = in_0.offset(1);
            in_0;
        }
        if *in_0 as libc::c_int == ' ' as i32 || *in_0 as libc::c_int == '>' as i32
            || *in_0 as libc::c_int == '/' as i32 || *in_0 as libc::c_int == '[' as i32
            || *in_0 as libc::c_int == ']' as i32 || *in_0 as libc::c_int == ':' as i32
            || *in_0 as libc::c_int == '@' as i32 || *in_0 as libc::c_int == '*' as i32
        {
            count = in_0.offset_from((*ctxt).cur) as libc::c_long as libc::c_int;
            if count == 0 as libc::c_int {
                return 0 as *mut xmlChar;
            }
            ret = xmlStrndup((*ctxt).cur, count);
            (*ctxt).cur = in_0;
            return ret;
        }
    }
    return xmlXPathParseNameComplex(ctxt, 0 as libc::c_int);
}
unsafe extern "C" fn xmlXPathParseQName(
    mut ctxt: xmlXPathParserContextPtr,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    *prefix = 0 as *mut xmlChar;
    ret = xmlXPathParseNCName(ctxt);
    if !ret.is_null() && *(*ctxt).cur as libc::c_int == ':' as i32 {
        *prefix = ret;
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        ret = xmlXPathParseNCName(ctxt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathParseName(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let mut in_0: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut count: size_t = 0 as libc::c_int as size_t;
    if ctxt.is_null() || ((*ctxt).cur).is_null() {
        return 0 as *mut xmlChar;
    }
    in_0 = (*ctxt).cur;
    if *in_0 as libc::c_int >= 0x61 as libc::c_int
        && *in_0 as libc::c_int <= 0x7a as libc::c_int
        || *in_0 as libc::c_int >= 0x41 as libc::c_int
            && *in_0 as libc::c_int <= 0x5a as libc::c_int
        || *in_0 as libc::c_int == '_' as i32 || *in_0 as libc::c_int == ':' as i32
    {
        in_0 = in_0.offset(1);
        in_0;
        while *in_0 as libc::c_int >= 0x61 as libc::c_int
            && *in_0 as libc::c_int <= 0x7a as libc::c_int
            || *in_0 as libc::c_int >= 0x41 as libc::c_int
                && *in_0 as libc::c_int <= 0x5a as libc::c_int
            || *in_0 as libc::c_int >= 0x30 as libc::c_int
                && *in_0 as libc::c_int <= 0x39 as libc::c_int
            || *in_0 as libc::c_int == '_' as i32 || *in_0 as libc::c_int == '-' as i32
            || *in_0 as libc::c_int == ':' as i32 || *in_0 as libc::c_int == '.' as i32
        {
            in_0 = in_0.offset(1);
            in_0;
        }
        if *in_0 as libc::c_int > 0 as libc::c_int
            && (*in_0 as libc::c_int) < 0x80 as libc::c_int
        {
            count = in_0.offset_from((*ctxt).cur) as libc::c_long as size_t;
            if count > 50000 as libc::c_int as size_t {
                (*ctxt).cur = in_0;
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                return 0 as *mut xmlChar;
            }
            ret = xmlStrndup((*ctxt).cur, count as libc::c_int);
            (*ctxt).cur = in_0;
            return ret;
        }
    }
    return xmlXPathParseNameComplex(ctxt, 1 as libc::c_int);
}
unsafe extern "C" fn xmlXPathParseNameComplex(
    mut ctxt: xmlXPathParserContextPtr,
    mut qualified: libc::c_int,
) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = xmlXPathCurrentChar(ctxt, &mut l);
    if c == ' ' as i32 || c == '>' as i32 || c == '/' as i32 || c == '[' as i32
        || c == ']' as i32 || c == '@' as i32 || c == '*' as i32
        || !((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup)
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0) && c != '_' as i32 && (qualified == 0 || c != ':' as i32)
    {
        return 0 as *mut xmlChar;
    }
    while c != ' ' as i32 && c != '>' as i32 && c != '/' as i32
        && ((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup)
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int) as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsDigitGroup)
            }) != 0 || c == '.' as i32 || c == '-' as i32 || c == '_' as i32
            || qualified != 0 && c == ':' as i32
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsCombiningGroup)
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (c == 0xb7 as libc::c_int) as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsExtenderGroup)
            }) != 0)
    {
        if l == 1 as libc::c_int {
            let fresh72 = len;
            len = len + 1;
            buf[fresh72 as usize] = c as xmlChar;
        } else {
            len += xmlCopyChar(l, &mut *buf.as_mut_ptr().offset(len as isize), c);
        }
        (*ctxt).cur = ((*ctxt).cur).offset(l as isize);
        c = xmlXPathCurrentChar(ctxt, &mut l);
        if len >= 100 as libc::c_int {
            let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
            let mut max: libc::c_int = len * 2 as libc::c_int;
            if len > 50000 as libc::c_int {
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                return 0 as *mut xmlChar;
            }
            buffer = xmlMallocAtomic
                .expect(
                    "non-null function pointer",
                )(
                (max as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<xmlChar>() as libc::c_ulong),
            ) as *mut xmlChar;
            if buffer.is_null() {
                xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as libc::c_int);
                return 0 as *mut xmlChar;
            }
            memcpy(
                buffer as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                len as libc::c_ulong,
            );
            while (if c < 0x100 as libc::c_int {
                (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                    || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                    || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                    || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                    || 0xf8 as libc::c_int <= c) as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup)
            }) != 0
                || (if c < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                        || c == 0x3007 as libc::c_int
                        || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                        as libc::c_int
                }) != 0
                || (if c < 0x100 as libc::c_int {
                    (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int) as libc::c_int
                } else {
                    xmlCharInRange(c as libc::c_uint, &xmlIsDigitGroup)
                }) != 0 || c == '.' as i32 || c == '-' as i32 || c == '_' as i32
                || qualified != 0 && c == ':' as i32
                || (if c < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    xmlCharInRange(c as libc::c_uint, &xmlIsCombiningGroup)
                }) != 0
                || (if c < 0x100 as libc::c_int {
                    (c == 0xb7 as libc::c_int) as libc::c_int
                } else {
                    xmlCharInRange(c as libc::c_uint, &xmlIsExtenderGroup)
                }) != 0
            {
                if len + 10 as libc::c_int > max {
                    if max > 50000 as libc::c_int {
                        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                        return 0 as *mut xmlChar;
                    }
                    max *= 2 as libc::c_int;
                    buffer = xmlRealloc
                        .expect(
                            "non-null function pointer",
                        )(
                        buffer as *mut libc::c_void,
                        (max as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<xmlChar>() as libc::c_ulong,
                            ),
                    ) as *mut xmlChar;
                    if buffer.is_null() {
                        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as libc::c_int);
                        return 0 as *mut xmlChar;
                    }
                }
                if l == 1 as libc::c_int {
                    let fresh73 = len;
                    len = len + 1;
                    *buffer.offset(fresh73 as isize) = c as xmlChar;
                } else {
                    len += xmlCopyChar(l, &mut *buffer.offset(len as isize), c);
                }
                (*ctxt).cur = ((*ctxt).cur).offset(l as isize);
                c = xmlXPathCurrentChar(ctxt, &mut l);
            }
            *buffer.offset(len as isize) = 0 as libc::c_int as xmlChar;
            return buffer;
        }
    }
    if len == 0 as libc::c_int {
        return 0 as *mut xmlChar;
    }
    return xmlStrndup(buf.as_mut_ptr(), len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathStringEvalNumber(
    mut str: *const xmlChar,
) -> libc::c_double {
    let mut cur: *const xmlChar = str;
    let mut ret: libc::c_double = 0.;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut isneg: libc::c_int = 0 as libc::c_int;
    let mut exponent: libc::c_int = 0 as libc::c_int;
    let mut is_exponent_negative: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut temp: libc::c_double = 0.;
    if cur.is_null() {
        return 0 as libc::c_int as libc::c_double;
    }
    while *cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *cur as libc::c_int
            && *cur as libc::c_int <= 0xa as libc::c_int
        || *cur as libc::c_int == 0xd as libc::c_int
    {
        cur = cur.offset(1);
        cur;
    }
    if *cur as libc::c_int != '.' as i32
        && ((*cur as libc::c_int) < '0' as i32 || *cur as libc::c_int > '9' as i32)
        && *cur as libc::c_int != '-' as i32
    {
        return ::core::f32::NAN as libc::c_double;
    }
    if *cur as libc::c_int == '-' as i32 {
        isneg = 1 as libc::c_int;
        cur = cur.offset(1);
        cur;
    }
    ret = 0 as libc::c_int as libc::c_double;
    while *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 {
        ret = ret * 10 as libc::c_int as libc::c_double;
        tmp = (*cur as libc::c_int - '0' as i32) as libc::c_ulong;
        ok = 1 as libc::c_int;
        cur = cur.offset(1);
        cur;
        temp = tmp as libc::c_double;
        ret = ret + temp;
    }
    if *cur as libc::c_int == '.' as i32 {
        let mut v: libc::c_int = 0;
        let mut frac: libc::c_int = 0 as libc::c_int;
        let mut max: libc::c_int = 0;
        let mut fraction: libc::c_double = 0 as libc::c_int as libc::c_double;
        cur = cur.offset(1);
        cur;
        if ((*cur as libc::c_int) < '0' as i32 || *cur as libc::c_int > '9' as i32)
            && ok == 0
        {
            return ::core::f32::NAN as libc::c_double;
        }
        while *cur as libc::c_int == '0' as i32 {
            frac = frac + 1 as libc::c_int;
            cur = cur.offset(1);
            cur;
        }
        max = frac + 20 as libc::c_int;
        while *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32
            && frac < max
        {
            v = *cur as libc::c_int - '0' as i32;
            fraction = fraction * 10 as libc::c_int as libc::c_double
                + v as libc::c_double;
            frac = frac + 1 as libc::c_int;
            cur = cur.offset(1);
            cur;
        }
        fraction /= pow(10.0f64, frac as libc::c_double);
        ret = ret + fraction;
        while *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 {
            cur = cur.offset(1);
            cur;
        }
    }
    if *cur as libc::c_int == 'e' as i32 || *cur as libc::c_int == 'E' as i32 {
        cur = cur.offset(1);
        cur;
        if *cur as libc::c_int == '-' as i32 {
            is_exponent_negative = 1 as libc::c_int;
            cur = cur.offset(1);
            cur;
        } else if *cur as libc::c_int == '+' as i32 {
            cur = cur.offset(1);
            cur;
        }
        while *cur as libc::c_int >= '0' as i32 && *cur as libc::c_int <= '9' as i32 {
            if exponent < 1000000 as libc::c_int {
                exponent = exponent * 10 as libc::c_int
                    + (*cur as libc::c_int - '0' as i32);
            }
            cur = cur.offset(1);
            cur;
        }
    }
    while *cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *cur as libc::c_int
            && *cur as libc::c_int <= 0xa as libc::c_int
        || *cur as libc::c_int == 0xd as libc::c_int
    {
        cur = cur.offset(1);
        cur;
    }
    if *cur as libc::c_int != 0 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if isneg != 0 {
        ret = -ret;
    }
    if is_exponent_negative != 0 {
        exponent = -exponent;
    }
    ret *= pow(10.0f64, exponent as libc::c_double);
    return ret;
}
unsafe extern "C" fn xmlXPathCompNumber(mut ctxt: xmlXPathParserContextPtr) {
    let mut ret: libc::c_double = 0.0f64;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut exponent: libc::c_int = 0 as libc::c_int;
    let mut is_exponent_negative: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut temp: libc::c_double = 0.;
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    if *(*ctxt).cur as libc::c_int != '.' as i32
        && ((*(*ctxt).cur as libc::c_int) < '0' as i32
            || *(*ctxt).cur as libc::c_int > '9' as i32)
    {
        xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as libc::c_int);
        return;
    }
    ret = 0 as libc::c_int as libc::c_double;
    while *(*ctxt).cur as libc::c_int >= '0' as i32
        && *(*ctxt).cur as libc::c_int <= '9' as i32
    {
        ret = ret * 10 as libc::c_int as libc::c_double;
        tmp = (*(*ctxt).cur as libc::c_int - '0' as i32) as libc::c_ulong;
        ok = 1 as libc::c_int;
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        temp = tmp as libc::c_double;
        ret = ret + temp;
    }
    if *(*ctxt).cur as libc::c_int == '.' as i32 {
        let mut v: libc::c_int = 0;
        let mut frac: libc::c_int = 0 as libc::c_int;
        let mut max: libc::c_int = 0;
        let mut fraction: libc::c_double = 0 as libc::c_int as libc::c_double;
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        if ((*(*ctxt).cur as libc::c_int) < '0' as i32
            || *(*ctxt).cur as libc::c_int > '9' as i32) && ok == 0
        {
            xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as libc::c_int);
            return;
        }
        while *(*ctxt).cur as libc::c_int == '0' as i32 {
            frac = frac + 1 as libc::c_int;
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        max = frac + 20 as libc::c_int;
        while *(*ctxt).cur as libc::c_int >= '0' as i32
            && *(*ctxt).cur as libc::c_int <= '9' as i32 && frac < max
        {
            v = *(*ctxt).cur as libc::c_int - '0' as i32;
            fraction = fraction * 10 as libc::c_int as libc::c_double
                + v as libc::c_double;
            frac = frac + 1 as libc::c_int;
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        fraction /= pow(10.0f64, frac as libc::c_double);
        ret = ret + fraction;
        while *(*ctxt).cur as libc::c_int >= '0' as i32
            && *(*ctxt).cur as libc::c_int <= '9' as i32
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
    if *(*ctxt).cur as libc::c_int == 'e' as i32
        || *(*ctxt).cur as libc::c_int == 'E' as i32
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        if *(*ctxt).cur as libc::c_int == '-' as i32 {
            is_exponent_negative = 1 as libc::c_int;
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        } else if *(*ctxt).cur as libc::c_int == '+' as i32 {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        while *(*ctxt).cur as libc::c_int >= '0' as i32
            && *(*ctxt).cur as libc::c_int <= '9' as i32
        {
            if exponent < 1000000 as libc::c_int {
                exponent = exponent * 10 as libc::c_int
                    + (*(*ctxt).cur as libc::c_int - '0' as i32);
            }
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        if is_exponent_negative != 0 {
            exponent = -exponent;
        }
        ret *= pow(10.0f64, exponent as libc::c_double);
    }
    xmlXPathCompExprAdd(
        (*ctxt).comp,
        (*(*ctxt).comp).last,
        -(1 as libc::c_int),
        XPATH_OP_VALUE,
        XPATH_NUMBER as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        xmlXPathCacheNewFloat((*ctxt).context, ret) as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn xmlXPathParseLiteral(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if *(*ctxt).cur as libc::c_int == '"' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        q = (*ctxt).cur;
        while (0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
            && *(*ctxt).cur as libc::c_int != '"' as i32
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        if !(0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int);
            return 0 as *mut xmlChar;
        } else {
            ret = xmlStrndup(
                q,
                ((*ctxt).cur).offset_from(q) as libc::c_long as libc::c_int,
            );
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    } else if *(*ctxt).cur as libc::c_int == '\'' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        q = (*ctxt).cur;
        while (0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
            && *(*ctxt).cur as libc::c_int != '\'' as i32
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        if !(0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int);
            return 0 as *mut xmlChar;
        } else {
            ret = xmlStrndup(
                q,
                ((*ctxt).cur).offset_from(q) as libc::c_long as libc::c_int,
            );
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    } else {
        xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as libc::c_int);
        return 0 as *mut xmlChar;
    }
    return ret;
}
unsafe extern "C" fn xmlXPathCompLiteral(mut ctxt: xmlXPathParserContextPtr) {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if *(*ctxt).cur as libc::c_int == '"' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        q = (*ctxt).cur;
        while (0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
            && *(*ctxt).cur as libc::c_int != '"' as i32
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        if !(0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int);
            return;
        } else {
            ret = xmlStrndup(
                q,
                ((*ctxt).cur).offset_from(q) as libc::c_long as libc::c_int,
            );
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    } else if *(*ctxt).cur as libc::c_int == '\'' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        q = (*ctxt).cur;
        while (0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
            && *(*ctxt).cur as libc::c_int != '\'' as i32
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        if !(0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *(*ctxt).cur as libc::c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as libc::c_int);
            return;
        } else {
            ret = xmlStrndup(
                q,
                ((*ctxt).cur).offset_from(q) as libc::c_long as libc::c_int,
            );
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    } else {
        xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as libc::c_int);
        return;
    }
    if ret.is_null() {
        return;
    }
    xmlXPathCompExprAdd(
        (*ctxt).comp,
        (*(*ctxt).comp).last,
        -(1 as libc::c_int),
        XPATH_OP_VALUE,
        XPATH_STRING as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        xmlXPathCacheNewString((*ctxt).context, ret) as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
}
unsafe extern "C" fn xmlXPathCompVariableReference(mut ctxt: xmlXPathParserContextPtr) {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int != '$' as i32 {
        xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as libc::c_int);
        return;
    }
    if *(*ctxt).cur as libc::c_int != 0 {
        (*ctxt).cur = ((*ctxt).cur).offset(1);
        (*ctxt).cur;
    } else {};
    name = xmlXPathParseQName(ctxt, &mut prefix);
    if name.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
        xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as libc::c_int);
        return;
    }
    (*(*ctxt).comp).last = -(1 as libc::c_int);
    xmlXPathCompExprAdd(
        (*ctxt).comp,
        (*(*ctxt).comp).last,
        -(1 as libc::c_int),
        XPATH_OP_VARIABLE,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        name as *mut libc::c_void,
        prefix as *mut libc::c_void,
    );
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if !((*ctxt).context).is_null()
        && (*(*ctxt).context).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        xmlXPathErr(ctxt, XPATH_FORBID_VARIABLE_ERROR as libc::c_int);
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathIsNodeType(mut name: *const xmlChar) -> libc::c_int {
    if name.is_null() {
        return 0 as libc::c_int;
    }
    if xmlStrEqual(name, b"node\0" as *const u8 as *const libc::c_char as *mut xmlChar)
        != 0
    {
        return 1 as libc::c_int;
    }
    if xmlStrEqual(name, b"text\0" as *const u8 as *const libc::c_char as *mut xmlChar)
        != 0
    {
        return 1 as libc::c_int;
    }
    if xmlStrEqual(
        name,
        b"comment\0" as *const u8 as *const libc::c_char as *mut xmlChar,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if xmlStrEqual(
        name,
        b"processing-instruction\0" as *const u8 as *const libc::c_char as *mut xmlChar,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXPathCompFunctionCall(mut ctxt: xmlXPathParserContextPtr) {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut nbargs: libc::c_int = 0 as libc::c_int;
    let mut sort: libc::c_int = 1 as libc::c_int;
    name = xmlXPathParseQName(ctxt, &mut prefix);
    if name.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int != '(' as i32 {
        xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
        return;
    }
    if *(*ctxt).cur as libc::c_int != 0 {
        (*ctxt).cur = ((*ctxt).cur).offset(1);
        (*ctxt).cur;
    } else {};
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if prefix.is_null()
        && *name.offset(0 as libc::c_int as isize) as libc::c_int == 'c' as i32
        && xmlStrEqual(
            name,
            b"count\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
    {
        sort = 0 as libc::c_int;
    }
    (*(*ctxt).comp).last = -(1 as libc::c_int);
    if *(*ctxt).cur as libc::c_int != ')' as i32 {
        while *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
            let mut op1: libc::c_int = (*(*ctxt).comp).last;
            (*(*ctxt).comp).last = -(1 as libc::c_int);
            xmlXPathCompileExpr(ctxt, sort);
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
                xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
                return;
            }
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                op1,
                (*(*ctxt).comp).last,
                XPATH_OP_ARG,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
            nbargs += 1;
            nbargs;
            if *(*ctxt).cur as libc::c_int == ')' as i32 {
                break;
            }
            if *(*ctxt).cur as libc::c_int != ',' as i32 {
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
                xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                return;
            }
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
            }
        }
    }
    xmlXPathCompExprAdd(
        (*ctxt).comp,
        (*(*ctxt).comp).last,
        -(1 as libc::c_int),
        XPATH_OP_FUNCTION,
        nbargs,
        0 as libc::c_int,
        0 as libc::c_int,
        name as *mut libc::c_void,
        prefix as *mut libc::c_void,
    );
    if *(*ctxt).cur as libc::c_int != 0 {
        (*ctxt).cur = ((*ctxt).cur).offset(1);
        (*ctxt).cur;
    } else {};
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
}
unsafe extern "C" fn xmlXPathCompPrimaryExpr(mut ctxt: xmlXPathParserContextPtr) {
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int == '$' as i32 {
        xmlXPathCompVariableReference(ctxt);
    } else if *(*ctxt).cur as libc::c_int == '(' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompileExpr(ctxt, 1 as libc::c_int);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        if *(*ctxt).cur as libc::c_int != ')' as i32 {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
            return;
        }
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    } else if 0x30 as libc::c_int <= *(*ctxt).cur as libc::c_int
        && *(*ctxt).cur as libc::c_int <= 0x39 as libc::c_int
        || *(*ctxt).cur as libc::c_int == '.' as i32
            && (0x30 as libc::c_int
                <= *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                    <= 0x39 as libc::c_int)
    {
        xmlXPathCompNumber(ctxt);
    } else if *(*ctxt).cur as libc::c_int == '\'' as i32
        || *(*ctxt).cur as libc::c_int == '"' as i32
    {
        xmlXPathCompLiteral(ctxt);
    } else {
        xmlXPathCompFunctionCall(ctxt);
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
}
unsafe extern "C" fn xmlXPathCompFilterExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompPrimaryExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '[' as i32 {
        xmlXPathCompPredicate(ctxt, 1 as libc::c_int);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathScanName(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut xmlChar {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    cur = (*ctxt).cur;
    c = xmlXPathCurrentChar(ctxt, &mut l);
    if c == ' ' as i32 || c == '>' as i32 || c == '/' as i32
        || !((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup)
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0) && c != '_' as i32 && c != ':' as i32
    {
        return 0 as *mut xmlChar;
    }
    while c != ' ' as i32 && c != '>' as i32 && c != '/' as i32
        && ((if c < 0x100 as libc::c_int {
            (0x41 as libc::c_int <= c && c <= 0x5a as libc::c_int
                || 0x61 as libc::c_int <= c && c <= 0x7a as libc::c_int
                || 0xc0 as libc::c_int <= c && c <= 0xd6 as libc::c_int
                || 0xd8 as libc::c_int <= c && c <= 0xf6 as libc::c_int
                || 0xf8 as libc::c_int <= c) as libc::c_int
        } else {
            xmlCharInRange(c as libc::c_uint, &xmlIsBaseCharGroup)
        }) != 0
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                (0x4e00 as libc::c_int <= c && c <= 0x9fa5 as libc::c_int
                    || c == 0x3007 as libc::c_int
                    || 0x3021 as libc::c_int <= c && c <= 0x3029 as libc::c_int)
                    as libc::c_int
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (0x30 as libc::c_int <= c && c <= 0x39 as libc::c_int) as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsDigitGroup)
            }) != 0 || c == '.' as i32 || c == '-' as i32 || c == '_' as i32
            || c == ':' as i32
            || (if c < 0x100 as libc::c_int {
                0 as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsCombiningGroup)
            }) != 0
            || (if c < 0x100 as libc::c_int {
                (c == 0xb7 as libc::c_int) as libc::c_int
            } else {
                xmlCharInRange(c as libc::c_uint, &xmlIsExtenderGroup)
            }) != 0)
    {
        len += l;
        (*ctxt).cur = ((*ctxt).cur).offset(l as isize);
        c = xmlXPathCurrentChar(ctxt, &mut l);
    }
    ret = xmlStrndup(cur, ((*ctxt).cur).offset_from(cur) as libc::c_long as libc::c_int);
    (*ctxt).cur = cur;
    return ret;
}
unsafe extern "C" fn xmlXPathCompPathExpr(mut ctxt: xmlXPathParserContextPtr) {
    let mut lc: libc::c_int = 1 as libc::c_int;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int == '$' as i32
        || *(*ctxt).cur as libc::c_int == '(' as i32
        || 0x30 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0x39 as libc::c_int
        || *(*ctxt).cur as libc::c_int == '\'' as i32
        || *(*ctxt).cur as libc::c_int == '"' as i32
        || *(*ctxt).cur as libc::c_int == '.' as i32
            && (0x30 as libc::c_int
                <= *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                    <= 0x39 as libc::c_int)
    {
        lc = 0 as libc::c_int;
    } else if *(*ctxt).cur as libc::c_int == '*' as i32 {
        lc = 1 as libc::c_int;
    } else if *(*ctxt).cur as libc::c_int == '/' as i32 {
        lc = 1 as libc::c_int;
    } else if *(*ctxt).cur as libc::c_int == '@' as i32 {
        lc = 1 as libc::c_int;
    } else if *(*ctxt).cur as libc::c_int == '.' as i32 {
        lc = 1 as libc::c_int;
    } else {
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        name = xmlXPathScanName(ctxt);
        if !name.is_null()
            && !(xmlStrstr(
                name,
                b"::\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ))
                .is_null()
        {
            lc = 1 as libc::c_int;
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        } else if !name.is_null() {
            let mut len: libc::c_int = xmlStrlen(name);
            while *((*ctxt).cur).offset(len as isize) as libc::c_int != 0 as libc::c_int
            {
                if *((*ctxt).cur).offset(len as isize) as libc::c_int == '/' as i32 {
                    lc = 1 as libc::c_int;
                    break;
                } else if *((*ctxt).cur).offset(len as isize) as libc::c_int
                    == 0x20 as libc::c_int
                    || 0x9 as libc::c_int
                        <= *((*ctxt).cur).offset(len as isize) as libc::c_int
                        && *((*ctxt).cur).offset(len as isize) as libc::c_int
                            <= 0xa as libc::c_int
                    || *((*ctxt).cur).offset(len as isize) as libc::c_int
                        == 0xd as libc::c_int
                {
                    len += 1;
                    len;
                } else if *((*ctxt).cur).offset(len as isize) as libc::c_int
                    == ':' as i32
                {
                    lc = 1 as libc::c_int;
                    break;
                } else if *((*ctxt).cur).offset(len as isize) as libc::c_int
                    == '(' as i32
                {
                    if xmlXPathIsNodeType(name) != 0 {
                        lc = 1 as libc::c_int;
                    } else if (*ctxt).xptr != 0
                        && xmlStrEqual(
                            name,
                            b"range-to\0" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        ) != 0
                    {
                        lc = 1 as libc::c_int;
                    } else {
                        lc = 0 as libc::c_int;
                    }
                    break;
                } else if *((*ctxt).cur).offset(len as isize) as libc::c_int
                    == '[' as i32
                {
                    lc = 1 as libc::c_int;
                    break;
                } else if *((*ctxt).cur).offset(len as isize) as libc::c_int
                    == '<' as i32
                    || *((*ctxt).cur).offset(len as isize) as libc::c_int == '>' as i32
                    || *((*ctxt).cur).offset(len as isize) as libc::c_int == '=' as i32
                {
                    lc = 1 as libc::c_int;
                    break;
                } else {
                    lc = 1 as libc::c_int;
                    break;
                }
            }
            if *((*ctxt).cur).offset(len as isize) as libc::c_int == 0 as libc::c_int {
                lc = 1 as libc::c_int;
            }
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        } else {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
            return;
        }
    }
    if lc != 0 {
        if *(*ctxt).cur as libc::c_int == '/' as i32 {
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                XPATH_OP_ROOT,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        } else {
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                XPATH_OP_NODE,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        }
        xmlXPathCompLocationPath(ctxt);
    } else {
        xmlXPathCompFilterExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        if *(*ctxt).cur as libc::c_int == '/' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == '/' as i32
        {
            (*ctxt).cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
            while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
            }
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                (*(*ctxt).comp).last,
                -(1 as libc::c_int),
                XPATH_OP_COLLECT,
                AXIS_DESCENDANT_OR_SELF as libc::c_int,
                NODE_TEST_TYPE as libc::c_int,
                NODE_TYPE_NODE as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
            xmlXPathCompRelativeLocationPath(ctxt);
        } else if *(*ctxt).cur as libc::c_int == '/' as i32 {
            xmlXPathCompRelativeLocationPath(ctxt);
        }
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
}
unsafe extern "C" fn xmlXPathCompUnionExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompPathExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '|' as i32 {
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            XPATH_OP_NODE,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompPathExpr(ctxt);
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_UNION,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompUnaryExpr(mut ctxt: xmlXPathParserContextPtr) {
    let mut minus: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '-' as i32 {
        minus = 1 as libc::c_int - minus;
        found = 1 as libc::c_int;
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
    xmlXPathCompUnionExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    if found != 0 {
        if minus != 0 {
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                (*(*ctxt).comp).last,
                -(1 as libc::c_int),
                XPATH_OP_PLUS,
                2 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        } else {
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                (*(*ctxt).comp).last,
                -(1 as libc::c_int),
                XPATH_OP_PLUS,
                3 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        }
    }
}
unsafe extern "C" fn xmlXPathCompMultiplicativeExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompUnaryExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '*' as i32
        || *(*ctxt).cur as libc::c_int == 'd' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == 'i' as i32
            && *((*ctxt).cur).offset(2 as libc::c_int as isize) as libc::c_int
                == 'v' as i32
        || *(*ctxt).cur as libc::c_int == 'm' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == 'o' as i32
            && *((*ctxt).cur).offset(2 as libc::c_int as isize) as libc::c_int
                == 'd' as i32
    {
        let mut op: libc::c_int = -(1 as libc::c_int);
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as libc::c_int == '*' as i32 {
            op = 0 as libc::c_int;
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        } else if *(*ctxt).cur as libc::c_int == 'd' as i32 {
            op = 1 as libc::c_int;
            (*ctxt).cur = ((*ctxt).cur).offset(3 as libc::c_int as isize);
        } else if *(*ctxt).cur as libc::c_int == 'm' as i32 {
            op = 2 as libc::c_int;
            (*ctxt).cur = ((*ctxt).cur).offset(3 as libc::c_int as isize);
        }
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompUnaryExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_MULT,
            op,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompAdditiveExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompMultiplicativeExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '+' as i32
        || *(*ctxt).cur as libc::c_int == '-' as i32
    {
        let mut plus: libc::c_int = 0;
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as libc::c_int == '+' as i32 {
            plus = 1 as libc::c_int;
        } else {
            plus = 0 as libc::c_int;
        }
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompMultiplicativeExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_PLUS,
            plus,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompRelationalExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompAdditiveExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '<' as i32
        || *(*ctxt).cur as libc::c_int == '>' as i32
        || *(*ctxt).cur as libc::c_int == '<' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == '=' as i32
        || *(*ctxt).cur as libc::c_int == '>' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == '=' as i32
    {
        let mut inf: libc::c_int = 0;
        let mut strict: libc::c_int = 0;
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as libc::c_int == '<' as i32 {
            inf = 1 as libc::c_int;
        } else {
            inf = 0 as libc::c_int;
        }
        if *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
        {
            strict = 0 as libc::c_int;
        } else {
            strict = 1 as libc::c_int;
        }
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        if strict == 0 {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompAdditiveExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_CMP,
            inf,
            strict,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompEqualityExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompRelationalExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '=' as i32
        || *(*ctxt).cur as libc::c_int == '!' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == '=' as i32
    {
        let mut eq: libc::c_int = 0;
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as libc::c_int == '=' as i32 {
            eq = 1 as libc::c_int;
        } else {
            eq = 0 as libc::c_int;
        }
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        if eq == 0 {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompRelationalExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_EQUAL,
            eq,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompAndExpr(mut ctxt: xmlXPathParserContextPtr) {
    xmlXPathCompEqualityExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == 'a' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
        && *((*ctxt).cur).offset(2 as libc::c_int as isize) as libc::c_int == 'd' as i32
    {
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        (*ctxt).cur = ((*ctxt).cur).offset(3 as libc::c_int as isize);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompEqualityExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_AND,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompileExpr(
    mut ctxt: xmlXPathParserContextPtr,
    mut sort: libc::c_int,
) {
    xmlXPathCompAndExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == 'o' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == 'r' as i32
    {
        let mut op1: libc::c_int = (*(*ctxt).comp).last;
        (*ctxt).cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompAndExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_OR,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
    if sort != 0
        && (*((*(*ctxt).comp).steps).offset((*(*ctxt).comp).last as isize)).op
            as libc::c_uint != XPATH_OP_VALUE as libc::c_int as libc::c_uint
    {
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            (*(*ctxt).comp).last,
            -(1 as libc::c_int),
            XPATH_OP_SORT,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn xmlXPathCompPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut filter: libc::c_int,
) {
    let mut op1: libc::c_int = (*(*ctxt).comp).last;
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int != '[' as i32 {
        xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as libc::c_int);
        return;
    }
    if *(*ctxt).cur as libc::c_int != 0 {
        (*ctxt).cur = ((*ctxt).cur).offset(1);
        (*ctxt).cur;
    } else {};
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    (*(*ctxt).comp).last = -(1 as libc::c_int);
    if filter == 0 {
        xmlXPathCompileExpr(ctxt, 0 as libc::c_int);
    } else {
        xmlXPathCompileExpr(ctxt, 1 as libc::c_int);
    }
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    if *(*ctxt).cur as libc::c_int != ']' as i32 {
        xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as libc::c_int);
        return;
    }
    if filter != 0 {
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_FILTER,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    } else {
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_PREDICATE,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    }
    if *(*ctxt).cur as libc::c_int != 0 {
        (*ctxt).cur = ((*ctxt).cur).offset(1);
        (*ctxt).cur;
    } else {};
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
}
unsafe extern "C" fn xmlXPathCompNodeTest(
    mut ctxt: xmlXPathParserContextPtr,
    mut test: *mut xmlXPathTestVal,
    mut type_0: *mut xmlXPathTypeVal,
    mut prefix: *mut *const xmlChar,
    mut name: *mut xmlChar,
) -> *mut xmlChar {
    let mut blanks: libc::c_int = 0;
    if test.is_null() || type_0.is_null() || prefix.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Internal error at %s:%d\n\0" as *const u8 as *const libc::c_char,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            11099 as libc::c_int,
        );
        return 0 as *mut xmlChar;
    }
    *type_0 = NODE_TYPE_NODE;
    *test = NODE_TEST_NONE;
    *prefix = 0 as *const xmlChar;
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if name.is_null() && *(*ctxt).cur as libc::c_int == '*' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        *test = NODE_TEST_ALL;
        return 0 as *mut xmlChar;
    }
    if name.is_null() {
        name = xmlXPathParseNCName(ctxt);
    }
    if name.is_null() {
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
        return 0 as *mut xmlChar;
    }
    blanks = (*(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int) as libc::c_int;
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int == '(' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        if xmlStrEqual(
            name,
            b"comment\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_COMMENT;
        } else if xmlStrEqual(
            name,
            b"node\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_NODE;
        } else if xmlStrEqual(
            name,
            b"processing-instruction\0" as *const u8 as *const libc::c_char
                as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_PI;
        } else if xmlStrEqual(
            name,
            b"text\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_TEXT;
        } else {
            if !name.is_null() {
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
            }
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
            return 0 as *mut xmlChar;
        }
        *test = NODE_TEST_TYPE;
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        if *type_0 as libc::c_uint == NODE_TYPE_PI as libc::c_int as libc::c_uint {
            if !name.is_null() {
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
            }
            name = 0 as *mut xmlChar;
            if *(*ctxt).cur as libc::c_int != ')' as i32 {
                name = xmlXPathParseLiteral(ctxt);
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as *mut xmlChar;
                }
                *test = NODE_TEST_PI;
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                }
            }
        }
        if *(*ctxt).cur as libc::c_int != ')' as i32 {
            if !name.is_null() {
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
            }
            xmlXPathErr(ctxt, XPATH_UNCLOSED_ERROR as libc::c_int);
            return 0 as *mut xmlChar;
        }
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        return name;
    }
    *test = NODE_TEST_NAME;
    if blanks == 0 && *(*ctxt).cur as libc::c_int == ':' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        *prefix = name;
        if *(*ctxt).cur as libc::c_int == '*' as i32 {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            *test = NODE_TEST_ALL;
            return 0 as *mut xmlChar;
        }
        name = xmlXPathParseNCName(ctxt);
        if name.is_null() {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
            return 0 as *mut xmlChar;
        }
    }
    return name;
}
unsafe extern "C" fn xmlXPathIsAxisName(mut name: *const xmlChar) -> xmlXPathAxisVal {
    let mut ret: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
    match *name.offset(0 as libc::c_int as isize) as libc::c_int {
        97 => {
            if xmlStrEqual(
                name,
                b"ancestor\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_ANCESTOR;
            }
            if xmlStrEqual(
                name,
                b"ancestor-or-self\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_ANCESTOR_OR_SELF;
            }
            if xmlStrEqual(
                name,
                b"attribute\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_ATTRIBUTE;
            }
        }
        99 => {
            if xmlStrEqual(
                name,
                b"child\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_CHILD;
            }
        }
        100 => {
            if xmlStrEqual(
                name,
                b"descendant\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_DESCENDANT;
            }
            if xmlStrEqual(
                name,
                b"descendant-or-self\0" as *const u8 as *const libc::c_char
                    as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_DESCENDANT_OR_SELF;
            }
        }
        102 => {
            if xmlStrEqual(
                name,
                b"following\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_FOLLOWING;
            }
            if xmlStrEqual(
                name,
                b"following-sibling\0" as *const u8 as *const libc::c_char
                    as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_FOLLOWING_SIBLING;
            }
        }
        110 => {
            if xmlStrEqual(
                name,
                b"namespace\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_NAMESPACE;
            }
        }
        112 => {
            if xmlStrEqual(
                name,
                b"parent\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_PARENT;
            }
            if xmlStrEqual(
                name,
                b"preceding\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_PRECEDING;
            }
            if xmlStrEqual(
                name,
                b"preceding-sibling\0" as *const u8 as *const libc::c_char
                    as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_PRECEDING_SIBLING;
            }
        }
        115 => {
            if xmlStrEqual(
                name,
                b"self\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_SELF;
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn xmlXPathCompStep(mut ctxt: xmlXPathParserContextPtr) {
    let mut rangeto: libc::c_int = 0 as libc::c_int;
    let mut op2: libc::c_int = -(1 as libc::c_int);
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int == '.' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        (*ctxt).cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            (*(*ctxt).comp).last,
            -(1 as libc::c_int),
            XPATH_OP_COLLECT,
            AXIS_PARENT as libc::c_int,
            NODE_TEST_TYPE as libc::c_int,
            NODE_TYPE_NODE as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    } else if *(*ctxt).cur as libc::c_int == '.' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    } else {
        let mut current_block_88: u64;
        let mut name: *mut xmlChar = 0 as *mut xmlChar;
        let mut prefix: *const xmlChar = 0 as *const xmlChar;
        let mut test: xmlXPathTestVal = NODE_TEST_NONE;
        let mut axis: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
        let mut type_0: xmlXPathTypeVal = NODE_TYPE_NODE;
        let mut op1: libc::c_int = 0;
        if (*ctxt).xptr != 0 {
            name = xmlXPathParseNCName(ctxt);
            if !name.is_null()
                && xmlStrEqual(
                    name,
                    b"range-to\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                ) != 0
            {
                op2 = (*(*ctxt).comp).last;
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                }
                if *(*ctxt).cur as libc::c_int != '(' as i32 {
                    xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                    return;
                }
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                }
                xmlXPathCompileExpr(ctxt, 1 as libc::c_int);
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return;
                }
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                }
                if *(*ctxt).cur as libc::c_int != ')' as i32 {
                    xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
                    return;
                }
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                rangeto = 1 as libc::c_int;
                current_block_88 = 9973656708329234690;
            } else {
                current_block_88 = 7333393191927787629;
            }
        } else {
            current_block_88 = 7333393191927787629;
        }
        match current_block_88 {
            7333393191927787629 => {
                if *(*ctxt).cur as libc::c_int == '*' as i32 {
                    axis = AXIS_CHILD;
                } else {
                    if name.is_null() {
                        name = xmlXPathParseNCName(ctxt);
                    }
                    if !name.is_null() {
                        axis = xmlXPathIsAxisName(name);
                        if axis as libc::c_uint != 0 as libc::c_int as libc::c_uint {
                            while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                            {
                                if *(*ctxt).cur as libc::c_int != 0 {
                                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                                    (*ctxt).cur;
                                } else {};
                            }
                            if *(*ctxt).cur as libc::c_int == ':' as i32
                                && *((*ctxt).cur).offset(1 as libc::c_int as isize)
                                    as libc::c_int == ':' as i32
                            {
                                (*ctxt)
                                    .cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(name as *mut libc::c_void);
                                name = 0 as *mut xmlChar;
                            } else {
                                axis = AXIS_CHILD;
                            }
                        } else {
                            axis = AXIS_CHILD;
                        }
                    } else if *(*ctxt).cur as libc::c_int == '@' as i32 {
                        if *(*ctxt).cur as libc::c_int != 0 {
                            (*ctxt).cur = ((*ctxt).cur).offset(1);
                            (*ctxt).cur;
                        } else {};
                        axis = AXIS_ATTRIBUTE;
                    } else {
                        axis = AXIS_CHILD;
                    }
                }
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    xmlFree
                        .expect("non-null function pointer")(name as *mut libc::c_void);
                    return;
                }
                name = xmlXPathCompNodeTest(
                    ctxt,
                    &mut test,
                    &mut type_0,
                    &mut prefix,
                    name,
                );
                if test as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                    return;
                }
                if !prefix.is_null() && !((*ctxt).context).is_null()
                    && (*(*ctxt).context).flags & (1 as libc::c_int) << 0 as libc::c_int
                        != 0
                {
                    if (xmlXPathNsLookup((*ctxt).context, prefix)).is_null() {
                        xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as libc::c_int);
                    }
                }
            }
            _ => {}
        }
        op1 = (*(*ctxt).comp).last;
        (*(*ctxt).comp).last = -(1 as libc::c_int);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        while *(*ctxt).cur as libc::c_int == '[' as i32 {
            xmlXPathCompPredicate(ctxt, 0 as libc::c_int);
        }
        if rangeto != 0 {
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                op2,
                op1,
                XPATH_OP_RANGETO,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        } else {
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                op1,
                (*(*ctxt).comp).last,
                XPATH_OP_COLLECT,
                axis as libc::c_int,
                test as libc::c_int,
                type_0 as libc::c_int,
                prefix as *mut libc::c_void,
                name as *mut libc::c_void,
            );
        }
    };
}
unsafe extern "C" fn xmlXPathCompRelativeLocationPath(
    mut ctxt: xmlXPathParserContextPtr,
) {
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int == '/' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        (*ctxt).cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
        xmlXPathCompExprAdd(
            (*ctxt).comp,
            (*(*ctxt).comp).last,
            -(1 as libc::c_int),
            XPATH_OP_COLLECT,
            AXIS_DESCENDANT_OR_SELF as libc::c_int,
            NODE_TEST_TYPE as libc::c_int,
            NODE_TYPE_NODE as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    } else if *(*ctxt).cur as libc::c_int == '/' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
    xmlXPathCompStep(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return;
    }
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    while *(*ctxt).cur as libc::c_int == '/' as i32 {
        if *(*ctxt).cur as libc::c_int == '/' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == '/' as i32
        {
            (*ctxt).cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
            while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
            }
            xmlXPathCompExprAdd(
                (*ctxt).comp,
                (*(*ctxt).comp).last,
                -(1 as libc::c_int),
                XPATH_OP_COLLECT,
                AXIS_DESCENDANT_OR_SELF as libc::c_int,
                NODE_TEST_TYPE as libc::c_int,
                NODE_TYPE_NODE as libc::c_int,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
            xmlXPathCompStep(ctxt);
        } else if *(*ctxt).cur as libc::c_int == '/' as i32 {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
            }
            xmlXPathCompStep(ctxt);
        }
        while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
        {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
        }
    }
}
unsafe extern "C" fn xmlXPathCompLocationPath(mut ctxt: xmlXPathParserContextPtr) {
    while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
        || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
            && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
        || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
    }
    if *(*ctxt).cur as libc::c_int != '/' as i32 {
        xmlXPathCompRelativeLocationPath(ctxt);
    } else {
        while *(*ctxt).cur as libc::c_int == '/' as i32 {
            if *(*ctxt).cur as libc::c_int == '/' as i32
                && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                    == '/' as i32
            {
                (*ctxt).cur = ((*ctxt).cur).offset(2 as libc::c_int as isize);
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                }
                xmlXPathCompExprAdd(
                    (*ctxt).comp,
                    (*(*ctxt).comp).last,
                    -(1 as libc::c_int),
                    XPATH_OP_COLLECT,
                    AXIS_DESCENDANT_OR_SELF as libc::c_int,
                    NODE_TEST_TYPE as libc::c_int,
                    NODE_TYPE_NODE as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                );
                xmlXPathCompRelativeLocationPath(ctxt);
            } else if *(*ctxt).cur as libc::c_int == '/' as i32 {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                while *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                }
                if *(*ctxt).cur as libc::c_int != 0 as libc::c_int
                    && (0x41 as libc::c_int <= *(*ctxt).cur as libc::c_int
                        && *(*ctxt).cur as libc::c_int <= 0x5a as libc::c_int
                        || 0x61 as libc::c_int <= *(*ctxt).cur as libc::c_int
                            && *(*ctxt).cur as libc::c_int <= 0x7a as libc::c_int
                        || *(*ctxt).cur as libc::c_int == '_' as i32
                        || *(*ctxt).cur as libc::c_int == '.' as i32
                        || *(*ctxt).cur as libc::c_int == '@' as i32
                        || *(*ctxt).cur as libc::c_int == '*' as i32)
                {
                    xmlXPathCompRelativeLocationPath(ctxt);
                }
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return;
            }
        }
    };
}
unsafe extern "C" fn xmlXPathCompOpEvalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut set: xmlNodeSetPtr,
    mut contextSize: libc::c_int,
    mut hasNsNodes: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if (*op).ch1 != -(1 as libc::c_int) {
        let mut comp: xmlXPathCompExprPtr = (*ctxt).comp;
        (*((*comp).steps).offset((*op).ch1 as isize)).op as libc::c_uint
            != XPATH_OP_PREDICATE as libc::c_int as libc::c_uint;
        contextSize = xmlXPathCompOpEvalPredicate(
            ctxt,
            &mut *((*comp).steps).offset((*op).ch1 as isize),
            set,
            contextSize,
            hasNsNodes,
        );
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return 0 as libc::c_int;
        }
        if contextSize <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if (*op).ch2 != -(1 as libc::c_int) {
        let mut xpctxt: xmlXPathContextPtr = (*ctxt).context;
        let mut contextNode: xmlNodePtr = 0 as *mut xmlNode;
        let mut oldContextNode: xmlNodePtr = 0 as *mut xmlNode;
        let mut oldContextDoc: xmlDocPtr = 0 as *mut xmlDoc;
        let mut oldcs: libc::c_int = 0;
        let mut oldpp: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut res: libc::c_int = 0;
        let mut contextPos: libc::c_int = 0 as libc::c_int;
        let mut newContextSize: libc::c_int = 0;
        let mut exprOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
        let mut contextObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
        let mut exprRes: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
        oldContextNode = (*xpctxt).node;
        oldContextDoc = (*xpctxt).doc;
        oldcs = (*xpctxt).contextSize;
        oldpp = (*xpctxt).proximityPosition;
        exprOp = &mut *((*(*ctxt).comp).steps).offset((*op).ch2 as isize)
            as *mut xmlXPathStepOp;
        newContextSize = 0 as libc::c_int;
        i = 0 as libc::c_int;
        loop {
            if !(i < (*set).nodeNr) {
                current_block = 15090052786889560393;
                break;
            }
            if !(*((*set).nodeTab).offset(i as isize)).is_null() {
                contextNode = *((*set).nodeTab).offset(i as isize);
                (*xpctxt).node = contextNode;
                (*xpctxt).contextSize = contextSize;
                contextPos += 1;
                (*xpctxt).proximityPosition = contextPos;
                if (*contextNode).type_0 as libc::c_uint
                    != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    && !((*contextNode).doc).is_null()
                {
                    (*xpctxt).doc = (*contextNode).doc;
                }
                if contextObj.is_null() {
                    contextObj = xmlXPathCacheNewNodeSet(xpctxt, contextNode);
                } else if xmlXPathNodeSetAddUnique((*contextObj).nodesetval, contextNode)
                    < 0 as libc::c_int
                {
                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                    current_block = 3862262692882872440;
                    break;
                }
                valuePush(ctxt, contextObj);
                res = xmlXPathCompOpEvalToBoolean(ctxt, exprOp, 1 as libc::c_int);
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int
                    || res == -(1 as libc::c_int)
                {
                    xmlXPathNodeSetClear(set, hasNsNodes);
                    newContextSize = 0 as libc::c_int;
                    current_block = 3862262692882872440;
                    break;
                } else {
                    if res != 0 as libc::c_int {
                        newContextSize += 1;
                        newContextSize;
                    } else {
                        let ref mut fresh74 = *((*set).nodeTab).offset(i as isize);
                        *fresh74 = 0 as xmlNodePtr;
                        if (*contextNode).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            xmlXPathNodeSetFreeNs(contextNode as xmlNsPtr);
                        }
                    }
                    if (*ctxt).value == contextObj {
                        valuePop(ctxt);
                        xmlXPathNodeSetClear((*contextObj).nodesetval, hasNsNodes);
                    } else {
                        contextObj = 0 as xmlXPathObjectPtr;
                    }
                }
            }
            i += 1;
            i;
        }
        match current_block {
            15090052786889560393 => {
                if !contextObj.is_null() {
                    if (*ctxt).value == contextObj {
                        valuePop(ctxt);
                    }
                    xmlXPathReleaseObject(xpctxt, contextObj);
                }
            }
            _ => {}
        }
        if !exprRes.is_null() {
            xmlXPathReleaseObject((*ctxt).context, exprRes);
        }
        (*xpctxt).node = oldContextNode;
        (*xpctxt).doc = oldContextDoc;
        (*xpctxt).contextSize = oldcs;
        (*xpctxt).proximityPosition = oldpp;
        return newContextSize;
    } else {
        return contextSize
    };
}
unsafe extern "C" fn xmlXPathCompOpEvalPositionalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut set: xmlNodeSetPtr,
    mut contextSize: libc::c_int,
    mut minPos: libc::c_int,
    mut maxPos: libc::c_int,
    mut hasNsNodes: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if (*op).ch1 != -(1 as libc::c_int) {
        let mut comp: xmlXPathCompExprPtr = (*ctxt).comp;
        (*((*comp).steps).offset((*op).ch1 as isize)).op as libc::c_uint
            != XPATH_OP_PREDICATE as libc::c_int as libc::c_uint;
        contextSize = xmlXPathCompOpEvalPredicate(
            ctxt,
            &mut *((*comp).steps).offset((*op).ch1 as isize),
            set,
            contextSize,
            hasNsNodes,
        );
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return 0 as libc::c_int;
        }
        if contextSize <= 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if contextSize < minPos {
        xmlXPathNodeSetClear(set, hasNsNodes);
        return 0 as libc::c_int;
    }
    if (*op).ch2 == -(1 as libc::c_int) {
        return contextSize
    } else {
        let mut oldContextDoc: xmlDocPtr = 0 as *mut xmlDoc;
        let mut oldcs: libc::c_int = 0;
        let mut oldpp: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut pos: libc::c_int = 0 as libc::c_int;
        let mut newContextSize: libc::c_int = 0 as libc::c_int;
        let mut contextPos: libc::c_int = 0 as libc::c_int;
        let mut res: libc::c_int = 0;
        let mut exprOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
        let mut contextObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
        let mut exprRes: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
        let mut oldContextNode: xmlNodePtr = 0 as *mut xmlNode;
        let mut contextNode: xmlNodePtr = 0 as xmlNodePtr;
        let mut xpctxt: xmlXPathContextPtr = (*ctxt).context;
        let mut frame: libc::c_int = 0;
        oldContextNode = (*xpctxt).node;
        oldContextDoc = (*xpctxt).doc;
        oldcs = (*xpctxt).contextSize;
        oldpp = (*xpctxt).proximityPosition;
        exprOp = &mut *((*(*ctxt).comp).steps).offset((*op).ch2 as isize)
            as *mut xmlXPathStepOp;
        i = 0 as libc::c_int;
        loop {
            if !(i < (*set).nodeNr) {
                current_block = 15431279983982904846;
                break;
            }
            let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            if !(*((*set).nodeTab).offset(i as isize)).is_null() {
                contextNode = *((*set).nodeTab).offset(i as isize);
                (*xpctxt).node = contextNode;
                (*xpctxt).contextSize = contextSize;
                contextPos += 1;
                (*xpctxt).proximityPosition = contextPos;
                if (*contextNode).type_0 as libc::c_uint
                    != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                    && !((*contextNode).doc).is_null()
                {
                    (*xpctxt).doc = (*contextNode).doc;
                }
                if contextObj.is_null() {
                    contextObj = xmlXPathCacheNewNodeSet(xpctxt, contextNode);
                } else if xmlXPathNodeSetAddUnique((*contextObj).nodesetval, contextNode)
                    < 0 as libc::c_int
                {
                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                    current_block = 15431279983982904846;
                    break;
                }
                valuePush(ctxt, contextObj);
                frame = xmlXPathSetFrame(ctxt);
                res = xmlXPathCompOpEvalToBoolean(ctxt, exprOp, 1 as libc::c_int);
                xmlXPathPopFrame(ctxt, frame);
                tmp = valuePop(ctxt);
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int
                    || res == -(1 as libc::c_int)
                {
                    while tmp != contextObj {
                        xmlXPathReleaseObject(xpctxt, tmp);
                        tmp = valuePop(ctxt);
                    }
                    current_block = 15588128655216379797;
                    break;
                } else {
                    valuePush(ctxt, tmp);
                    if res != 0 {
                        pos += 1;
                        pos;
                    }
                    if res != 0 && pos >= minPos && pos <= maxPos {
                        newContextSize += 1;
                        newContextSize;
                        if minPos == maxPos {
                            if (*contextNode).type_0 as libc::c_uint
                                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                            {
                                let ref mut fresh75 = *((*set).nodeTab).offset(i as isize);
                                *fresh75 = 0 as xmlNodePtr;
                            }
                            xmlXPathNodeSetClear(set, hasNsNodes);
                            (*set).nodeNr = 1 as libc::c_int;
                            let ref mut fresh76 = *((*set).nodeTab)
                                .offset(0 as libc::c_int as isize);
                            *fresh76 = contextNode;
                            current_block = 15431279983982904846;
                            break;
                        } else if pos == maxPos {
                            xmlXPathNodeSetClearFromPos(
                                set,
                                i + 1 as libc::c_int,
                                hasNsNodes,
                            );
                            current_block = 15431279983982904846;
                            break;
                        }
                    } else {
                        let ref mut fresh77 = *((*set).nodeTab).offset(i as isize);
                        *fresh77 = 0 as xmlNodePtr;
                        if (*contextNode).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            xmlXPathNodeSetFreeNs(contextNode as xmlNsPtr);
                        }
                    }
                    if !exprRes.is_null() {
                        xmlXPathReleaseObject((*ctxt).context, exprRes);
                        exprRes = 0 as xmlXPathObjectPtr;
                    }
                    if (*ctxt).value == contextObj {
                        valuePop(ctxt);
                        xmlXPathNodeSetClear((*contextObj).nodesetval, hasNsNodes);
                    } else {
                        contextObj = 0 as xmlXPathObjectPtr;
                    }
                }
            }
            i += 1;
            i;
        }
        match current_block {
            15588128655216379797 => {
                xmlXPathNodeSetClear(set, hasNsNodes);
                newContextSize = 0 as libc::c_int;
            }
            _ => {}
        }
        if !contextObj.is_null() {
            if (*ctxt).value == contextObj {
                valuePop(ctxt);
            }
            xmlXPathReleaseObject(xpctxt, contextObj);
        }
        if !exprRes.is_null() {
            xmlXPathReleaseObject((*ctxt).context, exprRes);
        }
        (*xpctxt).node = oldContextNode;
        (*xpctxt).doc = oldContextDoc;
        (*xpctxt).contextSize = oldcs;
        (*xpctxt).proximityPosition = oldpp;
        return newContextSize;
    };
}
unsafe extern "C" fn xmlXPathIsPositionalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut maxPos: *mut libc::c_int,
) -> libc::c_int {
    let mut exprOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    if (*op).op as libc::c_uint != XPATH_OP_PREDICATE as libc::c_int as libc::c_uint
        && (*op).op as libc::c_uint != XPATH_OP_FILTER as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*op).ch2 != -(1 as libc::c_int) {
        exprOp = &mut *((*(*ctxt).comp).steps).offset((*op).ch2 as isize)
            as *mut xmlXPathStepOp;
    } else {
        return 0 as libc::c_int
    }
    if !exprOp.is_null()
        && (*exprOp).op as libc::c_uint == XPATH_OP_VALUE as libc::c_int as libc::c_uint
        && !((*exprOp).value4).is_null()
        && (*((*exprOp).value4 as xmlXPathObjectPtr)).type_0 as libc::c_uint
            == XPATH_NUMBER as libc::c_int as libc::c_uint
    {
        let mut floatval: libc::c_double = (*((*exprOp).value4 as xmlXPathObjectPtr))
            .floatval;
        if floatval > (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
            && floatval < 2147483647 as libc::c_int as libc::c_double
        {
            *maxPos = floatval as libc::c_int;
            if floatval == *maxPos as libc::c_double {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXPathNodeCollectAndTest(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
    mut last: *mut xmlNodePtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut axis: xmlXPathAxisVal = (*op).value as xmlXPathAxisVal;
    let mut test: xmlXPathTestVal = (*op).value2 as xmlXPathTestVal;
    let mut type_0: xmlXPathTypeVal = (*op).value3 as xmlXPathTypeVal;
    let mut prefix: *const xmlChar = (*op).value4 as *const xmlChar;
    let mut name: *const xmlChar = (*op).value5 as *const xmlChar;
    let mut URI: *const xmlChar = 0 as *const xmlChar;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut hasNsNodes: libc::c_int = 0 as libc::c_int;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut contextSeq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut contextIdx: libc::c_int = 0;
    let mut contextNode: xmlNodePtr = 0 as *mut xmlNode;
    let mut outSeq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut seq: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut predOp: xmlXPathStepOpPtr = 0 as *mut xmlXPathStepOp;
    let mut maxPos: libc::c_int = 0;
    let mut hasPredicateRange: libc::c_int = 0;
    let mut hasAxisRange: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut newSize: libc::c_int = 0;
    let mut breakOnFirstHit: libc::c_int = 0;
    let mut next: xmlXPathTraversalFunction = None;
    let mut addNode: Option::<
        unsafe extern "C" fn(xmlNodeSetPtr, xmlNodePtr) -> libc::c_int,
    > = None;
    let mut mergeAndClear: xmlXPathNodeSetMergeFunction = None;
    let mut oldContextNode: xmlNodePtr = 0 as *mut xmlNode;
    let mut xpctxt: xmlXPathContextPtr = (*ctxt).context;
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return 0 as libc::c_int;
    }
    obj = valuePop(ctxt);
    if !prefix.is_null() {
        URI = xmlXPathNsLookup(xpctxt, prefix);
        if URI.is_null() {
            xmlXPathReleaseObject(xpctxt, obj);
            xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as libc::c_int);
            return 0 as libc::c_int;
        }
    }
    mergeAndClear = Some(
        xmlXPathNodeSetMergeAndClear
            as unsafe extern "C" fn(
                xmlNodeSetPtr,
                xmlNodeSetPtr,
                libc::c_int,
            ) -> xmlNodeSetPtr,
    );
    match axis as libc::c_uint {
        1 => {
            first = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextAncestor
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        2 => {
            first = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextAncestorOrSelf
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        3 => {
            first = 0 as *mut xmlNodePtr;
            last = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextAttribute
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(
                        xmlNodeSetPtr,
                        xmlNodeSetPtr,
                        libc::c_int,
                    ) -> xmlNodeSetPtr,
            );
        }
        4 => {
            last = 0 as *mut xmlNodePtr;
            if (test as libc::c_uint == NODE_TEST_NAME as libc::c_int as libc::c_uint
                || test as libc::c_uint == NODE_TEST_ALL as libc::c_int as libc::c_uint)
                && type_0 as libc::c_uint
                    == NODE_TYPE_NODE as libc::c_int as libc::c_uint
            {
                next = Some(
                    xmlXPathNextChildElement
                        as unsafe extern "C" fn(
                            xmlXPathParserContextPtr,
                            xmlNodePtr,
                        ) -> xmlNodePtr,
                );
            } else {
                next = Some(
                    xmlXPathNextChild
                        as unsafe extern "C" fn(
                            xmlXPathParserContextPtr,
                            xmlNodePtr,
                        ) -> xmlNodePtr,
                );
            }
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(
                        xmlNodeSetPtr,
                        xmlNodeSetPtr,
                        libc::c_int,
                    ) -> xmlNodeSetPtr,
            );
        }
        5 => {
            last = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextDescendant
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        6 => {
            last = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextDescendantOrSelf
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        7 => {
            last = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextFollowing
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        8 => {
            last = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextFollowingSibling
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        9 => {
            first = 0 as *mut xmlNodePtr;
            last = 0 as *mut xmlNodePtr;
            next = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
                >,
                xmlXPathTraversalFunction,
            >(
                Some(
                    xmlXPathNextNamespace
                        as unsafe extern "C" fn(
                            xmlXPathParserContextPtr,
                            xmlNodePtr,
                        ) -> xmlNodePtr,
                ),
            );
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(
                        xmlNodeSetPtr,
                        xmlNodeSetPtr,
                        libc::c_int,
                    ) -> xmlNodeSetPtr,
            );
        }
        10 => {
            first = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextParent
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        11 => {
            first = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextPrecedingInternal
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        12 => {
            first = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextPrecedingSibling
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
        }
        13 => {
            first = 0 as *mut xmlNodePtr;
            last = 0 as *mut xmlNodePtr;
            next = Some(
                xmlXPathNextSelf
                    as unsafe extern "C" fn(
                        xmlXPathParserContextPtr,
                        xmlNodePtr,
                    ) -> xmlNodePtr,
            );
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(
                        xmlNodeSetPtr,
                        xmlNodeSetPtr,
                        libc::c_int,
                    ) -> xmlNodeSetPtr,
            );
        }
        _ => {}
    }
    if next.is_none() {
        xmlXPathReleaseObject(xpctxt, obj);
        return 0 as libc::c_int;
    }
    contextSeq = (*obj).nodesetval;
    if contextSeq.is_null() || (*contextSeq).nodeNr <= 0 as libc::c_int {
        xmlXPathReleaseObject(xpctxt, obj);
        valuePush(ctxt, xmlXPathCacheWrapNodeSet(xpctxt, 0 as xmlNodeSetPtr));
        return 0 as libc::c_int;
    }
    maxPos = 0 as libc::c_int;
    predOp = 0 as xmlXPathStepOpPtr;
    hasPredicateRange = 0 as libc::c_int;
    hasAxisRange = 0 as libc::c_int;
    if (*op).ch2 != -(1 as libc::c_int) {
        predOp = &mut *((*(*ctxt).comp).steps).offset((*op).ch2 as isize)
            as *mut xmlXPathStepOp;
        if xmlXPathIsPositionalPredicate(ctxt, predOp, &mut maxPos) != 0 {
            if (*predOp).ch1 != -(1 as libc::c_int) {
                predOp = &mut *((*(*ctxt).comp).steps).offset((*predOp).ch1 as isize)
                    as *mut xmlXPathStepOp;
                hasPredicateRange = 1 as libc::c_int;
            } else {
                predOp = 0 as xmlXPathStepOpPtr;
                hasAxisRange = 1 as libc::c_int;
            }
        }
    }
    breakOnFirstHit = if toBool != 0 && predOp.is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    oldContextNode = (*xpctxt).node;
    addNode = Some(
        xmlXPathNodeSetAddUnique
            as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodePtr) -> libc::c_int,
    );
    outSeq = 0 as xmlNodeSetPtr;
    seq = 0 as xmlNodeSetPtr;
    contextNode = 0 as xmlNodePtr;
    contextIdx = 0 as libc::c_int;
    's_386: while (contextIdx < (*contextSeq).nodeNr || !contextNode.is_null())
        && (*ctxt).error == XPATH_EXPRESSION_OK as libc::c_int
    {
        let fresh78 = contextIdx;
        contextIdx = contextIdx + 1;
        (*xpctxt).node = *((*contextSeq).nodeTab).offset(fresh78 as isize);
        if seq.is_null() {
            seq = xmlXPathNodeSetCreate(0 as xmlNodePtr);
            if seq.is_null() {
                total = 0 as libc::c_int;
                break;
            }
        }
        pos = 0 as libc::c_int;
        cur = 0 as xmlNodePtr;
        hasNsNodes = 0 as libc::c_int;
        loop {
            cur = next.expect("non-null function pointer")(ctxt, cur);
            if cur.is_null() {
                current_block = 12539446900417709254;
                break;
            }
            if !first.is_null() && !(*first).is_null() {
                if *first == cur {
                    current_block = 12539446900417709254;
                    break;
                }
                if total % 256 as libc::c_int == 0 as libc::c_int
                    && xmlXPathCmpNodesExt(*first, cur) >= 0 as libc::c_int
                {
                    current_block = 12539446900417709254;
                    break;
                }
            }
            if !last.is_null() && !(*last).is_null() {
                if *last == cur {
                    current_block = 12539446900417709254;
                    break;
                }
                if total % 256 as libc::c_int == 0 as libc::c_int
                    && xmlXPathCmpNodesExt(cur, *last) >= 0 as libc::c_int
                {
                    current_block = 12539446900417709254;
                    break;
                }
            }
            total += 1;
            total;
            match test as libc::c_uint {
                0 => {
                    total = 0 as libc::c_int;
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Internal error at %s:%d\n\0" as *const u8
                            as *const libc::c_char,
                        b"xpath.c\0" as *const u8 as *const libc::c_char,
                        12361 as libc::c_int,
                    );
                    break 's_386;
                }
                1 => {
                    if type_0 as libc::c_uint
                        == NODE_TYPE_NODE as libc::c_int as libc::c_uint
                    {
                        match (*cur).type_0 as libc::c_uint {
                            9 | 13 | 21 | 1 | 2 | 7 | 8 | 4 | 3 => {
                                current_block = 9632556986220822630;
                                match current_block {
                                    9632556986220822630 => {
                                        if hasAxisRange != 0 as libc::c_int {
                                            pos += 1;
                                            if pos == maxPos {
                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                    < 0 as libc::c_int
                                                {
                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                }
                                                current_block = 8338618956516362613;
                                                break;
                                            }
                                        } else {
                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                < 0 as libc::c_int
                                            {
                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 18026577647358463527;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as libc::c_uint
                                            == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                                        {
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as libc::c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                    }
                                                    current_block = 8338618956516362613;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1 as libc::c_int;
                                                if xmlXPathNodeSetAddNs(
                                                    seq,
                                                    (*xpctxt).node,
                                                    cur as xmlNsPtr,
                                                ) < 0 as libc::c_int
                                                {
                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 18026577647358463527;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1 as libc::c_int;
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if addNode.expect("non-null function pointer")(seq, cur)
                                                        < 0 as libc::c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                    }
                                                    current_block = 8338618956516362613;
                                                    break;
                                                }
                                            } else {
                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                    < 0 as libc::c_int
                                                {
                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 18026577647358463527;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            18 => {
                                current_block = 9775647934248138666;
                                match current_block {
                                    9632556986220822630 => {
                                        if hasAxisRange != 0 as libc::c_int {
                                            pos += 1;
                                            if pos == maxPos {
                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                    < 0 as libc::c_int
                                                {
                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                }
                                                current_block = 8338618956516362613;
                                                break;
                                            }
                                        } else {
                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                < 0 as libc::c_int
                                            {
                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 18026577647358463527;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as libc::c_uint
                                            == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                                        {
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1 as libc::c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as libc::c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                    }
                                                    current_block = 8338618956516362613;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1 as libc::c_int;
                                                if xmlXPathNodeSetAddNs(
                                                    seq,
                                                    (*xpctxt).node,
                                                    cur as xmlNsPtr,
                                                ) < 0 as libc::c_int
                                                {
                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 18026577647358463527;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1 as libc::c_int;
                                            if hasAxisRange != 0 as libc::c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if addNode.expect("non-null function pointer")(seq, cur)
                                                        < 0 as libc::c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                    }
                                                    current_block = 8338618956516362613;
                                                    break;
                                                }
                                            } else {
                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                    < 0 as libc::c_int
                                                {
                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 18026577647358463527;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else if (*cur).type_0 as libc::c_uint
                        == type_0 as xmlElementType as libc::c_uint
                    {
                        if (*cur).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1 as libc::c_int;
                                    if xmlXPathNodeSetAddNs(
                                        seq,
                                        (*xpctxt).node,
                                        cur as xmlNsPtr,
                                    ) < 0 as libc::c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                    }
                                    current_block = 8338618956516362613;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1 as libc::c_int;
                                if xmlXPathNodeSetAddNs(
                                    seq,
                                    (*xpctxt).node,
                                    cur as xmlNsPtr,
                                ) < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 18026577647358463527;
                                    break;
                                }
                            }
                        } else if hasAxisRange != 0 as libc::c_int {
                            pos += 1;
                            if pos == maxPos {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                current_block = 8338618956516362613;
                                break;
                            }
                        } else {
                            if addNode.expect("non-null function pointer")(seq, cur)
                                < 0 as libc::c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 18026577647358463527;
                                break;
                            }
                        }
                    } else if type_0 as libc::c_uint
                        == NODE_TYPE_TEXT as libc::c_int as libc::c_uint
                        && (*cur).type_0 as libc::c_uint
                            == XML_CDATA_SECTION_NODE as libc::c_int as libc::c_uint
                    {
                        if hasAxisRange != 0 as libc::c_int {
                            pos += 1;
                            if pos == maxPos {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                current_block = 8338618956516362613;
                                break;
                            }
                        } else {
                            if addNode.expect("non-null function pointer")(seq, cur)
                                < 0 as libc::c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 18026577647358463527;
                                break;
                            }
                        }
                    }
                }
                2 => {
                    if (*cur).type_0 as libc::c_uint
                        == XML_PI_NODE as libc::c_int as libc::c_uint
                        && (name.is_null() || xmlStrEqual(name, (*cur).name) != 0)
                    {
                        if hasAxisRange != 0 as libc::c_int {
                            pos += 1;
                            if pos == maxPos {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                current_block = 8338618956516362613;
                                break;
                            }
                        } else {
                            if addNode.expect("non-null function pointer")(seq, cur)
                                < 0 as libc::c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 18026577647358463527;
                                break;
                            }
                        }
                    }
                }
                3 => {
                    if axis as libc::c_uint
                        == AXIS_ATTRIBUTE as libc::c_int as libc::c_uint
                    {
                        if (*cur).type_0 as libc::c_uint
                            == XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                        {
                            if prefix.is_null() {
                                if hasAxisRange != 0 as libc::c_int {
                                    pos += 1;
                                    if pos == maxPos {
                                        if addNode.expect("non-null function pointer")(seq, cur)
                                            < 0 as libc::c_int
                                        {
                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                        }
                                        current_block = 8338618956516362613;
                                        break;
                                    }
                                } else {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as libc::c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 18026577647358463527;
                                        break;
                                    }
                                }
                            } else if !((*cur).ns).is_null()
                                && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                            {
                                if hasAxisRange != 0 as libc::c_int {
                                    pos += 1;
                                    if pos == maxPos {
                                        if addNode.expect("non-null function pointer")(seq, cur)
                                            < 0 as libc::c_int
                                        {
                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                        }
                                        current_block = 8338618956516362613;
                                        break;
                                    }
                                } else {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as libc::c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 18026577647358463527;
                                        break;
                                    }
                                }
                            }
                        }
                    } else if axis as libc::c_uint
                        == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                    {
                        if (*cur).type_0 as libc::c_uint
                            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1 as libc::c_int;
                                    if xmlXPathNodeSetAddNs(
                                        seq,
                                        (*xpctxt).node,
                                        cur as xmlNsPtr,
                                    ) < 0 as libc::c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                    }
                                    current_block = 8338618956516362613;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1 as libc::c_int;
                                if xmlXPathNodeSetAddNs(
                                    seq,
                                    (*xpctxt).node,
                                    cur as xmlNsPtr,
                                ) < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 18026577647358463527;
                                    break;
                                }
                            }
                        }
                    } else if (*cur).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        if prefix.is_null() {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as libc::c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                    }
                                    current_block = 8338618956516362613;
                                    break;
                                }
                            } else {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 18026577647358463527;
                                    break;
                                }
                            }
                        } else if !((*cur).ns).is_null()
                            && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                        {
                            if hasAxisRange != 0 as libc::c_int {
                                pos += 1;
                                if pos == maxPos {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as libc::c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                    }
                                    current_block = 8338618956516362613;
                                    break;
                                }
                            } else {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as libc::c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 18026577647358463527;
                                    break;
                                }
                            }
                        }
                    }
                }
                4 => {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\0" as *const u8
                            as *const libc::c_char,
                        b"xpath.c\0" as *const u8 as *const libc::c_char,
                        12442 as libc::c_int,
                    );
                }
                5 => {
                    if axis as libc::c_uint
                        == AXIS_ATTRIBUTE as libc::c_int as libc::c_uint
                    {
                        if (*cur).type_0 as libc::c_uint
                            != XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                        {
                            current_block = 6838274324784804404;
                        } else {
                            current_block = 7409542616634851420;
                        }
                    } else if axis as libc::c_uint
                        == AXIS_NAMESPACE as libc::c_int as libc::c_uint
                    {
                        if (*cur).type_0 as libc::c_uint
                            != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            current_block = 6838274324784804404;
                        } else {
                            current_block = 7409542616634851420;
                        }
                    } else if (*cur).type_0 as libc::c_uint
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        current_block = 6838274324784804404;
                    } else {
                        current_block = 7409542616634851420;
                    }
                    match current_block {
                        6838274324784804404 => {}
                        _ => {
                            match (*cur).type_0 as libc::c_uint {
                                1 => {
                                    current_block = 799515476179724207;
                                    match current_block {
                                        799515476179724207 => {
                                            if xmlStrEqual(name, (*cur).name) != 0 {
                                                if prefix.is_null() {
                                                    if ((*cur).ns).is_null() {
                                                        if hasAxisRange != 0 as libc::c_int {
                                                            pos += 1;
                                                            if pos == maxPos {
                                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                                    < 0 as libc::c_int
                                                                {
                                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                                }
                                                                current_block = 8338618956516362613;
                                                                break;
                                                            }
                                                        } else {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            if breakOnFirstHit != 0 {
                                                                current_block = 18026577647358463527;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else if !((*cur).ns).is_null()
                                                    && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode.expect("non-null function pointer")(seq, cur)
                                                            < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        10632679619850208969 => {
                                            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                            if xmlStrEqual(name, (*attr).name) != 0 {
                                                if prefix.is_null() {
                                                    if ((*attr).ns).is_null()
                                                        || ((*(*attr).ns).prefix).is_null()
                                                    {
                                                        if hasAxisRange != 0 as libc::c_int {
                                                            pos += 1;
                                                            if pos == maxPos {
                                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                                    < 0 as libc::c_int
                                                                {
                                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                                }
                                                                current_block = 8338618956516362613;
                                                                break;
                                                            }
                                                        } else {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            if breakOnFirstHit != 0 {
                                                                current_block = 18026577647358463527;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else if !((*attr).ns).is_null()
                                                    && xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode.expect("non-null function pointer")(seq, cur)
                                                            < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            if (*cur).type_0 as libc::c_uint
                                                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                                            {
                                                let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                                if !((*ns).prefix).is_null() && !name.is_null()
                                                    && xmlStrEqual((*ns).prefix, name) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            hasNsNodes = 1 as libc::c_int;
                                                            if xmlXPathNodeSetAddNs(
                                                                seq,
                                                                (*xpctxt).node,
                                                                cur as xmlNsPtr,
                                                            ) < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        hasNsNodes = 1 as libc::c_int;
                                                        if xmlXPathNodeSetAddNs(
                                                            seq,
                                                            (*xpctxt).node,
                                                            cur as xmlNsPtr,
                                                        ) < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                2 => {
                                    current_block = 10632679619850208969;
                                    match current_block {
                                        799515476179724207 => {
                                            if xmlStrEqual(name, (*cur).name) != 0 {
                                                if prefix.is_null() {
                                                    if ((*cur).ns).is_null() {
                                                        if hasAxisRange != 0 as libc::c_int {
                                                            pos += 1;
                                                            if pos == maxPos {
                                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                                    < 0 as libc::c_int
                                                                {
                                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                                }
                                                                current_block = 8338618956516362613;
                                                                break;
                                                            }
                                                        } else {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            if breakOnFirstHit != 0 {
                                                                current_block = 18026577647358463527;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else if !((*cur).ns).is_null()
                                                    && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode.expect("non-null function pointer")(seq, cur)
                                                            < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        10632679619850208969 => {
                                            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                            if xmlStrEqual(name, (*attr).name) != 0 {
                                                if prefix.is_null() {
                                                    if ((*attr).ns).is_null()
                                                        || ((*(*attr).ns).prefix).is_null()
                                                    {
                                                        if hasAxisRange != 0 as libc::c_int {
                                                            pos += 1;
                                                            if pos == maxPos {
                                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                                    < 0 as libc::c_int
                                                                {
                                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                                }
                                                                current_block = 8338618956516362613;
                                                                break;
                                                            }
                                                        } else {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            if breakOnFirstHit != 0 {
                                                                current_block = 18026577647358463527;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else if !((*attr).ns).is_null()
                                                    && xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode.expect("non-null function pointer")(seq, cur)
                                                            < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            if (*cur).type_0 as libc::c_uint
                                                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                                            {
                                                let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                                if !((*ns).prefix).is_null() && !name.is_null()
                                                    && xmlStrEqual((*ns).prefix, name) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            hasNsNodes = 1 as libc::c_int;
                                                            if xmlXPathNodeSetAddNs(
                                                                seq,
                                                                (*xpctxt).node,
                                                                cur as xmlNsPtr,
                                                            ) < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        hasNsNodes = 1 as libc::c_int;
                                                        if xmlXPathNodeSetAddNs(
                                                            seq,
                                                            (*xpctxt).node,
                                                            cur as xmlNsPtr,
                                                        ) < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                18 => {
                                    current_block = 15778523166845625939;
                                    match current_block {
                                        799515476179724207 => {
                                            if xmlStrEqual(name, (*cur).name) != 0 {
                                                if prefix.is_null() {
                                                    if ((*cur).ns).is_null() {
                                                        if hasAxisRange != 0 as libc::c_int {
                                                            pos += 1;
                                                            if pos == maxPos {
                                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                                    < 0 as libc::c_int
                                                                {
                                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                                }
                                                                current_block = 8338618956516362613;
                                                                break;
                                                            }
                                                        } else {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            if breakOnFirstHit != 0 {
                                                                current_block = 18026577647358463527;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else if !((*cur).ns).is_null()
                                                    && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode.expect("non-null function pointer")(seq, cur)
                                                            < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        10632679619850208969 => {
                                            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                            if xmlStrEqual(name, (*attr).name) != 0 {
                                                if prefix.is_null() {
                                                    if ((*attr).ns).is_null()
                                                        || ((*(*attr).ns).prefix).is_null()
                                                    {
                                                        if hasAxisRange != 0 as libc::c_int {
                                                            pos += 1;
                                                            if pos == maxPos {
                                                                if addNode.expect("non-null function pointer")(seq, cur)
                                                                    < 0 as libc::c_int
                                                                {
                                                                    (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                                }
                                                                current_block = 8338618956516362613;
                                                                break;
                                                            }
                                                        } else {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            if breakOnFirstHit != 0 {
                                                                current_block = 18026577647358463527;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                } else if !((*attr).ns).is_null()
                                                    && xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                                < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode.expect("non-null function pointer")(seq, cur)
                                                            < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            if (*cur).type_0 as libc::c_uint
                                                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                                            {
                                                let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                                if !((*ns).prefix).is_null() && !name.is_null()
                                                    && xmlStrEqual((*ns).prefix, name) != 0
                                                {
                                                    if hasAxisRange != 0 as libc::c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            hasNsNodes = 1 as libc::c_int;
                                                            if xmlXPathNodeSetAddNs(
                                                                seq,
                                                                (*xpctxt).node,
                                                                cur as xmlNsPtr,
                                                            ) < 0 as libc::c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                            }
                                                            current_block = 8338618956516362613;
                                                            break;
                                                        }
                                                    } else {
                                                        hasNsNodes = 1 as libc::c_int;
                                                        if xmlXPathNodeSetAddNs(
                                                            seq,
                                                            (*xpctxt).node,
                                                            cur as xmlNsPtr,
                                                        ) < 0 as libc::c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 18026577647358463527;
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            if !(!cur.is_null() && (*ctxt).error == XPATH_EXPRESSION_OK as libc::c_int) {
                current_block = 12539446900417709254;
                break;
            }
        }
        match current_block {
            18026577647358463527 => {
                if outSeq.is_null() {
                    outSeq = seq;
                    seq = 0 as xmlNodeSetPtr;
                } else {
                    outSeq = mergeAndClear
                        .expect(
                            "non-null function pointer",
                        )(outSeq, seq, 0 as libc::c_int);
                }
                break;
            }
            12539446900417709254 => {
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    break;
                }
                if !predOp.is_null() && (*seq).nodeNr > 0 as libc::c_int {
                    size = (*seq).nodeNr;
                    if hasPredicateRange != 0 as libc::c_int {
                        newSize = xmlXPathCompOpEvalPositionalPredicate(
                            ctxt,
                            predOp,
                            seq,
                            size,
                            maxPos,
                            maxPos,
                            hasNsNodes,
                        );
                    } else {
                        newSize = xmlXPathCompOpEvalPredicate(
                            ctxt,
                            predOp,
                            seq,
                            size,
                            hasNsNodes,
                        );
                    }
                    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                        total = 0 as libc::c_int;
                        break;
                    } else if newSize == 0 as libc::c_int {
                        xmlXPathNodeSetClear(seq, hasNsNodes);
                    } else {
                        if !((*seq).nodeNr > 0 as libc::c_int) {
                            continue;
                        }
                        if outSeq.is_null() {
                            if size != newSize {
                                outSeq = mergeAndClear
                                    .expect(
                                        "non-null function pointer",
                                    )(0 as xmlNodeSetPtr, seq, 1 as libc::c_int);
                            } else {
                                outSeq = seq;
                                seq = 0 as xmlNodeSetPtr;
                            }
                        } else {
                            outSeq = mergeAndClear
                                .expect(
                                    "non-null function pointer",
                                )(
                                outSeq,
                                seq,
                                if size != newSize {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                },
                            );
                        }
                        if toBool != 0 {
                            break;
                        }
                    }
                } else if (*seq).nodeNr > 0 as libc::c_int {
                    if outSeq.is_null() {
                        outSeq = seq;
                        seq = 0 as xmlNodeSetPtr;
                    } else {
                        outSeq = mergeAndClear
                            .expect(
                                "non-null function pointer",
                            )(outSeq, seq, 0 as libc::c_int);
                    }
                }
            }
            _ => {
                if outSeq.is_null() {
                    outSeq = seq;
                    seq = 0 as xmlNodeSetPtr;
                } else {
                    outSeq = mergeAndClear
                        .expect(
                            "non-null function pointer",
                        )(outSeq, seq, 0 as libc::c_int);
                }
                if toBool != 0 {
                    break;
                }
            }
        }
    }
    if (*obj).boolval != 0 && !((*obj).user).is_null() {
        (*(*ctxt).value).boolval = 1 as libc::c_int;
        (*(*ctxt).value).user = (*obj).user;
        (*obj).user = 0 as *mut libc::c_void;
        (*obj).boolval = 0 as libc::c_int;
    }
    xmlXPathReleaseObject(xpctxt, obj);
    if outSeq.is_null() {
        if !seq.is_null() && (*seq).nodeNr == 0 as libc::c_int {
            outSeq = seq;
        } else {
            outSeq = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        }
    }
    if !seq.is_null() && seq != outSeq {
        xmlXPathFreeNodeSet(seq);
    }
    valuePush(ctxt, xmlXPathCacheWrapNodeSet(xpctxt, outSeq));
    (*xpctxt).node = oldContextNode;
    if !((*xpctxt).tmpNsList).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*xpctxt).tmpNsList as *mut libc::c_void);
        (*xpctxt).tmpNsList = 0 as *mut xmlNsPtr;
    }
    return total;
}
unsafe extern "C" fn xmlXPathCompOpEvalFirst(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    comp = (*ctxt).comp;
    match (*op).op as libc::c_uint {
        0 => return 0 as libc::c_int,
        7 => {
            total = xmlXPathCompOpEvalFirst(
                ctxt,
                &mut *((*comp).steps).offset((*op).ch1 as isize),
                first,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr >= 1 as libc::c_int
            {
                if (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int {
                    xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
                }
                *first = *((*(*(*ctxt).value).nodesetval).nodeTab)
                    .offset(0 as libc::c_int as isize);
            }
            cur = xmlXPathCompOpEvalFirst(
                ctxt,
                &mut *((*comp).steps).offset((*op).ch2 as isize),
                first,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            arg2 = valuePop(ctxt);
            arg1 = valuePop(ctxt);
            if arg1.is_null()
                || (*arg1).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint || arg2.is_null()
                || (*arg2).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                return 0 as libc::c_int;
            }
            (*arg1)
                .nodesetval = xmlXPathNodeSetMerge(
                (*arg1).nodesetval,
                (*arg2).nodesetval,
            );
            valuePush(ctxt, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            if total > cur {
                xmlXPathCompSwap(op);
            }
            return total + cur;
        }
        8 => {
            xmlXPathRoot(ctxt);
            return 0 as libc::c_int;
        }
        9 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
            );
            return total;
        }
        10 => {
            if (*op).ch1 == -(1 as libc::c_int) {
                return total;
            }
            total = xmlXPathCompOpEval(
                ctxt,
                &mut *((*comp).steps).offset((*op).ch1 as isize),
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    first,
                    0 as *mut xmlNodePtr,
                    0 as libc::c_int,
                );
            return total;
        }
        11 => {
            valuePush(
                ctxt,
                xmlXPathCacheObjectCopy(
                    (*ctxt).context,
                    (*op).value4 as xmlXPathObjectPtr,
                ),
            );
            return 0 as libc::c_int;
        }
        17 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEvalFirst(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                        first,
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int
            {
                xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
            }
            return total;
        }
        16 => {
            total += xmlXPathCompOpEvalFilterFirst(ctxt, op, first);
            return total;
        }
        _ => return xmlXPathCompOpEval(ctxt, op),
    };
}
unsafe extern "C" fn xmlXPathCompOpEvalLast(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut last: *mut xmlNodePtr,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut cur: libc::c_int = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    comp = (*ctxt).comp;
    match (*op).op as libc::c_uint {
        0 => return 0 as libc::c_int,
        7 => {
            total = xmlXPathCompOpEvalLast(
                ctxt,
                &mut *((*comp).steps).offset((*op).ch1 as isize),
                last,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr >= 1 as libc::c_int
            {
                if (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int {
                    xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
                }
                *last = *((*(*(*ctxt).value).nodesetval).nodeTab)
                    .offset(
                        ((*(*(*ctxt).value).nodesetval).nodeNr - 1 as libc::c_int)
                            as isize,
                    );
            }
            cur = xmlXPathCompOpEvalLast(
                ctxt,
                &mut *((*comp).steps).offset((*op).ch2 as isize),
                last,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr >= 1 as libc::c_int;
            arg2 = valuePop(ctxt);
            arg1 = valuePop(ctxt);
            if arg1.is_null()
                || (*arg1).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint || arg2.is_null()
                || (*arg2).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                return 0 as libc::c_int;
            }
            (*arg1)
                .nodesetval = xmlXPathNodeSetMerge(
                (*arg1).nodesetval,
                (*arg2).nodesetval,
            );
            valuePush(ctxt, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            if total > cur {
                xmlXPathCompSwap(op);
            }
            return total + cur;
        }
        8 => {
            xmlXPathRoot(ctxt);
            return 0 as libc::c_int;
        }
        9 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
            );
            return total;
        }
        10 => {
            if (*op).ch1 == -(1 as libc::c_int) {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    0 as *mut xmlNodePtr,
                    last,
                    0 as libc::c_int,
                );
            return total;
        }
        11 => {
            valuePush(
                ctxt,
                xmlXPathCacheObjectCopy(
                    (*ctxt).context,
                    (*op).value4 as xmlXPathObjectPtr,
                ),
            );
            return 0 as libc::c_int;
        }
        17 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEvalLast(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                        last,
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int
            {
                xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
            }
            return total;
        }
        _ => return xmlXPathCompOpEval(ctxt, op),
    };
}
unsafe extern "C" fn xmlXPathCompOpEvalFilterFirst(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
) -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut oldset: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut oldnode: xmlNodePtr = 0 as *mut xmlNode;
    let mut oldDoc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut oldcs: libc::c_int = 0;
    let mut oldpp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    comp = (*ctxt).comp;
    if (*op).ch1 != -(1 as libc::c_int) && (*op).ch2 != -(1 as libc::c_int)
        && (*((*comp).steps).offset((*op).ch1 as isize)).op as libc::c_uint
            == XPATH_OP_SORT as libc::c_int as libc::c_uint
        && (*((*comp).steps).offset((*op).ch2 as isize)).op as libc::c_uint
            == XPATH_OP_SORT as libc::c_int as libc::c_uint
    {
        let mut f: libc::c_int = (*((*comp).steps).offset((*op).ch2 as isize)).ch1;
        if f != -(1 as libc::c_int)
            && (*((*comp).steps).offset(f as isize)).op as libc::c_uint
                == XPATH_OP_FUNCTION as libc::c_int as libc::c_uint
            && ((*((*comp).steps).offset(f as isize)).value5).is_null()
            && (*((*comp).steps).offset(f as isize)).value == 0 as libc::c_int
            && !((*((*comp).steps).offset(f as isize)).value4).is_null()
            && xmlStrEqual(
                (*((*comp).steps).offset(f as isize)).value4 as *const xmlChar,
                b"last\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
        {
            let mut last: xmlNodePtr = 0 as xmlNodePtr;
            total
                += xmlXPathCompOpEvalLast(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                    &mut last,
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && !((*(*(*ctxt).value).nodesetval).nodeTab).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int
            {
                xmlXPathNodeSetKeepLast((*(*ctxt).value).nodesetval);
                *first = *(*(*(*ctxt).value).nodesetval).nodeTab;
            }
            return total;
        }
    }
    if (*op).ch1 != -(1 as libc::c_int) {
        total
            += xmlXPathCompOpEval(
                ctxt,
                &mut *((*comp).steps).offset((*op).ch1 as isize),
            );
    }
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*op).ch2 == -(1 as libc::c_int) {
        return total;
    }
    if ((*ctxt).value).is_null() {
        return total;
    }
    if (*(*ctxt).value).type_0 as libc::c_uint
        == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
    {
        let mut current_block_72: u64;
        let mut tmp: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
        let mut newlocset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
        let mut oldlocset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
        if ((*ctxt).value).is_null()
            || (*(*ctxt).value).type_0 as libc::c_uint
                != XPATH_LOCATIONSET as libc::c_int as libc::c_uint
        {
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
            return 0 as libc::c_int;
        }
        if ((*(*ctxt).value).user).is_null()
            || (*((*(*ctxt).value).user as xmlLocationSetPtr)).locNr == 0 as libc::c_int
        {
            return total;
        }
        obj = valuePop(ctxt);
        oldlocset = (*obj).user as xmlLocationSetPtr;
        oldnode = (*(*ctxt).context).node;
        oldcs = (*(*ctxt).context).contextSize;
        oldpp = (*(*ctxt).context).proximityPosition;
        newlocset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
        i = 0 as libc::c_int;
        loop {
            if !(i < (*oldlocset).locNr) {
                current_block_72 = 10380409671385728102;
                break;
            }
            (*(*ctxt).context)
                .node = (**((*oldlocset).locTab).offset(i as isize)).user as xmlNodePtr;
            (*(*ctxt).context).contextSize = (*oldlocset).locNr;
            (*(*ctxt).context).proximityPosition = i + 1 as libc::c_int;
            if tmp.is_null() {
                tmp = xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node);
            } else if xmlXPathNodeSetAddUnique(
                (*tmp).nodesetval,
                (*(*ctxt).context).node,
            ) < 0 as libc::c_int
            {
                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
            }
            valuePush(ctxt, tmp);
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                xmlXPtrFreeLocationSet(newlocset);
                current_block_72 = 16177518360893587816;
                break;
            } else {
                res = valuePop(ctxt);
                if xmlXPathEvaluatePredicateResult(ctxt, res) != 0 {
                    xmlXPtrLocationSetAdd(
                        newlocset,
                        xmlXPathCacheObjectCopy(
                            (*ctxt).context,
                            *((*oldlocset).locTab).offset(i as isize),
                        ),
                    );
                }
                if !res.is_null() {
                    xmlXPathReleaseObject((*ctxt).context, res);
                }
                if (*ctxt).value == tmp {
                    valuePop(ctxt);
                    xmlXPathNodeSetClear((*tmp).nodesetval, 1 as libc::c_int);
                } else {
                    tmp = 0 as xmlXPathObjectPtr;
                }
                if (*newlocset).locNr > 0 as libc::c_int {
                    *first = (**((*oldlocset).locTab).offset(i as isize)).user
                        as xmlNodePtr;
                    current_block_72 = 10380409671385728102;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        match current_block_72 {
            10380409671385728102 => {
                if !tmp.is_null() {
                    xmlXPathReleaseObject((*ctxt).context, tmp);
                }
                valuePush(ctxt, xmlXPtrWrapLocationSet(newlocset));
            }
            _ => {}
        }
        xmlXPathReleaseObject((*ctxt).context, obj);
        (*(*ctxt).context).node = oldnode;
        (*(*ctxt).context).contextSize = oldcs;
        (*(*ctxt).context).proximityPosition = oldpp;
        return total;
    }
    if ((*ctxt).value).is_null()
        || (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_NODESET as libc::c_int as libc::c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
        return 0 as libc::c_int;
    }
    if !((*(*ctxt).value).nodesetval).is_null()
        && (*(*(*ctxt).value).nodesetval).nodeNr != 0 as libc::c_int
    {
        let mut current_block_126: u64;
        let mut newset: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        let mut tmp_0: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
        obj = valuePop(ctxt);
        oldset = (*obj).nodesetval;
        oldnode = (*(*ctxt).context).node;
        oldDoc = (*(*ctxt).context).doc;
        oldcs = (*(*ctxt).context).contextSize;
        oldpp = (*(*ctxt).context).proximityPosition;
        newset = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        i = 0 as libc::c_int;
        loop {
            if !(i < (*oldset).nodeNr) {
                current_block_126 = 796174441944384681;
                break;
            }
            (*(*ctxt).context).node = *((*oldset).nodeTab).offset(i as isize);
            if (**((*oldset).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                && !((**((*oldset).nodeTab).offset(i as isize)).doc).is_null()
            {
                (*(*ctxt).context).doc = (**((*oldset).nodeTab).offset(i as isize)).doc;
            }
            if tmp_0.is_null() {
                tmp_0 = xmlXPathCacheNewNodeSet(
                    (*ctxt).context,
                    (*(*ctxt).context).node,
                );
            } else if xmlXPathNodeSetAddUnique(
                (*tmp_0).nodesetval,
                (*(*ctxt).context).node,
            ) < 0 as libc::c_int
            {
                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
            }
            valuePush(ctxt, tmp_0);
            (*(*ctxt).context).contextSize = (*oldset).nodeNr;
            (*(*ctxt).context).proximityPosition = i + 1 as libc::c_int;
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                xmlXPathFreeNodeSet(newset);
                current_block_126 = 2163030092955728735;
                break;
            } else {
                res = valuePop(ctxt);
                if xmlXPathEvaluatePredicateResult(ctxt, res) != 0 {
                    if xmlXPathNodeSetAdd(
                        newset,
                        *((*oldset).nodeTab).offset(i as isize),
                    ) < 0 as libc::c_int
                    {
                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                    }
                }
                if !res.is_null() {
                    xmlXPathReleaseObject((*ctxt).context, res);
                }
                if (*ctxt).value == tmp_0 {
                    valuePop(ctxt);
                    xmlXPathNodeSetClear((*tmp_0).nodesetval, 1 as libc::c_int);
                } else {
                    tmp_0 = 0 as xmlXPathObjectPtr;
                }
                if (*newset).nodeNr > 0 as libc::c_int {
                    *first = *(*newset).nodeTab;
                    current_block_126 = 796174441944384681;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        match current_block_126 {
            796174441944384681 => {
                if !tmp_0.is_null() {
                    xmlXPathReleaseObject((*ctxt).context, tmp_0);
                }
                valuePush(ctxt, xmlXPathCacheWrapNodeSet((*ctxt).context, newset));
            }
            _ => {}
        }
        xmlXPathReleaseObject((*ctxt).context, obj);
        (*(*ctxt).context).node = oldnode;
        (*(*ctxt).context).doc = oldDoc;
        (*(*ctxt).context).contextSize = oldcs;
        (*(*ctxt).context).proximityPosition = oldpp;
    }
    return total;
}
unsafe extern "C" fn xmlXPathCompOpEval(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
) -> libc::c_int {
    let mut current_block: u64;
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut equal: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut arg1: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut arg2: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        return 0 as libc::c_int;
    }
    comp = (*ctxt).comp;
    match (*op).op as libc::c_uint {
        0 => return 0 as libc::c_int,
        1 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
            if ((*ctxt).value).is_null() || (*(*ctxt).value).boolval == 0 as libc::c_int
            {
                return total;
            }
            arg2 = valuePop(ctxt);
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                );
            if (*ctxt).error != 0 {
                xmlXPathFreeObject(arg2);
                return 0 as libc::c_int;
            }
            xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
            arg1 = valuePop(ctxt);
            (*arg1).boolval &= (*arg2).boolval;
            valuePush(ctxt, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            return total;
        }
        2 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
            if ((*ctxt).value).is_null() || (*(*ctxt).value).boolval == 1 as libc::c_int
            {
                return total;
            }
            arg2 = valuePop(ctxt);
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                );
            if (*ctxt).error != 0 {
                xmlXPathFreeObject(arg2);
                return 0 as libc::c_int;
            }
            xmlXPathBooleanFunction(ctxt, 1 as libc::c_int);
            arg1 = valuePop(ctxt);
            (*arg1).boolval |= (*arg2).boolval;
            valuePush(ctxt, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            return total;
        }
        3 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).value != 0 {
                equal = xmlXPathEqualValues(ctxt);
            } else {
                equal = xmlXPathNotEqualValues(ctxt);
            }
            valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, equal));
            return total;
        }
        4 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            ret = xmlXPathCompareValues(ctxt, (*op).value, (*op).value2);
            valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, ret));
            return total;
        }
        5 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).value == 0 as libc::c_int {
                xmlXPathSubValues(ctxt);
            } else if (*op).value == 1 as libc::c_int {
                xmlXPathAddValues(ctxt);
            } else if (*op).value == 2 as libc::c_int {
                xmlXPathValueFlipSign(ctxt);
            } else if (*op).value == 3 as libc::c_int {
                if !((*ctxt).value).is_null()
                    && (*(*ctxt).value).type_0 as libc::c_uint
                        != XPATH_NUMBER as libc::c_int as libc::c_uint
                {
                    xmlXPathNumberFunction(ctxt, 1 as libc::c_int);
                }
                if ((*ctxt).value).is_null()
                    || (*(*ctxt).value).type_0 as libc::c_uint
                        != XPATH_NUMBER as libc::c_int as libc::c_uint
                {
                    xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                    return 0 as libc::c_int;
                }
            }
            return total;
        }
        6 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).value == 0 as libc::c_int {
                xmlXPathMultValues(ctxt);
            } else if (*op).value == 1 as libc::c_int {
                xmlXPathDivValues(ctxt);
            } else if (*op).value == 2 as libc::c_int {
                xmlXPathModValues(ctxt);
            }
            return total;
        }
        7 => {
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            arg2 = valuePop(ctxt);
            arg1 = valuePop(ctxt);
            if arg1.is_null()
                || (*arg1).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint || arg2.is_null()
                || (*arg2).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                return 0 as libc::c_int;
            }
            if ((*arg1).nodesetval).is_null()
                || !((*arg2).nodesetval).is_null()
                    && (*(*arg2).nodesetval).nodeNr != 0 as libc::c_int
            {
                (*arg1)
                    .nodesetval = xmlXPathNodeSetMerge(
                    (*arg1).nodesetval,
                    (*arg2).nodesetval,
                );
            }
            valuePush(ctxt, arg1);
            xmlXPathReleaseObject((*ctxt).context, arg2);
            return total;
        }
        8 => {
            xmlXPathRoot(ctxt);
            return total;
        }
        9 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
            );
            return total;
        }
        10 => {
            if (*op).ch1 == -(1 as libc::c_int) {
                return total;
            }
            total
                += xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*comp).steps).offset((*op).ch1 as isize),
                );
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            total
                += xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    0 as *mut xmlNodePtr,
                    0 as *mut xmlNodePtr,
                    0 as libc::c_int,
                );
            return total;
        }
        11 => {
            valuePush(
                ctxt,
                xmlXPathCacheObjectCopy(
                    (*ctxt).context,
                    (*op).value4 as xmlXPathObjectPtr,
                ),
            );
            return total;
        }
        12 => {
            let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
            }
            if ((*op).value5).is_null() {
                val = xmlXPathVariableLookup(
                    (*ctxt).context,
                    (*op).value4 as *const xmlChar,
                );
                if val.is_null() {
                    xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as libc::c_int);
                    return 0 as libc::c_int;
                }
                valuePush(ctxt, val);
            } else {
                let mut URI: *const xmlChar = 0 as *const xmlChar;
                URI = xmlXPathNsLookup((*ctxt).context, (*op).value5 as *const xmlChar);
                if URI.is_null() {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathCompOpEval: variable %s bound to undefined prefix %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*op).value4 as *mut libc::c_char,
                        (*op).value5 as *mut libc::c_char,
                    );
                    (*ctxt).error = XPATH_UNDEF_PREFIX_ERROR as libc::c_int;
                    return total;
                }
                val = xmlXPathVariableLookupNS(
                    (*ctxt).context,
                    (*op).value4 as *const xmlChar,
                    URI,
                );
                if val.is_null() {
                    xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as libc::c_int);
                    return 0 as libc::c_int;
                }
                valuePush(ctxt, val);
            }
            return total;
        }
        13 => {
            let mut func: xmlXPathFunction = None;
            let mut oldFunc: *const xmlChar = 0 as *const xmlChar;
            let mut oldFuncURI: *const xmlChar = 0 as *const xmlChar;
            let mut i: libc::c_int = 0;
            let mut frame: libc::c_int = 0;
            frame = xmlXPathSetFrame(ctxt);
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    xmlXPathPopFrame(ctxt, frame);
                    return total;
                }
            }
            if (*ctxt).valueNr < (*ctxt).valueFrame + (*op).value {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathCompOpEval: parameter error\n\0" as *const u8
                        as *const libc::c_char,
                );
                (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
                xmlXPathPopFrame(ctxt, frame);
                return total;
            }
            i = 0 as libc::c_int;
            while i < (*op).value {
                if (*((*ctxt).valueTab)
                    .offset(((*ctxt).valueNr - 1 as libc::c_int - i) as isize))
                    .is_null()
                {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathCompOpEval: parameter error\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
                    xmlXPathPopFrame(ctxt, frame);
                    return total;
                }
                i += 1;
                i;
            }
            if ((*op).cache).is_some() {
                func = (*op).cache;
            } else {
                let mut URI_0: *const xmlChar = 0 as *const xmlChar;
                if ((*op).value5).is_null() {
                    func = xmlXPathFunctionLookup(
                        (*ctxt).context,
                        (*op).value4 as *const xmlChar,
                    );
                } else {
                    URI_0 = xmlXPathNsLookup(
                        (*ctxt).context,
                        (*op).value5 as *const xmlChar,
                    );
                    if URI_0.is_null() {
                        (*__xmlGenericError())
                            .expect(
                                "non-null function pointer",
                            )(
                            *__xmlGenericErrorContext(),
                            b"xmlXPathCompOpEval: function %s bound to undefined prefix %s\n\0"
                                as *const u8 as *const libc::c_char,
                            (*op).value4 as *mut libc::c_char,
                            (*op).value5 as *mut libc::c_char,
                        );
                        xmlXPathPopFrame(ctxt, frame);
                        (*ctxt).error = XPATH_UNDEF_PREFIX_ERROR as libc::c_int;
                        return total;
                    }
                    func = xmlXPathFunctionLookupNS(
                        (*ctxt).context,
                        (*op).value4 as *const xmlChar,
                        URI_0,
                    );
                }
                if func.is_none() {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathCompOpEval: function %s not found\n\0" as *const u8
                            as *const libc::c_char,
                        (*op).value4 as *mut libc::c_char,
                    );
                    xmlXPathErr(ctxt, XPATH_UNKNOWN_FUNC_ERROR as libc::c_int);
                    return 0 as libc::c_int;
                }
                (*op).cache = func;
                (*op).cacheURI = URI_0 as *mut libc::c_void;
            }
            oldFunc = (*(*ctxt).context).function;
            oldFuncURI = (*(*ctxt).context).functionURI;
            (*(*ctxt).context).function = (*op).value4 as *const xmlChar;
            (*(*ctxt).context).functionURI = (*op).cacheURI as *const xmlChar;
            func.expect("non-null function pointer")(ctxt, (*op).value);
            (*(*ctxt).context).function = oldFunc;
            (*(*ctxt).context).functionURI = oldFuncURI;
            xmlXPathPopFrame(ctxt, frame);
            return total;
        }
        14 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            if (*op).ch2 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch2 as isize),
                    );
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            return total;
        }
        15 | 16 => {
            let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut tmp: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut newset: xmlNodeSetPtr = 0 as xmlNodeSetPtr;
            let mut oldset: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
            let mut oldnode: xmlNodePtr = 0 as *mut xmlNode;
            let mut oldDoc: xmlDocPtr = 0 as *mut xmlDoc;
            let mut oldcs: libc::c_int = 0;
            let mut oldpp: libc::c_int = 0;
            let mut i_0: libc::c_int = 0;
            if (*op).ch1 != -(1 as libc::c_int) && (*op).ch2 != -(1 as libc::c_int)
                && ((*((*comp).steps).offset((*op).ch1 as isize)).op as libc::c_uint
                    == XPATH_OP_SORT as libc::c_int as libc::c_uint
                    || (*((*comp).steps).offset((*op).ch1 as isize)).op as libc::c_uint
                        == XPATH_OP_FILTER as libc::c_int as libc::c_uint)
                && (*((*comp).steps).offset((*op).ch2 as isize)).op as libc::c_uint
                    == XPATH_OP_VALUE as libc::c_int as libc::c_uint
            {
                let mut val_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
                val_0 = (*((*comp).steps).offset((*op).ch2 as isize)).value4
                    as xmlXPathObjectPtr;
                if !val_0.is_null()
                    && (*val_0).type_0 as libc::c_uint
                        == XPATH_NUMBER as libc::c_int as libc::c_uint
                    && (*val_0).floatval == 1.0f64
                {
                    let mut first: xmlNodePtr = 0 as xmlNodePtr;
                    total
                        += xmlXPathCompOpEvalFirst(
                            ctxt,
                            &mut *((*comp).steps).offset((*op).ch1 as isize),
                            &mut first,
                        );
                    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    if !((*ctxt).value).is_null()
                        && (*(*ctxt).value).type_0 as libc::c_uint
                            == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !((*(*ctxt).value).nodesetval).is_null()
                        && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int
                    {
                        xmlXPathNodeSetClearFromPos(
                            (*(*ctxt).value).nodesetval,
                            1 as libc::c_int,
                            1 as libc::c_int,
                        );
                    }
                    return total;
                }
            }
            if (*op).ch1 != -(1 as libc::c_int) && (*op).ch2 != -(1 as libc::c_int)
                && (*((*comp).steps).offset((*op).ch1 as isize)).op as libc::c_uint
                    == XPATH_OP_SORT as libc::c_int as libc::c_uint
                && (*((*comp).steps).offset((*op).ch2 as isize)).op as libc::c_uint
                    == XPATH_OP_SORT as libc::c_int as libc::c_uint
            {
                let mut f: libc::c_int = (*((*comp).steps).offset((*op).ch2 as isize))
                    .ch1;
                if f != -(1 as libc::c_int)
                    && (*((*comp).steps).offset(f as isize)).op as libc::c_uint
                        == XPATH_OP_FUNCTION as libc::c_int as libc::c_uint
                    && ((*((*comp).steps).offset(f as isize)).value5).is_null()
                    && (*((*comp).steps).offset(f as isize)).value == 0 as libc::c_int
                    && !((*((*comp).steps).offset(f as isize)).value4).is_null()
                    && xmlStrEqual(
                        (*((*comp).steps).offset(f as isize)).value4 as *const xmlChar,
                        b"last\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    ) != 0
                {
                    let mut last: xmlNodePtr = 0 as xmlNodePtr;
                    total
                        += xmlXPathCompOpEvalLast(
                            ctxt,
                            &mut *((*comp).steps).offset((*op).ch1 as isize),
                            &mut last,
                        );
                    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    if !((*ctxt).value).is_null()
                        && (*(*ctxt).value).type_0 as libc::c_uint
                            == XPATH_NODESET as libc::c_int as libc::c_uint
                        && !((*(*ctxt).value).nodesetval).is_null()
                        && !((*(*(*ctxt).value).nodesetval).nodeTab).is_null()
                        && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int
                    {
                        xmlXPathNodeSetKeepLast((*(*ctxt).value).nodesetval);
                    }
                    return total;
                }
            }
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if (*op).ch2 == -(1 as libc::c_int) {
                return total;
            }
            if ((*ctxt).value).is_null() {
                return total;
            }
            if (*(*ctxt).value).type_0 as libc::c_uint
                == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
            {
                let mut current_block_303: u64;
                let mut newlocset: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
                let mut oldlocset: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
                if ((*ctxt).value).is_null()
                    || (*(*ctxt).value).type_0 as libc::c_uint
                        != XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                {
                    xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                    return 0 as libc::c_int;
                }
                if ((*(*ctxt).value).user).is_null()
                    || (*((*(*ctxt).value).user as xmlLocationSetPtr)).locNr
                        == 0 as libc::c_int
                {
                    return total;
                }
                obj = valuePop(ctxt);
                oldlocset = (*obj).user as xmlLocationSetPtr;
                oldnode = (*(*ctxt).context).node;
                oldcs = (*(*ctxt).context).contextSize;
                oldpp = (*(*ctxt).context).proximityPosition;
                newlocset = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
                i_0 = 0 as libc::c_int;
                loop {
                    if !(i_0 < (*oldlocset).locNr) {
                        current_block_303 = 5159167864558512679;
                        break;
                    }
                    (*(*ctxt).context)
                        .node = (**((*oldlocset).locTab).offset(i_0 as isize)).user
                        as xmlNodePtr;
                    (*(*ctxt).context).contextSize = (*oldlocset).locNr;
                    (*(*ctxt).context).proximityPosition = i_0 + 1 as libc::c_int;
                    tmp = xmlXPathCacheNewNodeSet(
                        (*ctxt).context,
                        (*(*ctxt).context).node,
                    );
                    valuePush(ctxt, tmp);
                    if (*op).ch2 != -(1 as libc::c_int) {
                        total
                            += xmlXPathCompOpEval(
                                ctxt,
                                &mut *((*comp).steps).offset((*op).ch2 as isize),
                            );
                    }
                    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                        xmlXPtrFreeLocationSet(newlocset);
                        current_block_303 = 17645950395752231617;
                        break;
                    } else {
                        res = valuePop(ctxt);
                        if xmlXPathEvaluatePredicateResult(ctxt, res) != 0 {
                            xmlXPtrLocationSetAdd(
                                newlocset,
                                xmlXPathObjectCopy(
                                    *((*oldlocset).locTab).offset(i_0 as isize),
                                ),
                            );
                        }
                        if !res.is_null() {
                            xmlXPathReleaseObject((*ctxt).context, res);
                        }
                        if (*ctxt).value == tmp {
                            res = valuePop(ctxt);
                            xmlXPathReleaseObject((*ctxt).context, res);
                        }
                        i_0 += 1;
                        i_0;
                    }
                }
                match current_block_303 {
                    5159167864558512679 => {
                        valuePush(ctxt, xmlXPtrWrapLocationSet(newlocset));
                    }
                    _ => {}
                }
                xmlXPathReleaseObject((*ctxt).context, obj);
                (*(*ctxt).context).node = oldnode;
                (*(*ctxt).context).contextSize = oldcs;
                (*(*ctxt).context).proximityPosition = oldpp;
                return total;
            }
            if ((*ctxt).value).is_null()
                || (*(*ctxt).value).type_0 as libc::c_uint
                    != XPATH_NODESET as libc::c_int as libc::c_uint
            {
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                return 0 as libc::c_int;
            }
            if !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr != 0 as libc::c_int
            {
                let mut current_block_356: u64;
                obj = valuePop(ctxt);
                oldset = (*obj).nodesetval;
                oldnode = (*(*ctxt).context).node;
                oldDoc = (*(*ctxt).context).doc;
                oldcs = (*(*ctxt).context).contextSize;
                oldpp = (*(*ctxt).context).proximityPosition;
                tmp = 0 as xmlXPathObjectPtr;
                newset = xmlXPathNodeSetCreate(0 as xmlNodePtr);
                i_0 = 0 as libc::c_int;
                loop {
                    if !(i_0 < (*oldset).nodeNr) {
                        current_block_356 = 4954337906651494757;
                        break;
                    }
                    (*(*ctxt).context).node = *((*oldset).nodeTab).offset(i_0 as isize);
                    if (**((*oldset).nodeTab).offset(i_0 as isize)).type_0
                        as libc::c_uint
                        != XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        && !((**((*oldset).nodeTab).offset(i_0 as isize)).doc).is_null()
                    {
                        (*(*ctxt).context)
                            .doc = (**((*oldset).nodeTab).offset(i_0 as isize)).doc;
                    }
                    if tmp.is_null() {
                        tmp = xmlXPathCacheNewNodeSet(
                            (*ctxt).context,
                            (*(*ctxt).context).node,
                        );
                    } else if xmlXPathNodeSetAddUnique(
                        (*tmp).nodesetval,
                        (*(*ctxt).context).node,
                    ) < 0 as libc::c_int
                    {
                        (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                    }
                    valuePush(ctxt, tmp);
                    (*(*ctxt).context).contextSize = (*oldset).nodeNr;
                    (*(*ctxt).context).proximityPosition = i_0 + 1 as libc::c_int;
                    if (*op).ch2 != -(1 as libc::c_int) {
                        total
                            += xmlXPathCompOpEval(
                                ctxt,
                                &mut *((*comp).steps).offset((*op).ch2 as isize),
                            );
                    }
                    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                        xmlXPathFreeNodeSet(newset);
                        current_block_356 = 7758969965451026134;
                        break;
                    } else {
                        res = valuePop(ctxt);
                        if xmlXPathEvaluatePredicateResult(ctxt, res) != 0 {
                            if xmlXPathNodeSetAdd(
                                newset,
                                *((*oldset).nodeTab).offset(i_0 as isize),
                            ) < 0 as libc::c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as libc::c_int;
                            }
                        }
                        if !res.is_null() {
                            xmlXPathReleaseObject((*ctxt).context, res);
                        }
                        if (*ctxt).value == tmp {
                            valuePop(ctxt);
                            xmlXPathNodeSetClear((*tmp).nodesetval, 1 as libc::c_int);
                        } else {
                            tmp = 0 as xmlXPathObjectPtr;
                        }
                        i_0 += 1;
                        i_0;
                    }
                }
                match current_block_356 {
                    4954337906651494757 => {
                        if !tmp.is_null() {
                            xmlXPathReleaseObject((*ctxt).context, tmp);
                        }
                        valuePush(
                            ctxt,
                            xmlXPathCacheWrapNodeSet((*ctxt).context, newset),
                        );
                    }
                    _ => {}
                }
                xmlXPathReleaseObject((*ctxt).context, obj);
                (*(*ctxt).context).node = oldnode;
                (*(*ctxt).context).doc = oldDoc;
                (*(*ctxt).context).contextSize = oldcs;
                (*(*ctxt).context).proximityPosition = oldpp;
            }
            return total;
        }
        17 => {
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                return 0 as libc::c_int;
            }
            if !((*ctxt).value).is_null()
                && (*(*ctxt).value).type_0 as libc::c_uint
                    == XPATH_NODESET as libc::c_int as libc::c_uint
                && !((*(*ctxt).value).nodesetval).is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as libc::c_int
            {
                xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
            }
            return total;
        }
        18 => {
            let mut range: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut res_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut obj_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut tmp_0: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
            let mut newlocset_0: xmlLocationSetPtr = 0 as xmlLocationSetPtr;
            let mut oldlocset_0: xmlLocationSetPtr = 0 as *mut xmlLocationSet;
            let mut oldset_0: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
            let mut oldnode_0: xmlNodePtr = (*(*ctxt).context).node;
            let mut oldcs_0: libc::c_int = (*(*ctxt).context).contextSize;
            let mut oldpp_0: libc::c_int = (*(*ctxt).context).proximityPosition;
            let mut i_1: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            if (*op).ch1 != -(1 as libc::c_int) {
                total
                    += xmlXPathCompOpEval(
                        ctxt,
                        &mut *((*comp).steps).offset((*op).ch1 as isize),
                    );
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            if ((*ctxt).value).is_null() {
                xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as libc::c_int);
                return 0 as libc::c_int;
            }
            if (*op).ch2 == -(1 as libc::c_int) {
                return total;
            }
            if (*(*ctxt).value).type_0 as libc::c_uint
                == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
            {
                if ((*ctxt).value).is_null()
                    || (*(*ctxt).value).type_0 as libc::c_uint
                        != XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                {
                    xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                    return 0 as libc::c_int;
                }
                if ((*(*ctxt).value).user).is_null()
                    || (*((*(*ctxt).value).user as xmlLocationSetPtr)).locNr
                        == 0 as libc::c_int
                {
                    return total;
                }
                obj_0 = valuePop(ctxt);
                oldlocset_0 = (*obj_0).user as xmlLocationSetPtr;
                newlocset_0 = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
                i_1 = 0 as libc::c_int;
                loop {
                    if !(i_1 < (*oldlocset_0).locNr) {
                        current_block = 14935586237616944036;
                        break;
                    }
                    (*(*ctxt).context)
                        .node = (**((*oldlocset_0).locTab).offset(i_1 as isize)).user
                        as xmlNodePtr;
                    (*(*ctxt).context).contextSize = (*oldlocset_0).locNr;
                    (*(*ctxt).context).proximityPosition = i_1 + 1 as libc::c_int;
                    tmp_0 = xmlXPathCacheNewNodeSet(
                        (*ctxt).context,
                        (*(*ctxt).context).node,
                    );
                    valuePush(ctxt, tmp_0);
                    if (*op).ch2 != -(1 as libc::c_int) {
                        total
                            += xmlXPathCompOpEval(
                                ctxt,
                                &mut *((*comp).steps).offset((*op).ch2 as isize),
                            );
                    }
                    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                        xmlXPtrFreeLocationSet(newlocset_0);
                        current_block = 12884548584841406460;
                        break;
                    } else {
                        res_0 = valuePop(ctxt);
                        if (*res_0).type_0 as libc::c_uint
                            == XPATH_LOCATIONSET as libc::c_int as libc::c_uint
                        {
                            let mut rloc: xmlLocationSetPtr = (*res_0).user
                                as xmlLocationSetPtr;
                            j = 0 as libc::c_int;
                            while j < (*rloc).locNr {
                                range = xmlXPtrNewRange(
                                    (**((*oldlocset_0).locTab).offset(i_1 as isize)).user
                                        as xmlNodePtr,
                                    (**((*oldlocset_0).locTab).offset(i_1 as isize)).index,
                                    (**((*rloc).locTab).offset(j as isize)).user2 as xmlNodePtr,
                                    (**((*rloc).locTab).offset(j as isize)).index2,
                                );
                                if !range.is_null() {
                                    xmlXPtrLocationSetAdd(newlocset_0, range);
                                }
                                j += 1;
                                j;
                            }
                        } else {
                            range = xmlXPtrNewRangeNodeObject(
                                (**((*oldlocset_0).locTab).offset(i_1 as isize)).user
                                    as xmlNodePtr,
                                res_0,
                            );
                            if !range.is_null() {
                                xmlXPtrLocationSetAdd(newlocset_0, range);
                            }
                        }
                        if !res_0.is_null() {
                            xmlXPathReleaseObject((*ctxt).context, res_0);
                        }
                        if (*ctxt).value == tmp_0 {
                            res_0 = valuePop(ctxt);
                            xmlXPathReleaseObject((*ctxt).context, res_0);
                        }
                        i_1 += 1;
                        i_1;
                    }
                }
            } else {
                if ((*ctxt).value).is_null()
                    || (*(*ctxt).value).type_0 as libc::c_uint
                        != XPATH_NODESET as libc::c_int as libc::c_uint
                {
                    xmlXPathErr(ctxt, XPATH_INVALID_TYPE as libc::c_int);
                    return 0 as libc::c_int;
                }
                obj_0 = valuePop(ctxt);
                oldset_0 = (*obj_0).nodesetval;
                newlocset_0 = xmlXPtrLocationSetCreate(0 as xmlXPathObjectPtr);
                if !oldset_0.is_null() {
                    i_1 = 0 as libc::c_int;
                    loop {
                        if !(i_1 < (*oldset_0).nodeNr) {
                            current_block = 14935586237616944036;
                            break;
                        }
                        (*(*ctxt).context)
                            .node = *((*oldset_0).nodeTab).offset(i_1 as isize);
                        tmp_0 = xmlXPathCacheNewNodeSet(
                            (*ctxt).context,
                            (*(*ctxt).context).node,
                        );
                        valuePush(ctxt, tmp_0);
                        if (*op).ch2 != -(1 as libc::c_int) {
                            total
                                += xmlXPathCompOpEval(
                                    ctxt,
                                    &mut *((*comp).steps).offset((*op).ch2 as isize),
                                );
                        }
                        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                            xmlXPtrFreeLocationSet(newlocset_0);
                            current_block = 12884548584841406460;
                            break;
                        } else {
                            res_0 = valuePop(ctxt);
                            range = xmlXPtrNewRangeNodeObject(
                                *((*oldset_0).nodeTab).offset(i_1 as isize),
                                res_0,
                            );
                            if !range.is_null() {
                                xmlXPtrLocationSetAdd(newlocset_0, range);
                            }
                            if !res_0.is_null() {
                                xmlXPathReleaseObject((*ctxt).context, res_0);
                            }
                            if (*ctxt).value == tmp_0 {
                                res_0 = valuePop(ctxt);
                                xmlXPathReleaseObject((*ctxt).context, res_0);
                            }
                            i_1 += 1;
                            i_1;
                        }
                    }
                } else {
                    current_block = 14935586237616944036;
                }
            }
            match current_block {
                14935586237616944036 => {
                    valuePush(ctxt, xmlXPtrWrapLocationSet(newlocset_0));
                }
                _ => {}
            }
            xmlXPathReleaseObject((*ctxt).context, obj_0);
            (*(*ctxt).context).node = oldnode_0;
            (*(*ctxt).context).contextSize = oldcs_0;
            (*(*ctxt).context).proximityPosition = oldpp_0;
            return total;
        }
        _ => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"XPath: unknown precompiled operation %d\n\0" as *const u8
                    as *const libc::c_char,
                (*op).op as libc::c_uint,
            );
            (*ctxt).error = XPATH_INVALID_OPERAND as libc::c_int;
            return total;
        }
    };
}
unsafe extern "C" fn xmlXPathCompOpEvalToBoolean(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut isPredicate: libc::c_int,
) -> libc::c_int {
    let mut resObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    loop {
        match (*op).op as libc::c_uint {
            0 => return 0 as libc::c_int,
            11 => {
                resObj = (*op).value4 as xmlXPathObjectPtr;
                if isPredicate != 0 {
                    return xmlXPathEvaluatePredicateResult(ctxt, resObj);
                }
                return xmlXPathCastToBoolean(resObj);
            }
            17 => {
                if (*op).ch1 != -(1 as libc::c_int) {
                    op = &mut *((*(*ctxt).comp).steps).offset((*op).ch1 as isize)
                        as *mut xmlXPathStepOp;
                } else {
                    return 0 as libc::c_int
                }
            }
            10 => {
                if (*op).ch1 == -(1 as libc::c_int) {
                    return 0 as libc::c_int;
                }
                xmlXPathCompOpEval(
                    ctxt,
                    &mut *((*(*ctxt).comp).steps).offset((*op).ch1 as isize),
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return -(1 as libc::c_int);
                }
                xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    0 as *mut xmlNodePtr,
                    0 as *mut xmlNodePtr,
                    1 as libc::c_int,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return -(1 as libc::c_int);
                }
                resObj = valuePop(ctxt);
                if resObj.is_null() {
                    return -(1 as libc::c_int);
                }
                break;
            }
            _ => {
                xmlXPathCompOpEval(ctxt, op);
                if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
                    return -(1 as libc::c_int);
                }
                resObj = valuePop(ctxt);
                if resObj.is_null() {
                    return -(1 as libc::c_int);
                }
                break;
            }
        }
    }
    if !resObj.is_null() {
        let mut res: libc::c_int = 0;
        if (*resObj).type_0 as libc::c_uint
            == XPATH_BOOLEAN as libc::c_int as libc::c_uint
        {
            res = (*resObj).boolval;
        } else if isPredicate != 0 {
            res = xmlXPathEvaluatePredicateResult(ctxt, resObj);
        } else {
            res = xmlXPathCastToBoolean(resObj);
        }
        xmlXPathReleaseObject((*ctxt).context, resObj);
        return res;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXPathRunStreamEval(
    mut ctxt: xmlXPathContextPtr,
    mut comp: xmlPatternPtr,
    mut resultSeq: *mut xmlXPathObjectPtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut max_depth: libc::c_int = 0;
    let mut min_depth: libc::c_int = 0;
    let mut from_root: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut eval_all_nodes: libc::c_int = 0;
    let mut cur: xmlNodePtr = 0 as xmlNodePtr;
    let mut limit: xmlNodePtr = 0 as xmlNodePtr;
    let mut patstream: xmlStreamCtxtPtr = 0 as xmlStreamCtxtPtr;
    let mut nb_nodes: libc::c_int = 0 as libc::c_int;
    if ctxt.is_null() || comp.is_null() {
        return -(1 as libc::c_int);
    }
    max_depth = xmlPatternMaxDepth(comp);
    if max_depth == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if max_depth == -(2 as libc::c_int) {
        max_depth = 10000 as libc::c_int;
    }
    min_depth = xmlPatternMinDepth(comp);
    if min_depth == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    from_root = xmlPatternFromRoot(comp);
    if from_root < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if toBool == 0 {
        if resultSeq.is_null() {
            return -(1 as libc::c_int);
        }
        *resultSeq = xmlXPathCacheNewNodeSet(ctxt, 0 as xmlNodePtr);
        if (*resultSeq).is_null() {
            return -(1 as libc::c_int);
        }
    }
    if min_depth == 0 as libc::c_int {
        if from_root != 0 {
            if toBool != 0 {
                return 1 as libc::c_int;
            }
            xmlXPathNodeSetAddUnique(
                (**resultSeq).nodesetval,
                (*ctxt).doc as xmlNodePtr,
            );
        } else {
            if toBool != 0 {
                return 1 as libc::c_int;
            }
            xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, (*ctxt).node);
        }
    }
    if max_depth == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if from_root != 0 {
        cur = (*ctxt).doc as xmlNodePtr;
    } else if !((*ctxt).node).is_null() {
        match (*(*ctxt).node).type_0 as libc::c_uint {
            1 | 9 | 11 | 13 | 21 => {
                cur = (*ctxt).node;
            }
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 10 | 15 | 16 | 17 | 18 | 19 | 20
            | _ => {}
        }
        limit = cur;
    }
    if cur.is_null() {
        return 0 as libc::c_int;
    }
    patstream = xmlPatternGetStreamCtxt(comp);
    if patstream.is_null() {
        return 0 as libc::c_int;
    }
    eval_all_nodes = xmlStreamWantsAnyNode(patstream);
    if from_root != 0 {
        ret = xmlStreamPush(patstream, 0 as *const xmlChar, 0 as *const xmlChar);
        if ret < 0 as libc::c_int {
            current_block = 7420279277351916581;
        } else if ret == 1 as libc::c_int {
            if toBool != 0 {
                current_block = 5103259925606320165;
            } else {
                xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur);
                current_block = 7420279277351916581;
            }
        } else {
            current_block = 7420279277351916581;
        }
    } else {
        current_block = 7420279277351916581;
    }
    match current_block {
        7420279277351916581 => {
            depth = 0 as libc::c_int;
            '_scan_children: loop {
                if (*cur).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    current_block = 3312168862196081937;
                    break;
                }
                if !((*cur).children).is_null() && depth < max_depth {
                    if (*(*cur).children).type_0 as libc::c_uint
                        != XML_ENTITY_DECL as libc::c_int as libc::c_uint
                    {
                        cur = (*cur).children;
                        depth += 1;
                        depth;
                        if (*cur).type_0 as libc::c_uint
                            != XML_DTD_NODE as libc::c_int as libc::c_uint
                        {
                            current_block = 2290177392965769716;
                        } else {
                            current_block = 17019156190352891614;
                        }
                    } else {
                        current_block = 17019156190352891614;
                    }
                } else {
                    current_block = 17019156190352891614;
                }
                match current_block {
                    17019156190352891614 => {
                        if cur == limit {
                            current_block = 3312168862196081937;
                            break;
                        }
                        loop {
                            if ((*cur).next).is_null() {
                                current_block = 8062065914618164218;
                                break;
                            }
                            cur = (*cur).next;
                            if (*cur).type_0 as libc::c_uint
                                != XML_ENTITY_DECL as libc::c_int as libc::c_uint
                                && (*cur).type_0 as libc::c_uint
                                    != XML_DTD_NODE as libc::c_int as libc::c_uint
                            {
                                current_block = 13281731871476506071;
                                break;
                            }
                        }
                        match current_block {
                            13281731871476506071 => {}
                            _ => {
                                loop {
                                    cur = (*cur).parent;
                                    depth -= 1;
                                    depth;
                                    if cur.is_null() || cur == limit {
                                        current_block = 3312168862196081937;
                                        break '_scan_children;
                                    }
                                    if (*cur).type_0 as libc::c_uint
                                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                                    {
                                        ret = xmlStreamPop(patstream);
                                    } else if eval_all_nodes != 0
                                        && ((*cur).type_0 as libc::c_uint
                                            == XML_TEXT_NODE as libc::c_int as libc::c_uint
                                            || (*cur).type_0 as libc::c_uint
                                                == XML_CDATA_SECTION_NODE as libc::c_int as libc::c_uint
                                            || (*cur).type_0 as libc::c_uint
                                                == XML_COMMENT_NODE as libc::c_int as libc::c_uint
                                            || (*cur).type_0 as libc::c_uint
                                                == XML_PI_NODE as libc::c_int as libc::c_uint)
                                    {
                                        ret = xmlStreamPop(patstream);
                                    }
                                    if !((*cur).next).is_null() {
                                        cur = (*cur).next;
                                        break;
                                    } else if cur.is_null() {
                                        break;
                                    }
                                }
                                current_block = 2290177392965769716;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    2290177392965769716 => {
                        if !(!cur.is_null() && depth >= 0 as libc::c_int) {
                            current_block = 3312168862196081937;
                            break;
                        }
                    }
                    _ => {}
                }
                's_271: loop {
                    nb_nodes += 1;
                    nb_nodes;
                    match (*cur).type_0 as libc::c_uint {
                        1 | 3 | 4 | 8 | 7 => {}
                        _ => {
                            break;
                        }
                    }
                    if (*cur).type_0 as libc::c_uint
                        == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        ret = xmlStreamPush(
                            patstream,
                            (*cur).name,
                            if !((*cur).ns).is_null() {
                                (*(*cur).ns).href
                            } else {
                                0 as *const xmlChar
                            },
                        );
                    } else {
                        if !(eval_all_nodes != 0) {
                            break;
                        }
                        ret = xmlStreamPushNode(
                            patstream,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                            (*cur).type_0 as libc::c_int,
                        );
                    }
                    if !(ret < 0 as libc::c_int) {
                        if ret == 1 as libc::c_int {
                            if toBool != 0 {
                                current_block = 5103259925606320165;
                                break '_scan_children;
                            }
                            if xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur)
                                < 0 as libc::c_int
                            {
                                (*ctxt).lastError.domain = XML_FROM_XPATH as libc::c_int;
                                (*ctxt).lastError.code = XML_ERR_NO_MEMORY as libc::c_int;
                            }
                        }
                    }
                    if !(((*cur).children).is_null() || depth >= max_depth) {
                        break;
                    }
                    ret = xmlStreamPop(patstream);
                    loop {
                        if ((*cur).next).is_null() {
                            break 's_271;
                        }
                        cur = (*cur).next;
                        if (*cur).type_0 as libc::c_uint
                            != XML_ENTITY_DECL as libc::c_int as libc::c_uint
                            && (*cur).type_0 as libc::c_uint
                                != XML_DTD_NODE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                    }
                }
            }
            match current_block {
                5103259925606320165 => {}
                _ => {
                    if !patstream.is_null() {
                        xmlFreeStreamCtxt(patstream);
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn xmlXPathRunEval(
    mut ctxt: xmlXPathParserContextPtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    if ctxt.is_null() || ((*ctxt).comp).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*ctxt).valueTab).is_null() {
        (*ctxt)
            .valueTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (10 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<xmlXPathObjectPtr>() as libc::c_ulong,
                ),
        ) as *mut xmlXPathObjectPtr;
        if ((*ctxt).valueTab).is_null() {
            xmlXPathPErrMemory(
                ctxt,
                b"creating evaluation context\n\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
        }
        (*ctxt).valueNr = 0 as libc::c_int;
        (*ctxt).valueMax = 10 as libc::c_int;
        (*ctxt).value = 0 as xmlXPathObjectPtr;
        (*ctxt).valueFrame = 0 as libc::c_int;
    }
    if !((*(*ctxt).comp).stream).is_null() {
        let mut res: libc::c_int = 0;
        if toBool != 0 {
            res = xmlXPathRunStreamEval(
                (*ctxt).context,
                (*(*ctxt).comp).stream,
                0 as *mut xmlXPathObjectPtr,
                1 as libc::c_int,
            );
            if res != -(1 as libc::c_int) {
                return res;
            }
        } else {
            let mut resObj: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
            res = xmlXPathRunStreamEval(
                (*ctxt).context,
                (*(*ctxt).comp).stream,
                &mut resObj,
                0 as libc::c_int,
            );
            if res != -(1 as libc::c_int) && !resObj.is_null() {
                valuePush(ctxt, resObj);
                return 0 as libc::c_int;
            }
            if !resObj.is_null() {
                xmlXPathReleaseObject((*ctxt).context, resObj);
            }
        }
    }
    comp = (*ctxt).comp;
    if (*comp).last < 0 as libc::c_int {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlXPathRunEval: last is less than zero\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if toBool != 0 {
        return xmlXPathCompOpEvalToBoolean(
            ctxt,
            &mut *((*comp).steps).offset((*comp).last as isize),
            0 as libc::c_int,
        )
    } else {
        xmlXPathCompOpEval(ctxt, &mut *((*comp).steps).offset((*comp).last as isize));
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalPredicate(
    mut ctxt: xmlXPathContextPtr,
    mut res: xmlXPathObjectPtr,
) -> libc::c_int {
    if ctxt.is_null() || res.is_null() {
        return 0 as libc::c_int;
    }
    match (*res).type_0 as libc::c_uint {
        2 => return (*res).boolval,
        3 => {
            return ((*res).floatval == (*ctxt).proximityPosition as libc::c_double)
                as libc::c_int;
        }
        1 | 9 => {
            if ((*res).nodesetval).is_null() {
                return 0 as libc::c_int;
            }
            return ((*(*res).nodesetval).nodeNr != 0 as libc::c_int) as libc::c_int;
        }
        4 => {
            return (!((*res).stringval).is_null()
                && xmlStrlen((*res).stringval) != 0 as libc::c_int) as libc::c_int;
        }
        _ => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Internal error at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                14418 as libc::c_int,
            );
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvaluatePredicateResult(
    mut ctxt: xmlXPathParserContextPtr,
    mut res: xmlXPathObjectPtr,
) -> libc::c_int {
    if ctxt.is_null() || res.is_null() {
        return 0 as libc::c_int;
    }
    match (*res).type_0 as libc::c_uint {
        2 => return (*res).boolval,
        3 => {
            return ((*res).floatval
                == (*(*ctxt).context).proximityPosition as libc::c_double) as libc::c_int;
        }
        1 | 9 => {
            if ((*res).nodesetval).is_null() {
                return 0 as libc::c_int;
            }
            return ((*(*res).nodesetval).nodeNr != 0 as libc::c_int) as libc::c_int;
        }
        4 => {
            return (!((*res).stringval).is_null()
                && *((*res).stringval).offset(0 as libc::c_int as isize) as libc::c_int
                    != 0 as libc::c_int) as libc::c_int;
        }
        7 => {
            let mut ptr: xmlLocationSetPtr = (*res).user as xmlLocationSetPtr;
            if ptr.is_null() {
                return 0 as libc::c_int;
            }
            return ((*ptr).locNr != 0 as libc::c_int) as libc::c_int;
        }
        _ => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Internal error at %s:%d\n\0" as *const u8 as *const libc::c_char,
                b"xpath.c\0" as *const u8 as *const libc::c_char,
                14469 as libc::c_int,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXPathTryStreamCompile(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let mut stream: xmlPatternPtr = 0 as *mut xmlPattern;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    let mut namespaces: *mut *const xmlChar = 0 as *mut *const xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (xmlStrchr(str, '[' as i32 as xmlChar)).is_null()
        && (xmlStrchr(str, '(' as i32 as xmlChar)).is_null()
        && (xmlStrchr(str, '@' as i32 as xmlChar)).is_null()
    {
        let mut tmp: *const xmlChar = 0 as *const xmlChar;
        tmp = xmlStrchr(str, ':' as i32 as xmlChar);
        if !tmp.is_null()
            && (ctxt.is_null() || (*ctxt).nsNr == 0 as libc::c_int
                || *tmp.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32)
        {
            return 0 as xmlXPathCompExprPtr;
        }
        if !ctxt.is_null() {
            dict = (*ctxt).dict;
            if (*ctxt).nsNr > 0 as libc::c_int {
                namespaces = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(
                    ((2 as libc::c_int * ((*ctxt).nsNr + 1 as libc::c_int))
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong,
                        ),
                ) as *mut *const xmlChar;
                if namespaces.is_null() {
                    xmlXPathErrMemory(
                        ctxt,
                        b"allocating namespaces array\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as xmlXPathCompExprPtr;
                }
                i = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < (*ctxt).nsNr {
                    ns = *((*ctxt).namespaces).offset(j as isize);
                    let fresh79 = i;
                    i = i + 1;
                    let ref mut fresh80 = *namespaces.offset(fresh79 as isize);
                    *fresh80 = (*ns).href;
                    let fresh81 = i;
                    i = i + 1;
                    let ref mut fresh82 = *namespaces.offset(fresh81 as isize);
                    *fresh82 = (*ns).prefix;
                    j += 1;
                    j;
                }
                let fresh83 = i;
                i = i + 1;
                let ref mut fresh84 = *namespaces.offset(fresh83 as isize);
                *fresh84 = 0 as *const xmlChar;
                let ref mut fresh85 = *namespaces.offset(i as isize);
                *fresh85 = 0 as *const xmlChar;
            }
        }
        stream = xmlPatterncompile(
            str,
            dict,
            XML_PATTERN_XPATH as libc::c_int,
            &mut *namespaces.offset(0 as libc::c_int as isize),
        );
        if !namespaces.is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )(namespaces as *mut *mut xmlChar as *mut libc::c_void);
        }
        if !stream.is_null() && xmlPatternStreamable(stream) == 1 as libc::c_int {
            comp = xmlXPathNewCompExpr();
            if comp.is_null() {
                xmlXPathErrMemory(
                    ctxt,
                    b"allocating streamable expression\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as xmlXPathCompExprPtr;
            }
            (*comp).stream = stream;
            (*comp).dict = dict;
            if !((*comp).dict).is_null() {
                xmlDictReference((*comp).dict);
            }
            return comp;
        }
        xmlFreePattern(stream);
    }
    return 0 as xmlXPathCompExprPtr;
}
unsafe extern "C" fn xmlXPathOptimizeExpression(
    mut comp: xmlXPathCompExprPtr,
    mut op: xmlXPathStepOpPtr,
) {
    if (*op).op as libc::c_uint == XPATH_OP_COLLECT as libc::c_int as libc::c_uint
        && (*op).ch1 != -(1 as libc::c_int) && (*op).ch2 == -(1 as libc::c_int)
    {
        let mut prevop: xmlXPathStepOpPtr = &mut *((*comp).steps)
            .offset((*op).ch1 as isize) as *mut xmlXPathStepOp;
        if (*prevop).op as libc::c_uint
            == XPATH_OP_COLLECT as libc::c_int as libc::c_uint
            && (*prevop).value as xmlXPathAxisVal as libc::c_uint
                == AXIS_DESCENDANT_OR_SELF as libc::c_int as libc::c_uint
            && (*prevop).ch2 == -(1 as libc::c_int)
            && (*prevop).value2 as xmlXPathTestVal as libc::c_uint
                == NODE_TEST_TYPE as libc::c_int as libc::c_uint
            && (*prevop).value3 as xmlXPathTypeVal as libc::c_uint
                == NODE_TYPE_NODE as libc::c_int as libc::c_uint
        {
            match (*op).value as xmlXPathAxisVal as libc::c_uint {
                4 | 5 => {
                    (*op).ch1 = (*prevop).ch1;
                    (*op).value = AXIS_DESCENDANT as libc::c_int;
                }
                13 | 6 => {
                    (*op).ch1 = (*prevop).ch1;
                    (*op).value = AXIS_DESCENDANT_OR_SELF as libc::c_int;
                }
                _ => {}
            }
        }
    }
    if (*op).op as libc::c_uint == XPATH_OP_VALUE as libc::c_int as libc::c_uint {
        return;
    }
    if (*op).ch1 != -(1 as libc::c_int) {
        xmlXPathOptimizeExpression(
            comp,
            &mut *((*comp).steps).offset((*op).ch1 as isize),
        );
    }
    if (*op).ch2 != -(1 as libc::c_int) {
        xmlXPathOptimizeExpression(
            comp,
            &mut *((*comp).steps).offset((*op).ch2 as isize),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCtxtCompile(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let mut pctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    comp = xmlXPathTryStreamCompile(ctxt, str);
    if !comp.is_null() {
        return comp;
    }
    xmlXPathInit();
    pctxt = xmlXPathNewParserContext(str, ctxt);
    if pctxt.is_null() {
        return 0 as xmlXPathCompExprPtr;
    }
    xmlXPathCompileExpr(pctxt, 1 as libc::c_int);
    if (*pctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        xmlXPathFreeParserContext(pctxt);
        return 0 as xmlXPathCompExprPtr;
    }
    if *(*pctxt).cur as libc::c_int != 0 as libc::c_int {
        xmlXPatherror(
            pctxt,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            14661 as libc::c_int,
            XPATH_EXPR_ERROR as libc::c_int,
        );
        comp = 0 as xmlXPathCompExprPtr;
    } else {
        comp = (*pctxt).comp;
        (*pctxt).comp = 0 as xmlXPathCompExprPtr;
    }
    xmlXPathFreeParserContext(pctxt);
    if !comp.is_null() {
        (*comp).expr = xmlStrdup(str);
        if (*comp).nbStep > 1 as libc::c_int && (*comp).last >= 0 as libc::c_int {
            xmlXPathOptimizeExpression(
                comp,
                &mut *((*comp).steps).offset((*comp).last as isize),
            );
        }
    }
    return comp;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompile(
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    return xmlXPathCtxtCompile(0 as xmlXPathContextPtr, str);
}
unsafe extern "C" fn xmlXPathCompiledEvalInternal(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
    mut resObjPtr: *mut xmlXPathObjectPtr,
    mut toBool: libc::c_int,
) -> libc::c_int {
    let mut pctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut resObj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut res: libc::c_int = 0;
    if ctxt.is_null() {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            XML_ERR_INTERNAL_ERROR as libc::c_int,
            XML_ERR_FATAL,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            14722 as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"NULL context pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    xmlXPathInit();
    pctxt = xmlXPathCompParserContext(comp, ctxt);
    res = xmlXPathRunEval(pctxt, toBool);
    if (*pctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        resObj = 0 as xmlXPathObjectPtr;
    } else {
        resObj = valuePop(pctxt);
        if resObj.is_null() {
            if toBool == 0 {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathCompiledEval: No result on the stack.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if (*pctxt).valueNr > 0 as libc::c_int {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\0" as *const u8
                    as *const libc::c_char,
                (*pctxt).valueNr,
            );
        }
    }
    if !resObjPtr.is_null() {
        *resObjPtr = resObj;
    } else {
        xmlXPathReleaseObject(ctxt, resObj);
    }
    (*pctxt).comp = 0 as xmlXPathCompExprPtr;
    xmlXPathFreeParserContext(pctxt);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompiledEval(
    mut comp: xmlXPathCompExprPtr,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let mut res: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
    xmlXPathCompiledEvalInternal(comp, ctx, &mut res, 0 as libc::c_int);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCompiledEvalToBoolean(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> libc::c_int {
    return xmlXPathCompiledEvalInternal(
        comp,
        ctxt,
        0 as *mut xmlXPathObjectPtr,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalExpr(mut ctxt: xmlXPathParserContextPtr) {
    let mut comp: xmlXPathCompExprPtr = 0 as *mut xmlXPathCompExpr;
    if ctxt.is_null() {
        return;
    }
    comp = xmlXPathTryStreamCompile((*ctxt).context, (*ctxt).base);
    if !comp.is_null() {
        if !((*ctxt).comp).is_null() {
            xmlXPathFreeCompExpr((*ctxt).comp);
        }
        (*ctxt).comp = comp;
    } else {
        xmlXPathCompileExpr(ctxt, 1 as libc::c_int);
        if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
            return;
        }
        if *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as libc::c_int);
            return;
        }
        if (*(*ctxt).comp).nbStep > 1 as libc::c_int
            && (*(*ctxt).comp).last >= 0 as libc::c_int
        {
            xmlXPathOptimizeExpression(
                (*ctxt).comp,
                &mut *((*(*ctxt).comp).steps).offset((*(*ctxt).comp).last as isize),
            );
        }
    }
    xmlXPathRunEval(ctxt, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEval(
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let mut ctxt: xmlXPathParserContextPtr = 0 as *mut xmlXPathParserContext;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    if ctx.is_null() {
        __xmlRaiseError(
            None,
            None,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_XPATH as libc::c_int,
            XML_ERR_INTERNAL_ERROR as libc::c_int,
            XML_ERR_FATAL,
            b"xpath.c\0" as *const u8 as *const libc::c_char,
            14864 as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            b"NULL context pointer\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXPathObjectPtr;
    }
    xmlXPathInit();
    ctxt = xmlXPathNewParserContext(str, ctx);
    if ctxt.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    xmlXPathEvalExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as libc::c_int {
        res = 0 as xmlXPathObjectPtr;
    } else {
        res = valuePop(ctxt);
        if res.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompiledEval: No result on the stack.\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if (*ctxt).valueNr > 0 as libc::c_int {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\0" as *const u8
                    as *const libc::c_char,
                (*ctxt).valueNr,
            );
        }
    }
    xmlXPathFreeParserContext(ctxt);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathSetContextNode(
    mut node: xmlNodePtr,
    mut ctx: xmlXPathContextPtr,
) -> libc::c_int {
    if node.is_null() || ctx.is_null() {
        return -(1 as libc::c_int);
    }
    if (*node).doc == (*ctx).doc {
        (*ctx).node = node;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNodeEval(
    mut node: xmlNodePtr,
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    if str.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if xmlXPathSetContextNode(node, ctx) < 0 as libc::c_int {
        return 0 as xmlXPathObjectPtr;
    }
    return xmlXPathEval(str, ctx);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEvalExpression(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    return xmlXPathEval(str, ctxt);
}
unsafe extern "C" fn xmlXPathEscapeUriFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut str: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut escape_reserved: libc::c_int = 0;
    let mut target: xmlBufPtr = 0 as *mut xmlBuf;
    let mut cptr: *mut xmlChar = 0 as *mut xmlChar;
    let mut escape: [xmlChar; 4] = [0; 4];
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as libc::c_int);
        return;
    }
    if (*ctxt).valueNr < (*ctxt).valueFrame + 2 as libc::c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as libc::c_int);
        return;
    }
    escape_reserved = xmlXPathPopBoolean(ctxt);
    if !((*ctxt).value).is_null()
        && (*(*ctxt).value).type_0 as libc::c_uint
            != XPATH_STRING as libc::c_int as libc::c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as libc::c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    escape[0 as libc::c_int as usize] = '%' as i32 as xmlChar;
    escape[3 as libc::c_int as usize] = 0 as libc::c_int as xmlChar;
    if !target.is_null() {
        cptr = (*str).stringval;
        while *cptr != 0 {
            if *cptr as libc::c_int >= 'A' as i32 && *cptr as libc::c_int <= 'Z' as i32
                || *cptr as libc::c_int >= 'a' as i32
                    && *cptr as libc::c_int <= 'z' as i32
                || *cptr as libc::c_int >= '0' as i32
                    && *cptr as libc::c_int <= '9' as i32
                || *cptr as libc::c_int == '-' as i32
                || *cptr as libc::c_int == '_' as i32
                || *cptr as libc::c_int == '.' as i32
                || *cptr as libc::c_int == '!' as i32
                || *cptr as libc::c_int == '~' as i32
                || *cptr as libc::c_int == '*' as i32
                || *cptr as libc::c_int == '\'' as i32
                || *cptr as libc::c_int == '(' as i32
                || *cptr as libc::c_int == ')' as i32
                || *cptr as libc::c_int == '%' as i32
                    && (*cptr.offset(1 as libc::c_int as isize) as libc::c_int
                        >= 'A' as i32
                        && *cptr.offset(1 as libc::c_int as isize) as libc::c_int
                            <= 'F' as i32
                        || *cptr.offset(1 as libc::c_int as isize) as libc::c_int
                            >= 'a' as i32
                            && *cptr.offset(1 as libc::c_int as isize) as libc::c_int
                                <= 'f' as i32
                        || *cptr.offset(1 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *cptr.offset(1 as libc::c_int as isize) as libc::c_int
                                <= '9' as i32)
                    && (*cptr.offset(2 as libc::c_int as isize) as libc::c_int
                        >= 'A' as i32
                        && *cptr.offset(2 as libc::c_int as isize) as libc::c_int
                            <= 'F' as i32
                        || *cptr.offset(2 as libc::c_int as isize) as libc::c_int
                            >= 'a' as i32
                            && *cptr.offset(2 as libc::c_int as isize) as libc::c_int
                                <= 'f' as i32
                        || *cptr.offset(2 as libc::c_int as isize) as libc::c_int
                            >= '0' as i32
                            && *cptr.offset(2 as libc::c_int as isize) as libc::c_int
                                <= '9' as i32)
                || escape_reserved == 0
                    && (*cptr as libc::c_int == ';' as i32
                        || *cptr as libc::c_int == '/' as i32
                        || *cptr as libc::c_int == '?' as i32
                        || *cptr as libc::c_int == ':' as i32
                        || *cptr as libc::c_int == '@' as i32
                        || *cptr as libc::c_int == '&' as i32
                        || *cptr as libc::c_int == '=' as i32
                        || *cptr as libc::c_int == '+' as i32
                        || *cptr as libc::c_int == '$' as i32
                        || *cptr as libc::c_int == ',' as i32)
            {
                xmlBufAdd(target, cptr, 1 as libc::c_int);
            } else {
                if (*cptr as libc::c_int >> 4 as libc::c_int) < 10 as libc::c_int {
                    escape[1 as libc::c_int
                        as usize] = ('0' as i32
                        + (*cptr as libc::c_int >> 4 as libc::c_int)) as xmlChar;
                } else {
                    escape[1 as libc::c_int
                        as usize] = ('A' as i32 - 10 as libc::c_int
                        + (*cptr as libc::c_int >> 4 as libc::c_int)) as xmlChar;
                }
                if (*cptr as libc::c_int & 0xf as libc::c_int) < 10 as libc::c_int {
                    escape[2 as libc::c_int
                        as usize] = ('0' as i32
                        + (*cptr as libc::c_int & 0xf as libc::c_int)) as xmlChar;
                } else {
                    escape[2 as libc::c_int
                        as usize] = ('A' as i32 - 10 as libc::c_int
                        + (*cptr as libc::c_int & 0xf as libc::c_int)) as xmlChar;
                }
                xmlBufAdd(
                    target,
                    &mut *escape.as_mut_ptr().offset(0 as libc::c_int as isize),
                    3 as libc::c_int,
                );
            }
            cptr = cptr.offset(1);
            cptr;
        }
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewString((*ctxt).context, xmlBufContent(target as *const xmlBuf)),
    );
    xmlBufFree(target);
    xmlXPathReleaseObject((*ctxt).context, str);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterAllFunctions(mut ctxt: xmlXPathContextPtr) {
    xmlXPathRegisterFunc(
        ctxt,
        b"boolean\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathBooleanFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"ceiling\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathCeilingFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"count\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathCountFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"concat\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathConcatFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"contains\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathContainsFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"id\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathIdFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"false\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathFalseFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"floor\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathFloorFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"last\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathLastFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"lang\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathLangFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"local-name\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathLocalNameFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"not\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathNotFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"name\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathNameFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"namespace-uri\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathNamespaceURIFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"normalize-space\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathNormalizeFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"number\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathNumberFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"position\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathPositionFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"round\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathRoundFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"string\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathStringFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"string-length\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathStringLengthFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"starts-with\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathStartsWithFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"substring\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathSubstringFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"substring-before\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathSubstringBeforeFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"substring-after\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathSubstringAfterFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"sum\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathSumFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"true\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathTrueFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"translate\0" as *const u8 as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathTranslateFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
    xmlXPathRegisterFuncNS(
        ctxt,
        b"escape-uri\0" as *const u8 as *const libc::c_char as *const xmlChar,
        b"http://www.w3.org/2002/08/xquery-functions\0" as *const u8
            as *const libc::c_char as *const xmlChar,
        Some(
            xmlXPathEscapeUriFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
        ),
    );
}
