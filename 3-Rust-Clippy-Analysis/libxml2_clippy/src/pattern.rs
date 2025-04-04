use ::libc;
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xmlDictReference(dict: xmlDictPtr) -> libc::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: libc::c_int,
    ) -> *const xmlChar;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlCharInRange(val: libc::c_uint, group: *const xmlChRangeGroup) -> libc::c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsCombiningGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsExtenderGroup: xmlChRangeGroup;
    fn xmlStringCurrentChar(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        len: *mut libc::c_int,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: libc::c_ushort,
    pub high: libc::c_ushort,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: libc::c_uint,
    pub high: libc::c_uint,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: libc::c_int,
    pub nbLongRange: libc::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlPattern {
    pub data: *mut libc::c_void,
    pub dict: xmlDictPtr,
    pub next: *mut _xmlPattern,
    pub pattern: *const xmlChar,
    pub flags: libc::c_int,
    pub nbStep: libc::c_int,
    pub maxStep: libc::c_int,
    pub steps: xmlStepOpPtr,
    pub stream: xmlStreamCompPtr,
}
pub type xmlStreamCompPtr = *mut xmlStreamComp;
pub type xmlStreamComp = _xmlStreamComp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamComp {
    pub dict: *mut xmlDict,
    pub nbStep: libc::c_int,
    pub maxStep: libc::c_int,
    pub steps: xmlStreamStepPtr,
    pub flags: libc::c_int,
}
pub type xmlStreamStepPtr = *mut xmlStreamStep;
pub type xmlStreamStep = _xmlStreamStep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamStep {
    pub flags: libc::c_int,
    pub name: *const xmlChar,
    pub ns: *const xmlChar,
    pub nodeType: libc::c_int,
}
pub type xmlStepOpPtr = *mut xmlStepOp;
pub type xmlStepOp = _xmlStepOp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepOp {
    pub op: xmlPatOp,
    pub value: *const xmlChar,
    pub value2: *const xmlChar,
}
pub type xmlPatOp = libc::c_uint;
pub const XML_OP_ALL: xmlPatOp = 8;
pub const XML_OP_NS: xmlPatOp = 7;
pub const XML_OP_ANCESTOR: xmlPatOp = 6;
pub const XML_OP_PARENT: xmlPatOp = 5;
pub const XML_OP_ATTR: xmlPatOp = 4;
pub const XML_OP_CHILD: xmlPatOp = 3;
pub const XML_OP_ELEM: xmlPatOp = 2;
pub const XML_OP_ROOT: xmlPatOp = 1;
pub const XML_OP_END: xmlPatOp = 0;
pub type xmlPattern = _xmlPattern;
pub type xmlPatternPtr = *mut xmlPattern;
pub type C2RustUnnamed = libc::c_uint;
pub const XML_PATTERN_XSFIELD: C2RustUnnamed = 4;
pub const XML_PATTERN_XSSEL: C2RustUnnamed = 2;
pub const XML_PATTERN_XPATH: C2RustUnnamed = 1;
pub const XML_PATTERN_DEFAULT: C2RustUnnamed = 0;
pub type xmlPatParserContextPtr = *mut xmlPatParserContext;
pub type xmlPatParserContext = _xmlPatParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlPatParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: libc::c_int,
    pub dict: xmlDictPtr,
    pub comp: xmlPatternPtr,
    pub elem: xmlNodePtr,
    pub namespaces: *mut *const xmlChar,
    pub nb_namespaces: libc::c_int,
}
pub type xmlStepStatePtr = *mut xmlStepState;
pub type xmlStepState = _xmlStepState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepState {
    pub step: libc::c_int,
    pub node: xmlNodePtr,
}
pub type xmlStepStates = _xmlStepStates;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepStates {
    pub nbstates: libc::c_int,
    pub maxstates: libc::c_int,
    pub states: xmlStepStatePtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamCtxt {
    pub next: *mut _xmlStreamCtxt,
    pub comp: xmlStreamCompPtr,
    pub nbState: libc::c_int,
    pub maxState: libc::c_int,
    pub level: libc::c_int,
    pub states: *mut libc::c_int,
    pub flags: libc::c_int,
    pub blockLevel: libc::c_int,
}
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;
unsafe extern "C" fn xmlNewPattern() -> xmlPatternPtr {
    let mut cur: xmlPatternPtr = 0 as *mut xmlPattern;
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlPattern>() as libc::c_ulong) as xmlPatternPtr;
    if cur.is_null() {
        return 0 as xmlPatternPtr;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlPattern>() as libc::c_ulong,
    );
    (*cur).maxStep = 10 as libc::c_int;
    (*cur)
        .steps = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*cur).maxStep as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xmlStepOp>() as libc::c_ulong),
    ) as xmlStepOpPtr;
    if ((*cur).steps).is_null() {
        xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
        return 0 as xmlPatternPtr;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreePattern(mut comp: xmlPatternPtr) {
    let mut op: xmlStepOpPtr = 0 as *mut xmlStepOp;
    let mut i: libc::c_int = 0;
    if comp.is_null() {
        return;
    }
    if !((*comp).next).is_null() {
        xmlFreePattern((*comp).next);
    }
    if !((*comp).stream).is_null() {
        xmlFreeStreamComp((*comp).stream);
    }
    if !((*comp).pattern).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*comp).pattern as *mut xmlChar as *mut libc::c_void);
    }
    if !((*comp).steps).is_null() {
        if ((*comp).dict).is_null() {
            i = 0 as libc::c_int;
            while i < (*comp).nbStep {
                op = &mut *((*comp).steps).offset(i as isize) as *mut xmlStepOp;
                if !((*op).value).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*op).value as *mut xmlChar as *mut libc::c_void);
                }
                if !((*op).value2).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*op).value2 as *mut xmlChar as *mut libc::c_void);
                }
                i += 1;
                i;
            }
        }
        xmlFree.expect("non-null function pointer")((*comp).steps as *mut libc::c_void);
    }
    if !((*comp).dict).is_null() {
        xmlDictFree((*comp).dict);
    }
    memset(
        comp as *mut libc::c_void,
        -(1 as libc::c_int),
        ::core::mem::size_of::<xmlPattern>() as libc::c_ulong,
    );
    xmlFree.expect("non-null function pointer")(comp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreePatternList(mut comp: xmlPatternPtr) {
    let mut cur: xmlPatternPtr = 0 as *mut xmlPattern;
    while !comp.is_null() {
        cur = comp;
        comp = (*comp).next;
        (*cur).next = 0 as *mut _xmlPattern;
        xmlFreePattern(cur);
    }
}
unsafe extern "C" fn xmlNewPatParserContext(
    mut pattern: *const xmlChar,
    mut dict: xmlDictPtr,
    mut namespaces: *mut *const xmlChar,
) -> xmlPatParserContextPtr {
    let mut cur: xmlPatParserContextPtr = 0 as *mut xmlPatParserContext;
    if pattern.is_null() {
        return 0 as xmlPatParserContextPtr;
    }
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlPatParserContext>() as libc::c_ulong)
        as xmlPatParserContextPtr;
    if cur.is_null() {
        return 0 as xmlPatParserContextPtr;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlPatParserContext>() as libc::c_ulong,
    );
    (*cur).dict = dict;
    (*cur).cur = pattern;
    (*cur).base = pattern;
    if !namespaces.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*namespaces.offset((2 as libc::c_int * i) as isize)).is_null() {
            i += 1;
            i;
        }
        (*cur).nb_namespaces = i;
    } else {
        (*cur).nb_namespaces = 0 as libc::c_int;
    }
    (*cur).namespaces = namespaces;
    return cur;
}
unsafe extern "C" fn xmlFreePatParserContext(mut ctxt: xmlPatParserContextPtr) {
    if ctxt.is_null() {
        return;
    }
    memset(
        ctxt as *mut libc::c_void,
        -(1 as libc::c_int),
        ::core::mem::size_of::<xmlPatParserContext>() as libc::c_ulong,
    );
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn xmlPatternAdd(
    mut ctxt: xmlPatParserContextPtr,
    mut comp: xmlPatternPtr,
    mut op: xmlPatOp,
    mut value: *mut xmlChar,
    mut value2: *mut xmlChar,
) -> libc::c_int {
    if (*comp).nbStep >= (*comp).maxStep {
        let mut temp: xmlStepOpPtr = 0 as *mut xmlStepOp;
        temp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*comp).steps as *mut libc::c_void,
            (((*comp).maxStep * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlStepOp>() as libc::c_ulong),
        ) as xmlStepOpPtr;
        if temp.is_null() {
            return -(1 as libc::c_int);
        }
        (*comp).steps = temp;
        (*comp).maxStep *= 2 as libc::c_int;
    }
    (*((*comp).steps).offset((*comp).nbStep as isize)).op = op;
    let ref mut fresh0 = (*((*comp).steps).offset((*comp).nbStep as isize)).value;
    *fresh0 = value;
    let ref mut fresh1 = (*((*comp).steps).offset((*comp).nbStep as isize)).value2;
    *fresh1 = value2;
    (*comp).nbStep += 1;
    (*comp).nbStep;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlReversePattern(mut comp: xmlPatternPtr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*comp).nbStep > 0 as libc::c_int
        && (*((*comp).steps).offset(0 as libc::c_int as isize)).op as libc::c_uint
            == XML_OP_ANCESTOR as libc::c_int as libc::c_uint
    {
        i = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j < (*comp).nbStep {
            let ref mut fresh2 = (*((*comp).steps).offset(i as isize)).value;
            *fresh2 = (*((*comp).steps).offset(j as isize)).value;
            let ref mut fresh3 = (*((*comp).steps).offset(i as isize)).value2;
            *fresh3 = (*((*comp).steps).offset(j as isize)).value2;
            (*((*comp).steps).offset(i as isize))
                .op = (*((*comp).steps).offset(j as isize)).op;
            i += 1;
            i;
            j += 1;
            j;
        }
        (*comp).nbStep -= 1;
        (*comp).nbStep;
    }
    if (*comp).nbStep >= (*comp).maxStep {
        let mut temp: xmlStepOpPtr = 0 as *mut xmlStepOp;
        temp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*comp).steps as *mut libc::c_void,
            (((*comp).maxStep * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlStepOp>() as libc::c_ulong),
        ) as xmlStepOpPtr;
        if temp.is_null() {
            return -(1 as libc::c_int);
        }
        (*comp).steps = temp;
        (*comp).maxStep *= 2 as libc::c_int;
    }
    i = 0 as libc::c_int;
    j = (*comp).nbStep - 1 as libc::c_int;
    while j > i {
        let mut tmp: *const xmlChar = 0 as *const xmlChar;
        let mut op: xmlPatOp = XML_OP_END;
        tmp = (*((*comp).steps).offset(i as isize)).value;
        let ref mut fresh4 = (*((*comp).steps).offset(i as isize)).value;
        *fresh4 = (*((*comp).steps).offset(j as isize)).value;
        let ref mut fresh5 = (*((*comp).steps).offset(j as isize)).value;
        *fresh5 = tmp;
        tmp = (*((*comp).steps).offset(i as isize)).value2;
        let ref mut fresh6 = (*((*comp).steps).offset(i as isize)).value2;
        *fresh6 = (*((*comp).steps).offset(j as isize)).value2;
        let ref mut fresh7 = (*((*comp).steps).offset(j as isize)).value2;
        *fresh7 = tmp;
        op = (*((*comp).steps).offset(i as isize)).op;
        (*((*comp).steps).offset(i as isize))
            .op = (*((*comp).steps).offset(j as isize)).op;
        (*((*comp).steps).offset(j as isize)).op = op;
        j -= 1;
        j;
        i += 1;
        i;
    }
    let ref mut fresh8 = (*((*comp).steps).offset((*comp).nbStep as isize)).value;
    *fresh8 = 0 as *const xmlChar;
    let ref mut fresh9 = (*((*comp).steps).offset((*comp).nbStep as isize)).value2;
    *fresh9 = 0 as *const xmlChar;
    let fresh10 = (*comp).nbStep;
    (*comp).nbStep = (*comp).nbStep + 1;
    (*((*comp).steps).offset(fresh10 as isize)).op = XML_OP_END;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlPatPushState(
    mut states: *mut xmlStepStates,
    mut step: libc::c_int,
    mut node: xmlNodePtr,
) -> libc::c_int {
    if ((*states).states).is_null() || (*states).maxstates <= 0 as libc::c_int {
        (*states).maxstates = 4 as libc::c_int;
        (*states).nbstates = 0 as libc::c_int;
        (*states)
            .states = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlStepState>() as libc::c_ulong),
        ) as xmlStepStatePtr;
    } else if (*states).maxstates <= (*states).nbstates {
        let mut tmp: *mut xmlStepState = 0 as *mut xmlStepState;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*states).states as *mut libc::c_void,
            ((2 as libc::c_int * (*states).maxstates) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlStepState>() as libc::c_ulong),
        ) as xmlStepStatePtr;
        if tmp.is_null() {
            return -(1 as libc::c_int);
        }
        (*states).states = tmp;
        (*states).maxstates *= 2 as libc::c_int;
    }
    (*((*states).states).offset((*states).nbstates as isize)).step = step;
    let fresh11 = (*states).nbstates;
    (*states).nbstates = (*states).nbstates + 1;
    let ref mut fresh12 = (*((*states).states).offset(fresh11 as isize)).node;
    *fresh12 = node;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlPatMatch(
    mut comp: xmlPatternPtr,
    mut node: xmlNodePtr,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut step: xmlStepOpPtr = 0 as *mut xmlStepOp;
    let mut states: xmlStepStates = {
        let mut init = _xmlStepStates {
            nbstates: 0 as libc::c_int,
            maxstates: 0 as libc::c_int,
            states: 0 as xmlStepStatePtr,
        };
        init
    };
    if comp.is_null() || node.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*comp).nbStep {
        step = &mut *((*comp).steps).offset(i as isize) as *mut xmlStepOp;
        match (*step).op as libc::c_uint {
            0 => {
                break;
            }
            1 => {
                if (*node).type_0 as libc::c_uint
                    == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else {
                    node = (*node).parent;
                    if (*node).type_0 as libc::c_uint
                        == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                        || (*node).type_0 as libc::c_uint
                            == XML_DOCB_DOCUMENT_NODE as libc::c_int as libc::c_uint
                        || (*node).type_0 as libc::c_uint
                            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                    {
                        current_block = 4644295000439058019;
                    } else {
                        current_block = 9914136504317020720;
                    }
                }
            }
            2 => {
                if (*node).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else if ((*step).value).is_null() {
                    current_block = 4644295000439058019;
                } else if *((*step).value).offset(0 as libc::c_int as isize)
                    as libc::c_int
                    != *((*node).name).offset(0 as libc::c_int as isize) as libc::c_int
                {
                    current_block = 9914136504317020720;
                } else if xmlStrEqual((*step).value, (*node).name) == 0 {
                    current_block = 9914136504317020720;
                } else if ((*node).ns).is_null() {
                    if !((*step).value2).is_null() {
                        current_block = 9914136504317020720;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else if !((*(*node).ns).href).is_null() {
                    if ((*step).value2).is_null() {
                        current_block = 9914136504317020720;
                    } else if xmlStrEqual((*step).value2, (*(*node).ns).href) == 0 {
                        current_block = 9914136504317020720;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else {
                    current_block = 4644295000439058019;
                }
            }
            3 => {
                let mut lst: xmlNodePtr = 0 as *mut xmlNode;
                if (*node).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && (*node).type_0 as libc::c_uint
                        != XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                    && (*node).type_0 as libc::c_uint
                        != XML_DOCB_DOCUMENT_NODE as libc::c_int as libc::c_uint
                    && (*node).type_0 as libc::c_uint
                        != XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else {
                    lst = (*node).children;
                    if !((*step).value).is_null() {
                        while !lst.is_null() {
                            if (*lst).type_0 as libc::c_uint
                                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                                && *((*step).value).offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    == *((*lst).name).offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                && xmlStrEqual((*step).value, (*lst).name) != 0
                            {
                                break;
                            }
                            lst = (*lst).next;
                        }
                        if !lst.is_null() {
                            current_block = 4644295000439058019;
                        } else {
                            current_block = 9914136504317020720;
                        }
                    } else {
                        current_block = 9914136504317020720;
                    }
                }
            }
            4 => {
                if (*node).type_0 as libc::c_uint
                    != XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else {
                    if !((*step).value).is_null() {
                        if *((*step).value).offset(0 as libc::c_int as isize)
                            as libc::c_int
                            != *((*node).name).offset(0 as libc::c_int as isize)
                                as libc::c_int
                        {
                            current_block = 9914136504317020720;
                        } else if xmlStrEqual((*step).value, (*node).name) == 0 {
                            current_block = 9914136504317020720;
                        } else {
                            current_block = 1118134448028020070;
                        }
                    } else {
                        current_block = 1118134448028020070;
                    }
                    match current_block {
                        9914136504317020720 => {}
                        _ => {
                            if ((*node).ns).is_null() {
                                if !((*step).value2).is_null() {
                                    current_block = 9914136504317020720;
                                } else {
                                    current_block = 4644295000439058019;
                                }
                            } else if !((*step).value2).is_null() {
                                if xmlStrEqual((*step).value2, (*(*node).ns).href) == 0 {
                                    current_block = 9914136504317020720;
                                } else {
                                    current_block = 4644295000439058019;
                                }
                            } else {
                                current_block = 4644295000439058019;
                            }
                        }
                    }
                }
            }
            5 => {
                if (*node).type_0 as libc::c_uint
                    == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                    || (*node).type_0 as libc::c_uint
                        == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                    || (*node).type_0 as libc::c_uint
                        == XML_DOCB_DOCUMENT_NODE as libc::c_int as libc::c_uint
                    || (*node).type_0 as libc::c_uint
                        == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else {
                    node = (*node).parent;
                    if node.is_null() {
                        current_block = 9914136504317020720;
                    } else if ((*step).value).is_null() {
                        current_block = 4644295000439058019;
                    } else if *((*step).value).offset(0 as libc::c_int as isize)
                        as libc::c_int
                        != *((*node).name).offset(0 as libc::c_int as isize)
                            as libc::c_int
                    {
                        current_block = 9914136504317020720;
                    } else if xmlStrEqual((*step).value, (*node).name) == 0 {
                        current_block = 9914136504317020720;
                    } else if ((*node).ns).is_null() {
                        if !((*step).value2).is_null() {
                            current_block = 9914136504317020720;
                        } else {
                            current_block = 4644295000439058019;
                        }
                    } else if !((*(*node).ns).href).is_null() {
                        if ((*step).value2).is_null() {
                            current_block = 9914136504317020720;
                        } else if xmlStrEqual((*step).value2, (*(*node).ns).href) == 0 {
                            current_block = 9914136504317020720;
                        } else {
                            current_block = 4644295000439058019;
                        }
                    } else {
                        current_block = 4644295000439058019;
                    }
                }
            }
            6 => {
                if ((*step).value).is_null() {
                    i += 1;
                    i;
                    step = &mut *((*comp).steps).offset(i as isize) as *mut xmlStepOp;
                    if (*step).op as libc::c_uint
                        == XML_OP_ROOT as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                    if (*step).op as libc::c_uint
                        != XML_OP_ELEM as libc::c_int as libc::c_uint
                    {
                        current_block = 9914136504317020720;
                    } else {
                        if ((*step).value).is_null() {
                            return -(1 as libc::c_int);
                        }
                        current_block = 5159818223158340697;
                    }
                } else {
                    current_block = 5159818223158340697;
                }
                match current_block {
                    9914136504317020720 => {}
                    _ => {
                        if node.is_null() {
                            current_block = 9914136504317020720;
                        } else if (*node).type_0 as libc::c_uint
                            == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                            || (*node).type_0 as libc::c_uint
                                == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
                            || (*node).type_0 as libc::c_uint
                                == XML_DOCB_DOCUMENT_NODE as libc::c_int as libc::c_uint
                            || (*node).type_0 as libc::c_uint
                                == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
                        {
                            current_block = 9914136504317020720;
                        } else {
                            node = (*node).parent;
                            while !node.is_null() {
                                if (*node).type_0 as libc::c_uint
                                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                                    && *((*step).value).offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                        == *((*node).name).offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                    && xmlStrEqual((*step).value, (*node).name) != 0
                                {
                                    if ((*node).ns).is_null() {
                                        if ((*step).value2).is_null() {
                                            break;
                                        }
                                    } else if !((*(*node).ns).href).is_null() {
                                        if !((*step).value2).is_null()
                                            && xmlStrEqual((*step).value2, (*(*node).ns).href) != 0
                                        {
                                            break;
                                        }
                                    }
                                }
                                node = (*node).parent;
                            }
                            if node.is_null() {
                                current_block = 9914136504317020720;
                            } else {
                                if (*step).op as libc::c_uint
                                    == XML_OP_ANCESTOR as libc::c_int as libc::c_uint
                                {
                                    xmlPatPushState(&mut states, i, node);
                                } else {
                                    xmlPatPushState(&mut states, i - 1 as libc::c_int, node);
                                }
                                current_block = 4644295000439058019;
                            }
                        }
                    }
                }
            }
            7 => {
                if (*node).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else if ((*node).ns).is_null() {
                    if !((*step).value).is_null() {
                        current_block = 9914136504317020720;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else if !((*(*node).ns).href).is_null() {
                    if ((*step).value).is_null() {
                        current_block = 9914136504317020720;
                    } else if xmlStrEqual((*step).value, (*(*node).ns).href) == 0 {
                        current_block = 9914136504317020720;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else {
                    current_block = 4644295000439058019;
                }
            }
            8 => {
                if (*node).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                {
                    current_block = 9914136504317020720;
                } else {
                    current_block = 4644295000439058019;
                }
            }
            _ => {
                current_block = 4644295000439058019;
            }
        }
        match current_block {
            4644295000439058019 => {
                i += 1;
                i;
            }
            _ => {
                if (states.states).is_null() {
                    return 0 as libc::c_int;
                }
                if states.nbstates <= 0 as libc::c_int {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(states.states as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
                states.nbstates -= 1;
                states.nbstates;
                i = (*(states.states).offset(states.nbstates as isize)).step;
                node = (*(states.states).offset(states.nbstates as isize)).node;
            }
        }
    }
    if !(states.states).is_null() {
        xmlFree.expect("non-null function pointer")(states.states as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn xmlPatScanName(mut ctxt: xmlPatParserContextPtr) -> *mut xmlChar {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut val: libc::c_int = 0;
    let mut len: libc::c_int = 0;
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
    q = (*ctxt).cur;
    cur = q;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    if !((if val < 0x100 as libc::c_int {
        (0x41 as libc::c_int <= val && val <= 0x5a as libc::c_int
            || 0x61 as libc::c_int <= val && val <= 0x7a as libc::c_int
            || 0xc0 as libc::c_int <= val && val <= 0xd6 as libc::c_int
            || 0xd8 as libc::c_int <= val && val <= 0xf6 as libc::c_int
            || 0xf8 as libc::c_int <= val) as libc::c_int
    } else {
        xmlCharInRange(val as libc::c_uint, &xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            (0x4e00 as libc::c_int <= val && val <= 0x9fa5 as libc::c_int
                || val == 0x3007 as libc::c_int
                || 0x3021 as libc::c_int <= val && val <= 0x3029 as libc::c_int)
                as libc::c_int
        }) != 0) && val != '_' as i32 && val != ':' as i32
    {
        return 0 as *mut xmlChar;
    }
    while (if val < 0x100 as libc::c_int {
        (0x41 as libc::c_int <= val && val <= 0x5a as libc::c_int
            || 0x61 as libc::c_int <= val && val <= 0x7a as libc::c_int
            || 0xc0 as libc::c_int <= val && val <= 0xd6 as libc::c_int
            || 0xd8 as libc::c_int <= val && val <= 0xf6 as libc::c_int
            || 0xf8 as libc::c_int <= val) as libc::c_int
    } else {
        xmlCharInRange(val as libc::c_uint, &xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            (0x4e00 as libc::c_int <= val && val <= 0x9fa5 as libc::c_int
                || val == 0x3007 as libc::c_int
                || 0x3021 as libc::c_int <= val && val <= 0x3029 as libc::c_int)
                as libc::c_int
        }) != 0
        || (if val < 0x100 as libc::c_int {
            (0x30 as libc::c_int <= val && val <= 0x39 as libc::c_int) as libc::c_int
        } else {
            xmlCharInRange(val as libc::c_uint, &xmlIsDigitGroup)
        }) != 0 || val == '.' as i32 || val == '-' as i32 || val == '_' as i32
        || (if val < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            xmlCharInRange(val as libc::c_uint, &xmlIsCombiningGroup)
        }) != 0
        || (if val < 0x100 as libc::c_int {
            (val == 0xb7 as libc::c_int) as libc::c_int
        } else {
            xmlCharInRange(val as libc::c_uint, &xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(len as isize);
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    }
    if !((*ctxt).dict).is_null() {
        ret = xmlDictLookup(
            (*ctxt).dict,
            q,
            cur.offset_from(q) as libc::c_long as libc::c_int,
        ) as *mut xmlChar;
    } else {
        ret = xmlStrndup(q, cur.offset_from(q) as libc::c_long as libc::c_int);
    }
    (*ctxt).cur = cur;
    return ret;
}
unsafe extern "C" fn xmlPatScanNCName(mut ctxt: xmlPatParserContextPtr) -> *mut xmlChar {
    let mut q: *const xmlChar = 0 as *const xmlChar;
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut val: libc::c_int = 0;
    let mut len: libc::c_int = 0;
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
    q = (*ctxt).cur;
    cur = q;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    if !((if val < 0x100 as libc::c_int {
        (0x41 as libc::c_int <= val && val <= 0x5a as libc::c_int
            || 0x61 as libc::c_int <= val && val <= 0x7a as libc::c_int
            || 0xc0 as libc::c_int <= val && val <= 0xd6 as libc::c_int
            || 0xd8 as libc::c_int <= val && val <= 0xf6 as libc::c_int
            || 0xf8 as libc::c_int <= val) as libc::c_int
    } else {
        xmlCharInRange(val as libc::c_uint, &xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            (0x4e00 as libc::c_int <= val && val <= 0x9fa5 as libc::c_int
                || val == 0x3007 as libc::c_int
                || 0x3021 as libc::c_int <= val && val <= 0x3029 as libc::c_int)
                as libc::c_int
        }) != 0) && val != '_' as i32
    {
        return 0 as *mut xmlChar;
    }
    while (if val < 0x100 as libc::c_int {
        (0x41 as libc::c_int <= val && val <= 0x5a as libc::c_int
            || 0x61 as libc::c_int <= val && val <= 0x7a as libc::c_int
            || 0xc0 as libc::c_int <= val && val <= 0xd6 as libc::c_int
            || 0xd8 as libc::c_int <= val && val <= 0xf6 as libc::c_int
            || 0xf8 as libc::c_int <= val) as libc::c_int
    } else {
        xmlCharInRange(val as libc::c_uint, &xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            (0x4e00 as libc::c_int <= val && val <= 0x9fa5 as libc::c_int
                || val == 0x3007 as libc::c_int
                || 0x3021 as libc::c_int <= val && val <= 0x3029 as libc::c_int)
                as libc::c_int
        }) != 0
        || (if val < 0x100 as libc::c_int {
            (0x30 as libc::c_int <= val && val <= 0x39 as libc::c_int) as libc::c_int
        } else {
            xmlCharInRange(val as libc::c_uint, &xmlIsDigitGroup)
        }) != 0 || val == '.' as i32 || val == '-' as i32 || val == '_' as i32
        || (if val < 0x100 as libc::c_int {
            0 as libc::c_int
        } else {
            xmlCharInRange(val as libc::c_uint, &xmlIsCombiningGroup)
        }) != 0
        || (if val < 0x100 as libc::c_int {
            (val == 0xb7 as libc::c_int) as libc::c_int
        } else {
            xmlCharInRange(val as libc::c_uint, &xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(len as isize);
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, &mut len);
    }
    if !((*ctxt).dict).is_null() {
        ret = xmlDictLookup(
            (*ctxt).dict,
            q,
            cur.offset_from(q) as libc::c_long as libc::c_int,
        ) as *mut xmlChar;
    } else {
        ret = xmlStrndup(q, cur.offset_from(q) as libc::c_long as libc::c_int);
    }
    (*ctxt).cur = cur;
    return ret;
}
unsafe extern "C" fn xmlCompileAttributeTest(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
    let mut token: *mut xmlChar = 0 as *mut xmlChar;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
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
    name = xmlPatScanNCName(ctxt);
    if name.is_null() {
        if *(*ctxt).cur as libc::c_int == '*' as i32 {
            if xmlPatternAdd(
                ctxt,
                (*ctxt).comp,
                XML_OP_ATTR,
                0 as *mut xmlChar,
                0 as *mut xmlChar,
            ) != 0
            {
                current_block = 18288716760668020644;
            } else {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                current_block = 3640593987805443782;
            }
        } else {
            (*ctxt).error = 1 as libc::c_int;
            current_block = 3640593987805443782;
        }
        match current_block {
            18288716760668020644 => {}
            _ => return,
        }
    } else {
        if *(*ctxt).cur as libc::c_int == ':' as i32 {
            let mut i: libc::c_int = 0;
            let mut prefix: *mut xmlChar = name;
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            if *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            {
                if ((*(*ctxt).comp).dict).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(prefix as *mut libc::c_void);
                }
                (*ctxt).error = 1 as libc::c_int;
                current_block = 18288716760668020644;
            } else {
                token = xmlPatScanName(ctxt);
                if *prefix.offset(0 as libc::c_int as isize) as libc::c_int == 'x' as i32
                    && *prefix.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'm' as i32
                    && *prefix.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'l' as i32
                    && *prefix.offset(3 as libc::c_int as isize) as libc::c_int
                        == 0 as libc::c_int
                {
                    if !((*(*ctxt).comp).dict).is_null() {
                        URL = xmlDictLookup(
                            (*(*ctxt).comp).dict,
                            b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                as *const libc::c_char as *const xmlChar as *mut xmlChar,
                            -(1 as libc::c_int),
                        ) as *mut xmlChar;
                    } else {
                        URL = xmlStrdup(
                            b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                as *const libc::c_char as *const xmlChar as *mut xmlChar,
                        );
                    }
                    current_block = 2873832966593178012;
                } else {
                    i = 0 as libc::c_int;
                    while i < (*ctxt).nb_namespaces {
                        if xmlStrEqual(
                            *((*ctxt).namespaces)
                                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize),
                            prefix,
                        ) != 0
                        {
                            if !((*(*ctxt).comp).dict).is_null() {
                                URL = xmlDictLookup(
                                    (*(*ctxt).comp).dict,
                                    *((*ctxt).namespaces)
                                        .offset((2 as libc::c_int * i) as isize) as *mut xmlChar,
                                    -(1 as libc::c_int),
                                ) as *mut xmlChar;
                            } else {
                                URL = xmlStrdup(
                                    *((*ctxt).namespaces)
                                        .offset((2 as libc::c_int * i) as isize) as *mut xmlChar,
                                );
                            }
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    if i >= (*ctxt).nb_namespaces {
                        if ((*(*ctxt).comp).dict).is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(prefix as *mut libc::c_void);
                        }
                        (*ctxt).error = 1 as libc::c_int;
                        current_block = 18288716760668020644;
                    } else {
                        current_block = 2873832966593178012;
                    }
                }
                match current_block {
                    18288716760668020644 => {}
                    _ => {
                        if ((*(*ctxt).comp).dict).is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(prefix as *mut libc::c_void);
                        }
                        if token.is_null() {
                            if *(*ctxt).cur as libc::c_int == '*' as i32 {
                                if *(*ctxt).cur as libc::c_int != 0 {
                                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                                    (*ctxt).cur;
                                } else {};
                                if xmlPatternAdd(
                                    ctxt,
                                    (*ctxt).comp,
                                    XML_OP_ATTR,
                                    0 as *mut xmlChar,
                                    URL,
                                ) != 0
                                {
                                    current_block = 18288716760668020644;
                                } else {
                                    current_block = 3689906465960840878;
                                }
                            } else {
                                (*ctxt).error = 1 as libc::c_int;
                                current_block = 18288716760668020644;
                            }
                        } else if xmlPatternAdd(
                            ctxt,
                            (*ctxt).comp,
                            XML_OP_ATTR,
                            token,
                            URL,
                        ) != 0
                        {
                            current_block = 18288716760668020644;
                        } else {
                            current_block = 3689906465960840878;
                        }
                    }
                }
            }
        } else if xmlPatternAdd(ctxt, (*ctxt).comp, XML_OP_ATTR, name, 0 as *mut xmlChar)
            != 0
        {
            current_block = 18288716760668020644;
        } else {
            current_block = 3689906465960840878;
        }
        match current_block {
            18288716760668020644 => {}
            _ => return,
        }
    }
    if !URL.is_null() {
        if ((*(*ctxt).comp).dict).is_null() {
            xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
        }
    }
    if !token.is_null() {
        if ((*(*ctxt).comp).dict).is_null() {
            xmlFree.expect("non-null function pointer")(token as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn xmlCompileStepPattern(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
    let mut token: *mut xmlChar = 0 as *mut xmlChar;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut hasBlanks: libc::c_int = 0 as libc::c_int;
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
    if *(*ctxt).cur as libc::c_int == '.' as i32 {
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        if !(xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ELEM,
            0 as *mut xmlChar,
            0 as *mut xmlChar,
        ) != 0)
        {
            return;
        }
    } else if *(*ctxt).cur as libc::c_int == '@' as i32 {
        if (*(*ctxt).comp).flags & XML_PATTERN_XSSEL as libc::c_int != 0 {
            (*ctxt).error = 1 as libc::c_int;
            return;
        }
        if *(*ctxt).cur as libc::c_int != 0 {
            (*ctxt).cur = ((*ctxt).cur).offset(1);
            (*ctxt).cur;
        } else {};
        xmlCompileAttributeTest(ctxt);
        if !((*ctxt).error != 0 as libc::c_int) {
            return;
        }
    } else {
        name = xmlPatScanNCName(ctxt);
        if name.is_null() {
            if *(*ctxt).cur as libc::c_int == '*' as i32 {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                if !(xmlPatternAdd(
                    ctxt,
                    (*ctxt).comp,
                    XML_OP_ALL,
                    0 as *mut xmlChar,
                    0 as *mut xmlChar,
                ) != 0)
                {
                    return;
                }
            } else {
                (*ctxt).error = 1 as libc::c_int;
                return;
            }
        } else {
            if *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                    && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
            {
                hasBlanks = 1 as libc::c_int;
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
            if *(*ctxt).cur as libc::c_int == ':' as i32 {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                if *(*ctxt).cur as libc::c_int != ':' as i32 {
                    let mut prefix: *mut xmlChar = name;
                    let mut i: libc::c_int = 0;
                    if hasBlanks != 0
                        || (*(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                            || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                                && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                            || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int)
                    {
                        (*ctxt).error = 1 as libc::c_int;
                        current_block = 17318331019046975573;
                    } else {
                        token = xmlPatScanName(ctxt);
                        if *prefix.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'x' as i32
                            && *prefix.offset(1 as libc::c_int as isize) as libc::c_int
                                == 'm' as i32
                            && *prefix.offset(2 as libc::c_int as isize) as libc::c_int
                                == 'l' as i32
                            && *prefix.offset(3 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                        {
                            if !((*(*ctxt).comp).dict).is_null() {
                                URL = xmlDictLookup(
                                    (*(*ctxt).comp).dict,
                                    b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                        as *const libc::c_char as *const xmlChar as *mut xmlChar,
                                    -(1 as libc::c_int),
                                ) as *mut xmlChar;
                            } else {
                                URL = xmlStrdup(
                                    b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                        as *const libc::c_char as *const xmlChar as *mut xmlChar,
                                );
                            }
                            current_block = 1345366029464561491;
                        } else {
                            i = 0 as libc::c_int;
                            while i < (*ctxt).nb_namespaces {
                                if xmlStrEqual(
                                    *((*ctxt).namespaces)
                                        .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize),
                                    prefix,
                                ) != 0
                                {
                                    if !((*(*ctxt).comp).dict).is_null() {
                                        URL = xmlDictLookup(
                                            (*(*ctxt).comp).dict,
                                            *((*ctxt).namespaces)
                                                .offset((2 as libc::c_int * i) as isize) as *mut xmlChar,
                                            -(1 as libc::c_int),
                                        ) as *mut xmlChar;
                                    } else {
                                        URL = xmlStrdup(
                                            *((*ctxt).namespaces)
                                                .offset((2 as libc::c_int * i) as isize) as *mut xmlChar,
                                        );
                                    }
                                    break;
                                } else {
                                    i += 1;
                                    i;
                                }
                            }
                            if i >= (*ctxt).nb_namespaces {
                                (*ctxt).error = 1 as libc::c_int;
                                current_block = 17318331019046975573;
                            } else {
                                current_block = 1345366029464561491;
                            }
                        }
                        match current_block {
                            17318331019046975573 => {}
                            _ => {
                                if ((*(*ctxt).comp).dict).is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(prefix as *mut libc::c_void);
                                }
                                name = 0 as *mut xmlChar;
                                if token.is_null() {
                                    if *(*ctxt).cur as libc::c_int == '*' as i32 {
                                        if *(*ctxt).cur as libc::c_int != 0 {
                                            (*ctxt).cur = ((*ctxt).cur).offset(1);
                                            (*ctxt).cur;
                                        } else {};
                                        if xmlPatternAdd(
                                            ctxt,
                                            (*ctxt).comp,
                                            XML_OP_NS,
                                            URL,
                                            0 as *mut xmlChar,
                                        ) != 0
                                        {
                                            current_block = 17318331019046975573;
                                        } else {
                                            current_block = 16910810822589621899;
                                        }
                                    } else {
                                        (*ctxt).error = 1 as libc::c_int;
                                        current_block = 17318331019046975573;
                                    }
                                } else if xmlPatternAdd(
                                    ctxt,
                                    (*ctxt).comp,
                                    XML_OP_ELEM,
                                    token,
                                    URL,
                                ) != 0
                                {
                                    current_block = 17318331019046975573;
                                } else {
                                    current_block = 16910810822589621899;
                                }
                            }
                        }
                    }
                } else {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                    if xmlStrEqual(
                        name,
                        b"child\0" as *const u8 as *const libc::c_char as *const xmlChar,
                    ) != 0
                    {
                        if ((*(*ctxt).comp).dict).is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(name as *mut libc::c_void);
                        }
                        name = xmlPatScanName(ctxt);
                        if name.is_null() {
                            if *(*ctxt).cur as libc::c_int == '*' as i32 {
                                if *(*ctxt).cur as libc::c_int != 0 {
                                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                                    (*ctxt).cur;
                                } else {};
                                if !(xmlPatternAdd(
                                    ctxt,
                                    (*ctxt).comp,
                                    XML_OP_ALL,
                                    0 as *mut xmlChar,
                                    0 as *mut xmlChar,
                                ) != 0)
                                {
                                    return;
                                }
                            } else {
                                (*ctxt).error = 1 as libc::c_int;
                            }
                        } else {
                            if *(*ctxt).cur as libc::c_int == ':' as i32 {
                                let mut prefix_0: *mut xmlChar = name;
                                let mut i_0: libc::c_int = 0;
                                if *(*ctxt).cur as libc::c_int != 0 {
                                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                                    (*ctxt).cur;
                                } else {};
                                if *(*ctxt).cur as libc::c_int == 0x20 as libc::c_int
                                    || 0x9 as libc::c_int <= *(*ctxt).cur as libc::c_int
                                        && *(*ctxt).cur as libc::c_int <= 0xa as libc::c_int
                                    || *(*ctxt).cur as libc::c_int == 0xd as libc::c_int
                                {
                                    (*ctxt).error = 1 as libc::c_int;
                                    current_block = 17318331019046975573;
                                } else {
                                    token = xmlPatScanName(ctxt);
                                    if *prefix_0.offset(0 as libc::c_int as isize)
                                        as libc::c_int == 'x' as i32
                                        && *prefix_0.offset(1 as libc::c_int as isize)
                                            as libc::c_int == 'm' as i32
                                        && *prefix_0.offset(2 as libc::c_int as isize)
                                            as libc::c_int == 'l' as i32
                                        && *prefix_0.offset(3 as libc::c_int as isize)
                                            as libc::c_int == 0 as libc::c_int
                                    {
                                        if !((*(*ctxt).comp).dict).is_null() {
                                            URL = xmlDictLookup(
                                                (*(*ctxt).comp).dict,
                                                b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                                    as *const libc::c_char as *const xmlChar as *mut xmlChar,
                                                -(1 as libc::c_int),
                                            ) as *mut xmlChar;
                                        } else {
                                            URL = xmlStrdup(
                                                b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                                    as *const libc::c_char as *const xmlChar as *mut xmlChar,
                                            );
                                        }
                                        current_block = 9505035279996566320;
                                    } else {
                                        i_0 = 0 as libc::c_int;
                                        while i_0 < (*ctxt).nb_namespaces {
                                            if xmlStrEqual(
                                                *((*ctxt).namespaces)
                                                    .offset(
                                                        (2 as libc::c_int * i_0 + 1 as libc::c_int) as isize,
                                                    ),
                                                prefix_0,
                                            ) != 0
                                            {
                                                if !((*(*ctxt).comp).dict).is_null() {
                                                    URL = xmlDictLookup(
                                                        (*(*ctxt).comp).dict,
                                                        *((*ctxt).namespaces)
                                                            .offset((2 as libc::c_int * i_0) as isize) as *mut xmlChar,
                                                        -(1 as libc::c_int),
                                                    ) as *mut xmlChar;
                                                } else {
                                                    URL = xmlStrdup(
                                                        *((*ctxt).namespaces)
                                                            .offset((2 as libc::c_int * i_0) as isize) as *mut xmlChar,
                                                    );
                                                }
                                                break;
                                            } else {
                                                i_0 += 1;
                                                i_0;
                                            }
                                        }
                                        if i_0 >= (*ctxt).nb_namespaces {
                                            (*ctxt).error = 1 as libc::c_int;
                                            current_block = 17318331019046975573;
                                        } else {
                                            current_block = 9505035279996566320;
                                        }
                                    }
                                    match current_block {
                                        17318331019046975573 => {}
                                        _ => {
                                            if ((*(*ctxt).comp).dict).is_null() {
                                                xmlFree
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(prefix_0 as *mut libc::c_void);
                                            }
                                            name = 0 as *mut xmlChar;
                                            if token.is_null() {
                                                if *(*ctxt).cur as libc::c_int == '*' as i32 {
                                                    if *(*ctxt).cur as libc::c_int != 0 {
                                                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                                                        (*ctxt).cur;
                                                    } else {};
                                                    if xmlPatternAdd(
                                                        ctxt,
                                                        (*ctxt).comp,
                                                        XML_OP_NS,
                                                        URL,
                                                        0 as *mut xmlChar,
                                                    ) != 0
                                                    {
                                                        current_block = 17318331019046975573;
                                                    } else {
                                                        current_block = 5265702136860997526;
                                                    }
                                                } else {
                                                    (*ctxt).error = 1 as libc::c_int;
                                                    current_block = 17318331019046975573;
                                                }
                                            } else if xmlPatternAdd(
                                                ctxt,
                                                (*ctxt).comp,
                                                XML_OP_CHILD,
                                                token,
                                                URL,
                                            ) != 0
                                            {
                                                current_block = 17318331019046975573;
                                            } else {
                                                current_block = 5265702136860997526;
                                            }
                                        }
                                    }
                                }
                            } else if xmlPatternAdd(
                                ctxt,
                                (*ctxt).comp,
                                XML_OP_CHILD,
                                name,
                                0 as *mut xmlChar,
                            ) != 0
                            {
                                current_block = 17318331019046975573;
                            } else {
                                current_block = 5265702136860997526;
                            }
                            match current_block {
                                17318331019046975573 => {}
                                _ => return,
                            }
                        }
                    } else if xmlStrEqual(
                        name,
                        b"attribute\0" as *const u8 as *const libc::c_char
                            as *const xmlChar,
                    ) != 0
                    {
                        if ((*(*ctxt).comp).dict).is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(name as *mut libc::c_void);
                        }
                        name = 0 as *mut xmlChar;
                        if (*(*ctxt).comp).flags & XML_PATTERN_XSSEL as libc::c_int != 0
                        {
                            (*ctxt).error = 1 as libc::c_int;
                        } else {
                            xmlCompileAttributeTest(ctxt);
                            if !((*ctxt).error != 0 as libc::c_int) {
                                return;
                            }
                        }
                    } else {
                        (*ctxt).error = 1 as libc::c_int;
                    }
                    current_block = 17318331019046975573;
                }
            } else if *(*ctxt).cur as libc::c_int == '*' as i32 {
                if !name.is_null() {
                    (*ctxt).error = 1 as libc::c_int;
                    current_block = 17318331019046975573;
                } else {
                    if *(*ctxt).cur as libc::c_int != 0 {
                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                        (*ctxt).cur;
                    } else {};
                    if xmlPatternAdd(
                        ctxt,
                        (*ctxt).comp,
                        XML_OP_ALL,
                        token,
                        0 as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 17318331019046975573;
                    } else {
                        current_block = 16910810822589621899;
                    }
                }
            } else if xmlPatternAdd(
                ctxt,
                (*ctxt).comp,
                XML_OP_ELEM,
                name,
                0 as *mut xmlChar,
            ) != 0
            {
                current_block = 17318331019046975573;
            } else {
                current_block = 16910810822589621899;
            }
            match current_block {
                17318331019046975573 => {}
                _ => return,
            }
        }
    }
    if !URL.is_null() {
        if ((*(*ctxt).comp).dict).is_null() {
            xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
        }
    }
    if !token.is_null() {
        if ((*(*ctxt).comp).dict).is_null() {
            xmlFree.expect("non-null function pointer")(token as *mut libc::c_void);
        }
    }
    if !name.is_null() {
        if ((*(*ctxt).comp).dict).is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn xmlCompilePathPattern(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
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
    if *(*ctxt).cur as libc::c_int == '/' as i32 {
        (*(*ctxt).comp).flags |= (1 as libc::c_int) << 8 as libc::c_int;
    } else if *(*ctxt).cur as libc::c_int == '.' as i32
        || (*(*ctxt).comp).flags
            & (XML_PATTERN_XPATH as libc::c_int | XML_PATTERN_XSSEL as libc::c_int
                | XML_PATTERN_XSFIELD as libc::c_int) != 0
    {
        (*(*ctxt).comp).flags |= (1 as libc::c_int) << 9 as libc::c_int;
    }
    if *(*ctxt).cur as libc::c_int == '/' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        if xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ANCESTOR,
            0 as *mut xmlChar,
            0 as *mut xmlChar,
        ) != 0
        {
            current_block = 13057993505109146761;
        } else {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            current_block = 4808432441040389987;
        }
    } else if *(*ctxt).cur as libc::c_int == '.' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *((*ctxt).cur).offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        if xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ANCESTOR,
            0 as *mut xmlChar,
            0 as *mut xmlChar,
        ) != 0
        {
            current_block = 13057993505109146761;
        } else {
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
            if *(*ctxt).cur as libc::c_int != 0 {
                (*ctxt).cur = ((*ctxt).cur).offset(1);
                (*ctxt).cur;
            } else {};
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
            if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                (*ctxt).error = 1 as libc::c_int;
                current_block = 13057993505109146761;
            } else {
                current_block = 4808432441040389987;
            }
        }
    } else {
        current_block = 4808432441040389987;
    }
    match current_block {
        4808432441040389987 => {
            if *(*ctxt).cur as libc::c_int == '@' as i32 {
                if *(*ctxt).cur as libc::c_int != 0 {
                    (*ctxt).cur = ((*ctxt).cur).offset(1);
                    (*ctxt).cur;
                } else {};
                xmlCompileAttributeTest(ctxt);
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
                if *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
                    xmlCompileStepPattern(ctxt);
                    if (*ctxt).error != 0 as libc::c_int {
                        current_block = 13057993505109146761;
                    } else {
                        current_block = 5159818223158340697;
                    }
                } else {
                    current_block = 5159818223158340697;
                }
            } else {
                if *(*ctxt).cur as libc::c_int == '/' as i32 {
                    if xmlPatternAdd(
                        ctxt,
                        (*ctxt).comp,
                        XML_OP_ROOT,
                        0 as *mut xmlChar,
                        0 as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 13057993505109146761;
                    } else {
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
                        if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                            (*ctxt).error = 1 as libc::c_int;
                            current_block = 13057993505109146761;
                        } else {
                            current_block = 7828949454673616476;
                        }
                    }
                } else {
                    current_block = 7828949454673616476;
                }
                match current_block {
                    13057993505109146761 => {}
                    _ => {
                        xmlCompileStepPattern(ctxt);
                        if (*ctxt).error != 0 as libc::c_int {
                            current_block = 13057993505109146761;
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
                            loop {
                                if !(*(*ctxt).cur as libc::c_int == '/' as i32) {
                                    current_block = 5159818223158340697;
                                    break;
                                }
                                if *((*ctxt).cur).offset(1 as libc::c_int as isize)
                                    as libc::c_int == '/' as i32
                                {
                                    if xmlPatternAdd(
                                        ctxt,
                                        (*ctxt).comp,
                                        XML_OP_ANCESTOR,
                                        0 as *mut xmlChar,
                                        0 as *mut xmlChar,
                                    ) != 0
                                    {
                                        current_block = 13057993505109146761;
                                        break;
                                    }
                                    if *(*ctxt).cur as libc::c_int != 0 {
                                        (*ctxt).cur = ((*ctxt).cur).offset(1);
                                        (*ctxt).cur;
                                    } else {};
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
                                    xmlCompileStepPattern(ctxt);
                                    if (*ctxt).error != 0 as libc::c_int {
                                        current_block = 13057993505109146761;
                                        break;
                                    }
                                } else {
                                    if xmlPatternAdd(
                                        ctxt,
                                        (*ctxt).comp,
                                        XML_OP_PARENT,
                                        0 as *mut xmlChar,
                                        0 as *mut xmlChar,
                                    ) != 0
                                    {
                                        current_block = 13057993505109146761;
                                        break;
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
                                    if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                                        (*ctxt).error = 1 as libc::c_int;
                                        current_block = 13057993505109146761;
                                        break;
                                    } else {
                                        xmlCompileStepPattern(ctxt);
                                        if (*ctxt).error != 0 as libc::c_int {
                                            current_block = 13057993505109146761;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                13057993505109146761 => {}
                _ => {
                    if *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
                        (*ctxt).error = 1 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn xmlCompileIDCXPathPath(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
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
    if !(*(*ctxt).cur as libc::c_int == '/' as i32) {
        (*(*ctxt).comp).flags |= (1 as libc::c_int) << 9 as libc::c_int;
        if *(*ctxt).cur as libc::c_int == '.' as i32 {
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
            if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                if xmlPatternAdd(
                    ctxt,
                    (*ctxt).comp,
                    XML_OP_ELEM,
                    0 as *mut xmlChar,
                    0 as *mut xmlChar,
                ) != 0
                {
                    current_block = 10574169844643716819;
                } else {
                    return
                }
            } else if *(*ctxt).cur as libc::c_int != '/' as i32 {
                current_block = 10574169844643716819;
            } else {
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
                if *(*ctxt).cur as libc::c_int == '/' as i32 {
                    if *((*ctxt).cur).offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == 0x20 as libc::c_int
                        || 0x9 as libc::c_int
                            <= *((*ctxt).cur).offset(-(1 as libc::c_int) as isize)
                                as libc::c_int
                            && *((*ctxt).cur).offset(-(1 as libc::c_int) as isize)
                                as libc::c_int <= 0xa as libc::c_int
                        || *((*ctxt).cur).offset(-(1 as libc::c_int) as isize)
                            as libc::c_int == 0xd as libc::c_int
                    {
                        current_block = 10574169844643716819;
                    } else if xmlPatternAdd(
                        ctxt,
                        (*ctxt).comp,
                        XML_OP_ANCESTOR,
                        0 as *mut xmlChar,
                        0 as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 10574169844643716819;
                    } else {
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
                        current_block = 11042950489265723346;
                    }
                } else {
                    current_block = 11042950489265723346;
                }
                match current_block {
                    10574169844643716819 => {}
                    _ => {
                        if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                            current_block = 5717714677271759834;
                        } else {
                            current_block = 2719512138335094285;
                        }
                    }
                }
            }
        } else {
            current_block = 2719512138335094285;
        }
        match current_block {
            10574169844643716819 => {}
            _ => {
                loop {
                    match current_block {
                        5717714677271759834 => {
                            (*ctxt).error = 1 as libc::c_int;
                            return;
                        }
                        _ => {
                            xmlCompileStepPattern(ctxt);
                            if (*ctxt).error != 0 as libc::c_int {
                                current_block = 10574169844643716819;
                                break;
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
                            if *(*ctxt).cur as libc::c_int != '/' as i32 {
                                current_block = 8180496224585318153;
                                break;
                            }
                            if xmlPatternAdd(
                                ctxt,
                                (*ctxt).comp,
                                XML_OP_PARENT,
                                0 as *mut xmlChar,
                                0 as *mut xmlChar,
                            ) != 0
                            {
                                current_block = 10574169844643716819;
                                break;
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
                            if *(*ctxt).cur as libc::c_int == '/' as i32 {
                                current_block = 10574169844643716819;
                                break;
                            }
                            if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                                current_block = 5717714677271759834;
                                continue;
                            }
                            if *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
                                current_block = 2719512138335094285;
                            } else {
                                current_block = 8180496224585318153;
                                break;
                            }
                        }
                    }
                }
                match current_block {
                    10574169844643716819 => {}
                    _ => {
                        if *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
                            (*ctxt).error = 1 as libc::c_int;
                        }
                        return;
                    }
                }
            }
        }
    }
    (*ctxt).error = 1 as libc::c_int;
}
unsafe extern "C" fn xmlNewStreamComp(mut size: libc::c_int) -> xmlStreamCompPtr {
    let mut cur: xmlStreamCompPtr = 0 as *mut xmlStreamComp;
    if size < 4 as libc::c_int {
        size = 4 as libc::c_int;
    }
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlStreamComp>() as libc::c_ulong) as xmlStreamCompPtr;
    if cur.is_null() {
        return 0 as xmlStreamCompPtr;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlStreamComp>() as libc::c_ulong,
    );
    (*cur)
        .steps = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<xmlStreamStep>() as libc::c_ulong),
    ) as xmlStreamStepPtr;
    if ((*cur).steps).is_null() {
        xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
        return 0 as xmlStreamCompPtr;
    }
    (*cur).nbStep = 0 as libc::c_int;
    (*cur).maxStep = size;
    return cur;
}
unsafe extern "C" fn xmlFreeStreamComp(mut comp: xmlStreamCompPtr) {
    if !comp.is_null() {
        if !((*comp).steps).is_null() {
            xmlFree
                .expect("non-null function pointer")((*comp).steps as *mut libc::c_void);
        }
        if !((*comp).dict).is_null() {
            xmlDictFree((*comp).dict);
        }
        xmlFree.expect("non-null function pointer")(comp as *mut libc::c_void);
    }
}
unsafe extern "C" fn xmlStreamCompAddStep(
    mut comp: xmlStreamCompPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut nodeType: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut cur: xmlStreamStepPtr = 0 as *mut xmlStreamStep;
    if (*comp).nbStep >= (*comp).maxStep {
        cur = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*comp).steps as *mut libc::c_void,
            (((*comp).maxStep * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlStreamStep>() as libc::c_ulong),
        ) as xmlStreamStepPtr;
        if cur.is_null() {
            return -(1 as libc::c_int);
        }
        (*comp).steps = cur;
        (*comp).maxStep *= 2 as libc::c_int;
    }
    let fresh13 = (*comp).nbStep;
    (*comp).nbStep = (*comp).nbStep + 1;
    cur = &mut *((*comp).steps).offset(fresh13 as isize) as *mut xmlStreamStep;
    (*cur).flags = flags;
    (*cur).name = name;
    (*cur).ns = ns;
    (*cur).nodeType = nodeType;
    return (*comp).nbStep - 1 as libc::c_int;
}
unsafe extern "C" fn xmlStreamCompile(mut comp: xmlPatternPtr) -> libc::c_int {
    let mut current_block: u64;
    let mut stream: xmlStreamCompPtr = 0 as *mut xmlStreamComp;
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0 as libc::c_int;
    let mut root: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut prevs: libc::c_int = -(1 as libc::c_int);
    let mut step: xmlStepOp = _xmlStepOp {
        op: XML_OP_END,
        value: 0 as *const xmlChar,
        value2: 0 as *const xmlChar,
    };
    if comp.is_null() || ((*comp).steps).is_null() {
        return -(1 as libc::c_int);
    }
    if (*comp).nbStep == 1 as libc::c_int
        && (*((*comp).steps).offset(0 as libc::c_int as isize)).op as libc::c_uint
            == XML_OP_ELEM as libc::c_int as libc::c_uint
        && ((*((*comp).steps).offset(0 as libc::c_int as isize)).value).is_null()
        && ((*((*comp).steps).offset(0 as libc::c_int as isize)).value2).is_null()
    {
        stream = xmlNewStreamComp(0 as libc::c_int);
        if stream.is_null() {
            return -(1 as libc::c_int);
        }
        (*stream).flags |= (1 as libc::c_int) << 14 as libc::c_int;
        (*comp).stream = stream;
        return 0 as libc::c_int;
    }
    stream = xmlNewStreamComp((*comp).nbStep / 2 as libc::c_int + 1 as libc::c_int);
    if stream.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*comp).dict).is_null() {
        (*stream).dict = (*comp).dict;
        xmlDictReference((*stream).dict);
    }
    i = 0 as libc::c_int;
    if (*comp).flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        (*stream).flags |= (1 as libc::c_int) << 15 as libc::c_int;
    }
    loop {
        if !(i < (*comp).nbStep) {
            current_block = 10380409671385728102;
            break;
        }
        step = *((*comp).steps).offset(i as isize);
        match step.op as libc::c_uint {
            1 => {
                if i != 0 as libc::c_int {
                    current_block = 3931296354299294212;
                    break;
                }
                root = 1 as libc::c_int;
            }
            7 => {
                s = xmlStreamCompAddStep(
                    stream,
                    0 as *const xmlChar,
                    step.value,
                    XML_ELEMENT_NODE as libc::c_int,
                    flags,
                );
                if s < 0 as libc::c_int {
                    current_block = 3931296354299294212;
                    break;
                }
                prevs = s;
                flags = 0 as libc::c_int;
            }
            4 => {
                flags |= 8 as libc::c_int;
                prevs = -(1 as libc::c_int);
                s = xmlStreamCompAddStep(
                    stream,
                    step.value,
                    step.value2,
                    XML_ATTRIBUTE_NODE as libc::c_int,
                    flags,
                );
                flags = 0 as libc::c_int;
                if s < 0 as libc::c_int {
                    current_block = 3931296354299294212;
                    break;
                }
            }
            2 => {
                if (step.value).is_null() && (step.value2).is_null() {
                    if (*comp).nbStep == i + 1 as libc::c_int
                        && flags & 1 as libc::c_int != 0
                    {
                        if (*comp).nbStep == i + 1 as libc::c_int {
                            (*stream).flags |= (1 as libc::c_int) << 14 as libc::c_int;
                        }
                        flags |= 16 as libc::c_int;
                        s = xmlStreamCompAddStep(
                            stream,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                            100 as libc::c_int,
                            flags,
                        );
                        if s < 0 as libc::c_int {
                            current_block = 3931296354299294212;
                            break;
                        }
                        flags = 0 as libc::c_int;
                        if prevs != -(1 as libc::c_int) {
                            (*((*stream).steps).offset(prevs as isize)).flags
                                |= 32 as libc::c_int;
                            prevs = -(1 as libc::c_int);
                        }
                    }
                } else {
                    s = xmlStreamCompAddStep(
                        stream,
                        step.value,
                        step.value2,
                        XML_ELEMENT_NODE as libc::c_int,
                        flags,
                    );
                    if s < 0 as libc::c_int {
                        current_block = 3931296354299294212;
                        break;
                    }
                    prevs = s;
                    flags = 0 as libc::c_int;
                }
            }
            3 => {
                s = xmlStreamCompAddStep(
                    stream,
                    step.value,
                    step.value2,
                    XML_ELEMENT_NODE as libc::c_int,
                    flags,
                );
                if s < 0 as libc::c_int {
                    current_block = 3931296354299294212;
                    break;
                }
                prevs = s;
                flags = 0 as libc::c_int;
            }
            8 => {
                s = xmlStreamCompAddStep(
                    stream,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    XML_ELEMENT_NODE as libc::c_int,
                    flags,
                );
                if s < 0 as libc::c_int {
                    current_block = 3931296354299294212;
                    break;
                }
                prevs = s;
                flags = 0 as libc::c_int;
            }
            6 => {
                if !(flags & 1 as libc::c_int != 0) {
                    flags |= 1 as libc::c_int;
                    if (*stream).flags & (1 as libc::c_int) << 16 as libc::c_int
                        == 0 as libc::c_int
                    {
                        (*stream).flags |= (1 as libc::c_int) << 16 as libc::c_int;
                    }
                }
            }
            0 | 5 | _ => {}
        }
        i += 1;
        i;
    }
    match current_block {
        10380409671385728102 => {
            if root == 0
                && (*comp).flags
                    & (XML_PATTERN_XPATH as libc::c_int
                        | XML_PATTERN_XSSEL as libc::c_int
                        | XML_PATTERN_XSFIELD as libc::c_int) == 0 as libc::c_int
            {
                if (*stream).flags & (1 as libc::c_int) << 16 as libc::c_int
                    == 0 as libc::c_int
                {
                    (*stream).flags |= (1 as libc::c_int) << 16 as libc::c_int;
                }
                if (*stream).nbStep > 0 as libc::c_int {
                    if (*((*stream).steps).offset(0 as libc::c_int as isize)).flags
                        & 1 as libc::c_int == 0 as libc::c_int
                    {
                        (*((*stream).steps).offset(0 as libc::c_int as isize)).flags
                            |= 1 as libc::c_int;
                    }
                }
            }
            if !((*stream).nbStep <= s) {
                (*((*stream).steps).offset(s as isize)).flags |= 2 as libc::c_int;
                if root != 0 {
                    (*((*stream).steps).offset(0 as libc::c_int as isize)).flags
                        |= 4 as libc::c_int;
                }
                (*comp).stream = stream;
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    xmlFreeStreamComp(stream);
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlNewStreamCtxt(mut stream: xmlStreamCompPtr) -> xmlStreamCtxtPtr {
    let mut cur: xmlStreamCtxtPtr = 0 as *mut xmlStreamCtxt;
    cur = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlStreamCtxt>() as libc::c_ulong) as xmlStreamCtxtPtr;
    if cur.is_null() {
        return 0 as xmlStreamCtxtPtr;
    }
    memset(
        cur as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlStreamCtxt>() as libc::c_ulong,
    );
    (*cur)
        .states = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((4 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*cur).states).is_null() {
        xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
        return 0 as xmlStreamCtxtPtr;
    }
    (*cur).nbState = 0 as libc::c_int;
    (*cur).maxState = 4 as libc::c_int;
    (*cur).level = 0 as libc::c_int;
    (*cur).comp = stream;
    (*cur).blockLevel = -(1 as libc::c_int);
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeStreamCtxt(mut stream: xmlStreamCtxtPtr) {
    let mut next: xmlStreamCtxtPtr = 0 as *mut xmlStreamCtxt;
    while !stream.is_null() {
        next = (*stream).next;
        if !((*stream).states).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*stream).states as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(stream as *mut libc::c_void);
        stream = next;
    }
}
unsafe extern "C" fn xmlStreamCtxtAddState(
    mut comp: xmlStreamCtxtPtr,
    mut idx: libc::c_int,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*comp).nbState {
        if *((*comp).states).offset((2 as libc::c_int * i) as isize) < 0 as libc::c_int {
            *((*comp).states).offset((2 as libc::c_int * i) as isize) = idx;
            *((*comp).states)
                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize) = level;
            return i;
        }
        i += 1;
        i;
    }
    if (*comp).nbState >= (*comp).maxState {
        let mut cur: *mut libc::c_int = 0 as *mut libc::c_int;
        cur = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*comp).states as *mut libc::c_void,
            (((*comp).maxState * 4 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if cur.is_null() {
            return -(1 as libc::c_int);
        }
        (*comp).states = cur;
        (*comp).maxState *= 2 as libc::c_int;
    }
    *((*comp).states).offset((2 as libc::c_int * (*comp).nbState) as isize) = idx;
    let fresh14 = (*comp).nbState;
    (*comp).nbState = (*comp).nbState + 1;
    *((*comp).states)
        .offset((2 as libc::c_int * fresh14 + 1 as libc::c_int) as isize) = level;
    return (*comp).nbState - 1 as libc::c_int;
}
unsafe extern "C" fn xmlStreamPushInternal(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut nodeType: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut final_0: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut stepNr: libc::c_int = 0;
    let mut desc: libc::c_int = 0;
    let mut comp: xmlStreamCompPtr = 0 as *mut xmlStreamComp;
    let mut step: xmlStreamStep = _xmlStreamStep {
        flags: 0,
        name: 0 as *const xmlChar,
        ns: 0 as *const xmlChar,
        nodeType: 0,
    };
    if stream.is_null() || (*stream).nbState < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    while !stream.is_null() {
        comp = (*stream).comp;
        if nodeType == XML_ELEMENT_NODE as libc::c_int && name.is_null() && ns.is_null()
        {
            (*stream).nbState = 0 as libc::c_int;
            (*stream).level = 0 as libc::c_int;
            (*stream).blockLevel = -(1 as libc::c_int);
            if (*comp).flags & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                if (*comp).nbStep == 0 as libc::c_int {
                    ret = 1 as libc::c_int;
                } else if (*comp).nbStep == 1 as libc::c_int
                    && (*((*comp).steps).offset(0 as libc::c_int as isize)).nodeType
                        == 100 as libc::c_int
                    && (*((*comp).steps).offset(0 as libc::c_int as isize)).flags
                        & 1 as libc::c_int != 0
                {
                    ret = 1 as libc::c_int;
                } else if (*((*comp).steps).offset(0 as libc::c_int as isize)).flags
                    & 4 as libc::c_int != 0
                {
                    tmp = xmlStreamCtxtAddState(
                        stream,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    if tmp < 0 as libc::c_int {
                        err += 1;
                        err;
                    }
                }
            }
            stream = (*stream).next;
        } else {
            if (*comp).nbStep == 0 as libc::c_int {
                if (*stream).flags & XML_PATTERN_XPATH as libc::c_int != 0 {
                    stream = (*stream).next;
                    continue;
                } else {
                    if nodeType != XML_ATTRIBUTE_NODE as libc::c_int
                        && ((*stream).flags
                            & (XML_PATTERN_XPATH as libc::c_int
                                | XML_PATTERN_XSSEL as libc::c_int
                                | XML_PATTERN_XSFIELD as libc::c_int) == 0 as libc::c_int
                            || (*stream).level == 0 as libc::c_int)
                    {
                        ret = 1 as libc::c_int;
                    }
                    (*stream).level += 1;
                    (*stream).level;
                }
            } else if (*stream).blockLevel != -(1 as libc::c_int) {
                (*stream).level += 1;
                (*stream).level;
            } else if nodeType != XML_ELEMENT_NODE as libc::c_int
                && nodeType != XML_ATTRIBUTE_NODE as libc::c_int
                && (*comp).flags & (1 as libc::c_int) << 14 as libc::c_int
                    == 0 as libc::c_int
            {
                (*stream).level += 1;
                (*stream).level;
            } else {
                i = 0 as libc::c_int;
                m = (*stream).nbState;
                while i < m {
                    if (*comp).flags & (1 as libc::c_int) << 16 as libc::c_int
                        == 0 as libc::c_int
                    {
                        stepNr = *((*stream).states)
                            .offset(
                                (2 as libc::c_int * ((*stream).nbState - 1 as libc::c_int))
                                    as isize,
                            );
                        if *((*stream).states)
                            .offset(
                                (2 as libc::c_int * ((*stream).nbState - 1 as libc::c_int)
                                    + 1 as libc::c_int) as isize,
                            ) < (*stream).level
                        {
                            return -(1 as libc::c_int);
                        }
                        desc = 0 as libc::c_int;
                        i = m;
                        current_block = 10758786907990354186;
                    } else {
                        stepNr = *((*stream).states)
                            .offset((2 as libc::c_int * i) as isize);
                        if stepNr < 0 as libc::c_int {
                            current_block = 5359220378648565450;
                        } else {
                            tmp = *((*stream).states)
                                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
                            if tmp > (*stream).level {
                                current_block = 5359220378648565450;
                            } else {
                                desc = (*((*comp).steps).offset(stepNr as isize)).flags
                                    & 1 as libc::c_int;
                                if tmp < (*stream).level && desc == 0 {
                                    current_block = 5359220378648565450;
                                } else {
                                    current_block = 10758786907990354186;
                                }
                            }
                        }
                    }
                    match current_block {
                        10758786907990354186 => {
                            step = *((*comp).steps).offset(stepNr as isize);
                            if step.nodeType != nodeType {
                                if step.nodeType == XML_ATTRIBUTE_NODE as libc::c_int {
                                    if (*comp).flags & (1 as libc::c_int) << 16 as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        (*stream).blockLevel = (*stream).level + 1 as libc::c_int;
                                    }
                                    current_block = 5359220378648565450;
                                } else if step.nodeType != 100 as libc::c_int {
                                    current_block = 5359220378648565450;
                                } else {
                                    current_block = 16738040538446813684;
                                }
                            } else {
                                current_block = 16738040538446813684;
                            }
                            match current_block {
                                5359220378648565450 => {}
                                _ => {
                                    match_0 = 0 as libc::c_int;
                                    if step.nodeType == 100 as libc::c_int {
                                        match_0 = 1 as libc::c_int;
                                    } else if (step.name).is_null() {
                                        if (step.ns).is_null() {
                                            match_0 = 1 as libc::c_int;
                                        } else if !ns.is_null() {
                                            match_0 = xmlStrEqual(step.ns, ns);
                                        }
                                    } else if (step.ns
                                        != 0 as *mut libc::c_void as *const xmlChar) as libc::c_int
                                        == (ns != 0 as *mut libc::c_void as *const xmlChar)
                                            as libc::c_int && !name.is_null()
                                        && *(step.name).offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == *name.offset(0 as libc::c_int as isize) as libc::c_int
                                        && xmlStrEqual(step.name, name) != 0
                                        && (step.ns == ns || xmlStrEqual(step.ns, ns) != 0)
                                    {
                                        match_0 = 1 as libc::c_int;
                                    }
                                    if match_0 != 0 {
                                        final_0 = step.flags & 2 as libc::c_int;
                                        if desc != 0 {
                                            if final_0 != 0 {
                                                ret = 1 as libc::c_int;
                                            } else {
                                                xmlStreamCtxtAddState(
                                                    stream,
                                                    stepNr + 1 as libc::c_int,
                                                    (*stream).level + 1 as libc::c_int,
                                                );
                                            }
                                        } else if final_0 != 0 {
                                            ret = 1 as libc::c_int;
                                        } else {
                                            xmlStreamCtxtAddState(
                                                stream,
                                                stepNr + 1 as libc::c_int,
                                                (*stream).level + 1 as libc::c_int,
                                            );
                                        }
                                        if ret != 1 as libc::c_int
                                            && step.flags & 32 as libc::c_int != 0
                                        {
                                            ret = 1 as libc::c_int;
                                        }
                                    }
                                    if (*comp).flags & (1 as libc::c_int) << 16 as libc::c_int
                                        == 0 as libc::c_int && (match_0 == 0 || final_0 != 0)
                                    {
                                        (*stream).blockLevel = (*stream).level + 1 as libc::c_int;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                    i;
                }
                (*stream).level += 1;
                (*stream).level;
                step = *((*comp).steps).offset(0 as libc::c_int as isize);
                if !(step.flags & 4 as libc::c_int != 0) {
                    desc = step.flags & 1 as libc::c_int;
                    if (*stream).flags
                        & (XML_PATTERN_XPATH as libc::c_int
                            | XML_PATTERN_XSSEL as libc::c_int
                            | XML_PATTERN_XSFIELD as libc::c_int) != 0
                    {
                        if (*stream).level == 1 as libc::c_int {
                            if (*stream).flags
                                & (XML_PATTERN_XSSEL as libc::c_int
                                    | XML_PATTERN_XSFIELD as libc::c_int) != 0
                            {
                                current_block = 10060051807816400284;
                            } else {
                                current_block = 17822945464197079757;
                            }
                        } else if desc != 0 {
                            current_block = 17822945464197079757;
                        } else if (*stream).level == 2 as libc::c_int
                            && (*stream).flags
                                & (XML_PATTERN_XSSEL as libc::c_int
                                    | XML_PATTERN_XSFIELD as libc::c_int) != 0
                        {
                            current_block = 17822945464197079757;
                        } else {
                            current_block = 10060051807816400284;
                        }
                    } else {
                        current_block = 17822945464197079757;
                    }
                    match current_block {
                        10060051807816400284 => {}
                        _ => {
                            if step.nodeType != nodeType {
                                if nodeType == XML_ATTRIBUTE_NODE as libc::c_int {
                                    current_block = 10060051807816400284;
                                } else if step.nodeType != 100 as libc::c_int {
                                    current_block = 10060051807816400284;
                                } else {
                                    current_block = 16313536926714486912;
                                }
                            } else {
                                current_block = 16313536926714486912;
                            }
                            match current_block {
                                10060051807816400284 => {}
                                _ => {
                                    match_0 = 0 as libc::c_int;
                                    if step.nodeType == 100 as libc::c_int {
                                        match_0 = 1 as libc::c_int;
                                    } else if (step.name).is_null() {
                                        if (step.ns).is_null() {
                                            match_0 = 1 as libc::c_int;
                                        } else if !ns.is_null() {
                                            match_0 = xmlStrEqual(step.ns, ns);
                                        }
                                    } else if (step.ns
                                        != 0 as *mut libc::c_void as *const xmlChar) as libc::c_int
                                        == (ns != 0 as *mut libc::c_void as *const xmlChar)
                                            as libc::c_int && !name.is_null()
                                        && *(step.name).offset(0 as libc::c_int as isize)
                                            as libc::c_int
                                            == *name.offset(0 as libc::c_int as isize) as libc::c_int
                                        && xmlStrEqual(step.name, name) != 0
                                        && (step.ns == ns || xmlStrEqual(step.ns, ns) != 0)
                                    {
                                        match_0 = 1 as libc::c_int;
                                    }
                                    final_0 = step.flags & 2 as libc::c_int;
                                    if match_0 != 0 {
                                        if final_0 != 0 {
                                            ret = 1 as libc::c_int;
                                        } else {
                                            xmlStreamCtxtAddState(
                                                stream,
                                                1 as libc::c_int,
                                                (*stream).level,
                                            );
                                        }
                                        if ret != 1 as libc::c_int
                                            && step.flags & 32 as libc::c_int != 0
                                        {
                                            ret = 1 as libc::c_int;
                                        }
                                    }
                                    if (*comp).flags & (1 as libc::c_int) << 16 as libc::c_int
                                        == 0 as libc::c_int && (match_0 == 0 || final_0 != 0)
                                    {
                                        (*stream).blockLevel = (*stream).level;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            stream = (*stream).next;
        }
    }
    if err > 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPush(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> libc::c_int {
    return xmlStreamPushInternal(stream, name, ns, XML_ELEMENT_NODE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPushNode(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut nodeType: libc::c_int,
) -> libc::c_int {
    return xmlStreamPushInternal(stream, name, ns, nodeType);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPushAttr(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> libc::c_int {
    return xmlStreamPushInternal(stream, name, ns, XML_ATTRIBUTE_NODE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPop(mut stream: xmlStreamCtxtPtr) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut lev: libc::c_int = 0;
    if stream.is_null() {
        return -(1 as libc::c_int);
    }
    while !stream.is_null() {
        if (*stream).blockLevel == (*stream).level {
            (*stream).blockLevel = -(1 as libc::c_int);
        }
        if (*stream).level != 0 {
            (*stream).level -= 1;
            (*stream).level;
        }
        i = (*stream).nbState - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            lev = *((*stream).states)
                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            if lev > (*stream).level {
                (*stream).nbState -= 1;
                (*stream).nbState;
            }
            if lev <= (*stream).level {
                break;
            }
            i -= 1;
            i;
        }
        stream = (*stream).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamWantsAnyNode(
    mut streamCtxt: xmlStreamCtxtPtr,
) -> libc::c_int {
    if streamCtxt.is_null() {
        return -(1 as libc::c_int);
    }
    while !streamCtxt.is_null() {
        if (*(*streamCtxt).comp).flags & (1 as libc::c_int) << 14 as libc::c_int != 0 {
            return 1 as libc::c_int;
        }
        streamCtxt = (*streamCtxt).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatterncompile(
    mut pattern: *const xmlChar,
    mut dict: *mut xmlDict,
    mut flags: libc::c_int,
    mut namespaces: *mut *const xmlChar,
) -> xmlPatternPtr {
    let mut current_block: u64;
    let mut ret: xmlPatternPtr = 0 as xmlPatternPtr;
    let mut cur: xmlPatternPtr = 0 as *mut xmlPattern;
    let mut ctxt: xmlPatParserContextPtr = 0 as xmlPatParserContextPtr;
    let mut or: *const xmlChar = 0 as *const xmlChar;
    let mut start: *const xmlChar = 0 as *const xmlChar;
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    let mut streamable: libc::c_int = 1 as libc::c_int;
    if pattern.is_null() {
        return 0 as xmlPatternPtr;
    }
    start = pattern;
    or = start;
    loop {
        if !(*or as libc::c_int != 0 as libc::c_int) {
            current_block = 313581471991351815;
            break;
        }
        tmp = 0 as *mut xmlChar;
        while *or as libc::c_int != 0 as libc::c_int && *or as libc::c_int != '|' as i32
        {
            or = or.offset(1);
            or;
        }
        if *or as libc::c_int == 0 as libc::c_int {
            ctxt = xmlNewPatParserContext(start, dict, namespaces);
        } else {
            tmp = xmlStrndup(
                start,
                or.offset_from(start) as libc::c_long as libc::c_int,
            );
            if !tmp.is_null() {
                ctxt = xmlNewPatParserContext(tmp, dict, namespaces);
            }
            or = or.offset(1);
            or;
        }
        if ctxt.is_null() {
            current_block = 14053147170392723153;
            break;
        }
        cur = xmlNewPattern();
        if cur.is_null() {
            current_block = 14053147170392723153;
            break;
        }
        if !dict.is_null() {
            (*cur).dict = dict;
            xmlDictReference(dict);
        }
        if ret.is_null() {
            ret = cur;
        } else {
            (*cur).next = (*ret).next;
            (*ret).next = cur;
        }
        (*cur).flags = flags;
        (*ctxt).comp = cur;
        if (*cur).flags
            & (XML_PATTERN_XSSEL as libc::c_int | XML_PATTERN_XSFIELD as libc::c_int)
            != 0
        {
            xmlCompileIDCXPathPath(ctxt);
        } else {
            xmlCompilePathPattern(ctxt);
        }
        if (*ctxt).error != 0 as libc::c_int {
            current_block = 14053147170392723153;
            break;
        }
        xmlFreePatParserContext(ctxt);
        ctxt = 0 as xmlPatParserContextPtr;
        if streamable != 0 {
            if type_0 == 0 as libc::c_int {
                type_0 = (*cur).flags
                    & ((1 as libc::c_int) << 8 as libc::c_int
                        | (1 as libc::c_int) << 9 as libc::c_int);
            } else if type_0 == (1 as libc::c_int) << 8 as libc::c_int {
                if (*cur).flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
                    streamable = 0 as libc::c_int;
                }
            } else if type_0 == (1 as libc::c_int) << 9 as libc::c_int {
                if (*cur).flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
                    streamable = 0 as libc::c_int;
                }
            }
        }
        if streamable != 0 {
            xmlStreamCompile(cur);
        }
        if xmlReversePattern(cur) < 0 as libc::c_int {
            current_block = 14053147170392723153;
            break;
        }
        if !tmp.is_null() {
            xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void);
            tmp = 0 as *mut xmlChar;
        }
        start = or;
    }
    match current_block {
        14053147170392723153 => {
            if !ctxt.is_null() {
                xmlFreePatParserContext(ctxt);
            }
            if !ret.is_null() {
                xmlFreePattern(ret);
            }
            if !tmp.is_null() {
                xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void);
            }
            return 0 as xmlPatternPtr;
        }
        _ => {
            if streamable == 0 as libc::c_int {
                cur = ret;
                while !cur.is_null() {
                    if !((*cur).stream).is_null() {
                        xmlFreeStreamComp((*cur).stream);
                        (*cur).stream = 0 as xmlStreamCompPtr;
                    }
                    cur = (*cur).next;
                }
            }
            return ret;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternMatch(
    mut comp: xmlPatternPtr,
    mut node: xmlNodePtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if comp.is_null() || node.is_null() {
        return -(1 as libc::c_int);
    }
    while !comp.is_null() {
        ret = xmlPatMatch(comp, node);
        if ret != 0 as libc::c_int {
            return ret;
        }
        comp = (*comp).next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternGetStreamCtxt(
    mut comp: xmlPatternPtr,
) -> xmlStreamCtxtPtr {
    let mut current_block: u64;
    let mut ret: xmlStreamCtxtPtr = 0 as xmlStreamCtxtPtr;
    let mut cur: xmlStreamCtxtPtr = 0 as *mut xmlStreamCtxt;
    if comp.is_null() || ((*comp).stream).is_null() {
        return 0 as xmlStreamCtxtPtr;
    }
    loop {
        if comp.is_null() {
            current_block = 11650488183268122163;
            break;
        }
        if ((*comp).stream).is_null() {
            current_block = 1172338785175279957;
            break;
        }
        cur = xmlNewStreamCtxt((*comp).stream);
        if cur.is_null() {
            current_block = 1172338785175279957;
            break;
        }
        if ret.is_null() {
            ret = cur;
        } else {
            (*cur).next = (*ret).next;
            (*ret).next = cur;
        }
        (*cur).flags = (*comp).flags;
        comp = (*comp).next;
    }
    match current_block {
        11650488183268122163 => return ret,
        _ => {
            xmlFreeStreamCtxt(ret);
            return 0 as xmlStreamCtxtPtr;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternStreamable(mut comp: xmlPatternPtr) -> libc::c_int {
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    while !comp.is_null() {
        if ((*comp).stream).is_null() {
            return 0 as libc::c_int;
        }
        comp = (*comp).next;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternMaxDepth(mut comp: xmlPatternPtr) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    while !comp.is_null() {
        if ((*comp).stream).is_null() {
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < (*(*comp).stream).nbStep {
            if (*((*(*comp).stream).steps).offset(i as isize)).flags & 1 as libc::c_int
                != 0
            {
                return -(2 as libc::c_int);
            }
            i += 1;
            i;
        }
        if (*(*comp).stream).nbStep > ret {
            ret = (*(*comp).stream).nbStep;
        }
        comp = (*comp).next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternMinDepth(mut comp: xmlPatternPtr) -> libc::c_int {
    let mut ret: libc::c_int = 12345678 as libc::c_int;
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    while !comp.is_null() {
        if ((*comp).stream).is_null() {
            return -(1 as libc::c_int);
        }
        if (*(*comp).stream).nbStep < ret {
            ret = (*(*comp).stream).nbStep;
        }
        if ret == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        comp = (*comp).next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternFromRoot(mut comp: xmlPatternPtr) -> libc::c_int {
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    while !comp.is_null() {
        if ((*comp).stream).is_null() {
            return -(1 as libc::c_int);
        }
        if (*comp).flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            return 1 as libc::c_int;
        }
        comp = (*comp).next;
    }
    return 0 as libc::c_int;
}
