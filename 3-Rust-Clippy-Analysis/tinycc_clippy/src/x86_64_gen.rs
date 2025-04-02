use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    pub type _tccdbg;
    static mut tcc_state: *mut TCCState;
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn _tcc_error(fmt: *const libc::c_char, _: ...) -> !;
    static mut char_pointer_type: CType;
    static mut vtop: *mut SValue;
    static mut ind: libc::c_int;
    static mut loc: libc::c_int;
    static mut nocode_wanted: libc::c_int;
    static mut func_vt: CType;
    static mut func_var: libc::c_int;
    static mut func_vc: libc::c_int;
    fn is_float(t: libc::c_int) -> libc::c_int;
    fn greloca(
        s: *mut Section,
        sym: *mut Sym,
        offset: libc::c_ulong,
        type_0: libc::c_int,
        addend: Elf64_Addr,
    );
    fn sym_push(
        v: libc::c_int,
        type_0: *mut CType,
        r: libc::c_int,
        c: libc::c_int,
    ) -> *mut Sym;
    fn external_helper_sym(v: libc::c_int) -> *mut Sym;
    fn vpush_helper_func(v: libc::c_int);
    fn vset(type_0: *mut CType, r: libc::c_int, v: libc::c_int);
    fn vset_VT_CMP(op: libc::c_int);
    fn vpushi(v: libc::c_int);
    fn vswap();
    fn vrotb(n: libc::c_int);
    fn vpop();
    fn save_reg(r: libc::c_int);
    fn get_reg(rc: libc::c_int) -> libc::c_int;
    fn save_regs(n: libc::c_int);
    fn gv(rc: libc::c_int) -> libc::c_int;
    fn gv2(rc1: libc::c_int, rc2: libc::c_int);
    fn type_size(type_0: *mut CType, a: *mut libc::c_int) -> libc::c_int;
    fn vstore();
    fn get_sym_ref(
        type_0: *mut CType,
        sec: *mut Section,
        offset: libc::c_ulong,
        size: libc::c_ulong,
    ) -> *mut Sym;
    fn gbound_args(nb_args: libc::c_int);
    fn section_realloc(sec: *mut Section, new_size: libc::c_ulong);
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut libc::c_void;
    fn gsym(t: libc::c_int);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub const __va_float_reg: __va_arg_type = 1;
