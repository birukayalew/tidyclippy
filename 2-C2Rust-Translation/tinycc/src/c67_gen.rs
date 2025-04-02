use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    pub type _tccdbg;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut tcc_state: *mut TCCState;
    fn _tcc_error(fmt: *const libc::c_char, _: ...) -> !;
    static mut char_pointer_type: CType;
    static mut vtop: *mut SValue;
    static mut ind: libc::c_int;
    static mut loc: libc::c_int;
    static mut nocode_wanted: libc::c_int;
    static mut func_vt: CType;
    static mut func_vc: libc::c_int;
    fn sym_push(
        v: libc::c_int,
        type_0: *mut CType,
        r: libc::c_int,
        c: libc::c_int,
    ) -> *mut Sym;
    fn vpush_helper_func(v: libc::c_int);
    fn vset_VT_CMP(op: libc::c_int);
    fn vpushi(v: libc::c_int);
    fn vswap();
    fn vrott(n: libc::c_int);
    fn get_reg(rc: libc::c_int) -> libc::c_int;
    fn gv(rc: libc::c_int) -> libc::c_int;
    fn gv2(rc1: libc::c_int, rc2: libc::c_int);
    fn type_size(type_0: *mut CType, a: *mut libc::c_int) -> libc::c_int;
    fn get_sym_ref(
        type_0: *mut CType,
        sec: *mut Section,
        offset: libc::c_ulong,
        size: libc::c_ulong,
    ) -> *mut Sym;
    fn section_realloc(sec: *mut Section, new_size: libc::c_ulong);
    fn gsym(t: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
#[no_mangle]
pub static mut target_machine_defs: *const libc::c_char = b"__C67__\0\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut reg_classes: [libc::c_int; 25] = [0; 25];
#[no_mangle]
pub static mut NoOfCurFuncArgs: libc::c_int = 0;
#[no_mangle]
pub static mut TranslateStackToReg: [libc::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut ParamLocOnStack: [libc::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut TotalBytesPushedOnStack: libc::c_int = 0;
static mut func_sub_sp_offset: libc::c_ulong = 0;
static mut func_ret_sub: libc::c_int = 0;
static mut C67_invert_test: libc::c_int = 0;
static mut C67_compare_reg: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn C67_g(mut c: libc::c_int) {
    let mut ind1: libc::c_int = 0;
    if nocode_wanted != 0 {
        return;
    }
    ind1 = ind + 4 as libc::c_int;
    if ind1 > (*(*tcc_state).cur_text_section).data_allocated as libc::c_int {
        section_realloc((*tcc_state).cur_text_section, ind1 as libc::c_ulong);
    }
    *((*(*tcc_state).cur_text_section).data)
        .offset(ind as isize) = (c & 0xff as libc::c_int) as libc::c_uchar;
    *((*(*tcc_state).cur_text_section).data)
        .offset(
            (ind + 1 as libc::c_int) as isize,
        ) = (c >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *((*(*tcc_state).cur_text_section).data)
        .offset(
            (ind + 2 as libc::c_int) as isize,
        ) = (c >> 16 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *((*(*tcc_state).cur_text_section).data)
        .offset(
            (ind + 3 as libc::c_int) as isize,
        ) = (c >> 24 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    ind = ind1;
}
#[no_mangle]
pub unsafe extern "C" fn gsym_addr(mut t: libc::c_int, mut a: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    while t != 0 {
        ptr = ((*(*tcc_state).cur_text_section).data).offset(t as isize)
            as *mut libc::c_int;
        let mut sym: *mut Sym = 0 as *mut Sym;
        n = *ptr >> 7 as libc::c_int & 0xffff as libc::c_int;
        n
            |= (*ptr.offset(1 as libc::c_int as isize) >> 7 as libc::c_int
                & 0xffff as libc::c_int) << 16 as libc::c_int;
        sym = get_sym_ref(
            &mut char_pointer_type,
            (*tcc_state).cur_text_section,
            a as libc::c_ulong,
            0 as libc::c_int as libc::c_ulong,
        );
        greloc((*tcc_state).cur_text_section, sym, t, 0x54 as libc::c_int);
        greloc(
            (*tcc_state).cur_text_section,
            sym,
            t + 4 as libc::c_int,
            0x55 as libc::c_int,
        );
        *ptr &= !((0xffff as libc::c_int) << 7 as libc::c_int);
        *ptr.offset(1 as libc::c_int as isize)
            &= !((0xffff as libc::c_int) << 7 as libc::c_int);
        t = n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ConvertRegToRegClass(mut r: libc::c_int) -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn C67_map_regc(mut r: libc::c_int) -> libc::c_int {
    if r == 0 as libc::c_int {
        return 0x5 as libc::c_int
    } else if r == 2 as libc::c_int {
        return 0x1 as libc::c_int
    } else if r == 3 as libc::c_int {
        return 0x2 as libc::c_int
    } else if r == 109 as libc::c_int {
        return 0x3 as libc::c_int
    } else if r == -(1 as libc::c_int) {
        return 0 as libc::c_int
    } else if 0 as libc::c_int == 0 {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            312 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn C67_map_S12(mut s: *mut libc::c_char) -> libc::c_int {
    if !(strstr(s, b".S1\0" as *const u8 as *const libc::c_char)).is_null() {
        return 0 as libc::c_int
    } else if strcmp(s, b".S2\0" as *const u8 as *const libc::c_char) != 0 {
        return 1 as libc::c_int
    } else if 0 as libc::c_int == 0 {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            355 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn C67_map_D12(mut s: *mut libc::c_char) -> libc::c_int {
    if !(strstr(s, b".D1\0" as *const u8 as *const libc::c_char)).is_null() {
        return 0 as libc::c_int
    } else if strcmp(s, b".D2\0" as *const u8 as *const libc::c_char) != 0 {
        return 1 as libc::c_int
    } else if 0 as libc::c_int == 0 {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            367 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn C67_asm(
    mut s: *const libc::c_char,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) {
    let mut xpath: libc::c_int = 0;
    if strstr(s, b"MVKL\0" as *const u8 as *const libc::c_char) == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int
                | (a & 0xffff as libc::c_int) << 7 as libc::c_int
                | (0xa as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int,
        );
    } else if strstr(s, b"MVKH\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int
                | (a >> 16 as libc::c_int & 0xffff as libc::c_int) << 7 as libc::c_int
                | (0x1a as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int,
        );
    } else if strstr(s, b"STW.D SP POST DEC\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (2 as libc::c_int) << 13 as libc::c_int
                | (0xa as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (7 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STB.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (3 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STH.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (5 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STB.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (3 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STH.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (5 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STW.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (7 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STW.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(b) << 7 as libc::c_int
                | (7 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STH.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(b) << 7 as libc::c_int
                | (5 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STB.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(b) << 7 as libc::c_int
                | (3 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"STW.D +*\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        if !(c < 32 as libc::c_int) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                492 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | c << 13 as libc::c_int | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(b) << 7 as libc::c_int
                | (7 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDW.D SP PRE INC\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (2 as libc::c_int) << 13 as libc::c_int
                | (9 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDDW.D SP PRE INC\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (9 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDW.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDDW.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDH.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (4 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDB.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (2 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDHU.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDBU.D *+SP[A0]\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(a) << 23 as libc::c_int
                | (15 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (5 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | (0 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(a) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDW.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDDW.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDH.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (4 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDB.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (2 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDHU.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (0 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDBU.D *\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"LDW.D +*\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            C67_map_regn(b) << 23 as libc::c_int | C67_map_regn(a) << 18 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int
                | (0 as libc::c_int) << 8 as libc::c_int
                | C67_map_regs(a) << 7 as libc::c_int
                | (6 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPLTSP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                670 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x3a as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPGTSP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                682 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x39 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPEQSP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                694 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x38 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPLTDP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                708 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x2a as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPGTDP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                720 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x29 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPEQDP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                732 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x28 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPLT\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                744 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x57 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPGT\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                756 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x47 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPEQ\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                768 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x53 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPLTU\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                780 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x5f as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"CMPGTU\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(a) ^ C67_map_regs(b);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                792 as libc::c_int,
            );
        }
        C67_g(
            C67_map_regn(c) << 23 as libc::c_int | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x4f as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"B DISP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int | a << 7 as libc::c_int
                | (0x4 as libc::c_int) << 2 as libc::c_int
                | (0 as libc::c_int) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"B.\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(c) ^ 1 as libc::c_int;
        C67_g(
            C67_map_regc(b) << 29 as libc::c_int | a << 28 as libc::c_int
                | (0 as libc::c_int) << 23 as libc::c_int
                | C67_map_regn(c) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0xd as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"MV.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x2 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SPTRUNC.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0xb as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"DPTRUNC.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | (C67_map_regn(b) + 1 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x1 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"INTSP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x4a as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"INTSPU.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x49 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"INTDP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x39 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"INTDPU.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | (C67_map_regn(b) + 1 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x3b as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SPDP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x2 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"DPSP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        if !(C67_map_regs(b) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                927 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | (C67_map_regn(b) + 1 as libc::c_int) << 18 as libc::c_int
                | (0 as libc::c_int) << 13 as libc::c_int
                | (0 as libc::c_int) << 12 as libc::c_int
                | (0x9 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"ADD.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                942 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x3 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SUB.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                957 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x7 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"OR.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                972 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x7f as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"AND.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                987 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x7b as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"XOR.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1002 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x6f as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"ADDSP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1017 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x10 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"ADDDP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1032 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x18 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SUBSP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1047 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x11 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SUBDP.L\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1062 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x19 as libc::c_int) << 5 as libc::c_int
                | (0x6 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"MPYSP.M\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1077 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x1c as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"MPYDP.M\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1092 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0xe as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"MPYI.M\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(a) == C67_map_regs(c)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1107 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x4 as libc::c_int) << 7 as libc::c_int
                | (0 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SHR.S\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1122 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x37 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SHRU.S\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1137 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x27 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"SHL.S\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = C67_map_regs(b) ^ C67_map_regs(c);
        if !(C67_map_regs(c) == C67_map_regs(a)) {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1152 as libc::c_int,
            );
        }
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(c) << 23 as libc::c_int
                | C67_map_regn(b) << 18 as libc::c_int
                | C67_map_regn(a) << 13 as libc::c_int | xpath << 12 as libc::c_int
                | (0x33 as libc::c_int) << 6 as libc::c_int
                | (0x8 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(c) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"||ADDK\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = 0 as libc::c_int;
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(b) << 23 as libc::c_int | a << 0o7 as libc::c_int
                | (0x14 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"ADDK\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        xpath = 0 as libc::c_int;
        C67_g(
            (0 as libc::c_int) << 29 as libc::c_int
                | (0 as libc::c_int) << 28 as libc::c_int
                | C67_map_regn(b) << 23 as libc::c_int | a << 0o7 as libc::c_int
                | (0x14 as libc::c_int) << 2 as libc::c_int
                | C67_map_regs(b) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if strstr(s, b"NOP\0" as *const u8 as *const libc::c_char)
        == s as *mut libc::c_char
    {
        C67_g(
            (a - 1 as libc::c_int) << 13 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int,
        );
    } else if 0 as libc::c_int == 0 {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            1188 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn C67_MVKL(mut r: libc::c_int, mut fc: libc::c_int) {
    C67_asm(b"MVKL.\0" as *const u8 as *const libc::c_char, fc, r, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_MVKH(mut r: libc::c_int, mut fc: libc::c_int) {
    C67_asm(b"MVKH.\0" as *const u8 as *const libc::c_char, fc, r, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_STB_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"STB.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_STH_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"STH.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_STW_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"STW.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_STB_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"STB.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_STH_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"STH.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_STW_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"STW.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_STW_PTR_PRE_INC(
    mut r: libc::c_int,
    mut r2: libc::c_int,
    mut n: libc::c_int,
) {
    C67_asm(b"STW.D +*\0" as *const u8 as *const libc::c_char, r, r2, n);
}
#[no_mangle]
pub unsafe extern "C" fn C67_PUSH(mut r: libc::c_int) {
    C67_asm(
        b"STW.D SP POST DEC\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDW_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"LDW.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDDW_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"LDDW.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDH_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"LDH.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDB_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"LDB.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDHU_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"LDHU.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDBU_SP_A0(mut r: libc::c_int) {
    C67_asm(
        b"LDBU.D *+SP[A0]\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDW_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDW.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDDW_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDDW.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDH_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDH.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDB_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDB.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDHU_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDHU.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDBU_PTR(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDBU.D *\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_LDW_PTR_PRE_INC(mut r: libc::c_int, mut r2: libc::c_int) {
    C67_asm(b"LDW.D +*\0" as *const u8 as *const libc::c_char, r, r2, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_POP(mut r: libc::c_int) {
    C67_asm(
        b"LDW.D SP PRE INC\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_POP_DW(mut r: libc::c_int) {
    C67_asm(
        b"LDDW.D SP PRE INC\0" as *const u8 as *const libc::c_char,
        r,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPLT(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPLT.L1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPGT(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPGT.L1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPEQ(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPEQ.L1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPLTU(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPLTU.L1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPGTU(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPGTU.L1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPLTSP(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPLTSP.S1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPGTSP(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPGTSP.S1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPEQSP(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPEQSP.S1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPLTDP(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPLTDP.S1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPGTDP(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPGTDP.S1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_CMPEQDP(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut dst: libc::c_int,
) {
    C67_asm(b"CMPEQDP.S1\0" as *const u8 as *const libc::c_char, s1, s2, dst);
}
#[no_mangle]
pub unsafe extern "C" fn C67_IREG_B_REG(
    mut inv: libc::c_int,
    mut r1: libc::c_int,
    mut r2: libc::c_int,
) {
    C67_asm(b"B.S2\0" as *const u8 as *const libc::c_char, inv, r1, r2);
}
#[no_mangle]
pub unsafe extern "C" fn C67_B_DISP(mut disp: libc::c_int) {
    C67_asm(
        b"B DISP\0" as *const u8 as *const libc::c_char,
        disp + ((ind & 31 as libc::c_int) >> 2 as libc::c_int),
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_NOP(mut n: libc::c_int) {
    C67_asm(
        b"NOP\0" as *const u8 as *const libc::c_char,
        n,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn C67_ADDK(mut n: libc::c_int, mut r: libc::c_int) {
    if !(abs(n) < 32767 as libc::c_int) {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            1408 as libc::c_int,
        );
    }
    C67_asm(b"ADDK\0" as *const u8 as *const libc::c_char, n, r, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_ADDK_PARALLEL(mut n: libc::c_int, mut r: libc::c_int) {
    if !(abs(n) < 32767 as libc::c_int) {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            1415 as libc::c_int,
        );
    }
    C67_asm(b"||ADDK\0" as *const u8 as *const libc::c_char, n, r, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn C67_Adjust_ADDK(
    mut inst: *mut libc::c_int,
    mut n: libc::c_int,
) {
    if !(abs(n) < 32767 as libc::c_int) {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            1422 as libc::c_int,
        );
    }
    *inst = *inst & !((0xffff as libc::c_int) << 7 as libc::c_int)
        | (n & 0xffff as libc::c_int) << 7 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn C67_MV(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"MV.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_DPTRUNC(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"DPTRUNC.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SPTRUNC(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SPTRUNC.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_INTSP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"INTSP.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_INTDP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"INTDP.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_INTSPU(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"INTSPU.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_INTDPU(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"INTDPU.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SPDP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SPDP.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_DPSP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"DPSP.L\0" as *const u8 as *const libc::c_char, 0 as libc::c_int, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_ADD(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"ADD.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SUB(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SUB.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_AND(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"AND.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_OR(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"OR.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_XOR(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"XOR.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_ADDSP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"ADDSP.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SUBSP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SUBSP.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_MPYSP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"MPYSP.M\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_ADDDP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"ADDDP.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SUBDP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SUBDP.L\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_MPYDP(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"MPYDP.M\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_MPYI(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"MPYI.M\0" as *const u8 as *const libc::c_char, v, r, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SHL(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SHL.S\0" as *const u8 as *const libc::c_char, r, v, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SHRU(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SHRU.S\0" as *const u8 as *const libc::c_char, r, v, v);
}
#[no_mangle]
pub unsafe extern "C" fn C67_SHR(mut r: libc::c_int, mut v: libc::c_int) {
    C67_asm(b"SHR.S\0" as *const u8 as *const libc::c_char, r, v, v);
}
#[no_mangle]
pub unsafe extern "C" fn load(mut r: libc::c_int, mut sv: *mut SValue) {
    let mut v: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut fc: libc::c_int = 0;
    let mut fr: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut element: libc::c_int = 0;
    let mut Unsigned: libc::c_int = 0 as libc::c_int;
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
    ft = (*sv).type_0.t;
    fc = (*sv).c2rust_unnamed.c.i as libc::c_int;
    v = fr & 0x3f as libc::c_int;
    if fr & 0x100 as libc::c_int != 0 {
        if v == 0x31 as libc::c_int {
            v1.type_0.t = 3 as libc::c_int;
            v1.r = (0x32 as libc::c_int | 0x100 as libc::c_int) as libc::c_ushort;
            v1.c2rust_unnamed.c.i = fc as uint64_t;
            load(r, &mut v1);
            fr = r;
        } else if ft & 0xf as libc::c_int == 10 as libc::c_int {
            _tcc_error(
                b"long double not supported\0" as *const u8 as *const libc::c_char,
            );
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == 1 as libc::c_int as libc::c_uint
        {
            size = 1 as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == (1 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        {
            size = 1 as libc::c_int;
            Unsigned = 1 as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == 2 as libc::c_int as libc::c_uint
        {
            size = 2 as libc::c_int;
        } else if ft as libc::c_uint
            & !((0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_uint
                | (((1 as libc::c_uint) << 6 as libc::c_int + 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) << 20 as libc::c_int
                    | 0x80 as libc::c_int as libc::c_uint))
            == (2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
        {
            size = 2 as libc::c_int;
            Unsigned = 1 as libc::c_int;
        } else if ft & 0xf as libc::c_int == 9 as libc::c_int {
            size = 8 as libc::c_int;
        } else {
            size = 4 as libc::c_int;
        }
        if v == 0x32 as libc::c_int && fc > 0 as libc::c_int {
            let mut stack_pos: libc::c_int = 8 as libc::c_int;
            t = 0 as libc::c_int;
            while t < 10 as libc::c_int {
                if fc == stack_pos {
                    break;
                }
                stack_pos += TranslateStackToReg[t as usize];
                t += 1;
                t;
            }
            fc = ParamLocOnStack[t as usize] - 8 as libc::c_int;
        }
        if (fr & 0x3f as libc::c_int) < 0x30 as libc::c_int {
            if size == 1 as libc::c_int {
                if Unsigned != 0 {
                    C67_LDBU_PTR(v, r);
                } else {
                    C67_LDB_PTR(v, r);
                }
            } else if size == 2 as libc::c_int {
                if Unsigned != 0 {
                    C67_LDHU_PTR(v, r);
                } else {
                    C67_LDH_PTR(v, r);
                }
            } else if size == 4 as libc::c_int {
                C67_LDW_PTR(v, r);
            } else if size == 8 as libc::c_int {
                C67_LDDW_PTR(v, r);
            }
            C67_NOP(4 as libc::c_int);
            return;
        } else if fr & 0x200 as libc::c_int != 0 {
            greloc(
                (*tcc_state).cur_text_section,
                (*sv).c2rust_unnamed_0.sym,
                ind,
                0x54 as libc::c_int,
            );
            greloc(
                (*tcc_state).cur_text_section,
                (*sv).c2rust_unnamed_0.sym,
                ind + 4 as libc::c_int,
                0x55 as libc::c_int,
            );
            C67_MVKL(105 as libc::c_int, fc);
            C67_MVKH(105 as libc::c_int, fc);
            if size == 1 as libc::c_int {
                if Unsigned != 0 {
                    C67_LDBU_PTR(105 as libc::c_int, r);
                } else {
                    C67_LDB_PTR(105 as libc::c_int, r);
                }
            } else if size == 2 as libc::c_int {
                if Unsigned != 0 {
                    C67_LDHU_PTR(105 as libc::c_int, r);
                } else {
                    C67_LDH_PTR(105 as libc::c_int, r);
                }
            } else if size == 4 as libc::c_int {
                C67_LDW_PTR(105 as libc::c_int, r);
            } else if size == 8 as libc::c_int {
                C67_LDDW_PTR(105 as libc::c_int, r);
            }
            C67_NOP(4 as libc::c_int);
            return;
        } else {
            element = size;
            C67_MVKL(105 as libc::c_int, fc / element + 8 as libc::c_int / element);
            C67_MVKH(105 as libc::c_int, fc / element + 8 as libc::c_int / element);
            if size == 1 as libc::c_int {
                if Unsigned != 0 {
                    C67_LDBU_SP_A0(r);
                } else {
                    C67_LDB_SP_A0(r);
                }
            } else if size == 2 as libc::c_int {
                if Unsigned != 0 {
                    C67_LDHU_SP_A0(r);
                } else {
                    C67_LDH_SP_A0(r);
                }
            } else if size == 4 as libc::c_int {
                C67_LDW_SP_A0(r);
            } else if size == 8 as libc::c_int {
                C67_LDDW_SP_A0(r);
            }
            C67_NOP(4 as libc::c_int);
            return;
        }
    } else if v == 0x30 as libc::c_int {
        if fr & 0x200 as libc::c_int != 0 {
            greloc(
                (*tcc_state).cur_text_section,
                (*sv).c2rust_unnamed_0.sym,
                ind,
                0x54 as libc::c_int,
            );
            greloc(
                (*tcc_state).cur_text_section,
                (*sv).c2rust_unnamed_0.sym,
                ind + 4 as libc::c_int,
                0x55 as libc::c_int,
            );
        }
        C67_MVKL(r, fc);
        C67_MVKH(r, fc);
    } else if v == 0x32 as libc::c_int {
        C67_MVKL(r, fc + 8 as libc::c_int);
        C67_MVKH(r, fc + 8 as libc::c_int);
        C67_ADD(108 as libc::c_int, r);
    } else if v == 0x33 as libc::c_int {
        C67_MV(C67_compare_reg, r);
    } else if v == 0x34 as libc::c_int || v == 0x35 as libc::c_int {
        t = v & 1 as libc::c_int;
        C67_B_DISP(4 as libc::c_int);
        C67_MVKL(r, t);
        C67_NOP(4 as libc::c_int);
        gsym(fc);
        C67_MVKL(r, t ^ 1 as libc::c_int);
    } else if v != r {
        C67_MV(v, r);
        if ft & 0xf as libc::c_int == 9 as libc::c_int {
            C67_MV(v + 1 as libc::c_int, r + 1 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn store(mut r: libc::c_int, mut v: *mut SValue) {
    let mut fr: libc::c_int = 0;
    let mut bt: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut fc: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut element: libc::c_int = 0;
    ft = (*v).type_0.t;
    fc = (*v).c2rust_unnamed.c.i as libc::c_int;
    fr = (*v).r as libc::c_int & 0x3f as libc::c_int;
    bt = ft & 0xf as libc::c_int;
    if bt == 10 as libc::c_int {
        _tcc_error(b"long double not supported\0" as *const u8 as *const libc::c_char);
    } else {
        if bt == 2 as libc::c_int {
            size = 2 as libc::c_int;
        } else if bt == 1 as libc::c_int {
            size = 1 as libc::c_int;
        } else if bt == 9 as libc::c_int {
            size = 8 as libc::c_int;
        } else {
            size = 4 as libc::c_int;
        }
        if (*v).r as libc::c_int & 0x3f as libc::c_int == 0x30 as libc::c_int {
            if (*v).r as libc::c_int & 0x200 as libc::c_int != 0 {
                greloc(
                    (*tcc_state).cur_text_section,
                    (*v).c2rust_unnamed_0.sym,
                    ind,
                    0x54 as libc::c_int,
                );
                greloc(
                    (*tcc_state).cur_text_section,
                    (*v).c2rust_unnamed_0.sym,
                    ind + 4 as libc::c_int,
                    0x55 as libc::c_int,
                );
            }
            C67_MVKL(105 as libc::c_int, fc);
            C67_MVKH(105 as libc::c_int, fc);
            if size == 1 as libc::c_int {
                C67_STB_PTR(r, 105 as libc::c_int);
            } else if size == 2 as libc::c_int {
                C67_STH_PTR(r, 105 as libc::c_int);
            } else if size == 4 as libc::c_int || size == 8 as libc::c_int {
                C67_STW_PTR(r, 105 as libc::c_int);
            }
            if size == 8 as libc::c_int {
                C67_STW_PTR_PRE_INC(
                    r + 1 as libc::c_int,
                    105 as libc::c_int,
                    1 as libc::c_int,
                );
            }
        } else if (*v).r as libc::c_int & 0x3f as libc::c_int == 0x32 as libc::c_int {
            if fc > 0 as libc::c_int {
                let mut stack_pos: libc::c_int = 8 as libc::c_int;
                t = 0 as libc::c_int;
                while t < 10 as libc::c_int {
                    if fc == stack_pos {
                        break;
                    }
                    stack_pos += TranslateStackToReg[t as usize];
                    t += 1;
                    t;
                }
                fc = ParamLocOnStack[t as usize] - 8 as libc::c_int;
            }
            if size == 8 as libc::c_int {
                element = 4 as libc::c_int;
            } else {
                element = size;
            }
            C67_MVKL(105 as libc::c_int, fc / element + 8 as libc::c_int / element);
            C67_MVKH(105 as libc::c_int, fc / element + 8 as libc::c_int / element);
            if size == 1 as libc::c_int {
                C67_STB_SP_A0(r);
            } else if size == 2 as libc::c_int {
                C67_STH_SP_A0(r);
            } else if size == 4 as libc::c_int || size == 8 as libc::c_int {
                C67_STW_SP_A0(r);
            }
            if size == 8 as libc::c_int {
                C67_ADDK(1 as libc::c_int, 105 as libc::c_int);
                C67_STW_SP_A0(r + 1 as libc::c_int);
            }
        } else {
            if size == 1 as libc::c_int {
                C67_STB_PTR(r, fr);
            } else if size == 2 as libc::c_int {
                C67_STH_PTR(r, fr);
            } else if size == 4 as libc::c_int || size == 8 as libc::c_int {
                C67_STW_PTR(r, fr);
            }
            if size == 8 as libc::c_int {
                C67_STW_PTR_PRE_INC(r + 1 as libc::c_int, fr, 1 as libc::c_int);
            }
        }
    };
}
unsafe extern "C" fn gcall_or_jmp(mut is_jmp: libc::c_int) {
    let mut r: libc::c_int = 0;
    let mut sym: *mut Sym = 0 as *mut Sym;
    if (*vtop).r as libc::c_int & (0x3f as libc::c_int | 0x100 as libc::c_int)
        == 0x30 as libc::c_int
    {
        if (*vtop).r as libc::c_int & 0x200 as libc::c_int != 0 {
            greloc(
                (*tcc_state).cur_text_section,
                (*vtop).c2rust_unnamed_0.sym,
                ind,
                0x54 as libc::c_int,
            );
            greloc(
                (*tcc_state).cur_text_section,
                (*vtop).c2rust_unnamed_0.sym,
                ind + 4 as libc::c_int,
                0x55 as libc::c_int,
            );
            C67_MVKL(105 as libc::c_int, 0 as libc::c_int);
            C67_MVKH(105 as libc::c_int, 0 as libc::c_int);
            C67_IREG_B_REG(0 as libc::c_int, -(1 as libc::c_int), 105 as libc::c_int);
            if is_jmp != 0 {
                C67_NOP(5 as libc::c_int);
            } else {
                sym = get_sym_ref(
                    &mut char_pointer_type,
                    (*tcc_state).cur_text_section,
                    (ind + 12 as libc::c_int) as libc::c_ulong,
                    0 as libc::c_int as libc::c_ulong,
                );
                greloc((*tcc_state).cur_text_section, sym, ind, 0x54 as libc::c_int);
                greloc(
                    (*tcc_state).cur_text_section,
                    sym,
                    ind + 4 as libc::c_int,
                    0x55 as libc::c_int,
                );
                C67_MVKL(107 as libc::c_int, 0 as libc::c_int);
                C67_MVKH(107 as libc::c_int, 0 as libc::c_int);
                C67_NOP(3 as libc::c_int);
            }
        } else if 0 as libc::c_int == 0 {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                1849 as libc::c_int,
            );
        }
    } else {
        r = gv(0x1 as libc::c_int);
        C67_IREG_B_REG(0 as libc::c_int, -(1 as libc::c_int), r);
        if is_jmp != 0 {
            C67_NOP(5 as libc::c_int);
        } else {
            sym = get_sym_ref(
                &mut char_pointer_type,
                (*tcc_state).cur_text_section,
                (ind + 12 as libc::c_int) as libc::c_ulong,
                0 as libc::c_int as libc::c_ulong,
            );
            greloc((*tcc_state).cur_text_section, sym, ind, 0x54 as libc::c_int);
            greloc(
                (*tcc_state).cur_text_section,
                sym,
                ind + 4 as libc::c_int,
                0x55 as libc::c_int,
            );
            C67_MVKL(107 as libc::c_int, 0 as libc::c_int);
            C67_MVKH(107 as libc::c_int, 0 as libc::c_int);
            C67_NOP(3 as libc::c_int);
        }
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
    *ret_align = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_call(mut nb_args: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut args_sizes: [libc::c_int; 10] = [0; 10];
    if nb_args > 10 as libc::c_int {
        _tcc_error(
            b"more than 10 function params not currently supported\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < nb_args {
        if (*vtop).type_0.t & 0xf as libc::c_int == 7 as libc::c_int {
            if 0 as libc::c_int == 0 {
                _tcc_error(
                    b"internal compiler error file at %s:%d\0" as *const u8
                        as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    1892 as libc::c_int,
                );
            }
        } else {
            if (*vtop).type_0.t & 0xf as libc::c_int == 4 as libc::c_int {
                _tcc_error(
                    b"long long not supported\0" as *const u8 as *const libc::c_char,
                );
            } else if (*vtop).type_0.t & 0xf as libc::c_int == 10 as libc::c_int {
                _tcc_error(
                    b"long double not supported\0" as *const u8 as *const libc::c_char,
                );
            } else if (*vtop).type_0.t & 0xf as libc::c_int == 9 as libc::c_int {
                size = 8 as libc::c_int;
            } else {
                size = 4 as libc::c_int;
            }
            C67_PUSH(r);
            if size == 8 as libc::c_int {
                C67_STW_PTR_PRE_INC(
                    r + 1 as libc::c_int,
                    106 as libc::c_int,
                    3 as libc::c_int,
                );
            }
            args_sizes[i as usize] = size;
        }
        vtop = vtop.offset(-1);
        vtop;
        i += 1;
        i;
    }
    i = nb_args - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        i -= 1;
        i;
    }
    gcall_or_jmp(0 as libc::c_int);
    vtop = vtop.offset(-1);
    vtop;
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_prolog(mut func_sym: *mut Sym) {
    let mut func_type: *mut CType = &mut (*func_sym).type_0;
    let mut addr: libc::c_int = 0;
    let mut align: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut func_call: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sym: *mut Sym = 0 as *mut Sym;
    let mut type_0: *mut CType = 0 as *mut CType;
    sym = (*func_type).ref_0;
    func_call = ((*sym).c2rust_unnamed.c2rust_unnamed.c2rust_unnamed.f).func_call()
        as libc::c_int;
    addr = 8 as libc::c_int;
    if func_vt.t & 0xf as libc::c_int == 7 as libc::c_int {
        func_vc = addr;
        addr += 4 as libc::c_int;
    }
    NoOfCurFuncArgs = 0 as libc::c_int;
    loop {
        sym = (*sym).c2rust_unnamed_0.next;
        if sym.is_null() {
            break;
        }
        type_0 = &mut (*sym).type_0;
        sym_push(
            (*sym).v & !(0x20000000 as libc::c_int),
            type_0,
            0x32 as libc::c_int | 0x100 as libc::c_int,
            addr,
        );
        size = type_size(type_0, &mut align);
        size = size + 3 as libc::c_int & !(3 as libc::c_int);
        TranslateStackToReg[NoOfCurFuncArgs as usize] = size;
        NoOfCurFuncArgs += 1;
        NoOfCurFuncArgs;
        addr += size;
    }
    func_ret_sub = 0 as libc::c_int;
    if func_call == 1 as libc::c_int {
        func_ret_sub = addr - 8 as libc::c_int;
    }
    C67_MV(108 as libc::c_int, 105 as libc::c_int);
    C67_MV(106 as libc::c_int, 108 as libc::c_int);
    loc = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < NoOfCurFuncArgs {
        ParamLocOnStack[i as usize] = loc;
        loc += -(8 as libc::c_int);
        TranslateStackToReg[i as usize] == 8 as libc::c_int;
        i += 1;
        i;
    }
    TotalBytesPushedOnStack = -loc;
    func_sub_sp_offset = ind as libc::c_ulong;
    C67_ADDK(0 as libc::c_int, 106 as libc::c_int);
    C67_PUSH(105 as libc::c_int);
    C67_PUSH(107 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gfunc_epilog() {
    let mut local: libc::c_int = -loc + 7 as libc::c_int & -(8 as libc::c_int);
    C67_POP(107 as libc::c_int);
    C67_NOP(4 as libc::c_int);
    C67_IREG_B_REG(0 as libc::c_int, -(1 as libc::c_int), 107 as libc::c_int);
    C67_POP(108 as libc::c_int);
    C67_ADDK(local, 106 as libc::c_int);
    C67_Adjust_ADDK(
        ((*(*tcc_state).cur_text_section).data).offset(func_sub_sp_offset as isize)
            as *mut libc::c_int,
        -local + TotalBytesPushedOnStack,
    );
    C67_NOP(3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gen_fill_nops(mut bytes: libc::c_int) {
    if bytes & 3 as libc::c_int != 0 {
        _tcc_error(
            b"alignment of code section not multiple of 4\0" as *const u8
                as *const libc::c_char,
        );
    }
    while bytes > 0 as libc::c_int {
        C67_NOP(4 as libc::c_int);
        bytes -= 4 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gjmp(mut t: libc::c_int) -> libc::c_int {
    let mut ind1: libc::c_int = ind;
    if nocode_wanted != 0 {
        return t;
    }
    C67_MVKL(105 as libc::c_int, t);
    C67_MVKH(105 as libc::c_int, t);
    C67_IREG_B_REG(0 as libc::c_int, -(1 as libc::c_int), 105 as libc::c_int);
    C67_NOP(5 as libc::c_int);
    return ind1;
}
#[no_mangle]
pub unsafe extern "C" fn gjmp_addr(mut a: libc::c_int) {
    let mut sym: *mut Sym = 0 as *mut Sym;
    sym = get_sym_ref(
        &mut char_pointer_type,
        (*tcc_state).cur_text_section,
        a as libc::c_ulong,
        0 as libc::c_int as libc::c_ulong,
    );
    greloc((*tcc_state).cur_text_section, sym, ind, 0x54 as libc::c_int);
    greloc(
        (*tcc_state).cur_text_section,
        sym,
        ind + 4 as libc::c_int,
        0x55 as libc::c_int,
    );
    gjmp(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gjmp_append(
    mut n0: libc::c_int,
    mut t: libc::c_int,
) -> libc::c_int {
    if n0 != 0 {
        let mut n: libc::c_int = n0;
        let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
        while n != 0 as libc::c_int {
            p = ((*(*tcc_state).cur_text_section).data).offset(n as isize)
                as *mut libc::c_int;
            n = *p >> 7 as libc::c_int & 0xffff as libc::c_int;
            n
                |= (*p.offset(1 as libc::c_int as isize) >> 7 as libc::c_int
                    & 0xffff as libc::c_int) << 16 as libc::c_int;
        }
        *p |= (t & 0xffff as libc::c_int) << 7 as libc::c_int;
        *p.offset(1 as libc::c_int as isize)
            |= (t >> 16 as libc::c_int & 0xffff as libc::c_int) << 7 as libc::c_int;
        t = n0;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn gen_opi(mut op: libc::c_int) {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut fr: libc::c_int = 0;
    let mut opc: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    match op {
        43 | 135 => {
            opc = 0 as libc::c_int;
            current_block = 13718251366282216682;
        }
        45 | 137 => {
            opc = 5 as libc::c_int;
            current_block = 13718251366282216682;
        }
        136 => {
            opc = 2 as libc::c_int;
            current_block = 13718251366282216682;
        }
        138 => {
            opc = 3 as libc::c_int;
            current_block = 13718251366282216682;
        }
        38 => {
            opc = 4 as libc::c_int;
            current_block = 13718251366282216682;
        }
        94 => {
            opc = 6 as libc::c_int;
            current_block = 13718251366282216682;
        }
        124 => {
            opc = 1 as libc::c_int;
            current_block = 13718251366282216682;
        }
        42 | 134 => {
            gv2(0x1 as libc::c_int, 0x1 as libc::c_int);
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            vtop = vtop.offset(-1);
            vtop;
            C67_MPYI(fr, r);
            C67_NOP(8 as libc::c_int);
            current_block = 6471821049853688503;
        }
        60 => {
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            vtop = vtop.offset(-1);
            vtop;
            C67_SHL(fr, r);
            current_block = 6471821049853688503;
        }
        139 => {
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            vtop = vtop.offset(-1);
            vtop;
            C67_SHRU(fr, r);
            current_block = 6471821049853688503;
        }
        62 => {
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            vtop = vtop.offset(-1);
            vtop;
            C67_SHR(fr, r);
            current_block = 6471821049853688503;
        }
        47 | 131 | 133 | 37 | 132 => {
            vswap();
            vpush_helper_func(t);
            vrott(3 as libc::c_int);
            gfunc_call(2 as libc::c_int);
            vpushi(0 as libc::c_int);
            (*vtop).r = TREG_RAX as libc::c_int as libc::c_ushort;
            (*vtop).r2 = 0x30 as libc::c_int as libc::c_ushort;
            current_block = 6471821049853688503;
        }
        _ => {
            opc = 7 as libc::c_int;
            current_block = 13718251366282216682;
        }
    }
    match current_block {
        13718251366282216682 => {
            if !(op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int) {
                gv2(0x1 as libc::c_int, 0x1 as libc::c_int);
            }
            r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
            fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
            C67_compare_reg = 109 as libc::c_int;
            if op == 0x9c as libc::c_int {
                C67_CMPLT(r, fr, 109 as libc::c_int);
                C67_invert_test = 0 as libc::c_int;
            } else if op == 0x9d as libc::c_int {
                C67_CMPLT(r, fr, 109 as libc::c_int);
                C67_invert_test = 1 as libc::c_int;
            } else if op == 0x9f as libc::c_int {
                C67_CMPGT(r, fr, 109 as libc::c_int);
                C67_invert_test = 0 as libc::c_int;
            } else if op == 0x9e as libc::c_int {
                C67_CMPGT(r, fr, 109 as libc::c_int);
                C67_invert_test = 1 as libc::c_int;
            } else if op == 0x94 as libc::c_int {
                C67_CMPEQ(r, fr, 109 as libc::c_int);
                C67_invert_test = 0 as libc::c_int;
            } else if op == 0x95 as libc::c_int {
                C67_CMPEQ(r, fr, 109 as libc::c_int);
                C67_invert_test = 1 as libc::c_int;
            } else if op == 0x92 as libc::c_int {
                C67_CMPLTU(r, fr, 109 as libc::c_int);
                C67_invert_test = 0 as libc::c_int;
            } else if op == 0x93 as libc::c_int {
                C67_CMPLTU(r, fr, 109 as libc::c_int);
                C67_invert_test = 1 as libc::c_int;
            } else if op == 0x97 as libc::c_int {
                C67_CMPGTU(r, fr, 109 as libc::c_int);
                C67_invert_test = 0 as libc::c_int;
            } else if op == 0x96 as libc::c_int {
                C67_CMPGTU(r, fr, 109 as libc::c_int);
                C67_invert_test = 1 as libc::c_int;
            } else if op == '+' as i32 {
                C67_ADD(fr, r);
            } else if op == '-' as i32 {
                C67_SUB(fr, r);
            } else if op == '&' as i32 {
                C67_AND(fr, r);
            } else if op == '|' as i32 {
                C67_OR(fr, r);
            } else if op == '^' as i32 {
                C67_XOR(fr, r);
            } else if 0 as libc::c_int == 0 {
                _tcc_error(
                    b"internal compiler error file at %s:%d\0" as *const u8
                        as *const libc::c_char,
                    b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                        as *const u8 as *const libc::c_char,
                    2200 as libc::c_int,
                );
            }
            vtop = vtop.offset(-1);
            vtop;
            if op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int {
                vset_VT_CMP(0x80 as libc::c_int);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_opf(mut op: libc::c_int) {
    let mut ft: libc::c_int = 0;
    let mut fc: libc::c_int = 0;
    let mut fr: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if !(op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int) {
        gv2(0x2 as libc::c_int, 0x2 as libc::c_int);
    }
    ft = (*vtop).type_0.t;
    fc = (*vtop).c2rust_unnamed.c.i as libc::c_int;
    r = (*vtop).r as libc::c_int;
    fr = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
    if ft & 0xf as libc::c_int == 10 as libc::c_int {
        _tcc_error(b"long doubles not supported\0" as *const u8 as *const libc::c_char);
    }
    if op >= 0x92 as libc::c_int && op <= 0x9f as libc::c_int {
        r = (*vtop.offset(-(1 as libc::c_int) as isize)).r as libc::c_int;
        fr = (*vtop.offset(0 as libc::c_int as isize)).r as libc::c_int;
        C67_compare_reg = 109 as libc::c_int;
        if op == 0x9c as libc::c_int {
            if ft & 0xf as libc::c_int == 9 as libc::c_int {
                C67_CMPLTDP(r, fr, 109 as libc::c_int);
            } else {
                C67_CMPLTSP(r, fr, 109 as libc::c_int);
            }
            C67_invert_test = 0 as libc::c_int;
        } else if op == 0x9d as libc::c_int {
            if ft & 0xf as libc::c_int == 9 as libc::c_int {
                C67_CMPLTDP(r, fr, 109 as libc::c_int);
            } else {
                C67_CMPLTSP(r, fr, 109 as libc::c_int);
            }
            C67_invert_test = 1 as libc::c_int;
        } else if op == 0x9f as libc::c_int {
            if ft & 0xf as libc::c_int == 9 as libc::c_int {
                C67_CMPGTDP(r, fr, 109 as libc::c_int);
            } else {
                C67_CMPGTSP(r, fr, 109 as libc::c_int);
            }
            C67_invert_test = 0 as libc::c_int;
        } else if op == 0x9e as libc::c_int {
            if ft & 0xf as libc::c_int == 9 as libc::c_int {
                C67_CMPGTDP(r, fr, 109 as libc::c_int);
            } else {
                C67_CMPGTSP(r, fr, 109 as libc::c_int);
            }
            C67_invert_test = 1 as libc::c_int;
        } else if op == 0x94 as libc::c_int {
            if ft & 0xf as libc::c_int == 9 as libc::c_int {
                C67_CMPEQDP(r, fr, 109 as libc::c_int);
            } else {
                C67_CMPEQSP(r, fr, 109 as libc::c_int);
            }
            C67_invert_test = 0 as libc::c_int;
        } else if op == 0x95 as libc::c_int {
            if ft & 0xf as libc::c_int == 9 as libc::c_int {
                C67_CMPEQDP(r, fr, 109 as libc::c_int);
            } else {
                C67_CMPEQSP(r, fr, 109 as libc::c_int);
            }
            C67_invert_test = 1 as libc::c_int;
        } else if 0 as libc::c_int == 0 {
            _tcc_error(
                b"internal compiler error file at %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                    as *const u8 as *const libc::c_char,
                2358 as libc::c_int,
            );
        }
        vset_VT_CMP(0x80 as libc::c_int);
    } else if op == '+' as i32 {
        if ft & 0xf as libc::c_int == 9 as libc::c_int {
            C67_ADDDP(r, fr);
            C67_NOP(6 as libc::c_int);
        } else {
            C67_ADDSP(r, fr);
            C67_NOP(3 as libc::c_int);
        }
        vtop = vtop.offset(-1);
        vtop;
    } else if op == '-' as i32 {
        if ft & 0xf as libc::c_int == 9 as libc::c_int {
            C67_SUBDP(r, fr);
            C67_NOP(6 as libc::c_int);
        } else {
            C67_SUBSP(r, fr);
            C67_NOP(3 as libc::c_int);
        }
        vtop = vtop.offset(-1);
        vtop;
    } else if op == '*' as i32 {
        if ft & 0xf as libc::c_int == 9 as libc::c_int {
            C67_MPYDP(r, fr);
            C67_NOP(9 as libc::c_int);
        } else {
            C67_MPYSP(r, fr);
            C67_NOP(3 as libc::c_int);
        }
        vtop = vtop.offset(-1);
        vtop;
    } else if op == '/' as i32 {
        if ft & 0xf as libc::c_int == 9 as libc::c_int {
            vswap();
            vrott(3 as libc::c_int);
            gfunc_call(2 as libc::c_int);
            vpushi(0 as libc::c_int);
            (*vtop).r = TREG_XMM0 as libc::c_int as libc::c_ushort;
            (*vtop).r2 = TREG_RDX as libc::c_int as libc::c_ushort;
        } else {
            vswap();
            vrott(3 as libc::c_int);
            gfunc_call(2 as libc::c_int);
            vpushi(0 as libc::c_int);
            (*vtop).r = TREG_XMM0 as libc::c_int as libc::c_ushort;
            (*vtop).r2 = 0x30 as libc::c_int as libc::c_ushort;
        }
    } else if 0 as libc::c_int == 0 {
        _tcc_error(
            b"internal compiler error file at %s:%d\0" as *const u8
                as *const libc::c_char,
            b"/mnt/c/Users/mazin/Downloads/School/Research/datasets/tinycc/c67-gen.c\0"
                as *const u8 as *const libc::c_char,
            2413 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_itof(mut t: libc::c_int) {
    let mut r: libc::c_int = 0;
    gv(0x1 as libc::c_int);
    r = (*vtop).r as libc::c_int;
    if t & 0xf as libc::c_int == 9 as libc::c_int {
        if t & 0x10 as libc::c_int != 0 {
            C67_INTDPU(r, r);
        } else {
            C67_INTDP(r, r);
        }
        C67_NOP(4 as libc::c_int);
        (*vtop).type_0.t = 9 as libc::c_int;
    } else {
        if t & 0x10 as libc::c_int != 0 {
            C67_INTSPU(r, r);
        } else {
            C67_INTSP(r, r);
        }
        C67_NOP(3 as libc::c_int);
        (*vtop).type_0.t = 8 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gen_cvt_ftoi(mut t: libc::c_int) {
    let mut r: libc::c_int = 0;
    gv(0x2 as libc::c_int);
    r = (*vtop).r as libc::c_int;
    if t != 3 as libc::c_int {
        _tcc_error(b"long long not supported\0" as *const u8 as *const libc::c_char);
    } else {
        if (*vtop).type_0.t & 0xf as libc::c_int == 9 as libc::c_int {
            C67_DPTRUNC(r, r);
            C67_NOP(3 as libc::c_int);
        } else {
            C67_SPTRUNC(r, r);
            C67_NOP(3 as libc::c_int);
        }
        (*vtop).type_0.t = 3 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ggoto() {
    gcall_or_jmp(1 as libc::c_int);
    vtop = vtop.offset(-1);
    vtop;
}
#[no_mangle]
pub unsafe extern "C" fn gen_vla_sp_save(mut addr: libc::c_int) {
    _tcc_error(
        b"variable length arrays unsupported for this target\0" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_vla_sp_restore(mut addr: libc::c_int) {
    _tcc_error(
        b"variable length arrays unsupported for this target\0" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gen_vla_alloc(mut type_0: *mut CType, mut align: libc::c_int) {
    _tcc_error(
        b"variable length arrays unsupported for this target\0" as *const u8
            as *const libc::c_char,
    );
}
