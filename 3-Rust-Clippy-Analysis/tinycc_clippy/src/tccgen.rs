use ::libc;
use ::c2rust_bitfields;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    pub type _tccdbg;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut tcc_state: *mut TCCState;
    static mut stk_data: *mut *mut libc::c_void;
    static mut nb_stk_data: libc::c_int;
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
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_mallocz(size: libc::c_ulong) -> *mut libc::c_void;
    fn _tcc_error(fmt: *const libc::c_char, _: ...) -> !;
    fn _tcc_warning(fmt: *const libc::c_char, _: ...);
    fn dynarray_add(
        ptab: *mut libc::c_void,
        nb_ptr: *mut libc::c_int,
        data: *mut libc::c_void,
    );
    fn dynarray_reset(pp: *mut libc::c_void, n: *mut libc::c_int);
    fn cstr_ccat(cstr: *mut CString, ch: libc::c_int);
    fn cstr_cat(cstr: *mut CString, str: *const libc::c_char, len: libc::c_int);
    fn cstr_new(cstr: *mut CString);
    fn cstr_free(cstr: *mut CString);
    fn cstr_reset(cstr: *mut CString);
    fn tcc_open_bf(
        s1: *mut TCCState,
        filename: *const libc::c_char,
        initlen: libc::c_int,
    );
    fn tcc_close();
    static mut file: *mut BufferedFile;
    static mut tok: libc::c_int;
    static mut tokc: CValue;
    static mut macro_ptr: *const libc::c_int;
    static mut parse_flags: libc::c_int;
    static mut tokcstr: CString;
    static mut tok_ident: libc::c_int;
    static mut table_ident: *mut *mut TokenSym;
    fn tok_alloc_const(str: *const libc::c_char) -> libc::c_int;
    fn get_tok_str(v: libc::c_int, cv: *mut CValue) -> *const libc::c_char;
    fn begin_macro(str: *mut TokenString, alloc: libc::c_int);
    fn end_macro();
    fn tok_str_alloc() -> *mut TokenString;
    fn tok_str_free(s: *mut TokenString);
    fn tok_str_free_str(str: *mut libc::c_int);
    fn tok_str_add(s: *mut TokenString, t: libc::c_int);
    fn tok_str_add_tok(s: *mut TokenString);
    fn free_defines(b: *mut Sym);
    fn next();
    fn unget_tok(last_tok: libc::c_int);
    fn tccpp_putfile(filename: *const libc::c_char);
    fn skip(c: libc::c_int);
    fn expect(msg: *const libc::c_char) -> !;
    fn classify_x86_64_va_arg(ty: *mut CType) -> libc::c_int;
    static mut func_bound_add_epilog: libc::c_int;
    fn section_add(sec: *mut Section, size: Elf64_Addr, align: libc::c_int) -> size_t;
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut libc::c_void;
    fn find_section(s1: *mut TCCState, name: *const libc::c_char) -> *mut Section;
    fn put_elf_sym(
        s: *mut Section,
        value: Elf64_Addr,
        size: libc::c_ulong,
        info: libc::c_int,
        other: libc::c_int,
        shndx: libc::c_int,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn put_elf_reloca(
        symtab: *mut Section,
        s: *mut Section,
        offset: libc::c_ulong,
        type_0: libc::c_int,
        symbol: libc::c_int,
        addend: Elf64_Addr,
    );
    fn add_array(s1: *mut TCCState, sec: *const libc::c_char, c: libc::c_int);
    static reg_classes: [libc::c_int; 25];
    fn gsym_addr(t: libc::c_int, a: libc::c_int);
    fn load(r: libc::c_int, sv: *mut SValue);
    fn store(r: libc::c_int, v: *mut SValue);
    fn gfunc_sret(
        vt: *mut CType,
        variadic: libc::c_int,
        ret: *mut CType,
        align: *mut libc::c_int,
        regsize: *mut libc::c_int,
    ) -> libc::c_int;
    fn gfunc_call(nb_args: libc::c_int);
    fn gfunc_prolog(func_sym: *mut Sym);
    fn gfunc_epilog();
    fn gen_fill_nops(_: libc::c_int);
    fn gjmp(t: libc::c_int) -> libc::c_int;
    fn gjmp_addr(a: libc::c_int);
    fn gjmp_cond(op: libc::c_int, t: libc::c_int) -> libc::c_int;
    fn gjmp_append(n: libc::c_int, t: libc::c_int) -> libc::c_int;
    fn gen_opi(op: libc::c_int);
    fn gen_opf(op: libc::c_int);
    fn gen_cvt_ftoi(t: libc::c_int);
    fn gen_cvt_itof(t: libc::c_int);
    fn gen_cvt_ftof(t: libc::c_int);
    fn ggoto();
    fn o(c: libc::c_uint);
    fn gen_vla_sp_save(addr: libc::c_int);
    fn gen_vla_sp_restore(addr: libc::c_int);
    fn gen_vla_alloc(type_0: *mut CType, align: libc::c_int);
    fn gen_cvt_csti(t: libc::c_int);
    fn gen_opl(op: libc::c_int);
    fn gen_cvt_sxtw();
    fn asm_instr();
    fn asm_global_instr();
    fn asm_parse_regvar(t: libc::c_int) -> libc::c_int;
    fn tcc_debug_start(s1: *mut TCCState);
    fn tcc_debug_end(s1: *mut TCCState);
    fn tcc_debug_line(s1: *mut TCCState);
    fn tcc_add_debug_info(
        s1: *mut TCCState,
        param: libc::c_int,
        s: *mut Sym,
        e: *mut Sym,
    );
    fn tcc_debug_funcstart(s1: *mut TCCState, sym: *mut Sym);
    fn tcc_debug_prolog_epilog(s1: *mut TCCState, value: libc::c_int);
    fn tcc_debug_funcend(s1: *mut TCCState, size: libc::c_int);
    fn tcc_debug_extern_sym(
        s1: *mut TCCState,
        sym: *mut Sym,
        sh_num: libc::c_int,
        sym_bind: libc::c_int,
        sym_type: libc::c_int,
    );
    fn tcc_debug_typedef(s1: *mut TCCState, sym: *mut Sym);
    fn tcc_debug_stabn(s1: *mut TCCState, type_0: libc::c_int, value: libc::c_int);
    fn tcc_debug_fix_anon(s1: *mut TCCState, t: *mut CType);
    fn tcc_eh_frame_end(s1: *mut TCCState);
    fn tcc_tcov_start(s1: *mut TCCState);
    fn tcc_tcov_end(s1: *mut TCCState);
    fn tcc_tcov_check_line(s1: *mut TCCState, start: libc::c_int);
    fn tcc_tcov_block_end(s1: *mut TCCState, line: libc::c_int);
    fn tcc_tcov_block_begin(s1: *mut TCCState);
    fn tcc_tcov_reset_ind(s1: *mut TCCState);
    fn gen_struct_copy(size: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Elf64_Word = uint32_t;
pub type Elf64_Section = uint16_t;
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const TREG_MEM: C2RustUnnamed_4 = 32;
pub const TREG_ST0: C2RustUnnamed_4 = 24;
pub const TREG_XMM7: C2RustUnnamed_4 = 23;
pub const TREG_XMM6: C2RustUnnamed_4 = 22;
pub const TREG_XMM5: C2RustUnnamed_4 = 21;
pub const TREG_XMM4: C2RustUnnamed_4 = 20;
pub const TREG_XMM3: C2RustUnnamed_4 = 19;
pub const TREG_XMM2: C2RustUnnamed_4 = 18;
pub const TREG_XMM1: C2RustUnnamed_4 = 17;
pub const TREG_XMM0: C2RustUnnamed_4 = 16;
pub const TREG_R11: C2RustUnnamed_4 = 11;
pub const TREG_R10: C2RustUnnamed_4 = 10;
pub const TREG_R9: C2RustUnnamed_4 = 9;
pub const TREG_R8: C2RustUnnamed_4 = 8;
pub const TREG_RDI: C2RustUnnamed_4 = 7;
pub const TREG_RSI: C2RustUnnamed_4 = 6;
pub const TREG_RSP: C2RustUnnamed_4 = 4;
pub const TREG_RDX: C2RustUnnamed_4 = 2;
pub const TREG_RCX: C2RustUnnamed_4 = 1;
pub const TREG_RAX: C2RustUnnamed_4 = 0;
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
    pub str_0: C2RustUnnamed_5,
    pub tab: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub data: *mut libc::c_char,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SValue {
    pub type_0: CType,
    pub r: libc::c_ushort,
    pub r2: libc::c_ushort,
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub c2rust_unnamed_0: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub sym: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub cmp_op: libc::c_ushort,
    pub cmp_r: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c: CValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub jtrue: libc::c_int,
    pub jfalse: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttributeDef {
    pub a: SymAttr,
    pub f: FuncAttr,
    pub section: *mut Section,
    pub cleanup_func: *mut Sym,
    pub alias_target: libc::c_int,
    pub asm_label: libc::c_int,
    pub attr_mode: libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub f: libc::c_float,
    pub u: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct temp_local_variable {
    pub location: libc::c_int,
    pub size: libc::c_short,
    pub align: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct init_params {
    pub sec: *mut Section,
    pub local_offset: libc::c_int,
    pub flex_array_ref: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub loc: libc::c_int,
    pub locorig: libc::c_int,
    pub num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scope {
    pub prev: *mut scope,
    pub vla: C2RustUnnamed_11,
    pub cl: C2RustUnnamed_12,
    pub bsym: *mut libc::c_int,
    pub csym: *mut libc::c_int,
    pub lstk: *mut Sym,
    pub llstk: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub s: *mut Sym,
    pub n: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switch_t {
    pub p: *mut *mut case_t,
    pub n: libc::c_int,
    pub def_sym: libc::c_int,
    pub nocode_wanted: libc::c_int,
    pub bsym: *mut libc::c_int,
    pub scope: *mut scope,
    pub prev: *mut switch_t,
    pub sv: SValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_t {
    pub v1: int64_t,
    pub v2: int64_t,
    pub ind: libc::c_int,
    pub line: libc::c_int,
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
unsafe extern "C" fn write32le(mut p: *mut libc::c_uchar, mut x: uint32_t) {
    write16le(p, x as uint16_t);
    write16le(p.offset(2 as libc::c_int as isize), (x >> 16 as libc::c_int) as uint16_t);
}
#[inline]
unsafe extern "C" fn write64le(mut p: *mut libc::c_uchar, mut x: uint64_t) {
    write32le(p, x as uint32_t);
    write32le(p.offset(4 as libc::c_int as isize), (x >> 32 as libc::c_int) as uint32_t);
}
#[no_mangle]
pub static mut rsym: libc::c_int = 0;
#[no_mangle]
pub static mut anon_sym: libc::c_int = 0;
#[no_mangle]
pub static mut ind: libc::c_int = 0;
#[no_mangle]
pub static mut loc: libc::c_int = 0;
#[no_mangle]
pub static mut global_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut local_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut define_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut global_label_stack: *mut Sym = 0 as *const Sym as *mut Sym;
#[no_mangle]
pub static mut local_label_stack: *mut Sym = 0 as *const Sym as *mut Sym;
static mut sym_free_first: *mut Sym = 0 as *const Sym as *mut Sym;
static mut sym_pools: *mut *mut libc::c_void = 0 as *const *mut libc::c_void
    as *mut *mut libc::c_void;
static mut nb_sym_pools: libc::c_int = 0;
static mut all_cleanups: *mut Sym = 0 as *const Sym as *mut Sym;
static mut pending_gotos: *mut Sym = 0 as *const Sym as *mut Sym;
static mut local_scope: libc::c_int = 0;
#[no_mangle]
pub static mut debug_modes: libc::c_char = 0;
#[no_mangle]
pub static mut vtop: *mut SValue = 0 as *const SValue as *mut SValue;
static mut _vstack: [SValue; 513] = [SValue {
    type_0: CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    },
    r: 0,
    r2: 0,
    c2rust_unnamed: C2RustUnnamed_8 {
        c2rust_unnamed: C2RustUnnamed_9 {
            jtrue: 0,
            jfalse: 0,
        },
    },
    c2rust_unnamed_0: C2RustUnnamed_6 {
        c2rust_unnamed: C2RustUnnamed_7 {
            cmp_op: 0,
            cmp_r: 0,
        },
    },
}; 513];
#[no_mangle]
pub static mut nocode_wanted: libc::c_int = 0;
#[no_mangle]
pub static mut global_expr: libc::c_int = 0;
#[no_mangle]
pub static mut func_vt: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut func_var: libc::c_int = 0;
#[no_mangle]
pub static mut func_vc: libc::c_int = 0;
#[no_mangle]
pub static mut func_ind: libc::c_int = 0;
#[no_mangle]
pub static mut funcname: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub static mut int_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut func_old_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut char_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
#[no_mangle]
pub static mut char_pointer_type: CType = CType {
    t: 0,
    ref_0: 0 as *const Sym as *mut Sym,
};
static mut initstr: CString = CString {
    size: 0,
    size_allocated: 0,
    data: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut cur_switch: *mut switch_t = 0 as *const switch_t as *mut switch_t;
static mut arr_temp_local_vars: [temp_local_variable; 8] = [temp_local_variable {
    location: 0,
    size: 0,
    align: 0,
}; 8];
static mut nb_temp_local_vars: libc::c_int = 0;
static mut cur_scope: *mut scope = 0 as *const scope as *mut scope;
static mut loop_scope: *mut scope = 0 as *const scope as *mut scope;
static mut root_scope: *mut scope = 0 as *const scope as *mut scope;
#[no_mangle]
pub unsafe extern "C" fn gsym(mut t: libc::c_int) {
    if t != 0 {
        gsym_addr(t, ind);
        nocode_wanted &= !(0x20000000 as libc::c_int);
    }
}
unsafe extern "C" fn gind() -> libc::c_int {
    let mut t: libc::c_int = ind;
    nocode_wanted &= !(0x20000000 as libc::c_int);
    if debug_modes != 0 {
        tcc_tcov_block_begin(tcc_state);
    }
    return t;
}
unsafe extern "C" fn gjmp_addr_acs(mut t: libc::c_int) {
    gjmp_addr(t);
    if nocode_wanted == 0 {
        nocode_wanted |= 0x20000000 as libc::c_int;
    }
}
unsafe extern "C" fn gjmp_acs(mut t: libc::c_int) -> libc::c_int {
    t = gjmp(t);
    if nocode_wanted == 0 {
        nocode_wanted |= 0x20000000 as libc::c_int;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn is_float(mut t: libc::c_int) -> libc::c_int {
    let mut bt: libc::c_int = t & 0xf as libc::c_int;
    return (bt == 10 as libc::c_int || bt == 9 as libc::c_int || bt == 8 as libc::c_int
        || bt == 14 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn is_integer_btype(mut bt: libc::c_int) -> libc::c_int {
    return (bt == 1 as libc::c_int || bt == 11 as libc::c_int || bt == 2 as libc::c_int
        || bt == 3 as libc::c_int || bt == 4 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn btype_size(mut bt: libc::c_int) -> libc::c_int {
    return if bt == 1 as libc::c_int || bt == 11 as libc::c_int {
        1 as libc::c_int
    } else if bt == 2 as libc::c_int {
        2 as libc::c_int
    } else if bt == 3 as libc::c_int {
        4 as libc::c_int
    } else if bt == 4 as libc::c_int {
        8 as libc::c_int
    } else if bt == 5 as libc::c_int {
        8 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn R_RET(mut t: libc::c_int) -> libc::c_int {
    if is_float(t) == 0 {
        return TREG_RAX as libc::c_int;
    }
    if t & 0xf as libc::c_int == 10 as libc::c_int {
        return TREG_ST0 as libc::c_int;
    }
    return TREG_XMM0 as libc::c_int;
}
unsafe extern "C" fn R2_RET(mut t: libc::c_int) -> libc::c_int {
    t &= 0xf as libc::c_int;
    if t == 13 as libc::c_int {
        return TREG_RDX as libc::c_int;
    }
    if t == 14 as libc::c_int {
        return TREG_XMM1 as libc::c_int;
    }
    return 0x30 as libc::c_int;
}
unsafe extern "C" fn PUT_R_RET(mut sv: *mut SValue, mut t: libc::c_int) {
    (*sv).r = R_RET(t) as libc::c_ushort;
    (*sv).r2 = R2_RET(t) as libc::c_ushort;
}
unsafe extern "C" fn RC_RET(mut t: libc::c_int) -> libc::c_int {
    return reg_classes[R_RET(t) as usize] & !(0x2 as libc::c_int | 0x1 as libc::c_int);
}
unsafe extern "C" fn RC_TYPE(mut t: libc::c_int) -> libc::c_int {
    if is_float(t) == 0 {
        return 0x1 as libc::c_int;
    }
    if t & 0xf as libc::c_int == 10 as libc::c_int {
        return 0x80 as libc::c_int;
    }
    if t & 0xf as libc::c_int == 14 as libc::c_int {
        return 0x1000 as libc::c_int;
    }
    return 0x2 as libc::c_int;
}
unsafe extern "C" fn RC2_TYPE(mut t: libc::c_int, mut rc: libc::c_int) -> libc::c_int {
    if !(R2_RET(t) != 0x30 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if rc == 0x4 as libc::c_int {
        return 0x8 as libc::c_int;
    }
    if rc == 0x1000 as libc::c_int {
        return 0x2000 as libc::c_int;
    }
    if rc & 0x2 as libc::c_int != 0 {
        return 0x2 as libc::c_int;
    }
    return 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ieee_finite(mut d: libc::c_double) -> libc::c_int {
    let mut p: [libc::c_int; 4] = [0; 4];
    memcpy(
        p.as_mut_ptr() as *mut libc::c_void,
        &mut d as *mut libc::c_double as *const libc::c_void,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    );
    return ((p[1 as libc::c_int as usize] as libc::c_uint | 0x800fffff as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) >> 31 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn test_lvalue() {
    if (*vtop).r as libc::c_int & 0x100 as libc::c_int == 0 {
        expect(b"lvalue\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_vstack() {
    if vtop
        != _vstack
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize))
    {
        _tcc_error(
            b"internal compiler error: vstack leak (%d)\0" as *const u8
                as *const libc::c_char,
            (vtop.offset_from(_vstack.as_mut_ptr().offset(1 as libc::c_int as isize))
                as libc::c_long + 1 as libc::c_int as libc::c_long) as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn tccgen_init(mut s1: *mut TCCState) {
    vtop = _vstack
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize));
    memset(
        vtop as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SValue>() as libc::c_ulong,
    );
    int_type.t = 3 as libc::c_int;
    char_type.t = 1 as libc::c_int;
    if (*s1).char_is_unsigned != 0 {
        char_type.t |= 0x10 as libc::c_int;
    }
    char_pointer_type = char_type;
    mk_pointer(&mut char_pointer_type);
    func_old_type.t = 6 as libc::c_int;
    func_old_type
        .ref_0 = sym_push(
        0x20000000 as libc::c_int,
        &mut int_type,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    ((*func_old_type.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
        .set_func_call(0 as libc::c_int as libc::c_uint);
    ((*func_old_type.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
        .set_func_type(2 as libc::c_int as libc::c_uint);
    init_prec();
    cstr_new(&mut initstr);
}
#[no_mangle]
pub unsafe extern "C" fn tccgen_compile(mut s1: *mut TCCState) -> libc::c_int {
    funcname = b"\0" as *const u8 as *const libc::c_char;
    func_ind = -(1 as libc::c_int);
    anon_sym = 0x10000000 as libc::c_int;
    nocode_wanted = 0x80000000 as libc::c_uint as libc::c_int;
    debug_modes = ((if (*s1).do_debug as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) | ((*s1).test_coverage as libc::c_int) << 1 as libc::c_int) as libc::c_char;
    tcc_debug_start(s1);
    tcc_tcov_start(s1);
    parse_flags = 0x1 as libc::c_int | 0x2 as libc::c_int | 0x40 as libc::c_int;
    next();
    decl(0x30 as libc::c_int);
    gen_inline_functions(s1);
    check_vstack();
    tcc_eh_frame_end(s1);
    tcc_debug_end(s1);
    tcc_tcov_end(s1);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tccgen_finish(mut s1: *mut TCCState) {
    tcc_debug_end(s1);
    free_inline_functions(s1);
    sym_pop(&mut global_stack, 0 as *mut Sym, 0 as libc::c_int);
    sym_pop(&mut local_stack, 0 as *mut Sym, 0 as libc::c_int);
    free_defines(0 as *mut Sym);
    dynarray_reset(
        &mut sym_pools as *mut *mut *mut libc::c_void as *mut libc::c_void,
        &mut nb_sym_pools,
    );
    cstr_free(&mut initstr);
    dynarray_reset(
        &mut stk_data as *mut *mut *mut libc::c_void as *mut libc::c_void,
        &mut nb_stk_data,
    );
    while !cur_switch.is_null() {
        end_switch();
    }
    local_scope = 0 as libc::c_int;
    loop_scope = 0 as *mut scope;
    all_cleanups = 0 as *mut Sym;
    pending_gotos = 0 as *mut Sym;
    nb_temp_local_vars = 0 as libc::c_int;
    global_label_stack = 0 as *mut Sym;
    local_label_stack = 0 as *mut Sym;
    (*tcc_state).cur_text_section = 0 as *mut Section;
    sym_free_first = 0 as *mut Sym;
}
#[no_mangle]
pub unsafe extern "C" fn elfsym(mut s: *mut Sym) -> *mut Elf64_Sym {
    if s.is_null() || (*s).c2rust_unnamed.c2rust_unnamed.c == 0 {
        return 0 as *mut Elf64_Sym;
    }
    return &mut *((*(*tcc_state).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset((*s).c2rust_unnamed.c2rust_unnamed.c as isize) as *mut Elf64_Sym;
}
#[no_mangle]
pub unsafe extern "C" fn update_storage(mut sym: *mut Sym) {
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_bind: libc::c_int = 0;
    let mut old_sym_bind: libc::c_int = 0;
    esym = elfsym(sym);
    if esym.is_null() {
        return;
    }
    if ((*sym).a).visibility() != 0 {
        (*esym)
            .st_other = ((*esym).st_other as libc::c_int
            & !(-(1 as libc::c_int) & 0x3 as libc::c_int)
            | ((*sym).a).visibility() as libc::c_int) as libc::c_uchar;
    }
    if (*sym).type_0.t & (0x2000 as libc::c_int | 0x8000 as libc::c_int) != 0 {
        sym_bind = 0 as libc::c_int;
    } else if ((*sym).a).weak() != 0 {
        sym_bind = 2 as libc::c_int;
    } else {
        sym_bind = 1 as libc::c_int;
    }
    old_sym_bind = (*esym).st_info as libc::c_int >> 4 as libc::c_int;
    if sym_bind != old_sym_bind {
        (*esym)
            .st_info = ((sym_bind << 4 as libc::c_int)
            + ((*esym).st_info as libc::c_int & 0xf as libc::c_int & 0xf as libc::c_int))
            as libc::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn put_extern_sym2(
    mut sym: *mut Sym,
    mut sh_num: libc::c_int,
    mut value: Elf64_Addr,
    mut size: libc::c_ulong,
    mut can_add_underscore: libc::c_int,
) {
    let mut sym_type: libc::c_int = 0;
    let mut sym_bind: libc::c_int = 0;
    let mut info: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf1: [libc::c_char; 256] = [0; 256];
    if (*sym).c2rust_unnamed.c2rust_unnamed.c == 0 {
        name = get_tok_str((*sym).v, 0 as *mut CValue);
        t = (*sym).type_0.t;
        if t & 0xf as libc::c_int == 6 as libc::c_int {
            sym_type = 2 as libc::c_int;
        } else if t & 0xf as libc::c_int == 0 as libc::c_int {
            sym_type = 0 as libc::c_int;
            if t
                & (0xf as libc::c_int
                    | (0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int
                        | (2 as libc::c_int) << 20 as libc::c_int))
                == 0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int
                    | (2 as libc::c_int) << 20 as libc::c_int
            {
                sym_type = 2 as libc::c_int;
            }
        } else {
            sym_type = 1 as libc::c_int;
        }
        if t & (0x2000 as libc::c_int | 0x8000 as libc::c_int) != 0 {
            sym_bind = 0 as libc::c_int;
        } else {
            sym_bind = 1 as libc::c_int;
        }
        other = 0 as libc::c_int;
        if (*sym).c2rust_unnamed_0.asm_label != 0 {
            name = get_tok_str((*sym).c2rust_unnamed_0.asm_label, 0 as *mut CValue);
            can_add_underscore = 0 as libc::c_int;
        }
        if (*tcc_state).leading_underscore as libc::c_int != 0 && can_add_underscore != 0
        {
            buf1[0 as libc::c_int as usize] = '_' as i32 as libc::c_char;
            pstrcpy(
                buf1.as_mut_ptr().offset(1 as libc::c_int as isize),
                (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                name,
            );
            name = buf1.as_mut_ptr();
        }
        info = (sym_bind << 4 as libc::c_int) + (sym_type & 0xf as libc::c_int);
        (*sym)
            .c2rust_unnamed
            .c2rust_unnamed
            .c = put_elf_sym(
            (*tcc_state).c2rust_unnamed.symtab_section,
            value,
            size,
            info,
            other,
            sh_num,
            name,
        );
        if debug_modes != 0 {
            tcc_debug_extern_sym(tcc_state, sym, sh_num, sym_bind, sym_type);
        }
    } else {
        esym = elfsym(sym);
        (*esym).st_value = value;
        (*esym).st_size = size;
        (*esym).st_shndx = sh_num as Elf64_Section;
    }
    update_storage(sym);
}
#[no_mangle]
pub unsafe extern "C" fn put_extern_sym(
    mut sym: *mut Sym,
    mut s: *mut Section,
    mut value: Elf64_Addr,
    mut size: libc::c_ulong,
) {
    if nocode_wanted != 0
        && (nocode_wanted > 0 as libc::c_int
            || !s.is_null() && s == (*tcc_state).cur_text_section)
    {
        return;
    }
    put_extern_sym2(
        sym,
        if !s.is_null() { (*s).sh_num } else { 0 as libc::c_int },
        value,
        size,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn greloca(
    mut s: *mut Section,
    mut sym: *mut Sym,
    mut offset: libc::c_ulong,
    mut type_0: libc::c_int,
    mut addend: Elf64_Addr,
) {
    let mut c: libc::c_int = 0 as libc::c_int;
    if nocode_wanted != 0 && s == (*tcc_state).cur_text_section {
        return;
    }
    if !sym.is_null() {
        if 0 as libc::c_int == (*sym).c2rust_unnamed.c2rust_unnamed.c {
            put_extern_sym(
                sym,
                0 as *mut Section,
                0 as libc::c_int as Elf64_Addr,
                0 as libc::c_int as libc::c_ulong,
            );
        }
        c = (*sym).c2rust_unnamed.c2rust_unnamed.c;
    }
    put_elf_reloca(
        (*tcc_state).c2rust_unnamed.symtab_section,
        s,
        offset,
        type_0,
        c,
        addend,
    );
}
unsafe extern "C" fn __sym_malloc() -> *mut Sym {
    let mut sym_pool: *mut Sym = 0 as *mut Sym;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut last_sym: *mut Sym = 0 as *mut Sym;
    let mut i: libc::c_int = 0;
    sym_pool = tcc_malloc(
        (8192 as libc::c_int as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Sym>() as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Sym>() as libc::c_ulong),
    ) as *mut Sym;
    dynarray_add(
        &mut sym_pools as *mut *mut *mut libc::c_void as *mut libc::c_void,
        &mut nb_sym_pools,
        sym_pool as *mut libc::c_void,
    );
    last_sym = sym_free_first;
    sym = sym_pool;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (8192 as libc::c_int as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Sym>() as libc::c_ulong)
    {
        (*sym).c2rust_unnamed_0.next = last_sym;
        last_sym = sym;
        sym = sym.offset(1);
        sym;
        i += 1;
        i;
    }
    sym_free_first = last_sym;
    return last_sym;
}
#[inline]
unsafe extern "C" fn sym_malloc() -> *mut Sym {
    let mut sym: *mut Sym = 0 as *mut Sym;
    sym = sym_free_first;
    if sym.is_null() {
        sym = __sym_malloc();
    }
    sym_free_first = (*sym).c2rust_unnamed_0.next;
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn sym_free(mut sym: *mut Sym) {
    (*sym).c2rust_unnamed_0.next = sym_free_first;
    sym_free_first = sym;
}
#[no_mangle]
pub unsafe extern "C" fn sym_push2(
    mut ps: *mut *mut Sym,
    mut v: libc::c_int,
    mut t: libc::c_int,
    mut c: libc::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_malloc();
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Sym>() as libc::c_ulong,
    );
    (*s).v = v;
    (*s).type_0.t = t;
    (*s).c2rust_unnamed.c2rust_unnamed.c = c;
    (*s).prev = *ps;
    *ps = s;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sym_find2(mut s: *mut Sym, mut v: libc::c_int) -> *mut Sym {
    while !s.is_null() {
        if (*s).v == v {
            return s;
        }
        s = (*s).prev;
    }
    return 0 as *mut Sym;
}
#[no_mangle]
pub unsafe extern "C" fn struct_find(mut v: libc::c_int) -> *mut Sym {
    v -= 256 as libc::c_int;
    if v as libc::c_uint >= (tok_ident - 256 as libc::c_int) as libc::c_uint {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_struct;
}
#[no_mangle]
pub unsafe extern "C" fn sym_find(mut v: libc::c_int) -> *mut Sym {
    v -= 256 as libc::c_int;
    if v as libc::c_uint >= (tok_ident - 256 as libc::c_int) as libc::c_uint {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_identifier;
}
unsafe extern "C" fn sym_scope(mut s: *mut Sym) -> libc::c_int {
    if (*s).type_0.t as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((3 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
    {
        return (*(*s).type_0.ref_0)
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .sym_scope
    } else {
        return (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope
    };
}
#[no_mangle]
pub unsafe extern "C" fn sym_push(
    mut v: libc::c_int,
    mut type_0: *mut CType,
    mut r: libc::c_int,
    mut c: libc::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    if !local_stack.is_null() {
        ps = &mut local_stack;
    } else {
        ps = &mut global_stack;
    }
    s = sym_push2(ps, v, (*type_0).t, c);
    (*s).type_0.ref_0 = (*type_0).ref_0;
    (*s).r = r as libc::c_ushort;
    if v & 0x20000000 as libc::c_int == 0
        && v & !(0x40000000 as libc::c_int) < 0x10000000 as libc::c_int
    {
        ts = *table_ident
            .offset(((v & !(0x40000000 as libc::c_int)) - 256 as libc::c_int) as isize);
        if v & 0x40000000 as libc::c_int != 0 {
            ps = &mut (*ts).sym_struct;
        } else {
            ps = &mut (*ts).sym_identifier;
        }
        (*s).prev_tok = *ps;
        *ps = s;
        (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope = local_scope;
        if !((*s).prev_tok).is_null()
            && sym_scope((*s).prev_tok)
                == (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope
        {
            _tcc_error(
                b"redeclaration of '%s'\0" as *const u8 as *const libc::c_char,
                get_tok_str(v & !(0x40000000 as libc::c_int), 0 as *mut CValue),
            );
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn global_identifier_push(
    mut v: libc::c_int,
    mut t: libc::c_int,
    mut c: libc::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    s = sym_push2(&mut global_stack, v, t, c);
    (*s).r = (0x30 as libc::c_int | 0x200 as libc::c_int) as libc::c_ushort;
    if v < 0x10000000 as libc::c_int {
        ps = &mut (**table_ident.offset((v - 256 as libc::c_int) as isize))
            .sym_identifier;
        while !(*ps).is_null()
            && (**ps).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope != 0
        {
            ps = &mut (**ps).prev_tok;
        }
        (*s).prev_tok = *ps;
        *ps = s;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn sym_pop(
    mut ptop: *mut *mut Sym,
    mut b: *mut Sym,
    mut keep: libc::c_int,
) {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ss: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut ts: *mut TokenSym = 0 as *mut TokenSym;
    let mut v: libc::c_int = 0;
    s = *ptop;
    while s != b {
        ss = (*s).prev;
        v = (*s).v;
        if v & 0x20000000 as libc::c_int == 0
            && v & !(0x40000000 as libc::c_int) < 0x10000000 as libc::c_int
        {
            ts = *table_ident
                .offset(
                    ((v & !(0x40000000 as libc::c_int)) - 256 as libc::c_int) as isize,
                );
            if v & 0x40000000 as libc::c_int != 0 {
                ps = &mut (*ts).sym_struct;
            } else {
                ps = &mut (*ts).sym_identifier;
            }
            *ps = (*s).prev_tok;
        }
        if keep == 0 {
            sym_free(s);
        }
        s = ss;
    }
    if keep == 0 {
        *ptop = b;
    }
}
#[no_mangle]
pub unsafe extern "C" fn label_find(mut v: libc::c_int) -> *mut Sym {
    v -= 256 as libc::c_int;
    if v as libc::c_uint >= (tok_ident - 256 as libc::c_int) as libc::c_uint {
        return 0 as *mut Sym;
    }
    return (**table_ident.offset(v as isize)).sym_label;
}
#[no_mangle]
pub unsafe extern "C" fn label_push(
    mut ptop: *mut *mut Sym,
    mut v: libc::c_int,
    mut flags: libc::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    s = sym_push2(ptop, v, 0x2000 as libc::c_int, 0 as libc::c_int);
    (*s).r = flags as libc::c_ushort;
    ps = &mut (**table_ident.offset((v - 256 as libc::c_int) as isize)).sym_label;
    if ptop == &mut global_label_stack as *mut *mut Sym {
        while !(*ps).is_null() {
            ps = &mut (**ps).prev_tok;
        }
    }
    (*s).prev_tok = *ps;
    *ps = s;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn label_pop(
    mut ptop: *mut *mut Sym,
    mut slast: *mut Sym,
    mut keep: libc::c_int,
) {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut s1: *mut Sym = 0 as *mut Sym;
    s = *ptop;
    while s != slast {
        s1 = (*s).prev;
        if (*s).r as libc::c_int == 2 as libc::c_int {
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
                b"label '%s' declared but not used\0" as *const u8
                    as *const libc::c_char,
                get_tok_str((*s).v, 0 as *mut CValue),
            );
        } else if (*s).r as libc::c_int == 1 as libc::c_int {
            _tcc_error(
                b"label '%s' used but not defined\0" as *const u8 as *const libc::c_char,
                get_tok_str((*s).v, 0 as *mut CValue),
            );
        } else if (*s).c2rust_unnamed.c2rust_unnamed.c != 0 {
            put_extern_sym(
                s,
                (*tcc_state).cur_text_section,
                (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext as Elf64_Addr,
                1 as libc::c_int as libc::c_ulong,
            );
        }
        if (*s).r as libc::c_int != 3 as libc::c_int {
            let ref mut fresh0 = (**table_ident
                .offset(((*s).v - 256 as libc::c_int) as isize))
                .sym_label;
            *fresh0 = (*s).prev_tok;
        }
        if keep == 0 {
            sym_free(s);
        } else {
            (*s).r = 3 as libc::c_int as libc::c_ushort;
        }
        s = s1;
    }
    if keep == 0 {
        *ptop = slast;
    }
}
unsafe extern "C" fn vcheck_cmp() {
    if (*vtop).r as libc::c_int == 0x33 as libc::c_int
        && 0 as libc::c_int == nocode_wanted & !(0x20000000 as libc::c_int)
    {
        gv(0x1 as libc::c_int);
    }
}
unsafe extern "C" fn vsetc(
    mut type_0: *mut CType,
    mut r: libc::c_int,
    mut vc: *mut CValue,
) {
    if vtop
        >= _vstack
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize)
            .offset((512 as libc::c_int - 1 as libc::c_int) as isize)
    {
        _tcc_error(b"memory full (vstack)\0" as *const u8 as *const libc::c_char);
    }
    vcheck_cmp();
    vtop = vtop.offset(1);
    vtop;
    (*vtop).type_0 = *type_0;
    (*vtop).r = r as libc::c_ushort;
    (*vtop).r2 = 0x30 as libc::c_int as libc::c_ushort;
    (*vtop).c2rust_unnamed.c = *vc;
    (*vtop).c2rust_unnamed_0.sym = 0 as *mut Sym;
}
#[no_mangle]
pub unsafe extern "C" fn vswap() {
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    vcheck_cmp();
    tmp = *vtop.offset(0 as libc::c_int as isize);
    *vtop.offset(0 as libc::c_int as isize) = *vtop.offset(-(1 as libc::c_int) as isize);
    *vtop.offset(-(1 as libc::c_int) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn vpop() {
    let mut v: libc::c_int = 0;
    v = (*vtop).r as libc::c_int & 0x3f as libc::c_int;
    if v == TREG_ST0 as libc::c_int {
        o(0xd8dd as libc::c_int as libc::c_uint);
    } else if v == 0x33 as libc::c_int {
        gsym((*vtop).c2rust_unnamed.c2rust_unnamed.jtrue);
        gsym((*vtop).c2rust_unnamed.c2rust_unnamed.jfalse);
    }
    vtop = vtop.offset(-1);
    vtop;
}
unsafe extern "C" fn vpush(mut type_0: *mut CType) {
    vset(type_0, 0x30 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn vpush64(mut ty: libc::c_int, mut v: libc::c_ulonglong) {
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    let mut ctype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    ctype.t = ty;
    ctype.ref_0 = 0 as *mut Sym;
    cval.i = v as uint64_t;
    vsetc(&mut ctype, 0x30 as libc::c_int, &mut cval);
}
#[no_mangle]
pub unsafe extern "C" fn vpushi(mut v: libc::c_int) {
    vpush64(3 as libc::c_int, v as libc::c_ulonglong);
}
unsafe extern "C" fn vpushs(mut v: Elf64_Addr) {
    vpush64(
        0x800 as libc::c_int | 4 as libc::c_int | 0x10 as libc::c_int,
        v as libc::c_ulonglong,
    );
}
#[inline]
unsafe extern "C" fn vpushll(mut v: libc::c_longlong) {
    vpush64(4 as libc::c_int, v as libc::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn vset(
    mut type_0: *mut CType,
    mut r: libc::c_int,
    mut v: libc::c_int,
) {
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    cval.i = v as uint64_t;
    vsetc(type_0, r, &mut cval);
}
unsafe extern "C" fn vseti(mut r: libc::c_int, mut v: libc::c_int) {
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    type_0.t = 3 as libc::c_int;
    type_0.ref_0 = 0 as *mut Sym;
    vset(&mut type_0, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn vpushv(mut v: *mut SValue) {
    if vtop
        >= _vstack
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize)
            .offset((512 as libc::c_int - 1 as libc::c_int) as isize)
    {
        _tcc_error(b"memory full (vstack)\0" as *const u8 as *const libc::c_char);
    }
    vtop = vtop.offset(1);
    vtop;
    *vtop = *v;
}
unsafe extern "C" fn vdup() {
    vpushv(vtop);
}
#[no_mangle]
pub unsafe extern "C" fn vrotb(mut n: libc::c_int) {
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    n -= 1;
    if n < 1 as libc::c_int {
        return;
    }
    vcheck_cmp();
    tmp = *vtop.offset(-n as isize);
    memmove(
        vtop.offset(-(n as isize)) as *mut libc::c_void,
        vtop.offset(-(n as isize)).offset(1 as libc::c_int as isize)
            as *const libc::c_void,
        (::core::mem::size_of::<SValue>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    *vtop.offset(0 as libc::c_int as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn vrott(mut n: libc::c_int) {
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    n -= 1;
    if n < 1 as libc::c_int {
        return;
    }
    vcheck_cmp();
    tmp = *vtop.offset(0 as libc::c_int as isize);
    memmove(
        vtop.offset(-(n as isize)).offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        vtop.offset(-(n as isize)) as *const libc::c_void,
        (::core::mem::size_of::<SValue>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    *vtop.offset(-n as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn vrev(mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut tmp: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    vcheck_cmp();
    i = 0 as libc::c_int;
    n = -n;
    loop {
        n += 1;
        if !(i > n) {
            break;
        }
        tmp = *vtop.offset(i as isize);
        *vtop.offset(i as isize) = *vtop.offset(n as isize);
        *vtop.offset(n as isize) = tmp;
        i -= 1;
        i;
    };
}
#[no_mangle]
pub unsafe extern "C" fn vset_VT_CMP(mut op: libc::c_int) {
    (*vtop).r = 0x33 as libc::c_int as libc::c_ushort;
    (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op = op as libc::c_ushort;
    (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse = 0 as libc::c_int;
    (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue = 0 as libc::c_int;
}
unsafe extern "C" fn vset_VT_JMP() {
    let mut op: libc::c_int = (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op
        as libc::c_int;
    if (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue != 0
        || (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse != 0
    {
        let mut origt: libc::c_int = (*vtop).type_0.t;
        let mut inv: libc::c_int = op & (op < 2 as libc::c_int) as libc::c_int;
        vseti(0x34 as libc::c_int + inv, gvtst(inv, 0 as libc::c_int));
        (*vtop).type_0.t |= origt & (0x10 as libc::c_int | 0x20 as libc::c_int);
    } else {
        (*vtop).c2rust_unnamed.c.i = op as uint64_t;
        if op < 2 as libc::c_int {
            (*vtop).r = 0x30 as libc::c_int as libc::c_ushort;
        }
    };
}
unsafe extern "C" fn gvtst_set(mut inv: libc::c_int, mut t: libc::c_int) {
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*vtop).r as libc::c_int != 0x33 as libc::c_int {
        vpushi(0 as libc::c_int);
        gen_op(0x95 as libc::c_int);
        if (*vtop).r as libc::c_int != 0x33 as libc::c_int {
            vset_VT_CMP(
                ((*vtop).c2rust_unnamed.c.i != 0 as libc::c_int as uint64_t)
                    as libc::c_int,
            );
        }
    }
    p = if inv != 0 {
        &mut (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse
    } else {
        &mut (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue
    };
    *p = gjmp_append(*p, t);
}
unsafe extern "C" fn gvtst(mut inv: libc::c_int, mut t: libc::c_int) -> libc::c_int {
    let mut op: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    gvtst_set(inv, t);
    t = (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue;
    u = (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse;
    if inv != 0 {
        x = u;
        u = t;
        t = x;
    }
    op = (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op as libc::c_int;
    if op > 1 as libc::c_int {
        t = gjmp_cond(op ^ inv, t);
    } else if op != inv {
        t = gjmp_acs(t);
    }
    gsym(u);
    vtop = vtop.offset(-1);
    vtop;
    return t;
}
unsafe extern "C" fn gen_test_zero(mut op: libc::c_int) {
    if (*vtop).r as libc::c_int == 0x33 as libc::c_int {
        let mut j: libc::c_int = 0;
        if op == 0x94 as libc::c_int {
            j = (*vtop).c2rust_unnamed.c2rust_unnamed.jfalse;
            (*vtop)
                .c2rust_unnamed
                .c2rust_unnamed
                .jfalse = (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue;
            (*vtop).c2rust_unnamed.c2rust_unnamed.jtrue = j;
            (*vtop)
                .c2rust_unnamed_0
                .c2rust_unnamed
                .cmp_op = ((*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_op as libc::c_int
                ^ 1 as libc::c_int) as libc::c_ushort;
        }
    } else {
        vpushi(0 as libc::c_int);
        gen_op(op);
    };
}
#[no_mangle]
pub unsafe extern "C" fn vpushsym(mut type_0: *mut CType, mut sym: *mut Sym) {
    let mut cval: CValue = CValue { ld: f128::f128::ZERO };
    cval.i = 0 as libc::c_int as uint64_t;
    vsetc(type_0, 0x30 as libc::c_int | 0x200 as libc::c_int, &mut cval);
    (*vtop).c2rust_unnamed_0.sym = sym;
}
#[no_mangle]
pub unsafe extern "C" fn get_sym_ref(
    mut type_0: *mut CType,
    mut sec: *mut Section,
    mut offset: libc::c_ulong,
    mut size: libc::c_ulong,
) -> *mut Sym {
    let mut v: libc::c_int = 0;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let fresh1 = anon_sym;
    anon_sym = anon_sym + 1;
    v = fresh1;
    sym = sym_push(
        v,
        type_0,
        0x30 as libc::c_int | 0x200 as libc::c_int,
        0 as libc::c_int,
    );
    (*sym).type_0.t |= 0x2000 as libc::c_int;
    put_extern_sym(sym, sec, offset, size);
    return sym;
}
unsafe extern "C" fn vpush_ref(
    mut type_0: *mut CType,
    mut sec: *mut Section,
    mut offset: libc::c_ulong,
    mut size: libc::c_ulong,
) {
    vpushsym(type_0, get_sym_ref(type_0, sec, offset, size));
}
#[no_mangle]
pub unsafe extern "C" fn external_global_sym(
    mut v: libc::c_int,
    mut type_0: *mut CType,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_find(v);
    if s.is_null() {
        s = global_identifier_push(
            v,
            (*type_0).t | 0x1000 as libc::c_int,
            0 as libc::c_int,
        );
        (*s).type_0.ref_0 = (*type_0).ref_0;
    } else if (*s).type_0.t
        & (0xf as libc::c_int
            | (0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int))
        == 0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int
    {
        (*s).type_0.t = (*type_0).t | (*s).type_0.t & 0x1000 as libc::c_int;
        (*s).type_0.ref_0 = (*type_0).ref_0;
        update_storage(s);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn external_helper_sym(mut v: libc::c_int) -> *mut Sym {
    let mut ct: CType = {
        let mut init = CType {
            t: 0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int
                | (2 as libc::c_int) << 20 as libc::c_int,
            ref_0: 0 as *mut Sym,
        };
        init
    };
    return external_global_sym(v, &mut ct);
}
#[no_mangle]
pub unsafe extern "C" fn vpush_helper_func(mut v: libc::c_int) {
    vpushsym(&mut func_old_type, external_helper_sym(v));
}
unsafe extern "C" fn merge_symattr(mut sa: *mut SymAttr, mut sa1: *mut SymAttr) {
    if (*sa1).aligned() as libc::c_int != 0 && (*sa).aligned() == 0 {
        (*sa).set_aligned((*sa1).aligned());
    }
    (*sa).set_packed((*sa).packed() | (*sa1).packed() as libc::c_int as libc::c_ushort);
    (*sa).set_weak((*sa).weak() | (*sa1).weak() as libc::c_int as libc::c_ushort);
    (*sa)
        .set_nodebug(
            (*sa).nodebug() | (*sa1).nodebug() as libc::c_int as libc::c_ushort,
        );
    if (*sa1).visibility() as libc::c_int != 0 as libc::c_int {
        let mut vis: libc::c_int = (*sa).visibility() as libc::c_int;
        if vis == 0 as libc::c_int || vis > (*sa1).visibility() as libc::c_int {
            vis = (*sa1).visibility() as libc::c_int;
        }
        (*sa).set_visibility(vis as libc::c_ushort);
    }
    (*sa)
        .set_dllexport(
            (*sa).dllexport() | (*sa1).dllexport() as libc::c_int as libc::c_ushort,
        );
    (*sa)
        .set_nodecorate(
            (*sa).nodecorate() | (*sa1).nodecorate() as libc::c_int as libc::c_ushort,
        );
    (*sa)
        .set_dllimport(
            (*sa).dllimport() | (*sa1).dllimport() as libc::c_int as libc::c_ushort,
        );
}
unsafe extern "C" fn merge_funcattr(mut fa: *mut FuncAttr, mut fa1: *mut FuncAttr) {
    if (*fa1).func_call() as libc::c_int != 0 && (*fa).func_call() == 0 {
        (*fa).set_func_call((*fa1).func_call());
    }
    if (*fa1).func_type() as libc::c_int != 0 && (*fa).func_type() == 0 {
        (*fa).set_func_type((*fa1).func_type());
    }
    if (*fa1).func_args() as libc::c_int != 0 && (*fa).func_args() == 0 {
        (*fa).set_func_args((*fa1).func_args());
    }
    if (*fa1).func_noreturn() != 0 {
        (*fa).set_func_noreturn(1 as libc::c_int as libc::c_uint);
    }
    if (*fa1).func_ctor() != 0 {
        (*fa).set_func_ctor(1 as libc::c_int as libc::c_uint);
    }
    if (*fa1).func_dtor() != 0 {
        (*fa).set_func_dtor(1 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn merge_attr(mut ad: *mut AttributeDef, mut ad1: *mut AttributeDef) {
    merge_symattr(&mut (*ad).a, &mut (*ad1).a);
    merge_funcattr(&mut (*ad).f, &mut (*ad1).f);
    if !((*ad1).section).is_null() {
        (*ad).section = (*ad1).section;
    }
    if (*ad1).alias_target != 0 {
        (*ad).alias_target = (*ad1).alias_target;
    }
    if (*ad1).asm_label != 0 {
        (*ad).asm_label = (*ad1).asm_label;
    }
    if (*ad1).attr_mode != 0 {
        (*ad).attr_mode = (*ad1).attr_mode;
    }
}
unsafe extern "C" fn patch_type(mut sym: *mut Sym, mut type_0: *mut CType) {
    if (*type_0).t & 0x1000 as libc::c_int == 0
        || (*sym).type_0.t as libc::c_uint
            & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint)
            == ((3 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
    {
        if (*sym).type_0.t & 0x1000 as libc::c_int == 0 {
            _tcc_error(
                b"redefinition of '%s'\0" as *const u8 as *const libc::c_char,
                get_tok_str((*sym).v, 0 as *mut CValue),
            );
        }
        (*sym).type_0.t &= !(0x1000 as libc::c_int);
    }
    if (*sym).type_0.t
        & (0xf as libc::c_int
            | (0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int))
        == 0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int
    {
        (*sym).type_0.t = (*type_0).t & ((*sym).type_0.t | !(0x2000 as libc::c_int));
        (*sym).type_0.ref_0 = (*type_0).ref_0;
        if (*type_0).t & 0xf as libc::c_int != 6 as libc::c_int
            && (*type_0).t & 0x40 as libc::c_int == 0
        {
            (*sym)
                .r = ((*sym).r as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
        }
    }
    if is_compatible_types(&mut (*sym).type_0, type_0) == 0 {
        _tcc_error(
            b"incompatible types for redefinition of '%s'\0" as *const u8
                as *const libc::c_char,
            get_tok_str((*sym).v, 0 as *mut CValue),
        );
    } else if (*sym).type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
        let mut static_proto: libc::c_int = (*sym).type_0.t & 0x2000 as libc::c_int;
        if (*type_0).t & 0x2000 as libc::c_int != 0 && static_proto == 0
            && ((*type_0).t | (*sym).type_0.t) & 0x8000 as libc::c_int == 0
        {
            _tcc_warning(
                b"static storage ignored for redefinition of '%s'\0" as *const u8
                    as *const libc::c_char,
                get_tok_str((*sym).v, 0 as *mut CValue),
            );
        }
        if ((*type_0).t | (*sym).type_0.t) & 0x8000 as libc::c_int != 0 {
            if ((*type_0).t ^ (*sym).type_0.t) & 0x8000 as libc::c_int == 0
                || ((*type_0).t | (*sym).type_0.t) & 0x2000 as libc::c_int != 0
            {
                static_proto |= 0x8000 as libc::c_int;
            }
        }
        if 0 as libc::c_int == (*type_0).t & 0x1000 as libc::c_int {
            let mut f: FuncAttr = (*(*sym).type_0.ref_0)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .f;
            (*sym)
                .type_0
                .t = (*type_0).t & !(0x2000 as libc::c_int | 0x8000 as libc::c_int)
                | static_proto;
            (*sym).type_0.ref_0 = (*type_0).ref_0;
            merge_funcattr(
                &mut (*(*sym).type_0.ref_0)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .f,
                &mut f,
            );
        } else {
            (*sym).type_0.t &= !(0x8000 as libc::c_int) | static_proto;
        }
        if ((*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
            .func_type() as libc::c_int == 2 as libc::c_int
            && ((*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
                .func_type() as libc::c_int != 2 as libc::c_int
        {
            (*sym).type_0.ref_0 = (*type_0).ref_0;
        }
    } else {
        if (*sym).type_0.t & 0x40 as libc::c_int != 0
            && (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c >= 0 as libc::c_int
        {
            (*(*sym).type_0.ref_0)
                .c2rust_unnamed
                .c2rust_unnamed
                .c = (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c;
        }
        if ((*type_0).t ^ (*sym).type_0.t) & 0x2000 as libc::c_int != 0 {
            _tcc_warning(
                b"storage mismatch for redefinition of '%s'\0" as *const u8
                    as *const libc::c_char,
                get_tok_str((*sym).v, 0 as *mut CValue),
            );
        }
    };
}
unsafe extern "C" fn patch_storage(
    mut sym: *mut Sym,
    mut ad: *mut AttributeDef,
    mut type_0: *mut CType,
) {
    if !type_0.is_null() {
        patch_type(sym, type_0);
    }
    merge_symattr(&mut (*sym).a, &mut (*ad).a);
    if (*ad).asm_label != 0 {
        (*sym).c2rust_unnamed_0.asm_label = (*ad).asm_label;
    }
    update_storage(sym);
}
unsafe extern "C" fn sym_copy(mut s0: *mut Sym, mut ps: *mut *mut Sym) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_malloc();
    *s = *s0;
    (*s).prev = *ps;
    *ps = s;
    if (*s).v < 0x10000000 as libc::c_int {
        ps = &mut (**table_ident.offset(((*s).v - 256 as libc::c_int) as isize))
            .sym_identifier;
        (*s).prev_tok = *ps;
        *ps = s;
    }
    return s;
}
unsafe extern "C" fn sym_copy_ref(mut s: *mut Sym, mut ps: *mut *mut Sym) {
    let mut bt: libc::c_int = (*s).type_0.t & 0xf as libc::c_int;
    if bt == 6 as libc::c_int || bt == 5 as libc::c_int
        || bt == 7 as libc::c_int
            && (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope != 0
    {
        let mut sp: *mut *mut Sym = &mut (*s).type_0.ref_0;
        s = *sp;
        *sp = 0 as *mut Sym;
        while !s.is_null() {
            let mut s2: *mut Sym = sym_copy(s, ps);
            *sp = s2;
            sp = &mut (**sp).c2rust_unnamed_0.next;
            sym_copy_ref(s2, ps);
            s = (*s).c2rust_unnamed_0.next;
        }
    }
}
unsafe extern "C" fn external_sym(
    mut v: libc::c_int,
    mut type_0: *mut CType,
    mut r: libc::c_int,
    mut ad: *mut AttributeDef,
) -> *mut Sym {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_find(v);
    while !s.is_null()
        && (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope != 0
    {
        s = (*s).prev_tok;
    }
    if s.is_null() {
        s = global_identifier_push(v, (*type_0).t, 0 as libc::c_int);
        (*s).r = ((*s).r as libc::c_int | r) as libc::c_ushort;
        (*s).a = (*ad).a;
        (*s).c2rust_unnamed_0.asm_label = (*ad).asm_label;
        (*s).type_0.ref_0 = (*type_0).ref_0;
        if !local_stack.is_null() {
            sym_copy_ref(s, &mut global_stack);
        }
    } else {
        patch_storage(s, ad, type_0);
    }
    if !local_stack.is_null() && (*s).type_0.t & 0xf as libc::c_int != 6 as libc::c_int {
        s = sym_copy(s, &mut local_stack);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn save_regs(mut n: libc::c_int) {
    let mut p: *mut SValue = 0 as *mut SValue;
    let mut p1: *mut SValue = 0 as *mut SValue;
    p = _vstack.as_mut_ptr().offset(1 as libc::c_int as isize);
    p1 = vtop.offset(-(n as isize));
    while p <= p1 {
        save_reg((*p).r as libc::c_int);
        p = p.offset(1);
        p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn save_reg(mut r: libc::c_int) {
    save_reg_upstack(r, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn save_reg_upstack(mut r: libc::c_int, mut n: libc::c_int) {
    let mut l: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut p: *mut SValue = 0 as *mut SValue;
    let mut p1: *mut SValue = 0 as *mut SValue;
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    r &= 0x3f as libc::c_int;
    if r >= 0x30 as libc::c_int {
        return;
    }
    if nocode_wanted != 0 {
        return;
    }
    r2 = 0 as libc::c_int;
    l = r2;
    let mut current_block_30: u64;
    p = _vstack.as_mut_ptr().offset(1 as libc::c_int as isize);
    p1 = vtop.offset(-(n as isize));
    while p <= p1 {
        if (*p).r as libc::c_int & 0x3f as libc::c_int == r
            || (*p).r2 as libc::c_int == r
        {
            if l == 0 {
                bt = (*p).type_0.t & 0xf as libc::c_int;
                if bt == 0 as libc::c_int {
                    current_block_30 = 7502529970979898288;
                } else {
                    if (*p).r as libc::c_int & 0x100 as libc::c_int != 0
                        || bt == 6 as libc::c_int
                    {
                        bt = 5 as libc::c_int;
                    }
                    sv.type_0.t = bt;
                    size = type_size(&mut sv.type_0, &mut align);
                    l = get_temp_local_var(size, align, &mut r2);
                    sv
                        .r = (0x32 as libc::c_int | 0x100 as libc::c_int)
                        as libc::c_ushort;
                    sv.c2rust_unnamed.c.i = l as uint64_t;
                    store((*p).r as libc::c_int & 0x3f as libc::c_int, &mut sv);
                    if r == TREG_ST0 as libc::c_int {
                        o(0xd8dd as libc::c_int as libc::c_uint);
                    }
                    if ((*p).r2 as libc::c_int) < 0x30 as libc::c_int
                        && R2_RET(bt) != 0x30 as libc::c_int
                    {
                        sv
                            .c2rust_unnamed
                            .c
                            .i = (sv.c2rust_unnamed.c.i)
                            .wrapping_add(8 as libc::c_int as uint64_t);
                        store((*p).r2 as libc::c_int, &mut sv);
                    }
                    current_block_30 = 13797916685926291137;
                }
            } else {
                current_block_30 = 13797916685926291137;
            }
            match current_block_30 {
                7502529970979898288 => {}
                _ => {
                    if (*p).r as libc::c_int & 0x100 as libc::c_int != 0 {
                        (*p)
                            .r = ((*p).r as libc::c_int
                            & !(0x3f as libc::c_int | 0x8000 as libc::c_int)
                            | 0x31 as libc::c_int) as libc::c_ushort;
                    } else {
                        (*p)
                            .r = (0x100 as libc::c_int | 0x32 as libc::c_int)
                            as libc::c_ushort;
                        (*p).type_0.t &= !(0x40 as libc::c_int);
                    }
                    (*p).c2rust_unnamed_0.sym = 0 as *mut Sym;
                    (*p).r2 = r2 as libc::c_ushort;
                    (*p).c2rust_unnamed.c.i = l as uint64_t;
                }
            }
        }
        p = p.offset(1);
        p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_reg(mut rc: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut p: *mut SValue = 0 as *mut SValue;
    r = 0 as libc::c_int;
    while r < 25 as libc::c_int {
        let mut current_block_5: u64;
        if reg_classes[r as usize] & rc != 0 {
            if nocode_wanted != 0 {
                return r;
            }
            p = _vstack.as_mut_ptr().offset(1 as libc::c_int as isize);
            loop {
                if !(p <= vtop) {
                    current_block_5 = 10886091980245723256;
                    break;
                }
                if (*p).r as libc::c_int & 0x3f as libc::c_int == r
                    || (*p).r2 as libc::c_int == r
                {
                    current_block_5 = 1917311967535052937;
                    break;
                }
                p = p.offset(1);
                p;
            }
            match current_block_5 {
                1917311967535052937 => {}
                _ => return r,
            }
        }
        r += 1;
        r;
    }
    p = _vstack.as_mut_ptr().offset(1 as libc::c_int as isize);
    while p <= vtop {
        's_92: {
            r = (*p).r2 as libc::c_int;
            if !(r < 0x30 as libc::c_int && reg_classes[r as usize] & rc != 0) {
                r = (*p).r as libc::c_int & 0x3f as libc::c_int;
                if !(r < 0x30 as libc::c_int && reg_classes[r as usize] & rc != 0) {
                    break 's_92;
                }
            }
            save_reg(r);
            return r;
        }
        p = p.offset(1);
        p;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_temp_local_var(
    mut size: libc::c_int,
    mut align: libc::c_int,
    mut r2: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut temp_var: *mut temp_local_variable = 0 as *mut temp_local_variable;
    let mut p: *mut SValue = 0 as *mut SValue;
    let mut r: libc::c_int = 0;
    let mut used: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    p = _vstack.as_mut_ptr().offset(1 as libc::c_int as isize);
    while p <= vtop {
        r = (*p).r as libc::c_int & 0x3f as libc::c_int;
        if r == 0x32 as libc::c_int || r == 0x31 as libc::c_int {
            r = (*p).r2 as libc::c_int - (0x30 as libc::c_int + 1 as libc::c_int);
            if r >= 0 as libc::c_int && r < 8 as libc::c_int {
                used |= ((1 as libc::c_int) << r) as libc::c_uint;
            }
        }
        p = p.offset(1);
        p;
    }
    i = 0 as libc::c_int;
    loop {
        if i < nb_temp_local_vars {
            temp_var = &mut *arr_temp_local_vars.as_mut_ptr().offset(i as isize)
                as *mut temp_local_variable;
            if used & ((1 as libc::c_int) << i) as libc::c_uint == 0
                && (*temp_var).size as libc::c_int >= size
                && (*temp_var).align as libc::c_int >= align
            {
                current_block = 10316786355869550362;
                break;
            }
            i += 1;
            i;
        } else {
            loc = loc - size & -align;
            if nb_temp_local_vars < 8 as libc::c_int {
                current_block = 12039483399334584727;
                break;
            } else {
                current_block = 4808432441040389987;
                break;
            }
        }
    }
    match current_block {
        12039483399334584727 => {
            temp_var = &mut *arr_temp_local_vars.as_mut_ptr().offset(i as isize)
                as *mut temp_local_variable;
            (*temp_var).location = loc;
            (*temp_var).size = size as libc::c_short;
            (*temp_var).align = align as libc::c_short;
            nb_temp_local_vars += 1;
            nb_temp_local_vars;
        }
        4808432441040389987 => {
            *r2 = 0x30 as libc::c_int;
            return loc;
        }
        _ => {}
    }
    *r2 = 0x30 as libc::c_int + 1 as libc::c_int + i;
    return (*temp_var).location;
}
unsafe extern "C" fn move_reg(
    mut r: libc::c_int,
    mut s: libc::c_int,
    mut t: libc::c_int,
) {
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    if r != s {
        save_reg(r);
        sv.type_0.t = t;
        sv.type_0.ref_0 = 0 as *mut Sym;
        sv.r = s as libc::c_ushort;
        sv.c2rust_unnamed.c.i = 0 as libc::c_int as uint64_t;
        load(r, &mut sv);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gaddrof() {
    (*vtop).r = ((*vtop).r as libc::c_int & !(0x100 as libc::c_int)) as libc::c_ushort;
    if (*vtop).r as libc::c_int & 0x3f as libc::c_int == 0x31 as libc::c_int {
        (*vtop)
            .r = ((*vtop).r as libc::c_int & !(0x3f as libc::c_int) | 0x32 as libc::c_int
            | 0x100 as libc::c_int) as libc::c_ushort;
    }
}
unsafe extern "C" fn gen_bounded_ptr_add() {
    let mut save: libc::c_int = ((*vtop.offset(-(1 as libc::c_int) as isize)).r
        as libc::c_int & 0x3f as libc::c_int == 0x32 as libc::c_int) as libc::c_int;
    if save != 0 {
        vpushv(&mut *vtop.offset(-(1 as libc::c_int) as isize));
        vrott(3 as libc::c_int);
    }
    vpush_helper_func(TOK___bound_ptr_add as libc::c_int);
    vrott(3 as libc::c_int);
    gfunc_call(2 as libc::c_int);
    vtop = vtop.offset(-(save as isize));
    vpushi(0 as libc::c_int);
    (*vtop).r = (TREG_RAX as libc::c_int | 0x8000 as libc::c_int) as libc::c_ushort;
    if nocode_wanted != 0 {
        return;
    }
    (*vtop)
        .c2rust_unnamed
        .c
        .i = ((*(*(*tcc_state).cur_text_section).reloc).data_offset)
        .wrapping_sub(::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong);
}
unsafe extern "C" fn gen_bounded_ptr_deref() {
    let mut func: Elf64_Addr = 0;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut sym: *mut Sym = 0 as *mut Sym;
    if nocode_wanted != 0 {
        return;
    }
    size = type_size(&mut (*vtop).type_0, &mut align);
    match size {
        1 => {
            func = TOK___bound_ptr_indir1 as libc::c_int as Elf64_Addr;
        }
        2 => {
            func = TOK___bound_ptr_indir2 as libc::c_int as Elf64_Addr;
        }
        4 => {
            func = TOK___bound_ptr_indir4 as libc::c_int as Elf64_Addr;
        }
        8 => {
            func = TOK___bound_ptr_indir8 as libc::c_int as Elf64_Addr;
        }
        12 => {
            func = TOK___bound_ptr_indir12 as libc::c_int as Elf64_Addr;
        }
        16 => {
            func = TOK___bound_ptr_indir16 as libc::c_int as Elf64_Addr;
        }
        _ => return,
    }
    sym = external_helper_sym(func as libc::c_int);
    if (*sym).c2rust_unnamed.c2rust_unnamed.c == 0 {
        put_extern_sym(
            sym,
            0 as *mut Section,
            0 as libc::c_int as Elf64_Addr,
            0 as libc::c_int as libc::c_ulong,
        );
    }
    rel = ((*(*(*tcc_state).cur_text_section).reloc).data)
        .offset((*vtop).c2rust_unnamed.c.i as isize) as *mut Elf64_Rela;
    (*rel)
        .r_info = (((*sym).c2rust_unnamed.c2rust_unnamed.c as Elf64_Xword)
        << 32 as libc::c_int)
        .wrapping_add((*rel).r_info & 0xffffffff as libc::c_uint as Elf64_Xword);
}
unsafe extern "C" fn gbound() {
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    (*vtop).r = ((*vtop).r as libc::c_int & !(0x4000 as libc::c_int)) as libc::c_ushort;
    if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {
        if (*vtop).r as libc::c_int & 0x8000 as libc::c_int == 0 {
            type1 = (*vtop).type_0;
            (*vtop).type_0.t = 5 as libc::c_int;
            gaddrof();
            vpushi(0 as libc::c_int);
            gen_bounded_ptr_add();
            (*vtop)
                .r = ((*vtop).r as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
            (*vtop).type_0 = type1;
        }
        gen_bounded_ptr_deref();
    }
}
#[no_mangle]
pub unsafe extern "C" fn gbound_args(mut nb_args: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut sv: *mut SValue = 0 as *mut SValue;
    i = 1 as libc::c_int;
    while i <= nb_args {
        if (*vtop.offset((1 as libc::c_int - i) as isize)).r as libc::c_int
            & 0x4000 as libc::c_int != 0
        {
            vrotb(i);
            gbound();
            vrott(i);
        }
        i += 1;
        i;
    }
    sv = vtop.offset(-(nb_args as isize));
    if (*sv).r as libc::c_int & 0x200 as libc::c_int != 0 {
        v = (*(*sv).c2rust_unnamed_0.sym).v;
        if v == TOK_setjmp as libc::c_int || v == TOK__setjmp as libc::c_int
            || v == TOK_sigsetjmp as libc::c_int || v == TOK___sigsetjmp as libc::c_int
        {
            vpush_helper_func(TOK___bound_setjmp as libc::c_int);
            vpushv(sv.offset(1 as libc::c_int as isize));
            gfunc_call(1 as libc::c_int);
            func_bound_add_epilog = 1 as libc::c_int;
        }
        if v == TOK_alloca as libc::c_int {
            func_bound_add_epilog = 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn add_local_bounds(mut s: *mut Sym, mut e: *mut Sym) {
    while s != e {
        if !((*s).v == 0
            || (*s).r as libc::c_int & 0x3f as libc::c_int != 0x32 as libc::c_int)
        {
            if (*s).type_0.t & 0x40 as libc::c_int != 0
                || (*s).type_0.t & 0xf as libc::c_int == 7 as libc::c_int
                || ((*s).a).addrtaken() as libc::c_int != 0
            {
                let mut align: libc::c_int = 0;
                let mut size: libc::c_int = type_size(&mut (*s).type_0, &mut align);
                let mut bounds_ptr: *mut Elf64_Addr = section_ptr_add(
                    (*tcc_state).lbounds_section,
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<Elf64_Addr>() as libc::c_ulong,
                        ),
                ) as *mut Elf64_Addr;
                *bounds_ptr
                    .offset(
                        0 as libc::c_int as isize,
                    ) = (*s).c2rust_unnamed.c2rust_unnamed.c as Elf64_Addr;
                *bounds_ptr.offset(1 as libc::c_int as isize) = size as Elf64_Addr;
            }
        }
        s = (*s).prev;
    }
}
unsafe extern "C" fn pop_local_syms(mut b: *mut Sym, mut keep: libc::c_int) {
    if (*tcc_state).do_bounds_check as libc::c_int != 0 && keep == 0
        && (local_scope != 0 || func_var == 0)
    {
        add_local_bounds(local_stack, b);
    }
    if debug_modes != 0 {
        tcc_add_debug_info(tcc_state, (local_scope == 0) as libc::c_int, local_stack, b);
    }
    sym_pop(&mut local_stack, b, keep);
}
unsafe extern "C" fn incr_offset(mut offset: libc::c_int) {
    let mut t: libc::c_int = (*vtop).type_0.t;
    gaddrof();
    (*vtop).type_0.t = 0x800 as libc::c_int | 4 as libc::c_int;
    vpushs(offset as Elf64_Addr);
    gen_op('+' as i32);
    (*vtop).r = ((*vtop).r as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
    (*vtop).type_0.t = t;
}
unsafe extern "C" fn incr_bf_adr(mut o_0: libc::c_int) {
    (*vtop).type_0.t = 1 as libc::c_int | 0x10 as libc::c_int;
    incr_offset(o_0);
}
unsafe extern "C" fn load_packed_bf(
    mut type_0: *mut CType,
    mut bit_pos: libc::c_int,
    mut bit_size: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut o_0: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    save_reg_upstack((*vtop).r as libc::c_int, 1 as libc::c_int);
    vpush64((*type_0).t & 0xf as libc::c_int, 0 as libc::c_int as libc::c_ulonglong);
    bits = 0 as libc::c_int;
    o_0 = bit_pos >> 3 as libc::c_int;
    bit_pos &= 7 as libc::c_int;
    loop {
        vswap();
        incr_bf_adr(o_0);
        vdup();
        n = 8 as libc::c_int - bit_pos;
        if n > bit_size {
            n = bit_size;
        }
        if bit_pos != 0 {
            vpushi(bit_pos);
            gen_op(0x8b as libc::c_int);
            bit_pos = 0 as libc::c_int;
        }
        if n < 8 as libc::c_int {
            vpushi(((1 as libc::c_int) << n) - 1 as libc::c_int);
            gen_op('&' as i32);
        }
        gen_cast(type_0);
        if bits != 0 {
            vpushi(bits);
            gen_op('<' as i32);
        }
        vrotb(3 as libc::c_int);
        gen_op('|' as i32);
        bits += n;
        bit_size -= n;
        o_0 = 1 as libc::c_int;
        if !(bit_size != 0) {
            break;
        }
    }
    vswap();
    vpop();
    if (*type_0).t & 0x10 as libc::c_int == 0 {
        n = (if (*type_0).t & 0xf as libc::c_int == 4 as libc::c_int {
            64 as libc::c_int
        } else {
            32 as libc::c_int
        }) - bits;
        vpushi(n);
        gen_op('<' as i32);
        vpushi(n);
        gen_op('>' as i32);
    }
}
unsafe extern "C" fn store_packed_bf(
    mut bit_pos: libc::c_int,
    mut bit_size: libc::c_int,
) {
    let mut bits: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut o_0: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = ((*vtop).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int) as libc::c_int;
    vswap();
    save_reg_upstack((*vtop).r as libc::c_int, 1 as libc::c_int);
    bits = 0 as libc::c_int;
    o_0 = bit_pos >> 3 as libc::c_int;
    bit_pos &= 7 as libc::c_int;
    loop {
        incr_bf_adr(o_0);
        vswap();
        if c != 0 {
            vdup();
        } else {
            gv_dup();
        };
        vrott(3 as libc::c_int);
        if bits != 0 {
            vpushi(bits);
            gen_op(0x8b as libc::c_int);
        }
        if bit_pos != 0 {
            vpushi(bit_pos);
            gen_op('<' as i32);
        }
        n = 8 as libc::c_int - bit_pos;
        if n > bit_size {
            n = bit_size;
        }
        if n < 8 as libc::c_int {
            m = (((1 as libc::c_int) << n) - 1 as libc::c_int) << bit_pos;
            vpushi(m);
            gen_op('&' as i32);
            vpushv(vtop.offset(-(1 as libc::c_int as isize)));
            vpushi(
                if m & 0x80 as libc::c_int != 0 { !m & 0x7f as libc::c_int } else { !m },
            );
            gen_op('&' as i32);
            gen_op('|' as i32);
        }
        vdup();
        *vtop
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *vtop.offset(-(2 as libc::c_int) as isize);
        vstore();
        vpop();
        bits += n;
        bit_size -= n;
        bit_pos = 0 as libc::c_int;
        o_0 = 1 as libc::c_int;
        if !(bit_size != 0) {
            break;
        }
    }
    vpop();
    vpop();
}
unsafe extern "C" fn adjust_bf(
    mut sv: *mut SValue,
    mut bit_pos: libc::c_int,
    mut bit_size: libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    if ((*sv).type_0.ref_0).is_null() {
        return 0 as libc::c_int;
    }
    t = (*(*sv).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.auxtype;
    if t != -(1 as libc::c_int) && t != 7 as libc::c_int {
        (*sv)
            .type_0
            .t = (*sv).type_0.t & !(0xf as libc::c_int | 0x800 as libc::c_int) | t;
        (*sv).r = ((*sv).r as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn gv(mut rc: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut r_ok: libc::c_int = 0;
    let mut r2_ok: libc::c_int = 0;
    let mut rc2: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut bit_pos: libc::c_int = 0;
    let mut bit_size: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    if (*vtop).type_0.t & 0x80 as libc::c_int != 0 {
        let mut type_0: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        bit_pos = (*vtop).type_0.t >> 20 as libc::c_int & 0x3f as libc::c_int;
        bit_size = (*vtop).type_0.t >> 20 as libc::c_int + 6 as libc::c_int
            & 0x3f as libc::c_int;
        (*vtop)
            .type_0
            .t = ((*vtop).type_0.t as libc::c_uint
            & !(((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint)) as libc::c_int;
        type_0.ref_0 = 0 as *mut Sym;
        type_0.t = (*vtop).type_0.t & 0x10 as libc::c_int;
        if (*vtop).type_0.t & 0xf as libc::c_int == 11 as libc::c_int {
            type_0.t |= 0x10 as libc::c_int;
        }
        r = adjust_bf(vtop, bit_pos, bit_size);
        if (*vtop).type_0.t & 0xf as libc::c_int == 4 as libc::c_int {
            type_0.t |= 4 as libc::c_int;
        } else {
            type_0.t |= 3 as libc::c_int;
        }
        if r == 7 as libc::c_int {
            load_packed_bf(&mut type_0, bit_pos, bit_size);
        } else {
            let mut bits: libc::c_int = if type_0.t & 0xf as libc::c_int
                == 4 as libc::c_int
            {
                64 as libc::c_int
            } else {
                32 as libc::c_int
            };
            gen_cast(&mut type_0);
            vpushi(bits - (bit_pos + bit_size));
            gen_op('<' as i32);
            vpushi(bits - bit_size);
            gen_op('>' as i32);
        }
        r = gv(rc);
    } else {
        if is_float((*vtop).type_0.t) != 0
            && (*vtop).r as libc::c_int & (0x3f as libc::c_int | 0x100 as libc::c_int)
                == 0x30 as libc::c_int
        {
            let mut p: init_params = {
                let mut init = init_params {
                    sec: (*tcc_state).rodata_section,
                    local_offset: 0,
                    flex_array_ref: 0 as *mut Sym,
                };
                init
            };
            let mut offset: libc::c_ulong = 0;
            size = type_size(&mut (*vtop).type_0, &mut align);
            if nocode_wanted > 0 as libc::c_int {
                size = 0 as libc::c_int;
                align = 1 as libc::c_int;
            }
            offset = section_add(p.sec, size as Elf64_Addr, align);
            vpush_ref(&mut (*vtop).type_0, p.sec, offset, size as libc::c_ulong);
            vswap();
            init_putv(&mut p, &mut (*vtop).type_0, offset);
            (*vtop)
                .r = ((*vtop).r as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
        }
        if (*vtop).r as libc::c_int & 0x4000 as libc::c_int != 0 {
            gbound();
        }
        bt = (*vtop).type_0.t & 0xf as libc::c_int;
        rc2 = RC2_TYPE(bt, rc);
        r = (*vtop).r as libc::c_int & 0x3f as libc::c_int;
        r_ok = ((*vtop).r as libc::c_int & 0x100 as libc::c_int == 0
            && r < 0x30 as libc::c_int && reg_classes[r as usize] & rc != 0)
            as libc::c_int;
        r2_ok = (rc2 == 0
            || ((*vtop).r2 as libc::c_int) < 0x30 as libc::c_int
                && reg_classes[(*vtop).r2 as usize] & rc2 != 0) as libc::c_int;
        if r_ok == 0 || r2_ok == 0 {
            if r_ok == 0 {
                if 1 as libc::c_int != 0 && r < 0x30 as libc::c_int
                    && reg_classes[r as usize] & rc != 0 && rc2 == 0
                {
                    save_reg_upstack(r, 1 as libc::c_int);
                } else {
                    r = get_reg(rc);
                }
            }
            if rc2 != 0 {
                let mut current_block_65: u64;
                let mut load_type: libc::c_int = if bt == 14 as libc::c_int {
                    9 as libc::c_int
                } else {
                    0x800 as libc::c_int | 4 as libc::c_int
                };
                let mut original_type: libc::c_int = (*vtop).type_0.t;
                if (*vtop).r as libc::c_int
                    & (0x3f as libc::c_int | 0x100 as libc::c_int) == 0x30 as libc::c_int
                {
                    let mut ll: libc::c_ulonglong = (*vtop).c2rust_unnamed.c.i
                        as libc::c_ulonglong;
                    (*vtop).c2rust_unnamed.c.i = ll as uint64_t;
                    load(r, vtop);
                    (*vtop).r = r as libc::c_ushort;
                    vpushi((ll >> 32 as libc::c_int) as libc::c_int);
                    current_block_65 = 7990025728955927862;
                } else if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {
                    save_reg_upstack((*vtop).r as libc::c_int, 1 as libc::c_int);
                    (*vtop).type_0.t = load_type;
                    load(r, vtop);
                    vdup();
                    (*vtop.offset(-(1 as libc::c_int) as isize)).r = r as libc::c_ushort;
                    incr_offset(8 as libc::c_int);
                    current_block_65 = 7990025728955927862;
                } else {
                    if r_ok == 0 {
                        load(r, vtop);
                    }
                    if r2_ok != 0 && ((*vtop).r2 as libc::c_int) < 0x30 as libc::c_int {
                        current_block_65 = 11148789996967328687;
                    } else {
                        vdup();
                        (*vtop.offset(-(1 as libc::c_int) as isize))
                            .r = r as libc::c_ushort;
                        (*vtop).r = (*vtop.offset(-(1 as libc::c_int) as isize)).r2;
                        current_block_65 = 7990025728955927862;
                    }
                }
                match current_block_65 {
                    7990025728955927862 => {
                        r2 = get_reg(rc2);
                        load(r2, vtop);
                        vpop();
                        (*vtop).r2 = r2 as libc::c_ushort;
                    }
                    _ => {}
                }
                (*vtop).type_0.t = original_type;
            } else {
                if (*vtop).r as libc::c_int == 0x33 as libc::c_int {
                    vset_VT_JMP();
                }
                load(r, vtop);
            }
        }
        (*vtop).r = r as libc::c_ushort;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn gv2(mut rc1: libc::c_int, mut rc2: libc::c_int) {
    if (*vtop).r as libc::c_int != 0x33 as libc::c_int && rc1 <= rc2 {
        vswap();
        gv(rc1);
        vswap();
        gv(rc2);
        if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
            & 0x3f as libc::c_int >= 0x30 as libc::c_int
        {
            vswap();
            gv(rc1);
            vswap();
        }
    } else {
        gv(rc2);
        vswap();
        gv(rc1);
        vswap();
        if (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int
            & 0x3f as libc::c_int >= 0x30 as libc::c_int
        {
            gv(rc2);
        }
    };
}
unsafe extern "C" fn gv_dup() {
    let mut t: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    t = (*vtop).type_0.t;
    rc = RC_TYPE(t);
    gv(rc);
    r = get_reg(rc);
    vdup();
    load(r, vtop);
    (*vtop).r = r as libc::c_ushort;
}
unsafe extern "C" fn value64(mut l1: uint64_t, mut t: libc::c_int) -> uint64_t {
    if t & 0xf as libc::c_int == 4 as libc::c_int
        || 8 as libc::c_int == 8 as libc::c_int
            && t & 0xf as libc::c_int == 5 as libc::c_int
    {
        return l1
    } else if t & 0x10 as libc::c_int != 0 {
        return l1 as uint32_t as uint64_t
    } else {
        return l1 as uint32_t as uint64_t
            | (l1 & 0x80000000 as libc::c_uint as uint64_t).wrapping_neg()
    };
}
unsafe extern "C" fn gen_opic_sdiv(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut x: uint64_t = (if a >> 63 as libc::c_int != 0 {
        a.wrapping_neg()
    } else {
        a
    }) / (if b >> 63 as libc::c_int != 0 { b.wrapping_neg() } else { b });
    return if (a ^ b) >> 63 as libc::c_int != 0 { x.wrapping_neg() } else { x };
}
unsafe extern "C" fn gen_opic_lt(mut a: uint64_t, mut b: uint64_t) -> libc::c_int {
    return ((a ^ (1 as libc::c_int as uint64_t) << 63 as libc::c_int)
        < b ^ (1 as libc::c_int as uint64_t) << 63 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn gen_opic(mut op: libc::c_int) {
    let mut current_block: u64;
    let mut v1: *mut SValue = vtop.offset(-(1 as libc::c_int as isize));
    let mut v2: *mut SValue = vtop;
    let mut t1: libc::c_int = (*v1).type_0.t & 0xf as libc::c_int;
    let mut t2: libc::c_int = (*v2).type_0.t & 0xf as libc::c_int;
    let mut c1: libc::c_int = ((*v1).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int) as libc::c_int;
    let mut c2: libc::c_int = ((*v2).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int) as libc::c_int;
    let mut l1: uint64_t = if c1 != 0 {
        value64((*v1).c2rust_unnamed.c.i, (*v1).type_0.t)
    } else {
        0 as libc::c_int as uint64_t
    };
    let mut l2: uint64_t = if c2 != 0 {
        value64((*v2).c2rust_unnamed.c.i, (*v2).type_0.t)
    } else {
        0 as libc::c_int as uint64_t
    };
    let mut shm: libc::c_int = if t1 == 4 as libc::c_int {
        63 as libc::c_int
    } else {
        31 as libc::c_int
    };
    let mut r: libc::c_int = 0;
    if c1 != 0 && c2 != 0 {
        match op {
            43 => {
                current_block = 9661106225323354372;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            45 => {
                current_block = 15968806681367933537;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            38 => {
                current_block = 1798345118324096392;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            94 => {
                current_block = 5592364819767578823;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            124 => {
                current_block = 2938922248466119513;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            42 => {
                current_block = 8080297887362746266;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            133 | 47 | 37 | 131 | 132 => {
                current_block = 16943022006405211372;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            60 => {
                current_block = 7968383275687041905;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            139 => {
                current_block = 18410362354071502069;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            62 => {
                current_block = 2132803976692972413;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            146 => {
                current_block = 3126526079058646937;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            147 => {
                current_block = 17790244489123001986;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            148 => {
                current_block = 9650218520264151726;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            149 => {
                current_block = 7980019948628414320;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            150 => {
                current_block = 2764888665249125282;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            151 => {
                current_block = 3636825383765150728;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            156 => {
                current_block = 10637694812653694167;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            157 => {
                current_block = 6413219609671322664;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            158 => {
                current_block = 2393856711937439556;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            159 => {
                current_block = 17164146780256253467;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            144 => {
                current_block = 17799175151349409307;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            145 => {
                current_block = 13198121612081791185;
                match current_block {
                    13198121612081791185 => {
                        l1 = (l1 != 0 || l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9661106225323354372 => {
                        l1 = l1.wrapping_add(l2);
                        current_block = 10150597327160359210;
                    }
                    15968806681367933537 => {
                        l1 = l1.wrapping_sub(l2);
                        current_block = 10150597327160359210;
                    }
                    1798345118324096392 => {
                        l1 &= l2;
                        current_block = 10150597327160359210;
                    }
                    5592364819767578823 => {
                        l1 ^= l2;
                        current_block = 10150597327160359210;
                    }
                    2938922248466119513 => {
                        l1 |= l2;
                        current_block = 10150597327160359210;
                    }
                    8080297887362746266 => {
                        l1 = l1 * l2;
                        current_block = 10150597327160359210;
                    }
                    7968383275687041905 => {
                        l1 <<= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    18410362354071502069 => {
                        l1 >>= l2 & shm as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2132803976692972413 => {
                        l1 = if l1 >> 63 as libc::c_int != 0 {
                            !(!l1 >> (l2 & shm as uint64_t))
                        } else {
                            l1 >> (l2 & shm as uint64_t)
                        };
                        current_block = 10150597327160359210;
                    }
                    3126526079058646937 => {
                        l1 = (l1 < l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17790244489123001986 => {
                        l1 = (l1 >= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    9650218520264151726 => {
                        l1 = (l1 == l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    7980019948628414320 => {
                        l1 = (l1 != l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2764888665249125282 => {
                        l1 = (l1 <= l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    3636825383765150728 => {
                        l1 = (l1 > l2) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    10637694812653694167 => {
                        l1 = gen_opic_lt(l1, l2) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    6413219609671322664 => {
                        l1 = (gen_opic_lt(l1, l2) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    2393856711937439556 => {
                        l1 = (gen_opic_lt(l2, l1) == 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17164146780256253467 => {
                        l1 = gen_opic_lt(l2, l1) as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    17799175151349409307 => {
                        l1 = (l1 != 0 && l2 != 0) as libc::c_int as uint64_t;
                        current_block = 10150597327160359210;
                    }
                    _ => {
                        if l2 == 0 as libc::c_int as uint64_t {
                            if nocode_wanted & 0xfff0000 as libc::c_int != 0
                                && nocode_wanted & 0xffff as libc::c_int == 0
                            {
                                _tcc_error(
                                    b"division by zero in constant\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            current_block = 6773336550393273737;
                        } else {
                            match op {
                                37 => {
                                    l1 = l1.wrapping_sub(l2 * gen_opic_sdiv(l1, l2));
                                }
                                131 => {
                                    l1 = l1 / l2;
                                }
                                132 => {
                                    l1 = l1 % l2;
                                }
                                _ => {
                                    l1 = gen_opic_sdiv(l1, l2);
                                }
                            }
                            current_block = 10150597327160359210;
                        }
                    }
                }
                match current_block {
                    6773336550393273737 => {}
                    _ => {
                        (*v1).c2rust_unnamed.c.i = value64(l1, (*v1).type_0.t);
                        (*v1)
                            .r = ((*v1).r as libc::c_int
                            | (*v2).r as libc::c_int & 0x1000 as libc::c_int)
                            as libc::c_ushort;
                        vtop = vtop.offset(-1);
                        vtop;
                        current_block = 1852451392920375136;
                    }
                }
            }
            _ => {
                current_block = 6773336550393273737;
            }
        }
    } else {
        if c1 != 0
            && (op == '+' as i32 || op == '&' as i32 || op == '^' as i32
                || op == '|' as i32 || op == '*' as i32 || op == 0x94 as libc::c_int
                || op == 0x95 as libc::c_int)
        {
            vswap();
            c2 = c1;
            l2 = l1;
        }
        if c1 != 0
            && (l1 == 0 as libc::c_int as uint64_t
                && (op == '<' as i32 || op == 0x8b as libc::c_int || op == '>' as i32)
                || l1 == -(1 as libc::c_int) as uint64_t && op == '>' as i32)
        {
            vpop();
            current_block = 10213293998891106930;
        } else if c2 != 0
            && (l2 == 0 as libc::c_int as uint64_t
                && (op == '&' as i32 || op == '*' as i32)
                || op == '|' as i32
                    && (l2 == -(1 as libc::c_int) as uint64_t
                        || l2 == 0xffffffff as libc::c_uint as uint64_t
                            && t2 != 4 as libc::c_int)
                || l2 == 1 as libc::c_int as uint64_t
                    && (op == '%' as i32 || op == 0x84 as libc::c_int))
        {
            if l2 == 1 as libc::c_int as uint64_t {
                (*vtop).c2rust_unnamed.c.i = 0 as libc::c_int as uint64_t;
            }
            vswap();
            vtop = vtop.offset(-1);
            vtop;
            current_block = 10213293998891106930;
        } else if c2 != 0
            && ((op == '*' as i32 || op == '/' as i32 || op == 0x83 as libc::c_int
                || op == 0x85 as libc::c_int) && l2 == 1 as libc::c_int as uint64_t
                || (op == '+' as i32 || op == '-' as i32 || op == '|' as i32
                    || op == '^' as i32 || op == '<' as i32 || op == 0x8b as libc::c_int
                    || op == '>' as i32) && l2 == 0 as libc::c_int as uint64_t
                || op == '&' as i32
                    && (l2 == -(1 as libc::c_int) as uint64_t
                        || l2 == 0xffffffff as libc::c_uint as uint64_t
                            && t2 != 4 as libc::c_int))
        {
            vtop = vtop.offset(-1);
            vtop;
            current_block = 10213293998891106930;
        } else if c2 != 0
            && (op == '*' as i32 || op == 0x85 as libc::c_int
                || op == 0x83 as libc::c_int || op == 0x84 as libc::c_int)
        {
            if l2 > 0 as libc::c_int as uint64_t
                && l2 & l2.wrapping_sub(1 as libc::c_int as uint64_t)
                    == 0 as libc::c_int as uint64_t
            {
                let mut n: libc::c_int = -(1 as libc::c_int);
                if op == 0x84 as libc::c_int {
                    (*vtop)
                        .c2rust_unnamed
                        .c
                        .i = l2.wrapping_sub(1 as libc::c_int as uint64_t);
                    op = '&' as i32;
                } else {
                    while l2 != 0 {
                        l2 >>= 1 as libc::c_int;
                        n += 1;
                        n;
                    }
                    (*vtop).c2rust_unnamed.c.i = n as uint64_t;
                    if op == '*' as i32 {
                        op = '<' as i32;
                    } else if op == 0x85 as libc::c_int {
                        op = '>' as i32;
                    } else {
                        op = 0x8b as libc::c_int;
                    }
                }
                current_block = 6773336550393273737;
            } else {
                current_block = 6773336550393273737;
            }
        } else if c2 != 0 && (op == '+' as i32 || op == '-' as i32)
            && {
                r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
                    & (0x3f as libc::c_int | 0x100 as libc::c_int
                        | 0x200 as libc::c_int);
                r == 0x30 as libc::c_int | 0x200 as libc::c_int
                    || r == 0x32 as libc::c_int
            }
        {
            if op == '-' as i32 {
                l2 = l2.wrapping_neg();
            }
            l2 = l2
                .wrapping_add(
                    (*vtop.offset(-(1 as libc::c_int) as isize)).c2rust_unnamed.c.i,
                );
            if l2 as libc::c_int as uint64_t != l2 {
                current_block = 6773336550393273737;
            } else {
                vtop = vtop.offset(-1);
                vtop;
                (*vtop).c2rust_unnamed.c.i = l2;
                current_block = 10213293998891106930;
            }
        } else {
            current_block = 6773336550393273737;
        }
    }
    match current_block {
        6773336550393273737 => {
            if t1 == 4 as libc::c_int || t2 == 4 as libc::c_int
                || 8 as libc::c_int == 8 as libc::c_int
                    && (t1 == 5 as libc::c_int || t2 == 5 as libc::c_int)
            {
                gen_opl(op);
            } else {
                gen_opi(op);
            }
            current_block = 10213293998891106930;
        }
        _ => {}
    }
    match current_block {
        10213293998891106930 => {
            if (*vtop).r as libc::c_int == 0x30 as libc::c_int {
                (*vtop)
                    .r = ((*vtop).r as libc::c_int | 0x1000 as libc::c_int)
                    as libc::c_ushort;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn gen_opif(mut op: libc::c_int) {
    let mut current_block: u64;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut v1: *mut SValue = 0 as *mut SValue;
    let mut v2: *mut SValue = 0 as *mut SValue;
    let mut f1: f128::f128 = f128::f128::ZERO;
    let mut f2: f128::f128 = f128::f128::ZERO;
    v1 = vtop.offset(-(1 as libc::c_int as isize));
    v2 = vtop;
    if op == 0x81 as libc::c_int {
        v1 = v2;
    }
    bt = (*v1).type_0.t & 0xf as libc::c_int;
    c1 = ((*v1).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int) as libc::c_int;
    c2 = ((*v2).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int) as libc::c_int;
    if c1 != 0 && c2 != 0 {
        if bt == 8 as libc::c_int {
            f1 = f128::f128::new((*v1).c2rust_unnamed.c.f);
            f2 = f128::f128::new((*v2).c2rust_unnamed.c.f);
        } else if bt == 9 as libc::c_int {
            f1 = f128::f128::new((*v1).c2rust_unnamed.c.d);
            f2 = f128::f128::new((*v2).c2rust_unnamed.c.d);
        } else {
            f1 = (*v1).c2rust_unnamed.c.ld;
            f2 = (*v2).c2rust_unnamed.c.ld;
        }
        if !(ieee_finite(f1.to_f64().unwrap()) != 0
            || ieee_finite(f2.to_f64().unwrap()) == 0)
            && nocode_wanted & 0xfff0000 as libc::c_int == 0
        {
            current_block = 17238176565185867311;
        } else {
            match op {
                43 => {
                    current_block = 9800186729993729775;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                45 => {
                    current_block = 13211970435926734750;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                42 => {
                    current_block = 3519527195531761551;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                47 => {
                    current_block = 3301552899506303830;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                129 => {
                    current_block = 12781635402893015640;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                148 => {
                    current_block = 5642914718990822444;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                149 => {
                    current_block = 14058446606128643901;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                156 => {
                    current_block = 6796999155919360785;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                157 => {
                    current_block = 9809622310318674422;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                158 => {
                    current_block = 7709902894200659361;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                159 => {
                    current_block = 15479678558367822412;
                    match current_block {
                        15479678558367822412 => {
                            i = (f1 > f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9800186729993729775 => {
                            f1 += f2;
                            current_block = 18435049525520518667;
                        }
                        13211970435926734750 => {
                            f1 -= f2;
                            current_block = 18435049525520518667;
                        }
                        3519527195531761551 => {
                            f1 *= f2;
                            current_block = 18435049525520518667;
                        }
                        12781635402893015640 => {
                            f1 = -f1;
                            current_block = 13513063113659714087;
                        }
                        5642914718990822444 => {
                            i = (f1 == f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        14058446606128643901 => {
                            i = (f1 != f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        6796999155919360785 => {
                            i = (f1 < f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        9809622310318674422 => {
                            i = (f1 >= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        7709902894200659361 => {
                            i = (f1 <= f2) as libc::c_int;
                            current_block = 16561699765516985451;
                        }
                        _ => {
                            if f2 == f128::f128::new(0.0f64) {
                                let mut x1: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut x2: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                let mut y: C2RustUnnamed_10 = C2RustUnnamed_10 { f: 0. };
                                if nocode_wanted & 0xfff0000 as libc::c_int == 0 {
                                    current_block = 17238176565185867311;
                                } else {
                                    x1.f = f1.to_f32().unwrap();
                                    x2.f = f2.to_f32().unwrap();
                                    if f1 == f128::f128::new(0.0f64) {
                                        y.u = 0x7fc00000 as libc::c_int as libc::c_uint;
                                    } else {
                                        y.u = 0x7f800000 as libc::c_int as libc::c_uint;
                                    }
                                    y.u |= (x1.u ^ x2.u) & 0x80000000 as libc::c_uint;
                                    f1 = f128::f128::new(y.f);
                                    current_block = 18435049525520518667;
                                }
                            } else {
                                f1 /= f2;
                                current_block = 18435049525520518667;
                            }
                        }
                    }
                    match current_block {
                        17238176565185867311 => {}
                        _ => {
                            match current_block {
                                16561699765516985451 => {
                                    vtop = vtop.offset(-(2 as libc::c_int as isize));
                                    vpushi(i);
                                    return;
                                }
                                18435049525520518667 => {
                                    vtop = vtop.offset(-1);
                                    vtop;
                                }
                                _ => {}
                            }
                            if bt == 8 as libc::c_int {
                                (*v1).c2rust_unnamed.c.f = f1.to_f32().unwrap();
                            } else if bt == 9 as libc::c_int {
                                (*v1).c2rust_unnamed.c.d = f1.to_f64().unwrap();
                            } else {
                                (*v1).c2rust_unnamed.c.ld = f1;
                            }
                            current_block = 9353995356876505083;
                        }
                    }
                }
                _ => {
                    current_block = 17238176565185867311;
                }
            }
        }
    } else {
        current_block = 17238176565185867311;
    }
    match current_block {
        17238176565185867311 => {
            if op == 0x81 as libc::c_int {
                gen_opf(op);
            } else {
                gen_opf(op);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn type_to_str(
    mut buf: *mut libc::c_char,
    mut buf_size: libc::c_int,
    mut type_0: *mut CType,
    mut varstr: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut bt: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut sa: *mut Sym = 0 as *mut Sym;
    let mut buf1: [libc::c_char; 256] = [0; 256];
    let mut tstr: *const libc::c_char = 0 as *const libc::c_char;
    t = (*type_0).t;
    bt = t & 0xf as libc::c_int;
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if t & 0x1000 as libc::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"extern \0" as *const u8 as *const libc::c_char,
        );
    }
    if t & 0x2000 as libc::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"static \0" as *const u8 as *const libc::c_char,
        );
    }
    if t & 0x4000 as libc::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"typedef \0" as *const u8 as *const libc::c_char,
        );
    }
    if t & 0x8000 as libc::c_int != 0 {
        pstrcat(
            buf,
            buf_size as size_t,
            b"inline \0" as *const u8 as *const libc::c_char,
        );
    }
    if bt != 5 as libc::c_int {
        if t & 0x200 as libc::c_int != 0 {
            pstrcat(
                buf,
                buf_size as size_t,
                b"volatile \0" as *const u8 as *const libc::c_char,
            );
        }
        if t & 0x100 as libc::c_int != 0 {
            pstrcat(
                buf,
                buf_size as size_t,
                b"const \0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if t & 0x20 as libc::c_int != 0 && bt == 1 as libc::c_int
        || t & 0x10 as libc::c_int != 0
            && (bt == 2 as libc::c_int || bt == 3 as libc::c_int
                || bt == 4 as libc::c_int)
            && !(t as libc::c_uint
                & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint)
                == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint)
    {
        pstrcat(
            buf,
            buf_size as size_t,
            if t & 0x10 as libc::c_int != 0 {
                b"unsigned \0" as *const u8 as *const libc::c_char
            } else {
                b"signed \0" as *const u8 as *const libc::c_char
            },
        );
    }
    buf_size = (buf_size as libc::c_ulong).wrapping_sub(strlen(buf)) as libc::c_int
        as libc::c_int;
    buf = buf.offset(strlen(buf) as isize);
    match bt {
        0 => {
            tstr = b"void\0" as *const u8 as *const libc::c_char;
            current_block = 13942576193328432622;
        }
        11 => {
            tstr = b"_Bool\0" as *const u8 as *const libc::c_char;
            current_block = 13942576193328432622;
        }
        1 => {
            tstr = b"char\0" as *const u8 as *const libc::c_char;
            current_block = 13942576193328432622;
        }
        2 => {
            tstr = b"short\0" as *const u8 as *const libc::c_char;
            current_block = 13942576193328432622;
        }
        3 => {
            tstr = b"int\0" as *const u8 as *const libc::c_char;
            current_block = 16597039226324776924;
        }
        4 => {
            tstr = b"long long\0" as *const u8 as *const libc::c_char;
            current_block = 16597039226324776924;
        }
        8 => {
            tstr = b"float\0" as *const u8 as *const libc::c_char;
            current_block = 13942576193328432622;
        }
        9 => {
            tstr = b"double\0" as *const u8 as *const libc::c_char;
            if t & 0x800 as libc::c_int == 0 {
                current_block = 13942576193328432622;
            } else {
                current_block = 12345493333541446586;
            }
        }
        10 => {
            current_block = 12345493333541446586;
        }
        7 => {
            tstr = b"struct \0" as *const u8 as *const libc::c_char;
            if t as libc::c_uint
                & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint
                    | 0xf as libc::c_int as libc::c_uint)
                == ((1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int)
                    as libc::c_uint
            {
                tstr = b"union \0" as *const u8 as *const libc::c_char;
            }
            current_block = 1292137934267012881;
        }
        6 => {
            s = (*type_0).ref_0;
            buf1[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            if !varstr.is_null() && '*' as i32 == *varstr as libc::c_int {
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"(\0" as *const u8 as *const libc::c_char,
                );
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    varstr,
                );
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b")\0" as *const u8 as *const libc::c_char,
                );
            }
            pstrcat(
                buf1.as_mut_ptr(),
                buf_size as size_t,
                b"(\0" as *const u8 as *const libc::c_char,
            );
            sa = (*s).c2rust_unnamed_0.next;
            while !sa.is_null() {
                let mut buf2: [libc::c_char; 256] = [0; 256];
                type_to_str(
                    buf2.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as libc::c_int,
                    &mut (*sa).type_0,
                    0 as *const libc::c_char,
                );
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    buf2.as_mut_ptr(),
                );
                sa = (*sa).c2rust_unnamed_0.next;
                if !sa.is_null() {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        b", \0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            if ((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
                as libc::c_int == 3 as libc::c_int
            {
                pstrcat(
                    buf1.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b", ...\0" as *const u8 as *const libc::c_char,
                );
            }
            pstrcat(
                buf1.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b")\0" as *const u8 as *const libc::c_char,
            );
            type_to_str(buf, buf_size, &mut (*s).type_0, buf1.as_mut_ptr());
            current_block = 17239133558811367971;
        }
        5 => {
            s = (*type_0).ref_0;
            if t & (0x40 as libc::c_int | 0x400 as libc::c_int) != 0 {
                if !varstr.is_null() && '*' as i32 == *varstr as libc::c_int {
                    snprintf(
                        buf1.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        b"(%s)[%d]\0" as *const u8 as *const libc::c_char,
                        varstr,
                        (*s).c2rust_unnamed.c2rust_unnamed.c,
                    );
                } else {
                    snprintf(
                        buf1.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        b"%s[%d]\0" as *const u8 as *const libc::c_char,
                        if !varstr.is_null() {
                            varstr
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        (*s).c2rust_unnamed.c2rust_unnamed.c,
                    );
                }
                type_to_str(buf, buf_size, &mut (*s).type_0, buf1.as_mut_ptr());
            } else {
                pstrcpy(
                    buf1.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"*\0" as *const u8 as *const libc::c_char,
                );
                if t & 0x100 as libc::c_int != 0 {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        buf_size as size_t,
                        b"const \0" as *const u8 as *const libc::c_char,
                    );
                }
                if t & 0x200 as libc::c_int != 0 {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        buf_size as size_t,
                        b"volatile \0" as *const u8 as *const libc::c_char,
                    );
                }
                if !varstr.is_null() {
                    pstrcat(
                        buf1.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        varstr,
                    );
                }
                type_to_str(buf, buf_size, &mut (*s).type_0, buf1.as_mut_ptr());
            }
            current_block = 17239133558811367971;
        }
        _ => {
            current_block = 7858101417678297991;
        }
    }
    match current_block {
        16597039226324776924 => {
            if t & 0x800 as libc::c_int != 0 {
                tstr = b"long\0" as *const u8 as *const libc::c_char;
            }
            if !(t as libc::c_uint
                & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint)
                == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint)
            {
                current_block = 13942576193328432622;
            } else {
                tstr = b"enum \0" as *const u8 as *const libc::c_char;
                current_block = 1292137934267012881;
            }
        }
        12345493333541446586 => {
            tstr = b"long double\0" as *const u8 as *const libc::c_char;
            current_block = 13942576193328432622;
        }
        _ => {}
    }
    match current_block {
        1292137934267012881 => {
            pstrcat(buf, buf_size as size_t, tstr);
            v = (*(*type_0).ref_0).v & !(0x40000000 as libc::c_int);
            if v >= 0x10000000 as libc::c_int {
                pstrcat(
                    buf,
                    buf_size as size_t,
                    b"<anonymous>\0" as *const u8 as *const libc::c_char,
                );
            } else {
                pstrcat(buf, buf_size as size_t, get_tok_str(v, 0 as *mut CValue));
            }
            current_block = 7858101417678297991;
        }
        13942576193328432622 => {
            pstrcat(buf, buf_size as size_t, tstr);
            current_block = 7858101417678297991;
        }
        _ => {}
    }
    match current_block {
        7858101417678297991 => {
            if !varstr.is_null() {
                pstrcat(
                    buf,
                    buf_size as size_t,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                pstrcat(buf, buf_size as size_t, varstr);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn type_incompatibility_error(
    mut st: *mut CType,
    mut dt: *mut CType,
    mut fmt: *const libc::c_char,
) {
    let mut buf1: [libc::c_char; 256] = [0; 256];
    let mut buf2: [libc::c_char; 256] = [0; 256];
    type_to_str(
        buf1.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        st,
        0 as *const libc::c_char,
    );
    type_to_str(
        buf2.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        dt,
        0 as *const libc::c_char,
    );
    _tcc_error(fmt, buf1.as_mut_ptr(), buf2.as_mut_ptr());
}
unsafe extern "C" fn type_incompatibility_warning(
    mut st: *mut CType,
    mut dt: *mut CType,
    mut fmt: *const libc::c_char,
) {
    let mut buf1: [libc::c_char; 256] = [0; 256];
    let mut buf2: [libc::c_char; 256] = [0; 256];
    type_to_str(
        buf1.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        st,
        0 as *const libc::c_char,
    );
    type_to_str(
        buf2.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        dt,
        0 as *const libc::c_char,
    );
    _tcc_warning(fmt, buf1.as_mut_ptr(), buf2.as_mut_ptr());
}
unsafe extern "C" fn pointed_size(mut type_0: *mut CType) -> libc::c_int {
    let mut align: libc::c_int = 0;
    return type_size(pointed_type(type_0), &mut align);
}
#[inline]
unsafe extern "C" fn is_null_pointer(mut p: *mut SValue) -> libc::c_int {
    if (*p).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
            | 0x1000 as libc::c_int) != 0x30 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return ((*p).type_0.t & 0xf as libc::c_int == 3 as libc::c_int
        && (*p).c2rust_unnamed.c.i as uint32_t == 0 as libc::c_int as uint32_t
        || (*p).type_0.t & 0xf as libc::c_int == 4 as libc::c_int
            && (*p).c2rust_unnamed.c.i == 0 as libc::c_int as uint64_t
        || (*p).type_0.t & 0xf as libc::c_int == 5 as libc::c_int
            && (if 8 as libc::c_int == 4 as libc::c_int {
                ((*p).c2rust_unnamed.c.i as uint32_t == 0 as libc::c_int as uint32_t)
                    as libc::c_int
            } else {
                ((*p).c2rust_unnamed.c.i == 0 as libc::c_int as uint64_t) as libc::c_int
            }) != 0
            && (*pointed_type(&mut (*p).type_0)).t & 0xf as libc::c_int
                == 0 as libc::c_int
            && 0 as libc::c_int
                == (*pointed_type(&mut (*p).type_0)).t
                    & (0x100 as libc::c_int | 0x200 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn is_compatible_func(
    mut type1: *mut CType,
    mut type2: *mut CType,
) -> libc::c_int {
    let mut s1: *mut Sym = 0 as *mut Sym;
    let mut s2: *mut Sym = 0 as *mut Sym;
    s1 = (*type1).ref_0;
    s2 = (*type2).ref_0;
    if ((*s1).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_call() as libc::c_int
        != ((*s2).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_call()
            as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if ((*s1).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type() as libc::c_int
        != ((*s2).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
            as libc::c_int
        && ((*s1).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
            as libc::c_int != 2 as libc::c_int
        && ((*s2).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
            as libc::c_int != 2 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    loop {
        if is_compatible_unqualified_types(&mut (*s1).type_0, &mut (*s2).type_0) == 0 {
            return 0 as libc::c_int;
        }
        if ((*s1).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
            as libc::c_int == 2 as libc::c_int
            || ((*s2).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
                as libc::c_int == 2 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        s1 = (*s1).c2rust_unnamed_0.next;
        s2 = (*s2).c2rust_unnamed_0.next;
        if s1.is_null() {
            return s2.is_null() as libc::c_int;
        }
        if s2.is_null() {
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn compare_types(
    mut type1: *mut CType,
    mut type2: *mut CType,
    mut unqualified: libc::c_int,
) -> libc::c_int {
    let mut bt1: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    if (*type1).t as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
    {
        if (*type2).t as libc::c_uint
            & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint)
            == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
        {
            return ((*type1).ref_0 == (*type2).ref_0) as libc::c_int;
        }
        type1 = &mut (*(*type1).ref_0).type_0;
    } else if (*type2).t as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
    {
        type2 = &mut (*(*type2).ref_0).type_0;
    }
    t1 = ((*type1).t as libc::c_uint
        & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
            | 0x8000 as libc::c_int) as libc::c_uint
            | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint))) as libc::c_int;
    t2 = ((*type2).t as libc::c_uint
        & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
            | 0x8000 as libc::c_int) as libc::c_uint
            | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint))) as libc::c_int;
    if unqualified != 0 {
        t1 &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
        t2 &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
    }
    if t1 & 0xf as libc::c_int != 1 as libc::c_int {
        t1 &= !(0x20 as libc::c_int);
        t2 &= !(0x20 as libc::c_int);
    }
    if t1 != t2 {
        return 0 as libc::c_int;
    }
    if t1 & 0x40 as libc::c_int != 0
        && !((*(*type1).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int
            || (*(*type2).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int
            || (*(*type1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                == (*(*type2).ref_0).c2rust_unnamed.c2rust_unnamed.c)
    {
        return 0 as libc::c_int;
    }
    bt1 = t1 & 0xf as libc::c_int;
    if bt1 == 5 as libc::c_int {
        type1 = pointed_type(type1);
        type2 = pointed_type(type2);
        return is_compatible_types(type1, type2);
    } else if bt1 == 7 as libc::c_int {
        return ((*type1).ref_0 == (*type2).ref_0) as libc::c_int
    } else if bt1 == 6 as libc::c_int {
        return is_compatible_func(type1, type2)
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn combine_types(
    mut dest: *mut CType,
    mut op1: *mut SValue,
    mut op2: *mut SValue,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut type1: *mut CType = 0 as *mut CType;
    let mut type2: *mut CType = 0 as *mut CType;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut bt1: libc::c_int = 0;
    let mut bt2: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    if op == 'S' as i32 {
        op2 = op1;
    }
    type1 = &mut (*op1).type_0;
    type2 = &mut (*op2).type_0;
    t1 = (*type1).t;
    t2 = (*type2).t;
    bt1 = t1 & 0xf as libc::c_int;
    bt2 = t2 & 0xf as libc::c_int;
    type_0.t = 0 as libc::c_int;
    type_0.ref_0 = 0 as *mut Sym;
    if bt1 == 0 as libc::c_int || bt2 == 0 as libc::c_int {
        ret = if op == '?' as i32 { 1 as libc::c_int } else { 0 as libc::c_int };
        type_0.t = 0 as libc::c_int;
    } else if bt1 == 5 as libc::c_int || bt2 == 5 as libc::c_int {
        if op == '+' as i32 {
            if is_integer_btype(if bt1 == 5 as libc::c_int { bt2 } else { bt1 }) == 0 {
                ret = 0 as libc::c_int;
            }
        } else if is_null_pointer(op2) != 0 {
            type_0 = *type1;
        } else if is_null_pointer(op1) != 0 {
            type_0 = *type2;
        } else if bt1 != bt2 {
            if (op == '?' as i32 || op == 'C' as i32)
                && (is_integer_btype(bt1) != 0 || is_integer_btype(bt2) != 0)
            {
                _tcc_warning(
                    b"pointer/integer mismatch in %s\0" as *const u8
                        as *const libc::c_char,
                    if op == '?' as i32 {
                        b"conditional expression\0" as *const u8 as *const libc::c_char
                    } else {
                        b"comparison\0" as *const u8 as *const libc::c_char
                    },
                );
            } else if op != '-' as i32 || is_integer_btype(bt2) == 0 {
                ret = 0 as libc::c_int;
            }
            type_0 = *if bt1 == 5 as libc::c_int { type1 } else { type2 };
        } else {
            let mut pt1: *mut CType = pointed_type(type1);
            let mut pt2: *mut CType = pointed_type(type2);
            let mut pbt1: libc::c_int = (*pt1).t & 0xf as libc::c_int;
            let mut pbt2: libc::c_int = (*pt2).t & 0xf as libc::c_int;
            let mut newquals: libc::c_int = 0;
            let mut copied: libc::c_int = 0 as libc::c_int;
            if pbt1 != 0 as libc::c_int && pbt2 != 0 as libc::c_int
                && compare_types(pt1, pt2, 1 as libc::c_int) == 0
            {
                if op != '?' as i32 && op != 'C' as i32 {
                    ret = 0 as libc::c_int;
                } else {
                    type_incompatibility_warning(
                        type1,
                        type2,
                        if op == '?' as i32 {
                            b"pointer type mismatch in conditional expression ('%s' and '%s')\0"
                                as *const u8 as *const libc::c_char
                        } else {
                            b"pointer type mismatch in comparison('%s' and '%s')\0"
                                as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
            if op == '?' as i32 {
                type_0 = *if pbt1 == 0 as libc::c_int { type1 } else { type2 };
                newquals = ((*pt1).t | (*pt2).t)
                    & (0x100 as libc::c_int | 0x200 as libc::c_int);
                if !(*pointed_type(&mut type_0)).t
                    & (0x100 as libc::c_int | 0x200 as libc::c_int) & newquals != 0
                {
                    type_0
                        .ref_0 = sym_push(
                        0x20000000 as libc::c_int,
                        &mut (*type_0.ref_0).type_0,
                        0 as libc::c_int,
                        (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c,
                    );
                    copied = 1 as libc::c_int;
                    (*pointed_type(&mut type_0)).t |= newquals;
                }
                if (*pt1).t & 0x40 as libc::c_int != 0
                    && (*pt2).t & 0x40 as libc::c_int != 0
                    && (*(*pointed_type(&mut type_0)).ref_0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c < 0 as libc::c_int
                    && ((*(*pt1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                        > 0 as libc::c_int
                        || (*(*pt2).ref_0).c2rust_unnamed.c2rust_unnamed.c
                            > 0 as libc::c_int)
                {
                    if copied == 0 {
                        type_0
                            .ref_0 = sym_push(
                            0x20000000 as libc::c_int,
                            &mut (*type_0.ref_0).type_0,
                            0 as libc::c_int,
                            (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c,
                        );
                    }
                    let ref mut fresh2 = (*pointed_type(&mut type_0)).ref_0;
                    *fresh2 = sym_push(
                        0x20000000 as libc::c_int,
                        &mut (*(*(pointed_type
                            as unsafe extern "C" fn(
                                *mut CType,
                            ) -> *mut CType)(&mut type_0))
                            .ref_0)
                            .type_0,
                        0 as libc::c_int,
                        (*(*pointed_type(&mut type_0)).ref_0)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .c,
                    );
                    (*(*pointed_type(&mut type_0)).ref_0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c = if (0 as libc::c_int)
                        < (*(*pt1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                    {
                        (*(*pt1).ref_0).c2rust_unnamed.c2rust_unnamed.c
                    } else {
                        (*(*pt2).ref_0).c2rust_unnamed.c2rust_unnamed.c
                    };
                }
            }
        }
        if op == 'C' as i32 {
            type_0.t = 0x800 as libc::c_int | 4 as libc::c_int | 0x10 as libc::c_int;
        }
    } else if bt1 == 7 as libc::c_int || bt2 == 7 as libc::c_int {
        if op != '?' as i32 || compare_types(type1, type2, 1 as libc::c_int) == 0 {
            ret = 0 as libc::c_int;
        }
        type_0 = *type1;
    } else if is_float(bt1) != 0 || is_float(bt2) != 0 {
        if bt1 == 10 as libc::c_int || bt2 == 10 as libc::c_int {
            type_0.t = 10 as libc::c_int;
        } else if bt1 == 9 as libc::c_int || bt2 == 9 as libc::c_int {
            type_0.t = 9 as libc::c_int;
        } else {
            type_0.t = 8 as libc::c_int;
        }
    } else if bt1 == 4 as libc::c_int || bt2 == 4 as libc::c_int {
        type_0.t = 4 as libc::c_int | 0x800 as libc::c_int;
        if bt1 == 4 as libc::c_int {
            type_0.t &= t1;
        }
        if bt2 == 4 as libc::c_int {
            type_0.t &= t2;
        }
        if t1 & (0xf as libc::c_int | 0x10 as libc::c_int | 0x80 as libc::c_int)
            == 4 as libc::c_int | 0x10 as libc::c_int
            || t2 & (0xf as libc::c_int | 0x10 as libc::c_int | 0x80 as libc::c_int)
                == 4 as libc::c_int | 0x10 as libc::c_int
        {
            type_0.t |= 0x10 as libc::c_int;
        }
    } else {
        type_0.t = 3 as libc::c_int | 0x800 as libc::c_int & (t1 | t2);
        if t1 & (0xf as libc::c_int | 0x10 as libc::c_int | 0x80 as libc::c_int)
            == 3 as libc::c_int | 0x10 as libc::c_int
            || t2 & (0xf as libc::c_int | 0x10 as libc::c_int | 0x80 as libc::c_int)
                == 3 as libc::c_int | 0x10 as libc::c_int
        {
            type_0.t |= 0x10 as libc::c_int;
        }
    }
    if !dest.is_null() {
        *dest = type_0;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn gen_op(mut op: libc::c_int) {
    let mut current_block: u64;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut bt1: libc::c_int = 0;
    let mut bt2: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut combtype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut op_class: libc::c_int = op;
    if op == 0x8b as libc::c_int || op == '>' as i32 || op == '<' as i32 {
        op_class = 'S' as i32;
    } else if op >= 0x90 as libc::c_int && op <= 0x9f as libc::c_int {
        op_class = 'C' as i32;
    }
    loop {
        t1 = (*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t;
        t2 = (*vtop.offset(0 as libc::c_int as isize)).type_0.t;
        bt1 = t1 & 0xf as libc::c_int;
        bt2 = t2 & 0xf as libc::c_int;
        if !(bt1 == 6 as libc::c_int || bt2 == 6 as libc::c_int) {
            break;
        }
        if bt2 == 6 as libc::c_int {
            mk_pointer(&mut (*vtop).type_0);
            gaddrof();
        }
        if bt1 == 6 as libc::c_int {
            vswap();
            mk_pointer(&mut (*vtop).type_0);
            gaddrof();
            vswap();
        }
    }
    if !(combine_types(
        &mut combtype,
        vtop.offset(-(1 as libc::c_int as isize)),
        vtop,
        op_class,
    ) == 0)
    {
        if bt1 == 5 as libc::c_int || bt2 == 5 as libc::c_int {
            let mut align: libc::c_int = 0;
            if op_class == 'C' as i32 {
                current_block = 10593934195135161506;
            } else if bt1 == 5 as libc::c_int && bt2 == 5 as libc::c_int {
                if op != '-' as i32 {
                    current_block = 2879981694389155144;
                } else {
                    vpush_type_size(
                        pointed_type(
                            &mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0,
                        ),
                        &mut align,
                    );
                    (*vtop).type_0.t &= !(0x10 as libc::c_int);
                    vrott(3 as libc::c_int);
                    gen_opic(op);
                    (*vtop).type_0.t = 0x800 as libc::c_int | 4 as libc::c_int;
                    vswap();
                    gen_op(0x85 as libc::c_int);
                    current_block = 15462640364611497761;
                }
            } else if op != '-' as i32 && op != '+' as i32 {
                current_block = 2879981694389155144;
            } else {
                if bt2 == 5 as libc::c_int {
                    vswap();
                    t = t1;
                    t1 = t2;
                    t2 = t;
                    bt2 = bt1;
                }
                type1 = (*vtop.offset(-(1 as libc::c_int) as isize)).type_0;
                vpush_type_size(
                    pointed_type(
                        &mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0,
                    ),
                    &mut align,
                );
                gen_op('*' as i32);
                if (*tcc_state).do_bounds_check as libc::c_int != 0
                    && nocode_wanted & 0xfff0000 as libc::c_int == 0
                {
                    if op == '-' as i32 {
                        vpushi(0 as libc::c_int);
                        vswap();
                        gen_op('-' as i32);
                    }
                    gen_bounded_ptr_add();
                } else {
                    gen_opic(op);
                }
                type1.t &= !(0x40 as libc::c_int | 0x400 as libc::c_int);
                (*vtop).type_0 = type1;
                current_block = 15462640364611497761;
            }
        } else if is_float(combtype.t) != 0 && op != '+' as i32 && op != '-' as i32
            && op != '*' as i32 && op != '/' as i32 && op_class != 'C' as i32
        {
            current_block = 2879981694389155144;
        } else {
            current_block = 10593934195135161506;
        }
        match current_block {
            2879981694389155144 => {}
            _ => {
                match current_block {
                    10593934195135161506 => {
                        t2 = combtype.t;
                        t = t2;
                        if op_class == 'S' as i32 {
                            t2 = 3 as libc::c_int;
                        }
                        if t & 0x10 as libc::c_int != 0 {
                            if op == '>' as i32 {
                                op = 0x8b as libc::c_int;
                            } else if op == '/' as i32 {
                                op = 0x83 as libc::c_int;
                            } else if op == '%' as i32 {
                                op = 0x84 as libc::c_int;
                            } else if op == 0x9c as libc::c_int {
                                op = 0x92 as libc::c_int;
                            } else if op == 0x9f as libc::c_int {
                                op = 0x97 as libc::c_int;
                            } else if op == 0x9e as libc::c_int {
                                op = 0x96 as libc::c_int;
                            } else if op == 0x9d as libc::c_int {
                                op = 0x93 as libc::c_int;
                            }
                        }
                        vswap();
                        gen_cast_s(t);
                        vswap();
                        gen_cast_s(t2);
                        if is_float(t) != 0 {
                            gen_opif(op);
                        } else {
                            gen_opic(op);
                        }
                        if op_class == 'C' as i32 {
                            (*vtop).type_0.t = 3 as libc::c_int;
                        } else {
                            (*vtop).type_0.t = t;
                        }
                    }
                    _ => {}
                }
                if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {
                    gv(
                        if is_float((*vtop).type_0.t & 0xf as libc::c_int) != 0 {
                            0x2 as libc::c_int
                        } else {
                            0x1 as libc::c_int
                        },
                    );
                }
                return;
            }
        }
    }
    _tcc_error(
        b"invalid operand types for binary operation\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn gen_cvt_itof1(mut t: libc::c_int) {
    if (*vtop).type_0.t & (0xf as libc::c_int | 0x10 as libc::c_int)
        == 4 as libc::c_int | 0x10 as libc::c_int
    {
        if t == 8 as libc::c_int {
            vpush_helper_func(TOK___floatundisf as libc::c_int);
        } else if t == 10 as libc::c_int {
            vpush_helper_func(TOK___floatundixf as libc::c_int);
        } else {
            vpush_helper_func(TOK___floatundidf as libc::c_int);
        }
        vrott(2 as libc::c_int);
        gfunc_call(1 as libc::c_int);
        vpushi(0 as libc::c_int);
        PUT_R_RET(vtop, t);
    } else {
        gen_cvt_itof(t);
    };
}
unsafe extern "C" fn gen_cvt_ftoi1(mut t: libc::c_int) {
    let mut st: libc::c_int = 0;
    if t == 4 as libc::c_int | 0x10 as libc::c_int {
        st = (*vtop).type_0.t & 0xf as libc::c_int;
        if st == 8 as libc::c_int {
            vpush_helper_func(TOK___fixunssfdi as libc::c_int);
        } else if st == 10 as libc::c_int {
            vpush_helper_func(TOK___fixunsxfdi as libc::c_int);
        } else {
            vpush_helper_func(TOK___fixunsdfdi as libc::c_int);
        }
        vrott(2 as libc::c_int);
        gfunc_call(1 as libc::c_int);
        vpushi(0 as libc::c_int);
        PUT_R_RET(vtop, t);
    } else {
        gen_cvt_ftoi(t);
    };
}
unsafe extern "C" fn force_charshort_cast() {
    let mut sbt: libc::c_int = if (((*vtop).r as libc::c_int & 0xc00 as libc::c_int)
        as libc::c_uint)
        .wrapping_div(
            ((0xc00 as libc::c_int & !((0xc00 as libc::c_int) << 1 as libc::c_int))
                as libc::c_uint)
                .wrapping_mul(1 as libc::c_int as libc::c_uint),
        ) == 2 as libc::c_int as libc::c_uint
    {
        4 as libc::c_int
    } else {
        3 as libc::c_int
    };
    let mut dbt: libc::c_int = (*vtop).type_0.t;
    (*vtop).r = ((*vtop).r as libc::c_int & !(0xc00 as libc::c_int)) as libc::c_ushort;
    (*vtop).type_0.t = sbt;
    gen_cast_s(
        if dbt == 11 as libc::c_int {
            1 as libc::c_int | 0x10 as libc::c_int
        } else {
            dbt
        },
    );
    (*vtop).type_0.t = dbt;
}
unsafe extern "C" fn gen_cast_s(mut t: libc::c_int) {
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    type_0.t = t;
    type_0.ref_0 = 0 as *mut Sym;
    gen_cast(&mut type_0);
}
unsafe extern "C" fn gen_cast(mut type_0: *mut CType) {
    let mut current_block: u64;
    let mut sbt: libc::c_int = 0;
    let mut dbt: libc::c_int = 0;
    let mut sf: libc::c_int = 0;
    let mut df: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut dbt_bt: libc::c_int = 0;
    let mut sbt_bt: libc::c_int = 0;
    let mut ds: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut trunc: libc::c_int = 0;
    if (*vtop).r as libc::c_int & 0xc00 as libc::c_int != 0 {
        force_charshort_cast();
    }
    if (*vtop).type_0.t & 0x80 as libc::c_int != 0 {
        gv(0x1 as libc::c_int);
    }
    if (*type_0).t as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
        && (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int
    {
        _tcc_error(b"cast to incomplete type\0" as *const u8 as *const libc::c_char);
    }
    dbt = (*type_0).t & (0xf as libc::c_int | 0x10 as libc::c_int);
    sbt = (*vtop).type_0.t & (0xf as libc::c_int | 0x10 as libc::c_int);
    if sbt == 6 as libc::c_int {
        sbt = 5 as libc::c_int;
    }
    '_again: loop {
        if !(sbt != dbt) {
            current_block = 8697884238813997716;
            break;
        }
        sf = is_float(sbt);
        df = is_float(dbt);
        dbt_bt = dbt & 0xf as libc::c_int;
        sbt_bt = sbt & 0xf as libc::c_int;
        if dbt_bt == 0 as libc::c_int {
            current_block = 8697884238813997716;
            break;
        }
        if sbt_bt == 0 as libc::c_int {
            current_block = 10303784256622860161;
        } else {
            current_block = 12349973810996921269;
        }
        loop {
            match current_block {
                10303784256622860161 => {
                    cast_error(&mut (*vtop).type_0, type_0);
                    current_block = 12349973810996921269;
                }
                _ => {
                    c = ((*vtop).r as libc::c_int
                        & (0x3f as libc::c_int | 0x100 as libc::c_int
                            | 0x200 as libc::c_int) == 0x30 as libc::c_int)
                        as libc::c_int;
                    if c != 0 {
                        if sbt == 8 as libc::c_int {
                            (*vtop)
                                .c2rust_unnamed
                                .c
                                .ld = f128::f128::new((*vtop).c2rust_unnamed.c.f);
                        } else if sbt == 9 as libc::c_int {
                            (*vtop)
                                .c2rust_unnamed
                                .c
                                .ld = f128::f128::new((*vtop).c2rust_unnamed.c.d);
                        }
                        if df != 0 {
                            if sbt_bt == 4 as libc::c_int {
                                if sbt & 0x10 as libc::c_int != 0
                                    || (*vtop).c2rust_unnamed.c.i >> 63 as libc::c_int == 0
                                {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .ld = f128::f128::new((*vtop).c2rust_unnamed.c.i);
                                } else {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .ld = -f128::f128::new(
                                        ((*vtop).c2rust_unnamed.c.i).wrapping_neg(),
                                    );
                                }
                            } else if sf == 0 {
                                if sbt & 0x10 as libc::c_int != 0
                                    || (*vtop).c2rust_unnamed.c.i >> 31 as libc::c_int == 0
                                {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .ld = f128::f128::new(
                                        (*vtop).c2rust_unnamed.c.i as uint32_t,
                                    );
                                } else {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .ld = -f128::f128::new(
                                        ((*vtop).c2rust_unnamed.c.i as uint32_t).wrapping_neg(),
                                    );
                                }
                            }
                            if dbt == 8 as libc::c_int {
                                (*vtop)
                                    .c2rust_unnamed
                                    .c
                                    .f = ((*vtop).c2rust_unnamed.c.ld).to_f32().unwrap();
                            } else if dbt == 9 as libc::c_int {
                                (*vtop)
                                    .c2rust_unnamed
                                    .c
                                    .d = ((*vtop).c2rust_unnamed.c.ld).to_f64().unwrap();
                            }
                        } else if sf != 0 && dbt == 11 as libc::c_int {
                            (*vtop)
                                .c2rust_unnamed
                                .c
                                .i = ((*vtop).c2rust_unnamed.c.ld
                                != f128::f128::new(0 as libc::c_int)) as libc::c_int
                                as uint64_t;
                        } else {
                            if sf != 0 {
                                if dbt & 0x10 as libc::c_int != 0 {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .i = ((*vtop).c2rust_unnamed.c.ld).to_u64().unwrap();
                                } else {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .i = ((*vtop).c2rust_unnamed.c.ld).to_i64().unwrap()
                                        as uint64_t;
                                }
                            } else if !(sbt_bt == 4 as libc::c_int
                                || 8 as libc::c_int == 8 as libc::c_int
                                    && sbt == 5 as libc::c_int)
                            {
                                if sbt & 0x10 as libc::c_int != 0 {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .i = (*vtop).c2rust_unnamed.c.i as uint32_t as uint64_t;
                                } else {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .i = (*vtop).c2rust_unnamed.c.i as uint32_t as uint64_t
                                        | ((*vtop).c2rust_unnamed.c.i
                                            & 0x80000000 as libc::c_uint as uint64_t)
                                            .wrapping_neg();
                                }
                            }
                            if !(dbt_bt == 4 as libc::c_int
                                || 8 as libc::c_int == 8 as libc::c_int
                                    && dbt == 5 as libc::c_int)
                            {
                                if dbt == 11 as libc::c_int {
                                    (*vtop)
                                        .c2rust_unnamed
                                        .c
                                        .i = ((*vtop).c2rust_unnamed.c.i
                                        != 0 as libc::c_int as uint64_t) as libc::c_int as uint64_t;
                                } else {
                                    let mut m: uint32_t = if dbt_bt == 1 as libc::c_int {
                                        0xff as libc::c_int as libc::c_uint
                                    } else if dbt_bt == 2 as libc::c_int {
                                        0xffff as libc::c_int as libc::c_uint
                                    } else {
                                        0xffffffff as libc::c_uint
                                    };
                                    (*vtop).c2rust_unnamed.c.i &= m as uint64_t;
                                    if dbt & 0x10 as libc::c_int == 0 {
                                        (*vtop).c2rust_unnamed.c.i
                                            |= ((*vtop).c2rust_unnamed.c.i
                                                & (m >> 1 as libc::c_int)
                                                    .wrapping_add(1 as libc::c_int as uint32_t) as uint64_t)
                                                .wrapping_neg();
                                    }
                                }
                            }
                        }
                        current_block = 8697884238813997716;
                        break '_again;
                    } else if dbt == 11 as libc::c_int
                        && (*vtop).r as libc::c_int
                            & (0x3f as libc::c_int | 0x100 as libc::c_int
                                | 0x200 as libc::c_int)
                            == 0x30 as libc::c_int | 0x200 as libc::c_int
                    {
                        (*vtop).r = 0x30 as libc::c_int as libc::c_ushort;
                        (*vtop).c2rust_unnamed.c.i = 1 as libc::c_int as uint64_t;
                        current_block = 8697884238813997716;
                        break '_again;
                    } else {
                        if nocode_wanted as libc::c_uint & 0x80000000 as libc::c_uint
                            != 0
                        {
                            current_block = 8697884238813997716;
                            break '_again;
                        }
                        if dbt == 11 as libc::c_int {
                            gen_test_zero(0x95 as libc::c_int);
                            current_block = 8697884238813997716;
                            break '_again;
                        } else if sf != 0 || df != 0 {
                            if sf != 0 && df != 0 {
                                current_block = 4741994311446740739;
                                break;
                            } else {
                                current_block = 11441799814184323368;
                                break;
                            }
                        } else {
                            ds = btype_size(dbt_bt);
                            ss = btype_size(sbt_bt);
                            if ds == 0 as libc::c_int || ss == 0 as libc::c_int {
                                current_block = 10303784256622860161;
                                continue;
                            }
                            if ds == ss && ds >= 4 as libc::c_int {
                                current_block = 8697884238813997716;
                                break '_again;
                            } else {
                                current_block = 5684854171168229155;
                                break '_again;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            4741994311446740739 => {
                gen_cvt_ftof(dbt);
                current_block = 8697884238813997716;
                break;
            }
            _ => {
                if df != 0 {
                    gen_cvt_itof1(dbt);
                    current_block = 8697884238813997716;
                    break;
                } else {
                    sbt = dbt;
                    if dbt_bt != 4 as libc::c_int && dbt_bt != 3 as libc::c_int {
                        sbt = 3 as libc::c_int;
                    }
                    gen_cvt_ftoi1(sbt);
                }
            }
        }
    }
    match current_block {
        5684854171168229155 => {
            if dbt_bt == 5 as libc::c_int || sbt_bt == 5 as libc::c_int {
                _tcc_warning(
                    b"cast between pointer and integer of different size\0" as *const u8
                        as *const libc::c_char,
                );
                if sbt_bt == 5 as libc::c_int {
                    (*vtop)
                        .type_0
                        .t = if 8 as libc::c_int == 8 as libc::c_int {
                        4 as libc::c_int
                    } else {
                        3 as libc::c_int
                    };
                }
            }
            if 1 as libc::c_int != 0
                && (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0
            {
                if ds <= ss {
                    current_block = 8697884238813997716;
                } else if ds <= 4 as libc::c_int
                    && !(dbt == 2 as libc::c_int | 0x10 as libc::c_int
                        && sbt == 1 as libc::c_int)
                {
                    gv(0x1 as libc::c_int);
                    current_block = 8697884238813997716;
                } else {
                    current_block = 993425571616822999;
                }
            } else {
                current_block = 993425571616822999;
            }
            match current_block {
                8697884238813997716 => {}
                _ => {
                    gv(0x1 as libc::c_int);
                    trunc = 0 as libc::c_int;
                    if ds == 8 as libc::c_int {
                        if !(sbt & 0x10 as libc::c_int != 0) {
                            gen_cvt_sxtw();
                        }
                    } else {
                        if ss == 8 as libc::c_int {
                            trunc = 32 as libc::c_int;
                        } else {
                            ss = 4 as libc::c_int;
                        }
                        if !(ds >= ss) {
                            if ss == 4 as libc::c_int {
                                gen_cvt_csti(dbt);
                            } else {
                                bits = (ss - ds) * 8 as libc::c_int;
                                (*vtop)
                                    .type_0
                                    .t = (if ss == 8 as libc::c_int {
                                    4 as libc::c_int
                                } else {
                                    3 as libc::c_int
                                }) | dbt & 0x10 as libc::c_int;
                                vpushi(bits);
                                gen_op('<' as i32);
                                vpushi(bits - trunc);
                                gen_op('>' as i32);
                                vpushi(trunc);
                                gen_op(0x8b as libc::c_int);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    (*vtop).type_0 = *type_0;
    (*vtop).type_0.t
        &= !(0x100 as libc::c_int | 0x200 as libc::c_int | 0x40 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn type_size(
    mut type_0: *mut CType,
    mut a: *mut libc::c_int,
) -> libc::c_int {
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut bt: libc::c_int = 0;
    bt = (*type_0).t & 0xf as libc::c_int;
    if bt == 7 as libc::c_int {
        s = (*type_0).ref_0;
        *a = (*s).r as libc::c_int;
        return (*s).c2rust_unnamed.c2rust_unnamed.c;
    } else if bt == 5 as libc::c_int {
        if (*type_0).t & 0x40 as libc::c_int != 0 {
            let mut ts: libc::c_int = 0;
            s = (*type_0).ref_0;
            ts = type_size(&mut (*s).type_0, a);
            if ts < 0 as libc::c_int
                && (*s).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int
            {
                ts = -ts;
            }
            return ts * (*s).c2rust_unnamed.c2rust_unnamed.c;
        } else {
            *a = 8 as libc::c_int;
            return 8 as libc::c_int;
        }
    } else if (*type_0).t as libc::c_uint
        & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
            | 0x80 as libc::c_int as libc::c_uint)
        == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
        && (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int
    {
        *a = 0 as libc::c_int;
        return -(1 as libc::c_int);
    } else if bt == 10 as libc::c_int {
        *a = 16 as libc::c_int;
        return 16 as libc::c_int;
    } else if bt == 9 as libc::c_int || bt == 4 as libc::c_int {
        *a = 8 as libc::c_int;
        return 8 as libc::c_int;
    } else if bt == 3 as libc::c_int || bt == 8 as libc::c_int {
        *a = 4 as libc::c_int;
        return 4 as libc::c_int;
    } else if bt == 2 as libc::c_int {
        *a = 2 as libc::c_int;
        return 2 as libc::c_int;
    } else if bt == 13 as libc::c_int || bt == 14 as libc::c_int {
        *a = 8 as libc::c_int;
        return 16 as libc::c_int;
    } else {
        *a = 1 as libc::c_int;
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn vpush_type_size(mut type_0: *mut CType, mut a: *mut libc::c_int) {
    if (*type_0).t & 0x400 as libc::c_int != 0 {
        type_size(&mut (*(*type_0).ref_0).type_0, a);
        vset(
            &mut int_type,
            0x32 as libc::c_int | 0x100 as libc::c_int,
            (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
    } else {
        let mut size: libc::c_int = type_size(type_0, a);
        if size < 0 as libc::c_int {
            _tcc_error(b"unknown type size\0" as *const u8 as *const libc::c_char);
        }
        vpushs(size as Elf64_Addr);
    };
}
#[inline]
unsafe extern "C" fn pointed_type(mut type_0: *mut CType) -> *mut CType {
    return &mut (*(*type_0).ref_0).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn mk_pointer(mut type_0: *mut CType) {
    let mut s: *mut Sym = 0 as *mut Sym;
    s = sym_push(
        0x20000000 as libc::c_int,
        type_0,
        0 as libc::c_int,
        -(1 as libc::c_int),
    );
    (*type_0)
        .t = 5 as libc::c_int
        | (*type_0).t
            & (0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int);
    (*type_0).ref_0 = s;
}
unsafe extern "C" fn is_compatible_types(
    mut type1: *mut CType,
    mut type2: *mut CType,
) -> libc::c_int {
    return compare_types(type1, type2, 0 as libc::c_int);
}
unsafe extern "C" fn is_compatible_unqualified_types(
    mut type1: *mut CType,
    mut type2: *mut CType,
) -> libc::c_int {
    return compare_types(type1, type2, 1 as libc::c_int);
}
unsafe extern "C" fn cast_error(mut st: *mut CType, mut dt: *mut CType) {
    type_incompatibility_error(
        st,
        dt,
        b"cannot convert '%s' to '%s'\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn verify_assign_cast(mut dt: *mut CType) {
    let mut st: *mut CType = 0 as *mut CType;
    let mut type1: *mut CType = 0 as *mut CType;
    let mut type2: *mut CType = 0 as *mut CType;
    let mut dbt: libc::c_int = 0;
    let mut sbt: libc::c_int = 0;
    let mut qualwarn: libc::c_int = 0;
    let mut lvl: libc::c_int = 0;
    st = &mut (*vtop).type_0;
    dbt = (*dt).t & 0xf as libc::c_int;
    sbt = (*st).t & 0xf as libc::c_int;
    if (*dt).t & 0x100 as libc::c_int != 0 {
        _tcc_warning(
            b"assignment of read-only location\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut current_block_26: u64;
    match dbt {
        0 => {
            if sbt != dbt {
                _tcc_error(
                    b"assignment to void expression\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block_26 = 7427571413727699167;
        }
        5 => {
            if is_null_pointer(vtop) != 0 {
                current_block_26 = 7427571413727699167;
            } else if is_integer_btype(sbt) != 0 {
                _tcc_warning(
                    b"assignment makes pointer from integer without a cast\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block_26 = 7427571413727699167;
            } else {
                type1 = pointed_type(dt);
                if sbt == 5 as libc::c_int {
                    type2 = pointed_type(st);
                    current_block_26 = 12599329904712511516;
                } else if sbt == 6 as libc::c_int {
                    type2 = st;
                    current_block_26 = 12599329904712511516;
                } else {
                    current_block_26 = 13354817413020277893;
                }
                match current_block_26 {
                    13354817413020277893 => {}
                    _ => {
                        if is_compatible_types(type1, type2) != 0 {
                            current_block_26 = 7427571413727699167;
                        } else {
                            lvl = 0 as libc::c_int;
                            qualwarn = lvl;
                            loop {
                                if (*type2).t & 0x100 as libc::c_int != 0
                                    && (*type1).t & 0x100 as libc::c_int == 0
                                    || (*type2).t & 0x200 as libc::c_int != 0
                                        && (*type1).t & 0x200 as libc::c_int == 0
                                {
                                    qualwarn = 1 as libc::c_int;
                                }
                                dbt = (*type1).t
                                    & (0xf as libc::c_int | 0x800 as libc::c_int);
                                sbt = (*type2).t
                                    & (0xf as libc::c_int | 0x800 as libc::c_int);
                                if dbt != 5 as libc::c_int || sbt != 5 as libc::c_int {
                                    break;
                                }
                                type1 = pointed_type(type1);
                                type2 = pointed_type(type2);
                                lvl += 1;
                                lvl;
                            }
                            if is_compatible_unqualified_types(type1, type2) == 0 {
                                if (dbt == 0 as libc::c_int || sbt == 0 as libc::c_int)
                                    && lvl == 0 as libc::c_int
                                {
                                    current_block_26 = 4775909272756257391;
                                } else if dbt == sbt
                                    && is_integer_btype(sbt & 0xf as libc::c_int) != 0
                                    && (((*type1).t as libc::c_uint
                                        & (((1 as libc::c_uint)
                                            << 6 as libc::c_int + 6 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint)
                                        == ((2 as libc::c_int) << 20 as libc::c_int)
                                            as libc::c_uint) as libc::c_int
                                        + ((*type2).t as libc::c_uint
                                            & (((1 as libc::c_uint)
                                                << 6 as libc::c_int + 6 as libc::c_int)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint)
                                            == ((2 as libc::c_int) << 20 as libc::c_int)
                                                as libc::c_uint) as libc::c_int
                                        + (((*type1).t ^ (*type2).t) & 0x10 as libc::c_int != 0)
                                            as libc::c_int) < 2 as libc::c_int
                                {
                                    current_block_26 = 4775909272756257391;
                                } else {
                                    _tcc_warning(
                                        b"assignment from incompatible pointer type\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block_26 = 7427571413727699167;
                                }
                            } else {
                                current_block_26 = 4775909272756257391;
                            }
                            match current_block_26 {
                                7427571413727699167 => {}
                                _ => {
                                    if qualwarn != 0 {
                                        (*tcc_state)
                                            .warn_num = (&mut (*(0 as *mut TCCState))
                                            .warn_discarded_qualifiers as *mut libc::c_uchar as size_t)
                                            .wrapping_sub(
                                                &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar
                                                    as size_t,
                                            ) as libc::c_uchar;
                                        (Some(
                                            _tcc_warning
                                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                                        ))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            b"assignment discards qualifiers from pointer target type\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    current_block_26 = 7427571413727699167;
                                }
                            }
                        }
                    }
                }
            }
        }
        1 | 2 | 3 | 4 => {
            if sbt == 5 as libc::c_int || sbt == 6 as libc::c_int {
                _tcc_warning(
                    b"assignment makes integer from pointer without a cast\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block_26 = 7427571413727699167;
            } else if sbt == 7 as libc::c_int {
                current_block_26 = 17605131769135552291;
            } else {
                current_block_26 = 7427571413727699167;
            }
        }
        7 => {
            current_block_26 = 17605131769135552291;
        }
        _ => {
            current_block_26 = 7427571413727699167;
        }
    }
    match current_block_26 {
        17605131769135552291 => {
            if is_compatible_unqualified_types(dt, st) == 0 {
                current_block_26 = 13354817413020277893;
            } else {
                current_block_26 = 7427571413727699167;
            }
        }
        _ => {}
    }
    match current_block_26 {
        13354817413020277893 => {
            cast_error(st, dt);
        }
        _ => {}
    };
}
unsafe extern "C" fn gen_assign_cast(mut dt: *mut CType) {
    verify_assign_cast(dt);
    gen_cast(dt);
}
#[no_mangle]
pub unsafe extern "C" fn vstore() {
    let mut sbt: libc::c_int = 0;
    let mut dbt: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut bit_size: libc::c_int = 0;
    let mut bit_pos: libc::c_int = 0;
    let mut delayed_cast: libc::c_int = 0;
    ft = (*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t;
    sbt = (*vtop).type_0.t & 0xf as libc::c_int;
    dbt = ft & 0xf as libc::c_int;
    verify_assign_cast(&mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0);
    if sbt == 7 as libc::c_int {
        size = type_size(&mut (*vtop).type_0, &mut align);
        vpushv(vtop.offset(-(1 as libc::c_int as isize)));
        if (*vtop).r as libc::c_int & 0x4000 as libc::c_int != 0 {
            gbound();
        }
        (*vtop).type_0.t = 5 as libc::c_int;
        gaddrof();
        vswap();
        if (*vtop).r as libc::c_int & 0x4000 as libc::c_int != 0 {
            gbound();
        }
        (*vtop).type_0.t = 5 as libc::c_int;
        gaddrof();
        if 1 as libc::c_int != 0 && (*tcc_state).do_bounds_check == 0 {
            gen_struct_copy(size);
        } else {
            vpushi(size);
            vpush_helper_func(TOK_memmove as libc::c_int);
            vrott(4 as libc::c_int);
            gfunc_call(3 as libc::c_int);
        }
    } else if ft & 0x80 as libc::c_int != 0 {
        vdup();
        *vtop
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *vtop.offset(-(2 as libc::c_int) as isize);
        bit_pos = ft >> 20 as libc::c_int & 0x3f as libc::c_int;
        bit_size = ft >> 20 as libc::c_int + 6 as libc::c_int & 0x3f as libc::c_int;
        (*vtop.offset(-(1 as libc::c_int) as isize))
            .type_0
            .t = (ft as libc::c_uint
            & !(((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                | 0x80 as libc::c_int as libc::c_uint)) as libc::c_int;
        if dbt == 11 as libc::c_int {
            gen_cast(&mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0);
            (*vtop.offset(-(1 as libc::c_int) as isize))
                .type_0
                .t = (*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t
                & !(0xf as libc::c_int) | (1 as libc::c_int | 0x10 as libc::c_int);
        }
        r = adjust_bf(vtop.offset(-(1 as libc::c_int as isize)), bit_pos, bit_size);
        if dbt != 11 as libc::c_int {
            gen_cast(&mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0);
            dbt = (*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t
                & 0xf as libc::c_int;
        }
        if r == 7 as libc::c_int {
            store_packed_bf(bit_pos, bit_size);
        } else {
            let mut mask: libc::c_ulonglong = ((1 as libc::c_ulonglong) << bit_size)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
            if dbt != 11 as libc::c_int {
                if dbt == 4 as libc::c_int {
                    vpushll(mask as libc::c_longlong);
                } else {
                    vpushi(mask as libc::c_uint as libc::c_int);
                }
                gen_op('&' as i32);
            }
            vpushi(bit_pos);
            gen_op('<' as i32);
            vswap();
            vdup();
            vrott(3 as libc::c_int);
            if dbt == 4 as libc::c_int {
                vpushll(!(mask << bit_pos) as libc::c_longlong);
            } else {
                vpushi(!((mask as libc::c_uint) << bit_pos) as libc::c_int);
            }
            gen_op('&' as i32);
            gen_op('|' as i32);
            vstore();
            vpop();
        }
    } else if dbt == 0 as libc::c_int {
        vtop = vtop.offset(-1);
        vtop;
    } else {
        delayed_cast = 0 as libc::c_int;
        if (dbt == 1 as libc::c_int || dbt == 2 as libc::c_int)
            && is_integer_btype(sbt) != 0
        {
            if (*vtop).r as libc::c_int & 0xc00 as libc::c_int != 0
                && btype_size(dbt) > btype_size(sbt)
            {
                force_charshort_cast();
            }
            delayed_cast = 1 as libc::c_int;
        } else {
            gen_cast(&mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0);
        }
        if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
            & 0x4000 as libc::c_int != 0
        {
            vswap();
            gbound();
            vswap();
        }
        gv(RC_TYPE(dbt));
        if delayed_cast != 0 {
            (*vtop)
                .r = ((*vtop).r as libc::c_uint
                | ((0xc00 as libc::c_int & !((0xc00 as libc::c_int) << 1 as libc::c_int))
                    as libc::c_uint)
                    .wrapping_mul(
                        ((sbt == 4 as libc::c_int) as libc::c_int + 1 as libc::c_int)
                            as libc::c_uint,
                    )) as libc::c_ushort;
            (*vtop)
                .type_0
                .t = (ft as libc::c_uint
                & !((0x1000 as libc::c_int | 0x2000 as libc::c_int
                    | 0x4000 as libc::c_int | 0x8000 as libc::c_int) as libc::c_uint
                    | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint)))
                as libc::c_int;
        }
        if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
            & 0x3f as libc::c_int == 0x31 as libc::c_int
        {
            let mut sv: SValue = SValue {
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                },
                r: 0,
                r2: 0,
                c2rust_unnamed: C2RustUnnamed_8 {
                    c2rust_unnamed: C2RustUnnamed_9 {
                        jtrue: 0,
                        jfalse: 0,
                    },
                },
                c2rust_unnamed_0: C2RustUnnamed_6 {
                    c2rust_unnamed: C2RustUnnamed_7 {
                        cmp_op: 0,
                        cmp_r: 0,
                    },
                },
            };
            r = get_reg(0x1 as libc::c_int);
            sv.type_0.t = 0x800 as libc::c_int | 4 as libc::c_int;
            sv.r = (0x32 as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
            sv
                .c2rust_unnamed
                .c
                .i = (*vtop.offset(-(1 as libc::c_int) as isize)).c2rust_unnamed.c.i;
            load(r, &mut sv);
            (*vtop.offset(-(1 as libc::c_int) as isize))
                .r = (r | 0x100 as libc::c_int) as libc::c_ushort;
        }
        r = (*vtop).r as libc::c_int & 0x3f as libc::c_int;
        if R2_RET(dbt) != 0x30 as libc::c_int {
            let mut load_type: libc::c_int = if dbt == 14 as libc::c_int {
                9 as libc::c_int
            } else {
                0x800 as libc::c_int | 4 as libc::c_int
            };
            (*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t = load_type;
            store(r, vtop.offset(-(1 as libc::c_int as isize)));
            vswap();
            incr_offset(8 as libc::c_int);
            vswap();
            store((*vtop).r2 as libc::c_int, vtop.offset(-(1 as libc::c_int as isize)));
        } else {
            store(r, vtop.offset(-(1 as libc::c_int as isize)));
        }
        vswap();
        vtop = vtop.offset(-1);
        vtop;
    };
}
#[no_mangle]
pub unsafe extern "C" fn inc(mut post: libc::c_int, mut c: libc::c_int) {
    test_lvalue();
    vdup();
    if post != 0 {
        gv_dup();
        vrotb(3 as libc::c_int);
        vrotb(3 as libc::c_int);
    }
    vpushi(c - 0x81 as libc::c_int);
    gen_op('+' as i32);
    vstore();
    if post != 0 {
        vpop();
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_mult_str(mut msg: *const libc::c_char) -> *mut CString {
    if tok != 0xc8 as libc::c_int {
        expect(msg);
    }
    cstr_reset(&mut initstr);
    while tok == 0xc8 as libc::c_int {
        cstr_cat(&mut initstr, tokc.str_0.data, -(1 as libc::c_int));
        next();
    }
    cstr_ccat(&mut initstr, '\0' as i32);
    return &mut initstr;
}
#[no_mangle]
pub unsafe extern "C" fn exact_log2p1(mut i: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if i == 0 {
        return 0 as libc::c_int;
    }
    ret = 1 as libc::c_int;
    while i >= (1 as libc::c_int) << 8 as libc::c_int {
        i >>= 8 as libc::c_int;
        ret += 8 as libc::c_int;
    }
    if i >= (1 as libc::c_int) << 4 as libc::c_int {
        ret += 4 as libc::c_int;
        i >>= 4 as libc::c_int;
    }
    if i >= (1 as libc::c_int) << 2 as libc::c_int {
        ret += 2 as libc::c_int;
        i >>= 2 as libc::c_int;
    }
    if i >= (1 as libc::c_int) << 1 as libc::c_int {
        ret += 1;
        ret;
    }
    return ret;
}
unsafe extern "C" fn parse_attribute(mut ad: *mut AttributeDef) {
    let mut t: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut astr: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        if tok != TOK_ATTRIBUTE1 as libc::c_int && tok != TOK_ATTRIBUTE2 as libc::c_int {
            return;
        }
        next();
        skip('(' as i32);
        skip('(' as i32);
        while tok != ')' as i32 {
            if tok < 256 as libc::c_int {
                expect(b"attribute name\0" as *const u8 as *const libc::c_char);
            }
            t = tok;
            next();
            match t {
                374 | 375 => {
                    let mut s: *mut Sym = 0 as *mut Sym;
                    skip('(' as i32);
                    s = sym_find(tok);
                    if s.is_null() {
                        (*tcc_state)
                            .warn_num = (&mut (*(0 as *mut TCCState))
                            .warn_implicit_function_declaration as *mut libc::c_uchar
                            as size_t)
                            .wrapping_sub(
                                &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar
                                    as size_t,
                            ) as libc::c_uchar;
                        (Some(
                            _tcc_warning
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            b"implicit declaration of function '%s'\0" as *const u8
                                as *const libc::c_char,
                            get_tok_str(tok, &mut tokc),
                        );
                        s = external_global_sym(tok, &mut func_old_type);
                    } else if (*s).type_0.t & 0xf as libc::c_int != 6 as libc::c_int {
                        _tcc_error(
                            b"'%s' is not declared as function\0" as *const u8
                                as *const libc::c_char,
                            get_tok_str(tok, &mut tokc),
                        );
                    }
                    (*ad).cleanup_func = s;
                    next();
                    skip(')' as i32);
                }
                376 | 377 => {
                    ((*ad).f).set_func_ctor(1 as libc::c_int as libc::c_uint);
                }
                378 | 379 => {
                    ((*ad).f).set_func_dtor(1 as libc::c_int as libc::c_uint);
                }
                380 | 381 => {
                    ((*ad).f).set_func_alwinl(1 as libc::c_int as libc::c_uint);
                }
                346 | 347 => {
                    skip('(' as i32);
                    astr = (*parse_mult_str(
                        b"section name\0" as *const u8 as *const libc::c_char,
                    ))
                        .data;
                    (*ad).section = find_section(tcc_state, astr);
                    skip(')' as i32);
                }
                354 | 355 => {
                    skip('(' as i32);
                    astr = (*parse_mult_str(
                        b"alias(\"target\")\0" as *const u8 as *const libc::c_char,
                    ))
                        .data;
                    (*ad).alias_target = tok_alloc_const(astr);
                    skip(')' as i32);
                }
                394 | 395 => {
                    skip('(' as i32);
                    astr = (*parse_mult_str(
                        b"visibility(\"default|hidden|internal|protected\")\0"
                            as *const u8 as *const libc::c_char,
                    ))
                        .data;
                    if strcmp(astr, b"default\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        ((*ad).a).set_visibility(0 as libc::c_int as libc::c_ushort);
                    } else if strcmp(
                        astr,
                        b"hidden\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        ((*ad).a).set_visibility(2 as libc::c_int as libc::c_ushort);
                    } else if strcmp(
                        astr,
                        b"internal\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        ((*ad).a).set_visibility(1 as libc::c_int as libc::c_ushort);
                    } else if strcmp(
                        astr,
                        b"protected\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        ((*ad).a).set_visibility(3 as libc::c_int as libc::c_ushort);
                    } else {
                        expect(
                            b"visibility(\"default|hidden|internal|protected\")\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    skip(')' as i32);
                }
                348 | 349 => {
                    if tok == '(' as i32 {
                        next();
                        n = expr_const();
                        if n <= 0 as libc::c_int
                            || n & n - 1 as libc::c_int != 0 as libc::c_int
                        {
                            _tcc_error(
                                b"alignment must be a positive power of two\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        skip(')' as i32);
                    } else {
                        n = 16 as libc::c_int;
                    }
                    ((*ad).a).set_aligned(exact_log2p1(n) as libc::c_ushort);
                    if n
                        != (1 as libc::c_int)
                            << ((*ad).a).aligned() as libc::c_int - 1 as libc::c_int
                    {
                        _tcc_error(
                            b"alignment of %d is larger than implemented\0" as *const u8
                                as *const libc::c_char,
                            n,
                        );
                    }
                }
                350 | 351 => {
                    ((*ad).a).set_packed(1 as libc::c_int as libc::c_ushort);
                }
                352 | 353 => {
                    ((*ad).a).set_weak(1 as libc::c_int as libc::c_ushort);
                }
                358 | 359 => {
                    ((*ad).a).set_nodebug(1 as libc::c_int as libc::c_ushort);
                }
                356 | 357 => {}
                391 | 392 => {
                    ((*ad).f).set_func_noreturn(1 as libc::c_int as libc::c_uint);
                }
                360 | 361 | 362 => {
                    ((*ad).f).set_func_call(0 as libc::c_int as libc::c_uint);
                }
                363 | 364 | 365 => {
                    ((*ad).f).set_func_call(1 as libc::c_int as libc::c_uint);
                }
                382 => {
                    skip('(' as i32);
                    match tok {
                        384 => {
                            (*ad)
                                .attr_mode = (4 as libc::c_int + 1 as libc::c_int)
                                as libc::c_char;
                        }
                        383 => {
                            (*ad)
                                .attr_mode = (1 as libc::c_int + 1 as libc::c_int)
                                as libc::c_char;
                        }
                        385 => {
                            (*ad)
                                .attr_mode = (2 as libc::c_int + 1 as libc::c_int)
                                as libc::c_char;
                        }
                        386 | 387 => {
                            (*ad)
                                .attr_mode = (3 as libc::c_int + 1 as libc::c_int)
                                as libc::c_char;
                        }
                        _ => {
                            _tcc_warning(
                                b"__mode__(%s) not supported\n\0" as *const u8
                                    as *const libc::c_char,
                                get_tok_str(tok, 0 as *mut CValue),
                            );
                        }
                    }
                    next();
                    skip(')' as i32);
                }
                388 => {
                    ((*ad).a).set_dllexport(1 as libc::c_int as libc::c_ushort);
                }
                390 => {
                    ((*ad).a).set_nodecorate(1 as libc::c_int as libc::c_ushort);
                }
                389 => {
                    ((*ad).a).set_dllimport(1 as libc::c_int as libc::c_ushort);
                }
                _ => {
                    (*tcc_state)
                        .warn_num = (&mut (*(0 as *mut TCCState)).warn_unsupported
                        as *mut libc::c_uchar as size_t)
                        .wrapping_sub(
                            &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar
                                as size_t,
                        ) as libc::c_uchar;
                    (Some(
                        _tcc_warning
                            as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"'%s' attribute ignored\0" as *const u8 as *const libc::c_char,
                        get_tok_str(t, 0 as *mut CValue),
                    );
                    if tok == '(' as i32 {
                        let mut parenthesis: libc::c_int = 0 as libc::c_int;
                        loop {
                            if tok == '(' as i32 {
                                parenthesis += 1;
                                parenthesis;
                            } else if tok == ')' as i32 {
                                parenthesis -= 1;
                                parenthesis;
                            }
                            next();
                            if !(parenthesis != 0 && tok != -(1 as libc::c_int)) {
                                break;
                            }
                        }
                    }
                }
            }
            if tok != ',' as i32 {
                break;
            }
            next();
        }
        skip(')' as i32);
        skip(')' as i32);
    };
}
unsafe extern "C" fn find_field(
    mut type_0: *mut CType,
    mut v: libc::c_int,
    mut cumofs: *mut libc::c_int,
) -> *mut Sym {
    let mut s: *mut Sym = (*type_0).ref_0;
    let mut v1: libc::c_int = v | 0x20000000 as libc::c_int;
    if v & 0x20000000 as libc::c_int == 0 {
        if (*type_0).t & 0xf as libc::c_int != 7 as libc::c_int {
            expect(b"struct or union\0" as *const u8 as *const libc::c_char);
        }
        if v < TOK_DEFINE as libc::c_int {
            expect(b"field name\0" as *const u8 as *const libc::c_char);
        }
        if (*s).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int {
            _tcc_error(
                b"dereferencing incomplete type '%s'\0" as *const u8
                    as *const libc::c_char,
                get_tok_str((*s).v & !(0x40000000 as libc::c_int), 0 as *mut CValue),
            );
        }
    }
    loop {
        s = (*s).c2rust_unnamed_0.next;
        if s.is_null() {
            break;
        }
        if (*s).v == v1 {
            *cumofs = (*s).c2rust_unnamed.c2rust_unnamed.c;
            return s;
        }
        if (*s).type_0.t & 0xf as libc::c_int == 7 as libc::c_int
            && (*s).v >= 0x10000000 as libc::c_int | 0x20000000 as libc::c_int
        {
            let mut ret: *mut Sym = find_field(&mut (*s).type_0, v1, cumofs);
            if !ret.is_null() {
                *cumofs += (*s).c2rust_unnamed.c2rust_unnamed.c;
                return ret;
            }
        }
    }
    if v & 0x20000000 as libc::c_int == 0 {
        _tcc_error(
            b"field not found: %s\0" as *const u8 as *const libc::c_char,
            get_tok_str(v, 0 as *mut CValue),
        );
    }
    return s;
}
unsafe extern "C" fn check_fields(mut type_0: *mut CType, mut check: libc::c_int) {
    let mut s: *mut Sym = (*type_0).ref_0;
    loop {
        s = (*s).c2rust_unnamed_0.next;
        if s.is_null() {
            break;
        }
        let mut v: libc::c_int = (*s).v & !(0x20000000 as libc::c_int);
        if v < 0x10000000 as libc::c_int {
            let mut ts: *mut TokenSym = *table_ident
                .offset((v - 256 as libc::c_int) as isize);
            if check != 0 && (*ts).tok & 0x20000000 as libc::c_int != 0 {
                _tcc_error(
                    b"duplicate member '%s'\0" as *const u8 as *const libc::c_char,
                    get_tok_str(v, 0 as *mut CValue),
                );
            }
            (*ts).tok ^= 0x20000000 as libc::c_int;
        } else if (*s).type_0.t & 0xf as libc::c_int == 7 as libc::c_int {
            check_fields(&mut (*s).type_0, check);
        }
    };
}
unsafe extern "C" fn struct_layout(mut type_0: *mut CType, mut ad: *mut AttributeDef) {
    let mut current_block: u64;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut maxalign: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut bit_pos: libc::c_int = 0;
    let mut bit_size: libc::c_int = 0;
    let mut packed: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut prevbt: libc::c_int = 0;
    let mut prev_bit_size: libc::c_int = 0;
    let mut pcc: libc::c_int = ((*tcc_state).ms_bitfields == 0) as libc::c_int;
    let mut pragma_pack: libc::c_int = *(*tcc_state).pack_stack_ptr;
    let mut f: *mut Sym = 0 as *mut Sym;
    maxalign = 1 as libc::c_int;
    offset = 0 as libc::c_int;
    c = 0 as libc::c_int;
    bit_pos = 0 as libc::c_int;
    prevbt = 7 as libc::c_int;
    prev_bit_size = 0 as libc::c_int;
    f = (*(*type_0).ref_0).c2rust_unnamed_0.next;
    while !f.is_null() {
        if (*f).type_0.t & 0x80 as libc::c_int != 0 {
            bit_size = (*f).type_0.t >> 20 as libc::c_int + 6 as libc::c_int
                & 0x3f as libc::c_int;
        } else {
            bit_size = -(1 as libc::c_int);
        }
        size = type_size(&mut (*f).type_0, &mut align);
        a = if ((*f).a).aligned() as libc::c_int != 0 {
            (1 as libc::c_int) << ((*f).a).aligned() as libc::c_int - 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        packed = 0 as libc::c_int;
        if !(pcc != 0 && bit_size == 0 as libc::c_int) {
            if pcc != 0
                && (((*f).a).packed() as libc::c_int != 0
                    || ((*ad).a).packed() as libc::c_int != 0)
            {
                packed = 1 as libc::c_int;
                align = packed;
            }
            if pragma_pack != 0 {
                packed = 1 as libc::c_int;
                if pragma_pack < align {
                    align = pragma_pack;
                }
                if pcc != 0 && pragma_pack < a {
                    a = 0 as libc::c_int;
                }
            }
        }
        if a != 0 {
            align = a;
        }
        if (*(*type_0).ref_0).type_0.t
            == (1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int
        {
            if pcc != 0 && bit_size >= 0 as libc::c_int {
                size = bit_size + 7 as libc::c_int >> 3 as libc::c_int;
            }
            offset = 0 as libc::c_int;
            if size > c {
                c = size;
            }
        } else if bit_size < 0 as libc::c_int {
            if pcc != 0 {
                c += bit_pos + 7 as libc::c_int >> 3 as libc::c_int;
            }
            c = c + align - 1 as libc::c_int & -align;
            offset = c;
            if size > 0 as libc::c_int {
                c += size;
            }
            bit_pos = 0 as libc::c_int;
            prevbt = 7 as libc::c_int;
            prev_bit_size = 0 as libc::c_int;
        } else {
            if pcc != 0 {
                if bit_size == 0 as libc::c_int {
                    current_block = 1102362364540802510;
                } else if ((*f).a).aligned() != 0 {
                    current_block = 1102362364540802510;
                } else if packed == 0 {
                    let mut a8: libc::c_int = align * 8 as libc::c_int;
                    let mut ofs: libc::c_int = ((c * 8 as libc::c_int + bit_pos) % a8
                        + bit_size + a8 - 1 as libc::c_int) / a8;
                    if ofs > size / align {
                        current_block = 1102362364540802510;
                    } else {
                        current_block = 15004371738079956865;
                    }
                } else {
                    current_block = 15004371738079956865;
                }
                match current_block {
                    1102362364540802510 => {
                        c = c + (bit_pos + 7 as libc::c_int >> 3 as libc::c_int) + align
                            - 1 as libc::c_int & -align;
                        bit_pos = 0 as libc::c_int;
                    }
                    _ => {}
                }
                if size == 8 as libc::c_int && bit_size <= 32 as libc::c_int {
                    (*f)
                        .type_0
                        .t = (*f).type_0.t & !(0xf as libc::c_int) | 3 as libc::c_int;
                    size = 4 as libc::c_int;
                }
                while bit_pos >= align * 8 as libc::c_int {
                    c += align;
                    bit_pos -= align * 8 as libc::c_int;
                }
                offset = c;
                if (*f).v & 0x10000000 as libc::c_int != 0 {
                    align = 1 as libc::c_int;
                }
            } else {
                bt = (*f).type_0.t & 0xf as libc::c_int;
                if bit_pos + bit_size > size * 8 as libc::c_int
                    || (bit_size > 0 as libc::c_int) as libc::c_int
                        == (bt != prevbt) as libc::c_int
                {
                    c = c + align - 1 as libc::c_int & -align;
                    offset = c;
                    bit_pos = 0 as libc::c_int;
                    if bit_size != 0 || prev_bit_size != 0 {
                        c += size;
                    }
                }
                if bit_size == 0 as libc::c_int && prevbt != bt {
                    align = 1 as libc::c_int;
                }
                prevbt = bt;
                prev_bit_size = bit_size;
            }
            (*f)
                .type_0
                .t = (*f).type_0.t & !((0x3f as libc::c_int) << 20 as libc::c_int)
                | bit_pos << 20 as libc::c_int;
            bit_pos += bit_size;
        }
        if align > maxalign {
            maxalign = align;
        }
        (*f).c2rust_unnamed.c2rust_unnamed.c = offset;
        (*f).r = 0 as libc::c_int as libc::c_ushort;
        f = (*f).c2rust_unnamed_0.next;
    }
    if pcc != 0 {
        c += bit_pos + 7 as libc::c_int >> 3 as libc::c_int;
    }
    bt = if ((*ad).a).aligned() as libc::c_int != 0 {
        (1 as libc::c_int) << ((*ad).a).aligned() as libc::c_int - 1 as libc::c_int
    } else {
        1 as libc::c_int
    };
    a = bt;
    if a < maxalign {
        a = maxalign;
    }
    (*(*type_0).ref_0).r = a as libc::c_ushort;
    if pragma_pack != 0 && pragma_pack < maxalign && 0 as libc::c_int == pcc {
        a = pragma_pack;
        if a < bt {
            a = bt;
        }
    }
    c = c + a - 1 as libc::c_int & -a;
    (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c = c;
    f = (*(*type_0).ref_0).c2rust_unnamed_0.next;
    while !f.is_null() {
        let mut s: libc::c_int = 0;
        let mut px: libc::c_int = 0;
        let mut cx: libc::c_int = 0;
        let mut c0: libc::c_int = 0;
        let mut t: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        if !(0 as libc::c_int == (*f).type_0.t & 0x80 as libc::c_int) {
            (*f).type_0.ref_0 = f;
            (*f)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .auxtype = -(1 as libc::c_int);
            bit_size = (*f).type_0.t >> 20 as libc::c_int + 6 as libc::c_int
                & 0x3f as libc::c_int;
            if !(bit_size == 0 as libc::c_int) {
                bit_pos = (*f).type_0.t >> 20 as libc::c_int & 0x3f as libc::c_int;
                size = type_size(&mut (*f).type_0, &mut align);
                if !(bit_pos + bit_size <= size * 8 as libc::c_int
                    && (*f).c2rust_unnamed.c2rust_unnamed.c + size <= c)
                {
                    c0 = -(1 as libc::c_int);
                    align = 1 as libc::c_int;
                    s = align;
                    t.t = 1 as libc::c_int;
                    loop {
                        px = (*f).c2rust_unnamed.c2rust_unnamed.c * 8 as libc::c_int
                            + bit_pos;
                        cx = px >> 3 as libc::c_int & -align;
                        px = px - (cx << 3 as libc::c_int);
                        if c0 == cx {
                            break;
                        }
                        s = px + bit_size + 7 as libc::c_int >> 3 as libc::c_int;
                        if s > 4 as libc::c_int {
                            t.t = 4 as libc::c_int;
                        } else if s > 2 as libc::c_int {
                            t.t = 3 as libc::c_int;
                        } else if s > 1 as libc::c_int {
                            t.t = 2 as libc::c_int;
                        } else {
                            t.t = 1 as libc::c_int;
                        }
                        s = type_size(&mut t, &mut align);
                        c0 = cx;
                    }
                    if px + bit_size <= s * 8 as libc::c_int && cx + s <= c {
                        (*f).c2rust_unnamed.c2rust_unnamed.c = cx;
                        bit_pos = px;
                        (*f)
                            .type_0
                            .t = (*f).type_0.t
                            & !((0x3f as libc::c_int) << 20 as libc::c_int)
                            | bit_pos << 20 as libc::c_int;
                        if s != size {
                            (*f)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .auxtype = t.t;
                        }
                    } else {
                        (*f)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .auxtype = 7 as libc::c_int;
                    }
                }
            }
        }
        f = (*f).c2rust_unnamed_0.next;
    }
}
unsafe extern "C" fn struct_decl(mut type_0: *mut CType, mut u: libc::c_int) {
    let mut current_block: u64;
    let mut v: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut flexible: libc::c_int = 0;
    let mut bit_size: libc::c_int = 0;
    let mut bsize: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut ut: libc::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ss: *mut Sym = 0 as *mut Sym;
    let mut ps: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut ad1: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut btype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    memset(
        &mut ad as *mut AttributeDef as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<AttributeDef>() as libc::c_ulong,
    );
    next();
    parse_attribute(&mut ad);
    v = 0 as libc::c_int;
    if tok >= 256 as libc::c_int {
        v = tok;
        next();
    }
    ut = 0 as libc::c_int;
    bt = ut;
    if u == (2 as libc::c_int) << 20 as libc::c_int {
        ut = 3 as libc::c_int;
        if tok == ':' as i32 {
            next();
            if parse_btype(&mut btype, &mut ad1, 0 as libc::c_int) == 0
                || is_integer_btype(btype.t & 0xf as libc::c_int) == 0
            {
                expect(b"enum type\0" as *const u8 as *const libc::c_char);
            }
            ut = btype.t
                & (0xf as libc::c_int | 0x800 as libc::c_int | 0x10 as libc::c_int
                    | 0x20 as libc::c_int);
            bt = ut;
        }
    }
    if v != 0 {
        s = struct_find(v);
        if !s.is_null()
            && ((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope
                == local_scope || tok != '{' as i32 && tok != ';' as i32)
        {
            if u == (*s).type_0.t {
                current_block = 5283331456837384137;
            } else if u == (2 as libc::c_int) << 20 as libc::c_int
                && (*s).type_0.t as libc::c_uint
                    & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint)
                    == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
            {
                current_block = 5283331456837384137;
            } else {
                _tcc_error(
                    b"redeclaration of '%s'\0" as *const u8 as *const libc::c_char,
                    get_tok_str(v, 0 as *mut CValue),
                );
            }
        } else {
            current_block = 5783071609795492627;
        }
    } else {
        if tok != '{' as i32 {
            expect(b"struct/union/enum name\0" as *const u8 as *const libc::c_char);
        }
        let fresh3 = anon_sym;
        anon_sym = anon_sym + 1;
        v = fresh3;
        current_block = 5783071609795492627;
    }
    match current_block {
        5783071609795492627 => {
            type1.t = u | ut;
            type1.ref_0 = 0 as *mut Sym;
            s = sym_push(
                v | 0x40000000 as libc::c_int,
                &mut type1,
                0 as libc::c_int,
                if bt != 0 { 0 as libc::c_int } else { -(1 as libc::c_int) },
            );
            (*s).r = 0 as libc::c_int as libc::c_ushort;
        }
        _ => {}
    }
    (*type_0).t = (*s).type_0.t;
    (*type_0).ref_0 = s;
    if tok == '{' as i32 {
        next();
        if (*s).c2rust_unnamed.c2rust_unnamed.c != -(1 as libc::c_int)
            && !(u == (2 as libc::c_int) << 20 as libc::c_int
                && (*s).c2rust_unnamed.c2rust_unnamed.c == 0 as libc::c_int)
        {
            _tcc_error(
                b"struct/union/enum already defined\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*s).c2rust_unnamed.c2rust_unnamed.c = -(2 as libc::c_int);
        ps = &mut (*s).c2rust_unnamed_0.next;
        if u == (2 as libc::c_int) << 20 as libc::c_int {
            let mut ll: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
            let mut pl: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
            let mut nl: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
            let mut t: CType = CType {
                t: 0,
                ref_0: 0 as *const Sym as *mut Sym,
            };
            t.ref_0 = s;
            t
                .t = 3 as libc::c_int | 0x2000 as libc::c_int
                | (3 as libc::c_int) << 20 as libc::c_int;
            if bt != 0 {
                t
                    .t = bt | 0x2000 as libc::c_int
                    | (3 as libc::c_int) << 20 as libc::c_int;
            }
            loop {
                v = tok;
                if v < TOK_DEFINE as libc::c_int {
                    expect(b"identifier\0" as *const u8 as *const libc::c_char);
                }
                ss = sym_find(v);
                if !ss.is_null() && local_stack.is_null() {
                    _tcc_error(
                        b"redefinition of enumerator '%s'\0" as *const u8
                            as *const libc::c_char,
                        get_tok_str(v, 0 as *mut CValue),
                    );
                }
                next();
                if tok == '=' as i32 {
                    next();
                    ll = expr_const64() as libc::c_longlong;
                }
                ss = sym_push(v, &mut t, 0x30 as libc::c_int, 0 as libc::c_int);
                (*ss).c2rust_unnamed.enum_val = ll;
                *ps = ss;
                ps = &mut (*ss).c2rust_unnamed_0.next;
                if ll < nl {
                    nl = ll;
                }
                if ll > pl {
                    pl = ll;
                }
                if tok != ',' as i32 {
                    break;
                }
                next();
                ll += 1;
                ll;
                if tok == '}' as i32 {
                    break;
                }
            }
            skip('}' as i32);
            if bt != 0 {
                t.t = bt;
                (*s).c2rust_unnamed.c2rust_unnamed.c = 2 as libc::c_int;
            } else {
                t.t = 3 as libc::c_int;
                if nl >= 0 as libc::c_int as libc::c_longlong {
                    if pl != pl as libc::c_uint as libc::c_longlong {
                        t
                            .t = if 8 as libc::c_int == 8 as libc::c_int {
                            4 as libc::c_int | 0x800 as libc::c_int
                        } else {
                            4 as libc::c_int
                        };
                    }
                    t.t |= 0x10 as libc::c_int;
                } else if pl != pl as libc::c_int as libc::c_longlong
                    || nl != nl as libc::c_int as libc::c_longlong
                {
                    t
                        .t = if 8 as libc::c_int == 8 as libc::c_int {
                        4 as libc::c_int | 0x800 as libc::c_int
                    } else {
                        4 as libc::c_int
                    };
                }
                let mut current_block_72: u64;
                ss = (*s).c2rust_unnamed_0.next;
                while !ss.is_null() {
                    ll = (*ss).c2rust_unnamed.enum_val;
                    if !(ll == ll as libc::c_int as libc::c_longlong) {
                        if t.t & 0x10 as libc::c_int != 0 {
                            (*ss).type_0.t |= 0x10 as libc::c_int;
                            if ll == ll as libc::c_uint as libc::c_longlong {
                                current_block_72 = 17075014677070940716;
                            } else {
                                current_block_72 = 16779030619667747692;
                            }
                        } else {
                            current_block_72 = 16779030619667747692;
                        }
                        match current_block_72 {
                            17075014677070940716 => {}
                            _ => {
                                (*ss)
                                    .type_0
                                    .t = (*ss).type_0.t & !(0xf as libc::c_int)
                                    | (if 8 as libc::c_int == 8 as libc::c_int {
                                        4 as libc::c_int | 0x800 as libc::c_int
                                    } else {
                                        4 as libc::c_int
                                    });
                            }
                        }
                    }
                    ss = (*ss).c2rust_unnamed_0.next;
                }
                (*s).c2rust_unnamed.c2rust_unnamed.c = 1 as libc::c_int;
            }
            (*type_0).t = t.t | (2 as libc::c_int) << 20 as libc::c_int;
            (*s).type_0.t = (*type_0).t;
        } else {
            c = 0 as libc::c_int;
            flexible = 0 as libc::c_int;
            while tok != '}' as i32 {
                if parse_btype(&mut btype, &mut ad1, 0 as libc::c_int) == 0 {
                    if tok == TOK_STATIC_ASSERT as libc::c_int {
                        do_Static_assert();
                    } else {
                        skip(';' as i32);
                    }
                } else {
                    loop {
                        if flexible != 0 {
                            _tcc_error(
                                b"flexible array member '%s' not at the end of struct\0"
                                    as *const u8 as *const libc::c_char,
                                get_tok_str(v, 0 as *mut CValue),
                            );
                        }
                        bit_size = -(1 as libc::c_int);
                        v = 0 as libc::c_int;
                        type1 = btype;
                        if tok != ':' as i32 {
                            if tok != ';' as i32 {
                                type_decl(&mut type1, &mut ad1, &mut v, 2 as libc::c_int);
                            }
                            if v == 0 as libc::c_int {
                                if type1.t & 0xf as libc::c_int != 7 as libc::c_int {
                                    expect(b"identifier\0" as *const u8 as *const libc::c_char);
                                } else {
                                    let mut v_0: libc::c_int = (*btype.ref_0).v;
                                    if v_0 & 0x20000000 as libc::c_int == 0
                                        && v_0 & !(0x40000000 as libc::c_int)
                                            < 0x10000000 as libc::c_int
                                    {
                                        if (*tcc_state).ms_extensions as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            expect(b"identifier\0" as *const u8 as *const libc::c_char);
                                        }
                                    }
                                }
                            }
                            if type_size(&mut type1, &mut align) < 0 as libc::c_int {
                                if u == 7 as libc::c_int
                                    && type1.t & 0x40 as libc::c_int != 0 && c != 0
                                {
                                    flexible = 1 as libc::c_int;
                                } else {
                                    _tcc_error(
                                        b"field '%s' has incomplete type\0" as *const u8
                                            as *const libc::c_char,
                                        get_tok_str(v, 0 as *mut CValue),
                                    );
                                }
                            }
                            if type1.t & 0xf as libc::c_int == 6 as libc::c_int
                                || type1.t & 0xf as libc::c_int == 0 as libc::c_int
                                || type1.t
                                    & (0x1000 as libc::c_int | 0x2000 as libc::c_int
                                        | 0x4000 as libc::c_int | 0x8000 as libc::c_int) != 0
                            {
                                _tcc_error(
                                    b"invalid type for '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                        }
                        if tok == ':' as i32 {
                            next();
                            bit_size = expr_const();
                            if bit_size < 0 as libc::c_int {
                                _tcc_error(
                                    b"negative width in bit-field '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            if v != 0 && bit_size == 0 as libc::c_int {
                                _tcc_error(
                                    b"zero width for bit-field '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            parse_attribute(&mut ad1);
                        }
                        size = type_size(&mut type1, &mut align);
                        if bit_size >= 0 as libc::c_int {
                            bt = type1.t & 0xf as libc::c_int;
                            if bt != 3 as libc::c_int && bt != 1 as libc::c_int
                                && bt != 2 as libc::c_int && bt != 11 as libc::c_int
                                && bt != 4 as libc::c_int
                            {
                                _tcc_error(
                                    b"bitfields must have scalar type\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            bsize = size * 8 as libc::c_int;
                            if bit_size > bsize {
                                _tcc_error(
                                    b"width of '%s' exceeds its type\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            } else if !(bit_size == bsize && (ad.a).packed() == 0
                                && (ad1.a).packed() == 0)
                            {
                                if bit_size == 64 as libc::c_int {
                                    _tcc_error(
                                        b"field width 64 not implemented\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    type1
                                        .t = (type1.t as libc::c_uint
                                        & !(((1 as libc::c_uint)
                                            << 6 as libc::c_int + 6 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint)
                                        | 0x80 as libc::c_int as libc::c_uint
                                        | (bit_size << 20 as libc::c_int + 6 as libc::c_int)
                                            as libc::c_uint) as libc::c_int;
                                }
                            }
                        }
                        if v != 0 as libc::c_int
                            || type1.t & 0xf as libc::c_int == 7 as libc::c_int
                        {
                            c = 1 as libc::c_int;
                        }
                        if v == 0 as libc::c_int
                            && (type1.t & 0xf as libc::c_int == 7 as libc::c_int
                                || bit_size >= 0 as libc::c_int)
                        {
                            let fresh4 = anon_sym;
                            anon_sym = anon_sym + 1;
                            v = fresh4;
                        }
                        if v != 0 {
                            ss = sym_push(
                                v | 0x20000000 as libc::c_int,
                                &mut type1,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            );
                            (*ss).a = ad1.a;
                            *ps = ss;
                            ps = &mut (*ss).c2rust_unnamed_0.next;
                        }
                        if tok == ';' as i32 || tok == -(1 as libc::c_int) {
                            break;
                        }
                        skip(',' as i32);
                    }
                    skip(';' as i32);
                }
            }
            skip('}' as i32);
            parse_attribute(&mut ad);
            if !(ad.cleanup_func).is_null() {
                _tcc_warning(
                    b"attribute '__cleanup__' ignored on type\0" as *const u8
                        as *const libc::c_char,
                );
            }
            check_fields(type_0, 1 as libc::c_int);
            check_fields(type_0, 0 as libc::c_int);
            struct_layout(type_0, &mut ad);
            if debug_modes != 0 {
                tcc_debug_fix_anon(tcc_state, type_0);
            }
        }
    }
}
unsafe extern "C" fn sym_to_attr(mut ad: *mut AttributeDef, mut s: *mut Sym) {
    merge_symattr(&mut (*ad).a, &mut (*s).a);
    merge_funcattr(
        &mut (*ad).f,
        &mut (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f,
    );
}
unsafe extern "C" fn parse_btype_qualify(
    mut type_0: *mut CType,
    mut qualifiers: libc::c_int,
) {
    while (*type_0).t & 0x40 as libc::c_int != 0 {
        (*type_0)
            .ref_0 = sym_push(
            0x20000000 as libc::c_int,
            &mut (*(*type_0).ref_0).type_0,
            0 as libc::c_int,
            (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
        type_0 = &mut (*(*type_0).ref_0).type_0;
    }
    (*type_0).t |= qualifiers;
}
unsafe extern "C" fn parse_btype(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut ignore_label: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut t: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut type_found: libc::c_int = 0;
    let mut typespec_found: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut type1: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    memset(
        ad as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<AttributeDef>() as libc::c_ulong,
    );
    type_found = 0 as libc::c_int;
    typespec_found = 0 as libc::c_int;
    t = 3 as libc::c_int;
    st = -(1 as libc::c_int);
    bt = st;
    (*type_0).ref_0 = 0 as *mut Sym;
    loop {
        match tok {
            292 => {
                next();
                continue;
            }
            297 => {
                u = 1 as libc::c_int;
                current_block = 16519494973125309678;
            }
            296 => {
                u = 0 as libc::c_int;
                current_block = 16519494973125309678;
            }
            303 => {
                u = 2 as libc::c_int;
                current_block = 16519494973125309678;
            }
            298 => {
                u = 3 as libc::c_int;
                current_block = 16519494973125309678;
            }
            315 => {
                let mut n_0: libc::c_int = 0;
                let mut ad1: AttributeDef = AttributeDef {
                    a: SymAttr {
                        aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                    },
                    f: FuncAttr {
                        func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
                    },
                    section: 0 as *mut Section,
                    cleanup_func: 0 as *mut Sym,
                    alias_target: 0,
                    asm_label: 0,
                    attr_mode: 0,
                };
                next();
                skip('(' as i32);
                memset(
                    &mut ad1 as *mut AttributeDef as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<AttributeDef>() as libc::c_ulong,
                );
                if parse_btype(&mut type1, &mut ad1, 0 as libc::c_int) != 0 {
                    type_decl(&mut type1, &mut ad1, &mut n_0, 1 as libc::c_int);
                    if (ad1.a).aligned() != 0 {
                        n_0 = (1 as libc::c_int)
                            << (ad1.a).aligned() as libc::c_int - 1 as libc::c_int;
                    } else {
                        type_size(&mut type1, &mut n_0);
                    }
                } else {
                    n_0 = expr_const();
                    if n_0 < 0 as libc::c_int
                        || n_0 & n_0 - 1 as libc::c_int != 0 as libc::c_int
                    {
                        _tcc_error(
                            b"alignment must be a positive power of two\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                skip(')' as i32);
                ((*ad).a).set_aligned(exact_log2p1(n_0) as libc::c_ushort);
                continue;
            }
            304 => {
                if t & 0xf as libc::c_int == 9 as libc::c_int {
                    t = t & !(0xf as libc::c_int | 0x800 as libc::c_int)
                        | 10 as libc::c_int;
                    current_block = 5330834795799507926;
                } else if t & (0xf as libc::c_int | 0x800 as libc::c_int)
                    == 0x800 as libc::c_int
                {
                    t = t & !(0xf as libc::c_int | 0x800 as libc::c_int)
                        | 4 as libc::c_int;
                    current_block = 5330834795799507926;
                } else {
                    u = 0x800 as libc::c_int;
                    current_block = 16519494973125309678;
                }
                match current_block {
                    16519494973125309678 => {}
                    _ => {
                        next();
                        current_block = 9073771928613846474;
                    }
                }
            }
            301 => {
                u = 11 as libc::c_int;
                current_block = 16519494973125309678;
            }
            302 => {
                _tcc_error(
                    b"_Complex is not yet supported\0" as *const u8
                        as *const libc::c_char,
                );
            }
            299 => {
                u = 8 as libc::c_int;
                current_block = 16519494973125309678;
            }
            300 => {
                if t & (0xf as libc::c_int | 0x800 as libc::c_int)
                    == 0x800 as libc::c_int
                {
                    t = t & !(0xf as libc::c_int | 0x800 as libc::c_int)
                        | 10 as libc::c_int;
                    next();
                    current_block = 9073771928613846474;
                } else {
                    u = 9 as libc::c_int;
                    current_block = 16519494973125309678;
                }
            }
            308 => {
                struct_decl(&mut type1, (2 as libc::c_int) << 20 as libc::c_int);
                current_block = 4727738346420477978;
            }
            305 => {
                struct_decl(&mut type1, 7 as libc::c_int);
                current_block = 4727738346420477978;
            }
            306 => {
                struct_decl(
                    &mut type1,
                    (1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int,
                );
                current_block = 4727738346420477978;
            }
            274 => {
                next();
                (*type_0).t = t;
                parse_btype_qualify(type_0, 0x200 as libc::c_int);
                t = (*type_0).t;
                if tok == '(' as i32 {
                    parse_expr_type(&mut type1);
                    type1.t
                        &= !((0x1000 as libc::c_int | 0x2000 as libc::c_int
                            | 0x4000 as libc::c_int | 0x8000 as libc::c_int)
                            & !(0x4000 as libc::c_int));
                    if !(type1.ref_0).is_null() {
                        sym_to_attr(ad, type1.ref_0);
                    }
                    current_block = 4727738346420477978;
                } else {
                    current_block = 9073771928613846474;
                }
            }
            275 | 276 | 277 => {
                (*type_0).t = t;
                parse_btype_qualify(type_0, 0x100 as libc::c_int);
                t = (*type_0).t;
                next();
                current_block = 9073771928613846474;
            }
            278 | 279 | 280 => {
                (*type_0).t = t;
                parse_btype_qualify(type_0, 0x200 as libc::c_int);
                t = (*type_0).t;
                next();
                current_block = 9073771928613846474;
            }
            282 | 283 | 284 => {
                if t & (0x20 as libc::c_int | 0x10 as libc::c_int)
                    == 0x20 as libc::c_int | 0x10 as libc::c_int
                {
                    _tcc_error(
                        b"signed and unsigned modifier\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                t |= 0x20 as libc::c_int;
                next();
                typespec_found = 1 as libc::c_int;
                current_block = 9073771928613846474;
            }
            281 | 285 | 289 | 290 | 291 => {
                next();
                current_block = 9073771928613846474;
            }
            273 => {
                if t & (0x20 as libc::c_int | 0x10 as libc::c_int) == 0x20 as libc::c_int
                {
                    _tcc_error(
                        b"signed and unsigned modifier\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                t |= 0x20 as libc::c_int | 0x10 as libc::c_int;
                next();
                typespec_found = 1 as libc::c_int;
                current_block = 9073771928613846474;
            }
            271 => {
                g = 0x1000 as libc::c_int;
                current_block = 7443633650025473897;
            }
            272 => {
                g = 0x2000 as libc::c_int;
                current_block = 7443633650025473897;
            }
            307 => {
                g = 0x4000 as libc::c_int;
                current_block = 7443633650025473897;
            }
            286 | 287 | 288 => {
                t |= 0x8000 as libc::c_int;
                next();
                current_block = 9073771928613846474;
            }
            393 => {
                next();
                ((*ad).f).set_func_noreturn(1 as libc::c_int as libc::c_uint);
                current_block = 9073771928613846474;
            }
            310 | 311 => {
                parse_attribute(ad);
                if (*ad).attr_mode != 0 {
                    u = (*ad).attr_mode as libc::c_int - 1 as libc::c_int;
                    t = t & !(0xf as libc::c_int | 0x800 as libc::c_int) | u;
                }
                continue;
            }
            316 | 317 | 318 => {
                next();
                parse_expr_type(&mut type1);
                type1.t
                    &= !((0x1000 as libc::c_int | 0x2000 as libc::c_int
                        | 0x4000 as libc::c_int | 0x8000 as libc::c_int)
                        & !(0x4000 as libc::c_int));
                if !(type1.ref_0).is_null() {
                    sym_to_attr(ad, type1.ref_0);
                }
                current_block = 4727738346420477978;
            }
            293 => {
                _tcc_error(
                    b"_Thread_local is not implemented\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _ => {
                if typespec_found != 0 {
                    break;
                }
                s = sym_find(tok);
                if s.is_null() || (*s).type_0.t & 0x4000 as libc::c_int == 0 {
                    break;
                }
                n = tok;
                next();
                if tok == ':' as i32 && ignore_label != 0 {
                    unget_tok(n);
                    break;
                } else {
                    t &= !(0xf as libc::c_int | 0x800 as libc::c_int);
                    u = t & !(0x100 as libc::c_int | 0x200 as libc::c_int);
                    t ^= u;
                    (*type_0).t = (*s).type_0.t & !(0x4000 as libc::c_int) | u;
                    (*type_0).ref_0 = (*s).type_0.ref_0;
                    if t != 0 {
                        parse_btype_qualify(type_0, t);
                    }
                    t = (*type_0).t;
                    sym_to_attr(ad, s);
                    typespec_found = 1 as libc::c_int;
                    bt = -(2 as libc::c_int);
                    st = bt;
                }
                current_block = 9073771928613846474;
            }
        }
        match current_block {
            7443633650025473897 => {
                if t
                    & (0x1000 as libc::c_int | 0x2000 as libc::c_int
                        | 0x4000 as libc::c_int) & !g != 0
                {
                    _tcc_error(
                        b"multiple storage classes\0" as *const u8 as *const libc::c_char,
                    );
                }
                t |= g;
                next();
                current_block = 9073771928613846474;
            }
            4727738346420477978 => {
                u = type1.t;
                (*type_0).ref_0 = type1.ref_0;
                current_block = 8948429133001550965;
            }
            16519494973125309678 => {
                next();
                current_block = 8948429133001550965;
            }
            _ => {}
        }
        match current_block {
            8948429133001550965 => {
                let mut current_block_11: u64;
                if u == 2 as libc::c_int || u == 0x800 as libc::c_int {
                    if st != -(1 as libc::c_int)
                        || bt != -(1 as libc::c_int) && bt != 3 as libc::c_int
                    {
                        current_block_11 = 2299121223138233582;
                    } else {
                        st = u;
                        current_block_11 = 17860125682698302841;
                    }
                } else if bt != -(1 as libc::c_int)
                    || st != -(1 as libc::c_int) && u != 3 as libc::c_int
                {
                    current_block_11 = 2299121223138233582;
                } else {
                    bt = u;
                    current_block_11 = 17860125682698302841;
                }
                match current_block_11 {
                    17860125682698302841 => {}
                    _ => {
                        _tcc_error(
                            b"too many basic types\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                if u != 3 as libc::c_int {
                    t = t & !(0xf as libc::c_int | 0x800 as libc::c_int) | u;
                }
                typespec_found = 1 as libc::c_int;
            }
            _ => {}
        }
        type_found = 1 as libc::c_int;
    }
    if (*tcc_state).char_is_unsigned != 0 {
        if t & (0x20 as libc::c_int | 0xf as libc::c_int) == 1 as libc::c_int {
            t |= 0x10 as libc::c_int;
        }
    }
    bt = t & (0xf as libc::c_int | 0x800 as libc::c_int);
    if bt == 0x800 as libc::c_int {
        t
            |= if 8 as libc::c_int == 8 as libc::c_int {
                4 as libc::c_int
            } else {
                3 as libc::c_int
            };
    }
    (*type_0).t = t;
    return type_found;
}
#[inline]
unsafe extern "C" fn convert_parameter_type(mut pt: *mut CType) {
    (*pt).t &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
    (*pt).t &= !(0x40 as libc::c_int | 0x400 as libc::c_int);
    if (*pt).t & 0xf as libc::c_int == 6 as libc::c_int {
        mk_pointer(pt);
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_asm_str() -> *mut CString {
    skip('(' as i32);
    return parse_mult_str(b"string constant\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn asm_label_instr() -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut astr: *mut libc::c_char = 0 as *mut libc::c_char;
    next();
    astr = (*parse_asm_str()).data;
    skip(')' as i32);
    v = tok_alloc_const(astr);
    return v;
}
unsafe extern "C" fn post_type(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut storage: libc::c_int,
    mut td: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut arg_size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut plast: *mut *mut Sym = 0 as *mut *mut Sym;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut first: *mut Sym = 0 as *mut Sym;
    let mut ad1: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut pt: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut vla_array_tok: *mut TokenString = 0 as *mut TokenString;
    let mut vla_array_str: *mut libc::c_int = 0 as *mut libc::c_int;
    if tok == '(' as i32 {
        next();
        if 2 as libc::c_int == td & (2 as libc::c_int | 1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        if tok == ')' as i32 {
            l = 0 as libc::c_int;
        } else if parse_btype(&mut pt, &mut ad1, 0 as libc::c_int) != 0 {
            l = 1 as libc::c_int;
        } else if td & (2 as libc::c_int | 1 as libc::c_int) != 0 {
            merge_attr(ad, &mut ad1);
            return 0 as libc::c_int;
        } else {
            l = 2 as libc::c_int;
        }
        first = 0 as *mut Sym;
        plast = &mut first;
        arg_size = 0 as libc::c_int;
        local_scope += 1;
        local_scope;
        if l != 0 {
            loop {
                if l != 2 as libc::c_int {
                    if pt.t & 0xf as libc::c_int == 0 as libc::c_int && tok == ')' as i32
                    {
                        break;
                    }
                    type_decl(
                        &mut pt,
                        &mut ad1,
                        &mut n,
                        2 as libc::c_int | 1 as libc::c_int | 4 as libc::c_int,
                    );
                    if pt.t & 0xf as libc::c_int == 0 as libc::c_int {
                        _tcc_error(
                            b"parameter declared as void\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if n == 0 as libc::c_int {
                        n = 0x20000000 as libc::c_int;
                    }
                } else {
                    n = tok;
                    pt.t = 0 as libc::c_int;
                    pt.ref_0 = 0 as *mut Sym;
                    next();
                }
                if n < TOK_DEFINE as libc::c_int {
                    expect(b"identifier\0" as *const u8 as *const libc::c_char);
                }
                convert_parameter_type(&mut pt);
                arg_size
                    += (type_size(&mut pt, &mut align) + 8 as libc::c_int
                        - 1 as libc::c_int) / 8 as libc::c_int;
                s = sym_push(
                    n,
                    &mut pt,
                    0x32 as libc::c_int | 0x100 as libc::c_int,
                    0 as libc::c_int,
                );
                *plast = s;
                plast = &mut (*s).c2rust_unnamed_0.next;
                if tok == ')' as i32 {
                    break;
                }
                skip(',' as i32);
                if l == 1 as libc::c_int && tok == 0xa1 as libc::c_int {
                    l = 3 as libc::c_int;
                    next();
                    break;
                } else if l == 1 as libc::c_int
                    && parse_btype(&mut pt, &mut ad1, 0 as libc::c_int) == 0
                {
                    _tcc_error(b"invalid type\0" as *const u8 as *const libc::c_char);
                }
            }
        } else {
            l = 2 as libc::c_int;
        }
        skip(')' as i32);
        if !first.is_null() {
            sym_pop(
                if !local_stack.is_null() {
                    &mut local_stack
                } else {
                    &mut global_stack
                },
                (*first).prev,
                1 as libc::c_int,
            );
            s = first;
            while !s.is_null() {
                (*s).v |= 0x20000000 as libc::c_int;
                s = (*s).c2rust_unnamed_0.next;
            }
        }
        local_scope -= 1;
        local_scope;
        (*type_0).t &= !(0x100 as libc::c_int);
        if tok == '[' as i32 {
            next();
            skip(']' as i32);
            mk_pointer(type_0);
        }
        ((*ad).f).set_func_args(arg_size as libc::c_uint);
        ((*ad).f).set_func_type(l as libc::c_uint);
        s = sym_push(
            0x20000000 as libc::c_int,
            type_0,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        (*s).a = (*ad).a;
        (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f = (*ad).f;
        (*s).c2rust_unnamed_0.next = first;
        (*type_0).t = 6 as libc::c_int;
        (*type_0).ref_0 = s;
    } else if tok == '[' as i32 {
        let mut saved_nocode_wanted: libc::c_int = nocode_wanted;
        next();
        n = -(1 as libc::c_int);
        t1 = 0 as libc::c_int;
        let mut current_block_92: u64;
        if td & 4 as libc::c_int != 0 {
            loop {
                match tok {
                    289 | 290 | 291 | 275 | 278 | 272 | 42 => {
                        next();
                    }
                    _ => {
                        if !(tok != ']' as i32) {
                            current_block_92 = 2310077433060450808;
                            break;
                        }
                        nocode_wanted = 1 as libc::c_int;
                        skip_or_save_block(&mut vla_array_tok);
                        unget_tok(0 as libc::c_int);
                        vla_array_str = (*vla_array_tok).str_0;
                        begin_macro(vla_array_tok, 2 as libc::c_int);
                        next();
                        gexpr();
                        end_macro();
                        next();
                        current_block_92 = 10627156214894864154;
                        break;
                    }
                }
            }
        } else if tok != ']' as i32 {
            if local_stack.is_null() || storage & 0x2000 as libc::c_int != 0 {
                vpushi(expr_const());
            } else {
                nocode_wanted = 0 as libc::c_int;
                gexpr();
            }
            current_block_92 = 10627156214894864154;
        } else {
            current_block_92 = 2310077433060450808;
        }
        match current_block_92 {
            10627156214894864154 => {
                if (*vtop).r as libc::c_int
                    & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
                    == 0x30 as libc::c_int
                {
                    n = (*vtop).c2rust_unnamed.c.i as libc::c_int;
                    if n < 0 as libc::c_int {
                        _tcc_error(
                            b"invalid array size\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    if is_integer_btype((*vtop).type_0.t & 0xf as libc::c_int) == 0 {
                        _tcc_error(
                            b"size of variable length array should be an integer\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    n = 0 as libc::c_int;
                    t1 = 0x400 as libc::c_int;
                }
            }
            _ => {}
        }
        skip(']' as i32);
        post_type(
            type_0,
            ad,
            storage,
            td & !(2 as libc::c_int | 1 as libc::c_int) | 8 as libc::c_int,
        );
        if (*type_0).t & 0xf as libc::c_int == 6 as libc::c_int {
            _tcc_error(
                b"declaration of an array of functions\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*type_0).t & 0xf as libc::c_int == 0 as libc::c_int
            || type_size(type_0, &mut align) < 0 as libc::c_int
        {
            _tcc_error(
                b"declaration of an array of incomplete type elements\0" as *const u8
                    as *const libc::c_char,
            );
        }
        t1 |= (*type_0).t & 0x400 as libc::c_int;
        if t1 & 0x400 as libc::c_int != 0 {
            if n < 0 as libc::c_int {
                if td & 8 as libc::c_int != 0 {
                    _tcc_error(
                        b"need explicit inner array size in VLAs\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                loc -= type_size(&mut int_type, &mut align);
                loc &= -align;
                n = loc;
                vpush_type_size(type_0, &mut align);
                gen_op('*' as i32);
                vset(&mut int_type, 0x32 as libc::c_int | 0x100 as libc::c_int, n);
                vswap();
                vstore();
            }
        }
        if n != -(1 as libc::c_int) {
            vpop();
        }
        nocode_wanted = saved_nocode_wanted;
        s = sym_push(0x20000000 as libc::c_int, type_0, 0 as libc::c_int, n);
        (*type_0)
            .t = (if t1 != 0 { 0x400 as libc::c_int } else { 0x40 as libc::c_int })
            | 5 as libc::c_int;
        (*type_0).ref_0 = s;
        if !vla_array_str.is_null() {
            if t1 & 0x400 as libc::c_int != 0 && td & 8 as libc::c_int != 0 {
                (*s).c2rust_unnamed_0.vla_array_str = vla_array_str;
            } else {
                tok_str_free_str(vla_array_str);
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn type_decl(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut v: *mut libc::c_int,
    mut td: libc::c_int,
) -> *mut CType {
    let mut post: *mut CType = 0 as *mut CType;
    let mut ret: *mut CType = 0 as *mut CType;
    let mut qualifiers: libc::c_int = 0;
    let mut storage: libc::c_int = 0;
    storage = (*type_0).t
        & (0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
            | 0x8000 as libc::c_int);
    (*type_0).t
        &= !(0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
            | 0x8000 as libc::c_int);
    ret = type_0;
    post = ret;
    while tok == '*' as i32 {
        qualifiers = 0 as libc::c_int;
        loop {
            next();
            match tok {
                274 => {
                    qualifiers |= 0x200 as libc::c_int;
                }
                275 | 276 | 277 => {
                    qualifiers |= 0x100 as libc::c_int;
                }
                278 | 279 | 280 => {
                    qualifiers |= 0x200 as libc::c_int;
                }
                289 | 290 | 291 => {}
                310 | 311 => {
                    parse_attribute(ad);
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        mk_pointer(type_0);
        (*type_0).t |= qualifiers;
        if ret == type_0 {
            ret = pointed_type(type_0);
        }
    }
    let mut current_block_22: u64;
    if tok == '(' as i32 {
        if post_type(type_0, ad, 0 as libc::c_int, td) == 0 {
            parse_attribute(ad);
            post = type_decl(type_0, ad, v, td);
            skip(')' as i32);
            current_block_22 = 4775909272756257391;
        } else {
            current_block_22 = 308315384070923257;
        }
    } else if tok >= 256 as libc::c_int && td & 2 as libc::c_int != 0 {
        *v = tok;
        next();
        current_block_22 = 4775909272756257391;
    } else {
        current_block_22 = 308315384070923257;
    }
    match current_block_22 {
        308315384070923257 => {
            if td & 1 as libc::c_int == 0 {
                expect(b"identifier\0" as *const u8 as *const libc::c_char);
            }
            *v = 0 as libc::c_int;
        }
        _ => {}
    }
    post_type(
        post,
        ad,
        if post != ret { 0 as libc::c_int } else { storage },
        td & !(2 as libc::c_int | 1 as libc::c_int),
    );
    parse_attribute(ad);
    (*type_0).t |= storage;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn indir() {
    if (*vtop).type_0.t & 0xf as libc::c_int != 5 as libc::c_int {
        if (*vtop).type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
            return;
        }
        expect(b"pointer\0" as *const u8 as *const libc::c_char);
    }
    if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {
        gv(0x1 as libc::c_int);
    }
    (*vtop).type_0 = *pointed_type(&mut (*vtop).type_0);
    if (*vtop).type_0.t & (0x40 as libc::c_int | 0x400 as libc::c_int) == 0
        && (*vtop).type_0.t & 0xf as libc::c_int != 6 as libc::c_int
    {
        (*vtop).r = ((*vtop).r as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
        if (*tcc_state).do_bounds_check != 0 {
            (*vtop)
                .r = ((*vtop).r as libc::c_int | 0x4000 as libc::c_int)
                as libc::c_ushort;
        }
    }
}
unsafe extern "C" fn gfunc_param_typed(mut func: *mut Sym, mut arg: *mut Sym) {
    let mut func_type: libc::c_int = 0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    func_type = ((*func).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
        as libc::c_int;
    if func_type == 2 as libc::c_int || func_type == 3 as libc::c_int && arg.is_null() {
        if (*vtop).type_0.t & 0xf as libc::c_int == 8 as libc::c_int {
            gen_cast_s(9 as libc::c_int);
        } else if (*vtop).type_0.t & 0x80 as libc::c_int != 0 {
            type_0.t = (*vtop).type_0.t & (0xf as libc::c_int | 0x10 as libc::c_int);
            type_0.ref_0 = (*vtop).type_0.ref_0;
            gen_cast(&mut type_0);
        } else if (*vtop).r as libc::c_int & 0xc00 as libc::c_int != 0 {
            force_charshort_cast();
        }
    } else if arg.is_null() {
        _tcc_error(
            b"too many arguments to function\0" as *const u8 as *const libc::c_char,
        );
    } else {
        type_0 = (*arg).type_0;
        type_0.t &= !(0x100 as libc::c_int);
        gen_assign_cast(&mut type_0);
    };
}
unsafe extern "C" fn expr_type(
    mut type_0: *mut CType,
    mut expr_fn: Option::<unsafe extern "C" fn() -> ()>,
) {
    nocode_wanted += 1;
    nocode_wanted;
    expr_fn.expect("non-null function pointer")();
    *type_0 = (*vtop).type_0;
    vpop();
    nocode_wanted -= 1;
    nocode_wanted;
}
unsafe extern "C" fn parse_expr_type(mut type_0: *mut CType) {
    let mut n: libc::c_int = 0;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    skip('(' as i32);
    if parse_btype(type_0, &mut ad, 0 as libc::c_int) != 0 {
        type_decl(type_0, &mut ad, &mut n, 1 as libc::c_int);
    } else {
        expr_type(type_0, Some(gexpr as unsafe extern "C" fn() -> ()));
    }
    skip(')' as i32);
}
unsafe extern "C" fn parse_type(mut type_0: *mut CType) {
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut n: libc::c_int = 0;
    if parse_btype(type_0, &mut ad, 0 as libc::c_int) == 0 {
        expect(b"type\0" as *const u8 as *const libc::c_char);
    }
    type_decl(type_0, &mut ad, &mut n, 1 as libc::c_int);
}
unsafe extern "C" fn parse_builtin_params(
    mut nc: libc::c_int,
    mut args: *const libc::c_char,
) {
    let mut c: libc::c_char = 0;
    let mut sep: libc::c_char = '(' as i32 as libc::c_char;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    if nc != 0 {
        nocode_wanted += 1;
        nocode_wanted;
    }
    next();
    if *args as libc::c_int == 0 as libc::c_int {
        skip(sep as libc::c_int);
    }
    let mut current_block_20: u64;
    loop {
        let fresh5 = args;
        args = args.offset(1);
        c = *fresh5;
        if !(c != 0) {
            break;
        }
        skip(sep as libc::c_int);
        sep = ',' as i32 as libc::c_char;
        if c as libc::c_int == 't' as i32 {
            parse_type(&mut type_0);
            vpush(&mut type_0);
        } else {
            expr_eq();
            type_0.ref_0 = 0 as *mut Sym;
            type_0.t = 0 as libc::c_int;
            match c as libc::c_int {
                101 => {
                    continue;
                }
                86 => {
                    type_0.t = 0x100 as libc::c_int;
                    current_block_20 = 4967047691291436774;
                }
                118 => {
                    current_block_20 = 4967047691291436774;
                }
                83 => {
                    type_0.t = 0x100 as libc::c_int;
                    current_block_20 = 7817329701761052946;
                }
                115 => {
                    current_block_20 = 7817329701761052946;
                }
                105 => {
                    type_0.t = 3 as libc::c_int;
                    current_block_20 = 5783071609795492627;
                }
                108 => {
                    type_0
                        .t = 0x800 as libc::c_int | 4 as libc::c_int
                        | 0x10 as libc::c_int;
                    current_block_20 = 5783071609795492627;
                }
                _ => {
                    current_block_20 = 5783071609795492627;
                }
            }
            match current_block_20 {
                4967047691291436774 => {
                    type_0.t |= 0 as libc::c_int;
                    mk_pointer(&mut type_0);
                }
                7817329701761052946 => {
                    type_0.t |= char_type.t;
                    mk_pointer(&mut type_0);
                }
                _ => {}
            }
            gen_assign_cast(&mut type_0);
        }
    }
    skip(')' as i32);
    if nc != 0 {
        nocode_wanted -= 1;
        nocode_wanted;
    }
}
unsafe extern "C" fn parse_atomic(mut atok: libc::c_int) {
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut save: libc::c_int = 0 as libc::c_int;
    let mut atom: *mut CType = 0 as *mut CType;
    let mut atom_ptr: *mut CType = 0 as *mut CType;
    let mut ct: CType = {
        let mut init = CType {
            t: 0 as libc::c_int,
            ref_0: 0 as *mut Sym,
        };
        init
    };
    let mut store_0: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    let mut buf: [libc::c_char; 40] = [0; 40];
    static mut templates: [*const libc::c_char; 16] = [
        b"alm.?\0" as *const u8 as *const libc::c_char,
        b"Asm.v\0" as *const u8 as *const libc::c_char,
        b"alsm.v\0" as *const u8 as *const libc::c_char,
        b"aplbmm.b\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
        b"avm.v\0" as *const u8 as *const libc::c_char,
    ];
    let mut template: *const libc::c_char = templates[(atok
        - TOK___atomic_store as libc::c_int) as usize];
    atom_ptr = 0 as *mut CType;
    atom = atom_ptr;
    size = 0 as libc::c_int;
    next();
    skip('(' as i32);
    arg = 0 as libc::c_int;
    loop {
        expr_eq();
        match *template.offset(arg as isize) as libc::c_int {
            97 | 65 => {
                atom_ptr = &mut (*vtop).type_0;
                if (*atom_ptr).t & 0xf as libc::c_int != 5 as libc::c_int {
                    expect(b"pointer\0" as *const u8 as *const libc::c_char);
                }
                atom = pointed_type(atom_ptr);
                size = type_size(atom, &mut align);
                if size > 8 as libc::c_int || size & size - 1 as libc::c_int != 0
                    || atok > TOK___atomic_compare_exchange as libc::c_int
                        && (0 as libc::c_int
                            == btype_size((*atom).t & 0xf as libc::c_int)
                            || (*atom).t & 0xf as libc::c_int == 5 as libc::c_int)
                {
                    expect(
                        b"integral or integer-sized pointer target type\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            112 => {
                if (*vtop).type_0.t & 0xf as libc::c_int != 5 as libc::c_int
                    || type_size(pointed_type(&mut (*vtop).type_0), &mut align) != size
                {
                    _tcc_error(
                        b"pointer target type mismatch in argument %d\0" as *const u8
                            as *const libc::c_char,
                        arg + 1 as libc::c_int,
                    );
                }
                gen_assign_cast(atom_ptr);
            }
            118 => {
                gen_assign_cast(atom);
            }
            108 => {
                indir();
                gen_assign_cast(atom);
            }
            115 => {
                save = 1 as libc::c_int;
                indir();
                store_0 = *vtop;
                vpop();
            }
            109 => {
                gen_assign_cast(&mut int_type);
            }
            98 => {
                ct.t = 11 as libc::c_int;
                gen_assign_cast(&mut ct);
            }
            _ => {}
        }
        arg += 1;
        if '.' as i32 == *template.offset(arg as isize) as libc::c_int {
            break;
        }
        skip(',' as i32);
    }
    skip(')' as i32);
    ct.t = 0 as libc::c_int;
    match *template.offset((arg + 1 as libc::c_int) as isize) as libc::c_int {
        98 => {
            ct.t = 11 as libc::c_int;
        }
        118 => {
            ct = *atom;
        }
        _ => {}
    }
    sprintf(
        buf.as_mut_ptr(),
        b"%s_%d\0" as *const u8 as *const libc::c_char,
        get_tok_str(atok, 0 as *mut CValue),
        size,
    );
    vpush_helper_func(tok_alloc_const(buf.as_mut_ptr()));
    vrott(arg - save + 1 as libc::c_int);
    gfunc_call(arg - save);
    vpush(&mut ct);
    PUT_R_RET(vtop, ct.t);
    t = ct.t & 0xf as libc::c_int;
    if t == 1 as libc::c_int || t == 2 as libc::c_int || t == 11 as libc::c_int {
        (*vtop)
            .r = ((*vtop).r as libc::c_uint
            | ((0xc00 as libc::c_int & !((0xc00 as libc::c_int) << 1 as libc::c_int))
                as libc::c_uint)
                .wrapping_mul(1 as libc::c_int as libc::c_uint)) as libc::c_ushort;
    }
    gen_cast(&mut ct);
    if save != 0 {
        vpush(&mut ct);
        *vtop = store_0;
        vswap();
        vstore();
    }
}
#[no_mangle]
pub unsafe extern "C" fn unary() {
    let mut current_block: u64;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    if debug_modes != 0 {
        tcc_debug_line(tcc_state);
        tcc_tcov_check_line(tcc_state, 1 as libc::c_int);
    }
    type_0.ref_0 = 0 as *mut Sym;
    loop {
        match tok {
            292 => {
                next();
            }
            193 | 194 | 192 => {
                t = 3 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            195 => {
                t = 3 as libc::c_int | 0x10 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            196 => {
                t = 4 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            197 => {
                t = 4 as libc::c_int | 0x10 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            202 => {
                t = 8 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            203 => {
                t = 9 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            204 => {
                t = 10 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            198 => {
                t = (if 8 as libc::c_int == 8 as libc::c_int {
                    4 as libc::c_int
                } else {
                    3 as libc::c_int
                }) | 0x800 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            199 => {
                t = (if 8 as libc::c_int == 8 as libc::c_int {
                    4 as libc::c_int
                } else {
                    3 as libc::c_int
                }) | 0x800 as libc::c_int | 0x10 as libc::c_int;
                current_block = 18067903794121657904;
                break;
            }
            337 => {
                if (*tcc_state).gnu_ext == 0 {
                    current_block = 11563380819428586977;
                    break;
                } else {
                    current_block = 9584481544819420741;
                    break;
                }
            }
            342 => {
                current_block = 9584481544819420741;
                break;
            }
            201 => {
                t = 3 as libc::c_int;
                current_block = 5074159522585948606;
                break;
            }
            200 => {
                current_block = 18385077487365379017;
                break;
            }
            167 | 40 => {
                t = tok;
                next();
                if parse_btype(&mut type_0, &mut ad, 0 as libc::c_int) != 0 {
                    type_decl(&mut type_0, &mut ad, &mut n, 1 as libc::c_int);
                    skip(')' as i32);
                    if tok == '{' as i32 {
                        if global_expr != 0 {
                            r = 0x30 as libc::c_int;
                        } else {
                            r = 0x32 as libc::c_int;
                        }
                        if type_0.t & 0x40 as libc::c_int == 0 {
                            r |= 0x100 as libc::c_int;
                        }
                        memset(
                            &mut ad as *mut AttributeDef as *mut libc::c_void,
                            0 as libc::c_int,
                            ::core::mem::size_of::<AttributeDef>() as libc::c_ulong,
                        );
                        decl_initializer_alloc(
                            &mut type_0,
                            &mut ad,
                            r,
                            1 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        );
                    } else if t == 0xa7 as libc::c_int {
                        vpush(&mut type_0);
                        return;
                    } else {
                        unary();
                        gen_cast(&mut type_0);
                    }
                } else if tok == '{' as i32 {
                    let mut saved_nocode_wanted: libc::c_int = nocode_wanted;
                    if nocode_wanted & 0xfff0000 as libc::c_int != 0
                        && nocode_wanted & 0xffff as libc::c_int == 0
                    {
                        expect(b"constant\0" as *const u8 as *const libc::c_char);
                    }
                    if 0 as libc::c_int == local_scope {
                        _tcc_error(
                            b"statement expression outside of function\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    save_regs(0 as libc::c_int);
                    block(1 as libc::c_int);
                    if saved_nocode_wanted != 0 {
                        nocode_wanted = saved_nocode_wanted;
                    }
                    skip(')' as i32);
                } else {
                    gexpr();
                    skip(')' as i32);
                }
                current_block = 8744643727662760496;
                break;
            }
            42 => {
                next();
                unary();
                indir();
                current_block = 8744643727662760496;
                break;
            }
            38 => {
                next();
                unary();
                if (*vtop).type_0.t & 0xf as libc::c_int != 6 as libc::c_int
                    && (*vtop).type_0.t & (0x40 as libc::c_int | 0x400 as libc::c_int)
                        == 0
                {
                    test_lvalue();
                }
                if !((*vtop).c2rust_unnamed_0.sym).is_null() {
                    ((*(*vtop).c2rust_unnamed_0.sym).a)
                        .set_addrtaken(1 as libc::c_int as libc::c_ushort);
                }
                mk_pointer(&mut (*vtop).type_0);
                gaddrof();
                current_block = 8744643727662760496;
                break;
            }
            33 => {
                next();
                unary();
                gen_test_zero(0x94 as libc::c_int);
                current_block = 8744643727662760496;
                break;
            }
            126 => {
                next();
                unary();
                vpushi(-(1 as libc::c_int));
                gen_op('^' as i32);
                current_block = 8744643727662760496;
                break;
            }
            43 => {
                next();
                unary();
                if (*vtop).type_0.t & 0xf as libc::c_int == 5 as libc::c_int {
                    _tcc_error(
                        b"pointer not accepted for unary plus\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if is_float((*vtop).type_0.t) == 0 {
                    vpushi(0 as libc::c_int);
                    gen_op('+' as i32);
                }
                current_block = 8744643727662760496;
                break;
            }
            309 | 312 | 313 | 314 => {
                t = tok;
                next();
                if tok == '(' as i32 {
                    tok = 0xa7 as libc::c_int;
                }
                expr_type(&mut type_0, Some(unary as unsafe extern "C" fn() -> ()));
                if t == TOK_SIZEOF as libc::c_int {
                    vpush_type_size(&mut type_0, &mut align);
                    gen_cast_s(
                        0x800 as libc::c_int | 4 as libc::c_int | 0x10 as libc::c_int,
                    );
                } else {
                    type_size(&mut type_0, &mut align);
                    s = 0 as *mut Sym;
                    if (*vtop.offset(1 as libc::c_int as isize)).r as libc::c_int
                        & 0x200 as libc::c_int != 0
                    {
                        s = (*vtop.offset(1 as libc::c_int as isize))
                            .c2rust_unnamed_0
                            .sym;
                    }
                    if !s.is_null() && ((*s).a).aligned() as libc::c_int != 0 {
                        align = (1 as libc::c_int)
                            << ((*s).a).aligned() as libc::c_int - 1 as libc::c_int;
                    }
                    vpushs(align as Elf64_Addr);
                }
                current_block = 8744643727662760496;
                break;
            }
            401 => {
                parse_builtin_params(
                    0 as libc::c_int,
                    b"ee\0" as *const u8 as *const libc::c_char,
                );
                vpop();
                current_block = 8744643727662760496;
                break;
            }
            396 => {
                parse_builtin_params(
                    0 as libc::c_int,
                    b"tt\0" as *const u8 as *const libc::c_char,
                );
                (*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t
                    &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
                (*vtop.offset(0 as libc::c_int as isize)).type_0.t
                    &= !(0x100 as libc::c_int | 0x200 as libc::c_int);
                n = is_compatible_types(
                    &mut (*vtop.offset(-(1 as libc::c_int) as isize)).type_0,
                    &mut (*vtop.offset(0 as libc::c_int as isize)).type_0,
                );
                vtop = vtop.offset(-(2 as libc::c_int as isize));
                vpushi(n);
                current_block = 8744643727662760496;
                break;
            }
            397 => {
                let mut c: int64_t = 0;
                next();
                skip('(' as i32);
                c = expr_const64();
                skip(',' as i32);
                if c == 0 {
                    nocode_wanted += 1;
                    nocode_wanted;
                }
                expr_eq();
                if c == 0 {
                    vpop();
                    nocode_wanted -= 1;
                    nocode_wanted;
                }
                skip(',' as i32);
                if c != 0 {
                    nocode_wanted += 1;
                    nocode_wanted;
                }
                expr_eq();
                if c != 0 {
                    vpop();
                    nocode_wanted -= 1;
                    nocode_wanted;
                }
                skip(')' as i32);
                current_block = 8744643727662760496;
                break;
            }
            398 => {
                parse_builtin_params(
                    1 as libc::c_int,
                    b"e\0" as *const u8 as *const libc::c_char,
                );
                n = 1 as libc::c_int;
                if (*vtop).r as libc::c_int
                    & (0x3f as libc::c_int | 0x100 as libc::c_int) != 0x30 as libc::c_int
                    || (*vtop).r as libc::c_int & 0x200 as libc::c_int != 0
                        && ((*(*vtop).c2rust_unnamed_0.sym).a).addrtaken() as libc::c_int
                            != 0
                {
                    n = 0 as libc::c_int;
                }
                vtop = vtop.offset(-1);
                vtop;
                vpushi(n);
                current_block = 8744643727662760496;
                break;
            }
            402 => {
                parse_builtin_params(
                    0 as libc::c_int,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                type_0.t = 0 as libc::c_int;
                vpush(&mut type_0);
                if nocode_wanted == 0 {
                    nocode_wanted |= 0x20000000 as libc::c_int;
                }
                current_block = 8744643727662760496;
                break;
            }
            399 | 400 => {
                let mut tok1: libc::c_int = tok;
                let mut level: libc::c_int = 0;
                next();
                skip('(' as i32);
                level = expr_const();
                if level < 0 as libc::c_int {
                    _tcc_error(
                        b"%s only takes positive integers\0" as *const u8
                            as *const libc::c_char,
                        get_tok_str(tok1, 0 as *mut CValue),
                    );
                }
                skip(')' as i32);
                type_0.t = 0 as libc::c_int;
                mk_pointer(&mut type_0);
                vset(&mut type_0, 0x32 as libc::c_int, 0 as libc::c_int);
                loop {
                    let fresh6 = level;
                    level = level - 1;
                    if !(fresh6 != 0) {
                        break;
                    }
                    mk_pointer(&mut (*vtop).type_0);
                    indir();
                }
                if tok1 == TOK_builtin_return_address as libc::c_int {
                    vpushi(8 as libc::c_int);
                    gen_op('+' as i32);
                    mk_pointer(&mut (*vtop).type_0);
                    indir();
                }
                current_block = 8744643727662760496;
                break;
            }
            403 => {
                parse_builtin_params(
                    0 as libc::c_int,
                    b"t\0" as *const u8 as *const libc::c_char,
                );
                vpushi(classify_x86_64_va_arg(&mut (*vtop).type_0));
                vswap();
                vpop();
                current_block = 8744643727662760496;
                break;
            }
            404 | 405 | 406 | 407 | 408 | 409 | 410 | 411 | 412 | 413 | 414 | 415 | 416
            | 417 | 418 | 419 => {
                parse_atomic(tok);
                current_block = 8744643727662760496;
                break;
            }
            130 | 128 => {
                t = tok;
                next();
                unary();
                inc(0 as libc::c_int, t);
                current_block = 8744643727662760496;
                break;
            }
            45 => {
                next();
                unary();
                if is_float((*vtop).type_0.t) != 0 {
                    gen_opif(0x81 as libc::c_int);
                } else {
                    vpushi(0 as libc::c_int);
                    vswap();
                    gen_op('-' as i32);
                }
                current_block = 8744643727662760496;
                break;
            }
            144 => {
                if (*tcc_state).gnu_ext == 0 {
                    current_block = 11563380819428586977;
                    break;
                } else {
                    current_block = 13918987447127423209;
                    break;
                }
            }
            294 => {
                let mut controlling_type: CType = CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                };
                let mut has_default: libc::c_int = 0 as libc::c_int;
                let mut has_match: libc::c_int = 0 as libc::c_int;
                let mut learn: libc::c_int = 0 as libc::c_int;
                let mut str: *mut TokenString = 0 as *mut TokenString;
                let mut saved_nocode_wanted_0: libc::c_int = nocode_wanted;
                nocode_wanted &= !(0xfff0000 as libc::c_int);
                next();
                skip('(' as i32);
                expr_type(
                    &mut controlling_type,
                    Some(expr_eq as unsafe extern "C" fn() -> ()),
                );
                convert_parameter_type(&mut controlling_type);
                nocode_wanted = saved_nocode_wanted_0;
                loop {
                    learn = 0 as libc::c_int;
                    skip(',' as i32);
                    if tok == TOK_DEFAULT as libc::c_int {
                        if has_default != 0 {
                            _tcc_error(
                                b"too many 'default'\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        has_default = 1 as libc::c_int;
                        if has_match == 0 {
                            learn = 1 as libc::c_int;
                        }
                        next();
                    } else {
                        let mut ad_tmp: AttributeDef = AttributeDef {
                            a: SymAttr {
                                aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                            },
                            f: FuncAttr {
                                func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
                            },
                            section: 0 as *mut Section,
                            cleanup_func: 0 as *mut Sym,
                            alias_target: 0,
                            asm_label: 0,
                            attr_mode: 0,
                        };
                        let mut itmp: libc::c_int = 0;
                        let mut cur_type: CType = CType {
                            t: 0,
                            ref_0: 0 as *const Sym as *mut Sym,
                        };
                        parse_btype(&mut cur_type, &mut ad_tmp, 0 as libc::c_int);
                        type_decl(
                            &mut cur_type,
                            &mut ad_tmp,
                            &mut itmp,
                            1 as libc::c_int,
                        );
                        if compare_types(
                            &mut controlling_type,
                            &mut cur_type,
                            0 as libc::c_int,
                        ) != 0
                        {
                            if has_match != 0 {
                                _tcc_error(
                                    b"type match twice\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            has_match = 1 as libc::c_int;
                            learn = 1 as libc::c_int;
                        }
                    }
                    skip(':' as i32);
                    if learn != 0 {
                        if !str.is_null() {
                            tok_str_free(str);
                        }
                        skip_or_save_block(&mut str);
                    } else {
                        skip_or_save_block(0 as *mut *mut TokenString);
                    }
                    if tok == ')' as i32 {
                        break;
                    }
                }
                if str.is_null() {
                    let mut buf: [libc::c_char; 60] = [0; 60];
                    type_to_str(
                        buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong
                            as libc::c_int,
                        &mut controlling_type,
                        0 as *const libc::c_char,
                    );
                    _tcc_error(
                        b"type '%s' does not match any association\0" as *const u8
                            as *const libc::c_char,
                        buf.as_mut_ptr(),
                    );
                }
                begin_macro(str, 1 as libc::c_int);
                next();
                expr_eq();
                if tok != -(1 as libc::c_int) {
                    expect(b",\0" as *const u8 as *const libc::c_char);
                }
                end_macro();
                next();
                current_block = 8744643727662760496;
                break;
            }
            343 => {
                n = 0x7fc00000 as libc::c_int;
                current_block = 8551340326779568702;
                break;
            }
            344 => {
                n = 0x7f800001 as libc::c_int;
                current_block = 8551340326779568702;
                break;
            }
            345 => {
                n = 0x7f800000 as libc::c_int;
                current_block = 8551340326779568702;
                break;
            }
            _ => {
                current_block = 11563380819428586977;
                break;
            }
        }
    }
    match current_block {
        13918987447127423209 => {
            next();
            if tok < TOK_DEFINE as libc::c_int {
                expect(b"label identifier\0" as *const u8 as *const libc::c_char);
            }
            s = label_find(tok);
            if s.is_null() {
                s = label_push(&mut global_label_stack, tok, 1 as libc::c_int);
            } else if (*s).r as libc::c_int == 2 as libc::c_int {
                (*s).r = 1 as libc::c_int as libc::c_ushort;
            }
            if (*s).type_0.t & 0xf as libc::c_int != 5 as libc::c_int {
                (*s).type_0.t = 0 as libc::c_int;
                mk_pointer(&mut (*s).type_0);
                (*s).type_0.t |= 0x2000 as libc::c_int;
            }
            vpushsym(&mut (*s).type_0, s);
            next();
            current_block = 8744643727662760496;
        }
        8551340326779568702 => {
            vpushi(n);
            (*vtop).type_0.t = 8 as libc::c_int;
            next();
            current_block = 8744643727662760496;
        }
        9584481544819420741 => {
            tok = 0xc8 as libc::c_int;
            cstr_reset(&mut tokcstr);
            cstr_cat(&mut tokcstr, funcname, 0 as libc::c_int);
            tokc.str_0.size = tokcstr.size;
            tokc.str_0.data = tokcstr.data;
            current_block = 18385077487365379017;
        }
        11563380819428586977 => {
            if tok < TOK_DEFINE as libc::c_int {
                _tcc_error(
                    b"expression expected before '%s'\0" as *const u8
                        as *const libc::c_char,
                    get_tok_str(tok, &mut tokc),
                );
            }
            t = tok;
            next();
            s = sym_find(t);
            if s.is_null()
                || (*s).type_0.t
                    & (0xf as libc::c_int
                        | (0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int))
                    == 0 as libc::c_int | (1 as libc::c_int) << 20 as libc::c_int
            {
                let mut name: *const libc::c_char = get_tok_str(t, 0 as *mut CValue);
                if tok != '(' as i32 {
                    _tcc_error(
                        b"'%s' undeclared\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
                (*tcc_state)
                    .warn_num = (&mut (*(0 as *mut TCCState))
                    .warn_implicit_function_declaration as *mut libc::c_uchar as size_t)
                    .wrapping_sub(
                        &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar
                            as size_t,
                    ) as libc::c_uchar;
                (Some(
                    _tcc_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"implicit declaration of function '%s'\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
                s = external_global_sym(t, &mut func_old_type);
            }
            r = (*s).r as libc::c_int;
            if (r & 0x3f as libc::c_int) < 0x30 as libc::c_int {
                r = r & !(0x3f as libc::c_int) | 0x32 as libc::c_int;
            }
            vset(&mut (*s).type_0, r, (*s).c2rust_unnamed.c2rust_unnamed.c);
            (*vtop).c2rust_unnamed_0.sym = s;
            if r & 0x200 as libc::c_int != 0 {
                (*vtop).c2rust_unnamed.c.i = 0 as libc::c_int as uint64_t;
            } else if r == 0x30 as libc::c_int
                && (*s).type_0.t as libc::c_uint
                    & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint)
                    == ((3 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
            {
                (*vtop).c2rust_unnamed.c.i = (*s).c2rust_unnamed.enum_val as uint64_t;
            }
            current_block = 8744643727662760496;
        }
        18067903794121657904 => {
            type_0.t = t;
            vsetc(&mut type_0, 0x30 as libc::c_int, &mut tokc);
            next();
            current_block = 8744643727662760496;
        }
        _ => {}
    }
    match current_block {
        18385077487365379017 => {
            t = char_type.t;
            current_block = 5074159522585948606;
        }
        _ => {}
    }
    match current_block {
        5074159522585948606 => {
            if (*tcc_state).warn_write_strings as libc::c_int & 1 as libc::c_int != 0 {
                t |= 0x100 as libc::c_int;
            }
            type_0.t = t;
            mk_pointer(&mut type_0);
            type_0.t |= 0x40 as libc::c_int;
            memset(
                &mut ad as *mut AttributeDef as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<AttributeDef>() as libc::c_ulong,
            );
            ad.section = (*tcc_state).rodata_section;
            decl_initializer_alloc(
                &mut type_0,
                &mut ad,
                0x30 as libc::c_int,
                2 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
        _ => {}
    }
    loop {
        if tok == 0x82 as libc::c_int || tok == 0x80 as libc::c_int {
            inc(1 as libc::c_int, tok);
            next();
        } else if tok == '.' as i32 || tok == 0xa0 as libc::c_int {
            let mut qualifiers: libc::c_int = 0;
            let mut cumofs: libc::c_int = 0;
            if tok == 0xa0 as libc::c_int {
                indir();
            }
            qualifiers = (*vtop).type_0.t
                & (0x100 as libc::c_int | 0x200 as libc::c_int);
            test_lvalue();
            next();
            s = find_field(&mut (*vtop).type_0, tok, &mut cumofs);
            gaddrof();
            (*vtop).type_0 = char_pointer_type;
            vpushi(cumofs);
            gen_op('+' as i32);
            (*vtop).type_0 = (*s).type_0;
            (*vtop).type_0.t |= qualifiers;
            if (*vtop).type_0.t & 0x40 as libc::c_int == 0 {
                (*vtop)
                    .r = ((*vtop).r as libc::c_int | 0x100 as libc::c_int)
                    as libc::c_ushort;
                if (*tcc_state).do_bounds_check != 0 {
                    (*vtop)
                        .r = ((*vtop).r as libc::c_int | 0x4000 as libc::c_int)
                        as libc::c_ushort;
                }
            }
            next();
        } else if tok == '[' as i32 {
            next();
            gexpr();
            gen_op('+' as i32);
            indir();
            skip(']' as i32);
        } else {
            if !(tok == '(' as i32) {
                break;
            }
            let mut ret: SValue = SValue {
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                },
                r: 0,
                r2: 0,
                c2rust_unnamed: C2RustUnnamed_8 {
                    c2rust_unnamed: C2RustUnnamed_9 {
                        jtrue: 0,
                        jfalse: 0,
                    },
                },
                c2rust_unnamed_0: C2RustUnnamed_6 {
                    c2rust_unnamed: C2RustUnnamed_7 {
                        cmp_op: 0,
                        cmp_r: 0,
                    },
                },
            };
            let mut sa: *mut Sym = 0 as *mut Sym;
            let mut nb_args: libc::c_int = 0;
            let mut ret_nregs: libc::c_int = 0;
            let mut ret_align: libc::c_int = 0;
            let mut regsize: libc::c_int = 0;
            let mut variadic: libc::c_int = 0;
            let mut p: *mut TokenString = 0 as *mut TokenString;
            let mut p2: *mut TokenString = 0 as *mut TokenString;
            if (*vtop).type_0.t & 0xf as libc::c_int != 6 as libc::c_int {
                's_1385: {
                    if (*vtop).type_0.t & (0xf as libc::c_int | 0x40 as libc::c_int)
                        == 5 as libc::c_int
                    {
                        (*vtop).type_0 = *pointed_type(&mut (*vtop).type_0);
                        if !((*vtop).type_0.t & 0xf as libc::c_int != 6 as libc::c_int) {
                            break 's_1385;
                        }
                    }
                    expect(b"function pointer\0" as *const u8 as *const libc::c_char);
                }
            } else {
                (*vtop)
                    .r = ((*vtop).r as libc::c_int & !(0x100 as libc::c_int))
                    as libc::c_ushort;
            }
            s = (*vtop).type_0.ref_0;
            next();
            sa = (*s).c2rust_unnamed_0.next;
            regsize = 0 as libc::c_int;
            nb_args = regsize;
            ret.r2 = 0x30 as libc::c_int as libc::c_ushort;
            if (*s).type_0.t & 0xf as libc::c_int == 7 as libc::c_int {
                variadic = (((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
                    .func_type() as libc::c_int == 3 as libc::c_int) as libc::c_int;
                ret_nregs = gfunc_sret(
                    &mut (*s).type_0,
                    variadic,
                    &mut ret.type_0,
                    &mut ret_align,
                    &mut regsize,
                );
                if ret_nregs <= 0 as libc::c_int {
                    size = type_size(&mut (*s).type_0, &mut align);
                    loc = loc - size & -align;
                    ret.type_0 = (*s).type_0;
                    ret
                        .r = (0x32 as libc::c_int | 0x100 as libc::c_int)
                        as libc::c_ushort;
                    vseti(0x32 as libc::c_int, loc);
                    if (*tcc_state).do_bounds_check != 0 {
                        loc -= 1;
                        loc;
                    }
                    ret.c2rust_unnamed.c = (*vtop).c2rust_unnamed.c;
                    if ret_nregs < 0 as libc::c_int {
                        vtop = vtop.offset(-1);
                        vtop;
                    } else {
                        nb_args += 1;
                        nb_args;
                    }
                }
            } else {
                ret_nregs = 1 as libc::c_int;
                ret.type_0 = (*s).type_0;
            }
            if ret_nregs > 0 as libc::c_int {
                ret.c2rust_unnamed.c.i = 0 as libc::c_int as uint64_t;
                PUT_R_RET(&mut ret, ret.type_0.t);
            }
            p = 0 as *mut TokenString;
            if tok != ')' as i32 {
                r = (*tcc_state).reverse_funcargs as libc::c_int;
                loop {
                    if r != 0 {
                        skip_or_save_block(&mut p2);
                        (*p2).prev = p;
                        p = p2;
                    } else {
                        expr_eq();
                        gfunc_param_typed(s, sa);
                    }
                    nb_args += 1;
                    nb_args;
                    if !sa.is_null() {
                        sa = (*sa).c2rust_unnamed_0.next;
                    }
                    if tok == ')' as i32 {
                        break;
                    }
                    skip(',' as i32);
                }
            }
            if !sa.is_null() {
                _tcc_error(
                    b"too few arguments to function\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !p.is_null() {
                n = 0 as libc::c_int;
                while !p.is_null() {
                    p2 = p;
                    sa = s;
                    loop {
                        sa = (*sa).c2rust_unnamed_0.next;
                        p2 = (*p2).prev;
                        if !(!p2.is_null() && !sa.is_null()) {
                            break;
                        }
                    }
                    p2 = (*p).prev;
                    begin_macro(p, 1 as libc::c_int);
                    next();
                    expr_eq();
                    gfunc_param_typed(s, sa);
                    end_macro();
                    p = p2;
                    n += 1;
                    n;
                }
                vrev(n);
            }
            next();
            vcheck_cmp();
            gfunc_call(nb_args);
            if ret_nregs < 0 as libc::c_int {
                vsetc(&mut ret.type_0, ret.r as libc::c_int, &mut ret.c2rust_unnamed.c);
            } else {
                n = ret_nregs;
                while n > 1 as libc::c_int {
                    let mut rc: libc::c_int = reg_classes[ret.r as usize]
                        & !(0x1 as libc::c_int | 0x2 as libc::c_int);
                    n -= 1;
                    rc <<= n;
                    r = 0 as libc::c_int;
                    while r < 25 as libc::c_int {
                        if reg_classes[r as usize] & rc != 0 {
                            break;
                        }
                        r += 1;
                        r;
                    }
                    vsetc(&mut ret.type_0, r, &mut ret.c2rust_unnamed.c);
                }
                vsetc(&mut ret.type_0, ret.r as libc::c_int, &mut ret.c2rust_unnamed.c);
                (*vtop).r2 = ret.r2;
                if (*s).type_0.t & 0xf as libc::c_int == 7 as libc::c_int
                    && ret_nregs != 0
                {
                    let mut addr: libc::c_int = 0;
                    let mut offset: libc::c_int = 0;
                    size = type_size(&mut (*s).type_0, &mut align);
                    size = size + regsize - 1 as libc::c_int & -regsize;
                    if ret_align > align {
                        align = ret_align;
                    }
                    loc = loc - size & -align;
                    addr = loc;
                    offset = 0 as libc::c_int;
                    loop {
                        vset(
                            &mut ret.type_0,
                            0x32 as libc::c_int | 0x100 as libc::c_int,
                            addr + offset,
                        );
                        vswap();
                        vstore();
                        vtop = vtop.offset(-1);
                        vtop;
                        ret_nregs -= 1;
                        if ret_nregs == 0 as libc::c_int {
                            break;
                        }
                        offset += regsize;
                    }
                    vset(
                        &mut (*s).type_0,
                        0x32 as libc::c_int | 0x100 as libc::c_int,
                        addr,
                    );
                }
                t = (*s).type_0.t & 0xf as libc::c_int;
                if t == 1 as libc::c_int || t == 2 as libc::c_int
                    || t == 11 as libc::c_int
                {
                    (*vtop)
                        .r = ((*vtop).r as libc::c_uint
                        | ((0xc00 as libc::c_int
                            & !((0xc00 as libc::c_int) << 1 as libc::c_int))
                            as libc::c_uint)
                            .wrapping_mul(1 as libc::c_int as libc::c_uint))
                        as libc::c_ushort;
                }
            }
            if ((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_noreturn() != 0
            {
                if debug_modes != 0 {
                    tcc_tcov_block_end(tcc_state, -(1 as libc::c_int));
                }
                if nocode_wanted == 0 {
                    nocode_wanted |= 0x20000000 as libc::c_int;
                }
            }
        }
    };
}
unsafe extern "C" fn precedence(mut tok_0: libc::c_int) -> libc::c_int {
    match tok_0 {
        145 => return 1 as libc::c_int,
        144 => return 2 as libc::c_int,
        124 => return 3 as libc::c_int,
        94 => return 4 as libc::c_int,
        38 => return 5 as libc::c_int,
        148 | 149 => return 6 as libc::c_int,
        146 | 147 => {}
        60 | 62 => return 8 as libc::c_int,
        43 | 45 => return 9 as libc::c_int,
        42 | 47 | 37 => return 10 as libc::c_int,
        _ => {
            if !(tok_0 >= 0x96 as libc::c_int && tok_0 <= 0x9f as libc::c_int) {
                return 0 as libc::c_int;
            }
        }
    }
    return 7 as libc::c_int;
}
static mut prec: [libc::c_uchar; 256] = [0; 256];
unsafe extern "C" fn init_prec() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        prec[i as usize] = precedence(i) as libc::c_uchar;
        i += 1;
        i;
    }
}
unsafe extern "C" fn expr_infix(mut p: libc::c_int) {
    let mut t: libc::c_int = tok;
    let mut p2: libc::c_int = 0;
    loop {
        p2 = (if (t as libc::c_uint) < 256 as libc::c_int as libc::c_uint {
            prec[t as usize] as libc::c_int
        } else {
            0 as libc::c_int
        });
        if !(p2 >= p) {
            break;
        }
        if t == 0x91 as libc::c_int || t == 0x90 as libc::c_int {
            expr_landor(t);
        } else {
            next();
            unary();
            if (if (tok as libc::c_uint) < 256 as libc::c_int as libc::c_uint {
                prec[tok as usize] as libc::c_int
            } else {
                0 as libc::c_int
            }) > p2
            {
                expr_infix(p2 + 1 as libc::c_int);
            }
            gen_op(t);
        }
        t = tok;
    };
}
unsafe extern "C" fn condition_3way() -> libc::c_int {
    let mut c: libc::c_int = -(1 as libc::c_int);
    if (*vtop).r as libc::c_int & (0x3f as libc::c_int | 0x100 as libc::c_int)
        == 0x30 as libc::c_int
        && ((*vtop).r as libc::c_int & 0x200 as libc::c_int == 0
            || ((*(*vtop).c2rust_unnamed_0.sym).a).weak() == 0)
    {
        vdup();
        gen_cast_s(11 as libc::c_int);
        c = (*vtop).c2rust_unnamed.c.i as libc::c_int;
        vpop();
    }
    return c;
}
unsafe extern "C" fn expr_landor(mut op: libc::c_int) {
    let mut t: libc::c_int = 0 as libc::c_int;
    let mut cc: libc::c_int = 1 as libc::c_int;
    let mut f: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = (op == 0x90 as libc::c_int) as libc::c_int;
    let mut c: libc::c_int = 0;
    loop {
        c = if f != 0 { i } else { condition_3way() };
        if c < 0 as libc::c_int {
            save_regs(1 as libc::c_int);
            cc = 0 as libc::c_int;
        } else if c != i {
            nocode_wanted += 1;
            nocode_wanted;
            f = 1 as libc::c_int;
        }
        if tok != op {
            break;
        }
        if c < 0 as libc::c_int {
            t = gvtst(i, t);
        } else {
            vpop();
        }
        next();
        unary();
        expr_infix(
            (if (op as libc::c_uint) < 256 as libc::c_int as libc::c_uint {
                prec[op as usize] as libc::c_int
            } else {
                0 as libc::c_int
            }) + 1 as libc::c_int,
        );
    }
    if cc != 0 || f != 0 {
        vpop();
        vpushi(i ^ f);
        gsym(t);
        nocode_wanted -= f;
    } else {
        gvtst_set(i, t);
    };
}
unsafe extern "C" fn is_cond_bool(mut sv: *mut SValue) -> libc::c_int {
    if (*sv).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int && (*sv).type_0.t & 0xf as libc::c_int == 3 as libc::c_int
    {
        return (((*sv).c2rust_unnamed.c.i as libc::c_uint)
            < 2 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    if (*sv).r as libc::c_int == 0x33 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn expr_cond() {
    let mut tt: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut islv: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut sv: SValue = SValue {
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        r: 0,
        r2: 0,
        c2rust_unnamed: C2RustUnnamed_8 {
            c2rust_unnamed: C2RustUnnamed_9 {
                jtrue: 0,
                jfalse: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_6 {
            c2rust_unnamed: C2RustUnnamed_7 {
                cmp_op: 0,
                cmp_r: 0,
            },
        },
    };
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    unary();
    expr_infix(1 as libc::c_int);
    if tok == '?' as i32 {
        next();
        c = condition_3way();
        g = (tok == ':' as i32 && (*tcc_state).gnu_ext as libc::c_int != 0)
            as libc::c_int;
        tt = 0 as libc::c_int;
        if g == 0 {
            if c < 0 as libc::c_int {
                save_regs(1 as libc::c_int);
                tt = gvtst(1 as libc::c_int, 0 as libc::c_int);
            } else {
                vpop();
            }
        } else if c < 0 as libc::c_int {
            save_regs(1 as libc::c_int);
            gv_dup();
            tt = gvtst(0 as libc::c_int, 0 as libc::c_int);
        }
        if c == 0 as libc::c_int {
            nocode_wanted += 1;
            nocode_wanted;
        }
        if g == 0 {
            gexpr();
        }
        if (*vtop).type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
            mk_pointer(&mut (*vtop).type_0);
        }
        sv = *vtop;
        vtop = vtop.offset(-1);
        vtop;
        if g != 0 {
            u = tt;
        } else if c < 0 as libc::c_int {
            u = gjmp_acs(0 as libc::c_int);
            gsym(tt);
        } else {
            u = 0 as libc::c_int;
        }
        if c == 0 as libc::c_int {
            nocode_wanted -= 1;
            nocode_wanted;
        }
        if c == 1 as libc::c_int {
            nocode_wanted += 1;
            nocode_wanted;
        }
        skip(':' as i32);
        expr_cond();
        if (*vtop).type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
            mk_pointer(&mut (*vtop).type_0);
        }
        if combine_types(&mut type_0, &mut sv, vtop, '?' as i32) == 0 {
            type_incompatibility_error(
                &mut sv.type_0,
                &mut (*vtop).type_0,
                b"type mismatch in conditional expression (have '%s' and '%s')\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if c < 0 as libc::c_int && is_cond_bool(vtop) != 0 && is_cond_bool(&mut sv) != 0
        {
            t1 = gvtst(0 as libc::c_int, 0 as libc::c_int);
            t2 = gjmp_acs(0 as libc::c_int);
            gsym(u);
            vpushv(&mut sv);
            gvtst_set(0 as libc::c_int, t1);
            gvtst_set(1 as libc::c_int, t2);
            gen_cast(&mut type_0);
            return;
        }
        islv = ((*vtop).r as libc::c_int & 0x100 as libc::c_int != 0
            && sv.r as libc::c_int & 0x100 as libc::c_int != 0
            && 7 as libc::c_int == type_0.t & 0xf as libc::c_int) as libc::c_int;
        if c != 1 as libc::c_int {
            gen_cast(&mut type_0);
            if islv != 0 {
                mk_pointer(&mut (*vtop).type_0);
                gaddrof();
            } else if 7 as libc::c_int == (*vtop).type_0.t & 0xf as libc::c_int {
                gaddrof();
            }
        }
        rc = RC_TYPE(type_0.t);
        if R2_RET(type_0.t) != 0x30 as libc::c_int {
            rc = RC_RET(type_0.t);
        }
        r2 = 0 as libc::c_int;
        tt = r2;
        if c < 0 as libc::c_int {
            r2 = gv(rc);
            tt = gjmp_acs(0 as libc::c_int);
        }
        gsym(u);
        if c == 1 as libc::c_int {
            nocode_wanted -= 1;
            nocode_wanted;
        }
        if c != 0 as libc::c_int {
            *vtop = sv;
            gen_cast(&mut type_0);
            if islv != 0 {
                mk_pointer(&mut (*vtop).type_0);
                gaddrof();
            } else if 7 as libc::c_int == (*vtop).type_0.t & 0xf as libc::c_int {
                gaddrof();
            }
        }
        if c < 0 as libc::c_int {
            r1 = gv(rc);
            move_reg(r2, r1, if islv != 0 { 5 as libc::c_int } else { type_0.t });
            (*vtop).r = r2 as libc::c_ushort;
            gsym(tt);
        }
        if islv != 0 {
            indir();
        }
    }
}
unsafe extern "C" fn expr_eq() {
    let mut t: libc::c_int = 0;
    expr_cond();
    t = tok;
    if t == '=' as i32 || t >= 0xb0 as libc::c_int && t <= 0xb9 as libc::c_int {
        test_lvalue();
        next();
        if t == '=' as i32 {
            expr_eq();
        } else {
            vdup();
            expr_eq();
            gen_op(
                (*::core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"+-*/%&|^<>\0"))[(t - 0xb0 as libc::c_int) as usize] as libc::c_int,
            );
        }
        vstore();
    }
}
#[no_mangle]
pub unsafe extern "C" fn gexpr() {
    expr_eq();
    if tok == ',' as i32 {
        loop {
            vpop();
            next();
            expr_eq();
            if !(tok == ',' as i32) {
                break;
            }
        }
        convert_parameter_type(&mut (*vtop).type_0);
        if (*vtop).r as libc::c_int & 0x3f as libc::c_int == 0x30 as libc::c_int
            && nocode_wanted != 0 && nocode_wanted & 0xfff0000 as libc::c_int == 0
        {
            gv(RC_TYPE((*vtop).type_0.t));
        }
    }
}
unsafe extern "C" fn expr_const1() {
    nocode_wanted += 0x10000 as libc::c_int;
    expr_cond();
    nocode_wanted -= 0x10000 as libc::c_int;
}
#[inline]
unsafe extern "C" fn expr_const64() -> int64_t {
    let mut c: int64_t = 0;
    expr_const1();
    if (*vtop).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
            | 0x1000 as libc::c_int) != 0x30 as libc::c_int
    {
        expect(b"constant expression\0" as *const u8 as *const libc::c_char);
    }
    c = (*vtop).c2rust_unnamed.c.i as int64_t;
    vpop();
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn expr_const() -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut wc: int64_t = expr_const64();
    c = wc as libc::c_int;
    if c as int64_t != wc && c as libc::c_uint as int64_t != wc {
        _tcc_error(b"constant exceeds 32 bit\0" as *const u8 as *const libc::c_char);
    }
    return c;
}
unsafe extern "C" fn gfunc_return(mut func_type: *mut CType) {
    if (*func_type).t & 0xf as libc::c_int == 7 as libc::c_int {
        let mut type_0: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        let mut ret_type: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        let mut ret_align: libc::c_int = 0;
        let mut ret_nregs: libc::c_int = 0;
        let mut regsize: libc::c_int = 0;
        ret_nregs = gfunc_sret(
            func_type,
            func_var,
            &mut ret_type,
            &mut ret_align,
            &mut regsize,
        );
        if !(ret_nregs < 0 as libc::c_int) {
            if 0 as libc::c_int == ret_nregs {
                type_0 = *func_type;
                mk_pointer(&mut type_0);
                vset(&mut type_0, 0x32 as libc::c_int | 0x100 as libc::c_int, func_vc);
                indir();
                vswap();
                vstore();
            } else {
                let mut size: libc::c_int = 0;
                let mut addr: libc::c_int = 0;
                let mut align: libc::c_int = 0;
                let mut rc: libc::c_int = 0;
                let mut n: libc::c_int = 0;
                size = type_size(func_type, &mut align);
                if align & ret_align - 1 as libc::c_int != 0
                    && (((*vtop).r as libc::c_int & 0x3f as libc::c_int)
                        < 0x30 as libc::c_int
                        || (*vtop).c2rust_unnamed.c.i
                            & (ret_align - 1 as libc::c_int) as uint64_t != 0)
                {
                    loc = loc - size & -ret_align;
                    addr = loc;
                    type_0 = *func_type;
                    vset(&mut type_0, 0x32 as libc::c_int | 0x100 as libc::c_int, addr);
                    vswap();
                    vstore();
                    vpop();
                    vset(
                        &mut ret_type,
                        0x32 as libc::c_int | 0x100 as libc::c_int,
                        addr,
                    );
                }
                (*vtop).type_0 = ret_type;
                rc = RC_RET(ret_type.t);
                n = ret_nregs;
                loop {
                    n -= 1;
                    if !(n > 0 as libc::c_int) {
                        break;
                    }
                    vdup();
                    gv(rc);
                    vswap();
                    incr_offset(regsize);
                    rc <<= 1 as libc::c_int;
                }
                gv(rc);
                vtop = vtop.offset(-((ret_nregs - 1 as libc::c_int) as isize));
            }
        }
    } else {
        gv(RC_RET((*func_type).t));
    }
    vtop = vtop.offset(-1);
    vtop;
}
unsafe extern "C" fn check_func_return() {
    if func_vt.t & 0xf as libc::c_int == 0 as libc::c_int {
        return;
    }
    if strcmp(funcname, b"main\0" as *const u8 as *const libc::c_char) == 0
        && func_vt.t & 0xf as libc::c_int == 3 as libc::c_int
    {
        vpushi(0 as libc::c_int);
        gen_assign_cast(&mut func_vt);
        gfunc_return(&mut func_vt);
    } else {
        _tcc_warning(
            b"function might return no value: '%s'\0" as *const u8
                as *const libc::c_char,
            funcname,
        );
    };
}
unsafe extern "C" fn case_cmp(mut a: uint64_t, mut b: uint64_t) -> libc::c_int {
    if (*cur_switch).sv.type_0.t & 0x10 as libc::c_int != 0 {
        return if a < b { -(1 as libc::c_int) } else { (a > b) as libc::c_int }
    } else {
        return if (a as int64_t) < b as int64_t {
            -(1 as libc::c_int)
        } else {
            (a as int64_t > b as int64_t) as libc::c_int
        }
    };
}
unsafe extern "C" fn case_cmp_qs(
    mut pa: *const libc::c_void,
    mut pb: *const libc::c_void,
) -> libc::c_int {
    return case_cmp(
        (**(pa as *mut *mut case_t)).v1 as uint64_t,
        (**(pb as *mut *mut case_t)).v1 as uint64_t,
    );
}
unsafe extern "C" fn case_sort(mut sw: *mut switch_t) {
    let mut p: *mut *mut case_t = 0 as *mut *mut case_t;
    if (*sw).n < 2 as libc::c_int {
        return;
    }
    qsort(
        (*sw).p as *mut libc::c_void,
        (*sw).n as size_t,
        ::core::mem::size_of::<*mut case_t>() as libc::c_ulong,
        Some(
            case_cmp_qs
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    p = (*sw).p;
    while p < ((*sw).p).offset((*sw).n as isize).offset(-(1 as libc::c_int as isize)) {
        if case_cmp(
            (**p.offset(0 as libc::c_int as isize)).v2 as uint64_t,
            (**p.offset(1 as libc::c_int as isize)).v1 as uint64_t,
        ) >= 0 as libc::c_int
        {
            let mut l1: libc::c_int = (**p.offset(0 as libc::c_int as isize)).line;
            let mut l2: libc::c_int = (**p.offset(1 as libc::c_int as isize)).line;
            _tcc_error(
                b"%i:duplicate case value\0" as *const u8 as *const libc::c_char,
                if l1 > l2 { l1 } else { l2 },
            );
        } else if (**p.offset(0 as libc::c_int as isize)).v2
            + 1 as libc::c_int as int64_t == (**p.offset(1 as libc::c_int as isize)).v1
            && (**p.offset(0 as libc::c_int as isize)).ind
                == (**p.offset(1 as libc::c_int as isize)).ind
        {
            (**p.offset(1 as libc::c_int as isize))
                .v1 = (**p.offset(0 as libc::c_int as isize)).v1;
            tcc_free(*p.offset(0 as libc::c_int as isize) as *mut libc::c_void);
            (*sw).n -= 1;
            memmove(
                p as *mut libc::c_void,
                p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                (((*sw).n as libc::c_long - p.offset_from((*sw).p) as libc::c_long)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut case_t>() as libc::c_ulong),
            );
        } else {
            p = p.offset(1);
            p;
        }
    }
}
unsafe extern "C" fn gcase(
    mut base: *mut *mut case_t,
    mut len: libc::c_int,
    mut dsym: libc::c_int,
) -> libc::c_int {
    let mut p: *mut case_t = 0 as *mut case_t;
    let mut t: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    t = (*vtop).type_0.t & 0xf as libc::c_int;
    if t != 4 as libc::c_int {
        t = 3 as libc::c_int;
    }
    while len != 0 {
        l2 = if len > 8 as libc::c_int {
            len / 2 as libc::c_int
        } else {
            0 as libc::c_int
        };
        p = *base.offset(l2 as isize);
        vdup();
        vpush64(t, (*p).v2 as libc::c_ulonglong);
        if l2 == 0 as libc::c_int && (*p).v1 == (*p).v2 {
            gen_op(0x94 as libc::c_int);
            gsym_addr(gvtst(0 as libc::c_int, 0 as libc::c_int), (*p).ind);
        } else {
            gen_op(0x9f as libc::c_int);
            if len == 1 as libc::c_int {
                dsym = gvtst(0 as libc::c_int, dsym);
                e = 0 as libc::c_int;
            } else {
                e = gvtst(0 as libc::c_int, 0 as libc::c_int);
            }
            vdup();
            vpush64(t, (*p).v1 as libc::c_ulonglong);
            gen_op(0x9d as libc::c_int);
            gsym_addr(gvtst(0 as libc::c_int, 0 as libc::c_int), (*p).ind);
            dsym = gcase(base, l2, dsym);
            gsym(e);
        }
        l2 += 1;
        l2;
        base = base.offset(l2 as isize);
        len -= l2;
    }
    return gjmp_acs(dsym);
}
unsafe extern "C" fn end_switch() {
    let mut sw: *mut switch_t = cur_switch;
    dynarray_reset(
        &mut (*sw).p as *mut *mut *mut case_t as *mut libc::c_void,
        &mut (*sw).n,
    );
    cur_switch = (*sw).prev;
    tcc_free(sw as *mut libc::c_void);
}
unsafe extern "C" fn try_call_scope_cleanup(mut stop: *mut Sym) {
    let mut cls: *mut Sym = (*cur_scope).cl.s;
    while cls != stop {
        let mut fs: *mut Sym = (*cls).c2rust_unnamed.cleanup_func;
        let mut vs: *mut Sym = (*cls).prev_tok;
        vpushsym(&mut (*fs).type_0, fs);
        vset(
            &mut (*vs).type_0,
            (*vs).r as libc::c_int,
            (*vs).c2rust_unnamed.c2rust_unnamed.c,
        );
        (*vtop).c2rust_unnamed_0.sym = vs;
        mk_pointer(&mut (*vtop).type_0);
        gaddrof();
        gfunc_call(1 as libc::c_int);
        cls = (*cls).c2rust_unnamed_0.next;
    }
}
unsafe extern "C" fn try_call_cleanup_goto(mut cleanupstate: *mut Sym) {
    let mut oc: *mut Sym = 0 as *mut Sym;
    let mut cc: *mut Sym = 0 as *mut Sym;
    let mut ocd: libc::c_int = 0;
    let mut ccd: libc::c_int = 0;
    if ((*cur_scope).cl.s).is_null() {
        return;
    }
    ocd = if !cleanupstate.is_null() {
        (*cleanupstate).v & !(0x20000000 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    ccd = (*cur_scope).cl.n;
    oc = cleanupstate;
    while ocd > ccd {
        ocd -= 1;
        ocd;
        oc = (*oc).c2rust_unnamed_0.next;
    }
    cc = (*cur_scope).cl.s;
    while ccd > ocd {
        ccd -= 1;
        ccd;
        cc = (*cc).c2rust_unnamed_0.next;
    }
    while cc != oc {
        cc = (*cc).c2rust_unnamed_0.next;
        oc = (*oc).c2rust_unnamed_0.next;
        ccd -= 1;
        ccd;
    }
    try_call_scope_cleanup(cc);
}
unsafe extern "C" fn block_cleanup(mut o_0: *mut scope) {
    let mut jmp: libc::c_int = 0 as libc::c_int;
    let mut g: *mut Sym = 0 as *mut Sym;
    let mut pg: *mut *mut Sym = 0 as *mut *mut Sym;
    pg = &mut pending_gotos;
    loop {
        g = *pg;
        if !(!g.is_null() && (*g).c2rust_unnamed.c2rust_unnamed.c > (*o_0).cl.n) {
            break;
        }
        let mut current_block_9: u64;
        if (*(*g).prev_tok).r as libc::c_int & 1 as libc::c_int != 0 {
            let mut pcl: *mut Sym = (*g).c2rust_unnamed_0.next;
            if jmp == 0 {
                jmp = gjmp_acs(0 as libc::c_int);
            }
            gsym((*pcl).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext);
            try_call_scope_cleanup((*o_0).cl.s);
            (*pcl)
                .c2rust_unnamed
                .c2rust_unnamed
                .c2rust_unnamed
                .jnext = gjmp_acs(0 as libc::c_int);
            if (*o_0).cl.n == 0 {
                current_block_9 = 5067434866001599359;
            } else {
                (*g).c2rust_unnamed.c2rust_unnamed.c = (*o_0).cl.n;
                pg = &mut (*g).prev;
                current_block_9 = 7651349459974463963;
            }
        } else {
            current_block_9 = 5067434866001599359;
        }
        match current_block_9 {
            5067434866001599359 => {
                *pg = (*g).prev;
                sym_free(g);
            }
            _ => {}
        }
    }
    gsym(jmp);
    try_call_scope_cleanup((*o_0).cl.s);
}
unsafe extern "C" fn vla_restore(mut loc_0: libc::c_int) {
    if loc_0 != 0 {
        gen_vla_sp_restore(loc_0);
    }
}
unsafe extern "C" fn vla_leave(mut o_0: *mut scope) {
    let mut c: *mut scope = cur_scope;
    let mut v: *mut scope = 0 as *mut scope;
    while c != o_0 && !c.is_null() {
        if (*c).vla.num != 0 {
            v = c;
        }
        c = (*c).prev;
    }
    if !v.is_null() {
        vla_restore((*v).vla.locorig);
    }
}
unsafe extern "C" fn new_scope(mut o_0: *mut scope) {
    *o_0 = *cur_scope;
    (*o_0).prev = cur_scope;
    cur_scope = o_0;
    (*cur_scope).vla.num = 0 as libc::c_int;
    (*o_0).lstk = local_stack;
    (*o_0).llstk = local_label_stack;
    local_scope += 1;
    local_scope;
}
unsafe extern "C" fn prev_scope(mut o_0: *mut scope, mut is_expr: libc::c_int) {
    vla_leave((*o_0).prev);
    if (*o_0).cl.s != (*(*o_0).prev).cl.s {
        block_cleanup((*o_0).prev);
    }
    label_pop(&mut local_label_stack, (*o_0).llstk, is_expr);
    pop_local_syms((*o_0).lstk, is_expr);
    cur_scope = (*o_0).prev;
    local_scope -= 1;
    local_scope;
}
unsafe extern "C" fn leave_scope(mut o_0: *mut scope) {
    if o_0.is_null() {
        return;
    }
    try_call_scope_cleanup((*o_0).cl.s);
    vla_leave(o_0);
}
unsafe extern "C" fn new_scope_s(mut o_0: *mut scope) {
    (*o_0).lstk = local_stack;
    local_scope += 1;
    local_scope;
}
unsafe extern "C" fn prev_scope_s(mut o_0: *mut scope) {
    sym_pop(&mut local_stack, (*o_0).lstk, 0 as libc::c_int);
    local_scope -= 1;
    local_scope;
}
unsafe extern "C" fn lblock(mut bsym: *mut libc::c_int, mut csym: *mut libc::c_int) {
    let mut lo: *mut scope = loop_scope;
    let mut co: *mut scope = cur_scope;
    let mut b: *mut libc::c_int = (*co).bsym;
    let mut c: *mut libc::c_int = (*co).csym;
    if !csym.is_null() {
        (*co).csym = csym;
        loop_scope = co;
    }
    (*co).bsym = bsym;
    block(0 as libc::c_int);
    (*co).bsym = b;
    if !csym.is_null() {
        (*co).csym = c;
        loop_scope = lo;
    }
}
unsafe extern "C" fn block(mut flags: libc::c_int) {
    let mut current_block: u64;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut o_0: scope = scope {
        prev: 0 as *mut scope,
        vla: C2RustUnnamed_11 {
            loc: 0,
            locorig: 0,
            num: 0,
        },
        cl: C2RustUnnamed_12 {
            s: 0 as *mut Sym,
            n: 0,
        },
        bsym: 0 as *mut libc::c_int,
        csym: 0 as *mut libc::c_int,
        lstk: 0 as *mut Sym,
        llstk: 0 as *mut Sym,
    };
    let mut s: *mut Sym = 0 as *mut Sym;
    if flags & 1 as libc::c_int != 0 {
        vpushi(0 as libc::c_int);
        (*vtop).type_0.t = 0 as libc::c_int;
    }
    loop {
        t = tok;
        if t >= 0xc0 as libc::c_int && t <= 0xcf as libc::c_int {
            current_block = 15571315640078739593;
            break;
        }
        next();
        if debug_modes != 0 {
            tcc_tcov_check_line(tcc_state, 0 as libc::c_int);
            tcc_tcov_block_begin(tcc_state);
        }
        if t == TOK_IF as libc::c_int {
            new_scope_s(&mut o_0);
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            a = gvtst(1 as libc::c_int, 0 as libc::c_int);
            block(0 as libc::c_int);
            if tok == TOK_ELSE as libc::c_int {
                d = gjmp_acs(0 as libc::c_int);
                gsym(a);
                next();
                block(0 as libc::c_int);
                gsym(d);
            } else {
                gsym(a);
            }
            prev_scope_s(&mut o_0);
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_WHILE as libc::c_int {
            new_scope_s(&mut o_0);
            d = gind();
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            a = gvtst(1 as libc::c_int, 0 as libc::c_int);
            b = 0 as libc::c_int;
            lblock(&mut a, &mut b);
            gjmp_addr_acs(d);
            gsym_addr(b, d);
            gsym(a);
            prev_scope_s(&mut o_0);
            current_block = 2626552447423186229;
            break;
        } else if t == '{' as i32 {
            if debug_modes != 0 {
                tcc_debug_stabn(tcc_state, N_LBRAC as libc::c_int, ind - func_ind);
            }
            new_scope(&mut o_0);
            while tok == TOK_LABEL as libc::c_int {
                loop {
                    next();
                    if tok < TOK_DEFINE as libc::c_int {
                        expect(
                            b"label identifier\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    label_push(&mut local_label_stack, tok, 2 as libc::c_int);
                    next();
                    if !(tok == ',' as i32) {
                        break;
                    }
                }
                skip(';' as i32);
            }
            while tok != '}' as i32 {
                decl(0x32 as libc::c_int);
                if tok != '}' as i32 {
                    if flags & 1 as libc::c_int != 0 {
                        vpop();
                    }
                    block(flags | 2 as libc::c_int);
                }
            }
            prev_scope(&mut o_0, flags & 1 as libc::c_int);
            if debug_modes != 0 {
                tcc_debug_stabn(tcc_state, N_RBRAC as libc::c_int, ind - func_ind);
            }
            if local_scope != 0 {
                next();
            } else if nocode_wanted == 0 {
                check_func_return();
            }
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_RETURN as libc::c_int {
            b = (func_vt.t & 0xf as libc::c_int != 0 as libc::c_int) as libc::c_int;
            if tok != ';' as i32 {
                gexpr();
                if b != 0 {
                    gen_assign_cast(&mut func_vt);
                } else {
                    if (*vtop).type_0.t != 0 as libc::c_int {
                        _tcc_warning(
                            b"void function returns a value\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    vtop = vtop.offset(-1);
                    vtop;
                }
            } else if b != 0 {
                _tcc_warning(
                    b"'return' with no value\0" as *const u8 as *const libc::c_char,
                );
                b = 0 as libc::c_int;
            }
            leave_scope(root_scope);
            if b != 0 {
                gfunc_return(&mut func_vt);
            }
            skip(';' as i32);
            if tok != '}' as i32 || local_scope != 1 as libc::c_int {
                rsym = gjmp_acs(rsym);
            }
            if debug_modes != 0 {
                tcc_tcov_block_end(tcc_state, -(1 as libc::c_int));
            }
            if nocode_wanted == 0 {
                nocode_wanted |= 0x20000000 as libc::c_int;
            }
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_BREAK as libc::c_int {
            if ((*cur_scope).bsym).is_null() {
                _tcc_error(b"cannot break\0" as *const u8 as *const libc::c_char);
            }
            if !cur_switch.is_null() && (*cur_scope).bsym == (*cur_switch).bsym {
                leave_scope((*cur_switch).scope);
            } else {
                leave_scope(loop_scope);
            }
            *(*cur_scope).bsym = gjmp_acs(*(*cur_scope).bsym);
            skip(';' as i32);
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_CONTINUE as libc::c_int {
            if ((*cur_scope).csym).is_null() {
                _tcc_error(b"cannot continue\0" as *const u8 as *const libc::c_char);
            }
            leave_scope(loop_scope);
            *(*cur_scope).csym = gjmp_acs(*(*cur_scope).csym);
            skip(';' as i32);
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_FOR as libc::c_int {
            new_scope(&mut o_0);
            skip('(' as i32);
            if tok != ';' as i32 {
                if decl(0x34 as libc::c_int) == 0 {
                    gexpr();
                    vpop();
                }
            }
            skip(';' as i32);
            b = 0 as libc::c_int;
            a = b;
            d = gind();
            c = d;
            if tok != ';' as i32 {
                gexpr();
                a = gvtst(1 as libc::c_int, 0 as libc::c_int);
            }
            skip(';' as i32);
            if tok != ')' as i32 {
                e = gjmp_acs(0 as libc::c_int);
                d = gind();
                gexpr();
                vpop();
                gjmp_addr_acs(c);
                gsym(e);
            }
            skip(')' as i32);
            lblock(&mut a, &mut b);
            gjmp_addr_acs(d);
            gsym_addr(b, d);
            gsym(a);
            prev_scope(&mut o_0, 0 as libc::c_int);
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_DO as libc::c_int {
            new_scope_s(&mut o_0);
            b = 0 as libc::c_int;
            a = b;
            d = gind();
            lblock(&mut a, &mut b);
            gsym(b);
            skip(TOK_WHILE as libc::c_int);
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            skip(';' as i32);
            c = gvtst(0 as libc::c_int, 0 as libc::c_int);
            gsym_addr(c, d);
            gsym(a);
            prev_scope_s(&mut o_0);
            current_block = 2626552447423186229;
            break;
        } else if t == TOK_SWITCH as libc::c_int {
            let mut sw: *mut switch_t = 0 as *mut switch_t;
            sw = tcc_mallocz(::core::mem::size_of::<switch_t>() as libc::c_ulong)
                as *mut switch_t;
            (*sw).bsym = &mut a;
            (*sw).scope = cur_scope;
            (*sw).prev = cur_switch;
            (*sw).nocode_wanted = nocode_wanted;
            cur_switch = sw;
            new_scope_s(&mut o_0);
            skip('(' as i32);
            gexpr();
            skip(')' as i32);
            if is_integer_btype((*vtop).type_0.t & 0xf as libc::c_int) == 0 {
                _tcc_error(
                    b"switch value not an integer\0" as *const u8 as *const libc::c_char,
                );
            }
            let fresh7 = vtop;
            vtop = vtop.offset(-1);
            (*sw).sv = *fresh7;
            a = 0 as libc::c_int;
            b = gjmp_acs(0 as libc::c_int);
            lblock(&mut a, 0 as *mut libc::c_int);
            a = gjmp_acs(a);
            gsym(b);
            prev_scope_s(&mut o_0);
            if !((*sw).nocode_wanted != 0) {
                case_sort(sw);
                (*sw).bsym = 0 as *mut libc::c_int;
                vpushv(&mut (*sw).sv);
                gv(0x1 as libc::c_int);
                d = gcase((*sw).p, (*sw).n, 0 as libc::c_int);
                vpop();
                if (*sw).def_sym != 0 {
                    gsym_addr(d, (*sw).def_sym);
                } else {
                    gsym(d);
                }
            }
            gsym(a);
            end_switch();
            current_block = 2626552447423186229;
            break;
        } else {
            if t == TOK_CASE as libc::c_int {
                let mut cr: *mut case_t = 0 as *mut case_t;
                if cur_switch.is_null() {
                    expect(b"switch\0" as *const u8 as *const libc::c_char);
                }
                cr = tcc_malloc(::core::mem::size_of::<case_t>() as libc::c_ulong)
                    as *mut case_t;
                dynarray_add(
                    &mut (*cur_switch).p as *mut *mut *mut case_t as *mut libc::c_void,
                    &mut (*cur_switch).n,
                    cr as *mut libc::c_void,
                );
                t = (*cur_switch).sv.type_0.t;
                (*cr).v2 = value64(expr_const64() as uint64_t, t) as int64_t;
                (*cr).v1 = (*cr).v2;
                if tok == 0xa1 as libc::c_int && (*tcc_state).gnu_ext as libc::c_int != 0
                {
                    next();
                    (*cr).v2 = value64(expr_const64() as uint64_t, t) as int64_t;
                    if case_cmp((*cr).v2 as uint64_t, (*cr).v1 as uint64_t)
                        < 0 as libc::c_int
                    {
                        _tcc_warning(
                            b"empty case range\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                if (*cur_switch).nocode_wanted == 0 {
                    (*cr).ind = gind();
                }
                (*cr).line = (*file).line_num;
                skip(':' as i32);
            } else if t == TOK_DEFAULT as libc::c_int {
                if cur_switch.is_null() {
                    expect(b"switch\0" as *const u8 as *const libc::c_char);
                }
                if (*cur_switch).def_sym != 0 {
                    _tcc_error(
                        b"too many 'default'\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*cur_switch)
                    .def_sym = if (*cur_switch).nocode_wanted != 0 {
                    -(1 as libc::c_int)
                } else {
                    gind()
                };
                skip(':' as i32);
            } else if t == TOK_GOTO as libc::c_int {
                vla_restore((*cur_scope).vla.locorig);
                if tok == '*' as i32 && (*tcc_state).gnu_ext as libc::c_int != 0 {
                    next();
                    gexpr();
                    if (*vtop).type_0.t & 0xf as libc::c_int != 5 as libc::c_int {
                        expect(b"pointer\0" as *const u8 as *const libc::c_char);
                    }
                    ggoto();
                } else if tok >= TOK_DEFINE as libc::c_int {
                    s = label_find(tok);
                    if s.is_null() {
                        s = label_push(&mut global_label_stack, tok, 1 as libc::c_int);
                    } else if (*s).r as libc::c_int == 2 as libc::c_int {
                        (*s).r = 1 as libc::c_int as libc::c_ushort;
                    }
                    if (*s).r as libc::c_int & 1 as libc::c_int != 0 {
                        if !((*cur_scope).cl.s).is_null() && nocode_wanted == 0 {
                            sym_push2(
                                &mut pending_gotos,
                                0x20000000 as libc::c_int,
                                0 as libc::c_int,
                                (*cur_scope).cl.n,
                            );
                            (*pending_gotos).prev_tok = s;
                            s = sym_push2(
                                &mut (*s).c2rust_unnamed_0.next,
                                0x20000000 as libc::c_int,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            );
                            (*pending_gotos).c2rust_unnamed_0.next = s;
                        }
                        (*s)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .jnext = gjmp_acs(
                            (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext,
                        );
                    } else {
                        try_call_cleanup_goto((*s).c2rust_unnamed_0.cleanupstate);
                        gjmp_addr_acs(
                            (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jind,
                        );
                    }
                    next();
                } else {
                    expect(b"label identifier\0" as *const u8 as *const libc::c_char);
                }
                skip(';' as i32);
                current_block = 2626552447423186229;
                break;
            } else if t == TOK_ASM1 as libc::c_int || t == TOK_ASM2 as libc::c_int
                || t == TOK_ASM3 as libc::c_int
            {
                asm_instr();
                current_block = 2626552447423186229;
                break;
            } else if tok == ':' as i32 && t >= TOK_DEFINE as libc::c_int {
                next();
                s = label_find(t);
                if !s.is_null() {
                    if (*s).r as libc::c_int == 0 as libc::c_int {
                        _tcc_error(
                            b"duplicate label '%s'\0" as *const u8
                                as *const libc::c_char,
                            get_tok_str((*s).v, 0 as *mut CValue),
                        );
                    }
                    (*s).r = 0 as libc::c_int as libc::c_ushort;
                    if !((*s).c2rust_unnamed_0.next).is_null() {
                        let mut pcl: *mut Sym = 0 as *mut Sym;
                        pcl = (*s).c2rust_unnamed_0.next;
                        while !pcl.is_null() {
                            gsym(
                                (*pcl).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext,
                            );
                            pcl = (*pcl).prev;
                        }
                        sym_pop(
                            &mut (*s).c2rust_unnamed_0.next,
                            0 as *mut Sym,
                            0 as libc::c_int,
                        );
                    } else {
                        gsym((*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jnext);
                    }
                } else {
                    s = label_push(&mut global_label_stack, t, 0 as libc::c_int);
                }
                (*s).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.jind = gind();
                (*s).c2rust_unnamed_0.cleanupstate = (*cur_scope).cl.s;
            } else if t != ';' as i32 {
                current_block = 16167632229894708628;
                break;
            } else {
                current_block = 2626552447423186229;
                break;
            }
            let mut ad_tmp: AttributeDef = AttributeDef {
                a: SymAttr {
                    aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                },
                f: FuncAttr {
                    func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
                },
                section: 0 as *mut Section,
                cleanup_func: 0 as *mut Sym,
                alias_target: 0,
                asm_label: 0,
                attr_mode: 0,
            };
            parse_attribute(&mut ad_tmp);
            if debug_modes != 0 {
                tcc_tcov_reset_ind(tcc_state);
            }
            vla_restore((*cur_scope).vla.loc);
            if tok != '}' as i32 {
                if !(0 as libc::c_int == flags & 2 as libc::c_int) {
                    current_block = 2626552447423186229;
                    break;
                }
            } else {
                (*tcc_state)
                    .warn_num = (&mut (*(0 as *mut TCCState)).warn_all
                    as *mut libc::c_uchar as size_t)
                    .wrapping_sub(
                        &mut (*(0 as *mut TCCState)).warn_none as *mut libc::c_uchar
                            as size_t,
                    ) as libc::c_uchar;
                (Some(
                    _tcc_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"deprecated use of label at end of compound statement\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 2626552447423186229;
                break;
            }
        }
    }
    match current_block {
        16167632229894708628 => {
            unget_tok(t);
            current_block = 15571315640078739593;
        }
        _ => {}
    }
    match current_block {
        15571315640078739593 => {
            if flags & 1 as libc::c_int != 0 {
                vpop();
                gexpr();
            } else {
                gexpr();
                vpop();
            }
            skip(';' as i32);
        }
        _ => {}
    }
    if debug_modes != 0 {
        tcc_tcov_check_line(tcc_state, 0 as libc::c_int);
        tcc_tcov_block_end(tcc_state, 0 as libc::c_int);
    }
}
unsafe extern "C" fn skip_or_save_block(mut str: *mut *mut TokenString) {
    let mut braces: libc::c_int = (tok == '{' as i32) as libc::c_int;
    let mut level: libc::c_int = 0 as libc::c_int;
    if !str.is_null() {
        *str = tok_str_alloc();
    }
    loop {
        let mut t: libc::c_int = tok;
        if level == 0 as libc::c_int
            && (t == ',' as i32 || t == ';' as i32 || t == '}' as i32 || t == ')' as i32
                || t == ']' as i32)
        {
            break;
        }
        if t == -(1 as libc::c_int) {
            if !(!str.is_null() || level > 0 as libc::c_int) {
                break;
            }
            _tcc_error(b"unexpected end of file\0" as *const u8 as *const libc::c_char);
        } else {
            if !str.is_null() {
                tok_str_add_tok(*str);
            }
            next();
            if t == '{' as i32 || t == '(' as i32 || t == '[' as i32 {
                level += 1;
                level;
            } else {
                if !(t == '}' as i32 || t == ')' as i32 || t == ']' as i32) {
                    continue;
                }
                level -= 1;
                level;
                if level == 0 as libc::c_int && braces != 0 && t == '}' as i32 {
                    break;
                }
            }
        }
    }
    if !str.is_null() {
        tok_str_add(*str, -(1 as libc::c_int));
    }
}
unsafe extern "C" fn parse_init_elem(mut expr_type_0: libc::c_int) {
    let mut saved_global_expr: libc::c_int = 0;
    match expr_type_0 {
        1 => {
            saved_global_expr = global_expr;
            global_expr = 1 as libc::c_int;
            expr_const1();
            global_expr = saved_global_expr;
            if (*vtop).r as libc::c_int & (0x3f as libc::c_int | 0x100 as libc::c_int)
                != 0x30 as libc::c_int
                && ((*vtop).r as libc::c_int
                    & (0x200 as libc::c_int | 0x100 as libc::c_int)
                    != 0x200 as libc::c_int | 0x100 as libc::c_int
                    || (*(*vtop).c2rust_unnamed_0.sym).v < 0x10000000 as libc::c_int)
            {
                _tcc_error(
                    b"initializer element is not constant\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        2 => {
            expr_eq();
        }
        _ => {}
    };
}
unsafe extern "C" fn init_assert(mut p: *mut init_params, mut offset: libc::c_int) {
    if if !((*p).sec).is_null() {
        (!(nocode_wanted > 0 as libc::c_int)
            && offset as libc::c_ulong > (*(*p).sec).data_offset) as libc::c_int
    } else {
        (nocode_wanted == 0 && offset > (*p).local_offset) as libc::c_int
    } != 0
    {
        _tcc_error(
            b"internal compiler error in %s:%d: %s\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"init_assert\0"))
                .as_ptr(),
            7444 as libc::c_int,
            b"initializer overflow\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn init_putz(
    mut p: *mut init_params,
    mut c: libc::c_ulong,
    mut size: libc::c_int,
) {
    init_assert(p, c.wrapping_add(size as libc::c_ulong) as libc::c_int);
    if ((*p).sec).is_null() {
        vpush_helper_func(TOK_memset as libc::c_int);
        vseti(0x32 as libc::c_int, c as libc::c_int);
        vpushi(0 as libc::c_int);
        vpushs(size as Elf64_Addr);
        gfunc_call(3 as libc::c_int);
    }
}
unsafe extern "C" fn decl_design_delrels(
    mut sec: *mut Section,
    mut c: libc::c_int,
    mut size: libc::c_int,
) {
    let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut rel2: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    let mut rel_end: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
    if sec.is_null() || ((*sec).reloc).is_null() {
        return;
    }
    rel2 = (*(*sec).reloc).data as *mut Elf64_Rela;
    rel = rel2;
    rel_end = ((*(*sec).reloc).data).offset((*(*sec).reloc).data_offset as isize)
        as *mut Elf64_Rela;
    while rel < rel_end {
        if (*rel).r_offset >= c as Elf64_Addr
            && (*rel).r_offset < (c + size) as Elf64_Addr
        {
            (*(*sec).reloc)
                .data_offset = ((*(*sec).reloc).data_offset)
                .wrapping_sub(::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong);
        } else {
            if rel2 != rel {
                memcpy(
                    rel2 as *mut libc::c_void,
                    rel as *const libc::c_void,
                    ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong,
                );
            }
            rel2 = rel2.offset(1);
            rel2;
        }
        rel = rel.offset(1);
        rel;
    }
}
unsafe extern "C" fn decl_design_flex(
    mut p: *mut init_params,
    mut ref_0: *mut Sym,
    mut index: libc::c_int,
) {
    if ref_0 == (*p).flex_array_ref {
        if index >= (*ref_0).c2rust_unnamed.c2rust_unnamed.c {
            (*ref_0).c2rust_unnamed.c2rust_unnamed.c = index + 1 as libc::c_int;
        }
    } else if (*ref_0).c2rust_unnamed.c2rust_unnamed.c < 0 as libc::c_int {
        _tcc_error(
            b"flexible array has zero size in this context\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn decl_designator(
    mut p: *mut init_params,
    mut type_0: *mut CType,
    mut c: libc::c_ulong,
    mut cur_field: *mut *mut Sym,
    mut flags: libc::c_int,
    mut al: libc::c_int,
) -> libc::c_int {
    let mut cumofs: libc::c_int = 0;
    let mut current_block: u64;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut f: *mut Sym = 0 as *mut Sym;
    let mut index: libc::c_int = 0;
    let mut index_last: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut nb_elems: libc::c_int = 0;
    let mut elem_size: libc::c_int = 0;
    let mut corig: libc::c_ulong = c;
    elem_size = 0 as libc::c_int;
    nb_elems = 1 as libc::c_int;
    if flags & 4 as libc::c_int != 0 {
        current_block = 1038783935948014023;
    } else {
        if (*tcc_state).gnu_ext as libc::c_int != 0 && tok >= TOK_DEFINE as libc::c_int {
            l = tok;
            next();
            if tok == ':' as i32 {
                current_block = 9611968974467953790;
            } else {
                unget_tok(l);
                current_block = 10879442775620481940;
            }
        } else {
            current_block = 10879442775620481940;
        }
        loop {
            match current_block {
                10879442775620481940 => {
                    if !(nb_elems == 1 as libc::c_int
                        && (tok == '[' as i32 || tok == '.' as i32))
                    {
                        break;
                    }
                    if tok == '[' as i32 {
                        if (*type_0).t & 0x40 as libc::c_int == 0 {
                            expect(b"array type\0" as *const u8 as *const libc::c_char);
                        }
                        next();
                        index_last = expr_const();
                        index = index_last;
                        if tok == 0xa1 as libc::c_int
                            && (*tcc_state).gnu_ext as libc::c_int != 0
                        {
                            next();
                            index_last = expr_const();
                        }
                        skip(']' as i32);
                        s = (*type_0).ref_0;
                        decl_design_flex(p, s, index_last);
                        if index < 0 as libc::c_int
                            || index_last >= (*s).c2rust_unnamed.c2rust_unnamed.c
                            || index_last < index
                        {
                            _tcc_error(
                                b"index exceeds array bounds or range is empty\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        if !cur_field.is_null() {
                            (**cur_field).c2rust_unnamed.c2rust_unnamed.c = index_last;
                        }
                        type_0 = pointed_type(type_0);
                        elem_size = type_size(type_0, &mut align);
                        c = c.wrapping_add((index * elem_size) as libc::c_ulong);
                        nb_elems = index_last - index + 1 as libc::c_int;
                    } else {
                        cumofs = 0;
                        next();
                        l = tok;
                        current_block = 9611968974467953790;
                        continue;
                    }
                }
                _ => {
                    next();
                    f = find_field(type_0, l, &mut cumofs);
                    if !cur_field.is_null() {
                        *cur_field = f;
                    }
                    type_0 = &mut (*f).type_0;
                    c = c.wrapping_add(cumofs as libc::c_ulong);
                }
            }
            cur_field = 0 as *mut *mut Sym;
            current_block = 10879442775620481940;
        }
        if cur_field.is_null() {
            if tok == '=' as i32 {
                next();
            } else if (*tcc_state).gnu_ext == 0 {
                expect(b"=\0" as *const u8 as *const libc::c_char);
            }
            current_block = 14945149239039849694;
        } else {
            current_block = 1038783935948014023;
        }
    }
    match current_block {
        1038783935948014023 => {
            if (*type_0).t & 0x40 as libc::c_int != 0 {
                index = (**cur_field).c2rust_unnamed.c2rust_unnamed.c;
                s = (*type_0).ref_0;
                decl_design_flex(p, s, index);
                if index >= (*s).c2rust_unnamed.c2rust_unnamed.c {
                    _tcc_error(
                        b"too many initializers\0" as *const u8 as *const libc::c_char,
                    );
                }
                type_0 = pointed_type(type_0);
                elem_size = type_size(type_0, &mut align);
                c = c.wrapping_add((index * elem_size) as libc::c_ulong);
            } else {
                f = *cur_field;
                while !f.is_null() && (*f).v & 0x10000000 as libc::c_int != 0
                    && is_integer_btype((*f).type_0.t & 0xf as libc::c_int) != 0
                {
                    f = (*f).c2rust_unnamed_0.next;
                    *cur_field = f;
                }
                if f.is_null() {
                    _tcc_error(
                        b"too many initializers\0" as *const u8 as *const libc::c_char,
                    );
                }
                type_0 = &mut (*f).type_0;
                c = c
                    .wrapping_add((*f).c2rust_unnamed.c2rust_unnamed.c as libc::c_ulong);
            }
        }
        _ => {}
    }
    if elem_size == 0 {
        elem_size = type_size(type_0, &mut align);
    }
    if flags & 2 as libc::c_int == 0 && c.wrapping_sub(corig) < al as libc::c_ulong {
        decl_design_delrels((*p).sec, c as libc::c_int, elem_size * nb_elems);
        flags &= !(8 as libc::c_int);
    }
    decl_initializer(p, type_0, c, flags & !(1 as libc::c_int));
    if flags & 2 as libc::c_int == 0 && nb_elems > 1 as libc::c_int {
        let mut aref: Sym = {
            let mut init = Sym {
                v: 0 as libc::c_int,
                r: 0,
                a: SymAttr {
                    aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
                },
                c2rust_unnamed: C2RustUnnamed_1 {
                    c2rust_unnamed: C2RustUnnamed_2 {
                        c: 0,
                        c2rust_unnamed: C2RustUnnamed_3 { sym_scope: 0 },
                    },
                },
                type_0: CType {
                    t: 0,
                    ref_0: 0 as *const Sym as *mut Sym,
                },
                c2rust_unnamed_0: C2RustUnnamed_0 {
                    next: 0 as *mut Sym,
                },
                prev: 0 as *mut Sym,
                prev_tok: 0 as *mut Sym,
            };
            init
        };
        let mut t1: CType = CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        };
        let mut i: libc::c_int = 0;
        if !((*p).sec).is_null() || (*type_0).t & 0x40 as libc::c_int != 0 {
            aref.c2rust_unnamed.c2rust_unnamed.c = elem_size;
            t1.t = 7 as libc::c_int;
            t1.ref_0 = &mut aref;
            type_0 = &mut t1;
        }
        if !((*p).sec).is_null() {
            vpush_ref(type_0, (*p).sec, c, elem_size as libc::c_ulong);
        } else {
            vset(type_0, 0x32 as libc::c_int | 0x100 as libc::c_int, c as libc::c_int);
        }
        i = 1 as libc::c_int;
        while i < nb_elems {
            vdup();
            init_putv(p, type_0, c.wrapping_add((elem_size * i) as libc::c_ulong));
            i += 1;
            i;
        }
        vpop();
    }
    c = c.wrapping_add((nb_elems * elem_size) as libc::c_ulong);
    if c.wrapping_sub(corig) > al as libc::c_ulong {
        al = c.wrapping_sub(corig) as libc::c_int;
    }
    return al;
}
unsafe extern "C" fn init_putv(
    mut p: *mut init_params,
    mut type_0: *mut CType,
    mut c: libc::c_ulong,
) {
    let mut bt: libc::c_int = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dtype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut sec: *mut Section = (*p).sec;
    let mut val: uint64_t = 0;
    dtype = *type_0;
    dtype.t &= !(0x100 as libc::c_int);
    size = type_size(type_0, &mut align);
    if (*type_0).t & 0x80 as libc::c_int != 0 {
        size = (((*type_0).t >> 20 as libc::c_int & 0x3f as libc::c_int)
            + ((*type_0).t >> 20 as libc::c_int + 6 as libc::c_int & 0x3f as libc::c_int)
            + 7 as libc::c_int) / 8 as libc::c_int;
    }
    init_assert(p, c.wrapping_add(size as libc::c_ulong) as libc::c_int);
    if !sec.is_null() {
        gen_assign_cast(&mut dtype);
        bt = (*type_0).t & 0xf as libc::c_int;
        if (*vtop).r as libc::c_int & 0x200 as libc::c_int != 0 && bt != 5 as libc::c_int
            && (bt
                != (if 8 as libc::c_int == 8 as libc::c_int {
                    4 as libc::c_int
                } else {
                    3 as libc::c_int
                }) || (*type_0).t & 0x80 as libc::c_int != 0)
            && !((*vtop).r as libc::c_int & 0x30 as libc::c_int != 0
                && (*(*vtop).c2rust_unnamed_0.sym).v >= 0x10000000 as libc::c_int)
        {
            _tcc_error(
                b"initializer element is not computable at load time\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if nocode_wanted > 0 as libc::c_int {
            vtop = vtop.offset(-1);
            vtop;
            return;
        }
        ptr = ((*sec).data).offset(c as isize) as *mut libc::c_void;
        val = (*vtop).c2rust_unnamed.c.i;
        if (*vtop).r as libc::c_int & (0x200 as libc::c_int | 0x30 as libc::c_int)
            == 0x200 as libc::c_int | 0x30 as libc::c_int
            && (*(*vtop).c2rust_unnamed_0.sym).v >= 0x10000000 as libc::c_int
            && (*vtop).type_0.t & 0xf as libc::c_int != 5 as libc::c_int
        {
            let mut ssec: *mut Section = 0 as *mut Section;
            let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
            let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
            esym = elfsym((*vtop).c2rust_unnamed_0.sym);
            ssec = *((*tcc_state).sections).offset((*esym).st_shndx as isize);
            memmove(
                ptr,
                ((*ssec).data)
                    .offset((*esym).st_value as isize)
                    .offset((*vtop).c2rust_unnamed.c.i as libc::c_int as isize)
                    as *const libc::c_void,
                size as libc::c_ulong,
            );
            if !((*ssec).reloc).is_null() {
                let mut relofs: libc::c_ulong = (*(*ssec).reloc).data_offset;
                while relofs >= ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong {
                    relofs = relofs
                        .wrapping_sub(
                            ::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong,
                        );
                    rel = ((*(*ssec).reloc).data).offset(relofs as isize)
                        as *mut Elf64_Rela;
                    if (*rel).r_offset
                        >= ((*esym).st_value).wrapping_add(size as Elf64_Addr)
                    {
                        continue;
                    }
                    if (*rel).r_offset < (*esym).st_value {
                        break;
                    }
                    put_elf_reloca(
                        (*tcc_state).c2rust_unnamed.symtab_section,
                        sec,
                        c.wrapping_add((*rel).r_offset).wrapping_sub((*esym).st_value),
                        ((*rel).r_info & 0xffffffff as libc::c_uint as Elf64_Xword)
                            as libc::c_int,
                        ((*rel).r_info >> 32 as libc::c_int) as libc::c_int,
                        (*rel).r_addend as Elf64_Addr,
                    );
                }
            }
        } else if (*type_0).t & 0x80 as libc::c_int != 0 {
            let mut bit_pos: libc::c_int = 0;
            let mut bit_size: libc::c_int = 0;
            let mut bits: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            let mut p_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut v: libc::c_uchar = 0;
            let mut m: libc::c_uchar = 0;
            bit_pos = (*vtop).type_0.t >> 20 as libc::c_int & 0x3f as libc::c_int;
            bit_size = (*vtop).type_0.t >> 20 as libc::c_int + 6 as libc::c_int
                & 0x3f as libc::c_int;
            p_0 = (ptr as *mut libc::c_uchar)
                .offset((bit_pos >> 3 as libc::c_int) as isize);
            bit_pos &= 7 as libc::c_int;
            bits = 0 as libc::c_int;
            while bit_size != 0 {
                n = 8 as libc::c_int - bit_pos;
                if n > bit_size {
                    n = bit_size;
                }
                v = (val >> bits << bit_pos) as libc::c_uchar;
                m = ((((1 as libc::c_int) << n) - 1 as libc::c_int) << bit_pos)
                    as libc::c_uchar;
                *p_0 = (*p_0 as libc::c_int & !(m as libc::c_int)
                    | v as libc::c_int & m as libc::c_int) as libc::c_uchar;
                bits += n;
                bit_size -= n;
                bit_pos = 0 as libc::c_int;
                p_0 = p_0.offset(1);
                p_0;
            }
        } else {
            match bt {
                11 => {
                    *(ptr
                        as *mut libc::c_char) = (val != 0 as libc::c_int as uint64_t)
                        as libc::c_int as libc::c_char;
                }
                1 => {
                    *(ptr as *mut libc::c_char) = val as libc::c_char;
                }
                2 => {
                    write16le(ptr as *mut libc::c_uchar, val as uint16_t);
                }
                8 => {
                    write32le(ptr as *mut libc::c_uchar, val as uint32_t);
                }
                9 => {
                    write64le(ptr as *mut libc::c_uchar, val);
                }
                10 => {
                    if ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                        >= 10 as libc::c_int as libc::c_ulong
                    {
                        memcpy(
                            ptr,
                            &mut (*vtop).c2rust_unnamed.c.ld as *mut f128::f128
                                as *const libc::c_void,
                            10 as libc::c_int as libc::c_ulong,
                        );
                    } else if ::core::mem::size_of::<f128::f128>() as libc::c_ulong
                        == 16 as libc::c_int as libc::c_ulong
                    {
                        memcpy(
                            ptr,
                            &mut (*vtop).c2rust_unnamed.c.ld as *mut f128::f128
                                as *const libc::c_void,
                            16 as libc::c_int as libc::c_ulong,
                        );
                    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
                        == 16 as libc::c_int as libc::c_ulong
                    {
                        *(ptr
                            as *mut libc::c_double) = ((*vtop).c2rust_unnamed.c.ld)
                            .to_f64()
                            .unwrap();
                    } else if 0 as libc::c_int
                        == memcmp(
                            ptr,
                            &mut (*vtop).c2rust_unnamed.c.ld as *mut f128::f128
                                as *const libc::c_void,
                            16 as libc::c_int as libc::c_ulong,
                        )
                    {} else {
                        _tcc_error(
                            b"can't cross compile long double constants\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                4 | 5 => {
                    if (*vtop).r as libc::c_int & 0x200 as libc::c_int != 0 {
                        greloca(
                            sec,
                            (*vtop).c2rust_unnamed_0.sym,
                            c,
                            1 as libc::c_int,
                            val,
                        );
                    } else {
                        write64le(ptr as *mut libc::c_uchar, val);
                    }
                }
                3 => {
                    write32le(ptr as *mut libc::c_uchar, val as uint32_t);
                }
                _ => {}
            }
        }
        vtop = vtop.offset(-1);
        vtop;
    } else {
        vset(&mut dtype, 0x32 as libc::c_int | 0x100 as libc::c_int, c as libc::c_int);
        vswap();
        vstore();
        vpop();
    };
}
unsafe extern "C" fn decl_initializer(
    mut p: *mut init_params,
    mut type_0: *mut CType,
    mut c: libc::c_ulong,
    mut flags: libc::c_int,
) {
    let mut len: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut no_oblock: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut size1: libc::c_int = 0;
    let mut align1: libc::c_int = 0;
    let mut s: *mut Sym = 0 as *mut Sym;
    let mut f: *mut Sym = 0 as *mut Sym;
    let mut indexsym: Sym = Sym {
        v: 0,
        r: 0,
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        c2rust_unnamed: C2RustUnnamed_1 {
            c2rust_unnamed: C2RustUnnamed_2 {
                c: 0,
                c2rust_unnamed: C2RustUnnamed_3 { sym_scope: 0 },
            },
        },
        type_0: CType {
            t: 0,
            ref_0: 0 as *const Sym as *mut Sym,
        },
        c2rust_unnamed_0: C2RustUnnamed_0 {
            next: 0 as *mut Sym,
        },
        prev: 0 as *mut Sym,
        prev_tok: 0 as *mut Sym,
    };
    let mut t1: *mut CType = 0 as *mut CType;
    if debug_modes as libc::c_int != 0 && flags & 2 as libc::c_int == 0
        && ((*p).sec).is_null()
    {
        tcc_debug_line(tcc_state);
        tcc_tcov_check_line(tcc_state, 1 as libc::c_int);
    }
    if flags & 4 as libc::c_int == 0 && tok != '{' as i32 && tok != 0xc9 as libc::c_int
        && tok != 0xc8 as libc::c_int
        && (flags & 2 as libc::c_int == 0
            || (*type_0).t & 0xf as libc::c_int == 7 as libc::c_int)
    {
        let mut ncw_prev: libc::c_int = nocode_wanted;
        if flags & 2 as libc::c_int != 0 && ((*p).sec).is_null() {
            nocode_wanted += 1;
            nocode_wanted;
        }
        parse_init_elem(
            if ((*p).sec).is_null() { 2 as libc::c_int } else { 1 as libc::c_int },
        );
        nocode_wanted = ncw_prev;
        flags |= 4 as libc::c_int;
    }
    let mut current_block_103: u64;
    if (*type_0).t & 0x40 as libc::c_int != 0 {
        no_oblock = 1 as libc::c_int;
        if flags & 1 as libc::c_int != 0 && tok != 0xc9 as libc::c_int
            && tok != 0xc8 as libc::c_int || tok == '{' as i32
        {
            skip('{' as i32);
            no_oblock = 0 as libc::c_int;
        }
        s = (*type_0).ref_0;
        n = (*s).c2rust_unnamed.c2rust_unnamed.c;
        t1 = pointed_type(type_0);
        size1 = type_size(t1, &mut align1);
        if tok == 0xc9 as libc::c_int && (*t1).t & 0xf as libc::c_int == 3 as libc::c_int
            || tok == 0xc8 as libc::c_int
                && (*t1).t & 0xf as libc::c_int == 1 as libc::c_int
        {
            len = 0 as libc::c_int;
            cstr_reset(&mut initstr);
            if size1 as libc::c_ulong
                != (if tok == 0xc8 as libc::c_int {
                    1 as libc::c_int as libc::c_ulong
                } else {
                    ::core::mem::size_of::<nwchar_t>() as libc::c_ulong
                })
            {
                _tcc_error(
                    b"unhandled string literal merging\0" as *const u8
                        as *const libc::c_char,
                );
            }
            while tok == 0xc8 as libc::c_int || tok == 0xc9 as libc::c_int {
                if initstr.size != 0 {
                    initstr.size -= size1;
                }
                if tok == 0xc8 as libc::c_int {
                    len += tokc.str_0.size;
                } else {
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            (tokc.str_0.size as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<nwchar_t>() as libc::c_ulong,
                                ),
                        ) as libc::c_int as libc::c_int;
                }
                len -= 1;
                len;
                cstr_cat(&mut initstr, tokc.str_0.data, tokc.str_0.size);
                next();
            }
            if tok != ')' as i32 && tok != '}' as i32 && tok != ',' as i32
                && tok != ';' as i32 && tok != -(1 as libc::c_int)
            {
                unget_tok(
                    if size1 == 1 as libc::c_int {
                        0xc8 as libc::c_int
                    } else {
                        0xc9 as libc::c_int
                    },
                );
                tokc.str_0.size = initstr.size;
                tokc.str_0.data = initstr.data;
                current_block_103 = 7668052626448690937;
            } else {
                decl_design_flex(p, s, len);
                if flags & 2 as libc::c_int == 0 {
                    let mut nb: libc::c_int = n;
                    let mut ch: libc::c_int = 0;
                    if len < nb {
                        nb = len;
                    }
                    if len > nb {
                        _tcc_warning(
                            b"initializer-string for array is too long\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if !((*p).sec).is_null() && size1 == 1 as libc::c_int {
                        init_assert(
                            p,
                            c.wrapping_add(nb as libc::c_ulong) as libc::c_int,
                        );
                        if !(nocode_wanted > 0 as libc::c_int) {
                            memcpy(
                                ((*(*p).sec).data).offset(c as isize) as *mut libc::c_void,
                                initstr.data as *const libc::c_void,
                                nb as libc::c_ulong,
                            );
                        }
                    } else {
                        i = 0 as libc::c_int;
                        while i < n {
                            if i >= nb {
                                if flags & 8 as libc::c_int != 0 {
                                    break;
                                }
                                if n - i >= 4 as libc::c_int {
                                    init_putz(
                                        p,
                                        c.wrapping_add((i * size1) as libc::c_ulong),
                                        (n - i) * size1,
                                    );
                                    break;
                                } else {
                                    ch = 0 as libc::c_int;
                                }
                            } else if size1 == 1 as libc::c_int {
                                ch = *(initstr.data as *mut libc::c_uchar)
                                    .offset(i as isize) as libc::c_int;
                            } else {
                                ch = *(initstr.data as *mut nwchar_t).offset(i as isize);
                            }
                            vpushi(ch);
                            init_putv(
                                p,
                                t1,
                                c.wrapping_add((i * size1) as libc::c_ulong),
                            );
                            i += 1;
                            i;
                        }
                    }
                }
                current_block_103 = 777662472977924419;
            }
        } else {
            current_block_103 = 7668052626448690937;
        }
        match current_block_103 {
            777662472977924419 => {}
            _ => {
                indexsym.c2rust_unnamed.c2rust_unnamed.c = 0 as libc::c_int;
                f = &mut indexsym;
                current_block_103 = 12582430263575324938;
            }
        }
    } else {
        if flags & 4 as libc::c_int != 0
            && is_compatible_unqualified_types(type_0, &mut (*vtop).type_0) != 0
        {
            current_block_103 = 2824481645768795782;
        } else if (*type_0).t & 0xf as libc::c_int == 7 as libc::c_int {
            no_oblock = 1 as libc::c_int;
            if flags & 1 as libc::c_int != 0 || tok == '{' as i32 {
                skip('{' as i32);
                no_oblock = 0 as libc::c_int;
            }
            s = (*type_0).ref_0;
            f = (*s).c2rust_unnamed_0.next;
            n = (*s).c2rust_unnamed.c2rust_unnamed.c;
            size1 = 1 as libc::c_int;
            current_block_103 = 12582430263575324938;
        } else if tok == '{' as i32 {
            if flags & 4 as libc::c_int != 0 {
                skip(';' as i32);
            }
            next();
            decl_initializer(p, type_0, c, flags & !(4 as libc::c_int));
            skip('}' as i32);
            current_block_103 = 796174441944384681;
        } else {
            current_block_103 = 2824481645768795782;
        }
        match current_block_103 {
            12582430263575324938 => {}
            796174441944384681 => {}
            _ => {
                if flags & 2 as libc::c_int != 0 {
                    if flags & 4 as libc::c_int != 0 {
                        vpop();
                    } else {
                        skip_or_save_block(0 as *mut *mut TokenString);
                    }
                } else {
                    if flags & 4 as libc::c_int == 0 {
                        if tok != 0xc8 as libc::c_int && tok != 0xc9 as libc::c_int {
                            expect(
                                b"string constant\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        parse_init_elem(
                            if ((*p).sec).is_null() {
                                2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                        );
                    }
                    if ((*p).sec).is_null() && flags & 8 as libc::c_int != 0
                        && (*vtop).r as libc::c_int
                            & (0x3f as libc::c_int | 0x100 as libc::c_int
                                | 0x200 as libc::c_int) == 0x30 as libc::c_int
                        && (*vtop).c2rust_unnamed.c.i == 0 as libc::c_int as uint64_t
                        && btype_size((*type_0).t & 0xf as libc::c_int) != 0
                    {
                        vpop();
                    } else {
                        init_putv(p, type_0, c);
                    }
                }
                current_block_103 = 796174441944384681;
            }
        }
    }
    match current_block_103 {
        12582430263575324938 => {
            if flags & (8 as libc::c_int | 2 as libc::c_int) == 0 {
                init_putz(p, c, n * size1);
                flags |= 8 as libc::c_int;
            }
            len = 0 as libc::c_int;
            decl_design_flex(p, s, len);
            while tok != '}' as i32 || flags & 4 as libc::c_int != 0 {
                len = decl_designator(p, type_0, c, &mut f, flags, len);
                flags &= !(4 as libc::c_int);
                if (*type_0).t & 0x40 as libc::c_int != 0 {
                    indexsym.c2rust_unnamed.c2rust_unnamed.c += 1;
                    indexsym.c2rust_unnamed.c2rust_unnamed.c;
                    if no_oblock != 0 && len >= n * size1 {
                        break;
                    }
                } else {
                    if (*s).type_0.t
                        == (1 as libc::c_int) << 20 as libc::c_int | 7 as libc::c_int
                    {
                        f = 0 as *mut Sym;
                    } else {
                        f = (*f).c2rust_unnamed_0.next;
                    }
                    if no_oblock != 0 && f.is_null() {
                        break;
                    }
                }
                if tok == '}' as i32 {
                    break;
                }
                skip(',' as i32);
            }
            current_block_103 = 777662472977924419;
        }
        _ => {}
    }
    match current_block_103 {
        777662472977924419 => {
            if no_oblock == 0 {
                skip('}' as i32);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn decl_initializer_alloc(
    mut type_0: *mut CType,
    mut ad: *mut AttributeDef,
    mut r: libc::c_int,
    mut has_init: libc::c_int,
    mut v: libc::c_int,
    mut global: libc::c_int,
) {
    let mut current_block: u64;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut addr: libc::c_int = 0;
    let mut init_str: *mut TokenString = 0 as *mut TokenString;
    let mut sec: *mut Section = 0 as *mut Section;
    let mut flexible_array: *mut Sym = 0 as *mut Sym;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut saved_nocode_wanted: libc::c_int = nocode_wanted;
    let mut bcheck: libc::c_int = ((*tcc_state).do_bounds_check as libc::c_int != 0
        && !(nocode_wanted > 0 as libc::c_int)) as libc::c_int;
    let mut p: init_params = {
        let mut init = init_params {
            sec: 0 as *mut Section,
            local_offset: 0,
            flex_array_ref: 0 as *mut Sym,
        };
        init
    };
    if v != 0 && r & 0x3f as libc::c_int == 0x30 as libc::c_int {
        nocode_wanted = (nocode_wanted as libc::c_uint | 0x80000000 as libc::c_uint)
            as libc::c_int;
    }
    flexible_array = 0 as *mut Sym;
    size = type_size(type_0, &mut align);
    if size < 0 as libc::c_int {
        if (*type_0).t & 0x40 as libc::c_int == 0 {
            _tcc_error(
                b"initialization of incomplete type\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*type_0)
            .ref_0 = sym_push(
            0x20000000 as libc::c_int,
            &mut (*(*type_0).ref_0).type_0,
            0 as libc::c_int,
            (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
        p.flex_array_ref = (*type_0).ref_0;
    } else if has_init != 0 && (*type_0).t & 0xf as libc::c_int == 7 as libc::c_int {
        let mut field: *mut Sym = (*(*type_0).ref_0).c2rust_unnamed_0.next;
        if !field.is_null() {
            while !((*field).c2rust_unnamed_0.next).is_null() {
                field = (*field).c2rust_unnamed_0.next;
            }
            if (*field).type_0.t & 0x40 as libc::c_int != 0
                && (*(*field).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                    < 0 as libc::c_int
            {
                flexible_array = field;
                p.flex_array_ref = (*field).type_0.ref_0;
                size = -(1 as libc::c_int);
            }
        }
    }
    if size < 0 as libc::c_int {
        if has_init == 0 {
            _tcc_error(b"unknown type size\0" as *const u8 as *const libc::c_char);
        }
        if has_init == 2 as libc::c_int {
            init_str = tok_str_alloc();
            while tok == 0xc8 as libc::c_int || tok == 0xc9 as libc::c_int {
                tok_str_add_tok(init_str);
                next();
            }
            tok_str_add(init_str, -(1 as libc::c_int));
        } else {
            skip_or_save_block(&mut init_str);
        }
        unget_tok(0 as libc::c_int);
        begin_macro(init_str, 1 as libc::c_int);
        next();
        decl_initializer(
            &mut p,
            type_0,
            0 as libc::c_int as libc::c_ulong,
            1 as libc::c_int | 2 as libc::c_int,
        );
        macro_ptr = (*init_str).str_0;
        next();
        size = type_size(type_0, &mut align);
        if size < 0 as libc::c_int {
            _tcc_error(b"unknown type size\0" as *const u8 as *const libc::c_char);
        }
        if !flexible_array.is_null()
            && (*(*flexible_array).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                > 0 as libc::c_int
        {
            size
                += (*(*flexible_array).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                    * pointed_size(&mut (*flexible_array).type_0);
        }
    }
    if ((*ad).a).aligned() != 0 {
        let mut speca: libc::c_int = (1 as libc::c_int)
            << ((*ad).a).aligned() as libc::c_int - 1 as libc::c_int;
        if speca > align {
            align = speca;
        }
    } else if ((*ad).a).packed() != 0 {
        align = 1 as libc::c_int;
    }
    if v == 0 && nocode_wanted > 0 as libc::c_int {
        size = 0 as libc::c_int;
        align = 1 as libc::c_int;
    }
    if r & 0x3f as libc::c_int == 0x32 as libc::c_int {
        sec = 0 as *mut Section;
        if bcheck != 0 && v != 0 {
            loc -= align;
        }
        loc = loc - size & -align;
        addr = loc;
        p.local_offset = addr + size;
        if bcheck != 0 && v != 0 {
            loc -= align;
        }
        if v != 0 {
            if (*ad).asm_label != 0 {
                let mut reg: libc::c_int = asm_parse_regvar((*ad).asm_label);
                if reg >= 0 as libc::c_int {
                    r = r & !(0x3f as libc::c_int) | reg;
                }
            }
            sym = sym_push(v, type_0, r, addr);
            if !((*ad).cleanup_func).is_null() {
                (*cur_scope).cl.n += 1;
                let mut cls: *mut Sym = sym_push2(
                    &mut all_cleanups,
                    0x20000000 as libc::c_int | (*cur_scope).cl.n,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                (*cls).prev_tok = sym;
                (*cls).c2rust_unnamed.cleanup_func = (*ad).cleanup_func;
                (*cls).c2rust_unnamed_0.next = (*cur_scope).cl.s;
                (*cur_scope).cl.s = cls;
            }
            (*sym).a = (*ad).a;
        } else {
            vset(type_0, r, addr);
        }
        current_block = 3024367268842933116;
    } else {
        sym = 0 as *mut Sym;
        if v != 0 && global != 0 {
            sym = sym_find(v);
            if !sym.is_null() {
                if !(p.flex_array_ref).is_null()
                    && (*sym).type_0.t & (*type_0).t & 0x40 as libc::c_int != 0
                    && (*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                        > (*(*type_0).ref_0).c2rust_unnamed.c2rust_unnamed.c
                {
                    (*(*type_0).ref_0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c = (*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c;
                    size = type_size(type_0, &mut align);
                }
                patch_storage(sym, ad, type_0);
                if has_init == 0 && (*sym).c2rust_unnamed.c2rust_unnamed.c != 0
                    && (*elfsym(sym)).st_shndx as libc::c_int != 0 as libc::c_int
                {
                    current_block = 12970108193173112931;
                } else {
                    current_block = 17020603795727957434;
                }
            } else {
                current_block = 17020603795727957434;
            }
        } else {
            current_block = 17020603795727957434;
        }
        match current_block {
            12970108193173112931 => {}
            _ => {
                sec = (*ad).section;
                if sec.is_null() {
                    let mut tp: *mut CType = type_0;
                    while (*tp).t & (0xf as libc::c_int | 0x40 as libc::c_int)
                        == 5 as libc::c_int | 0x40 as libc::c_int
                    {
                        tp = &mut (*(*tp).ref_0).type_0;
                    }
                    if (*tp).t & 0x100 as libc::c_int != 0 {
                        sec = (*tcc_state).rodata_section;
                    } else if has_init != 0 {
                        sec = (*tcc_state).data_section;
                    } else if (*tcc_state).nocommon != 0 {
                        sec = (*tcc_state).bss_section;
                    }
                }
                if !sec.is_null() {
                    addr = section_add(sec, size as Elf64_Addr, align) as libc::c_int;
                    if bcheck != 0 {
                        section_add(
                            sec,
                            1 as libc::c_int as Elf64_Addr,
                            1 as libc::c_int,
                        );
                    }
                } else {
                    addr = align;
                    sec = (*tcc_state).common_section;
                }
                if v != 0 {
                    if sym.is_null() {
                        sym = sym_push(
                            v,
                            type_0,
                            r | 0x200 as libc::c_int,
                            0 as libc::c_int,
                        );
                        patch_storage(sym, ad, 0 as *mut CType);
                    }
                    put_extern_sym(sym, sec, addr as Elf64_Addr, size as libc::c_ulong);
                } else {
                    vpush_ref(type_0, sec, addr as libc::c_ulong, size as libc::c_ulong);
                    sym = (*vtop).c2rust_unnamed_0.sym;
                    (*vtop).r = ((*vtop).r as libc::c_int | r) as libc::c_ushort;
                }
                if bcheck != 0 {
                    let mut bounds_ptr: *mut Elf64_Addr = 0 as *mut Elf64_Addr;
                    greloca(
                        (*tcc_state).bounds_section,
                        sym,
                        (*(*tcc_state).bounds_section).data_offset,
                        1 as libc::c_int,
                        0 as libc::c_int as Elf64_Addr,
                    );
                    bounds_ptr = section_ptr_add(
                        (*tcc_state).bounds_section,
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<Elf64_Addr>() as libc::c_ulong,
                            ),
                    ) as *mut Elf64_Addr;
                    *bounds_ptr
                        .offset(
                            0 as libc::c_int as isize,
                        ) = 0 as libc::c_int as Elf64_Addr;
                    *bounds_ptr.offset(1 as libc::c_int as isize) = size as Elf64_Addr;
                }
                current_block = 3024367268842933116;
            }
        }
    }
    match current_block {
        3024367268842933116 => {
            if (*type_0).t & 0x400 as libc::c_int != 0 {
                let mut a: libc::c_int = 0;
                if !(nocode_wanted > 0 as libc::c_int) {
                    if (*cur_scope).vla.num == 0 as libc::c_int {
                        if !((*cur_scope).prev).is_null()
                            && (*(*cur_scope).prev).vla.num != 0
                        {
                            (*cur_scope).vla.locorig = (*(*cur_scope).prev).vla.loc;
                        } else {
                            loc -= 8 as libc::c_int;
                            gen_vla_sp_save(loc);
                            (*cur_scope).vla.locorig = loc;
                        }
                    }
                    vpush_type_size(type_0, &mut a);
                    gen_vla_alloc(type_0, a);
                    gen_vla_sp_save(addr);
                    (*cur_scope).vla.loc = addr;
                    (*cur_scope).vla.num += 1;
                    (*cur_scope).vla.num;
                }
            } else if has_init != 0 {
                p.sec = sec;
                decl_initializer(
                    &mut p,
                    type_0,
                    addr as libc::c_ulong,
                    1 as libc::c_int,
                );
                if !flexible_array.is_null() {
                    (*(*flexible_array).type_0.ref_0)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .c = -(1 as libc::c_int);
                }
            }
        }
        _ => {}
    }
    if !init_str.is_null() {
        end_macro();
        next();
    }
    nocode_wanted = saved_nocode_wanted;
}
unsafe extern "C" fn func_vla_arg_code(mut arg: *mut Sym) {
    let mut align: libc::c_int = 0;
    let mut vla_array_tok: *mut TokenString = 0 as *mut TokenString;
    if !((*arg).type_0.ref_0).is_null() {
        func_vla_arg_code((*arg).type_0.ref_0);
    }
    if (*arg).type_0.t & 0x400 as libc::c_int != 0
        && !((*(*arg).type_0.ref_0).c2rust_unnamed_0.vla_array_str).is_null()
    {
        loc -= type_size(&mut int_type, &mut align);
        loc &= -align;
        (*(*arg).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c = loc;
        unget_tok(0 as libc::c_int);
        vla_array_tok = tok_str_alloc();
        (*vla_array_tok).str_0 = (*(*arg).type_0.ref_0).c2rust_unnamed_0.vla_array_str;
        begin_macro(vla_array_tok, 1 as libc::c_int);
        next();
        gexpr();
        end_macro();
        next();
        vpush_type_size(&mut (*(*arg).type_0.ref_0).type_0, &mut align);
        gen_op('*' as i32);
        vset(
            &mut int_type,
            0x32 as libc::c_int | 0x100 as libc::c_int,
            (*(*arg).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c,
        );
        vswap();
        vstore();
        vpop();
    }
}
unsafe extern "C" fn func_vla_arg(mut sym: *mut Sym) {
    let mut arg: *mut Sym = 0 as *mut Sym;
    arg = (*(*sym).type_0.ref_0).c2rust_unnamed_0.next;
    while !arg.is_null() {
        if (*arg).type_0.t & 0xf as libc::c_int == 5 as libc::c_int
            && (*(*arg).type_0.ref_0).type_0.t & 0x400 as libc::c_int != 0
        {
            func_vla_arg_code((*arg).type_0.ref_0);
        }
        arg = (*arg).c2rust_unnamed_0.next;
    }
}
unsafe extern "C" fn gen_function(mut sym: *mut Sym) {
    let mut f: scope = {
        let mut init = scope {
            prev: 0 as *mut scope,
            vla: C2RustUnnamed_11 {
                loc: 0,
                locorig: 0,
                num: 0,
            },
            cl: C2RustUnnamed_12 {
                s: 0 as *mut Sym,
                n: 0,
            },
            bsym: 0 as *mut libc::c_int,
            csym: 0 as *mut libc::c_int,
            lstk: 0 as *mut Sym,
            llstk: 0 as *mut Sym,
        };
        init
    };
    root_scope = &mut f;
    cur_scope = root_scope;
    nocode_wanted = 0 as libc::c_int;
    ind = (*(*tcc_state).cur_text_section).data_offset as libc::c_int;
    if ((*sym).a).aligned() != 0 {
        let mut newoff: size_t = section_add(
            (*tcc_state).cur_text_section,
            0 as libc::c_int as Elf64_Addr,
            (1 as libc::c_int) << ((*sym).a).aligned() as libc::c_int - 1 as libc::c_int,
        );
        gen_fill_nops(newoff.wrapping_sub(ind as size_t) as libc::c_int);
    }
    funcname = get_tok_str((*sym).v, 0 as *mut CValue);
    func_ind = ind;
    func_vt = (*(*sym).type_0.ref_0).type_0;
    func_var = (((*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
        .func_type() as libc::c_int == 3 as libc::c_int) as libc::c_int;
    put_extern_sym(
        sym,
        (*tcc_state).cur_text_section,
        ind as Elf64_Addr,
        0 as libc::c_int as libc::c_ulong,
    );
    if ((*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
        .func_ctor() != 0
    {
        add_array(
            tcc_state,
            b".init_array\0" as *const u8 as *const libc::c_char,
            (*sym).c2rust_unnamed.c2rust_unnamed.c,
        );
    }
    if ((*(*sym).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
        .func_dtor() != 0
    {
        add_array(
            tcc_state,
            b".fini_array\0" as *const u8 as *const libc::c_char,
            (*sym).c2rust_unnamed.c2rust_unnamed.c,
        );
    }
    tcc_debug_funcstart(tcc_state, sym);
    sym_push2(
        &mut local_stack,
        0x20000000 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    local_scope = 1 as libc::c_int;
    nb_temp_local_vars = 0 as libc::c_int;
    gfunc_prolog(sym);
    tcc_debug_prolog_epilog(tcc_state, 0 as libc::c_int);
    local_scope = 0 as libc::c_int;
    rsym = 0 as libc::c_int;
    func_vla_arg(sym);
    block(0 as libc::c_int);
    gsym(rsym);
    nocode_wanted = 0 as libc::c_int;
    pop_local_syms(0 as *mut Sym, 0 as libc::c_int);
    tcc_debug_prolog_epilog(tcc_state, 1 as libc::c_int);
    gfunc_epilog();
    tcc_debug_funcend(tcc_state, ind - func_ind);
    (*elfsym(sym)).st_size = (ind - func_ind) as Elf64_Xword;
    (*(*tcc_state).cur_text_section).data_offset = ind as libc::c_ulong;
    local_scope = 0 as libc::c_int;
    label_pop(&mut global_label_stack, 0 as *mut Sym, 0 as libc::c_int);
    sym_pop(&mut all_cleanups, 0 as *mut Sym, 0 as libc::c_int);
    (*tcc_state).cur_text_section = 0 as *mut Section;
    funcname = b"\0" as *const u8 as *const libc::c_char;
    func_vt.t = 0 as libc::c_int;
    func_var = 0 as libc::c_int;
    ind = 0 as libc::c_int;
    func_ind = -(1 as libc::c_int);
    nocode_wanted = 0x80000000 as libc::c_uint as libc::c_int;
    check_vstack();
    next();
}
unsafe extern "C" fn gen_inline_functions(mut s: *mut TCCState) {
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut inline_generated: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut fn_0: *mut InlineFunc = 0 as *mut InlineFunc;
    tcc_open_bf(s, b":inline:\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    loop {
        inline_generated = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*s).nb_inline_fns {
            fn_0 = *((*s).inline_fns).offset(i as isize);
            sym = (*fn_0).sym;
            if !sym.is_null()
                && ((*sym).c2rust_unnamed.c2rust_unnamed.c != 0
                    || (*sym).type_0.t & 0x8000 as libc::c_int == 0)
            {
                (*fn_0).sym = 0 as *mut Sym;
                tccpp_putfile(((*fn_0).filename).as_mut_ptr());
                begin_macro((*fn_0).func_str, 1 as libc::c_int);
                next();
                (*tcc_state).cur_text_section = (*tcc_state).text_section;
                gen_function(sym);
                end_macro();
                inline_generated = 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if !(inline_generated != 0) {
            break;
        }
    }
    tcc_close();
}
unsafe extern "C" fn free_inline_functions(mut s: *mut TCCState) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s).nb_inline_fns {
        let mut fn_0: *mut InlineFunc = *((*s).inline_fns).offset(i as isize);
        if !((*fn_0).sym).is_null() {
            tok_str_free((*fn_0).func_str);
        }
        i += 1;
        i;
    }
    dynarray_reset(
        &mut (*s).inline_fns as *mut *mut *mut InlineFunc as *mut libc::c_void,
        &mut (*s).nb_inline_fns,
    );
}
unsafe extern "C" fn do_Static_assert() {
    let mut c: libc::c_int = 0;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    next();
    skip('(' as i32);
    c = expr_const();
    msg = b"_Static_assert fail\0" as *const u8 as *const libc::c_char;
    if tok == ',' as i32 {
        next();
        msg = (*parse_mult_str(b"string constant\0" as *const u8 as *const libc::c_char))
            .data;
    }
    skip(')' as i32);
    if c == 0 as libc::c_int {
        _tcc_error(b"%s\0" as *const u8 as *const libc::c_char, msg);
    }
    skip(';' as i32);
}
unsafe extern "C" fn decl(mut l: libc::c_int) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut has_init: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut oldint: libc::c_int = 0;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut btype: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut ad: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut adbase: AttributeDef = AttributeDef {
        a: SymAttr {
            aligned_packed_weak_visibility_dllexport_nodecorate_dllimport_addrtaken_nodebug_xxxx: [0; 2],
        },
        f: FuncAttr {
            func_call_func_type_func_noreturn_func_ctor_func_dtor_func_args_func_alwinl_xxxx: [0; 4],
        },
        section: 0 as *mut Section,
        cleanup_func: 0 as *mut Sym,
        alias_target: 0,
        asm_label: 0,
        attr_mode: 0,
    };
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    loop {
        oldint = 0 as libc::c_int;
        if parse_btype(
            &mut btype,
            &mut adbase,
            (l == 0x32 as libc::c_int) as libc::c_int,
        ) == 0
        {
            if l == 0x34 as libc::c_int {
                return 0 as libc::c_int;
            }
            if tok == ';' as i32 && l != 0x33 as libc::c_int {
                next();
                continue;
            } else if tok == TOK_STATIC_ASSERT as libc::c_int {
                do_Static_assert();
                continue;
            } else {
                if l != 0x30 as libc::c_int {
                    break;
                }
                if tok == TOK_ASM1 as libc::c_int || tok == TOK_ASM2 as libc::c_int
                    || tok == TOK_ASM3 as libc::c_int
                {
                    asm_global_instr();
                    continue;
                } else if tok >= TOK_DEFINE as libc::c_int {
                    btype.t = 3 as libc::c_int;
                    oldint = 1 as libc::c_int;
                } else {
                    if tok != -(1 as libc::c_int) {
                        expect(b"declaration\0" as *const u8 as *const libc::c_char);
                    }
                    break;
                }
            }
        }
        if tok == ';' as i32 {
            if btype.t & 0xf as libc::c_int == 7 as libc::c_int {
                v = (*btype.ref_0).v;
                if v & 0x20000000 as libc::c_int == 0
                    && v & !(0x40000000 as libc::c_int) >= 0x10000000 as libc::c_int
                {
                    _tcc_warning(
                        b"unnamed struct/union that defines no instances\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                next();
                continue;
            } else if btype.t as libc::c_uint
                & (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint)
                == ((2 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
            {
                next();
                continue;
            }
        }
        loop {
            type_0 = btype;
            ad = adbase;
            type_decl(&mut type_0, &mut ad, &mut v, 2 as libc::c_int);
            if type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
                if type_0.t & 0x2000 as libc::c_int != 0 && l != 0x30 as libc::c_int {
                    _tcc_error(
                        b"function without file scope cannot be static\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                sym = type_0.ref_0;
                if ((*sym).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_type()
                    as libc::c_int == 2 as libc::c_int && l == 0x30 as libc::c_int
                {
                    func_vt = type_0;
                    decl(0x33 as libc::c_int);
                }
                if type_0.t & (0x1000 as libc::c_int | 0x8000 as libc::c_int)
                    == 0x1000 as libc::c_int | 0x8000 as libc::c_int
                {
                    if (*tcc_state).gnu89_inline as libc::c_int != 0
                        || ((*sym).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
                            .func_alwinl() as libc::c_int != 0
                    {
                        type_0
                            .t = type_0.t & !(0x1000 as libc::c_int)
                            | 0x2000 as libc::c_int;
                    } else {
                        type_0.t &= !(0x8000 as libc::c_int);
                    }
                }
            } else if oldint != 0 {
                _tcc_warning(
                    b"type defaults to int\0" as *const u8 as *const libc::c_char,
                );
            }
            if (*tcc_state).gnu_ext as libc::c_int != 0
                && (tok == TOK_ASM1 as libc::c_int || tok == TOK_ASM2 as libc::c_int
                    || tok == TOK_ASM3 as libc::c_int)
            {
                ad.asm_label = asm_label_instr();
                parse_attribute(&mut ad);
            }
            if tok == '{' as i32 {
                if l != 0x30 as libc::c_int {
                    _tcc_error(
                        b"cannot use local functions\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if type_0.t & 0xf as libc::c_int != 6 as libc::c_int {
                    expect(b"function definition\0" as *const u8 as *const libc::c_char);
                }
                sym = type_0.ref_0;
                loop {
                    sym = (*sym).c2rust_unnamed_0.next;
                    if sym.is_null() {
                        break;
                    }
                    if (*sym).v & !(0x20000000 as libc::c_int) == 0 {
                        expect(b"identifier\0" as *const u8 as *const libc::c_char);
                    }
                    if (*sym).type_0.t == 0 as libc::c_int {
                        (*sym).type_0 = int_type;
                    }
                }
                merge_funcattr(
                    &mut (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f,
                    &mut ad.f,
                );
                type_0.t &= !(0x1000 as libc::c_int);
                sym = external_sym(v, &mut type_0, 0 as libc::c_int, &mut ad);
                if (*sym).type_0.t & 0x8000 as libc::c_int != 0 {
                    let mut fn_0: *mut InlineFunc = 0 as *mut InlineFunc;
                    fn_0 = tcc_malloc(
                        (::core::mem::size_of::<InlineFunc>() as libc::c_ulong)
                            .wrapping_add(strlen(((*file).filename).as_mut_ptr())),
                    ) as *mut InlineFunc;
                    strcpy(
                        ((*fn_0).filename).as_mut_ptr(),
                        ((*file).filename).as_mut_ptr(),
                    );
                    (*fn_0).sym = sym;
                    dynarray_add(
                        &mut (*tcc_state).inline_fns as *mut *mut *mut InlineFunc
                            as *mut libc::c_void,
                        &mut (*tcc_state).nb_inline_fns,
                        fn_0 as *mut libc::c_void,
                    );
                    skip_or_save_block(&mut (*fn_0).func_str);
                } else {
                    (*tcc_state).cur_text_section = ad.section;
                    if ((*tcc_state).cur_text_section).is_null() {
                        (*tcc_state).cur_text_section = (*tcc_state).text_section;
                    } else if (*(*tcc_state).cur_text_section).sh_num
                        > (*(*tcc_state).bss_section).sh_num
                    {
                        (*(*tcc_state).cur_text_section)
                            .sh_flags = (*(*tcc_state).text_section).sh_flags;
                    }
                    gen_function(sym);
                }
                break;
            } else {
                if l == 0x33 as libc::c_int {
                    let mut current_block_75: u64;
                    sym = (*func_vt.ref_0).c2rust_unnamed_0.next;
                    loop {
                        if sym.is_null() {
                            current_block_75 = 13853033528615664019;
                            break;
                        }
                        if (*sym).v & !(0x20000000 as libc::c_int) == v {
                            current_block_75 = 9653725922949806897;
                            break;
                        }
                        sym = (*sym).c2rust_unnamed_0.next;
                    }
                    match current_block_75 {
                        13853033528615664019 => {
                            _tcc_error(
                                b"declaration for parameter '%s' but no such parameter\0"
                                    as *const u8 as *const libc::c_char,
                                get_tok_str(v, 0 as *mut CValue),
                            );
                        }
                        _ => {
                            if type_0.t
                                & (0x1000 as libc::c_int | 0x2000 as libc::c_int
                                    | 0x4000 as libc::c_int | 0x8000 as libc::c_int) != 0
                            {
                                _tcc_error(
                                    b"storage class specified for '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            if (*sym).type_0.t != 0 as libc::c_int {
                                _tcc_error(
                                    b"redefinition of parameter '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    get_tok_str(v, 0 as *mut CValue),
                                );
                            }
                            convert_parameter_type(&mut type_0);
                            (*sym).type_0 = type_0;
                        }
                    }
                } else if type_0.t & 0x4000 as libc::c_int != 0 {
                    sym = sym_find(v);
                    if !sym.is_null()
                        && (*sym).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.sym_scope
                            == local_scope
                    {
                        if is_compatible_types(&mut (*sym).type_0, &mut type_0) == 0
                            || (*sym).type_0.t & 0x4000 as libc::c_int == 0
                        {
                            _tcc_error(
                                b"incompatible redefinition of '%s'\0" as *const u8
                                    as *const libc::c_char,
                                get_tok_str(v, 0 as *mut CValue),
                            );
                        }
                        (*sym).type_0 = type_0;
                    } else {
                        sym = sym_push(
                            v,
                            &mut type_0,
                            0 as libc::c_int,
                            0 as libc::c_int,
                        );
                    }
                    (*sym).a = ad.a;
                    if type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
                        merge_funcattr(
                            &mut (*(*sym).type_0.ref_0)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .f,
                            &mut ad.f,
                        );
                    }
                    if debug_modes != 0 {
                        tcc_debug_typedef(tcc_state, sym);
                    }
                } else if type_0.t & 0xf as libc::c_int == 0 as libc::c_int
                    && type_0.t & 0x1000 as libc::c_int == 0
                {
                    _tcc_error(
                        b"declaration of void object\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    r = 0 as libc::c_int;
                    if type_0.t & 0xf as libc::c_int == 6 as libc::c_int {
                        merge_funcattr(
                            &mut (*type_0.ref_0)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .f,
                            &mut ad.f,
                        );
                    } else if type_0.t & 0x40 as libc::c_int == 0 {
                        r |= 0x100 as libc::c_int;
                    }
                    has_init = (tok == '=' as i32) as libc::c_int;
                    if has_init != 0 && type_0.t & 0x400 as libc::c_int != 0 {
                        _tcc_error(
                            b"variable length array cannot be initialized\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if type_0.t & 0x1000 as libc::c_int != 0
                        && (has_init == 0 || l != 0x30 as libc::c_int)
                        || type_0.t & 0xf as libc::c_int == 6 as libc::c_int
                        || type_0.t & 0x40 as libc::c_int != 0 && has_init == 0
                            && l == 0x30 as libc::c_int
                            && (*type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c
                                < 0 as libc::c_int
                    {
                        type_0.t |= 0x1000 as libc::c_int;
                        external_sym(v, &mut type_0, r, &mut ad);
                    } else {
                        if l == 0x30 as libc::c_int
                            || type_0.t & 0x2000 as libc::c_int != 0
                        {
                            r |= 0x30 as libc::c_int;
                        } else {
                            r |= 0x32 as libc::c_int;
                        }
                        if has_init != 0 {
                            next();
                        } else if l == 0x30 as libc::c_int {
                            type_0.t |= 0x1000 as libc::c_int;
                        }
                        decl_initializer_alloc(
                            &mut type_0,
                            &mut ad,
                            r,
                            has_init,
                            v,
                            (l == 0x30 as libc::c_int) as libc::c_int,
                        );
                    }
                    if ad.alias_target != 0 && l == 0x30 as libc::c_int {
                        esym = elfsym(sym_find(ad.alias_target));
                        if esym.is_null() {
                            _tcc_error(
                                b"unsupported forward __alias__ attribute\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        put_extern_sym2(
                            sym_find(v),
                            (*esym).st_shndx as libc::c_int,
                            (*esym).st_value,
                            (*esym).st_size,
                            1 as libc::c_int,
                        );
                    }
                }
                if tok != ',' as i32 {
                    if l == 0x34 as libc::c_int {
                        return 1 as libc::c_int;
                    }
                    skip(';' as i32);
                    break;
                } else {
                    next();
                }
            }
        }
    }
    return 0 as libc::c_int;
}