pub const x86_64_mode_sse: X86_64_Mode = 3;
pub const __va_gen_reg: __va_arg_type = 0;
pub const x86_64_mode_integer: X86_64_Mode = 2;
pub const __va_stack: __va_arg_type = 2;
pub type X86_64_Mode = libc::c_uint;
pub const x86_64_mode_x87: X86_64_Mode = 4;
pub const x86_64_mode_memory: X86_64_Mode = 1;
pub const x86_64_mode_none: X86_64_Mode = 0;
pub type __va_arg_type = libc::c_uint;
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
#[no_mangle]
pub static mut target_machine_defs: *const libc::c_char = b"__x86_64__\0__amd64__\0\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut reg_classes: [libc::c_int; 25] = [
    0x1 as libc::c_int | 0x4 as libc::c_int,
    0x1 as libc::c_int | 0x10 as libc::c_int,
    0x1 as libc::c_int | 0x8 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x20 as libc::c_int,
    0x40 as libc::c_int,
    0x100 as libc::c_int,
    0x200 as libc::c_int,
    0x400 as libc::c_int,
    0x800 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x2 as libc::c_int | 0x1000 as libc::c_int,
    0x2 as libc::c_int | 0x2000 as libc::c_int,
    0x2 as libc::c_int | 0x4000 as libc::c_int,
    0x2 as libc::c_int | 0x8000 as libc::c_int,
    0x2 as libc::c_int | 0x10000 as libc::c_int,
    0x2 as libc::c_int | 0x20000 as libc::c_int,
    0x40000 as libc::c_int,
    0x80000 as libc::c_int,
    0x80 as libc::c_int,
];
static mut func_sub_sp_offset: libc::c_ulong = 0;
static mut func_ret_sub: libc::c_int = 0;
static mut func_bound_offset: Elf64_Addr = 0;
static mut func_bound_ind: libc::c_ulong = 0;
#[no_mangle]
pub static mut func_bound_add_epilog: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn g(mut c: libc::c_int) {
    let mut ind1: libc::c_int = 0;
    if nocode_wanted != 0 {
        return;
    }
    ind1 = ind + 1 as libc::c_int;
    if ind1 as libc::c_ulong > (*(*tcc_state).cur_text_section).data_allocated {
        section_realloc((*tcc_state).cur_text_section, ind1 as libc::c_ulong);
    }
    *((*(*tcc_state).cur_text_section).data).offset(ind as isize) = c as libc::c_uchar;
    ind = ind1;
}
#[no_mangle]
pub unsafe extern "C" fn o(mut c: libc::c_uint) {
    while c != 0 {
        g(c as libc::c_int);
        c = c >> 8 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gen_le16(mut v: libc::c_int) {
    g(v);
    g(v >> 8 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gen_le32(mut c: libc::c_int) {
    g(c);
    g(c >> 8 as libc::c_int);
    g(c >> 16 as libc::c_int);
    g(c >> 24 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gen_le64(mut c: int64_t) {
    g(c as libc::c_int);
    g((c >> 8 as libc::c_int) as libc::c_int);
    g((c >> 16 as libc::c_int) as libc::c_int);
    g((c >> 24 as libc::c_int) as libc::c_int);
    g((c >> 32 as libc::c_int) as libc::c_int);
    g((c >> 40 as libc::c_int) as libc::c_int);
    g((c >> 48 as libc::c_int) as libc::c_int);
    g((c >> 56 as libc::c_int) as libc::c_int);
}
unsafe extern "C" fn orex(
    mut ll: libc::c_int,
    mut r: libc::c_int,
    mut r2: libc::c_int,
    mut b: libc::c_int,
) {
    if r & 0x3f as libc::c_int >= 0x30 as libc::c_int {
        r = 0 as libc::c_int;
    }
    if r2 & 0x3f as libc::c_int >= 0x30 as libc::c_int {
        r2 = 0 as libc::c_int;
    }
    if ll != 0 || r >> 3 as libc::c_int & 1 as libc::c_int != 0
        || r2 >> 3 as libc::c_int & 1 as libc::c_int != 0
    {
        o(
            (0x40 as libc::c_int | r >> 3 as libc::c_int & 1 as libc::c_int
                | (r2 >> 3 as libc::c_int & 1 as libc::c_int) << 2 as libc::c_int
                | ll << 3 as libc::c_int) as libc::c_uint,
        );
    }
    o(b as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn gsym_addr(mut t: libc::c_int, mut a: libc::c_int) {
    while t != 0 {
        let mut ptr: *mut libc::c_uchar = ((*(*tcc_state).cur_text_section).data)
            .offset(t as isize);
        let mut n: uint32_t = read32le(ptr);
        write32le(
            ptr,
            (if a < 0 as libc::c_int { -a } else { a - t - 4 as libc::c_int })
                as uint32_t,
        );
        t = n as libc::c_int;
    }
}
unsafe extern "C" fn is64_type(mut t: libc::c_int) -> libc::c_int {
    return (t & 0xf as libc::c_int == 5 as libc::c_int
        || t & 0xf as libc::c_int == 6 as libc::c_int
        || t & 0xf as libc::c_int == 4 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn oad(mut c: libc::c_int, mut s: libc::c_int) -> libc::c_int {
    let mut t: libc::c_int = 0;
    if nocode_wanted != 0 {
        return s;
    }
    o(c as libc::c_uint);
    t = ind;
    gen_le32(s);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn gen_addr32(
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: libc::c_int,
) {
    if r & 0x200 as libc::c_int != 0 {
        greloca(
            (*tcc_state).cur_text_section,
            sym,
            ind as libc::c_ulong,
            11 as libc::c_int,
            c as Elf64_Addr,
        );
        c = 0 as libc::c_int;
    }
    gen_le32(c);
}
#[no_mangle]
pub unsafe extern "C" fn gen_addr64(
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: int64_t,
) {
    if r & 0x200 as libc::c_int != 0 {
        greloca(
            (*tcc_state).cur_text_section,
            sym,
            ind as libc::c_ulong,
            1 as libc::c_int,
            c as Elf64_Addr,
        );
        c = 0 as libc::c_int as int64_t;
    }
    gen_le64(c);
}
#[no_mangle]
pub unsafe extern "C" fn gen_addrpc32(
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: libc::c_int,
) {
    if r & 0x200 as libc::c_int != 0 {
        greloca(
            (*tcc_state).cur_text_section,
            sym,
            ind as libc::c_ulong,
            2 as libc::c_int,
            (c - 4 as libc::c_int) as Elf64_Addr,
        );
        c = 4 as libc::c_int;
    }
    gen_le32(c - 4 as libc::c_int);
}
unsafe extern "C" fn gen_gotpcrel(
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: libc::c_int,
) {
    greloca(
        (*tcc_state).cur_text_section,
        sym,
        ind as libc::c_ulong,
        9 as libc::c_int,
        -(4 as libc::c_int) as Elf64_Addr,
    );
    gen_le32(0 as libc::c_int);
    if c != 0 {
        orex(1 as libc::c_int, r, 0 as libc::c_int, 0x81 as libc::c_int);
        o((0xc0 as libc::c_int + (r & 7 as libc::c_int)) as libc::c_uint);
        gen_le32(c);
    }
}
unsafe extern "C" fn gen_modrm_impl(
    mut op_reg: libc::c_int,
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: libc::c_int,
    mut is_got: libc::c_int,
) {
    op_reg = (op_reg & 7 as libc::c_int) << 3 as libc::c_int;
    if r & 0x3f as libc::c_int == 0x30 as libc::c_int {
        if r & 0x200 as libc::c_int == 0 {
            o((0x4 as libc::c_int | op_reg) as libc::c_uint);
            oad(0x25 as libc::c_int, c);
        } else {
            o((0x5 as libc::c_int | op_reg) as libc::c_uint);
            if is_got != 0 {
                gen_gotpcrel(r, sym, c);
            } else {
                gen_addrpc32(r, sym, c);
            }
        }
    } else if r & 0x3f as libc::c_int == 0x32 as libc::c_int {
        if c == c as libc::c_char as libc::c_int {
            o((0x45 as libc::c_int | op_reg) as libc::c_uint);
            g(c);
        } else {
            oad(0x85 as libc::c_int | op_reg, c);
        }
    } else if r & 0x3f as libc::c_int >= TREG_MEM as libc::c_int {
        if c != 0 {
            g(0x80 as libc::c_int | op_reg | r & 7 as libc::c_int);
            gen_le32(c);
        } else {
            g(0 as libc::c_int | op_reg | r & 7 as libc::c_int);
        }
    } else {
        g(0 as libc::c_int | op_reg | r & 7 as libc::c_int);
    };
}
unsafe extern "C" fn gen_modrm(
    mut op_reg: libc::c_int,
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: libc::c_int,
) {
    gen_modrm_impl(op_reg, r, sym, c, 0 as libc::c_int);
}
unsafe extern "C" fn gen_modrm64(
    mut opcode: libc::c_int,
    mut op_reg: libc::c_int,
    mut r: libc::c_int,
    mut sym: *mut Sym,
    mut c: libc::c_int,
) {
    let mut is_got: libc::c_int = 0;
    is_got = (op_reg & TREG_MEM as libc::c_int != 0
        && (*sym).type_0.t & 0x2000 as libc::c_int == 0) as libc::c_int;
    orex(1 as libc::c_int, r, op_reg, opcode);
    gen_modrm_impl(op_reg, r, sym, c, is_got);
}
#[no_mangle]
pub unsafe extern "C" fn load(mut r: libc::c_int, mut sv: *mut SValue) {
    let mut v: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut fc: libc::c_int = 0;
    let mut fr: libc::c_int = 0;
    let mut v1: SValue = SValue {
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
    fr = (*sv).r as libc::c_int;
    ft = (*sv).type_0.t & !(0x20 as libc::c_int);
    fc = (*sv).c2rust_unnamed.c.i as libc::c_int;
    if fc as uint64_t != (*sv).c2rust_unnamed.c.i && fr & 0x200 as libc::c_int != 0 {
        _tcc_error(b"64 bit addend in load\0" as *const u8 as *const libc::c_char);
    }
    ft &= !(0x200 as libc::c_int | 0x100 as libc::c_int);
    if fr & 0x3f as libc::c_int == 0x30 as libc::c_int && fr & 0x200 as libc::c_int != 0
        && fr & 0x100 as libc::c_int != 0
        && (*(*sv).c2rust_unnamed_0.sym).type_0.t & 0x2000 as libc::c_int == 0
    {
        let mut tr: libc::c_int = r | TREG_MEM as libc::c_int;
        if is_float(ft) != 0 {
            tr = get_reg(0x1 as libc::c_int) | TREG_MEM as libc::c_int;
        }
        gen_modrm64(
            0x8b as libc::c_int,
            tr,
            fr,
            (*sv).c2rust_unnamed_0.sym,
            0 as libc::c_int,
        );
        fr = tr | 0x100 as libc::c_int;
    }
    v = fr & 0x3f as libc::c_int;
    if fr & 0x100 as libc::c_int != 0 {
        let mut b: libc::c_int = 0;
        let mut ll: libc::c_int = 0;
        if v == 0x31 as libc::c_int {
            v1.type_0.t = 5 as libc::c_int;
            v1.r = (0x32 as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
            v1.c2rust_unnamed.c.i = fc as uint64_t;
            fr = r;
            if reg_classes[fr as usize] & (0x1 as libc::c_int | 0x800 as libc::c_int)
                == 0
            {
                fr = get_reg(0x1 as libc::c_int);
            }
            load(fr, &mut v1);
        }
        if fc as uint64_t != (*sv).c2rust_unnamed.c.i {
            v1.type_0.t = 4 as libc::c_int;
            v1.r = 0x30 as libc::c_int as libc::c_ushort;
            v1.c2rust_unnamed.c.i = (*sv).c2rust_unnamed.c.i;
            fr = r;
            if reg_classes[fr as usize] & (0x1 as libc::c_int | 0x800 as libc::c_int)
                == 0
            {
                fr = get_reg(0x1 as libc::c_int);
            }
            load(fr, &mut v1);
            fc = 0 as libc::c_int;
        }
        ll = 0 as libc::c_int;
        if ft & 0xf as libc::c_int == 7 as libc::c_int {
            let mut align: libc::c_int = 0;
            match type_size(&mut (*sv).type_0, &mut align) {
                1 => {
                    ft = 1 as libc::c_int;
                }
                2 => {
                    ft = 2 as libc::c_int;
                }
                4 => {
                    ft = 3 as libc::c_int;
                }
                8 => {
                    ft = 4 as libc::c_int;
                }
                _ => {
                    _tcc_error(
                        b"invalid aggregate type for register load\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        }
        if ft & 0xf as libc::c_int == 8 as libc::c_int {
            b = 0x6e0f66 as libc::c_int;
            r = r & 7 as libc::c_int;
        } else if ft & 0xf as libc::c_int == 9 as libc::c_int {
            b = 0x7e0ff3 as libc::c_int;
            r = r & 7 as libc::c_int;
        } else if ft & 0xf as libc::c_int == 10 as libc::c_int {
            b = 0xdb as libc::c_int;
            r = 5 as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == 1 as libc::c_int as libc::c_uint
            || ft as libc::c_uint
                & !((0x1000 as libc::c_int | 0x2000 as libc::c_int
                    | 0x4000 as libc::c_int | 0x8000 as libc::c_int) as libc::c_uint
                    | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        << 20 as libc::c_int | 0x80 as libc::c_int as libc::c_uint))
                == 11 as libc::c_int as libc::c_uint
        {
            b = 0xbe0f as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == (1 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        {
            b = 0xb60f as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == 2 as libc::c_int as libc::c_uint
        {
            b = 0xbf0f as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == (2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        {
            b = 0xb70f as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == 0 as libc::c_int as libc::c_uint
        {
            return
        } else {
            if ft & 0xf as libc::c_int == 3 as libc::c_int
                || ft & 0xf as libc::c_int == 4 as libc::c_int
                || ft & 0xf as libc::c_int == 5 as libc::c_int
                || ft & 0xf as libc::c_int == 6 as libc::c_int
            {} else {
                __assert_fail(
                    b"((ft & VT_BTYPE) == VT_INT) || ((ft & VT_BTYPE) == VT_LLONG) || ((ft & VT_BTYPE) == VT_PTR) || ((ft & VT_BTYPE) == VT_FUNC)\0"
                        as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    458 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void load(int, SValue *)\0"))
                        .as_ptr(),
                );
            }
            'c_13564: {
                if ft & 0xf as libc::c_int == 3 as libc::c_int
                    || ft & 0xf as libc::c_int == 4 as libc::c_int
                    || ft & 0xf as libc::c_int == 5 as libc::c_int
                    || ft & 0xf as libc::c_int == 6 as libc::c_int
                {} else {
                    __assert_fail(
                        b"((ft & VT_BTYPE) == VT_INT) || ((ft & VT_BTYPE) == VT_LLONG) || ((ft & VT_BTYPE) == VT_PTR) || ((ft & VT_BTYPE) == VT_FUNC)\0"
                            as *const u8 as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                            as *const u8 as *const libc::c_char,
                        458 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 25],
                            &[libc::c_char; 25],
                        >(b"void load(int, SValue *)\0"))
                            .as_ptr(),
                    );
                }
            };
            ll = is64_type(ft);
            b = 0x8b as libc::c_int;
        }
        if ll != 0 {
            gen_modrm64(b, r, fr, (*sv).c2rust_unnamed_0.sym, fc);
        } else {
            orex(ll, fr, r, b);
            gen_modrm(r, fr, (*sv).c2rust_unnamed_0.sym, fc);
        }
    } else if v == 0x30 as libc::c_int {
        if fr & 0x200 as libc::c_int != 0 {
            if (*(*sv).c2rust_unnamed_0.sym).type_0.t & 0x2000 as libc::c_int != 0 {
                orex(1 as libc::c_int, 0 as libc::c_int, r, 0x8d as libc::c_int);
                o(
                    (0x5 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                        as libc::c_uint,
                );
                gen_addrpc32(fr, (*sv).c2rust_unnamed_0.sym, fc);
            } else {
                orex(1 as libc::c_int, 0 as libc::c_int, r, 0x8b as libc::c_int);
                o(
                    (0x5 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                        as libc::c_uint,
                );
                gen_gotpcrel(r, (*sv).c2rust_unnamed_0.sym, fc);
            }
        } else if is64_type(ft) != 0 {
            if (*sv).c2rust_unnamed.c.i >> 32 as libc::c_int != 0 {
                orex(
                    1 as libc::c_int,
                    r,
                    0 as libc::c_int,
                    0xb8 as libc::c_int + (r & 7 as libc::c_int),
                );
                gen_le64((*sv).c2rust_unnamed.c.i as int64_t);
            } else if (*sv).c2rust_unnamed.c.i > 0 as libc::c_int as uint64_t {
                orex(
                    0 as libc::c_int,
                    r,
                    0 as libc::c_int,
                    0xb8 as libc::c_int + (r & 7 as libc::c_int),
                );
                gen_le32((*sv).c2rust_unnamed.c.i as libc::c_int);
            } else {
                o(
                    (0xc031 as libc::c_int
                        + (r & 7 as libc::c_int) * 0x900 as libc::c_int) as libc::c_uint,
                );
            }
        } else {
            orex(
                0 as libc::c_int,
                r,
                0 as libc::c_int,
                0xb8 as libc::c_int + (r & 7 as libc::c_int),
            );
            gen_le32(fc);
        }
    } else if v == 0x32 as libc::c_int {
        orex(1 as libc::c_int, 0 as libc::c_int, r, 0x8d as libc::c_int);
        gen_modrm(r, 0x32 as libc::c_int, (*sv).c2rust_unnamed_0.sym, fc);
    } else if v == 0x33 as libc::c_int {
        if fc & 0x100 as libc::c_int != 0 {
            v = (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_r as libc::c_int;
            fc &= !(0x100 as libc::c_int);
            orex(
                0 as libc::c_int,
                r,
                0 as libc::c_int,
                0xb0 as libc::c_int + (r & 7 as libc::c_int),
            );
            g(v ^ fc ^ (v == 0x95 as libc::c_int) as libc::c_int);
            o(
                (0x37a as libc::c_int
                    + ((r >> 3 as libc::c_int & 1 as libc::c_int) << 8 as libc::c_int))
                    as libc::c_uint,
            );
        }
        orex(0 as libc::c_int, r, 0 as libc::c_int, 0xf as libc::c_int);
        o(fc as libc::c_uint);
        o((0xc0 as libc::c_int + (r & 7 as libc::c_int)) as libc::c_uint);
        orex(0 as libc::c_int, r, 0 as libc::c_int, 0xf as libc::c_int);
        o(
            (0xc0b6 as libc::c_int + (r & 7 as libc::c_int) * 0x900 as libc::c_int)
                as libc::c_uint,
        );
    } else if v == 0x34 as libc::c_int || v == 0x35 as libc::c_int {
        t = v & 1 as libc::c_int;
        orex(0 as libc::c_int, r, 0 as libc::c_int, 0 as libc::c_int);
        oad(0xb8 as libc::c_int + (r & 7 as libc::c_int), t);
        o(
            (0x5eb as libc::c_int
                + ((r >> 3 as libc::c_int & 1 as libc::c_int) << 8 as libc::c_int))
                as libc::c_uint,
        );
        gsym(fc);
        orex(0 as libc::c_int, r, 0 as libc::c_int, 0 as libc::c_int);
        oad(0xb8 as libc::c_int + (r & 7 as libc::c_int), t ^ 1 as libc::c_int);
    } else if v != r {
        if r >= TREG_XMM0 as libc::c_int && r <= TREG_XMM7 as libc::c_int {
            if v == TREG_ST0 as libc::c_int {
                o(0xf0245cdd as libc::c_uint);
                o(0x100ff2 as libc::c_int as libc::c_uint);
                o(
                    (0x44 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                        as libc::c_uint,
                );
                o(0xf024 as libc::c_int as libc::c_uint);
            } else {
                if v >= TREG_XMM0 as libc::c_int && v <= TREG_XMM7 as libc::c_int
                {} else {
                    __assert_fail(
                        b"(v >= TREG_XMM0) && (v <= TREG_XMM7)\0" as *const u8
                            as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                            as *const u8 as *const libc::c_char,
                        538 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 25],
                            &[libc::c_char; 25],
                        >(b"void load(int, SValue *)\0"))
                            .as_ptr(),
                    );
                }
                'c_12383: {
                    if v >= TREG_XMM0 as libc::c_int && v <= TREG_XMM7 as libc::c_int
                    {} else {
                        __assert_fail(
                            b"(v >= TREG_XMM0) && (v <= TREG_XMM7)\0" as *const u8
                                as *const libc::c_char,
                            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                as *const u8 as *const libc::c_char,
                            538 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 25],
                                &[libc::c_char; 25],
                            >(b"void load(int, SValue *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if ft & 0xf as libc::c_int == 8 as libc::c_int {
                    o(0x100ff3 as libc::c_int as libc::c_uint);
                } else {
                    if ft & 0xf as libc::c_int == 9 as libc::c_int {} else {
                        __assert_fail(
                            b"(ft & VT_BTYPE) == VT_DOUBLE\0" as *const u8
                                as *const libc::c_char,
                            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                as *const u8 as *const libc::c_char,
                            542 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 25],
                                &[libc::c_char; 25],
                            >(b"void load(int, SValue *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_12328: {
                        if ft & 0xf as libc::c_int == 9 as libc::c_int {} else {
                            __assert_fail(
                                b"(ft & VT_BTYPE) == VT_DOUBLE\0" as *const u8
                                    as *const libc::c_char,
                                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                    as *const u8 as *const libc::c_char,
                                542 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 25],
                                    &[libc::c_char; 25],
                                >(b"void load(int, SValue *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    o(0x100ff2 as libc::c_int as libc::c_uint);
                }
                o(
                    (0xc0 as libc::c_int + (v & 7 as libc::c_int)
                        + (r & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
                );
            }
        } else if r == TREG_ST0 as libc::c_int {
            if v >= TREG_XMM0 as libc::c_int && v <= TREG_XMM7 as libc::c_int {} else {
                __assert_fail(
                    b"(v >= TREG_XMM0) && (v <= TREG_XMM7)\0" as *const u8
                        as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    548 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void load(int, SValue *)\0"))
                        .as_ptr(),
                );
            }
            'c_12235: {
                if v >= TREG_XMM0 as libc::c_int && v <= TREG_XMM7 as libc::c_int
                {} else {
                    __assert_fail(
                        b"(v >= TREG_XMM0) && (v <= TREG_XMM7)\0" as *const u8
                            as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                            as *const u8 as *const libc::c_char,
                        548 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 25],
                            &[libc::c_char; 25],
                        >(b"void load(int, SValue *)\0"))
                            .as_ptr(),
                    );
                }
            };
            o(0x110ff2 as libc::c_int as libc::c_uint);
            o(
                (0x44 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                    as libc::c_uint,
            );
            o(0xf024 as libc::c_int as libc::c_uint);
            o(0xf02444dd as libc::c_uint);
        } else {
            orex(is64_type(ft), r, v, 0x89 as libc::c_int);
            o(
                (0xc0 as libc::c_int + (r & 7 as libc::c_int)
                    + (v & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn store(mut r: libc::c_int, mut v: *mut SValue) {
    let mut fr: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut fc: libc::c_int = 0;
    let mut op64: libc::c_int = 0 as libc::c_int;
    let mut pic: libc::c_int = 0 as libc::c_int;
    fr = (*v).r as libc::c_int & 0x3f as libc::c_int;
    ft = (*v).type_0.t;
    fc = (*v).c2rust_unnamed.c.i as libc::c_int;
    if fc as uint64_t != (*v).c2rust_unnamed.c.i && fr & 0x200 as libc::c_int != 0 {
        _tcc_error(b"64 bit addend in store\0" as *const u8 as *const libc::c_char);
    }
    ft &= !(0x200 as libc::c_int | 0x100 as libc::c_int);
    bt = ft & 0xf as libc::c_int;
    if fr == 0x30 as libc::c_int && (*v).r as libc::c_int & 0x200 as libc::c_int != 0
        && (*(*v).c2rust_unnamed_0.sym).type_0.t & 0x2000 as libc::c_int == 0
    {
        o(0x1d8b4c as libc::c_int as libc::c_uint);
        gen_gotpcrel(
            TREG_R11 as libc::c_int,
            (*v).c2rust_unnamed_0.sym,
            (*v).c2rust_unnamed.c.i as libc::c_int,
        );
        pic = if is64_type(bt) != 0 { 0x49 as libc::c_int } else { 0x41 as libc::c_int };
    }
    if bt == 8 as libc::c_int {
        o(0x66 as libc::c_int as libc::c_uint);
        o(pic as libc::c_uint);
        o(0x7e0f as libc::c_int as libc::c_uint);
        r = r & 7 as libc::c_int;
    } else if bt == 9 as libc::c_int {
        o(0x66 as libc::c_int as libc::c_uint);
        o(pic as libc::c_uint);
        o(0xd60f as libc::c_int as libc::c_uint);
        r = r & 7 as libc::c_int;
    } else if bt == 10 as libc::c_int {
        o(0xc0d9 as libc::c_int as libc::c_uint);
        o(pic as libc::c_uint);
        o(0xdb as libc::c_int as libc::c_uint);
        r = 7 as libc::c_int;
    } else {
        if bt == 2 as libc::c_int {
            o(0x66 as libc::c_int as libc::c_uint);
        }
        o(pic as libc::c_uint);
        if bt == 1 as libc::c_int || bt == 11 as libc::c_int {
            orex(0 as libc::c_int, 0 as libc::c_int, r, 0x88 as libc::c_int);
        } else if is64_type(bt) != 0 {
            op64 = 0x89 as libc::c_int;
        } else {
            orex(0 as libc::c_int, 0 as libc::c_int, r, 0x89 as libc::c_int);
        }
    }
    if pic != 0 {
        if op64 != 0 {
            o(op64 as libc::c_uint);
        }
        o((3 as libc::c_int + (r << 3 as libc::c_int)) as libc::c_uint);
    } else if op64 != 0 {
        if fr == 0x30 as libc::c_int || fr == 0x32 as libc::c_int
            || (*v).r as libc::c_int & 0x100 as libc::c_int != 0
        {
            gen_modrm64(op64, r, (*v).r as libc::c_int, (*v).c2rust_unnamed_0.sym, fc);
        } else if fr != r {
            orex(1 as libc::c_int, fr, r, op64);
            o((0xc0 as libc::c_int + fr + r * 8 as libc::c_int) as libc::c_uint);
        }
    } else if fr == 0x30 as libc::c_int || fr == 0x32 as libc::c_int
        || (*v).r as libc::c_int & 0x100 as libc::c_int != 0
    {
        gen_modrm(r, (*v).r as libc::c_int, (*v).c2rust_unnamed_0.sym, fc);
    } else if fr != r {
        o((0xc0 as libc::c_int + fr + r * 8 as libc::c_int) as libc::c_uint);
    }
}
unsafe extern "C" fn gcall_or_jmp(mut is_jmp: libc::c_int) {
    let mut r: libc::c_int = 0;
    if (*vtop).r as libc::c_int & (0x3f as libc::c_int | 0x100 as libc::c_int)
        == 0x30 as libc::c_int
        && ((*vtop).r as libc::c_int & 0x200 as libc::c_int != 0
            && ((*vtop).c2rust_unnamed.c.i).wrapping_sub(4 as libc::c_int as uint64_t)
                == ((*vtop).c2rust_unnamed.c.i)
                    .wrapping_sub(4 as libc::c_int as uint64_t) as libc::c_int
                    as uint64_t)
    {
        greloca(
            (*tcc_state).cur_text_section,
            (*vtop).c2rust_unnamed_0.sym,
            (ind + 1 as libc::c_int) as libc::c_ulong,
            4 as libc::c_int,
            ((*vtop).c2rust_unnamed.c.i).wrapping_sub(4 as libc::c_int as uint64_t)
                as libc::c_int as Elf64_Addr,
        );
        oad(0xe8 as libc::c_int + is_jmp, 0 as libc::c_int);
    } else {
        r = TREG_R11 as libc::c_int;
        load(r, vtop);
        o(0x41 as libc::c_int as libc::c_uint);
        o(0xff as libc::c_int as libc::c_uint);
        o(
            (0xd0 as libc::c_int + (r & 7 as libc::c_int) + (is_jmp << 4 as libc::c_int))
                as libc::c_uint,
        );
    };
}
unsafe extern "C" fn gen_bounds_call(mut v: libc::c_int) {
    let mut sym: *mut Sym = external_helper_sym(v);
    oad(0xe8 as libc::c_int, 0 as libc::c_int);
    greloca(
        (*tcc_state).cur_text_section,
        sym,
        (ind - 4 as libc::c_int) as libc::c_ulong,
        4 as libc::c_int,
        -(4 as libc::c_int) as Elf64_Addr,
    );
}
unsafe extern "C" fn gen_bounds_prolog() {
    func_bound_offset = (*(*tcc_state).lbounds_section).data_offset;
    func_bound_ind = ind as libc::c_ulong;
    func_bound_add_epilog = 0 as libc::c_int;
    o(
        (0xd8d48 as libc::c_int
            + (TREG_RDI as libc::c_int == TREG_RDI as libc::c_int) as libc::c_int
                * 0x300000 as libc::c_int) as libc::c_uint,
    );
    gen_le32(0 as libc::c_int);
    oad(0xb8 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn gen_bounds_epilog() {
    let mut saved_ind: Elf64_Addr = 0;
    let mut bounds_ptr: *mut Elf64_Addr = 0 as *mut Elf64_Addr;
    let mut sym_data: *mut Sym = 0 as *mut Sym;
    let mut offset_modified: libc::c_int = (func_bound_offset
        != (*(*tcc_state).lbounds_section).data_offset) as libc::c_int;
    if offset_modified == 0 && func_bound_add_epilog == 0 {
        return;
    }
    bounds_ptr = section_ptr_add(
        (*tcc_state).lbounds_section,
        ::core::mem::size_of::<Elf64_Addr>() as libc::c_ulong,
    ) as *mut Elf64_Addr;
    *bounds_ptr = 0 as libc::c_int as Elf64_Addr;
    sym_data = get_sym_ref(
        &mut char_pointer_type,
        (*tcc_state).lbounds_section,
        func_bound_offset,
        8 as libc::c_int as libc::c_ulong,
    );
    if offset_modified != 0 {
        saved_ind = ind as Elf64_Addr;
        ind = func_bound_ind as libc::c_int;
        greloca(
            (*tcc_state).cur_text_section,
            sym_data,
            (ind + 3 as libc::c_int) as libc::c_ulong,
            2 as libc::c_int,
            -(4 as libc::c_int) as Elf64_Addr,
        );
        ind = ind + 7 as libc::c_int;
        gen_bounds_call(TOK___bound_local_new as libc::c_int);
        ind = saved_ind as libc::c_int;
    }
    o(0x5250 as libc::c_int as libc::c_uint);
    o(0x20ec8348 as libc::c_int as libc::c_uint);
    o(0x290f as libc::c_int as libc::c_uint);
    o(0x102444 as libc::c_int as libc::c_uint);
    o(0x240c290f as libc::c_int as libc::c_uint);
    greloca(
        (*tcc_state).cur_text_section,
        sym_data,
        (ind + 3 as libc::c_int) as libc::c_ulong,
        2 as libc::c_int,
        -(4 as libc::c_int) as Elf64_Addr,
    );
    o(
        (0xd8d48 as libc::c_int
            + (TREG_RDI as libc::c_int == TREG_RDI as libc::c_int) as libc::c_int
                * 0x300000 as libc::c_int) as libc::c_uint,
    );
    gen_le32(0 as libc::c_int);
    gen_bounds_call(TOK___bound_local_delete as libc::c_int);
    o(0x280f as libc::c_int as libc::c_uint);
    o(0x102444 as libc::c_int as libc::c_uint);
    o(0x240c280f as libc::c_int as libc::c_uint);
    o(0x20c48348 as libc::c_int as libc::c_uint);
    o(0x585a as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn gadd_sp(mut val: libc::c_int) {
    if val == val as libc::c_char as libc::c_int {
        o(0xc48348 as libc::c_int as libc::c_uint);
        g(val);
    } else {
        oad(0xc48148 as libc::c_int, val);
    };
}
unsafe extern "C" fn classify_x86_64_merge(
    mut a: X86_64_Mode,
    mut b: X86_64_Mode,
) -> X86_64_Mode {
    if a as libc::c_uint == b as libc::c_uint {
        return a
    } else if a as libc::c_uint == x86_64_mode_none as libc::c_int as libc::c_uint {
        return b
    } else if b as libc::c_uint == x86_64_mode_none as libc::c_int as libc::c_uint {
        return a
    } else if a as libc::c_uint == x86_64_mode_memory as libc::c_int as libc::c_uint
        || b as libc::c_uint == x86_64_mode_memory as libc::c_int as libc::c_uint
    {
        return x86_64_mode_memory
    } else if a as libc::c_uint == x86_64_mode_integer as libc::c_int as libc::c_uint
        || b as libc::c_uint == x86_64_mode_integer as libc::c_int as libc::c_uint
    {
        return x86_64_mode_integer
    } else if a as libc::c_uint == x86_64_mode_x87 as libc::c_int as libc::c_uint
        || b as libc::c_uint == x86_64_mode_x87 as libc::c_int as libc::c_uint
    {
        return x86_64_mode_memory
    } else {
        return x86_64_mode_sse
    };
}
unsafe extern "C" fn classify_x86_64_inner(mut ty: *mut CType) -> X86_64_Mode {
    let mut mode: X86_64_Mode = x86_64_mode_none;
    let mut f: *mut Sym = 0 as *mut Sym;
    match (*ty).t & 0xf as libc::c_int {
        0 => return x86_64_mode_none,
        3 | 1 | 2 | 4 | 11 | 5 | 6 => return x86_64_mode_integer,
        8 | 9 => return x86_64_mode_sse,
        10 => return x86_64_mode_x87,
        7 => {
            f = (*ty).ref_0;
            mode = x86_64_mode_none;
            f = (*f).c2rust_unnamed_0.next;
            while !f.is_null() {
                mode = classify_x86_64_merge(
                    mode,
                    classify_x86_64_inner(&mut (*f).type_0),
                );
                f = (*f).c2rust_unnamed_0.next;
            }
            return mode;
        }
        _ => {}
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
            as *const u8 as *const libc::c_char,
        1122 as libc::c_int as libc::c_uint,
        (*::core::mem::transmute::<
            &[u8; 43],
            &[libc::c_char; 43],
        >(b"X86_64_Mode classify_x86_64_inner(CType *)\0"))
            .as_ptr(),
    );
    'c_11235: {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                as *const u8 as *const libc::c_char,
            1122 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"X86_64_Mode classify_x86_64_inner(CType *)\0"))
                .as_ptr(),
        );
    };
    return x86_64_mode_none;
}
unsafe extern "C" fn classify_x86_64_arg(
    mut ty: *mut CType,
    mut ret: *mut CType,
    mut psize: *mut libc::c_int,
    mut palign: *mut libc::c_int,
    mut reg_count: *mut libc::c_int,
) -> X86_64_Mode {
    let mut mode: X86_64_Mode = x86_64_mode_none;
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut ret_t: libc::c_int = 0 as libc::c_int;
    if (*ty).t & (0x80 as libc::c_int | 0x40 as libc::c_int) != 0 {
        *psize = 8 as libc::c_int;
        *palign = 8 as libc::c_int;
        *reg_count = 1 as libc::c_int;
        ret_t = (*ty).t;
        mode = x86_64_mode_integer;
    } else {
        size = type_size(ty, &mut align);
        *psize = size + 7 as libc::c_int & !(7 as libc::c_int);
        *palign = align + 7 as libc::c_int & !(7 as libc::c_int);
        *reg_count = 0 as libc::c_int;
        if size > 16 as libc::c_int {
            mode = x86_64_mode_memory;
        } else {
            mode = classify_x86_64_inner(ty);
            match mode as libc::c_uint {
                2 => {
                    if size > 8 as libc::c_int {
                        *reg_count = 2 as libc::c_int;
                        ret_t = 13 as libc::c_int;
                    } else {
                        *reg_count = 1 as libc::c_int;
                        if size > 4 as libc::c_int {
                            ret_t = 4 as libc::c_int;
                        } else if size > 2 as libc::c_int {
                            ret_t = 3 as libc::c_int;
                        } else if size > 1 as libc::c_int {
                            ret_t = 2 as libc::c_int;
                        } else {
                            ret_t = 1 as libc::c_int;
                        }
                        if (*ty).t & 0xf as libc::c_int == 7 as libc::c_int
                            || (*ty).t & 0x10 as libc::c_int != 0
                        {
                            ret_t |= 0x10 as libc::c_int;
                        }
                    }
                }
                4 => {
                    *reg_count = 1 as libc::c_int;
                    ret_t = 10 as libc::c_int;
                }
                3 => {
                    if size > 8 as libc::c_int {
                        *reg_count = 2 as libc::c_int;
                        ret_t = 14 as libc::c_int;
                    } else {
                        *reg_count = 1 as libc::c_int;
                        ret_t = if size > 4 as libc::c_int {
                            9 as libc::c_int
                        } else {
                            8 as libc::c_int
                        };
                    }
                }
                _ => {}
            }
        }
    }
    if !ret.is_null() {
        (*ret).ref_0 = 0 as *mut Sym;
        (*ret).t = ret_t;
    }
    return mode;
}
#[no_mangle]
pub unsafe extern "C" fn classify_x86_64_va_arg(mut ty: *mut CType) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut reg_count: libc::c_int = 0;
    let mut mode: X86_64_Mode = classify_x86_64_arg(
        ty,
        0 as *mut CType,
        &mut size,
        &mut align,
        &mut reg_count,
    );
    match mode as libc::c_uint {
        2 => return __va_gen_reg as libc::c_int,
        3 => return __va_float_reg as libc::c_int,
        _ => return __va_stack as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_sret(
    mut vt: *mut CType,
    mut variadic: libc::c_int,
    mut ret: *mut CType,
    mut ret_align: *mut libc::c_int,
    mut regsize: *mut libc::c_int,
) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut reg_count: libc::c_int = 0;
    if classify_x86_64_arg(vt, ret, &mut size, &mut align, &mut reg_count)
        as libc::c_uint == x86_64_mode_memory as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    *ret_align = 1 as libc::c_int;
    *regsize = 8 as libc::c_int * reg_count;
    return 1 as libc::c_int;
}
static mut arg_regs: [uint8_t; 6] = [
    TREG_RDI as libc::c_int as uint8_t,
    TREG_RSI as libc::c_int as uint8_t,
    TREG_RDX as libc::c_int as uint8_t,
    TREG_RCX as libc::c_int as uint8_t,
    TREG_R8 as libc::c_int as uint8_t,
    TREG_R9 as libc::c_int as uint8_t,
];
unsafe extern "C" fn arg_prepare_reg(mut idx: libc::c_int) -> libc::c_int {
    if idx == 2 as libc::c_int || idx == 3 as libc::c_int {
        return idx + 8 as libc::c_int
    } else {
        return if idx >= 0 as libc::c_int && idx < 6 as libc::c_int {
            arg_regs[idx as usize] as libc::c_int
        } else {
            0 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_call(mut nb_args: libc::c_int) {
    let mut mode: X86_64_Mode = x86_64_mode_none;
    let mut type_0: CType = CType {
        t: 0,
        ref_0: 0 as *const Sym as *mut Sym,
    };
    let mut size: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut args_size: libc::c_int = 0;
    let mut stack_adjust: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut reg_count: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nb_reg_args: libc::c_int = 0 as libc::c_int;
    let mut nb_sse_args: libc::c_int = 0 as libc::c_int;
    let mut sse_reg: libc::c_int = 0;
    let mut gen_reg: libc::c_int = 0;
    let mut onstack: *mut libc::c_char = tcc_malloc(
        ((nb_args + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if (*tcc_state).do_bounds_check != 0 {
        gbound_args(nb_args);
    }
    stack_adjust = 0 as libc::c_int;
    i = nb_args - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        mode = classify_x86_64_arg(
            &mut (*vtop.offset(-i as isize)).type_0,
            0 as *mut CType,
            &mut size,
            &mut align,
            &mut reg_count,
        );
        if !(size == 0 as libc::c_int) {
            if mode as libc::c_uint == x86_64_mode_sse as libc::c_int as libc::c_uint
                && nb_sse_args + reg_count <= 8 as libc::c_int
            {
                nb_sse_args += reg_count;
                *onstack.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            } else if mode as libc::c_uint
                == x86_64_mode_integer as libc::c_int as libc::c_uint
                && nb_reg_args + reg_count <= 6 as libc::c_int
            {
                nb_reg_args += reg_count;
                *onstack.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            } else if mode as libc::c_uint
                == x86_64_mode_none as libc::c_int as libc::c_uint
            {
                *onstack.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            } else {
                if align == 16 as libc::c_int
                    && {
                        stack_adjust &= 15 as libc::c_int;
                        stack_adjust != 0
                    }
                {
                    *onstack.offset(i as isize) = 2 as libc::c_int as libc::c_char;
                    stack_adjust = 0 as libc::c_int;
                } else {
                    *onstack.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                }
                stack_adjust += size;
            }
        }
        i -= 1;
        i;
    }
    if nb_sse_args != 0 && (*tcc_state).nosse as libc::c_int != 0 {
        _tcc_error(
            b"SSE disabled but floating point arguments passed\0" as *const u8
                as *const libc::c_char,
        );
    }
    gen_reg = nb_reg_args;
    sse_reg = nb_sse_args;
    args_size = 0 as libc::c_int;
    stack_adjust &= 15 as libc::c_int;
    k = 0 as libc::c_int;
    i = k;
    while i < nb_args {
        mode = classify_x86_64_arg(
            &mut (*vtop.offset(-i as isize)).type_0,
            0 as *mut CType,
            &mut size,
            &mut align,
            &mut reg_count,
        );
        if size != 0 {
            if *onstack.offset((i + k) as isize) == 0 {
                i += 1;
                i;
                continue;
            } else {
                if stack_adjust != 0 {
                    o(0x50 as libc::c_int as libc::c_uint);
                    args_size += 8 as libc::c_int;
                    stack_adjust = 0 as libc::c_int;
                }
                if *onstack.offset((i + k) as isize) as libc::c_int == 2 as libc::c_int {
                    stack_adjust = 1 as libc::c_int;
                }
            }
        }
        vrotb(i + 1 as libc::c_int);
        match (*vtop).type_0.t & 0xf as libc::c_int {
            7 => {
                o(0x48 as libc::c_int as libc::c_uint);
                oad(0xec81 as libc::c_int, size);
                r = get_reg(0x1 as libc::c_int);
                orex(1 as libc::c_int, r, 0 as libc::c_int, 0x89 as libc::c_int);
                o((0xe0 as libc::c_int + (r & 7 as libc::c_int)) as libc::c_uint);
                vset(&mut (*vtop).type_0, r | 0x100 as libc::c_int, 0 as libc::c_int);
                vswap();
                o(0x10ec8348 as libc::c_int as libc::c_uint);
                o(0xf0e48348 as libc::c_uint);
                orex(
                    0 as libc::c_int,
                    r,
                    0 as libc::c_int,
                    0x50 as libc::c_int + (r & 7 as libc::c_int),
                );
                o(0x8ec8348 as libc::c_int as libc::c_uint);
                vstore();
                o(0x8c48348 as libc::c_int as libc::c_uint);
                o(0x5c as libc::c_int as libc::c_uint);
            }
            10 => {
                gv(0x80 as libc::c_int);
                oad(0xec8148 as libc::c_int, size);
                o(0x7cdb as libc::c_int as libc::c_uint);
                g(0x24 as libc::c_int);
                g(0 as libc::c_int);
            }
            8 | 9 => {
                if mode as libc::c_uint == x86_64_mode_sse as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"mode == x86_64_mode_sse\0" as *const u8 as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                            as *const u8 as *const libc::c_char,
                        1341 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void gfunc_call(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_15779: {
                    if mode as libc::c_uint
                        == x86_64_mode_sse as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"mode == x86_64_mode_sse\0" as *const u8
                                as *const libc::c_char,
                            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                as *const u8 as *const libc::c_char,
                            1341 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void gfunc_call(int)\0"))
                                .as_ptr(),
                        );
                    }
                };
                r = gv(0x2 as libc::c_int);
                o(0x50 as libc::c_int as libc::c_uint);
                o(0xd60f66 as libc::c_int as libc::c_uint);
                o(
                    (0x4 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                        as libc::c_uint,
                );
                o(0x24 as libc::c_int as libc::c_uint);
            }
            _ => {
                if mode as libc::c_uint
                    == x86_64_mode_integer as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"mode == x86_64_mode_integer\0" as *const u8
                            as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                            as *const u8 as *const libc::c_char,
                        1351 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void gfunc_call(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_15699: {
                    if mode as libc::c_uint
                        == x86_64_mode_integer as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"mode == x86_64_mode_integer\0" as *const u8
                                as *const libc::c_char,
                            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                as *const u8 as *const libc::c_char,
                            1351 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void gfunc_call(int)\0"))
                                .as_ptr(),
                        );
                    }
                };
                r = gv(0x1 as libc::c_int);
                orex(
                    0 as libc::c_int,
                    r,
                    0 as libc::c_int,
                    0x50 as libc::c_int + (r & 7 as libc::c_int),
                );
            }
        }
        args_size += size;
        vpop();
        nb_args -= 1;
        nb_args;
        k += 1;
        k;
    }
    tcc_free(onstack as *mut libc::c_void);
    save_regs(0 as libc::c_int);
    if gen_reg <= 6 as libc::c_int {} else {
        __assert_fail(
            b"gen_reg <= REGN\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                as *const u8 as *const libc::c_char,
            1374 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void gfunc_call(int)\0"))
                .as_ptr(),
        );
    }
    'c_15558: {
        if gen_reg <= 6 as libc::c_int {} else {
            __assert_fail(
                b"gen_reg <= REGN\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1374 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void gfunc_call(int)\0"))
                    .as_ptr(),
            );
        }
    };
    if sse_reg <= 8 as libc::c_int {} else {
        __assert_fail(
            b"sse_reg <= 8\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                as *const u8 as *const libc::c_char,
            1375 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void gfunc_call(int)\0"))
                .as_ptr(),
        );
    }
    'c_15522: {
        if sse_reg <= 8 as libc::c_int {} else {
            __assert_fail(
                b"sse_reg <= 8\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1375 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void gfunc_call(int)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < nb_args {
        mode = classify_x86_64_arg(
            &mut (*vtop).type_0,
            &mut type_0,
            &mut size,
            &mut align,
            &mut reg_count,
        );
        if !(size == 0 as libc::c_int) {
            (*vtop).type_0 = type_0;
            if mode as libc::c_uint == x86_64_mode_sse as libc::c_int as libc::c_uint {
                if reg_count == 2 as libc::c_int {
                    sse_reg -= 2 as libc::c_int;
                    gv(0x1000 as libc::c_int);
                    if sse_reg != 0 {
                        o(0x280f as libc::c_int as libc::c_uint);
                        o(
                            (0xc1 as libc::c_int
                                + ((sse_reg + 1 as libc::c_int) << 3 as libc::c_int))
                                as libc::c_uint,
                        );
                        o(0x280f as libc::c_int as libc::c_uint);
                        o(
                            (0xc0 as libc::c_int + (sse_reg << 3 as libc::c_int))
                                as libc::c_uint,
                        );
                    }
                } else {
                    if reg_count == 1 as libc::c_int {} else {
                        __assert_fail(
                            b"reg_count == 1\0" as *const u8 as *const libc::c_char,
                            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                as *const u8 as *const libc::c_char,
                            1394 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void gfunc_call(int)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_15384: {
                        if reg_count == 1 as libc::c_int {} else {
                            __assert_fail(
                                b"reg_count == 1\0" as *const u8 as *const libc::c_char,
                                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                                    as *const u8 as *const libc::c_char,
                                1394 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 21],
                                    &[libc::c_char; 21],
                                >(b"void gfunc_call(int)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    sse_reg -= 1;
                    sse_reg;
                    gv((0x1000 as libc::c_int) << sse_reg);
                }
            } else if mode as libc::c_uint
                == x86_64_mode_integer as libc::c_int as libc::c_uint
            {
                let mut d: libc::c_int = 0;
                gen_reg -= reg_count;
                r = gv(0x1 as libc::c_int);
                d = arg_prepare_reg(gen_reg);
                orex(1 as libc::c_int, d, r, 0x89 as libc::c_int);
                o(
                    (0xc0 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int
                        + (d & 7 as libc::c_int)) as libc::c_uint,
                );
                if reg_count == 2 as libc::c_int {
                    d = arg_prepare_reg(gen_reg + 1 as libc::c_int);
                    orex(
                        1 as libc::c_int,
                        d,
                        (*vtop).r2 as libc::c_int,
                        0x89 as libc::c_int,
                    );
                    o(
                        (0xc0 as libc::c_int
                            + ((*vtop).r2 as libc::c_int & 7 as libc::c_int)
                                * 8 as libc::c_int + (d & 7 as libc::c_int)) as libc::c_uint,
                    );
                }
            }
            vtop = vtop.offset(-1);
            vtop;
        }
        i += 1;
        i;
    }
    if gen_reg == 0 as libc::c_int {} else {
        __assert_fail(
            b"gen_reg == 0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                as *const u8 as *const libc::c_char,
            1416 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void gfunc_call(int)\0"))
                .as_ptr(),
        );
    }
    'c_15144: {
        if gen_reg == 0 as libc::c_int {} else {
            __assert_fail(
                b"gen_reg == 0\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1416 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void gfunc_call(int)\0"))
                    .as_ptr(),
            );
        }
    };
    if sse_reg == 0 as libc::c_int {} else {
        __assert_fail(
            b"sse_reg == 0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                as *const u8 as *const libc::c_char,
            1417 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void gfunc_call(int)\0"))
                .as_ptr(),
        );
    }
    'c_15105: {
        if sse_reg == 0 as libc::c_int {} else {
            __assert_fail(
                b"sse_reg == 0\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1417 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void gfunc_call(int)\0"))
                    .as_ptr(),
            );
        }
    };
    save_regs(0 as libc::c_int);
    if nb_reg_args > 2 as libc::c_int {
        o(0xd2894c as libc::c_int as libc::c_uint);
        if nb_reg_args > 3 as libc::c_int {
            o(0xd9894c as libc::c_int as libc::c_uint);
        }
    }
    if ((*(*vtop).type_0.ref_0).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f)
        .func_type() as libc::c_int != 1 as libc::c_int
    {
        oad(
            0xb8 as libc::c_int,
            if nb_sse_args < 8 as libc::c_int { nb_sse_args } else { 8 as libc::c_int },
        );
    }
    gcall_or_jmp(0 as libc::c_int);
    if args_size != 0 {
        gadd_sp(args_size);
    }
    vtop = vtop.offset(-1);
    vtop;
}
unsafe extern "C" fn push_arg_reg(mut i: libc::c_int) {
    loc -= 8 as libc::c_int;
    gen_modrm64(
        0x89 as libc::c_int,
        arg_regs[i as usize] as libc::c_int,
        0x32 as libc::c_int,
        0 as *mut Sym,
        loc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_prolog(mut func_sym: *mut Sym) {
    let mut func_type: *mut CType = &mut (*func_sym).type_0;
    let mut mode: X86_64_Mode = x86_64_mode_none;
    let mut ret_mode: X86_64_Mode = x86_64_mode_none;
    let mut i: libc::c_int = 0;
    let mut addr: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut reg_count: libc::c_int = 0;
    let mut param_addr: libc::c_int = 0 as libc::c_int;
    let mut reg_param_index: libc::c_int = 0;
    let mut sse_param_index: libc::c_int = 0;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut type_0: *mut CType = 0 as *mut CType;
    sym = (*func_type).ref_0;
    addr = 8 as libc::c_int * 2 as libc::c_int;
    loc = 0 as libc::c_int;
    ind += 11 as libc::c_int;
    func_sub_sp_offset = ind as libc::c_ulong;
    func_ret_sub = 0 as libc::c_int;
    ret_mode = classify_x86_64_arg(
        &mut func_vt,
        0 as *mut CType,
        &mut size,
        &mut align,
        &mut reg_count,
    );
    if func_var != 0 {
        let mut seen_reg_num: libc::c_int = 0;
        let mut seen_sse_num: libc::c_int = 0;
        let mut seen_stack_size: libc::c_int = 0;
        seen_reg_num = (ret_mode as libc::c_uint
            == x86_64_mode_memory as libc::c_int as libc::c_uint) as libc::c_int;
        seen_sse_num = 0 as libc::c_int;
        seen_stack_size = 8 as libc::c_int * 2 as libc::c_int;
        sym = (*func_type).ref_0;
        loop {
            sym = (*sym).c2rust_unnamed_0.next;
            if sym.is_null() {
                break;
            }
            type_0 = &mut (*sym).type_0;
            mode = classify_x86_64_arg(
                type_0,
                0 as *mut CType,
                &mut size,
                &mut align,
                &mut reg_count,
            );
            match mode as libc::c_uint {
                2 => {
                    if !(seen_reg_num + reg_count > 6 as libc::c_int) {
                        seen_reg_num += reg_count;
                        continue;
                    }
                }
                3 => {
                    if !(seen_sse_num + reg_count > 8 as libc::c_int) {
                        seen_sse_num += reg_count;
                        continue;
                    }
                }
                _ => {}
            }
            seen_stack_size = (seen_stack_size + align - 1 as libc::c_int & -align)
                + size;
        }
        loc -= 24 as libc::c_int;
        o(0xe845c7 as libc::c_int as libc::c_uint);
        gen_le32(seen_reg_num * 8 as libc::c_int);
        o(0xec45c7 as libc::c_int as libc::c_uint);
        gen_le32(seen_sse_num * 16 as libc::c_int + 48 as libc::c_int);
        o(0x9d8d4c as libc::c_int as libc::c_uint);
        gen_le32(seen_stack_size);
        o(0xf05d894c as libc::c_uint);
        o(0x9d8d4c as libc::c_int as libc::c_uint);
        gen_le32(-(176 as libc::c_int) - 24 as libc::c_int);
        o(0xf85d894c as libc::c_uint);
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            loc -= 16 as libc::c_int;
            if (*tcc_state).nosse == 0 {
                o(0xd60f66 as libc::c_int as libc::c_uint);
                gen_modrm(7 as libc::c_int - i, 0x32 as libc::c_int, 0 as *mut Sym, loc);
            }
            o(0x85c748 as libc::c_int as libc::c_uint);
            gen_le32(loc + 8 as libc::c_int);
            gen_le32(0 as libc::c_int);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            push_arg_reg(6 as libc::c_int - 1 as libc::c_int - i);
            i += 1;
            i;
        }
    }
    sym = (*func_type).ref_0;
    reg_param_index = 0 as libc::c_int;
    sse_param_index = 0 as libc::c_int;
    if ret_mode as libc::c_uint == x86_64_mode_memory as libc::c_int as libc::c_uint {
        push_arg_reg(reg_param_index);
        func_vc = loc;
        reg_param_index += 1;
        reg_param_index;
    }
    loop {
        sym = (*sym).c2rust_unnamed_0.next;
        if sym.is_null() {
            break;
        }
        type_0 = &mut (*sym).type_0;
        mode = classify_x86_64_arg(
            type_0,
            0 as *mut CType,
            &mut size,
            &mut align,
            &mut reg_count,
        );
        match mode as libc::c_uint {
            3 => {
                if (*tcc_state).nosse != 0 {
                    _tcc_error(
                        b"SSE disabled but floating point arguments used\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if sse_param_index + reg_count <= 8 as libc::c_int {
                    loc -= reg_count * 8 as libc::c_int;
                    param_addr = loc;
                    i = 0 as libc::c_int;
                    while i < reg_count {
                        o(0xd60f66 as libc::c_int as libc::c_uint);
                        gen_modrm(
                            sse_param_index,
                            0x32 as libc::c_int,
                            0 as *mut Sym,
                            param_addr + i * 8 as libc::c_int,
                        );
                        sse_param_index += 1;
                        sse_param_index;
                        i += 1;
                        i;
                    }
                } else {
                    addr = addr + align - 1 as libc::c_int & -align;
                    param_addr = addr;
                    addr += size;
                }
            }
            1 | 4 => {
                addr = addr + align - 1 as libc::c_int & -align;
                param_addr = addr;
                addr += size;
            }
            2 => {
                if reg_param_index + reg_count <= 6 as libc::c_int {
                    loc -= reg_count * 8 as libc::c_int;
                    param_addr = loc;
                    i = 0 as libc::c_int;
                    while i < reg_count {
                        gen_modrm64(
                            0x89 as libc::c_int,
                            arg_regs[reg_param_index as usize] as libc::c_int,
                            0x32 as libc::c_int,
                            0 as *mut Sym,
                            param_addr + i * 8 as libc::c_int,
                        );
                        reg_param_index += 1;
                        reg_param_index;
                        i += 1;
                        i;
                    }
                } else {
                    addr = addr + align - 1 as libc::c_int & -align;
                    param_addr = addr;
                    addr += size;
                }
            }
            _ => {}
        }
        sym_push(
            (*sym).v & !(0x20000000 as libc::c_int),
            type_0,
            0x32 as libc::c_int | 0x100 as libc::c_int,
            param_addr,
        );
    }
    if (*tcc_state).do_bounds_check != 0 {
        gen_bounds_prolog();
    }
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_epilog() {
    let mut v: libc::c_int = 0;
    let mut saved_ind: libc::c_int = 0;
    if (*tcc_state).do_bounds_check != 0 {
        gen_bounds_epilog();
    }
    o(0xc9 as libc::c_int as libc::c_uint);
    if func_ret_sub == 0 as libc::c_int {
        o(0xc3 as libc::c_int as libc::c_uint);
    } else {
        o(0xc2 as libc::c_int as libc::c_uint);
        g(func_ret_sub);
        g(func_ret_sub >> 8 as libc::c_int);
    }
    v = -loc + 15 as libc::c_int & -(16 as libc::c_int);
    saved_ind = ind;
    ind = func_sub_sp_offset.wrapping_sub(11 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    o(0xe5894855 as libc::c_uint);
    o(0xec8148 as libc::c_int as libc::c_uint);
    gen_le32(v);
    ind = saved_ind;
}
#[no_mangle]
pub unsafe extern "C" fn gen_fill_nops(mut bytes: libc::c_int) {
    loop {
        let fresh0 = bytes;
        bytes = bytes - 1;
        if !(fresh0 != 0) {
            break;
        }
        g(0x90 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gjmp(mut t: libc::c_int) -> libc::c_int {
    return oad(0xe9 as libc::c_int, t);
}
#[no_mangle]
pub unsafe extern "C" fn gjmp_addr(mut a: libc::c_int) {
    let mut r: libc::c_int = 0;
    r = a - ind - 2 as libc::c_int;
    if r == r as libc::c_char as libc::c_int {
        g(0xeb as libc::c_int);
        g(r);
    } else {
        oad(0xe9 as libc::c_int, a - ind - 5 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gjmp_append(
    mut n: libc::c_int,
    mut t: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if n != 0 {
        let mut n1: uint32_t = n as uint32_t;
        let mut n2: uint32_t = 0;
        loop {
            p = ((*(*tcc_state).cur_text_section).data).offset(n1 as isize)
                as *mut libc::c_void;
            n2 = read32le(p as *mut libc::c_uchar);
            if !(n2 != 0) {
                break;
            }
            n1 = n2;
        }
        write32le(p as *mut libc::c_uchar, t as uint32_t);
        t = n;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn gjmp_cond(
    mut op: libc::c_int,
    mut t: libc::c_int,
) -> libc::c_int {
    if op & 0x100 as libc::c_int != 0 {
        let mut v: libc::c_int = (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_r
            as libc::c_int;
        op &= !(0x100 as libc::c_int);
        if op ^ v ^ (v != 0x95 as libc::c_int) as libc::c_int != 0 {
            o(0x67a as libc::c_int as libc::c_uint);
        } else {
            g(0xf as libc::c_int);
            t = oad(0x8a as libc::c_int, t);
        }
    }
    g(0xf as libc::c_int);
    t = oad(op - 16 as libc::c_int, t);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn gen_opi(mut op: libc::c_int) {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut fr: libc::c_int = 0;
    let mut opc: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    let mut uu: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    ll = is64_type((*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t);
    uu = ((*vtop.offset(-(1 as libc::c_int) as isize)).type_0.t & 0x10 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    cc = ((*vtop).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int)
        == 0x30 as libc::c_int) as libc::c_int;
    match op {
        43 | 135 => {
            opc = 0 as libc::c_int;
            current_block = 10414141199606968855;
        }
        45 | 137 => {
            opc = 5 as libc::c_int;
            current_block = 10414141199606968855;
        }
        136 => {
            opc = 2 as libc::c_int;
            current_block = 10414141199606968855;
        }
        138 => {
            opc = 3 as libc::c_int;
            current_block = 10414141199606968855;
        }
        38 => {
            opc = 4 as libc::c_int;
            current_block = 10414141199606968855;
        }
        94 => {
            opc = 6 as libc::c_int;
            current_block = 10414141199606968855;
        }
        124 => {
            opc = 1 as libc::c_int;
            current_block = 10414141199606968855;
        }
        42 => {
            gv2(0x1 as libc::c_int, 0x1 as libc::c_int);
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            orex(ll, fr, r, 0xaf0f as libc::c_int);
            o(
                (0xc0 as libc::c_int + (fr & 7 as libc::c_int)
                    + (r & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
            );
            vtop = vtop.offset(-1);
            vtop;
            current_block = 5807581744382915773;
        }
        60 => {
            opc = 4 as libc::c_int;
            current_block = 15978470119703930270;
        }
        139 => {
            opc = 5 as libc::c_int;
            current_block = 15978470119703930270;
        }
        62 => {
            opc = 7 as libc::c_int;
            current_block = 15978470119703930270;
        }
        131 | 132 => {
            uu = 1 as libc::c_int;
            current_block = 10556386555271507493;
        }
        47 | 37 | 133 => {
            uu = 0 as libc::c_int;
            current_block = 10556386555271507493;
        }
        _ => {
            opc = 7 as libc::c_int;
            current_block = 10414141199606968855;
        }
    }
    match current_block {
        10556386555271507493 => {
            gv2(0x4 as libc::c_int, 0x10 as libc::c_int);
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            vtop = vtop.offset(-1);
            vtop;
            save_reg(TREG_RDX as libc::c_int);
            orex(
                ll,
                0 as libc::c_int,
                0 as libc::c_int,
                if uu != 0 { 0xd231 as libc::c_int } else { 0x99 as libc::c_int },
            );
            orex(ll, fr, 0 as libc::c_int, 0xf7 as libc::c_int);
            o(
                ((if uu != 0 { 0xf0 as libc::c_int } else { 0xf8 as libc::c_int })
                    + (fr & 7 as libc::c_int)) as libc::c_uint,
            );
            if op == '%' as i32 || op == 0x84 as libc::c_int {
                r = TREG_RDX as libc::c_int;
            } else {
                r = TREG_RAX as libc::c_int;
            }
            (*vtop).r = r as libc::c_ushort;
        }
        15978470119703930270 => {
            opc = 0xc0 as libc::c_int | opc << 3 as libc::c_int;
            if cc != 0 {
                vswap();
                r = gv(0x1 as libc::c_int);
                vswap();
                orex(ll, r, 0 as libc::c_int, 0xc1 as libc::c_int);
                o((opc | r & 7 as libc::c_int) as libc::c_uint);
                g(
                    ((*vtop).c2rust_unnamed.c.i
                        & (if ll != 0 { 63 as libc::c_int } else { 31 as libc::c_int })
                            as uint64_t) as libc::c_int,
                );
            } else {
                gv2(0x1 as libc::c_int, 0x10 as libc::c_int);
                r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
                orex(ll, r, 0 as libc::c_int, 0xd3 as libc::c_int);
                o((opc | r & 7 as libc::c_int) as libc::c_uint);
            }
            vtop = vtop.offset(-1);
            vtop;
        }
        10414141199606968855 => {
            if cc != 0
                && (ll == 0
                    || (*vtop).c2rust_unnamed.c.i as libc::c_int as uint64_t
                        == (*vtop).c2rust_unnamed.c.i)
            {
                vswap();
                r = gv(0x1 as libc::c_int);
                vswap();
                c = (*vtop).c2rust_unnamed.c.i as libc::c_int;
                if c == c as libc::c_char as libc::c_int {
                    orex(ll, r, 0 as libc::c_int, 0x83 as libc::c_int);
                    o(
                        (0xc0 as libc::c_int | opc << 3 as libc::c_int
                            | r & 7 as libc::c_int) as libc::c_uint,
                    );
                    g(c);
                } else {
                    orex(ll, r, 0 as libc::c_int, 0x81 as libc::c_int);
                    oad(
                        0xc0 as libc::c_int | opc << 3 as libc::c_int
                            | r & 7 as libc::c_int,
                        c,
                    );
                }
            } else {
                gv2(0x1 as libc::c_int, 0x1 as libc::c_int);
                r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
                fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
                orex(ll, r, fr, opc << 3 as libc::c_int | 0x1 as libc::c_int);
                o(
                    (0xc0 as libc::c_int + (r & 7 as libc::c_int)
                        + (fr & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
                );
            }
            vtop = vtop.offset(-1);
            vtop;
            if op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int {
                vset_VT_CMP(op);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_opl(mut op: libc::c_int) {
    gen_opi(op);
}
#[no_mangle]
pub unsafe extern "C" fn gen_opf(mut op: libc::c_int) {
    let mut a: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut fc: libc::c_int = 0;
    let mut swapped: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut bt: libc::c_int = (*vtop).type_0.t & 0xf as libc::c_int;
    let mut float_type: libc::c_int = if bt == 10 as libc::c_int {
        0x80 as libc::c_int
    } else {
        0x2 as libc::c_int
    };
    if op == 0x81 as libc::c_int {
        gv(float_type);
        if float_type == 0x80 as libc::c_int {
            o(0xe0d9 as libc::c_int as libc::c_uint);
        } else {
            save_reg((*vtop).r as libc::c_int);
            o(0x80 as libc::c_int as libc::c_uint);
            gen_modrm(
                6 as libc::c_int,
                (*vtop).r as libc::c_int,
                0 as *mut Sym,
                ((*vtop).c2rust_unnamed.c.i)
                    .wrapping_add(
                        (if bt == 9 as libc::c_int {
                            7 as libc::c_int
                        } else {
                            3 as libc::c_int
                        }) as uint64_t,
                    ) as libc::c_int,
            );
            o(0x80 as libc::c_int as libc::c_uint);
        }
        return;
    }
    if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int) == 0x30 as libc::c_int
    {
        vswap();
        gv(float_type);
        vswap();
    }
    if (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int
        & (0x3f as libc::c_int | 0x100 as libc::c_int) == 0x30 as libc::c_int
    {
        gv(float_type);
    }
    if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
        & 0x100 as libc::c_int != 0
        && (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int
            & 0x100 as libc::c_int != 0
    {
        vswap();
        gv(float_type);
        vswap();
    }
    swapped = 0 as libc::c_int;
    if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
        & 0x100 as libc::c_int != 0
    {
        vswap();
        swapped = 1 as libc::c_int;
    }
    if (*vtop).type_0.t & 0xf as libc::c_int == 10 as libc::c_int {
        if op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int {
            load(TREG_ST0 as libc::c_int, vtop);
            save_reg(TREG_RAX as libc::c_int);
            if op == 0x9d as libc::c_int || op == 0x9f as libc::c_int {
                swapped = (swapped == 0) as libc::c_int;
            } else if op == 0x94 as libc::c_int || op == 0x95 as libc::c_int {
                swapped = 0 as libc::c_int;
            }
            if swapped != 0 {
                o(0xc9d9 as libc::c_int as libc::c_uint);
            }
            if op == 0x94 as libc::c_int || op == 0x95 as libc::c_int {
                o(0xe9da as libc::c_int as libc::c_uint);
            } else {
                o(0xd9de as libc::c_int as libc::c_uint);
            }
            o(0xe0df as libc::c_int as libc::c_uint);
            if op == 0x94 as libc::c_int {
                o(0x45e480 as libc::c_int as libc::c_uint);
                o(0x40fc80 as libc::c_int as libc::c_uint);
            } else if op == 0x95 as libc::c_int {
                o(0x45e480 as libc::c_int as libc::c_uint);
                o(0x40f480 as libc::c_int as libc::c_uint);
                op = 0x95 as libc::c_int;
            } else if op == 0x9d as libc::c_int || op == 0x9e as libc::c_int {
                o(0x5c4f6 as libc::c_int as libc::c_uint);
                op = 0x94 as libc::c_int;
            } else {
                o(0x45c4f6 as libc::c_int as libc::c_uint);
                op = 0x94 as libc::c_int;
            }
            vtop = vtop.offset(-1);
            vtop;
            vset_VT_CMP(op);
        } else {
            load(TREG_ST0 as libc::c_int, vtop);
            swapped = (swapped == 0) as libc::c_int;
            match op {
                45 => {
                    a = 4 as libc::c_int;
                    if swapped != 0 {
                        a += 1;
                        a;
                    }
                }
                42 => {
                    a = 1 as libc::c_int;
                }
                47 => {
                    a = 6 as libc::c_int;
                    if swapped != 0 {
                        a += 1;
                        a;
                    }
                }
                43 | _ => {
                    a = 0 as libc::c_int;
                }
            }
            ft = (*vtop).type_0.t;
            fc = (*vtop).c2rust_unnamed.c.i as libc::c_int;
            o(0xde as libc::c_int as libc::c_uint);
            o((0xc1 as libc::c_int + (a << 3 as libc::c_int)) as libc::c_uint);
            vtop = vtop.offset(-1);
            vtop;
        }
    } else if op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int {
        r = (*vtop).r as libc::c_int;
        fc = (*vtop).c2rust_unnamed.c.i as libc::c_int;
        if r & 0x3f as libc::c_int == 0x31 as libc::c_int {
            let mut v1: SValue = SValue {
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
            v1.type_0.t = 5 as libc::c_int;
            v1.r = (0x32 as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
            v1.c2rust_unnamed.c.i = fc as uint64_t;
            load(r, &mut v1);
            fc = 0 as libc::c_int;
            r = r | 0x100 as libc::c_int;
            (*vtop).r = r as libc::c_ushort;
        }
        if op == 0x94 as libc::c_int || op == 0x95 as libc::c_int {
            swapped = 0 as libc::c_int;
        } else {
            if op == 0x9e as libc::c_int || op == 0x9c as libc::c_int {
                swapped = (swapped == 0) as libc::c_int;
            }
            if op == 0x9e as libc::c_int || op == 0x9d as libc::c_int {
                op = 0x93 as libc::c_int;
            } else {
                op = 0x97 as libc::c_int;
            }
        }
        if swapped != 0 {
            gv(0x2 as libc::c_int);
            vswap();
        }
        if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
            & 0x100 as libc::c_int == 0
        {} else {
            __assert_fail(
                b"!(vtop[-1].r & VT_LVAL)\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1966 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void gen_opf(int)\0"))
                    .as_ptr(),
            );
        }
        'c_18903: {
            if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
                & 0x100 as libc::c_int == 0
            {} else {
                __assert_fail(
                    b"!(vtop[-1].r & VT_LVAL)\0" as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    1966 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"void gen_opf(int)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*vtop).type_0.t & 0xf as libc::c_int == 9 as libc::c_int {
            o(0x66 as libc::c_int as libc::c_uint);
        }
        if op == 0x94 as libc::c_int || op == 0x95 as libc::c_int {
            o(0x2e0f as libc::c_int as libc::c_uint);
        } else {
            o(0x2f0f as libc::c_int as libc::c_uint);
        }
        if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {
            gen_modrm(
                (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int,
                r,
                (*vtop).c2rust_unnamed_0.sym,
                fc,
            );
        } else {
            o(
                (0xc0 as libc::c_int
                    + ((*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int
                        & 7 as libc::c_int)
                    + ((*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
                        & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
            );
        }
        vtop = vtop.offset(-1);
        vtop;
        vset_VT_CMP(op | 0x100 as libc::c_int);
        (*vtop).c2rust_unnamed_0.c2rust_unnamed.cmp_r = op as libc::c_ushort;
    } else {
        if (*vtop).type_0.t & 0xf as libc::c_int != 10 as libc::c_int {} else {
            __assert_fail(
                b"(vtop->type.t & VT_BTYPE) != VT_LDOUBLE\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1985 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void gen_opf(int)\0"))
                    .as_ptr(),
            );
        }
        'c_18723: {
            if (*vtop).type_0.t & 0xf as libc::c_int != 10 as libc::c_int {} else {
                __assert_fail(
                    b"(vtop->type.t & VT_BTYPE) != VT_LDOUBLE\0" as *const u8
                        as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    1985 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"void gen_opf(int)\0"))
                        .as_ptr(),
                );
            }
        };
        match op {
            45 => {
                a = 4 as libc::c_int;
            }
            42 => {
                a = 1 as libc::c_int;
            }
            47 => {
                a = 6 as libc::c_int;
            }
            43 | _ => {
                a = 0 as libc::c_int;
            }
        }
        ft = (*vtop).type_0.t;
        fc = (*vtop).c2rust_unnamed.c.i as libc::c_int;
        if ft & 0xf as libc::c_int != 10 as libc::c_int {} else {
            __assert_fail(
                b"(ft & VT_BTYPE) != VT_LDOUBLE\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                2003 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void gen_opf(int)\0"))
                    .as_ptr(),
            );
        }
        'c_18628: {
            if ft & 0xf as libc::c_int != 10 as libc::c_int {} else {
                __assert_fail(
                    b"(ft & VT_BTYPE) != VT_LDOUBLE\0" as *const u8
                        as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    2003 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"void gen_opf(int)\0"))
                        .as_ptr(),
                );
            }
        };
        r = (*vtop).r as libc::c_int;
        if (*vtop).r as libc::c_int & 0x3f as libc::c_int == 0x31 as libc::c_int {
            let mut v1_0: SValue = SValue {
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
            v1_0.type_0.t = 5 as libc::c_int;
            v1_0.r = (0x32 as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
            v1_0.c2rust_unnamed.c.i = fc as uint64_t;
            load(r, &mut v1_0);
            fc = 0 as libc::c_int;
            r = r | 0x100 as libc::c_int;
            (*vtop).r = r as libc::c_ushort;
        }
        if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
            & 0x100 as libc::c_int == 0
        {} else {
            __assert_fail(
                b"!(vtop[-1].r & VT_LVAL)\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                2018 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void gen_opf(int)\0"))
                    .as_ptr(),
            );
        }
        'c_18502: {
            if (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
                & 0x100 as libc::c_int == 0
            {} else {
                __assert_fail(
                    b"!(vtop[-1].r & VT_LVAL)\0" as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    2018 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"void gen_opf(int)\0"))
                        .as_ptr(),
                );
            }
        };
        if swapped != 0 {
            if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {} else {
                __assert_fail(
                    b"vtop->r & VT_LVAL\0" as *const u8 as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    2020 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"void gen_opf(int)\0"))
                        .as_ptr(),
                );
            }
            'c_18457: {
                if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {} else {
                    __assert_fail(
                        b"vtop->r & VT_LVAL\0" as *const u8 as *const libc::c_char,
                        b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                            as *const u8 as *const libc::c_char,
                        2020 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 18],
                            &[libc::c_char; 18],
                        >(b"void gen_opf(int)\0"))
                            .as_ptr(),
                    );
                }
            };
            gv(0x2 as libc::c_int);
            vswap();
            fc = (*vtop).c2rust_unnamed.c.i as libc::c_int;
        }
        if ft & 0xf as libc::c_int == 9 as libc::c_int {
            o(0xf2 as libc::c_int as libc::c_uint);
        } else {
            o(0xf3 as libc::c_int as libc::c_uint);
        }
        o(0xf as libc::c_int as libc::c_uint);
        o((0x58 as libc::c_int + a) as libc::c_uint);
        if (*vtop).r as libc::c_int & 0x100 as libc::c_int != 0 {
            gen_modrm(
                (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int,
                r,
                (*vtop).c2rust_unnamed_0.sym,
                fc,
            );
        } else {
            o(
                (0xc0 as libc::c_int
                    + ((*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int
                        & 7 as libc::c_int)
                    + ((*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int
                        & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
            );
        }
        vtop = vtop.offset(-1);
        vtop;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_itof(mut t: libc::c_int) {
    if t & 0xf as libc::c_int == 10 as libc::c_int {
        save_reg(TREG_ST0 as libc::c_int);
        gv(0x1 as libc::c_int);
        if (*vtop).type_0.t & 0xf as libc::c_int == 4 as libc::c_int {
            o(
                (0x50 as libc::c_int + ((*vtop).r as libc::c_int & 0x3f as libc::c_int))
                    as libc::c_uint,
            );
            o(0x242cdf as libc::c_int as libc::c_uint);
            o(0x8c48348 as libc::c_int as libc::c_uint);
        } else if (*vtop).type_0.t & (0xf as libc::c_int | 0x10 as libc::c_int)
            == 3 as libc::c_int | 0x10 as libc::c_int
        {
            o(0x6a as libc::c_int as libc::c_uint);
            g(0 as libc::c_int);
            o(
                (0x50 as libc::c_int + ((*vtop).r as libc::c_int & 0x3f as libc::c_int))
                    as libc::c_uint,
            );
            o(0x242cdf as libc::c_int as libc::c_uint);
            o(0x10c48348 as libc::c_int as libc::c_uint);
        } else {
            o(
                (0x50 as libc::c_int + ((*vtop).r as libc::c_int & 0x3f as libc::c_int))
                    as libc::c_uint,
            );
            o(0x2404db as libc::c_int as libc::c_uint);
            o(0x8c48348 as libc::c_int as libc::c_uint);
        }
        (*vtop).r = TREG_ST0 as libc::c_int as libc::c_ushort;
    } else {
        let mut r: libc::c_int = get_reg(0x2 as libc::c_int);
        gv(0x1 as libc::c_int);
        o(
            (0xf2 as libc::c_int
                + (if t & 0xf as libc::c_int == 8 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as libc::c_uint,
        );
        if (*vtop).type_0.t & (0xf as libc::c_int | 0x10 as libc::c_int)
            == 3 as libc::c_int | 0x10 as libc::c_int
            || (*vtop).type_0.t & 0xf as libc::c_int == 4 as libc::c_int
        {
            o(0x48 as libc::c_int as libc::c_uint);
        }
        o(0x2a0f as libc::c_int as libc::c_uint);
        o(
            (0xc0 as libc::c_int + ((*vtop).r as libc::c_int & 0x3f as libc::c_int)
                + (r & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
        );
        (*vtop).r = r as libc::c_ushort;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_ftof(mut t: libc::c_int) {
    let mut ft: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut tbt: libc::c_int = 0;
    ft = (*vtop).type_0.t;
    bt = ft & 0xf as libc::c_int;
    tbt = t & 0xf as libc::c_int;
    if bt == 8 as libc::c_int {
        gv(0x2 as libc::c_int);
        if tbt == 9 as libc::c_int {
            o(0x140f as libc::c_int as libc::c_uint);
            o(
                (0xc0 as libc::c_int
                    + ((*vtop).r as libc::c_int & 7 as libc::c_int) * 9 as libc::c_int)
                    as libc::c_uint,
            );
            o(0x5a0f as libc::c_int as libc::c_uint);
            o(
                (0xc0 as libc::c_int
                    + ((*vtop).r as libc::c_int & 7 as libc::c_int) * 9 as libc::c_int)
                    as libc::c_uint,
            );
        } else if tbt == 10 as libc::c_int {
            save_reg(0x80 as libc::c_int);
            o(0x110ff3 as libc::c_int as libc::c_uint);
            o(
                (0x44 as libc::c_int
                    + ((*vtop).r as libc::c_int & 7 as libc::c_int) * 8 as libc::c_int)
                    as libc::c_uint,
            );
            o(0xf024 as libc::c_int as libc::c_uint);
            o(0xf02444d9 as libc::c_uint);
            (*vtop).r = TREG_ST0 as libc::c_int as libc::c_ushort;
        }
    } else if bt == 9 as libc::c_int {
        gv(0x2 as libc::c_int);
        if tbt == 8 as libc::c_int {
            o(0x140f66 as libc::c_int as libc::c_uint);
            o(
                (0xc0 as libc::c_int
                    + ((*vtop).r as libc::c_int & 7 as libc::c_int) * 9 as libc::c_int)
                    as libc::c_uint,
            );
            o(0x5a0f66 as libc::c_int as libc::c_uint);
            o(
                (0xc0 as libc::c_int
                    + ((*vtop).r as libc::c_int & 7 as libc::c_int) * 9 as libc::c_int)
                    as libc::c_uint,
            );
        } else if tbt == 10 as libc::c_int {
            save_reg(0x80 as libc::c_int);
            o(0x110ff2 as libc::c_int as libc::c_uint);
            o(
                (0x44 as libc::c_int
                    + ((*vtop).r as libc::c_int & 7 as libc::c_int) * 8 as libc::c_int)
                    as libc::c_uint,
            );
            o(0xf024 as libc::c_int as libc::c_uint);
            o(0xf02444dd as libc::c_uint);
            (*vtop).r = TREG_ST0 as libc::c_int as libc::c_ushort;
        }
    } else {
        let mut r: libc::c_int = 0;
        gv(0x80 as libc::c_int);
        r = get_reg(0x2 as libc::c_int);
        if tbt == 9 as libc::c_int {
            o(0xf0245cdd as libc::c_uint);
            o(0x100ff2 as libc::c_int as libc::c_uint);
            o(
                (0x44 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                    as libc::c_uint,
            );
            o(0xf024 as libc::c_int as libc::c_uint);
            (*vtop).r = r as libc::c_ushort;
        } else if tbt == 8 as libc::c_int {
            o(0xf0245cd9 as libc::c_uint);
            o(0x100ff3 as libc::c_int as libc::c_uint);
            o(
                (0x44 as libc::c_int + (r & 7 as libc::c_int) * 8 as libc::c_int)
                    as libc::c_uint,
            );
            o(0xf024 as libc::c_int as libc::c_uint);
            (*vtop).r = r as libc::c_ushort;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_ftoi(mut t: libc::c_int) {
    let mut ft: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    ft = (*vtop).type_0.t;
    bt = ft & 0xf as libc::c_int;
    if bt == 10 as libc::c_int {
        gen_cvt_ftof(9 as libc::c_int);
        bt = 9 as libc::c_int;
    }
    gv(0x2 as libc::c_int);
    if t != 3 as libc::c_int {
        size = 8 as libc::c_int;
    } else {
        size = 4 as libc::c_int;
    }
    r = get_reg(0x1 as libc::c_int);
    if bt == 8 as libc::c_int {
        o(0xf3 as libc::c_int as libc::c_uint);
    } else if bt == 9 as libc::c_int {
        o(0xf2 as libc::c_int as libc::c_uint);
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                as *const u8 as *const libc::c_char,
            2174 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void gen_cvt_ftoi(int)\0"))
                .as_ptr(),
        );
        'c_19662: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/x86_64-gen.c\0"
                    as *const u8 as *const libc::c_char,
                2174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void gen_cvt_ftoi(int)\0"))
                    .as_ptr(),
            );
        };
    }
    orex(
        (size == 8 as libc::c_int) as libc::c_int,
        r,
        0 as libc::c_int,
        0x2c0f as libc::c_int,
    );
    o(
        (0xc0 as libc::c_int + ((*vtop).r as libc::c_int & 7 as libc::c_int)
            + (r & 7 as libc::c_int) * 8 as libc::c_int) as libc::c_uint,
    );
    (*vtop).r = r as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_sxtw() {
    let mut r: libc::c_int = gv(0x1 as libc::c_int);
    o(0x6348 as libc::c_int as libc::c_uint);
    o(
        (0xc0 as libc::c_int + ((r & 7 as libc::c_int) << 3 as libc::c_int)
            + (r & 7 as libc::c_int)) as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_csti(mut t: libc::c_int) {
    let mut r: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut xl: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    r = gv(0x1 as libc::c_int);
    sz = (t & 0x10 as libc::c_int == 0) as libc::c_int;
    xl = (t & 0xf as libc::c_int == 2 as libc::c_int) as libc::c_int;
    ll = ((*vtop).type_0.t & 0xf as libc::c_int == 4 as libc::c_int) as libc::c_int;
    orex(
        ll,
        r,
        0 as libc::c_int,
        0xc0b60f as libc::c_int | (sz << 3 as libc::c_int | xl) << 8 as libc::c_int
            | ((r & 7 as libc::c_int) << 3 as libc::c_int | r & 7 as libc::c_int)
                << 16 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_increment_tcov(mut sv: *mut SValue) {
    o(0x58348 as libc::c_int as libc::c_uint);
    greloca(
        (*tcc_state).cur_text_section,
        (*sv).c2rust_unnamed_0.sym,
        ind as libc::c_ulong,
        2 as libc::c_int,
        -(5 as libc::c_int) as Elf64_Addr,
    );
    gen_le32(0 as libc::c_int);
    o(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ggoto() {
    gcall_or_jmp(1 as libc::c_int);
    vtop = vtop.offset(-1);
    vtop;
}
#[no_mangle]
pub unsafe extern "C" fn gen_vla_sp_save(mut addr: libc::c_int) {
    gen_modrm64(
        0x89 as libc::c_int,
        TREG_RSP as libc::c_int,
        0x32 as libc::c_int,
        0 as *mut Sym,
        addr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_vla_sp_restore(mut addr: libc::c_int) {
    gen_modrm64(
        0x8b as libc::c_int,
        TREG_RSP as libc::c_int,
        0x32 as libc::c_int,
        0 as *mut Sym,
        addr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_vla_alloc(mut type_0: *mut CType, mut align: libc::c_int) {
    let mut use_call: libc::c_int = 0 as libc::c_int;
    use_call = (*tcc_state).do_bounds_check as libc::c_int;
    if use_call != 0 {
        vpush_helper_func(TOK_alloca as libc::c_int);
        vswap();
        gfunc_call(1 as libc::c_int);
    } else {
        let mut r: libc::c_int = 0;
        r = gv(0x1 as libc::c_int);
        o(0x2b48 as libc::c_int as libc::c_uint);
        o((0xe0 as libc::c_int | r & 7 as libc::c_int) as libc::c_uint);
        o(0xf0e48348 as libc::c_uint);
        vpop();
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_struct_copy(mut size: libc::c_int) {
    let mut n: libc::c_int = size / 8 as libc::c_int;
    gv2(0x40 as libc::c_int, 0x20 as libc::c_int);
    if n <= 4 as libc::c_int {
        while n != 0 {
            o(0xa548 as libc::c_int as libc::c_uint);
            n -= 1;
            n;
        }
    } else {
        vpushi(n);
        gv(0x10 as libc::c_int);
        o(0xa548f3 as libc::c_int as libc::c_uint);
        vpop();
    }
    if size & 0x4 as libc::c_int != 0 {
        o(0xa5 as libc::c_int as libc::c_uint);
    }
    if size & 0x2 as libc::c_int != 0 {
        o(0xa566 as libc::c_int as libc::c_uint);
    }
    if size & 0x1 as libc::c_int != 0 {
        o(0xa4 as libc::c_int as libc::c_uint);
    }
    vpop();
    vpop();
}
