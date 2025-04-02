use ::libc;
use ::c2rust_bitfields;
use ::f128;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    pub type _tccdbg;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_float;
    fn strtold(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f128::f128;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut tcc_state: *mut TCCState;
    fn pstrcpy(
        buf: *mut libc::c_char,
        buf_size: size_t,
        s: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn pstrcat(
        buf: *mut libc::c_char,
        buf_size: size_t,
        s: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn pstrncpy(
        out: *mut libc::c_char,
        buf_size: size_t,
        s: *const libc::c_char,
        num: size_t,
    ) -> *mut libc::c_char;
    fn tcc_basename(name: *const libc::c_char) -> *mut libc::c_char;
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_mallocz(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_realloc(ptr: *mut libc::c_void, size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_strdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn _tcc_error(fmt: *const libc::c_char, _: ...) -> !;
    fn _tcc_warning(fmt: *const libc::c_char, _: ...);
    fn dynarray_add(
        ptab: *mut libc::c_void,
        nb_ptr: *mut libc::c_int,
        data: *mut libc::c_void,
    );
    fn dynarray_reset(pp: *mut libc::c_void, n: *mut libc::c_int);
    fn tcc_open_bf(
        s1: *mut TCCState,
        filename: *const libc::c_char,
        initlen: libc::c_int,
    );
    fn tcc_open(s1: *mut TCCState, filename: *const libc::c_char) -> libc::c_int;
    fn tcc_close();
    fn normalized_PATHCMP(
        f1: *const libc::c_char,
        f2: *const libc::c_char,
    ) -> libc::c_int;
    static mut define_stack: *mut Sym;
    fn sym_free(sym: *mut Sym);
    fn sym_push2(
        ps: *mut *mut Sym,
        v: libc::c_int,
        t: libc::c_int,
        c: libc::c_int,
    ) -> *mut Sym;
    fn sym_find2(s: *mut Sym, v: libc::c_int) -> *mut Sym;
    fn expr_const() -> libc::c_int;
    static target_machine_defs: *const libc::c_char;
    fn tcc_debug_bincl(s1: *mut TCCState);
    fn tcc_debug_eincl(s1: *mut TCCState);
    fn tcc_debug_newfile(s1: *mut TCCState);
    fn tcc_set_options(s: *mut TCCState, str: *const libc::c_char) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenSym {
    pub hash_next: *mut TokenSym,
    pub sym_define: *mut Sym,
    pub sym_label: *mut Sym,
    pub sym_struct: *mut Sym,
    pub sym_identifier: *mut Sym,
    pub tok: libc::c_int,
    pub len: libc::c_int,
    pub str_0: [libc::c_char; 1],
}
pub type nwchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union CValue {
    pub ld: f128::f128,
    pub d: libc::c_double,
    pub f: libc::c_float,
    pub i: uint64_t,
    pub str_0: C2RustUnnamed_4,
    pub tab: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub data: *mut libc::c_char,
    pub size: libc::c_int,
}
pub type tcc_token = libc::c_uint;
pub const TOK_ASM_endbr64: tcc_token = 1319;
pub const TOK_ASM_clflush: tcc_token = 1318;
pub const TOK_ASM_sfence: tcc_token = 1317;
pub const TOK_ASM_mfence: tcc_token = 1316;
pub const TOK_ASM_lfence: tcc_token = 1315;
pub const TOK_ASM_prefetchw: tcc_token = 1314;
pub const TOK_ASM_prefetcht2: tcc_token = 1313;
pub const TOK_ASM_prefetcht1: tcc_token = 1312;
pub const TOK_ASM_prefetcht0: tcc_token = 1311;
pub const TOK_ASM_prefetchnta: tcc_token = 1310;
pub const TOK_ASM_movntiq: tcc_token = 1309;
pub const TOK_ASM_movntil: tcc_token = 1308;
pub const TOK_ASM_movnti: tcc_token = 1307;
pub const TOK_ASM_subps: tcc_token = 1306;
pub const TOK_ASM_sqrtps: tcc_token = 1305;
pub const TOK_ASM_rsqrtps: tcc_token = 1304;
pub const TOK_ASM_rcpss: tcc_token = 1303;
pub const TOK_ASM_pminub: tcc_token = 1302;
pub const TOK_ASM_pminsw: tcc_token = 1301;
pub const TOK_ASM_pmaxub: tcc_token = 1300;
pub const TOK_ASM_pmaxsw: tcc_token = 1299;
pub const TOK_ASM_pavgw: tcc_token = 1298;
pub const TOK_ASM_pavgb: tcc_token = 1297;
pub const TOK_ASM_mulps: tcc_token = 1296;
pub const TOK_ASM_minps: tcc_token = 1295;
pub const TOK_ASM_maxps: tcc_token = 1294;
pub const TOK_ASM_divps: tcc_token = 1293;
pub const TOK_ASM_cvttps2pi: tcc_token = 1292;
pub const TOK_ASM_cvtps2pi: tcc_token = 1291;
pub const TOK_ASM_cvtpi2ps: tcc_token = 1290;
pub const TOK_ASM_addps: tcc_token = 1289;
pub const TOK_ASM_movhps: tcc_token = 1288;
pub const TOK_ASM_movaps: tcc_token = 1287;
pub const TOK_ASM_movups: tcc_token = 1286;
pub const TOK_ASM_stmxcsr: tcc_token = 1285;
pub const TOK_ASM_ldmxcsr: tcc_token = 1284;
pub const TOK_ASM_pxor: tcc_token = 1283;
pub const TOK_ASM_punpckldq: tcc_token = 1282;
pub const TOK_ASM_punpcklwd: tcc_token = 1281;
pub const TOK_ASM_punpcklbw: tcc_token = 1280;
pub const TOK_ASM_punpckhdq: tcc_token = 1279;
pub const TOK_ASM_punpckhwd: tcc_token = 1278;
pub const TOK_ASM_punpckhbw: tcc_token = 1277;
pub const TOK_ASM_psubusw: tcc_token = 1276;
pub const TOK_ASM_psubusb: tcc_token = 1275;
pub const TOK_ASM_psubsw: tcc_token = 1274;
pub const TOK_ASM_psubsb: tcc_token = 1273;
pub const TOK_ASM_psubd: tcc_token = 1272;
pub const TOK_ASM_psubw: tcc_token = 1271;
pub const TOK_ASM_psubb: tcc_token = 1270;
pub const TOK_ASM_psrlq: tcc_token = 1269;
pub const TOK_ASM_psrld: tcc_token = 1268;
pub const TOK_ASM_psrlw: tcc_token = 1267;
pub const TOK_ASM_psrad: tcc_token = 1266;
pub const TOK_ASM_psraw: tcc_token = 1265;
pub const TOK_ASM_psllq: tcc_token = 1264;
pub const TOK_ASM_pslld: tcc_token = 1263;
pub const TOK_ASM_psllw: tcc_token = 1262;
pub const TOK_ASM_por: tcc_token = 1261;
pub const TOK_ASM_pmullw: tcc_token = 1260;
pub const TOK_ASM_pmulhw: tcc_token = 1259;
pub const TOK_ASM_pmaddwd: tcc_token = 1258;
pub const TOK_ASM_pcmpgtd: tcc_token = 1257;
pub const TOK_ASM_pcmpgtw: tcc_token = 1256;
pub const TOK_ASM_pcmpgtb: tcc_token = 1255;
pub const TOK_ASM_pcmpeqd: tcc_token = 1254;
pub const TOK_ASM_pcmpeqw: tcc_token = 1253;
pub const TOK_ASM_pcmpeqb: tcc_token = 1252;
pub const TOK_ASM_pandn: tcc_token = 1251;
pub const TOK_ASM_pand: tcc_token = 1250;
pub const TOK_ASM_paddusw: tcc_token = 1249;
pub const TOK_ASM_paddusb: tcc_token = 1248;
pub const TOK_ASM_paddsw: tcc_token = 1247;
pub const TOK_ASM_paddsb: tcc_token = 1246;
pub const TOK_ASM_paddd: tcc_token = 1245;
pub const TOK_ASM_paddw: tcc_token = 1244;
pub const TOK_ASM_paddb: tcc_token = 1243;
pub const TOK_ASM_packuswb: tcc_token = 1242;
pub const TOK_ASM_packsswb: tcc_token = 1241;
pub const TOK_ASM_packssdw: tcc_token = 1240;
pub const TOK_ASM_movd: tcc_token = 1239;
pub const TOK_ASM_fcomip: tcc_token = 1238;
pub const TOK_ASM_fucomip: tcc_token = 1237;
pub const TOK_ASM_fcomi: tcc_token = 1236;
pub const TOK_ASM_fucomi: tcc_token = 1235;
pub const TOK_ASM_fcmovnu: tcc_token = 1234;
pub const TOK_ASM_fcmovnbe: tcc_token = 1233;
pub const TOK_ASM_fcmovne: tcc_token = 1232;
pub const TOK_ASM_fcmovnb: tcc_token = 1231;
pub const TOK_ASM_fcmovu: tcc_token = 1230;
pub const TOK_ASM_fcmovbe: tcc_token = 1229;
pub const TOK_ASM_fcmove: tcc_token = 1228;
pub const TOK_ASM_fcmovb: tcc_token = 1227;
pub const TOK_ASM_cmpxchg16b: tcc_token = 1226;
pub const TOK_ASM_cmpxchg8b: tcc_token = 1225;
pub const TOK_ASM_invlpg: tcc_token = 1224;
pub const TOK_ASM_bswapq: tcc_token = 1223;
pub const TOK_ASM_bswapl: tcc_token = 1222;
pub const TOK_ASM_bswap: tcc_token = 1221;
pub const TOK_ASM_swapgs: tcc_token = 1220;
pub const TOK_ASM_verw: tcc_token = 1219;
pub const TOK_ASM_verr: tcc_token = 1218;
pub const TOK_ASM_str: tcc_token = 1217;
pub const TOK_ASM_smsw: tcc_token = 1216;
pub const TOK_ASM_sldt: tcc_token = 1215;
pub const TOK_ASM_sidtq: tcc_token = 1214;
pub const TOK_ASM_sidt: tcc_token = 1213;
pub const TOK_ASM_sgdtq: tcc_token = 1212;
pub const TOK_ASM_sgdt: tcc_token = 1211;
pub const TOK_ASM_ltr: tcc_token = 1210;
pub const TOK_ASM_lmsw: tcc_token = 1209;
pub const TOK_ASM_lldt: tcc_token = 1208;
pub const TOK_ASM_lidtq: tcc_token = 1207;
pub const TOK_ASM_lidt: tcc_token = 1206;
pub const TOK_ASM_lgdtq: tcc_token = 1205;
pub const TOK_ASM_lgdt: tcc_token = 1204;
pub const TOK_ASM_arpl: tcc_token = 1203;
pub const TOK_ASM_fxrstorq: tcc_token = 1202;
pub const TOK_ASM_fxsaveq: tcc_token = 1201;
pub const TOK_ASM_fxrstor: tcc_token = 1200;
pub const TOK_ASM_fxsave: tcc_token = 1199;
pub const TOK_ASM_ffreep: tcc_token = 1198;
pub const TOK_ASM_ffree: tcc_token = 1197;
pub const TOK_ASM_frstor: tcc_token = 1196;
pub const TOK_ASM_fsave: tcc_token = 1195;
pub const TOK_ASM_fnsave: tcc_token = 1194;
pub const TOK_ASM_fldenv: tcc_token = 1193;
pub const TOK_ASM_fstenv: tcc_token = 1192;
pub const TOK_ASM_fnstenv: tcc_token = 1191;
pub const TOK_ASM_fclex: tcc_token = 1190;
pub const TOK_ASM_fstsw: tcc_token = 1189;
pub const TOK_ASM_fstcw: tcc_token = 1188;
pub const TOK_ASM_fnstcw: tcc_token = 1187;
pub const TOK_ASM_fldcw: tcc_token = 1186;
pub const TOK_ASM_finit: tcc_token = 1185;
pub const TOK_ASM_fucomp: tcc_token = 1184;
pub const TOK_ASM_fucom: tcc_token = 1183;
pub const TOK_ASM_fbstp: tcc_token = 1182;
pub const TOK_ASM_fstpt: tcc_token = 1181;
pub const TOK_ASM_fistpll: tcc_token = 1180;
pub const TOK_ASM_fistpq: tcc_token = 1179;
pub const TOK_ASM_fstp: tcc_token = 1178;
pub const TOK_ASM_fistpl: tcc_token = 1177;
pub const TOK_ASM_fistl: tcc_token = 1176;
pub const TOK_ASM_fistp: tcc_token = 1175;
pub const TOK_ASM_fist: tcc_token = 1174;
pub const TOK_ASM_fstpl: tcc_token = 1173;
pub const TOK_ASM_fstps: tcc_token = 1172;
pub const TOK_ASM_fsts: tcc_token = 1171;
pub const TOK_ASM_fstl: tcc_token = 1170;
pub const TOK_ASM_fst: tcc_token = 1169;
pub const TOK_ASM_fbld: tcc_token = 1168;
pub const TOK_ASM_fldt: tcc_token = 1167;
pub const TOK_ASM_fildll: tcc_token = 1166;
pub const TOK_ASM_fildq: tcc_token = 1165;
pub const TOK_ASM_fildl: tcc_token = 1164;
pub const TOK_ASM_flds: tcc_token = 1163;
pub const TOK_ASM_fldl: tcc_token = 1162;
pub const TOK_ASM_fld: tcc_token = 1161;
pub const TOK_ASM_jecxz: tcc_token = 1160;
pub const TOK_ASM_loop: tcc_token = 1159;
pub const TOK_ASM_loopz: tcc_token = 1158;
pub const TOK_ASM_loope: tcc_token = 1157;
pub const TOK_ASM_loopnz: tcc_token = 1156;
pub const TOK_ASM_loopne: tcc_token = 1155;
pub const TOK_ASM_enter: tcc_token = 1154;
pub const TOK_ASM_ljmpl: tcc_token = 1153;
pub const TOK_ASM_ljmpw: tcc_token = 1152;
pub const TOK_ASM_callq: tcc_token = 1151;
pub const TOK_ASM_sysretq: tcc_token = 1150;
pub const TOK_ASM_vmxoff: tcc_token = 1149;
pub const TOK_ASM_vmresume: tcc_token = 1148;
pub const TOK_ASM_vmlaunch: tcc_token = 1147;
pub const TOK_ASM_vmcall: tcc_token = 1146;
pub const TOK_ASM_emms: tcc_token = 1145;
pub const TOK_ASM_fnstsw: tcc_token = 1144;
pub const TOK_ASM_fxch: tcc_token = 1143;
pub const TOK_ASM_fwait: tcc_token = 1142;
pub const TOK_ASM_fnop: tcc_token = 1141;
pub const TOK_ASM_fnclex: tcc_token = 1140;
pub const TOK_ASM_fninit: tcc_token = 1139;
pub const TOK_ASM_fabs: tcc_token = 1138;
pub const TOK_ASM_fchs: tcc_token = 1137;
pub const TOK_ASM_fcos: tcc_token = 1136;
pub const TOK_ASM_fsin: tcc_token = 1135;
pub const TOK_ASM_fscale: tcc_token = 1134;
pub const TOK_ASM_frndint: tcc_token = 1133;
pub const TOK_ASM_fsincos: tcc_token = 1132;
pub const TOK_ASM_fsqrt: tcc_token = 1131;
pub const TOK_ASM_fyl2xp1: tcc_token = 1130;
pub const TOK_ASM_fprem: tcc_token = 1129;
pub const TOK_ASM_fincstp: tcc_token = 1128;
pub const TOK_ASM_fdecstp: tcc_token = 1127;
pub const TOK_ASM_fprem1: tcc_token = 1126;
pub const TOK_ASM_fxtract: tcc_token = 1125;
pub const TOK_ASM_fpatan: tcc_token = 1124;
pub const TOK_ASM_fptan: tcc_token = 1123;
pub const TOK_ASM_fyl2x: tcc_token = 1122;
pub const TOK_ASM_f2xm1: tcc_token = 1121;
pub const TOK_ASM_fldz: tcc_token = 1120;
pub const TOK_ASM_fldln2: tcc_token = 1119;
pub const TOK_ASM_fldlg2: tcc_token = 1118;
pub const TOK_ASM_fldpi: tcc_token = 1117;
pub const TOK_ASM_fldl2e: tcc_token = 1116;
pub const TOK_ASM_fldl2t: tcc_token = 1115;
pub const TOK_ASM_fld1: tcc_token = 1114;
pub const TOK_ASM_fxam: tcc_token = 1113;
pub const TOK_ASM_ftst: tcc_token = 1112;
pub const TOK_ASM_fucompp: tcc_token = 1111;
pub const TOK_ASM_lret: tcc_token = 1110;
pub const TOK_ASM_retq: tcc_token = 1109;
pub const TOK_ASM_ret: tcc_token = 1108;
pub const TOK_ASM_leave: tcc_token = 1107;
pub const TOK_ASM_ud2: tcc_token = 1106;
pub const TOK_ASM_sysret: tcc_token = 1105;
pub const TOK_ASM_syscall: tcc_token = 1104;
pub const TOK_ASM_rdpmc: tcc_token = 1103;
pub const TOK_ASM_rdmsr: tcc_token = 1102;
pub const TOK_ASM_rdtsc: tcc_token = 1101;
pub const TOK_ASM_wrmsr: tcc_token = 1100;
pub const TOK_ASM_cpuid: tcc_token = 1099;
pub const TOK_ASM_wbinvd: tcc_token = 1098;
pub const TOK_ASM_invd: tcc_token = 1097;
pub const TOK_ASM_repnz: tcc_token = 1096;
pub const TOK_ASM_repne: tcc_token = 1095;
pub const TOK_ASM_repz: tcc_token = 1094;
pub const TOK_ASM_repe: tcc_token = 1093;
pub const TOK_ASM_rep: tcc_token = 1092;
pub const TOK_ASM_lock: tcc_token = 1091;
pub const TOK_ASM_xlat: tcc_token = 1090;
pub const TOK_ASM_pause: tcc_token = 1089;
pub const TOK_ASM_nop: tcc_token = 1088;
pub const TOK_ASM_wait: tcc_token = 1087;
pub const TOK_ASM_hlt: tcc_token = 1086;
pub const TOK_ASM_rsm: tcc_token = 1085;
pub const TOK_ASM_iretq: tcc_token = 1084;
pub const TOK_ASM_iretl: tcc_token = 1083;
pub const TOK_ASM_iretw: tcc_token = 1082;
pub const TOK_ASM_iret: tcc_token = 1081;
pub const TOK_ASM_into: tcc_token = 1080;
pub const TOK_ASM_int3: tcc_token = 1079;
pub const TOK_ASM_cqto: tcc_token = 1078;
pub const TOK_ASM_cltd: tcc_token = 1077;
pub const TOK_ASM_cwtd: tcc_token = 1076;
pub const TOK_ASM_cwtl: tcc_token = 1075;
pub const TOK_ASM_cbtw: tcc_token = 1074;
pub const TOK_ASM_cdq: tcc_token = 1073;
pub const TOK_ASM_cwde: tcc_token = 1072;
pub const TOK_ASM_cwd: tcc_token = 1071;
pub const TOK_ASM_cbw: tcc_token = 1070;
pub const TOK_ASM_aam: tcc_token = 1069;
pub const TOK_ASM_aad: tcc_token = 1068;
pub const TOK_ASM_das: tcc_token = 1067;
pub const TOK_ASM_daa: tcc_token = 1066;
pub const TOK_ASM_aas: tcc_token = 1065;
pub const TOK_ASM_aaa: tcc_token = 1064;
pub const TOK_ASM_sti: tcc_token = 1063;
pub const TOK_ASM_std: tcc_token = 1062;
pub const TOK_ASM_stc: tcc_token = 1061;
pub const TOK_ASM_popf: tcc_token = 1060;
pub const TOK_ASM_pushf: tcc_token = 1059;
pub const TOK_ASM_popfq: tcc_token = 1058;
pub const TOK_ASM_pushfq: tcc_token = 1057;
pub const TOK_ASM_sahf: tcc_token = 1056;
pub const TOK_ASM_lahf: tcc_token = 1055;
pub const TOK_ASM_cmc: tcc_token = 1054;
pub const TOK_ASM_clts: tcc_token = 1053;
pub const TOK_ASM_cli: tcc_token = 1052;
pub const TOK_ASM_cld: tcc_token = 1051;
pub const TOK_ASM_clc: tcc_token = 1050;
pub const TOK_ASM_ssto: tcc_token = 1049;
pub const TOK_ASM_sstoq: tcc_token = 1048;
pub const TOK_ASM_sstol: tcc_token = 1047;
pub const TOK_ASM_sstow: tcc_token = 1046;
pub const TOK_ASM_sstob: tcc_token = 1045;
pub const TOK_ASM_stos: tcc_token = 1044;
pub const TOK_ASM_stosq: tcc_token = 1043;
pub const TOK_ASM_stosl: tcc_token = 1042;
pub const TOK_ASM_stosw: tcc_token = 1041;
pub const TOK_ASM_stosb: tcc_token = 1040;
pub const TOK_ASM_ssca: tcc_token = 1039;
pub const TOK_ASM_sscaq: tcc_token = 1038;
pub const TOK_ASM_sscal: tcc_token = 1037;
pub const TOK_ASM_sscaw: tcc_token = 1036;
pub const TOK_ASM_sscab: tcc_token = 1035;
pub const TOK_ASM_scas: tcc_token = 1034;
pub const TOK_ASM_scasq: tcc_token = 1033;
pub const TOK_ASM_scasl: tcc_token = 1032;
pub const TOK_ASM_scasw: tcc_token = 1031;
pub const TOK_ASM_scasb: tcc_token = 1030;
pub const TOK_ASM_smov: tcc_token = 1029;
pub const TOK_ASM_smovq: tcc_token = 1028;
pub const TOK_ASM_smovl: tcc_token = 1027;
pub const TOK_ASM_smovw: tcc_token = 1026;
pub const TOK_ASM_smovb: tcc_token = 1025;
pub const TOK_ASM_movs: tcc_token = 1024;
pub const TOK_ASM_movsq: tcc_token = 1023;
pub const TOK_ASM_movsl: tcc_token = 1022;
pub const TOK_ASM_movsw: tcc_token = 1021;
pub const TOK_ASM_movsb: tcc_token = 1020;
pub const TOK_ASM_slod: tcc_token = 1019;
pub const TOK_ASM_slodq: tcc_token = 1018;
pub const TOK_ASM_slodl: tcc_token = 1017;
pub const TOK_ASM_slodw: tcc_token = 1016;
pub const TOK_ASM_slodb: tcc_token = 1015;
pub const TOK_ASM_lods: tcc_token = 1014;
pub const TOK_ASM_lodsq: tcc_token = 1013;
pub const TOK_ASM_lodsl: tcc_token = 1012;
pub const TOK_ASM_lodsw: tcc_token = 1011;
pub const TOK_ASM_lodsb: tcc_token = 1010;
pub const TOK_ASM_outs: tcc_token = 1009;
pub const TOK_ASM_outsl: tcc_token = 1008;
pub const TOK_ASM_outsw: tcc_token = 1007;
pub const TOK_ASM_outsb: tcc_token = 1006;
pub const TOK_ASM_ins: tcc_token = 1005;
pub const TOK_ASM_insl: tcc_token = 1004;
pub const TOK_ASM_insw: tcc_token = 1003;
pub const TOK_ASM_insb: tcc_token = 1002;
pub const TOK_ASM_scmp: tcc_token = 1001;
pub const TOK_ASM_scmpq: tcc_token = 1000;
pub const TOK_ASM_scmpl: tcc_token = 999;
pub const TOK_ASM_scmpw: tcc_token = 998;
pub const TOK_ASM_scmpb: tcc_token = 997;
pub const TOK_ASM_cmps: tcc_token = 996;
pub const TOK_ASM_cmpsq: tcc_token = 995;
pub const TOK_ASM_cmpsl: tcc_token = 994;
pub const TOK_ASM_cmpsw: tcc_token = 993;
pub const TOK_ASM_cmpsb: tcc_token = 992;
pub const TOK_ASM_cmpxchg: tcc_token = 991;
pub const TOK_ASM_cmpxchgq: tcc_token = 990;
pub const TOK_ASM_cmpxchgl: tcc_token = 989;
pub const TOK_ASM_cmpxchgw: tcc_token = 988;
pub const TOK_ASM_cmpxchgb: tcc_token = 987;
pub const TOK_ASM_xadd: tcc_token = 986;
pub const TOK_ASM_xaddq: tcc_token = 985;
pub const TOK_ASM_xaddl: tcc_token = 984;
pub const TOK_ASM_xaddw: tcc_token = 983;
pub const TOK_ASM_xaddb: tcc_token = 982;
pub const TOK_ASM_fidivrs: tcc_token = 981;
pub const TOK_ASM_fdivrl: tcc_token = 980;
pub const TOK_ASM_fidivrl: tcc_token = 979;
pub const TOK_ASM_fdivrs: tcc_token = 978;
pub const TOK_ASM_fdivrp: tcc_token = 977;
pub const TOK_ASM_fdivr: tcc_token = 976;
pub const TOK_ASM_fidivs: tcc_token = 975;
pub const TOK_ASM_fdivl: tcc_token = 974;
pub const TOK_ASM_fidivl: tcc_token = 973;
pub const TOK_ASM_fdivs: tcc_token = 972;
pub const TOK_ASM_fdivp: tcc_token = 971;
pub const TOK_ASM_fdiv: tcc_token = 970;
pub const TOK_ASM_fisubrs: tcc_token = 969;
pub const TOK_ASM_fsubrl: tcc_token = 968;
pub const TOK_ASM_fisubrl: tcc_token = 967;
pub const TOK_ASM_fsubrs: tcc_token = 966;
pub const TOK_ASM_fsubrp: tcc_token = 965;
pub const TOK_ASM_fsubr: tcc_token = 964;
pub const TOK_ASM_fisubs: tcc_token = 963;
pub const TOK_ASM_fsubl: tcc_token = 962;
pub const TOK_ASM_fisubl: tcc_token = 961;
pub const TOK_ASM_fsubs: tcc_token = 960;
pub const TOK_ASM_fsubp: tcc_token = 959;
pub const TOK_ASM_fsub: tcc_token = 958;
pub const TOK_ASM_ficomps: tcc_token = 957;
pub const TOK_ASM_fcompl: tcc_token = 956;
pub const TOK_ASM_ficompl: tcc_token = 955;
pub const TOK_ASM_fcomps: tcc_token = 954;
pub const TOK_ASM_fcompp: tcc_token = 953;
pub const TOK_ASM_fcomp: tcc_token = 952;
pub const TOK_ASM_ficoms: tcc_token = 951;
pub const TOK_ASM_fcoml: tcc_token = 950;
pub const TOK_ASM_ficoml: tcc_token = 949;
pub const TOK_ASM_fcoms: tcc_token = 948;
pub const TOK_ASM_fcom_1: tcc_token = 947;
pub const TOK_ASM_fcom: tcc_token = 946;
pub const TOK_ASM_fimuls: tcc_token = 945;
pub const TOK_ASM_fmull: tcc_token = 944;
pub const TOK_ASM_fimull: tcc_token = 943;
pub const TOK_ASM_fmuls: tcc_token = 942;
pub const TOK_ASM_fmulp: tcc_token = 941;
pub const TOK_ASM_fmul: tcc_token = 940;
pub const TOK_ASM_fiadds: tcc_token = 939;
pub const TOK_ASM_faddl: tcc_token = 938;
pub const TOK_ASM_fiaddl: tcc_token = 937;
pub const TOK_ASM_fadds: tcc_token = 936;
pub const TOK_ASM_faddp: tcc_token = 935;
pub const TOK_ASM_fadd: tcc_token = 934;
pub const TOK_ASM_lsl: tcc_token = 933;
pub const TOK_ASM_lslq: tcc_token = 932;
pub const TOK_ASM_lsll: tcc_token = 931;
pub const TOK_ASM_lslw: tcc_token = 930;
pub const TOK_ASM_lar: tcc_token = 929;
pub const TOK_ASM_larq: tcc_token = 928;
pub const TOK_ASM_larl: tcc_token = 927;
pub const TOK_ASM_larw: tcc_token = 926;
pub const TOK_ASM_lzcnt: tcc_token = 925;
pub const TOK_ASM_lzcntq: tcc_token = 924;
pub const TOK_ASM_lzcntl: tcc_token = 923;
pub const TOK_ASM_lzcntw: tcc_token = 922;
pub const TOK_ASM_tzcnt: tcc_token = 921;
pub const TOK_ASM_tzcntq: tcc_token = 920;
pub const TOK_ASM_tzcntl: tcc_token = 919;
pub const TOK_ASM_tzcntw: tcc_token = 918;
pub const TOK_ASM_popcnt: tcc_token = 917;
pub const TOK_ASM_popcntq: tcc_token = 916;
pub const TOK_ASM_popcntl: tcc_token = 915;
pub const TOK_ASM_popcntw: tcc_token = 914;
pub const TOK_ASM_btc: tcc_token = 913;
pub const TOK_ASM_btcq: tcc_token = 912;
pub const TOK_ASM_btcl: tcc_token = 911;
pub const TOK_ASM_btcw: tcc_token = 910;
pub const TOK_ASM_btr: tcc_token = 909;
pub const TOK_ASM_btrq: tcc_token = 908;
pub const TOK_ASM_btrl: tcc_token = 907;
pub const TOK_ASM_btrw: tcc_token = 906;
pub const TOK_ASM_bts: tcc_token = 905;
pub const TOK_ASM_btsq: tcc_token = 904;
pub const TOK_ASM_btsl: tcc_token = 903;
pub const TOK_ASM_btsw: tcc_token = 902;
pub const TOK_ASM_bt: tcc_token = 901;
pub const TOK_ASM_btq: tcc_token = 900;
pub const TOK_ASM_btl: tcc_token = 899;
pub const TOK_ASM_btw: tcc_token = 898;
pub const TOK_ASM_bsr: tcc_token = 897;
pub const TOK_ASM_bsrq: tcc_token = 896;
pub const TOK_ASM_bsrl: tcc_token = 895;
pub const TOK_ASM_bsrw: tcc_token = 894;
pub const TOK_ASM_bsf: tcc_token = 893;
pub const TOK_ASM_bsfq: tcc_token = 892;
pub const TOK_ASM_bsfl: tcc_token = 891;
pub const TOK_ASM_bsfw: tcc_token = 890;
pub const TOK_ASM_cmovg: tcc_token = 889;
pub const TOK_ASM_cmovnle: tcc_token = 888;
pub const TOK_ASM_cmovng: tcc_token = 887;
pub const TOK_ASM_cmovle: tcc_token = 886;
pub const TOK_ASM_cmovge: tcc_token = 885;
pub const TOK_ASM_cmovnl: tcc_token = 884;
pub const TOK_ASM_cmovnge: tcc_token = 883;
pub const TOK_ASM_cmovl: tcc_token = 882;
pub const TOK_ASM_cmovpo: tcc_token = 881;
pub const TOK_ASM_cmovnp: tcc_token = 880;
pub const TOK_ASM_cmovpe: tcc_token = 879;
pub const TOK_ASM_cmovp: tcc_token = 878;
pub const TOK_ASM_cmovns: tcc_token = 877;
pub const TOK_ASM_cmovs: tcc_token = 876;
pub const TOK_ASM_cmova: tcc_token = 875;
pub const TOK_ASM_cmovnbe: tcc_token = 874;
pub const TOK_ASM_cmovna: tcc_token = 873;
pub const TOK_ASM_cmovbe: tcc_token = 872;
pub const TOK_ASM_cmovnz: tcc_token = 871;
pub const TOK_ASM_cmovne: tcc_token = 870;
pub const TOK_ASM_cmovz: tcc_token = 869;
pub const TOK_ASM_cmove: tcc_token = 868;
pub const TOK_ASM_cmovae: tcc_token = 867;
pub const TOK_ASM_cmovnc: tcc_token = 866;
pub const TOK_ASM_cmovnb: tcc_token = 865;
pub const TOK_ASM_cmovnae: tcc_token = 864;
pub const TOK_ASM_cmovc: tcc_token = 863;
pub const TOK_ASM_cmovb: tcc_token = 862;
pub const TOK_ASM_cmovno: tcc_token = 861;
pub const TOK_ASM_cmovo: tcc_token = 860;
pub const TOK_ASM_setgb: tcc_token = 859;
pub const TOK_ASM_setnleb: tcc_token = 858;
pub const TOK_ASM_setngb: tcc_token = 857;
pub const TOK_ASM_setleb: tcc_token = 856;
pub const TOK_ASM_setgeb: tcc_token = 855;
pub const TOK_ASM_setnlb: tcc_token = 854;
pub const TOK_ASM_setngeb: tcc_token = 853;
pub const TOK_ASM_setlb: tcc_token = 852;
pub const TOK_ASM_setpob: tcc_token = 851;
pub const TOK_ASM_setnpb: tcc_token = 850;
pub const TOK_ASM_setpeb: tcc_token = 849;
pub const TOK_ASM_setpb: tcc_token = 848;
pub const TOK_ASM_setnsb: tcc_token = 847;
pub const TOK_ASM_setsb: tcc_token = 846;
pub const TOK_ASM_setab: tcc_token = 845;
pub const TOK_ASM_setnbeb: tcc_token = 844;
pub const TOK_ASM_setnab: tcc_token = 843;
pub const TOK_ASM_setbeb: tcc_token = 842;
pub const TOK_ASM_setnzb: tcc_token = 841;
pub const TOK_ASM_setneb: tcc_token = 840;
pub const TOK_ASM_setzb: tcc_token = 839;
pub const TOK_ASM_seteb: tcc_token = 838;
pub const TOK_ASM_setaeb: tcc_token = 837;
pub const TOK_ASM_setncb: tcc_token = 836;
pub const TOK_ASM_setnbb: tcc_token = 835;
pub const TOK_ASM_setnaeb: tcc_token = 834;
pub const TOK_ASM_setcb: tcc_token = 833;
pub const TOK_ASM_setbb: tcc_token = 832;
pub const TOK_ASM_setnob: tcc_token = 831;
pub const TOK_ASM_setob: tcc_token = 830;
pub const TOK_ASM_setg: tcc_token = 829;
pub const TOK_ASM_setnle: tcc_token = 828;
pub const TOK_ASM_setng: tcc_token = 827;
pub const TOK_ASM_setle: tcc_token = 826;
pub const TOK_ASM_setge: tcc_token = 825;
pub const TOK_ASM_setnl: tcc_token = 824;
pub const TOK_ASM_setnge: tcc_token = 823;
pub const TOK_ASM_setl: tcc_token = 822;
pub const TOK_ASM_setpo: tcc_token = 821;
pub const TOK_ASM_setnp: tcc_token = 820;
pub const TOK_ASM_setpe: tcc_token = 819;
pub const TOK_ASM_setp: tcc_token = 818;
pub const TOK_ASM_setns: tcc_token = 817;
pub const TOK_ASM_sets: tcc_token = 816;
pub const TOK_ASM_seta: tcc_token = 815;
pub const TOK_ASM_setnbe: tcc_token = 814;
pub const TOK_ASM_setna: tcc_token = 813;
pub const TOK_ASM_setbe: tcc_token = 812;
pub const TOK_ASM_setnz: tcc_token = 811;
pub const TOK_ASM_setne: tcc_token = 810;
pub const TOK_ASM_setz: tcc_token = 809;
pub const TOK_ASM_sete: tcc_token = 808;
pub const TOK_ASM_setae: tcc_token = 807;
pub const TOK_ASM_setnc: tcc_token = 806;
pub const TOK_ASM_setnb: tcc_token = 805;
pub const TOK_ASM_setnae: tcc_token = 804;
pub const TOK_ASM_setc: tcc_token = 803;
pub const TOK_ASM_setb: tcc_token = 802;
pub const TOK_ASM_setno: tcc_token = 801;
pub const TOK_ASM_seto: tcc_token = 800;
pub const TOK_ASM_jg: tcc_token = 799;
pub const TOK_ASM_jnle: tcc_token = 798;
pub const TOK_ASM_jng: tcc_token = 797;
pub const TOK_ASM_jle: tcc_token = 796;
pub const TOK_ASM_jge: tcc_token = 795;
pub const TOK_ASM_jnl: tcc_token = 794;
pub const TOK_ASM_jnge: tcc_token = 793;
pub const TOK_ASM_jl: tcc_token = 792;
pub const TOK_ASM_jpo: tcc_token = 791;
pub const TOK_ASM_jnp: tcc_token = 790;
pub const TOK_ASM_jpe: tcc_token = 789;
pub const TOK_ASM_jp: tcc_token = 788;
pub const TOK_ASM_jns: tcc_token = 787;
pub const TOK_ASM_js: tcc_token = 786;
pub const TOK_ASM_ja: tcc_token = 785;
pub const TOK_ASM_jnbe: tcc_token = 784;
pub const TOK_ASM_jna: tcc_token = 783;
pub const TOK_ASM_jbe: tcc_token = 782;
pub const TOK_ASM_jnz: tcc_token = 781;
pub const TOK_ASM_jne: tcc_token = 780;
pub const TOK_ASM_jz: tcc_token = 779;
pub const TOK_ASM_je: tcc_token = 778;
pub const TOK_ASM_jae: tcc_token = 777;
pub const TOK_ASM_jnc: tcc_token = 776;
pub const TOK_ASM_jnb: tcc_token = 775;
pub const TOK_ASM_jnae: tcc_token = 774;
pub const TOK_ASM_jc: tcc_token = 773;
pub const TOK_ASM_jb: tcc_token = 772;
pub const TOK_ASM_jno: tcc_token = 771;
pub const TOK_ASM_jo: tcc_token = 770;
pub const TOK_ASM_ljmp: tcc_token = 769;
pub const TOK_ASM_lcall: tcc_token = 768;
pub const TOK_ASM_jmp: tcc_token = 767;
pub const TOK_ASM_call: tcc_token = 766;
pub const TOK_ASM_lgs: tcc_token = 765;
pub const TOK_ASM_lfs: tcc_token = 764;
pub const TOK_ASM_lss: tcc_token = 763;
pub const TOK_ASM_lds: tcc_token = 762;
pub const TOK_ASM_les: tcc_token = 761;
pub const TOK_ASM_lea: tcc_token = 760;
pub const TOK_ASM_leaq: tcc_token = 759;
pub const TOK_ASM_leal: tcc_token = 758;
pub const TOK_ASM_leaw: tcc_token = 757;
pub const TOK_ASM_movslq: tcc_token = 756;
pub const TOK_ASM_movzwq: tcc_token = 755;
pub const TOK_ASM_movswq: tcc_token = 754;
pub const TOK_ASM_movsbq: tcc_token = 753;
pub const TOK_ASM_movswl: tcc_token = 752;
pub const TOK_ASM_movsbl: tcc_token = 751;
pub const TOK_ASM_movsbw: tcc_token = 750;
pub const TOK_ASM_movzwl: tcc_token = 749;
pub const TOK_ASM_movzb: tcc_token = 748;
pub const TOK_ASM_movzbq: tcc_token = 747;
pub const TOK_ASM_movzbl: tcc_token = 746;
pub const TOK_ASM_movzbw: tcc_token = 745;
pub const TOK_ASM_out: tcc_token = 744;
pub const TOK_ASM_outl: tcc_token = 743;
pub const TOK_ASM_outw: tcc_token = 742;
pub const TOK_ASM_outb: tcc_token = 741;
pub const TOK_ASM_in: tcc_token = 740;
pub const TOK_ASM_inl: tcc_token = 739;
pub const TOK_ASM_inw: tcc_token = 738;
pub const TOK_ASM_inb: tcc_token = 737;
pub const TOK_ASM_pop: tcc_token = 736;
pub const TOK_ASM_popq: tcc_token = 735;
pub const TOK_ASM_popl: tcc_token = 734;
pub const TOK_ASM_popw: tcc_token = 733;
pub const TOK_ASM_push: tcc_token = 732;
pub const TOK_ASM_pushq: tcc_token = 731;
pub const TOK_ASM_pushl: tcc_token = 730;
pub const TOK_ASM_pushw: tcc_token = 729;
pub const TOK_ASM_shrd: tcc_token = 728;
pub const TOK_ASM_shrdq: tcc_token = 727;
pub const TOK_ASM_shrdl: tcc_token = 726;
pub const TOK_ASM_shrdw: tcc_token = 725;
pub const TOK_ASM_shld: tcc_token = 724;
pub const TOK_ASM_shldq: tcc_token = 723;
pub const TOK_ASM_shldl: tcc_token = 722;
pub const TOK_ASM_shldw: tcc_token = 721;
pub const TOK_ASM_sar: tcc_token = 720;
pub const TOK_ASM_sarq: tcc_token = 719;
pub const TOK_ASM_sarl: tcc_token = 718;
pub const TOK_ASM_sarw: tcc_token = 717;
pub const TOK_ASM_sarb: tcc_token = 716;
pub const TOK_ASM_shr: tcc_token = 715;
pub const TOK_ASM_shrq: tcc_token = 714;
pub const TOK_ASM_shrl: tcc_token = 713;
pub const TOK_ASM_shrw: tcc_token = 712;
pub const TOK_ASM_shrb: tcc_token = 711;
pub const TOK_ASM_shl: tcc_token = 710;
pub const TOK_ASM_shlq: tcc_token = 709;
pub const TOK_ASM_shll: tcc_token = 708;
pub const TOK_ASM_shlw: tcc_token = 707;
pub const TOK_ASM_shlb: tcc_token = 706;
pub const TOK_ASM_rcr: tcc_token = 705;
pub const TOK_ASM_rcrq: tcc_token = 704;
pub const TOK_ASM_rcrl: tcc_token = 703;
pub const TOK_ASM_rcrw: tcc_token = 702;
pub const TOK_ASM_rcrb: tcc_token = 701;
pub const TOK_ASM_rcl: tcc_token = 700;
pub const TOK_ASM_rclq: tcc_token = 699;
pub const TOK_ASM_rcll: tcc_token = 698;
pub const TOK_ASM_rclw: tcc_token = 697;
pub const TOK_ASM_rclb: tcc_token = 696;
pub const TOK_ASM_ror: tcc_token = 695;
pub const TOK_ASM_rorq: tcc_token = 694;
pub const TOK_ASM_rorl: tcc_token = 693;
pub const TOK_ASM_rorw: tcc_token = 692;
pub const TOK_ASM_rorb: tcc_token = 691;
pub const TOK_ASM_rol: tcc_token = 690;
pub const TOK_ASM_rolq: tcc_token = 689;
pub const TOK_ASM_roll: tcc_token = 688;
pub const TOK_ASM_rolw: tcc_token = 687;
pub const TOK_ASM_rolb: tcc_token = 686;
pub const TOK_ASM_test: tcc_token = 685;
pub const TOK_ASM_testq: tcc_token = 684;
pub const TOK_ASM_testl: tcc_token = 683;
pub const TOK_ASM_testw: tcc_token = 682;
pub const TOK_ASM_testb: tcc_token = 681;
pub const TOK_ASM_xchg: tcc_token = 680;
pub const TOK_ASM_xchgq: tcc_token = 679;
pub const TOK_ASM_xchgl: tcc_token = 678;
pub const TOK_ASM_xchgw: tcc_token = 677;
pub const TOK_ASM_xchgb: tcc_token = 676;
pub const TOK_ASM_idiv: tcc_token = 675;
pub const TOK_ASM_idivq: tcc_token = 674;
pub const TOK_ASM_idivl: tcc_token = 673;
pub const TOK_ASM_idivw: tcc_token = 672;
pub const TOK_ASM_idivb: tcc_token = 671;
pub const TOK_ASM_div: tcc_token = 670;
pub const TOK_ASM_divq: tcc_token = 669;
pub const TOK_ASM_divl: tcc_token = 668;
pub const TOK_ASM_divw: tcc_token = 667;
pub const TOK_ASM_divb: tcc_token = 666;
pub const TOK_ASM_imul: tcc_token = 665;
pub const TOK_ASM_imulq: tcc_token = 664;
pub const TOK_ASM_imull: tcc_token = 663;
pub const TOK_ASM_imulw: tcc_token = 662;
pub const TOK_ASM_imulb: tcc_token = 661;
pub const TOK_ASM_mul: tcc_token = 660;
pub const TOK_ASM_mulq: tcc_token = 659;
pub const TOK_ASM_mull: tcc_token = 658;
pub const TOK_ASM_mulw: tcc_token = 657;
pub const TOK_ASM_mulb: tcc_token = 656;
pub const TOK_ASM_neg: tcc_token = 655;
pub const TOK_ASM_negq: tcc_token = 654;
pub const TOK_ASM_negl: tcc_token = 653;
pub const TOK_ASM_negw: tcc_token = 652;
pub const TOK_ASM_negb: tcc_token = 651;
pub const TOK_ASM_not: tcc_token = 650;
pub const TOK_ASM_notq: tcc_token = 649;
pub const TOK_ASM_notl: tcc_token = 648;
pub const TOK_ASM_notw: tcc_token = 647;
pub const TOK_ASM_notb: tcc_token = 646;
pub const TOK_ASM_dec: tcc_token = 645;
pub const TOK_ASM_decq: tcc_token = 644;
pub const TOK_ASM_decl: tcc_token = 643;
pub const TOK_ASM_decw: tcc_token = 642;
pub const TOK_ASM_decb: tcc_token = 641;
pub const TOK_ASM_inc: tcc_token = 640;
pub const TOK_ASM_incq: tcc_token = 639;
pub const TOK_ASM_incl: tcc_token = 638;
pub const TOK_ASM_incw: tcc_token = 637;
pub const TOK_ASM_incb: tcc_token = 636;
pub const TOK_ASM_cmp: tcc_token = 635;
pub const TOK_ASM_cmpq: tcc_token = 634;
pub const TOK_ASM_cmpl: tcc_token = 633;
pub const TOK_ASM_cmpw: tcc_token = 632;
pub const TOK_ASM_cmpb: tcc_token = 631;
pub const TOK_ASM_xor: tcc_token = 630;
pub const TOK_ASM_xorq: tcc_token = 629;
pub const TOK_ASM_xorl: tcc_token = 628;
pub const TOK_ASM_xorw: tcc_token = 627;
pub const TOK_ASM_xorb: tcc_token = 626;
pub const TOK_ASM_sub: tcc_token = 625;
pub const TOK_ASM_subq: tcc_token = 624;
pub const TOK_ASM_subl: tcc_token = 623;
pub const TOK_ASM_subw: tcc_token = 622;
pub const TOK_ASM_subb: tcc_token = 621;
pub const TOK_ASM_and: tcc_token = 620;
pub const TOK_ASM_andq: tcc_token = 619;
pub const TOK_ASM_andl: tcc_token = 618;
pub const TOK_ASM_andw: tcc_token = 617;
pub const TOK_ASM_andb: tcc_token = 616;
pub const TOK_ASM_sbb: tcc_token = 615;
pub const TOK_ASM_sbbq: tcc_token = 614;
pub const TOK_ASM_sbbl: tcc_token = 613;
pub const TOK_ASM_sbbw: tcc_token = 612;
pub const TOK_ASM_sbbb: tcc_token = 611;
pub const TOK_ASM_adc: tcc_token = 610;
pub const TOK_ASM_adcq: tcc_token = 609;
pub const TOK_ASM_adcl: tcc_token = 608;
pub const TOK_ASM_adcw: tcc_token = 607;
pub const TOK_ASM_adcb: tcc_token = 606;
pub const TOK_ASM_or: tcc_token = 605;
pub const TOK_ASM_orq: tcc_token = 604;
pub const TOK_ASM_orl: tcc_token = 603;
pub const TOK_ASM_orw: tcc_token = 602;
pub const TOK_ASM_orb: tcc_token = 601;
pub const TOK_ASM_add: tcc_token = 600;
pub const TOK_ASM_addq: tcc_token = 599;
pub const TOK_ASM_addl: tcc_token = 598;
pub const TOK_ASM_addw: tcc_token = 597;
pub const TOK_ASM_addb: tcc_token = 596;
pub const TOK_ASM_mov: tcc_token = 595;
pub const TOK_ASM_movq: tcc_token = 594;
pub const TOK_ASM_movl: tcc_token = 593;
pub const TOK_ASM_movw: tcc_token = 592;
pub const TOK_ASM_movb: tcc_token = 591;
pub const TOK_ASM_dil: tcc_token = 590;
pub const TOK_ASM_sil: tcc_token = 589;
pub const TOK_ASM_bpl: tcc_token = 588;
pub const TOK_ASM_spl: tcc_token = 587;
pub const TOK_ASM_rip: tcc_token = 586;
pub const TOK_ASM_st: tcc_token = 585;
pub const TOK_ASM_gs: tcc_token = 584;
pub const TOK_ASM_fs: tcc_token = 583;
pub const TOK_ASM_ds: tcc_token = 582;
pub const TOK_ASM_ss: tcc_token = 581;
pub const TOK_ASM_cs: tcc_token = 580;
pub const TOK_ASM_es: tcc_token = 579;
pub const TOK_ASM_dr7: tcc_token = 578;
pub const TOK_ASM_dr6: tcc_token = 577;
pub const TOK_ASM_dr5: tcc_token = 576;
pub const TOK_ASM_dr4: tcc_token = 575;
pub const TOK_ASM_dr3: tcc_token = 574;
pub const TOK_ASM_dr2: tcc_token = 573;
pub const TOK_ASM_dr1: tcc_token = 572;
pub const TOK_ASM_dr0: tcc_token = 571;
pub const TOK_ASM_db7: tcc_token = 570;
pub const TOK_ASM_db6: tcc_token = 569;
pub const TOK_ASM_db5: tcc_token = 568;
pub const TOK_ASM_db4: tcc_token = 567;
pub const TOK_ASM_db3: tcc_token = 566;
pub const TOK_ASM_db2: tcc_token = 565;
pub const TOK_ASM_db1: tcc_token = 564;
pub const TOK_ASM_db0: tcc_token = 563;
pub const TOK_ASM_tr7: tcc_token = 562;
pub const TOK_ASM_tr6: tcc_token = 561;
pub const TOK_ASM_tr5: tcc_token = 560;
pub const TOK_ASM_tr4: tcc_token = 559;
pub const TOK_ASM_tr3: tcc_token = 558;
pub const TOK_ASM_tr2: tcc_token = 557;
pub const TOK_ASM_tr1: tcc_token = 556;
pub const TOK_ASM_tr0: tcc_token = 555;
pub const TOK_ASM_cr7: tcc_token = 554;
pub const TOK_ASM_cr6: tcc_token = 553;
pub const TOK_ASM_cr5: tcc_token = 552;
pub const TOK_ASM_cr4: tcc_token = 551;
pub const TOK_ASM_cr3: tcc_token = 550;
pub const TOK_ASM_cr2: tcc_token = 549;
pub const TOK_ASM_cr1: tcc_token = 548;
pub const TOK_ASM_cr0: tcc_token = 547;
pub const TOK_ASM_xmm7: tcc_token = 546;
pub const TOK_ASM_xmm6: tcc_token = 545;
pub const TOK_ASM_xmm5: tcc_token = 544;
pub const TOK_ASM_xmm4: tcc_token = 543;
pub const TOK_ASM_xmm3: tcc_token = 542;
pub const TOK_ASM_xmm2: tcc_token = 541;
pub const TOK_ASM_xmm1: tcc_token = 540;
pub const TOK_ASM_xmm0: tcc_token = 539;
pub const TOK_ASM_mm7: tcc_token = 538;
pub const TOK_ASM_mm6: tcc_token = 537;
pub const TOK_ASM_mm5: tcc_token = 536;
pub const TOK_ASM_mm4: tcc_token = 535;
pub const TOK_ASM_mm3: tcc_token = 534;
pub const TOK_ASM_mm2: tcc_token = 533;
pub const TOK_ASM_mm1: tcc_token = 532;
pub const TOK_ASM_mm0: tcc_token = 531;
pub const TOK_ASM_rdi: tcc_token = 530;
pub const TOK_ASM_rsi: tcc_token = 529;
pub const TOK_ASM_rbp: tcc_token = 528;
pub const TOK_ASM_rsp: tcc_token = 527;
pub const TOK_ASM_rbx: tcc_token = 526;
pub const TOK_ASM_rdx: tcc_token = 525;
pub const TOK_ASM_rcx: tcc_token = 524;
pub const TOK_ASM_rax: tcc_token = 523;
pub const TOK_ASM_edi: tcc_token = 522;
pub const TOK_ASM_esi: tcc_token = 521;
pub const TOK_ASM_ebp: tcc_token = 520;
pub const TOK_ASM_esp: tcc_token = 519;
pub const TOK_ASM_ebx: tcc_token = 518;
pub const TOK_ASM_edx: tcc_token = 517;
pub const TOK_ASM_ecx: tcc_token = 516;
pub const TOK_ASM_eax: tcc_token = 515;
pub const TOK_ASM_di: tcc_token = 514;
pub const TOK_ASM_si: tcc_token = 513;
pub const TOK_ASM_bp: tcc_token = 512;
pub const TOK_ASM_sp: tcc_token = 511;
pub const TOK_ASM_bx: tcc_token = 510;
pub const TOK_ASM_dx: tcc_token = 509;
pub const TOK_ASM_cx: tcc_token = 508;
pub const TOK_ASM_ax: tcc_token = 507;
pub const TOK_ASM_bh: tcc_token = 506;
pub const TOK_ASM_dh: tcc_token = 505;
pub const TOK_ASM_ch: tcc_token = 504;
pub const TOK_ASM_ah: tcc_token = 503;
pub const TOK_ASM_bl: tcc_token = 502;
pub const TOK_ASM_dl: tcc_token = 501;
pub const TOK_ASM_cl: tcc_token = 500;
pub const TOK_ASM_al: tcc_token = 499;
pub const TOK_ASMDIR_section: tcc_token = 498;
pub const TOK_ASMDIR_symver: tcc_token = 497;
pub const TOK_ASMDIR_int: tcc_token = 496;
pub const TOK_ASMDIR_long: tcc_token = 495;
pub const TOK_ASMDIR_short: tcc_token = 494;
pub const TOK_ASMDIR_code64: tcc_token = 493;
pub const TOK_ASMDIR_quad: tcc_token = 492;
pub const TOK_ASMDIR_org: tcc_token = 491;
pub const TOK_ASMDIR_endr: tcc_token = 490;
pub const TOK_ASMDIR_rept: tcc_token = 489;
pub const TOK_ASMDIR_fill: tcc_token = 488;
pub const TOK_ASMDIR_popsection: tcc_token = 487;
pub const TOK_ASMDIR_pushsection: tcc_token = 486;
pub const TOK_ASMDIR_previous: tcc_token = 485;
pub const TOK_ASMDIR_bss: tcc_token = 484;
pub const TOK_ASMDIR_data: tcc_token = 483;
pub const TOK_ASMDIR_text: tcc_token = 482;
pub const TOK_ASMDIR_type: tcc_token = 481;
pub const TOK_ASMDIR_size: tcc_token = 480;
pub const TOK_ASMDIR_ident: tcc_token = 479;
pub const TOK_ASMDIR_hidden: tcc_token = 478;
pub const TOK_ASMDIR_weak: tcc_token = 477;
pub const TOK_ASMDIR_global: tcc_token = 476;
pub const TOK_ASMDIR_globl: tcc_token = 475;
pub const TOK_ASMDIR_file: tcc_token = 474;
pub const TOK_ASMDIR_ascii: tcc_token = 473;
pub const TOK_ASMDIR_asciz: tcc_token = 472;
pub const TOK_ASMDIR_string: tcc_token = 471;
pub const TOK_ASMDIR_space: tcc_token = 470;
pub const TOK_ASMDIR_skip: tcc_token = 469;
pub const TOK_ASMDIR_set: tcc_token = 468;
pub const TOK_ASMDIR_p2align: tcc_token = 467;
pub const TOK_ASMDIR_balign: tcc_token = 466;
pub const TOK_ASMDIR_align: tcc_token = 465;
pub const TOK_ASMDIR_word: tcc_token = 464;
pub const TOK_ASMDIR_byte: tcc_token = 463;
pub const TOK_longjmp: tcc_token = 462;
pub const TOK__setjmp: tcc_token = 461;
pub const TOK_setjmp: tcc_token = 460;
pub const TOK_siglongjmp: tcc_token = 459;
pub const TOK___sigsetjmp: tcc_token = 458;
pub const TOK_sigsetjmp: tcc_token = 457;
pub const TOK___bound_new_region: tcc_token = 456;
pub const TOK___bound_longjmp: tcc_token = 455;
pub const TOK___bound_setjmp: tcc_token = 454;
pub const TOK___bound_local_delete: tcc_token = 453;
pub const TOK___bound_local_new: tcc_token = 452;
pub const TOK___bound_main_arg: tcc_token = 451;
pub const TOK___bound_ptr_indir16: tcc_token = 450;
pub const TOK___bound_ptr_indir12: tcc_token = 449;
pub const TOK___bound_ptr_indir8: tcc_token = 448;
pub const TOK___bound_ptr_indir4: tcc_token = 447;
pub const TOK___bound_ptr_indir2: tcc_token = 446;
pub const TOK___bound_ptr_indir1: tcc_token = 445;
pub const TOK___bound_ptr_add: tcc_token = 444;
pub const TOK_alloca: tcc_token = 443;
pub const TOK___fixunsdfdi: tcc_token = 442;
pub const TOK___fixunssfdi: tcc_token = 441;
pub const TOK___fixunsxfdi: tcc_token = 440;
pub const TOK___floatundixf: tcc_token = 439;
pub const TOK___floatundidf: tcc_token = 438;
pub const TOK___floatundisf: tcc_token = 437;
pub const TOK___ashldi3: tcc_token = 436;
pub const TOK___lshrdi3: tcc_token = 435;
pub const TOK___ashrdi3: tcc_token = 434;
pub const TOK___umoddi3: tcc_token = 433;
pub const TOK___udivdi3: tcc_token = 432;
pub const TOK___moddi3: tcc_token = 431;
pub const TOK___divdi3: tcc_token = 430;
pub const TOK_memset: tcc_token = 429;
pub const TOK_memmove: tcc_token = 428;
pub const TOK_memcpy: tcc_token = 427;
pub const TOK_option: tcc_token = 426;
pub const TOK_once: tcc_token = 425;
pub const TOK_pop_macro: tcc_token = 424;
pub const TOK_push_macro: tcc_token = 423;
pub const TOK_lib: tcc_token = 422;
pub const TOK_comment: tcc_token = 421;
pub const TOK_pack: tcc_token = 420;
pub const TOK___atomic_nand_fetch: tcc_token = 419;
pub const TOK___atomic_and_fetch: tcc_token = 418;
pub const TOK___atomic_xor_fetch: tcc_token = 417;
pub const TOK___atomic_or_fetch: tcc_token = 416;
pub const TOK___atomic_sub_fetch: tcc_token = 415;
pub const TOK___atomic_add_fetch: tcc_token = 414;
pub const TOK___atomic_fetch_nand: tcc_token = 413;
pub const TOK___atomic_fetch_and: tcc_token = 412;
pub const TOK___atomic_fetch_xor: tcc_token = 411;
pub const TOK___atomic_fetch_or: tcc_token = 410;
pub const TOK___atomic_fetch_sub: tcc_token = 409;
pub const TOK___atomic_fetch_add: tcc_token = 408;
pub const TOK___atomic_compare_exchange: tcc_token = 407;
pub const TOK___atomic_exchange: tcc_token = 406;
pub const TOK___atomic_load: tcc_token = 405;
pub const TOK___atomic_store: tcc_token = 404;
pub const TOK_builtin_va_arg_types: tcc_token = 403;
pub const TOK_builtin_unreachable: tcc_token = 402;
pub const TOK_builtin_expect: tcc_token = 401;
pub const TOK_builtin_return_address: tcc_token = 400;
pub const TOK_builtin_frame_address: tcc_token = 399;
pub const TOK_builtin_constant_p: tcc_token = 398;
pub const TOK_builtin_choose_expr: tcc_token = 397;
pub const TOK_builtin_types_compatible_p: tcc_token = 396;
pub const TOK_VISIBILITY2: tcc_token = 395;
pub const TOK_VISIBILITY1: tcc_token = 394;
pub const TOK_NORETURN3: tcc_token = 393;
pub const TOK_NORETURN2: tcc_token = 392;
pub const TOK_NORETURN1: tcc_token = 391;
pub const TOK_NODECORATE: tcc_token = 390;
pub const TOK_DLLIMPORT: tcc_token = 389;
pub const TOK_DLLEXPORT: tcc_token = 388;
pub const TOK_MODE_word: tcc_token = 387;
pub const TOK_MODE_SI: tcc_token = 386;
pub const TOK_MODE_HI: tcc_token = 385;
pub const TOK_MODE_DI: tcc_token = 384;
pub const TOK_MODE_QI: tcc_token = 383;
pub const TOK_MODE: tcc_token = 382;
pub const TOK_ALWAYS_INLINE2: tcc_token = 381;
pub const TOK_ALWAYS_INLINE1: tcc_token = 380;
pub const TOK_DESTRUCTOR2: tcc_token = 379;
pub const TOK_DESTRUCTOR1: tcc_token = 378;
pub const TOK_CONSTRUCTOR2: tcc_token = 377;
pub const TOK_CONSTRUCTOR1: tcc_token = 376;
pub const TOK_CLEANUP2: tcc_token = 375;
pub const TOK_CLEANUP1: tcc_token = 374;
pub const TOK_REGPARM2: tcc_token = 373;
pub const TOK_REGPARM1: tcc_token = 372;
pub const TOK_THISCALL3: tcc_token = 371;
pub const TOK_THISCALL2: tcc_token = 370;
pub const TOK_THISCALL1: tcc_token = 369;
pub const TOK_FASTCALL3: tcc_token = 368;
pub const TOK_FASTCALL2: tcc_token = 367;
pub const TOK_FASTCALL1: tcc_token = 366;
pub const TOK_STDCALL3: tcc_token = 365;
pub const TOK_STDCALL2: tcc_token = 364;
pub const TOK_STDCALL1: tcc_token = 363;
pub const TOK_CDECL3: tcc_token = 362;
pub const TOK_CDECL2: tcc_token = 361;
pub const TOK_CDECL1: tcc_token = 360;
pub const TOK_NODEBUG2: tcc_token = 359;
pub const TOK_NODEBUG1: tcc_token = 358;
pub const TOK_UNUSED2: tcc_token = 357;
pub const TOK_UNUSED1: tcc_token = 356;
pub const TOK_ALIAS2: tcc_token = 355;
pub const TOK_ALIAS1: tcc_token = 354;
pub const TOK_WEAK2: tcc_token = 353;
pub const TOK_WEAK1: tcc_token = 352;
pub const TOK_PACKED2: tcc_token = 351;
pub const TOK_PACKED1: tcc_token = 350;
pub const TOK_ALIGNED2: tcc_token = 349;
pub const TOK_ALIGNED1: tcc_token = 348;
pub const TOK_SECTION2: tcc_token = 347;
pub const TOK_SECTION1: tcc_token = 346;
pub const TOK___INF__: tcc_token = 345;
pub const TOK___SNAN__: tcc_token = 344;
pub const TOK___NAN__: tcc_token = 343;
pub const TOK___FUNC__: tcc_token = 342;
pub const TOK___HAS_INCLUDE_NEXT: tcc_token = 341;
pub const TOK___HAS_INCLUDE: tcc_token = 340;
pub const TOK___COUNTER__: tcc_token = 339;
pub const TOK___VA_ARGS__: tcc_token = 338;
pub const TOK___FUNCTION__: tcc_token = 337;
pub const TOK___TIME__: tcc_token = 336;
pub const TOK___DATE__: tcc_token = 335;
pub const TOK___FILE__: tcc_token = 334;
pub const TOK___LINE__: tcc_token = 333;
pub const TOK_PRAGMA: tcc_token = 332;
pub const TOK_LINE: tcc_token = 331;
pub const TOK_WARNING: tcc_token = 330;
pub const TOK_ERROR: tcc_token = 329;
pub const TOK_UNDEF: tcc_token = 328;
pub const TOK_DEFINED: tcc_token = 327;
pub const TOK_ENDIF: tcc_token = 326;
pub const TOK_ELIF: tcc_token = 325;
pub const TOK_IFNDEF: tcc_token = 324;
pub const TOK_IFDEF: tcc_token = 323;
pub const TOK_INCLUDE_NEXT: tcc_token = 322;
pub const TOK_INCLUDE: tcc_token = 321;
pub const TOK_DEFINE: tcc_token = 320;
pub const TOK_LABEL: tcc_token = 319;
pub const TOK_TYPEOF3: tcc_token = 318;
pub const TOK_TYPEOF2: tcc_token = 317;
pub const TOK_TYPEOF1: tcc_token = 316;
pub const TOK_ALIGNAS: tcc_token = 315;
pub const TOK_ALIGNOF3: tcc_token = 314;
pub const TOK_ALIGNOF2: tcc_token = 313;
pub const TOK_ALIGNOF1: tcc_token = 312;
pub const TOK_ATTRIBUTE2: tcc_token = 311;
pub const TOK_ATTRIBUTE1: tcc_token = 310;
pub const TOK_SIZEOF: tcc_token = 309;
pub const TOK_ENUM: tcc_token = 308;
pub const TOK_TYPEDEF: tcc_token = 307;
pub const TOK_UNION: tcc_token = 306;
pub const TOK_STRUCT: tcc_token = 305;
pub const TOK_LONG: tcc_token = 304;
pub const TOK_SHORT: tcc_token = 303;
pub const TOK_COMPLEX: tcc_token = 302;
pub const TOK_BOOL: tcc_token = 301;
pub const TOK_DOUBLE: tcc_token = 300;
pub const TOK_FLOAT: tcc_token = 299;
pub const TOK_INT: tcc_token = 298;
pub const TOK_CHAR: tcc_token = 297;
pub const TOK_VOID: tcc_token = 296;
pub const TOK_STATIC_ASSERT: tcc_token = 295;
pub const TOK_GENERIC: tcc_token = 294;
pub const TOK_THREAD_LOCAL: tcc_token = 293;
pub const TOK_EXTENSION: tcc_token = 292;
pub const TOK_RESTRICT3: tcc_token = 291;
pub const TOK_RESTRICT2: tcc_token = 290;
pub const TOK_RESTRICT1: tcc_token = 289;
pub const TOK_INLINE3: tcc_token = 288;
pub const TOK_INLINE2: tcc_token = 287;
pub const TOK_INLINE1: tcc_token = 286;
pub const TOK_AUTO: tcc_token = 285;
pub const TOK_SIGNED3: tcc_token = 284;
pub const TOK_SIGNED2: tcc_token = 283;
pub const TOK_SIGNED1: tcc_token = 282;
pub const TOK_REGISTER: tcc_token = 281;
pub const TOK_VOLATILE3: tcc_token = 280;
pub const TOK_VOLATILE2: tcc_token = 279;
pub const TOK_VOLATILE1: tcc_token = 278;
pub const TOK_CONST3: tcc_token = 277;
pub const TOK_CONST2: tcc_token = 276;
pub const TOK_CONST1: tcc_token = 275;
pub const TOK__Atomic: tcc_token = 274;
pub const TOK_UNSIGNED: tcc_token = 273;
pub const TOK_STATIC: tcc_token = 272;
pub const TOK_EXTERN: tcc_token = 271;
pub const TOK_ASM3: tcc_token = 270;
pub const TOK_ASM2: tcc_token = 269;
pub const TOK_ASM1: tcc_token = 268;
pub const TOK_DEFAULT: tcc_token = 267;
pub const TOK_CASE: tcc_token = 266;
pub const TOK_SWITCH: tcc_token = 265;
pub const TOK_GOTO: tcc_token = 264;
pub const TOK_RETURN: tcc_token = 263;
pub const TOK_BREAK: tcc_token = 262;
pub const TOK_CONTINUE: tcc_token = 261;
pub const TOK_DO: tcc_token = 260;
pub const TOK_FOR: tcc_token = 259;
pub const TOK_WHILE: tcc_token = 258;
pub const TOK_ELSE: tcc_token = 257;
pub const TOK_IF: tcc_token = 256;
pub const TOK_LAST: tcc_token = 255;
pub type line_macro_output_format = libc::c_uint;
pub const LINE_MACRO_OUTPUT_FORMAT_P10: line_macro_output_format = 11;
pub const LINE_MACRO_OUTPUT_FORMAT_STD: line_macro_output_format = 2;
pub const LINE_MACRO_OUTPUT_FORMAT_NONE: line_macro_output_format = 1;
pub const LINE_MACRO_OUTPUT_FORMAT_GCC: line_macro_output_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TinyAlloc {
    pub limit: libc::c_uint,
    pub size: libc::c_uint,
    pub buffer: *mut uint8_t,
    pub p: *mut uint8_t,
    pub nb_allocs: libc::c_uint,
    pub next: *mut TinyAlloc,
    pub top: *mut TinyAlloc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tal_header_t {
    pub size: libc::c_uint,
}
#[inline]
unsafe extern "C" fn is_space(mut ch: libc::c_int) -> libc::c_int {
    return (ch == ' ' as i32 || ch == '\t' as i32 || ch == '\u{b}' as i32
        || ch == '\u{c}' as i32 || ch == '\r' as i32) as libc::c_int;
}
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
unsafe extern "C" fn isoct(mut c: libc::c_int) -> libc::c_int {
    return (c >= '0' as i32 && c <= '7' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn toup(mut c: libc::c_int) -> libc::c_int {
    return if c >= 'a' as i32 && c <= 'z' as i32 {
        c - 'a' as i32 + 'A' as i32
    } else {
        c
    };
}
#[no_mangle]
pub static mut tok_flags: libc::c_int = 0;
#[no_mangle]
pub static mut parse_flags: libc::c_int = 0;
#[no_mangle]
pub static mut file: *mut BufferedFile = 0 as *const BufferedFile as *mut BufferedFile;
#[no_mangle]
pub static mut tok: libc::c_int = 0;
#[no_mangle]
pub static mut tokc: CValue = CValue { ld: f128::f128::ZERO };
#[no_mangle]
pub static mut macro_ptr: *const libc::c_int = 0 as *const libc::c_int;
#[no_mangle]
pub static mut tokcstr: CString = CString {
    size: 0,
    size_allocated: 0,
    data: 0 as *const libc::c_char as *mut libc::c_char,
};
#[no_mangle]
pub static mut tok_ident: libc::c_int = 0;
#[no_mangle]
pub static mut table_ident: *mut *mut TokenSym = 0 as *const *mut TokenSym
    as *mut *mut TokenSym;
#[no_mangle]
pub static mut pp_expr: libc::c_int = 0;
static mut hash_ident: [*mut TokenSym; 16384] = [0 as *const TokenSym
    as *mut TokenSym; 16384];
static mut token_buf: [libc::c_char; 1025] = [0; 1025];
static mut cstr_buf: CString = CString {
    size: 0,
    size_allocated: 0,
    data: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut tokstr_buf: TokenString = TokenString {
    str_0: 0 as *const libc::c_int as *mut libc::c_int,
    len: 0,
    need_spc: 0,
    allocated_len: 0,
    last_line_num: 0,
    save_line_num: 0,
    prev: 0 as *const TokenString as *mut TokenString,
    prev_ptr: 0 as *const libc::c_int,
    alloc: 0,
};
static mut unget_buf: TokenString = TokenString {
    str_0: 0 as *const libc::c_int as *mut libc::c_int,
    len: 0,
    need_spc: 0,
    allocated_len: 0,
    last_line_num: 0,
    save_line_num: 0,
    prev: 0 as *const TokenString as *mut TokenString,
    prev_ptr: 0 as *const libc::c_int,
    alloc: 0,
};
static mut isidnum_table: [libc::c_uchar; 257] = [0; 257];
static mut pp_debug_tok: libc::c_int = 0;
static mut pp_debug_symv: libc::c_int = 0;
static mut pp_counter: libc::c_int = 0;
static mut toksym_alloc: *mut TinyAlloc = 0 as *const TinyAlloc as *mut TinyAlloc;
static mut tokstr_alloc: *mut TinyAlloc = 0 as *const TinyAlloc as *mut TinyAlloc;
static mut macro_stack: *mut TokenString = 0 as *const TokenString as *mut TokenString;
static mut tcc_keywords: [libc::c_char; 7148] = unsafe {
    *::core::mem::transmute::<
        &[u8; 7148],
        &[libc::c_char; 7148],
    >(
        b"if\0else\0while\0for\0do\0continue\0break\0return\0goto\0switch\0case\0default\0asm\0__asm\0__asm__\0extern\0static\0unsigned\0_Atomic\0const\0__const\0__const__\0volatile\0__volatile\0__volatile__\0register\0signed\0__signed\0__signed__\0auto\0inline\0__inline\0__inline__\0restrict\0__restrict\0__restrict__\0__extension__\0_Thread_local\0_Generic\0_Static_assert\0void\0char\0int\0float\0double\0_Bool\0_Complex\0short\0long\0struct\0union\0typedef\0enum\0sizeof\0__attribute\0__attribute__\0__alignof\0__alignof__\0_Alignof\0_Alignas\0typeof\0__typeof\0__typeof__\0__label__\0define\0include\0include_next\0ifdef\0ifndef\0elif\0endif\0defined\0undef\0error\0warning\0line\0pragma\0__LINE__\0__FILE__\0__DATE__\0__TIME__\0__FUNCTION__\0__VA_ARGS__\0__COUNTER__\0__has_include\0__has_include_next\0__func__\0__nan__\0__snan__\0__inf__\0section\0__section__\0aligned\0__aligned__\0packed\0__packed__\0weak\0__weak__\0alias\0__alias__\0unused\0__unused__\0nodebug\0__nodebug__\0cdecl\0__cdecl\0__cdecl__\0stdcall\0__stdcall\0__stdcall__\0fastcall\0__fastcall\0__fastcall__\0thiscall\0__thiscall\0__thiscall__\0regparm\0__regparm__\0cleanup\0__cleanup__\0constructor\0__constructor__\0destructor\0__destructor__\0always_inline\0__always_inline__\0__mode__\0__QI__\0__DI__\0__HI__\0__SI__\0__word__\0dllexport\0dllimport\0nodecorate\0noreturn\0__noreturn__\0_Noreturn\0visibility\0__visibility__\0__builtin_types_compatible_p\0__builtin_choose_expr\0__builtin_constant_p\0__builtin_frame_address\0__builtin_return_address\0__builtin_expect\0__builtin_unreachable\0__builtin_va_arg_types\0__atomic_store\0__atomic_load\0__atomic_exchange\0__atomic_compare_exchange\0__atomic_fetch_add\0__atomic_fetch_sub\0__atomic_fetch_or\0__atomic_fetch_xor\0__atomic_fetch_and\0__atomic_fetch_nand\0__atomic_add_fetch\0__atomic_sub_fetch\0__atomic_or_fetch\0__atomic_xor_fetch\0__atomic_and_fetch\0__atomic_nand_fetch\0pack\0comment\0lib\0push_macro\0pop_macro\0once\0option\0memcpy\0memmove\0memset\0__divdi3\0__moddi3\0__udivdi3\0__umoddi3\0__ashrdi3\0__lshrdi3\0__ashldi3\0__floatundisf\0__floatundidf\0__floatundixf\0__fixunsxfdi\0__fixunssfdi\0__fixunsdfdi\0alloca\0__bound_ptr_add\0__bound_ptr_indir1\0__bound_ptr_indir2\0__bound_ptr_indir4\0__bound_ptr_indir8\0__bound_ptr_indir12\0__bound_ptr_indir16\0__bound_main_arg\0__bound_local_new\0__bound_local_delete\0__bound_setjmp\0__bound_longjmp\0__bound_new_region\0sigsetjmp\0__sigsetjmp\0siglongjmp\0setjmp\0_setjmp\0longjmp\0.byte\0.word\0.align\0.balign\0.p2align\0.set\0.skip\0.space\0.string\0.asciz\0.ascii\0.file\0.globl\0.global\0.weak\0.hidden\0.ident\0.size\0.type\0.text\0.data\0.bss\0.previous\0.pushsection\0.popsection\0.fill\0.rept\0.endr\0.org\0.quad\0.code64\0.short\0.long\0.int\0.symver\0.section\0al\0cl\0dl\0bl\0ah\0ch\0dh\0bh\0ax\0cx\0dx\0bx\0sp\0bp\0si\0di\0eax\0ecx\0edx\0ebx\0esp\0ebp\0esi\0edi\0rax\0rcx\0rdx\0rbx\0rsp\0rbp\0rsi\0rdi\0mm0\0mm1\0mm2\0mm3\0mm4\0mm5\0mm6\0mm7\0xmm0\0xmm1\0xmm2\0xmm3\0xmm4\0xmm5\0xmm6\0xmm7\0cr0\0cr1\0cr2\0cr3\0cr4\0cr5\0cr6\0cr7\0tr0\0tr1\0tr2\0tr3\0tr4\0tr5\0tr6\0tr7\0db0\0db1\0db2\0db3\0db4\0db5\0db6\0db7\0dr0\0dr1\0dr2\0dr3\0dr4\0dr5\0dr6\0dr7\0es\0cs\0ss\0ds\0fs\0gs\0st\0rip\0spl\0bpl\0sil\0dil\0movb\0movw\0movl\0movq\0mov\0addb\0addw\0addl\0addq\0add\0orb\0orw\0orl\0orq\0or\0adcb\0adcw\0adcl\0adcq\0adc\0sbbb\0sbbw\0sbbl\0sbbq\0sbb\0andb\0andw\0andl\0andq\0and\0subb\0subw\0subl\0subq\0sub\0xorb\0xorw\0xorl\0xorq\0xor\0cmpb\0cmpw\0cmpl\0cmpq\0cmp\0incb\0incw\0incl\0incq\0inc\0decb\0decw\0decl\0decq\0dec\0notb\0notw\0notl\0notq\0not\0negb\0negw\0negl\0negq\0neg\0mulb\0mulw\0mull\0mulq\0mul\0imulb\0imulw\0imull\0imulq\0imul\0divb\0divw\0divl\0divq\0div\0idivb\0idivw\0idivl\0idivq\0idiv\0xchgb\0xchgw\0xchgl\0xchgq\0xchg\0testb\0testw\0testl\0testq\0test\0rolb\0rolw\0roll\0rolq\0rol\0rorb\0rorw\0rorl\0rorq\0ror\0rclb\0rclw\0rcll\0rclq\0rcl\0rcrb\0rcrw\0rcrl\0rcrq\0rcr\0shlb\0shlw\0shll\0shlq\0shl\0shrb\0shrw\0shrl\0shrq\0shr\0sarb\0sarw\0sarl\0sarq\0sar\0shldw\0shldl\0shldq\0shld\0shrdw\0shrdl\0shrdq\0shrd\0pushw\0pushl\0pushq\0push\0popw\0popl\0popq\0pop\0inb\0inw\0inl\0in\0outb\0outw\0outl\0out\0movzbw\0movzbl\0movzbq\0movzb\0movzwl\0movsbw\0movsbl\0movswl\0movsbq\0movswq\0movzwq\0movslq\0leaw\0leal\0leaq\0lea\0les\0lds\0lss\0lfs\0lgs\0call\0jmp\0lcall\0ljmp\0jo\0jno\0jb\0jc\0jnae\0jnb\0jnc\0jae\0je\0jz\0jne\0jnz\0jbe\0jna\0jnbe\0ja\0js\0jns\0jp\0jpe\0jnp\0jpo\0jl\0jnge\0jnl\0jge\0jle\0jng\0jnle\0jg\0seto\0setno\0setb\0setc\0setnae\0setnb\0setnc\0setae\0sete\0setz\0setne\0setnz\0setbe\0setna\0setnbe\0seta\0sets\0setns\0setp\0setpe\0setnp\0setpo\0setl\0setnge\0setnl\0setge\0setle\0setng\0setnle\0setg\0setob\0setnob\0setbb\0setcb\0setnaeb\0setnbb\0setncb\0setaeb\0seteb\0setzb\0setneb\0setnzb\0setbeb\0setnab\0setnbeb\0setab\0setsb\0setnsb\0setpb\0setpeb\0setnpb\0setpob\0setlb\0setngeb\0setnlb\0setgeb\0setleb\0setngb\0setnleb\0setgb\0cmovo\0cmovno\0cmovb\0cmovc\0cmovnae\0cmovnb\0cmovnc\0cmovae\0cmove\0cmovz\0cmovne\0cmovnz\0cmovbe\0cmovna\0cmovnbe\0cmova\0cmovs\0cmovns\0cmovp\0cmovpe\0cmovnp\0cmovpo\0cmovl\0cmovnge\0cmovnl\0cmovge\0cmovle\0cmovng\0cmovnle\0cmovg\0bsfw\0bsfl\0bsfq\0bsf\0bsrw\0bsrl\0bsrq\0bsr\0btw\0btl\0btq\0bt\0btsw\0btsl\0btsq\0bts\0btrw\0btrl\0btrq\0btr\0btcw\0btcl\0btcq\0btc\0popcntw\0popcntl\0popcntq\0popcnt\0tzcntw\0tzcntl\0tzcntq\0tzcnt\0lzcntw\0lzcntl\0lzcntq\0lzcnt\0larw\0larl\0larq\0lar\0lslw\0lsll\0lslq\0lsl\0fadd\0faddp\0fadds\0fiaddl\0faddl\0fiadds\0fmul\0fmulp\0fmuls\0fimull\0fmull\0fimuls\0fcom\0fcom_1\0fcoms\0ficoml\0fcoml\0ficoms\0fcomp\0fcompp\0fcomps\0ficompl\0fcompl\0ficomps\0fsub\0fsubp\0fsubs\0fisubl\0fsubl\0fisubs\0fsubr\0fsubrp\0fsubrs\0fisubrl\0fsubrl\0fisubrs\0fdiv\0fdivp\0fdivs\0fidivl\0fdivl\0fidivs\0fdivr\0fdivrp\0fdivrs\0fidivrl\0fdivrl\0fidivrs\0xaddb\0xaddw\0xaddl\0xaddq\0xadd\0cmpxchgb\0cmpxchgw\0cmpxchgl\0cmpxchgq\0cmpxchg\0cmpsb\0cmpsw\0cmpsl\0cmpsq\0cmps\0scmpb\0scmpw\0scmpl\0scmpq\0scmp\0insb\0insw\0insl\0ins\0outsb\0outsw\0outsl\0outs\0lodsb\0lodsw\0lodsl\0lodsq\0lods\0slodb\0slodw\0slodl\0slodq\0slod\0movsb\0movsw\0movsl\0movsq\0movs\0smovb\0smovw\0smovl\0smovq\0smov\0scasb\0scasw\0scasl\0scasq\0scas\0sscab\0sscaw\0sscal\0sscaq\0ssca\0stosb\0stosw\0stosl\0stosq\0stos\0sstob\0sstow\0sstol\0sstoq\0ssto\0clc\0cld\0cli\0clts\0cmc\0lahf\0sahf\0pushfq\0popfq\0pushf\0popf\0stc\0std\0sti\0aaa\0aas\0daa\0das\0aad\0aam\0cbw\0cwd\0cwde\0cdq\0cbtw\0cwtl\0cwtd\0cltd\0cqto\0int3\0into\0iret\0iretw\0iretl\0iretq\0rsm\0hlt\0wait\0nop\0pause\0xlat\0lock\0rep\0repe\0repz\0repne\0repnz\0invd\0wbinvd\0cpuid\0wrmsr\0rdtsc\0rdmsr\0rdpmc\0syscall\0sysret\0ud2\0leave\0ret\0retq\0lret\0fucompp\0ftst\0fxam\0fld1\0fldl2t\0fldl2e\0fldpi\0fldlg2\0fldln2\0fldz\0f2xm1\0fyl2x\0fptan\0fpatan\0fxtract\0fprem1\0fdecstp\0fincstp\0fprem\0fyl2xp1\0fsqrt\0fsincos\0frndint\0fscale\0fsin\0fcos\0fchs\0fabs\0fninit\0fnclex\0fnop\0fwait\0fxch\0fnstsw\0emms\0vmcall\0vmlaunch\0vmresume\0vmxoff\0sysretq\0callq\0ljmpw\0ljmpl\0enter\0loopne\0loopnz\0loope\0loopz\0loop\0jecxz\0fld\0fldl\0flds\0fildl\0fildq\0fildll\0fldt\0fbld\0fst\0fstl\0fsts\0fstps\0fstpl\0fist\0fistp\0fistl\0fistpl\0fstp\0fistpq\0fistpll\0fstpt\0fbstp\0fucom\0fucomp\0finit\0fldcw\0fnstcw\0fstcw\0fstsw\0fclex\0fnstenv\0fstenv\0fldenv\0fnsave\0fsave\0frstor\0ffree\0ffreep\0fxsave\0fxrstor\0fxsaveq\0fxrstorq\0arpl\0lgdt\0lgdtq\0lidt\0lidtq\0lldt\0lmsw\0ltr\0sgdt\0sgdtq\0sidt\0sidtq\0sldt\0smsw\0str\0verr\0verw\0swapgs\0bswap\0bswapl\0bswapq\0invlpg\0cmpxchg8b\0cmpxchg16b\0fcmovb\0fcmove\0fcmovbe\0fcmovu\0fcmovnb\0fcmovne\0fcmovnbe\0fcmovnu\0fucomi\0fcomi\0fucomip\0fcomip\0movd\0packssdw\0packsswb\0packuswb\0paddb\0paddw\0paddd\0paddsb\0paddsw\0paddusb\0paddusw\0pand\0pandn\0pcmpeqb\0pcmpeqw\0pcmpeqd\0pcmpgtb\0pcmpgtw\0pcmpgtd\0pmaddwd\0pmulhw\0pmullw\0por\0psllw\0pslld\0psllq\0psraw\0psrad\0psrlw\0psrld\0psrlq\0psubb\0psubw\0psubd\0psubsb\0psubsw\0psubusb\0psubusw\0punpckhbw\0punpckhwd\0punpckhdq\0punpcklbw\0punpcklwd\0punpckldq\0pxor\0ldmxcsr\0stmxcsr\0movups\0movaps\0movhps\0addps\0cvtpi2ps\0cvtps2pi\0cvttps2pi\0divps\0maxps\0minps\0mulps\0pavgb\0pavgw\0pmaxsw\0pmaxub\0pminsw\0pminub\0rcpss\0rsqrtps\0sqrtps\0subps\0movnti\0movntil\0movntiq\0prefetchnta\0prefetcht0\0prefetcht1\0prefetcht2\0prefetchw\0lfence\0mfence\0sfence\0clflush\0endbr64\0\0",
    )
};
static mut tok_two_chars: [libc::c_uchar; 64] = [
    '<' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0x9e as libc::c_int as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0x9d as libc::c_int as libc::c_uchar,
    '!' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0x95 as libc::c_int as libc::c_uchar,
    '&' as i32 as libc::c_uchar,
    '&' as i32 as libc::c_uchar,
    0x90 as libc::c_int as libc::c_uchar,
    '|' as i32 as libc::c_uchar,
    '|' as i32 as libc::c_uchar,
    0x91 as libc::c_int as libc::c_uchar,
    '+' as i32 as libc::c_uchar,
    '+' as i32 as libc::c_uchar,
    0x82 as libc::c_int as libc::c_uchar,
    '-' as i32 as libc::c_uchar,
    '-' as i32 as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0x94 as libc::c_int as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    '<' as i32 as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    '+' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb0 as libc::c_int as libc::c_uchar,
    '-' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb1 as libc::c_int as libc::c_uchar,
    '*' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb2 as libc::c_int as libc::c_uchar,
    '/' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb3 as libc::c_int as libc::c_uchar,
    '%' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb4 as libc::c_int as libc::c_uchar,
    '&' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb5 as libc::c_int as libc::c_uchar,
    '^' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb7 as libc::c_int as libc::c_uchar,
    '|' as i32 as libc::c_uchar,
    '=' as i32 as libc::c_uchar,
    0xb6 as libc::c_int as libc::c_uchar,
    '-' as i32 as libc::c_uchar,
    '>' as i32 as libc::c_uchar,
    0xa0 as libc::c_int as libc::c_uchar,
    '.' as i32 as libc::c_uchar,
    '.' as i32 as libc::c_uchar,
    0xa2 as libc::c_int as libc::c_uchar,
    '#' as i32 as libc::c_uchar,
    '#' as i32 as libc::c_uchar,
    0xa3 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn skip(mut c: libc::c_int) {
    if tok != c {
        let mut tmp: [libc::c_char; 40] = [0; 40];
        pstrcpy(
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            get_tok_str(c, &mut tokc),
        );
        _tcc_error(
            b"'%s' expected (got \"%s\")\0" as *const u8 as *const libc::c_char,
            tmp.as_mut_ptr(),
            get_tok_str(tok, &mut tokc),
        );
    }
    next();
}
#[no_mangle]
pub unsafe extern "C" fn expect(mut msg: *const libc::c_char) -> ! {
    _tcc_error(b"%s expected\0" as *const u8 as *const libc::c_char, msg);
}
unsafe extern "C" fn tal_new(
    mut pal: *mut *mut TinyAlloc,
    mut limit: libc::c_uint,
    mut size: libc::c_uint,
) -> *mut TinyAlloc {
    let mut al: *mut TinyAlloc = tcc_mallocz(
        ::core::mem::size_of::<TinyAlloc>() as libc::c_ulong,
    ) as *mut TinyAlloc;
    (*al).buffer = tcc_malloc(size as libc::c_ulong) as *mut uint8_t;
    (*al).p = (*al).buffer;
    (*al).limit = limit;
    (*al).size = size;
    if !pal.is_null() {
        *pal = al;
    }
    return al;
}
unsafe extern "C" fn tal_delete(mut al: *mut TinyAlloc) {
    let mut next_0: *mut TinyAlloc = 0 as *mut TinyAlloc;
    loop {
        if al.is_null() {
            return;
        }
        next_0 = (*al).next;
        tcc_free((*al).buffer as *mut libc::c_void);
        tcc_free(al as *mut libc::c_void);
        al = next_0;
    };
}
unsafe extern "C" fn tal_free_impl(mut al: *mut TinyAlloc, mut p: *mut libc::c_void) {
    if p.is_null() {
        return;
    }
    loop {
        if (*al).buffer <= p as *mut uint8_t
            && (p as *mut uint8_t) < ((*al).buffer).offset((*al).size as isize)
        {
            (*al).nb_allocs = ((*al).nb_allocs).wrapping_sub(1);
            (*al).nb_allocs;
            if (*al).nb_allocs == 0 {
                (*al).p = (*al).buffer;
            }
            break;
        } else if !((*al).next).is_null() {
            al = (*al).next;
        } else {
            tcc_free(p);
            break;
        }
    };
}
unsafe extern "C" fn tal_realloc_impl(
    mut pal: *mut *mut TinyAlloc,
    mut p: *mut libc::c_void,
    mut size: libc::c_uint,
) -> *mut libc::c_void {
    let mut header: *mut tal_header_t = 0 as *mut tal_header_t;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut is_own: libc::c_int = 0;
    let mut adj_size: libc::c_uint = size.wrapping_add(3 as libc::c_int as libc::c_uint)
        & -(4 as libc::c_int) as libc::c_uint;
    let mut al: *mut TinyAlloc = *pal;
    loop {
        is_own = ((*al).buffer <= p as *mut uint8_t
            && (p as *mut uint8_t) < ((*al).buffer).offset((*al).size as isize))
            as libc::c_int;
        if (p.is_null() || is_own != 0) && size <= (*al).limit {
            if ((((*al).p).offset_from((*al).buffer) as libc::c_long
                + adj_size as libc::c_long) as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<tal_header_t>() as libc::c_ulong)
                < (*al).size as libc::c_ulong
            {
                header = (*al).p as *mut tal_header_t;
                (*header).size = adj_size;
                ret = ((*al).p)
                    .offset(
                        ::core::mem::size_of::<tal_header_t>() as libc::c_ulong as isize,
                    ) as *mut libc::c_void;
                (*al)
                    .p = ((*al).p)
                    .offset(
                        (adj_size as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<tal_header_t>() as libc::c_ulong,
                            ) as isize,
                    );
                if is_own != 0 {
                    header = (p as *mut tal_header_t)
                        .offset(-(1 as libc::c_int as isize));
                    if !p.is_null() {
                        memcpy(ret, p, (*header).size as libc::c_ulong);
                    }
                } else {
                    (*al).nb_allocs = ((*al).nb_allocs).wrapping_add(1);
                    (*al).nb_allocs;
                }
                return ret;
            } else if is_own != 0 {
                (*al).nb_allocs = ((*al).nb_allocs).wrapping_sub(1);
                (*al).nb_allocs;
                ret = tal_realloc_impl(pal, 0 as *mut libc::c_void, size);
                header = (p as *mut tal_header_t).offset(-(1 as libc::c_int as isize));
                if !p.is_null() {
                    memcpy(ret, p, (*header).size as libc::c_ulong);
                }
                return ret;
            }
            if !((*al).next).is_null() {
                al = (*al).next;
            } else {
                let mut bottom: *mut TinyAlloc = al;
                let mut next_0: *mut TinyAlloc = if !((*al).top).is_null() {
                    (*al).top
                } else {
                    al
                };
                al = tal_new(
                    pal,
                    (*next_0).limit,
                    ((*next_0).size).wrapping_mul(2 as libc::c_int as libc::c_uint),
                );
                (*al).next = next_0;
                (*bottom).top = al;
            }
        } else if is_own != 0 {
            (*al).nb_allocs = ((*al).nb_allocs).wrapping_sub(1);
            (*al).nb_allocs;
            ret = tcc_malloc(size as libc::c_ulong);
            header = (p as *mut tal_header_t).offset(-(1 as libc::c_int as isize));
            if !p.is_null() {
                memcpy(ret, p, (*header).size as libc::c_ulong);
            }
            break;
        } else if !((*al).next).is_null() {
            al = (*al).next;
        } else {
            ret = tcc_realloc(p, size as libc::c_ulong);
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn cstr_realloc(mut cstr: *mut CString, mut new_size: libc::c_int) {
    let mut size: libc::c_int = 0;
    size = (*cstr).size_allocated;
    if size < 8 as libc::c_int {
        size = 8 as libc::c_int;
    }
    while size < new_size {
        size = size * 2 as libc::c_int;
    }
    (*cstr)
        .data = tcc_realloc((*cstr).data as *mut libc::c_void, size as libc::c_ulong)
        as *mut libc::c_char;
    (*cstr).size_allocated = size;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_ccat(mut cstr: *mut CString, mut ch: libc::c_int) {
    let mut size: libc::c_int = 0;
    size = (*cstr).size + 1 as libc::c_int;
    if size > (*cstr).size_allocated {
        cstr_realloc(cstr, size);
    }
    *((*cstr).data).offset((size - 1 as libc::c_int) as isize) = ch as libc::c_char;
    (*cstr).size = size;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_to_utf8(
    mut b: *mut libc::c_char,
    mut Uc: uint32_t,
) -> *mut libc::c_char {
    if Uc < 0x80 as libc::c_int as uint32_t {
        let fresh0 = b;
        b = b.offset(1);
        *fresh0 = Uc as libc::c_char;
    } else if Uc < 0x800 as libc::c_int as uint32_t {
        let fresh1 = b;
        b = b.offset(1);
        *fresh1 = (192 as libc::c_int as uint32_t)
            .wrapping_add(Uc / 64 as libc::c_int as uint32_t) as libc::c_char;
        let fresh2 = b;
        b = b.offset(1);
        *fresh2 = (128 as libc::c_int as uint32_t)
            .wrapping_add(Uc % 64 as libc::c_int as uint32_t) as libc::c_char;
    } else {
        's_37: {
            let mut current_block_4: u64;
            if !(Uc.wrapping_sub(0xd800 as libc::c_uint)
                < 0x800 as libc::c_int as libc::c_uint)
            {
                if Uc < 0x10000 as libc::c_int as uint32_t {
                    let fresh3 = b;
                    b = b.offset(1);
                    *fresh3 = (224 as libc::c_int as uint32_t)
                        .wrapping_add(Uc / 4096 as libc::c_int as uint32_t)
                        as libc::c_char;
                    let fresh4 = b;
                    b = b.offset(1);
                    *fresh4 = (128 as libc::c_int as uint32_t)
                        .wrapping_add(
                            Uc / 64 as libc::c_int as uint32_t
                                % 64 as libc::c_int as uint32_t,
                        ) as libc::c_char;
                    let fresh5 = b;
                    b = b.offset(1);
                    *fresh5 = (128 as libc::c_int as uint32_t)
                        .wrapping_add(Uc % 64 as libc::c_int as uint32_t)
                        as libc::c_char;
                    current_block_4 = 10879442775620481940;
                } else if Uc < 0x110000 as libc::c_int as uint32_t {
                    let fresh6 = b;
                    b = b.offset(1);
                    *fresh6 = (240 as libc::c_int as uint32_t)
                        .wrapping_add(Uc / 262144 as libc::c_int as uint32_t)
                        as libc::c_char;
                    let fresh7 = b;
                    b = b.offset(1);
                    *fresh7 = (128 as libc::c_int as uint32_t)
                        .wrapping_add(
                            Uc / 4096 as libc::c_int as uint32_t
                                % 64 as libc::c_int as uint32_t,
                        ) as libc::c_char;
                    let fresh8 = b;
                    b = b.offset(1);
                    *fresh8 = (128 as libc::c_int as uint32_t)
                        .wrapping_add(
                            Uc / 64 as libc::c_int as uint32_t
                                % 64 as libc::c_int as uint32_t,
                        ) as libc::c_char;
                    let fresh9 = b;
                    b = b.offset(1);
                    *fresh9 = (128 as libc::c_int as uint32_t)
                        .wrapping_add(Uc % 64 as libc::c_int as uint32_t)
                        as libc::c_char;
                    current_block_4 = 10879442775620481940;
                } else {
                    current_block_4 = 9609468495279740674;
                }
                match current_block_4 {
                    9609468495279740674 => {}
                    _ => {
                        break 's_37;
                    }
                }
            }
            _tcc_error(
                b"0x%x is not a valid universal character\0" as *const u8
                    as *const libc::c_char,
                Uc,
            );
        }
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_u8cat(mut cstr: *mut CString, mut ch: libc::c_int) {
    let mut buf: [libc::c_char; 4] = [0; 4];
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    e = unicode_to_utf8(buf.as_mut_ptr(), ch as uint32_t);
    cstr_cat(
        cstr,
        buf.as_mut_ptr(),
        e.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cstr_cat(
    mut cstr: *mut CString,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) {
    let mut size: libc::c_int = 0;
    if len <= 0 as libc::c_int {
        len = (strlen(str))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(len as libc::c_ulong) as libc::c_int;
    }
    size = (*cstr).size + len;
    if size > (*cstr).size_allocated {
        cstr_realloc(cstr, size);
    }
    memmove(
        ((*cstr).data).offset((*cstr).size as isize) as *mut libc::c_void,
        str as *const libc::c_void,
        len as libc::c_ulong,
    );
    (*cstr).size = size;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_wccat(mut cstr: *mut CString, mut ch: libc::c_int) {
    let mut size: libc::c_int = 0;
    size = ((*cstr).size as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<nwchar_t>() as libc::c_ulong)
        as libc::c_int;
    if size > (*cstr).size_allocated {
        cstr_realloc(cstr, size);
    }
    *(((*cstr).data)
        .offset(size as isize)
        .offset(-(::core::mem::size_of::<nwchar_t>() as libc::c_ulong as isize))
        as *mut nwchar_t) = ch;
    (*cstr).size = size;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_new(mut cstr: *mut CString) {
    memset(
        cstr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<CString>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cstr_free(mut cstr: *mut CString) {
    tcc_free((*cstr).data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cstr_reset(mut cstr: *mut CString) {
    (*cstr).size = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_vprintf(
    mut cstr: *mut CString,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut v: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 80 as libc::c_int;
    loop {
        size += (*cstr).size;
        if size > (*cstr).size_allocated {
            cstr_realloc(cstr, size);
        }
        size = (*cstr).size_allocated - (*cstr).size;
        v = ap.clone();
        len = vsnprintf(
            ((*cstr).data).offset((*cstr).size as isize),
            size as libc::c_ulong,
            fmt,
            v.as_va_list(),
        );
        if len >= 0 as libc::c_int && len < size {
            break;
        }
        size *= 2 as libc::c_int;
    }
    (*cstr).size += len;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn cstr_printf(
    mut cstr: *mut CString,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    ap = args.clone();
    len = cstr_vprintf(cstr, fmt, ap.as_va_list());
    return len;
}
unsafe extern "C" fn add_char(mut cstr: *mut CString, mut c: libc::c_int) {
    if c == '\'' as i32 || c == '"' as i32 || c == '\\' as i32 {
        cstr_ccat(cstr, '\\' as i32);
    }
    if c >= 32 as libc::c_int && c <= 126 as libc::c_int {
        cstr_ccat(cstr, c);
    } else {
        cstr_ccat(cstr, '\\' as i32);
        if c == '\n' as i32 {
            cstr_ccat(cstr, 'n' as i32);
        } else {
            cstr_ccat(cstr, '0' as i32 + (c >> 6 as libc::c_int & 7 as libc::c_int));
            cstr_ccat(cstr, '0' as i32 + (c >> 3 as libc::c_int & 7 as libc::c_int));
            cstr_ccat(cstr, '0' as i32 + (c & 7 as libc::c_int));
        }
    };
}
unsafe extern "C" fn tok_alloc_new(
    mut pts: *mut *mut TokenSym,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut TokenSym {
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut ptable: *mut *mut TokenSym = 0 as *mut *mut TokenSym;
    let mut i: libc::c_int = 0;
    if tok_ident >= 0x10000000 as libc::c_int {
        _tcc_error(b"memory full (symbols)\0" as *const u8 as *const libc::c_char);
    }
    i = tok_ident - 256 as libc::c_int;
    if i % 512 as libc::c_int == 0 as libc::c_int {
        ptable = tcc_realloc(
            table_ident as *mut libc::c_void,
            ((i + 512 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut TokenSym>() as libc::c_ulong),
        ) as *mut *mut TokenSym;
        table_ident = ptable;
    }
    ts = tal_realloc_impl(
        &mut toksym_alloc,
        0 as *mut libc::c_void,
        (::core::mem::size_of::<TokenSym>() as libc::c_ulong)
            .wrapping_add(len as libc::c_ulong) as libc::c_uint,
    ) as *mut TokenSym;
    let ref mut fresh10 = *table_ident.offset(i as isize);
    *fresh10 = ts;
    let fresh11 = tok_ident;
    tok_ident = tok_ident + 1;
    (*ts).tok = fresh11;
    (*ts).sym_define = 0 as *mut Sym;
    (*ts).sym_label = 0 as *mut Sym;
    (*ts).sym_struct = 0 as *mut Sym;
    (*ts).sym_identifier = 0 as *mut Sym;
    (*ts).len = len;
    (*ts).hash_next = 0 as *mut TokenSym;
    memcpy(
        ((*ts).str_0).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        len as libc::c_ulong,
    );
    *((*ts).str_0).as_mut_ptr().offset(len as isize) = '\0' as i32 as libc::c_char;
    *pts = ts;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn tok_alloc(
    mut str: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut TokenSym {
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut pts: *mut *mut TokenSym = 0 as *mut *mut TokenSym;
    let mut i: libc::c_int = 0;
    let mut h: libc::c_uint = 0;
    h = 1 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < len {
        h = h
            .wrapping_add(h << 5 as libc::c_int)
            .wrapping_add(h >> 27 as libc::c_int)
            .wrapping_add(
                *(str as *mut libc::c_uchar).offset(i as isize) as libc::c_uint,
            );
        i += 1;
        i;
    }
    h &= (16384 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    pts = &mut *hash_ident.as_mut_ptr().offset(h as isize) as *mut *mut TokenSym;
    loop {
        ts = *pts;
        if ts.is_null() {
            break;
        }
        if (*ts).len == len
            && memcmp(
                ((*ts).str_0).as_mut_ptr() as *const libc::c_void,
                str as *const libc::c_void,
                len as libc::c_ulong,
            ) == 0
        {
            return ts;
        }
        pts = &mut (*ts).hash_next;
    }
    return tok_alloc_new(pts, str, len);
}
#[no_mangle]
pub unsafe extern "C" fn tok_alloc_const(mut str: *const libc::c_char) -> libc::c_int {
    return (*tok_alloc(str, strlen(str) as libc::c_int)).tok;
}
#[no_mangle]
pub unsafe extern "C" fn get_tok_str(
    mut v: libc::c_int,
    mut cv: *mut CValue,
) -> *const libc::c_char {
    let mut q: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut current_block: u64;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    cstr_reset(&mut cstr_buf);
    p = cstr_buf.data;
    match v {
        194 | 195 | 198 | 199 | 196 | 197 => {
            sprintf(
                p,
                b"%llu\0" as *const u8 as *const libc::c_char,
                (*cv).i as libc::c_ulonglong,
            );
            current_block = 1724319918354933278;
        }
        193 => {
            cstr_ccat(&mut cstr_buf, 'L' as i32);
            current_block = 2938981289739956360;
        }
        192 => {
            current_block = 2938981289739956360;
        }
        205 | 206 => return (*cv).str_0.data,
        201 => {
            cstr_ccat(&mut cstr_buf, 'L' as i32);
            current_block = 1827290030907757406;
        }
        200 => {
            current_block = 1827290030907757406;
        }
        202 => return strcpy(p, b"<float>\0" as *const u8 as *const libc::c_char),
        203 => return strcpy(p, b"<double>\0" as *const u8 as *const libc::c_char),
        204 => return strcpy(p, b"<long double>\0" as *const u8 as *const libc::c_char),
        207 => return strcpy(p, b"<linenumber>\0" as *const u8 as *const libc::c_char),
        156 => {
            v = '<' as i32;
            current_block = 11059698821735974459;
        }
        159 => {
            v = '>' as i32;
            current_block = 11059698821735974459;
        }
        161 => return strcpy(p, b"...\0" as *const u8 as *const libc::c_char),
        184 => return strcpy(p, b"<<=\0" as *const u8 as *const libc::c_char),
        185 => return strcpy(p, b">>=\0" as *const u8 as *const libc::c_char),
        -1 => return strcpy(p, b"<eof>\0" as *const u8 as *const libc::c_char),
        0 => return strcpy(p, b"<no name>\0" as *const u8 as *const libc::c_char),
        _ => {
            v &= !(0x20000000 as libc::c_int | 0x40000000 as libc::c_int);
            if v < 256 as libc::c_int {
                q = tok_two_chars.as_ptr();
                while *q != 0 {
                    if *q.offset(2 as libc::c_int as isize) as libc::c_int == v {
                        let fresh12 = p;
                        p = p.offset(1);
                        *fresh12 = *q.offset(0 as libc::c_int as isize) as libc::c_char;
                        let fresh13 = p;
                        p = p.offset(1);
                        *fresh13 = *q.offset(1 as libc::c_int as isize) as libc::c_char;
                        *p = '\0' as i32 as libc::c_char;
                        return cstr_buf.data;
                    }
                    q = q.offset(3 as libc::c_int as isize);
                }
                if v >= 127 as libc::c_int
                    || v < 32 as libc::c_int && is_space(v) == 0 && v != '\n' as i32
                {
                    sprintf(p, b"<\\x%02x>\0" as *const u8 as *const libc::c_char, v);
                    current_block = 1724319918354933278;
                } else {
                    current_block = 11059698821735974459;
                }
            } else {
                if v < tok_ident {
                    return ((**table_ident.offset((v - 256 as libc::c_int) as isize))
                        .str_0)
                        .as_mut_ptr()
                } else if v >= 0x10000000 as libc::c_int {
                    sprintf(
                        p,
                        b"L.%u\0" as *const u8 as *const libc::c_char,
                        v - 0x10000000 as libc::c_int,
                    );
                } else {
                    return 0 as *const libc::c_char
                }
                current_block = 1724319918354933278;
            }
        }
    }
    match current_block {
        1827290030907757406 => {
            cstr_ccat(&mut cstr_buf, '"' as i32);
            if v == 0xc8 as libc::c_int {
                len = (*cv).str_0.size - 1 as libc::c_int;
                i = 0 as libc::c_int;
                while i < len {
                    add_char(
                        &mut cstr_buf,
                        *((*cv).str_0.data as *mut libc::c_uchar).offset(i as isize)
                            as libc::c_int,
                    );
                    i += 1;
                    i;
                }
            } else {
                len = ((*cv).str_0.size as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<nwchar_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                i = 0 as libc::c_int;
                while i < len {
                    add_char(
                        &mut cstr_buf,
                        *((*cv).str_0.data as *mut nwchar_t).offset(i as isize),
                    );
                    i += 1;
                    i;
                }
            }
            cstr_ccat(&mut cstr_buf, '"' as i32);
            cstr_ccat(&mut cstr_buf, '\0' as i32);
        }
        2938981289739956360 => {
            cstr_ccat(&mut cstr_buf, '\'' as i32);
            add_char(&mut cstr_buf, (*cv).i as libc::c_int);
            cstr_ccat(&mut cstr_buf, '\'' as i32);
            cstr_ccat(&mut cstr_buf, '\0' as i32);
        }
        11059698821735974459 => {
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = v as libc::c_char;
            *p = '\0' as i32 as libc::c_char;
        }
        _ => {}
    }
    return cstr_buf.data;
}
unsafe extern "C" fn handle_eob() -> libc::c_int {
    let mut bf: *mut BufferedFile = file;
    let mut len: libc::c_int = 0;
    if (*bf).buf_ptr >= (*bf).buf_end {
        if (*bf).fd >= 0 as libc::c_int {
            len = 8192 as libc::c_int;
            len = read(
                (*bf).fd,
                ((*bf).buffer).as_mut_ptr() as *mut libc::c_void,
                len as size_t,
            ) as libc::c_int;
            if len < 0 as libc::c_int {
                len = 0 as libc::c_int;
            }
        } else {
            len = 0 as libc::c_int;
        }
        (*tcc_state)
            .total_bytes = ((*tcc_state).total_bytes).wrapping_add(len as libc::c_uint);
        (*bf).buf_ptr = ((*bf).buffer).as_mut_ptr();
        (*bf).buf_end = ((*bf).buffer).as_mut_ptr().offset(len as isize);
        *(*bf).buf_end = '\\' as i32 as uint8_t;
    }
    if (*bf).buf_ptr < (*bf).buf_end {
        return *((*bf).buf_ptr).offset(0 as libc::c_int as isize) as libc::c_int
    } else {
        (*bf).buf_ptr = (*bf).buf_end;
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn next_c() -> libc::c_int {
    (*file).buf_ptr = ((*file).buf_ptr).offset(1);
    let mut ch: libc::c_int = *(*file).buf_ptr as libc::c_int;
    if ch == '\\' as i32 && (*file).buf_ptr >= (*file).buf_end {
        ch = handle_eob();
    }
    return ch;
}
unsafe extern "C" fn handle_stray_noerror(mut err: libc::c_int) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    loop {
        ch = next_c();
        if !(ch == '\\' as i32) {
            break;
        }
        ch = next_c();
        let mut current_block_6: u64;
        if !(ch == '\n' as i32) {
            if ch == '\r' as i32 {
                ch = next_c();
                if ch == '\n' as i32 {
                    current_block_6 = 15704241260956307476;
                } else {
                    (*file).buf_ptr = ((*file).buf_ptr).offset(-1);
                    *(*file).buf_ptr = '\r' as i32 as uint8_t;
                    current_block_6 = 7815301370352969686;
                }
            } else {
                current_block_6 = 7815301370352969686;
            }
            match current_block_6 {
                15704241260956307476 => {}
                _ => {
                    if err != 0 {
                        _tcc_error(
                            b"stray '\\' in program\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    (*file).buf_ptr = ((*file).buf_ptr).offset(-1);
                    *(*file).buf_ptr = '\\' as i32 as uint8_t;
                    return *(*file).buf_ptr as libc::c_int;
                }
            }
        }
        (*file).line_num += 1;
        (*file).line_num;
    }
    return ch;
}
unsafe extern "C" fn handle_bs(mut p: *mut *mut uint8_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    (*file).buf_ptr = (*p).offset(-(1 as libc::c_int as isize));
    c = handle_stray_noerror(0 as libc::c_int);
    *p = (*file).buf_ptr;
    return c;
}
unsafe extern "C" fn handle_stray(mut p: *mut *mut uint8_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    (*file).buf_ptr = (*p).offset(-(1 as libc::c_int as isize));
    c = handle_stray_noerror((parse_flags & 0x20 as libc::c_int == 0) as libc::c_int);
    *p = (*file).buf_ptr;
    return c;
}
unsafe extern "C" fn skip_spaces() -> libc::c_int {
    let mut ch: libc::c_int = 0;
    (*file).buf_ptr = ((*file).buf_ptr).offset(-1);
    (*file).buf_ptr;
    loop {
        ch = handle_stray_noerror(0 as libc::c_int);
        if !(isidnum_table[(ch - -(1 as libc::c_int)) as usize] as libc::c_int
            & 1 as libc::c_int != 0)
        {
            break;
        }
    }
    return ch;
}
unsafe extern "C" fn parse_line_comment(mut p: *mut uint8_t) -> *mut uint8_t {
    let mut c: libc::c_int = 0;
    's_14: loop {
        p = p.offset(1);
        c = *p as libc::c_int;
        loop {
            if !(c == '\n' as i32 || c == '\\' as i32) {
                p = p.offset(1);
                c = *p as libc::c_int;
                if !(c == '\n' as i32 || c == '\\' as i32) {
                    break;
                }
            }
            if c == '\n' as i32 {
                break 's_14;
            }
            c = handle_bs(&mut p);
            if c == -(1 as libc::c_int) {
                break 's_14;
            }
            if !(c != '\\' as i32) {
                break;
            }
        }
    }
    return p;
}
unsafe extern "C" fn parse_comment(mut p: *mut uint8_t) -> *mut uint8_t {
    let mut c: libc::c_int = 0;
    's_14: loop {
        p = p.offset(1);
        c = *p as libc::c_int;
        loop {
            if !(c == '\n' as i32 || c == '*' as i32 || c == '\\' as i32) {
                p = p.offset(1);
                c = *p as libc::c_int;
                if !(c == '\n' as i32 || c == '*' as i32 || c == '\\' as i32) {
                    continue 's_14;
                }
            }
            if c == '\n' as i32 {
                break;
            }
            if c == '*' as i32 {
                loop {
                    p = p.offset(1);
                    c = *p as libc::c_int;
                    if !(c == '*' as i32) {
                        break;
                    }
                }
                if c == '\\' as i32 {
                    c = handle_bs(&mut p);
                }
                if c == '/' as i32 {
                    break 's_14;
                }
            } else {
                c = handle_bs(&mut p);
            }
            if c == -(1 as libc::c_int) {
                _tcc_error(
                    b"unexpected end of file in comment\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !(c != '\\' as i32) {
                continue 's_14;
            }
        }
        (*file).line_num += 1;
        (*file).line_num;
    }
    return p.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn parse_pp_string(
    mut p: *mut uint8_t,
    mut sep: libc::c_int,
    mut str: *mut CString,
) -> *mut uint8_t {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    's_8: loop {
        p = p.offset(1);
        c = *p as libc::c_int;
        loop {
            if c == sep {
                break 's_8;
            }
            if c == '\\' as i32 {
                c = handle_bs(&mut p);
                if c == -(1 as libc::c_int) {
                    current_block = 7229837030640418248;
                    break;
                }
                if !(c == '\\' as i32) {
                    continue;
                }
                if !str.is_null() {
                    cstr_ccat(str, c);
                }
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    current_block = 2868539653012386629;
                    break;
                } else {
                    current_block = 4438186764655786454;
                    break;
                }
            } else {
                if c == '\n' as i32 {
                    current_block = 7149356873433890176;
                    break;
                }
                if !(c == '\r' as i32) {
                    current_block = 4438186764655786454;
                    break;
                }
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_bs(&mut p);
                }
                if c == '\n' as i32 {
                    current_block = 7149356873433890176;
                    break;
                }
                if c == -(1 as libc::c_int) {
                    current_block = 7229837030640418248;
                    break;
                }
                if !str.is_null() {
                    cstr_ccat(str, '\r' as i32);
                }
            }
        }
        match current_block {
            2868539653012386629 => {
                c = handle_bs(&mut p);
                if c == -(1 as libc::c_int) {
                    current_block = 7229837030640418248;
                } else {
                    current_block = 4438186764655786454;
                }
            }
            7149356873433890176 => {
                if !str.is_null() {
                    current_block = 7229837030640418248;
                } else {
                    return p
                }
            }
            _ => {}
        }
        match current_block {
            4438186764655786454 => {
                if !str.is_null() {
                    cstr_ccat(str, c);
                }
            }
            _ => {
                tok_flags &= !(0x1 as libc::c_int);
                _tcc_error(
                    b"missing terminating %c character\0" as *const u8
                        as *const libc::c_char,
                    sep,
                );
            }
        }
    }
    p = p.offset(1);
    p;
    return p;
}
unsafe extern "C" fn preprocess_skip() {
    let mut current_block: u64;
    let mut a: libc::c_int = 0;
    let mut start_of_line: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut in_warn_or_error: libc::c_int = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    p = (*file).buf_ptr;
    a = 0 as libc::c_int;
    '_redo_start: loop {
        start_of_line = 1 as libc::c_int;
        in_warn_or_error = 0 as libc::c_int;
        loop {
            c = *p as libc::c_int;
            match c {
                32 | 9 | 12 | 11 | 13 => {
                    p = p.offset(1);
                    p;
                    continue;
                }
                10 => {
                    (*file).line_num += 1;
                    (*file).line_num;
                    p = p.offset(1);
                    p;
                    break;
                }
                92 => {
                    c = handle_bs(&mut p);
                    if c == -(1 as libc::c_int) {
                        expect(b"#endif\0" as *const u8 as *const libc::c_char);
                    }
                    if c == '\\' as i32 {
                        p = p.offset(1);
                        p;
                    }
                    continue;
                }
                34 | 39 => {
                    if in_warn_or_error != 0 {
                        current_block = 2651605330182484733;
                    } else {
                        tok_flags &= !(0x1 as libc::c_int);
                        p = parse_pp_string(p, c, 0 as *mut CString);
                        current_block = 721385680381463314;
                    }
                }
                47 => {
                    if in_warn_or_error != 0 {
                        current_block = 2651605330182484733;
                    } else {
                        p = p.offset(1);
                        p;
                        c = handle_bs(&mut p);
                        if c == '*' as i32 {
                            p = parse_comment(p);
                        } else if c == '/' as i32 {
                            p = parse_line_comment(p);
                        }
                        continue;
                    }
                }
                35 => {
                    p = p.offset(1);
                    p;
                    if start_of_line != 0 {
                        (*file).buf_ptr = p;
                        next_nomacro();
                        p = (*file).buf_ptr;
                        if a == 0 as libc::c_int
                            && (tok == TOK_ELSE as libc::c_int
                                || tok == TOK_ELIF as libc::c_int
                                || tok == TOK_ENDIF as libc::c_int)
                        {
                            break '_redo_start;
                        }
                        if tok == TOK_IF as libc::c_int
                            || tok == TOK_IFDEF as libc::c_int
                            || tok == TOK_IFNDEF as libc::c_int
                        {
                            a += 1;
                            a;
                        } else if tok == TOK_ENDIF as libc::c_int {
                            a -= 1;
                            a;
                        } else if tok == TOK_ERROR as libc::c_int
                            || tok == TOK_WARNING as libc::c_int
                        {
                            in_warn_or_error = 1 as libc::c_int;
                        } else {
                            if tok == 10 as libc::c_int {
                                break;
                            }
                            if parse_flags & 0x8 as libc::c_int != 0 {
                                p = parse_line_comment(
                                    p.offset(-(1 as libc::c_int as isize)),
                                );
                            }
                        }
                    } else if parse_flags & 0x8 as libc::c_int != 0 {
                        p = parse_line_comment(p.offset(-(1 as libc::c_int as isize)));
                    }
                    current_block = 721385680381463314;
                }
                _ => {
                    current_block = 2651605330182484733;
                }
            }
            match current_block {
                2651605330182484733 => {
                    p = p.offset(1);
                    p;
                }
                _ => {}
            }
            start_of_line = 0 as libc::c_int;
        }
    }
    (*file).buf_ptr = p;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_new(mut s: *mut TokenString) {
    (*s).str_0 = 0 as *mut libc::c_int;
    (*s).need_spc = 0 as libc::c_int;
    (*s).len = (*s).need_spc;
    (*s).allocated_len = 0 as libc::c_int;
    (*s).last_line_num = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_alloc() -> *mut TokenString {
    let mut str: *mut TokenString = tal_realloc_impl(
        &mut tokstr_alloc,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<TokenString>() as libc::c_ulong as libc::c_uint,
    ) as *mut TokenString;
    tok_str_new(str);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_free_str(mut str: *mut libc::c_int) {
    tal_free_impl(tokstr_alloc, str as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_free(mut str: *mut TokenString) {
    tok_str_free_str((*str).str_0);
    tal_free_impl(tokstr_alloc, str as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_realloc(
    mut s: *mut TokenString,
    mut new_size: libc::c_int,
) -> *mut libc::c_int {
    let mut str: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut size: libc::c_int = 0;
    size = (*s).allocated_len;
    if size < 16 as libc::c_int {
        size = 16 as libc::c_int;
    }
    while size < new_size {
        size = size * 2 as libc::c_int;
    }
    if size > (*s).allocated_len {
        str = tal_realloc_impl(
            &mut tokstr_alloc,
            (*s).str_0 as *mut libc::c_void,
            (size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_int;
        (*s).allocated_len = size;
        (*s).str_0 = str;
    }
    return (*s).str_0;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_add(mut s: *mut TokenString, mut t: libc::c_int) {
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_int = 0 as *mut libc::c_int;
    len = (*s).len;
    str = (*s).str_0;
    if len >= (*s).allocated_len {
        str = tok_str_realloc(s, len + 1 as libc::c_int);
    }
    let fresh15 = len;
    len = len + 1;
    *str.offset(fresh15 as isize) = t;
    (*s).len = len;
}
#[no_mangle]
pub unsafe extern "C" fn begin_macro(mut str: *mut TokenString, mut alloc: libc::c_int) {
    (*str).alloc = alloc as libc::c_char;
    (*str).prev = macro_stack;
    (*str).prev_ptr = macro_ptr;
    (*str).save_line_num = (*file).line_num;
    macro_ptr = (*str).str_0;
    macro_stack = str;
}
#[no_mangle]
pub unsafe extern "C" fn end_macro() {
    let mut str: *mut TokenString = macro_stack;
    macro_stack = (*str).prev;
    macro_ptr = (*str).prev_ptr;
    (*file).line_num = (*str).save_line_num;
    if (*str).alloc as libc::c_int == 0 as libc::c_int {
        (*str).need_spc = 0 as libc::c_int;
        (*str).len = (*str).need_spc;
    } else {
        if (*str).alloc as libc::c_int == 2 as libc::c_int {
            (*str).str_0 = 0 as *mut libc::c_int;
        }
        tok_str_free(str);
    };
}
unsafe extern "C" fn tok_str_add2(
    mut s: *mut TokenString,
    mut t: libc::c_int,
    mut cv: *mut CValue,
) {
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_int = 0 as *mut libc::c_int;
    len = (*s).len;
    str = (*s).str_0;
    if len + 4 as libc::c_int >= (*s).allocated_len {
        str = tok_str_realloc(s, len + 4 as libc::c_int + 1 as libc::c_int);
    }
    let fresh16 = len;
    len = len + 1;
    *str.offset(fresh16 as isize) = t;
    match t {
        194 | 195 | 192 | 193 | 202 | 207 => {
            let fresh17 = len;
            len = len + 1;
            *str.offset(fresh17 as isize) = (*cv).tab[0 as libc::c_int as usize];
        }
        205 | 206 | 200 | 201 => {
            let mut nb_words: size_t = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    ((*cv).str_0.size as libc::c_ulong)
                        .wrapping_add(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
            if (len as size_t).wrapping_add(nb_words) >= (*s).allocated_len as size_t {
                str = tok_str_realloc(
                    s,
                    (len as size_t)
                        .wrapping_add(nb_words)
                        .wrapping_add(1 as libc::c_int as size_t) as libc::c_int,
                );
            }
            *str.offset(len as isize) = (*cv).str_0.size;
            memcpy(
                &mut *str.offset((len + 1 as libc::c_int) as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                (*cv).str_0.data as *const libc::c_void,
                (*cv).str_0.size as libc::c_ulong,
            );
            len = (len as size_t).wrapping_add(nb_words) as libc::c_int as libc::c_int;
        }
        203 | 196 | 197 | 198 | 199 => {
            let fresh18 = len;
            len = len + 1;
            *str.offset(fresh18 as isize) = (*cv).tab[0 as libc::c_int as usize];
            let fresh19 = len;
            len = len + 1;
            *str.offset(fresh19 as isize) = (*cv).tab[1 as libc::c_int as usize];
        }
        204 => {
            let fresh20 = len;
            len = len + 1;
            *str.offset(fresh20 as isize) = (*cv).tab[0 as libc::c_int as usize];
            let fresh21 = len;
            len = len + 1;
            *str.offset(fresh21 as isize) = (*cv).tab[1 as libc::c_int as usize];
            let fresh22 = len;
            len = len + 1;
            *str.offset(fresh22 as isize) = (*cv).tab[2 as libc::c_int as usize];
            let fresh23 = len;
            len = len + 1;
            *str.offset(fresh23 as isize) = (*cv).tab[3 as libc::c_int as usize];
        }
        _ => {}
    }
    (*s).len = len;
}
#[no_mangle]
pub unsafe extern "C" fn tok_str_add_tok(mut s: *mut TokenString) {
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    if (*file).line_num != (*s).last_line_num {
        (*s).last_line_num = (*file).line_num;
        cval.i = (*s).last_line_num as uint64_t;
        tok_str_add2(s, 0xcf as libc::c_int, &mut cval);
    }
    tok_str_add2(s, tok, &mut tokc);
}
unsafe extern "C" fn tok_str_add2_spc(
    mut s: *mut TokenString,
    mut t: libc::c_int,
    mut cv: *mut CValue,
) {
    if (*s).need_spc == 3 as libc::c_int {
        tok_str_add(s, ' ' as i32);
    }
    (*s).need_spc = 2 as libc::c_int;
    tok_str_add2(s, t, cv);
}
#[inline]
unsafe extern "C" fn tok_get(
    mut t: *mut libc::c_int,
    mut pp: *mut *const libc::c_int,
    mut cv: *mut CValue,
) {
    let mut p: *const libc::c_int = *pp;
    let mut n: libc::c_int = 0;
    let mut tab: *mut libc::c_int = 0 as *mut libc::c_int;
    tab = ((*cv).tab).as_mut_ptr();
    let mut current_block_10: u64;
    let fresh24 = p;
    p = p.offset(1);
    *t = *fresh24;
    match *t {
        194 | 192 | 193 | 207 => {
            let fresh25 = p;
            p = p.offset(1);
            (*cv).i = *fresh25 as uint64_t;
            current_block_10 = 5948590327928692120;
        }
        195 => {
            let fresh26 = p;
            p = p.offset(1);
            (*cv).i = *fresh26 as libc::c_uint as uint64_t;
            current_block_10 = 5948590327928692120;
        }
        202 => {
            let fresh27 = p;
            p = p.offset(1);
            *tab.offset(0 as libc::c_int as isize) = *fresh27;
            current_block_10 = 5948590327928692120;
        }
        200 | 201 | 205 | 206 => {
            let fresh28 = p;
            p = p.offset(1);
            (*cv).str_0.size = *fresh28;
            (*cv).str_0.data = p as *mut libc::c_char;
            p = p
                .offset(
                    ((*cv).str_0.size as libc::c_ulong)
                        .wrapping_add(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) as isize,
                );
            current_block_10 = 5948590327928692120;
        }
        203 | 196 | 197 | 198 | 199 => {
            n = 2 as libc::c_int;
            current_block_10 = 4825009266913157377;
        }
        204 => {
            n = 4 as libc::c_int;
            current_block_10 = 4825009266913157377;
        }
        _ => {
            current_block_10 = 5948590327928692120;
        }
    }
    match current_block_10 {
        4825009266913157377 => {
            loop {
                let fresh29 = p;
                p = p.offset(1);
                let fresh30 = tab;
                tab = tab.offset(1);
                *fresh30 = *fresh29;
                n -= 1;
                if !(n != 0) {
                    break;
                }
            }
        }
        _ => {}
    }
    *pp = p;
}
unsafe extern "C" fn macro_is_equal(
    mut a: *const libc::c_int,
    mut b: *const libc::c_int,
) -> libc::c_int {
    let mut cv: CValue = CValue { ld: f128::f128::ZERO };
    let mut t: libc::c_int = 0;
    if a.is_null() || b.is_null() {
        return 1 as libc::c_int;
    }
    while *a != 0 && *b != 0 {
        cstr_reset(&mut tokcstr);
        let mut _t: libc::c_int = *a;
        if _t >= 0xc0 as libc::c_int && _t <= 0xcf as libc::c_int {
            tok_get(&mut t, &mut a, &mut cv);
        } else {
            t = _t;
            a = a.offset(1);
            a;
        }
        cstr_cat(&mut tokcstr, get_tok_str(t, &mut cv), 0 as libc::c_int);
        let mut _t_0: libc::c_int = *b;
        if _t_0 >= 0xc0 as libc::c_int && _t_0 <= 0xcf as libc::c_int {
            tok_get(&mut t, &mut b, &mut cv);
        } else {
            t = _t_0;
            b = b.offset(1);
            b;
        }
        if strcmp(tokcstr.data, get_tok_str(t, &mut cv)) != 0 {
            return 0 as libc::c_int;
        }
    }
    return !(*a != 0 || *b != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn define_push(
    mut v: libc::c_int,
    mut macro_type: libc::c_int,
    mut str: *mut libc::c_int,
    mut first_arg: *mut Sym,
) {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut o: *mut Sym = 0 as *mut Sym;
    o = define_find(v);
    s = sym_push2(&mut define_stack, v, macro_type, 0 as libc::c_int);
    (*s).c2rust_unnamed.d = str;
    (*s).c2rust_unnamed_0.next = first_arg;
    let ref mut fresh31 = (**table_ident.offset((v - 256 as libc::c_int) as isize))
        .sym_define;
    *fresh31 = s;
    if !o.is_null() && macro_is_equal((*o).c2rust_unnamed.d, (*s).c2rust_unnamed.d) == 0
    {
        _tcc_warning(
            b"%s redefined\0" as *const u8 as *const libc::c_char,
            get_tok_str(v, 0 as *mut CValue),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_undef(mut s: *mut Sym) {
    let mut v: libc::c_int = (*s).v;
    if v >= 256 as libc::c_int && v < tok_ident {
        let ref mut fresh32 = (**table_ident.offset((v - 256 as libc::c_int) as isize))
            .sym_define;
        *fresh32 = 0 as *mut Sym;
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_find(mut v: libc::c_int) -> *mut Sym {
    v -= 256 as libc::c_int;
    if v as libc::c_uint >= (tok_ident - 256 as libc::c_int) as libc::c_uint {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_define;
}
#[no_mangle]
pub unsafe extern "C" fn free_defines(mut b: *mut Sym) {
    while define_stack != b {
        let mut top: *mut Sym = define_stack;
        define_stack = (*top).prev;
        tok_str_free_str((*top).c2rust_unnamed.d);
        define_undef(top);
        sym_free(top);
    }
}
unsafe extern "C" fn maybe_run_test(mut s: *mut TCCState) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if (*s).include_stack_ptr != ((*s).include_stack).as_mut_ptr() {
        return;
    }
    p = get_tok_str(tok, 0 as *mut CValue);
    if 0 as libc::c_int
        != memcmp(
            p as *const libc::c_void,
            b"test_\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        )
    {
        return;
    }
    (*s).run_test -= 1;
    if 0 as libc::c_int != (*s).run_test {
        return;
    }
    fprintf(
        (*s).ppfp,
        &*(b"\n[%s]\n\0" as *const u8 as *const libc::c_char)
            .offset(
                ((*s).dflag as libc::c_int & 32 as libc::c_int == 0) as libc::c_int
                    as isize,
            ) as *const libc::c_char,
        p,
    );
    fflush((*s).ppfp);
    define_push(tok, 0 as libc::c_int, 0 as *mut libc::c_int, 0 as *mut Sym);
}
#[no_mangle]
pub unsafe extern "C" fn skip_to_eol(mut warn: libc::c_int) {
    if tok == 10 as libc::c_int {
        return;
    }
    if warn != 0 {
        _tcc_warning(
            b"extra tokens after directive\0" as *const u8 as *const libc::c_char,
        );
    }
    while !macro_stack.is_null() {
        end_macro();
    }
    (*file)
        .buf_ptr = parse_line_comment(
        ((*file).buf_ptr).offset(-(1 as libc::c_int as isize)),
    );
    next_nomacro();
}
unsafe extern "C" fn parse_include(
    mut s1: *mut TCCState,
    mut do_next: libc::c_int,
    mut test: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut CachedInclude = 0 as *mut CachedInclude;
    c = skip_spaces();
    if c == '<' as i32 || c == '"' as i32 {
        cstr_reset(&mut tokcstr);
        (*file)
            .buf_ptr = parse_pp_string(
            (*file).buf_ptr,
            if c == '<' as i32 { '>' as i32 } else { c },
            &mut tokcstr,
        );
        i = tokcstr.size;
        pstrncpy(
            name.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            tokcstr.data,
            i as size_t,
        );
        next_nomacro();
    } else {
        parse_flags = 0x1 as libc::c_int | 0x4 as libc::c_int
            | parse_flags & 0x8 as libc::c_int;
        name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        loop {
            next();
            p = name.as_mut_ptr();
            i = (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            if i > 0 as libc::c_int
                && (*p.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
                    && *p.offset(i as isize) as libc::c_int == '"' as i32
                    || *p.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32
                        && *p.offset(i as isize) as libc::c_int == '>' as i32)
            {
                break;
            }
            if tok == 10 as libc::c_int {
                _tcc_error(
                    b"'#include' expects \"FILENAME\" or <FILENAME>\0" as *const u8
                        as *const libc::c_char,
                );
            }
            pstrcat(
                name.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                get_tok_str(tok, &mut tokc),
            );
        }
        c = *p.offset(0 as libc::c_int as isize) as libc::c_int;
        memmove(
            p as *mut libc::c_void,
            p.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (i - 1 as libc::c_int) as libc::c_ulong,
        );
        *p.offset((i - 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char;
    }
    if test == 0 {
        skip_to_eol(1 as libc::c_int);
    }
    i = if do_next != 0 { (*file).include_next_index } else { -(1 as libc::c_int) };
    loop {
        i += 1;
        i;
        if i == 0 as libc::c_int {
            if !(name[0 as libc::c_int as usize] as libc::c_int == '/' as i32) {
                continue;
            }
            buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        } else if i == 1 as libc::c_int {
            if c != '"' as i32 {
                continue;
            }
            p = (*file).true_filename;
            pstrncpy(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                p,
                (tcc_basename(p)).offset_from(p) as libc::c_long as size_t,
            );
        } else {
            let mut j: libc::c_int = i - 2 as libc::c_int;
            let mut k: libc::c_int = j - (*s1).nb_include_paths;
            if k < 0 as libc::c_int {
                p = *((*s1).include_paths).offset(j as isize);
            } else if k < (*s1).nb_sysinclude_paths {
                p = *((*s1).sysinclude_paths).offset(k as isize);
            } else if test != 0 {
                return 0 as libc::c_int
            } else {
                _tcc_error(
                    b"include file '%s' not found\0" as *const u8 as *const libc::c_char,
                    name.as_mut_ptr(),
                );
            }
            pstrcpy(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                p,
            );
            pstrcat(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"/\0" as *const u8 as *const libc::c_char,
            );
        }
        pstrcat(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            name.as_mut_ptr(),
        );
        e = search_cached_include(s1, buf.as_mut_ptr(), 0 as libc::c_int);
        if !e.is_null()
            && (!(define_find((*e).ifndef_macro)).is_null() || (*e).once != 0)
        {
            return 1 as libc::c_int;
        }
        if tcc_open(s1, buf.as_mut_ptr()) >= 0 as libc::c_int {
            break;
        }
    }
    if test != 0 {
        tcc_close();
    } else {
        if (*s1).include_stack_ptr
            >= ((*s1).include_stack).as_mut_ptr().offset(32 as libc::c_int as isize)
        {
            _tcc_error(
                b"#include recursion too deep\0" as *const u8 as *const libc::c_char,
            );
        }
        let fresh33 = (*s1).include_stack_ptr;
        (*s1).include_stack_ptr = ((*s1).include_stack_ptr).offset(1);
        *fresh33 = (*file).prev;
        (*file).include_next_index = i;
        if (*s1).gen_deps != 0 {
            let mut bf: *mut BufferedFile = file;
            while i == 1 as libc::c_int
                && {
                    bf = (*bf).prev;
                    !bf.is_null()
                }
            {
                i = (*bf).include_next_index;
            }
            if (*s1).include_sys_deps as libc::c_int != 0
                || (i - 2 as libc::c_int) < (*s1).nb_include_paths
            {
                dynarray_add(
                    &mut (*s1).target_deps as *mut *mut *mut libc::c_char
                        as *mut libc::c_void,
                    &mut (*s1).nb_target_deps,
                    tcc_strdup(buf.as_mut_ptr()) as *mut libc::c_void,
                );
            }
        }
        tcc_debug_bincl(s1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn expr_preprocess(mut s1: *mut TCCState) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut t0: libc::c_int = tok;
    let mut str: *mut TokenString = 0 as *mut TokenString;
    str = tok_str_alloc();
    pp_expr = 1 as libc::c_int;
    loop {
        next();
        t = tok;
        if tok < 256 as libc::c_int {
            if tok == 10 as libc::c_int || tok == -(1 as libc::c_int) {
                break;
            }
            if tok >= 0xc8 as libc::c_int && tok <= 0xcc as libc::c_int {
                _tcc_error(
                    b"invalid constant in preprocessor expression\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            if tok == TOK_DEFINED as libc::c_int {
                parse_flags &= !(0x1 as libc::c_int);
                next();
                t = tok;
                if t == '(' as i32 {
                    next();
                }
                parse_flags |= 0x1 as libc::c_int;
                if tok < 256 as libc::c_int {
                    expect(
                        b"identifier after 'defined'\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if (*s1).run_test != 0 {
                    maybe_run_test(s1);
                }
                c = 0 as libc::c_int;
                if !(define_find(tok)).is_null()
                    || tok == TOK___HAS_INCLUDE as libc::c_int
                    || tok == TOK___HAS_INCLUDE_NEXT as libc::c_int
                {
                    c = 1 as libc::c_int;
                }
                if t == '(' as i32 {
                    next();
                    if tok != ')' as i32 {
                        expect(b"')'\0" as *const u8 as *const libc::c_char);
                    }
                }
            } else if tok == TOK___HAS_INCLUDE as libc::c_int
                || tok == TOK___HAS_INCLUDE_NEXT as libc::c_int
            {
                t = tok;
                next();
                if tok != '(' as i32 {
                    expect(b"'('\0" as *const u8 as *const libc::c_char);
                }
                c = parse_include(
                    s1,
                    t - TOK___HAS_INCLUDE as libc::c_int,
                    1 as libc::c_int,
                );
                if tok != ')' as i32 {
                    expect(b"')'\0" as *const u8 as *const libc::c_char);
                }
            } else {
                c = 0 as libc::c_int;
            }
            tok = 0xc4 as libc::c_int;
            tokc.i = c as uint64_t;
        }
        tok_str_add_tok(str);
    }
    if 0 as libc::c_int == (*str).len {
        _tcc_error(
            b"#%s with no expression\0" as *const u8 as *const libc::c_char,
            get_tok_str(t0, 0 as *mut CValue),
        );
    }
    tok_str_add(str, -(1 as libc::c_int));
    pp_expr = t0;
    t = tok;
    begin_macro(str, 1 as libc::c_int);
    next();
    c = expr_const();
    if tok != -(1 as libc::c_int) {
        _tcc_error(b"...\0" as *const u8 as *const libc::c_char);
    }
    pp_expr = 0 as libc::c_int;
    end_macro();
    tok = t;
    return (c != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pp_error(mut cs: *mut CString) {
    cstr_printf(
        cs,
        b"bad preprocessor expression: #%s\0" as *const u8 as *const libc::c_char,
        get_tok_str(pp_expr, 0 as *mut CValue),
    );
    macro_ptr = (*macro_stack).str_0;
    loop {
        next();
        if !(tok != -(1 as libc::c_int)) {
            break;
        }
        cstr_printf(
            cs,
            b" %s\0" as *const u8 as *const libc::c_char,
            get_tok_str(tok, &mut tokc),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_define() {
    let mut current_block: u64;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut first: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut v: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut varg: libc::c_int = 0;
    let mut is_vaargs: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut saved_parse_flags: libc::c_int = parse_flags;
    let mut str: TokenString = TokenString {
        str_0: 0 as *const libc::c_int as *mut libc::c_int,
        len: 0,
        need_spc: 0,
        allocated_len: 0,
        last_line_num: 0,
        save_line_num: 0,
        prev: 0 as *const TokenString as *mut TokenString,
        prev_ptr: 0 as *const libc::c_int,
        alloc: 0,
    };
    v = tok;
    if v < 256 as libc::c_int || v == TOK_DEFINED as libc::c_int {
        _tcc_error(
            b"invalid macro name '%s'\0" as *const u8 as *const libc::c_char,
            get_tok_str(tok, &mut tokc),
        );
    }
    first = 0 as *mut Sym;
    t = 0 as libc::c_int;
    parse_flags = parse_flags & !(0x8 as libc::c_int) | 0x10 as libc::c_int;
    next_nomacro();
    parse_flags &= !(0x10 as libc::c_int);
    is_vaargs = 0 as libc::c_int;
    if tok == '(' as i32 {
        let mut dotid: libc::c_int = set_idnum('.' as i32, 0 as libc::c_int);
        next_nomacro();
        ps = &mut first;
        if tok != ')' as i32 {
            loop {
                varg = tok;
                next_nomacro();
                is_vaargs = 0 as libc::c_int;
                if varg == 0xa1 as libc::c_int {
                    varg = TOK___VA_ARGS__ as libc::c_int;
                    is_vaargs = 1 as libc::c_int;
                } else if tok == 0xa1 as libc::c_int
                    && (*tcc_state).gnu_ext as libc::c_int != 0
                {
                    is_vaargs = 1 as libc::c_int;
                    next_nomacro();
                }
                if !(varg < 256 as libc::c_int) {
                    s = sym_push2(
                        &mut define_stack,
                        varg | 0x20000000 as libc::c_int,
                        is_vaargs,
                        0 as libc::c_int,
                    );
                    *ps = s;
                    ps = &mut (*s).c2rust_unnamed_0.next;
                    if tok == ')' as i32 {
                        break;
                    }
                    if !(tok != ',' as i32 || is_vaargs != 0) {
                        next_nomacro();
                        continue;
                    }
                }
                _tcc_error(
                    b"bad macro parameter list\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        parse_flags |= 0x10 as libc::c_int;
        next_nomacro();
        t = 1 as libc::c_int;
        set_idnum('.' as i32, dotid);
    }
    parse_flags |= 0x20 as libc::c_int | 0x10 as libc::c_int | 0x4 as libc::c_int;
    tok_str_new(&mut str);
    t0 = 0 as libc::c_int;
    loop {
        if !(tok != 10 as libc::c_int && tok != -(1 as libc::c_int)) {
            current_block = 18435049525520518667;
            break;
        }
        if is_space(tok) != 0 {
            str.need_spc |= 1 as libc::c_int;
        } else {
            if 0xa3 as libc::c_int == tok {
                if 0 as libc::c_int == t0 {
                    current_block = 9338682612737968616;
                    break;
                }
                tok = 0xa3 as libc::c_int | 0x20000000 as libc::c_int;
                t |= 2 as libc::c_int;
            }
            tok_str_add2_spc(&mut str, tok, &mut tokc);
            t0 = tok;
        }
        next_nomacro();
    }
    match current_block {
        18435049525520518667 => {
            parse_flags = saved_parse_flags;
            tok_str_add(&mut str, 0 as libc::c_int);
            if !(t0 == 0xa3 as libc::c_int | 0x20000000 as libc::c_int) {
                define_push(v, t, str.str_0, first);
                return;
            }
        }
        _ => {}
    }
    _tcc_error(
        b"'##' cannot appear at either end of macro\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn search_cached_include(
    mut s1: *mut TCCState,
    mut filename: *const libc::c_char,
    mut add: libc::c_int,
) -> *mut CachedInclude {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut basename: *const libc::c_char = 0 as *const libc::c_char;
    let mut h: libc::c_uint = 0;
    let mut e: *mut CachedInclude = 0 as *mut CachedInclude;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    basename = tcc_basename(filename);
    s = basename;
    h = 1 as libc::c_int as libc::c_uint;
    loop {
        c = *s as libc::c_uchar as libc::c_int;
        if !(c != 0 as libc::c_int) {
            break;
        }
        h = h
            .wrapping_add(h << 5 as libc::c_int)
            .wrapping_add(h >> 27 as libc::c_int)
            .wrapping_add(c as libc::c_uint);
        s = s.offset(1);
        s;
    }
    h &= (32 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    i = (*s1).cached_includes_hash[h as usize];
    while !(i == 0 as libc::c_int) {
        e = *((*s1).cached_includes).offset((i - 1 as libc::c_int) as isize);
        if 0 as libc::c_int == strcmp(filename, ((*e).filename).as_mut_ptr()) {
            return e;
        }
        if (*e).once != 0
            && 0 as libc::c_int
                == strcmp(basename, tcc_basename(((*e).filename).as_mut_ptr()))
            && 0 as libc::c_int
                == normalized_PATHCMP(filename, ((*e).filename).as_mut_ptr())
        {
            return e;
        }
        i = (*e).hash_next;
    }
    if add == 0 {
        return 0 as *mut CachedInclude;
    }
    len = strlen(filename) as libc::c_int;
    e = tcc_malloc(
        (::core::mem::size_of::<CachedInclude>() as libc::c_ulong)
            .wrapping_add(len as libc::c_ulong),
    ) as *mut CachedInclude;
    memcpy(
        ((*e).filename).as_mut_ptr() as *mut libc::c_void,
        filename as *const libc::c_void,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    (*e).once = 0 as libc::c_int;
    (*e).ifndef_macro = (*e).once;
    dynarray_add(
        &mut (*s1).cached_includes as *mut *mut *mut CachedInclude as *mut libc::c_void,
        &mut (*s1).nb_cached_includes,
        e as *mut libc::c_void,
    );
    (*e).hash_next = (*s1).cached_includes_hash[h as usize];
    (*s1).cached_includes_hash[h as usize] = (*s1).nb_cached_includes;
    return e;
}
unsafe extern "C" fn pragma_parse(mut s1: *mut TCCState) -> libc::c_int {
    let mut current_block: u64;
    next_nomacro();
    if tok == TOK_push_macro as libc::c_int || tok == TOK_pop_macro as libc::c_int {
        let mut t: libc::c_int = tok;
        let mut v: libc::c_int = 0;
        let mut s: *mut Sym = 0 as *mut Sym;
        next();
        if tok != '(' as i32 {
            current_block = 6604937279878952055;
        } else {
            next();
            if tok != 0xc8 as libc::c_int {
                current_block = 6604937279878952055;
            } else {
                v = (*tok_alloc(tokc.str_0.data, tokc.str_0.size - 1 as libc::c_int))
                    .tok;
                next();
                if tok != ')' as i32 {
                    current_block = 6604937279878952055;
                } else {
                    if t == TOK_push_macro as libc::c_int {
                        loop {
                            s = define_find(v);
                            if !s.is_null() {
                                break;
                            }
                            define_push(
                                v,
                                0 as libc::c_int,
                                0 as *mut libc::c_int,
                                0 as *mut Sym,
                            );
                        }
                        (*s).type_0.ref_0 = s;
                    } else {
                        s = define_stack;
                        while !s.is_null() {
                            if (*s).v == v && (*s).type_0.ref_0 == s {
                                (*s).type_0.ref_0 = 0 as *mut Sym;
                                break;
                            } else {
                                s = (*s).prev;
                            }
                        }
                    }
                    if !s.is_null() {
                        let ref mut fresh34 = (**table_ident
                            .offset((v - 256 as libc::c_int) as isize))
                            .sym_define;
                        *fresh34 = if !((*s).c2rust_unnamed.d).is_null() {
                            s
                        } else {
                            0 as *mut Sym
                        };
                    } else {
                        _tcc_warning(
                            b"unbalanced #pragma pop_macro\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    pp_debug_tok = t;
                    pp_debug_symv = v;
                    current_block = 2706659501864706830;
                }
            }
        }
    } else if tok == TOK_once as libc::c_int {
        (*search_cached_include(s1, (*file).true_filename, 1 as libc::c_int))
            .once = 1 as libc::c_int;
        current_block = 2706659501864706830;
    } else if (*s1).output_type == 5 as libc::c_int {
        unget_tok(' ' as i32);
        unget_tok(TOK_PRAGMA as libc::c_int);
        unget_tok('#' as i32);
        unget_tok(10 as libc::c_int);
        return 1 as libc::c_int;
    } else if tok == TOK_pack as libc::c_int {
        next();
        skip('(' as i32);
        if tok == TOK_ASM_pop as libc::c_int {
            next();
            if (*s1).pack_stack_ptr <= ((*s1).pack_stack).as_mut_ptr() {
                current_block = 3815156440208211824;
            } else {
                (*s1).pack_stack_ptr = ((*s1).pack_stack_ptr).offset(-1);
                (*s1).pack_stack_ptr;
                current_block = 10758786907990354186;
            }
        } else {
            let mut val: libc::c_int = 0 as libc::c_int;
            if tok != ')' as i32 {
                if tok == TOK_ASM_push as libc::c_int {
                    next();
                    if (*s1).pack_stack_ptr
                        >= ((*s1).pack_stack)
                            .as_mut_ptr()
                            .offset(8 as libc::c_int as isize)
                            .offset(-(1 as libc::c_int as isize))
                    {
                        current_block = 3815156440208211824;
                    } else {
                        let fresh35 = (*s1).pack_stack_ptr;
                        (*s1).pack_stack_ptr = ((*s1).pack_stack_ptr).offset(1);
                        val = *fresh35;
                        if tok != ',' as i32 {
                            current_block = 5442915941387837436;
                        } else {
                            next();
                            current_block = 8180496224585318153;
                        }
                    }
                } else {
                    current_block = 8180496224585318153;
                }
                match current_block {
                    3815156440208211824 => {}
                    5442915941387837436 => {}
                    _ => {
                        if tok != 0xc2 as libc::c_int {
                            current_block = 6604937279878952055;
                        } else {
                            val = tokc.i as libc::c_int;
                            if val < 1 as libc::c_int || val > 16 as libc::c_int
                                || val & val - 1 as libc::c_int != 0 as libc::c_int
                            {
                                current_block = 6604937279878952055;
                            } else {
                                next();
                                current_block = 5442915941387837436;
                            }
                        }
                    }
                }
            } else {
                current_block = 5442915941387837436;
            }
            match current_block {
                6604937279878952055 => {}
                3815156440208211824 => {}
                _ => {
                    *(*s1).pack_stack_ptr = val;
                    current_block = 10758786907990354186;
                }
            }
        }
        match current_block {
            6604937279878952055 => {}
            _ => {
                match current_block {
                    3815156440208211824 => {
                        _tcc_error(
                            b"out of pack stack\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    _ => {
                        if tok != ')' as i32 {
                            current_block = 6604937279878952055;
                        } else {
                            current_block = 2706659501864706830;
                        }
                    }
                }
            }
        }
    } else if tok == TOK_comment as libc::c_int {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut t_0: libc::c_int = 0;
        next();
        skip('(' as i32);
        t_0 = tok;
        next();
        skip(',' as i32);
        if tok != 0xc8 as libc::c_int {
            current_block = 6604937279878952055;
        } else {
            p = tcc_strdup(tokc.str_0.data);
            next();
            if tok != ')' as i32 {
                current_block = 6604937279878952055;
            } else {
                if t_0 == TOK_lib as libc::c_int {
                    dynarray_add(
                        &mut (*s1).pragma_libs as *mut *mut *mut libc::c_char
                            as *mut libc::c_void,
                        &mut (*s1).nb_pragma_libs,
                        p as *mut libc::c_void,
                    );
                } else {
                    if t_0 == TOK_option as libc::c_int {
                        tcc_set_options(s1, p);
                    }
                    tcc_free(p as *mut libc::c_void);
                }
                current_block = 2706659501864706830;
            }
        }
    } else {
        (*tcc_state)
            .warn_num = (&mut (*(0 as *mut TCCState)).warn_all as *mut libc::c_uchar
            as size_t)
            .wrapping_sub(
                &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar as size_t,
            ) as libc::c_uchar;
        (Some(_tcc_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
            .expect(
                "non-null function pointer",
            )(
            b"#pragma %s ignored\0" as *const u8 as *const libc::c_char,
            get_tok_str(tok, &mut tokc),
        );
        return 0 as libc::c_int;
    }
    match current_block {
        6604937279878952055 => {
            _tcc_error(
                b"malformed #pragma directive\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            next();
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tccpp_putfile(mut filename: *const libc::c_char) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if !(*filename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        pstrcpy(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            (*file).true_filename,
        );
        *tcc_basename(buf.as_mut_ptr()) = 0 as libc::c_int as libc::c_char;
    }
    pstrcat(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        filename,
    );
    if 0 as libc::c_int == strcmp(((*file).filename).as_mut_ptr(), buf.as_mut_ptr()) {
        return;
    }
    if (*file).true_filename == ((*file).filename).as_mut_ptr() {
        (*file).true_filename = tcc_strdup(((*file).filename).as_mut_ptr());
    }
    pstrcpy(
        ((*file).filename).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        buf.as_mut_ptr(),
    );
    tcc_debug_newfile(tcc_state);
}
#[no_mangle]
pub unsafe extern "C" fn preprocess(mut is_bof: libc::c_int) {
    let mut current_block: u64;
    let mut s1: *mut TCCState = tcc_state;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut saved_parse_flags: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut Sym = 0 as *mut Sym;
    saved_parse_flags = parse_flags;
    parse_flags = 0x1 as libc::c_int | 0x2 as libc::c_int | 0x40 as libc::c_int
        | 0x4 as libc::c_int | parse_flags & 0x8 as libc::c_int;
    next_nomacro();
    loop {
        match tok {
            320 => {
                pp_debug_tok = tok;
                next_nomacro();
                pp_debug_symv = tok;
                parse_define();
                current_block = 3634396408142324656;
                break;
            }
            328 => {
                pp_debug_tok = tok;
                next_nomacro();
                pp_debug_symv = tok;
                s = define_find(tok);
                if !s.is_null() {
                    define_undef(s);
                }
                next_nomacro();
                current_block = 3634396408142324656;
                break;
            }
            321 | 322 => {
                parse_include(s1, tok - TOK_INCLUDE as libc::c_int, 0 as libc::c_int);
                current_block = 17342842477860173938;
                break;
            }
            324 => {
                c = 1 as libc::c_int;
                current_block = 11058167443633678639;
            }
            256 => {
                c = expr_preprocess(s1);
                current_block = 14923464045027961639;
            }
            323 => {
                c = 0 as libc::c_int;
                current_block = 11058167443633678639;
            }
            257 => {
                next_nomacro();
                if (*s1).ifdef_stack_ptr == ((*s1).ifdef_stack).as_mut_ptr() {
                    _tcc_error(
                        b"#else without matching #if\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if *((*s1).ifdef_stack_ptr).offset(-(1 as libc::c_int) as isize)
                    & 2 as libc::c_int != 0
                {
                    _tcc_error(
                        b"#else after #else\0" as *const u8 as *const libc::c_char,
                    );
                }
                let ref mut fresh37 = *((*s1).ifdef_stack_ptr)
                    .offset(-(1 as libc::c_int) as isize);
                *fresh37 ^= 3 as libc::c_int;
                c = *fresh37;
                current_block = 3772673460508844926;
            }
            325 => {
                if (*s1).ifdef_stack_ptr == ((*s1).ifdef_stack).as_mut_ptr() {
                    _tcc_error(
                        b"#elif without matching #if\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                c = *((*s1).ifdef_stack_ptr).offset(-(1 as libc::c_int) as isize);
                if c > 1 as libc::c_int {
                    _tcc_error(
                        b"#elif after #else\0" as *const u8 as *const libc::c_char,
                    );
                }
                if c == 1 as libc::c_int {
                    skip_to_eol(0 as libc::c_int);
                    c = 0 as libc::c_int;
                } else {
                    c = expr_preprocess(s1);
                    *((*s1).ifdef_stack_ptr).offset(-(1 as libc::c_int) as isize) = c;
                }
                current_block = 3772673460508844926;
            }
            326 => {
                next_nomacro();
                if (*s1).ifdef_stack_ptr <= (*file).ifdef_stack_ptr {
                    _tcc_error(
                        b"#endif without matching #if\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*s1).ifdef_stack_ptr = ((*s1).ifdef_stack_ptr).offset(-1);
                (*s1).ifdef_stack_ptr;
                if (*file).ifndef_macro != 0
                    && (*s1).ifdef_stack_ptr == (*file).ifdef_stack_ptr
                {
                    (*file).ifndef_macro_saved = (*file).ifndef_macro;
                    (*file).ifndef_macro = 0 as libc::c_int;
                    tok_flags |= 0x4 as libc::c_int;
                }
                current_block = 3634396408142324656;
                break;
            }
            331 => {
                parse_flags &= !(0x2 as libc::c_int);
                next();
                if tok != 0xcd as libc::c_int {
                    current_block = 2652821275789495331;
                    break;
                } else {
                    current_block = 11441799814184323368;
                    break;
                }
            }
            205 => {
                if parse_flags & 0x8 as libc::c_int != 0 {
                    current_block = 13522418323212889755;
                    break;
                } else {
                    current_block = 3392087639489470149;
                    break;
                }
            }
            329 | 330 => {
                q = buf.as_mut_ptr();
                c = skip_spaces();
                while c != '\n' as i32 && c != -(1 as libc::c_int) {
                    if (q.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong)
                        < (::core::mem::size_of::<[libc::c_char; 1024]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        let fresh38 = q;
                        q = q.offset(1);
                        *fresh38 = c as libc::c_char;
                    }
                    c = handle_stray_noerror(0 as libc::c_int);
                }
                *q = '\0' as i32 as libc::c_char;
                if tok == TOK_ERROR as libc::c_int {
                    _tcc_error(
                        b"#error %s\0" as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                    );
                } else {
                    _tcc_warning(
                        b"#warning %s\0" as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                    );
                }
                next_nomacro();
                current_block = 3634396408142324656;
                break;
            }
            332 => {
                if pragma_parse(s1) == 0 {
                    current_block = 13522418323212889755;
                    break;
                } else {
                    current_block = 3634396408142324656;
                    break;
                }
            }
            10 => {
                current_block = 17342842477860173938;
                break;
            }
            _ => {
                if saved_parse_flags & 0x8 as libc::c_int != 0 {
                    current_block = 13522418323212889755;
                    break;
                } else {
                    current_block = 11869735117417356968;
                    break;
                }
            }
        }
        match current_block {
            11058167443633678639 => {
                next_nomacro();
                if tok < 256 as libc::c_int {
                    _tcc_error(
                        b"invalid argument for '#if%sdef'\0" as *const u8
                            as *const libc::c_char,
                        if c != 0 {
                            b"n\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                if is_bof != 0 {
                    if c != 0 {
                        (*file).ifndef_macro = tok;
                    }
                }
                if !(define_find(tok)).is_null()
                    || tok == TOK___HAS_INCLUDE as libc::c_int
                    || tok == TOK___HAS_INCLUDE_NEXT as libc::c_int
                {
                    c ^= 1 as libc::c_int;
                }
                next_nomacro();
                current_block = 14923464045027961639;
            }
            3772673460508844926 => {
                if (*s1).ifdef_stack_ptr
                    == ((*file).ifdef_stack_ptr).offset(1 as libc::c_int as isize)
                {
                    (*file).ifndef_macro = 0 as libc::c_int;
                }
                current_block = 4058447955611644674;
            }
            _ => {}
        }
        match current_block {
            14923464045027961639 => {
                if (*s1).ifdef_stack_ptr
                    >= ((*s1).ifdef_stack)
                        .as_mut_ptr()
                        .offset(64 as libc::c_int as isize)
                {
                    _tcc_error(
                        b"memory full (ifdef)\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh36 = (*s1).ifdef_stack_ptr;
                (*s1).ifdef_stack_ptr = ((*s1).ifdef_stack_ptr).offset(1);
                *fresh36 = c;
            }
            _ => {}
        }
        if !(c & 1 as libc::c_int == 0) {
            current_block = 3634396408142324656;
            break;
        }
        skip_to_eol(1 as libc::c_int);
        preprocess_skip();
        is_bof = 0 as libc::c_int;
    }
    match current_block {
        11869735117417356968 => {
            if tok == '!' as i32 && is_bof != 0 {
                current_block = 13522418323212889755;
            } else {
                _tcc_warning(
                    b"Ignoring unknown preprocessing directive #%s\0" as *const u8
                        as *const libc::c_char,
                    get_tok_str(tok, &mut tokc),
                );
                current_block = 13522418323212889755;
            }
        }
        3392087639489470149 => {
            c = 0 as libc::c_int;
            current_block = 15594603006322722090;
        }
        11441799814184323368 => {
            c = 1 as libc::c_int;
            current_block = 15594603006322722090;
        }
        _ => {}
    }
    match current_block {
        15594603006322722090 => {
            n = 0 as libc::c_int;
            q = tokc.str_0.data;
            loop {
                if !(*q != 0) {
                    current_block = 2631791190359682872;
                    break;
                }
                if isnum(*q as libc::c_int) == 0 {
                    current_block = 2652821275789495331;
                    break;
                }
                n = n * 10 as libc::c_int + *q as libc::c_int - '0' as i32;
                q = q.offset(1);
                q;
            }
            match current_block {
                2652821275789495331 => {}
                _ => {
                    parse_flags &= !(0x40 as libc::c_int);
                    next();
                    if tok != 10 as libc::c_int {
                        if tok != 0xce as libc::c_int
                            || *(tokc.str_0.data).offset(0 as libc::c_int as isize)
                                as libc::c_int != '"' as i32
                        {
                            current_block = 2652821275789495331;
                        } else {
                            *(tokc.str_0.data)
                                .offset(
                                    (tokc.str_0.size - 2 as libc::c_int) as isize,
                                ) = 0 as libc::c_int as libc::c_char;
                            tccpp_putfile(
                                (tokc.str_0.data).offset(1 as libc::c_int as isize),
                            );
                            next();
                            skip_to_eol(c);
                            current_block = 2723324002591448311;
                        }
                    } else {
                        current_block = 2723324002591448311;
                    }
                    match current_block {
                        2652821275789495331 => {}
                        _ => {
                            if (*file).fd > 0 as libc::c_int {
                                (*tcc_state).total_lines += (*file).line_num - n;
                            }
                            (*file).line_num = n;
                            current_block = 3634396408142324656;
                        }
                    }
                }
            }
        }
        13522418323212889755 => {
            skip_to_eol(0 as libc::c_int);
            current_block = 17342842477860173938;
        }
        _ => {}
    }
    match current_block {
        3634396408142324656 => {
            skip_to_eol(1 as libc::c_int);
        }
        2652821275789495331 => {
            _tcc_error(b"wrong #line format\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    parse_flags = saved_parse_flags;
}
unsafe extern "C" fn parse_escape_string(
    mut outstr: *mut CString,
    mut buf: *const uint8_t,
    mut is_long: libc::c_int,
) {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    p = buf;
    loop {
        c = *p as libc::c_int;
        if c == '\0' as i32 {
            break;
        }
        if c == '\\' as i32 {
            p = p.offset(1);
            p;
            c = *p as libc::c_int;
            match c {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    current_block = 574164800201603741;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                120 => {
                    current_block = 13083926597545935697;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                117 => {
                    current_block = 5128239990476910458;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                85 => {
                    current_block = 2806078453958877543;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                97 => {
                    current_block = 5965680426769882812;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                98 => {
                    current_block = 11242695938819334496;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                102 => {
                    current_block = 16848497387872344240;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                110 => {
                    current_block = 5476904825463976137;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                114 => {
                    current_block = 10435907010544837076;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                116 => {
                    current_block = 10686558435802476299;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                118 => {
                    current_block = 808035530346077322;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                101 => {
                    current_block = 12407665068778534919;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
                39 | 34 | 92 | 63 => {
                    current_block = 10261677128829721533;
                }
                _ => {
                    current_block = 2803772711873006769;
                    match current_block {
                        574164800201603741 => {
                            n = c - '0' as i32;
                            p = p.offset(1);
                            p;
                            c = *p as libc::c_int;
                            if isoct(c) != 0 {
                                n = n * 8 as libc::c_int + c - '0' as i32;
                                p = p.offset(1);
                                p;
                                c = *p as libc::c_int;
                                if isoct(c) != 0 {
                                    n = n * 8 as libc::c_int + c - '0' as i32;
                                    p = p.offset(1);
                                    p;
                                }
                            }
                            c = n;
                            current_block = 1534102509770322473;
                        }
                        12407665068778534919 => {
                            if (*tcc_state).gnu_ext == 0 {
                                current_block = 2803772711873006769;
                            } else {
                                c = 27 as libc::c_int;
                                current_block = 10261677128829721533;
                            }
                        }
                        13083926597545935697 => {
                            i = 0 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5128239990476910458 => {
                            i = 4 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        2806078453958877543 => {
                            i = 8 as libc::c_int;
                            current_block = 3963085394198927886;
                        }
                        5965680426769882812 => {
                            c = '\u{7}' as i32;
                            current_block = 10261677128829721533;
                        }
                        11242695938819334496 => {
                            c = '\u{8}' as i32;
                            current_block = 10261677128829721533;
                        }
                        16848497387872344240 => {
                            c = '\u{c}' as i32;
                            current_block = 10261677128829721533;
                        }
                        5476904825463976137 => {
                            c = '\n' as i32;
                            current_block = 10261677128829721533;
                        }
                        10435907010544837076 => {
                            c = '\r' as i32;
                            current_block = 10261677128829721533;
                        }
                        10686558435802476299 => {
                            c = '\t' as i32;
                            current_block = 10261677128829721533;
                        }
                        808035530346077322 => {
                            c = '\u{b}' as i32;
                            current_block = 10261677128829721533;
                        }
                        _ => {}
                    }
                    match current_block {
                        1534102509770322473 => {}
                        10261677128829721533 => {}
                        _ => {
                            match current_block {
                                2803772711873006769 => {
                                    if c >= '!' as i32 && c <= '~' as i32 {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\%c'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    } else {
                                        _tcc_warning(
                                            b"unknown escape sequence: '\\x%x'\0" as *const u8
                                                as *const libc::c_char,
                                            c,
                                        );
                                    }
                                    current_block = 10261677128829721533;
                                }
                                _ => {
                                    p = p.offset(1);
                                    p;
                                    n = 0 as libc::c_int;
                                    loop {
                                        c = *p as libc::c_int;
                                        if c >= 'a' as i32 && c <= 'f' as i32 {
                                            c = c - 'a' as i32 + 10 as libc::c_int;
                                        } else if c >= 'A' as i32 && c <= 'F' as i32 {
                                            c = c - 'A' as i32 + 10 as libc::c_int;
                                        } else if isnum(c) != 0 {
                                            c = c - '0' as i32;
                                        } else {
                                            if !(i >= 0 as libc::c_int) {
                                                current_block = 9863067789333207682;
                                                break;
                                            }
                                            expect(
                                                b"more hex digits in universal-character-name\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        n = n * 16 as libc::c_int + c;
                                        p = p.offset(1);
                                        p;
                                        i -= 1;
                                        if !(i != 0) {
                                            current_block = 9520865839495247062;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        9520865839495247062 => {
                                            if !(is_long != 0) {
                                                cstr_u8cat(outstr, n);
                                                continue;
                                            }
                                        }
                                        _ => {}
                                    }
                                    c = n;
                                    current_block = 1534102509770322473;
                                }
                            }
                        }
                    }
                }
            }
        } else if is_long != 0 && c >= 0x80 as libc::c_int {
            let mut cont: libc::c_int = 0;
            let mut skip_0: libc::c_int = 0;
            let mut i_0: libc::c_int = 0;
            if c < 0xc2 as libc::c_int {
                skip_0 = 1 as libc::c_int;
                current_block = 12576784685179189593;
            } else {
                if c <= 0xdf as libc::c_int {
                    cont = 1 as libc::c_int;
                    n = c & 0x1f as libc::c_int;
                    current_block = 7858101417678297991;
                } else if c <= 0xef as libc::c_int {
                    cont = 2 as libc::c_int;
                    n = c & 0xf as libc::c_int;
                    current_block = 7858101417678297991;
                } else if c <= 0xf4 as libc::c_int {
                    cont = 3 as libc::c_int;
                    n = c & 0x7 as libc::c_int;
                    current_block = 7858101417678297991;
                } else {
                    skip_0 = 1 as libc::c_int;
                    current_block = 12576784685179189593;
                }
                match current_block {
                    12576784685179189593 => {}
                    _ => {
                        i_0 = 1 as libc::c_int;
                        loop {
                            if !(i_0 <= cont) {
                                current_block = 6838274324784804404;
                                break;
                            }
                            let mut l: libc::c_int = 0x80 as libc::c_int;
                            let mut h: libc::c_int = 0xbf as libc::c_int;
                            if i_0 == 1 as libc::c_int {
                                match c {
                                    224 => {
                                        l = 0xa0 as libc::c_int;
                                    }
                                    237 => {
                                        h = 0x9f as libc::c_int;
                                    }
                                    240 => {
                                        l = 0x90 as libc::c_int;
                                    }
                                    244 => {
                                        h = 0x8f as libc::c_int;
                                    }
                                    _ => {}
                                }
                            }
                            if (*p.offset(i_0 as isize) as libc::c_int) < l
                                || *p.offset(i_0 as isize) as libc::c_int > h
                            {
                                skip_0 = i_0;
                                current_block = 12576784685179189593;
                                break;
                            } else {
                                n = n << 6 as libc::c_int
                                    | *p.offset(i_0 as isize) as libc::c_int
                                        & 0x3f as libc::c_int;
                                i_0 += 1;
                                i_0;
                            }
                        }
                        match current_block {
                            12576784685179189593 => {}
                            _ => {
                                p = p.offset((1 as libc::c_int + cont) as isize);
                                c = n;
                                current_block = 1534102509770322473;
                            }
                        }
                    }
                }
            }
            match current_block {
                1534102509770322473 => {}
                _ => {
                    _tcc_warning(
                        b"ill-formed UTF-8 subsequence starting with: '\\x%x'\0"
                            as *const u8 as *const libc::c_char,
                        c,
                    );
                    c = 0xfffd as libc::c_int;
                    p = p.offset(skip_0 as isize);
                    current_block = 1534102509770322473;
                }
            }
        } else {
            current_block = 10261677128829721533;
        }
        match current_block {
            10261677128829721533 => {
                p = p.offset(1);
                p;
            }
            _ => {}
        }
        if is_long == 0 {
            cstr_ccat(outstr, c);
        } else {
            cstr_wccat(outstr, c);
        }
    }
    if is_long == 0 {
        cstr_ccat(outstr, '\0' as i32);
    } else {
        cstr_wccat(outstr, '\0' as i32);
    };
}
unsafe extern "C" fn parse_string(mut s: *const libc::c_char, mut len: libc::c_int) {
    let mut buf: [uint8_t; 1000] = [0; 1000];
    let mut p: *mut uint8_t = buf.as_mut_ptr();
    let mut is_long: libc::c_int = 0;
    let mut sep: libc::c_int = 0;
    is_long = (*s as libc::c_int == 'L' as i32) as libc::c_int;
    if is_long != 0 {
        s = s.offset(1);
        s;
        len -= 1;
        len;
    }
    let fresh39 = s;
    s = s.offset(1);
    sep = *fresh39 as libc::c_int;
    len -= 2 as libc::c_int;
    if len as libc::c_ulong >= ::core::mem::size_of::<[uint8_t; 1000]>() as libc::c_ulong
    {
        p = tcc_malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut uint8_t;
    }
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, len as libc::c_ulong);
    *p.offset(len as isize) = 0 as libc::c_int as uint8_t;
    cstr_reset(&mut tokcstr);
    parse_escape_string(&mut tokcstr, p, is_long);
    if p != buf.as_mut_ptr() {
        tcc_free(p as *mut libc::c_void);
    }
    if sep == '\'' as i32 {
        let mut char_size: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        if is_long == 0 {
            tok = 0xc0 as libc::c_int;
            char_size = 1 as libc::c_int;
        } else {
            tok = 0xc1 as libc::c_int;
            char_size = ::core::mem::size_of::<nwchar_t>() as libc::c_ulong
                as libc::c_int;
        }
        n = tokcstr.size / char_size - 1 as libc::c_int;
        if n < 1 as libc::c_int {
            _tcc_error(
                b"empty character constant\0" as *const u8 as *const libc::c_char,
            );
        }
        if n > 1 as libc::c_int {
            (*tcc_state)
                .warn_num = (&mut (*(0 as *mut TCCState)).warn_all as *mut libc::c_uchar
                as size_t)
                .wrapping_sub(
                    &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar
                        as size_t,
                ) as libc::c_uchar;
            (Some(_tcc_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect(
                    "non-null function pointer",
                )(
                b"multi-character character constant\0" as *const u8
                    as *const libc::c_char,
            );
        }
        i = 0 as libc::c_int;
        c = i;
        while i < n {
            if is_long != 0 {
                c = *(tokcstr.data as *mut nwchar_t).offset(i as isize);
            } else {
                c = c << 8 as libc::c_int
                    | *(tokcstr.data).offset(i as isize) as libc::c_int;
            }
            i += 1;
            i;
        }
        tokc.i = c as uint64_t;
    } else {
        tokc.str_0.size = tokcstr.size;
        tokc.str_0.data = tokcstr.data;
        if is_long == 0 {
            tok = 0xc8 as libc::c_int;
        } else {
            tok = 0xc9 as libc::c_int;
        }
    };
}
unsafe extern "C" fn bn_lshift(
    mut bn: *mut libc::c_uint,
    mut shift: libc::c_int,
    mut or_val: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_uint = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        v = *bn.offset(i as isize);
        *bn.offset(i as isize) = v << shift | or_val as libc::c_uint;
        or_val = (v >> 32 as libc::c_int - shift) as libc::c_int;
        i += 1;
        i;
    }
}
unsafe extern "C" fn bn_zero(mut bn: *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        *bn.offset(i as isize) = 0 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
}
unsafe extern "C" fn parse_number(mut p: *const libc::c_char) {
    let mut current_block: u64;
    let mut b: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut frac_bits: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut exp_val: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bn: [libc::c_uint; 2] = [0; 2];
    let mut d: libc::c_double = 0.;
    q = token_buf.as_mut_ptr();
    let fresh40 = p;
    p = p.offset(1);
    ch = *fresh40 as libc::c_int;
    t = ch;
    let fresh41 = p;
    p = p.offset(1);
    ch = *fresh41 as libc::c_int;
    let fresh42 = q;
    q = q.offset(1);
    *fresh42 = t as libc::c_char;
    b = 10 as libc::c_int;
    if t == '.' as i32 {
        current_block = 16791665189521845338;
    } else {
        if t == '0' as i32 {
            if ch == 'x' as i32 || ch == 'X' as i32 {
                q = q.offset(-1);
                q;
                let fresh43 = p;
                p = p.offset(1);
                ch = *fresh43 as libc::c_int;
                b = 16 as libc::c_int;
            } else if (*tcc_state).tcc_ext as libc::c_int != 0
                && (ch == 'b' as i32 || ch == 'B' as i32)
            {
                q = q.offset(-1);
                q;
                let fresh44 = p;
                p = p.offset(1);
                ch = *fresh44 as libc::c_int;
                b = 2 as libc::c_int;
            }
        }
        loop {
            if ch >= 'a' as i32 && ch <= 'f' as i32 {
                t = ch - 'a' as i32 + 10 as libc::c_int;
            } else if ch >= 'A' as i32 && ch <= 'F' as i32 {
                t = ch - 'A' as i32 + 10 as libc::c_int;
            } else {
                if !(isnum(ch) != 0) {
                    current_block = 4068382217303356765;
                    break;
                }
                t = ch - '0' as i32;
            }
            if t >= b {
                current_block = 4068382217303356765;
                break;
            }
            if q >= token_buf.as_mut_ptr().offset(1024 as libc::c_int as isize) {
                current_block = 6073504754624948367;
                break;
            }
            let fresh45 = q;
            q = q.offset(1);
            *fresh45 = ch as libc::c_char;
            let fresh46 = p;
            p = p.offset(1);
            ch = *fresh46 as libc::c_int;
        }
        match current_block {
            6073504754624948367 => {}
            _ => {
                if ch == '.' as i32
                    || (ch == 'e' as i32 || ch == 'E' as i32) && b == 10 as libc::c_int
                    || (ch == 'p' as i32 || ch == 'P' as i32)
                        && (b == 16 as libc::c_int || b == 2 as libc::c_int)
                {
                    if b != 10 as libc::c_int {
                        *q = '\0' as i32 as libc::c_char;
                        if b == 16 as libc::c_int {
                            shift = 4 as libc::c_int;
                        } else {
                            shift = 1 as libc::c_int;
                        }
                        bn_zero(bn.as_mut_ptr());
                        q = token_buf.as_mut_ptr();
                        loop {
                            let fresh47 = q;
                            q = q.offset(1);
                            t = *fresh47 as libc::c_int;
                            if t == '\0' as i32 {
                                break;
                            }
                            if t >= 'a' as i32 {
                                t = t - 'a' as i32 + 10 as libc::c_int;
                            } else if t >= 'A' as i32 {
                                t = t - 'A' as i32 + 10 as libc::c_int;
                            } else {
                                t = t - '0' as i32;
                            }
                            bn_lshift(bn.as_mut_ptr(), shift, t);
                        }
                        frac_bits = 0 as libc::c_int;
                        if ch == '.' as i32 {
                            let fresh48 = p;
                            p = p.offset(1);
                            ch = *fresh48 as libc::c_int;
                            loop {
                                t = ch;
                                if t >= 'a' as i32 && t <= 'f' as i32 {
                                    t = t - 'a' as i32 + 10 as libc::c_int;
                                } else if t >= 'A' as i32 && t <= 'F' as i32 {
                                    t = t - 'A' as i32 + 10 as libc::c_int;
                                } else {
                                    if !(t >= '0' as i32 && t <= '9' as i32) {
                                        break;
                                    }
                                    t = t - '0' as i32;
                                }
                                if t >= b {
                                    _tcc_error(
                                        b"invalid digit\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                bn_lshift(bn.as_mut_ptr(), shift, t);
                                frac_bits += shift;
                                let fresh49 = p;
                                p = p.offset(1);
                                ch = *fresh49 as libc::c_int;
                            }
                        }
                        if ch != 'p' as i32 && ch != 'P' as i32 {
                            expect(b"exponent\0" as *const u8 as *const libc::c_char);
                        }
                        let fresh50 = p;
                        p = p.offset(1);
                        ch = *fresh50 as libc::c_int;
                        s = 1 as libc::c_int;
                        exp_val = 0 as libc::c_int;
                        if ch == '+' as i32 {
                            let fresh51 = p;
                            p = p.offset(1);
                            ch = *fresh51 as libc::c_int;
                        } else if ch == '-' as i32 {
                            s = -(1 as libc::c_int);
                            let fresh52 = p;
                            p = p.offset(1);
                            ch = *fresh52 as libc::c_int;
                        }
                        if ch < '0' as i32 || ch > '9' as i32 {
                            expect(
                                b"exponent digits\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        while ch >= '0' as i32 && ch <= '9' as i32 {
                            exp_val = exp_val * 10 as libc::c_int + ch - '0' as i32;
                            let fresh53 = p;
                            p = p.offset(1);
                            ch = *fresh53 as libc::c_int;
                        }
                        exp_val = exp_val * s;
                        d = bn[1 as libc::c_int as usize] as libc::c_double
                            * 4294967296.0f64
                            + bn[0 as libc::c_int as usize] as libc::c_double;
                        d = ldexp(d, exp_val - frac_bits);
                        t = toup(ch);
                        if t == 'F' as i32 {
                            let fresh54 = p;
                            p = p.offset(1);
                            ch = *fresh54 as libc::c_int;
                            tok = 0xca as libc::c_int;
                            tokc.f = d as libc::c_float;
                        } else if t == 'L' as i32 {
                            let fresh55 = p;
                            p = p.offset(1);
                            ch = *fresh55 as libc::c_int;
                            tok = 0xcc as libc::c_int;
                            tokc.ld = f128::f128::new(d);
                        } else {
                            tok = 0xcb as libc::c_int;
                            tokc.d = d;
                        }
                        current_block = 15908231092227701503;
                    } else if ch == '.' as i32 {
                        if q
                            >= token_buf
                                .as_mut_ptr()
                                .offset(1024 as libc::c_int as isize)
                        {
                            current_block = 6073504754624948367;
                        } else {
                            let fresh56 = q;
                            q = q.offset(1);
                            *fresh56 = ch as libc::c_char;
                            let fresh57 = p;
                            p = p.offset(1);
                            ch = *fresh57 as libc::c_int;
                            current_block = 16791665189521845338;
                        }
                    } else {
                        current_block = 5023038348526654800;
                    }
                } else {
                    let mut n: libc::c_ulonglong = 0;
                    let mut n1: libc::c_ulonglong = 0;
                    let mut lcount: libc::c_int = 0;
                    let mut ucount: libc::c_int = 0;
                    let mut ov: libc::c_int = 0 as libc::c_int;
                    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
                    *q = '\0' as i32 as libc::c_char;
                    q = token_buf.as_mut_ptr();
                    if b == 10 as libc::c_int && *q as libc::c_int == '0' as i32 {
                        b = 8 as libc::c_int;
                        q = q.offset(1);
                        q;
                    }
                    n = 0 as libc::c_int as libc::c_ulonglong;
                    loop {
                        let fresh68 = q;
                        q = q.offset(1);
                        t = *fresh68 as libc::c_int;
                        if t == '\0' as i32 {
                            break;
                        }
                        if t >= 'a' as i32 {
                            t = t - 'a' as i32 + 10 as libc::c_int;
                        } else if t >= 'A' as i32 {
                            t = t - 'A' as i32 + 10 as libc::c_int;
                        } else {
                            t = t - '0' as i32;
                        }
                        if t >= b {
                            _tcc_error(
                                b"invalid digit\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        n1 = n;
                        n = n
                            .wrapping_mul(b as libc::c_ulonglong)
                            .wrapping_add(t as libc::c_ulonglong);
                        if n1 >= 0x1000000000000000 as libc::c_ulonglong
                            && n.wrapping_div(b as libc::c_ulonglong) != n1
                        {
                            ov = 1 as libc::c_int;
                        }
                    }
                    ucount = 0 as libc::c_int;
                    lcount = ucount;
                    p1 = p;
                    loop {
                        t = toup(ch);
                        if t == 'L' as i32 {
                            if lcount >= 2 as libc::c_int {
                                _tcc_error(
                                    b"three 'l's in integer constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if lcount != 0
                                && *p.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                    != ch
                            {
                                _tcc_error(
                                    b"incorrect integer suffix: %s\0" as *const u8
                                        as *const libc::c_char,
                                    p1,
                                );
                            }
                            lcount += 1;
                            lcount;
                            let fresh69 = p;
                            p = p.offset(1);
                            ch = *fresh69 as libc::c_int;
                        } else {
                            if !(t == 'U' as i32) {
                                break;
                            }
                            if ucount >= 1 as libc::c_int {
                                _tcc_error(
                                    b"two 'u's in integer constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            ucount += 1;
                            ucount;
                            let fresh70 = p;
                            p = p.offset(1);
                            ch = *fresh70 as libc::c_int;
                        }
                    }
                    if pp_expr != 0 {
                        lcount = 2 as libc::c_int;
                    }
                    if ucount == 0 as libc::c_int && b == 10 as libc::c_int {
                        if lcount
                            <= (8 as libc::c_int == 4 as libc::c_int) as libc::c_int
                        {
                            if n >= 0x80000000 as libc::c_uint as libc::c_ulonglong {
                                lcount = (8 as libc::c_int == 4 as libc::c_int)
                                    as libc::c_int + 1 as libc::c_int;
                            }
                        }
                        if n >= 0x8000000000000000 as libc::c_ulonglong {
                            ov = 1 as libc::c_int;
                            ucount = 1 as libc::c_int;
                        }
                    } else {
                        if lcount
                            <= (8 as libc::c_int == 4 as libc::c_int) as libc::c_int
                        {
                            if n >= 0x100000000 as libc::c_ulonglong {
                                lcount = (8 as libc::c_int == 4 as libc::c_int)
                                    as libc::c_int + 1 as libc::c_int;
                            } else if n
                                >= 0x80000000 as libc::c_uint as libc::c_ulonglong
                            {
                                ucount = 1 as libc::c_int;
                            }
                        }
                        if n >= 0x8000000000000000 as libc::c_ulonglong {
                            ucount = 1 as libc::c_int;
                        }
                    }
                    if ov != 0 {
                        _tcc_warning(
                            b"integer constant overflow\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    tok = 0xc2 as libc::c_int;
                    if lcount != 0 {
                        tok = 0xc6 as libc::c_int;
                        if lcount == 2 as libc::c_int {
                            tok = 0xc4 as libc::c_int;
                        }
                    }
                    if ucount != 0 {
                        tok += 1;
                        tok;
                    }
                    tokc.i = n as uint64_t;
                    current_block = 15908231092227701503;
                }
            }
        }
    }
    's_446: loop {
        match current_block {
            6073504754624948367 => {
                _tcc_error(b"number too long\0" as *const u8 as *const libc::c_char);
            }
            16791665189521845338 => {
                if !(ch >= '0' as i32 && ch <= '9' as i32) {
                    current_block = 5023038348526654800;
                    continue;
                }
                if q >= token_buf.as_mut_ptr().offset(1024 as libc::c_int as isize) {
                    current_block = 6073504754624948367;
                    continue;
                }
                let fresh58 = q;
                q = q.offset(1);
                *fresh58 = ch as libc::c_char;
                let fresh59 = p;
                p = p.offset(1);
                ch = *fresh59 as libc::c_int;
                current_block = 16791665189521845338;
            }
            15908231092227701503 => {
                if ch != 0 {
                    _tcc_error(b"invalid number\0" as *const u8 as *const libc::c_char);
                }
                break;
            }
            _ => {
                if ch == 'e' as i32 || ch == 'E' as i32 {
                    if q >= token_buf.as_mut_ptr().offset(1024 as libc::c_int as isize) {
                        current_block = 6073504754624948367;
                        continue;
                    }
                    let fresh60 = q;
                    q = q.offset(1);
                    *fresh60 = ch as libc::c_char;
                    let fresh61 = p;
                    p = p.offset(1);
                    ch = *fresh61 as libc::c_int;
                    if ch == '-' as i32 || ch == '+' as i32 {
                        if q
                            >= token_buf
                                .as_mut_ptr()
                                .offset(1024 as libc::c_int as isize)
                        {
                            current_block = 6073504754624948367;
                            continue;
                        }
                        let fresh62 = q;
                        q = q.offset(1);
                        *fresh62 = ch as libc::c_char;
                        let fresh63 = p;
                        p = p.offset(1);
                        ch = *fresh63 as libc::c_int;
                    }
                    if ch < '0' as i32 || ch > '9' as i32 {
                        expect(b"exponent digits\0" as *const u8 as *const libc::c_char);
                    }
                    while ch >= '0' as i32 && ch <= '9' as i32 {
                        if q
                            >= token_buf
                                .as_mut_ptr()
                                .offset(1024 as libc::c_int as isize)
                        {
                            current_block = 6073504754624948367;
                            continue 's_446;
                        }
                        let fresh64 = q;
                        q = q.offset(1);
                        *fresh64 = ch as libc::c_char;
                        let fresh65 = p;
                        p = p.offset(1);
                        ch = *fresh65 as libc::c_int;
                    }
                }
                *q = '\0' as i32 as libc::c_char;
                t = toup(ch);
                *__errno_location() = 0 as libc::c_int;
                if t == 'F' as i32 {
                    let fresh66 = p;
                    p = p.offset(1);
                    ch = *fresh66 as libc::c_int;
                    tok = 0xca as libc::c_int;
                    tokc.f = strtof(token_buf.as_mut_ptr(), 0 as *mut *mut libc::c_char);
                } else if t == 'L' as i32 {
                    let fresh67 = p;
                    p = p.offset(1);
                    ch = *fresh67 as libc::c_int;
                    tok = 0xcc as libc::c_int;
                    tokc
                        .ld = strtold(
                        token_buf.as_mut_ptr(),
                        0 as *mut *mut libc::c_char,
                    );
                } else {
                    tok = 0xcb as libc::c_int;
                    tokc.d = strtod(token_buf.as_mut_ptr(), 0 as *mut *mut libc::c_char);
                }
                current_block = 15908231092227701503;
            }
        }
    };
}
unsafe extern "C" fn next_nomacro() {
    let mut current_block: u64;
    let mut t: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut is_long: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut p1: *mut uint8_t = 0 as *mut uint8_t;
    let mut h: libc::c_uint = 0;
    p = (*file).buf_ptr;
    loop {
        c = *p as libc::c_int;
        match c {
            32 | 9 => {
                tok = c;
                p = p.offset(1);
                p;
                current_block = 18172576809536311102;
            }
            12 | 11 | 13 => {
                p = p.offset(1);
                p;
                continue;
            }
            92 => {
                c = handle_stray(&mut p);
                if c == '\\' as i32 {
                    current_block = 12867251523869061125;
                    break;
                }
                if !(c == -(1 as libc::c_int)) {
                    continue;
                }
                let mut s1: *mut TCCState = tcc_state;
                if tok_flags & 0x1 as libc::c_int == 0 {
                    current_block = 17630117312637865696;
                } else {
                    if parse_flags & 0x1 as libc::c_int == 0 {
                        tok = -(1 as libc::c_int);
                        current_block = 16832022279197466268;
                        break;
                    } else if (*s1).ifdef_stack_ptr != (*file).ifdef_stack_ptr {
                        _tcc_error(
                            b"missing #endif\0" as *const u8 as *const libc::c_char,
                        );
                    } else if (*s1).include_stack_ptr
                        == ((*s1).include_stack).as_mut_ptr()
                    {
                        tok = -(1 as libc::c_int);
                        current_block = 16832022279197466268;
                        break;
                    } else {
                        if tok_flags & 0x4 as libc::c_int != 0 {
                            (*search_cached_include(
                                s1,
                                (*file).true_filename,
                                1 as libc::c_int,
                            ))
                                .ifndef_macro = (*file).ifndef_macro_saved;
                            tok_flags &= !(0x4 as libc::c_int);
                        }
                        tcc_debug_eincl(tcc_state);
                        tcc_close();
                        (*s1).include_stack_ptr = ((*s1).include_stack_ptr).offset(-1);
                        (*s1).include_stack_ptr;
                        p = (*file).buf_ptr;
                    }
                    current_block = 17630117312637865696;
                }
            }
            10 => {
                (*file).line_num += 1;
                (*file).line_num;
                p = p.offset(1);
                p;
                current_block = 17630117312637865696;
            }
            35 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if tok_flags & 0x1 as libc::c_int != 0
                    && parse_flags & 0x1 as libc::c_int != 0
                {
                    tok_flags &= !(0x1 as libc::c_int);
                    (*file).buf_ptr = p;
                    preprocess(tok_flags & 0x2 as libc::c_int);
                    p = (*file).buf_ptr;
                } else if c == '#' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xa3 as libc::c_int;
                    current_block = 16832022279197466268;
                    break;
                } else if parse_flags & 0x8 as libc::c_int != 0 {
                    p = parse_line_comment(p.offset(-(1 as libc::c_int as isize)));
                    continue;
                } else {
                    tok = '#' as i32;
                    current_block = 16832022279197466268;
                    break;
                }
                current_block = 17630117312637865696;
            }
            36 => {
                if isidnum_table[('$' as i32 - -(1 as libc::c_int)) as usize]
                    as libc::c_int & 2 as libc::c_int == 0
                    || parse_flags & 0x8 as libc::c_int != 0
                {
                    current_block = 12867251523869061125;
                    break;
                } else {
                    current_block = 15071186488592647331;
                    break;
                }
            }
            97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109
            | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122
            | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 77 | 78 | 79 | 80
            | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 95 => {
                current_block = 15071186488592647331;
                break;
            }
            76 => {
                t = *p.offset(1 as libc::c_int as isize) as libc::c_int;
                if t != '\\' as i32 && t != '\'' as i32 && t != '"' as i32 {
                    current_block = 6074735043880891984;
                    break;
                } else {
                    current_block = 7739940392431776979;
                    break;
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                t = c;
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                current_block = 13234170741614611563;
                break;
            }
            46 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if isnum(c) != 0 {
                    current_block = 15650704408606443395;
                    break;
                } else {
                    current_block = 18221534353613080499;
                    break;
                }
            }
            39 | 34 => {
                is_long = 0 as libc::c_int;
                current_block = 13817234856877598261;
                break;
            }
            60 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x9e as libc::c_int;
                } else if c == '<' as i32 {
                    p = p.offset(1);
                    c = *p as libc::c_int;
                    if c == '\\' as i32 {
                        c = handle_stray(&mut p);
                    }
                    if c == '=' as i32 {
                        p = p.offset(1);
                        p;
                        tok = 0xb8 as libc::c_int;
                    } else {
                        tok = '<' as i32;
                    }
                } else {
                    tok = 0x9c as libc::c_int;
                }
                current_block = 16832022279197466268;
                break;
            }
            62 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x9d as libc::c_int;
                } else if c == '>' as i32 {
                    p = p.offset(1);
                    c = *p as libc::c_int;
                    if c == '\\' as i32 {
                        c = handle_stray(&mut p);
                    }
                    if c == '=' as i32 {
                        p = p.offset(1);
                        p;
                        tok = 0xb9 as libc::c_int;
                    } else {
                        tok = '>' as i32;
                    }
                } else {
                    tok = 0x9f as libc::c_int;
                }
                current_block = 16832022279197466268;
                break;
            }
            38 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '&' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x90 as libc::c_int;
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb5 as libc::c_int;
                } else {
                    tok = '&' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            124 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '|' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x91 as libc::c_int;
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb6 as libc::c_int;
                } else {
                    tok = '|' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            43 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '+' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x82 as libc::c_int;
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb0 as libc::c_int;
                } else {
                    tok = '+' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            45 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '-' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x80 as libc::c_int;
                } else if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb1 as libc::c_int;
                } else if c == '>' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xa0 as libc::c_int;
                } else {
                    tok = '-' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            33 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x95 as libc::c_int;
                } else {
                    tok = '!' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            61 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0x94 as libc::c_int;
                } else {
                    tok = '=' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            42 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb2 as libc::c_int;
                } else {
                    tok = '*' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            37 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb4 as libc::c_int;
                } else {
                    tok = '%' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            94 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '=' as i32 {
                    p = p.offset(1);
                    p;
                    tok = 0xb7 as libc::c_int;
                } else {
                    tok = '^' as i32;
                }
                current_block = 16832022279197466268;
                break;
            }
            47 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '*' as i32 {
                    p = parse_comment(p);
                    tok = ' ' as i32;
                } else if c == '/' as i32 {
                    p = parse_line_comment(p);
                    tok = ' ' as i32;
                } else {
                    if c == '=' as i32 {
                        p = p.offset(1);
                        p;
                        tok = 0xb3 as libc::c_int;
                    } else {
                        tok = '/' as i32;
                    }
                    current_block = 16832022279197466268;
                    break;
                }
                current_block = 18172576809536311102;
            }
            64 | 40 | 41 | 91 | 93 | 123 | 125 | 44 | 59 | 58 | 63 | 126 => {
                current_block = 12867251523869061125;
                break;
            }
            _ => {
                if c >= 0x80 as libc::c_int && c <= 0xff as libc::c_int {
                    current_block = 15071186488592647331;
                    break;
                } else {
                    current_block = 11271090240167667812;
                    break;
                }
            }
        }
        match current_block {
            18172576809536311102 => {
                if parse_flags & 0x10 as libc::c_int != 0 {
                    current_block = 8244477832637401375;
                    break;
                }
                while isidnum_table[(*p as libc::c_int - -(1 as libc::c_int)) as usize]
                    as libc::c_int & 1 as libc::c_int != 0
                {
                    p = p.offset(1);
                    p;
                }
            }
            _ => {
                tok_flags |= 0x1 as libc::c_int;
                if 0 as libc::c_int == parse_flags & 0x4 as libc::c_int {
                    continue;
                }
                tok = 10 as libc::c_int;
                current_block = 8244477832637401375;
                break;
            }
        }
    }
    match current_block {
        18221534353613080499 => {
            if isidnum_table[('.' as i32 - -(1 as libc::c_int)) as usize] as libc::c_int
                & 2 as libc::c_int != 0
                && isidnum_table[(c - -(1 as libc::c_int)) as usize] as libc::c_int
                    & (2 as libc::c_int | 4 as libc::c_int) != 0
            {
                c = '.' as i32;
                p = p.offset(-1);
                *p = c as uint8_t;
                current_block = 15071186488592647331;
            } else {
                if c == '.' as i32 {
                    p = p.offset(1);
                    c = *p as libc::c_int;
                    if c == '\\' as i32 {
                        c = handle_stray(&mut p);
                    }
                    if c == '.' as i32 {
                        p = p.offset(1);
                        p;
                        tok = 0xa1 as libc::c_int;
                    } else {
                        p = p.offset(-1);
                        *p = '.' as i32 as uint8_t;
                        tok = '.' as i32;
                    }
                } else {
                    tok = '.' as i32;
                }
                current_block = 16832022279197466268;
            }
        }
        7739940392431776979 => {
            p = p.offset(1);
            c = *p as libc::c_int;
            if c == '\\' as i32 {
                c = handle_stray(&mut p);
            }
            if c == '\'' as i32 || c == '"' as i32 {
                is_long = 1 as libc::c_int;
                current_block = 13817234856877598261;
            } else {
                cstr_reset(&mut tokcstr);
                cstr_ccat(&mut tokcstr, 'L' as i32);
                current_block = 17878060963804375525;
            }
        }
        11271090240167667812 => {
            if parse_flags & 0x8 as libc::c_int != 0 {
                current_block = 12867251523869061125;
            } else {
                _tcc_error(
                    b"unrecognized character \\x%02x\0" as *const u8
                        as *const libc::c_char,
                    c,
                );
            }
        }
        15650704408606443395 => {
            t = '.' as i32;
            current_block = 13234170741614611563;
        }
        6074735043880891984 => {
            current_block = 15071186488592647331;
        }
        _ => {}
    }
    match current_block {
        13817234856877598261 => {
            cstr_reset(&mut tokcstr);
            if is_long != 0 {
                cstr_ccat(&mut tokcstr, 'L' as i32);
            }
            cstr_ccat(&mut tokcstr, c);
            p = parse_pp_string(p, c, &mut tokcstr);
            cstr_ccat(&mut tokcstr, c);
            cstr_ccat(&mut tokcstr, '\0' as i32);
            tokc.str_0.size = tokcstr.size;
            tokc.str_0.data = tokcstr.data;
            tok = 0xce as libc::c_int;
            current_block = 16832022279197466268;
        }
        13234170741614611563 => {
            cstr_reset(&mut tokcstr);
            loop {
                cstr_ccat(&mut tokcstr, t);
                if !(isidnum_table[(c - -(1 as libc::c_int)) as usize] as libc::c_int
                    & (2 as libc::c_int | 4 as libc::c_int) != 0 || c == '.' as i32
                    || (c == '+' as i32 || c == '-' as i32)
                        && ((t == 'e' as i32 || t == 'E' as i32)
                            && !(parse_flags & 0x8 as libc::c_int != 0
                                && *(tokcstr.data).offset(0 as libc::c_int as isize)
                                    as libc::c_int == '0' as i32
                                && toup(
                                    *(tokcstr.data).offset(1 as libc::c_int as isize)
                                        as libc::c_int,
                                ) == 'X' as i32) || t == 'p' as i32 || t == 'P' as i32))
                {
                    break;
                }
                t = c;
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
            }
            cstr_ccat(&mut tokcstr, '\0' as i32);
            tokc.str_0.size = tokcstr.size;
            tokc.str_0.data = tokcstr.data;
            tok = 0xcd as libc::c_int;
            current_block = 16832022279197466268;
        }
        12867251523869061125 => {
            tok = c;
            p = p.offset(1);
            p;
            current_block = 16832022279197466268;
        }
        15071186488592647331 => {
            p1 = p;
            h = 1 as libc::c_int as libc::c_uint;
            h = h
                .wrapping_add(h << 5 as libc::c_int)
                .wrapping_add(h >> 27 as libc::c_int)
                .wrapping_add(c as libc::c_uint);
            loop {
                p = p.offset(1);
                c = *p as libc::c_int;
                if !(isidnum_table[(c - -(1 as libc::c_int)) as usize] as libc::c_int
                    & (2 as libc::c_int | 4 as libc::c_int) != 0)
                {
                    break;
                }
                h = h
                    .wrapping_add(h << 5 as libc::c_int)
                    .wrapping_add(h >> 27 as libc::c_int)
                    .wrapping_add(c as libc::c_uint);
            }
            len = p.offset_from(p1) as libc::c_long as libc::c_int;
            if c != '\\' as i32 {
                let mut current_block_52: u64;
                let mut pts: *mut *mut TokenSym = 0 as *mut *mut TokenSym;
                h &= (16384 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
                pts = &mut *hash_ident.as_mut_ptr().offset(h as isize)
                    as *mut *mut TokenSym;
                loop {
                    ts = *pts;
                    if ts.is_null() {
                        current_block_52 = 14329534724295951598;
                        break;
                    }
                    if (*ts).len == len
                        && memcmp(
                            ((*ts).str_0).as_mut_ptr() as *const libc::c_void,
                            p1 as *const libc::c_void,
                            len as libc::c_ulong,
                        ) == 0
                    {
                        current_block_52 = 8102658916883067714;
                        break;
                    }
                    pts = &mut (*ts).hash_next;
                }
                match current_block_52 {
                    14329534724295951598 => {
                        ts = tok_alloc_new(pts, p1 as *mut libc::c_char, len);
                    }
                    _ => {}
                }
                current_block = 15622658527355336244;
            } else {
                cstr_reset(&mut tokcstr);
                cstr_cat(&mut tokcstr, p1 as *mut libc::c_char, len);
                p = p.offset(-1);
                p;
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                current_block = 17878060963804375525;
            }
        }
        _ => {}
    }
    match current_block {
        17878060963804375525 => {
            while isidnum_table[(c - -(1 as libc::c_int)) as usize] as libc::c_int
                & (2 as libc::c_int | 4 as libc::c_int) != 0
            {
                cstr_ccat(&mut tokcstr, c);
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
            }
            ts = tok_alloc(tokcstr.data, tokcstr.size);
            current_block = 15622658527355336244;
        }
        _ => {}
    }
    match current_block {
        15622658527355336244 => {
            tok = (*ts).tok;
            current_block = 16832022279197466268;
        }
        _ => {}
    }
    match current_block {
        16832022279197466268 => {
            tok_flags = 0 as libc::c_int;
        }
        _ => {}
    }
    (*file).buf_ptr = p;
}
unsafe extern "C" fn macro_arg_subst(
    mut nested_list: *mut *mut Sym,
    mut macro_str: *const libc::c_int,
    mut args: *mut Sym,
) -> *mut libc::c_int {
    let mut t: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut st: *const libc::c_int = 0 as *const libc::c_int;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    let mut str: TokenString = TokenString {
        str_0: 0 as *const libc::c_int as *mut libc::c_int,
        len: 0,
        need_spc: 0,
        allocated_len: 0,
        last_line_num: 0,
        save_line_num: 0,
        prev: 0 as *const TokenString as *mut TokenString,
        prev_ptr: 0 as *const libc::c_int,
        alloc: 0,
    };
    tok_str_new(&mut str);
    t1 = 0 as libc::c_int;
    t0 = t1;
    loop {
        let mut _t: libc::c_int = *macro_str;
        if _t >= 0xc0 as libc::c_int && _t <= 0xcf as libc::c_int {
            tok_get(&mut t, &mut macro_str, &mut cval);
        } else {
            t = _t;
            macro_str = macro_str.offset(1);
            macro_str;
        }
        if t == 0 {
            break;
        }
        if t == '#' as i32 {
            loop {
                let fresh71 = macro_str;
                macro_str = macro_str.offset(1);
                t = *fresh71;
                if !(t == ' ' as i32) {
                    break;
                }
            }
            s = sym_find2(args, t);
            if !s.is_null() {
                cstr_reset(&mut tokcstr);
                cstr_ccat(&mut tokcstr, '"' as i32);
                st = (*s).c2rust_unnamed.d;
                while *st != -(1 as libc::c_int) {
                    let mut s_0: *const libc::c_char = 0 as *const libc::c_char;
                    let mut _t_0: libc::c_int = *st;
                    if _t_0 >= 0xc0 as libc::c_int && _t_0 <= 0xcf as libc::c_int {
                        tok_get(&mut t, &mut st, &mut cval);
                    } else {
                        t = _t_0;
                        st = st.offset(1);
                        st;
                    }
                    s_0 = get_tok_str(t, &mut cval);
                    while *s_0 != 0 {
                        if t == 0xce as libc::c_int && *s_0 as libc::c_int != '\'' as i32
                        {
                            add_char(&mut tokcstr, *s_0 as libc::c_int);
                        } else {
                            cstr_ccat(&mut tokcstr, *s_0 as libc::c_int);
                        }
                        s_0 = s_0.offset(1);
                        s_0;
                    }
                }
                cstr_ccat(&mut tokcstr, '"' as i32);
                cstr_ccat(&mut tokcstr, '\0' as i32);
                cval.str_0.size = tokcstr.size;
                cval.str_0.data = tokcstr.data;
                tok_str_add2(&mut str, 0xce as libc::c_int, &mut cval);
            } else {
                expect(
                    b"macro parameter after '#'\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if t >= 256 as libc::c_int {
            s = sym_find2(args, t);
            if !s.is_null() {
                st = (*s).c2rust_unnamed.d;
                n = 0 as libc::c_int;
                loop {
                    t2 = *macro_str.offset(n as isize);
                    if !(t2 == ' ' as i32) {
                        break;
                    }
                    n += 1;
                    n;
                }
                let mut current_block_57: u64;
                if t2 == 0xa3 as libc::c_int | 0x20000000 as libc::c_int
                    || t1 == 0xa3 as libc::c_int | 0x20000000 as libc::c_int
                {
                    if t1 == 0xa3 as libc::c_int | 0x20000000 as libc::c_int
                        && t0 == ',' as i32 && (*tcc_state).gnu_ext as libc::c_int != 0
                        && (*s).type_0.t != 0
                    {
                        let mut c: libc::c_int = *(str.str_0)
                            .offset((str.len - 1 as libc::c_int) as isize);
                        loop {
                            str.len -= 1;
                            if !(*(str.str_0).offset(str.len as isize) != ',' as i32) {
                                break;
                            }
                        }
                        if *st == -(1 as libc::c_int) {
                            current_block_57 = 2520131295878969859;
                        } else {
                            str.len += 1;
                            str.len;
                            if c == ' ' as i32 {
                                let fresh72 = str.len;
                                str.len = str.len + 1;
                                *(str.str_0).offset(fresh72 as isize) = c;
                            }
                            current_block_57 = 13740330959843590951;
                        }
                    } else {
                        if *st == -(1 as libc::c_int) {
                            tok_str_add(&mut str, 0xa4 as libc::c_int);
                        }
                        current_block_57 = 2520131295878969859;
                    }
                } else {
                    current_block_57 = 13740330959843590951;
                }
                match current_block_57 {
                    13740330959843590951 => {
                        if ((*s).c2rust_unnamed_0.e).is_null() {
                            let mut str2: TokenString = TokenString {
                                str_0: 0 as *const libc::c_int as *mut libc::c_int,
                                len: 0,
                                need_spc: 0,
                                allocated_len: 0,
                                last_line_num: 0,
                                save_line_num: 0,
                                prev: 0 as *const TokenString as *mut TokenString,
                                prev_ptr: 0 as *const libc::c_int,
                                alloc: 0,
                            };
                            tok_str_new(&mut str2);
                            macro_subst(&mut str2, nested_list, st);
                            tok_str_add(&mut str2, -(1 as libc::c_int));
                            (*s).c2rust_unnamed_0.e = str2.str_0;
                        }
                        st = (*s).c2rust_unnamed_0.e;
                    }
                    _ => {}
                }
                while *st != -(1 as libc::c_int) {
                    let mut _t_1: libc::c_int = *st;
                    if _t_1 >= 0xc0 as libc::c_int && _t_1 <= 0xcf as libc::c_int {
                        tok_get(&mut t2, &mut st, &mut cval);
                    } else {
                        t2 = _t_1;
                        st = st.offset(1);
                        st;
                    }
                    tok_str_add2(&mut str, t2, &mut cval);
                }
            } else {
                tok_str_add(&mut str, t);
            }
        } else {
            tok_str_add2(&mut str, t, &mut cval);
        }
        if t != ' ' as i32 {
            t0 = t1;
            t1 = t;
        }
    }
    tok_str_add(&mut str, 0 as libc::c_int);
    return str.str_0;
}
#[inline]
unsafe extern "C" fn macro_twosharps(mut ptr0: *const libc::c_int) -> *mut libc::c_int {
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cv1: CValue = CValue { ld: f128::f128::ZERO };
    let mut cv2: CValue = CValue { ld: f128::f128::ZERO };
    let mut macro_str1: TokenString = TokenString {
        str_0: 0 as *const libc::c_int as *mut libc::c_int,
        len: 0,
        need_spc: 0,
        allocated_len: 0,
        last_line_num: 0,
        save_line_num: 0,
        prev: 0 as *const TokenString as *mut TokenString,
        prev_ptr: 0 as *const libc::c_int,
        alloc: 0,
    };
    let mut ptr: *const libc::c_int = 0 as *const libc::c_int;
    tok_str_new(&mut macro_str1);
    cstr_reset(&mut tokcstr);
    ptr = ptr0;
    loop {
        let mut _t: libc::c_int = *ptr;
        if _t >= 0xc0 as libc::c_int && _t <= 0xcf as libc::c_int {
            tok_get(&mut t1, &mut ptr, &mut cv1);
        } else {
            t1 = _t;
            ptr = ptr.offset(1);
            ptr;
        }
        if t1 == 0 as libc::c_int {
            break;
        }
        loop {
            n = 0 as libc::c_int;
            loop {
                t2 = *ptr.offset(n as isize);
                if !(t2 == ' ' as i32) {
                    break;
                }
                n += 1;
                n;
            }
            if t2 != 0xa3 as libc::c_int | 0x20000000 as libc::c_int {
                break;
            }
            ptr = ptr.offset(n as isize);
            loop {
                ptr = ptr.offset(1);
                t2 = *ptr;
                if !(t2 == ' ' as i32
                    || t2 == 0xa3 as libc::c_int | 0x20000000 as libc::c_int)
                {
                    break;
                }
            }
            let mut _t_0: libc::c_int = *ptr;
            if _t_0 >= 0xc0 as libc::c_int && _t_0 <= 0xcf as libc::c_int {
                tok_get(&mut t2, &mut ptr, &mut cv2);
            } else {
                t2 = _t_0;
                ptr = ptr.offset(1);
                ptr;
            }
            if t2 == 0xa4 as libc::c_int {
                continue;
            }
            if t1 != 0xa4 as libc::c_int {
                cstr_cat(&mut tokcstr, get_tok_str(t1, &mut cv1), -(1 as libc::c_int));
                t1 = 0xa4 as libc::c_int;
            }
            cstr_cat(&mut tokcstr, get_tok_str(t2, &mut cv2), -(1 as libc::c_int));
        }
        if tokcstr.size != 0 {
            cstr_ccat(&mut tokcstr, 0 as libc::c_int);
            tcc_open_bf(
                tcc_state,
                b":paste:\0" as *const u8 as *const libc::c_char,
                tokcstr.size,
            );
            memcpy(
                ((*file).buffer).as_mut_ptr() as *mut libc::c_void,
                tokcstr.data as *const libc::c_void,
                tokcstr.size as libc::c_ulong,
            );
            tok_flags = 0 as libc::c_int;
            n = 0 as libc::c_int;
            loop {
                next_nomacro();
                tok_str_add2(&mut macro_str1, tok, &mut tokc);
                if *(*file).buf_ptr as libc::c_int == 0 as libc::c_int {
                    break;
                }
                tok_str_add(&mut macro_str1, ' ' as i32);
                l = ((*file).buf_ptr).offset_from(((*file).buffer).as_mut_ptr())
                    as libc::c_long as libc::c_int;
                _tcc_warning(
                    b"pasting \"%.*s\" and \"%s\" does not give a valid preprocessing token\0"
                        as *const u8 as *const libc::c_char,
                    l - n,
                    ((*file).buffer).as_mut_ptr().offset(n as isize),
                    (*file).buf_ptr,
                );
                n = l;
            }
            tcc_close();
            cstr_reset(&mut tokcstr);
        }
        if t1 != 0xa4 as libc::c_int {
            tok_str_add2(&mut macro_str1, t1, &mut cv1);
        }
    }
    tok_str_add(&mut macro_str1, 0 as libc::c_int);
    return macro_str1.str_0;
}
unsafe extern "C" fn peek_file(mut ws_str: *mut TokenString) -> libc::c_int {
    let mut p: *mut uint8_t = ((*file).buf_ptr).offset(-(1 as libc::c_int as isize));
    let mut c: libc::c_int = 0;
    let mut current_block_18: u64;
    loop {
        p = p.offset(1);
        c = *p as libc::c_int;
        if c == '\\' as i32 {
            c = handle_stray(&mut p);
        }
        match c {
            47 => {
                p = p.offset(1);
                c = *p as libc::c_int;
                if c == '\\' as i32 {
                    c = handle_stray(&mut p);
                }
                if c == '*' as i32 {
                    p = parse_comment(p);
                    current_block_18 = 12349973810996921269;
                } else if c == '/' as i32 {
                    p = parse_line_comment(p);
                    current_block_18 = 12349973810996921269;
                } else {
                    p = p.offset(-1);
                    *p = '/' as i32 as uint8_t;
                    c = *p as libc::c_int;
                    current_block_18 = 11845742232721160520;
                }
                match current_block_18 {
                    11845742232721160520 => {}
                    _ => {
                        p = p.offset(-1);
                        p;
                        c = ' ' as i32;
                        current_block_18 = 224731115979188411;
                    }
                }
            }
            32 | 9 => {
                current_block_18 = 224731115979188411;
            }
            12 | 11 | 13 => {
                continue;
            }
            10 => {
                (*file).line_num += 1;
                (*file).line_num;
                tok_flags |= 0x1 as libc::c_int;
                current_block_18 = 224731115979188411;
            }
            _ => {
                current_block_18 = 11845742232721160520;
            }
        }
        match current_block_18 {
            224731115979188411 => {
                if !ws_str.is_null() {
                    tok_str_add(ws_str, c);
                }
            }
            _ => {
                (*file).buf_ptr = p;
                return c;
            }
        }
    };
}
unsafe extern "C" fn next_argstream(
    mut nested_list: *mut *mut Sym,
    mut ws_str: *mut TokenString,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut sa: *mut Sym = 0 as *mut Sym;
    while !macro_ptr.is_null() {
        let mut m: *const libc::c_int = macro_ptr;
        loop {
            t = *m;
            if !(t != 0 as libc::c_int) {
                break;
            }
            if !ws_str.is_null() {
                if t != ' ' as i32 {
                    return t;
                }
                m = m.offset(1);
                m;
            } else {
                let mut _t: libc::c_int = *macro_ptr;
                if _t >= 0xc0 as libc::c_int && _t <= 0xcf as libc::c_int {
                    tok_get(&mut tok, &mut macro_ptr, &mut tokc);
                } else {
                    tok = _t;
                    macro_ptr = macro_ptr.offset(1);
                    macro_ptr;
                }
                return tok;
            }
        }
        end_macro();
        sa = *nested_list;
        if !sa.is_null() {
            *nested_list = (*sa).prev;
            sym_free(sa);
        }
    }
    if !ws_str.is_null() {
        return peek_file(ws_str)
    } else {
        next_nomacro();
        if tok == '\t' as i32 || tok == 10 as libc::c_int {
            tok = ' ' as i32;
        }
        return tok;
    };
}
unsafe extern "C" fn macro_subst_tok(
    mut tok_str: *mut TokenString,
    mut nested_list: *mut *mut Sym,
    mut s: *mut Sym,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut v: libc::c_int = (*s).v;
    if !((*s).c2rust_unnamed.d).is_null() {
        let mut mstr: *mut libc::c_int = (*s).c2rust_unnamed.d;
        let mut jstr: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut sa: *mut Sym = 0 as *mut Sym;
        let mut ret: libc::c_int = 0;
        if (*s).type_0.t & 1 as libc::c_int != 0 {
            let mut saved_parse_flags: libc::c_int = parse_flags;
            let mut str: TokenString = TokenString {
                str_0: 0 as *const libc::c_int as *mut libc::c_int,
                len: 0,
                need_spc: 0,
                allocated_len: 0,
                last_line_num: 0,
                save_line_num: 0,
                prev: 0 as *const TokenString as *mut TokenString,
                prev_ptr: 0 as *const libc::c_int,
                alloc: 0,
            };
            let mut parlevel: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut sa1: *mut Sym = 0 as *mut Sym;
            let mut args: *mut Sym = 0 as *mut Sym;
            parse_flags
                |= 0x10 as libc::c_int | 0x4 as libc::c_int | 0x20 as libc::c_int;
            tok_str_new(&mut str);
            t = next_argstream(nested_list, &mut str);
            if t != '(' as i32 {
                parse_flags = saved_parse_flags;
                tok_str_add2_spc(tok_str, v, 0 as *mut CValue);
                if parse_flags & 0x10 as libc::c_int != 0 {
                    i = 0 as libc::c_int;
                    while i < str.len {
                        tok_str_add(tok_str, *(str.str_0).offset(i as isize));
                        i += 1;
                        i;
                    }
                }
                tok_str_free_str(str.str_0);
                return 0 as libc::c_int;
            } else {
                tok_str_free_str(str.str_0);
            }
            args = 0 as *mut Sym;
            sa = (*s).c2rust_unnamed_0.next;
            i = 2 as libc::c_int;
            's_100: loop {
                loop {
                    t = next_argstream(nested_list, 0 as *mut TokenString);
                    if !(t == ' ' as i32
                        || {
                            i -= 1;
                            i != 0
                        })
                    {
                        break;
                    }
                }
                if sa.is_null() {
                    if t == ')' as i32 {
                        break;
                    }
                    _tcc_error(
                        b"macro '%s' used with too many args\0" as *const u8
                            as *const libc::c_char,
                        get_tok_str(v, 0 as *mut CValue),
                    );
                } else {
                    loop {
                        tok_str_new(&mut str);
                        parlevel = 0 as libc::c_int;
                        while parlevel > 0 as libc::c_int
                            || t != ')' as i32
                                && (t != ',' as i32 || (*sa).type_0.t != 0)
                        {
                            if t == -(1 as libc::c_int) {
                                _tcc_error(
                                    b"EOF in invocation of macro '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            if t == '(' as i32 {
                                parlevel += 1;
                                parlevel;
                            }
                            if t == ')' as i32 {
                                parlevel -= 1;
                                parlevel;
                            }
                            if t == ' ' as i32 {
                                str.need_spc |= 1 as libc::c_int;
                            } else {
                                tok_str_add2_spc(&mut str, t, &mut tokc);
                            }
                            t = next_argstream(nested_list, 0 as *mut TokenString);
                        }
                        tok_str_add(&mut str, -(1 as libc::c_int));
                        sa1 = sym_push2(
                            &mut args,
                            (*sa).v & !(0x20000000 as libc::c_int),
                            (*sa).type_0.t,
                            0 as libc::c_int,
                        );
                        (*sa1).c2rust_unnamed.d = str.str_0;
                        sa = (*sa).c2rust_unnamed_0.next;
                        if t == ')' as i32 {
                            if sa.is_null() {
                                break 's_100;
                            }
                            if (*sa).type_0.t != 0
                                && (*tcc_state).gnu_ext as libc::c_int != 0
                            {
                                continue;
                            }
                            _tcc_error(
                                b"macro '%s' used with too few args\0" as *const u8
                                    as *const libc::c_char,
                                get_tok_str(v, 0 as *mut CValue),
                            );
                        } else {
                            i = 1 as libc::c_int;
                            break;
                        }
                    }
                }
            }
            mstr = macro_arg_subst(nested_list, mstr, args);
            sa = args;
            while !sa.is_null() {
                sa1 = (*sa).prev;
                tok_str_free_str((*sa).c2rust_unnamed.d);
                tok_str_free_str((*sa).c2rust_unnamed_0.e);
                sym_free(sa);
                sa = sa1;
            }
            parse_flags = saved_parse_flags;
        }
        jstr = mstr;
        if (*s).type_0.t & 2 as libc::c_int != 0 {
            jstr = macro_twosharps(mstr);
        }
        sa = sym_push2(nested_list, v, 0 as libc::c_int, 0 as libc::c_int);
        ret = macro_subst(tok_str, nested_list, jstr);
        if sa == *nested_list {
            *nested_list = (*sa).prev;
            sym_free(sa);
        }
        if jstr != mstr {
            tok_str_free_str(jstr);
        }
        if mstr != (*s).c2rust_unnamed.d {
            tok_str_free_str(mstr);
        }
        return ret;
    } else {
        let mut cval: CValue = CValue { ld: f128::f128::ZERO };
        let mut buf: [libc::c_char; 32] = [0; 32];
        let mut cstrval: *mut libc::c_char = buf.as_mut_ptr();
        let mut ti: time_t = 0;
        let mut tm: *mut tm = 0 as *mut tm;
        let mut current_block_83: u64;
        if v == TOK___LINE__ as libc::c_int || v == TOK___COUNTER__ as libc::c_int {
            t = if v == TOK___LINE__ as libc::c_int {
                (*file).line_num
            } else {
                let fresh73 = pp_counter;
                pp_counter = pp_counter + 1;
                fresh73
            };
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                t,
            );
            t = 0xcd as libc::c_int;
            current_block_83 = 13490064824677194190;
        } else {
            if v == TOK___FILE__ as libc::c_int {
                cstrval = ((*file).filename).as_mut_ptr();
                current_block_83 = 9653919203729546252;
            } else if v == TOK___DATE__ as libc::c_int
                || v == TOK___TIME__ as libc::c_int
            {
                ti = 0;
                tm = 0 as *mut tm;
                time(&mut ti);
                tm = localtime(&mut ti);
                if v == TOK___DATE__ as libc::c_int {
                    static mut ab_month_name: [[libc::c_char; 4]; 12] = unsafe {
                        [
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Jan\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Feb\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Mar\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Apr\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"May\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Jun\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Jul\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Aug\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Sep\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Oct\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Nov\0"),
                            *::core::mem::transmute::<
                                &[u8; 4],
                                &[libc::c_char; 4],
                            >(b"Dec\0"),
                        ]
                    };
                    snprintf(
                        buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                        b"%s %2d %d\0" as *const u8 as *const libc::c_char,
                        (ab_month_name[(*tm).tm_mon as usize]).as_ptr(),
                        (*tm).tm_mday,
                        (*tm).tm_year + 1900 as libc::c_int,
                    );
                } else {
                    snprintf(
                        buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                        b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
                        (*tm).tm_hour,
                        (*tm).tm_min,
                        (*tm).tm_sec,
                    );
                }
                current_block_83 = 9653919203729546252;
            } else {
                current_block_83 = 479107131381816815;
            }
            match current_block_83 {
                479107131381816815 => {}
                _ => {
                    t = 0xc8 as libc::c_int;
                    current_block_83 = 13490064824677194190;
                }
            }
        }
        match current_block_83 {
            13490064824677194190 => {
                cval
                    .str_0
                    .size = (strlen(cstrval))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                cval.str_0.data = cstrval;
                tok_str_add2_spc(tok_str, t, &mut cval);
            }
            _ => {}
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn macro_subst(
    mut tok_str: *mut TokenString,
    mut nested_list: *mut *mut Sym,
    mut macro_str: *const libc::c_int,
) -> libc::c_int {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut t: libc::c_int = 0;
    let mut nosubst: libc::c_int = 0 as libc::c_int;
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    let mut str: *mut TokenString = 0 as *mut TokenString;
    loop {
        let mut _t: libc::c_int = *macro_str;
        if _t >= 0xc0 as libc::c_int && _t <= 0xcf as libc::c_int {
            tok_get(&mut t, &mut macro_str, &mut cval);
        } else {
            t = _t;
            macro_str = macro_str.offset(1);
            macro_str;
        }
        if t == 0 as libc::c_int || t == -(1 as libc::c_int) {
            break;
        }
        if t >= 256 as libc::c_int {
            s = define_find(t);
            if !(s.is_null() || nosubst != 0) {
                if !(sym_find2(*nested_list, t)).is_null() {
                    t |= 0x20000000 as libc::c_int;
                } else {
                    str = tok_str_alloc();
                    (*str).str_0 = macro_str as *mut libc::c_int;
                    begin_macro(str, 2 as libc::c_int);
                    nosubst = macro_subst_tok(tok_str, nested_list, s);
                    if macro_stack != str {
                        break;
                    }
                    macro_str = macro_ptr;
                    end_macro();
                    continue;
                }
            }
        } else if t == ' ' as i32 {
            if parse_flags & 0x10 as libc::c_int != 0 {
                (*tok_str).need_spc |= 1 as libc::c_int;
            }
            continue;
        }
        tok_str_add2_spc(tok_str, t, &mut cval);
        if nosubst != 0 && t != '(' as i32 {
            nosubst = 0 as libc::c_int;
        }
        if t == TOK_DEFINED as libc::c_int && pp_expr != 0 {
            nosubst = 1 as libc::c_int;
        }
    }
    return nosubst;
}
#[no_mangle]
pub unsafe extern "C" fn next() {
    let mut t: libc::c_int = 0;
    's_3: loop {
        if macro_ptr.is_null() {
            next_nomacro();
            t = tok;
            if !(t >= 256 as libc::c_int && parse_flags & 0x1 as libc::c_int != 0) {
                break;
            }
            let mut s: *mut Sym = define_find(t);
            if !s.is_null() {
                let mut nested_list: *mut Sym = 0 as *mut Sym;
                macro_subst_tok(&mut tokstr_buf, &mut nested_list, s);
                tok_str_add(&mut tokstr_buf, 0 as libc::c_int);
                begin_macro(&mut tokstr_buf, 0 as libc::c_int);
            } else {
                return
            }
        }
        loop {
            t = *macro_ptr;
            if !(t >= 0xc0 as libc::c_int && t <= 0xcf as libc::c_int) {
                break;
            }
            tok_get(&mut tok, &mut macro_ptr, &mut tokc);
            if !(t == 0xcf as libc::c_int) {
                break 's_3;
            }
            (*file).line_num = tokc.i as libc::c_int;
        }
        if t == 0 as libc::c_int {
            end_macro();
        } else {
            if !(t == -(1 as libc::c_int)) {
                macro_ptr = macro_ptr.offset(1);
                macro_ptr;
                t &= !(0x20000000 as libc::c_int);
                if t == '\\' as i32 {
                    if parse_flags & 0x20 as libc::c_int == 0 {
                        _tcc_error(
                            b"stray '\\' in program\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
            tok = t;
            return;
        }
    }
    if t == 0xcd as libc::c_int {
        if parse_flags & 0x2 as libc::c_int != 0 {
            parse_number(tokc.str_0.data);
        }
    } else if t == 0xce as libc::c_int {
        if parse_flags & 0x40 as libc::c_int != 0 {
            parse_string(tokc.str_0.data, tokc.str_0.size - 1 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn unget_tok(mut last_tok: libc::c_int) {
    let mut str: *mut TokenString = &mut unget_buf;
    let mut alloc: libc::c_int = 0 as libc::c_int;
    if (*str).len != 0 {
        str = tok_str_alloc();
        alloc = 1 as libc::c_int;
    }
    if tok != -(1 as libc::c_int) {
        tok_str_add2(str, tok, &mut tokc);
    }
    tok_str_add(str, 0 as libc::c_int);
    begin_macro(str, alloc);
    tok = last_tok;
}
static mut target_os_defs: *const libc::c_char = b"__linux__\0__linux\0__unix__\0__unix\0\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn putdef(mut cs: *mut CString, mut p: *const libc::c_char) {
    cstr_printf(
        cs,
        b"#define %s%s\n\0" as *const u8 as *const libc::c_char,
        p,
        &*(b" 1\0" as *const u8 as *const libc::c_char)
            .offset(
                (!((strchr
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        libc::c_int,
                    ) -> *mut libc::c_char)(p, ' ' as i32))
                    .is_null() as libc::c_int * 2 as libc::c_int) as isize,
            ) as *const libc::c_char,
    );
}
unsafe extern "C" fn putdefs(mut cs: *mut CString, mut p: *const libc::c_char) {
    while *p != 0 {
        putdef(cs, p);
        p = (strchr(p, 0 as libc::c_int)).offset(1 as libc::c_int as isize);
    }
}
unsafe extern "C" fn tcc_predefs(
    mut s1: *mut TCCState,
    mut cs: *mut CString,
    mut is_asm: libc::c_int,
) {
    cstr_printf(
        cs,
        b"#define __TINYC__ 9%.2s\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"0.9.28rc\0"))
            .as_ptr()
            .offset(4 as libc::c_int as isize),
    );
    putdefs(cs, target_machine_defs);
    putdefs(cs, target_os_defs);
    if is_asm != 0 {
        putdef(cs, b"__ASSEMBLER__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).output_type == 5 as libc::c_int {
        putdef(cs, b"__TCC_PP__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).output_type == 1 as libc::c_int {
        putdef(cs, b"__TCC_RUN__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).do_backtrace != 0 {
        putdef(cs, b"__TCC_BACKTRACE__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).do_bounds_check != 0 {
        putdef(cs, b"__TCC_BCHECK__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).char_is_unsigned != 0 {
        putdef(cs, b"__CHAR_UNSIGNED__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).optimize as libc::c_int > 0 as libc::c_int {
        putdef(cs, b"__OPTIMIZE__\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).option_pthread != 0 {
        putdef(cs, b"_REENTRANT\0" as *const u8 as *const libc::c_char);
    }
    if (*s1).leading_underscore != 0 {
        putdef(cs, b"__leading_underscore\0" as *const u8 as *const libc::c_char);
    }
    cstr_printf(
        cs,
        b"#define __SIZEOF_POINTER__ %d\n\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int,
    );
    cstr_printf(
        cs,
        b"#define __SIZEOF_LONG__ %d\n\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int,
    );
    if is_asm == 0 {
        putdef(cs, b"__STDC__\0" as *const u8 as *const libc::c_char);
        cstr_printf(
            cs,
            b"#define __STDC_VERSION__ %dL\n\0" as *const u8 as *const libc::c_char,
            (*s1).cversion,
        );
        cstr_cat(
            cs,
            b"#define __SIZE_TYPE__ unsigned long\n#define __PTRDIFF_TYPE__ long\n#define __LP64__ 1\n#define __INT64_TYPE__ long\n#define __SIZEOF_INT__ 4\n#define __INT_MAX__ 0x7fffffff\n#define __LONG_MAX__ 0x7fffffffffffffffL\n#define __SIZEOF_LONG_LONG__ 8\n#define __LONG_LONG_MAX__ 0x7fffffffffffffffLL\n#define __CHAR_BIT__ 8\n#define __ORDER_LITTLE_ENDIAN__ 1234\n#define __ORDER_BIG_ENDIAN__ 4321\n#define __BYTE_ORDER__ __ORDER_LITTLE_ENDIAN__\n#define __WCHAR_TYPE__ int\n#define __WINT_TYPE__ unsigned int\n#if __STDC_VERSION__>=201112L\n#define __STDC_NO_ATOMICS__ 1\n#define __STDC_NO_COMPLEX__ 1\n#define __STDC_NO_THREADS__ 1\n#define __STDC_UTF_16__ 1\n#define __STDC_UTF_32__ 1\n#endif\n#define __UINTPTR_TYPE__ unsigned __PTRDIFF_TYPE__\n#define __INTPTR_TYPE__ __PTRDIFF_TYPE__\n#define __INT32_TYPE__ int\n#define __REDIRECT(name,proto,alias) name proto __asm__(#alias)\n#define __REDIRECT_NTH(name,proto,alias) name proto __asm__(#alias)__THROW\n#define __REDIRECT_NTHNL(name,proto,alias) name proto __asm__(#alias)__THROWNL\n#define __PRETTY_FUNCTION__ __FUNCTION__\n#define __has_builtin(x) 0\n#define __has_feature(x) 0\n#define __has_attribute(x) 0\n#define _Nonnull\n#define _Nullable\n#define _Nullable_result\n#define _Null_unspecified\n#ifndef __TCC_PP__\n#define __builtin_offsetof(type,field) ((__SIZE_TYPE__)&((type*)0)->field)\n#define __builtin_extract_return_addr(x) x\ntypedef struct{\nunsigned gp_offset,fp_offset;\nunion{\nunsigned overflow_offset;\nchar*overflow_arg_area;\n};\nchar*reg_save_area;\n}__builtin_va_list[1];\nvoid*__va_arg(__builtin_va_list ap,int arg_type,int size,int align);\n#define __builtin_va_start(ap,last) (*(ap)=*(__builtin_va_list)((char*)__builtin_frame_address(0)-24))\n#define __builtin_va_arg(ap,t) (*(t*)(__va_arg(ap,__builtin_va_arg_types(t),sizeof(t),__alignof__(t))))\n#define __builtin_va_copy(dest,src) (*(dest)=*(src))\n#define __builtin_va_end(ap) (void)(ap)\n#ifndef __builtin_va_copy\n#define __builtin_va_copy(dest,src) (dest)=(src)\n#endif\n#ifdef __leading_underscore\n#define __RENAME(X) __asm__(\"_\"X)\n#else\n#define __RENAME(X) __asm__(X)\n#endif\n#ifdef __TCC_BCHECK__\n#define __BUILTINBC(ret,name,params) ret __builtin_##name params __RENAME(\"__bound_\"#name);\n#define __BOUND(ret,name,params) ret name params __RENAME(\"__bound_\"#name);\n#else\n#define __BUILTINBC(ret,name,params) ret __builtin_##name params __RENAME(#name);\n#define __BOUND(ret,name,params)\n#endif\n#define __BOTH(ret,name,params) __BUILTINBC(ret,name,params)__BOUND(ret,name,params)\n#define __BUILTIN(ret,name,params) ret __builtin_##name params __RENAME(#name);\n__BOTH(void*,memcpy,(void*,const void*,__SIZE_TYPE__))\n__BOTH(void*,memmove,(void*,const void*,__SIZE_TYPE__))\n__BOTH(void*,memset,(void*,int,__SIZE_TYPE__))\n__BOTH(int,memcmp,(const void*,const void*,__SIZE_TYPE__))\n__BOTH(__SIZE_TYPE__,strlen,(const char*))\n__BOTH(char*,strcpy,(char*,const char*))\n__BOTH(char*,strncpy,(char*,const char*,__SIZE_TYPE__))\n__BOTH(int,strcmp,(const char*,const char*))\n__BOTH(int,strncmp,(const char*,const char*,__SIZE_TYPE__))\n__BOTH(char*,strcat,(char*,const char*))\n__BOTH(char*,strncat,(char*,const char*,__SIZE_TYPE__))\n__BOTH(char*,strchr,(const char*,int))\n__BOTH(char*,strrchr,(const char*,int))\n__BOTH(char*,strdup,(const char*))\n#define __MAYBE_REDIR __BUILTIN\n__MAYBE_REDIR(void*,malloc,(__SIZE_TYPE__))\n__MAYBE_REDIR(void*,realloc,(void*,__SIZE_TYPE__))\n__MAYBE_REDIR(void*,calloc,(__SIZE_TYPE__,__SIZE_TYPE__))\n__MAYBE_REDIR(void*,memalign,(__SIZE_TYPE__,__SIZE_TYPE__))\n__MAYBE_REDIR(void,free,(void*))\n__BOTH(void*,alloca,(__SIZE_TYPE__))\n__BUILTIN(void,abort,(void))\n__BOUND(void,longjmp,())\n__BOUND(void*,mmap,())\n__BOUND(int,munmap,())\n#undef __BUILTINBC\n#undef __BUILTIN\n#undef __BOUND\n#undef __BOTH\n#undef __MAYBE_REDIR\n#undef __RENAME\n#define __BUILTIN_EXTERN(name,u) int __builtin_##name(u int);int __builtin_##name##l(u long);int __builtin_##name##ll(u long long);\n__BUILTIN_EXTERN(ffs,)\n__BUILTIN_EXTERN(clz,unsigned)\n__BUILTIN_EXTERN(ctz,unsigned)\n__BUILTIN_EXTERN(clrsb,)\n__BUILTIN_EXTERN(popcount,unsigned)\n__BUILTIN_EXTERN(parity,unsigned)\n#undef __BUILTIN_EXTERN\n#endif\n\0"
                as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    cstr_printf(
        cs,
        b"#define __BASE_FILE__ \"%s\"\n\0" as *const u8 as *const libc::c_char,
        ((*file).filename).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn preprocess_start(
    mut s1: *mut TCCState,
    mut filetype: libc::c_int,
) {
    let mut is_asm: libc::c_int = (filetype & (2 as libc::c_int | 4 as libc::c_int) != 0)
        as libc::c_int;
    tccpp_new(s1);
    (*s1).include_stack_ptr = ((*s1).include_stack).as_mut_ptr();
    (*s1).ifdef_stack_ptr = ((*s1).ifdef_stack).as_mut_ptr();
    (*file).ifdef_stack_ptr = (*s1).ifdef_stack_ptr;
    pp_expr = 0 as libc::c_int;
    pp_counter = 0 as libc::c_int;
    pp_debug_symv = 0 as libc::c_int;
    pp_debug_tok = pp_debug_symv;
    (*s1).pack_stack[0 as libc::c_int as usize] = 0 as libc::c_int;
    (*s1).pack_stack_ptr = ((*s1).pack_stack).as_mut_ptr();
    set_idnum(
        '$' as i32,
        if is_asm == 0 && (*s1).dollars_in_identifiers as libc::c_int != 0 {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        },
    );
    set_idnum('.' as i32, if is_asm != 0 { 2 as libc::c_int } else { 0 as libc::c_int });
    if filetype & 2 as libc::c_int == 0 {
        let mut cstr: CString = CString {
            size: 0,
            size_allocated: 0,
            data: 0 as *const libc::c_char as *mut libc::c_char,
        };
        cstr_new(&mut cstr);
        tcc_predefs(s1, &mut cstr, is_asm);
        if (*s1).cmdline_defs.size != 0 {
            cstr_cat(&mut cstr, (*s1).cmdline_defs.data, (*s1).cmdline_defs.size);
        }
        if (*s1).cmdline_incl.size != 0 {
            cstr_cat(&mut cstr, (*s1).cmdline_incl.data, (*s1).cmdline_incl.size);
        }
        let fresh74 = (*s1).include_stack_ptr;
        (*s1).include_stack_ptr = ((*s1).include_stack_ptr).offset(1);
        *fresh74 = file;
        tcc_open_bf(
            s1,
            b"<command line>\0" as *const u8 as *const libc::c_char,
            cstr.size,
        );
        memcpy(
            ((*file).buffer).as_mut_ptr() as *mut libc::c_void,
            cstr.data as *const libc::c_void,
            cstr.size as libc::c_ulong,
        );
        cstr_free(&mut cstr);
    }
    parse_flags = if is_asm != 0 { 0x8 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn preprocess_end(mut s1: *mut TCCState) {
    while !macro_stack.is_null() {
        end_macro();
    }
    macro_ptr = 0 as *const libc::c_int;
    while !file.is_null() {
        tcc_close();
    }
    tccpp_delete(s1);
}
#[no_mangle]
pub unsafe extern "C" fn set_idnum(
    mut c: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    let mut prev: libc::c_int = isidnum_table[(c - -(1 as libc::c_int)) as usize]
        as libc::c_int;
    isidnum_table[(c - -(1 as libc::c_int)) as usize] = val as libc::c_uchar;
    return prev;
}
#[no_mangle]
pub unsafe extern "C" fn tccpp_new(mut s: *mut TCCState) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    i = -(1 as libc::c_int);
    while i < 128 as libc::c_int {
        set_idnum(
            i,
            if is_space(i) != 0 {
                1 as libc::c_int
            } else if isid(i) != 0 {
                2 as libc::c_int
            } else if isnum(i) != 0 {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
        i += 1;
        i;
    }
    i = 128 as libc::c_int;
    while i < 256 as libc::c_int {
        set_idnum(i, 2 as libc::c_int);
        i += 1;
        i;
    }
    tal_new(
        &mut toksym_alloc,
        256 as libc::c_int as libc::c_uint,
        (768 as libc::c_int * 1024 as libc::c_int) as libc::c_uint,
    );
    tal_new(
        &mut tokstr_alloc,
        1024 as libc::c_int as libc::c_uint,
        (768 as libc::c_int * 1024 as libc::c_int) as libc::c_uint,
    );
    memset(
        hash_ident.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (16384 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut TokenSym>() as libc::c_ulong),
    );
    memset(
        ((*s).cached_includes_hash).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 32]>() as libc::c_ulong,
    );
    cstr_new(&mut tokcstr);
    cstr_new(&mut cstr_buf);
    cstr_realloc(&mut cstr_buf, 1024 as libc::c_int);
    tok_str_new(&mut tokstr_buf);
    tok_str_realloc(&mut tokstr_buf, 256 as libc::c_int);
    tok_str_new(&mut unget_buf);
    tok_ident = 256 as libc::c_int;
    p = tcc_keywords.as_ptr();
    while *p != 0 {
        r = p;
        loop {
            let fresh75 = r;
            r = r.offset(1);
            c = *fresh75 as libc::c_int;
            if c == '\0' as i32 {
                break;
            }
        }
        tok_alloc(
            p,
            (r.offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_int,
        );
        p = r;
    }
    define_push(
        TOK___LINE__ as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___FILE__ as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___DATE__ as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___TIME__ as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut Sym,
    );
    define_push(
        TOK___COUNTER__ as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut Sym,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tccpp_delete(mut s: *mut TCCState) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    dynarray_reset(
        &mut (*s).cached_includes as *mut *mut *mut CachedInclude as *mut libc::c_void,
        &mut (*s).nb_cached_includes,
    );
    n = tok_ident - 256 as libc::c_int;
    if n > (*tcc_state).total_idents {
        (*tcc_state).total_idents = n;
    }
    i = 0 as libc::c_int;
    while i < n {
        tal_free_impl(
            toksym_alloc,
            *table_ident.offset(i as isize) as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    tcc_free(table_ident as *mut libc::c_void);
    table_ident = 0 as *mut *mut TokenSym;
    cstr_free(&mut tokcstr);
    cstr_free(&mut cstr_buf);
    tok_str_free_str(tokstr_buf.str_0);
    tok_str_free_str(unget_buf.str_0);
    tal_delete(toksym_alloc);
    toksym_alloc = 0 as *mut TinyAlloc;
    tal_delete(tokstr_alloc);
    tokstr_alloc = 0 as *mut TinyAlloc;
}
unsafe extern "C" fn tok_print(
    mut str: *const libc::c_int,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut fp: *mut FILE = (*tcc_state).ppfp;
    let mut ap: ::core::ffi::VaListImpl;
    let mut t: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    ap = args.clone();
    vfprintf(fp, msg, ap.as_va_list());
    t0 = 0 as libc::c_int;
    s = t0;
    while !str.is_null() {
        let mut _t: libc::c_int = *str;
        if _t >= 0xc0 as libc::c_int && _t <= 0xcf as libc::c_int {
            tok_get(&mut t, &mut str, &mut cval);
        } else {
            t = _t;
            str = str.offset(1);
            str;
        }
        if t == 0 as libc::c_int || t == -(1 as libc::c_int) {
            break;
        }
        if pp_need_space(t0, t) != 0 {
            s = 0 as libc::c_int;
        }
        fprintf(
            fp,
            &*(b" %s\0" as *const u8 as *const libc::c_char).offset(s as isize)
                as *const libc::c_char,
            if t == 0xa4 as libc::c_int {
                b"<>\0" as *const u8 as *const libc::c_char
            } else {
                get_tok_str(t, &mut cval)
            },
        );
        s = 1 as libc::c_int;
        t0 = t;
    }
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pp_line(
    mut s1: *mut TCCState,
    mut f: *mut BufferedFile,
    mut level: libc::c_int,
) {
    let mut d: libc::c_int = (*f).line_num - (*f).line_ref;
    if (*s1).dflag as libc::c_int & 4 as libc::c_int != 0 {
        return;
    }
    if !((*s1).Pflag as libc::c_int == LINE_MACRO_OUTPUT_FORMAT_NONE as libc::c_int) {
        if level == 0 as libc::c_int && (*f).line_ref != 0 && d < 8 as libc::c_int {
            while d > 0 as libc::c_int {
                fputs(b"\n\0" as *const u8 as *const libc::c_char, (*s1).ppfp);
                d -= 1;
                d;
            }
        } else if (*s1).Pflag as libc::c_int
            == LINE_MACRO_OUTPUT_FORMAT_STD as libc::c_int
        {
            fprintf(
                (*s1).ppfp,
                b"#line %d \"%s\"\n\0" as *const u8 as *const libc::c_char,
                (*f).line_num,
                ((*f).filename).as_mut_ptr(),
            );
        } else {
            fprintf(
                (*s1).ppfp,
                b"# %d \"%s\"%s\n\0" as *const u8 as *const libc::c_char,
                (*f).line_num,
                ((*f).filename).as_mut_ptr(),
                if level > 0 as libc::c_int {
                    b" 1\0" as *const u8 as *const libc::c_char
                } else if level < 0 as libc::c_int {
                    b" 2\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    (*f).line_ref = (*f).line_num;
}
unsafe extern "C" fn define_print(mut s1: *mut TCCState, mut v: libc::c_int) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut Sym = 0 as *mut Sym;
    s = define_find(v);
    if s.is_null() || ((*s).c2rust_unnamed.d).is_null() {
        return;
    }
    fp = (*s1).ppfp;
    fprintf(
        fp,
        b"#define %s\0" as *const u8 as *const libc::c_char,
        get_tok_str(v, 0 as *mut CValue),
    );
    if (*s).type_0.t & 1 as libc::c_int != 0 {
        let mut a: *mut Sym = (*s).c2rust_unnamed_0.next;
        fprintf(fp, b"(\0" as *const u8 as *const libc::c_char);
        if !a.is_null() {
            loop {
                fprintf(
                    fp,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    get_tok_str((*a).v, 0 as *mut CValue),
                );
                a = (*a).c2rust_unnamed_0.next;
                if a.is_null() {
                    break;
                }
                fprintf(fp, b",\0" as *const u8 as *const libc::c_char);
            }
        }
        fprintf(fp, b")\0" as *const u8 as *const libc::c_char);
    }
    tok_print((*s).c2rust_unnamed.d, b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pp_debug_defines(mut s1: *mut TCCState) {
    let mut v: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut vs: *const libc::c_char = 0 as *const libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    t = pp_debug_tok;
    if t == 0 as libc::c_int {
        return;
    }
    (*file).line_num -= 1;
    (*file).line_num;
    pp_line(s1, file, 0 as libc::c_int);
    (*file).line_num += 1;
    (*file).line_ref = (*file).line_num;
    fp = (*s1).ppfp;
    v = pp_debug_symv;
    vs = get_tok_str(v, 0 as *mut CValue);
    if t == TOK_DEFINE as libc::c_int {
        define_print(s1, v);
    } else if t == TOK_UNDEF as libc::c_int {
        fprintf(fp, b"#undef %s\n\0" as *const u8 as *const libc::c_char, vs);
    } else if t == TOK_push_macro as libc::c_int {
        fprintf(
            fp,
            b"#pragma push_macro(\"%s\")\n\0" as *const u8 as *const libc::c_char,
            vs,
        );
    } else if t == TOK_pop_macro as libc::c_int {
        fprintf(
            fp,
            b"#pragma pop_macro(\"%s\")\n\0" as *const u8 as *const libc::c_char,
            vs,
        );
    }
    pp_debug_tok = 0 as libc::c_int;
}
unsafe extern "C" fn pp_need_space(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return if 'E' as i32 == a {
        ('+' as i32 == b || '-' as i32 == b) as libc::c_int
    } else if '+' as i32 == a {
        (0x82 as libc::c_int == b || '+' as i32 == b) as libc::c_int
    } else if '-' as i32 == a {
        (0x80 as libc::c_int == b || '-' as i32 == b) as libc::c_int
    } else if a >= 256 as libc::c_int || a == 0xcd as libc::c_int {
        (b >= 256 as libc::c_int || b == 0xcd as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn pp_check_he0xE(
    mut t: libc::c_int,
    mut p: *const libc::c_char,
) -> libc::c_int {
    if t == 0xcd as libc::c_int
        && toup(
            *(strchr(p, 0 as libc::c_int)).offset(-(1 as libc::c_int) as isize)
                as libc::c_int,
        ) == 'E' as i32
    {
        return 'E' as i32;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_preprocess(mut s1: *mut TCCState) -> libc::c_int {
    let mut iptr: *mut *mut BufferedFile = 0 as *mut *mut BufferedFile;
    let mut token_seen: libc::c_int = 0;
    let mut spcs: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut white: [libc::c_char; 400] = [0; 400];
    parse_flags = 0x1 as libc::c_int | parse_flags & 0x8 as libc::c_int
        | 0x4 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int;
    if (*s1).Pflag as libc::c_int == LINE_MACRO_OUTPUT_FORMAT_P10 as libc::c_int {
        parse_flags |= 0x2 as libc::c_int;
        (*s1).Pflag = 1 as libc::c_int as libc::c_uchar;
    }
    if (*s1).do_bench != 0 {
        loop {
            next();
            if !(tok != -(1 as libc::c_int)) {
                break;
            }
        }
        return 0 as libc::c_int;
    }
    token_seen = 10 as libc::c_int;
    spcs = 0 as libc::c_int;
    level = 0 as libc::c_int;
    if !((*file).prev).is_null() {
        let fresh76 = level;
        level = level + 1;
        pp_line(s1, (*file).prev, fresh76);
    }
    pp_line(s1, file, level);
    loop {
        iptr = (*s1).include_stack_ptr;
        next();
        if tok == -(1 as libc::c_int) {
            break;
        }
        level = ((*s1).include_stack_ptr).offset_from(iptr) as libc::c_long
            as libc::c_int;
        if level != 0 {
            if level > 0 as libc::c_int {
                pp_line(s1, *iptr, 0 as libc::c_int);
            }
            pp_line(s1, file, level);
        }
        if (*s1).dflag as libc::c_int & 7 as libc::c_int != 0 {
            pp_debug_defines(s1);
            if (*s1).dflag as libc::c_int & 4 as libc::c_int != 0 {
                continue;
            }
        }
        if is_space(tok) != 0 {
            if (spcs as libc::c_ulong)
                < (::core::mem::size_of::<[libc::c_char; 400]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                let fresh77 = spcs;
                spcs = spcs + 1;
                white[fresh77 as usize] = tok as libc::c_char;
            }
        } else {
            if tok == 10 as libc::c_int {
                spcs = 0 as libc::c_int;
                if token_seen == 10 as libc::c_int {
                    continue;
                }
                (*file).line_ref += 1;
                (*file).line_ref;
            } else if token_seen == 10 as libc::c_int {
                pp_line(s1, file, 0 as libc::c_int);
            } else if spcs == 0 as libc::c_int && pp_need_space(token_seen, tok) != 0 {
                let fresh78 = spcs;
                spcs = spcs + 1;
                white[fresh78 as usize] = ' ' as i32 as libc::c_char;
            }
            white[spcs as usize] = 0 as libc::c_int as libc::c_char;
            fputs(white.as_mut_ptr(), (*s1).ppfp);
            spcs = 0 as libc::c_int;
            p = get_tok_str(tok, &mut tokc);
            fputs(p, (*s1).ppfp);
            token_seen = pp_check_he0xE(tok, p);
        }
    }
    return 0 as libc::c_int;
}
