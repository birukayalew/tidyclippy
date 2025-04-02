use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    static mut tcc_state: *mut TCCState;
    fn pstrcat(
        buf: *mut libc::c_char,
        buf_size: size_t,
        s: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_mallocz(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_realloc(ptr: *mut libc::c_void, size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn cstr_new(cstr: *mut CString);
    fn cstr_free(cstr: *mut CString);
    fn cstr_printf(cs: *mut CString, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn cstr_reset(cstr: *mut CString);
    static mut file: *mut BufferedFile;
    fn get_tok_str(v: libc::c_int, cv: *mut CValue) -> *const libc::c_char;
    static mut ind: libc::c_int;
    static mut nocode_wanted: libc::c_int;
    static mut func_ind: libc::c_int;
    static mut funcname: *const libc::c_char;
    fn put_extern_sym(
        sym: *mut Sym,
        section: *mut Section,
        value: Elf64_Addr,
        size: libc::c_ulong,
    );
    fn greloca(
        s: *mut Section,
        sym: *mut Sym,
        offset: libc::c_ulong,
        type_0: libc::c_int,
        addend: Elf64_Addr,
    );
    fn type_size(type_0: *mut CType, a: *mut libc::c_int) -> libc::c_int;
    fn new_section(
        s1: *mut TCCState,
        name: *const libc::c_char,
        sh_type: libc::c_int,
        sh_flags: libc::c_int,
    ) -> *mut Section;
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut libc::c_void;
    fn put_elf_str(s: *mut Section, sym: *const libc::c_char) -> libc::c_int;
    fn put_elf_sym(
        s: *mut Section,
        value: Elf64_Addr,
        size: libc::c_ulong,
        info: libc::c_int,
        other: libc::c_int,
        shndx: libc::c_int,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn put_elf_reloc(
        symtab: *mut Section,
        s: *mut Section,
        offset: libc::c_ulong,
        type_0: libc::c_int,
        symbol: libc::c_int,
    );
    fn put_elf_reloca(
        symtab: *mut Section,
        s: *mut Section,
        offset: libc::c_ulong,
        type_0: libc::c_int,
        symbol: libc::c_int,
        addend: Elf64_Addr,
    );
    fn gen_increment_tcov(sv: *mut SValue);
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
    pub c2rust_unnamed: C2RustUnnamed_7,
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
pub struct _tccdbg {
    pub last_line_num: libc::c_int,
    pub new_file: libc::c_int,
    pub section_sym: libc::c_int,
    pub debug_next_type: libc::c_int,
    pub debug_hash: *mut _debug_hash,
    pub debug_anon_hash: *mut _debug_anon_hash,
    pub n_debug_hash: libc::c_int,
    pub n_debug_anon_hash: libc::c_int,
    pub debug_info: *mut _debug_info,
    pub debug_info_root: *mut _debug_info,
    pub dwarf_sym: C2RustUnnamed_6,
    pub dwarf_line: C2RustUnnamed_5,
    pub dwarf_info: C2RustUnnamed_0,
    pub tcov_data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub offset: libc::c_ulong,
    pub last_file_name: libc::c_ulong,
    pub last_func_name: libc::c_ulong,
    pub ind: libc::c_int,
    pub line: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub start: libc::c_int,
    pub func: *mut Sym,
    pub line: libc::c_int,
    pub base_type_used: [libc::c_int; 29],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sym {
    pub v: libc::c_int,
    pub r: libc::c_ushort,
    pub a: SymAttr,
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub type_0: CType,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
    pub prev: *mut Sym,
    pub prev_tok: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub enum_val: libc::c_longlong,
    pub d: *mut libc::c_int,
    pub cleanup_func: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub c: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub start: libc::c_int,
    pub dir_size: libc::c_int,
    pub dir_table: *mut *mut libc::c_char,
    pub filename_size: libc::c_int,
    pub filename_table: *mut dwarf_filename_struct,
    pub line_size: libc::c_int,
    pub line_max_size: libc::c_int,
    pub line_data: *mut libc::c_uchar,
    pub cur_file: libc::c_int,
    pub last_file: libc::c_int,
    pub last_pc: libc::c_int,
    pub last_line: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dwarf_filename_struct {
    pub dir_entry: libc::c_int,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub info: libc::c_int,
    pub abbrev: libc::c_int,
    pub line: libc::c_int,
    pub str_0: libc::c_int,
    pub line_str: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _debug_info {
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub n_sym: libc::c_int,
    pub sym: *mut debug_sym,
    pub child: *mut _debug_info,
    pub next: *mut _debug_info,
    pub last: *mut _debug_info,
    pub parent: *mut _debug_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_sym {
    pub type_0: libc::c_int,
    pub value: libc::c_ulong,
    pub str_0: *mut libc::c_char,
    pub sec: *mut Section,
    pub sym_index: libc::c_int,
    pub info: libc::c_int,
    pub file: libc::c_int,
    pub line: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _debug_anon_hash {
    pub type_0: *mut Sym,
    pub n_debug_type: libc::c_int,
    pub debug_type: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _debug_hash {
    pub debug_type: libc::c_int,
    pub type_0: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __stab_debug_code = libc::c_uint;
pub const LAST_UNUSED_STAB_CODE: __stab_debug_code = 255;
pub const N_LENG: __stab_debug_code = 254;
pub const N_NBLCS: __stab_debug_code = 248;
pub const N_NBSTS: __stab_debug_code = 246;
pub const N_NBBSS: __stab_debug_code = 244;
pub const N_NBDATA: __stab_debug_code = 242;
pub const N_NBTEXT: __stab_debug_code = 240;
pub const N_ECOML: __stab_debug_code = 232;
pub const N_ECOMM: __stab_debug_code = 228;
pub const N_BCOMM: __stab_debug_code = 226;
pub const N_RBRAC: __stab_debug_code = 224;
pub const N_SCOPE: __stab_debug_code = 196;
pub const N_EXCL: __stab_debug_code = 194;
pub const N_LBRAC: __stab_debug_code = 192;
pub const N_ENTRY: __stab_debug_code = 164;
pub const N_EINCL: __stab_debug_code = 162;
pub const N_PSYM: __stab_debug_code = 160;
pub const N_SOL: __stab_debug_code = 132;
pub const N_BINCL: __stab_debug_code = 130;
pub const N_LSYM: __stab_debug_code = 128;
pub const N_SO: __stab_debug_code = 100;
pub const N_SSYM: __stab_debug_code = 96;
pub const N_CATCH: __stab_debug_code = 84;
pub const N_MOD2: __stab_debug_code = 80;
pub const N_EHDECL: __stab_debug_code = 80;
pub const N_DEFD: __stab_debug_code = 74;
pub const N_BROWS: __stab_debug_code = 72;
pub const N_BSLINE: __stab_debug_code = 72;
pub const N_DSLINE: __stab_debug_code = 70;
pub const N_SLINE: __stab_debug_code = 68;
pub const N_M2C: __stab_debug_code = 66;
pub const N_RSYM: __stab_debug_code = 64;
pub const N_OPT: __stab_debug_code = 60;
pub const N_OBJ: __stab_debug_code = 56;
pub const N_NOMAP: __stab_debug_code = 52;
pub const N_NSYMS: __stab_debug_code = 50;
pub const N_PC: __stab_debug_code = 48;
pub const N_MAIN: __stab_debug_code = 42;
pub const N_LCSYM: __stab_debug_code = 40;
pub const N_STSYM: __stab_debug_code = 38;
pub const N_FUN: __stab_debug_code = 36;
pub const N_FNAME: __stab_debug_code = 34;
pub const N_GSYM: __stab_debug_code = 32;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const DW_UT_hi_user: C2RustUnnamed_8 = 255;
pub const DW_UT_lo_user: C2RustUnnamed_8 = 128;
pub const DW_UT_split_type: C2RustUnnamed_8 = 6;
pub const DW_UT_split_compile: C2RustUnnamed_8 = 5;
pub const DW_UT_skeleton: C2RustUnnamed_8 = 4;
pub const DW_UT_partial: C2RustUnnamed_8 = 3;
pub const DW_UT_type: C2RustUnnamed_8 = 2;
pub const DW_UT_compile: C2RustUnnamed_8 = 1;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const DW_TAG_hi_user: C2RustUnnamed_9 = 65535;
pub const DW_TAG_GNU_call_site_parameter: C2RustUnnamed_9 = 16650;
pub const DW_TAG_GNU_call_site: C2RustUnnamed_9 = 16649;
pub const DW_TAG_GNU_formal_parameter_pack: C2RustUnnamed_9 = 16648;
pub const DW_TAG_GNU_template_parameter_pack: C2RustUnnamed_9 = 16647;
pub const DW_TAG_GNU_template_template_param: C2RustUnnamed_9 = 16646;
pub const DW_TAG_GNU_EINCL: C2RustUnnamed_9 = 16645;
pub const DW_TAG_GNU_BINCL: C2RustUnnamed_9 = 16644;
pub const DW_TAG_class_template: C2RustUnnamed_9 = 16643;
pub const DW_TAG_function_template: C2RustUnnamed_9 = 16642;
pub const DW_TAG_format_label: C2RustUnnamed_9 = 16641;
pub const DW_TAG_MIPS_loop: C2RustUnnamed_9 = 16513;
pub const DW_TAG_lo_user: C2RustUnnamed_9 = 16512;
pub const DW_TAG_immutable_type: C2RustUnnamed_9 = 75;
pub const DW_TAG_skeleton_unit: C2RustUnnamed_9 = 74;
pub const DW_TAG_call_site_parameter: C2RustUnnamed_9 = 73;
pub const DW_TAG_call_site: C2RustUnnamed_9 = 72;
pub const DW_TAG_atomic_type: C2RustUnnamed_9 = 71;
pub const DW_TAG_dynamic_type: C2RustUnnamed_9 = 70;
pub const DW_TAG_generic_subrange: C2RustUnnamed_9 = 69;
pub const DW_TAG_coarray_type: C2RustUnnamed_9 = 68;
pub const DW_TAG_template_alias: C2RustUnnamed_9 = 67;
pub const DW_TAG_rvalue_reference_type: C2RustUnnamed_9 = 66;
pub const DW_TAG_type_unit: C2RustUnnamed_9 = 65;
pub const DW_TAG_shared_type: C2RustUnnamed_9 = 64;
pub const DW_TAG_condition: C2RustUnnamed_9 = 63;
pub const DW_TAG_imported_unit: C2RustUnnamed_9 = 61;
pub const DW_TAG_partial_unit: C2RustUnnamed_9 = 60;
pub const DW_TAG_unspecified_type: C2RustUnnamed_9 = 59;
pub const DW_TAG_imported_module: C2RustUnnamed_9 = 58;
pub const DW_TAG_namespace: C2RustUnnamed_9 = 57;
pub const DW_TAG_interface_type: C2RustUnnamed_9 = 56;
pub const DW_TAG_restrict_type: C2RustUnnamed_9 = 55;
pub const DW_TAG_dwarf_procedure: C2RustUnnamed_9 = 54;
pub const DW_TAG_volatile_type: C2RustUnnamed_9 = 53;
pub const DW_TAG_variable: C2RustUnnamed_9 = 52;
pub const DW_TAG_variant_part: C2RustUnnamed_9 = 51;
pub const DW_TAG_try_block: C2RustUnnamed_9 = 50;
pub const DW_TAG_thrown_type: C2RustUnnamed_9 = 49;
pub const DW_TAG_template_value_parameter: C2RustUnnamed_9 = 48;
pub const DW_TAG_template_type_parameter: C2RustUnnamed_9 = 47;
pub const DW_TAG_subprogram: C2RustUnnamed_9 = 46;
pub const DW_TAG_packed_type: C2RustUnnamed_9 = 45;
pub const DW_TAG_namelist_item: C2RustUnnamed_9 = 44;
pub const DW_TAG_namelist: C2RustUnnamed_9 = 43;
pub const DW_TAG_friend: C2RustUnnamed_9 = 42;
pub const DW_TAG_file_type: C2RustUnnamed_9 = 41;
pub const DW_TAG_enumerator: C2RustUnnamed_9 = 40;
pub const DW_TAG_constant: C2RustUnnamed_9 = 39;
pub const DW_TAG_const_type: C2RustUnnamed_9 = 38;
pub const DW_TAG_catch_block: C2RustUnnamed_9 = 37;
pub const DW_TAG_base_type: C2RustUnnamed_9 = 36;
pub const DW_TAG_access_declaration: C2RustUnnamed_9 = 35;
pub const DW_TAG_with_stmt: C2RustUnnamed_9 = 34;
pub const DW_TAG_subrange_type: C2RustUnnamed_9 = 33;
pub const DW_TAG_set_type: C2RustUnnamed_9 = 32;
pub const DW_TAG_ptr_to_member_type: C2RustUnnamed_9 = 31;
pub const DW_TAG_module: C2RustUnnamed_9 = 30;
pub const DW_TAG_inlined_subroutine: C2RustUnnamed_9 = 29;
pub const DW_TAG_inheritance: C2RustUnnamed_9 = 28;
pub const DW_TAG_common_inclusion: C2RustUnnamed_9 = 27;
pub const DW_TAG_common_block: C2RustUnnamed_9 = 26;
pub const DW_TAG_variant: C2RustUnnamed_9 = 25;
pub const DW_TAG_unspecified_parameters: C2RustUnnamed_9 = 24;
pub const DW_TAG_union_type: C2RustUnnamed_9 = 23;
pub const DW_TAG_typedef: C2RustUnnamed_9 = 22;
pub const DW_TAG_subroutine_type: C2RustUnnamed_9 = 21;
pub const DW_TAG_structure_type: C2RustUnnamed_9 = 19;
pub const DW_TAG_string_type: C2RustUnnamed_9 = 18;
pub const DW_TAG_compile_unit: C2RustUnnamed_9 = 17;
pub const DW_TAG_reference_type: C2RustUnnamed_9 = 16;
pub const DW_TAG_pointer_type: C2RustUnnamed_9 = 15;
pub const DW_TAG_member: C2RustUnnamed_9 = 13;
pub const DW_TAG_lexical_block: C2RustUnnamed_9 = 11;
pub const DW_TAG_label: C2RustUnnamed_9 = 10;
pub const DW_TAG_imported_declaration: C2RustUnnamed_9 = 8;
pub const DW_TAG_formal_parameter: C2RustUnnamed_9 = 5;
pub const DW_TAG_enumeration_type: C2RustUnnamed_9 = 4;
pub const DW_TAG_entry_point: C2RustUnnamed_9 = 3;
pub const DW_TAG_class_type: C2RustUnnamed_9 = 2;
pub const DW_TAG_array_type: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const DW_AT_hi_user: C2RustUnnamed_10 = 16383;
pub const DW_AT_GNU_bias: C2RustUnnamed_10 = 8965;
pub const DW_AT_GNU_denominator: C2RustUnnamed_10 = 8964;
pub const DW_AT_GNU_numerator: C2RustUnnamed_10 = 8963;
pub const DW_AT_GNU_pubtypes: C2RustUnnamed_10 = 8501;
pub const DW_AT_GNU_pubnames: C2RustUnnamed_10 = 8500;
pub const DW_AT_GNU_addr_base: C2RustUnnamed_10 = 8499;
pub const DW_AT_GNU_ranges_base: C2RustUnnamed_10 = 8498;
pub const DW_AT_GNU_dwo_id: C2RustUnnamed_10 = 8497;
pub const DW_AT_GNU_dwo_name: C2RustUnnamed_10 = 8496;
pub const DW_AT_GNU_deleted: C2RustUnnamed_10 = 8474;
pub const DW_AT_GNU_macros: C2RustUnnamed_10 = 8473;
pub const DW_AT_GNU_entry_view: C2RustUnnamed_10 = 8504;
pub const DW_AT_GNU_locviews: C2RustUnnamed_10 = 8503;
pub const DW_AT_GNU_all_source_call_sites: C2RustUnnamed_10 = 8472;
pub const DW_AT_GNU_all_call_sites: C2RustUnnamed_10 = 8471;
pub const DW_AT_GNU_all_tail_call_sites: C2RustUnnamed_10 = 8470;
pub const DW_AT_GNU_tail_call: C2RustUnnamed_10 = 8469;
pub const DW_AT_GNU_call_site_target_clobbered: C2RustUnnamed_10 = 8468;
pub const DW_AT_GNU_call_site_target: C2RustUnnamed_10 = 8467;
pub const DW_AT_GNU_call_site_data_value: C2RustUnnamed_10 = 8466;
pub const DW_AT_GNU_call_site_value: C2RustUnnamed_10 = 8465;
pub const DW_AT_GNU_template_name: C2RustUnnamed_10 = 8464;
pub const DW_AT_GNU_odr_signature: C2RustUnnamed_10 = 8463;
pub const DW_AT_GNU_shared_locks_required: C2RustUnnamed_10 = 8462;
pub const DW_AT_GNU_exclusive_locks_required: C2RustUnnamed_10 = 8461;
pub const DW_AT_GNU_locks_excluded: C2RustUnnamed_10 = 8460;
pub const DW_AT_GNU_pt_guarded: C2RustUnnamed_10 = 8459;
pub const DW_AT_GNU_guarded: C2RustUnnamed_10 = 8458;
pub const DW_AT_GNU_pt_guarded_by: C2RustUnnamed_10 = 8457;
pub const DW_AT_GNU_guarded_by: C2RustUnnamed_10 = 8456;
pub const DW_AT_GNU_vector: C2RustUnnamed_10 = 8455;
pub const DW_AT_body_end: C2RustUnnamed_10 = 8454;
pub const DW_AT_body_begin: C2RustUnnamed_10 = 8453;
pub const DW_AT_src_coords: C2RustUnnamed_10 = 8452;
pub const DW_AT_mac_info: C2RustUnnamed_10 = 8451;
pub const DW_AT_src_info: C2RustUnnamed_10 = 8450;
pub const DW_AT_sf_names: C2RustUnnamed_10 = 8449;
pub const DW_AT_MIPS_assumed_size: C2RustUnnamed_10 = 8209;
pub const DW_AT_MIPS_assumed_shape_dopetype: C2RustUnnamed_10 = 8208;
pub const DW_AT_MIPS_allocatable_dopetype: C2RustUnnamed_10 = 8207;
pub const DW_AT_MIPS_ptr_dopetype: C2RustUnnamed_10 = 8206;
pub const DW_AT_MIPS_stride_elem: C2RustUnnamed_10 = 8205;
pub const DW_AT_MIPS_stride_byte: C2RustUnnamed_10 = 8204;
pub const DW_AT_MIPS_has_inlines: C2RustUnnamed_10 = 8203;
pub const DW_AT_MIPS_clone_origin: C2RustUnnamed_10 = 8202;
pub const DW_AT_MIPS_abstract_name: C2RustUnnamed_10 = 8201;
pub const DW_AT_MIPS_stride: C2RustUnnamed_10 = 8200;
pub const DW_AT_MIPS_linkage_name: C2RustUnnamed_10 = 8199;
pub const DW_AT_MIPS_software_pipeline_depth: C2RustUnnamed_10 = 8198;
pub const DW_AT_MIPS_loop_unroll_factor: C2RustUnnamed_10 = 8197;
pub const DW_AT_MIPS_epilog_begin: C2RustUnnamed_10 = 8196;
pub const DW_AT_MIPS_tail_loop_begin: C2RustUnnamed_10 = 8195;
pub const DW_AT_MIPS_loop_begin: C2RustUnnamed_10 = 8194;
pub const DW_AT_MIPS_fde: C2RustUnnamed_10 = 8193;
pub const DW_AT_lo_user: C2RustUnnamed_10 = 8192;
pub const DW_AT_loclists_base: C2RustUnnamed_10 = 140;
pub const DW_AT_defaulted: C2RustUnnamed_10 = 139;
pub const DW_AT_deleted: C2RustUnnamed_10 = 138;
pub const DW_AT_export_symbols: C2RustUnnamed_10 = 137;
pub const DW_AT_alignment: C2RustUnnamed_10 = 136;
pub const DW_AT_noreturn: C2RustUnnamed_10 = 135;
pub const DW_AT_call_data_value: C2RustUnnamed_10 = 134;
pub const DW_AT_call_data_location: C2RustUnnamed_10 = 133;
pub const DW_AT_call_target_clobbered: C2RustUnnamed_10 = 132;
pub const DW_AT_call_target: C2RustUnnamed_10 = 131;
pub const DW_AT_call_tail_call: C2RustUnnamed_10 = 130;
pub const DW_AT_call_pc: C2RustUnnamed_10 = 129;
pub const DW_AT_call_parameter: C2RustUnnamed_10 = 128;
pub const DW_AT_call_origin: C2RustUnnamed_10 = 127;
pub const DW_AT_call_value: C2RustUnnamed_10 = 126;
pub const DW_AT_call_return_pc: C2RustUnnamed_10 = 125;
pub const DW_AT_call_all_tail_calls: C2RustUnnamed_10 = 124;
pub const DW_AT_call_all_source_calls: C2RustUnnamed_10 = 123;
pub const DW_AT_call_all_calls: C2RustUnnamed_10 = 122;
pub const DW_AT_macros: C2RustUnnamed_10 = 121;
pub const DW_AT_rvalue_reference: C2RustUnnamed_10 = 120;
pub const DW_AT_reference: C2RustUnnamed_10 = 119;
pub const DW_AT_dwo_name: C2RustUnnamed_10 = 118;
pub const DW_AT_rnglists_base: C2RustUnnamed_10 = 116;
pub const DW_AT_addr_base: C2RustUnnamed_10 = 115;
pub const DW_AT_str_offsets_base: C2RustUnnamed_10 = 114;
pub const DW_AT_rank: C2RustUnnamed_10 = 113;
pub const DW_AT_string_length_byte_size: C2RustUnnamed_10 = 112;
pub const DW_AT_string_length_bit_size: C2RustUnnamed_10 = 111;
pub const DW_AT_linkage_name: C2RustUnnamed_10 = 110;
pub const DW_AT_enum_class: C2RustUnnamed_10 = 109;
pub const DW_AT_const_expr: C2RustUnnamed_10 = 108;
pub const DW_AT_data_bit_offset: C2RustUnnamed_10 = 107;
pub const DW_AT_main_subprogram: C2RustUnnamed_10 = 106;
pub const DW_AT_signature: C2RustUnnamed_10 = 105;
pub const DW_AT_recursive: C2RustUnnamed_10 = 104;
pub const DW_AT_pure: C2RustUnnamed_10 = 103;
pub const DW_AT_elemental: C2RustUnnamed_10 = 102;
pub const DW_AT_endianity: C2RustUnnamed_10 = 101;
pub const DW_AT_object_pointer: C2RustUnnamed_10 = 100;
pub const DW_AT_explicit: C2RustUnnamed_10 = 99;
pub const DW_AT_threads_scaled: C2RustUnnamed_10 = 98;
pub const DW_AT_mutable: C2RustUnnamed_10 = 97;
pub const DW_AT_picture_string: C2RustUnnamed_10 = 96;
pub const DW_AT_digit_count: C2RustUnnamed_10 = 95;
pub const DW_AT_decimal_sign: C2RustUnnamed_10 = 94;
pub const DW_AT_small: C2RustUnnamed_10 = 93;
pub const DW_AT_decimal_scale: C2RustUnnamed_10 = 92;
pub const DW_AT_binary_scale: C2RustUnnamed_10 = 91;
pub const DW_AT_description: C2RustUnnamed_10 = 90;
pub const DW_AT_call_line: C2RustUnnamed_10 = 89;
pub const DW_AT_call_file: C2RustUnnamed_10 = 88;
pub const DW_AT_call_column: C2RustUnnamed_10 = 87;
pub const DW_AT_trampoline: C2RustUnnamed_10 = 86;
pub const DW_AT_ranges: C2RustUnnamed_10 = 85;
pub const DW_AT_extension: C2RustUnnamed_10 = 84;
pub const DW_AT_use_UTF8: C2RustUnnamed_10 = 83;
pub const DW_AT_entry_pc: C2RustUnnamed_10 = 82;
pub const DW_AT_byte_stride: C2RustUnnamed_10 = 81;
pub const DW_AT_data_location: C2RustUnnamed_10 = 80;
pub const DW_AT_associated: C2RustUnnamed_10 = 79;
pub const DW_AT_allocated: C2RustUnnamed_10 = 78;
pub const DW_AT_vtable_elem_location: C2RustUnnamed_10 = 77;
pub const DW_AT_virtuality: C2RustUnnamed_10 = 76;
pub const DW_AT_variable_parameter: C2RustUnnamed_10 = 75;
pub const DW_AT_use_location: C2RustUnnamed_10 = 74;
pub const DW_AT_type: C2RustUnnamed_10 = 73;
pub const DW_AT_static_link: C2RustUnnamed_10 = 72;
pub const DW_AT_specification: C2RustUnnamed_10 = 71;
pub const DW_AT_segment: C2RustUnnamed_10 = 70;
pub const DW_AT_priority: C2RustUnnamed_10 = 69;
pub const DW_AT_namelist_item: C2RustUnnamed_10 = 68;
pub const DW_AT_macro_info: C2RustUnnamed_10 = 67;
pub const DW_AT_identifier_case: C2RustUnnamed_10 = 66;
pub const DW_AT_friend: C2RustUnnamed_10 = 65;
pub const DW_AT_frame_base: C2RustUnnamed_10 = 64;
pub const DW_AT_external: C2RustUnnamed_10 = 63;
pub const DW_AT_encoding: C2RustUnnamed_10 = 62;
pub const DW_AT_discr_list: C2RustUnnamed_10 = 61;
pub const DW_AT_declaration: C2RustUnnamed_10 = 60;
pub const DW_AT_decl_line: C2RustUnnamed_10 = 59;
pub const DW_AT_decl_file: C2RustUnnamed_10 = 58;
pub const DW_AT_decl_column: C2RustUnnamed_10 = 57;
pub const DW_AT_data_member_location: C2RustUnnamed_10 = 56;
pub const DW_AT_count: C2RustUnnamed_10 = 55;
pub const DW_AT_calling_convention: C2RustUnnamed_10 = 54;
pub const DW_AT_base_types: C2RustUnnamed_10 = 53;
pub const DW_AT_artificial: C2RustUnnamed_10 = 52;
pub const DW_AT_address_class: C2RustUnnamed_10 = 51;
pub const DW_AT_accessibility: C2RustUnnamed_10 = 50;
pub const DW_AT_abstract_origin: C2RustUnnamed_10 = 49;
pub const DW_AT_upper_bound: C2RustUnnamed_10 = 47;
pub const DW_AT_bit_stride: C2RustUnnamed_10 = 46;
pub const DW_AT_start_scope: C2RustUnnamed_10 = 44;
pub const DW_AT_return_addr: C2RustUnnamed_10 = 42;
pub const DW_AT_prototyped: C2RustUnnamed_10 = 39;
pub const DW_AT_producer: C2RustUnnamed_10 = 37;
pub const DW_AT_lower_bound: C2RustUnnamed_10 = 34;
pub const DW_AT_is_optional: C2RustUnnamed_10 = 33;
pub const DW_AT_inline: C2RustUnnamed_10 = 32;
pub const DW_AT_default_value: C2RustUnnamed_10 = 30;
pub const DW_AT_containing_type: C2RustUnnamed_10 = 29;
pub const DW_AT_const_value: C2RustUnnamed_10 = 28;
pub const DW_AT_comp_dir: C2RustUnnamed_10 = 27;
pub const DW_AT_common_reference: C2RustUnnamed_10 = 26;
pub const DW_AT_string_length: C2RustUnnamed_10 = 25;
pub const DW_AT_import: C2RustUnnamed_10 = 24;
pub const DW_AT_visibility: C2RustUnnamed_10 = 23;
pub const DW_AT_discr_value: C2RustUnnamed_10 = 22;
pub const DW_AT_discr: C2RustUnnamed_10 = 21;
pub const DW_AT_language: C2RustUnnamed_10 = 19;
pub const DW_AT_high_pc: C2RustUnnamed_10 = 18;
pub const DW_AT_low_pc: C2RustUnnamed_10 = 17;
pub const DW_AT_stmt_list: C2RustUnnamed_10 = 16;
pub const DW_AT_bit_size: C2RustUnnamed_10 = 13;
pub const DW_AT_bit_offset: C2RustUnnamed_10 = 12;
pub const DW_AT_byte_size: C2RustUnnamed_10 = 11;
pub const DW_AT_ordering: C2RustUnnamed_10 = 9;
pub const DW_AT_name: C2RustUnnamed_10 = 3;
pub const DW_AT_location: C2RustUnnamed_10 = 2;
pub const DW_AT_sibling: C2RustUnnamed_10 = 1;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const DW_FORM_GNU_strp_alt: C2RustUnnamed_11 = 7969;
pub const DW_FORM_GNU_ref_alt: C2RustUnnamed_11 = 7968;
pub const DW_FORM_GNU_str_index: C2RustUnnamed_11 = 7938;
pub const DW_FORM_GNU_addr_index: C2RustUnnamed_11 = 7937;
pub const DW_FORM_addrx4: C2RustUnnamed_11 = 44;
pub const DW_FORM_addrx3: C2RustUnnamed_11 = 43;
pub const DW_FORM_addrx2: C2RustUnnamed_11 = 42;
pub const DW_FORM_addrx1: C2RustUnnamed_11 = 41;
pub const DW_FORM_strx4: C2RustUnnamed_11 = 40;
pub const DW_FORM_strx3: C2RustUnnamed_11 = 39;
pub const DW_FORM_strx2: C2RustUnnamed_11 = 38;
pub const DW_FORM_strx1: C2RustUnnamed_11 = 37;
pub const DW_FORM_ref_sup8: C2RustUnnamed_11 = 36;
pub const DW_FORM_rnglistx: C2RustUnnamed_11 = 35;
pub const DW_FORM_loclistx: C2RustUnnamed_11 = 34;
pub const DW_FORM_implicit_const: C2RustUnnamed_11 = 33;
pub const DW_FORM_ref_sig8: C2RustUnnamed_11 = 32;
pub const DW_FORM_line_strp: C2RustUnnamed_11 = 31;
pub const DW_FORM_data16: C2RustUnnamed_11 = 30;
pub const DW_FORM_strp_sup: C2RustUnnamed_11 = 29;
pub const DW_FORM_ref_sup4: C2RustUnnamed_11 = 28;
pub const DW_FORM_addrx: C2RustUnnamed_11 = 27;
pub const DW_FORM_strx: C2RustUnnamed_11 = 26;
pub const DW_FORM_flag_present: C2RustUnnamed_11 = 25;
pub const DW_FORM_exprloc: C2RustUnnamed_11 = 24;
pub const DW_FORM_sec_offset: C2RustUnnamed_11 = 23;
pub const DW_FORM_indirect: C2RustUnnamed_11 = 22;
pub const DW_FORM_ref_udata: C2RustUnnamed_11 = 21;
pub const DW_FORM_ref8: C2RustUnnamed_11 = 20;
pub const DW_FORM_ref4: C2RustUnnamed_11 = 19;
pub const DW_FORM_ref2: C2RustUnnamed_11 = 18;
pub const DW_FORM_ref1: C2RustUnnamed_11 = 17;
pub const DW_FORM_ref_addr: C2RustUnnamed_11 = 16;
pub const DW_FORM_udata: C2RustUnnamed_11 = 15;
pub const DW_FORM_strp: C2RustUnnamed_11 = 14;
pub const DW_FORM_sdata: C2RustUnnamed_11 = 13;
pub const DW_FORM_flag: C2RustUnnamed_11 = 12;
pub const DW_FORM_data1: C2RustUnnamed_11 = 11;
pub const DW_FORM_block1: C2RustUnnamed_11 = 10;
pub const DW_FORM_block: C2RustUnnamed_11 = 9;
pub const DW_FORM_string: C2RustUnnamed_11 = 8;
pub const DW_FORM_data8: C2RustUnnamed_11 = 7;
pub const DW_FORM_data4: C2RustUnnamed_11 = 6;
pub const DW_FORM_data2: C2RustUnnamed_11 = 5;
pub const DW_FORM_block4: C2RustUnnamed_11 = 4;
pub const DW_FORM_block2: C2RustUnnamed_11 = 3;
pub const DW_FORM_addr: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const DW_OP_hi_user: C2RustUnnamed_12 = 255;
pub const DW_OP_lo_user: C2RustUnnamed_12 = 224;
pub const DW_OP_GNU_variable_value: C2RustUnnamed_12 = 253;
pub const DW_OP_GNU_const_index: C2RustUnnamed_12 = 252;
pub const DW_OP_GNU_addr_index: C2RustUnnamed_12 = 251;
pub const DW_OP_GNU_parameter_ref: C2RustUnnamed_12 = 250;
pub const DW_OP_GNU_reinterpret: C2RustUnnamed_12 = 249;
pub const DW_OP_GNU_convert: C2RustUnnamed_12 = 247;
pub const DW_OP_GNU_deref_type: C2RustUnnamed_12 = 246;
pub const DW_OP_GNU_regval_type: C2RustUnnamed_12 = 245;
pub const DW_OP_GNU_const_type: C2RustUnnamed_12 = 244;
pub const DW_OP_GNU_entry_value: C2RustUnnamed_12 = 243;
pub const DW_OP_GNU_implicit_pointer: C2RustUnnamed_12 = 242;
pub const DW_OP_GNU_encoded_addr: C2RustUnnamed_12 = 241;
pub const DW_OP_GNU_uninit: C2RustUnnamed_12 = 240;
pub const DW_OP_GNU_push_tls_address: C2RustUnnamed_12 = 224;
pub const DW_OP_reinterpret: C2RustUnnamed_12 = 169;
pub const DW_OP_convert: C2RustUnnamed_12 = 168;
pub const DW_OP_xderef_type: C2RustUnnamed_12 = 167;
pub const DW_OP_deref_type: C2RustUnnamed_12 = 166;
pub const DW_OP_regval_type: C2RustUnnamed_12 = 165;
pub const DW_OP_const_type: C2RustUnnamed_12 = 164;
pub const DW_OP_entry_value: C2RustUnnamed_12 = 163;
pub const DW_OP_constx: C2RustUnnamed_12 = 162;
pub const DW_OP_addrx: C2RustUnnamed_12 = 161;
pub const DW_OP_implicit_pointer: C2RustUnnamed_12 = 160;
pub const DW_OP_stack_value: C2RustUnnamed_12 = 159;
pub const DW_OP_implicit_value: C2RustUnnamed_12 = 158;
pub const DW_OP_bit_piece: C2RustUnnamed_12 = 157;
pub const DW_OP_call_frame_cfa: C2RustUnnamed_12 = 156;
pub const DW_OP_form_tls_address: C2RustUnnamed_12 = 155;
pub const DW_OP_call_ref: C2RustUnnamed_12 = 154;
pub const DW_OP_call4: C2RustUnnamed_12 = 153;
pub const DW_OP_call2: C2RustUnnamed_12 = 152;
pub const DW_OP_push_object_address: C2RustUnnamed_12 = 151;
pub const DW_OP_nop: C2RustUnnamed_12 = 150;
pub const DW_OP_xderef_size: C2RustUnnamed_12 = 149;
pub const DW_OP_deref_size: C2RustUnnamed_12 = 148;
pub const DW_OP_piece: C2RustUnnamed_12 = 147;
pub const DW_OP_bregx: C2RustUnnamed_12 = 146;
pub const DW_OP_fbreg: C2RustUnnamed_12 = 145;
pub const DW_OP_regx: C2RustUnnamed_12 = 144;
pub const DW_OP_breg31: C2RustUnnamed_12 = 143;
pub const DW_OP_breg30: C2RustUnnamed_12 = 142;
pub const DW_OP_breg29: C2RustUnnamed_12 = 141;
pub const DW_OP_breg28: C2RustUnnamed_12 = 140;
pub const DW_OP_breg27: C2RustUnnamed_12 = 139;
pub const DW_OP_breg26: C2RustUnnamed_12 = 138;
pub const DW_OP_breg25: C2RustUnnamed_12 = 137;
pub const DW_OP_breg24: C2RustUnnamed_12 = 136;
pub const DW_OP_breg23: C2RustUnnamed_12 = 135;
pub const DW_OP_breg22: C2RustUnnamed_12 = 134;
pub const DW_OP_breg21: C2RustUnnamed_12 = 133;
pub const DW_OP_breg20: C2RustUnnamed_12 = 132;
pub const DW_OP_breg19: C2RustUnnamed_12 = 131;
pub const DW_OP_breg18: C2RustUnnamed_12 = 130;
pub const DW_OP_breg17: C2RustUnnamed_12 = 129;
pub const DW_OP_breg16: C2RustUnnamed_12 = 128;
pub const DW_OP_breg15: C2RustUnnamed_12 = 127;
pub const DW_OP_breg14: C2RustUnnamed_12 = 126;
pub const DW_OP_breg13: C2RustUnnamed_12 = 125;
pub const DW_OP_breg12: C2RustUnnamed_12 = 124;
pub const DW_OP_breg11: C2RustUnnamed_12 = 123;
pub const DW_OP_breg10: C2RustUnnamed_12 = 122;
pub const DW_OP_breg9: C2RustUnnamed_12 = 121;
pub const DW_OP_breg8: C2RustUnnamed_12 = 120;
pub const DW_OP_breg7: C2RustUnnamed_12 = 119;
pub const DW_OP_breg6: C2RustUnnamed_12 = 118;
pub const DW_OP_breg5: C2RustUnnamed_12 = 117;
pub const DW_OP_breg4: C2RustUnnamed_12 = 116;
pub const DW_OP_breg3: C2RustUnnamed_12 = 115;
pub const DW_OP_breg2: C2RustUnnamed_12 = 114;
pub const DW_OP_breg1: C2RustUnnamed_12 = 113;
pub const DW_OP_breg0: C2RustUnnamed_12 = 112;
pub const DW_OP_reg31: C2RustUnnamed_12 = 111;
pub const DW_OP_reg30: C2RustUnnamed_12 = 110;
pub const DW_OP_reg29: C2RustUnnamed_12 = 109;
pub const DW_OP_reg28: C2RustUnnamed_12 = 108;
pub const DW_OP_reg27: C2RustUnnamed_12 = 107;
pub const DW_OP_reg26: C2RustUnnamed_12 = 106;
pub const DW_OP_reg25: C2RustUnnamed_12 = 105;
pub const DW_OP_reg24: C2RustUnnamed_12 = 104;
pub const DW_OP_reg23: C2RustUnnamed_12 = 103;
pub const DW_OP_reg22: C2RustUnnamed_12 = 102;
pub const DW_OP_reg21: C2RustUnnamed_12 = 101;
pub const DW_OP_reg20: C2RustUnnamed_12 = 100;
pub const DW_OP_reg19: C2RustUnnamed_12 = 99;
pub const DW_OP_reg18: C2RustUnnamed_12 = 98;
pub const DW_OP_reg17: C2RustUnnamed_12 = 97;
pub const DW_OP_reg16: C2RustUnnamed_12 = 96;
pub const DW_OP_reg15: C2RustUnnamed_12 = 95;
pub const DW_OP_reg14: C2RustUnnamed_12 = 94;
pub const DW_OP_reg13: C2RustUnnamed_12 = 93;
pub const DW_OP_reg12: C2RustUnnamed_12 = 92;
pub const DW_OP_reg11: C2RustUnnamed_12 = 91;
pub const DW_OP_reg10: C2RustUnnamed_12 = 90;
pub const DW_OP_reg9: C2RustUnnamed_12 = 89;
pub const DW_OP_reg8: C2RustUnnamed_12 = 88;
pub const DW_OP_reg7: C2RustUnnamed_12 = 87;
pub const DW_OP_reg6: C2RustUnnamed_12 = 86;
pub const DW_OP_reg5: C2RustUnnamed_12 = 85;
pub const DW_OP_reg4: C2RustUnnamed_12 = 84;
pub const DW_OP_reg3: C2RustUnnamed_12 = 83;
pub const DW_OP_reg2: C2RustUnnamed_12 = 82;
pub const DW_OP_reg1: C2RustUnnamed_12 = 81;
pub const DW_OP_reg0: C2RustUnnamed_12 = 80;
pub const DW_OP_lit31: C2RustUnnamed_12 = 79;
pub const DW_OP_lit30: C2RustUnnamed_12 = 78;
pub const DW_OP_lit29: C2RustUnnamed_12 = 77;
pub const DW_OP_lit28: C2RustUnnamed_12 = 76;
pub const DW_OP_lit27: C2RustUnnamed_12 = 75;
pub const DW_OP_lit26: C2RustUnnamed_12 = 74;
pub const DW_OP_lit25: C2RustUnnamed_12 = 73;
pub const DW_OP_lit24: C2RustUnnamed_12 = 72;
pub const DW_OP_lit23: C2RustUnnamed_12 = 71;
pub const DW_OP_lit22: C2RustUnnamed_12 = 70;
pub const DW_OP_lit21: C2RustUnnamed_12 = 69;
pub const DW_OP_lit20: C2RustUnnamed_12 = 68;
pub const DW_OP_lit19: C2RustUnnamed_12 = 67;
pub const DW_OP_lit18: C2RustUnnamed_12 = 66;
pub const DW_OP_lit17: C2RustUnnamed_12 = 65;
pub const DW_OP_lit16: C2RustUnnamed_12 = 64;
pub const DW_OP_lit15: C2RustUnnamed_12 = 63;
pub const DW_OP_lit14: C2RustUnnamed_12 = 62;
pub const DW_OP_lit13: C2RustUnnamed_12 = 61;
pub const DW_OP_lit12: C2RustUnnamed_12 = 60;
pub const DW_OP_lit11: C2RustUnnamed_12 = 59;
pub const DW_OP_lit10: C2RustUnnamed_12 = 58;
pub const DW_OP_lit9: C2RustUnnamed_12 = 57;
pub const DW_OP_lit8: C2RustUnnamed_12 = 56;
pub const DW_OP_lit7: C2RustUnnamed_12 = 55;
pub const DW_OP_lit6: C2RustUnnamed_12 = 54;
pub const DW_OP_lit5: C2RustUnnamed_12 = 53;
pub const DW_OP_lit4: C2RustUnnamed_12 = 52;
pub const DW_OP_lit3: C2RustUnnamed_12 = 51;
pub const DW_OP_lit2: C2RustUnnamed_12 = 50;
pub const DW_OP_lit1: C2RustUnnamed_12 = 49;
pub const DW_OP_lit0: C2RustUnnamed_12 = 48;
pub const DW_OP_skip: C2RustUnnamed_12 = 47;
pub const DW_OP_ne: C2RustUnnamed_12 = 46;
pub const DW_OP_lt: C2RustUnnamed_12 = 45;
pub const DW_OP_le: C2RustUnnamed_12 = 44;
pub const DW_OP_gt: C2RustUnnamed_12 = 43;
pub const DW_OP_ge: C2RustUnnamed_12 = 42;
pub const DW_OP_eq: C2RustUnnamed_12 = 41;
pub const DW_OP_bra: C2RustUnnamed_12 = 40;
pub const DW_OP_xor: C2RustUnnamed_12 = 39;
pub const DW_OP_shra: C2RustUnnamed_12 = 38;
pub const DW_OP_shr: C2RustUnnamed_12 = 37;
pub const DW_OP_shl: C2RustUnnamed_12 = 36;
pub const DW_OP_plus_uconst: C2RustUnnamed_12 = 35;
pub const DW_OP_plus: C2RustUnnamed_12 = 34;
pub const DW_OP_or: C2RustUnnamed_12 = 33;
pub const DW_OP_not: C2RustUnnamed_12 = 32;
pub const DW_OP_neg: C2RustUnnamed_12 = 31;
pub const DW_OP_mul: C2RustUnnamed_12 = 30;
pub const DW_OP_mod: C2RustUnnamed_12 = 29;
pub const DW_OP_minus: C2RustUnnamed_12 = 28;
pub const DW_OP_div: C2RustUnnamed_12 = 27;
pub const DW_OP_and: C2RustUnnamed_12 = 26;
pub const DW_OP_abs: C2RustUnnamed_12 = 25;
pub const DW_OP_xderef: C2RustUnnamed_12 = 24;
pub const DW_OP_rot: C2RustUnnamed_12 = 23;
pub const DW_OP_swap: C2RustUnnamed_12 = 22;
pub const DW_OP_pick: C2RustUnnamed_12 = 21;
pub const DW_OP_over: C2RustUnnamed_12 = 20;
pub const DW_OP_drop: C2RustUnnamed_12 = 19;
pub const DW_OP_dup: C2RustUnnamed_12 = 18;
pub const DW_OP_consts: C2RustUnnamed_12 = 17;
pub const DW_OP_constu: C2RustUnnamed_12 = 16;
pub const DW_OP_const8s: C2RustUnnamed_12 = 15;
pub const DW_OP_const8u: C2RustUnnamed_12 = 14;
pub const DW_OP_const4s: C2RustUnnamed_12 = 13;
pub const DW_OP_const4u: C2RustUnnamed_12 = 12;
pub const DW_OP_const2s: C2RustUnnamed_12 = 11;
pub const DW_OP_const2u: C2RustUnnamed_12 = 10;
pub const DW_OP_const1s: C2RustUnnamed_12 = 9;
pub const DW_OP_const1u: C2RustUnnamed_12 = 8;
pub const DW_OP_deref: C2RustUnnamed_12 = 6;
pub const DW_OP_addr: C2RustUnnamed_12 = 3;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const DW_ATE_hi_user: C2RustUnnamed_13 = 255;
pub const DW_ATE_lo_user: C2RustUnnamed_13 = 128;
pub const DW_ATE_ASCII: C2RustUnnamed_13 = 18;
pub const DW_ATE_UCS: C2RustUnnamed_13 = 17;
pub const DW_ATE_UTF: C2RustUnnamed_13 = 16;
pub const DW_ATE_decimal_float: C2RustUnnamed_13 = 15;
pub const DW_ATE_unsigned_fixed: C2RustUnnamed_13 = 14;
pub const DW_ATE_signed_fixed: C2RustUnnamed_13 = 13;
pub const DW_ATE_edited: C2RustUnnamed_13 = 12;
pub const DW_ATE_numeric_string: C2RustUnnamed_13 = 11;
pub const DW_ATE_packed_decimal: C2RustUnnamed_13 = 10;
pub const DW_ATE_imaginary_float: C2RustUnnamed_13 = 9;
pub const DW_ATE_unsigned_char: C2RustUnnamed_13 = 8;
pub const DW_ATE_unsigned: C2RustUnnamed_13 = 7;
pub const DW_ATE_signed_char: C2RustUnnamed_13 = 6;
pub const DW_ATE_signed: C2RustUnnamed_13 = 5;
pub const DW_ATE_float: C2RustUnnamed_13 = 4;
pub const DW_ATE_complex_float: C2RustUnnamed_13 = 3;
pub const DW_ATE_boolean: C2RustUnnamed_13 = 2;
pub const DW_ATE_address: C2RustUnnamed_13 = 1;
pub const DW_ATE_void: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const DW_LANG_hi_user: C2RustUnnamed_14 = 65535;
pub const DW_LANG_Mips_Assembler: C2RustUnnamed_14 = 32769;
pub const DW_LANG_lo_user: C2RustUnnamed_14 = 32768;
pub const DW_LANG_BLISS: C2RustUnnamed_14 = 37;
pub const DW_LANG_RenderScript: C2RustUnnamed_14 = 36;
pub const DW_LANG_Fortran08: C2RustUnnamed_14 = 35;
pub const DW_LANG_Fortran03: C2RustUnnamed_14 = 34;
pub const DW_LANG_C_plus_plus_14: C2RustUnnamed_14 = 33;
pub const DW_LANG_Dylan: C2RustUnnamed_14 = 32;
pub const DW_LANG_Julia: C2RustUnnamed_14 = 31;
pub const DW_LANG_Swift: C2RustUnnamed_14 = 30;
pub const DW_LANG_C11: C2RustUnnamed_14 = 29;
pub const DW_LANG_Rust: C2RustUnnamed_14 = 28;
pub const DW_LANG_OCaml: C2RustUnnamed_14 = 27;
pub const DW_LANG_C_plus_plus_11: C2RustUnnamed_14 = 26;
pub const DW_LANG_C_plus_plus_03: C2RustUnnamed_14 = 25;
pub const DW_LANG_Haskell: C2RustUnnamed_14 = 24;
pub const DW_LANG_Modula3: C2RustUnnamed_14 = 23;
pub const DW_LANG_Go: C2RustUnnamed_14 = 22;
pub const DW_LANG_OpenCL: C2RustUnnamed_14 = 21;
pub const DW_LANG_Python: C2RustUnnamed_14 = 20;
pub const DW_LANG_D: C2RustUnnamed_14 = 19;
pub const DW_LANG_UPC: C2RustUnnamed_14 = 18;
pub const DW_LANG_ObjC_plus_plus: C2RustUnnamed_14 = 17;
pub const DW_LANG_ObjC: C2RustUnnamed_14 = 16;
pub const DW_LANG_PLI: C2RustUnnamed_14 = 15;
pub const DW_LANG_Fortran95: C2RustUnnamed_14 = 14;
pub const DW_LANG_Ada95: C2RustUnnamed_14 = 13;
pub const DW_LANG_C99: C2RustUnnamed_14 = 12;
pub const DW_LANG_Java: C2RustUnnamed_14 = 11;
pub const DW_LANG_Modula2: C2RustUnnamed_14 = 10;
pub const DW_LANG_Pascal83: C2RustUnnamed_14 = 9;
pub const DW_LANG_Fortran90: C2RustUnnamed_14 = 8;
pub const DW_LANG_Fortran77: C2RustUnnamed_14 = 7;
pub const DW_LANG_Cobol85: C2RustUnnamed_14 = 6;
pub const DW_LANG_Cobol74: C2RustUnnamed_14 = 5;
pub const DW_LANG_C_plus_plus: C2RustUnnamed_14 = 4;
pub const DW_LANG_Ada83: C2RustUnnamed_14 = 3;
pub const DW_LANG_C: C2RustUnnamed_14 = 2;
pub const DW_LANG_C89: C2RustUnnamed_14 = 1;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const DW_LNCT_hi_user: C2RustUnnamed_15 = 16383;
pub const DW_LNCT_lo_user: C2RustUnnamed_15 = 8192;
pub const DW_LNCT_MD5: C2RustUnnamed_15 = 5;
pub const DW_LNCT_size: C2RustUnnamed_15 = 4;
pub const DW_LNCT_timestamp: C2RustUnnamed_15 = 3;
pub const DW_LNCT_directory_index: C2RustUnnamed_15 = 2;
pub const DW_LNCT_path: C2RustUnnamed_15 = 1;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const DW_LNS_set_isa: C2RustUnnamed_16 = 12;
pub const DW_LNS_set_epilogue_begin: C2RustUnnamed_16 = 11;
pub const DW_LNS_set_prologue_end: C2RustUnnamed_16 = 10;
pub const DW_LNS_fixed_advance_pc: C2RustUnnamed_16 = 9;
pub const DW_LNS_const_add_pc: C2RustUnnamed_16 = 8;
pub const DW_LNS_set_basic_block: C2RustUnnamed_16 = 7;
pub const DW_LNS_negate_stmt: C2RustUnnamed_16 = 6;
pub const DW_LNS_set_column: C2RustUnnamed_16 = 5;
pub const DW_LNS_set_file: C2RustUnnamed_16 = 4;
pub const DW_LNS_advance_line: C2RustUnnamed_16 = 3;
pub const DW_LNS_advance_pc: C2RustUnnamed_16 = 2;
pub const DW_LNS_copy: C2RustUnnamed_16 = 1;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const DW_LNE_hi_user: C2RustUnnamed_17 = 255;
pub const DW_LNE_NVIDIA_set_function_name: C2RustUnnamed_17 = 145;
pub const DW_LNE_NVIDIA_inlined_call: C2RustUnnamed_17 = 144;
pub const DW_LNE_lo_user: C2RustUnnamed_17 = 128;
pub const DW_LNE_set_discriminator: C2RustUnnamed_17 = 4;
pub const DW_LNE_define_file: C2RustUnnamed_17 = 3;
pub const DW_LNE_set_address: C2RustUnnamed_17 = 2;
pub const DW_LNE_end_sequence: C2RustUnnamed_17 = 1;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const DW_CFA_high_user: C2RustUnnamed_18 = 63;
pub const DW_CFA_GNU_negative_offset_extended: C2RustUnnamed_18 = 47;
pub const DW_CFA_GNU_args_size: C2RustUnnamed_18 = 46;
pub const DW_CFA_AARCH64_negate_ra_state: C2RustUnnamed_18 = 45;
pub const DW_CFA_GNU_window_save: C2RustUnnamed_18 = 45;
pub const DW_CFA_MIPS_advance_loc8: C2RustUnnamed_18 = 29;
pub const DW_CFA_low_user: C2RustUnnamed_18 = 28;
pub const DW_CFA_val_expression: C2RustUnnamed_18 = 22;
pub const DW_CFA_val_offset_sf: C2RustUnnamed_18 = 21;
pub const DW_CFA_val_offset: C2RustUnnamed_18 = 20;
pub const DW_CFA_def_cfa_offset_sf: C2RustUnnamed_18 = 19;
pub const DW_CFA_def_cfa_sf: C2RustUnnamed_18 = 18;
pub const DW_CFA_offset_extended_sf: C2RustUnnamed_18 = 17;
pub const DW_CFA_expression: C2RustUnnamed_18 = 16;
pub const DW_CFA_def_cfa_expression: C2RustUnnamed_18 = 15;
pub const DW_CFA_def_cfa_offset: C2RustUnnamed_18 = 14;
pub const DW_CFA_def_cfa_register: C2RustUnnamed_18 = 13;
pub const DW_CFA_def_cfa: C2RustUnnamed_18 = 12;
pub const DW_CFA_restore_state: C2RustUnnamed_18 = 11;
pub const DW_CFA_remember_state: C2RustUnnamed_18 = 10;
pub const DW_CFA_register: C2RustUnnamed_18 = 9;
pub const DW_CFA_same_value: C2RustUnnamed_18 = 8;
pub const DW_CFA_undefined: C2RustUnnamed_18 = 7;
pub const DW_CFA_restore_extended: C2RustUnnamed_18 = 6;
pub const DW_CFA_offset_extended: C2RustUnnamed_18 = 5;
pub const DW_CFA_advance_loc4: C2RustUnnamed_18 = 4;
pub const DW_CFA_advance_loc2: C2RustUnnamed_18 = 3;
pub const DW_CFA_advance_loc1: C2RustUnnamed_18 = 2;
pub const DW_CFA_set_loc: C2RustUnnamed_18 = 1;
pub const DW_CFA_nop: C2RustUnnamed_18 = 0;
pub const DW_CFA_extended: C2RustUnnamed_18 = 0;
pub const DW_CFA_restore: C2RustUnnamed_18 = 192;
pub const DW_CFA_offset: C2RustUnnamed_18 = 128;
pub const DW_CFA_advance_loc: C2RustUnnamed_18 = 64;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const DW_EH_PE_indirect: C2RustUnnamed_19 = 128;
pub const DW_EH_PE_aligned: C2RustUnnamed_19 = 80;
pub const DW_EH_PE_funcrel: C2RustUnnamed_19 = 64;
pub const DW_EH_PE_datarel: C2RustUnnamed_19 = 48;
pub const DW_EH_PE_textrel: C2RustUnnamed_19 = 32;
pub const DW_EH_PE_pcrel: C2RustUnnamed_19 = 16;
pub const DW_EH_PE_signed: C2RustUnnamed_19 = 8;
pub const DW_EH_PE_sdata8: C2RustUnnamed_19 = 12;
pub const DW_EH_PE_sdata4: C2RustUnnamed_19 = 11;
pub const DW_EH_PE_sdata2: C2RustUnnamed_19 = 10;
pub const DW_EH_PE_sleb128: C2RustUnnamed_19 = 9;
pub const DW_EH_PE_udata8: C2RustUnnamed_19 = 4;
pub const DW_EH_PE_udata4: C2RustUnnamed_19 = 3;
pub const DW_EH_PE_udata2: C2RustUnnamed_19 = 2;
pub const DW_EH_PE_uleb128: C2RustUnnamed_19 = 1;
pub const DW_EH_PE_omit: C2RustUnnamed_19 = 255;
pub const DW_EH_PE_absptr: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union CValue {
    pub ld: f128::f128,
    pub d: libc::c_double,
    pub f: libc::c_float,
    pub i: uint64_t,
    pub str_0: C2RustUnnamed_20,
    pub tab: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub data: *mut libc::c_char,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SValue {
    pub type_0: CType,
    pub r: libc::c_ushort,
    pub r2: libc::c_ushort,
    pub c2rust_unnamed: C2RustUnnamed_23,
    pub c2rust_unnamed_0: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub c2rust_unnamed: C2RustUnnamed_22,
    pub sym: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub cmp_op: libc::c_ushort,
    pub cmp_r: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub c2rust_unnamed: C2RustUnnamed_24,
    pub c: CValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub jtrue: libc::c_int,
    pub jfalse: libc::c_int,
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
pub struct C2RustUnnamed_25 {
    pub type_0: libc::c_int,
    pub size: libc::c_int,
    pub encoding: libc::c_int,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eh_search_table {
    pub pc_offset: uint32_t,
    pub fde_offset: uint32_t,
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
unsafe extern "C" fn read64le(mut p: *mut libc::c_uchar) -> uint64_t {
    return read32le(p) as uint64_t
        | (read32le(p.offset(4 as libc::c_int as isize)) as uint64_t)
            << 32 as libc::c_int;
}
#[inline]
unsafe extern "C" fn write64le(mut p: *mut libc::c_uchar, mut x: uint64_t) {
    write32le(p, x as uint32_t);
    write32le(p.offset(4 as libc::c_int as isize), (x >> 32 as libc::c_int) as uint32_t);
}
#[inline]
unsafe extern "C" fn dwarf_read_uleb128(
    mut ln: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> uint64_t {
    let mut cp: *mut libc::c_uchar = *ln;
    let mut retval: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int64_t>() as libc::c_ulong)
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_div(7 as libc::c_int as libc::c_ulong)
    {
        let mut byte: uint64_t = (if cp < end {
            let fresh0 = cp;
            cp = cp.offset(1);
            *fresh0 as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint64_t;
        retval |= (byte & 0x7f as libc::c_int as uint64_t) << i * 7 as libc::c_int;
        if byte & 0x80 as libc::c_int as uint64_t == 0 as libc::c_int as uint64_t {
            break;
        }
        i += 1;
        i;
    }
    *ln = cp;
    return retval;
}
#[inline]
unsafe extern "C" fn dwarf_read_sleb128(
    mut ln: *mut *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> int64_t {
    let mut cp: *mut libc::c_uchar = *ln;
    let mut retval: int64_t = 0 as libc::c_int as int64_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int64_t>() as libc::c_ulong)
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            .wrapping_div(7 as libc::c_int as libc::c_ulong)
    {
        let mut byte: uint64_t = (if cp < end {
            let fresh1 = cp;
            cp = cp.offset(1);
            *fresh1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint64_t;
        retval = (retval as uint64_t
            | (byte & 0x7f as libc::c_int as uint64_t) << i * 7 as libc::c_int)
            as int64_t;
        if byte & 0x80 as libc::c_int as uint64_t == 0 as libc::c_int as uint64_t {
            if byte & 0x40 as libc::c_int as uint64_t != 0
                && ((i + 1 as libc::c_int) * 7 as libc::c_int) < 64 as libc::c_int
            {
                retval = (retval as libc::c_longlong
                    | -(1 as libc::c_longlong)
                        << (i + 1 as libc::c_int) * 7 as libc::c_int) as int64_t;
            }
            break;
        } else {
            i += 1;
            i;
        }
    }
    *ln = cp;
    return retval;
}
static mut default_debug: [C2RustUnnamed_25; 29] = [
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 3 as libc::c_int,
            size: 4 as libc::c_int,
            encoding: DW_ATE_signed as libc::c_int,
            name: b"int:t1=r1;-2147483648;2147483647;\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 1 as libc::c_int,
            size: 1 as libc::c_int,
            encoding: DW_ATE_signed_char as libc::c_int,
            name: b"char:t2=r2;0;127;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 4 as libc::c_int | 0x800 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_signed as libc::c_int,
            name: b"long int:t3=r3;-9223372036854775808;9223372036854775807;\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 3 as libc::c_int | 0x10 as libc::c_int,
            size: 4 as libc::c_int,
            encoding: DW_ATE_unsigned as libc::c_int,
            name: b"unsigned int:t4=r4;0;037777777777;\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 4 as libc::c_int | 0x800 as libc::c_int | 0x10 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_unsigned as libc::c_int,
            name: b"long unsigned int:t5=r5;0;01777777777777777777777;\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 13 as libc::c_int,
            size: 16 as libc::c_int,
            encoding: DW_ATE_signed as libc::c_int,
            name: b"__int128:t6=r6;0;-1;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 13 as libc::c_int | 0x10 as libc::c_int,
            size: 16 as libc::c_int,
            encoding: DW_ATE_unsigned as libc::c_int,
            name: b"__int128 unsigned:t7=r7;0;-1;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 4 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_signed as libc::c_int,
            name: b"long long int:t8=r8;-9223372036854775808;9223372036854775807;\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 4 as libc::c_int | 0x10 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_unsigned as libc::c_int,
            name: b"long long unsigned int:t9=r9;0;01777777777777777777777;\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 2 as libc::c_int,
            size: 2 as libc::c_int,
            encoding: DW_ATE_signed as libc::c_int,
            name: b"short int:t10=r10;-32768;32767;\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 2 as libc::c_int | 0x10 as libc::c_int,
            size: 2 as libc::c_int,
            encoding: DW_ATE_unsigned as libc::c_int,
            name: b"short unsigned int:t11=r11;0;65535;\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 1 as libc::c_int | 0x20 as libc::c_int,
            size: 1 as libc::c_int,
            encoding: DW_ATE_signed_char as libc::c_int,
            name: b"signed char:t12=r12;-128;127;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 1 as libc::c_int | 0x20 as libc::c_int | 0x10 as libc::c_int,
            size: 1 as libc::c_int,
            encoding: DW_ATE_unsigned_char as libc::c_int,
            name: b"unsigned char:t13=r13;0;255;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 8 as libc::c_int,
            size: 4 as libc::c_int,
            encoding: DW_ATE_float as libc::c_int,
            name: b"float:t14=r1;4;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 9 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_float as libc::c_int,
            name: b"double:t15=r1;8;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 10 as libc::c_int,
            size: 16 as libc::c_int,
            encoding: DW_ATE_float as libc::c_int,
            name: b"long double:t16=r1;16;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Float32:t17=r1;4;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Float64:t18=r1;8;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Float128:t19=r1;16;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Float32x:t20=r1;8;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Float64x:t21=r1;16;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Decimal32:t22=r1;4;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Decimal64:t23=r1;8;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: -(1 as libc::c_int),
            size: -(1 as libc::c_int),
            encoding: -(1 as libc::c_int),
            name: b"_Decimal128:t24=r1;16;0;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 1 as libc::c_int | 0x10 as libc::c_int,
            size: 1 as libc::c_int,
            encoding: DW_ATE_unsigned_char as libc::c_int,
            name: b"unsigned char:t25=r25;0;255;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 11 as libc::c_int,
            size: 1 as libc::c_int,
            encoding: DW_ATE_boolean as libc::c_int,
            name: b"bool:t26=r26;0;255;\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 0x800 as libc::c_int | 3 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_signed as libc::c_int,
            name: b"long int:t27=r27;-9223372036854775808;9223372036854775807;\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 0x800 as libc::c_int | 3 as libc::c_int | 0x10 as libc::c_int,
            size: 8 as libc::c_int,
            encoding: DW_ATE_unsigned as libc::c_int,
            name: b"long unsigned int:t28=r28;0;01777777777777777777777;\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_25 {
            type_0: 0 as libc::c_int,
            size: 1 as libc::c_int,
            encoding: DW_ATE_unsigned_char as libc::c_int,
            name: b"void:t29=29\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
static mut dwarf_abbrev_init: [libc::c_uchar; 335] = [
    1 as libc::c_int as libc::c_uchar,
    DW_TAG_compile_unit as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_producer as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_language as libc::c_int as libc::c_uchar,
    DW_FORM_data1 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_line_strp as libc::c_int as libc::c_uchar,
    DW_AT_comp_dir as libc::c_int as libc::c_uchar,
    DW_FORM_line_strp as libc::c_int as libc::c_uchar,
    DW_AT_low_pc as libc::c_int as libc::c_uchar,
    DW_FORM_addr as libc::c_int as libc::c_uchar,
    DW_AT_high_pc as libc::c_int as libc::c_uchar,
    DW_FORM_data8 as libc::c_int as libc::c_uchar,
    DW_AT_stmt_list as libc::c_int as libc::c_uchar,
    DW_FORM_sec_offset as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    DW_TAG_base_type as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_encoding as libc::c_int as libc::c_uchar,
    DW_FORM_data1 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    DW_TAG_variable as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_external as libc::c_int as libc::c_uchar,
    DW_FORM_flag as libc::c_int as libc::c_uchar,
    DW_AT_location as libc::c_int as libc::c_uchar,
    DW_FORM_exprloc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    DW_TAG_variable as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_location as libc::c_int as libc::c_uchar,
    DW_FORM_exprloc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    DW_TAG_variable as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_location as libc::c_int as libc::c_uchar,
    DW_FORM_exprloc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    DW_TAG_formal_parameter as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_location as libc::c_int as libc::c_uchar,
    DW_FORM_exprloc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    DW_TAG_pointer_type as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_data1 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    DW_TAG_array_type as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    DW_TAG_subrange_type as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_upper_bound as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    DW_TAG_typedef as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    DW_TAG_enumerator as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_const_value as libc::c_int as libc::c_uchar,
    DW_FORM_sdata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    DW_TAG_enumerator as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_const_value as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    DW_TAG_enumeration_type as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_encoding as libc::c_int as libc::c_uchar,
    DW_FORM_data1 as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_data1 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    DW_TAG_member as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_data_member_location as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    DW_TAG_member as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_bit_size as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_data_bit_offset as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    DW_TAG_structure_type as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    DW_TAG_structure_type as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    DW_TAG_union_type as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    DW_TAG_union_type as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_byte_size as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    DW_TAG_subprogram as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_external as libc::c_int as libc::c_uchar,
    DW_FORM_flag as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_low_pc as libc::c_int as libc::c_uchar,
    DW_FORM_addr as libc::c_int as libc::c_uchar,
    DW_AT_high_pc as libc::c_int as libc::c_uchar,
    DW_FORM_data8 as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_frame_base as libc::c_int as libc::c_uchar,
    DW_FORM_exprloc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    DW_TAG_subprogram as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_name as libc::c_int as libc::c_uchar,
    DW_FORM_strp as libc::c_int as libc::c_uchar,
    DW_AT_decl_file as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_decl_line as libc::c_int as libc::c_uchar,
    DW_FORM_udata as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_low_pc as libc::c_int as libc::c_uchar,
    DW_FORM_addr as libc::c_int as libc::c_uchar,
    DW_AT_high_pc as libc::c_int as libc::c_uchar,
    DW_FORM_data8 as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_frame_base as libc::c_int as libc::c_uchar,
    DW_FORM_exprloc as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    DW_TAG_lexical_block as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_low_pc as libc::c_int as libc::c_uchar,
    DW_FORM_addr as libc::c_int as libc::c_uchar,
    DW_AT_high_pc as libc::c_int as libc::c_uchar,
    DW_FORM_data8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    DW_TAG_lexical_block as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_low_pc as libc::c_int as libc::c_uchar,
    DW_FORM_addr as libc::c_int as libc::c_uchar,
    DW_AT_high_pc as libc::c_int as libc::c_uchar,
    DW_FORM_data8 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    DW_TAG_subroutine_type as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    DW_AT_sibling as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    DW_TAG_subroutine_type as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    DW_TAG_formal_parameter as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    DW_AT_type as libc::c_int as libc::c_uchar,
    DW_FORM_ref4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut dwarf_line_opcodes: [libc::c_uchar; 12] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_new(mut s1: *mut TCCState) {
    let mut shf: libc::c_int = 0 as libc::c_int;
    if ((*s1).dState).is_null() {
        (*s1)
            .dState = tcc_mallocz(::core::mem::size_of::<_tccdbg>() as libc::c_ulong)
            as *mut _tccdbg;
    }
    if (*s1).do_debug as libc::c_int != 0 && (*s1).output_type == 1 as libc::c_int {
        (*s1).do_backtrace = 1 as libc::c_int as libc::c_uchar;
    }
    if (*s1).do_backtrace != 0 {
        shf = (1 as libc::c_int) << 1 as libc::c_int;
    }
    if (*s1).dwarf != 0 {
        (*s1).dwlo = (*s1).nb_sections;
        (*s1)
            .dwarf_info_section = new_section(
            s1,
            b".debug_info\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            shf,
        );
        (*s1)
            .dwarf_abbrev_section = new_section(
            s1,
            b".debug_abbrev\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            shf,
        );
        (*s1)
            .dwarf_line_section = new_section(
            s1,
            b".debug_line\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            shf,
        );
        (*s1)
            .dwarf_aranges_section = new_section(
            s1,
            b".debug_aranges\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            shf,
        );
        shf
            |= (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int;
        (*s1)
            .dwarf_str_section = new_section(
            s1,
            b".debug_str\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            shf,
        );
        (*(*s1).dwarf_str_section).sh_entsize = 1 as libc::c_int;
        (*(*s1).dwarf_str_section).sh_addralign = 1 as libc::c_int;
        (*(*s1).dwarf_aranges_section)
            .sh_addralign = (*(*s1).dwarf_str_section).sh_addralign;
        (*(*s1).dwarf_line_section)
            .sh_addralign = (*(*s1).dwarf_aranges_section).sh_addralign;
        (*(*s1).dwarf_abbrev_section)
            .sh_addralign = (*(*s1).dwarf_line_section).sh_addralign;
        (*(*s1).dwarf_info_section)
            .sh_addralign = (*(*s1).dwarf_abbrev_section).sh_addralign;
        if (*s1).dwarf as libc::c_int >= 5 as libc::c_int {
            (*s1)
                .dwarf_line_str_section = new_section(
                s1,
                b".debug_line_str\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                shf,
            );
            (*(*s1).dwarf_line_str_section).sh_entsize = 1 as libc::c_int;
            (*(*s1).dwarf_line_str_section).sh_addralign = 1 as libc::c_int;
        }
        (*s1).dwhi = (*s1).nb_sections;
    } else {
        (*s1)
            .stab_section = new_section(
            s1,
            b".stab\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            shf,
        );
        (*(*s1).stab_section)
            .sh_entsize = ::core::mem::size_of::<Stab_Sym>() as libc::c_ulong
            as libc::c_int;
        (*(*s1).stab_section)
            .sh_addralign = ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
            as libc::c_int;
        (*(*s1).stab_section)
            .link = new_section(
            s1,
            b".stabstr\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int,
            shf,
        );
        put_stabs(
            s1,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn put_stabs(
    mut s1: *mut TCCState,
    mut str: *const libc::c_char,
    mut type_0: libc::c_int,
    mut other: libc::c_int,
    mut desc: libc::c_int,
    mut value: libc::c_ulong,
) {
    let mut sym: *mut Stab_Sym = 0 as *mut Stab_Sym;
    let mut offset: libc::c_uint = 0;
    if type_0 == N_SLINE as libc::c_int
        && {
            offset = (*(*s1).stab_section).data_offset as libc::c_uint;
            offset != 0
        }
        && {
            sym = (((*(*s1).stab_section).data).offset(offset as isize) as *mut Stab_Sym)
                .offset(-(1 as libc::c_int as isize));
            !sym.is_null()
        } && (*sym).n_type as libc::c_int == type_0
        && (*sym).n_value as libc::c_ulong == value
    {
        (*sym).n_desc = desc as libc::c_ushort;
        return;
    }
    sym = section_ptr_add(
        (*s1).stab_section,
        ::core::mem::size_of::<Stab_Sym>() as libc::c_ulong,
    ) as *mut Stab_Sym;
    if !str.is_null() {
        (*sym).n_strx = put_elf_str((*(*s1).stab_section).link, str) as libc::c_uint;
    } else {
        (*sym).n_strx = 0 as libc::c_int as libc::c_uint;
    }
    (*sym).n_type = type_0 as libc::c_uchar;
    (*sym).n_other = other as libc::c_uchar;
    (*sym).n_desc = desc as libc::c_ushort;
    (*sym).n_value = value as libc::c_uint;
}
unsafe extern "C" fn put_stabs_r(
    mut s1: *mut TCCState,
    mut str: *const libc::c_char,
    mut type_0: libc::c_int,
    mut other: libc::c_int,
    mut desc: libc::c_int,
    mut value: libc::c_ulong,
    mut sec: *mut Section,
    mut sym_index: libc::c_int,
) {
    put_elf_reloc(
        (*s1).c2rust_unnamed.symtab_section,
        (*s1).stab_section,
        ((*(*s1).stab_section).data_offset)
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
        if ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
        {
            1 as libc::c_int
        } else {
            11 as libc::c_int
        },
        sym_index,
    );
    put_stabs(s1, str, type_0, other, desc, value);
}
unsafe extern "C" fn put_stabn(
    mut s1: *mut TCCState,
    mut type_0: libc::c_int,
    mut other: libc::c_int,
    mut desc: libc::c_int,
    mut value: libc::c_int,
) {
    put_stabs(s1, 0 as *const libc::c_char, type_0, other, desc, value as libc::c_ulong);
}
unsafe extern "C" fn dwarf_get_section_sym(mut s: *mut Section) -> libc::c_int {
    let mut s1: *mut TCCState = (*s).s1;
    return put_elf_sym(
        (*s1).c2rust_unnamed.symtab_section,
        0 as libc::c_int as Elf64_Addr,
        0 as libc::c_int as libc::c_ulong,
        ((0 as libc::c_int) << 4 as libc::c_int)
            + (3 as libc::c_int & 0xf as libc::c_int),
        0 as libc::c_int,
        (*s).sh_num,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn dwarf_reloc(
    mut s: *mut Section,
    mut sym: libc::c_int,
    mut rel: libc::c_int,
) {
    let mut s1: *mut TCCState = (*s).s1;
    put_elf_reloca(
        (*s1).c2rust_unnamed.symtab_section,
        s,
        (*s).data_offset,
        rel,
        sym,
        0 as libc::c_int as Elf64_Addr,
    );
}
unsafe extern "C" fn dwarf_string(
    mut s: *mut Section,
    mut dw: *mut Section,
    mut sym: libc::c_int,
    mut str: *const libc::c_char,
) {
    let mut s1: *mut TCCState = (*s).s1;
    let mut offset: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    len = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    offset = (*dw).data_offset as libc::c_int;
    ptr = section_ptr_add(dw, len as Elf64_Addr) as *mut libc::c_char;
    memmove(ptr as *mut libc::c_void, str as *const libc::c_void, len as libc::c_ulong);
    put_elf_reloca(
        (*s1).c2rust_unnamed.symtab_section,
        s,
        (*s).data_offset,
        10 as libc::c_int,
        sym,
        (if 8 as libc::c_int == 4 as libc::c_int { 0 as libc::c_int } else { offset })
            as Elf64_Addr,
    );
    write32le(
        section_ptr_add(s, 4 as libc::c_int as Elf64_Addr) as *mut libc::c_uchar,
        (if 8 as libc::c_int == 4 as libc::c_int { offset } else { 0 as libc::c_int })
            as uint32_t,
    );
}
unsafe extern "C" fn dwarf_strp(mut s: *mut Section, mut str: *const libc::c_char) {
    let mut s1: *mut TCCState = (*s).s1;
    dwarf_string(s, (*s1).dwarf_str_section, (*(*s1).dState).dwarf_sym.str_0, str);
}
unsafe extern "C" fn dwarf_line_strp(mut s: *mut Section, mut str: *const libc::c_char) {
    let mut s1: *mut TCCState = (*s).s1;
    dwarf_string(
        s,
        (*s1).dwarf_line_str_section,
        (*(*s1).dState).dwarf_sym.line_str,
        str,
    );
}
unsafe extern "C" fn dwarf_line_op(mut s1: *mut TCCState, mut op: libc::c_uchar) {
    if (*(*s1).dState).dwarf_line.line_size >= (*(*s1).dState).dwarf_line.line_max_size {
        (*(*s1).dState).dwarf_line.line_max_size += 1024 as libc::c_int;
        (*(*s1).dState)
            .dwarf_line
            .line_data = tcc_realloc(
            (*(*s1).dState).dwarf_line.line_data as *mut libc::c_void,
            (*(*s1).dState).dwarf_line.line_max_size as libc::c_ulong,
        ) as *mut libc::c_uchar;
    }
    let fresh2 = (*(*s1).dState).dwarf_line.line_size;
    (*(*s1).dState).dwarf_line.line_size = (*(*s1).dState).dwarf_line.line_size + 1;
    *((*(*s1).dState).dwarf_line.line_data).offset(fresh2 as isize) = op;
}
unsafe extern "C" fn dwarf_file(mut s1: *mut TCCState) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index_offset: libc::c_int = (((*s1).dwarf as libc::c_int) < 5 as libc::c_int)
        as libc::c_int;
    if strcmp(
        ((*file).filename).as_mut_ptr(),
        b"<command line>\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*(*s1).dState).dwarf_line.cur_file = 1 as libc::c_int;
        return;
    }
    filename = strrchr(((*file).filename).as_mut_ptr(), '/' as i32);
    if filename.is_null() {
        i = 1 as libc::c_int;
        while i < (*(*s1).dState).dwarf_line.filename_size {
            if (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                .dir_entry == 0 as libc::c_int
                && strcmp(
                    (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                        .name,
                    ((*file).filename).as_mut_ptr(),
                ) == 0 as libc::c_int
            {
                (*(*s1).dState).dwarf_line.cur_file = i + index_offset;
                return;
            }
            i += 1;
            i;
        }
        i = -index_offset;
        filename = ((*file).filename).as_mut_ptr();
    } else {
        let mut undo: *mut libc::c_char = filename;
        let mut dir: *mut libc::c_char = ((*file).filename).as_mut_ptr();
        let fresh3 = filename;
        filename = filename.offset(1);
        *fresh3 = '\0' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).dwarf_line.dir_size {
            if strcmp(*((*(*s1).dState).dwarf_line.dir_table).offset(i as isize), dir)
                == 0 as libc::c_int
            {
                j = 1 as libc::c_int;
                while j < (*(*s1).dState).dwarf_line.filename_size {
                    if (*((*(*s1).dState).dwarf_line.filename_table).offset(j as isize))
                        .dir_entry - index_offset == i
                        && strcmp(
                            (*((*(*s1).dState).dwarf_line.filename_table)
                                .offset(j as isize))
                                .name,
                            filename,
                        ) == 0 as libc::c_int
                    {
                        *undo = '/' as i32 as libc::c_char;
                        (*(*s1).dState).dwarf_line.cur_file = j + index_offset;
                        return;
                    }
                    j += 1;
                    j;
                }
                break;
            } else {
                i += 1;
                i;
            }
        }
        if i == (*(*s1).dState).dwarf_line.dir_size {
            (*(*s1).dState).dwarf_line.dir_size += 1;
            (*(*s1).dState).dwarf_line.dir_size;
            (*(*s1).dState)
                .dwarf_line
                .dir_table = tcc_realloc(
                (*(*s1).dState).dwarf_line.dir_table as *mut libc::c_void,
                ((*(*s1).dState).dwarf_line.dir_size as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            let ref mut fresh4 = *((*(*s1).dState).dwarf_line.dir_table)
                .offset(i as isize);
            *fresh4 = tcc_strdup(dir);
        }
        *undo = '/' as i32 as libc::c_char;
    }
    (*(*s1).dState)
        .dwarf_line
        .filename_table = tcc_realloc(
        (*(*s1).dState).dwarf_line.filename_table as *mut libc::c_void,
        (((*(*s1).dState).dwarf_line.filename_size + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<dwarf_filename_struct>() as libc::c_ulong,
            ),
    ) as *mut dwarf_filename_struct;
    (*((*(*s1).dState).dwarf_line.filename_table)
        .offset((*(*s1).dState).dwarf_line.filename_size as isize))
        .dir_entry = i + index_offset;
    let ref mut fresh5 = (*((*(*s1).dState).dwarf_line.filename_table)
        .offset((*(*s1).dState).dwarf_line.filename_size as isize))
        .name;
    *fresh5 = tcc_strdup(filename);
    let fresh6 = (*(*s1).dState).dwarf_line.filename_size;
    (*(*s1).dState)
        .dwarf_line
        .filename_size = (*(*s1).dState).dwarf_line.filename_size + 1;
    (*(*s1).dState).dwarf_line.cur_file = fresh6 + index_offset;
}
unsafe extern "C" fn dwarf_sleb128_size(mut value: libc::c_longlong) -> libc::c_int {
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut end: libc::c_longlong = value >> 63 as libc::c_int;
    let mut last: libc::c_uchar = (end & 0x40 as libc::c_int as libc::c_longlong)
        as libc::c_uchar;
    let mut byte: libc::c_uchar = 0;
    loop {
        byte = (value & 0x7f as libc::c_int as libc::c_longlong) as libc::c_uchar;
        value >>= 7 as libc::c_int;
        size += 1;
        size;
        if !(value != end
            || byte as libc::c_int & 0x40 as libc::c_int != last as libc::c_int)
        {
            break;
        }
    }
    return size;
}
unsafe extern "C" fn dwarf_uleb128(mut s: *mut Section, mut value: libc::c_ulonglong) {
    loop {
        let mut byte: libc::c_uchar = (value & 0x7f as libc::c_int as libc::c_ulonglong)
            as libc::c_uchar;
        value >>= 7 as libc::c_int;
        *(section_ptr_add(s, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = (byte as libc::c_int
            | (if value != 0 { 0x80 as libc::c_int } else { 0 as libc::c_int }))
            as uint8_t;
        if !(value != 0 as libc::c_int as libc::c_ulonglong) {
            break;
        }
    };
}
unsafe extern "C" fn dwarf_sleb128(mut s: *mut Section, mut value: libc::c_longlong) {
    let mut more: libc::c_int = 0;
    let mut end: libc::c_longlong = value >> 63 as libc::c_int;
    let mut last: libc::c_uchar = (end & 0x40 as libc::c_int as libc::c_longlong)
        as libc::c_uchar;
    loop {
        let mut byte: libc::c_uchar = (value & 0x7f as libc::c_int as libc::c_longlong)
            as libc::c_uchar;
        value >>= 7 as libc::c_int;
        more = (value != end
            || byte as libc::c_int & 0x40 as libc::c_int != last as libc::c_int)
            as libc::c_int;
        *(section_ptr_add(s, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = (byte as libc::c_int | 0x80 as libc::c_int * more)
            as uint8_t;
        if !(more != 0) {
            break;
        }
    };
}
unsafe extern "C" fn dwarf_uleb128_op(
    mut s1: *mut TCCState,
    mut value: libc::c_ulonglong,
) {
    loop {
        let mut byte: libc::c_uchar = (value & 0x7f as libc::c_int as libc::c_ulonglong)
            as libc::c_uchar;
        value >>= 7 as libc::c_int;
        dwarf_line_op(
            s1,
            (byte as libc::c_int
                | (if value != 0 { 0x80 as libc::c_int } else { 0 as libc::c_int }))
                as libc::c_uchar,
        );
        if !(value != 0 as libc::c_int as libc::c_ulonglong) {
            break;
        }
    };
}
unsafe extern "C" fn dwarf_sleb128_op(
    mut s1: *mut TCCState,
    mut value: libc::c_longlong,
) {
    let mut more: libc::c_int = 0;
    let mut end: libc::c_longlong = value >> 63 as libc::c_int;
    let mut last: libc::c_uchar = (end & 0x40 as libc::c_int as libc::c_longlong)
        as libc::c_uchar;
    loop {
        let mut byte: libc::c_uchar = (value & 0x7f as libc::c_int as libc::c_longlong)
            as libc::c_uchar;
        value >>= 7 as libc::c_int;
        more = (value != end
            || byte as libc::c_int & 0x40 as libc::c_int != last as libc::c_int)
            as libc::c_int;
        dwarf_line_op(
            s1,
            (byte as libc::c_int | 0x80 as libc::c_int * more) as libc::c_uchar,
        );
        if !(more != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_eh_frame_start(mut s1: *mut TCCState) {
    if (*s1).unwind_tables == 0 {
        return;
    }
    (*s1)
        .eh_frame_section = new_section(
        s1,
        b".eh_frame\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    (*s1).eh_start = (*(*s1).eh_frame_section).data_offset;
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        0 as libc::c_int as uint32_t,
    );
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        0 as libc::c_int as uint32_t,
    );
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = 1 as libc::c_int as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = 'z' as i32 as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = 'R' as i32 as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = 0 as libc::c_int as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 1 as libc::c_int as libc::c_ulonglong);
    dwarf_sleb128((*s1).eh_frame_section, -(8 as libc::c_int) as libc::c_longlong);
    dwarf_uleb128((*s1).eh_frame_section, 16 as libc::c_int as libc::c_ulonglong);
    dwarf_uleb128((*s1).eh_frame_section, 1 as libc::c_int as libc::c_ulonglong);
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_EH_PE_udata4 as libc::c_int
        | DW_EH_PE_signed as libc::c_int | DW_EH_PE_pcrel as libc::c_int) as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = DW_CFA_def_cfa as libc::c_int as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 7 as libc::c_int as libc::c_ulonglong);
    dwarf_uleb128((*s1).eh_frame_section, 8 as libc::c_int as libc::c_ulonglong);
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_CFA_offset as libc::c_int + 16 as libc::c_int) as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 1 as libc::c_int as libc::c_ulonglong);
    while ((*(*s1).eh_frame_section).data_offset).wrapping_sub((*s1).eh_start)
        & 3 as libc::c_int as libc::c_ulong != 0
    {
        *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = DW_CFA_nop as libc::c_int as uint8_t;
    }
    write32le(
        ((*(*s1).eh_frame_section).data).offset((*s1).eh_start as isize),
        ((*(*s1).eh_frame_section).data_offset)
            .wrapping_sub((*s1).eh_start)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong) as uint32_t,
    );
}
unsafe extern "C" fn tcc_debug_frame_end(mut s1: *mut TCCState, mut size: libc::c_int) {
    let mut eh_section_sym: libc::c_int = 0;
    let mut fde_start: libc::c_ulong = 0;
    if ((*s1).eh_frame_section).is_null() {
        return;
    }
    eh_section_sym = dwarf_get_section_sym((*s1).text_section);
    fde_start = (*(*s1).eh_frame_section).data_offset;
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        0 as libc::c_int as uint32_t,
    );
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        fde_start
            .wrapping_sub((*s1).eh_start)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as uint32_t,
    );
    dwarf_reloc((*s1).eh_frame_section, eh_section_sym, 2 as libc::c_int);
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        func_ind as uint32_t,
    );
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        size as uint32_t,
    );
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = 0 as libc::c_int as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_CFA_advance_loc as libc::c_int + 1 as libc::c_int)
        as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = DW_CFA_def_cfa_offset as libc::c_int as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 16 as libc::c_int as libc::c_ulonglong);
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_CFA_offset as libc::c_int + 6 as libc::c_int) as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 2 as libc::c_int as libc::c_ulonglong);
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_CFA_advance_loc as libc::c_int + 3 as libc::c_int)
        as uint8_t;
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = DW_CFA_def_cfa_register as libc::c_int as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 6 as libc::c_int as libc::c_ulonglong);
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = DW_CFA_advance_loc4 as libc::c_int as uint8_t;
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        (size - 5 as libc::c_int) as uint32_t,
    );
    *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = DW_CFA_def_cfa as libc::c_int as uint8_t;
    dwarf_uleb128((*s1).eh_frame_section, 7 as libc::c_int as libc::c_ulonglong);
    dwarf_uleb128((*s1).eh_frame_section, 8 as libc::c_int as libc::c_ulonglong);
    while ((*(*s1).eh_frame_section).data_offset).wrapping_sub(fde_start)
        & 3 as libc::c_int as libc::c_ulong != 0
    {
        *(section_ptr_add((*s1).eh_frame_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = DW_CFA_nop as libc::c_int as uint8_t;
    }
    write32le(
        ((*(*s1).eh_frame_section).data).offset(fde_start as isize),
        ((*(*s1).eh_frame_section).data_offset)
            .wrapping_sub(fde_start)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong) as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tcc_eh_frame_end(mut s1: *mut TCCState) {
    if ((*s1).eh_frame_section).is_null() {
        return;
    }
    write32le(
        section_ptr_add((*s1).eh_frame_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        0 as libc::c_int as uint32_t,
    );
}
unsafe extern "C" fn sort_eh_table(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut pc1: uint32_t = (*(a as *const eh_search_table)).pc_offset;
    let mut pc2: uint32_t = (*(b as *const eh_search_table)).pc_offset;
    return if pc1 < pc2 {
        -(1 as libc::c_int)
    } else if pc1 > pc2 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_eh_frame_hdr(
    mut s1: *mut TCCState,
    mut final_0: libc::c_int,
) {
    let mut current_block: u64;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut offset: libc::c_int = 0;
    let mut count_offset: libc::c_ulong = 0;
    let mut tab_offset: libc::c_ulong = 0;
    let mut ln: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut last_cie_offset: libc::c_uint = 0xffffffff as libc::c_uint;
    if ((*s1).eh_frame_section).is_null() || (*(*s1).eh_frame_section).data_offset == 0 {
        return;
    }
    if final_0 != 0 && ((*s1).eh_frame_hdr_section).is_null() {
        return;
    }
    if final_0 == 0 as libc::c_int {
        (*s1)
            .eh_frame_hdr_section = new_section(
            s1,
            b".eh_frame_hdr\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int,
        );
    }
    (*(*s1).eh_frame_hdr_section).data_offset = 0 as libc::c_int as libc::c_ulong;
    *(section_ptr_add((*s1).eh_frame_hdr_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = 1 as libc::c_int as uint8_t;
    *(section_ptr_add((*s1).eh_frame_hdr_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_EH_PE_sdata4 as libc::c_int
        | DW_EH_PE_pcrel as libc::c_int) as uint8_t;
    *(section_ptr_add((*s1).eh_frame_hdr_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_EH_PE_udata4 as libc::c_int
        | DW_EH_PE_absptr as libc::c_int) as uint8_t;
    *(section_ptr_add((*s1).eh_frame_hdr_section, 1 as libc::c_int as Elf64_Addr)
        as *mut uint8_t) = (DW_EH_PE_sdata4 as libc::c_int
        | DW_EH_PE_datarel as libc::c_int) as uint8_t;
    offset = ((*(*s1).eh_frame_section).sh_addr)
        .wrapping_sub((*(*s1).eh_frame_hdr_section).sh_addr)
        .wrapping_sub((*(*s1).eh_frame_hdr_section).data_offset) as libc::c_int;
    write32le(
        section_ptr_add((*s1).eh_frame_hdr_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        offset as uint32_t,
    );
    count_offset = (*(*s1).eh_frame_hdr_section).data_offset;
    write32le(
        section_ptr_add((*s1).eh_frame_hdr_section, 4 as libc::c_int as Elf64_Addr)
            as *mut libc::c_uchar,
        0 as libc::c_int as uint32_t,
    );
    tab_offset = (*(*s1).eh_frame_hdr_section).data_offset;
    ln = (*(*s1).eh_frame_section).data;
    end = ((*(*s1).eh_frame_section).data)
        .offset((*(*s1).eh_frame_section).data_offset as isize);
    while ln < end {
        let mut fde: *mut libc::c_uchar = ln;
        let mut rd: *mut libc::c_uchar = ln;
        let mut cie_offset: libc::c_uint = 0;
        let mut version: libc::c_uint = 0;
        let mut length: libc::c_uint = if rd.offset(3 as libc::c_int as isize) < end {
            rd = rd.offset(4 as libc::c_int as isize);
            read32le(rd.offset(-(4 as libc::c_int as isize)))
        } else {
            0 as libc::c_int as uint32_t
        };
        let mut pc_offset: libc::c_uint = 0;
        let mut fde_offset: libc::c_uint = 0;
        if !(length == 0 as libc::c_int as libc::c_uint) {
            cie_offset = if rd.offset(3 as libc::c_int as isize) < end {
                rd = rd.offset(4 as libc::c_int as isize);
                read32le(rd.offset(-(4 as libc::c_int as isize)))
            } else {
                0 as libc::c_int as uint32_t
            };
            if !(cie_offset == 0 as libc::c_int as libc::c_uint) {
                if cie_offset != last_cie_offset {
                    let mut cie: *mut libc::c_uchar = rd
                        .offset(-(cie_offset as isize))
                        .offset(4 as libc::c_int as isize);
                    if cie < (*(*s1).eh_frame_section).data {
                        current_block = 8743088701226518218;
                    } else {
                        version = (if cie < end {
                            let fresh7 = cie;
                            cie = cie.offset(1);
                            *fresh7 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as libc::c_uint;
                        if (version == 1 as libc::c_int as libc::c_uint
                            || version == 3 as libc::c_int as libc::c_uint)
                            && (if cie < end {
                                let fresh8 = cie;
                                cie = cie.offset(1);
                                *fresh8 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) == 'z' as i32
                            && (if cie < end {
                                let fresh9 = cie;
                                cie = cie.offset(1);
                                *fresh9 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) == 'R' as i32
                            && (if cie < end {
                                let fresh10 = cie;
                                cie = cie.offset(1);
                                *fresh10 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) == 0 as libc::c_int
                        {
                            dwarf_read_uleb128(&mut cie, end);
                            dwarf_read_sleb128(&mut cie, end);
                            if cie < end {
                                let fresh11 = cie;
                                cie = cie.offset(1);
                                *fresh11;
                            } else {};
                            if dwarf_read_uleb128(&mut cie, end)
                                == 1 as libc::c_int as uint64_t
                                && (if cie < end {
                                    let fresh12 = cie;
                                    cie = cie.offset(1);
                                    *fresh12 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                    == DW_EH_PE_udata4 as libc::c_int
                                        | DW_EH_PE_signed as libc::c_int
                                        | DW_EH_PE_pcrel as libc::c_int
                            {
                                last_cie_offset = cie_offset;
                                current_block = 14818589718467733107;
                            } else {
                                current_block = 8743088701226518218;
                            }
                        } else {
                            current_block = 8743088701226518218;
                        }
                    }
                } else {
                    current_block = 14818589718467733107;
                }
                match current_block {
                    8743088701226518218 => {}
                    _ => {
                        count += 1;
                        count;
                        fde_offset = ((*(*s1).eh_frame_section).sh_addr)
                            .wrapping_add(
                                fde.offset_from((*(*s1).eh_frame_section).data)
                                    as libc::c_long as Elf64_Addr,
                            )
                            .wrapping_sub((*(*s1).eh_frame_hdr_section).sh_addr)
                            as libc::c_uint;
                        pc_offset = (if rd.offset(3 as libc::c_int as isize) < end {
                            rd = rd.offset(4 as libc::c_int as isize);
                            read32le(rd.offset(-(4 as libc::c_int as isize)))
                        } else {
                            0 as libc::c_int as uint32_t
                        })
                            .wrapping_add(fde_offset)
                            .wrapping_add(8 as libc::c_int as libc::c_uint);
                        write32le(
                            section_ptr_add(
                                (*s1).eh_frame_hdr_section,
                                4 as libc::c_int as Elf64_Addr,
                            ) as *mut libc::c_uchar,
                            pc_offset,
                        );
                        write32le(
                            section_ptr_add(
                                (*s1).eh_frame_hdr_section,
                                4 as libc::c_int as Elf64_Addr,
                            ) as *mut libc::c_uchar,
                            fde_offset,
                        );
                    }
                }
            }
        }
        ln = ln.offset(length.wrapping_add(4 as libc::c_int as libc::c_uint) as isize);
    }
    add32le(((*(*s1).eh_frame_hdr_section).data).offset(count_offset as isize), count);
    qsort(
        ((*(*s1).eh_frame_hdr_section).data).offset(tab_offset as isize)
            as *mut libc::c_void,
        count as size_t,
        ::core::mem::size_of::<eh_search_table>() as libc::c_ulong,
        Some(
            sort_eh_table
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_start(mut s1: *mut TCCState) {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    filename = if !((*file).prev).is_null() {
        ((*(*file).prev).filename).as_mut_ptr()
    } else {
        ((*file).filename).as_mut_ptr()
    };
    put_elf_sym(
        (*s1).c2rust_unnamed.symtab_section,
        0 as libc::c_int as Elf64_Addr,
        0 as libc::c_int as libc::c_ulong,
        ((0 as libc::c_int) << 4 as libc::c_int)
            + (4 as libc::c_int & 0xf as libc::c_int),
        0 as libc::c_int,
        0xfff1 as libc::c_int,
        filename,
    );
    if (*s1).do_debug != 0 {
        put_elf_sym(
            (*s1).c2rust_unnamed.symtab_section,
            (*(*s1).text_section).data_offset,
            0 as libc::c_int as libc::c_ulong,
            ((0 as libc::c_int) << 4 as libc::c_int)
                + (0 as libc::c_int & 0xf as libc::c_int),
            0 as libc::c_int,
            (*(*s1).text_section).sh_num,
            b"$a\0" as *const u8 as *const libc::c_char,
        );
        (*(*s1).dState).last_line_num = 0 as libc::c_int;
        (*(*s1).dState).new_file = (*(*s1).dState).last_line_num;
        (*(*s1).dState)
            .debug_next_type = (::core::mem::size_of::<[C2RustUnnamed_25; 29]>()
            as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong)
            as libc::c_int;
        (*(*s1).dState).debug_hash = 0 as *mut _debug_hash;
        (*(*s1).dState).debug_anon_hash = 0 as *mut _debug_anon_hash;
        (*(*s1).dState).n_debug_hash = 0 as libc::c_int;
        (*(*s1).dState).n_debug_anon_hash = 0 as libc::c_int;
        getcwd(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        if (*s1).dwarf != 0 {
            let mut start_abbrev: libc::c_int = 0;
            let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut undo: *mut libc::c_char = 0 as *mut libc::c_char;
            start_abbrev = (*(*s1).dwarf_abbrev_section).data_offset as libc::c_int;
            ptr = section_ptr_add(
                (*s1).dwarf_abbrev_section,
                ::core::mem::size_of::<[libc::c_uchar; 335]>() as libc::c_ulong,
            ) as *mut libc::c_uchar;
            memcpy(
                ptr as *mut libc::c_void,
                dwarf_abbrev_init.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uchar; 335]>() as libc::c_ulong,
            );
            if ((*s1).dwarf as libc::c_int) < 5 as libc::c_int {
                while *ptr != 0 {
                    ptr = ptr.offset(3 as libc::c_int as isize);
                    while *ptr != 0 {
                        if *ptr.offset(1 as libc::c_int as isize) as libc::c_int
                            == DW_FORM_line_strp as libc::c_int
                        {
                            *ptr
                                .offset(
                                    1 as libc::c_int as isize,
                                ) = DW_FORM_strp as libc::c_int as libc::c_uchar;
                        }
                        if ((*s1).dwarf as libc::c_int) < 4 as libc::c_int {
                            if *ptr.offset(1 as libc::c_int as isize) as libc::c_int
                                == DW_FORM_sec_offset as libc::c_int
                            {
                                *ptr
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = DW_FORM_data4 as libc::c_int as libc::c_uchar;
                            }
                            if *ptr.offset(1 as libc::c_int as isize) as libc::c_int
                                == DW_FORM_exprloc as libc::c_int
                            {
                                *ptr
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = DW_FORM_block1 as libc::c_int as libc::c_uchar;
                            }
                        }
                        ptr = ptr.offset(2 as libc::c_int as isize);
                    }
                    ptr = ptr.offset(2 as libc::c_int as isize);
                }
            }
            (*(*s1).dState)
                .dwarf_sym
                .info = dwarf_get_section_sym((*s1).dwarf_info_section);
            (*(*s1).dState)
                .dwarf_sym
                .abbrev = dwarf_get_section_sym((*s1).dwarf_abbrev_section);
            (*(*s1).dState)
                .dwarf_sym
                .line = dwarf_get_section_sym((*s1).dwarf_line_section);
            (*(*s1).dState)
                .dwarf_sym
                .str_0 = dwarf_get_section_sym((*s1).dwarf_str_section);
            if (*tcc_state).dwarf as libc::c_int >= 5 as libc::c_int {
                (*(*s1).dState)
                    .dwarf_sym
                    .line_str = dwarf_get_section_sym((*s1).dwarf_line_str_section);
            } else {
                (*s1).dwarf_line_str_section = (*s1).dwarf_str_section;
                (*(*s1).dState).dwarf_sym.line_str = (*(*s1).dState).dwarf_sym.str_0;
            }
            (*(*s1).dState).section_sym = dwarf_get_section_sym((*s1).text_section);
            (*(*s1).dState)
                .dwarf_info
                .start = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            write16le(
                section_ptr_add((*s1).dwarf_info_section, 2 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                (*s1).dwarf as uint16_t,
            );
            if (*s1).dwarf as libc::c_int >= 5 as libc::c_int {
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = DW_UT_compile as libc::c_int as uint8_t;
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 8 as libc::c_int as uint8_t;
                dwarf_reloc(
                    (*s1).dwarf_info_section,
                    (*(*s1).dState).dwarf_sym.abbrev,
                    10 as libc::c_int,
                );
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    start_abbrev as uint32_t,
                );
            } else {
                dwarf_reloc(
                    (*s1).dwarf_info_section,
                    (*(*s1).dState).dwarf_sym.abbrev,
                    10 as libc::c_int,
                );
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    start_abbrev as uint32_t,
                );
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 8 as libc::c_int as uint8_t;
            }
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 1 as libc::c_int as uint8_t;
            dwarf_strp(
                (*s1).dwarf_info_section,
                b"tcc 0.9.28rc\0" as *const u8 as *const libc::c_char,
            );
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = (if (*s1).cversion
                == 201112 as libc::c_int as libc::c_uint
            {
                DW_LANG_C11 as libc::c_int
            } else {
                DW_LANG_C99 as libc::c_int
            }) as uint8_t;
            dwarf_line_strp((*s1).dwarf_info_section, filename);
            dwarf_line_strp((*s1).dwarf_info_section, buf.as_mut_ptr());
            dwarf_reloc(
                (*s1).dwarf_info_section,
                (*(*s1).dState).section_sym,
                1 as libc::c_int,
            );
            write64le(
                section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                ind as uint64_t,
            );
            write64le(
                section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint64_t,
            );
            dwarf_reloc(
                (*s1).dwarf_info_section,
                (*(*s1).dState).dwarf_sym.line,
                10 as libc::c_int,
            );
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                (*(*s1).dwarf_line_section).data_offset as uint32_t,
            );
            (*(*s1).dState)
                .dwarf_line
                .start = (*(*s1).dwarf_line_section).data_offset as libc::c_int;
            write32le(
                section_ptr_add((*s1).dwarf_line_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            write16le(
                section_ptr_add((*s1).dwarf_line_section, 2 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                (*s1).dwarf as uint16_t,
            );
            if (*s1).dwarf as libc::c_int >= 5 as libc::c_int {
                *(section_ptr_add(
                    (*s1).dwarf_line_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 8 as libc::c_int as uint8_t;
                *(section_ptr_add(
                    (*s1).dwarf_line_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 0 as libc::c_int as uint8_t;
            }
            write32le(
                section_ptr_add((*s1).dwarf_line_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 1 as libc::c_int as uint8_t;
            if (*s1).dwarf as libc::c_int >= 4 as libc::c_int {
                *(section_ptr_add(
                    (*s1).dwarf_line_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 1 as libc::c_int as uint8_t;
            }
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 1 as libc::c_int as uint8_t;
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = -(5 as libc::c_int) as uint8_t;
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 14 as libc::c_int as uint8_t;
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 13 as libc::c_int as uint8_t;
            ptr = section_ptr_add(
                (*s1).dwarf_line_section,
                ::core::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong,
            ) as *mut libc::c_uchar;
            memcpy(
                ptr as *mut libc::c_void,
                dwarf_line_opcodes.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong,
            );
            undo = strrchr(filename, '/' as i32);
            if !undo.is_null() {
                *undo = 0 as libc::c_int as libc::c_char;
            }
            (*(*s1).dState)
                .dwarf_line
                .dir_size = 1 as libc::c_int
                + (undo != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
            (*(*s1).dState)
                .dwarf_line
                .dir_table = tcc_malloc(
                (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((*(*s1).dState).dwarf_line.dir_size as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            let ref mut fresh13 = *((*(*s1).dState).dwarf_line.dir_table)
                .offset(0 as libc::c_int as isize);
            *fresh13 = tcc_strdup(buf.as_mut_ptr());
            if !undo.is_null() {
                let ref mut fresh14 = *((*(*s1).dState).dwarf_line.dir_table)
                    .offset(1 as libc::c_int as isize);
                *fresh14 = tcc_strdup(filename);
            }
            (*(*s1).dState).dwarf_line.filename_size = 2 as libc::c_int;
            (*(*s1).dState)
                .dwarf_line
                .filename_table = tcc_malloc(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<dwarf_filename_struct>() as libc::c_ulong,
                    ),
            ) as *mut dwarf_filename_struct;
            (*((*(*s1).dState).dwarf_line.filename_table)
                .offset(0 as libc::c_int as isize))
                .dir_entry = 0 as libc::c_int;
            if !undo.is_null() {
                let ref mut fresh15 = (*((*(*s1).dState).dwarf_line.filename_table)
                    .offset(0 as libc::c_int as isize))
                    .name;
                *fresh15 = tcc_strdup(undo.offset(1 as libc::c_int as isize));
                (*((*(*s1).dState).dwarf_line.filename_table)
                    .offset(1 as libc::c_int as isize))
                    .dir_entry = 1 as libc::c_int;
                let ref mut fresh16 = (*((*(*s1).dState).dwarf_line.filename_table)
                    .offset(1 as libc::c_int as isize))
                    .name;
                *fresh16 = tcc_strdup(undo.offset(1 as libc::c_int as isize));
                *undo = '/' as i32 as libc::c_char;
            } else {
                let ref mut fresh17 = (*((*(*s1).dState).dwarf_line.filename_table)
                    .offset(0 as libc::c_int as isize))
                    .name;
                *fresh17 = tcc_strdup(filename);
                (*((*(*s1).dState).dwarf_line.filename_table)
                    .offset(1 as libc::c_int as isize))
                    .dir_entry = 0 as libc::c_int;
                let ref mut fresh18 = (*((*(*s1).dState).dwarf_line.filename_table)
                    .offset(1 as libc::c_int as isize))
                    .name;
                *fresh18 = tcc_strdup(filename);
            }
            (*(*s1).dState).dwarf_line.line_max_size = 0 as libc::c_int;
            (*(*s1).dState)
                .dwarf_line
                .line_size = (*(*s1).dState).dwarf_line.line_max_size;
            (*(*s1).dState).dwarf_line.line_data = 0 as *mut libc::c_uchar;
            (*(*s1).dState).dwarf_line.cur_file = 1 as libc::c_int;
            (*(*s1).dState).dwarf_line.last_file = 0 as libc::c_int;
            (*(*s1).dState).dwarf_line.last_pc = 0 as libc::c_int;
            (*(*s1).dState).dwarf_line.last_line = 1 as libc::c_int;
            dwarf_line_op(s1, 0 as libc::c_int as libc::c_uchar);
            dwarf_uleb128_op(
                s1,
                (1 as libc::c_int + 8 as libc::c_int) as libc::c_ulonglong,
            );
            dwarf_line_op(s1, DW_LNE_set_address as libc::c_int as libc::c_uchar);
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                dwarf_line_op(s1, 0 as libc::c_int as libc::c_uchar);
                i += 1;
                i;
            }
            memset(
                &mut (*(*s1).dState).dwarf_info.base_type_used as *mut [libc::c_int; 29]
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_int; 29]>() as libc::c_ulong,
            );
        } else {
            pstrcat(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"/\0" as *const u8 as *const libc::c_char,
            );
            (*(*s1).dState)
                .section_sym = put_elf_sym(
                (*s1).c2rust_unnamed.symtab_section,
                0 as libc::c_int as Elf64_Addr,
                0 as libc::c_int as libc::c_ulong,
                ((0 as libc::c_int) << 4 as libc::c_int)
                    + (3 as libc::c_int & 0xf as libc::c_int),
                0 as libc::c_int,
                (*(*s1).text_section).sh_num,
                0 as *const libc::c_char,
            );
            put_stabs_r(
                s1,
                buf.as_mut_ptr(),
                N_SO as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*(*s1).text_section).data_offset,
                (*s1).text_section,
                (*(*s1).dState).section_sym,
            );
            put_stabs_r(
                s1,
                filename,
                N_SO as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*(*s1).text_section).data_offset,
                (*s1).text_section,
                (*(*s1).dState).section_sym,
            );
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < (::core::mem::size_of::<[C2RustUnnamed_25; 29]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong,
                    )
            {
                put_stabs(
                    s1,
                    default_debug[i as usize].name,
                    N_LSYM as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int as libc::c_ulong,
                );
                i += 1;
                i;
            }
        }
        tcc_debug_bincl(s1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_end(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 || (*(*s1).dState).debug_next_type == 0 as libc::c_int {
        return;
    }
    if !((*(*s1).dState).debug_info_root).is_null() {
        tcc_debug_funcend(s1, 0 as libc::c_int);
    }
    if (*s1).dwarf != 0 {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut start_aranges: libc::c_int = 0;
        let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut text_size: libc::c_int = (*(*s1).text_section).data_offset
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).n_debug_anon_hash {
            let mut t: *mut Sym = (*((*(*s1).dState).debug_anon_hash).offset(i as isize))
                .type_0;
            let mut pos: libc::c_int = (*(*s1).dwarf_info_section).data_offset
                as libc::c_int;
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = (if (*t).type_0.t as libc::c_uint
                & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint
                    | 0xf as libc::c_int as libc::c_uint)
                == ((1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int)
                    as libc::c_uint
            {
                19 as libc::c_int
            } else {
                17 as libc::c_int
            }) as uint8_t;
            dwarf_strp(
                (*s1).dwarf_info_section,
                if (*t).v & !(0x40000000 as libc::c_int) >= 0x10000000 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    get_tok_str((*t).v, 0 as *mut CValue)
                },
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                0 as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*file).line_num as libc::c_ulonglong,
            );
            j = 0 as libc::c_int;
            while j
                < (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).n_debug_type
            {
                write32le(
                    ((*(*s1).dwarf_info_section).data)
                        .offset(
                            *((*((*(*s1).dState).debug_anon_hash).offset(i as isize))
                                .debug_type)
                                .offset(j as isize) as isize,
                        ),
                    (pos - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
                j += 1;
                j;
            }
            tcc_free(
                (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).debug_type
                    as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        tcc_free((*(*s1).dState).debug_anon_hash as *mut libc::c_void);
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = 0 as libc::c_int as uint8_t;
        ptr = ((*(*s1).dwarf_info_section).data)
            .offset((*(*s1).dState).dwarf_info.start as isize);
        write32le(
            ptr,
            ((*(*s1).dwarf_info_section).data_offset)
                .wrapping_sub((*(*s1).dState).dwarf_info.start as libc::c_ulong)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong) as uint32_t,
        );
        write32le(
            ptr
                .offset(25 as libc::c_int as isize)
                .offset(
                    ((*s1).dwarf as libc::c_int >= 5 as libc::c_int) as libc::c_int
                        as isize,
                )
                .offset(8 as libc::c_int as isize),
            text_size as uint32_t,
        );
        start_aranges = (*(*s1).dwarf_aranges_section).data_offset as libc::c_int;
        write32le(
            section_ptr_add((*s1).dwarf_aranges_section, 4 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint32_t,
        );
        write16le(
            section_ptr_add((*s1).dwarf_aranges_section, 2 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            2 as libc::c_int as uint16_t,
        );
        dwarf_reloc(
            (*s1).dwarf_aranges_section,
            (*(*s1).dState).dwarf_sym.info,
            10 as libc::c_int,
        );
        write32le(
            section_ptr_add((*s1).dwarf_aranges_section, 4 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint32_t,
        );
        *(section_ptr_add((*s1).dwarf_aranges_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = 8 as libc::c_int as uint8_t;
        *(section_ptr_add((*s1).dwarf_aranges_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = 0 as libc::c_int as uint8_t;
        write32le(
            section_ptr_add((*s1).dwarf_aranges_section, 4 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint32_t,
        );
        dwarf_reloc(
            (*s1).dwarf_aranges_section,
            (*(*s1).dState).section_sym,
            1 as libc::c_int,
        );
        write64le(
            section_ptr_add((*s1).dwarf_aranges_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint64_t,
        );
        write64le(
            section_ptr_add((*s1).dwarf_aranges_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            text_size as uint64_t,
        );
        write64le(
            section_ptr_add((*s1).dwarf_aranges_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint64_t,
        );
        write64le(
            section_ptr_add((*s1).dwarf_aranges_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint64_t,
        );
        ptr = ((*(*s1).dwarf_aranges_section).data).offset(start_aranges as isize);
        write32le(
            ptr,
            ((*(*s1).dwarf_aranges_section).data_offset)
                .wrapping_sub(start_aranges as libc::c_ulong)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong) as uint32_t,
        );
        if (*s1).dwarf as libc::c_int >= 5 as libc::c_int {
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 1 as libc::c_int as uint8_t;
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                DW_LNCT_path as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                DW_FORM_line_strp as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                (*(*s1).dState).dwarf_line.dir_size as libc::c_ulonglong,
            );
            i = 0 as libc::c_int;
            while i < (*(*s1).dState).dwarf_line.dir_size {
                dwarf_line_strp(
                    (*s1).dwarf_line_section,
                    *((*(*s1).dState).dwarf_line.dir_table).offset(i as isize),
                );
                i += 1;
                i;
            }
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 2 as libc::c_int as uint8_t;
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                DW_LNCT_path as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                DW_FORM_line_strp as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                DW_LNCT_directory_index as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                DW_FORM_udata as libc::c_int as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_line_section,
                (*(*s1).dState).dwarf_line.filename_size as libc::c_ulonglong,
            );
            i = 0 as libc::c_int;
            while i < (*(*s1).dState).dwarf_line.filename_size {
                dwarf_line_strp(
                    (*s1).dwarf_line_section,
                    (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                        .name,
                );
                dwarf_uleb128(
                    (*s1).dwarf_line_section,
                    (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                        .dir_entry as libc::c_ulonglong,
                );
                i += 1;
                i;
            }
        } else {
            let mut len: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*(*s1).dState).dwarf_line.dir_size {
                len = (strlen(
                    *((*(*s1).dState).dwarf_line.dir_table).offset(i as isize),
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                ptr = section_ptr_add((*s1).dwarf_line_section, len as Elf64_Addr)
                    as *mut libc::c_uchar;
                memmove(
                    ptr as *mut libc::c_void,
                    *((*(*s1).dState).dwarf_line.dir_table).offset(i as isize)
                        as *const libc::c_void,
                    len as libc::c_ulong,
                );
                i += 1;
                i;
            }
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 0 as libc::c_int as uint8_t;
            i = 0 as libc::c_int;
            while i < (*(*s1).dState).dwarf_line.filename_size {
                len = (strlen(
                    (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                        .name,
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                ptr = section_ptr_add((*s1).dwarf_line_section, len as Elf64_Addr)
                    as *mut libc::c_uchar;
                memmove(
                    ptr as *mut libc::c_void,
                    (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                        .name as *const libc::c_void,
                    len as libc::c_ulong,
                );
                dwarf_uleb128(
                    (*s1).dwarf_line_section,
                    (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize))
                        .dir_entry as libc::c_ulonglong,
                );
                dwarf_uleb128(
                    (*s1).dwarf_line_section,
                    0 as libc::c_int as libc::c_ulonglong,
                );
                dwarf_uleb128(
                    (*s1).dwarf_line_section,
                    0 as libc::c_int as libc::c_ulonglong,
                );
                i += 1;
                i;
            }
            *(section_ptr_add((*s1).dwarf_line_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 0 as libc::c_int as uint8_t;
        }
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).dwarf_line.dir_size {
            tcc_free(
                *((*(*s1).dState).dwarf_line.dir_table).offset(i as isize)
                    as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        tcc_free((*(*s1).dState).dwarf_line.dir_table as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).dwarf_line.filename_size {
            tcc_free(
                (*((*(*s1).dState).dwarf_line.filename_table).offset(i as isize)).name
                    as *mut libc::c_void,
            );
            i += 1;
            i;
        }
        tcc_free((*(*s1).dState).dwarf_line.filename_table as *mut libc::c_void);
        dwarf_line_op(s1, 0 as libc::c_int as libc::c_uchar);
        dwarf_uleb128_op(s1, 1 as libc::c_int as libc::c_ulonglong);
        dwarf_line_op(s1, DW_LNE_end_sequence as libc::c_int as libc::c_uchar);
        i = ((*s1).dwarf as libc::c_int >= 5 as libc::c_int) as libc::c_int
            * 2 as libc::c_int;
        write32le(
            &mut *((*(*s1).dwarf_line_section).data)
                .offset(
                    ((*(*s1).dState).dwarf_line.start + 6 as libc::c_int + i) as isize,
                ),
            ((*(*s1).dwarf_line_section).data_offset)
                .wrapping_sub((*(*s1).dState).dwarf_line.start as libc::c_ulong)
                .wrapping_sub((10 as libc::c_int + i) as libc::c_ulong) as uint32_t,
        );
        section_ptr_add((*s1).dwarf_line_section, 3 as libc::c_int as Elf64_Addr);
        dwarf_reloc(
            (*s1).dwarf_line_section,
            (*(*s1).dState).section_sym,
            1 as libc::c_int,
        );
        ptr = section_ptr_add(
            (*s1).dwarf_line_section,
            ((*(*s1).dState).dwarf_line.line_size - 3 as libc::c_int) as Elf64_Addr,
        ) as *mut libc::c_uchar;
        memmove(
            ptr.offset(-(3 as libc::c_int as isize)) as *mut libc::c_void,
            (*(*s1).dState).dwarf_line.line_data as *const libc::c_void,
            (*(*s1).dState).dwarf_line.line_size as libc::c_ulong,
        );
        tcc_free((*(*s1).dState).dwarf_line.line_data as *mut libc::c_void);
        write32le(
            ((*(*s1).dwarf_line_section).data)
                .offset((*(*s1).dState).dwarf_line.start as isize),
            ((*(*s1).dwarf_line_section).data_offset)
                .wrapping_sub((*(*s1).dState).dwarf_line.start as libc::c_ulong)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong) as uint32_t,
        );
    } else {
        put_stabs_r(
            s1,
            0 as *const libc::c_char,
            N_SO as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            (*(*s1).text_section).data_offset,
            (*s1).text_section,
            (*(*s1).dState).section_sym,
        );
    }
    tcc_free((*(*s1).dState).debug_hash as *mut libc::c_void);
    (*(*s1).dState).debug_next_type = 0 as libc::c_int;
}
unsafe extern "C" fn put_new_file(mut s1: *mut TCCState) -> *mut BufferedFile {
    let mut f: *mut BufferedFile = file;
    if (*f).filename[0 as libc::c_int as usize] as libc::c_int == ':' as i32 {
        f = (*f).prev;
    }
    if !f.is_null() && (*(*s1).dState).new_file != 0 {
        (*(*s1).dState).last_line_num = 0 as libc::c_int;
        (*(*s1).dState).new_file = (*(*s1).dState).last_line_num;
        if (*s1).dwarf != 0 {
            dwarf_file(s1);
        } else {
            put_stabs_r(
                s1,
                ((*f).filename).as_mut_ptr(),
                N_SOL as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                ind as libc::c_ulong,
                (*s1).text_section,
                (*(*s1).dState).section_sym,
            );
        }
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_newfile(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 {
        return;
    }
    if (*s1).dwarf != 0 {
        dwarf_file(s1);
    }
    (*(*s1).dState).new_file = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_bincl(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 {
        return;
    }
    if (*s1).dwarf != 0 {
        dwarf_file(s1);
    } else {
        put_stabs(
            s1,
            ((*file).filename).as_mut_ptr(),
            N_BINCL as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as libc::c_ulong,
        );
    }
    (*(*s1).dState).new_file = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_eincl(mut s1: *mut TCCState) {
    if (*s1).do_debug == 0 {
        return;
    }
    if (*s1).dwarf != 0 {
        dwarf_file(s1);
    } else {
        put_stabn(
            s1,
            N_EINCL as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    (*(*s1).dState).new_file = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_line(mut s1: *mut TCCState) {
    let mut f: *mut BufferedFile = 0 as *mut BufferedFile;
    if (*s1).do_debug == 0 {
        return;
    }
    if (*s1).cur_text_section != (*s1).text_section || nocode_wanted != 0 {
        return;
    }
    f = put_new_file(s1);
    if f.is_null() {
        return;
    }
    if (*(*s1).dState).last_line_num == (*f).line_num {
        return;
    }
    (*(*s1).dState).last_line_num = (*f).line_num;
    if (*s1).dwarf != 0 {
        let mut len_pc: libc::c_int = (ind - (*(*s1).dState).dwarf_line.last_pc)
            / 1 as libc::c_int;
        let mut len_line: libc::c_int = (*f).line_num
            - (*(*s1).dState).dwarf_line.last_line;
        let mut n: libc::c_int = len_pc * 14 as libc::c_int + len_line
            + 13 as libc::c_int - -(5 as libc::c_int);
        if (*(*s1).dState).dwarf_line.cur_file != (*(*s1).dState).dwarf_line.last_file {
            (*(*s1).dState).dwarf_line.last_file = (*(*s1).dState).dwarf_line.cur_file;
            dwarf_line_op(s1, DW_LNS_set_file as libc::c_int as libc::c_uchar);
            dwarf_uleb128_op(
                s1,
                (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
            );
        }
        if len_pc != 0 && len_line >= -(5 as libc::c_int)
            && len_line <= 13 as libc::c_int + -(5 as libc::c_int)
            && n >= 13 as libc::c_int && n <= 255 as libc::c_int
        {
            dwarf_line_op(s1, n as libc::c_uchar);
        } else {
            if len_pc != 0 {
                n = len_pc * 14 as libc::c_int + 0 as libc::c_int + 13 as libc::c_int
                    - -(5 as libc::c_int);
                if n >= 13 as libc::c_int && n <= 255 as libc::c_int {
                    dwarf_line_op(s1, n as libc::c_uchar);
                } else {
                    dwarf_line_op(s1, DW_LNS_advance_pc as libc::c_int as libc::c_uchar);
                    dwarf_uleb128_op(s1, len_pc as libc::c_ulonglong);
                }
            }
            if len_line != 0 {
                n = 0 as libc::c_int * 14 as libc::c_int + len_line + 13 as libc::c_int
                    - -(5 as libc::c_int);
                if len_line >= -(5 as libc::c_int)
                    && len_line <= 13 as libc::c_int + -(5 as libc::c_int)
                    && n >= 13 as libc::c_int && n <= 255 as libc::c_int
                {
                    dwarf_line_op(s1, n as libc::c_uchar);
                } else {
                    dwarf_line_op(
                        s1,
                        DW_LNS_advance_line as libc::c_int as libc::c_uchar,
                    );
                    dwarf_sleb128_op(s1, len_line as libc::c_longlong);
                }
            }
        }
        (*(*s1).dState).dwarf_line.last_pc = ind;
        (*(*s1).dState).dwarf_line.last_line = (*f).line_num;
    } else if func_ind != -(1 as libc::c_int) {
        put_stabn(
            s1,
            N_SLINE as libc::c_int,
            0 as libc::c_int,
            (*f).line_num,
            ind - func_ind,
        );
    } else {
        put_stabs_r(
            s1,
            0 as *const libc::c_char,
            N_SLINE as libc::c_int,
            0 as libc::c_int,
            (*f).line_num,
            ind as libc::c_ulong,
            (*s1).text_section,
            (*(*s1).dState).section_sym,
        );
    };
}
unsafe extern "C" fn tcc_debug_stabs(
    mut s1: *mut TCCState,
    mut str: *const libc::c_char,
    mut type_0: libc::c_int,
    mut value: libc::c_ulong,
    mut sec: *mut Section,
    mut sym_index: libc::c_int,
    mut info: libc::c_int,
) {
    let mut s: *mut debug_sym = 0 as *mut debug_sym;
    if !((*(*s1).dState).debug_info).is_null() {
        (*(*(*s1).dState).debug_info)
            .sym = tcc_realloc(
            (*(*(*s1).dState).debug_info).sym as *mut libc::c_void,
            (::core::mem::size_of::<debug_sym>() as libc::c_ulong)
                .wrapping_mul(
                    ((*(*(*s1).dState).debug_info).n_sym + 1 as libc::c_int)
                        as libc::c_ulong,
                ),
        ) as *mut debug_sym;
        let fresh19 = (*(*(*s1).dState).debug_info).n_sym;
        (*(*(*s1).dState).debug_info).n_sym = (*(*(*s1).dState).debug_info).n_sym + 1;
        s = ((*(*(*s1).dState).debug_info).sym).offset(fresh19 as isize);
        (*s).type_0 = type_0;
        (*s).value = value;
        (*s).str_0 = tcc_strdup(str);
        (*s).sec = sec;
        (*s).sym_index = sym_index;
        (*s).info = info;
        (*s).file = (*(*s1).dState).dwarf_line.cur_file;
        (*s).line = (*file).line_num;
    } else if !sec.is_null() {
        put_stabs_r(
            s1,
            str,
            type_0,
            0 as libc::c_int,
            0 as libc::c_int,
            value,
            sec,
            sym_index,
        );
    } else {
        put_stabs(s1, str, type_0, 0 as libc::c_int, 0 as libc::c_int, value);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_stabn(
    mut s1: *mut TCCState,
    mut type_0: libc::c_int,
    mut value: libc::c_int,
) {
    if (*s1).do_debug == 0 {
        return;
    }
    if type_0 == N_LBRAC as libc::c_int {
        let mut info: *mut _debug_info = tcc_mallocz(
            ::core::mem::size_of::<_debug_info>() as libc::c_ulong,
        ) as *mut _debug_info;
        (*info).start = value;
        (*info).parent = (*(*s1).dState).debug_info;
        if !((*(*s1).dState).debug_info).is_null() {
            if !((*(*(*s1).dState).debug_info).child).is_null() {
                if !((*(*(*(*s1).dState).debug_info).child).last).is_null() {
                    (*(*(*(*(*s1).dState).debug_info).child).last).next = info;
                } else {
                    (*(*(*(*s1).dState).debug_info).child).next = info;
                }
                (*(*(*(*s1).dState).debug_info).child).last = info;
            } else {
                (*(*(*s1).dState).debug_info).child = info;
            }
        } else {
            (*(*s1).dState).debug_info_root = info;
        }
        (*(*s1).dState).debug_info = info;
    } else {
        (*(*(*s1).dState).debug_info).end = value;
        (*(*s1).dState).debug_info = (*(*(*s1).dState).debug_info).parent;
    };
}
unsafe extern "C" fn tcc_debug_find(
    mut s1: *mut TCCState,
    mut t: *mut Sym,
    mut dwarf: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if ((*(*s1).dState).debug_info).is_null() && dwarf != 0
        && (*t).type_0.t & 0xf as libc::c_int == 7 as libc::c_int
        && (*t).c2rust_unnamed.c2rust_unnamed.c == -(1 as libc::c_int)
    {
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).n_debug_anon_hash {
            if t == (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).type_0 {
                return 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        (*(*s1).dState)
            .debug_anon_hash = tcc_realloc(
            (*(*s1).dState).debug_anon_hash as *mut libc::c_void,
            (((*(*s1).dState).n_debug_anon_hash + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<_debug_anon_hash>() as libc::c_ulong,
                ),
        ) as *mut _debug_anon_hash;
        (*((*(*s1).dState).debug_anon_hash)
            .offset((*(*s1).dState).n_debug_anon_hash as isize))
            .n_debug_type = 0 as libc::c_int;
        let ref mut fresh20 = (*((*(*s1).dState).debug_anon_hash)
            .offset((*(*s1).dState).n_debug_anon_hash as isize))
            .debug_type;
        *fresh20 = 0 as *mut libc::c_int;
        let fresh21 = (*(*s1).dState).n_debug_anon_hash;
        (*(*s1).dState).n_debug_anon_hash = (*(*s1).dState).n_debug_anon_hash + 1;
        let ref mut fresh22 = (*((*(*s1).dState).debug_anon_hash)
            .offset(fresh21 as isize))
            .type_0;
        *fresh22 = t;
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*(*s1).dState).n_debug_hash {
        if t == (*((*(*s1).dState).debug_hash).offset(i as isize)).type_0 {
            return (*((*(*s1).dState).debug_hash).offset(i as isize)).debug_type;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn tcc_debug_check_anon(
    mut s1: *mut TCCState,
    mut t: *mut Sym,
    mut debug_type: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if ((*(*s1).dState).debug_info).is_null()
        && (*t).type_0.t & 0xf as libc::c_int == 7 as libc::c_int
        && (*(*t).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c == -(1 as libc::c_int)
    {
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).n_debug_anon_hash {
            if (*t).type_0.ref_0
                == (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).type_0
            {
                let ref mut fresh23 = (*((*(*s1).dState).debug_anon_hash)
                    .offset(i as isize))
                    .debug_type;
                *fresh23 = tcc_realloc(
                    (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).debug_type
                        as *mut libc::c_void,
                    (((*((*(*s1).dState).debug_anon_hash).offset(i as isize))
                        .n_debug_type + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int;
                let ref mut fresh24 = (*((*(*s1).dState).debug_anon_hash)
                    .offset(i as isize))
                    .n_debug_type;
                let fresh25 = *fresh24;
                *fresh24 = *fresh24 + 1;
                *((*((*(*s1).dState).debug_anon_hash).offset(i as isize)).debug_type)
                    .offset(fresh25 as isize) = debug_type;
            }
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_fix_anon(mut s1: *mut TCCState, mut t: *mut CType) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut debug_type: libc::c_int = 0;
    if (*s1).do_debug as libc::c_int & 2 as libc::c_int == 0 || (*s1).dwarf == 0
        || !((*(*s1).dState).debug_info).is_null()
    {
        return;
    }
    if (*t).t & 0xf as libc::c_int == 7 as libc::c_int
        && (*(*t).ref_0).c2rust_unnamed.c2rust_unnamed.c != -(1 as libc::c_int)
    {
        i = 0 as libc::c_int;
        while i < (*(*s1).dState).n_debug_anon_hash {
            if (*t).ref_0
                == (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).type_0
            {
                let mut sym: Sym = {
                    let mut init = Sym {
                        v: 0 as libc::c_int,
                        r: 0,
                        a: SymAttr {
                            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                        },
                        c2rust_unnamed: C2RustUnnamed_2 {
                            c2rust_unnamed: C2RustUnnamed_3 {
                                c: 0,
                                c2rust_unnamed: C2RustUnnamed_4 { sym_scope: 0 },
                            },
                        },
                        type_0: CType {
                            t: 0,
                            ref_0: 0 as *mut Sym,
                        },
                        c2rust_unnamed_0: C2RustUnnamed_1 {
                            next: 0 as *mut Sym,
                        },
                        prev: 0 as *mut Sym,
                        prev_tok: 0 as *mut Sym,
                    };
                    init
                };
                sym.type_0 = *t;
                (*(*s1).dState).debug_info = t as *mut _debug_info;
                debug_type = tcc_get_dwarf_info(s1, &mut sym);
                (*(*s1).dState).debug_info = 0 as *mut _debug_info;
                j = 0 as libc::c_int;
                while j
                    < (*((*(*s1).dState).debug_anon_hash).offset(i as isize))
                        .n_debug_type
                {
                    write32le(
                        ((*(*s1).dwarf_info_section).data)
                            .offset(
                                *((*((*(*s1).dState).debug_anon_hash).offset(i as isize))
                                    .debug_type)
                                    .offset(j as isize) as isize,
                            ),
                        (debug_type - (*(*s1).dState).dwarf_info.start) as uint32_t,
                    );
                    j += 1;
                    j;
                }
                tcc_free(
                    (*((*(*s1).dState).debug_anon_hash).offset(i as isize)).debug_type
                        as *mut libc::c_void,
                );
                (*(*s1).dState).n_debug_anon_hash -= 1;
                (*(*s1).dState).n_debug_anon_hash;
                while i < (*(*s1).dState).n_debug_anon_hash {
                    *((*(*s1).dState).debug_anon_hash)
                        .offset(
                            i as isize,
                        ) = *((*(*s1).dState).debug_anon_hash)
                        .offset((i + 1 as libc::c_int) as isize);
                    i += 1;
                    i;
                }
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn tcc_debug_add(
    mut s1: *mut TCCState,
    mut t: *mut Sym,
    mut dwarf: libc::c_int,
) -> libc::c_int {
    let mut offset: libc::c_int = (if dwarf != 0 {
        (*(*s1).dwarf_info_section).data_offset
    } else {
        (*(*s1).dState).debug_next_type += 1;
        (*(*s1).dState).debug_next_type as libc::c_ulong
    }) as libc::c_int;
    (*(*s1).dState)
        .debug_hash = tcc_realloc(
        (*(*s1).dState).debug_hash as *mut libc::c_void,
        (((*(*s1).dState).n_debug_hash + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<_debug_hash>() as libc::c_ulong),
    ) as *mut _debug_hash;
    (*((*(*s1).dState).debug_hash).offset((*(*s1).dState).n_debug_hash as isize))
        .debug_type = offset;
    let fresh26 = (*(*s1).dState).n_debug_hash;
    (*(*s1).dState).n_debug_hash = (*(*s1).dState).n_debug_hash + 1;
    let ref mut fresh27 = (*((*(*s1).dState).debug_hash).offset(fresh26 as isize))
        .type_0;
    *fresh27 = t;
    return offset;
}
unsafe extern "C" fn tcc_debug_remove(mut s1: *mut TCCState, mut t: *mut Sym) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*s1).dState).n_debug_hash {
        if t == (*((*(*s1).dState).debug_hash).offset(i as isize)).type_0 {
            (*(*s1).dState).n_debug_hash -= 1;
            (*(*s1).dState).n_debug_hash;
            while i < (*(*s1).dState).n_debug_hash {
                *((*(*s1).dState).debug_hash)
                    .offset(
                        i as isize,
                    ) = *((*(*s1).dState).debug_hash)
                    .offset((i + 1 as libc::c_int) as isize);
                i += 1;
                i;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn tcc_get_debug_info(
    mut s1: *mut TCCState,
    mut s: *mut Sym,
    mut result: *mut CString,
) {
    let mut type_0: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut debug_type: libc::c_int = -(1 as libc::c_int);
    let mut t: *mut Sym = s;
    let mut str: CString = CString {
        size: 0,
        size_allocated: 0,
        data: 0 as *mut libc::c_char,
    };
    loop {
        type_0 = (*t).type_0.t
            & !(0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                | 0x400 as libc::c_int);
        if type_0 & 0xf as libc::c_int != 1 as libc::c_int {
            type_0 &= !(0x20 as libc::c_int);
        }
        if !(type_0 == 5 as libc::c_int
            || type_0 == 5 as libc::c_int | 0x40 as libc::c_int)
        {
            break;
        }
        n += 1;
        n;
        t = (*t).type_0.ref_0;
    }
    if type_0 & 0xf as libc::c_int == 7 as libc::c_int {
        let mut e: *mut Sym = t;
        t = (*t).type_0.ref_0;
        debug_type = tcc_debug_find(s1, t, 0 as libc::c_int);
        if debug_type == -(1 as libc::c_int) {
            debug_type = tcc_debug_add(s1, t, 0 as libc::c_int);
            cstr_new(&mut str);
            cstr_printf(
                &mut str as *mut CString,
                b"%s:T%d=%c%d\0" as *const u8 as *const libc::c_char,
                if (*t).v & !(0x40000000 as libc::c_int) >= 0x10000000 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    get_tok_str((*t).v, 0 as *mut CValue)
                },
                debug_type,
                if (*t).type_0.t as libc::c_uint
                    & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint
                        | 0xf as libc::c_int as libc::c_uint)
                    == ((1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int)
                        as libc::c_uint
                {
                    'u' as i32
                } else {
                    's' as i32
                },
                (*t).c2rust_unnamed.c2rust_unnamed.c,
            );
            while !((*t).c2rust_unnamed_0.next).is_null() {
                let mut pos: libc::c_int = 0;
                let mut size: libc::c_int = 0;
                let mut align: libc::c_int = 0;
                t = (*t).c2rust_unnamed_0.next;
                if ((*t).a).nodebug() as libc::c_int != 0
                    || (*t).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int
                        && ((*t).type_0.t & 0xf as libc::c_int == 1 as libc::c_int
                            || (*t).type_0.t & 0xf as libc::c_int == 11 as libc::c_int
                            || (*t).type_0.t & 0xf as libc::c_int == 2 as libc::c_int
                            || (*t).type_0.t & 0xf as libc::c_int == 3 as libc::c_int
                            || (*t).type_0.t & 0xf as libc::c_int == 4 as libc::c_int)
                {
                    continue;
                }
                cstr_printf(
                    &mut str as *mut CString,
                    b"%s:\0" as *const u8 as *const libc::c_char,
                    get_tok_str((*t).v, 0 as *mut CValue),
                );
                tcc_get_debug_info(s1, t, &mut str);
                if (*t).type_0.t & 0x80 as libc::c_int != 0 {
                    pos = (*t).c2rust_unnamed.c2rust_unnamed.c * 8 as libc::c_int
                        + ((*t).type_0.t >> 20 as libc::c_int & 0x3f as libc::c_int);
                    size = (*t).type_0.t >> 20 as libc::c_int + 6 as libc::c_int
                        & 0x3f as libc::c_int;
                } else {
                    pos = (*t).c2rust_unnamed.c2rust_unnamed.c * 8 as libc::c_int;
                    size = type_size(&mut (*t).type_0, &mut align) * 8 as libc::c_int;
                }
                cstr_printf(
                    &mut str as *mut CString,
                    b",%d,%d;\0" as *const u8 as *const libc::c_char,
                    pos,
                    size,
                );
            }
            cstr_printf(
                &mut str as *mut CString,
                b";\0" as *const u8 as *const libc::c_char,
            );
            tcc_debug_stabs(
                s1,
                str.data,
                N_LSYM as libc::c_int,
                0 as libc::c_int as libc::c_ulong,
                0 as *mut Section,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            cstr_free(&mut str);
            if !((*(*s1).dState).debug_info).is_null() {
                tcc_debug_remove(s1, e);
            }
        }
    } else if type_0 as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
    {
        t = (*t).type_0.ref_0;
        let mut e_0: *mut Sym = t;
        debug_type = tcc_debug_find(s1, t, 0 as libc::c_int);
        if debug_type == -(1 as libc::c_int) {
            debug_type = tcc_debug_add(s1, t, 0 as libc::c_int);
            cstr_new(&mut str);
            cstr_printf(
                &mut str as *mut CString,
                b"%s:T%d=e\0" as *const u8 as *const libc::c_char,
                if (*t).v & !(0x40000000 as libc::c_int) >= 0x10000000 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    get_tok_str((*t).v, 0 as *mut CValue)
                },
                debug_type,
            );
            while !((*t).c2rust_unnamed_0.next).is_null() {
                t = (*t).c2rust_unnamed_0.next;
                cstr_printf(
                    &mut str as *mut CString,
                    b"%s:\0" as *const u8 as *const libc::c_char,
                    if (*t).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int
                    {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        get_tok_str((*t).v, 0 as *mut CValue)
                    },
                );
                cstr_printf(
                    &mut str as *mut CString,
                    if (*e_0).type_0.t & 0x10 as libc::c_int != 0 {
                        b"%u,\0" as *const u8 as *const libc::c_char
                    } else {
                        b"%d,\0" as *const u8 as *const libc::c_char
                    },
                    (*t).c2rust_unnamed.enum_val as libc::c_int,
                );
            }
            cstr_printf(
                &mut str as *mut CString,
                b";\0" as *const u8 as *const libc::c_char,
            );
            tcc_debug_stabs(
                s1,
                str.data,
                N_LSYM as libc::c_int,
                0 as libc::c_int as libc::c_ulong,
                0 as *mut Section,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            cstr_free(&mut str);
            if !((*(*s1).dState).debug_info).is_null() {
                tcc_debug_remove(s1, e_0);
            }
        }
    } else if type_0 & 0xf as libc::c_int != 6 as libc::c_int {
        type_0 = (type_0 as libc::c_uint
            & !(((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint)) as libc::c_int;
        debug_type = 1 as libc::c_int;
        while debug_type as libc::c_ulong
            <= (::core::mem::size_of::<[C2RustUnnamed_25; 29]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong,
                )
        {
            if default_debug[(debug_type - 1 as libc::c_int) as usize].type_0 == type_0 {
                break;
            }
            debug_type += 1;
            debug_type;
        }
        if debug_type as libc::c_ulong
            > (::core::mem::size_of::<[C2RustUnnamed_25; 29]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong,
                )
        {
            return;
        }
    }
    if n > 0 as libc::c_int {
        (*(*s1).dState).debug_next_type += 1;
        cstr_printf(
            result,
            b"%d=\0" as *const u8 as *const libc::c_char,
            (*(*s1).dState).debug_next_type,
        );
    }
    t = s;
    loop {
        type_0 = (*t).type_0.t
            & !(0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                | 0x400 as libc::c_int);
        if type_0 & 0xf as libc::c_int != 1 as libc::c_int {
            type_0 &= !(0x20 as libc::c_int);
        }
        if type_0 == 5 as libc::c_int {
            (*(*s1).dState).debug_next_type += 1;
            cstr_printf(
                result,
                b"%d=*\0" as *const u8 as *const libc::c_char,
                (*(*s1).dState).debug_next_type,
            );
        } else if type_0 == 5 as libc::c_int | 0x40 as libc::c_int {
            (*(*s1).dState).debug_next_type += 1;
            cstr_printf(
                result,
                b"%d=ar1;0;%d;\0" as *const u8 as *const libc::c_char,
                (*(*s1).dState).debug_next_type,
                (*(*t).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c - 1 as libc::c_int,
            );
        } else {
            if !(type_0 == 6 as libc::c_int) {
                break;
            }
            (*(*s1).dState).debug_next_type += 1;
            cstr_printf(
                result,
                b"%d=f\0" as *const u8 as *const libc::c_char,
                (*(*s1).dState).debug_next_type,
            );
            tcc_get_debug_info(s1, (*t).type_0.ref_0, result);
            return;
        }
        t = (*t).type_0.ref_0;
    }
    cstr_printf(result, b"%d\0" as *const u8 as *const libc::c_char, debug_type);
}
unsafe extern "C" fn tcc_get_dwarf_info(
    mut s1: *mut TCCState,
    mut s: *mut Sym,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut debug_type: libc::c_int = -(1 as libc::c_int);
    let mut e: *mut Sym = 0 as *mut Sym;
    let mut t: *mut Sym = s;
    let mut i: libc::c_int = 0;
    let mut last_pos: libc::c_int = -(1 as libc::c_int);
    let mut retval: libc::c_int = 0;
    if (*(*s1).dState).new_file != 0 {
        put_new_file(s1);
    }
    loop {
        type_0 = (*t).type_0.t
            & !(0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                | 0x400 as libc::c_int);
        if type_0 & 0xf as libc::c_int != 1 as libc::c_int {
            type_0 &= !(0x20 as libc::c_int);
        }
        if !(type_0 == 5 as libc::c_int
            || type_0 == 5 as libc::c_int | 0x40 as libc::c_int)
        {
            break;
        }
        t = (*t).type_0.ref_0;
    }
    if type_0 & 0xf as libc::c_int == 7 as libc::c_int {
        t = (*t).type_0.ref_0;
        debug_type = tcc_debug_find(s1, t, 1 as libc::c_int);
        if debug_type == -(1 as libc::c_int) {
            let mut pos_sib: libc::c_int = 0 as libc::c_int;
            let mut i_0: libc::c_int = 0;
            let mut pos_type: *mut libc::c_int = 0 as *mut libc::c_int;
            debug_type = tcc_debug_add(s1, t, 1 as libc::c_int);
            e = t;
            i_0 = 0 as libc::c_int;
            while !((*e).c2rust_unnamed_0.next).is_null() {
                e = (*e).c2rust_unnamed_0.next;
                if ((*e).a).nodebug() as libc::c_int != 0
                    || (*e).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int
                        && ((*e).type_0.t & 0xf as libc::c_int == 1 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 11 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 2 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 3 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 4 as libc::c_int)
                {
                    continue;
                }
                i_0 += 1;
                i_0;
            }
            pos_type = tcc_malloc(
                (i_0 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = (if (*t).type_0.t as libc::c_uint
                & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint
                    | 0xf as libc::c_int as libc::c_uint)
                == ((1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int)
                    as libc::c_uint
            {
                if !((*t).c2rust_unnamed_0.next).is_null() {
                    18 as libc::c_int
                } else {
                    19 as libc::c_int
                }
            } else if !((*t).c2rust_unnamed_0.next).is_null() {
                16 as libc::c_int
            } else {
                17 as libc::c_int
            }) as uint8_t;
            dwarf_strp(
                (*s1).dwarf_info_section,
                if (*t).v & !(0x40000000 as libc::c_int) >= 0x10000000 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    get_tok_str((*t).v, 0 as *mut CValue)
                },
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*t).c2rust_unnamed.c2rust_unnamed.c as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*file).line_num as libc::c_ulonglong,
            );
            if !((*t).c2rust_unnamed_0.next).is_null() {
                pos_sib = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    0 as libc::c_int as uint32_t,
                );
            }
            e = t;
            i_0 = 0 as libc::c_int;
            while !((*e).c2rust_unnamed_0.next).is_null() {
                e = (*e).c2rust_unnamed_0.next;
                if ((*e).a).nodebug() as libc::c_int != 0
                    || (*e).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int
                        && ((*e).type_0.t & 0xf as libc::c_int == 1 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 11 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 2 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 3 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 4 as libc::c_int)
                {
                    continue;
                }
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                )
                    as *mut uint8_t) = (if (*e).type_0.t & 0x80 as libc::c_int != 0 {
                    15 as libc::c_int
                } else {
                    14 as libc::c_int
                }) as uint8_t;
                dwarf_strp(
                    (*s1).dwarf_info_section,
                    get_tok_str((*e).v, 0 as *mut CValue),
                );
                dwarf_uleb128(
                    (*s1).dwarf_info_section,
                    (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
                );
                dwarf_uleb128(
                    (*s1).dwarf_info_section,
                    (*file).line_num as libc::c_ulonglong,
                );
                let fresh28 = i_0;
                i_0 = i_0 + 1;
                *pos_type
                    .offset(
                        fresh28 as isize,
                    ) = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    0 as libc::c_int as uint32_t,
                );
                if (*e).type_0.t & 0x80 as libc::c_int != 0 {
                    let mut pos: libc::c_int = (*e).c2rust_unnamed.c2rust_unnamed.c
                        * 8 as libc::c_int
                        + ((*e).type_0.t >> 20 as libc::c_int & 0x3f as libc::c_int);
                    let mut size: libc::c_int = (*e).type_0.t
                        >> 20 as libc::c_int + 6 as libc::c_int & 0x3f as libc::c_int;
                    dwarf_uleb128((*s1).dwarf_info_section, size as libc::c_ulonglong);
                    dwarf_uleb128((*s1).dwarf_info_section, pos as libc::c_ulonglong);
                } else {
                    dwarf_uleb128(
                        (*s1).dwarf_info_section,
                        (*e).c2rust_unnamed.c2rust_unnamed.c as libc::c_ulonglong,
                    );
                }
            }
            if !((*t).c2rust_unnamed_0.next).is_null() {
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 0 as libc::c_int as uint8_t;
                write32le(
                    ((*(*s1).dwarf_info_section).data).offset(pos_sib as isize),
                    ((*(*s1).dwarf_info_section).data_offset)
                        .wrapping_sub((*(*s1).dState).dwarf_info.start as libc::c_ulong)
                        as uint32_t,
                );
            }
            e = t;
            i_0 = 0 as libc::c_int;
            while !((*e).c2rust_unnamed_0.next).is_null() {
                e = (*e).c2rust_unnamed_0.next;
                if ((*e).a).nodebug() as libc::c_int != 0
                    || (*e).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int
                        && ((*e).type_0.t & 0xf as libc::c_int == 1 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 11 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 2 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 3 as libc::c_int
                            || (*e).type_0.t & 0xf as libc::c_int == 4 as libc::c_int)
                {
                    continue;
                }
                type_0 = tcc_get_dwarf_info(s1, e);
                tcc_debug_check_anon(s1, e, *pos_type.offset(i_0 as isize));
                let fresh29 = i_0;
                i_0 = i_0 + 1;
                write32le(
                    ((*(*s1).dwarf_info_section).data)
                        .offset(*pos_type.offset(fresh29 as isize) as isize),
                    (type_0 - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
            }
            tcc_free(pos_type as *mut libc::c_void);
            if !((*(*s1).dState).debug_info).is_null() {
                tcc_debug_remove(s1, t);
            }
        }
    } else if type_0 as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
    {
        t = (*t).type_0.ref_0;
        debug_type = tcc_debug_find(s1, t, 1 as libc::c_int);
        if debug_type == -(1 as libc::c_int) {
            let mut pos_sib_0: libc::c_int = 0;
            let mut pos_type_0: libc::c_int = 0;
            let mut sym: Sym = {
                let mut init = Sym {
                    v: 0 as libc::c_int,
                    r: 0,
                    a: SymAttr {
                        aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                    },
                    c2rust_unnamed: C2RustUnnamed_2 {
                        c2rust_unnamed: C2RustUnnamed_3 {
                            c: 0,
                            c2rust_unnamed: C2RustUnnamed_4 { sym_scope: 0 },
                        },
                    },
                    type_0: CType {
                        t: 0,
                        ref_0: 0 as *mut Sym,
                    },
                    c2rust_unnamed_0: C2RustUnnamed_1 {
                        next: 0 as *mut Sym,
                    },
                    prev: 0 as *mut Sym,
                    prev_tok: 0 as *mut Sym,
                };
                init
            };
            sym.type_0.t = 3 as libc::c_int | type_0 & 0x10 as libc::c_int;
            pos_type_0 = tcc_get_dwarf_info(s1, &mut sym);
            debug_type = tcc_debug_add(s1, t, 1 as libc::c_int);
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 13 as libc::c_int as uint8_t;
            dwarf_strp(
                (*s1).dwarf_info_section,
                if (*t).v & !(0x40000000 as libc::c_int) >= 0x10000000 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    get_tok_str((*t).v, 0 as *mut CValue)
                },
            );
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = (if type_0 & 0x10 as libc::c_int != 0 {
                DW_ATE_unsigned as libc::c_int
            } else {
                DW_ATE_signed as libc::c_int
            }) as uint8_t;
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 4 as libc::c_int as uint8_t;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                (pos_type_0 - (*(*s1).dState).dwarf_info.start) as uint32_t,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*file).line_num as libc::c_ulonglong,
            );
            pos_sib_0 = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            e = t;
            while !((*e).c2rust_unnamed_0.next).is_null() {
                e = (*e).c2rust_unnamed_0.next;
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                )
                    as *mut uint8_t) = (if type_0 & 0x10 as libc::c_int != 0 {
                    12 as libc::c_int
                } else {
                    11 as libc::c_int
                }) as uint8_t;
                dwarf_strp(
                    (*s1).dwarf_info_section,
                    if (*e).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int
                    {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        get_tok_str((*e).v, 0 as *mut CValue)
                    },
                );
                if type_0 & 0x10 as libc::c_int != 0 {
                    dwarf_uleb128(
                        (*s1).dwarf_info_section,
                        (*e).c2rust_unnamed.enum_val as libc::c_ulonglong,
                    );
                } else {
                    dwarf_sleb128(
                        (*s1).dwarf_info_section,
                        (*e).c2rust_unnamed.enum_val,
                    );
                }
            }
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 0 as libc::c_int as uint8_t;
            write32le(
                ((*(*s1).dwarf_info_section).data).offset(pos_sib_0 as isize),
                ((*(*s1).dwarf_info_section).data_offset)
                    .wrapping_sub((*(*s1).dState).dwarf_info.start as libc::c_ulong)
                    as uint32_t,
            );
            if !((*(*s1).dState).debug_info).is_null() {
                tcc_debug_remove(s1, t);
            }
        }
    } else if type_0 & 0xf as libc::c_int != 6 as libc::c_int {
        type_0 = (type_0 as libc::c_uint
            & !(((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint)) as libc::c_int;
        i = 1 as libc::c_int;
        while i as libc::c_ulong
            <= (::core::mem::size_of::<[C2RustUnnamed_25; 29]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong,
                )
        {
            if default_debug[(i - 1 as libc::c_int) as usize].type_0 == type_0 {
                break;
            }
            i += 1;
            i;
        }
        if i as libc::c_ulong
            > (::core::mem::size_of::<[C2RustUnnamed_25; 29]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong,
                )
        {
            return 0 as libc::c_int;
        }
        debug_type = (*(*s1).dState)
            .dwarf_info
            .base_type_used[(i - 1 as libc::c_int) as usize];
        if debug_type == 0 as libc::c_int {
            let mut name: [libc::c_char; 100] = [0; 100];
            debug_type = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 2 as libc::c_int as uint8_t;
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                default_debug[(i - 1 as libc::c_int) as usize].size as libc::c_ulonglong,
            );
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = default_debug[(i - 1 as libc::c_int) as usize]
                .encoding as uint8_t;
            strncpy(
                name.as_mut_ptr(),
                default_debug[(i - 1 as libc::c_int) as usize].name,
                (::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            *strchr(name.as_mut_ptr(), ':' as i32) = 0 as libc::c_int as libc::c_char;
            dwarf_strp((*s1).dwarf_info_section, name.as_mut_ptr());
            (*(*s1).dState)
                .dwarf_info
                .base_type_used[(i - 1 as libc::c_int) as usize] = debug_type;
        }
    }
    retval = debug_type;
    e = 0 as *mut Sym;
    t = s;
    loop {
        type_0 = (*t).type_0.t
            & !(0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                | 0x400 as libc::c_int);
        if type_0 & 0xf as libc::c_int != 1 as libc::c_int {
            type_0 &= !(0x20 as libc::c_int);
        }
        if type_0 == 5 as libc::c_int {
            i = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            if retval == debug_type {
                retval = i;
            }
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 7 as libc::c_int as uint8_t;
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 8 as libc::c_int as uint8_t;
            if last_pos != -(1 as libc::c_int) {
                tcc_debug_check_anon(s1, e, last_pos);
                write32le(
                    ((*(*s1).dwarf_info_section).data).offset(last_pos as isize),
                    (i - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
            }
            last_pos = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            e = (*t).type_0.ref_0;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
        } else if type_0 == 5 as libc::c_int | 0x40 as libc::c_int {
            let mut sib_pos: libc::c_int = 0;
            let mut sub_type: libc::c_int = 0;
            let mut sym_0: Sym = {
                let mut init = Sym {
                    v: 0 as libc::c_int,
                    r: 0,
                    a: SymAttr {
                        aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                    },
                    c2rust_unnamed: C2RustUnnamed_2 {
                        c2rust_unnamed: C2RustUnnamed_3 {
                            c: 0,
                            c2rust_unnamed: C2RustUnnamed_4 { sym_scope: 0 },
                        },
                    },
                    type_0: CType {
                        t: 0,
                        ref_0: 0 as *mut Sym,
                    },
                    c2rust_unnamed_0: C2RustUnnamed_1 {
                        next: 0 as *mut Sym,
                    },
                    prev: 0 as *mut Sym,
                    prev_tok: 0 as *mut Sym,
                };
                init
            };
            sym_0
                .type_0
                .t = 4 as libc::c_int | 0x800 as libc::c_int | 0x10 as libc::c_int;
            sub_type = tcc_get_dwarf_info(s1, &mut sym_0);
            i = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            if retval == debug_type {
                retval = i;
            }
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 8 as libc::c_int as uint8_t;
            if last_pos != -(1 as libc::c_int) {
                tcc_debug_check_anon(s1, e, last_pos);
                write32le(
                    ((*(*s1).dwarf_info_section).data).offset(last_pos as isize),
                    (i - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
            }
            last_pos = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            e = (*t).type_0.ref_0;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            sib_pos = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            loop {
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 9 as libc::c_int as uint8_t;
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    (sub_type - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
                dwarf_uleb128(
                    (*s1).dwarf_info_section,
                    ((*(*t).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                        - 1 as libc::c_int) as libc::c_ulonglong,
                );
                s = (*t).type_0.ref_0;
                type_0 = (*s).type_0.t
                    & !(0x1000 as libc::c_int | 0x2000 as libc::c_int
                        | 0x4000 as libc::c_int | 0x8000 as libc::c_int
                        | 0x100 as libc::c_int | 0x200 as libc::c_int);
                if type_0 != 5 as libc::c_int | 0x40 as libc::c_int {
                    break;
                }
                t = s;
            }
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 0 as libc::c_int as uint8_t;
            write32le(
                ((*(*s1).dwarf_info_section).data).offset(sib_pos as isize),
                ((*(*s1).dwarf_info_section).data_offset)
                    .wrapping_sub((*(*s1).dState).dwarf_info.start as libc::c_ulong)
                    as uint32_t,
            );
        } else if type_0 == 6 as libc::c_int {
            let mut sib_pos_0: libc::c_int = 0 as libc::c_int;
            let mut pos_type_1: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut f: *mut Sym = 0 as *mut Sym;
            i = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            debug_type = tcc_get_dwarf_info(s1, (*t).type_0.ref_0);
            if retval == debug_type {
                retval = i;
            }
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = (if !((*(*t).type_0.ref_0).c2rust_unnamed_0.next)
                .is_null()
            {
                24 as libc::c_int
            } else {
                25 as libc::c_int
            }) as uint8_t;
            if last_pos != -(1 as libc::c_int) {
                tcc_debug_check_anon(s1, e, last_pos);
                write32le(
                    ((*(*s1).dwarf_info_section).data).offset(last_pos as isize),
                    (i - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
            }
            last_pos = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
            e = (*t).type_0.ref_0;
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                0 as libc::c_int as uint32_t,
            );
            if !((*(*t).type_0.ref_0).c2rust_unnamed_0.next).is_null() {
                sib_pos_0 = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    0 as libc::c_int as uint32_t,
                );
            }
            f = (*t).type_0.ref_0;
            i = 0 as libc::c_int;
            while !((*f).c2rust_unnamed_0.next).is_null() {
                f = (*f).c2rust_unnamed_0.next;
                i += 1;
                i;
            }
            pos_type_1 = tcc_malloc(
                (i as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            f = (*t).type_0.ref_0;
            i = 0 as libc::c_int;
            while !((*f).c2rust_unnamed_0.next).is_null() {
                f = (*f).c2rust_unnamed_0.next;
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 26 as libc::c_int as uint8_t;
                let fresh30 = i;
                i = i + 1;
                *pos_type_1
                    .offset(
                        fresh30 as isize,
                    ) = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    0 as libc::c_int as uint32_t,
                );
            }
            if !((*(*t).type_0.ref_0).c2rust_unnamed_0.next).is_null() {
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 0 as libc::c_int as uint8_t;
                write32le(
                    ((*(*s1).dwarf_info_section).data).offset(sib_pos_0 as isize),
                    ((*(*s1).dwarf_info_section).data_offset)
                        .wrapping_sub((*(*s1).dState).dwarf_info.start as libc::c_ulong)
                        as uint32_t,
                );
            }
            f = (*t).type_0.ref_0;
            i = 0 as libc::c_int;
            while !((*f).c2rust_unnamed_0.next).is_null() {
                f = (*f).c2rust_unnamed_0.next;
                type_0 = tcc_get_dwarf_info(s1, f);
                tcc_debug_check_anon(s1, f, *pos_type_1.offset(i as isize));
                let fresh31 = i;
                i = i + 1;
                write32le(
                    ((*(*s1).dwarf_info_section).data)
                        .offset(*pos_type_1.offset(fresh31 as isize) as isize),
                    (type_0 - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
            }
            tcc_free(pos_type_1 as *mut libc::c_void);
        } else {
            if last_pos != -(1 as libc::c_int) {
                tcc_debug_check_anon(s1, e, last_pos);
                write32le(
                    ((*(*s1).dwarf_info_section).data).offset(last_pos as isize),
                    (debug_type - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
            }
            break;
        }
        t = (*t).type_0.ref_0;
    }
    return retval;
}
unsafe extern "C" fn tcc_debug_finish(mut s1: *mut TCCState, mut cur: *mut _debug_info) {
    while !cur.is_null() {
        let mut next: *mut _debug_info = (*cur).next;
        let mut i: libc::c_int = 0;
        if (*s1).dwarf != 0 {
            i = (*cur).n_sym - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                let mut s: *mut debug_sym = &mut *((*cur).sym).offset(i as isize)
                    as *mut debug_sym;
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                )
                    as *mut uint8_t) = (if (*s).type_0 == N_PSYM as libc::c_int {
                    6 as libc::c_int
                } else if (*s).type_0 == N_GSYM as libc::c_int {
                    3 as libc::c_int
                } else if (*s).type_0 == N_STSYM as libc::c_int {
                    4 as libc::c_int
                } else {
                    5 as libc::c_int
                }) as uint8_t;
                dwarf_strp((*s1).dwarf_info_section, (*s).str_0);
                if (*s).type_0 == N_GSYM as libc::c_int
                    || (*s).type_0 == N_STSYM as libc::c_int
                {
                    dwarf_uleb128(
                        (*s1).dwarf_info_section,
                        (*s).file as libc::c_ulonglong,
                    );
                    dwarf_uleb128(
                        (*s1).dwarf_info_section,
                        (*s).line as libc::c_ulonglong,
                    );
                }
                write32le(
                    section_ptr_add(
                        (*s1).dwarf_info_section,
                        4 as libc::c_int as Elf64_Addr,
                    ) as *mut libc::c_uchar,
                    ((*s).info - (*(*s1).dState).dwarf_info.start) as uint32_t,
                );
                if (*s).type_0 == N_GSYM as libc::c_int
                    || (*s).type_0 == N_STSYM as libc::c_int
                {
                    if (*s).type_0 == N_GSYM as libc::c_int {
                        *(section_ptr_add(
                            (*s1).dwarf_info_section,
                            1 as libc::c_int as Elf64_Addr,
                        ) as *mut uint8_t) = 1 as libc::c_int as uint8_t;
                    }
                    *(section_ptr_add(
                        (*s1).dwarf_info_section,
                        1 as libc::c_int as Elf64_Addr,
                    )
                        as *mut uint8_t) = (8 as libc::c_int + 1 as libc::c_int)
                        as uint8_t;
                    *(section_ptr_add(
                        (*s1).dwarf_info_section,
                        1 as libc::c_int as Elf64_Addr,
                    ) as *mut uint8_t) = DW_OP_addr as libc::c_int as uint8_t;
                    if (*s).type_0 == N_STSYM as libc::c_int {
                        dwarf_reloc(
                            (*s1).dwarf_info_section,
                            (*(*s1).dState).section_sym,
                            1 as libc::c_int,
                        );
                    }
                    write64le(
                        section_ptr_add(
                            (*s1).dwarf_info_section,
                            8 as libc::c_int as Elf64_Addr,
                        ) as *mut libc::c_uchar,
                        (*s).value,
                    );
                } else {
                    *(section_ptr_add(
                        (*s1).dwarf_info_section,
                        1 as libc::c_int as Elf64_Addr,
                    )
                        as *mut uint8_t) = (dwarf_sleb128_size(
                        (*s).value as libc::c_longlong,
                    ) + 1 as libc::c_int) as uint8_t;
                    *(section_ptr_add(
                        (*s1).dwarf_info_section,
                        1 as libc::c_int as Elf64_Addr,
                    ) as *mut uint8_t) = DW_OP_fbreg as libc::c_int as uint8_t;
                    dwarf_sleb128(
                        (*s1).dwarf_info_section,
                        (*s).value as libc::c_longlong,
                    );
                }
                tcc_free((*s).str_0 as *mut libc::c_void);
                i -= 1;
                i;
            }
            tcc_free((*cur).sym as *mut libc::c_void);
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = (if !((*cur).child).is_null() {
                22 as libc::c_int
            } else {
                23 as libc::c_int
            }) as uint8_t;
            dwarf_reloc(
                (*s1).dwarf_info_section,
                (*(*s1).dState).section_sym,
                1 as libc::c_int,
            );
            write64le(
                section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                (func_ind + (*cur).start) as uint64_t,
            );
            write64le(
                section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                ((*cur).end - (*cur).start) as uint64_t,
            );
            tcc_debug_finish(s1, (*cur).child);
            if !((*cur).child).is_null() {
                *(section_ptr_add(
                    (*s1).dwarf_info_section,
                    1 as libc::c_int as Elf64_Addr,
                ) as *mut uint8_t) = 0 as libc::c_int as uint8_t;
            }
        } else {
            i = 0 as libc::c_int;
            while i < (*cur).n_sym {
                let mut s_0: *mut debug_sym = &mut *((*cur).sym).offset(i as isize)
                    as *mut debug_sym;
                if !((*s_0).sec).is_null() {
                    put_stabs_r(
                        s1,
                        (*s_0).str_0,
                        (*s_0).type_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        (*s_0).value,
                        (*s_0).sec,
                        (*s_0).sym_index,
                    );
                } else {
                    put_stabs(
                        s1,
                        (*s_0).str_0,
                        (*s_0).type_0,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        (*s_0).value,
                    );
                }
                tcc_free((*s_0).str_0 as *mut libc::c_void);
                i += 1;
                i;
            }
            tcc_free((*cur).sym as *mut libc::c_void);
            put_stabn(
                s1,
                N_LBRAC as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*cur).start,
            );
            tcc_debug_finish(s1, (*cur).child);
            put_stabn(
                s1,
                N_RBRAC as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*cur).end,
            );
        }
        tcc_free(cur as *mut libc::c_void);
        cur = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_add_debug_info(
    mut s1: *mut TCCState,
    mut param: libc::c_int,
    mut s: *mut Sym,
    mut e: *mut Sym,
) {
    let mut debug_str: CString = CString {
        size: 0,
        size_allocated: 0,
        data: 0 as *mut libc::c_char,
    };
    if (*s1).do_debug as libc::c_int & 2 as libc::c_int == 0 {
        return;
    }
    cstr_new(&mut debug_str);
    while s != e {
        if !((*s).v == 0
            || (*s).r as libc::c_int & 0x3f as libc::c_int != 0x32 as libc::c_int)
        {
            if (*s1).dwarf != 0 {
                tcc_debug_stabs(
                    s1,
                    get_tok_str((*s).v, 0 as *mut CValue),
                    if param != 0 {
                        N_PSYM as libc::c_int
                    } else {
                        N_LSYM as libc::c_int
                    },
                    (*s).c2rust_unnamed.c2rust_unnamed.c as libc::c_ulong,
                    0 as *mut Section,
                    0 as libc::c_int,
                    tcc_get_dwarf_info(s1, s),
                );
            } else {
                cstr_reset(&mut debug_str);
                cstr_printf(
                    &mut debug_str as *mut CString,
                    b"%s:%s\0" as *const u8 as *const libc::c_char,
                    get_tok_str((*s).v, 0 as *mut CValue),
                    if param != 0 {
                        b"p\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                tcc_get_debug_info(s1, s, &mut debug_str);
                tcc_debug_stabs(
                    s1,
                    debug_str.data,
                    if param != 0 {
                        N_PSYM as libc::c_int
                    } else {
                        N_LSYM as libc::c_int
                    },
                    (*s).c2rust_unnamed.c2rust_unnamed.c as libc::c_ulong,
                    0 as *mut Section,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        }
        s = (*s).prev;
    }
    cstr_free(&mut debug_str);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_funcstart(mut s1: *mut TCCState, mut sym: *mut Sym) {
    let mut debug_str: CString = CString {
        size: 0,
        size_allocated: 0,
        data: 0 as *mut libc::c_char,
    };
    let mut f: *mut BufferedFile = 0 as *mut BufferedFile;
    if (*s1).do_debug == 0 {
        return;
    }
    (*(*s1).dState).debug_info_root = 0 as *mut _debug_info;
    (*(*s1).dState).debug_info = 0 as *mut _debug_info;
    tcc_debug_stabn(s1, N_LBRAC as libc::c_int, ind - func_ind);
    f = put_new_file(s1);
    if f.is_null() {
        return;
    }
    if (*s1).dwarf != 0 {
        tcc_debug_line(s1);
        (*(*s1).dState).dwarf_info.func = sym;
        (*(*s1).dState).dwarf_info.line = (*file).line_num;
        if (*s1).do_backtrace != 0 {
            let mut i: libc::c_int = 0;
            let mut len: libc::c_int = 0;
            dwarf_line_op(s1, 0 as libc::c_int as libc::c_uchar);
            dwarf_uleb128_op(
                s1,
                (strlen(funcname)).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as libc::c_ulonglong,
            );
            dwarf_line_op(
                s1,
                (DW_LNE_hi_user as libc::c_int - 1 as libc::c_int) as libc::c_uchar,
            );
            len = (strlen(funcname)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            i = 0 as libc::c_int;
            while i < len {
                dwarf_line_op(s1, *funcname.offset(i as isize) as libc::c_uchar);
                i += 1;
                i;
            }
        }
    } else {
        cstr_new(&mut debug_str);
        cstr_printf(
            &mut debug_str as *mut CString,
            b"%s:%c\0" as *const u8 as *const libc::c_char,
            funcname,
            if (*sym).type_0.t & 0x2000 as libc::c_int != 0 {
                'f' as i32
            } else {
                'F' as i32
            },
        );
        tcc_get_debug_info(s1, (*sym).type_0.ref_0, &mut debug_str);
        put_stabs_r(
            s1,
            debug_str.data,
            N_FUN as libc::c_int,
            0 as libc::c_int,
            (*f).line_num,
            0 as libc::c_int as libc::c_ulong,
            (*s1).cur_text_section,
            (*sym).c2rust_unnamed.c2rust_unnamed.c,
        );
        cstr_free(&mut debug_str);
        tcc_debug_line(s1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_prolog_epilog(
    mut s1: *mut TCCState,
    mut value: libc::c_int,
) {
    if (*s1).do_debug == 0 {
        return;
    }
    if (*s1).dwarf != 0 {
        dwarf_line_op(
            s1,
            (if value == 0 as libc::c_int {
                DW_LNS_set_prologue_end as libc::c_int
            } else {
                DW_LNS_set_epilogue_begin as libc::c_int
            }) as libc::c_uchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_funcend(
    mut s1: *mut TCCState,
    mut size: libc::c_int,
) {
    let mut min_instr_len: libc::c_int = 0;
    tcc_debug_frame_end(s1, size);
    if (*s1).do_debug == 0 {
        return;
    }
    min_instr_len = if (*(*s1).dState).dwarf_line.last_pc == ind {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    ind -= min_instr_len;
    tcc_debug_line(s1);
    ind += min_instr_len;
    tcc_debug_stabn(s1, N_RBRAC as libc::c_int, size);
    if (*s1).dwarf != 0 {
        let mut func_sib: libc::c_int = 0 as libc::c_int;
        let mut sym: *mut Sym = (*(*s1).dState).dwarf_info.func;
        let mut n_debug_info: libc::c_int = tcc_get_dwarf_info(s1, (*sym).type_0.ref_0);
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = (if (*sym).type_0.t & 0x2000 as libc::c_int != 0 {
            21 as libc::c_int
        } else {
            20 as libc::c_int
        }) as uint8_t;
        if (*sym).type_0.t & 0x2000 as libc::c_int == 0 as libc::c_int {
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 1 as libc::c_int as uint8_t;
        }
        dwarf_strp((*s1).dwarf_info_section, funcname);
        dwarf_uleb128(
            (*s1).dwarf_info_section,
            (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
        );
        dwarf_uleb128(
            (*s1).dwarf_info_section,
            (*(*s1).dState).dwarf_info.line as libc::c_ulonglong,
        );
        tcc_debug_check_anon(
            s1,
            (*sym).type_0.ref_0,
            (*(*s1).dwarf_info_section).data_offset as libc::c_int,
        );
        write32le(
            section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            (n_debug_info - (*(*s1).dState).dwarf_info.start) as uint32_t,
        );
        dwarf_reloc(
            (*s1).dwarf_info_section,
            (*(*s1).dState).section_sym,
            1 as libc::c_int,
        );
        write64le(
            section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            func_ind as uint64_t,
        );
        write64le(
            section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            size as uint64_t,
        );
        func_sib = (*(*s1).dwarf_info_section).data_offset as libc::c_int;
        write32le(
            section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint32_t,
        );
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = 1 as libc::c_int as uint8_t;
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = DW_OP_reg6 as libc::c_int as uint8_t;
        tcc_debug_finish(s1, (*(*s1).dState).debug_info_root);
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = 0 as libc::c_int as uint8_t;
        write32le(
            ((*(*s1).dwarf_info_section).data).offset(func_sib as isize),
            ((*(*s1).dwarf_info_section).data_offset)
                .wrapping_sub((*(*s1).dState).dwarf_info.start as libc::c_ulong)
                as uint32_t,
        );
    } else {
        tcc_debug_finish(s1, (*(*s1).dState).debug_info_root);
    }
    (*(*s1).dState).debug_info_root = 0 as *mut _debug_info;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_extern_sym(
    mut s1: *mut TCCState,
    mut sym: *mut Sym,
    mut sh_num: libc::c_int,
    mut sym_bind: libc::c_int,
    mut sym_type: libc::c_int,
) {
    if (*s1).do_debug as libc::c_int & 2 as libc::c_int == 0 {
        return;
    }
    if sym_type == 2 as libc::c_int || (*sym).v >= 0x10000000 as libc::c_int {
        return;
    }
    if (*s1).dwarf != 0 {
        let mut debug_type: libc::c_int = 0;
        debug_type = tcc_get_dwarf_info(s1, sym);
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = (if sym_bind == 1 as libc::c_int {
            3 as libc::c_int
        } else {
            4 as libc::c_int
        }) as uint8_t;
        dwarf_strp((*s1).dwarf_info_section, get_tok_str((*sym).v, 0 as *mut CValue));
        dwarf_uleb128(
            (*s1).dwarf_info_section,
            (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
        );
        dwarf_uleb128((*s1).dwarf_info_section, (*file).line_num as libc::c_ulonglong);
        tcc_debug_check_anon(
            s1,
            sym,
            (*(*s1).dwarf_info_section).data_offset as libc::c_int,
        );
        write32le(
            section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            (debug_type - (*(*s1).dState).dwarf_info.start) as uint32_t,
        );
        if sym_bind == 1 as libc::c_int {
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 1 as libc::c_int as uint8_t;
        }
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = (8 as libc::c_int + 1 as libc::c_int) as uint8_t;
        *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
            as *mut uint8_t) = DW_OP_addr as libc::c_int as uint8_t;
        greloca(
            (*s1).dwarf_info_section,
            sym,
            (*(*s1).dwarf_info_section).data_offset,
            1 as libc::c_int,
            0 as libc::c_int as Elf64_Addr,
        );
        write64le(
            section_ptr_add((*s1).dwarf_info_section, 8 as libc::c_int as Elf64_Addr)
                as *mut libc::c_uchar,
            0 as libc::c_int as uint64_t,
        );
    } else {
        let mut s: *mut Section = if sh_num == 0xfff2 as libc::c_int {
            (*s1).common_section
        } else {
            *((*s1).sections).offset(sh_num as isize)
        };
        let mut str: CString = CString {
            size: 0,
            size_allocated: 0,
            data: 0 as *mut libc::c_char,
        };
        cstr_new(&mut str);
        cstr_printf(
            &mut str as *mut CString,
            b"%s:%c\0" as *const u8 as *const libc::c_char,
            get_tok_str((*sym).v, 0 as *mut CValue),
            if sym_bind == 1 as libc::c_int {
                'G' as i32
            } else if func_ind != -(1 as libc::c_int) {
                'V' as i32
            } else {
                'S' as i32
            },
        );
        tcc_get_debug_info(s1, sym, &mut str);
        if sym_bind == 1 as libc::c_int {
            tcc_debug_stabs(
                s1,
                str.data,
                N_GSYM as libc::c_int,
                0 as libc::c_int as libc::c_ulong,
                0 as *mut Section,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            tcc_debug_stabs(
                s1,
                str.data,
                if (*sym).type_0.t & 0x2000 as libc::c_int != 0
                    && (*s1).data_section == s
                {
                    N_STSYM as libc::c_int
                } else {
                    N_LCSYM as libc::c_int
                },
                0 as libc::c_int as libc::c_ulong,
                s,
                (*sym).c2rust_unnamed.c2rust_unnamed.c,
                0 as libc::c_int,
            );
        }
        cstr_free(&mut str);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_debug_typedef(mut s1: *mut TCCState, mut sym: *mut Sym) {
    if (*s1).do_debug as libc::c_int & 2 as libc::c_int == 0 {
        return;
    }
    if (*s1).dwarf != 0 {
        let mut debug_type: libc::c_int = 0;
        debug_type = tcc_get_dwarf_info(s1, sym);
        if debug_type != -(1 as libc::c_int) {
            *(section_ptr_add((*s1).dwarf_info_section, 1 as libc::c_int as Elf64_Addr)
                as *mut uint8_t) = 10 as libc::c_int as uint8_t;
            dwarf_strp(
                (*s1).dwarf_info_section,
                get_tok_str((*sym).v, 0 as *mut CValue),
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*(*s1).dState).dwarf_line.cur_file as libc::c_ulonglong,
            );
            dwarf_uleb128(
                (*s1).dwarf_info_section,
                (*file).line_num as libc::c_ulonglong,
            );
            tcc_debug_check_anon(
                s1,
                sym,
                (*(*s1).dwarf_info_section).data_offset as libc::c_int,
            );
            write32le(
                section_ptr_add((*s1).dwarf_info_section, 4 as libc::c_int as Elf64_Addr)
                    as *mut libc::c_uchar,
                (debug_type - (*(*s1).dState).dwarf_info.start) as uint32_t,
            );
        }
    } else {
        let mut str: CString = CString {
            size: 0,
            size_allocated: 0,
            data: 0 as *mut libc::c_char,
        };
        cstr_new(&mut str);
        cstr_printf(
            &mut str as *mut CString,
            b"%s:t\0" as *const u8 as *const libc::c_char,
            if (*sym).v & !(0x20000000 as libc::c_int) >= 0x10000000 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                get_tok_str((*sym).v, 0 as *mut CValue)
            },
        );
        tcc_get_debug_info(s1, sym, &mut str);
        tcc_debug_stabs(
            s1,
            str.data,
            N_LSYM as libc::c_int,
            0 as libc::c_int as libc::c_ulong,
            0 as *mut Section,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        cstr_free(&mut str);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tcov_block_begin(mut s1: *mut TCCState) {
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_23 {
            c2rust_unnamed: C2RustUnnamed_24 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_21 {
            c2rust_unnamed: C2RustUnnamed_22 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut last_offset: libc::c_ulong = (*(*s1).dState).tcov_data.offset;
    tcc_tcov_block_end(tcc_state, 0 as libc::c_int);
    if (*s1).test_coverage as libc::c_int == 0 as libc::c_int || nocode_wanted != 0 {
        return;
    }
    if (*(*s1).dState).tcov_data.last_file_name == 0 as libc::c_int as libc::c_ulong
        || strcmp(
            ((*(*s1).tcov_section).data)
                .offset((*(*s1).dState).tcov_data.last_file_name as isize)
                as *const libc::c_char,
            (*file).true_filename,
        ) != 0 as libc::c_int
    {
        let mut wd: [libc::c_char; 1024] = [0; 1024];
        let mut cstr: CString = CString {
            size: 0,
            size_allocated: 0,
            data: 0 as *mut libc::c_char,
        };
        if (*(*s1).dState).tcov_data.last_func_name != 0 {
            section_ptr_add((*s1).tcov_section, 1 as libc::c_int as Elf64_Addr);
        }
        if (*(*s1).dState).tcov_data.last_file_name != 0 {
            section_ptr_add((*s1).tcov_section, 1 as libc::c_int as Elf64_Addr);
        }
        (*(*s1).dState).tcov_data.last_func_name = 0 as libc::c_int as libc::c_ulong;
        cstr_new(&mut cstr);
        if *((*file).true_filename).offset(0 as libc::c_int as isize) as libc::c_int
            == '/' as i32
        {
            (*(*s1).dState).tcov_data.last_file_name = (*(*s1).tcov_section).data_offset;
            cstr_printf(
                &mut cstr as *mut CString,
                b"%s\0" as *const u8 as *const libc::c_char,
                (*file).true_filename,
            );
        } else {
            getcwd(
                wd.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
            (*(*s1).dState)
                .tcov_data
                .last_file_name = ((*(*s1).tcov_section).data_offset)
                .wrapping_add(strlen(wd.as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            cstr_printf(
                &mut cstr as *mut CString,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                wd.as_mut_ptr(),
                (*file).true_filename,
            );
        }
        ptr = section_ptr_add(
            (*s1).tcov_section,
            (cstr.size + 1 as libc::c_int) as Elf64_Addr,
        );
        strcpy(ptr as *mut libc::c_char, cstr.data);
        cstr_free(&mut cstr);
    }
    if (*(*s1).dState).tcov_data.last_func_name == 0 as libc::c_int as libc::c_ulong
        || strcmp(
            ((*(*s1).tcov_section).data)
                .offset((*(*s1).dState).tcov_data.last_func_name as isize)
                as *const libc::c_char,
            funcname,
        ) != 0 as libc::c_int
    {
        let mut len: size_t = 0;
        if (*(*s1).dState).tcov_data.last_func_name != 0 {
            section_ptr_add((*s1).tcov_section, 1 as libc::c_int as Elf64_Addr);
        }
        (*(*s1).dState).tcov_data.last_func_name = (*(*s1).tcov_section).data_offset;
        len = strlen(funcname);
        ptr = section_ptr_add(
            (*s1).tcov_section,
            len.wrapping_add(1 as libc::c_int as size_t),
        );
        strcpy(ptr as *mut libc::c_char, funcname);
        section_ptr_add(
            (*s1).tcov_section,
            ((*(*s1).tcov_section).data_offset).wrapping_neg()
                & 7 as libc::c_int as libc::c_ulong,
        );
        ptr = section_ptr_add((*s1).tcov_section, 8 as libc::c_int as Elf64_Addr);
        write64le(ptr as *mut libc::c_uchar, (*file).line_num as uint64_t);
    }
    if ind == (*(*s1).dState).tcov_data.ind
        && (*(*s1).dState).tcov_data.line == (*file).line_num
    {
        (*(*s1).dState).tcov_data.offset = last_offset;
    } else {
        let mut label: Sym = {
            let mut init = Sym {
                v: 0 as libc::c_int,
                r: 0,
                a: SymAttr {
                    aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                },
                c2rust_unnamed: C2RustUnnamed_2 {
                    c2rust_unnamed: C2RustUnnamed_3 {
                        c: 0,
                        c2rust_unnamed: C2RustUnnamed_4 { sym_scope: 0 },
                    },
                },
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *mut Sym,
                },
                c2rust_unnamed_0: C2RustUnnamed_1 {
                    next: 0 as *mut Sym,
                },
                prev: 0 as *mut Sym,
                prev_tok: 0 as *mut Sym,
            };
            init
        };
        label.type_0.t = 4 as libc::c_int | 0x2000 as libc::c_int;
        ptr = section_ptr_add((*s1).tcov_section, 16 as libc::c_int as Elf64_Addr);
        (*(*s1).dState).tcov_data.line = (*file).line_num;
        write64le(
            ptr as *mut libc::c_uchar,
            ((*(*s1).dState).tcov_data.line << 8 as libc::c_int | 0xff as libc::c_int)
                as uint64_t,
        );
        put_extern_sym(
            &mut label,
            (*s1).tcov_section,
            ((ptr as *mut libc::c_uchar).offset_from((*(*s1).tcov_section).data)
                as libc::c_long + 8 as libc::c_int as libc::c_long) as Elf64_Addr,
            0 as libc::c_int as libc::c_ulong,
        );
        sv.type_0 = label.type_0;
        sv
            .r = (0x200 as libc::c_int | 0x100 as libc::c_int | 0x30 as libc::c_int)
            as libc::c_ushort;
        sv.r2 = 0x30 as libc::c_int as libc::c_ushort;
        sv.c2rust_unnamed.c.i = 0 as libc::c_int as uint64_t;
        sv.c2rust_unnamed_0.sym = &mut label;
        gen_increment_tcov(&mut sv);
        (*(*s1).dState)
            .tcov_data
            .offset = (ptr as *mut libc::c_uchar).offset_from((*(*s1).tcov_section).data)
            as libc::c_long as libc::c_ulong;
        (*(*s1).dState).tcov_data.ind = ind;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tcov_block_end(
    mut s1: *mut TCCState,
    mut line: libc::c_int,
) {
    if (*s1).test_coverage as libc::c_int == 0 as libc::c_int {
        return;
    }
    if line == -(1 as libc::c_int) {
        line = (*(*s1).dState).tcov_data.line;
    }
    if (*(*s1).dState).tcov_data.offset != 0 {
        let mut ptr: *mut libc::c_void = ((*(*s1).tcov_section).data)
            .offset((*(*s1).dState).tcov_data.offset as isize) as *mut libc::c_void;
        let mut nline: libc::c_ulonglong = (if line != 0 {
            line
        } else {
            (*file).line_num
        }) as libc::c_ulonglong;
        write64le(
            ptr as *mut libc::c_uchar,
            (read64le(ptr as *mut libc::c_uchar) as libc::c_ulonglong
                & 0xfffffffff as libc::c_ulonglong | nline << 36 as libc::c_int)
                as uint64_t,
        );
        (*(*s1).dState).tcov_data.offset = 0 as libc::c_int as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tcov_check_line(
    mut s1: *mut TCCState,
    mut start: libc::c_int,
) {
    if (*s1).test_coverage as libc::c_int == 0 as libc::c_int {
        return;
    }
    if (*(*s1).dState).tcov_data.line != (*file).line_num {
        if (*(*s1).dState).tcov_data.line + 1 as libc::c_int != (*file).line_num {
            tcc_tcov_block_end(s1, -(1 as libc::c_int));
            if start != 0 {
                tcc_tcov_block_begin(s1);
            }
        } else {
            (*(*s1).dState).tcov_data.line = (*file).line_num;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tcov_start(mut s1: *mut TCCState) {
    if (*s1).test_coverage as libc::c_int == 0 as libc::c_int {
        return;
    }
    if ((*s1).dState).is_null() {
        (*s1)
            .dState = tcc_mallocz(::core::mem::size_of::<_tccdbg>() as libc::c_ulong)
            as *mut _tccdbg;
    }
    memset(
        &mut (*(*s1).dState).tcov_data as *mut C2RustUnnamed as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
    );
    if ((*s1).tcov_section).is_null() {
        (*s1)
            .tcov_section = new_section(
            tcc_state,
            b".tcov\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        );
        section_ptr_add((*s1).tcov_section, 4 as libc::c_int as Elf64_Addr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tcov_end(mut s1: *mut TCCState) {
    if (*s1).test_coverage as libc::c_int == 0 as libc::c_int {
        return;
    }
    if (*(*s1).dState).tcov_data.last_func_name != 0 {
        section_ptr_add((*s1).tcov_section, 1 as libc::c_int as Elf64_Addr);
    }
    if (*(*s1).dState).tcov_data.last_file_name != 0 {
        section_ptr_add((*s1).tcov_section, 1 as libc::c_int as Elf64_Addr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tcov_reset_ind(mut s1: *mut TCCState) {
    (*(*s1).dState).tcov_data.ind = 0 as libc::c_int;
}
