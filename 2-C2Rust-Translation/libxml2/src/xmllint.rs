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
    pub type _xmlRelaxNGParserCtxt;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchema;
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
    pub type _xmlStreamCtxt;
    pub type _xmlXPathCompExpr;
    pub type _xmlSchematron;
    pub type _xmlSchematronParserCtxt;
    pub type _xmlSchematronValidCtxt;
    pub type _xmlSaveCtxt;
    fn xmlCheckVersion(version: libc::c_int);
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn xmlParserInputBufferCreateFilename(
        URI: *const libc::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlNoNetExternalEntityLoader(
        URL: *const libc::c_char,
        ID: *const libc::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlCopyDoc(doc: xmlDocPtr, recursive: libc::c_int) -> xmlDocPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlDocSetRootElement(doc: xmlDocPtr, root: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlNodeSetContent(cur: xmlNodePtr, content: *const xmlChar);
    fn xmlDocDumpFormatMemory(
        cur: xmlDocPtr,
        mem: *mut *mut xmlChar,
        size: *mut libc::c_int,
        format_0: libc::c_int,
    );
    fn xmlDocDumpMemory(cur: xmlDocPtr, mem: *mut *mut xmlChar, size: *mut libc::c_int);
    fn xmlDocDumpMemoryEnc(
        out_doc: xmlDocPtr,
        doc_txt_ptr: *mut *mut xmlChar,
        doc_txt_len: *mut libc::c_int,
        txt_encoding: *const libc::c_char,
    );
    fn xmlDocDumpFormatMemoryEnc(
        out_doc: xmlDocPtr,
        doc_txt_ptr: *mut *mut xmlChar,
        doc_txt_len: *mut libc::c_int,
        txt_encoding: *const libc::c_char,
        format_0: libc::c_int,
    );
    fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> libc::c_int;
    fn xmlSaveFile(filename: *const libc::c_char, cur: xmlDocPtr) -> libc::c_int;
    fn xmlSaveFormatFile(
        filename: *const libc::c_char,
        cur: xmlDocPtr,
        format_0: libc::c_int,
    ) -> libc::c_int;
    fn xmlSaveFormatFileEnc(
        filename: *const libc::c_char,
        cur: xmlDocPtr,
        encoding_0: *const libc::c_char,
        format_0: libc::c_int,
    ) -> libc::c_int;
    fn xmlSaveFileEnc(
        filename: *const libc::c_char,
        cur: xmlDocPtr,
        encoding_0: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlSetCompressMode(mode: libc::c_int);
    fn xmlAddEncodingAlias(
        name: *const libc::c_char,
        alias: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlCleanupParser();
    fn xmlParseFile(filename: *const libc::c_char) -> xmlDocPtr;
    fn xmlSubstituteEntitiesDefault(val: libc::c_int) -> libc::c_int;
    fn xmlKeepBlanksDefault(val: libc::c_int) -> libc::c_int;
    fn xmlPedanticParserDefault(val: libc::c_int) -> libc::c_int;
    fn xmlLineNumbersDefault(val: libc::c_int) -> libc::c_int;
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> libc::c_int;
    fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar) -> xmlDtdPtr;
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlCreatePushParserCtxt(
        sax_0: xmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        chunk: *const libc::c_char,
        size: libc::c_int,
        filename: *const libc::c_char,
    ) -> xmlParserCtxtPtr;
    fn xmlParseChunk(
        ctxt: xmlParserCtxtPtr,
        chunk: *const libc::c_char,
        size: libc::c_int,
        terminate: libc::c_int,
    ) -> libc::c_int;
    fn xmlNewIOInputStream(
        ctxt: xmlParserCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
    ) -> xmlParserInputPtr;
    fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader);
    fn xmlGetExternalEntityLoader() -> xmlExternalEntityLoader;
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options_0: libc::c_int) -> libc::c_int;
    fn xmlReadFile(
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlReadMemory(
        buffer: *const libc::c_char,
        size: libc::c_int,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlReadFd(
        fd: libc::c_int,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlReadIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut libc::c_void,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlCtxtReadFile(
        ctxt: xmlParserCtxtPtr,
        filename: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlCtxtReadMemory(
        ctxt: xmlParserCtxtPtr,
        buffer: *const libc::c_char,
        size: libc::c_int,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlCtxtReadIO(
        ctxt: xmlParserCtxtPtr,
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut libc::c_void,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlDocPtr;
    fn xmlHasFeature(feature: xmlFeature) -> libc::c_int;
    fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
    fn xmlNewValidCtxt() -> xmlValidCtxtPtr;
    fn xmlFreeValidCtxt(_: xmlValidCtxtPtr);
    fn xmlValidateDtd(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        dtd: xmlDtdPtr,
    ) -> libc::c_int;
    fn xmlValidateDocument(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> libc::c_int;
    fn xmlValidGetValidElements(
        prev: *mut xmlNode,
        next: *mut xmlNode,
        names: *mut *const xmlChar,
        max: libc::c_int,
    ) -> libc::c_int;
    fn xmlMemSetup(
        freeFunc: xmlFreeFunc,
        mallocFunc: xmlMallocFunc,
        reallocFunc: xmlReallocFunc,
        strdupFunc: xmlStrdupFunc,
    ) -> libc::c_int;
    fn xmlMemUsed() -> libc::c_int;
    fn xmlMemoryDump();
    fn xmlMemMalloc(size: size_t) -> *mut libc::c_void;
    fn xmlMemRealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xmlMemFree(ptr: *mut libc::c_void);
    fn xmlMemoryStrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn xmlSAXDefaultVersion(version: libc::c_int) -> libc::c_int;
    fn xmlRegisterNodeDefault(func: xmlRegisterNodeFunc) -> xmlRegisterNodeFunc;
    fn xmlDeregisterNodeDefault(func: xmlDeregisterNodeFunc) -> xmlDeregisterNodeFunc;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlDoValidityCheckingDefaultValue() -> *mut libc::c_int;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn __xmlGetWarningsDefaultValue() -> *mut libc::c_int;
    fn __xmlTreeIndentString() -> *mut *const libc::c_char;
    fn __xmlLoadExtDtdDefaultValue() -> *mut libc::c_int;
    fn __xmlParserDebugEntities() -> *mut libc::c_int;
    fn __xmlParserVersion() -> *mut *const libc::c_char;
    fn xmlRelaxNGCleanupTypes();
    fn xmlRelaxNGNewParserCtxt(URL: *const libc::c_char) -> xmlRelaxNGParserCtxtPtr;
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    fn xmlRelaxNGSetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    fn xmlRelaxNGFree(schema_0: xmlRelaxNGPtr);
    fn xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlRelaxNGNewValidCtxt(schema_0: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    fn xmlRelaxNGValidateDoc(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
    ) -> libc::c_int;
    fn xmlSchemaNewParserCtxt(URL: *const libc::c_char) -> xmlSchemaParserCtxtPtr;
    fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
    fn xmlSchemaSetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    fn xmlSchemaFree(schema_0: xmlSchemaPtr);
    fn xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaValidateSetFilename(
        vctxt: xmlSchemaValidCtxtPtr,
        filename: *const libc::c_char,
    );
    fn xmlSchemaNewValidCtxt(schema_0: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    fn xmlSchemaValidateDoc(
        ctxt: xmlSchemaValidCtxtPtr,
        instance: xmlDocPtr,
    ) -> libc::c_int;
    fn xmlSchemaValidateStream(
        ctxt: xmlSchemaValidCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
        sax_0: xmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn xmllint_endTimer(fmt: *const libc::c_char, _: ...);
    fn xmlHTMLError(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlHTMLWarning(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlHTMLValidityError(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlHTMLValidityWarning(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmllint_warningDebug(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmllint_errorDebug(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmllint_fatalErrorDebug(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> libc::c_int;
    fn xmlTextReaderDepth(reader: xmlTextReaderPtr) -> libc::c_int;
    fn xmlTextReaderHasValue(reader: xmlTextReaderPtr) -> libc::c_int;
    fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> libc::c_int;
    fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> libc::c_int;
    fn xmlTextReaderConstLocalName(reader: xmlTextReaderPtr) -> *const xmlChar;
    fn xmlTextReaderConstName(reader: xmlTextReaderPtr) -> *const xmlChar;
    fn xmlTextReaderConstNamespaceUri(reader: xmlTextReaderPtr) -> *const xmlChar;
    fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const xmlChar;
    fn xmlTextReaderSetParserProp(
        reader: xmlTextReaderPtr,
        prop: libc::c_int,
        value: libc::c_int,
    ) -> libc::c_int;
    fn xmlTextReaderCurrentNode(reader: xmlTextReaderPtr) -> xmlNodePtr;
    fn xmlTextReaderIsValid(reader: xmlTextReaderPtr) -> libc::c_int;
    fn xmlTextReaderRelaxNGValidate(
        reader: xmlTextReaderPtr,
        rng: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlTextReaderSchemaValidate(
        reader: xmlTextReaderPtr,
        xsd: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlReaderWalker(doc: xmlDocPtr) -> xmlTextReaderPtr;
    fn xmlReaderForFile(
        filename: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlTextReaderPtr;
    fn xmlReaderForMemory(
        buffer: *const libc::c_char,
        size: libc::c_int,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlTextReaderPtr;
    fn xmlXIncludeProcessFlags(doc: xmlDocPtr, flags: libc::c_int) -> libc::c_int;
    fn xmlFreePattern(comp: xmlPatternPtr);
    fn xmlPatterncompile(
        pattern_0: *const xmlChar,
        dict: *mut xmlDict,
        flags: libc::c_int,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> libc::c_int;
    fn xmlPatternGetStreamCtxt(comp: xmlPatternPtr) -> xmlStreamCtxtPtr;
    fn xmlFreeStreamCtxt(stream_0: xmlStreamCtxtPtr);
    fn xmlStreamPush(
        stream_0: xmlStreamCtxtPtr,
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> libc::c_int;
    fn xmlStreamPop(stream_0: xmlStreamCtxtPtr) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn htmlCreatePushParserCtxt(
        sax_0: htmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        chunk: *const libc::c_char,
        size: libc::c_int,
        filename: *const libc::c_char,
        enc: xmlCharEncoding,
    ) -> htmlParserCtxtPtr;
    fn htmlParseChunk(
        ctxt: htmlParserCtxtPtr,
        chunk: *const libc::c_char,
        size: libc::c_int,
        terminate: libc::c_int,
    ) -> libc::c_int;
    fn htmlFreeParserCtxt(ctxt: htmlParserCtxtPtr);
    fn htmlReadFile(
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> htmlDocPtr;
    fn htmlReadMemory(
        buffer: *const libc::c_char,
        size: libc::c_int,
        URL: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> htmlDocPtr;
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> libc::c_int;
    fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> libc::c_int;
    fn htmlSaveFile(filename: *const libc::c_char, cur: xmlDocPtr) -> libc::c_int;
    fn htmlSaveFileFormat(
        filename: *const libc::c_char,
        cur: xmlDocPtr,
        encoding_0: *const libc::c_char,
        format_0: libc::c_int,
    ) -> libc::c_int;
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    fn xmlXPathOrderDocElems(doc: xmlDocPtr) -> libc::c_long;
    fn xmlXPathEval(str: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    fn xmlXPathIsNaN(val: libc::c_double) -> libc::c_int;
    fn xmlXPathIsInf(val: libc::c_double) -> libc::c_int;
    fn xmlDebugDumpDocument(output_0: *mut FILE, doc: xmlDocPtr);
    fn xmlDebugDumpEntities(output_0: *mut FILE, doc: xmlDocPtr);
    fn xmlShell(
        doc: xmlDocPtr,
        filename: *mut libc::c_char,
        input: xmlShellReadlineFunc,
        output_0: *mut FILE,
    );
    fn xmlLoadCatalogs(paths_0: *const libc::c_char);
    fn xmlSchematronNewParserCtxt(
        URL: *const libc::c_char,
    ) -> xmlSchematronParserCtxtPtr;
    fn xmlSchematronFreeParserCtxt(ctxt: xmlSchematronParserCtxtPtr);
    fn xmlSchematronParse(ctxt: xmlSchematronParserCtxtPtr) -> xmlSchematronPtr;
    fn xmlSchematronFree(schema_0: xmlSchematronPtr);
    fn xmlSchematronNewValidCtxt(
        schema_0: xmlSchematronPtr,
        options_0: libc::c_int,
    ) -> xmlSchematronValidCtxtPtr;
    fn xmlSchematronFreeValidCtxt(ctxt: xmlSchematronValidCtxtPtr);
    fn xmlSchematronValidateDoc(
        ctxt: xmlSchematronValidCtxtPtr,
        instance: xmlDocPtr,
    ) -> libc::c_int;
    fn xmlC14NDocDumpMemory(
        doc: xmlDocPtr,
        nodes: xmlNodeSetPtr,
        mode: libc::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: libc::c_int,
        doc_txt_ptr: *mut *mut xmlChar,
    ) -> libc::c_int;
    fn xmlSaveToFd(
        fd: libc::c_int,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlSaveCtxtPtr;
    fn xmlSaveToFilename(
        filename: *const libc::c_char,
        encoding_0: *const libc::c_char,
        options_0: libc::c_int,
    ) -> xmlSaveCtxtPtr;
    fn xmlSaveDoc(ctxt: xmlSaveCtxtPtr, doc: xmlDocPtr) -> libc::c_long;
    fn xmlSaveTree(ctxt: xmlSaveCtxtPtr, node: xmlNodePtr) -> libc::c_long;
    fn xmlSaveClose(ctxt: xmlSaveCtxtPtr) -> libc::c_int;
    static mut xmllint_noout: libc::c_int;
    static mut xmllint_progresult: xmllintReturnCode;
    static mut xmllint_begin: timeval;
    static mut xmllint_callbacks: libc::c_int;
}
pub type xmlChar = libc::c_uchar;
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
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
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type xmlStrdupFunc = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
>;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlExternalEntityLoader = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        xmlParserCtxtPtr,
    ) -> xmlParserInputPtr,
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
pub type C2RustUnnamed = libc::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed = 1;
pub type xmlFeature = libc::c_uint;
pub const XML_WITH_NONE: xmlFeature = 99999;
pub const XML_WITH_LZMA: xmlFeature = 33;
pub const XML_WITH_ICU: xmlFeature = 32;
pub const XML_WITH_ZLIB: xmlFeature = 31;
pub const XML_WITH_DEBUG_RUN: xmlFeature = 30;
pub const XML_WITH_DEBUG_MEM: xmlFeature = 29;
pub const XML_WITH_DEBUG: xmlFeature = 28;
pub const XML_WITH_MODULES: xmlFeature = 27;
pub const XML_WITH_SCHEMATRON: xmlFeature = 26;
pub const XML_WITH_SCHEMAS: xmlFeature = 25;
pub const XML_WITH_EXPR: xmlFeature = 24;
pub const XML_WITH_AUTOMATA: xmlFeature = 23;
pub const XML_WITH_REGEXP: xmlFeature = 22;
pub const XML_WITH_UNICODE: xmlFeature = 21;
pub const XML_WITH_ISO8859X: xmlFeature = 20;
pub const XML_WITH_ICONV: xmlFeature = 19;
pub const XML_WITH_XINCLUDE: xmlFeature = 18;
pub const XML_WITH_XPTR: xmlFeature = 17;
pub const XML_WITH_XPATH: xmlFeature = 16;
pub const XML_WITH_CATALOG: xmlFeature = 15;
pub const XML_WITH_C14N: xmlFeature = 14;
pub const XML_WITH_LEGACY: xmlFeature = 13;
pub const XML_WITH_HTML: xmlFeature = 12;
pub const XML_WITH_VALID: xmlFeature = 11;
pub const XML_WITH_HTTP: xmlFeature = 10;
pub const XML_WITH_FTP: xmlFeature = 9;
pub const XML_WITH_SAX1: xmlFeature = 8;
pub const XML_WITH_WRITER: xmlFeature = 7;
pub const XML_WITH_PATTERN: xmlFeature = 6;
pub const XML_WITH_READER: xmlFeature = 5;
pub const XML_WITH_PUSH: xmlFeature = 4;
pub const XML_WITH_OUTPUT: xmlFeature = 3;
pub const XML_WITH_TREE: xmlFeature = 2;
pub const XML_WITH_THREAD: xmlFeature = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type xmlRegisterNodeFunc = Option::<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option::<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlRelaxNGValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlSchemaValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const XML_PARSER_SUBST_ENTITIES: C2RustUnnamed_0 = 4;
pub const XML_PARSER_VALIDATE: C2RustUnnamed_0 = 3;
pub const XML_PARSER_DEFAULTATTRS: C2RustUnnamed_0 = 2;
pub const XML_PARSER_LOADDTD: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_1 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_1 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_1 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_1 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_1 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_1 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_1 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_1 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_1 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_1 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_1 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_1 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_1 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_1 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_1 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_1 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_1 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_1 = 0;
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
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type htmlSAXHandlerPtr = xmlSAXHandlerPtr;
pub type htmlDocPtr = xmlDocPtr;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const HTML_PARSE_IGNORE_ENC: C2RustUnnamed_2 = 2097152;
pub const HTML_PARSE_COMPACT: C2RustUnnamed_2 = 65536;
pub const HTML_PARSE_NOIMPLIED: C2RustUnnamed_2 = 8192;
pub const HTML_PARSE_NONET: C2RustUnnamed_2 = 2048;
pub const HTML_PARSE_NOBLANKS: C2RustUnnamed_2 = 256;
pub const HTML_PARSE_PEDANTIC: C2RustUnnamed_2 = 128;
pub const HTML_PARSE_NOWARNING: C2RustUnnamed_2 = 64;
pub const HTML_PARSE_NOERROR: C2RustUnnamed_2 = 32;
pub const HTML_PARSE_NODEFDTD: C2RustUnnamed_2 = 4;
pub const HTML_PARSE_RECOVER: C2RustUnnamed_2 = 1;
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
pub type xmlShellReadlineFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_char,
>;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const XML_SCHEMATRON_OUT_IO: C2RustUnnamed_3 = 1024;
pub const XML_SCHEMATRON_OUT_BUFFER: C2RustUnnamed_3 = 512;
pub const XML_SCHEMATRON_OUT_FILE: C2RustUnnamed_3 = 256;
pub const XML_SCHEMATRON_OUT_ERROR: C2RustUnnamed_3 = 8;
pub const XML_SCHEMATRON_OUT_XML: C2RustUnnamed_3 = 4;
pub const XML_SCHEMATRON_OUT_TEXT: C2RustUnnamed_3 = 2;
pub const XML_SCHEMATRON_OUT_QUIET: C2RustUnnamed_3 = 1;
pub type xmlSchematron = _xmlSchematron;
pub type xmlSchematronPtr = *mut xmlSchematron;
pub type xmlSchematronParserCtxt = _xmlSchematronParserCtxt;
pub type xmlSchematronParserCtxtPtr = *mut xmlSchematronParserCtxt;
pub type xmlSchematronValidCtxt = _xmlSchematronValidCtxt;
pub type xmlSchematronValidCtxtPtr = *mut xmlSchematronValidCtxt;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const XML_C14N_1_1: C2RustUnnamed_4 = 2;
pub const XML_C14N_EXCLUSIVE_1_0: C2RustUnnamed_4 = 1;
pub const XML_C14N_1_0: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const XML_SAVE_WSNONSIG: C2RustUnnamed_5 = 128;
pub const XML_SAVE_AS_HTML: C2RustUnnamed_5 = 64;
pub const XML_SAVE_AS_XML: C2RustUnnamed_5 = 32;
pub const XML_SAVE_XHTML: C2RustUnnamed_5 = 16;
pub const XML_SAVE_NO_XHTML: C2RustUnnamed_5 = 8;
pub const XML_SAVE_NO_EMPTY: C2RustUnnamed_5 = 4;
pub const XML_SAVE_NO_DECL: C2RustUnnamed_5 = 2;
pub const XML_SAVE_FORMAT: C2RustUnnamed_5 = 1;
pub type xmlSaveCtxt = _xmlSaveCtxt;
pub type xmlSaveCtxtPtr = *mut xmlSaveCtxt;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut shell: libc::c_int = 0 as libc::c_int;
static mut debugent: libc::c_int = 0 as libc::c_int;
static mut debug: libc::c_int = 0 as libc::c_int;
static mut maxmem: libc::c_int = 0 as libc::c_int;
static mut copy: libc::c_int = 0 as libc::c_int;
static mut recovery: libc::c_int = 0 as libc::c_int;
static mut noent: libc::c_int = 0 as libc::c_int;
static mut noenc: libc::c_int = 0 as libc::c_int;
static mut noblanks: libc::c_int = 0 as libc::c_int;
static mut nowrap: libc::c_int = 0 as libc::c_int;
static mut format: libc::c_int = 0 as libc::c_int;
static mut output: *const libc::c_char = 0 as *const libc::c_char;
static mut compress: libc::c_int = 0 as libc::c_int;
static mut oldout: libc::c_int = 0 as libc::c_int;
static mut valid: libc::c_int = 0 as libc::c_int;
static mut postvalid: libc::c_int = 0 as libc::c_int;
static mut dtdvalid: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut dtdvalidfpi: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut relaxng: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut relaxngschemas: xmlRelaxNGPtr = 0 as *const xmlRelaxNG as xmlRelaxNGPtr;
static mut schema: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut wxschemas: xmlSchemaPtr = 0 as *const xmlSchema as xmlSchemaPtr;
static mut schematron: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut wxschematron: xmlSchematronPtr = 0 as *const xmlSchematron
    as xmlSchematronPtr;
static mut repeat: libc::c_int = 0 as libc::c_int;
static mut insert: libc::c_int = 0 as libc::c_int;
static mut html: libc::c_int = 0 as libc::c_int;
static mut xmlout: libc::c_int = 0 as libc::c_int;
static mut htmlout: libc::c_int = 0 as libc::c_int;
static mut nodefdtd: libc::c_int = 0 as libc::c_int;
static mut push: libc::c_int = 0 as libc::c_int;
static mut pushsize: libc::c_int = 4096 as libc::c_int;
static mut memory: libc::c_int = 0 as libc::c_int;
static mut testIO: libc::c_int = 0 as libc::c_int;
static mut encoding: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut xinclude: libc::c_int = 0 as libc::c_int;
static mut dtdattrs: libc::c_int = 0 as libc::c_int;
static mut loaddtd: libc::c_int = 0 as libc::c_int;
static mut timing: libc::c_int = 0 as libc::c_int;
static mut generate: libc::c_int = 0 as libc::c_int;
static mut dropdtd: libc::c_int = 0 as libc::c_int;
static mut catalogs: libc::c_int = 0 as libc::c_int;
static mut nocatalogs: libc::c_int = 0 as libc::c_int;
static mut canonical: libc::c_int = 0 as libc::c_int;
static mut canonical_11: libc::c_int = 0 as libc::c_int;
static mut exc_canonical: libc::c_int = 0 as libc::c_int;
static mut stream: libc::c_int = 0 as libc::c_int;
static mut walker: libc::c_int = 0 as libc::c_int;
static mut chkregister: libc::c_int = 0 as libc::c_int;
static mut nbregister: libc::c_int = 0 as libc::c_int;
static mut sax1: libc::c_int = 0 as libc::c_int;
static mut pattern: *const libc::c_char = 0 as *const libc::c_char;
static mut patternc: xmlPatternPtr = 0 as *const xmlPattern as xmlPatternPtr;
static mut patstream: xmlStreamCtxtPtr = 0 as *const xmlStreamCtxt as xmlStreamCtxtPtr;
static mut xpathquery: *const libc::c_char = 0 as *const libc::c_char;
static mut options: libc::c_int = XML_PARSE_COMPACT as libc::c_int
    | XML_PARSE_BIG_LINES as libc::c_int;
static mut sax: libc::c_int = 0 as libc::c_int;
static mut oldxml10: libc::c_int = 0 as libc::c_int;
static mut paths: [*mut xmlChar; 65] = [0 as *const xmlChar as *mut xmlChar; 65];
static mut nbpaths: libc::c_int = 0 as libc::c_int;
static mut load_trace: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn parsePath(mut path: *const xmlChar) {
    let mut cur: *const xmlChar = 0 as *const xmlChar;
    if path.is_null() {
        return;
    }
    while *path as libc::c_int != 0 as libc::c_int {
        if nbpaths >= 64 as libc::c_int {
            fprintf(
                stderr,
                b"MAX_PATHS reached: too many paths\n\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        cur = path;
        while *cur as libc::c_int == ' ' as i32 || *cur as libc::c_int == ':' as i32 {
            cur = cur.offset(1);
            cur;
        }
        path = cur;
        while *cur as libc::c_int != 0 as libc::c_int
            && *cur as libc::c_int != ' ' as i32 && *cur as libc::c_int != ':' as i32
        {
            cur = cur.offset(1);
            cur;
        }
        if cur != path {
            paths[nbpaths
                as usize] = xmlStrndup(
                path,
                cur.offset_from(path) as libc::c_long as libc::c_int,
            );
            if !(paths[nbpaths as usize]).is_null() {
                nbpaths += 1;
                nbpaths;
            }
            path = cur;
        }
    }
}
static mut defaultEntityLoader: xmlExternalEntityLoader = None;
unsafe extern "C" fn xmllintExternalEntityLoader(
    mut URL: *const libc::c_char,
    mut ID: *const libc::c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut warning: warningSAXFunc = None;
    let mut err: errorSAXFunc = None;
    let mut i: libc::c_int = 0;
    let mut lastsegment: *const libc::c_char = URL;
    let mut iter: *const libc::c_char = URL;
    if nbpaths > 0 as libc::c_int && !iter.is_null() {
        while *iter as libc::c_int != 0 as libc::c_int {
            if *iter as libc::c_int == '/' as i32 {
                lastsegment = iter.offset(1 as libc::c_int as isize);
            }
            iter = iter.offset(1);
            iter;
        }
    }
    if !ctxt.is_null() && !((*ctxt).sax).is_null() {
        warning = (*(*ctxt).sax).warning;
        err = (*(*ctxt).sax).error;
        (*(*ctxt).sax).warning = None;
        (*(*ctxt).sax).error = None;
    }
    if defaultEntityLoader.is_some() {
        ret = defaultEntityLoader.expect("non-null function pointer")(URL, ID, ctxt);
        if !ret.is_null() {
            if warning.is_some() {
                (*(*ctxt).sax).warning = warning;
            }
            if err.is_some() {
                (*(*ctxt).sax).error = err;
            }
            if load_trace != 0 {
                fprintf(
                    stderr,
                    b"Loaded URL=\"%s\" ID=\"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    if !URL.is_null() {
                        URL
                    } else {
                        b"(null)\0" as *const u8 as *const libc::c_char
                    },
                    if !ID.is_null() {
                        ID
                    } else {
                        b"(null)\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            return ret;
        }
    }
    i = 0 as libc::c_int;
    while i < nbpaths {
        let mut newURL: *mut xmlChar = 0 as *mut xmlChar;
        newURL = xmlStrdup(paths[i as usize] as *const xmlChar);
        newURL = xmlStrcat(
            newURL,
            b"/\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
        newURL = xmlStrcat(newURL, lastsegment as *const xmlChar);
        if !newURL.is_null() {
            ret = defaultEntityLoader
                .expect(
                    "non-null function pointer",
                )(newURL as *const libc::c_char, ID, ctxt);
            if !ret.is_null() {
                if warning.is_some() {
                    (*(*ctxt).sax).warning = warning;
                }
                if err.is_some() {
                    (*(*ctxt).sax).error = err;
                }
                if load_trace != 0 {
                    fprintf(
                        stderr,
                        b"Loaded URL=\"%s\" ID=\"%s\"\n\0" as *const u8
                            as *const libc::c_char,
                        newURL,
                        if !ID.is_null() {
                            ID
                        } else {
                            b"(null)\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                xmlFree.expect("non-null function pointer")(newURL as *mut libc::c_void);
                return ret;
            }
            xmlFree.expect("non-null function pointer")(newURL as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    if err.is_some() {
        (*(*ctxt).sax).error = err;
    }
    if warning.is_some() {
        (*(*ctxt).sax).warning = warning;
        if !URL.is_null() {
            warning
                .expect(
                    "non-null function pointer",
                )(
                ctxt as *mut libc::c_void,
                b"failed to load external entity \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                URL,
            );
        } else if !ID.is_null() {
            warning
                .expect(
                    "non-null function pointer",
                )(
                ctxt as *mut libc::c_void,
                b"failed to load external entity \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                ID,
            );
        }
    }
    return 0 as xmlParserInputPtr;
}
unsafe extern "C" fn OOM() {
    fprintf(
        stderr,
        b"Ran out of memory needs > %d bytes\n\0" as *const u8 as *const libc::c_char,
        maxmem,
    );
    xmllint_progresult = XMLLINT_ERR_MEM;
}
unsafe extern "C" fn myFreeFunc(mut mem: *mut libc::c_void) {
    xmlMemFree(mem);
}
unsafe extern "C" fn myMallocFunc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = xmlMemMalloc(size);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlMemFree(ret);
            return 0 as *mut libc::c_void;
        }
    }
    return ret;
}
unsafe extern "C" fn myReallocFunc(
    mut mem: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = xmlMemRealloc(mem, size);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlMemFree(ret);
            return 0 as *mut libc::c_void;
        }
    }
    return ret;
}
unsafe extern "C" fn myStrdupFunc(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = xmlMemoryStrdup(str);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
    }
    return ret;
}
unsafe extern "C" fn xmllint_startTimer() {
    gettimeofday(&mut xmllint_begin, 0 as *mut libc::c_void);
}
unsafe extern "C" fn xmlShellReadline(
    mut prompt: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut line_read: [libc::c_char; 501] = [0; 501];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if !prompt.is_null() {
        fprintf(stdout, b"%s\0" as *const u8 as *const libc::c_char, prompt);
    }
    fflush(stdout);
    if (fgets(line_read.as_mut_ptr(), 500 as libc::c_int, stdin)).is_null() {
        return 0 as *mut libc::c_char;
    }
    line_read[500 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    len = strlen(line_read.as_mut_ptr()) as libc::c_int;
    ret = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    if !ret.is_null() {
        memcpy(
            ret as *mut libc::c_void,
            line_read.as_mut_ptr() as *const libc::c_void,
            (len + 1 as libc::c_int) as libc::c_ulong,
        );
    }
    return ret;
}
unsafe extern "C" fn myRead(
    mut f: *mut libc::c_void,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    return fread(
        buf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len as libc::c_ulong,
        f as *mut FILE,
    ) as libc::c_int;
}
unsafe extern "C" fn myClose(mut context: *mut libc::c_void) -> libc::c_int {
    let mut f: *mut FILE = context as *mut FILE;
    if f == stdin {
        return 0 as libc::c_int;
    }
    return fclose(f);
}
static mut emptySAXHandlerStruct: xmlSAXHandler = {
    let mut init = _xmlSAXHandler {
        internalSubset: None,
        isStandalone: None,
        hasInternalSubset: None,
        hasExternalSubset: None,
        resolveEntity: None,
        getEntity: None,
        entityDecl: None,
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: None,
        startDocument: None,
        endDocument: None,
        startElement: None,
        endElement: None,
        reference: None,
        characters: None,
        ignorableWhitespace: None,
        processingInstruction: None,
        comment: None,
        warning: None,
        error: None,
        fatalError: None,
        getParameterEntity: None,
        cdataBlock: None,
        externalSubset: None,
        initialized: 0xdeedbeaf as libc::c_uint,
        _private: 0 as *const libc::c_void as *mut libc::c_void,
        startElementNs: None,
        endElementNs: None,
        serror: None,
    };
    init
};
static mut emptySAXHandler: xmlSAXHandlerPtr = unsafe {
    &emptySAXHandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
};
unsafe extern "C" fn isStandaloneDebug(mut ctx: *mut libc::c_void) -> libc::c_int {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return 0 as libc::c_int;
    }
    fprintf(stdout, b"SAX.isStandalone()\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn hasInternalSubsetDebug(mut ctx: *mut libc::c_void) -> libc::c_int {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return 0 as libc::c_int;
    }
    fprintf(stdout, b"SAX.hasInternalSubset()\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn hasExternalSubsetDebug(mut ctx: *mut libc::c_void) -> libc::c_int {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return 0 as libc::c_int;
    }
    fprintf(stdout, b"SAX.hasExternalSubset()\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn internalSubsetDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.internalSubset(%s,\0" as *const u8 as *const libc::c_char,
        name,
    );
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stdout, b" %s,\0" as *const u8 as *const libc::c_char, ExternalID);
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stdout, b" %s)\n\0" as *const u8 as *const libc::c_char, SystemID);
    };
}
unsafe extern "C" fn externalSubsetDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.externalSubset(%s,\0" as *const u8 as *const libc::c_char,
        name,
    );
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stdout, b" %s,\0" as *const u8 as *const libc::c_char, ExternalID);
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stdout, b" %s)\n\0" as *const u8 as *const libc::c_char, SystemID);
    };
}
unsafe extern "C" fn resolveEntityDebug(
    mut ctx: *mut libc::c_void,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) -> xmlParserInputPtr {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return 0 as xmlParserInputPtr;
    }
    fprintf(stdout, b"SAX.resolveEntity(\0" as *const u8 as *const libc::c_char);
    if !publicId.is_null() {
        fprintf(
            stdout,
            b"%s\0" as *const u8 as *const libc::c_char,
            publicId as *mut libc::c_char,
        );
    } else {
        fprintf(stdout, b" \0" as *const u8 as *const libc::c_char);
    }
    if !systemId.is_null() {
        fprintf(
            stdout,
            b", %s)\n\0" as *const u8 as *const libc::c_char,
            systemId as *mut libc::c_char,
        );
    } else {
        fprintf(stdout, b", )\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as xmlParserInputPtr;
}
unsafe extern "C" fn getEntityDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return 0 as xmlEntityPtr;
    }
    fprintf(stdout, b"SAX.getEntity(%s)\n\0" as *const u8 as *const libc::c_char, name);
    return 0 as xmlEntityPtr;
}
unsafe extern "C" fn getParameterEntityDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return 0 as xmlEntityPtr;
    }
    fprintf(
        stdout,
        b"SAX.getParameterEntity(%s)\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    return 0 as xmlEntityPtr;
}
unsafe extern "C" fn entityDeclDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut content: *mut xmlChar,
) {
    let mut nullstr: *const xmlChar = b"(null)\0" as *const u8 as *const libc::c_char
        as *mut xmlChar;
    if publicId.is_null() {
        publicId = nullstr;
    }
    if systemId.is_null() {
        systemId = nullstr;
    }
    if content.is_null() {
        content = nullstr as *mut xmlChar;
    }
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.entityDecl(%s, %d, %s, %s, %s)\n\0" as *const u8 as *const libc::c_char,
        name,
        type_0,
        publicId,
        systemId,
        content,
    );
}
unsafe extern "C" fn attributeDeclDebug(
    mut ctx: *mut libc::c_void,
    mut elem: *const xmlChar,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut def: libc::c_int,
    mut defaultValue: *const xmlChar,
    mut tree: xmlEnumerationPtr,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    if defaultValue.is_null() {
        fprintf(
            stdout,
            b"SAX.attributeDecl(%s, %s, %d, %d, NULL, ...)\n\0" as *const u8
                as *const libc::c_char,
            elem,
            name,
            type_0,
            def,
        );
    } else {
        fprintf(
            stdout,
            b"SAX.attributeDecl(%s, %s, %d, %d, %s, ...)\n\0" as *const u8
                as *const libc::c_char,
            elem,
            name,
            type_0,
            def,
            defaultValue,
        );
    }
    xmlFreeEnumeration(tree);
}
unsafe extern "C" fn elementDeclDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut content: xmlElementContentPtr,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.elementDecl(%s, %d, ...)\n\0" as *const u8 as *const libc::c_char,
        name,
        type_0,
    );
}
unsafe extern "C" fn notationDeclDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.notationDecl(%s, %s, %s)\n\0" as *const u8 as *const libc::c_char,
        name as *mut libc::c_char,
        publicId as *mut libc::c_char,
        systemId as *mut libc::c_char,
    );
}
unsafe extern "C" fn unparsedEntityDeclDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut notationName: *const xmlChar,
) {
    let mut nullstr: *const xmlChar = b"(null)\0" as *const u8 as *const libc::c_char
        as *mut xmlChar;
    if publicId.is_null() {
        publicId = nullstr;
    }
    if systemId.is_null() {
        systemId = nullstr;
    }
    if notationName.is_null() {
        notationName = nullstr;
    }
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.unparsedEntityDecl(%s, %s, %s, %s)\n\0" as *const u8
            as *const libc::c_char,
        name as *mut libc::c_char,
        publicId as *mut libc::c_char,
        systemId as *mut libc::c_char,
        notationName as *mut libc::c_char,
    );
}
unsafe extern "C" fn setDocumentLocatorDebug(
    mut ctx: *mut libc::c_void,
    mut loc: xmlSAXLocatorPtr,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(stdout, b"SAX.setDocumentLocator()\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn startDocumentDebug(mut ctx: *mut libc::c_void) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(stdout, b"SAX.startDocument()\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn endDocumentDebug(mut ctx: *mut libc::c_void) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(stdout, b"SAX.endDocument()\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn startElementDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    let mut i: libc::c_int = 0;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.startElement(%s\0" as *const u8 as *const libc::c_char,
        name as *mut libc::c_char,
    );
    if !atts.is_null() {
        i = 0 as libc::c_int;
        while !(*atts.offset(i as isize)).is_null() {
            let fresh0 = i;
            i = i + 1;
            fprintf(
                stdout,
                b", %s='\0" as *const u8 as *const libc::c_char,
                *atts.offset(fresh0 as isize),
            );
            if !(*atts.offset(i as isize)).is_null() {
                fprintf(
                    stdout,
                    b"%s'\0" as *const u8 as *const libc::c_char,
                    *atts.offset(i as isize),
                );
            }
            i += 1;
            i;
        }
    }
    fprintf(stdout, b")\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn endElementDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.endElement(%s)\n\0" as *const u8 as *const libc::c_char,
        name as *mut libc::c_char,
    );
}
unsafe extern "C" fn charactersDebug(
    mut ctx: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: libc::c_int,
) {
    let mut out: [libc::c_char; 40] = [0; 40];
    let mut i: libc::c_int = 0;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < len && i < 30 as libc::c_int {
        out[i as usize] = *ch.offset(i as isize) as libc::c_char;
        i += 1;
        i;
    }
    out[i as usize] = 0 as libc::c_int as libc::c_char;
    fprintf(
        stdout,
        b"SAX.characters(%s, %d)\n\0" as *const u8 as *const libc::c_char,
        out.as_mut_ptr(),
        len,
    );
}
unsafe extern "C" fn referenceDebug(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(stdout, b"SAX.reference(%s)\n\0" as *const u8 as *const libc::c_char, name);
}
unsafe extern "C" fn ignorableWhitespaceDebug(
    mut ctx: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: libc::c_int,
) {
    let mut out: [libc::c_char; 40] = [0; 40];
    let mut i: libc::c_int = 0;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < len && i < 30 as libc::c_int {
        out[i as usize] = *ch.offset(i as isize) as libc::c_char;
        i += 1;
        i;
    }
    out[i as usize] = 0 as libc::c_int as libc::c_char;
    fprintf(
        stdout,
        b"SAX.ignorableWhitespace(%s, %d)\n\0" as *const u8 as *const libc::c_char,
        out.as_mut_ptr(),
        len,
    );
}
unsafe extern "C" fn processingInstructionDebug(
    mut ctx: *mut libc::c_void,
    mut target: *const xmlChar,
    mut data: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    if !data.is_null() {
        fprintf(
            stdout,
            b"SAX.processingInstruction(%s, %s)\n\0" as *const u8 as *const libc::c_char,
            target as *mut libc::c_char,
            data as *mut libc::c_char,
        );
    } else {
        fprintf(
            stdout,
            b"SAX.processingInstruction(%s, NULL)\n\0" as *const u8
                as *const libc::c_char,
            target as *mut libc::c_char,
        );
    };
}
unsafe extern "C" fn cdataBlockDebug(
    mut ctx: *mut libc::c_void,
    mut value: *const xmlChar,
    mut len: libc::c_int,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.pcdata(%.20s, %d)\n\0" as *const u8 as *const libc::c_char,
        value as *mut libc::c_char,
        len,
    );
}
unsafe extern "C" fn commentDebug(
    mut ctx: *mut libc::c_void,
    mut value: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(stdout, b"SAX.comment(%s)\n\0" as *const u8 as *const libc::c_char, value);
}
static mut debugSAXHandlerStruct: xmlSAXHandler = unsafe {
    {
        let mut init = _xmlSAXHandler {
            internalSubset: Some(
                internalSubsetDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            isStandalone: Some(
                isStandaloneDebug
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            hasInternalSubset: Some(
                hasInternalSubsetDebug
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            hasExternalSubset: Some(
                hasExternalSubsetDebug
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            resolveEntity: Some(
                resolveEntityDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> xmlParserInputPtr,
            ),
            getEntity: Some(
                getEntityDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            entityDecl: Some(
                entityDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                        *const xmlChar,
                        *const xmlChar,
                        *mut xmlChar,
                    ) -> (),
            ),
            notationDecl: Some(
                notationDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            attributeDecl: Some(
                attributeDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        libc::c_int,
                        libc::c_int,
                        *const xmlChar,
                        xmlEnumerationPtr,
                    ) -> (),
            ),
            elementDecl: Some(
                elementDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                        xmlElementContentPtr,
                    ) -> (),
            ),
            unparsedEntityDecl: Some(
                unparsedEntityDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            setDocumentLocator: Some(
                setDocumentLocatorDebug
                    as unsafe extern "C" fn(*mut libc::c_void, xmlSAXLocatorPtr) -> (),
            ),
            startDocument: Some(
                startDocumentDebug as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            endDocument: Some(
                endDocumentDebug as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            startElement: Some(
                startElementDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *mut *const xmlChar,
                    ) -> (),
            ),
            endElement: Some(
                endElementDebug
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            reference: Some(
                referenceDebug
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            characters: Some(
                charactersDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            ignorableWhitespace: Some(
                ignorableWhitespaceDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            processingInstruction: Some(
                processingInstructionDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            comment: Some(
                commentDebug
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            warning: Some(
                xmllint_warningDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            error: Some(
                xmllint_errorDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            fatalError: Some(
                xmllint_fatalErrorDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            getParameterEntity: Some(
                getParameterEntityDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            cdataBlock: Some(
                cdataBlockDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            externalSubset: Some(
                externalSubsetDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            initialized: 1 as libc::c_int as libc::c_uint,
            _private: 0 as *const libc::c_void as *mut libc::c_void,
            startElementNs: None,
            endElementNs: None,
            serror: None,
        };
        init
    }
};
#[no_mangle]
pub static mut debugSAXHandler: xmlSAXHandlerPtr = unsafe {
    &debugSAXHandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
};
unsafe extern "C" fn startElementNsDebug(
    mut ctx: *mut libc::c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
    mut nb_namespaces: libc::c_int,
    mut namespaces: *mut *const xmlChar,
    mut nb_attributes: libc::c_int,
    mut nb_defaulted: libc::c_int,
    mut attributes: *mut *const xmlChar,
) {
    let mut i: libc::c_int = 0;
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.startElementNs(%s\0" as *const u8 as *const libc::c_char,
        localname as *mut libc::c_char,
    );
    if prefix.is_null() {
        fprintf(stdout, b", NULL\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            stdout,
            b", %s\0" as *const u8 as *const libc::c_char,
            prefix as *mut libc::c_char,
        );
    }
    if URI.is_null() {
        fprintf(stdout, b", NULL\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            stdout,
            b", '%s'\0" as *const u8 as *const libc::c_char,
            URI as *mut libc::c_char,
        );
    }
    fprintf(stdout, b", %d\0" as *const u8 as *const libc::c_char, nb_namespaces);
    if !namespaces.is_null() {
        i = 0 as libc::c_int;
        while i < nb_namespaces * 2 as libc::c_int {
            fprintf(stdout, b", xmlns\0" as *const u8 as *const libc::c_char);
            if !(*namespaces.offset(i as isize)).is_null() {
                fprintf(
                    stdout,
                    b":%s\0" as *const u8 as *const libc::c_char,
                    *namespaces.offset(i as isize),
                );
            }
            i += 1;
            i;
            fprintf(
                stdout,
                b"='%s'\0" as *const u8 as *const libc::c_char,
                *namespaces.offset(i as isize),
            );
            i += 1;
            i;
        }
    }
    fprintf(
        stdout,
        b", %d, %d\0" as *const u8 as *const libc::c_char,
        nb_attributes,
        nb_defaulted,
    );
    if !attributes.is_null() {
        i = 0 as libc::c_int;
        while i < nb_attributes * 5 as libc::c_int {
            if !(*attributes.offset((i + 1 as libc::c_int) as isize)).is_null() {
                fprintf(
                    stdout,
                    b", %s:%s='\0" as *const u8 as *const libc::c_char,
                    *attributes.offset((i + 1 as libc::c_int) as isize),
                    *attributes.offset(i as isize),
                );
            } else {
                fprintf(
                    stdout,
                    b", %s='\0" as *const u8 as *const libc::c_char,
                    *attributes.offset(i as isize),
                );
            }
            fprintf(
                stdout,
                b"%.4s...', %d\0" as *const u8 as *const libc::c_char,
                *attributes.offset((i + 3 as libc::c_int) as isize),
                (*attributes.offset((i + 4 as libc::c_int) as isize))
                    .offset_from(*attributes.offset((i + 3 as libc::c_int) as isize))
                    as libc::c_long as libc::c_int,
            );
            i += 5 as libc::c_int;
        }
    }
    fprintf(stdout, b")\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn endElementNsDebug(
    mut ctx: *mut libc::c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
) {
    xmllint_callbacks += 1;
    xmllint_callbacks;
    if xmllint_noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.endElementNs(%s\0" as *const u8 as *const libc::c_char,
        localname as *mut libc::c_char,
    );
    if prefix.is_null() {
        fprintf(stdout, b", NULL\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            stdout,
            b", %s\0" as *const u8 as *const libc::c_char,
            prefix as *mut libc::c_char,
        );
    }
    if URI.is_null() {
        fprintf(stdout, b", NULL)\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            stdout,
            b", '%s')\n\0" as *const u8 as *const libc::c_char,
            URI as *mut libc::c_char,
        );
    };
}
static mut debugSAX2HandlerStruct: xmlSAXHandler = unsafe {
    {
        let mut init = _xmlSAXHandler {
            internalSubset: Some(
                internalSubsetDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            isStandalone: Some(
                isStandaloneDebug
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            hasInternalSubset: Some(
                hasInternalSubsetDebug
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            hasExternalSubset: Some(
                hasExternalSubsetDebug
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            resolveEntity: Some(
                resolveEntityDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> xmlParserInputPtr,
            ),
            getEntity: Some(
                getEntityDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            entityDecl: Some(
                entityDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                        *const xmlChar,
                        *const xmlChar,
                        *mut xmlChar,
                    ) -> (),
            ),
            notationDecl: Some(
                notationDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            attributeDecl: Some(
                attributeDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        libc::c_int,
                        libc::c_int,
                        *const xmlChar,
                        xmlEnumerationPtr,
                    ) -> (),
            ),
            elementDecl: Some(
                elementDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                        xmlElementContentPtr,
                    ) -> (),
            ),
            unparsedEntityDecl: Some(
                unparsedEntityDeclDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            setDocumentLocator: Some(
                setDocumentLocatorDebug
                    as unsafe extern "C" fn(*mut libc::c_void, xmlSAXLocatorPtr) -> (),
            ),
            startDocument: Some(
                startDocumentDebug as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            endDocument: Some(
                endDocumentDebug as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            startElement: None,
            endElement: None,
            reference: Some(
                referenceDebug
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            characters: Some(
                charactersDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            ignorableWhitespace: Some(
                ignorableWhitespaceDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            processingInstruction: Some(
                processingInstructionDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            comment: Some(
                commentDebug
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            warning: Some(
                xmllint_warningDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            error: Some(
                xmllint_errorDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            fatalError: Some(
                xmllint_fatalErrorDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            getParameterEntity: Some(
                getParameterEntityDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            cdataBlock: Some(
                cdataBlockDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            externalSubset: Some(
                externalSubsetDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            initialized: 0xdeedbeaf as libc::c_uint,
            _private: 0 as *const libc::c_void as *mut libc::c_void,
            startElementNs: Some(
                startElementNsDebug
                    as unsafe extern "C" fn(
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
            ),
            endElementNs: Some(
                endElementNsDebug
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            serror: None,
        };
        init
    }
};
static mut debugSAX2Handler: xmlSAXHandlerPtr = unsafe {
    &debugSAX2HandlerStruct as *const xmlSAXHandler as *mut xmlSAXHandler
};
unsafe extern "C" fn testSAX(mut filename: *const libc::c_char) {
    let mut handler: xmlSAXHandlerPtr = 0 as *mut xmlSAXHandler;
    let mut user_data: *const libc::c_char = b"user_data\0" as *const u8
        as *const libc::c_char;
    let mut buf: xmlParserInputBufferPtr = 0 as xmlParserInputBufferPtr;
    let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
    let mut ctxt: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
    let mut old_sax: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    xmllint_callbacks = 0 as libc::c_int;
    if xmllint_noout != 0 {
        handler = emptySAXHandler;
    } else if sax1 != 0 {
        handler = debugSAXHandler;
    } else {
        handler = debugSAX2Handler;
    }
    buf = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if !buf.is_null() {
        if !wxschemas.is_null() {
            let mut ret: libc::c_int = 0;
            let mut vctxt: xmlSchemaValidCtxtPtr = 0 as *mut xmlSchemaValidCtxt;
            vctxt = xmlSchemaNewValidCtxt(wxschemas);
            xmlSchemaSetValidErrors(
                vctxt,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                    >,
                    xmlSchemaValidityErrorFunc,
                >(
                    Some(
                        fprintf
                            as unsafe extern "C" fn(
                                *mut FILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                ),
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                    >,
                    xmlSchemaValidityWarningFunc,
                >(
                    Some(
                        fprintf
                            as unsafe extern "C" fn(
                                *mut FILE,
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ),
                ),
                stderr as *mut libc::c_void,
            );
            xmlSchemaValidateSetFilename(vctxt, filename);
            ret = xmlSchemaValidateStream(
                vctxt,
                buf,
                XML_CHAR_ENCODING_NONE,
                handler,
                user_data as *mut libc::c_void,
            );
            if repeat == 0 as libc::c_int {
                if ret == 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s validates\n\0" as *const u8 as *const libc::c_char,
                        filename,
                    );
                } else if ret > 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s fails to validate\n\0" as *const u8 as *const libc::c_char,
                        filename,
                    );
                    xmllint_progresult = XMLLINT_ERR_VALID;
                } else {
                    fprintf(
                        stderr,
                        b"%s validation generated an internal error\n\0" as *const u8
                            as *const libc::c_char,
                        filename,
                    );
                    xmllint_progresult = XMLLINT_ERR_VALID;
                }
            }
            xmlSchemaFreeValidCtxt(vctxt);
        } else {
            ctxt = xmlNewParserCtxt();
            if ctxt.is_null() {
                xmlFreeParserInputBuffer(buf);
            } else {
                old_sax = (*ctxt).sax;
                (*ctxt).sax = handler;
                (*ctxt).userData = user_data as *mut libc::c_void;
                inputStream = xmlNewIOInputStream(ctxt, buf, XML_CHAR_ENCODING_NONE);
                if inputStream.is_null() {
                    xmlFreeParserInputBuffer(buf);
                } else {
                    inputPush(ctxt, inputStream);
                    xmlParseDocument(ctxt);
                    if !((*ctxt).myDoc).is_null() {
                        fprintf(
                            stderr,
                            b"SAX generated a doc !\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        xmlFreeDoc((*ctxt).myDoc);
                        (*ctxt).myDoc = 0 as xmlDocPtr;
                    }
                }
            }
        }
    }
    if !ctxt.is_null() {
        (*ctxt).sax = old_sax;
        xmlFreeParserCtxt(ctxt);
    }
}
unsafe extern "C" fn processNode(mut reader: xmlTextReaderPtr) {
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut value: *const xmlChar = 0 as *const xmlChar;
    let mut type_0: libc::c_int = 0;
    let mut empty: libc::c_int = 0;
    type_0 = xmlTextReaderNodeType(reader);
    empty = xmlTextReaderIsEmptyElement(reader);
    if debug != 0 {
        name = xmlTextReaderConstName(reader);
        if name.is_null() {
            name = b"--\0" as *const u8 as *const libc::c_char as *mut xmlChar;
        }
        value = xmlTextReaderConstValue(reader);
        printf(
            b"%d %d %s %d %d\0" as *const u8 as *const libc::c_char,
            xmlTextReaderDepth(reader),
            type_0,
            name,
            empty,
            xmlTextReaderHasValue(reader),
        );
        if value.is_null() {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b" %s\n\0" as *const u8 as *const libc::c_char, value);
        }
    }
    if !patternc.is_null() {
        let mut path: *mut xmlChar = 0 as *mut xmlChar;
        let mut match_0: libc::c_int = -(1 as libc::c_int);
        if type_0 == XML_READER_TYPE_ELEMENT as libc::c_int {
            match_0 = xmlPatternMatch(patternc, xmlTextReaderCurrentNode(reader));
            if match_0 != 0 {
                path = xmlGetNodePath(
                    xmlTextReaderCurrentNode(reader) as *const xmlNode,
                );
                printf(
                    b"Node %s matches pattern %s\n\0" as *const u8
                        as *const libc::c_char,
                    path,
                    pattern,
                );
            }
        }
        if !patstream.is_null() {
            let mut ret: libc::c_int = 0;
            if type_0 == XML_READER_TYPE_ELEMENT as libc::c_int {
                ret = xmlStreamPush(
                    patstream,
                    xmlTextReaderConstLocalName(reader),
                    xmlTextReaderConstNamespaceUri(reader),
                );
                if ret < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"xmlStreamPush() failure\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    xmlFreeStreamCtxt(patstream);
                    patstream = 0 as xmlStreamCtxtPtr;
                } else if ret != match_0 {
                    if path.is_null() {
                        path = xmlGetNodePath(
                            xmlTextReaderCurrentNode(reader) as *const xmlNode,
                        );
                    }
                    fprintf(
                        stderr,
                        b"xmlPatternMatch and xmlStreamPush disagree\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !path.is_null() {
                        fprintf(
                            stderr,
                            b"  pattern %s node %s\n\0" as *const u8
                                as *const libc::c_char,
                            pattern,
                            path,
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"  pattern %s node %s\n\0" as *const u8
                                as *const libc::c_char,
                            pattern,
                            xmlTextReaderConstName(reader),
                        );
                    }
                }
            }
            if type_0 == XML_READER_TYPE_END_ELEMENT as libc::c_int
                || type_0 == XML_READER_TYPE_ELEMENT as libc::c_int && empty != 0
            {
                ret = xmlStreamPop(patstream);
                if ret < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"xmlStreamPop() failure\n\0" as *const u8 as *const libc::c_char,
                    );
                    xmlFreeStreamCtxt(patstream);
                    patstream = 0 as xmlStreamCtxtPtr;
                }
            }
        }
        if !path.is_null() {
            xmlFree.expect("non-null function pointer")(path as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn streamFile(mut filename: *mut libc::c_char) {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: libc::c_int = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut info: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    let mut input: xmlParserInputBufferPtr = 0 as xmlParserInputBufferPtr;
    if memory != 0 {
        if stat(filename, &mut info) < 0 as libc::c_int {
            return;
        }
        fd = open(filename, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            return;
        }
        base = mmap(
            0 as *mut libc::c_void,
            info.st_size as size_t,
            0x1 as libc::c_int,
            0x1 as libc::c_int,
            fd,
            0 as libc::c_int as __off64_t,
        ) as *const libc::c_char;
        if base == -(1 as libc::c_int) as *mut libc::c_void as *const libc::c_char {
            close(fd);
            fprintf(
                stderr,
                b"mmap failure for file %s\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_RDFILE;
            return;
        }
        reader = xmlReaderForMemory(
            base,
            info.st_size as libc::c_int,
            filename,
            0 as *const libc::c_char,
            options,
        );
    } else {
        reader = xmlReaderForFile(filename, 0 as *const libc::c_char, options);
    }
    if !pattern.is_null() {
        patternc = xmlPatterncompile(
            pattern as *const xmlChar,
            0 as *mut xmlDict,
            0 as libc::c_int,
            0 as *mut *const xmlChar,
        );
        if patternc.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Pattern %s failed to compile\n\0" as *const u8 as *const libc::c_char,
                pattern,
            );
            xmllint_progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = 0 as *const libc::c_char;
        }
    }
    if !patternc.is_null() {
        patstream = xmlPatternGetStreamCtxt(patternc);
        if !patstream.is_null() {
            ret = xmlStreamPush(patstream, 0 as *const xmlChar, 0 as *const xmlChar);
            if ret < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"xmlStreamPush() failure\n\0" as *const u8 as *const libc::c_char,
                );
                xmlFreeStreamCtxt(patstream);
                patstream = 0 as xmlStreamCtxtPtr;
            }
        }
    }
    if !reader.is_null() {
        if valid != 0 {
            xmlTextReaderSetParserProp(
                reader,
                XML_PARSER_VALIDATE as libc::c_int,
                1 as libc::c_int,
            );
        } else if loaddtd != 0 {
            xmlTextReaderSetParserProp(
                reader,
                XML_PARSER_LOADDTD as libc::c_int,
                1 as libc::c_int,
            );
        }
        if !relaxng.is_null() {
            if timing != 0 && repeat == 0 {
                xmllint_startTimer();
            }
            ret = xmlTextReaderRelaxNGValidate(reader, relaxng);
            if ret < 0 as libc::c_int {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Relax-NG schema %s failed to compile\n\0" as *const u8
                        as *const libc::c_char,
                    relaxng,
                );
                xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
                relaxng = 0 as *mut libc::c_char;
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(
                    b"Compiling the schemas\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if !schema.is_null() {
            if timing != 0 && repeat == 0 {
                xmllint_startTimer();
            }
            ret = xmlTextReaderSchemaValidate(reader, schema);
            if ret < 0 as libc::c_int {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"XSD schema %s failed to compile\n\0" as *const u8
                        as *const libc::c_char,
                    schema,
                );
                xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
                schema = 0 as *mut libc::c_char;
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(
                    b"Compiling the schemas\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        ret = xmlTextReaderRead(reader);
        while ret == 1 as libc::c_int {
            if debug != 0 || !patternc.is_null() {
                processNode(reader);
            }
            ret = xmlTextReaderRead(reader);
        }
        if timing != 0 && repeat == 0 {
            if !relaxng.is_null() {
                xmllint_endTimer(
                    b"Parsing and validating\0" as *const u8 as *const libc::c_char,
                );
            } else if valid != 0 {
                xmllint_endTimer(
                    b"Parsing and validating\0" as *const u8 as *const libc::c_char,
                );
            } else {
                xmllint_endTimer(b"Parsing\0" as *const u8 as *const libc::c_char);
            }
        }
        if valid != 0 {
            if xmlTextReaderIsValid(reader) != 1 as libc::c_int {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Document %s does not validate\n\0" as *const u8
                        as *const libc::c_char,
                    filename,
                );
                xmllint_progresult = XMLLINT_ERR_VALID;
            }
        }
        if !relaxng.is_null() || !schema.is_null() {
            if xmlTextReaderIsValid(reader) != 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s fails to validate\n\0" as *const u8 as *const libc::c_char,
                    filename,
                );
                xmllint_progresult = XMLLINT_ERR_VALID;
            } else {
                fprintf(
                    stderr,
                    b"%s validates\n\0" as *const u8 as *const libc::c_char,
                    filename,
                );
            }
        }
        xmlFreeTextReader(reader);
        if ret != 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s : failed to parse\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_UNCLASS;
        }
    } else {
        fprintf(
            stderr,
            b"Unable to open %s\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        xmllint_progresult = XMLLINT_ERR_UNCLASS;
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
        patstream = 0 as xmlStreamCtxtPtr;
    }
    if memory != 0 {
        xmlFreeParserInputBuffer(input);
        munmap(base as *mut libc::c_char as *mut libc::c_void, info.st_size as size_t);
        close(fd);
    }
}
unsafe extern "C" fn walkDoc(mut doc: xmlDocPtr) {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut ret: libc::c_int = 0;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut namespaces: [*const xmlChar; 22] = [0 as *const xmlChar; 22];
    let mut i: libc::c_int = 0;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Document does not have a root element\0" as *const u8
                as *const libc::c_char,
        );
        xmllint_progresult = XMLLINT_ERR_UNCLASS;
        return;
    }
    ns = (*root).nsDef;
    i = 0 as libc::c_int;
    while !ns.is_null() && i < 20 as libc::c_int {
        let fresh1 = i;
        i = i + 1;
        namespaces[fresh1 as usize] = (*ns).href;
        let fresh2 = i;
        i = i + 1;
        namespaces[fresh2 as usize] = (*ns).prefix;
        ns = (*ns).next;
    }
    let fresh3 = i;
    i = i + 1;
    namespaces[fresh3 as usize] = 0 as *const xmlChar;
    namespaces[i as usize] = 0 as *const xmlChar;
    if !pattern.is_null() {
        patternc = xmlPatterncompile(
            pattern as *const xmlChar,
            (*doc).dict,
            0 as libc::c_int,
            &mut *namespaces.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        if patternc.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Pattern %s failed to compile\n\0" as *const u8 as *const libc::c_char,
                pattern,
            );
            xmllint_progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = 0 as *const libc::c_char;
        }
    }
    if !patternc.is_null() {
        patstream = xmlPatternGetStreamCtxt(patternc);
        if !patstream.is_null() {
            ret = xmlStreamPush(patstream, 0 as *const xmlChar, 0 as *const xmlChar);
            if ret < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"xmlStreamPush() failure\n\0" as *const u8 as *const libc::c_char,
                );
                xmlFreeStreamCtxt(patstream);
                patstream = 0 as xmlStreamCtxtPtr;
            }
        }
    }
    reader = xmlReaderWalker(doc);
    if !reader.is_null() {
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        ret = xmlTextReaderRead(reader);
        while ret == 1 as libc::c_int {
            if debug != 0 || !patternc.is_null() {
                processNode(reader);
            }
            ret = xmlTextReaderRead(reader);
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(
                b"walking through the doc\0" as *const u8 as *const libc::c_char,
            );
        }
        xmlFreeTextReader(reader);
        if ret != 0 as libc::c_int {
            fprintf(
                stderr,
                b"failed to walk through the doc\n\0" as *const u8 as *const libc::c_char,
            );
            xmllint_progresult = XMLLINT_ERR_UNCLASS;
        }
    } else {
        fprintf(
            stderr,
            b"Failed to crate a reader from the document\n\0" as *const u8
                as *const libc::c_char,
        );
        xmllint_progresult = XMLLINT_ERR_UNCLASS;
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
        patstream = 0 as xmlStreamCtxtPtr;
    }
}
unsafe extern "C" fn doXPathDump(mut cur: xmlXPathObjectPtr) {
    match (*cur).type_0 as libc::c_uint {
        1 => {
            let mut i: libc::c_int = 0;
            let mut node: xmlNodePtr = 0 as *mut xmlNode;
            let mut ctxt: xmlSaveCtxtPtr = 0 as *mut xmlSaveCtxt;
            if ((*cur).nodesetval).is_null()
                || (*(*cur).nodesetval).nodeNr <= 0 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"XPath set is empty\n\0" as *const u8 as *const libc::c_char,
                );
                xmllint_progresult = XMLLINT_ERR_XPATH;
            } else {
                ctxt = xmlSaveToFd(
                    1 as libc::c_int,
                    0 as *const libc::c_char,
                    0 as libc::c_int,
                );
                if ctxt.is_null() {
                    fprintf(
                        stderr,
                        b"Out of memory for XPath\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    xmllint_progresult = XMLLINT_ERR_MEM;
                    return;
                }
                i = 0 as libc::c_int;
                while i < (*(*cur).nodesetval).nodeNr {
                    node = *((*(*cur).nodesetval).nodeTab).offset(i as isize);
                    xmlSaveTree(ctxt, node);
                    i += 1;
                    i;
                }
                xmlSaveClose(ctxt);
            }
        }
        2 => {
            if (*cur).boolval != 0 {
                printf(b"true\0" as *const u8 as *const libc::c_char);
            } else {
                printf(b"false\0" as *const u8 as *const libc::c_char);
            }
        }
        3 => {
            match xmlXPathIsInf((*cur).floatval) {
                1 => {
                    printf(b"Infinity\0" as *const u8 as *const libc::c_char);
                }
                -1 => {
                    printf(b"-Infinity\0" as *const u8 as *const libc::c_char);
                }
                _ => {
                    if xmlXPathIsNaN((*cur).floatval) != 0 {
                        printf(b"NaN\0" as *const u8 as *const libc::c_char);
                    } else {
                        printf(
                            b"%0g\0" as *const u8 as *const libc::c_char,
                            (*cur).floatval,
                        );
                    }
                }
            }
        }
        4 => {
            printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                (*cur).stringval as *const libc::c_char,
            );
        }
        0 => {
            fprintf(
                stderr,
                b"XPath Object is uninitialized\n\0" as *const u8 as *const libc::c_char,
            );
            xmllint_progresult = XMLLINT_ERR_XPATH;
        }
        _ => {
            fprintf(
                stderr,
                b"XPath object of unexpected type\n\0" as *const u8
                    as *const libc::c_char,
            );
            xmllint_progresult = XMLLINT_ERR_XPATH;
        }
    };
}
unsafe extern "C" fn doXPathQuery(mut doc: xmlDocPtr, mut query: *const libc::c_char) {
    let mut ctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut res: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    ctxt = xmlXPathNewContext(doc);
    if ctxt.is_null() {
        fprintf(
            stderr,
            b"Out of memory for XPath\n\0" as *const u8 as *const libc::c_char,
        );
        xmllint_progresult = XMLLINT_ERR_MEM;
        return;
    }
    (*ctxt).node = doc as xmlNodePtr;
    res = xmlXPathEval(query as *mut xmlChar, ctxt);
    xmlXPathFreeContext(ctxt);
    if res.is_null() {
        fprintf(
            stderr,
            b"XPath evaluation failure\n\0" as *const u8 as *const libc::c_char,
        );
        xmllint_progresult = XMLLINT_ERR_XPATH;
        return;
    }
    doXPathDump(res);
    xmlXPathFreeObject(res);
}
unsafe extern "C" fn parseAndPrintFile(
    mut filename: *mut libc::c_char,
    mut rectxt: xmlParserCtxtPtr,
) {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    let mut tmp: xmlDocPtr = 0 as *mut xmlDoc;
    if timing != 0 && repeat == 0 {
        xmllint_startTimer();
    }
    if filename.is_null() {
        if generate != 0 {
            let mut n: xmlNodePtr = 0 as *mut xmlNode;
            doc = xmlNewDoc(
                b"1.0\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            );
            n = xmlNewDocNode(
                doc,
                0 as xmlNsPtr,
                b"info\0" as *const u8 as *const libc::c_char as *mut xmlChar,
                0 as *const xmlChar,
            );
            xmlNodeSetContent(
                n,
                b"abc\0" as *const u8 as *const libc::c_char as *mut xmlChar,
            );
            xmlDocSetRootElement(doc, n);
        }
    } else if html != 0 && push != 0 {
        let mut f: *mut FILE = 0 as *mut FILE;
        f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        if !f.is_null() {
            let mut res: libc::c_int = 0;
            let mut chars: [libc::c_char; 4096] = [0; 4096];
            let mut ctxt: htmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            res = fread(
                chars.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                4 as libc::c_int as libc::c_ulong,
                f,
            ) as libc::c_int;
            if res > 0 as libc::c_int {
                ctxt = htmlCreatePushParserCtxt(
                    0 as htmlSAXHandlerPtr,
                    0 as *mut libc::c_void,
                    chars.as_mut_ptr(),
                    res,
                    filename,
                    XML_CHAR_ENCODING_NONE,
                );
                xmlCtxtUseOptions(ctxt, options);
                loop {
                    res = fread(
                        chars.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        pushsize as libc::c_ulong,
                        f,
                    ) as libc::c_int;
                    if !(res > 0 as libc::c_int) {
                        break;
                    }
                    htmlParseChunk(ctxt, chars.as_mut_ptr(), res, 0 as libc::c_int);
                }
                htmlParseChunk(
                    ctxt,
                    chars.as_mut_ptr(),
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                doc = (*ctxt).myDoc;
                htmlFreeParserCtxt(ctxt);
            }
            fclose(f);
        }
    } else if html != 0 && memory != 0 {
        let mut fd: libc::c_int = 0;
        let mut info: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut base: *const libc::c_char = 0 as *const libc::c_char;
        if stat(filename, &mut info) < 0 as libc::c_int {
            return;
        }
        fd = open(filename, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            return;
        }
        base = mmap(
            0 as *mut libc::c_void,
            info.st_size as size_t,
            0x1 as libc::c_int,
            0x1 as libc::c_int,
            fd,
            0 as libc::c_int as __off64_t,
        ) as *const libc::c_char;
        if base == -(1 as libc::c_int) as *mut libc::c_void as *const libc::c_char {
            close(fd);
            fprintf(
                stderr,
                b"mmap failure for file %s\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_RDFILE;
            return;
        }
        doc = htmlReadMemory(
            base as *mut libc::c_char,
            info.st_size as libc::c_int,
            filename,
            0 as *const libc::c_char,
            options,
        );
        munmap(base as *mut libc::c_char as *mut libc::c_void, info.st_size as size_t);
        close(fd);
    } else if html != 0 {
        doc = htmlReadFile(filename, 0 as *const libc::c_char, options);
    } else if push != 0 {
        let mut f_0: *mut FILE = 0 as *mut FILE;
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *filename.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
        {
            f_0 = stdin;
        } else {
            f_0 = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
        }
        if !f_0.is_null() {
            let mut ret: libc::c_int = 0;
            let mut res_0: libc::c_int = 0;
            let mut size: libc::c_int = 1024 as libc::c_int;
            let mut chars_0: [libc::c_char; 1024] = [0; 1024];
            let mut ctxt_0: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
            res_0 = fread(
                chars_0.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                4 as libc::c_int as libc::c_ulong,
                f_0,
            ) as libc::c_int;
            if res_0 > 0 as libc::c_int {
                ctxt_0 = xmlCreatePushParserCtxt(
                    0 as xmlSAXHandlerPtr,
                    0 as *mut libc::c_void,
                    chars_0.as_mut_ptr(),
                    res_0,
                    filename,
                );
                xmlCtxtUseOptions(ctxt_0, options);
                loop {
                    res_0 = fread(
                        chars_0.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        size as libc::c_ulong,
                        f_0,
                    ) as libc::c_int;
                    if !(res_0 > 0 as libc::c_int) {
                        break;
                    }
                    xmlParseChunk(ctxt_0, chars_0.as_mut_ptr(), res_0, 0 as libc::c_int);
                }
                xmlParseChunk(
                    ctxt_0,
                    chars_0.as_mut_ptr(),
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                doc = (*ctxt_0).myDoc;
                ret = (*ctxt_0).wellFormed;
                xmlFreeParserCtxt(ctxt_0);
                if ret == 0 {
                    xmlFreeDoc(doc);
                    doc = 0 as xmlDocPtr;
                }
            }
            if f_0 != stdin {
                fclose(f_0);
            }
        }
    } else if testIO != 0 {
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *filename.offset(1 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
        {
            doc = xmlReadFd(
                0 as libc::c_int,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                options,
            );
        } else {
            let mut f_1: *mut FILE = 0 as *mut FILE;
            f_1 = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
            if !f_1.is_null() {
                if rectxt.is_null() {
                    doc = xmlReadIO(
                        Some(
                            myRead
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_char,
                                    libc::c_int,
                                ) -> libc::c_int,
                        ),
                        Some(
                            myClose
                                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                        ),
                        f_1 as *mut libc::c_void,
                        filename,
                        0 as *const libc::c_char,
                        options,
                    );
                } else {
                    doc = xmlCtxtReadIO(
                        rectxt,
                        Some(
                            myRead
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_char,
                                    libc::c_int,
                                ) -> libc::c_int,
                        ),
                        Some(
                            myClose
                                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                        ),
                        f_1 as *mut libc::c_void,
                        filename,
                        0 as *const libc::c_char,
                        options,
                    );
                }
            } else {
                doc = 0 as xmlDocPtr;
            }
        }
    } else if htmlout != 0 {
        let mut ctxt_1: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
        if rectxt.is_null() {
            ctxt_1 = xmlNewParserCtxt();
        } else {
            ctxt_1 = rectxt;
        }
        if ctxt_1.is_null() {
            doc = 0 as xmlDocPtr;
        } else {
            (*(*ctxt_1).sax)
                .error = Some(
                xmlHTMLError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            );
            (*(*ctxt_1).sax)
                .warning = Some(
                xmlHTMLWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            );
            (*ctxt_1)
                .vctxt
                .error = Some(
                xmlHTMLValidityError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            );
            (*ctxt_1)
                .vctxt
                .warning = Some(
                xmlHTMLValidityWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            );
            doc = xmlCtxtReadFile(ctxt_1, filename, 0 as *const libc::c_char, options);
            if rectxt.is_null() {
                xmlFreeParserCtxt(ctxt_1);
            }
        }
    } else if memory != 0 {
        let mut fd_0: libc::c_int = 0;
        let mut info_0: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut base_0: *const libc::c_char = 0 as *const libc::c_char;
        if stat(filename, &mut info_0) < 0 as libc::c_int {
            return;
        }
        fd_0 = open(filename, 0 as libc::c_int);
        if fd_0 < 0 as libc::c_int {
            return;
        }
        base_0 = mmap(
            0 as *mut libc::c_void,
            info_0.st_size as size_t,
            0x1 as libc::c_int,
            0x1 as libc::c_int,
            fd_0,
            0 as libc::c_int as __off64_t,
        ) as *const libc::c_char;
        if base_0 == -(1 as libc::c_int) as *mut libc::c_void as *const libc::c_char {
            close(fd_0);
            fprintf(
                stderr,
                b"mmap failure for file %s\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_RDFILE;
            return;
        }
        if rectxt.is_null() {
            doc = xmlReadMemory(
                base_0 as *mut libc::c_char,
                info_0.st_size as libc::c_int,
                filename,
                0 as *const libc::c_char,
                options,
            );
        } else {
            doc = xmlCtxtReadMemory(
                rectxt,
                base_0 as *mut libc::c_char,
                info_0.st_size as libc::c_int,
                filename,
                0 as *const libc::c_char,
                options,
            );
        }
        munmap(
            base_0 as *mut libc::c_char as *mut libc::c_void,
            info_0.st_size as size_t,
        );
        close(fd_0);
    } else if valid != 0 {
        let mut ctxt_2: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
        if rectxt.is_null() {
            ctxt_2 = xmlNewParserCtxt();
        } else {
            ctxt_2 = rectxt;
        }
        if ctxt_2.is_null() {
            doc = 0 as xmlDocPtr;
        } else {
            doc = xmlCtxtReadFile(ctxt_2, filename, 0 as *const libc::c_char, options);
            if (*ctxt_2).valid == 0 as libc::c_int {
                xmllint_progresult = XMLLINT_ERR_RDFILE;
            }
            if rectxt.is_null() {
                xmlFreeParserCtxt(ctxt_2);
            }
        }
    } else if !rectxt.is_null() {
        doc = xmlCtxtReadFile(rectxt, filename, 0 as *const libc::c_char, options);
    } else if sax1 != 0 {
        doc = xmlParseFile(filename);
    } else {
        doc = xmlReadFile(filename, 0 as *const libc::c_char, options);
    }
    if doc.is_null() {
        xmllint_progresult = XMLLINT_ERR_UNCLASS;
        return;
    }
    if timing != 0 && repeat == 0 {
        xmllint_endTimer(b"Parsing\0" as *const u8 as *const libc::c_char);
    }
    if dropdtd != 0 {
        let mut dtd: xmlDtdPtr = 0 as *mut xmlDtd;
        dtd = xmlGetIntSubset(doc as *const xmlDoc);
        if !dtd.is_null() {
            xmlUnlinkNode(dtd as xmlNodePtr);
            xmlFreeDtd(dtd);
        }
    }
    if xinclude != 0 {
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        if xmlXIncludeProcessFlags(doc, options) < 0 as libc::c_int {
            xmllint_progresult = XMLLINT_ERR_UNCLASS;
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(
                b"Xinclude processing\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if !xpathquery.is_null() {
        doXPathQuery(doc, xpathquery);
    }
    if shell != 0 {
        xmlXPathOrderDocElems(doc);
        xmlShell(
            doc,
            filename,
            Some(
                xmlShellReadline
                    as unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_char,
            ),
            stdout,
        );
    }
    if copy != 0 {
        tmp = doc;
        if timing != 0 {
            xmllint_startTimer();
        }
        doc = xmlCopyDoc(doc, 1 as libc::c_int);
        if timing != 0 {
            xmllint_endTimer(b"Copying\0" as *const u8 as *const libc::c_char);
        }
        if timing != 0 {
            xmllint_startTimer();
        }
        xmlFreeDoc(tmp);
        if timing != 0 {
            xmllint_endTimer(b"Freeing original\0" as *const u8 as *const libc::c_char);
        }
    }
    if insert != 0 && html == 0 {
        let mut list: [*const xmlChar; 256] = [0 as *const xmlChar; 256];
        let mut nb: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        if !((*doc).children).is_null() {
            node = (*doc).children;
            while !node.is_null() && ((*node).last).is_null() {
                node = (*node).next;
            }
            if !node.is_null() {
                nb = xmlValidGetValidElements(
                    (*node).last,
                    0 as *mut xmlNode,
                    list.as_mut_ptr(),
                    256 as libc::c_int,
                );
                if nb < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"could not get valid list of elements\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if nb == 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"No element can be inserted under root\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%d element types can be inserted under root:\n\0" as *const u8
                            as *const libc::c_char,
                        nb,
                    );
                    i = 0 as libc::c_int;
                    while i < nb {
                        fprintf(
                            stderr,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            list[i as usize] as *mut libc::c_char,
                        );
                        i += 1;
                        i;
                    }
                }
            }
        }
    } else if walker != 0 {
        walkDoc(doc);
    }
    if xmllint_noout == 0 as libc::c_int {
        let mut ret_0: libc::c_int = 0;
        if debug == 0 {
            if timing != 0 && repeat == 0 {
                xmllint_startTimer();
            }
            if html != 0 && xmlout == 0 {
                if compress != 0 {
                    htmlSaveFile(
                        if !output.is_null() {
                            output
                        } else {
                            b"-\0" as *const u8 as *const libc::c_char
                        },
                        doc,
                    );
                } else if !encoding.is_null() {
                    if format == 1 as libc::c_int {
                        htmlSaveFileFormat(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                            doc,
                            encoding,
                            1 as libc::c_int,
                        );
                    } else {
                        htmlSaveFileFormat(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                            doc,
                            encoding,
                            0 as libc::c_int,
                        );
                    }
                } else if format == 1 as libc::c_int {
                    htmlSaveFileFormat(
                        if !output.is_null() {
                            output
                        } else {
                            b"-\0" as *const u8 as *const libc::c_char
                        },
                        doc,
                        0 as *const libc::c_char,
                        1 as libc::c_int,
                    );
                } else {
                    let mut out: *mut FILE = 0 as *mut FILE;
                    if output.is_null() {
                        out = stdout;
                    } else {
                        out = fopen(output, b"wb\0" as *const u8 as *const libc::c_char);
                    }
                    if !out.is_null() {
                        if htmlDocDump(out, doc) < 0 as libc::c_int {
                            xmllint_progresult = XMLLINT_ERR_OUT;
                        }
                        if !output.is_null() {
                            fclose(out);
                        }
                    } else {
                        fprintf(
                            stderr,
                            b"failed to open %s\n\0" as *const u8 as *const libc::c_char,
                            output,
                        );
                        xmllint_progresult = XMLLINT_ERR_OUT;
                    }
                }
                if timing != 0 && repeat == 0 {
                    xmllint_endTimer(b"Saving\0" as *const u8 as *const libc::c_char);
                }
            } else if canonical != 0 {
                let mut result: *mut xmlChar = 0 as *mut xmlChar;
                let mut size_0: libc::c_int = 0;
                size_0 = xmlC14NDocDumpMemory(
                    doc,
                    0 as xmlNodeSetPtr,
                    XML_C14N_1_0 as libc::c_int,
                    0 as *mut *mut xmlChar,
                    1 as libc::c_int,
                    &mut result,
                );
                if size_0 >= 0 as libc::c_int {
                    if write(
                        1 as libc::c_int,
                        result as *const libc::c_void,
                        size_0 as size_t,
                    ) == -(1 as libc::c_int) as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(result as *mut libc::c_void);
                } else {
                    fprintf(
                        stderr,
                        b"Failed to canonicalize\n\0" as *const u8 as *const libc::c_char,
                    );
                    xmllint_progresult = XMLLINT_ERR_OUT;
                }
            } else if canonical_11 != 0 {
                let mut result_0: *mut xmlChar = 0 as *mut xmlChar;
                let mut size_1: libc::c_int = 0;
                size_1 = xmlC14NDocDumpMemory(
                    doc,
                    0 as xmlNodeSetPtr,
                    XML_C14N_1_1 as libc::c_int,
                    0 as *mut *mut xmlChar,
                    1 as libc::c_int,
                    &mut result_0,
                );
                if size_1 >= 0 as libc::c_int {
                    if write(
                        1 as libc::c_int,
                        result_0 as *const libc::c_void,
                        size_1 as size_t,
                    ) == -(1 as libc::c_int) as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(result_0 as *mut libc::c_void);
                } else {
                    fprintf(
                        stderr,
                        b"Failed to canonicalize\n\0" as *const u8 as *const libc::c_char,
                    );
                    xmllint_progresult = XMLLINT_ERR_OUT;
                }
            } else if exc_canonical != 0 {
                let mut result_1: *mut xmlChar = 0 as *mut xmlChar;
                let mut size_2: libc::c_int = 0;
                size_2 = xmlC14NDocDumpMemory(
                    doc,
                    0 as xmlNodeSetPtr,
                    XML_C14N_EXCLUSIVE_1_0 as libc::c_int,
                    0 as *mut *mut xmlChar,
                    1 as libc::c_int,
                    &mut result_1,
                );
                if size_2 >= 0 as libc::c_int {
                    if write(
                        1 as libc::c_int,
                        result_1 as *const libc::c_void,
                        size_2 as size_t,
                    ) == -(1 as libc::c_int) as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(result_1 as *mut libc::c_void);
                } else {
                    fprintf(
                        stderr,
                        b"Failed to canonicalize\n\0" as *const u8 as *const libc::c_char,
                    );
                    xmllint_progresult = XMLLINT_ERR_OUT;
                }
            } else if memory != 0 {
                let mut result_2: *mut xmlChar = 0 as *mut xmlChar;
                let mut len: libc::c_int = 0;
                if !encoding.is_null() {
                    if format == 1 as libc::c_int {
                        xmlDocDumpFormatMemoryEnc(
                            doc,
                            &mut result_2,
                            &mut len,
                            encoding,
                            1 as libc::c_int,
                        );
                    } else {
                        xmlDocDumpMemoryEnc(doc, &mut result_2, &mut len, encoding);
                    }
                } else if format == 1 as libc::c_int {
                    xmlDocDumpFormatMemory(
                        doc,
                        &mut result_2,
                        &mut len,
                        1 as libc::c_int,
                    );
                } else {
                    xmlDocDumpMemory(doc, &mut result_2, &mut len);
                }
                if result_2.is_null() {
                    fprintf(
                        stderr,
                        b"Failed to save\n\0" as *const u8 as *const libc::c_char,
                    );
                    xmllint_progresult = XMLLINT_ERR_OUT;
                } else {
                    if write(
                        1 as libc::c_int,
                        result_2 as *const libc::c_void,
                        len as size_t,
                    ) == -(1 as libc::c_int) as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(result_2 as *mut libc::c_void);
                }
            } else if compress != 0 {
                xmlSaveFile(
                    if !output.is_null() {
                        output
                    } else {
                        b"-\0" as *const u8 as *const libc::c_char
                    },
                    doc,
                );
            } else if oldout != 0 {
                if !encoding.is_null() {
                    if format == 1 as libc::c_int {
                        ret_0 = xmlSaveFormatFileEnc(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                            doc,
                            encoding,
                            1 as libc::c_int,
                        );
                    } else {
                        ret_0 = xmlSaveFileEnc(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                            doc,
                            encoding,
                        );
                    }
                    if ret_0 < 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"failed save to %s\n\0" as *const u8 as *const libc::c_char,
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                        );
                        xmllint_progresult = XMLLINT_ERR_OUT;
                    }
                } else if format == 1 as libc::c_int {
                    ret_0 = xmlSaveFormatFile(
                        if !output.is_null() {
                            output
                        } else {
                            b"-\0" as *const u8 as *const libc::c_char
                        },
                        doc,
                        1 as libc::c_int,
                    );
                    if ret_0 < 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"failed save to %s\n\0" as *const u8 as *const libc::c_char,
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                        );
                        xmllint_progresult = XMLLINT_ERR_OUT;
                    }
                } else {
                    let mut out_0: *mut FILE = 0 as *mut FILE;
                    if output.is_null() {
                        out_0 = stdout;
                    } else {
                        out_0 = fopen(
                            output,
                            b"wb\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if !out_0.is_null() {
                        if xmlDocDump(out_0, doc) < 0 as libc::c_int {
                            xmllint_progresult = XMLLINT_ERR_OUT;
                        }
                        if !output.is_null() {
                            fclose(out_0);
                        }
                    } else {
                        fprintf(
                            stderr,
                            b"failed to open %s\n\0" as *const u8 as *const libc::c_char,
                            output,
                        );
                        xmllint_progresult = XMLLINT_ERR_OUT;
                    }
                }
            } else {
                let mut ctxt_3: xmlSaveCtxtPtr = 0 as *mut xmlSaveCtxt;
                let mut saveOpts: libc::c_int = 0 as libc::c_int;
                if format == 1 as libc::c_int {
                    saveOpts |= XML_SAVE_FORMAT as libc::c_int;
                } else if format == 2 as libc::c_int {
                    saveOpts |= XML_SAVE_WSNONSIG as libc::c_int;
                }
                if xmlout != 0 {
                    saveOpts |= XML_SAVE_AS_XML as libc::c_int;
                }
                if output.is_null() {
                    ctxt_3 = xmlSaveToFd(1 as libc::c_int, encoding, saveOpts);
                } else {
                    ctxt_3 = xmlSaveToFilename(output, encoding, saveOpts);
                }
                if !ctxt_3.is_null() {
                    if xmlSaveDoc(ctxt_3, doc) < 0 as libc::c_int as libc::c_long {
                        fprintf(
                            stderr,
                            b"failed save to %s\n\0" as *const u8 as *const libc::c_char,
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const libc::c_char
                            },
                        );
                        xmllint_progresult = XMLLINT_ERR_OUT;
                    }
                    xmlSaveClose(ctxt_3);
                } else {
                    xmllint_progresult = XMLLINT_ERR_OUT;
                }
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(b"Saving\0" as *const u8 as *const libc::c_char);
            }
        } else {
            let mut out_1: *mut FILE = 0 as *mut FILE;
            if output.is_null() {
                out_1 = stdout;
            } else {
                out_1 = fopen(output, b"wb\0" as *const u8 as *const libc::c_char);
            }
            if !out_1.is_null() {
                xmlDebugDumpDocument(out_1, doc);
                if !output.is_null() {
                    fclose(out_1);
                }
            } else {
                fprintf(
                    stderr,
                    b"failed to open %s\n\0" as *const u8 as *const libc::c_char,
                    output,
                );
                xmllint_progresult = XMLLINT_ERR_OUT;
            }
        }
    }
    if !dtdvalid.is_null() || !dtdvalidfpi.is_null() {
        let mut dtd_0: xmlDtdPtr = 0 as *mut xmlDtd;
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        if !dtdvalid.is_null() {
            dtd_0 = xmlParseDTD(0 as *const xmlChar, dtdvalid as *const xmlChar);
        } else {
            dtd_0 = xmlParseDTD(dtdvalidfpi as *const xmlChar, 0 as *const xmlChar);
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Parsing DTD\0" as *const u8 as *const libc::c_char);
        }
        if dtd_0.is_null() {
            if !dtdvalid.is_null() {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Could not parse DTD %s\n\0" as *const u8 as *const libc::c_char,
                    dtdvalid,
                );
            } else {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Could not parse DTD %s\n\0" as *const u8 as *const libc::c_char,
                    dtdvalidfpi,
                );
            }
            xmllint_progresult = XMLLINT_ERR_DTD;
        } else {
            let mut cvp: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
            cvp = xmlNewValidCtxt();
            if cvp.is_null() {
                (*__xmlGenericError())
                    .expect(
                        "non-null function pointer",
                    )(
                    *__xmlGenericErrorContext(),
                    b"Couldn't allocate validation context\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            (*cvp).userData = stderr as *mut libc::c_void;
            (*cvp)
                .error = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlValidityErrorFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            );
            (*cvp)
                .warning = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlValidityWarningFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            );
            if timing != 0 && repeat == 0 {
                xmllint_startTimer();
            }
            if xmlValidateDtd(cvp, doc, dtd_0) == 0 {
                if !dtdvalid.is_null() {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Document %s does not validate against %s\n\0" as *const u8
                            as *const libc::c_char,
                        filename,
                        dtdvalid,
                    );
                } else {
                    (*__xmlGenericError())
                        .expect(
                            "non-null function pointer",
                        )(
                        *__xmlGenericErrorContext(),
                        b"Document %s does not validate against %s\n\0" as *const u8
                            as *const libc::c_char,
                        filename,
                        dtdvalidfpi,
                    );
                }
                xmllint_progresult = XMLLINT_ERR_VALID;
            }
            if timing != 0 && repeat == 0 {
                xmllint_endTimer(
                    b"Validating against DTD\0" as *const u8 as *const libc::c_char,
                );
            }
            xmlFreeValidCtxt(cvp);
            xmlFreeDtd(dtd_0);
        }
    } else if postvalid != 0 {
        let mut cvp_0: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
        cvp_0 = xmlNewValidCtxt();
        if cvp_0.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Couldn't allocate validation context\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        (*cvp_0).userData = stderr as *mut libc::c_void;
        (*cvp_0)
            .error = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
            >,
            xmlValidityErrorFunc,
        >(
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
        );
        (*cvp_0)
            .warning = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
            >,
            xmlValidityWarningFunc,
        >(
            Some(
                fprintf
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
        );
        if xmlValidateDocument(cvp_0, doc) == 0 {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Document %s does not validate\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        }
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\0" as *const u8 as *const libc::c_char);
        }
        xmlFreeValidCtxt(cvp_0);
    }
    if !wxschematron.is_null() {
        let mut ctxt_4: xmlSchematronValidCtxtPtr = 0 as *mut xmlSchematronValidCtxt;
        let mut ret_1: libc::c_int = 0;
        let mut flag: libc::c_int = 0;
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        if debug != 0 {
            flag = XML_SCHEMATRON_OUT_XML as libc::c_int;
        } else {
            flag = XML_SCHEMATRON_OUT_TEXT as libc::c_int;
        }
        if xmllint_noout != 0 {
            flag |= XML_SCHEMATRON_OUT_QUIET as libc::c_int;
        }
        ctxt_4 = xmlSchematronNewValidCtxt(wxschematron, flag);
        ret_1 = xmlSchematronValidateDoc(ctxt_4, doc);
        if ret_1 == 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s validates\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
        } else if ret_1 > 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s fails to validate\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        } else {
            fprintf(
                stderr,
                b"%s validation generated an internal error\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        }
        xmlSchematronFreeValidCtxt(ctxt_4);
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\0" as *const u8 as *const libc::c_char);
        }
    }
    if !relaxngschemas.is_null() {
        let mut ctxt_5: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
        let mut ret_2: libc::c_int = 0;
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        ctxt_5 = xmlRelaxNGNewValidCtxt(relaxngschemas);
        xmlRelaxNGSetValidErrors(
            ctxt_5,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlRelaxNGValidityErrorFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlRelaxNGValidityWarningFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            stderr as *mut libc::c_void,
        );
        ret_2 = xmlRelaxNGValidateDoc(ctxt_5, doc);
        if ret_2 == 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s validates\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
        } else if ret_2 > 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s fails to validate\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        } else {
            fprintf(
                stderr,
                b"%s validation generated an internal error\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        }
        xmlRelaxNGFreeValidCtxt(ctxt_5);
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\0" as *const u8 as *const libc::c_char);
        }
    } else if !wxschemas.is_null() {
        let mut ctxt_6: xmlSchemaValidCtxtPtr = 0 as *mut xmlSchemaValidCtxt;
        let mut ret_3: libc::c_int = 0;
        if timing != 0 && repeat == 0 {
            xmllint_startTimer();
        }
        ctxt_6 = xmlSchemaNewValidCtxt(wxschemas);
        xmlSchemaSetValidErrors(
            ctxt_6,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlSchemaValidityErrorFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlSchemaValidityWarningFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            stderr as *mut libc::c_void,
        );
        ret_3 = xmlSchemaValidateDoc(ctxt_6, doc);
        if ret_3 == 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s validates\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
        } else if ret_3 > 0 as libc::c_int {
            fprintf(
                stderr,
                b"%s fails to validate\n\0" as *const u8 as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        } else {
            fprintf(
                stderr,
                b"%s validation generated an internal error\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            xmllint_progresult = XMLLINT_ERR_VALID;
        }
        xmlSchemaFreeValidCtxt(ctxt_6);
        if timing != 0 && repeat == 0 {
            xmllint_endTimer(b"Validating\0" as *const u8 as *const libc::c_char);
        }
    }
    if debugent != 0 && html == 0 {
        xmlDebugDumpEntities(stderr, doc);
    }
    if timing != 0 && repeat == 0 {
        xmllint_startTimer();
    }
    xmlFreeDoc(doc);
    if timing != 0 && repeat == 0 {
        xmllint_endTimer(b"Freeing\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn showVersion(mut name: *const libc::c_char) {
    fprintf(
        stderr,
        b"%s: using libxml version %s\n\0" as *const u8 as *const libc::c_char,
        name,
        *__xmlParserVersion(),
    );
    fprintf(stderr, b"   compiled with: \0" as *const u8 as *const libc::c_char);
    if xmlHasFeature(XML_WITH_THREAD) != 0 {
        fprintf(stderr, b"Threads \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_TREE) != 0 {
        fprintf(stderr, b"Tree \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_OUTPUT) != 0 {
        fprintf(stderr, b"Output \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_PUSH) != 0 {
        fprintf(stderr, b"Push \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_READER) != 0 {
        fprintf(stderr, b"Reader \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_PATTERN) != 0 {
        fprintf(stderr, b"Patterns \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_WRITER) != 0 {
        fprintf(stderr, b"Writer \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_SAX1) != 0 {
        fprintf(stderr, b"SAXv1 \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_FTP) != 0 {
        fprintf(stderr, b"FTP \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_HTTP) != 0 {
        fprintf(stderr, b"HTTP \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_VALID) != 0 {
        fprintf(stderr, b"DTDValid \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_HTML) != 0 {
        fprintf(stderr, b"HTML \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_LEGACY) != 0 {
        fprintf(stderr, b"Legacy \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_C14N) != 0 {
        fprintf(stderr, b"C14N \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_CATALOG) != 0 {
        fprintf(stderr, b"Catalog \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_XPATH) != 0 {
        fprintf(stderr, b"XPath \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_XPTR) != 0 {
        fprintf(stderr, b"XPointer \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_XINCLUDE) != 0 {
        fprintf(stderr, b"XInclude \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_ICONV) != 0 {
        fprintf(stderr, b"Iconv \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_ICU) != 0 {
        fprintf(stderr, b"ICU \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_ISO8859X) != 0 {
        fprintf(stderr, b"ISO8859X \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_UNICODE) != 0 {
        fprintf(stderr, b"Unicode \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_REGEXP) != 0 {
        fprintf(stderr, b"Regexps \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_AUTOMATA) != 0 {
        fprintf(stderr, b"Automata \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_EXPR) != 0 {
        fprintf(stderr, b"Expr \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_SCHEMAS) != 0 {
        fprintf(stderr, b"Schemas \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_SCHEMATRON) != 0 {
        fprintf(stderr, b"Schematron \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_MODULES) != 0 {
        fprintf(stderr, b"Modules \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_DEBUG) != 0 {
        fprintf(stderr, b"Debug \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_DEBUG_MEM) != 0 {
        fprintf(stderr, b"MemDebug \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_DEBUG_RUN) != 0 {
        fprintf(stderr, b"RunDebug \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_ZLIB) != 0 {
        fprintf(stderr, b"Zlib \0" as *const u8 as *const libc::c_char);
    }
    if xmlHasFeature(XML_WITH_LZMA) != 0 {
        fprintf(stderr, b"Lzma \0" as *const u8 as *const libc::c_char);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn usage(mut f: *mut FILE, mut name: *const libc::c_char) {
    fprintf(
        f,
        b"Usage : %s [options] XMLfiles ...\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    fprintf(
        f,
        b"\tParse the XML files and output the result of the parsing\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--version : display the version of the XML library used\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--debug : dump a debug tree of the in-memory document\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--shell : run a navigating shell\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--debugent : debug the entities defined in the document\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--copy : used to test the internal copy implementation\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--recover : output what was parsable on broken XML documents\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--huge : remove any internal arbitrary parser limits\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--noent : substitute entity references by their value\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--noenc : ignore any encoding specified inside the document\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--noout : don't output the result tree\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--path 'paths': provide a set of paths for resources\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--load-trace : print trace of all external entities loaded\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nonet : refuse to fetch DTDs or entities over network\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nocompact : do not generate compact text nodes\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--htmlout : output results as HTML\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nowrap : do not put HTML doc wrapper\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--valid : validate the document in addition to std well-formed check\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--postvalid : do a posteriori validation, i.e after parsing\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--dtdvalid URL : do a posteriori validation against a given DTD\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--dtdvalidfpi FPI : same but name the DTD with a Public Identifier\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--timing : print some timings\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--output file or -o file: save to a given file\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--repeat : repeat 100 times, for timing or profiling\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--insert : ad-hoc test for valid insertions\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--compress : turn on gzip compression of output\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--html : use the HTML parser\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--xmlout : force to use the XML serializer when using --html\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nodefdtd : do not default HTML doctype\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--push : use the push mode of the parser\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--pushsmall : use the push mode of the parser using tiny increments\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--memory : parse from memory\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--maxmem nbbytes : limits memory allocation to nbbytes bytes\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nowarning : do not emit warnings from parser/validator\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--noblanks : drop (ignorable?) blanks spaces\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nocdata : replace cdata section with text nodes\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--format : reformat/reindent the output\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--encode encoding : output in the given encoding\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--dropdtd : remove the DOCTYPE of the input docs\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--pretty STYLE : pretty-print in a particular style\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t                 0 Do not pretty print\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t                 1 Format the XML content, as --format\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t                 2 Add whitespace inside tags, preserving content\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--c14n : save in W3C canonical format v1.0 (with comments)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--c14n11 : save in W3C canonical format v1.1 (with comments)\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--exc-c14n : save in W3C exclusive canonical format (with comments)\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nsclean : remove redundant namespace declarations\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--testIO : test user I/O support\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--catalogs : use SGML catalogs from $SGML_CATALOG_FILES\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t             otherwise XML Catalogs starting from \n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t         %s are activated by default\n\0" as *const u8
            as *const libc::c_char,
        b"file:///etc/xml/catalog\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nocatalogs: deactivate all catalogs\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--auto : generate a small doc on the fly\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--xinclude : do XInclude processing\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--noxincludenode : same but do not generate XInclude nodes\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--nofixup-base-uris : do not fixup xml:base uris\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--loaddtd : fetch external DTD\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--dtdattr : loaddtd + populate the tree with inherited attributes \n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--stream : use the streaming interface to process very large files\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--walker : create a reader and walk though the resulting doc\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--pattern pattern_value : test the pattern support\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--chkregister : verify the node registration code\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--relaxng schema : do RelaxNG validation against the schema\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--schema schema : do validation against the WXS schema\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--schematron schema : do validation against a schematron\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--sax1: use the old SAX1 interfaces for processing\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--sax: do not build a tree but work just at the SAX level\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--oldxml10: use XML-1.0 parsing rules before the 5th edition\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        f,
        b"\t--xpath expr: evaluate the XPath expression, imply --noout\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"\nLibxml project home page: http://xmlsoft.org/\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        f,
        b"To report bugs or get some help check: http://xmlsoft.org/bugs.html\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn registerNode(mut node: xmlNodePtr) {
    (*node)._private = malloc(::core::mem::size_of::<libc::c_long>() as libc::c_ulong);
    if ((*node)._private).is_null() {
        fprintf(
            stderr,
            b"Out of memory in xmllint:registerNode()\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(XMLLINT_ERR_MEM as libc::c_int);
    }
    *((*node)._private
        as *mut libc::c_long) = 0x81726354 as libc::c_uint as libc::c_long;
    nbregister += 1;
    nbregister;
}
unsafe extern "C" fn deregisterNode(mut node: xmlNodePtr) {
    if !((*node)._private).is_null() {} else {
        __assert_fail(
            b"node->_private != NULL\0" as *const u8 as *const libc::c_char,
            b"xmllint.c\0" as *const u8 as *const libc::c_char,
            3110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void deregisterNode(xmlNodePtr)\0"))
                .as_ptr(),
        );
    }
    'c_21825: {
        if !((*node)._private).is_null() {} else {
            __assert_fail(
                b"node->_private != NULL\0" as *const u8 as *const libc::c_char,
                b"xmllint.c\0" as *const u8 as *const libc::c_char,
                3110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void deregisterNode(xmlNodePtr)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*node)._private as *mut libc::c_long)
        == 0x81726354 as libc::c_uint as libc::c_long
    {} else {
        __assert_fail(
            b"*(long*)node->_private == (long) 0x81726354\0" as *const u8
                as *const libc::c_char,
            b"xmllint.c\0" as *const u8 as *const libc::c_char,
            3111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void deregisterNode(xmlNodePtr)\0"))
                .as_ptr(),
        );
    }
    'c_21775: {
        if *((*node)._private as *mut libc::c_long)
            == 0x81726354 as libc::c_uint as libc::c_long
        {} else {
            __assert_fail(
                b"*(long*)node->_private == (long) 0x81726354\0" as *const u8
                    as *const libc::c_char,
                b"xmllint.c\0" as *const u8 as *const libc::c_char,
                3111 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void deregisterNode(xmlNodePtr)\0"))
                    .as_ptr(),
            );
        }
    };
    free((*node)._private);
    nbregister -= 1;
    nbregister;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut acount: libc::c_int = 0;
    let mut files: libc::c_int = 0 as libc::c_int;
    let mut version: libc::c_int = 0 as libc::c_int;
    let mut indent: *const libc::c_char = 0 as *const libc::c_char;
    if argc <= 1 as libc::c_int {
        usage(stderr, *argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    xmlCheckVersion(20908 as libc::c_int);
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(*argv.offset(i as isize), b"-\0" as *const u8 as *const libc::c_char)
            == 0
        {
            break;
        }
        if !(*(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != '-' as i32)
        {
            if strcmp(
                *argv.offset(i as isize),
                b"-debug\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--debug\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                debug += 1;
                debug;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-shell\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--shell\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                shell += 1;
                shell;
                xmllint_noout = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-copy\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--copy\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                copy += 1;
                copy;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-recover\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--recover\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                recovery += 1;
                recovery;
                options |= XML_PARSE_RECOVER as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-huge\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--huge\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                options |= XML_PARSE_HUGE as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noent\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noent\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                noent += 1;
                noent;
                options |= XML_PARSE_NOENT as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noenc\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noenc\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                noenc += 1;
                noenc;
                options |= XML_PARSE_IGNORE_ENC as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nsclean\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nsclean\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                options |= XML_PARSE_NSCLEAN as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nocdata\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nocdata\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                options |= XML_PARSE_NOCDATA as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nodict\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nodict\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                options |= XML_PARSE_NODICT as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-version\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--version\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                showVersion(*argv.offset(0 as libc::c_int as isize));
                version = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noout\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noout\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                xmllint_noout += 1;
                xmllint_noout;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-o\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"-output\0" as *const u8 as *const libc::c_char,
                ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--output\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                output = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-htmlout\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--htmlout\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                htmlout += 1;
                htmlout;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nowrap\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nowrap\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                nowrap += 1;
                nowrap;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-html\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--html\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                html += 1;
                html;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-xmlout\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--xmlout\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                xmlout += 1;
                xmlout;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nodefdtd\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nodefdtd\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                nodefdtd += 1;
                nodefdtd;
                options |= HTML_PARSE_NODEFDTD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-loaddtd\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--loaddtd\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                loaddtd += 1;
                loaddtd;
                options |= XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dtdattr\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dtdattr\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                loaddtd += 1;
                loaddtd;
                dtdattrs += 1;
                dtdattrs;
                options |= XML_PARSE_DTDATTR as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-valid\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--valid\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                valid += 1;
                valid;
                options |= XML_PARSE_DTDVALID as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-postvalid\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--postvalid\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                postvalid += 1;
                postvalid;
                loaddtd += 1;
                loaddtd;
                options |= XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dtdvalid\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dtdvalid\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                dtdvalid = *argv.offset(i as isize);
                loaddtd += 1;
                loaddtd;
                options |= XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dtdvalidfpi\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dtdvalidfpi\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                dtdvalidfpi = *argv.offset(i as isize);
                loaddtd += 1;
                loaddtd;
                options |= XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dropdtd\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dropdtd\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                dropdtd += 1;
                dropdtd;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-insert\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--insert\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                insert += 1;
                insert;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-timing\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--timing\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                timing += 1;
                timing;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-auto\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--auto\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                generate += 1;
                generate;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-repeat\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--repeat\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                if repeat != 0 {
                    repeat *= 10 as libc::c_int;
                } else {
                    repeat = 100 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-push\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--push\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                push += 1;
                push;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pushsmall\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pushsmall\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                push += 1;
                push;
                pushsize = 10 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-memory\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--memory\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                memory += 1;
                memory;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-testIO\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--testIO\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                testIO += 1;
                testIO;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-xinclude\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--xinclude\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                xinclude += 1;
                xinclude;
                options |= XML_PARSE_XINCLUDE as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noxincludenode\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noxincludenode\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                xinclude += 1;
                xinclude;
                options |= XML_PARSE_XINCLUDE as libc::c_int;
                options |= XML_PARSE_NOXINCNODE as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nofixup-base-uris\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nofixup-base-uris\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                xinclude += 1;
                xinclude;
                options |= XML_PARSE_XINCLUDE as libc::c_int;
                options |= XML_PARSE_NOBASEFIX as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-compress\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--compress\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                compress += 1;
                compress;
                xmlSetCompressMode(9 as libc::c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nowarning\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nowarning\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                *__xmlGetWarningsDefaultValue() = 0 as libc::c_int;
                xmlPedanticParserDefault(0 as libc::c_int);
                options |= XML_PARSE_NOWARNING as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pedantic\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pedantic\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                *__xmlGetWarningsDefaultValue() = 1 as libc::c_int;
                xmlPedanticParserDefault(1 as libc::c_int);
                options |= XML_PARSE_PEDANTIC as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-debugent\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--debugent\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                debugent += 1;
                debugent;
                *__xmlParserDebugEntities() = 1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-c14n\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--c14n\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                canonical += 1;
                canonical;
                options
                    |= XML_PARSE_NOENT as libc::c_int | XML_PARSE_DTDATTR as libc::c_int
                        | XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-c14n11\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--c14n11\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                canonical_11 += 1;
                canonical_11;
                options
                    |= XML_PARSE_NOENT as libc::c_int | XML_PARSE_DTDATTR as libc::c_int
                        | XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-exc-c14n\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--exc-c14n\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                exc_canonical += 1;
                exc_canonical;
                options
                    |= XML_PARSE_NOENT as libc::c_int | XML_PARSE_DTDATTR as libc::c_int
                        | XML_PARSE_DTDLOAD as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-catalogs\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--catalogs\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                catalogs += 1;
                catalogs;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nocatalogs\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nocatalogs\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                nocatalogs += 1;
                nocatalogs;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-encode\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--encode\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                encoding = *argv.offset(i as isize);
                xmlAddEncodingAlias(
                    b"UTF-8\0" as *const u8 as *const libc::c_char,
                    b"DVEnc\0" as *const u8 as *const libc::c_char,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noblanks\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noblanks\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                noblanks += 1;
                noblanks;
                xmlKeepBlanksDefault(0 as libc::c_int);
                options |= XML_PARSE_NOBLANKS as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-maxmem\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--maxmem\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                if sscanf(
                    *argv.offset(i as isize),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut maxmem as *mut libc::c_int,
                ) == 1 as libc::c_int
                {
                    xmlMemSetup(
                        Some(
                            myFreeFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
                        ),
                        Some(
                            myMallocFunc
                                as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
                        ),
                        Some(
                            myReallocFunc
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    size_t,
                                ) -> *mut libc::c_void,
                        ),
                        Some(
                            myStrdupFunc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> *mut libc::c_char,
                        ),
                    );
                } else {
                    maxmem = 0 as libc::c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-format\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--format\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                noblanks += 1;
                noblanks;
                format = 1 as libc::c_int;
                xmlKeepBlanksDefault(0 as libc::c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pretty\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pretty\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                if !(*argv.offset(i as isize)).is_null() {
                    format = atoi(*argv.offset(i as isize));
                    if format == 1 as libc::c_int {
                        noblanks += 1;
                        noblanks;
                        xmlKeepBlanksDefault(0 as libc::c_int);
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-stream\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--stream\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                stream += 1;
                stream;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-walker\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--walker\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                walker += 1;
                walker;
                xmllint_noout += 1;
                xmllint_noout;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-sax1\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--sax1\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                sax1 += 1;
                sax1;
                options |= XML_PARSE_SAX1 as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-sax\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--sax\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                sax += 1;
                sax;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-chkregister\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--chkregister\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                chkregister += 1;
                chkregister;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-relaxng\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--relaxng\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                relaxng = *argv.offset(i as isize);
                noent += 1;
                noent;
                options |= XML_PARSE_NOENT as libc::c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-schema\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--schema\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                schema = *argv.offset(i as isize);
                noent += 1;
                noent;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-schematron\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--schematron\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                schematron = *argv.offset(i as isize);
                noent += 1;
                noent;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nonet\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nonet\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                options |= XML_PARSE_NONET as libc::c_int;
                xmlSetExternalEntityLoader(
                    Some(
                        xmlNoNetExternalEntityLoader
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *const libc::c_char,
                                xmlParserCtxtPtr,
                            ) -> xmlParserInputPtr,
                    ),
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nocompact\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nocompact\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                options &= !(XML_PARSE_COMPACT as libc::c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-load-trace\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--load-trace\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                load_trace += 1;
                load_trace;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-path\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--path\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                parsePath(*argv.offset(i as isize) as *mut xmlChar);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pattern\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pattern\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                pattern = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-xpath\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--xpath\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                i += 1;
                i;
                xmllint_noout += 1;
                xmllint_noout;
                xpathquery = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-oldxml10\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--oldxml10\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                oldxml10 += 1;
                oldxml10;
                options |= XML_PARSE_OLD10 as libc::c_int;
            } else {
                fprintf(
                    stderr,
                    b"Unknown option %s\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
                usage(stderr, *argv.offset(0 as libc::c_int as isize));
                return 1 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    if nocatalogs == 0 as libc::c_int {
        if catalogs != 0 {
            let mut catal: *const libc::c_char = 0 as *const libc::c_char;
            catal = getenv(b"SGML_CATALOG_FILES\0" as *const u8 as *const libc::c_char);
            if !catal.is_null() {
                xmlLoadCatalogs(catal);
            } else {
                fprintf(
                    stderr,
                    b"Variable $SGML_CATALOG_FILES not set\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if sax1 != 0 {
        xmlSAXDefaultVersion(1 as libc::c_int);
    } else {
        xmlSAXDefaultVersion(2 as libc::c_int);
    }
    if chkregister != 0 {
        xmlRegisterNodeDefault(
            Some(registerNode as unsafe extern "C" fn(xmlNodePtr) -> ()),
        );
        xmlDeregisterNodeDefault(
            Some(deregisterNode as unsafe extern "C" fn(xmlNodePtr) -> ()),
        );
    }
    indent = getenv(b"XMLLINT_INDENT\0" as *const u8 as *const libc::c_char);
    if !indent.is_null() {
        let ref mut fresh4 = *__xmlTreeIndentString();
        *fresh4 = indent;
    }
    defaultEntityLoader = xmlGetExternalEntityLoader();
    xmlSetExternalEntityLoader(
        Some(
            xmllintExternalEntityLoader
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    xmlParserCtxtPtr,
                ) -> xmlParserInputPtr,
        ),
    );
    xmlLineNumbersDefault(1 as libc::c_int);
    if loaddtd != 0 as libc::c_int {
        *__xmlLoadExtDtdDefaultValue() |= 2 as libc::c_int;
    }
    if dtdattrs != 0 {
        *__xmlLoadExtDtdDefaultValue() |= 4 as libc::c_int;
    }
    if noent != 0 as libc::c_int {
        xmlSubstituteEntitiesDefault(1 as libc::c_int);
    }
    if valid != 0 as libc::c_int {
        *__xmlDoValidityCheckingDefaultValue() = 1 as libc::c_int;
    }
    if htmlout != 0 && nowrap == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0 Transitional//EN\"\n\0"
                as *const u8 as *const libc::c_char,
        );
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"\t\"http://www.w3.org/TR/REC-html40/loose.dtd\">\n\0" as *const u8
                as *const libc::c_char,
        );
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"<html><head><title>%s output</title></head>\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"<body bgcolor=\"#ffffff\"><h1 align=\"center\">%s output</h1>\n\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
    }
    if !schematron.is_null() && sax == 0 as libc::c_int && stream == 0 as libc::c_int {
        let mut ctxt: xmlSchematronParserCtxtPtr = 0 as *mut xmlSchematronParserCtxt;
        *__xmlLoadExtDtdDefaultValue() |= 1 as libc::c_int;
        options |= XML_PARSE_DTDLOAD as libc::c_int;
        if timing != 0 {
            xmllint_startTimer();
        }
        ctxt = xmlSchematronNewParserCtxt(schematron);
        wxschematron = xmlSchematronParse(ctxt);
        if wxschematron.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Schematron schema %s failed to compile\n\0" as *const u8
                    as *const libc::c_char,
                schematron,
            );
            xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
            schematron = 0 as *mut libc::c_char;
        }
        xmlSchematronFreeParserCtxt(ctxt);
        if timing != 0 {
            xmllint_endTimer(
                b"Compiling the schemas\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if !relaxng.is_null() && sax == 0 as libc::c_int && stream == 0 as libc::c_int {
        let mut ctxt_0: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
        *__xmlLoadExtDtdDefaultValue() |= 1 as libc::c_int;
        options |= XML_PARSE_DTDLOAD as libc::c_int;
        if timing != 0 {
            xmllint_startTimer();
        }
        ctxt_0 = xmlRelaxNGNewParserCtxt(relaxng);
        xmlRelaxNGSetParserErrors(
            ctxt_0,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlRelaxNGValidityErrorFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlRelaxNGValidityWarningFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            stderr as *mut libc::c_void,
        );
        relaxngschemas = xmlRelaxNGParse(ctxt_0);
        if relaxngschemas.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Relax-NG schema %s failed to compile\n\0" as *const u8
                    as *const libc::c_char,
                relaxng,
            );
            xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
            relaxng = 0 as *mut libc::c_char;
        }
        xmlRelaxNGFreeParserCtxt(ctxt_0);
        if timing != 0 {
            xmllint_endTimer(
                b"Compiling the schemas\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if !schema.is_null() && stream == 0 as libc::c_int {
        let mut ctxt_1: xmlSchemaParserCtxtPtr = 0 as *mut xmlSchemaParserCtxt;
        if timing != 0 {
            xmllint_startTimer();
        }
        ctxt_1 = xmlSchemaNewParserCtxt(schema);
        xmlSchemaSetParserErrors(
            ctxt_1,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlSchemaValidityErrorFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
                >,
                xmlSchemaValidityWarningFunc,
            >(
                Some(
                    fprintf
                        as unsafe extern "C" fn(
                            *mut FILE,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
            ),
            stderr as *mut libc::c_void,
        );
        wxschemas = xmlSchemaParse(ctxt_1);
        if wxschemas.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"WXS schema %s failed to compile\n\0" as *const u8
                    as *const libc::c_char,
                schema,
            );
            xmllint_progresult = XMLLINT_ERR_SCHEMACOMP;
            schema = 0 as *mut libc::c_char;
        }
        xmlSchemaFreeParserCtxt(ctxt_1);
        if timing != 0 {
            xmllint_endTimer(
                b"Compiling the schemas\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if !pattern.is_null() && walker == 0 as libc::c_int {
        patternc = xmlPatterncompile(
            pattern as *const xmlChar,
            0 as *mut xmlDict,
            0 as libc::c_int,
            0 as *mut *const xmlChar,
        );
        if patternc.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Pattern %s failed to compile\n\0" as *const u8 as *const libc::c_char,
                pattern,
            );
            xmllint_progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = 0 as *const libc::c_char;
        }
    }
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-encode\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--encode\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-o\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"-output\0" as *const u8 as *const libc::c_char,
            ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--output\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-dtdvalid\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--dtdvalid\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-path\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--path\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-dtdvalidfpi\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--dtdvalidfpi\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-relaxng\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--relaxng\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-maxmem\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--maxmem\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-pretty\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--pretty\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-schema\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--schema\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-schematron\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--schematron\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-pattern\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--pattern\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-xpath\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--xpath\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            i += 1;
            i;
        } else {
            if timing != 0 && repeat != 0 {
                xmllint_startTimer();
            }
            if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int != '-' as i32
                || strcmp(
                    *argv.offset(i as isize),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                if repeat != 0 {
                    let mut ctxt_2: xmlParserCtxtPtr = 0 as xmlParserCtxtPtr;
                    acount = 0 as libc::c_int;
                    while acount < repeat {
                        if stream != 0 as libc::c_int {
                            streamFile(*argv.offset(i as isize));
                        } else if sax != 0 {
                            testSAX(*argv.offset(i as isize));
                        } else {
                            if ctxt_2.is_null() {
                                ctxt_2 = xmlNewParserCtxt();
                            }
                            parseAndPrintFile(*argv.offset(i as isize), ctxt_2);
                        }
                        acount += 1;
                        acount;
                    }
                    if !ctxt_2.is_null() {
                        xmlFreeParserCtxt(ctxt_2);
                    }
                } else {
                    nbregister = 0 as libc::c_int;
                    if stream != 0 as libc::c_int {
                        streamFile(*argv.offset(i as isize));
                    } else if sax != 0 {
                        testSAX(*argv.offset(i as isize));
                    } else {
                        parseAndPrintFile(
                            *argv.offset(i as isize),
                            0 as xmlParserCtxtPtr,
                        );
                    }
                    if chkregister != 0 && nbregister != 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Registration count off: %d\n\0" as *const u8
                                as *const libc::c_char,
                            nbregister,
                        );
                        xmllint_progresult = XMLLINT_ERR_RDREGIS;
                    }
                }
                files += 1;
                files;
                if timing != 0 && repeat != 0 {
                    xmllint_endTimer(
                        b"%d iterations\0" as *const u8 as *const libc::c_char,
                        repeat,
                    );
                }
            }
        }
        i += 1;
        i;
    }
    if generate != 0 {
        parseAndPrintFile(0 as *mut libc::c_char, 0 as xmlParserCtxtPtr);
    }
    if htmlout != 0 && nowrap == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"</body></html>\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if files == 0 as libc::c_int && generate == 0 && version == 0 as libc::c_int {
        usage(stderr, *argv.offset(0 as libc::c_int as isize));
    }
    if !wxschematron.is_null() {
        xmlSchematronFree(wxschematron);
    }
    if !relaxngschemas.is_null() {
        xmlRelaxNGFree(relaxngschemas);
    }
    if !wxschemas.is_null() {
        xmlSchemaFree(wxschemas);
    }
    xmlRelaxNGCleanupTypes();
    if !patternc.is_null() {
        xmlFreePattern(patternc);
    }
    xmlCleanupParser();
    xmlMemoryDump();
    return xmllint_progresult as libc::c_int;
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
