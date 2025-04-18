use ::libc;
extern "C" {
    pub type _xmlDict;
    pub type _xmlHashTable;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlStrlen(str: *const xmlChar) -> libc::c_int;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn xmlBufferAdd(
        buf: xmlBufferPtr,
        str: *const xmlChar,
        len: libc::c_int,
    ) -> libc::c_int;
    fn xmlBufferCCat(buf: xmlBufferPtr, str: *const libc::c_char) -> libc::c_int;
    fn xmlFreeNodeList(cur: xmlNodePtr);
    fn xmlBufferWriteCHAR(buf: xmlBufferPtr, string: *const xmlChar);
    fn xmlBufferWriteChar(buf: xmlBufferPtr, string: *const libc::c_char);
    fn xmlBufferWriteQuotedString(buf: xmlBufferPtr, string: *const xmlChar);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: libc::c_int,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> libc::c_int;
    fn __xmlSimpleError(
        domain: libc::c_int,
        code: libc::c_int,
        node: xmlNodePtr,
        msg: *const libc::c_char,
        extra: *const libc::c_char,
    );
    fn xmlHashCreate(size: libc::c_int) -> xmlHashTablePtr;
    fn xmlHashCreateDict(size: libc::c_int, dict: xmlDictPtr) -> xmlHashTablePtr;
    fn xmlHashFree(table: xmlHashTablePtr, f: xmlHashDeallocator);
    fn xmlHashAddEntry(
        table: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn xmlHashLookup(table: xmlHashTablePtr, name: *const xmlChar) -> *mut libc::c_void;
    fn xmlHashCopy(table: xmlHashTablePtr, f: xmlHashCopier) -> xmlHashTablePtr;
    fn xmlHashScan(table: xmlHashTablePtr, f: xmlHashScanner, data: *mut libc::c_void);
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
}
pub type xmlChar = libc::c_uchar;
pub type size_t = libc::c_ulong;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
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
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
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
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlHashDeallocator = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type xmlHashCopier = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> *mut libc::c_void,
>;
pub type xmlHashScanner = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, *const xmlChar) -> (),
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
pub type xmlEntitiesTable = _xmlHashTable;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
static mut xmlEntityLt: xmlEntity = {
    let mut init = _xmlEntity {
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        type_0: XML_ENTITY_DECL,
        name: b"lt\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        children: 0 as *const _xmlNode as *mut _xmlNode,
        last: 0 as *const _xmlNode as *mut _xmlNode,
        parent: 0 as *const _xmlDtd as *mut _xmlDtd,
        next: 0 as *const _xmlNode as *mut _xmlNode,
        prev: 0 as *const _xmlNode as *mut _xmlNode,
        doc: 0 as *const _xmlDoc as *mut _xmlDoc,
        orig: b"<\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        content: b"<\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        length: 1 as libc::c_int,
        etype: XML_INTERNAL_PREDEFINED_ENTITY,
        ExternalID: 0 as *const xmlChar,
        SystemID: 0 as *const xmlChar,
        nexte: 0 as *const _xmlEntity as *mut _xmlEntity,
        URI: 0 as *const xmlChar,
        owner: 0 as libc::c_int,
        checked: 1 as libc::c_int,
    };
    init
};
static mut xmlEntityGt: xmlEntity = {
    let mut init = _xmlEntity {
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        type_0: XML_ENTITY_DECL,
        name: b"gt\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        children: 0 as *const _xmlNode as *mut _xmlNode,
        last: 0 as *const _xmlNode as *mut _xmlNode,
        parent: 0 as *const _xmlDtd as *mut _xmlDtd,
        next: 0 as *const _xmlNode as *mut _xmlNode,
        prev: 0 as *const _xmlNode as *mut _xmlNode,
        doc: 0 as *const _xmlDoc as *mut _xmlDoc,
        orig: b">\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        content: b">\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        length: 1 as libc::c_int,
        etype: XML_INTERNAL_PREDEFINED_ENTITY,
        ExternalID: 0 as *const xmlChar,
        SystemID: 0 as *const xmlChar,
        nexte: 0 as *const _xmlEntity as *mut _xmlEntity,
        URI: 0 as *const xmlChar,
        owner: 0 as libc::c_int,
        checked: 1 as libc::c_int,
    };
    init
};
static mut xmlEntityAmp: xmlEntity = {
    let mut init = _xmlEntity {
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        type_0: XML_ENTITY_DECL,
        name: b"amp\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        children: 0 as *const _xmlNode as *mut _xmlNode,
        last: 0 as *const _xmlNode as *mut _xmlNode,
        parent: 0 as *const _xmlDtd as *mut _xmlDtd,
        next: 0 as *const _xmlNode as *mut _xmlNode,
        prev: 0 as *const _xmlNode as *mut _xmlNode,
        doc: 0 as *const _xmlDoc as *mut _xmlDoc,
        orig: b"&\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        content: b"&\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        length: 1 as libc::c_int,
        etype: XML_INTERNAL_PREDEFINED_ENTITY,
        ExternalID: 0 as *const xmlChar,
        SystemID: 0 as *const xmlChar,
        nexte: 0 as *const _xmlEntity as *mut _xmlEntity,
        URI: 0 as *const xmlChar,
        owner: 0 as libc::c_int,
        checked: 1 as libc::c_int,
    };
    init
};
static mut xmlEntityQuot: xmlEntity = {
    let mut init = _xmlEntity {
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        type_0: XML_ENTITY_DECL,
        name: b"quot\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        children: 0 as *const _xmlNode as *mut _xmlNode,
        last: 0 as *const _xmlNode as *mut _xmlNode,
        parent: 0 as *const _xmlDtd as *mut _xmlDtd,
        next: 0 as *const _xmlNode as *mut _xmlNode,
        prev: 0 as *const _xmlNode as *mut _xmlNode,
        doc: 0 as *const _xmlDoc as *mut _xmlDoc,
        orig: b"\"\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        content: b"\"\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        length: 1 as libc::c_int,
        etype: XML_INTERNAL_PREDEFINED_ENTITY,
        ExternalID: 0 as *const xmlChar,
        SystemID: 0 as *const xmlChar,
        nexte: 0 as *const _xmlEntity as *mut _xmlEntity,
        URI: 0 as *const xmlChar,
        owner: 0 as libc::c_int,
        checked: 1 as libc::c_int,
    };
    init
};
static mut xmlEntityApos: xmlEntity = {
    let mut init = _xmlEntity {
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        type_0: XML_ENTITY_DECL,
        name: b"apos\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        children: 0 as *const _xmlNode as *mut _xmlNode,
        last: 0 as *const _xmlNode as *mut _xmlNode,
        parent: 0 as *const _xmlDtd as *mut _xmlDtd,
        next: 0 as *const _xmlNode as *mut _xmlNode,
        prev: 0 as *const _xmlNode as *mut _xmlNode,
        doc: 0 as *const _xmlDoc as *mut _xmlDoc,
        orig: b"'\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        content: b"'\0" as *const u8 as *const libc::c_char as *mut xmlChar,
        length: 1 as libc::c_int,
        etype: XML_INTERNAL_PREDEFINED_ENTITY,
        ExternalID: 0 as *const xmlChar,
        SystemID: 0 as *const xmlChar,
        nexte: 0 as *const _xmlEntity as *mut _xmlEntity,
        URI: 0 as *const xmlChar,
        owner: 0 as libc::c_int,
        checked: 1 as libc::c_int,
    };
    init
};
unsafe extern "C" fn xmlEntitiesErrMemory(mut extra: *const libc::c_char) {
    __xmlSimpleError(
        XML_FROM_TREE as libc::c_int,
        XML_ERR_NO_MEMORY as libc::c_int,
        0 as xmlNodePtr,
        0 as *const libc::c_char,
        extra,
    );
}
unsafe extern "C" fn xmlEntitiesErr(
    mut code: xmlParserErrors,
    mut msg: *const libc::c_char,
) {
    __xmlSimpleError(
        XML_FROM_TREE as libc::c_int,
        code as libc::c_int,
        0 as xmlNodePtr,
        msg,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn xmlFreeEntity(mut entity: xmlEntityPtr) {
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    if entity.is_null() {
        return;
    }
    if !((*entity).doc).is_null() {
        dict = (*(*entity).doc).dict;
    }
    if !((*entity).children).is_null() && (*entity).owner == 1 as libc::c_int
        && entity == (*(*entity).children).parent as xmlEntityPtr
    {
        xmlFreeNodeList((*entity).children);
    }
    if !dict.is_null() {
        if !((*entity).name).is_null() && xmlDictOwns(dict, (*entity).name) == 0 {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).name as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).ExternalID).is_null()
            && xmlDictOwns(dict, (*entity).ExternalID) == 0
        {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).ExternalID as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).SystemID).is_null() && xmlDictOwns(dict, (*entity).SystemID) == 0
        {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).SystemID as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).URI).is_null() && xmlDictOwns(dict, (*entity).URI) == 0 {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).URI as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).content).is_null() && xmlDictOwns(dict, (*entity).content) == 0 {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).content as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).orig).is_null() && xmlDictOwns(dict, (*entity).orig) == 0 {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).orig as *mut libc::c_char as *mut libc::c_void);
        }
    } else {
        if !((*entity).name).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).name as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).ExternalID).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).ExternalID as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).SystemID).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).SystemID as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).URI).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).URI as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).content).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).content as *mut libc::c_char as *mut libc::c_void);
        }
        if !((*entity).orig).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*entity).orig as *mut libc::c_char as *mut libc::c_void);
        }
    }
    xmlFree.expect("non-null function pointer")(entity as *mut libc::c_void);
}
unsafe extern "C" fn xmlCreateEntity(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlEntityPtr {
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlEntity>() as libc::c_ulong) as xmlEntityPtr;
    if ret.is_null() {
        xmlEntitiesErrMemory(
            b"xmlCreateEntity: malloc failed\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlEntityPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlEntity>() as libc::c_ulong,
    );
    (*ret).type_0 = XML_ENTITY_DECL;
    (*ret).checked = 0 as libc::c_int;
    (*ret).etype = type_0 as xmlEntityType;
    if dict.is_null() {
        (*ret).name = xmlStrdup(name);
        if !ExternalID.is_null() {
            (*ret).ExternalID = xmlStrdup(ExternalID);
        }
        if !SystemID.is_null() {
            (*ret).SystemID = xmlStrdup(SystemID);
        }
    } else {
        (*ret).name = xmlDictLookup(dict, name, -(1 as libc::c_int));
        if !ExternalID.is_null() {
            (*ret).ExternalID = xmlDictLookup(dict, ExternalID, -(1 as libc::c_int));
        }
        if !SystemID.is_null() {
            (*ret).SystemID = xmlDictLookup(dict, SystemID, -(1 as libc::c_int));
        }
    }
    if !content.is_null() {
        (*ret).length = xmlStrlen(content);
        if !dict.is_null() && (*ret).length < 5 as libc::c_int {
            (*ret).content = xmlDictLookup(dict, content, (*ret).length) as *mut xmlChar;
        } else {
            (*ret).content = xmlStrndup(content, (*ret).length);
        }
    } else {
        (*ret).length = 0 as libc::c_int;
        (*ret).content = 0 as *mut xmlChar;
    }
    (*ret).URI = 0 as *const xmlChar;
    (*ret).orig = 0 as *mut xmlChar;
    (*ret).owner = 0 as libc::c_int;
    return ret;
}
unsafe extern "C" fn xmlAddEntity(
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlEntityPtr {
    let mut dict: xmlDictPtr = 0 as xmlDictPtr;
    let mut table: xmlEntitiesTablePtr = 0 as xmlEntitiesTablePtr;
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    if name.is_null() {
        return 0 as xmlEntityPtr;
    }
    if dtd.is_null() {
        return 0 as xmlEntityPtr;
    }
    if !((*dtd).doc).is_null() {
        dict = (*(*dtd).doc).dict;
    }
    match type_0 {
        1 | 2 | 3 => {
            if ((*dtd).entities).is_null() {
                (*dtd)
                    .entities = xmlHashCreateDict(0 as libc::c_int, dict)
                    as *mut libc::c_void;
            }
            table = (*dtd).entities as xmlEntitiesTablePtr;
        }
        4 | 5 => {
            if ((*dtd).pentities).is_null() {
                (*dtd)
                    .pentities = xmlHashCreateDict(0 as libc::c_int, dict)
                    as *mut libc::c_void;
            }
            table = (*dtd).pentities as xmlEntitiesTablePtr;
        }
        6 => return 0 as xmlEntityPtr,
        _ => {}
    }
    if table.is_null() {
        return 0 as xmlEntityPtr;
    }
    ret = xmlCreateEntity(dict, name, type_0, ExternalID, SystemID, content);
    if ret.is_null() {
        return 0 as xmlEntityPtr;
    }
    (*ret).doc = (*dtd).doc;
    if xmlHashAddEntry(table, name, ret as *mut libc::c_void) != 0 {
        xmlFreeEntity(ret);
        return 0 as xmlEntityPtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetPredefinedEntity(
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    if name.is_null() {
        return 0 as xmlEntityPtr;
    }
    match *name.offset(0 as libc::c_int as isize) as libc::c_int {
        108 => {
            if xmlStrEqual(
                name,
                b"lt\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                return &mut xmlEntityLt;
            }
        }
        103 => {
            if xmlStrEqual(
                name,
                b"gt\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                return &mut xmlEntityGt;
            }
        }
        97 => {
            if xmlStrEqual(
                name,
                b"amp\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                return &mut xmlEntityAmp;
            }
            if xmlStrEqual(
                name,
                b"apos\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                return &mut xmlEntityApos;
            }
        }
        113 => {
            if xmlStrEqual(
                name,
                b"quot\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            ) != 0
            {
                return &mut xmlEntityQuot;
            }
        }
        _ => {}
    }
    return 0 as xmlEntityPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddDtdEntity(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlEntityPtr {
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    if doc.is_null() {
        xmlEntitiesErr(
            XML_DTD_NO_DOC,
            b"xmlAddDtdEntity: document is NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlEntityPtr;
    }
    if ((*doc).extSubset).is_null() {
        xmlEntitiesErr(
            XML_DTD_NO_DTD,
            b"xmlAddDtdEntity: document without external subset\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as xmlEntityPtr;
    }
    dtd = (*doc).extSubset;
    ret = xmlAddEntity(dtd, name, type_0, ExternalID, SystemID, content);
    if ret.is_null() {
        return 0 as xmlEntityPtr;
    }
    (*ret).parent = dtd;
    (*ret).doc = (*dtd).doc;
    if ((*dtd).last).is_null() {
        (*dtd).last = ret as xmlNodePtr;
        (*dtd).children = (*dtd).last;
    } else {
        (*(*dtd).last).next = ret as xmlNodePtr;
        (*ret).prev = (*dtd).last;
        (*dtd).last = ret as xmlNodePtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddDocEntity(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlEntityPtr {
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
    if doc.is_null() {
        xmlEntitiesErr(
            XML_DTD_NO_DOC,
            b"xmlAddDocEntity: document is NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlEntityPtr;
    }
    if ((*doc).intSubset).is_null() {
        xmlEntitiesErr(
            XML_DTD_NO_DTD,
            b"xmlAddDocEntity: document without internal subset\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as xmlEntityPtr;
    }
    dtd = (*doc).intSubset;
    ret = xmlAddEntity(dtd, name, type_0, ExternalID, SystemID, content);
    if ret.is_null() {
        return 0 as xmlEntityPtr;
    }
    (*ret).parent = dtd;
    (*ret).doc = (*dtd).doc;
    if ((*dtd).last).is_null() {
        (*dtd).last = ret as xmlNodePtr;
        (*dtd).children = (*dtd).last;
    } else {
        (*(*dtd).last).next = ret as xmlNodePtr;
        (*ret).prev = (*dtd).last;
        (*dtd).last = ret as xmlNodePtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewEntity(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlEntityPtr {
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if !doc.is_null() && !((*doc).intSubset).is_null() {
        return xmlAddDocEntity(doc, name, type_0, ExternalID, SystemID, content);
    }
    if !doc.is_null() {
        dict = (*doc).dict;
    } else {
        dict = 0 as xmlDictPtr;
    }
    ret = xmlCreateEntity(dict, name, type_0, ExternalID, SystemID, content);
    if ret.is_null() {
        return 0 as xmlEntityPtr;
    }
    (*ret).doc = doc;
    return ret;
}
unsafe extern "C" fn xmlGetEntityFromTable(
    mut table: xmlEntitiesTablePtr,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    return xmlHashLookup(table, name) as xmlEntityPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetParameterEntity(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut table: xmlEntitiesTablePtr = 0 as *mut xmlEntitiesTable;
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    if doc.is_null() {
        return 0 as xmlEntityPtr;
    }
    if !((*doc).intSubset).is_null() && !((*(*doc).intSubset).pentities).is_null() {
        table = (*(*doc).intSubset).pentities as xmlEntitiesTablePtr;
        ret = xmlGetEntityFromTable(table, name);
        if !ret.is_null() {
            return ret;
        }
    }
    if !((*doc).extSubset).is_null() && !((*(*doc).extSubset).pentities).is_null() {
        table = (*(*doc).extSubset).pentities as xmlEntitiesTablePtr;
        return xmlGetEntityFromTable(table, name);
    }
    return 0 as xmlEntityPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdEntity(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut table: xmlEntitiesTablePtr = 0 as *mut xmlEntitiesTable;
    if doc.is_null() {
        return 0 as xmlEntityPtr;
    }
    if !((*doc).extSubset).is_null() && !((*(*doc).extSubset).entities).is_null() {
        table = (*(*doc).extSubset).entities as xmlEntitiesTablePtr;
        return xmlGetEntityFromTable(table, name);
    }
    return 0 as xmlEntityPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDocEntity(
    mut doc: *const xmlDoc,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut cur: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut table: xmlEntitiesTablePtr = 0 as *mut xmlEntitiesTable;
    if !doc.is_null() {
        if !((*doc).intSubset).is_null() && !((*(*doc).intSubset).entities).is_null() {
            table = (*(*doc).intSubset).entities as xmlEntitiesTablePtr;
            cur = xmlGetEntityFromTable(table, name);
            if !cur.is_null() {
                return cur;
            }
        }
        if (*doc).standalone != 1 as libc::c_int {
            if !((*doc).extSubset).is_null() && !((*(*doc).extSubset).entities).is_null()
            {
                table = (*(*doc).extSubset).entities as xmlEntitiesTablePtr;
                cur = xmlGetEntityFromTable(table, name);
                if !cur.is_null() {
                    return cur;
                }
            }
        }
    }
    return xmlGetPredefinedEntity(name);
}
unsafe extern "C" fn xmlEncodeEntitiesInternal(
    mut doc: xmlDocPtr,
    mut input: *const xmlChar,
    mut attr: libc::c_int,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut cur: *const xmlChar = input;
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer_size: size_t = 0 as libc::c_int as size_t;
    let mut html: libc::c_int = 0 as libc::c_int;
    if input.is_null() {
        return 0 as *mut xmlChar;
    }
    if !doc.is_null() {
        html = ((*doc).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint) as libc::c_int;
    }
    buffer_size = 1000 as libc::c_int as size_t;
    buffer = xmlMalloc
        .expect(
            "non-null function pointer",
        )(buffer_size.wrapping_mul(::core::mem::size_of::<xmlChar>() as libc::c_ulong))
        as *mut xmlChar;
    if buffer.is_null() {
        xmlEntitiesErrMemory(
            b"xmlEncodeEntities: malloc failed\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut xmlChar;
    }
    out = buffer;
    's_48: loop {
        if !(*cur as libc::c_int != '\0' as i32) {
            current_block = 12065775993741208975;
            break;
        }
        let mut indx: size_t = out.offset_from(buffer) as libc::c_long as size_t;
        if indx.wrapping_add(100 as libc::c_int as size_t) > buffer_size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            let mut new_size: size_t = buffer_size * 2 as libc::c_int as size_t;
            if new_size < buffer_size {
                current_block = 16055609613819027732;
                break;
            }
            tmp = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(buffer as *mut libc::c_void, new_size) as *mut xmlChar;
            if tmp.is_null() {
                current_block = 16055609613819027732;
                break;
            }
            buffer = tmp;
            buffer_size = new_size;
            out = &mut *buffer.offset(indx as isize) as *mut xmlChar;
        }
        if *cur as libc::c_int == '<' as i32 {
            let mut end: *const xmlChar = 0 as *const xmlChar;
            if html != 0 && attr != 0
                && *cur.offset(1 as libc::c_int as isize) as libc::c_int == '!' as i32
                && *cur.offset(2 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *cur.offset(3 as libc::c_int as isize) as libc::c_int == '-' as i32
                && {
                    end = xmlStrstr(
                        cur,
                        b"-->\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    );
                    !end.is_null()
                }
            {
                while cur != end {
                    let fresh0 = cur;
                    cur = cur.offset(1);
                    let fresh1 = out;
                    out = out.offset(1);
                    *fresh1 = *fresh0;
                    indx = out.offset_from(buffer) as libc::c_long as size_t;
                    if !(indx.wrapping_add(100 as libc::c_int as size_t) > buffer_size) {
                        continue;
                    }
                    let mut tmp_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut new_size_0: size_t = buffer_size
                        * 2 as libc::c_int as size_t;
                    if new_size_0 < buffer_size {
                        current_block = 16055609613819027732;
                        break 's_48;
                    }
                    tmp_0 = xmlRealloc
                        .expect(
                            "non-null function pointer",
                        )(buffer as *mut libc::c_void, new_size_0) as *mut xmlChar;
                    if tmp_0.is_null() {
                        current_block = 16055609613819027732;
                        break 's_48;
                    }
                    buffer = tmp_0;
                    buffer_size = new_size_0;
                    out = &mut *buffer.offset(indx as isize) as *mut xmlChar;
                }
                let fresh2 = cur;
                cur = cur.offset(1);
                let fresh3 = out;
                out = out.offset(1);
                *fresh3 = *fresh2;
                let fresh4 = cur;
                cur = cur.offset(1);
                let fresh5 = out;
                out = out.offset(1);
                *fresh5 = *fresh4;
                let fresh6 = cur;
                cur = cur.offset(1);
                let fresh7 = out;
                out = out.offset(1);
                *fresh7 = *fresh6;
                continue;
            } else {
                let fresh8 = out;
                out = out.offset(1);
                *fresh8 = '&' as i32 as xmlChar;
                let fresh9 = out;
                out = out.offset(1);
                *fresh9 = 'l' as i32 as xmlChar;
                let fresh10 = out;
                out = out.offset(1);
                *fresh10 = 't' as i32 as xmlChar;
                let fresh11 = out;
                out = out.offset(1);
                *fresh11 = ';' as i32 as xmlChar;
            }
        } else if *cur as libc::c_int == '>' as i32 {
            let fresh12 = out;
            out = out.offset(1);
            *fresh12 = '&' as i32 as xmlChar;
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = 'g' as i32 as xmlChar;
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = 't' as i32 as xmlChar;
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = ';' as i32 as xmlChar;
        } else if *cur as libc::c_int == '&' as i32 {
            if html != 0 && attr != 0
                && *cur.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
                && !(strchr(cur as *const libc::c_char, '}' as i32)).is_null()
            {
                while *cur as libc::c_int != '}' as i32 {
                    let fresh16 = cur;
                    cur = cur.offset(1);
                    let fresh17 = out;
                    out = out.offset(1);
                    *fresh17 = *fresh16;
                    indx = out.offset_from(buffer) as libc::c_long as size_t;
                    if !(indx.wrapping_add(100 as libc::c_int as size_t) > buffer_size) {
                        continue;
                    }
                    let mut tmp_1: *mut xmlChar = 0 as *mut xmlChar;
                    let mut new_size_1: size_t = buffer_size
                        * 2 as libc::c_int as size_t;
                    if new_size_1 < buffer_size {
                        current_block = 16055609613819027732;
                        break 's_48;
                    }
                    tmp_1 = xmlRealloc
                        .expect(
                            "non-null function pointer",
                        )(buffer as *mut libc::c_void, new_size_1) as *mut xmlChar;
                    if tmp_1.is_null() {
                        current_block = 16055609613819027732;
                        break 's_48;
                    }
                    buffer = tmp_1;
                    buffer_size = new_size_1;
                    out = &mut *buffer.offset(indx as isize) as *mut xmlChar;
                }
                let fresh18 = cur;
                cur = cur.offset(1);
                let fresh19 = out;
                out = out.offset(1);
                *fresh19 = *fresh18;
                continue;
            } else {
                let fresh20 = out;
                out = out.offset(1);
                *fresh20 = '&' as i32 as xmlChar;
                let fresh21 = out;
                out = out.offset(1);
                *fresh21 = 'a' as i32 as xmlChar;
                let fresh22 = out;
                out = out.offset(1);
                *fresh22 = 'm' as i32 as xmlChar;
                let fresh23 = out;
                out = out.offset(1);
                *fresh23 = 'p' as i32 as xmlChar;
                let fresh24 = out;
                out = out.offset(1);
                *fresh24 = ';' as i32 as xmlChar;
            }
        } else if *cur as libc::c_int >= 0x20 as libc::c_int
            && (*cur as libc::c_int) < 0x80 as libc::c_int
            || *cur as libc::c_int == '\n' as i32 || *cur as libc::c_int == '\t' as i32
            || html != 0 && *cur as libc::c_int == '\r' as i32
        {
            let fresh25 = out;
            out = out.offset(1);
            *fresh25 = *cur;
        } else if *cur as libc::c_int >= 0x80 as libc::c_int {
            if !doc.is_null() && !((*doc).encoding).is_null() || html != 0 {
                let fresh26 = out;
                out = out.offset(1);
                *fresh26 = *cur;
            } else {
                let mut buf: [libc::c_char; 11] = [0; 11];
                let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut val: libc::c_int = 0 as libc::c_int;
                let mut l: libc::c_int = 1 as libc::c_int;
                if (*cur as libc::c_int) < 0xc0 as libc::c_int {
                    xmlEntitiesErr(
                        XML_CHECK_NOT_UTF8,
                        b"xmlEncodeEntities: input not UTF-8\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !doc.is_null() {
                        (*doc)
                            .encoding = xmlStrdup(
                            b"ISO-8859-1\0" as *const u8 as *const libc::c_char
                                as *mut xmlChar,
                        );
                    }
                    snprintf(
                        buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
                        b"&#%d;\0" as *const u8 as *const libc::c_char,
                        *cur as libc::c_int,
                    );
                    buf[(::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as usize] = 0 as libc::c_int as libc::c_char;
                    ptr = buf.as_mut_ptr();
                    while *ptr as libc::c_int != 0 as libc::c_int {
                        let fresh27 = ptr;
                        ptr = ptr.offset(1);
                        let fresh28 = out;
                        out = out.offset(1);
                        *fresh28 = *fresh27 as xmlChar;
                    }
                    cur = cur.offset(1);
                    cur;
                    continue;
                } else {
                    if (*cur as libc::c_int) < 0xe0 as libc::c_int {
                        val = *cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x1f as libc::c_int;
                        val <<= 6 as libc::c_int;
                        val
                            |= *cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int;
                        l = 2 as libc::c_int;
                    } else if (*cur as libc::c_int) < 0xf0 as libc::c_int {
                        val = *cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0xf as libc::c_int;
                        val <<= 6 as libc::c_int;
                        val
                            |= *cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int;
                        val <<= 6 as libc::c_int;
                        val
                            |= *cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int;
                        l = 3 as libc::c_int;
                    } else if (*cur as libc::c_int) < 0xf8 as libc::c_int {
                        val = *cur.offset(0 as libc::c_int as isize) as libc::c_int
                            & 0x7 as libc::c_int;
                        val <<= 6 as libc::c_int;
                        val
                            |= *cur.offset(1 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int;
                        val <<= 6 as libc::c_int;
                        val
                            |= *cur.offset(2 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int;
                        val <<= 6 as libc::c_int;
                        val
                            |= *cur.offset(3 as libc::c_int as isize) as libc::c_int
                                & 0x3f as libc::c_int;
                        l = 4 as libc::c_int;
                    }
                    if l == 1 as libc::c_int
                        || (if val < 0x100 as libc::c_int {
                            (0x9 as libc::c_int <= val && val <= 0xa as libc::c_int
                                || val == 0xd as libc::c_int || 0x20 as libc::c_int <= val)
                                as libc::c_int
                        } else {
                            (0x100 as libc::c_int <= val && val <= 0xd7ff as libc::c_int
                                || 0xe000 as libc::c_int <= val
                                    && val <= 0xfffd as libc::c_int
                                || 0x10000 as libc::c_int <= val
                                    && val <= 0x10ffff as libc::c_int) as libc::c_int
                        }) == 0
                    {
                        xmlEntitiesErr(
                            XML_ERR_INVALID_CHAR,
                            b"xmlEncodeEntities: char out of range\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        if !doc.is_null() {
                            (*doc)
                                .encoding = xmlStrdup(
                                b"ISO-8859-1\0" as *const u8 as *const libc::c_char
                                    as *mut xmlChar,
                            );
                        }
                        snprintf(
                            buf.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 11]>()
                                as libc::c_ulong,
                            b"&#%d;\0" as *const u8 as *const libc::c_char,
                            *cur as libc::c_int,
                        );
                        buf[(::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as usize] = 0 as libc::c_int as libc::c_char;
                        ptr = buf.as_mut_ptr();
                        while *ptr as libc::c_int != 0 as libc::c_int {
                            let fresh29 = ptr;
                            ptr = ptr.offset(1);
                            let fresh30 = out;
                            out = out.offset(1);
                            *fresh30 = *fresh29 as xmlChar;
                        }
                        cur = cur.offset(1);
                        cur;
                        continue;
                    } else {
                        snprintf(
                            buf.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 11]>()
                                as libc::c_ulong,
                            b"&#x%X;\0" as *const u8 as *const libc::c_char,
                            val,
                        );
                        buf[(::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as usize] = 0 as libc::c_int as libc::c_char;
                        ptr = buf.as_mut_ptr();
                        while *ptr as libc::c_int != 0 as libc::c_int {
                            let fresh31 = ptr;
                            ptr = ptr.offset(1);
                            let fresh32 = out;
                            out = out.offset(1);
                            *fresh32 = *fresh31 as xmlChar;
                        }
                        cur = cur.offset(l as isize);
                        continue;
                    }
                }
            }
        } else if 0x9 as libc::c_int <= *cur as libc::c_int
            && *cur as libc::c_int <= 0xa as libc::c_int
            || *cur as libc::c_int == 0xd as libc::c_int
            || 0x20 as libc::c_int <= *cur as libc::c_int
        {
            let mut buf_0: [libc::c_char; 11] = [0; 11];
            let mut ptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
            snprintf(
                buf_0.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
                b"&#%d;\0" as *const u8 as *const libc::c_char,
                *cur as libc::c_int,
            );
            buf_0[(::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = 0 as libc::c_int as libc::c_char;
            ptr_0 = buf_0.as_mut_ptr();
            while *ptr_0 as libc::c_int != 0 as libc::c_int {
                let fresh33 = ptr_0;
                ptr_0 = ptr_0.offset(1);
                let fresh34 = out;
                out = out.offset(1);
                *fresh34 = *fresh33 as xmlChar;
            }
        }
        cur = cur.offset(1);
        cur;
    }
    match current_block {
        16055609613819027732 => {
            xmlEntitiesErrMemory(
                b"xmlEncodeEntities: realloc failed\0" as *const u8
                    as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(buffer as *mut libc::c_void);
            return 0 as *mut xmlChar;
        }
        _ => {
            *out = 0 as libc::c_int as xmlChar;
            return buffer;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlEncodeAttributeEntities(
    mut doc: xmlDocPtr,
    mut input: *const xmlChar,
) -> *mut xmlChar {
    return xmlEncodeEntitiesInternal(doc, input, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlEncodeEntitiesReentrant(
    mut doc: xmlDocPtr,
    mut input: *const xmlChar,
) -> *mut xmlChar {
    return xmlEncodeEntitiesInternal(doc, input, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlEncodeSpecialChars(
    mut doc: *const xmlDoc,
    mut input: *const xmlChar,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut cur: *const xmlChar = input;
    let mut buffer: *mut xmlChar = 0 as *mut xmlChar;
    let mut out: *mut xmlChar = 0 as *mut xmlChar;
    let mut buffer_size: size_t = 0 as libc::c_int as size_t;
    if input.is_null() {
        return 0 as *mut xmlChar;
    }
    buffer_size = 1000 as libc::c_int as size_t;
    buffer = xmlMalloc
        .expect(
            "non-null function pointer",
        )(buffer_size.wrapping_mul(::core::mem::size_of::<xmlChar>() as libc::c_ulong))
        as *mut xmlChar;
    if buffer.is_null() {
        xmlEntitiesErrMemory(
            b"xmlEncodeSpecialChars: malloc failed\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut xmlChar;
    }
    out = buffer;
    loop {
        if !(*cur as libc::c_int != '\0' as i32) {
            current_block = 12997042908615822766;
            break;
        }
        let mut indx: size_t = out.offset_from(buffer) as libc::c_long as size_t;
        if indx.wrapping_add(10 as libc::c_int as size_t) > buffer_size {
            let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
            let mut new_size: size_t = buffer_size * 2 as libc::c_int as size_t;
            if new_size < buffer_size {
                current_block = 13841710423951159019;
                break;
            }
            tmp = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(buffer as *mut libc::c_void, new_size) as *mut xmlChar;
            if tmp.is_null() {
                current_block = 13841710423951159019;
                break;
            }
            buffer = tmp;
            buffer_size = new_size;
            out = &mut *buffer.offset(indx as isize) as *mut xmlChar;
        }
        if *cur as libc::c_int == '<' as i32 {
            let fresh35 = out;
            out = out.offset(1);
            *fresh35 = '&' as i32 as xmlChar;
            let fresh36 = out;
            out = out.offset(1);
            *fresh36 = 'l' as i32 as xmlChar;
            let fresh37 = out;
            out = out.offset(1);
            *fresh37 = 't' as i32 as xmlChar;
            let fresh38 = out;
            out = out.offset(1);
            *fresh38 = ';' as i32 as xmlChar;
        } else if *cur as libc::c_int == '>' as i32 {
            let fresh39 = out;
            out = out.offset(1);
            *fresh39 = '&' as i32 as xmlChar;
            let fresh40 = out;
            out = out.offset(1);
            *fresh40 = 'g' as i32 as xmlChar;
            let fresh41 = out;
            out = out.offset(1);
            *fresh41 = 't' as i32 as xmlChar;
            let fresh42 = out;
            out = out.offset(1);
            *fresh42 = ';' as i32 as xmlChar;
        } else if *cur as libc::c_int == '&' as i32 {
            let fresh43 = out;
            out = out.offset(1);
            *fresh43 = '&' as i32 as xmlChar;
            let fresh44 = out;
            out = out.offset(1);
            *fresh44 = 'a' as i32 as xmlChar;
            let fresh45 = out;
            out = out.offset(1);
            *fresh45 = 'm' as i32 as xmlChar;
            let fresh46 = out;
            out = out.offset(1);
            *fresh46 = 'p' as i32 as xmlChar;
            let fresh47 = out;
            out = out.offset(1);
            *fresh47 = ';' as i32 as xmlChar;
        } else if *cur as libc::c_int == '"' as i32 {
            let fresh48 = out;
            out = out.offset(1);
            *fresh48 = '&' as i32 as xmlChar;
            let fresh49 = out;
            out = out.offset(1);
            *fresh49 = 'q' as i32 as xmlChar;
            let fresh50 = out;
            out = out.offset(1);
            *fresh50 = 'u' as i32 as xmlChar;
            let fresh51 = out;
            out = out.offset(1);
            *fresh51 = 'o' as i32 as xmlChar;
            let fresh52 = out;
            out = out.offset(1);
            *fresh52 = 't' as i32 as xmlChar;
            let fresh53 = out;
            out = out.offset(1);
            *fresh53 = ';' as i32 as xmlChar;
        } else if *cur as libc::c_int == '\r' as i32 {
            let fresh54 = out;
            out = out.offset(1);
            *fresh54 = '&' as i32 as xmlChar;
            let fresh55 = out;
            out = out.offset(1);
            *fresh55 = '#' as i32 as xmlChar;
            let fresh56 = out;
            out = out.offset(1);
            *fresh56 = '1' as i32 as xmlChar;
            let fresh57 = out;
            out = out.offset(1);
            *fresh57 = '3' as i32 as xmlChar;
            let fresh58 = out;
            out = out.offset(1);
            *fresh58 = ';' as i32 as xmlChar;
        } else {
            let fresh59 = out;
            out = out.offset(1);
            *fresh59 = *cur;
        }
        cur = cur.offset(1);
        cur;
    }
    match current_block {
        13841710423951159019 => {
            xmlEntitiesErrMemory(
                b"xmlEncodeSpecialChars: realloc failed\0" as *const u8
                    as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(buffer as *mut libc::c_void);
            return 0 as *mut xmlChar;
        }
        _ => {
            *out = 0 as libc::c_int as xmlChar;
            return buffer;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateEntitiesTable() -> xmlEntitiesTablePtr {
    return xmlHashCreate(0 as libc::c_int) as xmlEntitiesTablePtr;
}
unsafe extern "C" fn xmlFreeEntityWrapper(
    mut entity: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    if !entity.is_null() {
        xmlFreeEntity(entity as xmlEntityPtr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeEntitiesTable(mut table: xmlEntitiesTablePtr) {
    xmlHashFree(
        table,
        Some(
            xmlFreeEntityWrapper
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
}
unsafe extern "C" fn xmlCopyEntity(
    mut payload: *mut libc::c_void,
    mut name: *const xmlChar,
) -> *mut libc::c_void {
    let mut ent: xmlEntityPtr = payload as xmlEntityPtr;
    let mut cur: xmlEntityPtr = 0 as *mut xmlEntity;
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlEntity>() as libc::c_ulong) as xmlEntityPtr;
    if cur.is_null() {
        xmlEntitiesErrMemory(
            b"xmlCopyEntity:: malloc failed\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlEntity>() as libc::c_ulong,
    );
    (*cur).type_0 = XML_ENTITY_DECL;
    (*cur).etype = (*ent).etype;
    if !((*ent).name).is_null() {
        (*cur).name = xmlStrdup((*ent).name);
    }
    if !((*ent).ExternalID).is_null() {
        (*cur).ExternalID = xmlStrdup((*ent).ExternalID);
    }
    if !((*ent).SystemID).is_null() {
        (*cur).SystemID = xmlStrdup((*ent).SystemID);
    }
    if !((*ent).content).is_null() {
        (*cur).content = xmlStrdup((*ent).content);
    }
    if !((*ent).orig).is_null() {
        (*cur).orig = xmlStrdup((*ent).orig);
    }
    if !((*ent).URI).is_null() {
        (*cur).URI = xmlStrdup((*ent).URI);
    }
    return cur as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyEntitiesTable(
    mut table: xmlEntitiesTablePtr,
) -> xmlEntitiesTablePtr {
    return xmlHashCopy(
        table,
        Some(
            xmlCopyEntity
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                ) -> *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn xmlDumpEntityContent(
    mut buf: xmlBufferPtr,
    mut content: *const xmlChar,
) {
    if (*buf).alloc as libc::c_uint
        == XML_BUFFER_ALLOC_IMMUTABLE as libc::c_int as libc::c_uint
    {
        return;
    }
    if !(xmlStrchr(content, '%' as i32 as xmlChar)).is_null() {
        let mut base: *const xmlChar = 0 as *const xmlChar;
        let mut cur: *const xmlChar = 0 as *const xmlChar;
        xmlBufferCCat(buf, b"\"\0" as *const u8 as *const libc::c_char);
        cur = content;
        base = cur;
        while *cur as libc::c_int != 0 as libc::c_int {
            if *cur as libc::c_int == '"' as i32 {
                if base != cur {
                    xmlBufferAdd(
                        buf,
                        base,
                        cur.offset_from(base) as libc::c_long as libc::c_int,
                    );
                }
                xmlBufferAdd(
                    buf,
                    b"&quot;\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    6 as libc::c_int,
                );
                cur = cur.offset(1);
                cur;
                base = cur;
            } else if *cur as libc::c_int == '%' as i32 {
                if base != cur {
                    xmlBufferAdd(
                        buf,
                        base,
                        cur.offset_from(base) as libc::c_long as libc::c_int,
                    );
                }
                xmlBufferAdd(
                    buf,
                    b"&#x25;\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                    6 as libc::c_int,
                );
                cur = cur.offset(1);
                cur;
                base = cur;
            } else {
                cur = cur.offset(1);
                cur;
            }
        }
        if base != cur {
            xmlBufferAdd(
                buf,
                base,
                cur.offset_from(base) as libc::c_long as libc::c_int,
            );
        }
        xmlBufferCCat(buf, b"\"\0" as *const u8 as *const libc::c_char);
    } else {
        xmlBufferWriteQuotedString(buf, content);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpEntityDecl(
    mut buf: xmlBufferPtr,
    mut ent: xmlEntityPtr,
) {
    if buf.is_null() || ent.is_null() {
        return;
    }
    match (*ent).etype as libc::c_uint {
        1 => {
            xmlBufferWriteChar(buf, b"<!ENTITY \0" as *const u8 as *const libc::c_char);
            xmlBufferWriteCHAR(buf, (*ent).name);
            xmlBufferWriteChar(buf, b" \0" as *const u8 as *const libc::c_char);
            if !((*ent).orig).is_null() {
                xmlBufferWriteQuotedString(buf, (*ent).orig);
            } else {
                xmlDumpEntityContent(buf, (*ent).content);
            }
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            xmlBufferWriteChar(buf, b"<!ENTITY \0" as *const u8 as *const libc::c_char);
            xmlBufferWriteCHAR(buf, (*ent).name);
            if !((*ent).ExternalID).is_null() {
                xmlBufferWriteChar(
                    buf,
                    b" PUBLIC \0" as *const u8 as *const libc::c_char,
                );
                xmlBufferWriteQuotedString(buf, (*ent).ExternalID);
                xmlBufferWriteChar(buf, b" \0" as *const u8 as *const libc::c_char);
                xmlBufferWriteQuotedString(buf, (*ent).SystemID);
            } else {
                xmlBufferWriteChar(
                    buf,
                    b" SYSTEM \0" as *const u8 as *const libc::c_char,
                );
                xmlBufferWriteQuotedString(buf, (*ent).SystemID);
            }
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            xmlBufferWriteChar(buf, b"<!ENTITY \0" as *const u8 as *const libc::c_char);
            xmlBufferWriteCHAR(buf, (*ent).name);
            if !((*ent).ExternalID).is_null() {
                xmlBufferWriteChar(
                    buf,
                    b" PUBLIC \0" as *const u8 as *const libc::c_char,
                );
                xmlBufferWriteQuotedString(buf, (*ent).ExternalID);
                xmlBufferWriteChar(buf, b" \0" as *const u8 as *const libc::c_char);
                xmlBufferWriteQuotedString(buf, (*ent).SystemID);
            } else {
                xmlBufferWriteChar(
                    buf,
                    b" SYSTEM \0" as *const u8 as *const libc::c_char,
                );
                xmlBufferWriteQuotedString(buf, (*ent).SystemID);
            }
            if !((*ent).content).is_null() {
                xmlBufferWriteChar(
                    buf,
                    b" NDATA \0" as *const u8 as *const libc::c_char,
                );
                if !((*ent).orig).is_null() {
                    xmlBufferWriteCHAR(buf, (*ent).orig);
                } else {
                    xmlBufferWriteCHAR(buf, (*ent).content);
                }
            }
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const libc::c_char);
        }
        4 => {
            xmlBufferWriteChar(
                buf,
                b"<!ENTITY % \0" as *const u8 as *const libc::c_char,
            );
            xmlBufferWriteCHAR(buf, (*ent).name);
            xmlBufferWriteChar(buf, b" \0" as *const u8 as *const libc::c_char);
            if ((*ent).orig).is_null() {
                xmlDumpEntityContent(buf, (*ent).content);
            } else {
                xmlBufferWriteQuotedString(buf, (*ent).orig);
            }
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const libc::c_char);
        }
        5 => {
            xmlBufferWriteChar(
                buf,
                b"<!ENTITY % \0" as *const u8 as *const libc::c_char,
            );
            xmlBufferWriteCHAR(buf, (*ent).name);
            if !((*ent).ExternalID).is_null() {
                xmlBufferWriteChar(
                    buf,
                    b" PUBLIC \0" as *const u8 as *const libc::c_char,
                );
                xmlBufferWriteQuotedString(buf, (*ent).ExternalID);
                xmlBufferWriteChar(buf, b" \0" as *const u8 as *const libc::c_char);
                xmlBufferWriteQuotedString(buf, (*ent).SystemID);
            } else {
                xmlBufferWriteChar(
                    buf,
                    b" SYSTEM \0" as *const u8 as *const libc::c_char,
                );
                xmlBufferWriteQuotedString(buf, (*ent).SystemID);
            }
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            xmlEntitiesErr(
                XML_DTD_UNKNOWN_ENTITY,
                b"xmlDumpEntitiesDecl: internal: unknown type entity type\0" as *const u8
                    as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn xmlDumpEntityDeclScan(
    mut ent: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    xmlDumpEntityDecl(buf as xmlBufferPtr, ent as xmlEntityPtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpEntitiesTable(
    mut buf: xmlBufferPtr,
    mut table: xmlEntitiesTablePtr,
) {
    xmlHashScan(
        table,
        Some(
            xmlDumpEntityDeclScan
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *const xmlChar,
                ) -> (),
        ),
        buf as *mut libc::c_void,
    );
}
