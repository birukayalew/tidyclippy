use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    pub type _tccdbg;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn memcpy(
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
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn tcc_basename(name: *const libc::c_char) -> *mut libc::c_char;
    fn tcc_fileextension(name: *const libc::c_char) -> *mut libc::c_char;
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_realloc(ptr: *mut libc::c_void, size: libc::c_ulong) -> *mut libc::c_void;
    fn _tcc_error_noabort(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn tcc_enter_state(s1: *mut TCCState);
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
pub type Elf64_Half = uint16_t;
pub type Elf64_Word = uint32_t;
pub type Elf64_Off = uint64_t;
pub type Elf64_Section = uint16_t;
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
pub struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: libc::c_uchar,
    pub st_other: libc::c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArHdr {
    pub ar_name: [libc::c_char; 16],
    pub ar_date: [libc::c_char; 12],
    pub ar_uid: [libc::c_char; 6],
    pub ar_gid: [libc::c_char; 6],
    pub ar_mode: [libc::c_char; 8],
    pub ar_size: [libc::c_char; 10],
    pub ar_fmag: [libc::c_char; 2],
}
#[inline]
unsafe extern "C" fn is_space(mut ch: libc::c_int) -> libc::c_int {
    return (ch == ' ' as i32 || ch == '\t' as i32 || ch == '\u{b}' as i32
        || ch == '\u{c}' as i32 || ch == '\r' as i32) as libc::c_int;
}
unsafe extern "C" fn le2belong(mut ul: libc::c_ulong) -> libc::c_ulong {
    return ((ul & 0xff0000 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int)
        .wrapping_add(
            (ul & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int,
        )
        .wrapping_add((ul & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int)
        .wrapping_add((ul & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int);
}
unsafe extern "C" fn ar_usage(mut ret: libc::c_int) -> libc::c_int {
    fprintf(
        stderr,
        b"usage: tcc -ar [crstvx] lib [files]\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"create library ([abdiopN] not supported).\n\0" as *const u8
            as *const libc::c_char,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tool_ar(
    mut s1: *mut TCCState,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    static mut arhdr_init: ArHdr = unsafe {
        {
            let mut init = ArHdr {
                ar_name: *::core::mem::transmute::<
                    &[u8; 16],
                    &mut [libc::c_char; 16],
                >(b"/               "),
                ar_date: *::core::mem::transmute::<
                    &[u8; 12],
                    &mut [libc::c_char; 12],
                >(b"0           "),
                ar_uid: *::core::mem::transmute::<
                    &[u8; 6],
                    &mut [libc::c_char; 6],
                >(b"0     "),
                ar_gid: *::core::mem::transmute::<
                    &[u8; 6],
                    &mut [libc::c_char; 6],
                >(b"0     "),
                ar_mode: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b"0       "),
                ar_size: *::core::mem::transmute::<
                    &[u8; 10],
                    &mut [libc::c_char; 10],
                >(b"0         "),
                ar_fmag: *::core::mem::transmute::<
                    &[u8; 2],
                    &mut [libc::c_char; 2],
                >(b"`\n"),
            };
            init
        }
    };
    let mut arhdr: ArHdr = arhdr_init;
    let mut arhdro: ArHdr = arhdr_init;
    let mut fi: *mut FILE = 0 as *mut FILE;
    let mut fh: *mut FILE = 0 as *mut FILE;
    let mut fo: *mut FILE = 0 as *mut FILE;
    let mut created_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut ehdr: *mut Elf64_Ehdr = 0 as *mut Elf64_Ehdr;
    let mut shdr: *mut Elf64_Shdr = 0 as *mut Elf64_Shdr;
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut i: libc::c_int = 0;
    let mut fsize: libc::c_int = 0;
    let mut i_lib: libc::c_int = 0;
    let mut i_obj: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut symtab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strtab: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut symtabsize: libc::c_int = 0 as libc::c_int;
    let mut anames: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut afpos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut istrlen: libc::c_int = 0;
    let mut strpos: libc::c_int = 0 as libc::c_int;
    let mut fpos: libc::c_int = 0 as libc::c_int;
    let mut funccnt: libc::c_int = 0 as libc::c_int;
    let mut funcmax: libc::c_int = 0;
    let mut hofs: libc::c_int = 0;
    let mut tfile: [libc::c_char; 260] = [0; 260];
    let mut stmp: [libc::c_char; 20] = [0; 20];
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 2 as libc::c_int;
    let mut ops_conflict: *const libc::c_char = b"habdiopN\0" as *const u8
        as *const libc::c_char;
    let mut extract: libc::c_int = 0 as libc::c_int;
    let mut table: libc::c_int = 0 as libc::c_int;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    i_lib = 0 as libc::c_int;
    i_obj = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        let mut a: *const libc::c_char = *argv.offset(i as isize);
        if *a as libc::c_int == '-' as i32 && !(strchr(a, '.' as i32)).is_null() {
            ret = 1 as libc::c_int;
        }
        if *a as libc::c_int == '-' as i32
            || i == 1 as libc::c_int && (strchr(a, '.' as i32)).is_null()
        {
            if !(strpbrk(a, ops_conflict)).is_null() {
                ret = 1 as libc::c_int;
            }
            if !(strchr(a, 'x' as i32)).is_null() {
                extract = 1 as libc::c_int;
            }
            if !(strchr(a, 't' as i32)).is_null() {
                table = 1 as libc::c_int;
            }
            if !(strchr(a, 'v' as i32)).is_null() {
                verbose = 1 as libc::c_int;
            }
        } else if i_lib == 0 {
            i_lib = i;
        } else if i_obj == 0 {
            i_obj = i;
        }
        i += 1;
        i;
    }
    if i_lib == 0 {
        ret = 1 as libc::c_int;
    }
    i_obj = if i_obj != 0 { i_obj } else { argc };
    if ret == 1 as libc::c_int {
        return ar_usage(ret);
    }
    if extract != 0 || table != 0 {
        fh = fopen(
            *argv.offset(i_lib as isize),
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        if fh.is_null() {
            fprintf(
                stderr,
                b"tcc: ar: can't open file %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(i_lib as isize),
            );
        } else {
            fread(
                stmp.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                8 as libc::c_int as libc::c_ulong,
                fh,
            );
            if memcmp(
                stmp.as_mut_ptr() as *const libc::c_void,
                b"!<arch>\n\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                current_block = 4255445616763058643;
            } else {
                loop {
                    if !(fread(
                        &mut arhdr as *mut ArHdr as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        ::core::mem::size_of::<ArHdr>() as libc::c_ulong,
                        fh,
                    ) == ::core::mem::size_of::<ArHdr>() as libc::c_ulong)
                    {
                        current_block = 2520131295878969859;
                        break;
                    }
                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
                    if memcmp(
                        (arhdr.ar_fmag).as_mut_ptr() as *const libc::c_void,
                        b"`\n\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        2 as libc::c_int as libc::c_ulong,
                    ) != 0
                    {
                        current_block = 4255445616763058643;
                        break;
                    }
                    p = (arhdr.ar_name).as_mut_ptr();
                    e = p
                        .offset(
                            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                as isize,
                        );
                    while e > p
                        && *e.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == ' ' as i32
                    {
                        e = e.offset(-1);
                        e;
                    }
                    *e = '\0' as i32 as libc::c_char;
                    arhdr
                        .ar_size[(::core::mem::size_of::<[libc::c_char; 10]>()
                        as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as usize] = 0 as libc::c_int as libc::c_char;
                    fsize = atoi((arhdr.ar_size).as_mut_ptr());
                    buf = tcc_malloc((fsize + 1 as libc::c_int) as libc::c_ulong)
                        as *mut libc::c_char;
                    fread(
                        buf as *mut libc::c_void,
                        fsize as libc::c_ulong,
                        1 as libc::c_int as libc::c_ulong,
                        fh,
                    );
                    if strcmp(
                        (arhdr.ar_name).as_mut_ptr(),
                        b"/\0" as *const u8 as *const libc::c_char,
                    ) != 0
                        && strcmp(
                            (arhdr.ar_name).as_mut_ptr(),
                            b"/SYM64/\0" as *const u8 as *const libc::c_char,
                        ) != 0
                    {
                        if e > p
                            && *e.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                == '/' as i32
                        {
                            *e
                                .offset(
                                    -(1 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                        }
                        if table != 0 || verbose != 0 {
                            printf(
                                b"%s%s\n\0" as *const u8 as *const libc::c_char,
                                if extract != 0 {
                                    b"x - \0" as *const u8 as *const libc::c_char
                                } else {
                                    b"\0" as *const u8 as *const libc::c_char
                                },
                                (arhdr.ar_name).as_mut_ptr(),
                            );
                        }
                        if extract != 0 {
                            fo = fopen(
                                (arhdr.ar_name).as_mut_ptr(),
                                b"wb\0" as *const u8 as *const libc::c_char,
                            );
                            if fo.is_null() {
                                fprintf(
                                    stderr,
                                    b"tcc: ar: can't create file %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    (arhdr.ar_name).as_mut_ptr(),
                                );
                                tcc_free(buf as *mut libc::c_void);
                                current_block = 16975373889772506857;
                                break;
                            } else {
                                fwrite(
                                    buf as *const libc::c_void,
                                    fsize as libc::c_ulong,
                                    1 as libc::c_int as libc::c_ulong,
                                    fo,
                                );
                                fclose(fo);
                            }
                        }
                    }
                    if fsize & 1 as libc::c_int != 0 {
                        fgetc(fh);
                    }
                    tcc_free(buf as *mut libc::c_void);
                }
                match current_block {
                    4255445616763058643 => {}
                    16975373889772506857 => {}
                    _ => {
                        ret = 0 as libc::c_int;
                        current_block = 16975373889772506857;
                    }
                }
            }
            match current_block {
                16975373889772506857 => {}
                _ => {
                    fprintf(
                        stderr,
                        b"tcc: ar: not an ar archive %s\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(i_lib as isize),
                    );
                }
            }
        }
        if !fh.is_null() {
            fclose(fh);
        }
        return ret;
    } else {
        fh = fopen(
            *argv.offset(i_lib as isize),
            b"wb\0" as *const u8 as *const libc::c_char,
        );
        if fh.is_null() {
            fprintf(
                stderr,
                b"tcc: ar: can't create file %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(i_lib as isize),
            );
        } else {
            created_file = *argv.offset(i_lib as isize);
            sprintf(
                tfile.as_mut_ptr(),
                b"%s.tmp\0" as *const u8 as *const libc::c_char,
                *argv.offset(i_lib as isize),
            );
            fo = fopen(tfile.as_mut_ptr(), b"wb+\0" as *const u8 as *const libc::c_char);
            if fo.is_null() {
                fprintf(
                    stderr,
                    b"tcc: ar: can't create temporary file %s\n\0" as *const u8
                        as *const libc::c_char,
                    tfile.as_mut_ptr(),
                );
            } else {
                funcmax = 250 as libc::c_int;
                afpos = tcc_realloc(
                    0 as *mut libc::c_void,
                    (funcmax as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int;
                memcpy(
                    &mut arhdro.ar_mode as *mut [libc::c_char; 8] as *mut libc::c_void,
                    b"100644\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    6 as libc::c_int as libc::c_ulong,
                );
                loop {
                    if !(i_obj < argc) {
                        current_block = 17917672080766325409;
                        break;
                    }
                    if **argv.offset(i_obj as isize) as libc::c_int == '-' as i32 {
                        i_obj += 1;
                        i_obj;
                    } else {
                        fi = fopen(
                            *argv.offset(i_obj as isize),
                            b"rb\0" as *const u8 as *const libc::c_char,
                        );
                        if fi.is_null() {
                            fprintf(
                                stderr,
                                b"tcc: ar: can't open file %s \n\0" as *const u8
                                    as *const libc::c_char,
                                *argv.offset(i_obj as isize),
                            );
                            current_block = 17216933127195416668;
                            break;
                        } else {
                            if verbose != 0 {
                                printf(
                                    b"a - %s\n\0" as *const u8 as *const libc::c_char,
                                    *argv.offset(i_obj as isize),
                                );
                            }
                            fseek(
                                fi,
                                0 as libc::c_int as libc::c_long,
                                2 as libc::c_int,
                            );
                            fsize = ftell(fi) as libc::c_int;
                            fseek(
                                fi,
                                0 as libc::c_int as libc::c_long,
                                0 as libc::c_int,
                            );
                            buf = tcc_malloc((fsize + 1 as libc::c_int) as libc::c_ulong)
                                as *mut libc::c_char;
                            fread(
                                buf as *mut libc::c_void,
                                fsize as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                fi,
                            );
                            fclose(fi);
                            ehdr = buf as *mut Elf64_Ehdr;
                            if (*ehdr).e_ident[4 as libc::c_int as usize] as libc::c_int
                                != 2 as libc::c_int
                            {
                                fprintf(
                                    stderr,
                                    b"tcc: ar: Unsupported Elf Class: %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    *argv.offset(i_obj as isize),
                                );
                                current_block = 17216933127195416668;
                                break;
                            } else {
                                shdr = buf
                                    .offset((*ehdr).e_shoff as isize)
                                    .offset(
                                        ((*ehdr).e_shstrndx as libc::c_int
                                            * (*ehdr).e_shentsize as libc::c_int) as isize,
                                    ) as *mut Elf64_Shdr;
                                shstr = buf.offset((*shdr).sh_offset as isize);
                                strtab = 0 as *mut libc::c_char;
                                symtab = strtab;
                                i = 0 as libc::c_int;
                                while i < (*ehdr).e_shnum as libc::c_int {
                                    shdr = buf
                                        .offset((*ehdr).e_shoff as isize)
                                        .offset((i * (*ehdr).e_shentsize as libc::c_int) as isize)
                                        as *mut Elf64_Shdr;
                                    if !((*shdr).sh_offset == 0) {
                                        if (*shdr).sh_type == 2 as libc::c_int as Elf64_Word {
                                            symtab = buf.offset((*shdr).sh_offset as isize);
                                            symtabsize = (*shdr).sh_size as libc::c_int;
                                        }
                                        if (*shdr).sh_type == 3 as libc::c_int as Elf64_Word {
                                            if strcmp(
                                                shstr.offset((*shdr).sh_name as isize),
                                                b".strtab\0" as *const u8 as *const libc::c_char,
                                            ) == 0
                                            {
                                                strtab = buf.offset((*shdr).sh_offset as isize);
                                            }
                                        }
                                    }
                                    i += 1;
                                    i;
                                }
                                if !symtab.is_null() && !strtab.is_null() {
                                    let mut nsym: libc::c_int = (symtabsize as libc::c_ulong)
                                        .wrapping_div(
                                            ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong,
                                        ) as libc::c_int;
                                    i = 1 as libc::c_int;
                                    while i < nsym {
                                        sym = symtab
                                            .offset(
                                                (i as libc::c_ulong)
                                                    .wrapping_mul(
                                                        ::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong,
                                                    ) as isize,
                                            ) as *mut Elf64_Sym;
                                        if (*sym).st_shndx as libc::c_int != 0
                                            && ((*sym).st_info as libc::c_int == 0x10 as libc::c_int
                                                || (*sym).st_info as libc::c_int == 0x11 as libc::c_int
                                                || (*sym).st_info as libc::c_int == 0x12 as libc::c_int
                                                || (*sym).st_info as libc::c_int == 0x20 as libc::c_int
                                                || (*sym).st_info as libc::c_int == 0x21 as libc::c_int
                                                || (*sym).st_info as libc::c_int == 0x22 as libc::c_int)
                                        {
                                            istrlen = (strlen(strtab.offset((*sym).st_name as isize)))
                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                as libc::c_int;
                                            anames = tcc_realloc(
                                                anames as *mut libc::c_void,
                                                (strpos + istrlen) as libc::c_ulong,
                                            ) as *mut libc::c_char;
                                            strcpy(
                                                anames.offset(strpos as isize),
                                                strtab.offset((*sym).st_name as isize),
                                            );
                                            strpos += istrlen;
                                            funccnt += 1;
                                            if funccnt >= funcmax {
                                                funcmax += 250 as libc::c_int;
                                                afpos = tcc_realloc(
                                                    afpos as *mut libc::c_void,
                                                    (funcmax as libc::c_ulong)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                                        ),
                                                ) as *mut libc::c_int;
                                            }
                                            *afpos.offset(funccnt as isize) = fpos;
                                        }
                                        i += 1;
                                        i;
                                    }
                                }
                                file = *argv.offset(i_obj as isize);
                                name = strchr(file, 0 as libc::c_int);
                                while name > file
                                    && *name.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                        != '/' as i32
                                    && *name.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                        != '\\' as i32
                                {
                                    name = name.offset(-1);
                                    name;
                                }
                                istrlen = strlen(name) as libc::c_int;
                                if istrlen as libc::c_ulong
                                    >= ::core::mem::size_of::<[libc::c_char; 16]>()
                                        as libc::c_ulong
                                {
                                    istrlen = (::core::mem::size_of::<[libc::c_char; 16]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int;
                                }
                                memset(
                                    (arhdro.ar_name).as_mut_ptr() as *mut libc::c_void,
                                    ' ' as i32,
                                    ::core::mem::size_of::<[libc::c_char; 16]>()
                                        as libc::c_ulong,
                                );
                                memcpy(
                                    (arhdro.ar_name).as_mut_ptr() as *mut libc::c_void,
                                    name as *const libc::c_void,
                                    istrlen as libc::c_ulong,
                                );
                                arhdro
                                    .ar_name[istrlen as usize] = '/' as i32 as libc::c_char;
                                sprintf(
                                    stmp.as_mut_ptr(),
                                    b"%-10d\0" as *const u8 as *const libc::c_char,
                                    fsize,
                                );
                                memcpy(
                                    &mut arhdro.ar_size as *mut [libc::c_char; 10]
                                        as *mut libc::c_void,
                                    stmp.as_mut_ptr() as *const libc::c_void,
                                    10 as libc::c_int as libc::c_ulong,
                                );
                                fwrite(
                                    &mut arhdro as *mut ArHdr as *const libc::c_void,
                                    ::core::mem::size_of::<ArHdr>() as libc::c_ulong,
                                    1 as libc::c_int as libc::c_ulong,
                                    fo,
                                );
                                fwrite(
                                    buf as *const libc::c_void,
                                    fsize as libc::c_ulong,
                                    1 as libc::c_int as libc::c_ulong,
                                    fo,
                                );
                                tcc_free(buf as *mut libc::c_void);
                                i_obj += 1;
                                i_obj;
                                fpos = (fpos as libc::c_ulong)
                                    .wrapping_add(
                                        (fsize as libc::c_ulong)
                                            .wrapping_add(
                                                ::core::mem::size_of::<ArHdr>() as libc::c_ulong,
                                            ),
                                    ) as libc::c_int as libc::c_int;
                                if fpos & 1 as libc::c_int != 0 {
                                    fputc(0 as libc::c_int, fo);
                                    fpos += 1;
                                    fpos;
                                }
                            }
                        }
                    }
                }
                match current_block {
                    17216933127195416668 => {}
                    _ => {
                        hofs = (8 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<ArHdr>() as libc::c_ulong,
                            )
                            .wrapping_add(strpos as libc::c_ulong)
                            .wrapping_add(
                                ((funccnt + 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                    ),
                            ) as libc::c_int;
                        fpos = 0 as libc::c_int;
                        if hofs & 1 as libc::c_int != 0 {
                            hofs += 1;
                            hofs;
                            fpos = 1 as libc::c_int;
                        }
                        fwrite(
                            b"!<arch>\n\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            8 as libc::c_int as libc::c_ulong,
                            1 as libc::c_int as libc::c_ulong,
                            fh,
                        );
                        if funccnt == 0 {
                            ret = 0 as libc::c_int;
                        } else {
                            sprintf(
                                stmp.as_mut_ptr(),
                                b"%-10d\0" as *const u8 as *const libc::c_char,
                                (strpos as libc::c_ulong)
                                    .wrapping_add(
                                        ((funccnt + 1 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                            ),
                                    ) as libc::c_int + fpos,
                            );
                            memcpy(
                                &mut arhdr.ar_size as *mut [libc::c_char; 10]
                                    as *mut libc::c_void,
                                stmp.as_mut_ptr() as *const libc::c_void,
                                10 as libc::c_int as libc::c_ulong,
                            );
                            fwrite(
                                &mut arhdr as *mut ArHdr as *const libc::c_void,
                                ::core::mem::size_of::<ArHdr>() as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                fh,
                            );
                            *afpos
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = le2belong(funccnt as libc::c_ulong) as libc::c_int;
                            i = 1 as libc::c_int;
                            while i <= funccnt {
                                *afpos
                                    .offset(
                                        i as isize,
                                    ) = le2belong(
                                    (*afpos.offset(i as isize) + hofs) as libc::c_ulong,
                                ) as libc::c_int;
                                i += 1;
                                i;
                            }
                            fwrite(
                                afpos as *const libc::c_void,
                                ((funccnt + 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                    ),
                                1 as libc::c_int as libc::c_ulong,
                                fh,
                            );
                            fwrite(
                                anames as *const libc::c_void,
                                strpos as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                fh,
                            );
                            if fpos != 0 {
                                fwrite(
                                    b"\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    1 as libc::c_int as libc::c_ulong,
                                    1 as libc::c_int as libc::c_ulong,
                                    fh,
                                );
                            }
                            fseek(
                                fo,
                                0 as libc::c_int as libc::c_long,
                                2 as libc::c_int,
                            );
                            fsize = ftell(fo) as libc::c_int;
                            fseek(
                                fo,
                                0 as libc::c_int as libc::c_long,
                                0 as libc::c_int,
                            );
                            buf = tcc_malloc((fsize + 1 as libc::c_int) as libc::c_ulong)
                                as *mut libc::c_char;
                            fread(
                                buf as *mut libc::c_void,
                                fsize as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                fo,
                            );
                            fwrite(
                                buf as *const libc::c_void,
                                fsize as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                fh,
                            );
                            tcc_free(buf as *mut libc::c_void);
                            ret = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        if !anames.is_null() {
            tcc_free(anames as *mut libc::c_void);
        }
        if !afpos.is_null() {
            tcc_free(afpos as *mut libc::c_void);
        }
        if !fh.is_null() {
            fclose(fh);
        }
        if !created_file.is_null() && ret != 0 as libc::c_int {
            remove(created_file);
        }
        if !fo.is_null() {
            fclose(fo);
            remove(tfile.as_mut_ptr());
        }
        return ret;
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcc_tool_cross(
    mut s1: *mut TCCState,
    mut argv: *mut *mut libc::c_char,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut program: [libc::c_char; 4096] = [0; 4096];
    let mut a0: *mut libc::c_char = *argv.offset(0 as libc::c_int as isize);
    let mut prefix: libc::c_int = (tcc_basename(a0)).offset_from(a0) as libc::c_long
        as libc::c_int;
    snprintf(
        program.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"%.*s%s-tcc\0" as *const u8 as *const libc::c_char,
        prefix,
        a0,
        if target == 64 as libc::c_int {
            b"x86_64\0" as *const u8 as *const libc::c_char
        } else {
            b"i386\0" as *const u8 as *const libc::c_char
        },
    );
    if strcmp(a0, program.as_mut_ptr()) != 0 {
        let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
        *fresh0 = program.as_mut_ptr();
        execvp(*fresh0, argv as *const *mut libc::c_char);
    }
    tcc_enter_state(s1);
    (Some(
        _tcc_error_noabort
            as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
    ))
        .expect(
            "non-null function pointer",
        )(
        b"could not run '%s'\0" as *const u8 as *const libc::c_char,
        program.as_mut_ptr(),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn escape_target_dep(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = tcc_malloc(
        (strlen(s))
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while *s != 0 {
        if is_space(*s as libc::c_int) != 0 {
            let fresh1 = j;
            j = j + 1;
            *res.offset(fresh1 as isize) = '\\' as i32 as libc::c_char;
        }
        *res.offset(j as isize) = *s;
        s = s.offset(1);
        s;
        j += 1;
        j;
    }
    *res.offset(j as isize) = '\0' as i32 as libc::c_char;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn gen_makedeps(
    mut s1: *mut TCCState,
    mut target: *const libc::c_char,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut depout: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut escaped_targets: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut num_targets: libc::c_int = 0;
    if filename.is_null() {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"%.*s.d\0" as *const u8 as *const libc::c_char,
            (tcc_fileextension(target)).offset_from(target) as libc::c_long
                as libc::c_int,
            target,
        );
        filename = buf.as_mut_ptr();
    }
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        depout = fdopen(1 as libc::c_int, b"w\0" as *const u8 as *const libc::c_char);
    } else {
        depout = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    }
    if depout.is_null() {
        tcc_enter_state(s1);
        return (Some(
            _tcc_error_noabort
                as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
        ))
            .expect(
                "non-null function pointer",
            )(b"could not open '%s'\0" as *const u8 as *const libc::c_char, filename);
    }
    if (*s1).verbose != 0 {
        printf(b"<- %s\n\0" as *const u8 as *const libc::c_char, filename);
    }
    escaped_targets = tcc_malloc(
        ((*s1).nb_target_deps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    num_targets = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*s1).nb_target_deps {
        let mut current_block_16: u64;
        k = 0 as libc::c_int;
        loop {
            if !(k < i) {
                current_block_16 = 13056961889198038528;
                break;
            }
            if 0 as libc::c_int
                == strcmp(
                    *((*s1).target_deps).offset(i as isize),
                    *((*s1).target_deps).offset(k as isize),
                )
            {
                current_block_16 = 17833034027772472439;
                break;
            }
            k += 1;
            k;
        }
        match current_block_16 {
            13056961889198038528 => {
                let fresh2 = num_targets;
                num_targets = num_targets + 1;
                let ref mut fresh3 = *escaped_targets.offset(fresh2 as isize);
                *fresh3 = escape_target_dep(*((*s1).target_deps).offset(i as isize));
            }
            _ => {}
        }
        i += 1;
        i;
    }
    fprintf(depout, b"%s:\0" as *const u8 as *const libc::c_char, target);
    i = 0 as libc::c_int;
    while i < num_targets {
        fprintf(
            depout,
            b" \\\n  %s\0" as *const u8 as *const libc::c_char,
            *escaped_targets.offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(depout, b"\n\0" as *const u8 as *const libc::c_char);
    if (*s1).gen_phony_deps != 0 {
        i = 1 as libc::c_int;
        while i < num_targets {
            fprintf(
                depout,
                b"%s:\n\0" as *const u8 as *const libc::c_char,
                *escaped_targets.offset(i as isize),
            );
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < num_targets {
        tcc_free(*escaped_targets.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    tcc_free(escaped_targets as *mut libc::c_void);
    fclose(depout);
    return 0 as libc::c_int;
}
