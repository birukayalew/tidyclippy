use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type _tccdbg;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn pstrcpy(
        buf: *mut libc::c_char,
        buf_size: size_t,
        s: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn tcc_basename(name: *const libc::c_char) -> *mut libc::c_char;
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_mallocz(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_realloc(ptr: *mut libc::c_void, size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn _tcc_error_noabort(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _tcc_warning(fmt: *const libc::c_char, _: ...);
    fn dynarray_add(
        ptab: *mut libc::c_void,
        nb_ptr: *mut libc::c_int,
        data: *mut libc::c_void,
    );
    fn dynarray_reset(pp: *mut libc::c_void, n: *mut libc::c_int);
    fn cstr_new(cstr: *mut CString);
    fn cstr_free(cstr: *mut CString);
    fn cstr_printf(cs: *mut CString, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn tcc_add_file_internal(
        s1: *mut TCCState,
        filename: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn tcc_add_crt(s: *mut TCCState, filename: *const libc::c_char) -> libc::c_int;
    fn tcc_add_dll(
        s: *mut TCCState,
        filename: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn tcc_add_support(s1: *mut TCCState, filename: *const libc::c_char) -> libc::c_int;
    fn tcc_add_pragma_libs(s1: *mut TCCState);
    fn tcc_add_dllref(
        s1: *mut TCCState,
        dllname: *const libc::c_char,
        level: libc::c_int,
    ) -> *mut DLLReference;
    fn tcc_load_text(fd: libc::c_int) -> *mut libc::c_char;
    fn code_reloc(reloc_type: libc::c_int) -> libc::c_int;
    fn gotplt_entry_type(reloc_type: libc::c_int) -> libc::c_int;
    fn create_plt_entry(
        s1: *mut TCCState,
        got_offset: libc::c_uint,
        attr: *mut sym_attr,
    ) -> libc::c_uint;
    fn relocate_plt(s1: *mut TCCState);
    fn relocate(
        s1: *mut TCCState,
        rel: *mut Elf64_Rela,
        type_0: libc::c_int,
        ptr: *mut libc::c_uchar,
        addr: Elf64_Addr,
        val: Elf64_Addr,
    );
    fn tcc_debug_new(s: *mut TCCState);
    fn tcc_eh_frame_start(s1: *mut TCCState);
    fn tcc_eh_frame_hdr(s1: *mut TCCState, final_0: libc::c_int);
    fn tcc_enter_state(s1: *mut TCCState);
    fn tcc_compile_string(s: *mut TCCState, buf: *const libc::c_char) -> libc::c_int;
    fn tcc_add_library(
        s: *mut TCCState,
        libraryname: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCCState {
    pub verbose: libc::c_uchar,
    pub nostdinc: libc::c_uchar,
    pub nostdlib: libc::c_uchar,
    pub nostdlib_paths: libc::c_uchar,
    pub nocommon: libc::c_uchar,
    pub static_link: libc::c_uchar,
    pub rdynamic: libc::c_uchar,
    pub symbolic: libc::c_uchar,
    pub filetype: libc::c_uchar,
    pub optimize: libc::c_uchar,
    pub option_pthread: libc::c_uchar,
    pub enable_new_dtags: libc::c_uchar,
    pub cversion: libc::c_uint,
    pub char_is_unsigned: libc::c_uchar,
    pub leading_underscore: libc::c_uchar,
    pub ms_extensions: libc::c_uchar,
    pub dollars_in_identifiers: libc::c_uchar,
    pub ms_bitfields: libc::c_uchar,
    pub reverse_funcargs: libc::c_uchar,
    pub gnu89_inline: libc::c_uchar,
    pub unwind_tables: libc::c_uchar,
    pub warn_none: libc::c_uchar,
    pub warn_all: libc::c_uchar,
    pub warn_error: libc::c_uchar,
    pub warn_write_strings: libc::c_uchar,
    pub warn_unsupported: libc::c_uchar,
    pub warn_implicit_function_declaration: libc::c_uchar,
    pub warn_discarded_qualifiers: libc::c_uchar,
    pub warn_num: libc::c_uchar,
    pub option_r: libc::c_uchar,
    pub do_bench: libc::c_uchar,
    pub just_deps: libc::c_uchar,
    pub gen_deps: libc::c_uchar,
    pub include_sys_deps: libc::c_uchar,
    pub gen_phony_deps: libc::c_uchar,
    pub do_debug: libc::c_uchar,
    pub dwarf: libc::c_uchar,
    pub do_backtrace: libc::c_uchar,
    pub do_bounds_check: libc::c_uchar,
    pub test_coverage: libc::c_uchar,
    pub gnu_ext: libc::c_uchar,
    pub tcc_ext: libc::c_uchar,
    pub dflag: libc::c_uchar,
    pub Pflag: libc::c_uchar,
    pub nosse: libc::c_uchar,
    pub has_text_addr: libc::c_uchar,
    pub text_addr: Elf64_Addr,
    pub section_align: libc::c_uint,
    pub tcc_lib_path: *mut libc::c_char,
    pub soname: *mut libc::c_char,
    pub rpath: *mut libc::c_char,
    pub elfint: *mut libc::c_char,
    pub elf_entryname: *mut libc::c_char,
    pub init_symbol: *mut libc::c_char,
    pub fini_symbol: *mut libc::c_char,
    pub mapfile: *mut libc::c_char,
    pub output_type: libc::c_int,
    pub output_format: libc::c_int,
    pub run_test: libc::c_int,
    pub loaded_dlls: *mut *mut DLLReference,
    pub nb_loaded_dlls: libc::c_int,
    pub include_paths: *mut *mut libc::c_char,
    pub nb_include_paths: libc::c_int,
    pub sysinclude_paths: *mut *mut libc::c_char,
    pub nb_sysinclude_paths: libc::c_int,
    pub library_paths: *mut *mut libc::c_char,
    pub nb_library_paths: libc::c_int,
    pub crt_paths: *mut *mut libc::c_char,
    pub nb_crt_paths: libc::c_int,
    pub cmdline_defs: CString,
    pub cmdline_incl: CString,
    pub error_opaque: *mut libc::c_void,
    pub error_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> (),
    >,
    pub error_set_jmp_enabled: libc::c_int,
    pub error_jmp_buf: jmp_buf,
    pub nb_errors: libc::c_int,
    pub ppfp: *mut FILE,
    pub target_deps: *mut *mut libc::c_char,
    pub nb_target_deps: libc::c_int,
    pub include_stack: [*mut BufferedFile; 32],
    pub include_stack_ptr: *mut *mut BufferedFile,
    pub ifdef_stack: [libc::c_int; 64],
    pub ifdef_stack_ptr: *mut libc::c_int,
    pub cached_includes_hash: [libc::c_int; 32],
    pub cached_includes: *mut *mut CachedInclude,
    pub nb_cached_includes: libc::c_int,
    pub pack_stack: [libc::c_int; 8],
    pub pack_stack_ptr: *mut libc::c_int,
    pub pragma_libs: *mut *mut libc::c_char,
    pub nb_pragma_libs: libc::c_int,
    pub inline_fns: *mut *mut InlineFunc,
    pub nb_inline_fns: libc::c_int,
    pub sections: *mut *mut Section,
    pub nb_sections: libc::c_int,
    pub priv_sections: *mut *mut Section,
    pub nb_priv_sections: libc::c_int,
    pub text_section: *mut Section,
    pub data_section: *mut Section,
    pub rodata_section: *mut Section,
    pub bss_section: *mut Section,
    pub common_section: *mut Section,
    pub cur_text_section: *mut Section,
    pub bounds_section: *mut Section,
    pub lbounds_section: *mut Section,
    pub c2rust_unnamed: C2RustUnnamed,
    pub dynsymtab_section: *mut Section,
    pub dynsym: *mut Section,
    pub got: *mut Section,
    pub plt: *mut Section,
    pub eh_frame_section: *mut Section,
    pub eh_frame_hdr_section: *mut Section,
    pub eh_start: libc::c_ulong,
    pub stab_section: *mut Section,
    pub dwarf_info_section: *mut Section,
    pub dwarf_abbrev_section: *mut Section,
    pub dwarf_line_section: *mut Section,
    pub dwarf_aranges_section: *mut Section,
    pub dwarf_str_section: *mut Section,
    pub dwarf_line_str_section: *mut Section,
    pub dwlo: libc::c_int,
    pub dwhi: libc::c_int,
    pub tcov_section: *mut Section,
    pub dState: *mut _tccdbg,
    pub sym_attrs: *mut sym_attr,
    pub nb_sym_attrs: libc::c_int,
    pub qrel: *mut Elf64_Rela,
    pub nb_sym_versions: libc::c_int,
    pub sym_versions: *mut sym_version,
    pub nb_sym_to_version: libc::c_int,
    pub sym_to_version: *mut libc::c_int,
    pub dt_verneednum: libc::c_int,
    pub versym_section: *mut Section,
    pub verneed_section: *mut Section,
    pub run_main: *const libc::c_char,
    pub run_ptr: *mut libc::c_void,
    pub run_size: libc::c_uint,
    pub next: *mut TCCState,
    pub rc: *mut rt_context,
    pub run_lj: *mut libc::c_void,
    pub run_jb: *mut libc::c_void,
    pub bt_func: Option::<TCCBtFunc>,
    pub bt_data: *mut libc::c_void,
    pub rt_num_callers: libc::c_int,
    pub total_idents: libc::c_int,
    pub total_lines: libc::c_int,
    pub total_bytes: libc::c_uint,
    pub total_output: [libc::c_uint; 4],
    pub ld_p: *mut libc::c_uchar,
    pub current_filename: *const libc::c_char,
    pub files: *mut *mut filespec,
    pub nb_files: libc::c_int,
    pub nb_libraries: libc::c_int,
    pub outfile: *mut libc::c_char,
    pub deps_outfile: *mut libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub link_argv: *mut *mut libc::c_char,
    pub link_argc: libc::c_int,
    pub link_optind: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filespec {
    pub type_0: libc::c_char,
    pub name: [libc::c_char; 1],
}
pub type TCCBtFunc = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    *const libc::c_char,
    libc::c_int,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Section {
    pub data_offset: libc::c_ulong,
    pub data: *mut libc::c_uchar,
    pub data_allocated: libc::c_ulong,
    pub s1: *mut TCCState,
    pub sh_name: libc::c_int,
    pub sh_num: libc::c_int,
    pub sh_type: libc::c_int,
    pub sh_flags: libc::c_int,
    pub sh_info: libc::c_int,
    pub sh_addralign: libc::c_int,
    pub sh_entsize: libc::c_int,
    pub sh_size: libc::c_ulong,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: libc::c_ulong,
    pub nb_hashed_syms: libc::c_int,
    pub link: *mut Section,
    pub reloc: *mut Section,
    pub hash: *mut Section,
    pub prev: *mut Section,
    pub name: [libc::c_char; 1],
}
pub type Elf64_Addr = uint64_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_version {
    pub lib: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub out_index: libc::c_int,
    pub prev_same_lib: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}
pub type Elf64_Sxword = int64_t;
pub type Elf64_Xword = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_attr {
    pub got_offset: libc::c_uint,
    pub plt_offset: libc::c_uint,
    pub plt_sym: libc::c_int,
    pub dyn_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub symtab_section: *mut Section,
    pub symtab: *mut Section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InlineFunc {
    pub func_str: *mut TokenString,
    pub sym: *mut Sym,
    pub filename: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sym {
    pub v: libc::c_int,
    pub r: libc::c_ushort,
    pub a: SymAttr,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub type_0: CType,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub prev: *mut Sym,
    pub prev_tok: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub next: *mut Sym,
    pub e: *mut libc::c_int,
    pub asm_label: libc::c_int,
    pub cleanupstate: *mut Sym,
    pub vla_array_str: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CType {
    pub t: libc::c_int,
    pub ref_0: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub enum_val: libc::c_longlong,
    pub d: *mut libc::c_int,
    pub cleanup_func: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub c: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub sym_scope: libc::c_int,
    pub jnext: libc::c_int,
    pub jind: libc::c_int,
    pub f: FuncAttr,
    pub auxtype: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct FuncAttr {
    #[bitfield(name = "func_call", ty = "libc::c_uint", bits = "0..=2")]
    #[bitfield(name = "func_type", ty = "libc::c_uint", bits = "3..=4")]
    #[bitfield(name = "func_noreturn", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "func_ctor", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "func_dtor", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "func_args", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "func_alwinl", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "xxxx", ty = "libc::c_uint", bits = "17..=31")]
    pub func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SymAttr {
    #[bitfield(name = "aligned", ty = "libc::c_ushort", bits = "0..=4")]
    #[bitfield(name = "packed", ty = "libc::c_ushort", bits = "5..=5")]
    #[bitfield(name = "weak", ty = "libc::c_ushort", bits = "6..=6")]
    #[bitfield(name = "visibility", ty = "libc::c_ushort", bits = "7..=8")]
    #[bitfield(name = "dllexport", ty = "libc::c_ushort", bits = "9..=9")]
    #[bitfield(name = "nodecorate", ty = "libc::c_ushort", bits = "10..=10")]
    #[bitfield(name = "dllimport", ty = "libc::c_ushort", bits = "11..=11")]
    #[bitfield(name = "addrtaken", ty = "libc::c_ushort", bits = "12..=12")]
    #[bitfield(name = "nodebug", ty = "libc::c_ushort", bits = "13..=13")]
    #[bitfield(name = "xxxx", ty = "libc::c_ushort", bits = "14..=15")]
    pub aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenString {
    pub str_0: *mut libc::c_int,
    pub len: libc::c_int,
    pub need_spc: libc::c_int,
    pub allocated_len: libc::c_int,
    pub last_line_num: libc::c_int,
    pub save_line_num: libc::c_int,
    pub prev: *mut TokenString,
    pub prev_ptr: *const libc::c_int,
    pub alloc: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CachedInclude {
    pub ifndef_macro: libc::c_int,
    pub once: libc::c_int,
    pub hash_next: libc::c_int,
    pub filename: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferedFile {
    pub buf_ptr: *mut uint8_t,
    pub buf_end: *mut uint8_t,
    pub fd: libc::c_int,
    pub prev: *mut BufferedFile,
    pub line_num: libc::c_int,
    pub line_ref: libc::c_int,
    pub ifndef_macro: libc::c_int,
    pub ifndef_macro_saved: libc::c_int,
    pub ifdef_stack_ptr: *mut libc::c_int,
    pub include_next_index: libc::c_int,
    pub prev_tok_flags: libc::c_int,
    pub filename: [libc::c_char; 1024],
    pub true_filename: *mut libc::c_char,
    pub unget: [libc::c_uchar; 4],
    pub buffer: [libc::c_uchar; 1],
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CString {
    pub size: libc::c_int,
    pub size_allocated: libc::c_int,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DLLReference {
    pub level: libc::c_int,
    pub handle: *mut libc::c_void,
    pub found: libc::c_uchar,
    pub index: libc::c_uchar,
    pub name: [libc::c_char; 1],
}
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
pub type Elf64_Section = uint16_t;
pub type uint16_t = __uint16_t;
pub type Elf64_Word = uint32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
pub type Elf64_Off = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_inf {
    pub dynamic: *mut Section,
    pub dynstr: *mut Section,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub phdr: *mut Elf64_Phdr,
    pub phnum: libc::c_int,
    pub shnum: libc::c_int,
    pub interp: *mut Section,
    pub note: *mut Section,
    pub gnu_hash: *mut Section,
    pub _roinf: Section,
    pub roinf: *mut Section,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub data_offset: libc::c_ulong,
    pub rel_addr: Elf64_Addr,
    pub rel_size: Elf64_Addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [libc::c_uchar; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
pub type Elf64_Half = uint16_t;
pub type Elf32_Word = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub first: libc::c_int,
    pub last: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub d_val: Elf64_Xword,
    pub d_ptr: Elf64_Addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Dyn {
    pub d_tag: Elf64_Sxword,
    pub d_un: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stab_Sym {
    pub n_strx: libc::c_uint,
    pub n_type: libc::c_uchar,
    pub n_other: libc::c_uchar,
    pub n_desc: libc::c_ushort,
    pub n_value: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Verneed {
    pub vn_version: Elf64_Half,
    pub vn_cnt: Elf64_Half,
    pub vn_file: Elf64_Word,
    pub vn_aux: Elf64_Word,
    pub vn_next: Elf64_Word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Vernaux {
    pub vna_hash: Elf64_Word,
    pub vna_flags: Elf64_Half,
    pub vna_other: Elf64_Half,
    pub vna_name: Elf64_Word,
    pub vna_next: Elf64_Word,
}
pub const BUILD_GOT_ONLY: gotplt_entry = 1;
pub const AUTO_GOTPLT_ENTRY: gotplt_entry = 2;
pub const NO_GOTPLT_ENTRY: gotplt_entry = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Verdef {
    pub vd_version: Elf64_Half,
    pub vd_flags: Elf64_Half,
    pub vd_ndx: Elf64_Half,
    pub vd_cnt: Elf64_Half,
    pub vd_hash: Elf64_Word,
    pub vd_aux: Elf64_Word,
    pub vd_next: Elf64_Word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Verdaux {
    pub vda_name: Elf64_Word,
    pub vda_next: Elf64_Word,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SectionMergeInfo {
    pub s: *mut Section,
    pub offset: libc::c_ulong,
    pub new_section: uint8_t,
    pub link_once: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArchiveHeader {
    pub ar_name: [libc::c_char; 16],
    pub ar_date: [libc::c_char; 12],
    pub ar_uid: [libc::c_char; 6],
    pub ar_gid: [libc::c_char; 6],
    pub ar_mode: [libc::c_char; 8],
    pub ar_size: [libc::c_char; 10],
    pub ar_fmag: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct versym_info {
    pub nb_versyms: libc::c_int,
    pub verdef: *mut Elf64_Verdef,
    pub verneed: *mut Elf64_Verneed,
    pub versym: *mut Elf64_Half,
    pub nb_local_ver: libc::c_int,
    pub local_ver: *mut libc::c_int,
}
pub type gotplt_entry = libc::c_uint;
pub const ALWAYS_GOTPLT_ENTRY: gotplt_entry = 3;
#[inline]
unsafe extern "C" fn isid(mut c: libc::c_int) -> libc::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
        || c == '_' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn isnum(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '9' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn read16le(mut p: *mut libc::c_uchar) -> uint16_t {
    return (*p.offset(0 as libc::c_int as isize) as libc::c_int
        | (*p.offset(1 as libc::c_int as isize) as uint16_t as libc::c_int)
            << 8 as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn write16le(mut p: *mut libc::c_uchar, mut x: uint16_t) {
    *p
        .offset(
            0 as libc::c_int as isize,
        ) = (x as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
    *p
        .offset(
            1 as libc::c_int as isize,
        ) = (x as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn read32le(mut p: *mut libc::c_uchar) -> uint32_t {
    return read16le(p) as uint32_t
        | (read16le(p.offset(2 as libc::c_int as isize)) as uint32_t)
            << 16 as libc::c_int;
}
#[inline]
unsafe extern "C" fn write32le(mut p: *mut libc::c_uchar, mut x: uint32_t) {
    write16le(p, x as uint16_t);
    write16le(p.offset(2 as libc::c_int as isize), (x >> 16 as libc::c_int) as uint16_t);
}
#[inline]
unsafe extern "C" fn add32le(mut p: *mut libc::c_uchar, mut x: int32_t) {
    write32le(p, (read32le(p)).wrapping_add(x as uint32_t));
}
#[inline]
unsafe extern "C" fn write64le(mut p: *mut libc::c_uchar, mut x: uint64_t) {
    write32le(p, x as uint32_t);
    write32le(p.offset(4 as libc::c_int as isize), (x >> 32 as libc::c_int) as uint32_t);
}
static mut rdata: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b".data.ro\0")
};
#[no_mangle]
pub unsafe extern "C" fn tccelf_new(mut s: *mut TCCState) {
    let mut s1: *mut TCCState = s;
    dynarray_add(
        &mut (*s).sections as *mut *mut *mut Section as *mut libc::c_void,
        &mut (*s).nb_sections,
        0 as *mut libc::c_void,
    );
    (*s1)
        .text_section = new_section(
        s,
        b".text\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
    );
    (*s1)
        .data_section = new_section(
        s,
        b".data\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int,
    );
    (*s1)
        .rodata_section = new_section(
        s,
        rdata.as_ptr(),
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    (*s1)
        .bss_section = new_section(
        s,
        b".bss\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int,
    );
    (*s1)
        .common_section = new_section(
        s,
        b".common\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int,
        0x80000000 as libc::c_uint as libc::c_int,
    );
    (*(*s1).common_section).sh_num = 0xfff2 as libc::c_int;
    (*s1)
        .c2rust_unnamed
        .symtab_section = new_symtab(
        s,
        b".symtab\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        0 as libc::c_int,
        b".strtab\0" as *const u8 as *const libc::c_char,
        b".hashtab\0" as *const u8 as *const libc::c_char,
        0x80000000 as libc::c_uint as libc::c_int,
    );
    (*s)
        .dynsymtab_section = new_symtab(
        s,
        b".dynsymtab\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        (0x80000000 as libc::c_uint | 0x40000000 as libc::c_int as libc::c_uint)
            as libc::c_int,
        b".dynstrtab\0" as *const u8 as *const libc::c_char,
        b".dynhashtab\0" as *const u8 as *const libc::c_char,
        0x80000000 as libc::c_uint as libc::c_int,
    );
    get_sym_attr(s, 0 as libc::c_int, 1 as libc::c_int);
    if (*s).do_debug != 0 {
        tcc_debug_new(s);
    }
    if (*s).output_format != 0 as libc::c_int {
        (*s).unwind_tables = 0 as libc::c_int as libc::c_uchar;
    }
    tcc_eh_frame_start(s);
    if (*s).do_bounds_check != 0 {
        (*s1)
            .bounds_section = new_section(
            s,
            b".bounds\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int,
        );
        (*s1)
            .lbounds_section = new_section(
            s,
            b".lbounds\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_section(mut s: *mut Section) {
    if s.is_null() {
        return;
    }
    tcc_free((*s).data as *mut libc::c_void);
    (*s).data = 0 as *mut libc::c_uchar;
    (*s).data_offset = 0 as libc::c_int as libc::c_ulong;
    (*s).data_allocated = (*s).data_offset;
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_delete(mut s1: *mut TCCState) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s1).nb_sym_versions {
        tcc_free(
            (*((*s1).sym_versions).offset(i as isize)).version as *mut libc::c_void,
        );
        tcc_free((*((*s1).sym_versions).offset(i as isize)).lib as *mut libc::c_void);
        i += 1;
        i;
    }
    tcc_free((*s1).sym_versions as *mut libc::c_void);
    tcc_free((*s1).sym_to_version as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        free_section(*((*s1).sections).offset(i as isize));
        i += 1;
        i;
    }
    dynarray_reset(
        &mut (*s1).sections as *mut *mut *mut Section as *mut libc::c_void,
        &mut (*s1).nb_sections,
    );
    i = 0 as libc::c_int;
    while i < (*s1).nb_priv_sections {
        free_section(*((*s1).priv_sections).offset(i as isize));
        i += 1;
        i;
    }
    dynarray_reset(
        &mut (*s1).priv_sections as *mut *mut *mut Section as *mut libc::c_void,
        &mut (*s1).nb_priv_sections,
    );
    tcc_free((*s1).sym_attrs as *mut libc::c_void);
    (*s1).c2rust_unnamed.symtab_section = 0 as *mut Section;
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_begin_file(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        (*s).sh_offset = (*s).data_offset;
        i += 1;
        i;
    }
    s = (*s1).c2rust_unnamed.symtab;
    (*s).reloc = (*s).hash;
    (*s).hash = 0 as *mut Section;
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_end_file(mut s1: *mut TCCState) {
    let mut s: *mut Section = (*s1).c2rust_unnamed.symtab;
    let mut first_sym: libc::c_int = 0;
    let mut nb_syms: libc::c_int = 0;
    let mut tr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    first_sym = ((*s).sh_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    nb_syms = ((*s).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        .wrapping_sub(first_sym as libc::c_ulong) as libc::c_int;
    (*s).data_offset = (*s).sh_offset;
    (*(*s).link).data_offset = (*(*s).link).sh_offset;
    (*s).hash = (*s).reloc;
    (*s).reloc = 0 as *mut Section;
    tr = tcc_mallocz(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < nb_syms {
        let mut sym: *mut Elf64_Sym = ((*s).data as *mut Elf64_Sym)
            .offset(first_sym as isize)
            .offset(i as isize);
        if (*sym).st_shndx as libc::c_int == 0 as libc::c_int {
            let mut sym_bind: libc::c_int = (*sym).st_info as libc::c_int
                >> 4 as libc::c_int;
            let mut sym_type: libc::c_int = (*sym).st_info as libc::c_int
                & 0xf as libc::c_int;
            if sym_bind == 0 as libc::c_int {
                sym_bind = 1 as libc::c_int;
            }
            if sym_bind == 1 as libc::c_int && (*s1).output_type == 3 as libc::c_int {
                sym_type = 0 as libc::c_int;
            }
            (*sym)
                .st_info = ((sym_bind << 4 as libc::c_int)
                + (sym_type & 0xf as libc::c_int)) as libc::c_uchar;
        }
        *tr
            .offset(
                i as isize,
            ) = set_elf_sym(
            s,
            (*sym).st_value,
            (*sym).st_size,
            (*sym).st_info as libc::c_int,
            (*sym).st_other as libc::c_int,
            (*sym).st_shndx as libc::c_int,
            ((*(*s).link).data as *mut libc::c_char).offset((*sym).st_name as isize),
        );
        i += 1;
        i;
    }
    update_relocs(s1, s, tr, first_sym);
    tcc_free(tr as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        s = *((*s1).sections).offset((i + 1 as libc::c_int) as isize);
        (*s1)
            .total_output[i
            as usize] = ((*s1).total_output[i as usize] as libc::c_ulong)
            .wrapping_add(((*s).data_offset).wrapping_sub((*s).sh_offset))
            as libc::c_uint as libc::c_uint;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn new_section(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
    mut sh_type: libc::c_int,
    mut sh_flags: libc::c_int,
) -> *mut Section {
    let mut sec: *mut Section = 0 as *mut Section;
    sec = tcc_mallocz(
        (::core::mem::size_of::<Section>() as libc::c_ulong).wrapping_add(strlen(name)),
    ) as *mut Section;
    (*sec).s1 = s1;
    strcpy(((*sec).name).as_mut_ptr(), name);
    (*sec).sh_type = sh_type;
    (*sec).sh_flags = sh_flags;
    match sh_type {
        1879048191 => {
            (*sec).sh_addralign = 2 as libc::c_int;
        }
        5 | 1879048182 | 9 | 4 | 11 | 2 | 6 | 1879048190 | 1879048189 => {
            (*sec).sh_addralign = 8 as libc::c_int;
        }
        3 => {
            (*sec).sh_addralign = 1 as libc::c_int;
        }
        _ => {
            (*sec).sh_addralign = 8 as libc::c_int;
        }
    }
    if sh_flags as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
        dynarray_add(
            &mut (*s1).priv_sections as *mut *mut *mut Section as *mut libc::c_void,
            &mut (*s1).nb_priv_sections,
            sec as *mut libc::c_void,
        );
    } else {
        (*sec).sh_num = (*s1).nb_sections;
        dynarray_add(
            &mut (*s1).sections as *mut *mut *mut Section as *mut libc::c_void,
            &mut (*s1).nb_sections,
            sec as *mut libc::c_void,
        );
    }
    return sec;
}
#[no_mangle]
pub unsafe extern "C" fn init_symtab(mut s: *mut Section) {
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nb_buckets: libc::c_int = 1 as libc::c_int;
    put_elf_str((*s).link, b"\0" as *const u8 as *const libc::c_char);
    section_ptr_add(s, ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong);
    ptr = section_ptr_add(
        (*s).hash,
        ((2 as libc::c_int + nb_buckets + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *ptr.offset(0 as libc::c_int as isize) = nb_buckets;
    *ptr.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    memset(
        ptr.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        ((nb_buckets + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn new_symtab(
    mut s1: *mut TCCState,
    mut symtab_name: *const libc::c_char,
    mut sh_type: libc::c_int,
    mut sh_flags: libc::c_int,
    mut strtab_name: *const libc::c_char,
    mut hash_name: *const libc::c_char,
    mut hash_sh_flags: libc::c_int,
) -> *mut Section {
    let mut symtab: *mut Section = 0 as *mut Section;
    let mut strtab: *mut Section = 0 as *mut Section;
    let mut hash: *mut Section = 0 as *mut Section;
    symtab = new_section(s1, symtab_name, sh_type, sh_flags);
    (*symtab)
        .sh_entsize = ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong
        as libc::c_int;
    strtab = new_section(s1, strtab_name, 3 as libc::c_int, sh_flags);
    (*symtab).link = strtab;
    hash = new_section(s1, hash_name, 5 as libc::c_int, hash_sh_flags);
    (*hash)
        .sh_entsize = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as libc::c_int;
    (*symtab).hash = hash;
    (*hash).link = symtab;
    init_symtab(symtab);
    return symtab;
}
#[no_mangle]
pub unsafe extern "C" fn section_realloc(
    mut sec: *mut Section,
    mut new_size: libc::c_ulong,
) {
    let mut size: libc::c_ulong = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    size = (*sec).data_allocated;
    if size == 0 as libc::c_int as libc::c_ulong {
        size = 1 as libc::c_int as libc::c_ulong;
    }
    while size < new_size {
        size = size.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    }
    data = tcc_realloc((*sec).data as *mut libc::c_void, size) as *mut libc::c_uchar;
    memset(
        data.offset((*sec).data_allocated as isize) as *mut libc::c_void,
        0 as libc::c_int,
        size.wrapping_sub((*sec).data_allocated),
    );
    (*sec).data = data;
    (*sec).data_allocated = size;
}
#[no_mangle]
pub unsafe extern "C" fn section_add(
    mut sec: *mut Section,
    mut size: Elf64_Addr,
    mut align: libc::c_int,
) -> size_t {
    let mut offset: size_t = 0;
    let mut offset1: size_t = 0;
    offset = ((*sec).data_offset)
        .wrapping_add(align as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) & -align as libc::c_ulong;
    offset1 = offset.wrapping_add(size);
    if (*sec).sh_type != 8 as libc::c_int && offset1 > (*sec).data_allocated {
        section_realloc(sec, offset1);
    }
    (*sec).data_offset = offset1;
    if align > (*sec).sh_addralign {
        (*sec).sh_addralign = align;
    }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn section_ptr_add(
    mut sec: *mut Section,
    mut size: Elf64_Addr,
) -> *mut libc::c_void {
    let mut offset: size_t = section_add(sec, size, 1 as libc::c_int);
    return ((*sec).data).offset(offset as isize) as *mut libc::c_void;
}
unsafe extern "C" fn section_reserve(mut sec: *mut Section, mut size: libc::c_ulong) {
    if size > (*sec).data_allocated {
        section_realloc(sec, size);
    }
    if size > (*sec).data_offset {
        (*sec).data_offset = size;
    }
}
unsafe extern "C" fn have_section(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
) -> *mut Section {
    let mut sec: *mut Section = 0 as *mut Section;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        sec = *((*s1).sections).offset(i as isize);
        if strcmp(name, ((*sec).name).as_mut_ptr()) == 0 {
            return sec;
        }
        i += 1;
        i;
    }
    return 0 as *mut Section;
}
#[no_mangle]
pub unsafe extern "C" fn find_section(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
) -> *mut Section {
    let mut sec: *mut Section = have_section(s1, name);
    if !sec.is_null() {
        return sec;
    }
    return new_section(
        s1,
        name,
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn put_elf_str(
    mut s: *mut Section,
    mut sym: *const libc::c_char,
) -> libc::c_int {
    let mut offset: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (strlen(sym)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    offset = (*s).data_offset as libc::c_int;
    ptr = section_ptr_add(s, len as Elf64_Addr) as *mut libc::c_char;
    memmove(ptr as *mut libc::c_void, sym as *const libc::c_void, len as libc::c_ulong);
    return offset;
}
unsafe extern "C" fn elf_hash(mut name: *const libc::c_uchar) -> Elf64_Word {
    let mut h: Elf64_Word = 0 as libc::c_int as Elf64_Word;
    let mut g: Elf64_Word = 0;
    while *name != 0 {
        let fresh0 = name;
        name = name.offset(1);
        h = (h << 4 as libc::c_int).wrapping_add(*fresh0 as Elf64_Word);
        g = h & 0xf0000000 as libc::c_uint;
        if g != 0 {
            h ^= g >> 24 as libc::c_int;
        }
        h &= !g;
    }
    return h;
}
unsafe extern "C" fn rebuild_hash(mut s: *mut Section, mut nb_buckets: libc::c_uint) {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut hash: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nb_syms: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut strtab: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    strtab = (*(*s).link).data;
    nb_syms = ((*s).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    if nb_buckets == 0 {
        nb_buckets = *((*(*s).hash).data as *mut libc::c_int)
            .offset(0 as libc::c_int as isize) as libc::c_uint;
    }
    (*(*s).hash).data_offset = 0 as libc::c_int as libc::c_ulong;
    ptr = section_ptr_add(
        (*s).hash,
        ((2 as libc::c_int as libc::c_uint)
            .wrapping_add(nb_buckets)
            .wrapping_add(nb_syms as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *ptr.offset(0 as libc::c_int as isize) = nb_buckets as libc::c_int;
    *ptr.offset(1 as libc::c_int as isize) = nb_syms;
    ptr = ptr.offset(2 as libc::c_int as isize);
    hash = ptr;
    memset(
        hash as *mut libc::c_void,
        0 as libc::c_int,
        (nb_buckets.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    ptr = ptr.offset(nb_buckets.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
    sym = ((*s).data as *mut Elf64_Sym).offset(1 as libc::c_int as isize);
    sym_index = 1 as libc::c_int;
    while sym_index < nb_syms {
        if (*sym).st_info as libc::c_int >> 4 as libc::c_int != 0 as libc::c_int {
            h = (elf_hash(strtab.offset((*sym).st_name as isize)))
                .wrapping_rem(nb_buckets) as libc::c_int;
            *ptr = *hash.offset(h as isize);
            *hash.offset(h as isize) = sym_index;
        } else {
            *ptr = 0 as libc::c_int;
        }
        ptr = ptr.offset(1);
        ptr;
        sym = sym.offset(1);
        sym;
        sym_index += 1;
        sym_index;
    }
}
#[no_mangle]
pub unsafe extern "C" fn put_elf_sym(
    mut s: *mut Section,
    mut value: Elf64_Addr,
    mut size: libc::c_ulong,
    mut info: libc::c_int,
    mut other: libc::c_int,
    mut shndx: libc::c_int,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut name_offset: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut nbuckets: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut hs: *mut Section = 0 as *mut Section;
    sym = section_ptr_add(s, ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as *mut Elf64_Sym;
    if !name.is_null() && *name.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        name_offset = put_elf_str((*s).link, name);
    } else {
        name_offset = 0 as libc::c_int;
    }
    (*sym).st_name = name_offset as Elf64_Word;
    (*sym).st_value = value;
    (*sym).st_size = size;
    (*sym).st_info = info as libc::c_uchar;
    (*sym).st_other = other as libc::c_uchar;
    (*sym).st_shndx = shndx as Elf64_Section;
    sym_index = sym.offset_from((*s).data as *mut Elf64_Sym) as libc::c_long
        as libc::c_int;
    hs = (*s).hash;
    if !hs.is_null() {
        let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut base: *mut libc::c_int = 0 as *mut libc::c_int;
        ptr = section_ptr_add(hs, ::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as *mut libc::c_int;
        base = (*hs).data as *mut libc::c_int;
        if info as libc::c_uchar as libc::c_int >> 4 as libc::c_int != 0 as libc::c_int {
            nbuckets = *base.offset(0 as libc::c_int as isize);
            h = (elf_hash(((*(*s).link).data).offset(name_offset as isize))
                % nbuckets as Elf64_Word) as libc::c_int;
            *ptr = *base.offset((2 as libc::c_int + h) as isize);
            *base.offset((2 as libc::c_int + h) as isize) = sym_index;
            let ref mut fresh1 = *base.offset(1 as libc::c_int as isize);
            *fresh1 += 1;
            *fresh1;
            (*hs).nb_hashed_syms += 1;
            (*hs).nb_hashed_syms;
            if (*hs).nb_hashed_syms > 2 as libc::c_int * nbuckets {
                rebuild_hash(s, (2 as libc::c_int * nbuckets) as libc::c_uint);
            }
        } else {
            *ptr = 0 as libc::c_int;
            let ref mut fresh2 = *base.offset(1 as libc::c_int as isize);
            *fresh2 += 1;
            *fresh2;
        }
    }
    return sym_index;
}
#[no_mangle]
pub unsafe extern "C" fn find_elf_sym(
    mut s: *mut Section,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut hs: *mut Section = 0 as *mut Section;
    let mut nbuckets: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut name1: *const libc::c_char = 0 as *const libc::c_char;
    hs = (*s).hash;
    if hs.is_null() {
        return 0 as libc::c_int;
    }
    nbuckets = *((*hs).data as *mut libc::c_int).offset(0 as libc::c_int as isize);
    h = (elf_hash(name as *mut libc::c_uchar) % nbuckets as Elf64_Word) as libc::c_int;
    sym_index = *((*hs).data as *mut libc::c_int)
        .offset((2 as libc::c_int + h) as isize);
    while sym_index != 0 as libc::c_int {
        sym = &mut *((*s).data as *mut Elf64_Sym).offset(sym_index as isize)
            as *mut Elf64_Sym;
        name1 = ((*(*s).link).data as *mut libc::c_char).offset((*sym).st_name as isize);
        if strcmp(name, name1) == 0 {
            return sym_index;
        }
        sym_index = *((*hs).data as *mut libc::c_int)
            .offset((2 as libc::c_int + nbuckets + sym_index) as isize);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_sym_addr(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
    mut err: libc::c_int,
    mut forc: libc::c_int,
) -> Elf64_Addr {
    let mut sym_index: libc::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut buf: [libc::c_char; 256] = [0; 256];
    if forc != 0 && (*s1).leading_underscore as libc::c_int != 0 {
        buf[0 as libc::c_int as usize] = '_' as i32 as libc::c_char;
        pstrcpy(
            buf.as_mut_ptr().offset(1 as libc::c_int as isize),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            name,
        );
        name = buf.as_mut_ptr();
    }
    sym_index = find_elf_sym((*s1).c2rust_unnamed.symtab, name);
    sym = &mut *((*(*s1).c2rust_unnamed.symtab).data as *mut Elf64_Sym)
        .offset(sym_index as isize) as *mut Elf64_Sym;
    if sym_index == 0 || (*sym).st_shndx as libc::c_int == 0 as libc::c_int {
        if err != 0 {
            tcc_enter_state(s1);
            (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(b"%s not defined\0" as *const u8 as *const libc::c_char, name);
        }
        return -(1 as libc::c_int) as Elf64_Addr;
    }
    return (*sym).st_value;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_get_symbol(
    mut s: *mut TCCState,
    mut name: *const libc::c_char,
) -> *mut libc::c_void {
    let mut addr: Elf64_Addr = get_sym_addr(s, name, 0 as libc::c_int, 1 as libc::c_int);
    return if addr == -(1 as libc::c_int) as Elf64_Addr {
        0 as *mut libc::c_void
    } else {
        addr as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_symbol(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
    mut val: *const libc::c_void,
) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    if (*s1).leading_underscore != 0 {
        buf[0 as libc::c_int as usize] = '_' as i32 as libc::c_char;
        pstrcpy(
            buf.as_mut_ptr().offset(1 as libc::c_int as isize),
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            name,
        );
        name = buf.as_mut_ptr();
    }
    set_global_sym(s1, name, 0 as *mut Section, val as uintptr_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn list_elf_symbols(
    mut s: *mut TCCState,
    mut ctx: *mut libc::c_void,
    mut symbol_cb: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_void,
        ) -> (),
    >,
) {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut symtab: *mut Section = 0 as *mut Section;
    let mut sym_index: libc::c_int = 0;
    let mut end_sym: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut sym_vis: libc::c_uchar = 0;
    let mut sym_bind: libc::c_uchar = 0;
    symtab = (*s).c2rust_unnamed.symtab;
    end_sym = ((*symtab).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    sym_index = 0 as libc::c_int;
    while sym_index < end_sym {
        sym = &mut *((*symtab).data as *mut Elf64_Sym).offset(sym_index as isize)
            as *mut Elf64_Sym;
        if (*sym).st_value != 0 {
            name = ((*(*symtab).link).data as *mut libc::c_char)
                .offset((*sym).st_name as isize);
            sym_bind = ((*sym).st_info as libc::c_int >> 4 as libc::c_int)
                as libc::c_uchar;
            sym_vis = ((*sym).st_other as libc::c_int & 0x3 as libc::c_int)
                as libc::c_uchar;
            if sym_bind as libc::c_int == 1 as libc::c_int
                && sym_vis as libc::c_int == 0 as libc::c_int
            {
                symbol_cb
                    .expect(
                        "non-null function pointer",
                    )(ctx, name, (*sym).st_value as *mut libc::c_void);
            }
        }
        sym_index += 1;
        sym_index;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_list_symbols(
    mut s: *mut TCCState,
    mut ctx: *mut libc::c_void,
    mut symbol_cb: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_void,
        ) -> (),
    >,
) {
    list_elf_symbols(s, ctx, symbol_cb);
}
unsafe extern "C" fn version_add(mut s1: *mut TCCState) {
    let mut i: libc::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut vn: *mut Elf64_Verneed = 0 as *mut Elf64_Verneed;
    let mut symtab: *mut Section = 0 as *mut Section;
    let mut sym_index: libc::c_int = 0;
    let mut end_sym: libc::c_int = 0;
    let mut nb_versions: libc::c_int = 2 as libc::c_int;
    let mut nb_entries: libc::c_int = 0 as libc::c_int;
    let mut versym: *mut Elf64_Half = 0 as *mut Elf64_Half;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if 0 as libc::c_int == (*s1).nb_sym_versions {
        return;
    }
    (*s1)
        .versym_section = new_section(
        s1,
        b".gnu.version\0" as *const u8 as *const libc::c_char,
        0x6fffffff as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    (*(*s1).versym_section)
        .sh_entsize = ::core::mem::size_of::<Elf64_Half>() as libc::c_ulong
        as libc::c_int;
    (*(*s1).versym_section).link = (*s1).dynsym;
    symtab = (*s1).dynsym;
    end_sym = ((*symtab).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    versym = section_ptr_add(
        (*s1).versym_section,
        (end_sym as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Half>() as libc::c_ulong),
    ) as *mut Elf64_Half;
    sym_index = 1 as libc::c_int;
    while sym_index < end_sym {
        let mut dllindex: libc::c_int = 0;
        let mut verndx: libc::c_int = 0;
        sym = &mut *((*symtab).data as *mut Elf64_Sym).offset(sym_index as isize)
            as *mut Elf64_Sym;
        name = ((*(*symtab).link).data as *mut libc::c_char)
            .offset((*sym).st_name as isize);
        dllindex = find_elf_sym((*s1).dynsymtab_section, name);
        verndx = if dllindex != 0 && dllindex < (*s1).nb_sym_to_version {
            *((*s1).sym_to_version).offset(dllindex as isize)
        } else {
            -(1 as libc::c_int)
        };
        if verndx >= 0 as libc::c_int
            && ((*sym).st_shndx as libc::c_int == 0 as libc::c_int
                || (*s1).output_type & 2 as libc::c_int != 0)
        {
            if (*((*s1).sym_versions).offset(verndx as isize)).out_index == 0 {
                let fresh3 = nb_versions;
                nb_versions = nb_versions + 1;
                (*((*s1).sym_versions).offset(verndx as isize)).out_index = fresh3;
            }
            *versym
                .offset(
                    sym_index as isize,
                ) = (*((*s1).sym_versions).offset(verndx as isize)).out_index
                as Elf64_Half;
        } else {
            *versym.offset(sym_index as isize) = 1 as libc::c_int as Elf64_Half;
        }
        sym_index += 1;
        sym_index;
    }
    if nb_versions > 2 as libc::c_int {
        (*s1)
            .verneed_section = new_section(
            s1,
            b".gnu.version_r\0" as *const u8 as *const libc::c_char,
            0x6ffffffe as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int,
        );
        (*(*s1).verneed_section).link = (*(*s1).dynsym).link;
        i = (*s1).nb_sym_versions;
        loop {
            let fresh4 = i;
            i = i - 1;
            if !(fresh4 > 0 as libc::c_int) {
                break;
            }
            let mut sv: *mut sym_version = &mut *((*s1).sym_versions).offset(i as isize)
                as *mut sym_version;
            let mut n_same_libs: libc::c_int = 0 as libc::c_int;
            let mut prev: libc::c_int = 0;
            let mut vnofs: size_t = 0;
            let mut vna: *mut Elf64_Vernaux = 0 as *mut Elf64_Vernaux;
            if (*sv).out_index < 1 as libc::c_int {
                continue;
            }
            if strcmp((*sv).lib, b"ld-linux.so.2\0" as *const u8 as *const libc::c_char)
                != 0
            {
                tcc_add_dllref(s1, (*sv).lib, 0 as libc::c_int);
            }
            vnofs = section_add(
                (*s1).verneed_section,
                ::core::mem::size_of::<Elf64_Verneed>() as libc::c_ulong,
                1 as libc::c_int,
            );
            vn = ((*(*s1).verneed_section).data).offset(vnofs as isize)
                as *mut Elf64_Verneed;
            (*vn).vn_version = 1 as libc::c_int as Elf64_Half;
            (*vn)
                .vn_file = put_elf_str((*(*s1).verneed_section).link, (*sv).lib)
                as Elf64_Word;
            (*vn)
                .vn_aux = ::core::mem::size_of::<Elf64_Verneed>() as libc::c_ulong
                as Elf64_Word;
            loop {
                prev = (*sv).prev_same_lib;
                if (*sv).out_index > 0 as libc::c_int {
                    vna = section_ptr_add(
                        (*s1).verneed_section,
                        ::core::mem::size_of::<Elf64_Vernaux>() as libc::c_ulong,
                    ) as *mut Elf64_Vernaux;
                    (*vna).vna_hash = elf_hash((*sv).version as *const libc::c_uchar);
                    (*vna).vna_flags = 0 as libc::c_int as Elf64_Half;
                    (*vna).vna_other = (*sv).out_index as Elf64_Half;
                    (*sv).out_index = -(2 as libc::c_int);
                    (*vna)
                        .vna_name = put_elf_str(
                        (*(*s1).verneed_section).link,
                        (*sv).version,
                    ) as Elf64_Word;
                    (*vna)
                        .vna_next = ::core::mem::size_of::<Elf64_Vernaux>()
                        as libc::c_ulong as Elf64_Word;
                    n_same_libs += 1;
                    n_same_libs;
                }
                if prev >= 0 as libc::c_int {
                    sv = &mut *((*s1).sym_versions).offset(prev as isize)
                        as *mut sym_version;
                }
                if !(prev >= 0 as libc::c_int) {
                    break;
                }
            }
            (*vna).vna_next = 0 as libc::c_int as Elf64_Word;
            vn = ((*(*s1).verneed_section).data).offset(vnofs as isize)
                as *mut Elf64_Verneed;
            (*vn).vn_cnt = n_same_libs as Elf64_Half;
            (*vn)
                .vn_next = (::core::mem::size_of::<Elf64_Verneed>() as libc::c_ulong)
                .wrapping_add(
                    (n_same_libs as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<Elf64_Vernaux>() as libc::c_ulong,
                        ),
                ) as Elf64_Word;
            nb_entries += 1;
            nb_entries;
        }
        if !vn.is_null() {
            (*vn).vn_next = 0 as libc::c_int as Elf64_Word;
        }
        (*(*s1).verneed_section).sh_info = nb_entries;
    }
    (*s1).dt_verneednum = nb_entries;
}
#[no_mangle]
pub unsafe extern "C" fn set_elf_sym(
    mut s: *mut Section,
    mut value: Elf64_Addr,
    mut size: libc::c_ulong,
    mut info: libc::c_int,
    mut other: libc::c_int,
    mut shndx: libc::c_int,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s1: *mut TCCState = (*s).s1;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_bind: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut sym_type: libc::c_int = 0;
    let mut esym_bind: libc::c_int = 0;
    let mut sym_vis: libc::c_uchar = 0;
    let mut esym_vis: libc::c_uchar = 0;
    let mut new_vis: libc::c_uchar = 0;
    sym_bind = info as libc::c_uchar as libc::c_int >> 4 as libc::c_int;
    sym_type = info & 0xf as libc::c_int;
    sym_vis = (other & 0x3 as libc::c_int) as libc::c_uchar;
    if sym_bind != 0 as libc::c_int {
        sym_index = find_elf_sym(s, name);
        if sym_index == 0 {
            current_block = 2304011700013756175;
        } else {
            esym = &mut *((*s).data as *mut Elf64_Sym).offset(sym_index as isize)
                as *mut Elf64_Sym;
            if (*esym).st_value == value && (*esym).st_size == size
                && (*esym).st_info as libc::c_int == info
                && (*esym).st_other as libc::c_int == other
                && (*esym).st_shndx as libc::c_int == shndx
            {
                return sym_index;
            }
            if (*esym).st_shndx as libc::c_int != 0 as libc::c_int {
                esym_bind = (*esym).st_info as libc::c_int >> 4 as libc::c_int;
                esym_vis = ((*esym).st_other as libc::c_int & 0x3 as libc::c_int)
                    as libc::c_uchar;
                if esym_vis as libc::c_int == 0 as libc::c_int {
                    new_vis = sym_vis;
                } else if sym_vis as libc::c_int == 0 as libc::c_int {
                    new_vis = esym_vis;
                } else {
                    new_vis = (if (esym_vis as libc::c_int) < sym_vis as libc::c_int {
                        esym_vis as libc::c_int
                    } else {
                        sym_vis as libc::c_int
                    }) as libc::c_uchar;
                }
                (*esym)
                    .st_other = ((*esym).st_other as libc::c_int
                    & !(-(1 as libc::c_int) & 0x3 as libc::c_int)
                    | new_vis as libc::c_int) as libc::c_uchar;
                if shndx == 0 as libc::c_int {
                    current_block = 10891380440665537214;
                } else if sym_bind == 1 as libc::c_int && esym_bind == 2 as libc::c_int {
                    current_block = 13211290129031043897;
                } else if sym_bind == 2 as libc::c_int && esym_bind == 1 as libc::c_int {
                    current_block = 10891380440665537214;
                } else if sym_bind == 2 as libc::c_int && esym_bind == 2 as libc::c_int {
                    current_block = 10891380440665537214;
                } else if sym_vis as libc::c_int == 2 as libc::c_int
                    || sym_vis as libc::c_int == 1 as libc::c_int
                {
                    current_block = 10891380440665537214;
                } else if ((*esym).st_shndx as libc::c_int == 0xfff2 as libc::c_int
                    || (*esym).st_shndx as libc::c_int == (*(*s1).bss_section).sh_num)
                    && (shndx < 0xff00 as libc::c_int
                        && shndx != (*(*s1).bss_section).sh_num)
                {
                    current_block = 13211290129031043897;
                } else if shndx == 0xfff2 as libc::c_int
                    || shndx == (*(*s1).bss_section).sh_num
                {
                    current_block = 10891380440665537214;
                } else if (*s).sh_flags & 0x40000000 as libc::c_int != 0 {
                    current_block = 10891380440665537214;
                } else if (*esym).st_other as libc::c_int & 0x4 as libc::c_int != 0 {
                    current_block = 13211290129031043897;
                } else {
                    tcc_enter_state(s1);
                    (Some(
                        _tcc_error_noabort
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"'%s' defined twice\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                    current_block = 10891380440665537214;
                }
            } else {
                (*esym).st_other = other as libc::c_uchar;
                current_block = 13211290129031043897;
            }
            match current_block {
                10891380440665537214 => {}
                _ => {
                    (*esym)
                        .st_info = ((sym_bind << 4 as libc::c_int)
                        + (sym_type & 0xf as libc::c_int)) as libc::c_uchar;
                    (*esym).st_shndx = shndx as Elf64_Section;
                    (*esym).st_value = value;
                    (*esym).st_size = size;
                    current_block = 10891380440665537214;
                }
            }
        }
    } else {
        current_block = 2304011700013756175;
    }
    match current_block {
        2304011700013756175 => {
            sym_index = put_elf_sym(
                s,
                value,
                size,
                (sym_bind << 4 as libc::c_int) + (sym_type & 0xf as libc::c_int),
                other,
                shndx,
                name,
            );
        }
        _ => {}
    }
    return sym_index;
}
#[no_mangle]
pub unsafe extern "C" fn put_elf_reloca(
    mut symtab: *mut Section,
    mut s: *mut Section,
    mut offset: libc::c_ulong,
    mut type_0: libc::c_int,
    mut symbol: libc::c_int,
    mut addend: Elf64_Addr,
) {
    let mut s1: *mut TCCState = (*s).s1;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut sr: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    sr = (*s).reloc;
    if sr.is_null() {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b".rela%s\0" as *const u8 as *const libc::c_char,
            ((*s).name).as_mut_ptr(),
        );
        sr = new_section(
            (*s).s1,
            buf.as_mut_ptr(),
            4 as libc::c_int,
            (*symtab).sh_flags,
        );
        (*sr)
            .sh_entsize = ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong
            as libc::c_int;
        (*sr).link = symtab;
        (*sr).sh_info = (*s).sh_num;
        (*s).reloc = sr;
    }
    rel = section_ptr_add(sr, ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong)
        as *mut Elf64_Rela;
    (*rel).r_offset = offset;
    (*rel)
        .r_info = ((symbol as Elf64_Xword) << 32 as libc::c_int)
        .wrapping_add(type_0 as Elf64_Xword);
    (*rel).r_addend = addend as Elf64_Sxword;
    if 4 as libc::c_int != 4 as libc::c_int && addend != 0 {
        tcc_enter_state(s1);
        (Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
        ))
            .expect(
                "non-null function pointer",
            )(
            b"non-zero addend on REL architecture\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn put_elf_reloc(
    mut symtab: *mut Section,
    mut s: *mut Section,
    mut offset: libc::c_ulong,
    mut type_0: libc::c_int,
    mut symbol: libc::c_int,
) {
    put_elf_reloca(symtab, s, offset, type_0, symbol, 0 as libc::c_int as Elf64_Addr);
}
#[no_mangle]
pub unsafe extern "C" fn get_sym_attr(
    mut s1: *mut TCCState,
    mut index: libc::c_int,
    mut alloc: libc::c_int,
) -> *mut sym_attr {
    let mut n: libc::c_int = 0;
    let mut tab: *mut sym_attr = 0 as *mut sym_attr;
    if index >= (*s1).nb_sym_attrs {
        if alloc == 0 {
            return (*s1).sym_attrs;
        }
        n = 1 as libc::c_int;
        while index >= n {
            n *= 2 as libc::c_int;
        }
        tab = tcc_realloc(
            (*s1).sym_attrs as *mut libc::c_void,
            (n as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<sym_attr>() as libc::c_ulong),
        ) as *mut sym_attr;
        (*s1).sym_attrs = tab;
        memset(
            ((*s1).sym_attrs).offset((*s1).nb_sym_attrs as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((n - (*s1).nb_sym_attrs) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<sym_attr>() as libc::c_ulong),
        );
        (*s1).nb_sym_attrs = n;
    }
    return &mut *((*s1).sym_attrs).offset(index as isize) as *mut sym_attr;
}
unsafe extern "C" fn update_relocs(
    mut s1: *mut TCCState,
    mut s: *mut Section,
    mut old_to_new_syms: *mut libc::c_int,
    mut first_sym: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut sr: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        sr = *((*s1).sections).offset(i as isize);
        if (*sr).sh_type == 4 as libc::c_int && (*sr).link == s {
            rel = ((*sr).data as *mut Elf64_Rela).offset(0 as libc::c_int as isize);
            while rel
                < ((*sr).data).offset((*sr).data_offset as isize) as *mut Elf64_Rela
            {
                sym_index = ((*rel).r_info >> 32 as libc::c_int) as libc::c_int;
                type_0 = ((*rel).r_info & 0xffffffff as libc::c_uint as Elf64_Xword)
                    as libc::c_int;
                sym_index -= first_sym;
                if !(sym_index < 0 as libc::c_int) {
                    sym_index = *old_to_new_syms.offset(sym_index as isize);
                    (*rel)
                        .r_info = ((sym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(type_0 as Elf64_Xword);
                }
                rel = rel.offset(1);
                rel;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn sort_syms(mut s1: *mut TCCState, mut s: *mut Section) {
    let mut old_to_new_syms: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut new_syms: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut nb_syms: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut q: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    nb_syms = ((*s).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    new_syms = tcc_malloc(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong),
    ) as *mut Elf64_Sym;
    old_to_new_syms = tcc_malloc(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    p = (*s).data as *mut Elf64_Sym;
    q = new_syms;
    i = 0 as libc::c_int;
    while i < nb_syms {
        if (*p).st_info as libc::c_int >> 4 as libc::c_int == 0 as libc::c_int {
            *old_to_new_syms
                .offset(
                    i as isize,
                ) = q.offset_from(new_syms) as libc::c_long as libc::c_int;
            let fresh5 = q;
            q = q.offset(1);
            *fresh5 = *p;
        }
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    if (*s).sh_size != 0 {
        (*s).sh_info = q.offset_from(new_syms) as libc::c_long as libc::c_int;
    }
    p = (*s).data as *mut Elf64_Sym;
    i = 0 as libc::c_int;
    while i < nb_syms {
        if (*p).st_info as libc::c_int >> 4 as libc::c_int != 0 as libc::c_int {
            *old_to_new_syms
                .offset(
                    i as isize,
                ) = q.offset_from(new_syms) as libc::c_long as libc::c_int;
            let fresh6 = q;
            q = q.offset(1);
            *fresh6 = *p;
        }
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    memcpy(
        (*s).data as *mut libc::c_void,
        new_syms as *const libc::c_void,
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong),
    );
    tcc_free(new_syms as *mut libc::c_void);
    update_relocs(s1, s, old_to_new_syms, 0 as libc::c_int);
    tcc_free(old_to_new_syms as *mut libc::c_void);
}
unsafe extern "C" fn create_gnu_hash(mut s1: *mut TCCState) -> *mut Section {
    let mut nb_syms: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ndef: libc::c_int = 0;
    let mut nbuckets: libc::c_int = 0;
    let mut symoffset: libc::c_int = 0;
    let mut bloom_size: libc::c_int = 0;
    let mut bloom_shift: libc::c_int = 0;
    let mut p: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut gnu_hash: *mut Section = 0 as *mut Section;
    let mut dynsym: *mut Section = (*s1).dynsym;
    let mut ptr: *mut Elf32_Word = 0 as *mut Elf32_Word;
    gnu_hash = new_section(
        s1,
        b".gnu.hash\0" as *const u8 as *const libc::c_char,
        0x6ffffff6 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    (*gnu_hash).link = (*(*dynsym).hash).link;
    nb_syms = ((*dynsym).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    ndef = 0 as libc::c_int;
    p = (*dynsym).data as *mut Elf64_Sym;
    i = 0 as libc::c_int;
    while i < nb_syms {
        ndef += ((*p).st_shndx as libc::c_int != 0 as libc::c_int) as libc::c_int;
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    nbuckets = ndef / 4 as libc::c_int + 1 as libc::c_int;
    symoffset = nb_syms - ndef;
    bloom_shift = if 8 as libc::c_int == 8 as libc::c_int {
        6 as libc::c_int
    } else {
        5 as libc::c_int
    };
    bloom_size = 1 as libc::c_int;
    while ndef >= bloom_size * ((1 as libc::c_int) << bloom_shift - 3 as libc::c_int) {
        bloom_size *= 2 as libc::c_int;
    }
    ptr = section_ptr_add(
        gnu_hash,
        (4 as libc::c_int * 4 as libc::c_int + 8 as libc::c_int * bloom_size
            + nbuckets * 4 as libc::c_int + ndef * 4 as libc::c_int) as Elf64_Addr,
    ) as *mut Elf32_Word;
    *ptr.offset(0 as libc::c_int as isize) = nbuckets as Elf32_Word;
    *ptr.offset(1 as libc::c_int as isize) = symoffset as Elf32_Word;
    *ptr.offset(2 as libc::c_int as isize) = bloom_size as Elf32_Word;
    *ptr.offset(3 as libc::c_int as isize) = bloom_shift as Elf32_Word;
    return gnu_hash;
}
unsafe extern "C" fn elf_gnu_hash(mut name: *const libc::c_uchar) -> Elf32_Word {
    let mut h: Elf32_Word = 5381 as libc::c_int as Elf32_Word;
    let mut c: libc::c_uchar = 0;
    loop {
        let fresh7 = name;
        name = name.offset(1);
        c = *fresh7;
        if !(c != 0) {
            break;
        }
        h = (h * 33 as libc::c_int as Elf32_Word).wrapping_add(c as Elf32_Word);
    }
    return h;
}
unsafe extern "C" fn update_gnu_hash(mut s1: *mut TCCState, mut gnu_hash: *mut Section) {
    let mut old_to_new_syms: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut new_syms: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut nb_syms: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nbuckets: libc::c_int = 0;
    let mut bloom_size: libc::c_int = 0;
    let mut bloom_shift: libc::c_int = 0;
    let mut p: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut q: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut vs: *mut Section = 0 as *mut Section;
    let mut dynsym: *mut Section = (*s1).dynsym;
    let mut ptr: *mut Elf32_Word = 0 as *mut Elf32_Word;
    let mut buckets: *mut Elf32_Word = 0 as *mut Elf32_Word;
    let mut chain: *mut Elf32_Word = 0 as *mut Elf32_Word;
    let mut hash: *mut Elf32_Word = 0 as *mut Elf32_Word;
    let mut nextbuck: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut bloom: *mut Elf64_Addr = 0 as *mut Elf64_Addr;
    let mut strtab: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buck: *mut C2RustUnnamed_5 = 0 as *mut C2RustUnnamed_5;
    strtab = (*(*dynsym).link).data;
    nb_syms = ((*dynsym).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    new_syms = tcc_malloc(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong),
    ) as *mut Elf64_Sym;
    old_to_new_syms = tcc_malloc(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    hash = tcc_malloc(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf32_Word>() as libc::c_ulong),
    ) as *mut Elf32_Word;
    nextbuck = tcc_malloc(
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    p = (*dynsym).data as *mut Elf64_Sym;
    q = new_syms;
    i = 0 as libc::c_int;
    while i < nb_syms {
        if (*p).st_shndx as libc::c_int == 0 as libc::c_int {
            *old_to_new_syms
                .offset(
                    i as isize,
                ) = q.offset_from(new_syms) as libc::c_long as libc::c_int;
            let fresh8 = q;
            q = q.offset(1);
            *fresh8 = *p;
        } else {
            *hash
                .offset(i as isize) = elf_gnu_hash(strtab.offset((*p).st_name as isize));
        }
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    ptr = (*gnu_hash).data as *mut Elf32_Word;
    nbuckets = *ptr.offset(0 as libc::c_int as isize) as libc::c_int;
    bloom_size = *ptr.offset(2 as libc::c_int as isize) as libc::c_int;
    bloom_shift = *ptr.offset(3 as libc::c_int as isize) as libc::c_int;
    bloom = &mut *ptr.offset(4 as libc::c_int as isize) as *mut Elf32_Word
        as *mut libc::c_void as *mut Elf64_Addr;
    buckets = &mut *bloom.offset(bloom_size as isize) as *mut Elf64_Addr
        as *mut libc::c_void as *mut Elf32_Word;
    chain = &mut *buckets.offset(nbuckets as isize) as *mut Elf32_Word;
    buck = tcc_malloc(
        (nbuckets as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong),
    ) as *mut C2RustUnnamed_5;
    if (*gnu_hash).data_offset
        != ((4 as libc::c_int * 4 as libc::c_int + 8 as libc::c_int * bloom_size
            + nbuckets * 4 as libc::c_int) as libc::c_long
            + (nb_syms as libc::c_long - q.offset_from(new_syms) as libc::c_long)
                * 4 as libc::c_int as libc::c_long) as libc::c_ulong
    {
        tcc_enter_state(s1);
        (Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
        ))
            .expect(
                "non-null function pointer",
            )(b"gnu_hash size incorrect\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < nbuckets {
        (*buck.offset(i as isize)).first = -(1 as libc::c_int);
        i += 1;
        i;
    }
    p = (*dynsym).data as *mut Elf64_Sym;
    i = 0 as libc::c_int;
    while i < nb_syms {
        if (*p).st_shndx as libc::c_int != 0 as libc::c_int {
            let mut bucket: libc::c_int = (*hash.offset(i as isize)
                % nbuckets as Elf32_Word) as libc::c_int;
            if (*buck.offset(bucket as isize)).first == -(1 as libc::c_int) {
                let ref mut fresh9 = (*buck.offset(bucket as isize)).last;
                *fresh9 = i;
                (*buck.offset(bucket as isize)).first = *fresh9;
            } else {
                *nextbuck
                    .offset(
                        (*buck.offset(bucket as isize)).last as isize,
                    ) = i as libc::c_uint;
                (*buck.offset(bucket as isize)).last = i;
            }
        }
        i += 1;
        i;
        p = p.offset(1);
        p;
    }
    p = (*dynsym).data as *mut Elf64_Sym;
    i = 0 as libc::c_int;
    while i < nbuckets {
        let mut cur: libc::c_int = (*buck.offset(i as isize)).first;
        if cur != -(1 as libc::c_int) {
            *buckets
                .offset(
                    i as isize,
                ) = q.offset_from(new_syms) as libc::c_long as Elf32_Word;
            loop {
                *old_to_new_syms
                    .offset(
                        cur as isize,
                    ) = q.offset_from(new_syms) as libc::c_long as libc::c_int;
                let fresh10 = q;
                q = q.offset(1);
                *fresh10 = *p.offset(cur as isize);
                let fresh11 = chain;
                chain = chain.offset(1);
                *fresh11 = *hash.offset(cur as isize)
                    & !(1 as libc::c_int) as Elf32_Word;
                *bloom
                    .offset(
                        (*hash.offset(cur as isize)
                            / (8 as libc::c_int * 8 as libc::c_int) as Elf32_Word
                            % bloom_size as Elf32_Word) as isize,
                    )
                    |= (1 as libc::c_int as Elf64_Addr)
                        << *hash.offset(cur as isize)
                            % (8 as libc::c_int * 8 as libc::c_int) as Elf32_Word
                        | (1 as libc::c_int as Elf64_Addr)
                            << (*hash.offset(cur as isize) >> bloom_shift)
                                % (8 as libc::c_int * 8 as libc::c_int) as Elf32_Word;
                if cur == (*buck.offset(i as isize)).last {
                    break;
                }
                cur = *nextbuck.offset(cur as isize) as libc::c_int;
            }
            *chain.offset(-(1 as libc::c_int) as isize)
                |= 1 as libc::c_int as Elf32_Word;
        }
        i += 1;
        i;
    }
    memcpy(
        (*dynsym).data as *mut libc::c_void,
        new_syms as *const libc::c_void,
        (nb_syms as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong),
    );
    tcc_free(new_syms as *mut libc::c_void);
    tcc_free(hash as *mut libc::c_void);
    tcc_free(buck as *mut libc::c_void);
    tcc_free(nextbuck as *mut libc::c_void);
    update_relocs(s1, dynsym, old_to_new_syms, 0 as libc::c_int);
    vs = (*s1).versym_section;
    if !vs.is_null() {
        let mut newver: *mut Elf64_Half = 0 as *mut Elf64_Half;
        let mut versym: *mut Elf64_Half = (*vs).data as *mut Elf64_Half;
        newver = tcc_malloc(
            (nb_syms as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Elf64_Half>() as libc::c_ulong),
        ) as *mut Elf64_Half;
        i = 0 as libc::c_int;
        while i < nb_syms {
            *newver
                .offset(
                    *old_to_new_syms.offset(i as isize) as isize,
                ) = *versym.offset(i as isize);
            i += 1;
            i;
        }
        memcpy(
            (*vs).data as *mut libc::c_void,
            newver as *const libc::c_void,
            (nb_syms as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Elf64_Half>() as libc::c_ulong),
        );
        tcc_free(newver as *mut libc::c_void);
    }
    tcc_free(old_to_new_syms as *mut libc::c_void);
    ptr = (*(*dynsym).hash).data as *mut Elf32_Word;
    rebuild_hash(dynsym, *ptr.offset(0 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn relocate_syms(
    mut s1: *mut TCCState,
    mut symtab: *mut Section,
    mut do_resolve: libc::c_int,
) {
    let mut current_block: u64;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_bind: libc::c_int = 0;
    let mut sh_num: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    sym = ((*symtab).data as *mut Elf64_Sym).offset(1 as libc::c_int as isize);
    while sym < ((*symtab).data).offset((*symtab).data_offset as isize) as *mut Elf64_Sym
    {
        sh_num = (*sym).st_shndx as libc::c_int;
        if sh_num == 0 as libc::c_int {
            if !(do_resolve == 2 as libc::c_int) {
                name = ((*(*(*s1).c2rust_unnamed.symtab).link).data as *mut libc::c_char)
                    .offset((*sym).st_name as isize);
                if do_resolve != 0 {
                    let mut name_ud: *const libc::c_char = &*name
                        .offset((*s1).leading_underscore as isize)
                        as *const libc::c_char;
                    let mut addr: *mut libc::c_void = 0 as *mut libc::c_void;
                    if (*s1).nostdlib == 0 {
                        addr = dlsym(0 as *mut libc::c_void, name_ud);
                    }
                    if addr.is_null() {
                        let mut i: libc::c_int = 0;
                        i = 0 as libc::c_int;
                        while i < (*s1).nb_loaded_dlls {
                            addr = dlsym(
                                (**((*s1).loaded_dlls).offset(i as isize)).handle,
                                name_ud,
                            );
                            if !addr.is_null() {
                                break;
                            }
                            i += 1;
                            i;
                        }
                    }
                    if !addr.is_null() {
                        (*sym).st_value = addr as Elf64_Addr;
                        current_block = 6239978542346980191;
                    } else {
                        current_block = 5689001924483802034;
                    }
                } else if !((*s1).dynsym).is_null()
                    && find_elf_sym((*s1).dynsym, name) != 0
                {
                    current_block = 6239978542346980191;
                } else {
                    current_block = 5689001924483802034;
                }
                match current_block {
                    6239978542346980191 => {}
                    _ => {
                        if !(strcmp(
                            name,
                            b"_fp_hw\0" as *const u8 as *const libc::c_char,
                        ) == 0)
                        {
                            sym_bind = (*sym).st_info as libc::c_int >> 4 as libc::c_int;
                            if sym_bind == 2 as libc::c_int {
                                (*sym).st_value = 0 as libc::c_int as Elf64_Addr;
                            } else {
                                tcc_enter_state(s1);
                                (Some(
                                    _tcc_error_noabort
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                            ...
                                        ) -> libc::c_int,
                                ))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    b"undefined symbol '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    name,
                                );
                            }
                        }
                    }
                }
            }
        } else if sh_num < 0xff00 as libc::c_int {
            (*sym)
                .st_value = ((*sym).st_value)
                .wrapping_add(
                    (**((*s1).sections).offset((*sym).st_shndx as isize)).sh_addr,
                );
        }
        sym = sym.offset(1);
        sym;
    }
}
unsafe extern "C" fn relocate_section(
    mut s1: *mut TCCState,
    mut s: *mut Section,
    mut sr: *mut Section,
) {
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut type_0: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tgt: Elf64_Addr = 0;
    let mut addr: Elf64_Addr = 0;
    let mut is_dwarf: libc::c_int = ((*s).sh_num >= (*s1).dwlo
        && (*s).sh_num < (*s1).dwhi) as libc::c_int;
    (*s1).qrel = (*sr).data as *mut Elf64_Rela;
    rel = ((*sr).data as *mut Elf64_Rela).offset(0 as libc::c_int as isize);
    while rel < ((*sr).data).offset((*sr).data_offset as isize) as *mut Elf64_Rela {
        ptr = ((*s).data).offset((*rel).r_offset as isize);
        sym_index = ((*rel).r_info >> 32 as libc::c_int) as libc::c_int;
        sym = &mut *((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
            .offset(sym_index as isize) as *mut Elf64_Sym;
        type_0 = ((*rel).r_info & 0xffffffff as libc::c_uint as Elf64_Xword)
            as libc::c_int;
        tgt = (*sym).st_value;
        tgt = tgt.wrapping_add((*rel).r_addend as Elf64_Addr);
        if is_dwarf != 0 && type_0 == 10 as libc::c_int
            && (*sym).st_shndx as libc::c_int >= (*s1).dwlo
            && ((*sym).st_shndx as libc::c_int) < (*s1).dwhi
        {
            add32le(
                ptr,
                tgt
                    .wrapping_sub(
                        (**((*s1).sections).offset((*sym).st_shndx as isize)).sh_addr,
                    ) as int32_t,
            );
        } else {
            addr = ((*s).sh_addr).wrapping_add((*rel).r_offset);
            relocate(s1, rel, type_0, ptr, addr, tgt);
        }
        rel = rel.offset(1);
        rel;
    }
    if (*sr).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*sr).link = (*s1).dynsym;
        if (*s1).output_type & 4 as libc::c_int != 0 {
            let mut r: size_t = ((*s1).qrel as *mut uint8_t).offset_from((*sr).data)
                as libc::c_long as size_t;
            if (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                < 8 as libc::c_int as libc::c_ulong
                && 0 as libc::c_int
                    == strcmp(
                        ((*s).name).as_mut_ptr(),
                        b".stab\0" as *const u8 as *const libc::c_char,
                    )
            {
                r = 0 as libc::c_int as size_t;
            }
            (*sr).sh_size = r;
            (*sr).data_offset = (*sr).sh_size;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn relocate_sections(mut s1: *mut TCCState) {
    let mut i: libc::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut sr: *mut Section = 0 as *mut Section;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        sr = *((*s1).sections).offset(i as isize);
        if !((*sr).sh_type != 4 as libc::c_int) {
            s = *((*s1).sections).offset((*sr).sh_info as isize);
            if s != (*s1).got || (*s1).static_link as libc::c_int != 0
                || (*s1).output_type == 1 as libc::c_int
            {
                relocate_section(s1, s, sr);
            }
            if (*sr).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
                rel = ((*sr).data as *mut Elf64_Rela).offset(0 as libc::c_int as isize);
                while rel
                    < ((*sr).data).offset((*sr).data_offset as isize) as *mut Elf64_Rela
                {
                    (*rel).r_offset = ((*rel).r_offset).wrapping_add((*s).sh_addr);
                    rel = rel.offset(1);
                    rel;
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn prepare_dynamic_rel(
    mut s1: *mut TCCState,
    mut sr: *mut Section,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    rel = ((*sr).data as *mut Elf64_Rela).offset(0 as libc::c_int as isize);
    while rel < ((*sr).data).offset((*sr).data_offset as isize) as *mut Elf64_Rela {
        let mut sym_index: libc::c_int = ((*rel).r_info >> 32 as libc::c_int)
            as libc::c_int;
        let mut type_0: libc::c_int = ((*rel).r_info
            & 0xffffffff as libc::c_uint as Elf64_Xword) as libc::c_int;
        match type_0 {
            10 | 11 | 1 => {
                count += 1;
                count;
            }
            2 => {
                let mut sym: *mut Elf64_Sym = &mut *((*(*s1)
                    .c2rust_unnamed
                    .symtab_section)
                    .data as *mut Elf64_Sym)
                    .offset(sym_index as isize) as *mut Elf64_Sym;
                if (*sym).st_shndx as libc::c_int != 0 as libc::c_int
                    && (*sym).st_other as libc::c_int & 0x3 as libc::c_int
                        == 2 as libc::c_int
                {
                    (*rel)
                        .r_info = ((sym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(4 as libc::c_int as Elf64_Xword);
                } else if !((*s1).output_type != 4 as libc::c_int) {
                    if (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).dyn_index != 0 {
                        count += 1;
                        count;
                    }
                }
            }
            _ => {}
        }
        rel = rel.offset(1);
        rel;
    }
    return count;
}
unsafe extern "C" fn build_got(mut s1: *mut TCCState) -> libc::c_int {
    (*s1)
        .got = new_section(
        s1,
        b".got\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int,
    );
    (*(*s1).got).sh_entsize = 4 as libc::c_int;
    section_ptr_add((*s1).got, (3 as libc::c_int * 8 as libc::c_int) as Elf64_Addr);
    return set_elf_sym(
        (*s1).c2rust_unnamed.symtab_section,
        0 as libc::c_int as Elf64_Addr,
        0 as libc::c_int as libc::c_ulong,
        ((1 as libc::c_int) << 4 as libc::c_int)
            + (1 as libc::c_int & 0xf as libc::c_int),
        0 as libc::c_int,
        (*(*s1).got).sh_num,
        b"_GLOBAL_OFFSET_TABLE_\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn put_got_entry(
    mut s1: *mut TCCState,
    mut dyn_reloc_type: libc::c_int,
    mut sym_index: libc::c_int,
) -> *mut sym_attr {
    let mut need_plt_entry: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut attr: *mut sym_attr = 0 as *mut sym_attr;
    let mut got_offset: libc::c_uint = 0;
    let mut plt_name: [libc::c_char; 200] = [0; 200];
    let mut len: libc::c_int = 0;
    let mut s_rel: *mut Section = 0 as *mut Section;
    need_plt_entry = (dyn_reloc_type == 7 as libc::c_int) as libc::c_int;
    attr = get_sym_attr(s1, sym_index, 1 as libc::c_int);
    if if need_plt_entry != 0 { (*attr).plt_offset } else { (*attr).got_offset } != 0 {
        return attr;
    }
    s_rel = (*s1).got;
    if need_plt_entry != 0 {
        if ((*s1).plt).is_null() {
            (*s1)
                .plt = new_section(
                s1,
                b".plt\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int,
            );
            (*(*s1).plt).sh_entsize = 4 as libc::c_int;
        }
        s_rel = (*s1).plt;
    }
    got_offset = (*(*s1).got).data_offset as libc::c_uint;
    section_ptr_add((*s1).got, 8 as libc::c_int as Elf64_Addr);
    sym = &mut *((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset(sym_index as isize) as *mut Elf64_Sym;
    name = ((*(*(*s1).c2rust_unnamed.symtab_section).link).data as *mut libc::c_char)
        .offset((*sym).st_name as isize);
    if !((*s1).dynsym).is_null() {
        if (*sym).st_info as libc::c_int >> 4 as libc::c_int == 0 as libc::c_int {
            put_elf_reloc(
                (*s1).dynsym,
                (*s1).got,
                got_offset as libc::c_ulong,
                8 as libc::c_int,
                sym_index,
            );
        } else {
            if 0 as libc::c_int == (*attr).dyn_index {
                (*attr)
                    .dyn_index = set_elf_sym(
                    (*s1).dynsym,
                    (*sym).st_value,
                    (*sym).st_size,
                    (*sym).st_info as libc::c_int,
                    0 as libc::c_int,
                    (*sym).st_shndx as libc::c_int,
                    name,
                );
            }
            put_elf_reloc(
                (*s1).dynsym,
                s_rel,
                got_offset as libc::c_ulong,
                dyn_reloc_type,
                (*attr).dyn_index,
            );
        }
    } else {
        put_elf_reloc(
            (*s1).c2rust_unnamed.symtab_section,
            (*s1).got,
            got_offset as libc::c_ulong,
            dyn_reloc_type,
            sym_index,
        );
    }
    if need_plt_entry != 0 {
        (*attr).plt_offset = create_plt_entry(s1, got_offset, attr);
        len = strlen(name) as libc::c_int;
        if len as libc::c_ulong
            > (::core::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong)
                .wrapping_sub(5 as libc::c_int as libc::c_ulong)
        {
            len = (::core::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong)
                .wrapping_sub(5 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        memcpy(
            plt_name.as_mut_ptr() as *mut libc::c_void,
            name as *const libc::c_void,
            len as libc::c_ulong,
        );
        strcpy(
            plt_name.as_mut_ptr().offset(len as isize),
            b"@plt\0" as *const u8 as *const libc::c_char,
        );
        (*attr)
            .plt_sym = put_elf_sym(
            (*s1).c2rust_unnamed.symtab,
            (*attr).plt_offset as Elf64_Addr,
            0 as libc::c_int as libc::c_ulong,
            ((1 as libc::c_int) << 4 as libc::c_int)
                + (2 as libc::c_int & 0xf as libc::c_int),
            0 as libc::c_int,
            (*(*s1).plt).sh_num,
            plt_name.as_mut_ptr(),
        );
    } else {
        (*attr).got_offset = got_offset;
    }
    return attr;
}
#[no_mangle]
pub unsafe extern "C" fn build_got_entries(
    mut s1: *mut TCCState,
    mut got_sym: libc::c_int,
) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut gotplt_entry: libc::c_int = 0;
    let mut reloc_type: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut attr: *mut sym_attr = 0 as *mut sym_attr;
    let mut pass: libc::c_int = 0 as libc::c_int;
    loop {
        i = 1 as libc::c_int;
        while i < (*s1).nb_sections {
            s = *((*s1).sections).offset(i as isize);
            if !((*s).sh_type != 4 as libc::c_int) {
                if !((*s).link != (*s1).c2rust_unnamed.symtab_section) {
                    let mut current_block_19: u64;
                    rel = ((*s).data as *mut Elf64_Rela)
                        .offset(0 as libc::c_int as isize);
                    while rel
                        < ((*s).data).offset((*s).data_offset as isize)
                            as *mut Elf64_Rela
                    {
                        type_0 = ((*rel).r_info
                            & 0xffffffff as libc::c_uint as Elf64_Xword) as libc::c_int;
                        gotplt_entry = gotplt_entry_type(type_0);
                        if gotplt_entry == -(1 as libc::c_int) {
                            tcc_enter_state(s1);
                            (Some(
                                _tcc_error_noabort
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        ...
                                    ) -> libc::c_int,
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                b"Unknown relocation type for got: %d\0" as *const u8
                                    as *const libc::c_char,
                                type_0,
                            );
                        } else {
                            sym_index = ((*rel).r_info >> 32 as libc::c_int)
                                as libc::c_int;
                            sym = &mut *((*(*s1).c2rust_unnamed.symtab_section).data
                                as *mut Elf64_Sym)
                                .offset(sym_index as isize) as *mut Elf64_Sym;
                            if !(gotplt_entry == NO_GOTPLT_ENTRY as libc::c_int) {
                                if gotplt_entry == AUTO_GOTPLT_ENTRY as libc::c_int {
                                    if (*sym).st_shndx as libc::c_int == 0 as libc::c_int {
                                        let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                                        let mut dynindex: libc::c_int = 0;
                                        if 1 as libc::c_int == 0
                                            && (*s1).output_type & 4 as libc::c_int != 0
                                        {
                                            current_block_19 = 7351195479953500246;
                                        } else if !((*s1).dynsym).is_null() {
                                            dynindex = (*get_sym_attr(s1, sym_index, 0 as libc::c_int))
                                                .dyn_index;
                                            esym = ((*(*s1).dynsym).data as *mut Elf64_Sym)
                                                .offset(dynindex as isize);
                                            if dynindex != 0
                                                && ((*esym).st_info as libc::c_int & 0xf as libc::c_int
                                                    == 2 as libc::c_int
                                                    || (*esym).st_info as libc::c_int & 0xf as libc::c_int
                                                        == 0 as libc::c_int
                                                        && (*sym).st_info as libc::c_int & 0xf as libc::c_int
                                                            == 2 as libc::c_int)
                                            {
                                                current_block_19 = 16827425992766083582;
                                            } else {
                                                current_block_19 = 4068382217303356765;
                                            }
                                        } else {
                                            current_block_19 = 4068382217303356765;
                                        }
                                    } else if (*sym).st_shndx as libc::c_int
                                        == 0xfff1 as libc::c_int
                                    {
                                        if (*sym).st_value == 0 as libc::c_int as Elf64_Addr {
                                            current_block_19 = 7351195479953500246;
                                        } else if 8 as libc::c_int != 8 as libc::c_int {
                                            current_block_19 = 7351195479953500246;
                                        } else {
                                            current_block_19 = 4068382217303356765;
                                        }
                                    } else {
                                        current_block_19 = 7351195479953500246;
                                    }
                                } else {
                                    current_block_19 = 4068382217303356765;
                                }
                                match current_block_19 {
                                    7351195479953500246 => {}
                                    _ => {
                                        match current_block_19 {
                                            4068382217303356765 => {
                                                if (type_0 == 4 as libc::c_int
                                                    || type_0 == 2 as libc::c_int)
                                                    && (*sym).st_shndx as libc::c_int != 0 as libc::c_int
                                                    && ((*sym).st_other as libc::c_int & 0x3 as libc::c_int
                                                        != 0 as libc::c_int
                                                        || (*sym).st_info as libc::c_int >> 4 as libc::c_int
                                                            == 0 as libc::c_int
                                                        || (*s1).output_type & 2 as libc::c_int != 0)
                                                {
                                                    if pass != 0 as libc::c_int {
                                                        current_block_19 = 7351195479953500246;
                                                    } else {
                                                        (*rel)
                                                            .r_info = ((sym_index as Elf64_Xword) << 32 as libc::c_int)
                                                            .wrapping_add(2 as libc::c_int as Elf64_Xword);
                                                        current_block_19 = 7351195479953500246;
                                                    }
                                                } else {
                                                    reloc_type = code_reloc(type_0);
                                                    if reloc_type == -(1 as libc::c_int) {
                                                        tcc_enter_state(s1);
                                                        (Some(
                                                            _tcc_error_noabort
                                                                as unsafe extern "C" fn(
                                                                    *const libc::c_char,
                                                                    ...
                                                                ) -> libc::c_int,
                                                        ))
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            b"Unknown relocation type: %d\0" as *const u8
                                                                as *const libc::c_char,
                                                            type_0,
                                                        );
                                                        current_block_19 = 7351195479953500246;
                                                    } else if reloc_type != 0 as libc::c_int {
                                                        current_block_19 = 16827425992766083582;
                                                    } else if pass != 1 as libc::c_int {
                                                        current_block_19 = 7351195479953500246;
                                                    } else {
                                                        reloc_type = 6 as libc::c_int;
                                                        current_block_19 = 14832935472441733737;
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block_19 {
                                            7351195479953500246 => {}
                                            _ => {
                                                match current_block_19 {
                                                    16827425992766083582 => {
                                                        if pass != 0 as libc::c_int {
                                                            current_block_19 = 7351195479953500246;
                                                        } else {
                                                            reloc_type = 7 as libc::c_int;
                                                            current_block_19 = 14832935472441733737;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                                match current_block_19 {
                                                    7351195479953500246 => {}
                                                    _ => {
                                                        if ((*s1).got).is_null() {
                                                            got_sym = build_got(s1);
                                                        }
                                                        if !(gotplt_entry == BUILD_GOT_ONLY as libc::c_int) {
                                                            attr = put_got_entry(s1, reloc_type, sym_index);
                                                            if reloc_type == 7 as libc::c_int {
                                                                (*rel)
                                                                    .r_info = (((*attr).plt_sym as Elf64_Xword)
                                                                    << 32 as libc::c_int)
                                                                    .wrapping_add(type_0 as Elf64_Xword);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        rel = rel.offset(1);
                        rel;
                    }
                }
            }
            i += 1;
            i;
        }
        pass += 1;
        if !(pass < 2 as libc::c_int) {
            break;
        }
    }
    if !((*s1).plt).is_null() && !((*(*s1).plt).reloc).is_null() {
        (*(*(*s1).plt).reloc).sh_info = (*(*s1).got).sh_num;
    }
    if got_sym != 0 {
        (*((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
            .offset(got_sym as isize))
            .st_size = (*(*s1).got).data_offset;
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_global_sym(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
    mut sec: *mut Section,
    mut offs: Elf64_Addr,
) -> libc::c_int {
    let mut shn: libc::c_int = if !sec.is_null() {
        (*sec).sh_num
    } else if offs != 0 || name.is_null() {
        0xfff1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if !sec.is_null() && offs == -(1 as libc::c_int) as Elf64_Addr {
        offs = (*sec).data_offset;
    }
    return set_elf_sym(
        (*s1).c2rust_unnamed.symtab_section,
        offs,
        0 as libc::c_int as libc::c_ulong,
        ((if !name.is_null() { 1 as libc::c_int } else { 0 as libc::c_int })
            << 4 as libc::c_int) + (0 as libc::c_int & 0xf as libc::c_int),
        0 as libc::c_int,
        shn,
        name,
    );
}
unsafe extern "C" fn add_init_array_defines(
    mut s1: *mut TCCState,
    mut section_name: *const libc::c_char,
) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut end_offset: Elf64_Addr = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    s = have_section(s1, section_name);
    if s.is_null() || (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        end_offset = 0 as libc::c_int as Elf64_Addr;
        s = (*s1).text_section;
    } else {
        end_offset = (*s).data_offset;
    }
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"__%s_start\0" as *const u8 as *const libc::c_char,
        section_name.offset(1 as libc::c_int as isize),
    );
    set_global_sym(s1, buf.as_mut_ptr(), s, 0 as libc::c_int as Elf64_Addr);
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"__%s_end\0" as *const u8 as *const libc::c_char,
        section_name.offset(1 as libc::c_int as isize),
    );
    set_global_sym(s1, buf.as_mut_ptr(), s, end_offset);
}
#[no_mangle]
pub unsafe extern "C" fn add_array(
    mut s1: *mut TCCState,
    mut sec: *const libc::c_char,
    mut c: libc::c_int,
) {
    let mut s: *mut Section = 0 as *mut Section;
    s = find_section(s1, sec);
    (*s).sh_flags = (1 as libc::c_int) << 1 as libc::c_int;
    (*s)
        .sh_type = if *sec.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32
    {
        14 as libc::c_int
    } else {
        15 as libc::c_int
    };
    put_elf_reloc((*s1).c2rust_unnamed.symtab, s, (*s).data_offset, 1 as libc::c_int, c);
    section_ptr_add(s, 8 as libc::c_int as Elf64_Addr);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_bcheck(mut s1: *mut TCCState) {
    if 0 as libc::c_int == (*s1).do_bounds_check as libc::c_int {
        return;
    }
    section_ptr_add(
        (*s1).bounds_section,
        ::core::mem::size_of::<Elf64_Addr>() as libc::c_ulong,
    );
}
unsafe extern "C" fn set_local_sym(
    mut s1: *mut TCCState,
    mut name: *const libc::c_char,
    mut s: *mut Section,
    mut offset: libc::c_int,
) {
    let mut c: libc::c_int = find_elf_sym((*s1).c2rust_unnamed.symtab, name);
    if c != 0 {
        let mut esym: *mut Elf64_Sym = ((*(*s1).c2rust_unnamed.symtab).data
            as *mut Elf64_Sym)
            .offset(c as isize);
        (*esym)
            .st_info = (((0 as libc::c_int) << 4 as libc::c_int)
            + (0 as libc::c_int & 0xf as libc::c_int)) as libc::c_uchar;
        (*esym).st_value = offset as Elf64_Addr;
        (*esym).st_shndx = (*s).sh_num as Elf64_Section;
    }
}
unsafe extern "C" fn tcc_compile_string_no_debug(
    mut s: *mut TCCState,
    mut str: *const libc::c_char,
) {
    let mut save_do_debug: libc::c_int = (*s).do_debug as libc::c_int;
    let mut save_test_coverage: libc::c_int = (*s).test_coverage as libc::c_int;
    (*s).do_debug = 0 as libc::c_int as libc::c_uchar;
    (*s).test_coverage = 0 as libc::c_int as libc::c_uchar;
    tcc_compile_string(s, str);
    (*s).do_debug = save_do_debug as libc::c_uchar;
    (*s).test_coverage = save_test_coverage as libc::c_uchar;
}
unsafe extern "C" fn put_ptr(
    mut s1: *mut TCCState,
    mut s: *mut Section,
    mut offs: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    c = set_global_sym(s1, 0 as *const libc::c_char, s, offs as Elf64_Addr);
    s = (*s1).data_section;
    put_elf_reloc((*s1).c2rust_unnamed.symtab, s, (*s).data_offset, 1 as libc::c_int, c);
    section_ptr_add(s, 8 as libc::c_int as Elf64_Addr);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_btstub(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut n: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cstr: CString = CString {
        size: 0,
        size_allocated: 0,
        data: 0 as *mut libc::c_char,
    };
    let mut __rt_info: *const libc::c_char = &*(b"___rt_info\0" as *const u8
        as *const libc::c_char)
        .offset(((*s1).leading_underscore == 0) as libc::c_int as isize)
        as *const libc::c_char;
    s = (*s1).data_section;
    section_ptr_add(
        s,
        ((*s).data_offset).wrapping_neg()
            & (8 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    o = (*s).data_offset as libc::c_int;
    if (*s1).dwarf != 0 {
        put_ptr(s1, (*s1).dwarf_line_section, 0 as libc::c_int);
        put_ptr(s1, (*s1).dwarf_line_section, -(1 as libc::c_int));
        if (*s1).dwarf as libc::c_int >= 5 as libc::c_int {
            put_ptr(s1, (*s1).dwarf_line_str_section, 0 as libc::c_int);
        } else {
            put_ptr(s1, (*s1).dwarf_str_section, 0 as libc::c_int);
        }
    } else {
        put_ptr(s1, (*s1).stab_section, 0 as libc::c_int);
        put_ptr(s1, (*s1).stab_section, -(1 as libc::c_int));
        put_ptr(s1, (*(*s1).stab_section).link, 0 as libc::c_int);
    }
    section_ptr_add(s, (3 as libc::c_int * 8 as libc::c_int) as Elf64_Addr);
    if (*s1).output_type == 1 as libc::c_int
        && 0 as libc::c_int == (*s1).dwarf as libc::c_int
    {
        put_ptr(s1, (*s1).text_section, 0 as libc::c_int);
    } else {
        put_ptr(s1, 0 as *mut Section, 0 as libc::c_int);
    }
    n = 3 as libc::c_int * 8 as libc::c_int;
    if (*s1).do_bounds_check != 0 {
        put_ptr(s1, (*s1).bounds_section, 0 as libc::c_int);
        n -= 8 as libc::c_int;
    }
    section_ptr_add(s, n as Elf64_Addr);
    p = section_ptr_add(
        s,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *p.offset(0 as libc::c_int as isize) = (*s1).rt_num_callers;
    *p.offset(1 as libc::c_int as isize) = (*s1).dwarf as libc::c_int;
    if (*s1).output_type == 1 as libc::c_int {
        set_global_sym(s1, __rt_info, s, o as Elf64_Addr);
        return;
    }
    cstr_new(&mut cstr);
    cstr_printf(
        &mut cstr as *mut CString,
        b"extern void __bt_init(),__bt_exit(),__bt_init_dll();static void *__rt_info[];__attribute__((constructor)) static void __bt_init_rt(){\0"
            as *const u8 as *const libc::c_char,
    );
    cstr_printf(
        &mut cstr as *mut CString,
        b"__bt_init(__rt_info,%d);}\0" as *const u8 as *const libc::c_char,
        ((*s1).output_type != 4 as libc::c_int) as libc::c_int,
    );
    cstr_printf(
        &mut cstr as *mut CString,
        b"__attribute__((destructor)) static void __bt_exit_rt(){__bt_exit(__rt_info);}\0"
            as *const u8 as *const libc::c_char,
    );
    tcc_compile_string_no_debug(s1, cstr.data);
    cstr_free(&mut cstr);
    set_local_sym(s1, __rt_info, s, o);
}
unsafe extern "C" fn tcc_tcov_add_file(
    mut s1: *mut TCCState,
    mut filename: *const libc::c_char,
) {
    let mut cstr: CString = CString {
        size: 0,
        size_allocated: 0,
        data: 0 as *mut libc::c_char,
    };
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut wd: [libc::c_char; 1024] = [0; 1024];
    if ((*s1).tcov_section).is_null() {
        return;
    }
    section_ptr_add((*s1).tcov_section, 1 as libc::c_int as Elf64_Addr);
    write32le((*(*s1).tcov_section).data, (*(*s1).tcov_section).data_offset as uint32_t);
    cstr_new(&mut cstr);
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        cstr_printf(
            &mut cstr as *mut CString,
            b"%s.tcov\0" as *const u8 as *const libc::c_char,
            filename,
        );
    } else {
        getcwd(
            wd.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        cstr_printf(
            &mut cstr as *mut CString,
            b"%s/%s.tcov\0" as *const u8 as *const libc::c_char,
            wd.as_mut_ptr(),
            filename,
        );
    }
    ptr = section_ptr_add(
        (*s1).tcov_section,
        (cstr.size + 1 as libc::c_int) as Elf64_Addr,
    );
    strcpy(ptr as *mut libc::c_char, cstr.data);
    unlink(ptr as *mut libc::c_char);
    cstr_free(&mut cstr);
    cstr_new(&mut cstr);
    cstr_printf(
        &mut cstr as *mut CString,
        b"extern char *__tcov_data[];extern void __store_test_coverage ();__attribute__((destructor)) static void __tcov_exit() {__store_test_coverage(__tcov_data);}\0"
            as *const u8 as *const libc::c_char,
    );
    tcc_compile_string_no_debug(s1, cstr.data);
    cstr_free(&mut cstr);
    set_local_sym(
        s1,
        &*(b"___tcov_data\0" as *const u8 as *const libc::c_char)
            .offset(((*s1).leading_underscore == 0) as libc::c_int as isize),
        (*s1).tcov_section,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_add_crtbegin(mut s1: *mut TCCState) {
    if (*s1).output_type != 4 as libc::c_int {
        tcc_add_crt(s1, b"crt1.o\0" as *const u8 as *const libc::c_char);
    }
    tcc_add_crt(s1, b"crti.o\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tccelf_add_crtend(mut s1: *mut TCCState) {
    tcc_add_crt(s1, b"crtn.o\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_runtime(mut s1: *mut TCCState) {
    (*s1).filetype = 0 as libc::c_int as libc::c_uchar;
    tcc_add_bcheck(s1);
    tcc_add_pragma_libs(s1);
    if (*s1).nostdlib == 0 {
        let mut lpthread: libc::c_int = (*s1).option_pthread as libc::c_int;
        if (*s1).do_bounds_check as libc::c_int != 0
            && (*s1).output_type != 4 as libc::c_int
        {
            tcc_add_support(s1, b"bcheck.o\0" as *const u8 as *const libc::c_char);
            tcc_add_library(s1, b"dl\0" as *const u8 as *const libc::c_char);
            lpthread = 1 as libc::c_int;
        }
        if (*s1).do_backtrace != 0 {
            if (*s1).output_type & 2 as libc::c_int != 0 {
                tcc_add_support(s1, b"bt-exe.o\0" as *const u8 as *const libc::c_char);
            }
            if (*s1).output_type != 4 as libc::c_int {
                tcc_add_support(s1, b"bt-log.o\0" as *const u8 as *const libc::c_char);
            }
            tcc_add_btstub(s1);
            lpthread = 1 as libc::c_int;
        }
        if lpthread != 0 {
            tcc_add_library(s1, b"pthread\0" as *const u8 as *const libc::c_char);
        }
        tcc_add_library(s1, b"c\0" as *const u8 as *const libc::c_char);
        if (*::core::mem::transmute::<
            &[u8; 10],
            &[libc::c_char; 10],
        >(b"libtcc1.a\0"))[0 as libc::c_int as usize] != 0
        {
            tcc_add_support(s1, b"libtcc1.a\0" as *const u8 as *const libc::c_char);
        }
        if (*s1).output_type != 1 as libc::c_int {
            tccelf_add_crtend(s1);
        }
    }
}
unsafe extern "C" fn tcc_add_linker_symbols(mut s1: *mut TCCState) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    set_global_sym(
        s1,
        b"_etext\0" as *const u8 as *const libc::c_char,
        (*s1).text_section,
        -(1 as libc::c_int) as Elf64_Addr,
    );
    set_global_sym(
        s1,
        b"_edata\0" as *const u8 as *const libc::c_char,
        (*s1).data_section,
        -(1 as libc::c_int) as Elf64_Addr,
    );
    set_global_sym(
        s1,
        b"_end\0" as *const u8 as *const libc::c_char,
        (*s1).bss_section,
        -(1 as libc::c_int) as Elf64_Addr,
    );
    add_init_array_defines(s1, b".preinit_array\0" as *const u8 as *const libc::c_char);
    add_init_array_defines(s1, b".init_array\0" as *const u8 as *const libc::c_char);
    add_init_array_defines(s1, b".fini_array\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        let mut current_block_17: u64;
        s = *((*s1).sections).offset(i as isize);
        if (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            && ((*s).sh_type == 1 as libc::c_int || (*s).sh_type == 8 as libc::c_int
                || (*s).sh_type == 3 as libc::c_int)
        {
            let mut p0: *const libc::c_char = 0 as *const libc::c_char;
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            p0 = ((*s).name).as_mut_ptr();
            if *p0 as libc::c_int == '.' as i32 {
                p0 = p0.offset(1);
                p0;
            }
            p = p0;
            loop {
                let mut c: libc::c_int = *p as libc::c_int;
                if c == 0 {
                    current_block_17 = 13056961889198038528;
                    break;
                }
                if isid(c) == 0 && isnum(c) == 0 {
                    current_block_17 = 4808432441040389987;
                    break;
                }
                p = p.offset(1);
                p;
            }
            match current_block_17 {
                4808432441040389987 => {}
                _ => {
                    snprintf(
                        buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"__start_%s\0" as *const u8 as *const libc::c_char,
                        p0,
                    );
                    set_global_sym(
                        s1,
                        buf.as_mut_ptr(),
                        s,
                        0 as libc::c_int as Elf64_Addr,
                    );
                    snprintf(
                        buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"__stop_%s\0" as *const u8 as *const libc::c_char,
                        p0,
                    );
                    set_global_sym(
                        s1,
                        buf.as_mut_ptr(),
                        s,
                        -(1 as libc::c_int) as Elf64_Addr,
                    );
                }
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn resolve_common_syms(mut s1: *mut TCCState) {
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    sym = ((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset(1 as libc::c_int as isize);
    while sym
        < ((*(*s1).c2rust_unnamed.symtab_section).data)
            .offset((*(*s1).c2rust_unnamed.symtab_section).data_offset as isize)
            as *mut Elf64_Sym
    {
        if (*sym).st_shndx as libc::c_int == 0xfff2 as libc::c_int {
            (*sym)
                .st_value = section_add(
                (*s1).bss_section,
                (*sym).st_size,
                (*sym).st_value as libc::c_int,
            );
            (*sym).st_shndx = (*(*s1).bss_section).sh_num as Elf64_Section;
        }
        sym = sym.offset(1);
        sym;
    }
    tcc_add_linker_symbols(s1);
}
#[no_mangle]
pub unsafe extern "C" fn fill_got_entry(
    mut s1: *mut TCCState,
    mut rel: *mut Elf64_Rela,
) {
    let mut sym_index: libc::c_int = ((*rel).r_info >> 32 as libc::c_int) as libc::c_int;
    let mut sym: *mut Elf64_Sym = &mut *((*(*s1).c2rust_unnamed.symtab_section).data
        as *mut Elf64_Sym)
        .offset(sym_index as isize) as *mut Elf64_Sym;
    let mut attr: *mut sym_attr = get_sym_attr(s1, sym_index, 0 as libc::c_int);
    let mut offset: libc::c_uint = (*attr).got_offset;
    if 0 as libc::c_int as libc::c_uint == offset {
        return;
    }
    section_reserve(
        (*s1).got,
        offset.wrapping_add(8 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    write64le(((*(*s1).got).data).offset(offset as isize), (*sym).st_value);
}
#[no_mangle]
pub unsafe extern "C" fn fill_got(mut s1: *mut TCCState) {
    let mut s: *mut Section = 0 as *mut Section;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if !((*s).sh_type != 4 as libc::c_int) {
            if !((*s).link != (*s1).c2rust_unnamed.symtab_section) {
                rel = ((*s).data as *mut Elf64_Rela).offset(0 as libc::c_int as isize);
                while rel
                    < ((*s).data).offset((*s).data_offset as isize) as *mut Elf64_Rela
                {
                    match (*rel).r_info & 0xffffffff as libc::c_uint as Elf64_Xword {
                        3 | 9 | 41 | 42 | 4 => {
                            fill_got_entry(s1, rel);
                        }
                        _ => {}
                    }
                    rel = rel.offset(1);
                    rel;
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn fill_local_got_entries(mut s1: *mut TCCState) {
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    if ((*(*s1).got).reloc).is_null() {
        return;
    }
    rel = ((*(*(*s1).got).reloc).data as *mut Elf64_Rela)
        .offset(0 as libc::c_int as isize);
    while rel
        < ((*(*(*s1).got).reloc).data).offset((*(*(*s1).got).reloc).data_offset as isize)
            as *mut Elf64_Rela
    {
        if (*rel).r_info & 0xffffffff as libc::c_uint as Elf64_Xword
            == 8 as libc::c_int as Elf64_Xword
        {
            let mut sym_index: libc::c_int = ((*rel).r_info >> 32 as libc::c_int)
                as libc::c_int;
            let mut sym: *mut Elf64_Sym = &mut *((*(*s1).c2rust_unnamed.symtab_section)
                .data as *mut Elf64_Sym)
                .offset(sym_index as isize) as *mut Elf64_Sym;
            let mut attr: *mut sym_attr = get_sym_attr(s1, sym_index, 0 as libc::c_int);
            let mut offset: libc::c_uint = (*attr).got_offset;
            if offset as Elf64_Addr
                != ((*rel).r_offset).wrapping_sub((*(*s1).got).sh_addr)
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"fill_local_got_entries: huh?\0" as *const u8 as *const libc::c_char,
                );
            }
            (*rel)
                .r_info = ((0 as libc::c_int as Elf64_Xword) << 32 as libc::c_int)
                .wrapping_add(8 as libc::c_int as Elf64_Xword);
            (*rel).r_addend = (*sym).st_value as Elf64_Sxword;
        }
        rel = rel.offset(1);
        rel;
    }
}
unsafe extern "C" fn bind_exe_dynsyms(mut s1: *mut TCCState, mut is_PIE: libc::c_int) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut sym_index: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut type_0: libc::c_int = 0;
    sym = ((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset(1 as libc::c_int as isize);
    while sym
        < ((*(*s1).c2rust_unnamed.symtab_section).data)
            .offset((*(*s1).c2rust_unnamed.symtab_section).data_offset as isize)
            as *mut Elf64_Sym
    {
        if (*sym).st_shndx as libc::c_int == 0 as libc::c_int {
            name = ((*(*(*s1).c2rust_unnamed.symtab_section).link).data
                as *mut libc::c_char)
                .offset((*sym).st_name as isize);
            sym_index = find_elf_sym((*s1).dynsymtab_section, name);
            if sym_index != 0 {
                if !(is_PIE != 0) {
                    esym = &mut *((*(*s1).dynsymtab_section).data as *mut Elf64_Sym)
                        .offset(sym_index as isize) as *mut Elf64_Sym;
                    type_0 = (*esym).st_info as libc::c_int & 0xf as libc::c_int;
                    if type_0 == 2 as libc::c_int || type_0 == 10 as libc::c_int {
                        let mut dynindex: libc::c_int = put_elf_sym(
                            (*s1).dynsym,
                            0 as libc::c_int as Elf64_Addr,
                            (*esym).st_size,
                            ((1 as libc::c_int) << 4 as libc::c_int)
                                + (2 as libc::c_int & 0xf as libc::c_int),
                            0 as libc::c_int,
                            0 as libc::c_int,
                            name,
                        );
                        let mut index_0: libc::c_int = sym
                            .offset_from(
                                (*(*s1).c2rust_unnamed.symtab_section).data
                                    as *mut Elf64_Sym,
                            ) as libc::c_long as libc::c_int;
                        (*get_sym_attr(s1, index_0, 1 as libc::c_int))
                            .dyn_index = dynindex;
                    } else if type_0 == 1 as libc::c_int {
                        let mut offset: libc::c_ulong = 0;
                        let mut dynsym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                        offset = (*(*s1).bss_section).data_offset;
                        offset = offset
                            .wrapping_add(16 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            & -(16 as libc::c_int) as libc::c_ulong;
                        set_elf_sym(
                            (*s1).c2rust_unnamed.symtab,
                            offset,
                            (*esym).st_size,
                            (*esym).st_info as libc::c_int,
                            0 as libc::c_int,
                            (*(*s1).bss_section).sh_num,
                            name,
                        );
                        index = put_elf_sym(
                            (*s1).dynsym,
                            offset,
                            (*esym).st_size,
                            (*esym).st_info as libc::c_int,
                            0 as libc::c_int,
                            (*(*s1).bss_section).sh_num,
                            name,
                        );
                        if (*esym).st_info as libc::c_int >> 4 as libc::c_int
                            == 2 as libc::c_int
                        {
                            dynsym = ((*(*s1).dynsymtab_section).data as *mut Elf64_Sym)
                                .offset(1 as libc::c_int as isize);
                            while dynsym
                                < ((*(*s1).dynsymtab_section).data)
                                    .offset((*(*s1).dynsymtab_section).data_offset as isize)
                                    as *mut Elf64_Sym
                            {
                                if (*dynsym).st_value == (*esym).st_value
                                    && (*dynsym).st_info as libc::c_int >> 4 as libc::c_int
                                        == 1 as libc::c_int
                                {
                                    let mut dynname: *mut libc::c_char = ((*(*(*s1)
                                        .dynsymtab_section)
                                        .link)
                                        .data as *mut libc::c_char)
                                        .offset((*dynsym).st_name as isize);
                                    put_elf_sym(
                                        (*s1).dynsym,
                                        offset,
                                        (*dynsym).st_size,
                                        (*dynsym).st_info as libc::c_int,
                                        0 as libc::c_int,
                                        (*(*s1).bss_section).sh_num,
                                        dynname,
                                    );
                                    break;
                                } else {
                                    dynsym = dynsym.offset(1);
                                    dynsym;
                                }
                            }
                        }
                        put_elf_reloc(
                            (*s1).dynsym,
                            (*s1).bss_section,
                            offset,
                            5 as libc::c_int,
                            index,
                        );
                        offset = offset.wrapping_add((*esym).st_size);
                        (*(*s1).bss_section).data_offset = offset;
                    }
                }
            } else if !((*sym).st_info as libc::c_int >> 4 as libc::c_int
                == 2 as libc::c_int
                || strcmp(name, b"_fp_hw\0" as *const u8 as *const libc::c_char) == 0)
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"undefined symbol '%s'\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
        }
        sym = sym.offset(1);
        sym;
    }
}
unsafe extern "C" fn bind_libs_dynsyms(mut s1: *mut TCCState) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut dynsym_index: libc::c_int = 0;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    sym = ((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset(1 as libc::c_int as isize);
    while sym
        < ((*(*s1).c2rust_unnamed.symtab_section).data)
            .offset((*(*s1).c2rust_unnamed.symtab_section).data_offset as isize)
            as *mut Elf64_Sym
    {
        name = ((*(*(*s1).c2rust_unnamed.symtab_section).link).data as *mut libc::c_char)
            .offset((*sym).st_name as isize);
        dynsym_index = find_elf_sym((*s1).dynsymtab_section, name);
        if (*sym).st_shndx as libc::c_int != 0 as libc::c_int {
            if (*sym).st_info as libc::c_int >> 4 as libc::c_int != 0 as libc::c_int
                && (dynsym_index != 0 || (*s1).rdynamic as libc::c_int != 0)
            {
                set_elf_sym(
                    (*s1).dynsym,
                    (*sym).st_value,
                    (*sym).st_size,
                    (*sym).st_info as libc::c_int,
                    0 as libc::c_int,
                    (*sym).st_shndx as libc::c_int,
                    name,
                );
            }
        } else if dynsym_index != 0 {
            esym = ((*(*s1).dynsymtab_section).data as *mut Elf64_Sym)
                .offset(dynsym_index as isize);
            if (*esym).st_shndx as libc::c_int == 0 as libc::c_int {
                if (*esym).st_info as libc::c_int >> 4 as libc::c_int != 2 as libc::c_int
                {
                    tcc_enter_state(s1);
                    (Some(
                        _tcc_warning
                            as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"undefined dynamic symbol '%s'\0" as *const u8
                            as *const libc::c_char,
                        name,
                    );
                }
            }
        }
        sym = sym.offset(1);
        sym;
    }
}
unsafe extern "C" fn export_global_syms(mut s1: *mut TCCState) {
    let mut dynindex: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    sym = ((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset(1 as libc::c_int as isize);
    while sym
        < ((*(*s1).c2rust_unnamed.symtab_section).data)
            .offset((*(*s1).c2rust_unnamed.symtab_section).data_offset as isize)
            as *mut Elf64_Sym
    {
        if (*sym).st_info as libc::c_int >> 4 as libc::c_int != 0 as libc::c_int {
            name = ((*(*(*s1).c2rust_unnamed.symtab_section).link).data
                as *mut libc::c_char)
                .offset((*sym).st_name as isize);
            dynindex = set_elf_sym(
                (*s1).dynsym,
                (*sym).st_value,
                (*sym).st_size,
                (*sym).st_info as libc::c_int,
                0 as libc::c_int,
                (*sym).st_shndx as libc::c_int,
                name,
            );
            index = sym
                .offset_from(
                    (*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym,
                ) as libc::c_long as libc::c_int;
            (*get_sym_attr(s1, index, 1 as libc::c_int)).dyn_index = dynindex;
        }
        sym = sym.offset(1);
        sym;
    }
}
unsafe extern "C" fn set_sec_sizes(mut s1: *mut TCCState) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut textrel: libc::c_int = 0 as libc::c_int;
    let mut file_type: libc::c_int = (*s1).output_type;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if (*s).sh_type == 4 as libc::c_int
            && (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int == 0
        {
            if file_type & 4 as libc::c_int != 0
                && (**((*s1).sections).offset((*s).sh_info as isize)).sh_flags
                    & (1 as libc::c_int) << 1 as libc::c_int != 0
            {
                let mut count: libc::c_int = prepare_dynamic_rel(s1, s);
                if count != 0 {
                    (*s).sh_flags |= (1 as libc::c_int) << 1 as libc::c_int;
                    (*s)
                        .sh_size = (count as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong,
                        );
                    if (**((*s1).sections).offset((*s).sh_info as isize)).sh_flags
                        & (1 as libc::c_int) << 2 as libc::c_int != 0
                    {
                        textrel += count;
                    }
                }
            }
        } else if (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            || (*s1).do_debug as libc::c_int != 0
        {
            (*s).sh_size = (*s).data_offset;
        }
        i += 1;
        i;
    }
    return textrel;
}
unsafe extern "C" fn sort_sections(
    mut s1: *mut TCCState,
    mut sec_order: *mut libc::c_int,
    mut d: *mut dyn_inf,
) -> libc::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut f0: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nb_sections: libc::c_int = (*s1).nb_sections;
    let mut sec_cls: *mut libc::c_int = sec_order.offset(nb_sections as isize);
    i = 1 as libc::c_int;
    while i < nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if 0 as libc::c_int == (*s).sh_name {
            j = 0x900 as libc::c_int;
        } else if (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            j = 0x100 as libc::c_int;
            if (*s).sh_flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                j = 0x200 as libc::c_int;
            }
            if (*s).sh_flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                j += 0x200 as libc::c_int;
            }
        } else {
            j = 0x700 as libc::c_int;
        }
        if j >= 0x700 as libc::c_int && (*s1).output_format != 0 as libc::c_int {
            (*s).sh_size = 0 as libc::c_int as libc::c_ulong;
            j = 0x900 as libc::c_int;
        }
        if (*s).sh_type == 2 as libc::c_int || (*s).sh_type == 11 as libc::c_int {
            k = 0x10 as libc::c_int;
        } else if (*s).sh_type == 3 as libc::c_int
            && strcmp(
                ((*s).name).as_mut_ptr(),
                b".stabstr\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            k = 0x11 as libc::c_int;
            if i == nb_sections - 1 as libc::c_int {
                k = 0xff as libc::c_int;
            }
        } else if (*s).sh_type == 5 as libc::c_int
            || (*s).sh_type == 0x6ffffff6 as libc::c_int
        {
            k = 0x12 as libc::c_int;
        } else if (*s).sh_type == 0x6ffffffd as libc::c_int
            || (*s).sh_type == 0x6ffffffe as libc::c_int
            || (*s).sh_type == 0x6fffffff as libc::c_int
        {
            k = 0x13 as libc::c_int;
        } else if (*s).sh_type == 4 as libc::c_int {
            k = 0x20 as libc::c_int;
            if !((*s1).plt).is_null() && s == (*(*s1).plt).reloc {
                k = 0x21 as libc::c_int;
            }
        } else if (*s).sh_flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            k = 0x30 as libc::c_int;
        } else if (*s).sh_type == 16 as libc::c_int {
            k = 0x41 as libc::c_int;
        } else if (*s).sh_type == 14 as libc::c_int {
            k = 0x42 as libc::c_int;
        } else if (*s).sh_type == 15 as libc::c_int {
            k = 0x43 as libc::c_int;
        } else if (*s).sh_type == 6 as libc::c_int {
            k = 0x46 as libc::c_int;
        } else if s == (*s1).got {
            k = 0x47 as libc::c_int;
        } else if !((*s).reloc).is_null()
            && (*(*s).reloc).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            && j == 0x100 as libc::c_int
        {
            k = 0x44 as libc::c_int;
        } else if (*s).sh_type == 7 as libc::c_int {
            k = 0x60 as libc::c_int;
        } else if (*s).sh_type == 8 as libc::c_int {
            k = 0x70 as libc::c_int;
        } else if s == (*d).interp {
            k = 0 as libc::c_int;
        } else {
            k = 0x50 as libc::c_int;
        }
        k += j;
        if k & 0xfff0 as libc::c_int == 0x140 as libc::c_int {
            k += 0x100 as libc::c_int;
            (*s).sh_flags |= (1 as libc::c_int) << 0 as libc::c_int;
        }
        n = i;
        while n > 1 as libc::c_int
            && {
                f = *sec_cls.offset((n - 1 as libc::c_int) as isize);
                k < f
            }
        {
            *sec_cls.offset(n as isize) = f;
            *sec_order
                .offset(n as isize) = *sec_order.offset((n - 1 as libc::c_int) as isize);
            n -= 1;
            n;
        }
        *sec_cls.offset(n as isize) = k;
        *sec_order.offset(n as isize) = i;
        i += 1;
        i;
    }
    *sec_order.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    (*d).shnum = 1 as libc::c_int;
    f0 = 0 as libc::c_int;
    n = f0;
    i = 1 as libc::c_int;
    while i < nb_sections {
        s = *((*s1).sections).offset(*sec_order.offset(i as isize) as isize);
        k = *sec_cls.offset(i as isize);
        f = 0 as libc::c_int;
        if k < 0x900 as libc::c_int {
            (*d).shnum += 1;
            (*d).shnum;
        }
        if k < 0x700 as libc::c_int {
            f = (*s).sh_flags
                & ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int);
            if k & 0xfff0 as libc::c_int == 0x240 as libc::c_int {
                f |= (1 as libc::c_int) << 4 as libc::c_int;
            }
            if f != f0 && (*s).sh_size != 0 {
                f0 = f;
                n += 1;
                n;
                f |= (1 as libc::c_int) << 8 as libc::c_int;
            }
        }
        *sec_cls.offset(i as isize) = f;
        i += 1;
        i;
    }
    return n;
}
unsafe extern "C" fn fill_phdr(
    mut ph: *mut Elf64_Phdr,
    mut type_0: libc::c_int,
    mut s: *mut Section,
) -> *mut Elf64_Phdr {
    if !s.is_null() {
        (*ph).p_offset = (*s).sh_offset;
        (*ph).p_vaddr = (*s).sh_addr;
        (*ph).p_filesz = (*s).sh_size;
        (*ph).p_align = (*s).sh_addralign as Elf64_Xword;
    }
    (*ph).p_type = type_0 as Elf64_Word;
    (*ph).p_flags = ((1 as libc::c_int) << 2 as libc::c_int) as Elf64_Word;
    (*ph).p_paddr = (*ph).p_vaddr;
    (*ph).p_memsz = (*ph).p_filesz;
    return ph;
}
unsafe extern "C" fn layout_sections(
    mut s1: *mut TCCState,
    mut sec_order: *mut libc::c_int,
    mut d: *mut dyn_inf,
) -> libc::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut addr: Elf64_Addr = 0;
    let mut tmp: Elf64_Addr = 0;
    let mut align: Elf64_Addr = 0;
    let mut s_align: Elf64_Addr = 0;
    let mut base: Elf64_Addr = 0;
    let mut ph: *mut Elf64_Phdr = 0 as *mut Elf64_Phdr;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut phnum: libc::c_int = 0;
    let mut phfill: libc::c_int = 0;
    let mut file_offset: libc::c_int = 0;
    phnum = sort_sections(s1, sec_order, d);
    phfill = 0 as libc::c_int;
    if !((*d).interp).is_null() {
        phfill = 2 as libc::c_int;
    }
    phnum += phfill;
    if !((*d).note).is_null() {
        phnum += 1;
        phnum;
    }
    if !((*d).dynamic).is_null() {
        phnum += 1;
        phnum;
    }
    if !((*s1).eh_frame_hdr_section).is_null() {
        phnum += 1;
        phnum;
    }
    if !((*d).roinf).is_null() {
        phnum += 1;
        phnum;
    }
    (*d).phnum = phnum;
    (*d)
        .phdr = tcc_mallocz(
        (phnum as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Phdr>() as libc::c_ulong),
    ) as *mut Elf64_Phdr;
    file_offset = 0 as libc::c_int;
    if (*s1).output_format == 0 as libc::c_int {
        file_offset = ((::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong)
            .wrapping_add(
                (phnum as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<Elf64_Phdr>() as libc::c_ulong),
            )
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            & -(4 as libc::c_int) as libc::c_ulong) as libc::c_int;
        file_offset = (file_offset as libc::c_ulong)
            .wrapping_add(
                ((*d).shnum as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
    }
    s_align = 0x200000 as libc::c_int as Elf64_Addr;
    if (*s1).section_align != 0 {
        s_align = (*s1).section_align as Elf64_Addr;
    }
    addr = 0x400000 as libc::c_int as Elf64_Addr;
    if (*s1).output_type & 4 as libc::c_int != 0 {
        addr = 0 as libc::c_int as Elf64_Addr;
    }
    if (*s1).has_text_addr != 0 {
        addr = (*s1).text_addr;
    }
    base = addr;
    addr = addr.wrapping_add(file_offset as Elf64_Addr);
    n = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(*sec_order.offset(i as isize) as isize);
        f = *sec_order.offset((i + (*s1).nb_sections) as isize);
        align = ((*s).sh_addralign - 1 as libc::c_int) as Elf64_Addr;
        if f == 0 as libc::c_int {
            file_offset = ((file_offset as Elf64_Addr).wrapping_add(align) & !align)
                as libc::c_int;
            (*s).sh_offset = file_offset as libc::c_ulong;
            if (*s).sh_type != 8 as libc::c_int {
                file_offset = (file_offset as libc::c_ulong).wrapping_add((*s).sh_size)
                    as libc::c_int as libc::c_int;
            }
        } else {
            if f & (1 as libc::c_int) << 8 as libc::c_int != 0 && n != 0 {
                if (*s1).output_format == 0 as libc::c_int {
                    if addr & s_align.wrapping_sub(1 as libc::c_int as Elf64_Addr)
                        != 0 as libc::c_int as Elf64_Addr
                    {
                        addr = addr.wrapping_add(s_align);
                    }
                } else {
                    align = s_align.wrapping_sub(1 as libc::c_int as Elf64_Addr);
                }
            }
            tmp = addr;
            addr = addr.wrapping_add(align) & !align;
            file_offset += addr.wrapping_sub(tmp) as libc::c_int;
            (*s).sh_offset = file_offset as libc::c_ulong;
            (*s).sh_addr = addr;
            if f & (1 as libc::c_int) << 8 as libc::c_int != 0 {
                ph = &mut *((*d).phdr).offset((phfill + n) as isize) as *mut Elf64_Phdr;
                (*ph).p_type = 1 as libc::c_int as Elf64_Word;
                (*ph).p_align = s_align;
                (*ph).p_flags = ((1 as libc::c_int) << 2 as libc::c_int) as Elf64_Word;
                if f & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                    (*ph).p_flags
                        |= ((1 as libc::c_int) << 1 as libc::c_int) as Elf64_Word;
                }
                if f & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    (*ph).p_flags
                        |= ((1 as libc::c_int) << 0 as libc::c_int) as Elf64_Word;
                }
                if f & (1 as libc::c_int) << 10 as libc::c_int != 0 {
                    (*ph).p_type = 7 as libc::c_int as Elf64_Word;
                    (*ph).p_align = align.wrapping_add(1 as libc::c_int as Elf64_Addr);
                }
                (*ph).p_offset = file_offset as Elf64_Off;
                (*ph).p_vaddr = addr;
                if n == 0 as libc::c_int {
                    (*ph).p_offset = 0 as libc::c_int as Elf64_Off;
                    (*ph).p_vaddr = base;
                }
                (*ph).p_paddr = (*ph).p_vaddr;
                n += 1;
                n;
            }
            if f & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                let mut roinf: *mut Section = &mut (*d)._roinf;
                if (*roinf).sh_size == 0 as libc::c_int as libc::c_ulong {
                    (*roinf).sh_offset = (*s).sh_offset;
                    (*roinf).sh_addr = (*s).sh_addr;
                    (*roinf).sh_addralign = 1 as libc::c_int;
                }
                (*roinf)
                    .sh_size = addr
                    .wrapping_sub((*roinf).sh_addr)
                    .wrapping_add((*s).sh_size);
            }
            addr = (addr as libc::c_ulong).wrapping_add((*s).sh_size) as Elf64_Addr
                as Elf64_Addr;
            if (*s).sh_type != 8 as libc::c_int {
                file_offset = (file_offset as libc::c_ulong).wrapping_add((*s).sh_size)
                    as libc::c_int as libc::c_int;
            }
            (*ph).p_filesz = (file_offset as Elf64_Off).wrapping_sub((*ph).p_offset);
            (*ph).p_memsz = addr.wrapping_sub((*ph).p_vaddr);
        }
        i += 1;
        i;
    }
    if !((*d).note).is_null() {
        ph = ph.offset(1);
        fill_phdr(ph, 4 as libc::c_int, (*d).note);
    }
    if !((*d).dynamic).is_null() {
        ph = ph.offset(1);
        (*fill_phdr(ph, 2 as libc::c_int, (*d).dynamic)).p_flags
            |= ((1 as libc::c_int) << 1 as libc::c_int) as Elf64_Word;
    }
    if !((*s1).eh_frame_hdr_section).is_null() {
        ph = ph.offset(1);
        fill_phdr(ph, 0x6474e550 as libc::c_int, (*s1).eh_frame_hdr_section);
    }
    if !((*d).roinf).is_null() {
        ph = ph.offset(1);
        (*fill_phdr(ph, 0x6474e552 as libc::c_int, (*d).roinf)).p_flags
            |= ((1 as libc::c_int) << 1 as libc::c_int) as Elf64_Word;
    }
    if !((*d).interp).is_null() {
        fill_phdr(
            &mut *((*d).phdr).offset(1 as libc::c_int as isize),
            3 as libc::c_int,
            (*d).interp,
        );
    }
    if phfill != 0 {
        ph = &mut *((*d).phdr).offset(0 as libc::c_int as isize) as *mut Elf64_Phdr;
        (*ph).p_offset = ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong;
        (*ph).p_vaddr = base.wrapping_add((*ph).p_offset);
        (*ph)
            .p_filesz = (phnum as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Elf64_Phdr>() as libc::c_ulong);
        (*ph).p_align = 4 as libc::c_int as Elf64_Xword;
        fill_phdr(ph, 6 as libc::c_int, 0 as *mut Section);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn put_dt(
    mut dynamic: *mut Section,
    mut dt: libc::c_int,
    mut val: Elf64_Addr,
) {
    let mut dyn_0: *mut Elf64_Dyn = 0 as *mut Elf64_Dyn;
    dyn_0 = section_ptr_add(
        dynamic,
        ::core::mem::size_of::<Elf64_Dyn>() as libc::c_ulong,
    ) as *mut Elf64_Dyn;
    (*dyn_0).d_tag = dt as Elf64_Sxword;
    (*dyn_0).d_un.d_val = val;
}
unsafe extern "C" fn fill_dynamic(mut s1: *mut TCCState, mut dyninf: *mut dyn_inf) {
    let mut dynamic: *mut Section = (*dyninf).dynamic;
    let mut s: *mut Section = 0 as *mut Section;
    put_dt(dynamic, 4 as libc::c_int, (*(*(*s1).dynsym).hash).sh_addr);
    put_dt(dynamic, 0x6ffffef5 as libc::c_int, (*(*dyninf).gnu_hash).sh_addr);
    put_dt(dynamic, 5 as libc::c_int, (*(*dyninf).dynstr).sh_addr);
    put_dt(dynamic, 6 as libc::c_int, (*(*s1).dynsym).sh_addr);
    put_dt(dynamic, 10 as libc::c_int, (*(*dyninf).dynstr).data_offset);
    put_dt(
        dynamic,
        11 as libc::c_int,
        ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong,
    );
    put_dt(dynamic, 7 as libc::c_int, (*dyninf).c2rust_unnamed.rel_addr);
    put_dt(dynamic, 8 as libc::c_int, (*dyninf).c2rust_unnamed.rel_size);
    put_dt(
        dynamic,
        9 as libc::c_int,
        ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong,
    );
    if !((*s1).plt).is_null() && !((*(*s1).plt).reloc).is_null() {
        put_dt(dynamic, 3 as libc::c_int, (*(*s1).got).sh_addr);
        put_dt(dynamic, 2 as libc::c_int, (*(*(*s1).plt).reloc).data_offset);
        put_dt(dynamic, 23 as libc::c_int, (*(*(*s1).plt).reloc).sh_addr);
        put_dt(dynamic, 20 as libc::c_int, 7 as libc::c_int as Elf64_Addr);
    }
    put_dt(dynamic, 0x6ffffff9 as libc::c_int, 0 as libc::c_int as Elf64_Addr);
    if !((*s1).versym_section).is_null() && !((*s1).verneed_section).is_null() {
        put_dt(dynamic, 0x6ffffff0 as libc::c_int, (*(*s1).versym_section).sh_addr);
        put_dt(dynamic, 0x6ffffffe as libc::c_int, (*(*s1).verneed_section).sh_addr);
        put_dt(dynamic, 0x6fffffff as libc::c_int, (*s1).dt_verneednum as Elf64_Addr);
    }
    s = have_section(s1, b".preinit_array\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 32 as libc::c_int, (*s).sh_addr);
        put_dt(dynamic, 33 as libc::c_int, (*s).data_offset);
    }
    s = have_section(s1, b".init_array\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 25 as libc::c_int, (*s).sh_addr);
        put_dt(dynamic, 27 as libc::c_int, (*s).data_offset);
    }
    s = have_section(s1, b".fini_array\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 26 as libc::c_int, (*s).sh_addr);
        put_dt(dynamic, 28 as libc::c_int, (*s).data_offset);
    }
    s = have_section(s1, b".init\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 12 as libc::c_int, (*s).sh_addr);
    }
    s = have_section(s1, b".fini\0" as *const u8 as *const libc::c_char);
    if !s.is_null() && (*s).data_offset != 0 {
        put_dt(dynamic, 13 as libc::c_int, (*s).sh_addr);
    }
    if (*s1).do_debug != 0 {
        put_dt(dynamic, 21 as libc::c_int, 0 as libc::c_int as Elf64_Addr);
    }
    put_dt(dynamic, 0 as libc::c_int, 0 as libc::c_int as Elf64_Addr);
}
unsafe extern "C" fn update_reloc_sections(
    mut s1: *mut TCCState,
    mut dyninf: *mut dyn_inf,
) {
    let mut i: libc::c_int = 0;
    let mut file_offset: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut s: *mut Section = 0 as *mut Section;
    let mut relocplt: *mut Section = if !((*s1).plt).is_null() {
        (*(*s1).plt).reloc
    } else {
        0 as *mut Section
    };
    (*dyninf).c2rust_unnamed.rel_size = 0 as libc::c_int as Elf64_Addr;
    (*dyninf).c2rust_unnamed.rel_addr = (*dyninf).c2rust_unnamed.rel_size;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if (*s).sh_type == 4 as libc::c_int && s != relocplt {
            if (*dyninf).c2rust_unnamed.rel_size == 0 as libc::c_int as Elf64_Addr {
                (*dyninf).c2rust_unnamed.rel_addr = (*s).sh_addr;
                file_offset = (*s).sh_offset;
            } else {
                (*s)
                    .sh_addr = ((*dyninf).c2rust_unnamed.rel_addr)
                    .wrapping_add((*dyninf).c2rust_unnamed.rel_size);
                (*s)
                    .sh_offset = file_offset
                    .wrapping_add((*dyninf).c2rust_unnamed.rel_size);
            }
            (*dyninf)
                .c2rust_unnamed
                .rel_size = ((*dyninf).c2rust_unnamed.rel_size as libc::c_ulong)
                .wrapping_add((*s).sh_size) as Elf64_Addr as Elf64_Addr;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn tcc_output_elf(
    mut s1: *mut TCCState,
    mut f: *mut FILE,
    mut phnum: libc::c_int,
    mut phdr: *mut Elf64_Phdr,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut shnum: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut file_type: libc::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    let mut shdr: Elf64_Shdr = Elf64_Shdr {
        sh_name: 0,
        sh_type: 0,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: 0,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };
    let mut sh: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    file_type = (*s1).output_type;
    shnum = (*s1).nb_sections;
    memset(
        &mut ehdr as *mut Elf64_Ehdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong,
    );
    if phnum > 0 as libc::c_int {
        ehdr
            .e_phentsize = ::core::mem::size_of::<Elf64_Phdr>() as libc::c_ulong
            as Elf64_Half;
        ehdr.e_phnum = phnum as Elf64_Half;
        ehdr.e_phoff = ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong;
    }
    ehdr.e_ident[0 as libc::c_int as usize] = 0x7f as libc::c_int as libc::c_uchar;
    ehdr.e_ident[1 as libc::c_int as usize] = 'E' as i32 as libc::c_uchar;
    ehdr.e_ident[2 as libc::c_int as usize] = 'L' as i32 as libc::c_uchar;
    ehdr.e_ident[3 as libc::c_int as usize] = 'F' as i32 as libc::c_uchar;
    ehdr.e_ident[4 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    ehdr.e_ident[5 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    ehdr.e_ident[6 as libc::c_int as usize] = 1 as libc::c_int as libc::c_uchar;
    if file_type == 3 as libc::c_int {
        ehdr.e_type = 1 as libc::c_int as Elf64_Half;
    } else {
        if file_type & 4 as libc::c_int != 0 {
            ehdr.e_type = 3 as libc::c_int as Elf64_Half;
        } else {
            ehdr.e_type = 2 as libc::c_int as Elf64_Half;
        }
        if !((*s1).elf_entryname).is_null() {
            ehdr
                .e_entry = get_sym_addr(
                s1,
                (*s1).elf_entryname,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            ehdr
                .e_entry = get_sym_addr(
                s1,
                b"_start\0" as *const u8 as *const libc::c_char,
                (file_type & 2 as libc::c_int != 0) as libc::c_int,
                0 as libc::c_int,
            );
        }
        if ehdr.e_entry == -(1 as libc::c_int) as Elf64_Addr {
            ehdr.e_entry = (*(*s1).text_section).sh_addr;
        }
        if (*s1).nb_errors != 0 {
            return -(1 as libc::c_int);
        }
    }
    sort_syms(s1, (*s1).c2rust_unnamed.symtab);
    ehdr.e_machine = 62 as libc::c_int as Elf64_Half;
    ehdr.e_version = 1 as libc::c_int as Elf64_Word;
    ehdr
        .e_shoff = (::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong)
        .wrapping_add(
            (phnum as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Elf64_Phdr>() as libc::c_ulong),
        )
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        & -(4 as libc::c_int) as libc::c_ulong;
    ehdr.e_ehsize = ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong as Elf64_Half;
    ehdr
        .e_shentsize = ::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong
        as Elf64_Half;
    ehdr.e_shnum = shnum as Elf64_Half;
    ehdr.e_shstrndx = (shnum - 1 as libc::c_int) as Elf64_Half;
    offset = fwrite(
        &mut ehdr as *mut Elf64_Ehdr as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong,
        f,
    ) as libc::c_int;
    if !phdr.is_null() {
        offset = (offset as libc::c_ulong)
            .wrapping_add(
                fwrite(
                    phdr as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    (phnum as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<Elf64_Phdr>() as libc::c_ulong,
                        ),
                    f,
                ),
            ) as libc::c_int as libc::c_int;
    }
    while (offset as Elf64_Off) < ehdr.e_shoff {
        fputc(0 as libc::c_int, f);
        offset += 1;
        offset;
    }
    i = 0 as libc::c_int;
    while i < shnum {
        sh = &mut shdr;
        memset(
            sh as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong,
        );
        if i != 0 {
            s = *((*s1).sections).offset(i as isize);
            (*sh).sh_name = (*s).sh_name as Elf64_Word;
            (*sh).sh_type = (*s).sh_type as Elf64_Word;
            (*sh).sh_flags = (*s).sh_flags as Elf64_Xword;
            (*sh).sh_entsize = (*s).sh_entsize as Elf64_Xword;
            (*sh).sh_info = (*s).sh_info as Elf64_Word;
            if !((*s).link).is_null() {
                (*sh).sh_link = (*(*s).link).sh_num as Elf64_Word;
            }
            (*sh).sh_addralign = (*s).sh_addralign as Elf64_Xword;
            (*sh).sh_addr = (*s).sh_addr;
            (*sh).sh_offset = (*s).sh_offset;
            (*sh).sh_size = (*s).sh_size;
        }
        offset = (offset as libc::c_ulong)
            .wrapping_add(
                fwrite(
                    sh as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong,
                    f,
                ),
            ) as libc::c_int as libc::c_int;
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if (*s).sh_type != 8 as libc::c_int {
            while (offset as libc::c_ulong) < (*s).sh_offset {
                fputc(0 as libc::c_int, f);
                offset += 1;
                offset;
            }
            size = (*s).sh_size as libc::c_int;
            if size != 0 {
                offset = (offset as libc::c_ulong)
                    .wrapping_add(
                        fwrite(
                            (*s).data as *const libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            size as libc::c_ulong,
                            f,
                        ),
                    ) as libc::c_int as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcc_output_binary(
    mut s1: *mut TCCState,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    offset = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if (*s).sh_type != 8 as libc::c_int
            && (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            while (offset as libc::c_ulong) < (*s).sh_offset {
                fputc(0 as libc::c_int, f);
                offset += 1;
                offset;
            }
            size = (*s).sh_size as libc::c_int;
            fwrite(
                (*s).data as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                size as libc::c_ulong,
                f,
            );
            offset += size;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tcc_write_elf_file(
    mut s1: *mut TCCState,
    mut filename: *const libc::c_char,
    mut phnum: libc::c_int,
    mut phdr: *mut Elf64_Phdr,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut file_type: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    file_type = (*s1).output_type;
    if file_type == 3 as libc::c_int {
        mode = 0o666 as libc::c_int;
    } else {
        mode = 0o777 as libc::c_int;
    }
    unlink(filename);
    fd = open(
        filename,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int
            | 0 as libc::c_int,
        mode,
    );
    if fd < 0 as libc::c_int
        || {
            f = fdopen(fd, b"wb\0" as *const u8 as *const libc::c_char);
            f.is_null()
        }
    {
        tcc_enter_state(s1);
        return (Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
        ))
            .expect(
                "non-null function pointer",
            )(
            b"could not write '%s: %s'\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    if (*s1).verbose != 0 {
        printf(b"<- %s\n\0" as *const u8 as *const libc::c_char, filename);
    }
    if (*s1).output_format == 0 as libc::c_int {
        ret = tcc_output_elf(s1, f, phnum, phdr);
    } else {
        ret = tcc_output_binary(s1, f);
    }
    fclose(f);
    return ret;
}
unsafe extern "C" fn reorder_sections(
    mut s1: *mut TCCState,
    mut sec_order: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut nnew: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut backmap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut snew: *mut *mut Section = 0 as *mut *mut Section;
    let mut s: *mut Section = 0 as *mut Section;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    backmap = tcc_malloc(
        ((*s1).nb_sections as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    nnew = 0 as libc::c_int;
    snew = 0 as *mut *mut Section;
    while i < (*s1).nb_sections {
        k = *sec_order.offset(i as isize);
        s = *((*s1).sections).offset(k as isize);
        if i == 0 || (*s).sh_name != 0 {
            *backmap.offset(k as isize) = nnew;
            dynarray_add(
                &mut snew as *mut *mut *mut Section as *mut libc::c_void,
                &mut nnew,
                s as *mut libc::c_void,
            );
        } else {
            *backmap.offset(k as isize) = 0 as libc::c_int;
            dynarray_add(
                &mut (*s1).priv_sections as *mut *mut *mut Section as *mut libc::c_void,
                &mut (*s1).nb_priv_sections,
                s as *mut libc::c_void,
            );
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < nnew {
        s = *snew.offset(i as isize);
        (*s).sh_num = i;
        if (*s).sh_type == 4 as libc::c_int {
            (*s).sh_info = *backmap.offset((*s).sh_info as isize);
        } else if (*s).sh_type == 2 as libc::c_int || (*s).sh_type == 11 as libc::c_int {
            sym = ((*s).data as *mut Elf64_Sym).offset(1 as libc::c_int as isize);
            while sym < ((*s).data).offset((*s).data_offset as isize) as *mut Elf64_Sym {
                if ((*sym).st_shndx as libc::c_int) < (*s1).nb_sections {
                    (*sym)
                        .st_shndx = *backmap.offset((*sym).st_shndx as isize)
                        as Elf64_Section;
                }
                sym = sym.offset(1);
                sym;
            }
        }
        i += 1;
        i;
    }
    tcc_free((*s1).sections as *mut libc::c_void);
    (*s1).sections = snew;
    (*s1).nb_sections = nnew;
    tcc_free(backmap as *mut libc::c_void);
}
unsafe extern "C" fn elf_output_file(
    mut s1: *mut TCCState,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut file_type: libc::c_int = 0;
    let mut sec_order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dyninf: dyn_inf = {
        let mut init = dyn_inf {
            dynamic: 0 as *mut Section,
            dynstr: 0 as *mut Section,
            c2rust_unnamed: C2RustUnnamed_4 {
                data_offset: 0,
                rel_addr: 0,
                rel_size: 0,
            },
            phdr: 0 as *mut Elf64_Phdr,
            phnum: 0,
            shnum: 0,
            interp: 0 as *mut Section,
            note: 0 as *mut Section,
            gnu_hash: 0 as *mut Section,
            _roinf: Section {
                data_offset: 0,
                data: 0 as *mut libc::c_uchar,
                data_allocated: 0,
                s1: 0 as *mut TCCState,
                sh_name: 0,
                sh_num: 0,
                sh_type: 0,
                sh_flags: 0,
                sh_info: 0,
                sh_addralign: 0,
                sh_entsize: 0,
                sh_size: 0,
                sh_addr: 0,
                sh_offset: 0,
                nb_hashed_syms: 0,
                link: 0 as *mut Section,
                reloc: 0 as *mut Section,
                hash: 0 as *mut Section,
                prev: 0 as *mut Section,
                name: [0; 1],
            },
            roinf: 0 as *mut Section,
        };
        init
    };
    let mut interp: *mut Section = 0 as *mut Section;
    let mut dynstr: *mut Section = 0 as *mut Section;
    let mut dynamic: *mut Section = 0 as *mut Section;
    let mut textrel: libc::c_int = 0;
    let mut got_sym: libc::c_int = 0;
    let mut dt_flags_1: libc::c_int = 0;
    file_type = (*s1).output_type;
    (*s1).nb_errors = 0 as libc::c_int;
    ret = -(1 as libc::c_int);
    dynamic = 0 as *mut Section;
    dynstr = dynamic;
    interp = dynstr;
    sec_order = 0 as *mut libc::c_int;
    dyninf.roinf = &mut dyninf._roinf;
    tcc_add_runtime(s1);
    resolve_common_syms(s1);
    if (*s1).static_link == 0 {
        if file_type & 2 as libc::c_int != 0 {
            let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut elfint: *const libc::c_char = (*s1).elfint;
            if elfint.is_null() {
                elfint = getenv(b"LD_SO\0" as *const u8 as *const libc::c_char);
            }
            if elfint.is_null() {
                elfint = b"/lib64/ld-linux-x86-64.so.2\0" as *const u8
                    as *const libc::c_char;
            }
            interp = new_section(
                s1,
                b".interp\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                (1 as libc::c_int) << 1 as libc::c_int,
            );
            (*interp).sh_addralign = 1 as libc::c_int;
            ptr = section_ptr_add(
                interp,
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(elfint)),
            ) as *mut libc::c_char;
            strcpy(ptr, elfint);
            dyninf.interp = interp;
        }
        (*s1)
            .dynsym = new_symtab(
            s1,
            b".dynsym\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int,
            b".dynstr\0" as *const u8 as *const libc::c_char,
            b".hash\0" as *const u8 as *const libc::c_char,
            (1 as libc::c_int) << 1 as libc::c_int,
        );
        (*(*s1).dynsym).sh_info = 1 as libc::c_int;
        dynstr = (*(*s1).dynsym).link;
        dynamic = new_section(
            s1,
            b".dynamic\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        );
        (*dynamic).link = dynstr;
        (*dynamic)
            .sh_entsize = ::core::mem::size_of::<Elf64_Dyn>() as libc::c_ulong
            as libc::c_int;
        got_sym = build_got(s1);
        if file_type & 2 as libc::c_int != 0 {
            bind_exe_dynsyms(s1, file_type & 4 as libc::c_int);
            if (*s1).nb_errors != 0 {
                current_block = 16131944088842856154;
            } else {
                current_block = 14401909646449704462;
            }
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            16131944088842856154 => {}
            _ => {
                build_got_entries(s1, got_sym);
                if file_type & 2 as libc::c_int != 0 {
                    bind_libs_dynsyms(s1);
                } else {
                    export_global_syms(s1);
                }
                tcc_eh_frame_hdr(s1, 0 as libc::c_int);
                dyninf.gnu_hash = create_gnu_hash(s1);
                current_block = 11636175345244025579;
            }
        }
    } else {
        build_got_entries(s1, 0 as libc::c_int);
        current_block = 11636175345244025579;
    }
    match current_block {
        11636175345244025579 => {
            version_add(s1);
            textrel = set_sec_sizes(s1);
            if (*s1).static_link == 0 {
                i = 0 as libc::c_int;
                while i < (*s1).nb_loaded_dlls {
                    let mut dllref: *mut DLLReference = *((*s1).loaded_dlls)
                        .offset(i as isize);
                    if (*dllref).level == 0 as libc::c_int {
                        put_dt(
                            dynamic,
                            1 as libc::c_int,
                            put_elf_str(dynstr, ((*dllref).name).as_mut_ptr())
                                as Elf64_Addr,
                        );
                    }
                    i += 1;
                    i;
                }
                if !((*s1).rpath).is_null() {
                    put_dt(
                        dynamic,
                        if (*s1).enable_new_dtags as libc::c_int != 0 {
                            29 as libc::c_int
                        } else {
                            15 as libc::c_int
                        },
                        put_elf_str(dynstr, (*s1).rpath) as Elf64_Addr,
                    );
                }
                dt_flags_1 = 0x1 as libc::c_int;
                if file_type & 4 as libc::c_int != 0 {
                    if !((*s1).soname).is_null() {
                        put_dt(
                            dynamic,
                            14 as libc::c_int,
                            put_elf_str(dynstr, (*s1).soname) as Elf64_Addr,
                        );
                    }
                    if textrel != 0 {
                        put_dt(
                            dynamic,
                            22 as libc::c_int,
                            0 as libc::c_int as Elf64_Addr,
                        );
                    }
                    if file_type & 2 as libc::c_int != 0 {
                        dt_flags_1 = 0x1 as libc::c_int | 0x8000000 as libc::c_int;
                    }
                }
                put_dt(dynamic, 30 as libc::c_int, 0x8 as libc::c_int as Elf64_Addr);
                put_dt(dynamic, 0x6ffffffb as libc::c_int, dt_flags_1 as Elf64_Addr);
                if (*s1).symbolic != 0 {
                    put_dt(dynamic, 16 as libc::c_int, 0 as libc::c_int as Elf64_Addr);
                }
                dyninf.dynamic = dynamic;
                dyninf.dynstr = dynstr;
                dyninf.c2rust_unnamed.data_offset = (*dynamic).data_offset;
                fill_dynamic(s1, &mut dyninf);
                (*dynamic).sh_size = (*dynamic).data_offset;
                (*dynstr).sh_size = (*dynstr).data_offset;
            }
            alloc_sec_names(s1, 0 as libc::c_int);
            sec_order = tcc_malloc(
                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul((*s1).nb_sections as libc::c_ulong),
            ) as *mut libc::c_int;
            layout_sections(s1, sec_order, &mut dyninf);
            if !dynamic.is_null() {
                write32le((*(*s1).got).data, (*dynamic).sh_addr as uint32_t);
                if file_type == 2 as libc::c_int
                    || 1 as libc::c_int != 0 && file_type & 4 as libc::c_int != 0
                {
                    relocate_plt(s1);
                }
                relocate_syms(s1, (*s1).dynsym, 2 as libc::c_int);
            }
            relocate_syms(s1, (*s1).c2rust_unnamed.symtab, 0 as libc::c_int);
            if !((*s1).nb_errors != 0 as libc::c_int) {
                relocate_sections(s1);
                if !dynamic.is_null() {
                    update_reloc_sections(s1, &mut dyninf);
                    (*dynamic).data_offset = dyninf.c2rust_unnamed.data_offset;
                    fill_dynamic(s1, &mut dyninf);
                }
                if file_type == 2 as libc::c_int && (*s1).static_link as libc::c_int != 0
                {
                    fill_got(s1);
                } else if !((*s1).got).is_null() {
                    fill_local_got_entries(s1);
                }
                if !(dyninf.gnu_hash).is_null() {
                    update_gnu_hash(s1, dyninf.gnu_hash);
                }
                reorder_sections(s1, sec_order);
                tcc_eh_frame_hdr(s1, 1 as libc::c_int);
                ret = tcc_write_elf_file(s1, filename, dyninf.phnum, dyninf.phdr);
            }
        }
        _ => {}
    }
    tcc_free(sec_order as *mut libc::c_void);
    tcc_free(dyninf.phdr as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn alloc_sec_names(mut s1: *mut TCCState, mut is_obj: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut s: *mut Section = 0 as *mut Section;
    let mut strsec: *mut Section = 0 as *mut Section;
    strsec = new_section(
        s1,
        b".shstrtab\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        0 as libc::c_int,
    );
    put_elf_str(strsec, b"\0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        if is_obj != 0 {
            (*s).sh_size = (*s).data_offset;
        }
        if (*s).sh_size != 0 || s == strsec
            || (*s).sh_flags & (1 as libc::c_int) << 1 as libc::c_int != 0 || is_obj != 0
        {
            (*s).sh_name = put_elf_str(strsec, ((*s).name).as_mut_ptr());
        }
        i += 1;
        i;
    }
    (*strsec).sh_size = (*strsec).data_offset;
}
unsafe extern "C" fn elf_output_obj(
    mut s1: *mut TCCState,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut file_offset: libc::c_int = 0;
    (*s1).nb_errors = 0 as libc::c_int;
    alloc_sec_names(s1, 1 as libc::c_int);
    file_offset = ((::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        & -(4 as libc::c_int) as libc::c_ulong) as libc::c_int;
    file_offset = (file_offset as libc::c_ulong)
        .wrapping_add(
            ((*s1).nb_sections as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*s1).nb_sections {
        s = *((*s1).sections).offset(i as isize);
        file_offset = file_offset + 15 as libc::c_int & -(16 as libc::c_int);
        (*s).sh_offset = file_offset as libc::c_ulong;
        if (*s).sh_type != 8 as libc::c_int {
            file_offset = (file_offset as libc::c_ulong).wrapping_add((*s).sh_size)
                as libc::c_int as libc::c_int;
        }
        i += 1;
        i;
    }
    ret = tcc_write_elf_file(s1, filename, 0 as libc::c_int, 0 as *mut Elf64_Phdr);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_output_file(
    mut s: *mut TCCState,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if (*s).test_coverage != 0 {
        tcc_tcov_add_file(s, filename);
    }
    if (*s).output_type == 3 as libc::c_int {
        return elf_output_obj(s, filename);
    }
    return elf_output_file(s, filename);
}
#[no_mangle]
pub unsafe extern "C" fn full_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut cbuf: *mut libc::c_char = buf as *mut libc::c_char;
    let mut rnum: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut num: ssize_t = read(
            fd,
            cbuf as *mut libc::c_void,
            count.wrapping_sub(rnum),
        );
        if num < 0 as libc::c_int as ssize_t {
            return num;
        }
        if num == 0 as libc::c_int as ssize_t {
            return rnum as ssize_t;
        }
        rnum = rnum.wrapping_add(num as size_t);
        cbuf = cbuf.offset(num as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_data(
    mut fd: libc::c_int,
    mut file_offset: libc::c_ulong,
    mut size: libc::c_ulong,
) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    data = tcc_malloc(size);
    lseek(fd, file_offset as __off_t, 0 as libc::c_int);
    full_read(fd, data, size);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_object_type(
    mut fd: libc::c_int,
    mut h: *mut Elf64_Ehdr,
) -> libc::c_int {
    let mut size: libc::c_int = full_read(
        fd,
        h as *mut libc::c_void,
        ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong,
    ) as libc::c_int;
    if size as libc::c_ulong == ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong
        && 0 as libc::c_int
            == memcmp(
                h as *const libc::c_void,
                b"\x7FELF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            )
    {
        if (*h).e_type as libc::c_int == 1 as libc::c_int {
            return 1 as libc::c_int;
        }
        if (*h).e_type as libc::c_int == 3 as libc::c_int {
            return 2 as libc::c_int;
        }
    } else if size >= 8 as libc::c_int {
        if 0 as libc::c_int
            == memcmp(
                h as *const libc::c_void,
                b"!<arch>\n\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            )
        {
            return 3 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_load_object_file(
    mut s1: *mut TCCState,
    mut fd: libc::c_int,
    mut file_offset: libc::c_ulong,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    let mut shdr: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sh: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut size: libc::c_ulong = 0;
    let mut offset: libc::c_ulong = 0;
    let mut offseti: libc::c_ulong = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nb_syms: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut seencompressed: libc::c_int = 0;
    let mut strsec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stab_index: libc::c_int = 0;
    let mut stabstr_index: libc::c_int = 0;
    let mut old_to_new_syms: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sh_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sm_table: *mut SectionMergeInfo = 0 as *mut SectionMergeInfo;
    let mut sm: *mut SectionMergeInfo = 0 as *mut SectionMergeInfo;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut symtab: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut s: *mut Section = 0 as *mut Section;
    lseek(fd, file_offset as __off_t, 0 as libc::c_int);
    if !(tcc_object_type(fd, &mut ehdr) != 1 as libc::c_int) {
        if !(ehdr.e_ident[5 as libc::c_int as usize] as libc::c_int != 1 as libc::c_int
            || ehdr.e_machine as libc::c_int != 62 as libc::c_int)
        {
            shdr = load_data(
                fd,
                file_offset.wrapping_add(ehdr.e_shoff),
                (::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong)
                    .wrapping_mul(ehdr.e_shnum as libc::c_ulong),
            ) as *mut Elf64_Shdr;
            sm_table = tcc_mallocz(
                (::core::mem::size_of::<SectionMergeInfo>() as libc::c_ulong)
                    .wrapping_mul(ehdr.e_shnum as libc::c_ulong),
            ) as *mut SectionMergeInfo;
            sh = &mut *shdr.offset(ehdr.e_shstrndx as isize) as *mut Elf64_Shdr;
            strsec = load_data(
                fd,
                file_offset.wrapping_add((*sh).sh_offset),
                (*sh).sh_size,
            ) as *mut libc::c_char;
            old_to_new_syms = 0 as *mut libc::c_int;
            symtab = 0 as *mut Elf64_Sym;
            strtab = 0 as *mut libc::c_char;
            nb_syms = 0 as libc::c_int;
            seencompressed = 0 as libc::c_int;
            stabstr_index = 0 as libc::c_int;
            stab_index = stabstr_index;
            ret = -(1 as libc::c_int);
            i = 1 as libc::c_int;
            loop {
                if !(i < ehdr.e_shnum as libc::c_int) {
                    current_block = 6669252993407410313;
                    break;
                }
                sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                if (*sh).sh_type == 2 as libc::c_int as Elf64_Word {
                    if !symtab.is_null() {
                        tcc_enter_state(s1);
                        (Some(
                            _tcc_error_noabort
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    ...
                                ) -> libc::c_int,
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            b"object must contain only one symtab\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block = 11719220873777950690;
                        break;
                    } else {
                        nb_syms = ((*sh).sh_size)
                            .wrapping_div(
                                ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong,
                            ) as libc::c_int;
                        symtab = load_data(
                            fd,
                            file_offset.wrapping_add((*sh).sh_offset),
                            (*sh).sh_size,
                        ) as *mut Elf64_Sym;
                        let ref mut fresh12 = (*sm_table.offset(i as isize)).s;
                        *fresh12 = (*s1).c2rust_unnamed.symtab_section;
                        sh = &mut *shdr.offset((*sh).sh_link as isize)
                            as *mut Elf64_Shdr;
                        strtab = load_data(
                            fd,
                            file_offset.wrapping_add((*sh).sh_offset),
                            (*sh).sh_size,
                        ) as *mut libc::c_char;
                    }
                }
                if (*sh).sh_flags
                    & ((1 as libc::c_int) << 11 as libc::c_int) as Elf64_Xword != 0
                {
                    seencompressed = 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            match current_block {
                6669252993407410313 => {
                    i = 1 as libc::c_int;
                    's_132: loop {
                        if !(i < ehdr.e_shnum as libc::c_int) {
                            current_block = 5028470053297453708;
                            break;
                        }
                        if !(i == ehdr.e_shstrndx as libc::c_int) {
                            sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                            if (*sh).sh_type == 4 as libc::c_int as Elf64_Word {
                                sh = &mut *shdr.offset((*sh).sh_info as isize)
                                    as *mut Elf64_Shdr;
                            }
                            sh_name = strsec.offset((*sh).sh_name as isize);
                            if 0 as libc::c_int
                                == strncmp(
                                    sh_name,
                                    b".debug_\0" as *const u8 as *const libc::c_char,
                                    7 as libc::c_int as libc::c_ulong,
                                )
                                || 0 as libc::c_int
                                    == strncmp(
                                        sh_name,
                                        b".stab\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int as libc::c_ulong,
                                    )
                            {
                                if (*s1).do_debug == 0 || seencompressed != 0 {
                                    current_block = 7056779235015430508;
                                } else {
                                    current_block = 15597372965620363352;
                                }
                            } else if 0 as libc::c_int
                                == strncmp(
                                    sh_name,
                                    b".eh_frame\0" as *const u8 as *const libc::c_char,
                                    9 as libc::c_int as libc::c_ulong,
                                )
                            {
                                if ((*s1).eh_frame_section).is_null() {
                                    current_block = 7056779235015430508;
                                } else {
                                    current_block = 15597372965620363352;
                                }
                            } else if (*sh).sh_type != 1 as libc::c_int as Elf64_Word
                                && (*sh).sh_type != 7 as libc::c_int as Elf64_Word
                                && (*sh).sh_type != 8 as libc::c_int as Elf64_Word
                                && (*sh).sh_type != 16 as libc::c_int as Elf64_Word
                                && (*sh).sh_type != 14 as libc::c_int as Elf64_Word
                                && (*sh).sh_type != 15 as libc::c_int as Elf64_Word
                            {
                                current_block = 7056779235015430508;
                            } else {
                                current_block = 15597372965620363352;
                            }
                            match current_block {
                                7056779235015430508 => {}
                                _ => {
                                    sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                    sh_name = strsec.offset((*sh).sh_name as isize);
                                    if (*sh).sh_addralign < 1 as libc::c_int as Elf64_Xword {
                                        (*sh).sh_addralign = 1 as libc::c_int as Elf64_Xword;
                                    }
                                    j = 1 as libc::c_int;
                                    loop {
                                        if !(j < (*s1).nb_sections) {
                                            current_block = 2290177392965769716;
                                            break;
                                        }
                                        s = *((*s1).sections).offset(j as isize);
                                        if strcmp(((*s).name).as_mut_ptr(), sh_name) != 0 {
                                            j += 1;
                                            j;
                                        } else if (*sh).sh_type != (*s).sh_type as Elf64_Word
                                            && strcmp(
                                                ((*s).name).as_mut_ptr(),
                                                b".eh_frame\0" as *const u8 as *const libc::c_char,
                                            ) != 0
                                        {
                                            tcc_enter_state(s1);
                                            (Some(
                                                _tcc_error_noabort
                                                    as unsafe extern "C" fn(
                                                        *const libc::c_char,
                                                        ...
                                                    ) -> libc::c_int,
                                            ))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                b"section type conflict: %s %02x <> %02x\0" as *const u8
                                                    as *const libc::c_char,
                                                ((*s).name).as_mut_ptr(),
                                                (*sh).sh_type,
                                                (*s).sh_type,
                                            );
                                            current_block = 11719220873777950690;
                                            break 's_132;
                                        } else if strncmp(
                                            sh_name,
                                            b".gnu.linkonce\0" as *const u8 as *const libc::c_char,
                                            13 as libc::c_int as libc::c_ulong,
                                        ) == 0
                                        {
                                            (*sm_table.offset(i as isize))
                                                .link_once = 1 as libc::c_int as uint8_t;
                                            current_block = 7056779235015430508;
                                            break;
                                        } else {
                                            if !((*s1).stab_section).is_null() {
                                                if s == (*s1).stab_section {
                                                    stab_index = i;
                                                }
                                                if s == (*(*s1).stab_section).link {
                                                    stabstr_index = i;
                                                }
                                            }
                                            current_block = 5056262341453043922;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        7056779235015430508 => {}
                                        _ => {
                                            match current_block {
                                                2290177392965769716 => {
                                                    s = new_section(
                                                        s1,
                                                        sh_name,
                                                        (*sh).sh_type as libc::c_int,
                                                        ((*sh).sh_flags
                                                            & !((1 as libc::c_int) << 9 as libc::c_int) as Elf64_Xword)
                                                            as libc::c_int,
                                                    );
                                                    (*s).sh_addralign = (*sh).sh_addralign as libc::c_int;
                                                    (*s).sh_entsize = (*sh).sh_entsize as libc::c_int;
                                                    (*sm_table.offset(i as isize))
                                                        .new_section = 1 as libc::c_int as uint8_t;
                                                }
                                                _ => {}
                                            }
                                            (*s)
                                                .data_offset = ((*s).data_offset)
                                                .wrapping_add(
                                                    ((*s).data_offset).wrapping_neg()
                                                        & ((*sh).sh_addralign)
                                                            .wrapping_sub(1 as libc::c_int as Elf64_Xword),
                                                );
                                            if (*sh).sh_addralign > (*s).sh_addralign as Elf64_Xword {
                                                (*s).sh_addralign = (*sh).sh_addralign as libc::c_int;
                                            }
                                            (*sm_table.offset(i as isize)).offset = (*s).data_offset;
                                            let ref mut fresh13 = (*sm_table.offset(i as isize)).s;
                                            *fresh13 = s;
                                            size = (*sh).sh_size;
                                            if (*sh).sh_type != 8 as libc::c_int as Elf64_Word {
                                                let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                                lseek(
                                                    fd,
                                                    file_offset.wrapping_add((*sh).sh_offset) as __off_t,
                                                    0 as libc::c_int,
                                                );
                                                ptr = section_ptr_add(s, size) as *mut libc::c_uchar;
                                                full_read(fd, ptr as *mut libc::c_void, size);
                                            } else {
                                                (*s).data_offset = ((*s).data_offset).wrapping_add(size);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        i += 1;
                        i;
                    }
                    match current_block {
                        11719220873777950690 => {}
                        _ => {
                            if stab_index != 0 && stabstr_index != 0 {
                                let mut a: *mut Stab_Sym = 0 as *mut Stab_Sym;
                                let mut b: *mut Stab_Sym = 0 as *mut Stab_Sym;
                                let mut o: libc::c_uint = 0;
                                s = (*sm_table.offset(stab_index as isize)).s;
                                a = ((*s).data)
                                    .offset(
                                        (*sm_table.offset(stab_index as isize)).offset as isize,
                                    ) as *mut Stab_Sym;
                                b = ((*s).data).offset((*s).data_offset as isize)
                                    as *mut Stab_Sym;
                                o = (*sm_table.offset(stabstr_index as isize)).offset
                                    as libc::c_uint;
                                while a < b {
                                    if (*a).n_strx != 0 {
                                        (*a).n_strx = ((*a).n_strx).wrapping_add(o);
                                    }
                                    a = a.offset(1);
                                    a;
                                }
                            }
                            i = 1 as libc::c_int;
                            while i < ehdr.e_shnum as libc::c_int {
                                s = (*sm_table.offset(i as isize)).s;
                                if !(s.is_null()
                                    || (*sm_table.offset(i as isize)).new_section == 0)
                                {
                                    sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                    if (*sh).sh_link > 0 as libc::c_int as Elf64_Word {
                                        (*s).link = (*sm_table.offset((*sh).sh_link as isize)).s;
                                    }
                                    if (*sh).sh_type == 4 as libc::c_int as Elf64_Word {
                                        (*s)
                                            .sh_info = (*(*sm_table.offset((*sh).sh_info as isize)).s)
                                            .sh_num;
                                        let ref mut fresh14 = (**((*s1).sections)
                                            .offset((*s).sh_info as isize))
                                            .reloc;
                                        *fresh14 = s;
                                    }
                                }
                                i += 1;
                                i;
                            }
                            old_to_new_syms = tcc_mallocz(
                                (nb_syms as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                    ),
                            ) as *mut libc::c_int;
                            sym = symtab.offset(1 as libc::c_int as isize);
                            let mut current_block_95: u64;
                            i = 1 as libc::c_int;
                            while i < nb_syms {
                                if (*sym).st_shndx as libc::c_int != 0 as libc::c_int
                                    && ((*sym).st_shndx as libc::c_int) < 0xff00 as libc::c_int
                                {
                                    sm = &mut *sm_table.offset((*sym).st_shndx as isize)
                                        as *mut SectionMergeInfo;
                                    if (*sm).link_once != 0 {
                                        if (*sym).st_info as libc::c_int >> 4 as libc::c_int
                                            != 0 as libc::c_int
                                        {
                                            name = strtab.offset((*sym).st_name as isize);
                                            sym_index = find_elf_sym(
                                                (*s1).c2rust_unnamed.symtab_section,
                                                name,
                                            );
                                            if sym_index != 0 {
                                                *old_to_new_syms.offset(i as isize) = sym_index;
                                            }
                                        }
                                        current_block_95 = 12705158477165241210;
                                    } else if ((*sm).s).is_null() {
                                        current_block_95 = 12705158477165241210;
                                    } else {
                                        (*sym).st_shndx = (*(*sm).s).sh_num as Elf64_Section;
                                        (*sym)
                                            .st_value = ((*sym).st_value as libc::c_ulong)
                                            .wrapping_add((*sm).offset) as Elf64_Addr as Elf64_Addr;
                                        current_block_95 = 1417769144978639029;
                                    }
                                } else {
                                    current_block_95 = 1417769144978639029;
                                }
                                match current_block_95 {
                                    1417769144978639029 => {
                                        name = strtab.offset((*sym).st_name as isize);
                                        sym_index = set_elf_sym(
                                            (*s1).c2rust_unnamed.symtab_section,
                                            (*sym).st_value,
                                            (*sym).st_size,
                                            (*sym).st_info as libc::c_int,
                                            (*sym).st_other as libc::c_int,
                                            (*sym).st_shndx as libc::c_int,
                                            name,
                                        );
                                        *old_to_new_syms.offset(i as isize) = sym_index;
                                    }
                                    _ => {}
                                }
                                i += 1;
                                i;
                                sym = sym.offset(1);
                                sym;
                            }
                            i = 1 as libc::c_int;
                            's_516: loop {
                                if !(i < ehdr.e_shnum as libc::c_int) {
                                    current_block = 9180031981464905198;
                                    break;
                                }
                                s = (*sm_table.offset(i as isize)).s;
                                if !s.is_null() {
                                    sh = &mut *shdr.offset(i as isize) as *mut Elf64_Shdr;
                                    offset = (*sm_table.offset(i as isize)).offset;
                                    size = (*sh).sh_size;
                                    match (*s).sh_type {
                                        4 => {
                                            offseti = (*sm_table.offset((*sh).sh_info as isize)).offset;
                                            rel = ((*s).data as *mut Elf64_Rela)
                                                .offset(
                                                    offset
                                                        .wrapping_div(
                                                            ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong,
                                                        ) as isize,
                                                );
                                            while rel
                                                < ((*s).data as *mut Elf64_Rela)
                                                    .offset(
                                                        offset
                                                            .wrapping_add(size)
                                                            .wrapping_div(
                                                                ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong,
                                                            ) as isize,
                                                    )
                                            {
                                                let mut type_0: libc::c_int = 0;
                                                let mut sym_index_0: libc::c_uint = 0;
                                                type_0 = ((*rel).r_info
                                                    & 0xffffffff as libc::c_uint as Elf64_Xword) as libc::c_int;
                                                sym_index_0 = ((*rel).r_info >> 32 as libc::c_int)
                                                    as libc::c_uint;
                                                if !(sym_index_0 >= nb_syms as libc::c_uint) {
                                                    sym_index_0 = *old_to_new_syms.offset(sym_index_0 as isize)
                                                        as libc::c_uint;
                                                    if !(sym_index_0 == 0
                                                        && (*sm_table.offset((*sh).sh_info as isize)).link_once
                                                            == 0)
                                                    {
                                                        (*rel)
                                                            .r_info = ((sym_index_0 as Elf64_Xword)
                                                            << 32 as libc::c_int)
                                                            .wrapping_add(type_0 as Elf64_Xword);
                                                        (*rel)
                                                            .r_offset = ((*rel).r_offset as libc::c_ulong)
                                                            .wrapping_add(offseti) as Elf64_Addr as Elf64_Addr;
                                                        rel = rel.offset(1);
                                                        rel;
                                                        continue;
                                                    }
                                                }
                                                tcc_enter_state(s1);
                                                (Some(
                                                    _tcc_error_noabort
                                                        as unsafe extern "C" fn(
                                                            *const libc::c_char,
                                                            ...
                                                        ) -> libc::c_int,
                                                ))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    b"Invalid relocation entry [%2d] '%s' @ %.8x\0" as *const u8
                                                        as *const libc::c_char,
                                                    i,
                                                    strsec.offset((*sh).sh_name as isize),
                                                    (*rel).r_offset as libc::c_int,
                                                );
                                                current_block = 11719220873777950690;
                                                break 's_516;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11719220873777950690 => {}
                                _ => {
                                    ret = 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            tcc_free(symtab as *mut libc::c_void);
            tcc_free(strtab as *mut libc::c_void);
            tcc_free(old_to_new_syms as *mut libc::c_void);
            tcc_free(sm_table as *mut libc::c_void);
            tcc_free(strsec as *mut libc::c_void);
            tcc_free(shdr as *mut libc::c_void);
            return ret;
        }
    }
    tcc_enter_state(s1);
    return (Some(
        _tcc_error_noabort
            as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
    ))
        .expect(
            "non-null function pointer",
        )(b"invalid object file\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn get_be(
    mut b: *const uint8_t,
    mut n: libc::c_int,
) -> libc::c_ulonglong {
    let mut ret: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    while n != 0 {
        let fresh15 = b;
        b = b.offset(1);
        ret = ret << 8 as libc::c_int | *fresh15 as libc::c_ulonglong;
        n -= 1;
        n;
    }
    return ret;
}
unsafe extern "C" fn read_ar_header(
    mut fd: libc::c_int,
    mut offset: libc::c_int,
    mut hdr: *mut ArchiveHeader,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    lseek(fd, offset as __off_t, 0 as libc::c_int);
    len = full_read(
        fd,
        hdr as *mut libc::c_void,
        ::core::mem::size_of::<ArchiveHeader>() as libc::c_ulong,
    ) as libc::c_int;
    if len as libc::c_ulong != ::core::mem::size_of::<ArchiveHeader>() as libc::c_ulong {
        return if len != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
    }
    if memcmp(
        ((*hdr).ar_fmag).as_mut_ptr() as *const libc::c_void,
        b"`\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    p = ((*hdr).ar_name).as_mut_ptr();
    e = p.offset(::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize);
    while e > p && *e.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32 {
        e = e.offset(-1);
        e;
    }
    *e = '\0' as i32 as libc::c_char;
    (*hdr)
        .ar_size[(::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    return len;
}
unsafe extern "C" fn tcc_load_alacarte(
    mut s1: *mut TCCState,
    mut fd: libc::c_int,
    mut size: libc::c_int,
    mut entrysize: libc::c_int,
) -> libc::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut bound: libc::c_int = 0;
    let mut nsyms: libc::c_int = 0;
    let mut sym_index: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut off: libc::c_ulonglong = 0;
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut ar_names: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ar_index: *const uint8_t = 0 as *const uint8_t;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut hdr: ArchiveHeader = ArchiveHeader {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    data = tcc_malloc(size as libc::c_ulong) as *mut uint8_t;
    if full_read(fd, data as *mut libc::c_void, size as size_t) != size as ssize_t {
        current_block = 13985115651840601093;
    } else {
        nsyms = get_be(data, entrysize) as libc::c_int;
        ar_index = data.offset(entrysize as isize);
        ar_names = (ar_index as *mut libc::c_char).offset((nsyms * entrysize) as isize);
        's_35: loop {
            bound = 0 as libc::c_int;
            p = ar_names;
            i = 0 as libc::c_int;
            while i < nsyms {
                s = (*s1).c2rust_unnamed.symtab_section;
                sym_index = find_elf_sym(s, p);
                if !(sym_index == 0) {
                    sym = &mut *((*s).data as *mut Elf64_Sym).offset(sym_index as isize)
                        as *mut Elf64_Sym;
                    if !((*sym).st_shndx as libc::c_int != 0 as libc::c_int) {
                        off = get_be(
                            ar_index.offset((i * entrysize) as isize),
                            entrysize,
                        );
                        len = read_ar_header(fd, off as libc::c_int, &mut hdr);
                        if len <= 0 as libc::c_int
                            || memcmp(
                                (hdr.ar_fmag).as_mut_ptr() as *const libc::c_void,
                                b"`\n\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                2 as libc::c_int as libc::c_ulong,
                            ) != 0
                        {
                            current_block = 13985115651840601093;
                            break 's_35;
                        }
                        off = off.wrapping_add(len as libc::c_ulonglong);
                        if (*s1).verbose as libc::c_int == 2 as libc::c_int {
                            printf(
                                b"   -> %s\n\0" as *const u8 as *const libc::c_char,
                                (hdr.ar_name).as_mut_ptr(),
                            );
                        }
                        if tcc_load_object_file(s1, fd, off as libc::c_ulong)
                            < 0 as libc::c_int
                        {
                            current_block = 11841983363366110973;
                            break 's_35;
                        }
                        bound += 1;
                        bound;
                    }
                }
                i += 1;
                i;
                p = p
                    .offset(
                        (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
            }
            if bound != 0 {
                continue;
            }
            ret = 0 as libc::c_int;
            current_block = 11841983363366110973;
            break;
        }
    }
    match current_block {
        13985115651840601093 => {
            tcc_enter_state(s1);
            (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(b"invalid archive\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    tcc_free(data as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_load_archive(
    mut s1: *mut TCCState,
    mut fd: libc::c_int,
    mut alacarte: libc::c_int,
) -> libc::c_int {
    let mut hdr: ArchiveHeader = ArchiveHeader {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut size: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut file_offset: libc::c_ulong = 0;
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    file_offset = (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        len = read_ar_header(fd, file_offset as libc::c_int, &mut hdr);
        if len == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if len < 0 as libc::c_int {
            tcc_enter_state(s1);
            return (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(b"invalid archive\0" as *const u8 as *const libc::c_char);
        }
        file_offset = file_offset.wrapping_add(len as libc::c_ulong);
        size = strtol(
            (hdr.ar_size).as_mut_ptr(),
            0 as *mut *mut libc::c_char,
            0 as libc::c_int,
        ) as libc::c_int;
        if alacarte != 0 {
            if strcmp(
                (hdr.ar_name).as_mut_ptr(),
                b"/\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                return tcc_load_alacarte(s1, fd, size, 4 as libc::c_int);
            }
            if strcmp(
                (hdr.ar_name).as_mut_ptr(),
                b"/SYM64/\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                return tcc_load_alacarte(s1, fd, size, 8 as libc::c_int);
            }
        } else if tcc_object_type(fd, &mut ehdr) == 1 as libc::c_int {
            if (*s1).verbose as libc::c_int == 2 as libc::c_int {
                printf(
                    b"   -> %s\n\0" as *const u8 as *const libc::c_char,
                    (hdr.ar_name).as_mut_ptr(),
                );
            }
            if tcc_load_object_file(s1, fd, file_offset) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        file_offset = file_offset
            .wrapping_add(size as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            & !(1 as libc::c_int) as libc::c_ulong;
    };
}
unsafe extern "C" fn set_ver_to_ver(
    mut s1: *mut TCCState,
    mut n: *mut libc::c_int,
    mut lv: *mut *mut libc::c_int,
    mut i: libc::c_int,
    mut lib: *mut libc::c_char,
    mut version: *mut libc::c_char,
) {
    while i >= *n {
        *lv = tcc_realloc(
            *lv as *mut libc::c_void,
            ((*n + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let fresh16 = *n;
        *n = *n + 1;
        *(*lv).offset(fresh16 as isize) = -(1 as libc::c_int);
    }
    if *(*lv).offset(i as isize) == -(1 as libc::c_int) {
        let mut v: libc::c_int = 0;
        let mut prev_same_lib: libc::c_int = -(1 as libc::c_int);
        v = 0 as libc::c_int;
        while v < (*s1).nb_sym_versions {
            if !(strcmp((*((*s1).sym_versions).offset(v as isize)).lib, lib) != 0) {
                prev_same_lib = v;
                if strcmp((*((*s1).sym_versions).offset(v as isize)).version, version)
                    == 0
                {
                    break;
                }
            }
            v += 1;
            v;
        }
        if v == (*s1).nb_sym_versions {
            (*s1)
                .sym_versions = tcc_realloc(
                (*s1).sym_versions as *mut libc::c_void,
                ((v + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sym_version>() as libc::c_ulong),
            ) as *mut sym_version;
            let ref mut fresh17 = (*((*s1).sym_versions).offset(v as isize)).lib;
            *fresh17 = tcc_strdup(lib);
            let ref mut fresh18 = (*((*s1).sym_versions).offset(v as isize)).version;
            *fresh18 = tcc_strdup(version);
            (*((*s1).sym_versions).offset(v as isize)).out_index = 0 as libc::c_int;
            (*((*s1).sym_versions).offset(v as isize)).prev_same_lib = prev_same_lib;
            (*s1).nb_sym_versions += 1;
            (*s1).nb_sym_versions;
        }
        *(*lv).offset(i as isize) = v;
    }
}
unsafe extern "C" fn set_sym_version(
    mut s1: *mut TCCState,
    mut sym_index: libc::c_int,
    mut verndx: libc::c_int,
) {
    if sym_index >= (*s1).nb_sym_to_version {
        let mut newelems: libc::c_int = if sym_index != 0 {
            sym_index * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        (*s1)
            .sym_to_version = tcc_realloc(
            (*s1).sym_to_version as *mut libc::c_void,
            (newelems as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        memset(
            ((*s1).sym_to_version).offset((*s1).nb_sym_to_version as isize)
                as *mut libc::c_void,
            -(1 as libc::c_int),
            ((newelems - (*s1).nb_sym_to_version) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*s1).nb_sym_to_version = newelems;
    }
    if *((*s1).sym_to_version).offset(sym_index as isize) < 0 as libc::c_int {
        *((*s1).sym_to_version).offset(sym_index as isize) = verndx;
    }
}
unsafe extern "C" fn store_version(
    mut s1: *mut TCCState,
    mut v: *mut versym_info,
    mut dynstr: *mut libc::c_char,
) {
    let mut lib: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut version: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: uint32_t = 0;
    let mut i: libc::c_int = 0;
    if !((*v).versym).is_null() && !((*v).verdef).is_null() {
        let mut vdef: *mut Elf64_Verdef = (*v).verdef;
        lib = 0 as *mut libc::c_char;
        loop {
            let mut verdaux: *mut Elf64_Verdaux = (vdef as *mut libc::c_char)
                .offset((*vdef).vd_aux as isize) as *mut Elf64_Verdaux;
            if (*vdef).vd_cnt != 0 {
                version = dynstr.offset((*verdaux).vda_name as isize);
                if lib.is_null() {
                    lib = version;
                } else {
                    set_ver_to_ver(
                        s1,
                        &mut (*v).nb_local_ver,
                        &mut (*v).local_ver,
                        (*vdef).vd_ndx as libc::c_int,
                        lib,
                        version,
                    );
                }
            }
            next = (*vdef).vd_next;
            vdef = (vdef as *mut libc::c_char).offset(next as isize)
                as *mut Elf64_Verdef;
            if !(next != 0) {
                break;
            }
        }
    }
    if !((*v).versym).is_null() && !((*v).verneed).is_null() {
        let mut vneed: *mut Elf64_Verneed = (*v).verneed;
        loop {
            let mut vernaux: *mut Elf64_Vernaux = (vneed as *mut libc::c_char)
                .offset((*vneed).vn_aux as isize) as *mut Elf64_Vernaux;
            lib = dynstr.offset((*vneed).vn_file as isize);
            i = 0 as libc::c_int;
            while i < (*vneed).vn_cnt as libc::c_int {
                if (*vernaux).vna_other as libc::c_int & 0x8000 as libc::c_int
                    == 0 as libc::c_int
                {
                    version = dynstr.offset((*vernaux).vna_name as isize);
                    set_ver_to_ver(
                        s1,
                        &mut (*v).nb_local_ver,
                        &mut (*v).local_ver,
                        (*vernaux).vna_other as libc::c_int,
                        lib,
                        version,
                    );
                }
                vernaux = (vernaux as *mut libc::c_char)
                    .offset((*vernaux).vna_next as isize) as *mut Elf64_Vernaux;
                i += 1;
                i;
            }
            next = (*vneed).vn_next;
            vneed = (vneed as *mut libc::c_char).offset(next as isize)
                as *mut Elf64_Verneed;
            if !(next != 0) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_load_dll(
    mut s1: *mut TCCState,
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut ehdr: Elf64_Ehdr = Elf64_Ehdr {
        e_ident: [0; 16],
        e_type: 0,
        e_machine: 0,
        e_version: 0,
        e_entry: 0,
        e_phoff: 0,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0,
        e_phentsize: 0,
        e_phnum: 0,
        e_shentsize: 0,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    let mut shdr: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sh: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sh1: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut i: libc::c_int = 0;
    let mut nb_syms: libc::c_int = 0;
    let mut nb_dts: libc::c_int = 0;
    let mut sym_bind: libc::c_int = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut dynsym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut dt: *mut Elf64_Dyn = 0 as *mut Elf64_Dyn;
    let mut dynamic: *mut Elf64_Dyn = 0 as *mut Elf64_Dyn;
    let mut dynstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sym_index: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut soname: *const libc::c_char = 0 as *const libc::c_char;
    let mut v: versym_info = versym_info {
        nb_versyms: 0,
        verdef: 0 as *mut Elf64_Verdef,
        verneed: 0 as *mut Elf64_Verneed,
        versym: 0 as *mut Elf64_Half,
        nb_local_ver: 0,
        local_ver: 0 as *mut libc::c_int,
    };
    full_read(
        fd,
        &mut ehdr as *mut Elf64_Ehdr as *mut libc::c_void,
        ::core::mem::size_of::<Elf64_Ehdr>() as libc::c_ulong,
    );
    if ehdr.e_ident[5 as libc::c_int as usize] as libc::c_int != 1 as libc::c_int
        || ehdr.e_machine as libc::c_int != 62 as libc::c_int
    {
        tcc_enter_state(s1);
        return (Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
        ))
            .expect(
                "non-null function pointer",
            )(b"bad architecture\0" as *const u8 as *const libc::c_char);
    }
    shdr = load_data(
        fd,
        ehdr.e_shoff,
        (::core::mem::size_of::<Elf64_Shdr>() as libc::c_ulong)
            .wrapping_mul(ehdr.e_shnum as libc::c_ulong),
    ) as *mut Elf64_Shdr;
    nb_syms = 0 as libc::c_int;
    nb_dts = 0 as libc::c_int;
    dynamic = 0 as *mut Elf64_Dyn;
    dynsym = 0 as *mut Elf64_Sym;
    dynstr = 0 as *mut libc::c_char;
    memset(
        &mut v as *mut versym_info as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<versym_info>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    sh = shdr;
    while i < ehdr.e_shnum as libc::c_int {
        match (*sh).sh_type {
            6 => {
                nb_dts = ((*sh).sh_size)
                    .wrapping_div(::core::mem::size_of::<Elf64_Dyn>() as libc::c_ulong)
                    as libc::c_int;
                dynamic = load_data(fd, (*sh).sh_offset, (*sh).sh_size)
                    as *mut Elf64_Dyn;
            }
            11 => {
                nb_syms = ((*sh).sh_size)
                    .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
                    as libc::c_int;
                dynsym = load_data(fd, (*sh).sh_offset, (*sh).sh_size) as *mut Elf64_Sym;
                sh1 = &mut *shdr.offset((*sh).sh_link as isize) as *mut Elf64_Shdr;
                dynstr = load_data(fd, (*sh1).sh_offset, (*sh1).sh_size)
                    as *mut libc::c_char;
            }
            1879048189 => {
                v
                    .verdef = load_data(fd, (*sh).sh_offset, (*sh).sh_size)
                    as *mut Elf64_Verdef;
            }
            1879048190 => {
                v
                    .verneed = load_data(fd, (*sh).sh_offset, (*sh).sh_size)
                    as *mut Elf64_Verneed;
            }
            1879048191 => {
                v
                    .nb_versyms = ((*sh).sh_size)
                    .wrapping_div(::core::mem::size_of::<Elf64_Half>() as libc::c_ulong)
                    as libc::c_int;
                v
                    .versym = load_data(fd, (*sh).sh_offset, (*sh).sh_size)
                    as *mut Elf64_Half;
            }
            _ => {}
        }
        i += 1;
        i;
        sh = sh.offset(1);
        sh;
    }
    if !dynamic.is_null() {
        soname = tcc_basename(filename);
        i = 0 as libc::c_int;
        dt = dynamic;
        while i < nb_dts {
            if (*dt).d_tag == 14 as libc::c_int as Elf64_Sxword {
                soname = dynstr.offset((*dt).d_un.d_val as isize);
            }
            i += 1;
            i;
            dt = dt.offset(1);
            dt;
        }
        if !((*tcc_add_dllref(s1, soname, level)).found != 0) {
            if v.nb_versyms != nb_syms {
                tcc_free(v.versym as *mut libc::c_void);
                v.versym = 0 as *mut Elf64_Half;
            } else {
                store_version(s1, &mut v, dynstr);
            }
            i = 1 as libc::c_int;
            sym = dynsym.offset(1 as libc::c_int as isize);
            while i < nb_syms {
                sym_bind = (*sym).st_info as libc::c_int >> 4 as libc::c_int;
                if !(sym_bind == 0 as libc::c_int) {
                    name = dynstr.offset((*sym).st_name as isize);
                    sym_index = set_elf_sym(
                        (*s1).dynsymtab_section,
                        (*sym).st_value,
                        (*sym).st_size,
                        (*sym).st_info as libc::c_int,
                        (*sym).st_other as libc::c_int,
                        (*sym).st_shndx as libc::c_int,
                        name,
                    );
                    if !(v.versym).is_null() {
                        let mut vsym: Elf64_Half = *(v.versym).offset(i as isize);
                        if vsym as libc::c_int & 0x8000 as libc::c_int
                            == 0 as libc::c_int && vsym as libc::c_int > 0 as libc::c_int
                            && (vsym as libc::c_int) < v.nb_local_ver
                        {
                            set_sym_version(
                                s1,
                                sym_index,
                                *(v.local_ver).offset(vsym as isize),
                            );
                        }
                    }
                }
                i += 1;
                i;
                sym = sym.offset(1);
                sym;
            }
        }
        ret = 0 as libc::c_int;
    }
    tcc_free(dynstr as *mut libc::c_void);
    tcc_free(dynsym as *mut libc::c_void);
    tcc_free(dynamic as *mut libc::c_void);
    tcc_free(shdr as *mut libc::c_void);
    tcc_free(v.local_ver as *mut libc::c_void);
    tcc_free(v.verdef as *mut libc::c_void);
    tcc_free(v.verneed as *mut libc::c_void);
    tcc_free(v.versym as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn ld_inp(mut s1: *mut TCCState) -> libc::c_int {
    let mut c: libc::c_int = *(*s1).ld_p as libc::c_int;
    if c == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*s1).ld_p = ((*s1).ld_p).offset(1);
    (*s1).ld_p;
    return c;
}
unsafe extern "C" fn ld_next(
    mut s1: *mut TCCState,
    mut name: *mut libc::c_char,
    mut name_size: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        ch = ld_inp(s1);
        q = name;
        let fresh19 = q;
        q = q.offset(1);
        *fresh19 = ch as libc::c_char;
        match ch {
            32 | 9 | 12 | 11 | 13 | 10 => {}
            47 => {
                ch = ld_inp(s1);
                if !(ch == '*' as i32) {
                    current_block = 5801418792739332595;
                    break;
                }
                d = 0 as libc::c_int;
                loop {
                    ch = ld_inp(s1);
                    if ch == -(1 as libc::c_int) || ch == '/' as i32 && d == '*' as i32 {
                        break;
                    }
                    d = ch;
                }
            }
            92 => {
                current_block = 10599921512955367680;
                break;
            }
            97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109
            | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122
            | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79
            | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 45 | 95 | 46 | 36
            | 126 => {
                current_block = 10599921512955367680;
                break;
            }
            -1 => {
                c = -(1 as libc::c_int);
                current_block = 313581471991351815;
                break;
            }
            _ => {
                c = ch;
                current_block = 313581471991351815;
                break;
            }
        }
    }
    loop {
        match current_block {
            313581471991351815 => {
                *q = '\0' as i32 as libc::c_char;
                break;
            }
            10599921512955367680 => {
                ch = ld_inp(s1);
                current_block = 5801418792739332595;
            }
            _ => {
                if !(ch >= 'a' as i32 && ch <= 'z' as i32
                    || ch >= 'A' as i32 && ch <= 'Z' as i32
                    || ch >= '0' as i32 && ch <= '9' as i32
                    || !(strchr(
                        b"/.-_+=$:\\,~\0" as *const u8 as *const libc::c_char,
                        ch,
                    ))
                        .is_null())
                {
                    if ch != -(1 as libc::c_int) {
                        (*s1).ld_p = ((*s1).ld_p).offset(-1);
                        (*s1).ld_p;
                    }
                    c = 256 as libc::c_int;
                    current_block = 313581471991351815;
                } else {
                    if (q.offset_from(name) as libc::c_long)
                        < (name_size - 1 as libc::c_int) as libc::c_long
                    {
                        let fresh20 = q;
                        q = q.offset(1);
                        *fresh20 = ch as libc::c_char;
                    }
                    current_block = 10599921512955367680;
                }
            }
        }
    }
    return c;
}
unsafe extern "C" fn ld_add_file(
    mut s1: *mut TCCState,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
        && *filename.offset(1 as libc::c_int as isize) as libc::c_int == 'l' as i32
    {
        return tcc_add_library(s1, filename.offset(2 as libc::c_int as isize));
    }
    if (*::core::mem::transmute::<
        &[u8; 1],
        &[libc::c_char; 1],
    >(b"\0"))[0 as libc::c_int as usize] as libc::c_int != '\0' as i32
    {
        let mut ret: libc::c_int = tcc_add_dll(
            s1,
            tcc_basename(filename),
            0 as libc::c_int,
        );
        if ret != -(2 as libc::c_int) {
            return ret;
        }
    }
    return tcc_add_file_internal(s1, filename, 0x10 as libc::c_int);
}
unsafe extern "C" fn new_undef_sym(
    mut s1: *mut TCCState,
    mut sym_offset: libc::c_int,
) -> libc::c_int {
    while (sym_offset as libc::c_ulong) < (*(*s1).c2rust_unnamed.symtab).data_offset {
        let mut esym: *mut Elf64_Sym = ((*(*s1).c2rust_unnamed.symtab).data)
            .offset(sym_offset as isize) as *mut libc::c_void as *mut Elf64_Sym;
        if (*esym).st_shndx as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        sym_offset = (sym_offset as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
            as libc::c_int as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ld_add_file_list(
    mut s1: *mut TCCState,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut t: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut sym_offset: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut pos: *mut libc::c_uchar = (*s1).ld_p;
    loop {
        (*s1).ld_p = pos;
        sym_offset = (*(*s1).c2rust_unnamed.symtab).data_offset as libc::c_int;
        c = *cmd.offset(0 as libc::c_int as isize) as libc::c_int;
        t = ld_next(
            s1,
            filename.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
        );
        if t != '(' as i32 {
            tcc_enter_state(s1);
            return (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(b"expected '(' after %s\0" as *const u8 as *const libc::c_char, cmd);
        }
        t = ld_next(
            s1,
            filename.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
        );
        loop {
            if t == -(1 as libc::c_int) {
                tcc_enter_state(s1);
                return (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(b"unexpected end of file\0" as *const u8 as *const libc::c_char);
            } else {
                if t == ')' as i32 {
                    break;
                }
                if t != 256 as libc::c_int {
                    tcc_enter_state(s1);
                    return (Some(
                        _tcc_error_noabort
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                ...
                            ) -> libc::c_int,
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"unexpected token '%c'\0" as *const u8 as *const libc::c_char,
                        t,
                    );
                } else if strcmp(
                    filename.as_mut_ptr(),
                    b"AS_NEEDED\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    ret = ld_add_file_list(s1, filename.as_mut_ptr());
                } else if c == 'I' as i32 || c == 'G' as i32 || c == 'A' as i32 {
                    ret = ld_add_file(s1, filename.as_mut_ptr() as *const libc::c_char);
                }
                if ret != 0 {
                    return -(1 as libc::c_int);
                }
                t = ld_next(
                    s1,
                    filename.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                );
                if t == ',' as i32 {
                    t = ld_next(
                        s1,
                        filename.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as libc::c_int,
                    );
                }
            }
        }
        if !(c == 'G' as i32 && new_undef_sym(s1, sym_offset) != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_load_ldscript(
    mut s1: *mut TCCState,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut cmd: [libc::c_char; 64] = [0; 64];
    let mut t: libc::c_int = 0;
    let mut ret: libc::c_int = -(3 as libc::c_int);
    let mut text_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut saved_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    saved_ptr = (*s1).ld_p;
    text_ptr = tcc_load_text(fd) as *mut libc::c_void as *mut libc::c_uchar;
    (*s1).ld_p = text_ptr;
    loop {
        t = ld_next(
            s1,
            cmd.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        if t == -(1 as libc::c_int) {
            break;
        }
        if strcmp(cmd.as_mut_ptr(), b"INPUT\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(cmd.as_mut_ptr(), b"GROUP\0" as *const u8 as *const libc::c_char)
                == 0
        {
            ret = ld_add_file_list(s1, cmd.as_mut_ptr());
        } else if strcmp(
            cmd.as_mut_ptr(),
            b"OUTPUT_FORMAT\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(cmd.as_mut_ptr(), b"TARGET\0" as *const u8 as *const libc::c_char)
                == 0
        {
            ret = ld_add_file_list(s1, cmd.as_mut_ptr());
        } else if 0 as libc::c_int == ret {
            tcc_enter_state(s1);
            ret = (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(
                b"unexpected '%s'\0" as *const u8 as *const libc::c_char,
                cmd.as_mut_ptr(),
            );
        }
        if ret != 0 {
            break;
        }
    }
    tcc_free(text_ptr as *mut libc::c_void);
    (*s1).ld_p = saved_ptr;
    return ret;
}
