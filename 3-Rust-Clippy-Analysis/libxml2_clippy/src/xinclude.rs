use ::libc;
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlXPathCompExpr;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn xmlParserInputBufferRead(
        in_0: xmlParserInputBufferPtr,
        len: libc::c_int,
    ) -> libc::c_int;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlParserGetDirectory(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewText(content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewTextLen(content: *const xmlChar, len: libc::c_int) -> xmlNodePtr;
    fn xmlCopyNode(node: xmlNodePtr, recursive: libc::c_int) -> xmlNodePtr;
    fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: libc::c_int,
    ) -> xmlNodePtr;
    fn xmlDocCopyNodeList(doc: xmlDocPtr, node: xmlNodePtr) -> xmlNodePtr;
    fn xmlCopyNodeList(node: xmlNodePtr) -> xmlNodePtr;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddNextSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlNodeAddContentLen(cur: xmlNodePtr, content: *const xmlChar, len: libc::c_int);
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar);
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;
    fn xmlParseCharEncoding(name: *const libc::c_char) -> xmlCharEncoding;
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler) -> libc::c_int;
    fn xmlInitParser();
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> libc::c_int;
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlLoadExternalEntity(
        URL: *const libc::c_char,
        ID: *const libc::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: libc::c_int) -> libc::c_int;
    fn xmlAddDocEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: libc::c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlDictReference(dict: xmlDictPtr) -> libc::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
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
    fn xmlHashScan(table: xmlHashTablePtr, f: xmlHashScanner, data: *mut libc::c_void);
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlBuildRelativeURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlParseURI(str: *const libc::c_char) -> xmlURIPtr;
    fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
    fn xmlURIEscape(str: *const xmlChar) -> *mut xmlChar;
    fn xmlFreeURI(uri: xmlURIPtr);
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    fn xmlXPtrNewContext(
        doc: xmlDocPtr,
        here: xmlNodePtr,
        origin: xmlNodePtr,
    ) -> xmlXPathContextPtr;
    fn xmlXPtrEval(str: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> libc::c_int;
    fn xmlStringCurrentChar(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        len: *mut libc::c_int,
    ) -> libc::c_int;
    fn xmlBufLength(buf: xmlBufPtr) -> size_t;
    fn xmlXPtrAdvanceNode(cur: xmlNodePtr, level: *mut libc::c_int) -> xmlNodePtr;
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
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_1 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_1 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_1 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_1 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_1 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_1 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_1 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_1 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_1 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_1 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_1 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_1 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_1 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_1 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_1 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_1 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_1 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_1 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_1 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_1 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_1 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_1 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut libc::c_char,
    pub opaque: *mut libc::c_char,
    pub authority: *mut libc::c_char,
    pub server: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    pub query: *mut libc::c_char,
    pub fragment: *mut libc::c_char,
    pub cleanup: libc::c_int,
    pub query_raw: *mut libc::c_char,
}
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLocationSet {
    pub locNr: libc::c_int,
    pub locMax: libc::c_int,
    pub locTab: *mut xmlXPathObjectPtr,
}
pub type xmlLocationSet = _xmlLocationSet;
pub type xmlLocationSetPtr = *mut xmlLocationSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeCtxt {
    pub doc: xmlDocPtr,
    pub incBase: libc::c_int,
    pub incNr: libc::c_int,
    pub incMax: libc::c_int,
    pub incTab: *mut xmlXIncludeRefPtr,
    pub txtNr: libc::c_int,
    pub txtMax: libc::c_int,
    pub txtTab: *mut xmlNodePtr,
    pub txturlTab: *mut xmlURL,
    pub url: *mut xmlChar,
    pub urlNr: libc::c_int,
    pub urlMax: libc::c_int,
    pub urlTab: *mut *mut xmlChar,
    pub nbErrors: libc::c_int,
    pub legacy: libc::c_int,
    pub parseFlags: libc::c_int,
    pub base: *mut xmlChar,
    pub _private: *mut libc::c_void,
}
pub type xmlURL = *mut xmlChar;
pub type xmlXIncludeRefPtr = *mut xmlXIncludeRef;
pub type xmlXIncludeRef = _xmlXIncludeRef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeRef {
    pub URI: *mut xmlChar,
    pub fragment: *mut xmlChar,
    pub doc: xmlDocPtr,
    pub ref_0: xmlNodePtr,
    pub inc: xmlNodePtr,
    pub xml: libc::c_int,
    pub count: libc::c_int,
    pub xptr: xmlXPathObjectPtr,
    pub emptyFb: libc::c_int,
}
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeMergeData = _xmlXIncludeMergeData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeMergeData {
    pub doc: xmlDocPtr,
    pub ctxt: xmlXIncludeCtxtPtr,
}
pub type xmlXIncludeMergeDataPtr = *mut xmlXIncludeMergeData;
unsafe extern "C" fn xmlXIncludeErrMemory(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
    mut extra: *const libc::c_char,
) {
    if !ctxt.is_null() {
        (*ctxt).nbErrors += 1;
        (*ctxt).nbErrors;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        node as *mut libc::c_void,
        XML_FROM_XINCLUDE as libc::c_int,
        XML_ERR_NO_MEMORY as libc::c_int,
        XML_ERR_ERROR,
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
}
unsafe extern "C" fn xmlXIncludeErr(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
    mut error: libc::c_int,
    mut msg: *const libc::c_char,
    mut extra: *const xmlChar,
) {
    if !ctxt.is_null() {
        (*ctxt).nbErrors += 1;
        (*ctxt).nbErrors;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        node as *mut libc::c_void,
        XML_FROM_XINCLUDE as libc::c_int,
        error,
        XML_ERR_ERROR,
        0 as *const libc::c_char,
        0 as libc::c_int,
        extra as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        msg,
        extra as *const libc::c_char,
    );
}
unsafe extern "C" fn xmlXIncludeGetProp(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut cur: xmlNodePtr,
    mut name: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    ret = xmlGetNsProp(
        cur as *const xmlNode,
        b"http://www.w3.org/2003/XInclude\0" as *const u8 as *const libc::c_char
            as *const xmlChar,
        name,
    );
    if !ret.is_null() {
        return ret;
    }
    if (*ctxt).legacy != 0 as libc::c_int {
        ret = xmlGetNsProp(
            cur as *const xmlNode,
            b"http://www.w3.org/2001/XInclude\0" as *const u8 as *const libc::c_char
                as *const xmlChar,
            name,
        );
        if !ret.is_null() {
            return ret;
        }
    }
    ret = xmlGetProp(cur as *const xmlNode, name);
    return ret;
}
unsafe extern "C" fn xmlXIncludeFreeRef(mut ref_0: xmlXIncludeRefPtr) {
    if ref_0.is_null() {
        return;
    }
    if !((*ref_0).doc).is_null() {
        xmlFreeDoc((*ref_0).doc);
    }
    if !((*ref_0).URI).is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).URI as *mut libc::c_void);
    }
    if !((*ref_0).fragment).is_null() {
        xmlFree
            .expect("non-null function pointer")((*ref_0).fragment as *mut libc::c_void);
    }
    if !((*ref_0).xptr).is_null() {
        xmlXPathFreeObject((*ref_0).xptr);
    }
    xmlFree.expect("non-null function pointer")(ref_0 as *mut libc::c_void);
}
unsafe extern "C" fn xmlXIncludeNewRef(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut URI: *const xmlChar,
    mut ref_0: xmlNodePtr,
) -> xmlXIncludeRefPtr {
    let mut ret: xmlXIncludeRefPtr = 0 as *mut xmlXIncludeRef;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXIncludeRef>() as libc::c_ulong)
        as xmlXIncludeRefPtr;
    if ret.is_null() {
        xmlXIncludeErrMemory(
            ctxt,
            ref_0,
            b"growing XInclude context\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXIncludeRefPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXIncludeRef>() as libc::c_ulong,
    );
    if URI.is_null() {
        (*ret).URI = 0 as *mut xmlChar;
    } else {
        (*ret).URI = xmlStrdup(URI);
    }
    (*ret).fragment = 0 as *mut xmlChar;
    (*ret).ref_0 = ref_0;
    (*ret).doc = 0 as xmlDocPtr;
    (*ret).count = 0 as libc::c_int;
    (*ret).xml = 0 as libc::c_int;
    (*ret).inc = 0 as xmlNodePtr;
    if (*ctxt).incMax == 0 as libc::c_int {
        (*ctxt).incMax = 4 as libc::c_int;
        (*ctxt)
            .incTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).incMax as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<xmlXIncludeRefPtr>() as libc::c_ulong,
                ),
        ) as *mut xmlXIncludeRefPtr;
        if ((*ctxt).incTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                ref_0,
                b"growing XInclude context\0" as *const u8 as *const libc::c_char,
            );
            xmlXIncludeFreeRef(ret);
            return 0 as xmlXIncludeRefPtr;
        }
    }
    if (*ctxt).incNr >= (*ctxt).incMax {
        (*ctxt).incMax *= 2 as libc::c_int;
        (*ctxt)
            .incTab = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).incTab as *mut libc::c_void,
            ((*ctxt).incMax as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<xmlXIncludeRefPtr>() as libc::c_ulong,
                ),
        ) as *mut xmlXIncludeRefPtr;
        if ((*ctxt).incTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                ref_0,
                b"growing XInclude context\0" as *const u8 as *const libc::c_char,
            );
            xmlXIncludeFreeRef(ret);
            return 0 as xmlXIncludeRefPtr;
        }
    }
    let fresh0 = (*ctxt).incNr;
    (*ctxt).incNr = (*ctxt).incNr + 1;
    let ref mut fresh1 = *((*ctxt).incTab).offset(fresh0 as isize);
    *fresh1 = ret;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeNewContext(
    mut doc: xmlDocPtr,
) -> xmlXIncludeCtxtPtr {
    let mut ret: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    if doc.is_null() {
        return 0 as xmlXIncludeCtxtPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlXIncludeCtxt>() as libc::c_ulong)
        as xmlXIncludeCtxtPtr;
    if ret.is_null() {
        xmlXIncludeErrMemory(
            0 as xmlXIncludeCtxtPtr,
            doc as xmlNodePtr,
            b"creating XInclude context\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlXIncludeCtxtPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlXIncludeCtxt>() as libc::c_ulong,
    );
    (*ret).doc = doc;
    (*ret).incNr = 0 as libc::c_int;
    (*ret).incBase = 0 as libc::c_int;
    (*ret).incMax = 0 as libc::c_int;
    (*ret).incTab = 0 as *mut xmlXIncludeRefPtr;
    (*ret).nbErrors = 0 as libc::c_int;
    return ret;
}
unsafe extern "C" fn xmlXIncludeURLPush(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut value: *const xmlChar,
) -> libc::c_int {
    if (*ctxt).urlNr > 40 as libc::c_int {
        xmlXIncludeErr(
            ctxt,
            0 as xmlNodePtr,
            XML_XINCLUDE_RECURSION as libc::c_int,
            b"detected a recursion in %s\n\0" as *const u8 as *const libc::c_char,
            value,
        );
        return -(1 as libc::c_int);
    }
    if ((*ctxt).urlTab).is_null() {
        (*ctxt).urlMax = 4 as libc::c_int;
        (*ctxt).urlNr = 0 as libc::c_int;
        (*ctxt)
            .urlTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).urlMax as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        ) as *mut *mut xmlChar;
        if ((*ctxt).urlTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                0 as xmlNodePtr,
                b"adding URL\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*ctxt).urlNr >= (*ctxt).urlMax {
        (*ctxt).urlMax *= 2 as libc::c_int;
        (*ctxt)
            .urlTab = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).urlTab as *mut libc::c_void,
            ((*ctxt).urlMax as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as libc::c_ulong),
        ) as *mut *mut xmlChar;
        if ((*ctxt).urlTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                0 as xmlNodePtr,
                b"adding URL\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    let ref mut fresh2 = *((*ctxt).urlTab).offset((*ctxt).urlNr as isize);
    *fresh2 = xmlStrdup(value);
    (*ctxt).url = *fresh2;
    let fresh3 = (*ctxt).urlNr;
    (*ctxt).urlNr = (*ctxt).urlNr + 1;
    return fresh3;
}
unsafe extern "C" fn xmlXIncludeURLPop(mut ctxt: xmlXIncludeCtxtPtr) {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if (*ctxt).urlNr <= 0 as libc::c_int {
        return;
    }
    (*ctxt).urlNr -= 1;
    (*ctxt).urlNr;
    if (*ctxt).urlNr > 0 as libc::c_int {
        (*ctxt)
            .url = *((*ctxt).urlTab).offset(((*ctxt).urlNr - 1 as libc::c_int) as isize);
    } else {
        (*ctxt).url = 0 as *mut xmlChar;
    }
    ret = *((*ctxt).urlTab).offset((*ctxt).urlNr as isize);
    let ref mut fresh4 = *((*ctxt).urlTab).offset((*ctxt).urlNr as isize);
    *fresh4 = 0 as *mut xmlChar;
    if !ret.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeFreeContext(mut ctxt: xmlXIncludeCtxtPtr) {
    let mut i: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    while (*ctxt).urlNr > 0 as libc::c_int {
        xmlXIncludeURLPop(ctxt);
    }
    if !((*ctxt).urlTab).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).urlTab as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while i < (*ctxt).incNr {
        if !(*((*ctxt).incTab).offset(i as isize)).is_null() {
            xmlXIncludeFreeRef(*((*ctxt).incTab).offset(i as isize));
        }
        i += 1;
        i;
    }
    if !((*ctxt).txturlTab).is_null() {
        i = 0 as libc::c_int;
        while i < (*ctxt).txtNr {
            if !(*((*ctxt).txturlTab).offset(i as isize)).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(*((*ctxt).txturlTab).offset(i as isize) as *mut libc::c_void);
            }
            i += 1;
            i;
        }
    }
    if !((*ctxt).incTab).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).incTab as *mut libc::c_void);
    }
    if !((*ctxt).txtTab).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).txtTab as *mut libc::c_void);
    }
    if !((*ctxt).txturlTab).is_null() {
        xmlFree
            .expect("non-null function pointer")((*ctxt).txturlTab as *mut libc::c_void);
    }
    if !((*ctxt).base).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).base as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn xmlXIncludeParseFile(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut URL: *const libc::c_char,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = 0 as *mut xmlDoc;
    let mut pctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    xmlInitParser();
    pctxt = xmlNewParserCtxt();
    if pctxt.is_null() {
        xmlXIncludeErrMemory(
            ctxt,
            0 as xmlNodePtr,
            b"cannot allocate parser context\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlDocPtr;
    }
    (*pctxt)._private = (*ctxt)._private;
    if !((*ctxt).doc).is_null() && !((*(*ctxt).doc).dict).is_null() {
        if !((*pctxt).dict).is_null() {
            xmlDictFree((*pctxt).dict);
        }
        (*pctxt).dict = (*(*ctxt).doc).dict;
        xmlDictReference((*pctxt).dict);
    }
    xmlCtxtUseOptions(pctxt, (*ctxt).parseFlags | XML_PARSE_DTDLOAD as libc::c_int);
    inputStream = xmlLoadExternalEntity(URL, 0 as *const libc::c_char, pctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt(pctxt);
        return 0 as xmlDocPtr;
    }
    inputPush(pctxt, inputStream);
    if ((*pctxt).directory).is_null() {
        (*pctxt).directory = xmlParserGetDirectory(URL);
    }
    (*pctxt).loadsubset |= 2 as libc::c_int;
    xmlParseDocument(pctxt);
    if (*pctxt).wellFormed != 0 {
        ret = (*pctxt).myDoc;
    } else {
        ret = 0 as xmlDocPtr;
        if !((*pctxt).myDoc).is_null() {
            xmlFreeDoc((*pctxt).myDoc);
        }
        (*pctxt).myDoc = 0 as xmlDocPtr;
    }
    xmlFreeParserCtxt(pctxt);
    return ret;
}
unsafe extern "C" fn xmlXIncludeAddNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut cur: xmlNodePtr,
) -> libc::c_int {
    let mut ref_0: xmlXIncludeRefPtr = 0 as *mut xmlXIncludeRef;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut fragment: *mut xmlChar = 0 as *mut xmlChar;
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut parse: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut xml: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut local: libc::c_int = 0 as libc::c_int;
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if cur.is_null() {
        return -(1 as libc::c_int);
    }
    href = xmlXIncludeGetProp(
        ctxt,
        cur,
        b"href\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if href.is_null() {
        href = xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *mut xmlChar);
        if href.is_null() {
            return -(1 as libc::c_int);
        }
    }
    if *href.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        || *href.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        local = 1 as libc::c_int;
    }
    parse = xmlXIncludeGetProp(
        ctxt,
        cur,
        b"parse\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if !parse.is_null() {
        if xmlStrEqual(
            parse,
            b"xml\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) != 0
        {
            xml = 1 as libc::c_int;
        } else if xmlStrEqual(
            parse,
            b"text\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) != 0
        {
            xml = 0 as libc::c_int;
        } else {
            xmlXIncludeErr(
                ctxt,
                cur,
                XML_XINCLUDE_PARSE_VALUE as libc::c_int,
                b"invalid value %s for 'parse'\n\0" as *const u8 as *const libc::c_char,
                parse,
            );
            if !href.is_null() {
                xmlFree.expect("non-null function pointer")(href as *mut libc::c_void);
            }
            if !parse.is_null() {
                xmlFree.expect("non-null function pointer")(parse as *mut libc::c_void);
            }
            return -(1 as libc::c_int);
        }
    }
    base = xmlNodeGetBase((*ctxt).doc as *const xmlDoc, cur as *const xmlNode);
    if base.is_null() {
        URI = xmlBuildURI(href, (*(*ctxt).doc).URL);
    } else {
        URI = xmlBuildURI(href, base);
    }
    if URI.is_null() {
        let mut escbase: *mut xmlChar = 0 as *mut xmlChar;
        let mut eschref: *mut xmlChar = 0 as *mut xmlChar;
        escbase = xmlURIEscape(base);
        eschref = xmlURIEscape(href);
        URI = xmlBuildURI(eschref, escbase);
        if !escbase.is_null() {
            xmlFree.expect("non-null function pointer")(escbase as *mut libc::c_void);
        }
        if !eschref.is_null() {
            xmlFree.expect("non-null function pointer")(eschref as *mut libc::c_void);
        }
    }
    if !parse.is_null() {
        xmlFree.expect("non-null function pointer")(parse as *mut libc::c_void);
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as *mut libc::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut libc::c_void);
    }
    if URI.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"failed build URL\n\0" as *const u8 as *const libc::c_char,
            0 as *const xmlChar,
        );
        return -(1 as libc::c_int);
    }
    fragment = xmlXIncludeGetProp(
        ctxt,
        cur,
        b"xpointer\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    uri = xmlParseURI(URI as *const libc::c_char);
    if uri.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
            URI,
        );
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(URI as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if !((*uri).fragment).is_null() {
        if (*ctxt).legacy != 0 as libc::c_int {
            if fragment.is_null() {
                fragment = (*uri).fragment as *mut xmlChar;
            } else {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*uri).fragment as *mut libc::c_void);
            }
        } else {
            xmlXIncludeErr(
                ctxt,
                cur,
                XML_XINCLUDE_FRAGMENT_ID as libc::c_int,
                b"Invalid fragment identifier in URI %s use the xpointer attribute\n\0"
                    as *const u8 as *const libc::c_char,
                URI,
            );
            if !fragment.is_null() {
                xmlFree
                    .expect("non-null function pointer")(fragment as *mut libc::c_void);
            }
            xmlFreeURI(uri);
            xmlFree.expect("non-null function pointer")(URI as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        (*uri).fragment = 0 as *mut libc::c_char;
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    xmlFree.expect("non-null function pointer")(URI as *mut libc::c_void);
    if URL.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
            URI,
        );
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    if local == 1 as libc::c_int && xml == 1 as libc::c_int
        && (fragment.is_null()
            || *fragment.offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int)
    {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_RECURSION as libc::c_int,
            b"detected a local recursion with no xpointer in %s\n\0" as *const u8
                as *const libc::c_char,
            URL,
        );
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    if local == 0 && xml == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*ctxt).urlNr {
            if xmlStrEqual(URL, *((*ctxt).urlTab).offset(i as isize)) != 0 {
                xmlXIncludeErr(
                    ctxt,
                    cur,
                    XML_XINCLUDE_RECURSION as libc::c_int,
                    b"detected a recursion in %s\n\0" as *const u8
                        as *const libc::c_char,
                    URL,
                );
                return -(1 as libc::c_int);
            }
            i += 1;
            i;
        }
    }
    ref_0 = xmlXIncludeNewRef(ctxt, URL, cur);
    if ref_0.is_null() {
        return -(1 as libc::c_int);
    }
    (*ref_0).fragment = fragment;
    (*ref_0).doc = 0 as xmlDocPtr;
    (*ref_0).xml = xml;
    (*ref_0).count = 1 as libc::c_int;
    xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeRecurseDoc(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut doc: xmlDocPtr,
    url: xmlURL,
) {
    let mut newctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut i: libc::c_int = 0;
    newctxt = xmlXIncludeNewContext(doc);
    if !newctxt.is_null() {
        (*newctxt)._private = (*ctxt)._private;
        (*newctxt).incMax = (*ctxt).incMax;
        (*newctxt).incNr = (*ctxt).incNr;
        (*newctxt)
            .incTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*newctxt).incMax as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<xmlXIncludeRefPtr>() as libc::c_ulong,
                ),
        ) as *mut xmlXIncludeRefPtr;
        if ((*newctxt).incTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                doc as xmlNodePtr,
                b"processing doc\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(newctxt as *mut libc::c_void);
            return;
        }
        (*newctxt).urlMax = (*ctxt).urlMax;
        (*newctxt).urlNr = (*ctxt).urlNr;
        (*newctxt).urlTab = (*ctxt).urlTab;
        (*newctxt).base = xmlStrdup((*ctxt).base);
        (*newctxt).incBase = (*ctxt).incNr;
        i = 0 as libc::c_int;
        while i < (*ctxt).incNr {
            let ref mut fresh5 = *((*newctxt).incTab).offset(i as isize);
            *fresh5 = *((*ctxt).incTab).offset(i as isize);
            let ref mut fresh6 = (**((*newctxt).incTab).offset(i as isize)).count;
            *fresh6 += 1;
            *fresh6;
            i += 1;
            i;
        }
        (*newctxt).parseFlags = (*ctxt).parseFlags;
        xmlXIncludeDoProcess(newctxt, doc, xmlDocGetRootElement(doc as *const xmlDoc));
        i = 0 as libc::c_int;
        while i < (*ctxt).incNr {
            let ref mut fresh7 = (**((*newctxt).incTab).offset(i as isize)).count;
            *fresh7 -= 1;
            *fresh7;
            let ref mut fresh8 = *((*newctxt).incTab).offset(i as isize);
            *fresh8 = 0 as xmlXIncludeRefPtr;
            i += 1;
            i;
        }
        (*ctxt).urlTab = (*newctxt).urlTab;
        (*ctxt).urlMax = (*newctxt).urlMax;
        (*newctxt).urlMax = 0 as libc::c_int;
        (*newctxt).urlNr = 0 as libc::c_int;
        (*newctxt).urlTab = 0 as *mut *mut xmlChar;
        xmlXIncludeFreeContext(newctxt);
    }
}
unsafe extern "C" fn xmlXIncludeAddTxt(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut txt: xmlNodePtr,
    url: xmlURL,
) {
    if (*ctxt).txtMax == 0 as libc::c_int {
        (*ctxt).txtMax = 4 as libc::c_int;
        (*ctxt)
            .txtTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).txtMax as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if ((*ctxt).txtTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                0 as xmlNodePtr,
                b"processing text\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        (*ctxt)
            .txturlTab = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).txtMax as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlURL>() as libc::c_ulong),
        ) as *mut xmlURL;
        if ((*ctxt).txturlTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                0 as xmlNodePtr,
                b"processing text\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    if (*ctxt).txtNr >= (*ctxt).txtMax {
        (*ctxt).txtMax *= 2 as libc::c_int;
        (*ctxt)
            .txtTab = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).txtTab as *mut libc::c_void,
            ((*ctxt).txtMax as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
        ) as *mut xmlNodePtr;
        if ((*ctxt).txtTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                0 as xmlNodePtr,
                b"processing text\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        (*ctxt)
            .txturlTab = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).txturlTab as *mut libc::c_void,
            ((*ctxt).txtMax as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlURL>() as libc::c_ulong),
        ) as *mut xmlURL;
        if ((*ctxt).txturlTab).is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                0 as xmlNodePtr,
                b"processing text\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    let ref mut fresh9 = *((*ctxt).txtTab).offset((*ctxt).txtNr as isize);
    *fresh9 = txt;
    let ref mut fresh10 = *((*ctxt).txturlTab).offset((*ctxt).txtNr as isize);
    *fresh10 = xmlStrdup(url as *const xmlChar);
    (*ctxt).txtNr += 1;
    (*ctxt).txtNr;
}
unsafe extern "C" fn xmlXIncludeCopyNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut target: xmlDocPtr,
    mut source: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> xmlNodePtr {
    let mut result: xmlNodePtr = 0 as xmlNodePtr;
    if ctxt.is_null() || target.is_null() || source.is_null() || elem.is_null() {
        return 0 as xmlNodePtr;
    }
    if (*elem).type_0 as libc::c_uint == XML_DTD_NODE as libc::c_int as libc::c_uint {
        return 0 as xmlNodePtr;
    }
    if (*elem).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        result = xmlXIncludeCopyNodeList(ctxt, target, source, (*elem).children);
    } else {
        result = xmlDocCopyNode(elem, target, 1 as libc::c_int);
    }
    return result;
}
unsafe extern "C" fn xmlXIncludeCopyNodeList(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut target: xmlDocPtr,
    mut source: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    let mut result: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    if ctxt.is_null() || target.is_null() || source.is_null() || elem.is_null() {
        return 0 as xmlNodePtr;
    }
    cur = elem;
    while !cur.is_null() {
        res = xmlXIncludeCopyNode(ctxt, target, source, cur);
        if !res.is_null() {
            if result.is_null() {
                last = res;
                result = last;
            } else {
                (*last).next = res;
                (*res).prev = last;
                last = res;
            }
        }
        cur = (*cur).next;
    }
    return result;
}
unsafe extern "C" fn xmlXIncludeGetNthChild(
    mut cur: xmlNodePtr,
    mut no: libc::c_int,
) -> xmlNodePtr {
    let mut i: libc::c_int = 0;
    if cur.is_null()
        || (*cur).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    cur = (*cur).children;
    i = 0 as libc::c_int;
    while i <= no {
        if cur.is_null() {
            return cur;
        }
        if (*cur).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            || (*cur).type_0 as libc::c_uint
                == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
            || (*cur).type_0 as libc::c_uint
                == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        {
            i += 1;
            i;
            if i == no {
                break;
            }
        }
        cur = (*cur).next;
    }
    return cur;
}
unsafe extern "C" fn xmlXIncludeCopyRange(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut target: xmlDocPtr,
    mut source: xmlDocPtr,
    mut range: xmlXPathObjectPtr,
) -> xmlNodePtr {
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut listParent: xmlNodePtr = 0 as xmlNodePtr;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp2: xmlNodePtr = 0 as *mut xmlNode;
    let mut start: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut end: xmlNodePtr = 0 as *mut xmlNode;
    let mut index1: libc::c_int = 0;
    let mut index2: libc::c_int = 0;
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut lastLevel: libc::c_int = 0 as libc::c_int;
    let mut endLevel: libc::c_int = 0 as libc::c_int;
    let mut endFlag: libc::c_int = 0 as libc::c_int;
    if ctxt.is_null() || target.is_null() || source.is_null() || range.is_null() {
        return 0 as xmlNodePtr;
    }
    if (*range).type_0 as libc::c_uint != XPATH_RANGE as libc::c_int as libc::c_uint {
        return 0 as xmlNodePtr;
    }
    start = (*range).user as xmlNodePtr;
    if start.is_null()
        || (*start).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    end = (*range).user2 as xmlNodePtr;
    if end.is_null() {
        return xmlDocCopyNode(start, target, 1 as libc::c_int);
    }
    if (*end).type_0 as libc::c_uint == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return 0 as xmlNodePtr;
    }
    cur = start;
    index1 = (*range).index;
    index2 = (*range).index2;
    while !cur.is_null() {
        if level < 0 as libc::c_int {
            while level < 0 as libc::c_int {
                tmp2 = xmlDocCopyNode(listParent, target, 2 as libc::c_int);
                xmlAddChild(tmp2, list);
                list = tmp2;
                listParent = (*listParent).parent;
                level += 1;
                level;
            }
            last = list;
            lastLevel = 0 as libc::c_int;
        }
        while level < lastLevel {
            last = (*last).parent;
            lastLevel -= 1;
            lastLevel;
        }
        if cur == end {
            if (*cur).type_0 as libc::c_uint
                == XML_TEXT_NODE as libc::c_int as libc::c_uint
            {
                let mut content: *const xmlChar = (*cur).content;
                let mut len: libc::c_int = 0;
                if content.is_null() {
                    tmp = xmlNewTextLen(0 as *const xmlChar, 0 as libc::c_int);
                } else {
                    len = index2;
                    if cur == start && index1 > 1 as libc::c_int {
                        content = content.offset((index1 - 1 as libc::c_int) as isize);
                        len -= index1 - 1 as libc::c_int;
                    } else {
                        len = index2;
                    }
                    tmp = xmlNewTextLen(content, len);
                }
                if list.is_null() {
                    return tmp;
                }
                if level == lastLevel {
                    xmlAddNextSibling(last, tmp);
                } else {
                    xmlAddChild(last, tmp);
                }
                return list;
            } else {
                endLevel = level;
                endFlag = 1 as libc::c_int;
                tmp = xmlDocCopyNode(cur, target, 2 as libc::c_int);
                if list.is_null() {
                    list = tmp;
                    listParent = (*cur).parent;
                } else if level == lastLevel {
                    xmlAddNextSibling(last, tmp);
                } else {
                    xmlAddChild(last, tmp);
                    lastLevel = level;
                }
                last = tmp;
                if index2 > 1 as libc::c_int {
                    end = xmlXIncludeGetNthChild(cur, index2 - 1 as libc::c_int);
                    index2 = 0 as libc::c_int;
                }
                if cur == start && index1 > 1 as libc::c_int {
                    cur = xmlXIncludeGetNthChild(cur, index1 - 1 as libc::c_int);
                    index1 = 0 as libc::c_int;
                } else {
                    cur = (*cur).children;
                }
                level += 1;
                level;
            }
        } else {
            if cur == start {
                if (*cur).type_0 as libc::c_uint
                    == XML_TEXT_NODE as libc::c_int as libc::c_uint
                    || (*cur).type_0 as libc::c_uint
                        == XML_CDATA_SECTION_NODE as libc::c_int as libc::c_uint
                {
                    let mut content_0: *const xmlChar = (*cur).content;
                    if content_0.is_null() {
                        tmp = xmlNewTextLen(0 as *const xmlChar, 0 as libc::c_int);
                    } else {
                        if index1 > 1 as libc::c_int {
                            content_0 = content_0
                                .offset((index1 - 1 as libc::c_int) as isize);
                            index1 = 0 as libc::c_int;
                        }
                        tmp = xmlNewText(content_0);
                    }
                    list = tmp;
                    last = list;
                    listParent = (*cur).parent;
                } else {
                    tmp = xmlDocCopyNode(cur, target, 2 as libc::c_int);
                    last = tmp;
                    list = last;
                    listParent = (*cur).parent;
                    if index1 > 1 as libc::c_int {
                        cur = xmlXIncludeGetNthChild(cur, index1 - 1 as libc::c_int);
                        lastLevel = 1 as libc::c_int;
                        level = lastLevel;
                        index1 = 0 as libc::c_int;
                        continue;
                    }
                }
            } else {
                tmp = 0 as xmlNodePtr;
                match (*cur).type_0 as libc::c_uint {
                    14 | 15 | 16 | 6 => {}
                    17 => {}
                    19 | 20 => {}
                    2 => {}
                    _ => {
                        tmp = xmlDocCopyNode(cur, target, 2 as libc::c_int);
                    }
                }
                if !tmp.is_null() {
                    if level == lastLevel {
                        xmlAddNextSibling(last, tmp);
                    } else {
                        xmlAddChild(last, tmp);
                        lastLevel = level;
                    }
                    last = tmp;
                }
            }
            cur = xmlXPtrAdvanceNode(cur, &mut level);
            if endFlag != 0 && level >= endLevel {
                break;
            }
        }
    }
    return list;
}
unsafe extern "C" fn xmlXIncludeCopyXPointer(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut target: xmlDocPtr,
    mut source: xmlDocPtr,
    mut obj: xmlXPathObjectPtr,
) -> xmlNodePtr {
    let mut list: xmlNodePtr = 0 as xmlNodePtr;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut i: libc::c_int = 0;
    if source.is_null() {
        source = (*ctxt).doc;
    }
    if ctxt.is_null() || target.is_null() || source.is_null() || obj.is_null() {
        return 0 as xmlNodePtr;
    }
    match (*obj).type_0 as libc::c_uint {
        1 => {
            let mut set: xmlNodeSetPtr = (*obj).nodesetval;
            if set.is_null() {
                return 0 as xmlNodePtr;
            }
            let mut current_block_22: u64;
            i = 0 as libc::c_int;
            while i < (*set).nodeNr {
                if !(*((*set).nodeTab).offset(i as isize)).is_null() {
                    match (**((*set).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                    {
                        19 => {
                            current_block_22 = 1841672684692190573;
                            match current_block_22 {
                                4775909272756257391 => {
                                    if last.is_null() {
                                        last = xmlXIncludeCopyNode(
                                            ctxt,
                                            target,
                                            source,
                                            *((*set).nodeTab).offset(i as isize),
                                        );
                                        list = last;
                                    } else {
                                        xmlAddNextSibling(
                                            last,
                                            xmlXIncludeCopyNode(
                                                ctxt,
                                                target,
                                                source,
                                                *((*set).nodeTab).offset(i as isize),
                                            ),
                                        );
                                        if !((*last).next).is_null() {
                                            last = (*last).next;
                                        }
                                    }
                                }
                                _ => {
                                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                                    let mut cur: xmlNodePtr = *((*set).nodeTab)
                                        .offset(i as isize);
                                    cur = (*cur).next;
                                    while !cur.is_null() {
                                        match (*cur).type_0 as libc::c_uint {
                                            3 | 4 | 1 | 5 | 6 | 7 | 8 => {}
                                            _ => {
                                                break;
                                            }
                                        }
                                        tmp = xmlXIncludeCopyNode(ctxt, target, source, cur);
                                        if last.is_null() {
                                            last = tmp;
                                            list = last;
                                        } else {
                                            xmlAddNextSibling(last, tmp);
                                            last = tmp;
                                        }
                                        cur = (*cur).next;
                                    }
                                }
                            }
                        }
                        2 | 18 | 10 | 11 | 12 | 14 | 15 | 16 | 17 => {}
                        3 | 4 | 1 | 5 | 6 | 7 | 8 | 9 | 13 | 21 | 20 | _ => {
                            current_block_22 = 4775909272756257391;
                            match current_block_22 {
                                4775909272756257391 => {
                                    if last.is_null() {
                                        last = xmlXIncludeCopyNode(
                                            ctxt,
                                            target,
                                            source,
                                            *((*set).nodeTab).offset(i as isize),
                                        );
                                        list = last;
                                    } else {
                                        xmlAddNextSibling(
                                            last,
                                            xmlXIncludeCopyNode(
                                                ctxt,
                                                target,
                                                source,
                                                *((*set).nodeTab).offset(i as isize),
                                            ),
                                        );
                                        if !((*last).next).is_null() {
                                            last = (*last).next;
                                        }
                                    }
                                }
                                _ => {
                                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                                    let mut cur: xmlNodePtr = *((*set).nodeTab)
                                        .offset(i as isize);
                                    cur = (*cur).next;
                                    while !cur.is_null() {
                                        match (*cur).type_0 as libc::c_uint {
                                            3 | 4 | 1 | 5 | 6 | 7 | 8 => {}
                                            _ => {
                                                break;
                                            }
                                        }
                                        tmp = xmlXIncludeCopyNode(ctxt, target, source, cur);
                                        if last.is_null() {
                                            last = tmp;
                                            list = last;
                                        } else {
                                            xmlAddNextSibling(last, tmp);
                                            last = tmp;
                                        }
                                        cur = (*cur).next;
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
                i;
            }
        }
        7 => {
            let mut set_0: xmlLocationSetPtr = (*obj).user as xmlLocationSetPtr;
            if set_0.is_null() {
                return 0 as xmlNodePtr;
            }
            i = 0 as libc::c_int;
            while i < (*set_0).locNr {
                if last.is_null() {
                    last = xmlXIncludeCopyXPointer(
                        ctxt,
                        target,
                        source,
                        *((*set_0).locTab).offset(i as isize),
                    );
                    list = last;
                } else {
                    xmlAddNextSibling(
                        last,
                        xmlXIncludeCopyXPointer(
                            ctxt,
                            target,
                            source,
                            *((*set_0).locTab).offset(i as isize),
                        ),
                    );
                }
                if !last.is_null() {
                    while !((*last).next).is_null() {
                        last = (*last).next;
                    }
                }
                i += 1;
                i;
            }
        }
        6 => return xmlXIncludeCopyRange(ctxt, target, source, obj),
        5 | _ => {}
    }
    return list;
}
unsafe extern "C" fn xmlXIncludeMergeEntity(
    mut payload: *mut libc::c_void,
    mut vdata: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut current_block: u64;
    let mut ent: xmlEntityPtr = payload as xmlEntityPtr;
    let mut data: xmlXIncludeMergeDataPtr = vdata as xmlXIncludeMergeDataPtr;
    let mut ret: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut prev: xmlEntityPtr = 0 as *mut xmlEntity;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    if ent.is_null() || data.is_null() {
        return;
    }
    ctxt = (*data).ctxt;
    doc = (*data).doc;
    if ctxt.is_null() || doc.is_null() {
        return;
    }
    match (*ent).etype as libc::c_uint {
        4 | 5 | 6 => return,
        1 | 2 | 3 | _ => {}
    }
    ret = xmlAddDocEntity(
        doc,
        (*ent).name,
        (*ent).etype as libc::c_int,
        (*ent).ExternalID,
        (*ent).SystemID,
        (*ent).content,
    );
    if !ret.is_null() {
        if !((*ent).URI).is_null() {
            (*ret).URI = xmlStrdup((*ent).URI);
        }
    } else {
        prev = xmlGetDocEntity(doc as *const xmlDoc, (*ent).name);
        if !prev.is_null() {
            if (*ent).etype as libc::c_uint != (*prev).etype as libc::c_uint {
                current_block = 2185874197629284460;
            } else if !((*ent).SystemID).is_null() && !((*prev).SystemID).is_null() {
                if xmlStrEqual((*ent).SystemID, (*prev).SystemID) == 0 {
                    current_block = 2185874197629284460;
                } else {
                    current_block = 9828876828309294594;
                }
            } else if !((*ent).ExternalID).is_null() && !((*prev).ExternalID).is_null() {
                if xmlStrEqual((*ent).ExternalID, (*prev).ExternalID) == 0 {
                    current_block = 2185874197629284460;
                } else {
                    current_block = 9828876828309294594;
                }
            } else if !((*ent).content).is_null() && !((*prev).content).is_null() {
                if xmlStrEqual((*ent).content, (*prev).content) == 0 {
                    current_block = 2185874197629284460;
                } else {
                    current_block = 9828876828309294594;
                }
            } else {
                current_block = 2185874197629284460;
            }
            match current_block {
                9828876828309294594 => {}
                _ => {
                    match (*ent).etype as libc::c_uint {
                        4 | 5 | 6 | 1 | 2 => return,
                        3 | _ => {}
                    }
                    xmlXIncludeErr(
                        ctxt,
                        ent as xmlNodePtr,
                        XML_XINCLUDE_ENTITY_DEF_MISMATCH as libc::c_int,
                        b"mismatch in redefinition of entity %s\n\0" as *const u8
                            as *const libc::c_char,
                        (*ent).name,
                    );
                    return;
                }
            }
        }
    };
}
unsafe extern "C" fn xmlXIncludeMergeEntities(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut doc: xmlDocPtr,
    mut from: xmlDocPtr,
) -> libc::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut target: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut source: xmlDtdPtr = 0 as *mut xmlDtd;
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if from.is_null() || ((*from).intSubset).is_null() {
        return 0 as libc::c_int;
    }
    target = (*doc).intSubset;
    if target.is_null() {
        cur = xmlDocGetRootElement(doc as *const xmlDoc);
        if cur.is_null() {
            return -(1 as libc::c_int);
        }
        target = xmlCreateIntSubset(
            doc,
            (*cur).name,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        if target.is_null() {
            return -(1 as libc::c_int);
        }
    }
    source = (*from).intSubset;
    if !source.is_null() && !((*source).entities).is_null() {
        let mut data: xmlXIncludeMergeData = _xmlXIncludeMergeData {
            doc: 0 as *mut xmlDoc,
            ctxt: 0 as *mut xmlXIncludeCtxt,
        };
        data.ctxt = ctxt;
        data.doc = doc;
        xmlHashScan(
            (*source).entities as xmlHashTablePtr,
            Some(
                xmlXIncludeMergeEntity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            &mut data as *mut xmlXIncludeMergeData as *mut libc::c_void,
        );
    }
    source = (*from).extSubset;
    if !source.is_null() && !((*source).entities).is_null() {
        let mut data_0: xmlXIncludeMergeData = _xmlXIncludeMergeData {
            doc: 0 as *mut xmlDoc,
            ctxt: 0 as *mut xmlXIncludeCtxt,
        };
        data_0.ctxt = ctxt;
        data_0.doc = doc;
        if xmlStrEqual((*target).ExternalID, (*source).ExternalID) == 0
            && xmlStrEqual((*target).SystemID, (*source).SystemID) == 0
        {
            xmlHashScan(
                (*source).entities as xmlHashTablePtr,
                Some(
                    xmlXIncludeMergeEntity
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                            *const xmlChar,
                        ) -> (),
                ),
                &mut data_0 as *mut xmlXIncludeMergeData as *mut libc::c_void,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeLoadDoc(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut url: *const xmlChar,
    mut nr: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut fragment: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut saveFlags: libc::c_int = 0;
    uri = xmlParseURI(url as *const libc::c_char);
    if uri.is_null() {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
            url,
        );
        return -(1 as libc::c_int);
    }
    if !((*uri).fragment).is_null() {
        fragment = (*uri).fragment as *mut xmlChar;
        (*uri).fragment = 0 as *mut libc::c_char;
    }
    if !((*ctxt).incTab).is_null() && !(*((*ctxt).incTab).offset(nr as isize)).is_null()
        && !((**((*ctxt).incTab).offset(nr as isize)).fragment).is_null()
    {
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
        }
        fragment = xmlStrdup((**((*ctxt).incTab).offset(nr as isize)).fragment);
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    if URL.is_null() {
        if !((*ctxt).incTab).is_null() {
            xmlXIncludeErr(
                ctxt,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                XML_XINCLUDE_HREF_URI as libc::c_int,
                b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
                url,
            );
        } else {
            xmlXIncludeErr(
                ctxt,
                0 as xmlNodePtr,
                XML_XINCLUDE_HREF_URI as libc::c_int,
                b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
                url,
            );
        }
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    if *URL.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        || *URL.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        || !((*ctxt).doc).is_null() && xmlStrEqual(URL, (*(*ctxt).doc).URL) != 0
    {
        doc = 0 as xmlDocPtr;
    } else {
        i = 0 as libc::c_int;
        loop {
            if !(i < (*ctxt).incNr) {
                current_block = 2232869372362427478;
                break;
            }
            if xmlStrEqual(URL, (**((*ctxt).incTab).offset(i as isize)).URI) != 0
                && !((**((*ctxt).incTab).offset(i as isize)).doc).is_null()
            {
                doc = (**((*ctxt).incTab).offset(i as isize)).doc;
                current_block = 16258539723651955374;
                break;
            } else {
                i += 1;
                i;
            }
        }
        match current_block {
            16258539723651955374 => {}
            _ => {
                saveFlags = (*ctxt).parseFlags;
                if !fragment.is_null() {
                    (*ctxt).parseFlags |= XML_PARSE_NOENT as libc::c_int;
                }
                doc = xmlXIncludeParseFile(ctxt, URL as *const libc::c_char);
                (*ctxt).parseFlags = saveFlags;
                if doc.is_null() {
                    xmlFree
                        .expect("non-null function pointer")(URL as *mut libc::c_void);
                    if !fragment.is_null() {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(fragment as *mut libc::c_void);
                    }
                    return -(1 as libc::c_int);
                }
                let ref mut fresh11 = (**((*ctxt).incTab).offset(nr as isize)).doc;
                *fresh11 = doc;
                if xmlStrEqual(URL, (*doc).URL) == 0 {
                    xmlFree
                        .expect("non-null function pointer")(URL as *mut libc::c_void);
                    URL = xmlStrdup((*doc).URL);
                }
                i = nr + 1 as libc::c_int;
                while i < (*ctxt).incNr {
                    if xmlStrEqual(URL, (**((*ctxt).incTab).offset(i as isize)).URI) != 0
                    {
                        let ref mut fresh12 = (**((*ctxt).incTab).offset(nr as isize))
                            .count;
                        *fresh12 += 1;
                        *fresh12;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                xmlXIncludeMergeEntities(ctxt, (*ctxt).doc, doc);
                xmlXIncludeRecurseDoc(ctxt, doc, URL);
            }
        }
    }
    if fragment.is_null() {
        if doc.is_null() {
            let ref mut fresh13 = (**((*ctxt).incTab).offset(nr as isize)).inc;
            *fresh13 = xmlCopyNodeList((*(*ctxt).doc).children);
        } else {
            let ref mut fresh14 = (**((*ctxt).incTab).offset(nr as isize)).inc;
            *fresh14 = xmlXIncludeCopyNodeList(ctxt, (*ctxt).doc, doc, (*doc).children);
        }
    } else {
        let mut xptr: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
        let mut xptrctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
        let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
        if doc.is_null() {
            xptrctxt = xmlXPtrNewContext(
                (*ctxt).doc,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                0 as xmlNodePtr,
            );
        } else {
            xptrctxt = xmlXPtrNewContext(doc, 0 as xmlNodePtr, 0 as xmlNodePtr);
        }
        if xptrctxt.is_null() {
            xmlXIncludeErr(
                ctxt,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                XML_XINCLUDE_XPTR_FAILED as libc::c_int,
                b"could not create XPointer context\n\0" as *const u8
                    as *const libc::c_char,
                0 as *const xmlChar,
            );
            xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        xptr = xmlXPtrEval(fragment, xptrctxt);
        if xptr.is_null() {
            xmlXIncludeErr(
                ctxt,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                XML_XINCLUDE_XPTR_FAILED as libc::c_int,
                b"XPointer evaluation failed: #%s\n\0" as *const u8
                    as *const libc::c_char,
                fragment,
            );
            xmlXPathFreeContext(xptrctxt);
            xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        match (*xptr).type_0 as libc::c_uint {
            0 | 2 | 3 | 4 | 5 | 8 | 9 => {
                xmlXIncludeErr(
                    ctxt,
                    (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                    XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                    b"XPointer is not a range: #%s\n\0" as *const u8
                        as *const libc::c_char,
                    fragment,
                );
                xmlXPathFreeContext(xptrctxt);
                xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
                xmlFree
                    .expect("non-null function pointer")(fragment as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            1 => {
                if ((*xptr).nodesetval).is_null()
                    || (*(*xptr).nodesetval).nodeNr <= 0 as libc::c_int
                {
                    xmlXPathFreeContext(xptrctxt);
                    xmlFree
                        .expect("non-null function pointer")(URL as *mut libc::c_void);
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(fragment as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
            }
            6 | 7 | _ => {}
        }
        set = (*xptr).nodesetval;
        if !set.is_null() {
            let mut current_block_94: u64;
            i = 0 as libc::c_int;
            while i < (*set).nodeNr {
                if !(*((*set).nodeTab).offset(i as isize)).is_null() {
                    match (**((*set).nodeTab).offset(i as isize)).type_0 as libc::c_uint
                    {
                        2 => {
                            current_block_94 = 13602255773016820604;
                            match current_block_94 {
                                13602255773016820604 => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects an attribute: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh15 = *((*set).nodeTab).offset(i as isize);
                                    *fresh15 = 0 as xmlNodePtr;
                                }
                                5155961325756815452 => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects a namespace: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh16 = *((*set).nodeTab).offset(i as isize);
                                    *fresh16 = 0 as xmlNodePtr;
                                }
                                _ => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects unexpected nodes: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh17 = *((*set).nodeTab).offset(i as isize);
                                    *fresh17 = 0 as xmlNodePtr;
                                    let ref mut fresh18 = *((*set).nodeTab).offset(i as isize);
                                    *fresh18 = 0 as xmlNodePtr;
                                }
                            }
                        }
                        18 => {
                            current_block_94 = 5155961325756815452;
                            match current_block_94 {
                                13602255773016820604 => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects an attribute: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh15 = *((*set).nodeTab).offset(i as isize);
                                    *fresh15 = 0 as xmlNodePtr;
                                }
                                5155961325756815452 => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects a namespace: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh16 = *((*set).nodeTab).offset(i as isize);
                                    *fresh16 = 0 as xmlNodePtr;
                                }
                                _ => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects unexpected nodes: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh17 = *((*set).nodeTab).offset(i as isize);
                                    *fresh17 = 0 as xmlNodePtr;
                                    let ref mut fresh18 = *((*set).nodeTab).offset(i as isize);
                                    *fresh18 = 0 as xmlNodePtr;
                                }
                            }
                        }
                        10 | 11 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
                            current_block_94 = 4674771282363326800;
                            match current_block_94 {
                                13602255773016820604 => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects an attribute: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh15 = *((*set).nodeTab).offset(i as isize);
                                    *fresh15 = 0 as xmlNodePtr;
                                }
                                5155961325756815452 => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects a namespace: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh16 = *((*set).nodeTab).offset(i as isize);
                                    *fresh16 = 0 as xmlNodePtr;
                                }
                                _ => {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_XPTR_RESULT as libc::c_int,
                                        b"XPointer selects unexpected nodes: #%s\n\0" as *const u8
                                            as *const libc::c_char,
                                        fragment,
                                    );
                                    let ref mut fresh17 = *((*set).nodeTab).offset(i as isize);
                                    *fresh17 = 0 as xmlNodePtr;
                                    let ref mut fresh18 = *((*set).nodeTab).offset(i as isize);
                                    *fresh18 = 0 as xmlNodePtr;
                                }
                            }
                        }
                        1 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 13 | 21 | _ => {}
                    }
                }
                i += 1;
                i;
            }
        }
        if doc.is_null() {
            let ref mut fresh19 = (**((*ctxt).incTab).offset(nr as isize)).xptr;
            *fresh19 = xptr;
            let ref mut fresh20 = (**((*ctxt).incTab).offset(nr as isize)).inc;
            *fresh20 = 0 as xmlNodePtr;
        } else {
            let ref mut fresh21 = (**((*ctxt).incTab).offset(nr as isize)).inc;
            *fresh21 = xmlXIncludeCopyXPointer(ctxt, (*ctxt).doc, doc, xptr);
            xmlXPathFreeObject(xptr);
        }
        xmlXPathFreeContext(xptrctxt);
        xmlFree.expect("non-null function pointer")(fragment as *mut libc::c_void);
    }
    if !doc.is_null() && !URL.is_null()
        && (*ctxt).parseFlags & XML_PARSE_NOBASEFIX as libc::c_int == 0
        && (*doc).parseFlags & XML_PARSE_NOBASEFIX as libc::c_int == 0
    {
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        let mut base: *mut xmlChar = 0 as *mut xmlChar;
        let mut curBase: *mut xmlChar = 0 as *mut xmlChar;
        base = xmlGetNsProp(
            (**((*ctxt).incTab).offset(nr as isize)).ref_0 as *const xmlNode,
            b"base\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            b"http://www.w3.org/XML/1998/namespace\0" as *const u8 as *const libc::c_char
                as *const xmlChar,
        );
        if base.is_null() {
            curBase = xmlBuildRelativeURI(URL, (*ctxt).base);
            if curBase.is_null() {
                xmlXIncludeErr(
                    ctxt,
                    (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                    XML_XINCLUDE_HREF_URI as libc::c_int,
                    b"trying to build relative URI from %s\n\0" as *const u8
                        as *const libc::c_char,
                    URL,
                );
            } else if (xmlStrchr(curBase, '/' as i32 as xmlChar)).is_null() {
                xmlFree
                    .expect("non-null function pointer")(curBase as *mut libc::c_void);
            } else {
                base = curBase;
            }
        }
        if !base.is_null() {
            node = (**((*ctxt).incTab).offset(nr as isize)).inc;
            while !node.is_null() {
                if (*node).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                {
                    curBase = xmlNodeGetBase((*node).doc, node as *const xmlNode);
                    if curBase.is_null() {
                        xmlNodeSetBase(node, base);
                    } else {
                        if xmlStrEqual(curBase, (*(*node).doc).URL) != 0 {
                            xmlNodeSetBase(node, base);
                        } else {
                            let mut xmlBase: *mut xmlChar = 0 as *mut xmlChar;
                            xmlBase = xmlGetNsProp(
                                node as *const xmlNode,
                                b"base\0" as *const u8 as *const libc::c_char
                                    as *mut xmlChar,
                                b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                    as *const libc::c_char as *const xmlChar,
                            );
                            if !xmlBase.is_null() {
                                let mut relBase: *mut xmlChar = 0 as *mut xmlChar;
                                relBase = xmlBuildURI(xmlBase, base);
                                if relBase.is_null() {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                        XML_XINCLUDE_HREF_URI as libc::c_int,
                                        b"trying to rebuild base from %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        xmlBase,
                                    );
                                } else {
                                    xmlNodeSetBase(node, relBase);
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(relBase as *mut libc::c_void);
                                }
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(xmlBase as *mut libc::c_void);
                            }
                        }
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(curBase as *mut libc::c_void);
                    }
                }
                node = (*node).next;
            }
            xmlFree.expect("non-null function pointer")(base as *mut libc::c_void);
        }
    }
    if nr < (*ctxt).incNr && !((**((*ctxt).incTab).offset(nr as isize)).doc).is_null()
        && (**((*ctxt).incTab).offset(nr as isize)).count <= 1 as libc::c_int
    {
        xmlFreeDoc((**((*ctxt).incTab).offset(nr as isize)).doc);
        let ref mut fresh22 = (**((*ctxt).incTab).offset(nr as isize)).doc;
        *fresh22 = 0 as xmlDocPtr;
    }
    xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeLoadTxt(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut url: *const xmlChar,
    mut nr: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: libc::c_int = 0;
    let mut encoding: *mut xmlChar = 0 as *mut xmlChar;
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut pctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut xinclude_multibyte_fallback_used: libc::c_int = 0 as libc::c_int;
    uri = xmlParseURI(url as *const libc::c_char);
    if uri.is_null() {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
            url,
        );
        return -(1 as libc::c_int);
    }
    if !((*uri).fragment).is_null() {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_TEXT_FRAGMENT as libc::c_int,
            b"fragment identifier forbidden for text: %s\n\0" as *const u8
                as *const libc::c_char,
            (*uri).fragment as *const xmlChar,
        );
        xmlFreeURI(uri);
        return -(1 as libc::c_int);
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    if URL.is_null() {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const libc::c_char,
            url,
        );
        return -(1 as libc::c_int);
    }
    if *URL.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_TEXT_DOCUMENT as libc::c_int,
            b"text serialization of document not available\n\0" as *const u8
                as *const libc::c_char,
            0 as *const xmlChar,
        );
        xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    loop {
        if !(i < (*ctxt).txtNr) {
            current_block = 5783071609795492627;
            break;
        }
        if xmlStrEqual(URL, *((*ctxt).txturlTab).offset(i as isize) as *const xmlChar)
            != 0
        {
            node = xmlCopyNode(*((*ctxt).txtTab).offset(i as isize), 1 as libc::c_int);
            current_block = 10107863781968144882;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        5783071609795492627 => {
            if !(*((*ctxt).incTab).offset(nr as isize)).is_null()
                && !((**((*ctxt).incTab).offset(nr as isize)).ref_0).is_null()
            {
                encoding = xmlGetProp(
                    (**((*ctxt).incTab).offset(nr as isize)).ref_0 as *const xmlNode,
                    b"encoding\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
            }
            if !encoding.is_null() {
                enc = xmlParseCharEncoding(encoding as *const libc::c_char);
                if enc as libc::c_int == XML_CHAR_ENCODING_ERROR as libc::c_int {
                    xmlXIncludeErr(
                        ctxt,
                        (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                        XML_XINCLUDE_UNKNOWN_ENCODING as libc::c_int,
                        b"encoding %s not supported\n\0" as *const u8
                            as *const libc::c_char,
                        encoding,
                    );
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(encoding as *mut libc::c_void);
                    xmlFree
                        .expect("non-null function pointer")(URL as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
                xmlFree
                    .expect("non-null function pointer")(encoding as *mut libc::c_void);
            }
            pctxt = xmlNewParserCtxt();
            inputStream = xmlLoadExternalEntity(
                URL as *const libc::c_char,
                0 as *const libc::c_char,
                pctxt,
            );
            if inputStream.is_null() {
                xmlFreeParserCtxt(pctxt);
                xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            buf = (*inputStream).buf;
            if buf.is_null() {
                xmlFreeInputStream(inputStream);
                xmlFreeParserCtxt(pctxt);
                xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if !((*buf).encoder).is_null() {
                xmlCharEncCloseFunc((*buf).encoder);
            }
            (*buf).encoder = xmlGetCharEncodingHandler(enc);
            node = xmlNewText(0 as *const xmlChar);
            's_215: while xmlParserInputBufferRead(buf, 128 as libc::c_int)
                > 0 as libc::c_int
            {
                let mut len: libc::c_int = 0;
                let mut content: *const xmlChar = 0 as *const xmlChar;
                content = xmlBufContent((*buf).buffer as *const xmlBuf);
                len = xmlBufLength((*buf).buffer) as libc::c_int;
                i = 0 as libc::c_int;
                while i < len {
                    let mut cur: libc::c_int = 0;
                    let mut l: libc::c_int = 0;
                    cur = xmlStringCurrentChar(
                        0 as xmlParserCtxtPtr,
                        &*content.offset(i as isize),
                        &mut l,
                    );
                    if if cur < 0x100 as libc::c_int {
                        (0x9 as libc::c_int <= cur && cur <= 0xa as libc::c_int
                            || cur == 0xd as libc::c_int || 0x20 as libc::c_int <= cur)
                            as libc::c_int
                    } else {
                        (0x100 as libc::c_int <= cur && cur <= 0xd7ff as libc::c_int
                            || 0xe000 as libc::c_int <= cur
                                && cur <= 0xfffd as libc::c_int
                            || 0x10000 as libc::c_int <= cur
                                && cur <= 0x10ffff as libc::c_int) as libc::c_int
                    } == 0
                    {
                        if len - i < 4 as libc::c_int
                            && xinclude_multibyte_fallback_used == 0
                        {
                            xinclude_multibyte_fallback_used = 1 as libc::c_int;
                            xmlBufShrink((*buf).buffer, i as size_t);
                            continue 's_215;
                        } else {
                            xmlXIncludeErr(
                                ctxt,
                                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                                XML_XINCLUDE_INVALID_CHAR as libc::c_int,
                                b"%s contains invalid char\n\0" as *const u8
                                    as *const libc::c_char,
                                URL,
                            );
                            xmlFreeParserInputBuffer(buf);
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(URL as *mut libc::c_void);
                            return -(1 as libc::c_int);
                        }
                    } else {
                        xinclude_multibyte_fallback_used = 0 as libc::c_int;
                        xmlNodeAddContentLen(node, &*content.offset(i as isize), l);
                        i += l;
                    }
                }
                xmlBufShrink((*buf).buffer, len as size_t);
            }
            xmlFreeParserCtxt(pctxt);
            xmlXIncludeAddTxt(ctxt, node, URL);
            xmlFreeInputStream(inputStream);
        }
        _ => {}
    }
    let ref mut fresh23 = (**((*ctxt).incTab).offset(nr as isize)).inc;
    *fresh23 = node;
    xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeLoadFallback(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut fallback: xmlNodePtr,
    mut nr: libc::c_int,
) -> libc::c_int {
    let mut newctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if fallback.is_null()
        || (*fallback).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint || ctxt.is_null()
    {
        return -(1 as libc::c_int);
    }
    if !((*fallback).children).is_null() {
        newctxt = xmlXIncludeNewContext((*ctxt).doc);
        if newctxt.is_null() {
            return -(1 as libc::c_int);
        }
        (*newctxt)._private = (*ctxt)._private;
        (*newctxt).base = xmlStrdup((*ctxt).base);
        xmlXIncludeSetFlags(newctxt, (*ctxt).parseFlags);
        ret = xmlXIncludeDoProcess(newctxt, (*ctxt).doc, (*fallback).children);
        if (*ctxt).nbErrors > 0 as libc::c_int {
            ret = -(1 as libc::c_int);
        } else if ret > 0 as libc::c_int {
            ret = 0 as libc::c_int;
        }
        xmlXIncludeFreeContext(newctxt);
        let ref mut fresh24 = (**((*ctxt).incTab).offset(nr as isize)).inc;
        *fresh24 = xmlDocCopyNodeList((*ctxt).doc, (*fallback).children);
    } else {
        let ref mut fresh25 = (**((*ctxt).incTab).offset(nr as isize)).inc;
        *fresh25 = 0 as xmlNodePtr;
        (**((*ctxt).incTab).offset(nr as isize)).emptyFb = 1 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn xmlXIncludePreProcessNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlNodePtr {
    xmlXIncludeAddNode(ctxt, node);
    return 0 as xmlNodePtr;
}
unsafe extern "C" fn xmlXIncludeLoadNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut nr: libc::c_int,
) -> libc::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut parse: *mut xmlChar = 0 as *mut xmlChar;
    let mut base: *mut xmlChar = 0 as *mut xmlChar;
    let mut oldBase: *mut xmlChar = 0 as *mut xmlChar;
    let mut URI: *mut xmlChar = 0 as *mut xmlChar;
    let mut xml: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = 0;
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if nr < 0 as libc::c_int || nr >= (*ctxt).incNr {
        return -(1 as libc::c_int);
    }
    cur = (**((*ctxt).incTab).offset(nr as isize)).ref_0;
    if cur.is_null() {
        return -(1 as libc::c_int);
    }
    href = xmlXIncludeGetProp(
        ctxt,
        cur,
        b"href\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if href.is_null() {
        href = xmlStrdup(b"\0" as *const u8 as *const libc::c_char as *mut xmlChar);
        if href.is_null() {
            return -(1 as libc::c_int);
        }
    }
    parse = xmlXIncludeGetProp(
        ctxt,
        cur,
        b"parse\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if !parse.is_null() {
        if xmlStrEqual(
            parse,
            b"xml\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) != 0
        {
            xml = 1 as libc::c_int;
        } else if xmlStrEqual(
            parse,
            b"text\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) != 0
        {
            xml = 0 as libc::c_int;
        } else {
            xmlXIncludeErr(
                ctxt,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                XML_XINCLUDE_PARSE_VALUE as libc::c_int,
                b"invalid value %s for 'parse'\n\0" as *const u8 as *const libc::c_char,
                parse,
            );
            if !href.is_null() {
                xmlFree.expect("non-null function pointer")(href as *mut libc::c_void);
            }
            if !parse.is_null() {
                xmlFree.expect("non-null function pointer")(parse as *mut libc::c_void);
            }
            return -(1 as libc::c_int);
        }
    }
    base = xmlNodeGetBase((*ctxt).doc as *const xmlDoc, cur as *const xmlNode);
    if base.is_null() {
        URI = xmlBuildURI(href, (*(*ctxt).doc).URL);
    } else {
        URI = xmlBuildURI(href, base);
    }
    if URI.is_null() {
        let mut escbase: *mut xmlChar = 0 as *mut xmlChar;
        let mut eschref: *mut xmlChar = 0 as *mut xmlChar;
        escbase = xmlURIEscape(base);
        eschref = xmlURIEscape(href);
        URI = xmlBuildURI(eschref, escbase);
        if !escbase.is_null() {
            xmlFree.expect("non-null function pointer")(escbase as *mut libc::c_void);
        }
        if !eschref.is_null() {
            xmlFree.expect("non-null function pointer")(eschref as *mut libc::c_void);
        }
    }
    if URI.is_null() {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_HREF_URI as libc::c_int,
            b"failed build URL\n\0" as *const u8 as *const libc::c_char,
            0 as *const xmlChar,
        );
        if !parse.is_null() {
            xmlFree.expect("non-null function pointer")(parse as *mut libc::c_void);
        }
        if !href.is_null() {
            xmlFree.expect("non-null function pointer")(href as *mut libc::c_void);
        }
        if !base.is_null() {
            xmlFree.expect("non-null function pointer")(base as *mut libc::c_void);
        }
        return -(1 as libc::c_int);
    }
    oldBase = (*ctxt).base;
    (*ctxt).base = base;
    if xml != 0 {
        ret = xmlXIncludeLoadDoc(ctxt, URI, nr);
    } else {
        ret = xmlXIncludeLoadTxt(ctxt, URI, nr);
    }
    (*ctxt).base = oldBase;
    if ret < 0 as libc::c_int {
        let mut children: xmlNodePtr = 0 as *mut xmlNode;
        children = (*cur).children;
        while !children.is_null() {
            if (*children).type_0 as libc::c_uint
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                && !((*children).ns).is_null()
                && xmlStrEqual(
                    (*children).name,
                    b"fallback\0" as *const u8 as *const libc::c_char as *const xmlChar,
                ) != 0
                && (xmlStrEqual(
                    (*(*children).ns).href,
                    b"http://www.w3.org/2003/XInclude\0" as *const u8
                        as *const libc::c_char as *const xmlChar,
                ) != 0
                    || xmlStrEqual(
                        (*(*children).ns).href,
                        b"http://www.w3.org/2001/XInclude\0" as *const u8
                            as *const libc::c_char as *const xmlChar,
                    ) != 0)
            {
                ret = xmlXIncludeLoadFallback(ctxt, children, nr);
                if ret == 0 as libc::c_int {
                    break;
                }
            }
            children = (*children).next;
        }
    }
    if ret < 0 as libc::c_int {
        xmlXIncludeErr(
            ctxt,
            (**((*ctxt).incTab).offset(nr as isize)).ref_0,
            XML_XINCLUDE_NO_FALLBACK as libc::c_int,
            b"could not load %s, and no fallback was found\n\0" as *const u8
                as *const libc::c_char,
            URI,
        );
    }
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut libc::c_void);
    }
    if !parse.is_null() {
        xmlFree.expect("non-null function pointer")(parse as *mut libc::c_void);
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as *mut libc::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeIncludeNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut nr: libc::c_int,
) -> libc::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut end: xmlNodePtr = 0 as *mut xmlNode;
    let mut list: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if nr < 0 as libc::c_int || nr >= (*ctxt).incNr {
        return -(1 as libc::c_int);
    }
    cur = (**((*ctxt).incTab).offset(nr as isize)).ref_0;
    if cur.is_null()
        || (*cur).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if ((**((*ctxt).incTab).offset(nr as isize)).inc).is_null()
        && !((**((*ctxt).incTab).offset(nr as isize)).xptr).is_null()
    {
        let ref mut fresh26 = (**((*ctxt).incTab).offset(nr as isize)).inc;
        *fresh26 = xmlXIncludeCopyXPointer(
            ctxt,
            (*ctxt).doc,
            (*ctxt).doc,
            (**((*ctxt).incTab).offset(nr as isize)).xptr,
        );
        xmlXPathFreeObject((**((*ctxt).incTab).offset(nr as isize)).xptr);
        let ref mut fresh27 = (**((*ctxt).incTab).offset(nr as isize)).xptr;
        *fresh27 = 0 as xmlXPathObjectPtr;
    }
    list = (**((*ctxt).incTab).offset(nr as isize)).inc;
    let ref mut fresh28 = (**((*ctxt).incTab).offset(nr as isize)).inc;
    *fresh28 = 0 as xmlNodePtr;
    if !((*cur).parent).is_null()
        && (*(*cur).parent).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        let mut nb_elem: libc::c_int = 0 as libc::c_int;
        tmp = list;
        while !tmp.is_null() {
            if (*tmp).type_0 as libc::c_uint
                == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            {
                nb_elem += 1;
                nb_elem;
            }
            tmp = (*tmp).next;
        }
        if nb_elem > 1 as libc::c_int {
            xmlXIncludeErr(
                ctxt,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                XML_XINCLUDE_MULTIPLE_ROOT as libc::c_int,
                b"XInclude error: would result in multiple root nodes\n\0" as *const u8
                    as *const libc::c_char,
                0 as *const xmlChar,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*ctxt).parseFlags & XML_PARSE_NOXINCNODE as libc::c_int != 0 {
        while !list.is_null() {
            end = list;
            list = (*list).next;
            xmlAddPrevSibling(cur, end);
        }
        xmlUnlinkNode(cur);
        xmlFreeNode(cur);
    } else {
        (*cur).type_0 = XML_XINCLUDE_START;
        end = xmlNewDocNode((*cur).doc, (*cur).ns, (*cur).name, 0 as *const xmlChar);
        if end.is_null() {
            xmlXIncludeErr(
                ctxt,
                (**((*ctxt).incTab).offset(nr as isize)).ref_0,
                XML_XINCLUDE_BUILD_FAILED as libc::c_int,
                b"failed to build node\n\0" as *const u8 as *const libc::c_char,
                0 as *const xmlChar,
            );
            return -(1 as libc::c_int);
        }
        (*end).type_0 = XML_XINCLUDE_END;
        xmlAddNextSibling(cur, end);
        while !list.is_null() {
            cur = list;
            list = (*list).next;
            xmlAddPrevSibling(end, cur);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeTestNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
) -> libc::c_int {
    if node.is_null() {
        return 0 as libc::c_int;
    }
    if (*node).type_0 as libc::c_uint != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if ((*node).ns).is_null() {
        return 0 as libc::c_int;
    }
    if xmlStrEqual(
        (*(*node).ns).href,
        b"http://www.w3.org/2003/XInclude\0" as *const u8 as *const libc::c_char
            as *const xmlChar,
    ) != 0
        || xmlStrEqual(
            (*(*node).ns).href,
            b"http://www.w3.org/2001/XInclude\0" as *const u8 as *const libc::c_char
                as *const xmlChar,
        ) != 0
    {
        if xmlStrEqual(
            (*(*node).ns).href,
            b"http://www.w3.org/2001/XInclude\0" as *const u8 as *const libc::c_char
                as *const xmlChar,
        ) != 0
        {
            if (*ctxt).legacy == 0 as libc::c_int {
                (*ctxt).legacy = 1 as libc::c_int;
            }
        }
        if xmlStrEqual(
            (*node).name,
            b"include\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) != 0
        {
            let mut child: xmlNodePtr = (*node).children;
            let mut nb_fallback: libc::c_int = 0 as libc::c_int;
            while !child.is_null() {
                if (*child).type_0 as libc::c_uint
                    == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    && !((*child).ns).is_null()
                    && (xmlStrEqual(
                        (*(*child).ns).href,
                        b"http://www.w3.org/2003/XInclude\0" as *const u8
                            as *const libc::c_char as *const xmlChar,
                    ) != 0
                        || xmlStrEqual(
                            (*(*child).ns).href,
                            b"http://www.w3.org/2001/XInclude\0" as *const u8
                                as *const libc::c_char as *const xmlChar,
                        ) != 0)
                {
                    if xmlStrEqual(
                        (*child).name,
                        b"include\0" as *const u8 as *const libc::c_char
                            as *const xmlChar,
                    ) != 0
                    {
                        xmlXIncludeErr(
                            ctxt,
                            node,
                            XML_XINCLUDE_INCLUDE_IN_INCLUDE as libc::c_int,
                            b"%s has an 'include' child\n\0" as *const u8
                                as *const libc::c_char,
                            b"include\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        );
                        return 0 as libc::c_int;
                    }
                    if xmlStrEqual(
                        (*child).name,
                        b"fallback\0" as *const u8 as *const libc::c_char
                            as *const xmlChar,
                    ) != 0
                    {
                        nb_fallback += 1;
                        nb_fallback;
                    }
                }
                child = (*child).next;
            }
            if nb_fallback > 1 as libc::c_int {
                xmlXIncludeErr(
                    ctxt,
                    node,
                    XML_XINCLUDE_FALLBACKS_IN_INCLUDE as libc::c_int,
                    b"%s has multiple fallback children\n\0" as *const u8
                        as *const libc::c_char,
                    b"include\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        if xmlStrEqual(
            (*node).name,
            b"fallback\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) != 0
        {
            if ((*node).parent).is_null()
                || (*(*node).parent).type_0 as libc::c_uint
                    != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                || ((*(*node).parent).ns).is_null()
                || xmlStrEqual(
                    (*(*(*node).parent).ns).href,
                    b"http://www.w3.org/2003/XInclude\0" as *const u8
                        as *const libc::c_char as *const xmlChar,
                ) == 0
                    && xmlStrEqual(
                        (*(*(*node).parent).ns).href,
                        b"http://www.w3.org/2001/XInclude\0" as *const u8
                            as *const libc::c_char as *const xmlChar,
                    ) == 0
                || xmlStrEqual(
                    (*(*node).parent).name,
                    b"include\0" as *const u8 as *const libc::c_char as *const xmlChar,
                ) == 0
            {
                xmlXIncludeErr(
                    ctxt,
                    node,
                    XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE as libc::c_int,
                    b"%s is not the child of an 'include'\n\0" as *const u8
                        as *const libc::c_char,
                    b"fallback\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlXIncludeDoProcess(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut doc: xmlDocPtr,
    mut tree: xmlNodePtr,
) -> libc::c_int {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    if doc.is_null() || tree.is_null()
        || (*tree).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*doc).URL).is_null() {
        ret = xmlXIncludeURLPush(ctxt, (*doc).URL);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    start = (*ctxt).incNr;
    cur = tree;
    if xmlXIncludeTestNode(ctxt, cur) == 1 as libc::c_int {
        xmlXIncludePreProcessNode(ctxt, cur);
    }
    while !cur.is_null() && cur != (*tree).parent {
        if !((*cur).children).is_null()
            && (*(*cur).children).type_0 as libc::c_uint
                != XML_ENTITY_DECL as libc::c_int as libc::c_uint
            && (*(*cur).children).type_0 as libc::c_uint
                != XML_XINCLUDE_START as libc::c_int as libc::c_uint
            && (*(*cur).children).type_0 as libc::c_uint
                != XML_XINCLUDE_END as libc::c_int as libc::c_uint
        {
            cur = (*cur).children;
            if xmlXIncludeTestNode(ctxt, cur) != 0 {
                xmlXIncludePreProcessNode(ctxt, cur);
            }
        } else if !((*cur).next).is_null() {
            cur = (*cur).next;
            if xmlXIncludeTestNode(ctxt, cur) != 0 {
                xmlXIncludePreProcessNode(ctxt, cur);
            }
        } else {
            if cur == tree {
                break;
            }
            loop {
                cur = (*cur).parent;
                if cur.is_null() || cur == (*tree).parent {
                    break;
                }
                if !((*cur).next).is_null() {
                    cur = (*cur).next;
                    if xmlXIncludeTestNode(ctxt, cur) != 0 {
                        xmlXIncludePreProcessNode(ctxt, cur);
                    }
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    i = start;
    while i < (*ctxt).incNr {
        xmlXIncludeLoadNode(ctxt, i);
        ret += 1;
        ret;
        i += 1;
        i;
    }
    i = (*ctxt).incBase;
    while i < (*ctxt).incNr {
        if !((**((*ctxt).incTab).offset(i as isize)).inc).is_null()
            || !((**((*ctxt).incTab).offset(i as isize)).xptr).is_null()
            || (**((*ctxt).incTab).offset(i as isize)).emptyFb != 0 as libc::c_int
        {
            xmlXIncludeIncludeNode(ctxt, i);
        }
        i += 1;
        i;
    }
    if !((*doc).URL).is_null() {
        xmlXIncludeURLPop(ctxt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeSetFlags(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut flags: libc::c_int,
) -> libc::c_int {
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    (*ctxt).parseFlags = flags;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTreeFlagsData(
    mut tree: xmlNodePtr,
    mut flags: libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut ctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if tree.is_null()
        || (*tree).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        || ((*tree).doc).is_null()
    {
        return -(1 as libc::c_int);
    }
    ctxt = xmlXIncludeNewContext((*tree).doc);
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    (*ctxt)._private = data;
    (*ctxt).base = xmlStrdup((*(*tree).doc).URL as *mut xmlChar);
    xmlXIncludeSetFlags(ctxt, flags);
    ret = xmlXIncludeDoProcess(ctxt, (*tree).doc, tree);
    if ret >= 0 as libc::c_int && (*ctxt).nbErrors > 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    xmlXIncludeFreeContext(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessFlagsData(
    mut doc: xmlDocPtr,
    mut flags: libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut tree: xmlNodePtr = 0 as *mut xmlNode;
    if doc.is_null() {
        return -(1 as libc::c_int);
    }
    tree = xmlDocGetRootElement(doc as *const xmlDoc);
    if tree.is_null() {
        return -(1 as libc::c_int);
    }
    return xmlXIncludeProcessTreeFlagsData(tree, flags, data);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessFlags(
    mut doc: xmlDocPtr,
    mut flags: libc::c_int,
) -> libc::c_int {
    return xmlXIncludeProcessFlagsData(doc, flags, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcess(mut doc: xmlDocPtr) -> libc::c_int {
    return xmlXIncludeProcessFlags(doc, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTreeFlags(
    mut tree: xmlNodePtr,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut ctxt: xmlXIncludeCtxtPtr = 0 as *mut xmlXIncludeCtxt;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if tree.is_null()
        || (*tree).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        || ((*tree).doc).is_null()
    {
        return -(1 as libc::c_int);
    }
    ctxt = xmlXIncludeNewContext((*tree).doc);
    if ctxt.is_null() {
        return -(1 as libc::c_int);
    }
    (*ctxt).base = xmlNodeGetBase((*tree).doc, tree as *const xmlNode);
    xmlXIncludeSetFlags(ctxt, flags);
    ret = xmlXIncludeDoProcess(ctxt, (*tree).doc, tree);
    if ret >= 0 as libc::c_int && (*ctxt).nbErrors > 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    xmlXIncludeFreeContext(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTree(mut tree: xmlNodePtr) -> libc::c_int {
    return xmlXIncludeProcessTreeFlags(tree, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if node.is_null()
        || (*node).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        || ((*node).doc).is_null() || ctxt.is_null()
    {
        return -(1 as libc::c_int);
    }
    ret = xmlXIncludeDoProcess(ctxt, (*node).doc, node);
    if ret >= 0 as libc::c_int && (*ctxt).nbErrors > 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    return ret;
}
